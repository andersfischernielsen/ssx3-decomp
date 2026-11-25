//! `src/animation/animmodel.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::aianimeventmap::BUILTIN_DELETE;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::input::inputparse::o406_lw;
pub(crate) use crate::game::input::inputparse::o406_sw;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_add;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_lb;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_lbu;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_lw;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_sw;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_sw0;
pub(crate) use crate::game::visualfx::boardwakefx::gp;
pub(crate) use crate::game::visualfx::boardwakefx::ldf_2 as ldf;
pub(crate) use crate::game::visualfx::boardwakefx::lh;
pub(crate) use crate::game::visualfx::boardwakefx::lw;
pub(crate) use crate::game::visualfx::boardwakefx::p;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0030_db90,
        rider_model_compare_int,
        "RiderModel_CompareInt",
    ),
    (
        0x0030_dba0,
        rider_model_set_color_scale,
        "RiderModel_SetColorScale",
    ),
    (0x0030_e9e0, rider_model_show_part, "RiderModel_ShowPart"),
    (0x0030_ea80, rider_model_hide_part, "RiderModel_HidePart"),
    (
        0x0030_eb60,
        rider_model_hide_all_parts,
        "RiderModel_HideAllParts",
    ),
    (
        0x0030_ebc0,
        rider_model_is_part_visible,
        "RiderModel_IsPartVisible",
    ),
    (
        0x0030_ec18,
        rider_model_show_part_variant,
        "RiderModel_ShowPartVariant",
    ),
    (0x0030_eca0, rider_model_lock_part, "RiderModel_LockPart"),
    (
        0x0030_f2b0,
        rider_model_blend_anim_pose,
        "RiderModel_BlendAnimPose",
    ),
    (0x0031_0640, rider_model_render, "RiderModel_Render"),
    (
        0x0031_0948,
        rider_model_render_shadow,
        "RiderModel_RenderShadow",
    ),
    (0x0031_0ee0, rider_model_load_state, "RiderModel_LoadState"),
    (0x0031_1048, anim_data_base_ctor, "AnimDataBase_ctor"),
    (0x0031_1088, anim_data_base_dtor, "AnimDataBase_dtor"),
    (
        0x0031_10d0,
        anim_data_base_reset_chan_cache_table,
        "AnimDataBase_ResetChanCacheTable",
    ),
    (
        0x0031_1290,
        adf_data_source_set_data,
        "AdfDataSource_SetData",
    ),
    (0x0031_12c8, adf_data_source_dtor, "AdfDataSource_dtor"),
    (
        0x0031_1318,
        anim_data_base_sample_channel,
        "AnimDataBase_SampleChannel",
    ),
    (
        0x0030_ecd8,
        rider_model_sample_part_transform,
        "RiderModel_SamplePartTransform",
    ),
    (0x0031_0ea8, rider_model_save_state, "RiderModel_SaveState"),
    (
        0x0030_d8b8,
        rider_model_load_variant,
        "RiderModel_LoadVariant",
    ),
    (0x0030_dbd0, rider_model_setup, "RiderModel_Setup"),
    (0x0031_0c48, rider_model_find_bone, "RiderModel_FindBone"),
    (
        0x0031_0ce8,
        rider_model_get_bone_mask,
        "RiderModel_GetBoneMask",
    ),
    (
        0x0031_0f18,
        anim_data_base_alloc_chan_cache_2,
        "AnimDataBase_AllocChanCache",
    ),
    (
        0x0030_db70,
        rider_model_add_model_ref,
        "RiderModel_AddModelRef",
    ),
    (
        0x0031_1110,
        anim_data_base_free_chan_caches,
        "AnimDataBase_FreeChanCaches",
    ),
    (
        0x0031_1220,
        oc436_cadf_data_source_ctor,
        "cAdfDataSource_ctor",
    ),
    (
        0x0031_0458,
        rider_model_get_part_point_world,
        "RiderModel_GetPartPointWorld",
    ),
];

// ---------------------------------------------------------------- AnimSequence

/// Direct-call targets of the animation functions (shard s10).
pub(crate) mod anim_callee {
    use super::Callee;
    use ssx3_ee::s10 as ee;

    pub const FIRE_TRIGGERS_AT_TIME: Callee = Callee {
        addr: 0x0031_3938,
        f: ee::AnimSequence_FireTriggersAtTime,
    };

    pub const FIRE_END_TRIGGERS: Callee = Callee {
        addr: 0x0031_39a8,
        f: ee::AnimSequence_FireEndTriggers,
    };

    pub const FIRE_TRIGGERS_IN_RANGE: Callee = Callee {
        addr: 0x0031_3868,
        f: ee::AnimSequence_FireTriggersInRange,
    };

    pub const GET_TRACK_BLEND_WEIGHT: Callee = Callee {
        addr: 0x0031_3d70,
        f: ee::AnimSequence_GetTrackBlendWeight,
    };

    pub const ALLOC_CHAN_CACHE: Callee = Callee {
        addr: 0x0031_0f18,
        f: ee::AnimDataBase_AllocChanCache,
    };

    pub const DECODE_CHAN_CACHE: Callee = Callee {
        addr: 0x0031_30a8,
        f: ee::AnimDataBase_DecodeChanCache,
    };

    pub const EVAL_CHAN_CURVES: Callee = Callee {
        addr: 0x0031_2c20,
        f: ee::AnimDataBase_EvalChanCurves,
    };
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
        0x0031_2d4c,
        0x0031_2d54,
        0x0031_2d60,
        0x0031_2d78,
        0x0031_2d9c,
        0x0031_2dcc,
        0x0031_2e08,
        0x0031_2e4c,
        0x0031_2e90,
        0x0031_2ed8,
    ];

    pub const KEY_TABLE: [u32; 10] = [
        0x0031_2f3c,
        0x0031_2f44,
        0x0031_2f4c,
        0x0031_2f60,
        0x0031_2f80,
        0x0031_2fac,
        0x0031_2fec,
        0x0031_3014,
        0x0031_303c,
        0x0031_3068,
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
}

/// `RiderModel` (an `sAnimModel`): the two bone tables, the parts array and
/// the per-part LOD info.
#[allow(dead_code)]
pub(crate) mod model {
    pub(crate) use crate::game::ai::rider::model::BONE_TABLE_A;
    pub(crate) use crate::game::ai::rider::model::BONE_TABLE_A_SHADOW;
    pub(crate) use crate::game::ai::rider::model::BONE_TABLE_B;
    pub(crate) use crate::game::ai::rider::model::HAS_PARTS;
    #[cfg(test)]
    pub(crate) use crate::game::ai::rider::model::LOD_INFO;
    #[cfg(test)]
    pub(crate) use crate::game::ai::rider::model::PART_STRIDE;

    pub(crate) use crate::game::ai::rider::model::NUM_BONES;
    pub(crate) use crate::game::ai::rider::model::NUM_PARTS;
    pub(crate) use crate::game::ai::rider::model::PARTS;
    pub(crate) use crate::game::ai::rider::model::PART_LODS;
    pub(crate) use crate::game::ai::rider::model::PART_REF_INDEX;

    pub(crate) use crate::game::ai::rider::model::SCALE_Y;
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;

    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
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

pub(crate) const ADF_DATA_SOURCE_SET_DATA: Callee = Callee {
    addr: 0x0031_1290,
    f: ee::s10::f_00311290,
};

pub(crate) const MEMSET_ADDR: u32 = 0x0041_6210;

pub(crate) const ANIM_DATA_BASE_RESET: Callee = Callee {
    addr: 0x0031_10d0,
    f: ee::s10::f_003110d0,
};

pub(crate) const ANIM_DATA_BASE_FREE: Callee = Callee {
    addr: 0x0031_1110,
    f: ee::s10::f_00311110,
};

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
    let _y = r32(c, A3);
    c.g[S6][0] = c.g[A3][0];
    frame.save(c, S0, 0x110);
    let _x = r32(c, A2);
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
    let _y = r32(c, A3);
    c.g[S6][0] = c.g[A3][0];
    frame.save(c, S0, 0x110);
    let _x = r32(c, A2);
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
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                        'l_30f3a8: loop {
                                            c.cycles += 7;
                                            c.sw(c.g[V0][0] as u32, 0u64);
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
                                c.g[A0][0] = c.lw(c.g[T3][0] as u32);
                                c.g[V0][0] = 0x14;
                                c.g[A2][0] = c.lw(frame.at(c, 0x4c8c));
                                c.g[T1][0] = 0u64;
                                c.g[V1][0] = c.g[A0][0] & 0xff;
                                c.g[T5][0] = c.ld((c.g[S5][0] as u32).wrapping_add(0x150));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[A0][0] = sx((c.g[A0][0] as u32) >> 8);
                                c.g[V1][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.mult(0, 4, 4, 2);
                                c.g[V1][0] = c.lw(c.g[V1][0] as u32);
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
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
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    'l_30f4a0: loop {
                                        'b_30f4dc: {
                                            c.cycles += 4;
                                            c.g[A0][0] = c.lbu(c.g[A1][0] as u32);
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
                                                        .wrapping_add(c.g[V1][0] as u32));
                                                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
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
                                                            .wrapping_add(c.g[A0][0] as u32));
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
                                            sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
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
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[T6][0] as u32));
                                c.g[S1][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[T7][0] as u32));
                                c.g[S6][0] = c.g[V1][0];
                                'l_30f530: loop {
                                    'b_30f668: {
                                        'b_30f55c: {
                                            c.cycles += 5;
                                            c.g[V0][0] = c.ld(c.g[S0][0] as u32);
                                            c.g[V1][0] = !(0u64 | c.g[T1][0]);
                                            c.g[S2][0] = c.g[V0][0] & c.g[T1][0];
                                            let t = c.g[S2][0] != 0u64;
                                            c.g[V1][0] = c.g[V0][0] & c.g[V1][0];
                                            if t {
                                                break 'b_30f55c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                                                c.sd(c.g[S0][0] as u32, c.g[V1][0]);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[T2][0] as u32));
                                                c.g[V0][0] = sx((c.g[T0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                let t = true;
                                                c.sw(c.g[V0][0] as u32, 0u64);
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
                                                    c.sd(c.g[S1][0] as u32, c.g[V1][0]);
                                                    c.g[V0][0] = sx((c.g[S4][0] as u32)
                                                        .wrapping_add(c.g[A3][0] as u32));
                                                    c.g[V0][0] = sx((c.g[T0][0] as u32)
                                                        .wrapping_add(c.g[V0][0] as u32));
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
                                                    c.sw(c.g[V0][0] as u32, 0u64);
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
                                                c.sd(c.g[S0][0] as u32, c.g[S2][0]);
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
                                                        .wrapping_add(c.g[T2][0] as u32));
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
                                            c.g[V0][0] =
                                                sx((c.g[S4][0] as u32)
                                                    .wrapping_add(c.g[T2][0] as u32));
                                            break 'b_30f660;
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] =
                                            sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.swc1(1, c.g[V0][0] as u32);
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
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    c.sw(frame.at(c, 0x4ca4), c.g[A2][0]);
                                    c.g[V1][0] = sx(sp(c).wrapping_add(c.g[V0][0] as u32));
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
                                            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                                            c.g[V0][0] = c.g[V1][0] & 0xff;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 8);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.mult(0, 3, 3, 4);
                                            c.g[A0][0] = c.lw(frame.at(c, 0x4c8c));
                                            c.g[V0][0] =
                                                sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[A1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[A1][0] as u32));
                                            c.g[V1][0] =
                                                c.lhu((c.g[V1][0] as u32).wrapping_add(0x8));
                                            let t = c.g[V1][0] == 0u64;
                                            c.sw(frame.at(c, 0x4c9c), c.g[V1][0]);
                                            if t {
                                                break 'b_30ffa4;
                                            } else {
                                                c.cycles += 2;
                                                c.g[A1][0] = c.lw(c.g[S3][0] as u32);
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
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.mult(0, 5, 5, 3);
                                                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A2][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(c.g[A1][0] as u32));
                                                            c.g[V1][0] = c.lhu(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xa),
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.mult(0, 3, 18, 3);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 3);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[A2][0] as u32));
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
                                                                        c.g[V1][0] as u32,
                                                                    ));
                                                                c.g[V0][0] =
                                                                    c.lw(c.g[V0][0] as u32);
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
                                                                            c.g[A2][0] as u32,
                                                                        ));
                                                                    c.g[FP][0] = c.g[V0][0];
                                                                    c.g[V1][0] =
                                                                        c.ld(c.g[V1][0] as u32);
                                                                    c.g[S6][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[A0][0] as u32,
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
                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
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
                                                                                                    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
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
                                                                                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
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
                                                                                                    c.g[V0][0] = c.lbu(c.g[S3][0] as u32);
                                                                                                    c.g[V1][0] = c.lw(frame.at(c, 0x4c8c));
                                                                                                    c.g[A1][0] = sx(frame.at(c, 0x220));
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                                    c.g[A2][0] = c.g[S3][0];
                                                                                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                    c.g[A3][0] = c.g[S2][0];
                                                                                                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
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
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[S0][0] as i64)) as u64;
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.lwc1(0, c.g[V0][0] as u32);
    c.swc1(0, c.g[A2][0] as u32);
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
                                                                                            c.g[V0][0] = c.lbu(c.g[S3][0] as u32);
                                                                                            c.g[A0][0] = c.lw(frame.at(c, 0x4c8c));
                                                                                            c.g[A1][0] = c.g[S1][0];
                                                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                            c.g[A2][0] = c.g[S3][0];
                                                                                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                            c.g[A3][0] = c.g[S2][0];
                                                                                            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
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
                                                                                                c.sw(c.g[V0][0] as u32, c.g[V1][0]);
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
                                                                                        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                        if t {
                                                                                            break 'b_30f980;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            c.g[A1][0] = c.g[S1][0];
                                                                                            c.g[T1][0] = c.g[S0][0];
                                                                                            'l_30f958: loop {
                                                                                                c.cycles += 10;
                                                                                                c.lwc1(1, c.g[V0][0] as u32);
                                                                                                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                                                                                                c.lwc1(0, c.g[A1][0] as u32);
                                                                                                c.mul_s(1, 2, 1);
                                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                                                                                c.mul_s(0, 20, 0);
                                                                                                c.add_s(1, 1, 0);
                                                                                                c.swc1(1, c.g[V0][0] as u32);
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
                                                                                    c.g[S3][0]
                                                                                        as u32,
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
                                                                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                c.g[T0][0] = 0u64;
                                                                                c.g[A0][0] = c
                                                                                    .lw(c.g[V0][0]
                                                                                        as u32);
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
                                                                                    c.g[V0][0]
                                                                                        as u32,
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
                                                                                        c.g[FP][0]
                                                                                            as u32,
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
                                                                                        c.g[T6][0]
                                                                                            as u32,
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
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                                    c.g[A0][0] = c.g[V1][0];
                                                                                                    let t = true;
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                                                                                    if t {
    break 'b_30fa5c;
    } else {
    break 'b_30fa50;
    }
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 3;
                                                                                            c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                                                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                                                                            c.g[A0][0] = c.g[V1][0];
                                                                                            break 'b_30fa5c;
                                                                                        }
                                                                                        'b_30fc28: {
                                                                                            c.cycles += 8;
                                                                                            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                                                                            c.g[V0][0] = sx((c.g[T2][0] as u32) << 2);
                                                                                            c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x14));
                                                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                                                                            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                                                                                            c.g[V1][0] = c.g[V1][0] & 0x1;
                                                                                            let t = c.g[V1][0] == 0u64;
                                                                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
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
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
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
                                                                                                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                                                    c.lwc1(5, c.g[V0][0] as u32);
                                                                                                    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                                                                                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                                                                                                    c.swc1(5, c.g[A0][0] as u32);
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                    c.lwc1(3, c.g[V1][0] as u32);
                                                                                                    c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x4));
                                                                                                    c.lwc1(4, c.g[V0][0] as u32);
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
    c.sq(2, c.g[A0][0] as u32);
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
    c.lqc2(5, c.g[A0][0] as u32);
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
    c.lqc2(3, c.g[A0][0] as u32);
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
    c.sq(2, c.g[A0][0] as u32);
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
    c.sw(c.g[T4][0] as u32, c.g[T7][0]);
    if t {
    break 'b_30fc28;
    } else {
    break 'b_30fc00;
    }
    }
                                                                                                }
                                                                                                c.cycles += 10;
                                                                                                c.g[V1][0] = sx((c.g[T0][0] as u32) << 4);
                                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                c.mfc1(2, 20);
                                                                                                c.lqc2(4, c.g[V1][0] as u32);
                                                                                                c.qmtc2(2, 3);
                                                                                                c.lqc2(5, c.g[A0][0] as u32);
                                                                                                c.vu0().mula(0b1111, 4, Src::Bc(0, W));
                                                                                                c.vu0().msuba(0b1111, 4, Src::Bc(3, X));
                                                                                                c.vu0().madd(0b1111, 4, 5, Src::Bc(3, X));
                                                                                                c.sqc2(4, c.g[V1][0] as u32);
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
    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
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
                                                                                                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                                c.lwc1(0, c.g[V0][0] as u32);
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
                                                                                                c.swc1(3, c.g[A2][0] as u32);
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
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let t = true;
    c.lwc1(3, c.g[V0][0] as u32);
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
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.lwc1(0, c.g[V0][0] as u32);
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
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let t = true;
    c.lwc1(2, c.g[V0][0] as u32);
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
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.lwc1(0, c.g[V0][0] as u32);
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
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let t = true;
    c.lwc1(1, c.g[V0][0] as u32);
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
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.lwc1(0, c.g[V0][0] as u32);
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
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let t = true;
    c.lwc1(0, c.g[V0][0] as u32);
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
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.lwc1(0, c.g[V0][0] as u32);
    c.neg_s(0, 0);
    break 'b_30fe7c;
    }
    c.cycles += 7;
    frame.save_fpr(c, 3, 0x4c4c);
    frame.save_fpr(c, 2, 0x4c40);
    frame.save_fpr(c, 1, 0x4c44);
    frame.save_fpr(c, 0, 0x4c48);
    frame.restore(c, 2, 0x4c40);
    c.sq(2, c.g[A2][0] as u32);
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
                                                                                                    c.lqc2(5, c.g[A2][0] as u32);
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
                                                                                                    c.sq(2, c.g[A2][0] as u32);
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
    c.sw(c.g[T3][0] as u32, c.g[T7][0]);
    if t {
    break 'b_30ff70;
    } else {
    break 'b_30fef4;
    }
    }
                                                                                                }
                                                                                                c.cycles += 31;
                                                                                                c.g[V0][0] = sx((c.g[T0][0] as u32) << 4);
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                                                                                c.mfc1(4, 20);
                                                                                                c.lqc2(4, c.g[V0][0] as u32);
                                                                                                c.qmtc2(4, 3);
                                                                                                c.lqc2(5, c.g[A2][0] as u32);
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
                                                                                                c.sqc2(5, c.g[V0][0] as u32);
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
                                                                                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
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
                                                        c.g[A1][0] = c.lw(c.g[S3][0] as u32);
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
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
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
                                        sx((c.g[T2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                    c.g[A2][0] =
                                        sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[A1][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[T4][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[V0][0] =
                                        sx((c.g[T3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    if t {
                                        break 'b_310050;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
                                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 4);
                                        c.lq(4, (c.g[A3][0] as u32).wrapping_sub(0x10));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.sq(4, c.g[V1][0] as u32);
                                        c.g[V0][0] =
                                            sx((c.g[T3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                        break 'b_310050;
                                    }
                                }
                                'b_310074: {
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                    if c.g[V1][0] != 0u64 {
                                        c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                                        break 'b_310074;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28));
                                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 4);
                                        c.lq(4, c.g[A3][0] as u32);
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.sq(4, c.g[V1][0] as u32);
                                        c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                                        break 'b_310074;
                                    }
                                }
                                c.cycles += 7;
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x50));
                                c.g[V1][0] =
                                    sx((c.g[T2][0] as u32).wrapping_add(c.g[A1][0] as u32));
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
                            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            let t = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            if t {
                                break 'b_3100d8;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(sp(c)));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x820));
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
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
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
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

pub(crate) mod sample_part_xf {
    use crate::call::Callee;

    /// `gp + 0xd08`: the registered animation databases, indexed by the low
    /// byte of an anim ref's first word.
    pub const ANIM_DBS_GP: u32 = 0xd08;

    /// The identity transform (position, rotation) at `0x500000 - 0xdd0`.
    pub const IDENTITY_XF: u32 = 0x0050_0000 - 0xdd0;

    /// The origin the root position is taken relative to (`0x500000 - 0xed0`).
    #[cfg(test)]
    pub const ROOT_ORIGIN: u32 = 0x0050_0000 - 0xed0;

    // AnimDataBase: +8 clip table (0x14-byte clips), +0xc chunk descriptors.
    pub const DB_CLIPS: u32 = 0x8;

    pub const DB_CHUNKS: u32 = 0xc;

    pub const CLIP_SIZE: u32 = 0x14;

    // Clip: +4 first chunk descriptor, +8 channel count, +0xa chunks per
    // channel. Chunk descriptor (8 bytes): +4 the part id of its channel.
    pub const CLIP_FIRST_CHUNK: u32 = 0x4;

    pub const CLIP_CHANNELS: u32 = 0x8;

    pub const CLIP_CHUNKS: u32 = 0xa;

    pub const DESC_PART: u32 = 0x4;

    // Anim state (the anim ref): +0 db index | clip << 8, +0x20 root
    // position, +0x30 root rotation (quaternion), +0x40 use-alternate-curves
    // and scale flag.
    pub const ST_ROOT_POS: u32 = 0x20;

    pub const ST_ROOT_ROT: u32 = 0x30;

    pub const ST_ALT: u32 = 0x40;

    // RiderModel: +0xc bones (0x58 bytes), +0x1c bone index per part, +0x20
    // first curve per curve slot.
    pub const MODEL_BONES: u32 = 0xc;

    pub const MODEL_PART_BONE: u32 = 0x1c;

    pub const MODEL_CURVE_BASE: u32 = 0x20;

    pub const BONE_SIZE: u32 = 0x58;

    // Bone: +4 curve slot, +0x38 bone info.
    pub const BONE_SLOT: u32 = 0x4;

    pub const BONE_INFO: u32 = 0x38;

    // Bone info: +0x10 parent (-1: root), +0x16 alternate curve slot offset,
    // +0x18..0x1b rotation axis selectors (0..3 component, 4..7 negated),
    // +0x40 position scale.
    pub const INFO_PARENT: u32 = 0x10;

    pub const INFO_ALT_SLOT: u32 = 0x16;

    pub const INFO_SCALE: u32 = 0x40;

    pub const SAMPLE_CHANNEL: Callee = Callee {
        addr: 0x0031_1318,
        f: ssx3_ee::s10::AnimDataBase_SampleChannel,
    };
}

// ---------------------------------------------------------------------------
// 0x00310ea8 RiderModel_SaveState

/// RiderModel fields: bone count and the bone array (32 bytes per bone).
pub(crate) const RM_NUM_BONES: u32 = 0x10;

pub(crate) const RM_BONES: u32 = 0x2c;

/// `void RiderModel_SaveState(RiderModel* self, cStream* stream)`
/// (0x310ea8): write the bone array (`numBones * 32` bytes) to the stream
/// (`stream->write`, vtable slot 1 at `+0x8`). The twin of
/// `RiderModel_LoadState`.
pub fn rider_model_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        let stream = c.g[A1][0];
        c.g[T0][0] = stream;
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(stream as u32);
        c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(RM_NUM_BONES));
        c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(RM_BONES));
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 5);
        c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let write = c.g[A3][0] as u32;
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, write, 0x0031_0ed4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Xform* RiderModel_SamplePartTransform(Xform* out, RiderModel* model,
/// AnimState* st, int part)` (0x30ecd8): find the channel of the state's
/// clip that animates `part` (the first one whose first chunk descriptor
/// carries that part id) and sample its bone's position (3 curves, w = 1,
/// optionally scaled by the bone's scale) and rotation (3 curves, a unit
/// quaternion with the axes remapped). A root bone (parent -1) is moved
/// into the state's root frame (rotated by the root rotation, offset by the
/// root position). With no matching channel the result is the identity
/// transform. Returns `out` (position at +0, rotation at +0x10).
pub fn rider_model_sample_part_transform(c: &mut Cpu) {
    use sample_part_xf as k;
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x110);
        c.g[T0][0] = 0x14;
        frame.save(c, S2, 0xd0);
        frame.save(c, S3, 0xc0);
        frame.save(c, S4, 0xb0);
        frame.save(c, S0, 0xf0);
        frame.save(c, S1, 0xe0);
        frame.save(c, S5, 0xa0);
        frame.save(c, S6, 0x90);
        frame.save(c, S7, 0x80);
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 20, 0x100);
        // s2 = state, s3 = channel index, s4 = out
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = 0;
        c.g[S4][0] = c.g[A0][0];
        let st = Addr(c.g[S2][0] as u32);
        // t2 = the database table, t1 = the clip's channel count, t0 = model
        let dbs = Addr(abi::r32(c, GP).wrapping_add(k::ANIM_DBS_GP)).lw(c);
        let word = st.lw(c) as u32;
        c.g[T2][0] = dbs;
        c.g[V0][0] = sx((word & 0xff) << 2);
        c.g[V1][0] = sx(word >> 8);
        c.g[A2][0] = abi::addu(dbs, c.g[V0][0]);
        c.mult(0, V1, V1, T0);
        c.g[A0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(k::DB_CLIPS).lw(c);
        c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
        c.g[T1][0] = Addr(c.g[V1][0] as u32).off(k::CLIP_CHANNELS).lhu(c);
        c.g[T0][0] = c.g[A1][0];
        let mut found = false;
        if c.g[T1][0] != 0 {
            c.cycles += 7;
            c.g[AT][0] = FpuF32::ONE.0 as u64;
            c.f[20] = FpuF32::ONE.0;
            c.g[S7][0] = 0x50_0000;
            c.g[S5][0] = sx(sp(c).wrapping_add(0x10));
            c.g[S6][0] = u64::MAX;
            c.g[A1][0] = Addr(c.g[S2][0] as u32).lw(c);
            loop {
                if channel_part_matches(c) {
                    sample_matched_part(c)?;
                    found = true;
                    break;
                }
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A1][0] = Addr(c.g[S2][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0030_ed68);
                }
            }
        }
        if !found {
            // no channel animates the part: the identity transform
            c.cycles += 6;
            c.g[V0][0] = k::IDENTITY_XF as u64;
            c.lq(3, k::IDENTITY_XF + 0x10);
            c.lq(4, k::IDENTITY_XF);
            let out = c.g[S4][0] as u32;
            c.sq(3, out.wrapping_add(0x10));
            c.sq(4, out);
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    });

    /// One step of the channel search (a1 = the state's first word, s3 =
    /// channel): the part id of the channel's first chunk descriptor (in
    /// v1), true when it is `part` (a3).
    fn channel_part_matches(c: &mut Cpu) -> bool {
        c.cycles += 19;
        let word = c.g[A1][0] as u32;
        c.g[V1][0] = k::CLIP_SIZE as u64;
        c.g[V0][0] = sx((word & 0xff) << 2);
        c.g[A1][0] = sx(word >> 8);
        c.mult(0, A1, A1, V1);
        c.g[V0][0] = abi::addu(c.g[T2][0], c.g[V0][0]);
        let db = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] = db;
        c.g[A0][0] = Addr(db as u32).off(k::DB_CLIPS).lw(c);
        c.g[A2][0] = Addr(db as u32).off(k::DB_CHUNKS).lw(c);
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[A1][0]);
        let clip = Addr(c.g[A0][0] as u32);
        c.g[V1][0] = clip.off(k::CLIP_CHUNKS).lhu(c);
        c.g[V0][0] = clip.off(k::CLIP_FIRST_CHUNK).lw(c);
        c.mult(0, V1, S3, V1);
        c.g[V0][0] = abi::addu(c.g[V0][0], c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[V0][0] = abi::addu(c.g[V0][0], c.g[A2][0]);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(k::DESC_PART).lbu(c);
        c.g[V1][0] == c.g[A3][0]
    }

    /// The matching channel's bone: position, rotation, root frame, then
    /// the result into `out`.
    fn sample_matched_part(c: &mut Cpu) -> Flow {
        let base_addr = curve_base_slot(c);
        sample_position(c, base_addr)?;
        if is_root(c) {
            position_to_root_frame(c);
        } else {
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lbu(c);
        }
        sample_rotation(c)?;
        c.cycles += 3;
        let parent = Addr(c.g[S0][0] as u32).off(k::INFO_PARENT).lh(c);
        c.g[V0][0] = parent;
        let s = sp(c);
        c.lq(2, s);
        if parent == c.g[S6][0] {
            rotation_to_root_frame(c);
        }
        c.cycles += 4;
        let s = sp(c);
        c.lq(3, s.wrapping_add(0x10));
        let out = c.g[S4][0] as u32;
        c.sq(2, out);
        c.sq(3, out.wrapping_add(0x10));
        Ok(())
    }

    /// The bone of `part` (a3) and its curve slots: s0 = bone info, a3 =
    /// curve slot; returns the address of the curve-base entry to sample
    /// from (the alternate slot `+info.alt` when the state's flag is set),
    /// with a0/a1 = the base table and the slot's byte offset.
    fn curve_base_slot(c: &mut Cpu) -> u32 {
        c.cycles += 12;
        let model = Addr(c.g[T0][0] as u32);
        c.g[V0][0] = model.off(k::MODEL_PART_BONE).lw(c);
        c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
        c.g[A1][0] = k::BONE_SIZE as u64;
        c.g[A0][0] = model.off(k::MODEL_BONES).lw(c);
        c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
        c.g[A2][0] = Addr(c.g[S2][0] as u32).off(k::ST_ALT).lw(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.mult(0, V0, V0, A1);
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[V0][0]);
        let bone = Addr(c.g[A0][0] as u32);
        c.g[S0][0] = bone.off(k::BONE_INFO).lw(c);
        c.g[A3][0] = bone.off(k::BONE_SLOT).lw(c);
        let slot = c.g[A3][0] as u32;
        if c.g[A2][0] != 0 {
            c.cycles += 8;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(k::INFO_ALT_SLOT).lh(c);
            c.g[A1][0] = sx(slot << 2);
            c.g[V1][0] = model.off(k::MODEL_CURVE_BASE).lw(c);
            c.g[V0][0] = abi::addu(c.g[V0][0], slot as u64);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A0][0] = c.g[V1][0];
            c.g[V0][0] = abi::addu(c.g[V0][0], c.g[V1][0]);
        } else {
            c.cycles += 5;
            c.g[V1][0] = model.off(k::MODEL_CURVE_BASE).lw(c);
            c.g[V0][0] = sx(slot << 2);
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = abi::addu(c.g[V0][0], c.g[V1][0]);
            c.g[A0][0] = c.g[V1][0];
        }
        c.g[V0][0] as u32
    }

    /// `AnimDataBase_SampleChannel(db, out, st, channel, first, 3)` for the
    /// state's database (v0/v1 = its table index and entry, as left by the
    /// original).
    fn sample_curves(c: &mut Cpu, out: u32, first: u64, ret: u32) -> Flow {
        let st = c.g[S2][0] as u32;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
        c.g[A1][0] = sx(out);
        c.g[A2][0] = sx(st);
        c.g[A3][0] = c.g[S3][0];
        c.g[T0][0] = first;
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[T1][0] = 3;
        k::SAMPLE_CHANNEL.call(c, ret)
    }

    /// The position: curves `s1 = base[slot'] - base[slot]` onward into
    /// sp+0x20, w = 1 (f20), copied to sp+0 (through sp+0x30), multiplied by
    /// the bone's scale when the state's flag is set.
    fn sample_position(c: &mut Cpu, base_addr: u32) -> Flow {
        c.cycles += 26;
        c.g[S1][0] = Addr(base_addr).lw(c);
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lbu(c);
        c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
        c.g[V1][0] = Addr(abi::r32(c, GP).wrapping_add(k::ANIM_DBS_GP)).lw(c);
        let slot_base = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A3][0] = slot_base;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(slot_base as u32));
        let s = sp(c);
        let first = c.g[S1][0];
        sample_curves(c, s.wrapping_add(0x20), first, 0x0030_ee54)?;
        let s = sp(c);
        for i in 0..3 {
            let v = Addr(s + 0x20 + 4 * i).read_fpu(c);
            c.f[i as usize] = v.0;
        }
        for i in 0..3 {
            let v = FpuF32(c.f[i as usize]);
            Addr(s + 0x30 + 4 * i).write_fpu(c, v);
        }
        let w = FpuF32(c.f[20]);
        Addr(s + 0x3c).write_fpu(c, w);
        c.lq(2, s + 0x30);
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(k::ST_ALT).lw(c);
        c.sq(2, s);
        if c.g[V1][0] != 0 {
            c.cycles += 18;
            let scale = Addr(c.g[S0][0] as u32).off(k::INFO_SCALE);
            let mut sc = [FpuF32::ZERO; 4];
            for i in 0..4 {
                sc[i] = scale.off(4 * i as u32).read_fpu(c);
                c.f[4 + i] = sc[i].0;
            }
            let mut p = [FpuF32::ZERO; 4];
            for i in 0..4 {
                p[i] = Addr(s + 4 * i as u32).read_fpu(c) * sc[i];
            }
            // f1..f3 = x, y, z; f0 = w
            c.f[1] = p[0].0;
            c.f[2] = p[1].0;
            c.f[3] = p[2].0;
            c.f[0] = p[3].0;
            for i in 0..4 {
                Addr(s + 0x30 + 4 * i as u32).write_fpu(c, p[i]);
            }
            c.lq(2, s + 0x30);
            c.sq(2, s);
        }
        Ok(())
    }

    /// `info.parent == -1` (v0 = the parent).
    fn is_root(c: &mut Cpu) -> bool {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(k::INFO_PARENT).lh(c);
        c.g[V0][0] == c.g[S6][0]
    }

    /// vf8.xyz = the vector `vec` rotated by the quaternion `quat`
    /// (q v q* through two cross products), vf8.x zeroed first.
    fn rotate_by_quat(c: &mut Cpu, quat: u32, vec: u32) {
        c.lqc2(4, quat);
        c.lqc2(5, vec);
        let vu = c.vu0();
        vu.sub(0b0001, 8, 8, Src::Vec(8));
        vu.opmula(4, 5);
        vu.opmsub(6, 5, 4);
        vu.opmula(4, 6);
        vu.opmsub(7, 6, 4);
        vu.mula(0b1110, 5, Src::Bc(0, W));
        vu.madda(0b1110, 6, Src::Bc(4, W));
        vu.madda(0b1110, 6, Src::Bc(4, W));
        vu.madda(0b1110, 7, Src::Bc(0, W));
        vu.madd(0b1110, 8, 7, Src::Bc(0, W));
    }

    /// A root bone's position into the state's frame: a direction (w = 0)
    /// is only rotated; a point is taken relative to the origin, rotated
    /// and offset by the root position. The result goes to sp+0 (via
    /// sp+0x30); v0 = the state's db index.
    fn position_to_root_frame(c: &mut Cpu) {
        c.cycles += 5;
        let s = sp(c);
        let w = Addr(s + 0xc).read_fpu(c);
        c.f[1] = w.0;
        c.f[0] = 0;
        let direction = w.eq_s(FpuF32::ZERO);
        abi::set_fcc(c, direction);
        let st = c.g[S2][0] as u32;
        if direction {
            c.cycles += 15;
            rotate_by_quat(c, st + k::ST_ROOT_ROT, s);
            c.sqc2(8, s + 0x40);
            c.lq(2, s + 0x40);
        } else {
            c.cycles += 26;
            c.lqc2(3, s);
            let origin = (c.g[S7][0] as u32).wrapping_sub(0xed0);
            c.lqc2(4, origin);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s + 0x60);
            c.lq(2, s + 0x60);
            c.sq(2, s + 0x50);
            rotate_by_quat(c, st + k::ST_ROOT_ROT, s + 0x50);
            c.sqc2(8, s + 0x60);
            c.lq(2, s + 0x60);
            c.sq(2, s + 0x40);
            c.lqc2(3, s + 0x40);
            c.lqc2(4, st + k::ST_ROOT_POS);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s + 0x60);
            c.lq(2, s + 0x60);
        }
        c.cycles += 4;
        c.sq(2, s + 0x30);
        c.lq(2, s + 0x30);
        c.sq(2, s);
        c.g[V0][0] = Addr(st).lbu(c);
    }

    /// The rotation: curves `s1 + 3` onward into sp+0x30 (v0 = the state's
    /// db index), completed to a unit quaternion at sp+0x10, its axes
    /// remapped when the state's flag is set.
    fn sample_rotation(c: &mut Cpu) -> Flow {
        c.cycles += 92;
        let first = sx((c.g[S1][0] as u32).wrapping_add(3));
        c.g[V1][0] = Addr(abi::r32(c, GP).wrapping_add(k::ANIM_DBS_GP)).lw(c);
        let s = sp(c);
        sample_curves(c, s.wrapping_add(0x30), first, 0x0030_efc4)?;
        quat_from_vector_part(c);
        if c.g[A2][0] != 0 {
            remap_axis(c, 0x1b, 3, true);
            remap_axis(c, 0x18, 2, false);
            remap_axis(c, 0x19, 1, false);
            remap_axis(c, 0x1a, 0, false);
            c.cycles += 6;
            let s = sp(c);
            for (f, off) in [(3usize, 0x4cu32), (2, 0x40), (1, 0x44), (0, 0x48)] {
                let v = FpuF32(c.f[f]);
                Addr(s + off).write_fpu(c, v);
            }
            c.lq(2, s + 0x40);
            c.sq(2, s + 0x10);
        }
        Ok(())
    }

    /// The quaternion from its sampled vector part (sp+0x30..0x38): the
    /// MMI/VU0 sequence the original issues to get w = sqrt(1 - |v|^2)
    /// with the signs restored, written to sp+0x10 as (x, y, z, w) products;
    /// a2 = the state's flag.
    fn quat_from_vector_part(c: &mut Cpu) {
        let s = sp(c);
        // (x, y, z, 0) to sp+0x40 through f0..f2
        for (f, src) in [(1usize, 0x34u32), (2, 0x38), (0, 0x30)] {
            let v = Addr(s + src).read_fpu(c);
            c.f[f] = v.0;
        }
        for (f, dst) in [(1usize, 0x44u32), (2, 0x48), (0, 0x40)] {
            let v = FpuF32(c.f[f]);
            Addr(s + dst).write_fpu(c, v);
        }
        Addr(s + 0x4c).sw(c, 0);
        c.lqc2(3, s + 0x40);
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
        c.sq(5, s + 0x50);
        c.sqc2(10, s + 0x60);
        for i in 0..3 {
            let v = Addr(s + 0x60 + 4 * i).read_fpu(c).sqrt();
            c.f[0] = v.0;
            Addr(s + 0x60 + 4 * i).write_fpu(c, v);
        }
        c.lqc2(3, s + 0x60);
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
        c.sq(5, s + 0x60);
        let rd = |c: &mut Cpu, off: u32| Addr(s + off).read_fpu(c);
        let f4 = rd(c, 0x50);
        let f0 = rd(c, 0x54);
        let f1 = rd(c, 0x68);
        let f0 = f4 * f0;
        let f5 = rd(c, 0x58);
        let f2 = rd(c, 0x64);
        let f3 = rd(c, 0x60);
        let f1 = f0 * f1;
        c.g[A2][0] = Addr(c.g[S2][0] as u32).off(k::ST_ALT).lw(c);
        let f4 = f4 * f2;
        let f0 = f0 * f5;
        c.f[0] = f0.0;
        c.f[1] = f1.0;
        c.f[2] = f2.0;
        c.f[3] = f3.0;
        c.f[4] = f4.0;
        c.f[5] = f5.0;
        Addr(s + 0x10).write_fpu(c, f3);
        Addr(s + 0x14).write_fpu(c, f1);
        Addr(s + 0x18).write_fpu(c, f4);
        Addr(s + 0x1c).write_fpu(c, f0);
    }

    /// One rotation component of the axis remap into `f[dest]`: component
    /// `sel` of sp+0x10 for selector `sel < 4`, else component `sel - 4`
    /// negated (through f0). The first one also sets v1 = sp+0x10 (and
    /// reads through s5, the same address).
    fn remap_axis(c: &mut Cpu, field: u32, dest: usize, first: bool) {
        let info = Addr(c.g[S0][0] as u32);
        c.cycles += 4;
        let sel = info.off(field).lbu(c) as u32;
        let direct = sel < 4;
        let base = if first {
            c.g[V1][0] = sx(sp(c).wrapping_add(0x10));
            c.cycles += if direct { 5 } else { 6 };
            c.g[S5][0] as u32
        } else {
            c.cycles += if direct { 4 } else { 5 };
            c.g[V1][0] as u32
        };
        let index = if direct { sel } else { sel - 4 };
        let at = base.wrapping_add(index << 2);
        c.g[V0][0] = sx(at);
        let v = Addr(at).read_fpu(c);
        if direct {
            c.f[dest] = v.0;
        } else {
            c.f[0] = v.0;
            c.f[dest] = (-v).0;
        }
    }

    /// A root bone's rotation into the state's frame: sp+0x10 = root
    /// rotation * rotation (quaternion product), also at sp+0x40; v0's
    /// quadword ($2) is reloaded from sp+0.
    fn rotation_to_root_frame(c: &mut Cpu) {
        c.cycles += 16;
        let s = sp(c);
        let st = c.g[S2][0] as u32;
        c.lqc2(4, st + k::ST_ROOT_ROT);
        c.lqc2(5, s + 0x10);
        let vu = c.vu0();
        vu.mul(0b1111, 7, 4, Src::Vec(5));
        vu.opmula(4, 5);
        vu.opmsub(6, 5, 4);
        vu.mula(0b1110, 4, Src::Bc(5, W));
        vu.madda(0b1110, 5, Src::Bc(4, W));
        vu.suba(0b0001, 7, Src::Bc(7, X));
        vu.msuba(0b0001, 0, Src::Bc(7, Y));
        vu.msub(0b0001, 8, 0, Src::Bc(7, Z));
        vu.madd(0b1110, 8, 6, Src::Bc(0, W));
        c.sqc2(8, s + 0x50);
        c.lq(2, s + 0x50);
        c.sq(2, s + 0x10);
        c.sq(2, s + 0x40);
        c.lq(2, s);
    }
}

// ------------------------------------------------------ RiderModel_FindBone

/// `int RiderModel_FindBone(RiderModel *self, int part, const char *name)`
/// (0x310c48): the index of the bone called `name` among the part's bone
/// records (0x50 bytes each from the part's table), or -1. The table entry
/// (0x58 bytes at `self+0xc`) holds the bone array pointer (+0x38), the
/// base index (+4) and the count (+0x44).
pub fn rider_model_find_bone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S0, 0x40);
        c.g[V1][0] = 0x58;
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = o406_lw(c, A0, 0x1c);
        c.g[A3][0] = o406_lw(c, A0, 0xc);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = o406_lw(c, A1, 0);
        c.mult(0, V0, V0, V1);
        c.g[S2][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = o406_lw(c, S2, 0x44);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = o406_lw(c, S2, 0x38);
        'body: {
            if !empty {
                c.cycles += 1;
                c.g[S1][0] = c.g[A0][0];
                loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x0041_65a8, 0x0031_0ca4)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = o406_lw(c, S2, 4);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        break 'body;
                    }
                    c.g[V0][0] = o406_lw(c, S2, 0x44);
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let again = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x50));
                    if !again {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_0c98);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------- RiderModel_LoadVariant

/// `void RiderModel_LoadVariant(RiderModel *self, int lod, ModelPart *part,
/// int index)` (0x30d8b8): register a loaded model part under its id
/// (`part+0x52`). If the part table (count +8, 0x58-byte entries at +0xc)
/// has the id already, the entry's LOD record gets `used = 1` and the part
/// and its data (`part+0x60`) filed at `index`. Otherwise a new entry is
/// appended: four LOD ref tables are allocated and cleared, then the same
/// fields plus the bone names, the two other data offsets, the bone count
/// and the two counters are filled in.
pub fn rider_model_load_variant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S2, 0x80);
        frame.save(c, S4, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S7, 0x30);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, FP, 0x20);
        c.g[S7][0] = sx((c.g[S4][0] as u32).wrapping_add(0x60));
        frame.save(c, S0, 0xa0);
        c.g[FP][0] = c.g[A3][0];
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = o406_lw(c, S2, 8);
        let no_parts = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = 0;
        'exit: {
            if !no_parts {
                c.cycles += 4;
                c.g[T0][0] = sx((c.g[A1][0] as u32) << 3);
                c.g[T2][0] = 1;
                c.g[T1][0] = sx((c.g[FP][0] as u32) << 4);
                c.g[A3][0] = 0;
                loop {
                    c.cycles += 6;
                    c.g[V0][0] = o406_lw(c, S2, 0xc);
                    let a = (c.g[S4][0] as u32).wrapping_add(0x52);
                    c.g[A0][0] = c.lb(a);
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = o406_lw(c, V0, 0);
                    if c.g[V1][0] == c.g[A0][0] {
                        // The part is known: file this lod's ref in it.
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        o406_sw(c, V1, 0x18, T2);
                        c.g[V0][0] = o406_lw(c, S2, 0xc);
                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V1][0] = o406_lw(c, V0, 0x1c);
                        c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        o406_sw(c, V1, 0, S4);
                        c.g[V0][0] = o406_lw(c, S2, 0xc);
                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V1][0] = o406_lw(c, V0, 0x1c);
                        c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        o406_sw(c, V1, 4, S7);
                        break 'exit;
                    }
                    c.g[V0][0] = o406_lw(c, S2, 8);
                    c.cycles += 4;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x58));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_d910);
                    }
                }
            }
            // A new part entry.
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[FP][0] as u32) << 4);
            c.g[S6][0] = sx((c.g[A1][0] as u32) << 3);
            let a = frame.at(c, 0);
            c.sw(a, c.g[V0][0]);
            c.g[S3][0] = 0;
            c.g[S5][0] = 0x58;
            c.g[S0][0] = o406_lw(c, S2, 8);
            loop {
                c.cycles += 28;
                c.g[S1][0] = sx((c.g[S3][0] as u32) << 3);
                c.g[V0][0] = o406_lw(c, S2, 0xc);
                c.g[V1][0] = 0x0049_0000;
                c.mult(0, S0, S0, S5);
                c.g[A0][0] = o406_lw(c, S2, 4);
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6380));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                call_ptr(c, 0x0031_7e30, 0x0030_d9c8)?;
                o406_sw(c, S0, 0, V0);
                c.g[A1][0] = 0;
                c.g[V0][0] = o406_lw(c, S2, 8);
                c.g[V1][0] = o406_lw(c, S2, 0xc);
                c.mult(0, V0, V0, S5);
                c.g[A2][0] = o406_lw(c, S2, 4);
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[A0][0] = o406_lw(c, V0, 0x1c);
                call_ptr(c, 0x003e_6448, 0x0030_d9f4)?;
                c.g[V0][0] = ((c.g[S3][0] as i64) < 4) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[S0][0] = o406_lw(c, S2, 8);
                if event_due(c) {
                    deliver_event_here(c, 0x0030_d990);
                }
            }
            c.cycles += 79;
            c.g[V1][0] = o406_lw(c, S2, 8);
            c.g[A1][0] = 0x58;
            c.g[V0][0] = o406_lw(c, S2, 0xc);
            c.g[A2][0] = 1;
            c.mult(0, V1, V1, A1);
            let a = (c.g[S4][0] as u32).wrapping_add(0x52);
            c.g[A0][0] = c.lb(a);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            o406_sw(c, V1, 0, A0);
            c.g[V0][0] = o406_lw(c, S2, 8);
            c.g[V1][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V0, V0, A1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
            o406_sw(c, V0, 0x18, A2);
            c.g[V1][0] = o406_lw(c, S2, 8);
            c.g[V0][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V1, V1, A1);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S6][0] as u32));
            c.g[V0][0] = c.lw(frame.at(c, 0));
            c.g[A0][0] = o406_lw(c, V1, 0x1c);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            o406_sw(c, A0, 0, S4);
            c.g[V0][0] = o406_lw(c, S2, 8);
            c.g[V1][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V0, V0, A1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
            c.g[V1][0] = o406_lw(c, V0, 0x1c);
            c.g[V0][0] = c.lw(frame.at(c, 0));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            o406_sw(c, V1, 4, S7);
            c.g[A0][0] = o406_lw(c, S2, 8);
            c.g[V1][0] = o406_lw(c, S2, 0xc);
            c.mult(0, A0, A0, A1);
            c.g[V0][0] = o406_lw(c, S4, 0x18);
            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            o406_sw(c, A0, 0x38, V0);
            c.g[V1][0] = o406_lw(c, S2, 8);
            c.g[A0][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V1, V1, A1);
            c.g[V0][0] = o406_lw(c, S4, 0x2c);
            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            o406_sw(c, V1, 0x40, V0);
            c.g[A0][0] = o406_lw(c, S2, 8);
            c.g[V1][0] = o406_lw(c, S2, 0xc);
            c.mult(0, A0, A0, A1);
            c.g[V0][0] = o406_lw(c, S4, 0x1c);
            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            o406_sw(c, A0, 0x3c, V0);
            c.g[V1][0] = o406_lw(c, S2, 8);
            c.g[V0][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V1, V1, A1);
            let a = (c.g[S4][0] as u32).wrapping_add(0x4a);
            c.g[A0][0] = c.lh(a);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            o406_sw(c, V1, 0x44, A0);
            c.g[V0][0] = o406_lw(c, S2, 8);
            c.g[A0][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V0, V0, A1);
            let a = (c.g[S4][0] as u32).wrapping_add(0x4e);
            c.g[V1][0] = c.lh(a);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            o406_sw(c, V0, 0x48, V1);
            c.g[A0][0] = o406_lw(c, S2, 8);
            c.g[V1][0] = o406_lw(c, S2, 0xc);
            c.mult(0, A0, A0, A1);
            let a = (c.g[S4][0] as u32).wrapping_add(0x50);
            c.g[V0][0] = c.lh(a);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            o406_sw(c, A0, 0x4c, V0);
            c.g[V1][0] = o406_lw(c, S2, 8);
            c.g[V0][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V1, V1, A1);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            o406_sw(c, V1, 0x50, FP);
            c.g[V0][0] = o406_lw(c, S2, 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            o406_sw(c, S2, 8, V0);
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderModel_Setup(RiderModel* self, int a1)` (0x30dbd0, DevvyDont:
/// `cAnimModel_compile`): finish a rider model after its parts were loaded.
/// Shrinks the part array to its count, sorts the parts, hands out mask bits and
/// matrix slots, allocates the matrix / index tables, registers the shape banks
/// with the graphics manager, compiles every part mesh for the four LOD channels
/// through the graphics manager's vtable (kept in `+0x40..`, `+0x44..`, `+0x80..`),
/// and builds the rest-pose matrices.
///
/// Fields: `+0` capacity, `+4` LOD count, `+8` part count, `+0xc` parts (0x58
/// each: `+0` index, `+4` bit offset, `+8` mesh offset, `+0xc` mesh slot, `+0x10`
/// mask, `+0x38` node array (0x50 each), `+0x44` node count, `+0x4c` mesh count),
/// `+0x14` max part index, `+0x18` the argument, `+0x1c`/`+0x20` index tables,
/// `+0x2c/+0x30/+0x34/+0x38` matrix arrays, `+0x60/+0x64` shape banks.
///
/// The body is a register-exact transcription of the generated control flow
/// (a goto-structured compiler output with many shared tails); calls go through
/// `call_ptr` and every unwind check is kept.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_variables,
    unused_labels,
    unused_parens,
    unused_mut
)]
pub fn rider_model_setup(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue; shrink the part array to its count when count < capacity.
        'b_30dce0: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x31c0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x31a0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x3190));
            c.g[S1][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x3180));
            c.g[S2][0] = 0x310000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x3170));
            c.g[S3][0] = 0x490000;
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x3130));
            c.g[S4][0] = 0x490000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x31b0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x3160));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x3150));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x3140));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x3120), c.g[RA][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[A1][0]);
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[FP][0] = 0x490000;
            if t {
                break 'b_30dce0;
            } else {
                'b_30dccc: {
                    c.cycles += 13;
                    c.g[A0][0] = 0x58;
                    c.g[A1][0] = 0x490000;
                    c.mult(0, 0, 3, 4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6390));
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.lo[0];
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317e30, 0x0030dc48)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = 0u64;
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if t {
                        break 'b_30dccc;
                    } else {
                        c.cycles += 2;
                        c.g[A2][0] = 0x58;
                        c.mult(0, 2, 4, 6);
                        'l_30dc60: loop {
                            c.cycles += 5;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                            'l_30dc74: loop {
                                c.cycles += 11;
                                c.g[A3][0] = c.ld((c.g[A0][0] as u32));
                                c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.g[T1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                                c.g[T2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                                c.sd((c.g[V0][0] as u32), c.g[A3][0]);
                                c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                                c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                                c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                let t = c.g[A0][0] != c.g[V1][0];
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0030dc74);
                                    }
                                    continue 'l_30dc74;
                                } else {
                                    c.cycles += 11;
                                    c.g[T4][0] = c.ld((c.g[A0][0] as u32));
                                    c.g[V1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                                    c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                                    c.sd((c.g[V0][0] as u32), c.g[T4][0]);
                                    c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                    c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.g[A1][0];
                                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.mult(0, 2, 4, 6);
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0030dc60);
                                        }
                                        continue 'l_30dc60;
                                    } else {
                                        break 'b_30dccc;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                let t = c.g[S0][0] == 0u64;
                c.sw((c.g[S1][0] as u32), c.g[V0][0]);
                if t {
                    break 'b_30dce0;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317e98, 0x0030dce0)?;
                    break 'b_30dce0;
                }
            }
        }
        // Sort the parts (qsort, RiderModel_CompareInt), size the part-index tables and
        // give every part its bit range in the 64-bit mask.
        'b_30de8c: {
            c.cycles += 56;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x2470));
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[A2][0] = 0x58;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x30f4), 0u64);
            c.g[S0][0] = 0u64;
            call_ptr(c, 0x00418ef8, 0x0030dcfc)?;
            c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x30));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = 0x58;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[T1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
            c.mult(0, 3, 3, 2);
            c.g[T2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x28));
            c.g[T4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x3104), c.g[T1][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x3108), c.g[T2][0]);
            c.g[A3][0] = 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x3114), c.g[T4][0]);
            c.g[A2][0] = 0x100;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6370));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x58));
            c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x34));
            c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2c));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            call_ptr(c, 0x00317e30, 0x0030dd54)?;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x30f8), c.g[V0][0]);
            c.g[A1][0] = 0xffffffffffff0000;
            c.g[A1][0] = c.g[A1][0] | 0xffff;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
            call_ptr(c, 0x003e6448, 0x0030dd74)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6360));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            call_ptr(c, 0x00317e30, 0x0030dd90)?;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            c.g[A1][0] = 0xffffffffffff0000;
            c.g[A0][0] = c.g[V0][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = c.g[A1][0] | 0xffff;
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
            call_ptr(c, 0x003e6448, 0x0030ddb0)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[T0][0] = 0u64;
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A3][0] = 0u64;
            if t {
                break 'b_30de8c;
            } else {
                c.cycles += 4;
                c.g[T1][0] = 0x40;
                c.g[T3][0] = 0xffffffffffffffff;
                c.g[T2][0] = 0xffffffffffffffff;
                c.g[A2][0] = 0u64;
                'l_30ddd0: loop {
                    'b_30de78: {
                        'b_30de74: {
                            c.cycles += 38;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x44));
                            c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A1][0] as u32)));
                            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub((c.g[A1][0] as u32)));
                            c.g[V1][0] = c.g[T3][0] << (c.g[V1][0] & 63);
                            c.g[V1][0] = c.g[V1][0] >> (c.g[V0][0] & 63);
                            c.g[V1][0] = c.g[V1][0] << (c.g[T4][0] & 63);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[T4][0]);
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f8));
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T4][0] as u32)));
                            c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                            c.sw((c.g[V0][0] as u32), c.g[T4][0]);
                            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x30f4), c.g[T4][0]);
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            if t {
                                break 'b_30de74;
                            } else {
                                c.cycles += 3;
                                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T0][0]);
                                let t = true;
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_30de78;
                                } else {
                                    break 'b_30de74;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T2][0]);
                        break 'b_30de78;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x58));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030ddd0);
                        }
                        continue 'l_30ddd0;
                    } else {
                        break 'b_30de8c;
                    }
                }
            }
        }
        // Mask of all used bits (+0x150), totals.
        'b_30df28: {
            c.cycles += 14;
            c.sd((c.g[S1][0] as u32).wrapping_add(0x150), 0u64);
            c.g[A3][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x3118), c.g[V1][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x310c), c.g[A1][0]);
            c.g[S2][0] = sx((c.g[A2][0] as u32) << 5);
            c.g[S3][0] = sx((c.g[A2][0] as u32) << 4);
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S4][0] = sx((c.g[A2][0] as u32) << 2);
            if t {
                break 'b_30df28;
            } else {
                c.cycles += 3;
                c.g[T0][0] = 0x1;
                c.g[A2][0] = 0u64;
                'l_30ded0: loop {
                    'b_30df00: {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] == 0u64;
                        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                        if t {
                            break 'b_30df00;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[V0][0] = c.g[T0][0] << (c.g[V0][0] & 63);
                            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            break 'b_30df00;
                        }
                    }
                    c.cycles += 10;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x150));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A0][0] = c.g[A0][0] | c.g[A1][0];
                    let t = c.g[V1][0] != 0u64;
                    c.sd((c.g[S1][0] as u32).wrapping_add(0x150), c.g[A0][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030ded0);
                        }
                        continue 'l_30ded0;
                    } else {
                        break 'b_30df28;
                    }
                }
            }
        }
        // Matrix arrays (+0x30, +0x34, +0x2c), initialised to identity.
        'b_30df64: {
            c.cycles += 13;
            c.sd((c.g[S1][0] as u32).wrapping_add(0x158), 0u64);
            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6350));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3118));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
            call_ptr(c, 0x00317e30, 0x0030df48)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310c));
            c.g[V0][0] = 0xffffffffffffffff;
            if c.g[V1][0] == c.g[V0][0] {
                c.sw((c.g[S5][0] as u32), c.g[A0][0]);
                break 'b_30df64;
            } else {
                c.cycles += 2;
                c.sw((c.g[S5][0] as u32), c.g[A0][0]);
                break 'b_30df64;
            }
        }
        'b_30df94: {
            c.cycles += 10;
            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6350));
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3118));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0030df78)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310c));
            c.g[V0][0] = 0xffffffffffffffff;
            if c.g[V1][0] == c.g[V0][0] {
                c.sw((c.g[S6][0] as u32), c.g[A0][0]);
                break 'b_30df94;
            } else {
                c.cycles += 2;
                c.sw((c.g[S6][0] as u32), c.g[A0][0]);
                break 'b_30df94;
            }
        }
        'b_30dfc4: {
            c.cycles += 11;
            c.g[T4][0] = 0x490000;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x6340));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0030dfac)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310c));
            c.g[V0][0] = 0xffffffffffffffff;
            if c.g[V1][0] == c.g[V0][0] {
                c.sw((c.g[S7][0] as u32), c.g[A0][0]);
                break 'b_30dfc4;
            } else {
                c.cycles += 1;
                c.sw((c.g[S7][0] as u32), c.g[A0][0]);
                break 'b_30dfc4;
            }
        }
        'b_30e060: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A2][0] = 0u64;
            if t {
                break 'b_30e060;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x500000;
                c.g[V1][0] = 0x500000;
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
                c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xdd0));
                'l_30dfe0: loop {
                    c.cycles += 32;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = sx((c.g[A2][0] as u32) << 6);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.lqc2(1, (c.g[T0][0] as u32));
                    c.lqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                    c.lqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                    c.sqc2(1, (c.g[V0][0] as u32));
                    c.sqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                    c.sqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                    c.sqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.lqc2(1, (c.g[T0][0] as u32));
                    c.lqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                    c.lqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                    c.sqc2(1, (c.g[A0][0] as u32));
                    c.sqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.sqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 5);
                    c.lq(4, (c.g[A3][0] as u32));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                    c.sq(4, (c.g[V1][0] as u32));
                    c.g[A1][0] = ((c.g[A2][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    c.lq(2, (c.g[A3][0] as u32).wrapping_add(0x10));
                    let t = c.g[A1][0] != 0u64;
                    c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030dfe0);
                        }
                        continue 'l_30dfe0;
                    } else {
                        break 'b_30e060;
                    }
                }
            }
        }
        'b_30e0ac: {
            c.cycles += 11;
            c.g[T2][0] = 0x490000;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x6330));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0030e078)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310c));
            c.g[V0][0] = 0xffffffffffffffff;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3104));
            if t {
                break 'b_30e0ac;
            } else {
                c.cycles += 1;
                'l_30e090: loop {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3104));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030e090);
                        }
                        continue 'l_30e090;
                    } else {
                        break 'b_30e0ac;
                    }
                }
            }
        }
        'b_30e0e4: {
            c.cycles += 12;
            c.g[V0][0] = 0x490000;
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6320));
            c.g[A2][0] = 0u64;
            c.sw((c.g[T4][0] as u32), c.g[A0][0]);
            c.g[A3][0] = 0u64;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x00317e30, 0x0030e0c8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310c));
            c.g[V0][0] = 0xffffffffffffffff;
            if c.g[V1][0] == c.g[V0][0] {
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3108));
                break 'b_30e0e4;
            } else {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3108));
                break 'b_30e0e4;
            }
        }
        'b_30e1a8: {
            c.cycles += 12;
            c.g[A2][0] = 0x490000;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6310));
            c.g[A3][0] = 0u64;
            c.sw((c.g[V1][0] as u32), c.g[A0][0]);
            c.g[A2][0] = 0u64;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x00317e30, 0x0030e100)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[T1][0] = 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
            let t = (c.g[V1][0] as i64) <= 0;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_30e1a8;
            } else {
                c.cycles += 3;
                c.g[T6][0] = 0x58;
                c.g[T5][0] = 0x58;
                c.mult(0, 2, 4, 14);
                'l_30e120: loop {
                    'b_30e194: {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[T0][0] = 0u64;
                        c.g[T4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[T2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = (c.g[T2][0] as i64) <= 0;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x38));
                        if t {
                            break 'b_30e194;
                        } else {
                            c.cycles += 3;
                            c.mult(0, 11, 4, 13);
                            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
                            'l_30e148: loop {
                                c.cycles += 19;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x3));
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[V1][0] =
                                    sx((c.g[T3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                c.g[A1][0] = ((c.g[T0][0] as i64) < (c.g[T2][0] as i64)) as u64;
                                c.sw((c.g[V0][0] as u32), c.g[T1][0]);
                                c.g[V1][0] = c.lhu((c.g[A3][0] as u32));
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x50));
                                c.g[A0][0] = c.g[V1][0] & 0x1;
                                if c.g[A0][0] != 0 {
                                    c.g[T1][0] = c.g[A2][0];
                                }
                                c.g[V1][0] = c.g[V1][0] & 0x2;
                                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x3));
                                let t = c.g[A1][0] != 0u64;
                                if c.g[V1][0] != 0 {
                                    c.g[T1][0] = c.g[V0][0];
                                }
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0030e148);
                                    }
                                    continue 'l_30e148;
                                } else {
                                    break 'b_30e194;
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.g[T4][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.mult(0, 2, 4, 14);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030e120);
                        }
                        continue 'l_30e120;
                    } else {
                        break 'b_30e1a8;
                    }
                }
            }
        }
        // Per-part-mesh tables (+0x3c).
        'b_30e1c8: {
            c.cycles += 2;
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x490000;
            if t {
                break 'b_30e1c8;
            } else {
                c.cycles += 6;
                c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x62f8));
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317e30, 0x0030e1c4)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                break 'b_30e1c8;
            }
        }
        // Register the shape banks and look up their textures (PS2GraphicsMan).
        'b_30e260: {
            'b_30e25c: {
                c.cycles += 3;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                let t = (c.g[A1][0] as i64) <= 0;
                c.g[S6][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                if t {
                    break 'b_30e25c;
                } else {
                    c.cycles += 7;
                    c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x64));
                    c.g[A0][0] = c.g[S6][0];
                    c.g[S5][0] = c.g[A2][0];
                    call_ptr(c, 0x00369890, 0x0030e1e4)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30f0), 0u64);
                        break 'b_30e260;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[V1][0] = 0u64;
                        if t {
                            break 'b_30e25c;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                            'l_30e200: loop {
                                'b_30e248: {
                                    c.cycles += 6;
                                    c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] =
                                        sx((c.g[S5][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                    c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                    let t = (c.g[S2][0] as i64) <= 0;
                                    c.g[S0][0] = 0u64;
                                    if t {
                                        break 'b_30e248;
                                    } else {
                                        c.cycles += 2;
                                        c.g[S3][0] = c.g[A0][0];
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32));
                                        'l_30e220: loop {
                                            c.cycles += 10;
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[A2][0] = c.g[SP][0];
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            call_ptr(c, 0x003b3d00, 0x0030e230)?;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[SP][0];
                                            call_ptr(c, 0x003696c8, 0x0030e23c)?;
                                            c.g[V1][0] =
                                                ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                            if c.g[V1][0] != 0u64 {
                                                c.g[A0][0] = c.lw((c.g[S3][0] as u32));
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0030e220);
                                                }
                                                continue 'l_30e220;
                                            } else {
                                                break 'b_30e248;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                                c.g[V1][0] = c.g[S4][0];
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0030e200);
                                    }
                                    continue 'l_30e200;
                                } else {
                                    break 'b_30e25c;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x30f0), 0u64);
            break 'b_30e260;
        }
        c.cycles += 2;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30fc), 0u64);
        c.g[A0][0] = c.g[S6][0];
        // For each of the 4 LOD channels: compile every part mesh through the graphics
        // manager's vtable and keep the result.
        'l_30e268: loop {
            'b_30e424: {
                c.cycles += 8;
                c.g[S0][0] = 0u64;
                call_ptr(c, 0x003698a0, 0x0030e270)?;
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30fc));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x3110), c.g[A3][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30fc));
                if t {
                    break 'b_30e424;
                } else {
                    c.cycles += 4;
                    c.g[T0][0] = sx((c.g[T0][0] as u32) << 3);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x3100), c.g[T0][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    'l_30e298: loop {
                        'b_30e414: {
                            'b_30e410: {
                                'b_30e3a4: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[S3][0] = 0u64;
                                    if t {
                                        break 'b_30e3a4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_30e410;
                                        } else {
                                            c.cycles += 7;
                                            c.g[T1][0] = 0x58;
                                            c.g[T2][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x30fc));
                                            c.mult(0, 21, 16, 9);
                                            c.g[FP][0] = 0xc;
                                            c.g[S4][0] = sx((c.g[T2][0] as u32) << 3);
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                            'l_30e2c8: loop {
                                                'b_30e388: {
                                                    c.cycles += 10;
                                                    c.g[A0][0] = sx((c.g[S3][0] as u32) << 4);
                                                    c.g[T1][0] = c.g[A0][0];
                                                    c.g[A3][0] = sx((c.g[S5][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[T0][0] = c.g[V1][0];
                                                    c.g[V0][0] = sx((c.g[A3][0] as u32)
                                                        .wrapping_add((c.g[S4][0] as u32)));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                    c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[A1][0] = c.lw((c.g[A0][0] as u32));
                                                    let t = c.g[A1][0] == 0u64;
                                                    c.g[A2][0] =
                                                        c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    if t {
                                                        break 'b_30e388;
                                                    } else {
                                                        'b_30e30c: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xc));
                                                            let t = (c.g[V0][0] as i64) < 0;
                                                            c.g[T2][0] = 0u64;
                                                            if t {
                                                                break 'b_30e30c;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A3][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x3c));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.g[T2][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                break 'b_30e30c;
                                                            }
                                                        }
                                                        c.cycles += 18;
                                                        c.g[V0][0] = sx((c.g[S5][0] as u32)
                                                            .wrapping_add((c.g[T0][0] as u32)));
                                                        c.g[A3][0] = c.lw((c.g[S6][0] as u32)
                                                            .wrapping_add(0x10d8));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[S4][0] as u32)));
                                                        c.g[S2][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1c));
                                                        c.g[T0][0] = sx((c.g[SP][0] as u32)
                                                            .wrapping_add(0x30f0));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[A3][0] as u32)
                                                                .wrapping_add(0x368));
                                                        c.g[T3][0] = c
                                                            .lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x36c));
                                                        c.g[S0][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[T1][0] as u32)));
                                                        c.g[A3][0] = c.lw((c.g[SP][0] as u32)
                                                            .wrapping_add(0x30f8));
                                                        c.g[A0][0] = sx((c.g[S6][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        let t = c.g[T3][0] as u32;
                                                        c.g[T1][0] = c.g[S2][0];
                                                        call_ptr(c, t, 0x0030e340)?;
                                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                            .wrapping_add(0x30f0));
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x8),
                                                            c.g[V0][0],
                                                        );
                                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                            .wrapping_add(0x30f0));
                                                        let t = c.g[A0][0] == 0u64;
                                                        c.g[T4][0] = 0x490000;
                                                        if t {
                                                            break 'b_30e388;
                                                        } else {
                                                            c.cycles += 13;
                                                            c.mult(0, 4, 4, 30);
                                                            c.g[A1][0] = sx((c.g[T4][0] as u32)
                                                                .wrapping_sub(0x62e8));
                                                            c.g[A2][0] = 0x20000000;
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317e30, 0x0030e368)?;
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A1][0] = c.g[S2][0];
                                                            c.g[A2][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30f0));
                                                            c.mult(0, 0, 6, 30);
                                                            c.g[A2][0] = c.lo[0];
                                                            c.g[A0][0] = c.g[V0][0];
                                                            call_ptr(c, 0x003e6574, 0x0030e384)?;
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30f0),
                                                                0u64,
                                                            );
                                                            break 'b_30e388;
                                                        }
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                                c.g[S3][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                                c.g[V0][0] = ((c.g[S3][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                if c.g[V0][0] != 0u64 {
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0030e2c8);
                                                    }
                                                    continue 'l_30e2c8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    if t {
                                                        break 'b_30e414;
                                                    } else {
                                                        break 'b_30e3a4;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 11;
                                c.g[V0][0] = 0x58;
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3100));
                                c.mult(0, 4, 16, 2);
                                c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V1][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                let t = c.g[A1][0] == 0u64;
                                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_30e410;
                                } else {
                                    'b_30e3ec: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[T2][0] = 0u64;
                                        if t {
                                            break 'b_30e3ec;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[T2][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            break 'b_30e3ec;
                                        }
                                    }
                                    c.cycles += 9;
                                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
                                    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30f0));
                                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f8));
                                    c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x368));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x36c));
                                    c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x0030e410)?;
                                    break 'b_30e410;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            break 'b_30e414;
                        }
                        c.cycles += 4;
                        c.g[S0][0] = c.g[S7][0];
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            if event_due(c) {
                                deliver_event_here(c, 0x0030e298);
                            }
                            continue 'l_30e298;
                        } else {
                            break 'b_30e424;
                        }
                    }
                }
            }
            'b_30e494: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3110));
                if t {
                    break 'b_30e494;
                } else {
                    'b_30e490: {
                        c.cycles += 6;
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30fc));
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f0));
                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        let t = c.g[A0][0] == 0u64;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x40), c.g[A0][0]);
                        if t {
                            break 'b_30e490;
                        } else {
                            c.cycles += 18;
                            c.g[A3][0] = 0xc;
                            c.g[T0][0] = 0x490000;
                            c.mult(0, 4, 4, 7);
                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x44));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x62e8));
                            c.g[A2][0] = 0x20000000;
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317e30, 0x0030e46c)?;
                            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f0));
                            c.g[A0][0] = c.g[V0][0];
                            c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                            c.g[V0][0] = 0xc;
                            c.mult(0, 0, 6, 2);
                            c.g[A2][0] = c.lo[0];
                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                            call_ptr(c, 0x003e6574, 0x0030e48c)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x30f0), 0u64);
                            break 'b_30e490;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3110));
                    break 'b_30e494;
                }
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S6][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x30fc), c.g[V1][0]);
            call_ptr(c, 0x003698a0, 0x0030e4a0)?;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30fc));
            c.g[V0][0] = ((c.g[A1][0] as i64) < 4i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S6][0];
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0030e268);
                }
                continue 'l_30e268;
            } else {
                'b_30e4c8: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f8));
                    if c.g[A2][0] == 0u64 {
                        c.g[A3][0] = 0x490000;
                        break 'b_30e4c8;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[A2][0];
                        call_ptr(c, 0x00317e98, 0x0030e4c4)?;
                        c.g[A3][0] = 0x490000;
                        break 'b_30e4c8;
                    }
                }
                'b_30e50c: {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3118));
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x62d0));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317e30, 0x0030e4dc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310c));
                    c.g[V0][0] = 0xffffffffffffffff;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3114));
                    if t {
                        break 'b_30e50c;
                    } else {
                        'l_30e4f0: loop {
                            c.cycles += 7;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3114));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0030e4f0);
                                }
                                continue 'l_30e4f0;
                            } else {
                                break 'b_30e50c;
                            }
                        }
                    }
                }
                // Compose the parts' rest matrices with their parents'.
                'b_30e748: {
                    c.cycles += 4;
                    c.sw((c.g[T0][0] as u32), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = 0u64;
                    if t {
                        break 'b_30e748;
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = 0x58;
                        c.g[T7][0] = 0x58;
                        c.mult(0, 3, 4, 16);
                        'l_30e528: loop {
                            'b_30e734: {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A3][0] = 0u64;
                                c.g[T6][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                                let t = (c.g[V0][0] as i64) <= 0;
                                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x38));
                                if t {
                                    break 'b_30e734;
                                } else {
                                    c.cycles += 7;
                                    c.mult(0, 9, 4, 15);
                                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                                    c.g[T5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3010));
                                    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3030));
                                    c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3020));
                                    c.g[T3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30b0));
                                    c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3070));
                                    'l_30e560: loop {
                                        'b_30e71c: {
                                            'b_30e6dc: {
                                                c.cycles += 40;
                                                c.lq(2, (c.g[A2][0] as u32).wrapping_sub(0x10));
                                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x3010));
                                                c.lq(3, (c.g[A2][0] as u32));
                                                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x3020));
                                                c.lqc2(4, (c.g[T4][0] as u32));
                                                c.lqc2(3, (c.g[T5][0] as u32));
                                                c.vu0().add(0b1110, 1, 0, Src::Bc(0, W));
                                                c.vu0().add(0b1110, 5, 4, Src::Vec(4));
                                                c.vu0().sub(0b0001, 10, 10, Src::Vec(10));
                                                c.vu0().sub(0b0001, 11, 11, Src::Vec(11));
                                                c.vu0().sub(0b0001, 12, 12, Src::Vec(12));
                                                c.vu0().mul(0b1110, 6, 5, Src::Vec(4));
                                                c.vu0().mul(0b1110, 7, 5, Src::Bc(4, W));
                                                c.vu0().opmula(5, 4);
                                                c.vu0().madd(0b1110, 8, 0, Src::Vec(0));
                                                c.vu0().suba(0b1000, 1, Src::Bc(6, Y));
                                                c.vu0().msub(0b1000, 10, 1, Src::Bc(6, Z));
                                                c.vu0().suba(0b0100, 1, Src::Bc(6, Z));
                                                c.vu0().msub(0b0100, 11, 1, Src::Bc(6, X));
                                                c.vu0().suba(0b0010, 1, Src::Bc(6, X));
                                                c.vu0().msub(0b0010, 12, 1, Src::Bc(6, Y));
                                                c.vu0().adda(0b0100, 0, Src::Bc(8, Z));
                                                c.vu0().madd(0b0100, 10, 1, Src::Bc(7, Z));
                                                c.vu0().adda(0b0010, 0, Src::Bc(8, X));
                                                c.vu0().madd(0b0010, 11, 1, Src::Bc(7, X));
                                                c.vu0().adda(0b0100, 0, Src::Bc(8, X));
                                                c.vu0().msub(0b0100, 12, 1, Src::Bc(7, X));
                                                c.vu0().adda(0b0010, 0, Src::Bc(8, Y));
                                                c.vu0().msub(0b0010, 10, 1, Src::Bc(7, Y));
                                                c.vu0().adda(0b1000, 0, Src::Bc(8, Z));
                                                c.vu0().msub(0b1000, 11, 1, Src::Bc(7, Z));
                                                c.vu0().adda(0b1000, 0, Src::Bc(8, Y));
                                                c.vu0().madd(0b1000, 12, 1, Src::Bc(7, Y));
                                                c.sqc2(3, (c.g[T0][0] as u32).wrapping_add(0x30));
                                                c.sqc2(10, (c.g[T0][0] as u32));
                                                c.sqc2(11, (c.g[T0][0] as u32).wrapping_add(0x10));
                                                c.sqc2(12, (c.g[T0][0] as u32).wrapping_add(0x20));
                                                c.g[A0][0] =
                                                    c.lh((c.g[A2][0] as u32).wrapping_sub(0x1e));
                                                let t = (c.g[A0][0] as i64) < 0;
                                                c.g[A0][0] = sx((c.g[A0][0] as u32) << 6);
                                                if t {
                                                    break 'b_30e6dc;
                                                } else {
                                                    c.cycles += 55;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                    c.g[A1][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                                    c.g[V1][0] = sx((c.g[T1][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[V0][0] =
                                                        c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                                    c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_add((c.g[A1][0] as u32)));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[A3][0] as u32)));
                                                    c.lqc2(4, (c.g[A0][0] as u32));
                                                    c.lqc2(
                                                        5,
                                                        (c.g[A0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.lqc2(
                                                        6,
                                                        (c.g[A0][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.lqc2(
                                                        7,
                                                        (c.g[A0][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.lqc2(8, (c.g[T0][0] as u32));
                                                    c.lqc2(
                                                        9,
                                                        (c.g[T0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.lqc2(
                                                        10,
                                                        (c.g[T0][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.lqc2(
                                                        11,
                                                        (c.g[T0][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                                                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                                                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                                                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                                                    c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                                                    c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                                                    c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                                                    c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                                                    c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                                                    c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                                                    c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                                                    c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                                                    c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                                                    c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                                                    c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                                                    c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                                                    c.sqc2(12, (c.g[T3][0] as u32));
                                                    c.sqc2(
                                                        13,
                                                        (c.g[T3][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sqc2(
                                                        14,
                                                        (c.g[T3][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.sqc2(
                                                        15,
                                                        (c.g[T3][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.lqc2(1, (c.g[T3][0] as u32));
                                                    c.lqc2(
                                                        2,
                                                        (c.g[T3][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.lqc2(
                                                        3,
                                                        (c.g[T3][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.lqc2(
                                                        4,
                                                        (c.g[T3][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.sqc2(1, (c.g[T2][0] as u32));
                                                    c.sqc2(
                                                        2,
                                                        (c.g[T2][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sqc2(
                                                        3,
                                                        (c.g[T2][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.sqc2(
                                                        4,
                                                        (c.g[T2][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.lqc2(1, (c.g[T2][0] as u32));
                                                    c.lqc2(
                                                        2,
                                                        (c.g[T2][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.lqc2(
                                                        3,
                                                        (c.g[T2][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.lqc2(
                                                        4,
                                                        (c.g[T2][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.sqc2(1, (c.g[V0][0] as u32));
                                                    c.sqc2(
                                                        2,
                                                        (c.g[V0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sqc2(
                                                        3,
                                                        (c.g[V0][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.sqc2(
                                                        4,
                                                        (c.g[V0][0] as u32).wrapping_add(0x30),
                                                    );
                                                    let t = true;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                    if t {
                                                        break 'b_30e71c;
                                                    } else {
                                                        break 'b_30e6dc;
                                                    }
                                                }
                                            }
                                            c.cycles += 16;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                            c.g[V0][0] = sx((c.g[T1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A3][0] as u32)));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            c.lqc2(1, (c.g[T0][0] as u32));
                                            c.lqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                                            c.lqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                                            c.lqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                                            c.sqc2(1, (c.g[V1][0] as u32));
                                            c.sqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                                            c.sqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                                            c.sqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                            break 'b_30e71c;
                                        }
                                        c.cycles += 6;
                                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                        c.g[V1][0] =
                                            sx((c.g[T1][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                                        c.g[V0][0] =
                                            ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x50));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0030e560);
                                            }
                                            continue 'l_30e560;
                                        } else {
                                            break 'b_30e734;
                                        }
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.g[T6][0];
                            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.mult(0, 3, 4, 16);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0030e528);
                                }
                                continue 'l_30e528;
                            } else {
                                break 'b_30e748;
                            }
                        }
                    }
                }
                // Normalise the part matrices.
                'b_30e81c: {
                    c.cycles += 3;
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                    let t = (c.g[T1][0] as i64) <= 0;
                    c.g[A2][0] = 0u64;
                    if t {
                        break 'b_30e81c;
                    } else {
                        c.cycles += 5;
                        c.g[AT][0] = 0xffffffffbf800000;
                        c.mtc1(1, 4);
                        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3010));
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 5);
                        'l_30e768: loop {
                            c.cycles += 45;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[V1][0] = sx((c.g[A2][0] as u32) << 6);
                            c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = ((c.g[A2][0] as i64) < (c.g[T2][0] as i64)) as u64;
                            c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x20));
                            c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                            c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x24));
                            c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x30));
                            c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x24));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x3020));
                            c.mfc1(4, 4);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x3020));
                            c.qmtc2(4, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x3030));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x3030));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x3010));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x301c), 0u64);
                            c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x3010));
                            c.lqc2(4, (c.g[V1][0] as u32));
                            c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.lqc2(6, (c.g[V1][0] as u32).wrapping_add(0x20));
                            c.lqc2(7, (c.g[V1][0] as u32).wrapping_add(0x30));
                            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                            c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x3030));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x3030));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x3010));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x3020));
                            c.swc1(5, (c.g[A3][0] as u32).wrapping_add(0xc));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x3010));
                            let t = c.g[A1][0] != 0u64;
                            c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x30));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0030e768);
                                }
                                continue 'l_30e768;
                            } else {
                                break 'b_30e81c;
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[S5][0] = 0u64;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[A0][0] = 0u64;
                // Per bank: compile the shadow/vertex data through the vtable (+0x374).
                'l_30e830: loop {
                    'b_30e91c: {
                        c.cycles += 2;
                        let t = (c.g[A1][0] as i64) <= 0;
                        c.g[FP][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_30e91c;
                        } else {
                            c.cycles += 2;
                            c.g[S7][0] = 0x58;
                            'l_30e840: loop {
                                'b_30e908: {
                                    'b_30e904: {
                                        'b_30e8b8: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[S0][0] = 0u64;
                                            if t {
                                                break 'b_30e8b8;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                                let t = (c.g[V0][0] as i64) <= 0;
                                                c.g[S4][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_30e904;
                                                } else {
                                                    c.cycles += 3;
                                                    c.mult(0, 19, 4, 23);
                                                    c.g[S2][0] = sx((c.g[S5][0] as u32) << 3);
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                    'l_30e860: loop {
                                                        'b_30e89c: {
                                                            c.cycles += 8;
                                                            c.g[A0][0] =
                                                                sx((c.g[S0][0] as u32) << 4);
                                                            c.g[V0][0] = sx((c.g[S3][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[S2][0] as u32)));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            c.g[A2][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add((c.g[V1][0] as u32)));
                                                            c.g[A1][0] = c.lw((c.g[A2][0] as u32));
                                                            let t = c.g[A1][0] == 0u64;
                                                            c.g[T0][0] = 0u64;
                                                            if t {
                                                                break 'b_30e89c;
                                                            } else {
                                                                c.cycles += 7;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S6][0] as u32)
                                                                        .wrapping_add(0x10d8));
                                                                c.g[A3][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x370));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x374));
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[A0][0] = sx((c.g[S6][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                call_ptr(c, t, 0x0030e89c)?;
                                                                break 'b_30e89c;
                                                            }
                                                        }
                                                        c.cycles += 5;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[S0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] = ((c.g[S0][0] as i64)
                                                            < (c.g[V0][0] as i64))
                                                            as u64;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x0030e860);
                                                            }
                                                            continue 'l_30e860;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            if t {
                                                                break 'b_30e908;
                                                            } else {
                                                                break 'b_30e8b8;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 9;
                                        c.mult(0, 2, 4, 23);
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                        c.g[A2][0] = sx((c.g[S5][0] as u32) << 3);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A2][0] as u32)));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                        c.g[A1][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A1][0] == 0u64;
                                        c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_30e904;
                                        } else {
                                            c.cycles += 10;
                                            c.g[V1][0] =
                                                c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
                                            c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A2][0] as u32)));
                                            c.g[T1][0] = c.g[V0][0];
                                            c.g[A3][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x370));
                                            c.g[T0][0] = 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x374));
                                            c.g[A2][0] =
                                                c.lw((c.g[T1][0] as u32).wrapping_add(0x40));
                                            let t = c.g[V0][0] as u32;
                                            c.g[A0][0] = sx((c.g[S6][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x0030e904)?;
                                            break 'b_30e904;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_30e908;
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S4][0];
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0030e840);
                                    }
                                    continue 'l_30e840;
                                } else {
                                    break 'b_30e91c;
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[S5][0] = c.g[FP][0];
                    c.g[V0][0] = ((c.g[S5][0] as i64) < 4i64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = 0u64;
                        if event_due(c) {
                            deliver_event_here(c, 0x0030e830);
                        }
                        continue 'l_30e830;
                    } else {
                        'b_30e9b4: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x31b0));
                            if t {
                                break 'b_30e9b4;
                            } else {
                                'b_30e9b0: {
                                    c.cycles += 14;
                                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xe80));
                                    c.g[A0][0] = 0xc;
                                    c.g[A1][0] = 0x490000;
                                    c.g[A2][0] = 0u64;
                                    c.mult(0, 0, 2, 4);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x62c0));
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
                                    c.g[A0][0] = c.lo[0];
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317e30, 0x0030e95c)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x12c));
                                    c.g[A1][0] = 0u64;
                                    let t = (c.g[V1][0] as i64) <= 0;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x130), c.g[V0][0]);
                                    if t {
                                        break 'b_30e9b0;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x280));
                                        c.g[V1][0] = 0u64;
                                        'l_30e978: loop {
                                            c.cycles += 14;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x130));
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                                            c.ldr(6, (c.g[A0][0] as u32));
                                            c.g[A3][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                            c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x7));
                                            c.sdr(6, (c.g[V0][0] as u32));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x12c));
                                            c.g[V0][0] =
                                                ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0030e978);
                                                }
                                                continue 'l_30e978;
                                            } else {
                                                break 'b_30e9b0;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x31b0));
                                break 'b_30e9b4;
                            }
                        }
                        c.cycles += 11;
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x31a0));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x3190));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x3180));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x3170));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x3160));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x3150));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x3140));
                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x3130));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3120));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x31c0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderModel_GetBoneMask (0x310ce8)

/// `cAnimModel::getGraftMask` field offsets.
pub(crate) const BONE_MASK_MODEL_PARTS: u32 = 0xc;

pub(crate) const BONE_MASK_MODEL_NUM_BONES: u32 = 0x10;

pub(crate) const BONE_MASK_MODEL_MAX_PART: u32 = 0x14;

pub(crate) const BONE_MASK_MODEL_PART_INDEX: u32 = 0x1c;

/// `sAnimModelPart` (0x58 bytes): first bone, morph slot, bone-name list,
/// bone count.
pub(crate) const BONE_MASK_PART_FIRST_BONE: u32 = 0x4;

pub(crate) const BONE_MASK_PART_MORPH: u32 = 0xc;

pub(crate) const BONE_MASK_PART_NAMES: u32 = 0x38;

pub(crate) const BONE_MASK_PART_NUM_BONES: u32 = 0x44;

/// Size of one bone-name slot in a part's list.
pub(crate) const BONE_MASK_NAME_STRIDE: u32 = 0x50;

/// The "morph" token string (`str_morph`).
#[cfg(test)]
pub(crate) const BONE_MASK_STR_MORPH: u32 = 0x004a_3df0;

/// `strncmp`-like comparison (0x416b18).
pub(crate) const BONE_MASK_STRNCMP: u32 = 0x0041_6b18;

/// `u64 getGraftMask(cAnimModel* model, int part, const char* names)`
/// (0x310ce8): the bone/morph bit mask of a comma-separated list of names
/// for one part of a rider model. For each token it first checks the part's
/// morph slot (bit `morph + numBones`), else the part's bone names (bit
/// `firstBone + j` of the first bone whose full name equals the token).
/// 0 when the part index is out of range or has no table entry.
pub fn rider_model_get_bone_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xb0);
        c.g[V1][0] = c.g[A1][0];
        let model_slot = frame.at(c, 0);
        c.sw(model_slot, c.g[A0][0]);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = oc417_lw(c, A0, BONE_MASK_MODEL_MAX_PART);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let out_of_range = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[A2][0];
        'body: {
            let mut valid = !out_of_range;
            if valid {
                c.cycles += 4;
                c.g[V0][0] = oc417_lw(c, SP, 0);
                c.g[A1][0] = oc417_lw(c, V0, BONE_MASK_MODEL_PART_INDEX);
                let no_table = c.g[A1][0] == 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                if no_table {
                    valid = false;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A1][0]);
                    c.g[A1][0] = oc417_lw(c, V0, 0);
                    let found = (c.g[A1][0] as i64) >= 0;
                    c.g[S2][0] = c.g[A0][0];
                    valid = found;
                }
            }
            if !valid {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'body;
            }
            // The part record and the mask accumulator.
            c.cycles += 9;
            c.g[V0][0] = 0x58;
            c.g[A0][0] = oc417_lw(c, SP, 0);
            c.mult(0, V0, A1, V0);
            c.g[S7][0] = 0;
            c.g[A2][0] = oc417_lbu(c, S2, 0);
            c.g[V1][0] = oc417_lw(c, A0, BONE_MASK_MODEL_PARTS);
            c.g[A0][0] = oc417_lb(c, S2, 0);
            c.g[S6][0] = oc417_add(c.g[V1][0], c.g[V0][0]);
            if c.g[A0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = c.g[S7][0];
                break 'body;
            }
            'tokens: loop {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[A2][0] as u32) << 24);
                c.g[S3][0] = c.g[S2][0];
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[A0][0] = oc417_lw(c, S6, BONE_MASK_PART_MORPH);
                // Find the end of the token (NUL or ',').
                'scan: {
                    if c.g[V0][0] == 0 {
                        break 'scan;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = 0x2c;
                    if c.g[V0][0] == c.g[A1][0] {
                        break 'scan;
                    }
                    c.cycles += 2;
                    c.g[V1][0] = 0x2c;
                    c.g[S3][0] = oc417_add(c.g[S3][0], 1);
                    loop {
                        c.cycles += 3;
                        c.g[V0][0] = oc417_lb(c, S3, 0);
                        if c.g[V0][0] == 0 {
                            break 'scan;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[V1][0] {
                            c.g[S3][0] = oc417_add(c.g[S3][0], 1);
                            if event_due(c) {
                                deliver_event_here(c, 0x0031_0da0);
                            }
                            continue;
                        }
                        break 'scan;
                    }
                }
                c.cycles += 2;
                let no_morph = (c.g[A0][0] as i64) < 0;
                c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                'token_done: {
                    if !no_morph {
                        // `strncmp(token, "morph", len) == 0`: the morph bit.
                        c.cycles += 7;
                        c.g[V0][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = oc417_add(c.g[V0][0], 0x3df0);
                        c.g[A2][0] = c.g[S5][0];
                        call_ptr(c, BONE_MASK_STRNCMP, 0x0031_0dd0)?;
                        if c.g[V0][0] == 0 {
                            c.cycles += 8;
                            c.g[A1][0] = oc417_lw(c, SP, 0);
                            c.g[A0][0] = 1;
                            c.g[V0][0] = oc417_lw(c, S6, BONE_MASK_PART_MORPH);
                            c.g[V1][0] = oc417_lw(c, A1, BONE_MASK_MODEL_NUM_BONES);
                            c.g[V0][0] = oc417_add(c.g[V0][0], c.g[V1][0]);
                            c.g[A0][0] <<= c.g[V0][0] & 63;
                            c.g[S7][0] |= c.g[A0][0];
                            break 'token_done;
                        }
                        c.g[V0][0] = oc417_lw(c, S6, BONE_MASK_PART_NAMES);
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = oc417_lw(c, S6, BONE_MASK_PART_NAMES);
                    }
                    // Search the part's bone names for the token.
                    c.cycles += 5;
                    c.g[S4][0] = 0;
                    c.g[FP][0] = 1;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[S0][0] = oc417_add(c.g[V0][0], c.g[S5][0]);
                    loop {
                        c.cycles += 4;
                        c.g[V0][0] = oc417_lw(c, S6, BONE_MASK_PART_NUM_BONES);
                        c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        c.g[A0][0] = c.g[S1][0];
                        if c.g[V0][0] == 0 {
                            break 'token_done;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S5][0];
                        call_ptr(c, BONE_MASK_STRNCMP, 0x0031_0e34)?;
                        let next = if c.g[V0][0] != 0 {
                            c.g[S0][0] = oc417_add(c.g[S0][0], BONE_MASK_NAME_STRIDE as u64);
                            true
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = oc417_lb(c, S0, 0);
                            if c.g[V0][0] != 0 {
                                c.g[S0][0] = oc417_add(c.g[S0][0], BONE_MASK_NAME_STRIDE as u64);
                                true
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = oc417_lw(c, S6, BONE_MASK_PART_FIRST_BONE);
                                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[S4][0]);
                                c.g[V0][0] = c.g[FP][0] << (c.g[V0][0] & 63);
                                c.g[S7][0] |= c.g[V0][0];
                                false
                            }
                        };
                        if !next {
                            break 'token_done;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_0e10);
                        }
                        c.cycles += 2;
                        c.g[S1][0] = oc417_add(c.g[S1][0], BONE_MASK_NAME_STRIDE as u64);
                        c.g[S4][0] = oc417_add(c.g[S4][0], 1);
                    }
                }
                // Next token: skip the ',' (if that is what ended this one).
                c.cycles += 7;
                c.g[V0][0] = oc417_lb(c, S3, 0);
                c.g[V0][0] ^= 0x2c;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
                c.g[S2][0] = oc417_add(c.g[S3][0], c.g[V0][0]);
                c.g[V1][0] = oc417_lb(c, S2, 0);
                let more = c.g[V1][0] != 0;
                c.g[A2][0] = oc417_lbu(c, S2, 0);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_0d78);
                    }
                    continue 'tokens;
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S7][0];
                break 'body;
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimDataBase_AllocChanCache (0x310f18)

/// `operator new(size, name, flags, extra)` (0x317e30).
pub(crate) const CHAN_CACHE_NEW_NAMED: Callee = Callee {
    addr: 0x0031_7e30,
    f: ee::s10::operator_new_named,
};

/// `tCmpChanCache* gGenCacheBucket(int partID, int numChan)` (0x310f18):
/// build the channel cache of animation part `partID`: a ring of 20 nodes
/// (0x24 bytes each, one block named "AnimChanCache", stored at
/// `0x4fe7a0[partID]`), each node owning a separately allocated array of
/// `numChan` 20-byte elements (`+4`), `used` (`+0`) 0, and `next`/`prev`
/// (`+0x20`/`+0x1c`) pointing at its neighbours. The first node's `prev`
/// and the last node's `next` close the ring (`+0x2ac`, `+0x2cc`), and the
/// ring head also goes to `0x4fe860[partID]`.
pub fn anim_data_base_alloc_chan_cache_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x50_0000;
        frame.save(c, S1, 0x70);
        c.g[A2][0] = 0x2000_0000;
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        frame.save(c, S7, 0x10);
        c.g[S0][0] = sx((c.g[A0][0] as u32) << 2);
        frame.save(c, S5, 0x30);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S6, 0x20);
        c.g[S7][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1860));
        c.g[S5][0] = c.g[S0][0];
        c.g[S6][0] = c.g[A1][0];
        c.g[S0][0] = oc417_add(c.g[S0][0], c.g[V0][0]);
        frame.save(c, S2, 0x60);
        c.g[A0][0] = 0x2d0;
        frame.save(c, S3, 0x50);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x62b0));
        frame.save(c, S4, 0x40);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[S0][0];
        CHAN_CACHE_NEW_NAMED.call(c, 0x0031_0f7c)?;
        c.g[S3][0] = 0x13;
        c.g[V1][0] = 0x14;
        oc417_sw(c, S0, 0, V0);
        c.mult(0, S4, S1, V1);
        c.g[S1][0] = 0;
        loop {
            c.cycles += 22;
            c.g[T0][0] = oc417_lw(c, S2, 0);
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x62b0));
            c.g[A2][0] = 0x2000_0000;
            c.g[T0][0] = oc417_add(c.g[S1][0], c.g[T0][0]);
            c.g[A3][0] = 0;
            c.g[V0][0] = oc417_add(c.g[T0][0], 0x24);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            oc417_sw(c, T0, 0x20, V0);
            c.g[V1][0] = oc417_lw(c, S2, 0);
            c.g[V1][0] = oc417_add(c.g[S1][0], c.g[V1][0]);
            c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x24));
            oc417_sw(c, V1, 0x1c, T0);
            c.g[V0][0] = oc417_lw(c, S2, 0);
            c.g[V0][0] = oc417_add(c.g[S1][0], c.g[V0][0]);
            oc417_sw0(c, V0, 0);
            c.g[S0][0] = oc417_lw(c, S2, 0);
            c.g[S0][0] = oc417_add(c.g[S1][0], c.g[S0][0]);
            CHAN_CACHE_NEW_NAMED.call(c, 0x0031_0fdc)?;
            c.g[S1][0] = oc417_add(c.g[S1][0], 0x24);
            let more = (c.g[S3][0] as i64) >= 0;
            oc417_sw(c, S0, 4, V0);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_0f90);
                }
                continue;
            }
            break;
        }
        c.cycles += 23;
        c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x1860));
        c.g[A2][0] = 0x50_0000;
        c.g[A1][0] = oc417_add(c.g[S5][0], c.g[A1][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x17a0));
        c.g[A0][0] = oc417_lw(c, A1, 0);
        c.g[A2][0] = oc417_add(c.g[S5][0], c.g[A2][0]);
        frame.restore(c, S0, 0x80);
        c.g[V1][0] = oc417_add(c.g[A0][0], 0x2ac);
        frame.restore(c, S1, 0x70);
        oc417_sw(c, A0, 0x1c, V1);
        frame.restore(c, S2, 0x60);
        c.g[V0][0] = oc417_lw(c, A1, 0);
        frame.restore(c, S3, 0x50);
        oc417_sw(c, V0, 0x2cc, V0);
        frame.restore(c, S4, 0x40);
        c.g[V0][0] = oc417_lw(c, A1, 0);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        oc417_sw(c, A2, 0, V0);
        frame.close(c);
        Ok(())
    })
}

/// `void BELibrary::LoadCharDB()` (0x149bb8, `cBELibrary::loadData`): reset
/// the back-end library: the reward DB, the race DB (6 ids), the six rider
/// slots, load `data/be/charDB.dbl` into the character table, build the ten
/// rows of the equipment value table (+5 of the base table), initialise the
/// three profile libraries and fill the grab-point score table with its
/// constants.
/// NOT installed (no `OVERRIDES` entry, no test): its real callees
/// (`RewardsFile_Load`, `EAFile_LoadFileEx`) go through the file queue and
/// `BoltFile_Acquire`, which have no override slot to fake, so a differential
/// fixture cannot be bounded.
pub fn belibrary_load_char_db(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x00149bb8);
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[A0][0] = 0x4a0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6750));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S3][0] = 0x530000;
        Callee {
            addr: 0x0014c6a0,
            f: ee::s01::Bolts_LoadDB,
        }
        .call(c, 0x00149be0)?;
        c.g[A0][0] = 0x4c0000;
        c.g[A1][0] = 0u64;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3e98));
        Callee {
            addr: 0x0015ad58,
            f: ee::s01::RewardsFile_Load,
        }
        .call(c, 0x00149bf0)?;
        c.g[S0][0] = 0x530000;
        c.g[A1][0] = 0u64;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b0));
        c.g[A2][0] = 0x4c;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00416210, 0x00149c08)?;
        c.g[T4][0] = 0u64;
        'l_149c10: loop {
            c.cycles += 7;
            c.sw(c.g[S0][0] as u32, c.g[T4][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[T4][0]);
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[T4][0] as i64) < 6i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00149c10);
                }
                continue 'l_149c10;
            } else {
                c.cycles += 9;
                c.g[S0][0] = 0x530000;
                c.g[A1][0] = 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b20));
                c.g[A2][0] = 0xa8;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00416210, 0x00149c44)?;
                c.g[A0][0] = 0x4;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa8));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                'l_149c50: loop {
                    c.cycles += 8;
                    c.sb(c.g[S0][0] as u32, 0u64);
                    c.g[V0][0] = c.g[V0][0] | 0x2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sb((c.g[S0][0] as u32).wrapping_add(0x11), c.g[A0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        if event_due(c) {
                            deliver_event_here(c, 0x00149c50);
                        }
                        continue 'l_149c50;
                    } else {
                        'b_149cac: {
                            c.cycles += 12;
                            c.g[A0][0] = 0x460000;
                            c.g[A1][0] = 0x100;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5d20));
                            call_ptr(c, 0x003e18c8, 0x00149c80)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = 0x530000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x970));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A2][0] = 0x550;
                            call_ptr(c, 0x0041605c, 0x00149c98)?;
                            let t = c.g[S0][0] == 0u64;
                            c.g[V0][0] = 0x530000;
                            if t {
                                break 'b_149cac;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00317e98, 0x00149ca8)?;
                                c.g[V0][0] = 0x530000;
                                break 'b_149cac;
                            }
                        }
                        c.cycles += 7;
                        c.g[V1][0] = 0x440000;
                        c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8d8));
                        c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf00));
                        c.g[T4][0] = 0u64;
                        c.g[T5][0] = 0xb;
                        c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0xd));
                        'l_149cc8: loop {
                            c.cycles += 35;
                            c.g[A0][0] = c.lbu(c.g[T2][0] as u32);
                            c.g[A2][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x1));
                            c.g[A3][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x2));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5));
                            c.g[T0][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x3));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5));
                            c.g[A1][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5));
                            c.g[V1][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x5));
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5));
                            c.g[V0][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x6));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5));
                            c.sb(c.g[T1][0] as u32, c.g[T4][0]);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5));
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x1), c.g[A0][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x7), c.g[V0][0]);
                            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x2), c.g[A2][0]);
                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x7));
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x3), c.g[A3][0]);
                            c.g[V0][0] = ((c.g[T4][0] as i64) < 10i64) as u64;
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x4), c.g[T0][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x5), c.g[A1][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x6), c.g[V1][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x4), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x2), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x5), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x1), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_add(0x1), c.g[T5][0]);
                            c.sb(c.g[T3][0] as u32, c.g[T5][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0xa), c.g[T5][0]);
                            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0xf));
                            let t = c.g[V0][0] != 0u64;
                            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0xf));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00149cc8);
                                }
                                continue 'l_149cc8;
                            } else {
                                c.cycles += 13;
                                c.g[A0][0] = 0x530000;
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5898));
                                c.g[A2][0] = 0x288;
                                call_ptr(c, 0x00416210, 0x00149d68)?;
                                c.g[S2][0] = 0x9b50;
                                c.g[V1][0] = 0x4a0000;
                                c.g[V0][0] = 0x10000;
                                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                                c.g[V0][0] = c.g[V0][0] | 0xd1f0;
                                c.g[S1][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A0][0] = c.g[S0][0];
                                'l_149d88: loop {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x001567b8,
                                        f: ee::s01::BE_ResetProfileCharacters,
                                    }
                                    .call(c, 0x00149d90)?;
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00149d88);
                                        }
                                        continue 'l_149d88;
                                    } else {
                                        c.cycles += 182;
                                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x600));
                                        c.sw((c.g[S3][0] as u32).wrapping_add(0x600), 0u64);
                                        c.g[A1][0] = 0xc8;
                                        c.g[V1][0] = 0x190;
                                        c.g[A3][0] = 0x12c;
                                        c.g[T1][0] = 0xfa;
                                        c.g[T0][0] = 0x15e;
                                        c.g[A2][0] = 0x1f4;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[T0][0]);
                                        c.g[A0][0] = 0xbb8;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x48), c.g[T1][0]);
                                        c.g[T2][0] = 0x3e8;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x88), c.g[A3][0]);
                                        c.g[T0][0] = 0x7d0;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), c.g[V1][0]);
                                        c.g[T3][0] = 0x9c4;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                                        c.g[T4][0] = 0xfa0;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                                        c.g[T5][0] = 0x1388;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x2c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x34), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x38), c.g[T1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x40), c.g[T1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x44), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x50), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x54), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x60), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x64), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x68), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x80), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x98), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x108), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x10c), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xa0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xa4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xa8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xac), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xb0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xb4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xb8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xbc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xcc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xdc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xe0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xe4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xe8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xec), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xf0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xf4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xf8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xfc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x100), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x104), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x148), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x14c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x158), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x15c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x168), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x16c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x150), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x154), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x160), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x164), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x170), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x174), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x178), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x17c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x188), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x18c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x198), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x19c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x180), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x184), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x190), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x194), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1a0), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1a4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1a8), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1ac), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1b8), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1bc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c8), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1cc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1b0), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1b4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c0), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d0), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x118), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x11c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x128), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x12c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1e0), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1f0), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x200), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x138), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x13c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x120), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x124), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x130), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x134), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x140), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x144), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x208), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x20c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x218), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x21c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x228), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x22c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x210), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x214), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x220), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x224), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x230), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x234), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d8), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1dc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1e8), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1ec), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1f8), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1e4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1f4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x204), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x238), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x23c), c.g[T4][0]);
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x2a4), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x280), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x284), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x240), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x244), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x248), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x24c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x250), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x254), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x258), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x25c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x260), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x264), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x268), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x26c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x270), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x274), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x278), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x27c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x288), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x28c), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x290), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x294), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x298), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x29c), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x2a0), c.g[T5][0]);
                                        let t = c.g[RA][0] as u32;
                                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.pc = t;
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

/// `void RiderModel::AddModelRef(this, int ref)` (0x30db70): append `ref` to the
/// model reference list (`count` at +0x60, entries at +0x64).
pub fn rider_model_add_model_ref(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030db70);
    c.cycles += 7;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x60));
    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x64), c.g[A1][0]);
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x60), c.g[V1][0]);
    c.pc = t;
    return;
}

/// `void AnimDataBase_FreeChanCaches(void*)` (0x311110): releases the 48
/// channel caches. Entry `i` of the table at 0x4fe7a0 (`cache`) holds a
/// pointer to 20 records of 0x24 bytes; for each record the byte at `+5` of
/// the record's first pointer is set to 0xff and the block at `+4` is freed;
/// then the table block itself is freed and the entry and its twin in the table
/// at 0x4fe860 are cleared.
pub fn anim_data_base_free_chan_caches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x50_0000;
        frame.save(c, S6, 0x30);
        c.g[V1][0] = 0x50_0000;
        frame.save(c, S7, 0x20);
        c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1860));
        frame.save(c, FP, 0x10);
        c.g[S7][0] = c.g[S6][0];
        frame.save(c, S0, 0x90);
        c.g[FP][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x17a0));
        frame.save(c, S1, 0x80);
        c.g[A0][0] = 0;
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save_ra(c, 0);
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        loop {
            c.cycles += 5;
            c.g[S3][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S6][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            let empty = c.g[V0][0] == 0;
            c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            if !empty {
                c.cycles += 5;
                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S7][0] as u32));
                c.g[S4][0] = 0xff;
                c.g[S1][0] = 0;
                c.g[S2][0] = 0x13;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                loop {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    let none = c.g[V0][0] == 0;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    if !none {
                        c.cycles += 1;
                        c.sb((c.g[V0][0] as u32).wrapping_add(5), c.g[S4][0]);
                    }
                    c.cycles += 6;
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
                    let no_block = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[V0][0];
                    if !no_block {
                        c.cycles += 2;
                        call_ptr(c, 0x0031_7e98, 0x0031_11b4)?; // EA_MemFree
                    }
                    c.cycles += 2;
                    if (c.g[S2][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_1180);
                        }
                        continue;
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    let no_table = c.g[A0][0] == 0;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    if !no_table {
                        c.cycles += 3;
                        call_ptr(c, 0x0031_7e98, 0x0031_11d4)?; // EA_MemFree
                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    }
                    c.cycles += 2;
                    c.sw(c.g[S0][0] as u32, 0);
                    c.sw(c.g[V0][0] as u32, 0);
                    break;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S5][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 48) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_1158);
                }
                continue;
            }
            break;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* AdfDataSource_ctor(AdfDataSource* self, const char* path)`
/// (0x311250): load the (decompressed) file `path` with
/// `EAFile_LoadDecompressed(path, 0)`, store the buffer at `self+0`, and hand
/// it to `AdfDataSource_SetData(self, buffer)`; returns `self`.
///
/// No `OVERRIDES` entry and no test: the file loader cannot be bounded in a
/// fixture (it reaches `EAFile_CallWithPriority`). Transcribed from the
/// generated body.
pub fn oc436_adf_data_source_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x003e_22f0, 0x0031_126c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        call_ptr(c, 0x0031_1290, 0x0031_127c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `AdfDataSource* cAdfDataSource_ctor(AdfDataSource* self, data)` (0x311220):
/// clear `self+0`, then `AdfDataSource_SetData(self, data)`; returns `self`.
pub fn oc436_cadf_data_source_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw(c.g[S0][0] as u32, 0);
        call_ptr(c, 0x0031_1290, 0x0031_1238)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `Vec4 RiderModel_GetPartPointWorld(RiderModel* model, int id, int index)`
/// (0x310458, with `out` in a0): hot spot `index` of the model part whose id
/// is `id`, as a world point. The hot spot's local point (w = 1) goes through
/// the part's bone matrix (`skel + (boneId + firstBone) * 64`) on VU0. Writes
/// `out` and returns it in v0; `t4vect_Origin` when no part has the id.
///
/// Args: a0 = out, a1 = model, a2 = id, a3 = index.
pub fn rider_model_get_part_point_world(c: &mut Cpu) {
    let frame = Frame::open(c, 0x20);
    'b_310524: {
        'b_310514: {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
            c.g[T2][0] = c.g[A0][0];
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[T0][0] = 0;
            if t {
                break 'b_310514;
            }
            c.cycles += 5;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 4);
            c.g[T1][0] = 0;
            'part_loop: loop {
                'b_310504: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    let part = (c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32);
                    c.g[A0][0] = sx(part);
                    c.g[V1][0] = c.lw(part);
                    if c.g[V1][0] != c.g[A2][0] {
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                        break 'b_310504;
                    }
                    c.cycles += 28;
                    c.g[V0][0] = c.lw(part.wrapping_add(0x3c));
                    c.g[A0][0] = c.lw(part.wrapping_add(0x4));
                    let pt = (c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32);
                    c.g[V0][0] = sx(pt);
                    c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x34));
                    c.lwc1(0, pt);
                    c.g[V1][0] = c.lw(pt.wrapping_add(0xc));
                    c.swc1(0, frame.at(c, 0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.lwc1(1, pt.wrapping_add(4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    let mat = c.g[V1][0] as u32;
                    c.swc1(1, frame.at(c, 4));
                    c.lwc1(0, pt.wrapping_add(8));
                    c.swc1(2, frame.at(c, 0xc));
                    c.swc1(0, frame.at(c, 8));
                    c.lqc2(8, frame.at(c, 0));
                    c.lqc2(4, mat);
                    c.lqc2(5, mat.wrapping_add(0x10));
                    c.lqc2(6, mat.wrapping_add(0x20));
                    c.lqc2(7, mat.wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, frame.at(c, 0x10));
                    c.lq(2, frame.at(c, 0x10));
                    let out = c.g[T2][0] as u32;
                    c.sq(2, out);
                    break 'b_310524;
                }
                c.cycles += 4;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x58));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_0480);
                    }
                    continue 'part_loop;
                }
                break 'b_310514;
            }
        }
        // No part has the id: the origin point.
        c.cycles += 4;
        c.g[V0][0] = 0x500000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
        let origin = c.g[V0][0] as u32;
        c.lq(3, origin);
        let out = c.g[T2][0] as u32;
        c.sq(3, out);
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[T2][0];
    frame.close(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::game::intersect::riderspheretree::tests::oc430_install_fakes;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;

    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_3;
    pub(crate) use crate::game::ai::rider::tests::MODEL_2 as MODEL;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear;

    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_5;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_3;

    pub(crate) use crate::game::sound::soundsys::tests::w as w_4;

    pub(crate) use crate::game::visualfx::boardwakefx::tests::put_ctx;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::uf;

    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS_3 as SEEDS_2;

    pub(crate) use crate::game::visualfx::worldlightman::tests::f;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_3;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::w as w_3;

    use ssx3_ee as ee;

    #[test]
    fn rider_model_set_color_scale_matches() {
        check(
            ssx3_ee::s10::f_0030dba0,
            rider_model_set_color_scale,
            200,
            &|c, seed| {
                let mut r = Rng(0x200a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.bus.write32(DATA + 0x140, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_model_is_part_visible_matches() {
        check_lohi(
            ssx3_ee::s10::f_0030ebc0,
            rider_model_is_part_visible,
            300,
            &|c, seed| {
                let mut r = Rng(0x200b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let idx = seed % 4;
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.bus
                    .write32(DATA + 0x14, if seed % 3 == 0 { idx } else { 0x10 });
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
            },
        );
    }

    #[test]
    fn adf_data_source_set_data_matches() {
        check(
            ssx3_ee::s10::f_00311290,
            adf_data_source_set_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x200c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                for i in 0..4 {
                    c.bus.write32(DATA + 0x100 + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn anim_data_base_ctor_matches() {
        check(
            ee::s10::f_00311048,
            anim_data_base_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1048, 0);
                c.g[A0][0] = DATA as u64;
                crate::difftest::fill(c, &mut r, DATA, 0x3fc / 4);
            },
        );
    }

    #[test]
    fn anim_data_base_dtor_matches() {
        check(
            ee::s10::f_00311088,
            anim_data_base_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1088, 0);
                alloc_setup(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    // -------------------------------- ADF data source / animation combo

    #[test]
    fn adf_data_source_dtor_matches() {
        check(
            ee::s10::f_003112c8,
            adf_data_source_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12c8, 0);
                alloc_setup(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.bus
                    .write32(DATA, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
            },
        );
    }

    #[test]
    fn anim_data_base_reset_chan_cache_table_matches() {
        check(
            ee::s10::f_003110d0,
            anim_data_base_reset_chan_cache_table,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x3110d0 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(0x4fe7a0, r.next_u32());
                c.bus.write32(0x4fe860, r.next_u32());
            },
        );
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
        let (db, anim, clips, table, data) =
            (DATA, DATA + 0x80, DATA + 0x100, DATA + 0x200, DATA + 0x2000);
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
        check(
            ee::s10::f_00311318__generated,
            anim_data_base_sample_channel,
            300,
            &sample_setup,
        );
    }

    /// A `RiderModel` for the two model renderers: bone tables, optional parts
    /// with their LOD references, and the LOD info words.
    pub(crate) fn put_render_model(c: &mut Cpu, r: &mut Rng, base: u32, seed: u32, shadow: bool) {
        let bones = if seed % 3 == 0 { 0 } else { 2 };
        let parts = if seed % 4 == 0 { 0 } else { 2 };
        c.bus.write32(base + model::NUM_BONES, bones);
        c.bus.write32(base + model::NUM_PARTS, parts);
        c.bus
            .write32(base + model::HAS_PARTS, (seed % 5 != 0) as u32);
        c.bus.write32(base + model::PARTS, base + 0x8000);
        c.bus.write32(base + model::SCALE_Y, uf(r, 0.5, 2.0));
        for lod in 0..2u32 {
            c.bus
                .write32(base + model::LOD_INFO + 8 * lod, r.next_u32());
            c.bus
                .write32(base + model::LOD_INFO + 8 * lod + 4, r.next_u32());
        }
        let a = base + 0x1000;
        let b = base + 0x2000;
        c.bus.write32(
            base + if shadow {
                model::BONE_TABLE_A_SHADOW
            } else {
                model::BONE_TABLE_A
            },
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
                c.bus
                    .write32(rec, if (seed + k) % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(rec + 4, r.next_u32());
                c.bus.write32(rec + 8, r.next_u32());
                c.bus.write32(rec + 0xc, r.next_u32());
            }
        }
    }

    #[test]
    fn rider_model_render_matches() {
        check(
            ee::s10::f_00310640__generated,
            rider_model_render,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0640, 3);
                put_ctx(c, &mut r, &[0x378, 0x318]);
                c.bus.write32(GAME_GP + 0x2858, (seed % 6 == 0) as u32);
                let m = MODEL;
                put_render_model(c, &mut r, m, seed, false);
                c.g[A0][0] = sx(m);
                c.g[A1][0] = sx(seed % 2);
                c.g[A2][0] = sx(1 + seed % 3);
                c.g[A3][0] = sx(4 + seed % 5);
            },
        );
    }

    #[test]
    fn rider_model_render_shadow_matches() {
        check(
            ee::s10::f_00310948__generated,
            rider_model_render_shadow,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0948, 3);
                put_ctx(c, &mut r, &[0x378, 0x320]);
                c.bus.write32(GAME_GP + 0x285c, (seed % 6 == 0) as u32);
                let m = MODEL;
                put_render_model(c, &mut r, m, seed, true);
                c.g[A0][0] = sx(m);
                c.g[A1][0] = sx(seed % 2);
                c.g[A2][0] = sx(1 + seed % 3);
                c.g[A3][0] = sx(4 + seed % 5);
            },
        );
    }

    #[test]
    fn rider_model_lock_part_matches() {
        check(
            ee::s10::f_0030eca0__generated,
            rider_model_lock_part,
            200,
            &|c, seed| {
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
            },
        );
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
        check(
            ssx3_ee::s10::f_00310ee0__generated,
            rider_model_load_state,
            64,
            &setup,
        );
        crate::difftest::fakes::covered(
            ssx3_ee::s10::f_00310ee0__generated,
            rider_model_load_state,
            64,
            &setup,
        );
    }

    #[test]
    fn rider_model_compare_int_matches() {
        check(
            ssx3_ee::s10::f_0030db90__generated,
            rider_model_compare_int,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30db90, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 4);
                word(c, DATA, r.next_u32());
                word(c, DATA + 4, r.next_u32());
            },
        );
    }

    pub(crate) const SEEDS_4: u32 = 60;

    #[test]
    fn rider_model_hide_all_parts_matches() {
        check_lohi(
            ee::s10::f_0030eb60__generated,
            rider_model_hide_all_parts,
            SEEDS_4,
            &|c, seed| {
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
            },
        );
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
        check(
            ssx3_ee::s10::f_0030ec18__generated,
            rider_model_show_part_variant,
            SEEDS_5,
            &|c, seed| {
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
            },
        );
    }

    // ----------------------------------------------------------- rider model

    #[test]
    fn rider_model_show_part_matches() {
        check(
            ssx3_ee::s10::f_0030e9e0__generated,
            rider_model_show_part,
            24,
            &|c, seed| {
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
                    w_3(
                        c,
                        table + 4 * i,
                        if (seed as u32 + i) % 4 == 0 {
                            0xffff_ffff
                        } else {
                            i
                        },
                    );
                }
                w_3(c, model + 0x150, r.next_u32());
                w_3(c, model + 0x154, r.next_u32());
                c.g[A0][0] = sx(model);
                c.g[A1][0] = part as u64;
                c.g[A2][0] = if seed % 3 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    (seed % 4) as u64
                };
            },
        );
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

    use crate::difftest::SP;
    use crate::game::animation::animmodel::tests::sample_setup as sample_setup_m2;

    use ssx3_ee_rt::cpu::sx;
    use ssx3_ee_rt::Cpu;

    /// RiderModel_SaveState: the stream's write is a fake (some unwind);
    /// random bone counts and array pointers.
    #[test]
    fn rider_model_save_state_matches() {
        use crate::difftest::fakes::{covered, machine, ptr, FAKE};
        use crate::difftest::DATA;
        covered(
            ee::s10::f_00310ea8__generated,
            rider_model_save_state,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_0ea8, 1);
                c.pc = 0x0031_0ea8;
                let this = DATA;
                let stream = DATA + 0x800;
                let vt = DATA + 0x900;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
                c.bus.write32(stream, vt);
                c.bus.write16(vt + 0x8, (r.next_u32() & 7) as u16);
                c.bus.write32(vt + 0xc, FAKE);
                c.bus.write32(this + RM_NUM_BONES, r.next_u32() % 8);
                c.bus.write32(this + RM_BONES, ptr(&mut r));
            },
        );
    }

    /// `RiderModel_SamplePartTransform` on top of the
    /// `AnimDataBase_SampleChannel` fixture (database at DATA, clip 1 with
    /// three chunks per channel, the anim state at DATA+0x80): 0..2
    /// channels, the part on channel 0, channel 1 or none; root and child
    /// bones, the alternate-slot/scale flag on and off, a zero position w
    /// (direction path), all axis selectors.
    #[test]
    fn rider_model_sample_part_transform_matches() {
        use sample_part_xf as k;
        const STATE: u32 = DATA + 0x80;
        const TABLE: u32 = DATA + 0x200; // chunk descriptors
        const DBS: u32 = DATA + 0x2_0000;
        const MODEL: u32 = DATA + 0x2_1000;
        const PART_BONE: u32 = DATA + 0x2_1100;
        const BONES: u32 = DATA + 0x2_1200;
        const CURVE_BASE: u32 = DATA + 0x2_1400;
        const INFOS: u32 = DATA + 0x2_1800;
        const OUT: u32 = DATA + 0x2_2000;
        check(
            ee::s10::f_0030ecd8__generated,
            rider_model_sample_part_transform,
            250,
            &|c, seed| {
                sample_setup_m2(c, seed);
                let mut r = Rng(0x30ec_d8a1 ^ (seed as u64) << 20);
                let fv = |r: &mut Rng| {
                    if r.next_u32() % 6 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(2.0)
                    }
                };
                // stack below sp: what the sampler leaves unwritten
                for i in 0..0x80 {
                    let v = fv(&mut r);
                    c.bus.write32(SP - 0x200 + 4 * i, v);
                }
                // the identity transform and the root origin
                for i in 0..8 {
                    let v = fv(&mut r);
                    c.bus.write32(k::IDENTITY_XF + 4 * i, v);
                }
                for i in 0..4 {
                    let v = fv(&mut r);
                    c.bus.write32(k::ROOT_ORIGIN + 4 * i, v);
                }
                // the database table: the state's index -> the database
                let gp_m2 = c.g[GP][0] as u32;
                c.bus.write32(gp_m2 + k::ANIM_DBS_GP, DBS);
                let db_index = c.bus.read32(STATE) & 0xff;
                c.bus.write32(DBS + 4 * db_index, DATA);
                // channels of clip 1; channel 0's part is the bank or 7
                let channels = seed % 3;
                c.bus
                    .write16(DATA + 0x100 + 0x14 + k::CLIP_CHANNELS, channels as u16);
                let bank = c.bus.read32(TABLE + 4) & 0xff;
                let first_desc = TABLE + 8; // clip 1 starts at descriptor 1
                if (seed / 3) % 2 == 1 {
                    c.bus.write8(first_desc + k::DESC_PART, 7);
                }
                let part = if seed % 7 == 0 { 9 } else { bank };
                // the state: root position and rotation, the flag
                put_vec(
                    c,
                    STATE + k::ST_ROOT_POS,
                    [fv(&mut r), fv(&mut r), fv(&mut r), fv(&mut r)],
                );
                put_vec(
                    c,
                    STATE + k::ST_ROOT_ROT,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)],
                );
                let flag = if seed % 4 < 2 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(STATE + k::ST_ALT, flag);
                // the model: part -> bone, bones, curve bases
                c.bus.write32(MODEL + k::MODEL_PART_BONE, PART_BONE);
                c.bus.write32(MODEL + k::MODEL_BONES, BONES);
                c.bus.write32(MODEL + k::MODEL_CURVE_BASE, CURVE_BASE);
                for p in 0..2 {
                    c.bus.write32(PART_BONE + 4 * p, r.next_u32() % 3);
                }
                let mut base = r.next_u32() % 3;
                for i in 0..8 {
                    c.bus.write32(CURVE_BASE + 4 * i, base);
                    base += r.next_u32() % 2;
                }
                for b in 0..3 {
                    let bone = BONES + k::BONE_SIZE * b;
                    let info = INFOS + 0x80 * b;
                    c.bus.write32(bone + k::BONE_INFO, info);
                    c.bus.write32(bone + k::BONE_SLOT, r.next_u32() % 4);
                    let parent = if (seed + b) % 3 == 0 {
                        0xffff
                    } else {
                        r.next_u32() % 5
                    };
                    c.bus.write16(info + k::INFO_PARENT, parent as u16);
                    c.bus
                        .write16(info + k::INFO_ALT_SLOT, (r.next_u32() % 4) as u16);
                    for a in 0..4 {
                        c.bus.write8(info + 0x18 + a, (r.next_u32() % 8) as u8);
                    }
                    for i in 0..4 {
                        let v = fv(&mut r);
                        c.bus.write32(info + k::INFO_SCALE + 4 * i, v);
                    }
                    if seed % 5 == 0 {
                        // zero w: the root position is a direction
                        c.bus.write32(info + k::INFO_SCALE + 0xc, 0);
                    }
                }
                c.g[A0][0] = sx(OUT);
                c.g[A1][0] = sx(MODEL);
                c.g[A2][0] = sx(STATE);
                c.g[A3][0] = part as u64;
                c.pc = 0x0030_ecd8;
            },
        );
    }

    use crate::difftest::fakes;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::mix;
    use crate::game::worldpainter::quadtree::tests::heap_init;
    use crate::game::worldpainter::quadtree::tests::pool_allocator;
    use crate::game::worldpainter::quadtree::tests::run;
    use crate::game::worldpainter::quadtree::tests::MEM_HEAP;

    pub(crate) use crate::game::input::inputparse::tests::o406_w;

    pub(crate) use crate::game::input::inputparse::tests::o406_base;

    pub(crate) use crate::game::seg::seg_1fbe38::tests::FK_NEW_NAMED;

    pub(crate) use crate::game::ai::rider::tests::oc417_force;
    pub(crate) use crate::game::seg::seg_1fbe38::tests::oc417_machine;

    // ------------------------------------------------- RiderModel_FindBone

    /// Look a bone up by name in a part's bone records (real `strcmp`):
    /// found at some index, not found, or an empty part.
    #[test]
    fn rider_model_find_bone_matches() {
        crate::difftest::check(
            ee::s10::f_00310c48__generated,
            rider_model_find_bone,
            160,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x310c48, 0, 0x0031_0c48);
                let part = (r.next_u32() % 3) as u32;
                let table = DATA + 0x200;
                let bones = DATA + 0x1000;
                o406_w(c, DATA + 0xc, table);
                o406_w(c, DATA + 0x1c, DATA + 0x100);
                o406_w(c, DATA + 0x100 + 4 * part, 1);
                let count = r.next_u32() % 5;
                o406_w(c, table + 0x58 + 0x44, count);
                o406_w(c, table + 0x58 + 0x38, bones);
                o406_w(c, table + 0x58 + 4, r.next_u32() % 100);
                for i in 0..4u32 {
                    let name = [b'b', b'o', b'n', b'e', b'0' + i as u8, 0, 0, 0];
                    for (k, b) in name.iter().enumerate() {
                        c.bus.write8(bones + 0x50 * i + k as u32, *b);
                    }
                }
                let want = r.next_u32() % 5;
                let name = [b'b', b'o', b'n', b'e', b'0' + want as u8, 0, 0, 0];
                for (k, b) in name.iter().enumerate() {
                    c.bus.write8(DATA + 0x800 + k as u32, *b);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = part as u64;
                c.g[A2][0] = (DATA + 0x800) as u64;
            },
        );
    }

    // ------------------------------------------------ RiderModel_LoadVariant

    /// Register a model part: known id (the lod record is filed) or a new
    /// part (four lod tables allocated and cleared through the fakes).
    #[test]
    fn rider_model_load_variant_matches() {
        fakes::covered(
            ee::s10::f_0030d8b8__generated,
            rider_model_load_variant,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x30d8b8, 8, 0x0030_d8b8);
                let this = DATA + 0x5_0000;
                let parts = DATA + 0x5_1000;
                let part = DATA + 0x5_8000;
                let count = r.next_u32() % 4;
                o406_w(c, this + 4, r.next_u32() % 4);
                o406_w(c, this + 8, count);
                o406_w(c, this + 0xc, parts);
                for i in 0..4u32 {
                    let e = parts + 0x58 * i;
                    o406_w(c, e, 1 + r.next_u32() % 3);
                    for lod in 0..4u32 {
                        o406_w(
                            c,
                            e + 0x1c + 8 * lod,
                            DATA + 0x6_0000 + 0x100 * (4 * i + lod),
                        );
                    }
                }
                c.bus.write8(part + 0x52, (1 + r.next_u32() % 3) as u8);
                for off in [0x18u32, 0x1c, 0x2c] {
                    o406_w(c, part + off, r.next_u32() & 0xfff);
                }
                for off in [0x4au32, 0x4e, 0x50] {
                    c.bus.write16(part + off, r.next_u32() as u16);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = part as u64;
                c.g[A3][0] = (r.next_u32() % 3) as u64;
            },
        );
    }

    pub(crate) const BASE: u32 = DATA + 0x20_0000;

    pub(crate) const MODEL_3: u32 = BASE;

    pub(crate) const PARTS: u32 = BASE + 0x1000;

    pub(crate) const NODES: u32 = BASE + 0x2000;

    pub(crate) const LOD_TABLES: u32 = BASE + 0x4000;

    pub(crate) const MESH_DATA: u32 = BASE + 0x6000;

    pub(crate) const GRAPHICS_MAN: u32 = BASE + 0x8000;

    pub(crate) const MAN_VTABLE: u32 = BASE + 0x9000;

    pub(crate) const BANKS: u32 = BASE + 0xa000;

    /// The mesh compiler behind the graphics manager's vtable slot `+0x36c`
    /// reports how many triangles it wrote through its `t0` argument.
    pub(crate) fn compile_miss(c: &mut Cpu, target: u32) {
        if target == FAKE {
            let n = (mix(c.g[A1][0] ^ c.cycles) % 4) as u32;
            let out = c.g[T0][0] as u32;
            c.bus.write32(out, n);
        }
        fake_call(c, target)
    }

    /// `RiderModel_Setup`: 1 to 4 parts (in or out of index order, so the real
    /// `qsort` has work), each with 0 to 3 nodes, 0 to 2 meshes and per-LOD mesh
    /// tables; the part array at its capacity or larger (the shrink path with a
    /// real heap block); the argument zero or not (compile the first mesh only,
    /// or every mesh of every channel); 0 to 2 shape banks. The graphics
    /// manager's compile calls are fakes that return triangle counts and
    /// sometimes unwind. The texture-name lookup loop (non-zero argument with
    /// banks that have entries) is not reached: banks have no entries.
    #[test]
    fn rider_model_setup_matches() {
        covered(
            ssx3_ee::s10::f_0030dbd0__generated,
            rider_model_setup,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_dbd0, 6);
                pool_allocator(c);
                heap_init(c);
                c.hooks.miss = compile_miss;
                // Constant matrices the setup copies (0x4ff1a0.. and 0x4ff230..).
                for i in 0..4u32 {
                    let mut v = [0u32; 4];
                    v[i as usize] = 0x3f80_0000;
                    put_vec(c, 0x004f_f1a0 + 0x10 * i, v);
                }
                for i in 0..2u32 {
                    put_vec(
                        c,
                        0x004f_f230 + 0x10 * i,
                        [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                    );
                }
                for a in (MODEL_3..MODEL + 0x200).step_by(4) {
                    c.bus.write32(a, 0);
                }
                for a in (BASE + 0x1000..BASE + 0xb000).step_by(4) {
                    c.bus.write32(a, 0);
                }
                let count = 1 + seed % 4;
                let arg = (seed >> 2) & 1;
                let cap = if (seed >> 3) & 1 == 0 {
                    count + 1 + (r.next_u32() & 1)
                } else {
                    count
                };
                let reversed = (seed >> 4) & 1 == 1;
                let lods = 1 + (r.next_u32() & 1);
                let parts = if cap > count {
                    run(
                        c,
                        0x0032_0058,
                        ssx3_ee::s10::f_00320058,
                        &[MEM_HEAP as u64, 16, (0x58 * cap) as u64],
                    ) as u32
                } else {
                    PARTS
                };
                let mut nodes = [0u32; 4];
                for n in nodes.iter_mut().take(count as usize) {
                    *n = r.next_u32() % 4;
                }
                if nodes.iter().sum::<u32>() == 0 {
                    nodes[0] = 1;
                }
                let total: u32 = nodes.iter().sum();
                for i in 0..count {
                    let e = parts + 0x58 * i;
                    for k in (0..0x58).step_by(4) {
                        c.bus.write32(e + k, 0);
                    }
                    let index = (if reversed { count - 1 - i } else { i }) * 2;
                    c.bus.write32(e, index);
                    c.bus.write32(e + 0x44, nodes[i as usize]);
                    c.bus.write32(e + 0x4c, r.next_u32() % 3);
                    let node_base = NODES + 0x200 * i;
                    c.bus.write32(e + 0x38, node_base);
                    for j in 0..nodes[i as usize] {
                        let n = node_base + 0x50 * j;
                        for k in (0..0x50).step_by(4) {
                            let v = r.f32(2.0);
                            c.bus.write32(n + k, v);
                        }
                        let parent = (r.next_u32() % (total + 1)) as i32 - 1;
                        c.bus.write16(n + 0x12, parent as u16);
                        c.bus.write16(n + 0x14, (r.next_u32() & 3) as u16);
                    }
                    for ch in 0..4u32 {
                        let table = LOD_TABLES + (i * 4 + ch) * 0x40;
                        c.bus.write32(e + 0x1c + 8 * ch, table);
                        for j in 0..lods {
                            let data = if r.next_u32() % 3 == 0 { 0 } else { MESH_DATA };
                            c.bus.write32(table + 0x10 * j, data);
                            c.bus.write32(table + 0x10 * j + 4, r.next_u32() & 0xff);
                        }
                    }
                }
                c.bus.write32(MODEL_3, cap);
                c.bus.write32(MODEL_3 + 4, lods);
                c.bus.write32(MODEL_3 + 8, count);
                c.bus.write32(MODEL_3 + 0xc, parts);
                let banks = r.next_u32() % 3;
                c.bus.write32(MODEL_3 + 0x60, banks);
                for k in 0..banks {
                    c.bus.write32(MODEL_3 + 0x64 + 4 * k, BANKS + 0x100 * k);
                }
                // The graphics manager: vtable object, name cache, no history buffer.
                c.bus.write32(GAME_GP + 0x2a90, GRAPHICS_MAN);
                c.bus.write32(GRAPHICS_MAN + 0x10d8, MAN_VTABLE);
                c.bus.write32(GRAPHICS_MAN + 0xe80, r.next_u32() % 4);
                for k in (0x280..0x2b0).step_by(4) {
                    c.bus.write32(GRAPHICS_MAN + k, r.next_u32());
                }
                for off in [0x368u32, 0x370] {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(MAN_VTABLE + off, delta as u16);
                }
                c.bus.write32(MAN_VTABLE + 0x36c, FAKE);
                c.bus.write32(MAN_VTABLE + 0x374, FAKE + 0x10);
                c.g[A0][0] = sx(MODEL_3);
                c.g[A1][0] = arg as u64;
                c.pc = 0x0030_dbd0;
            },
        );
    }

    /// A short NUL-terminated string.
    pub(crate) fn oc417_put_str(c: &mut Cpu, at: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    // ----------------------------------------------------------------------
    // RiderModel_GetBoneMask

    /// A rider model with 4 parts; the part table, bone names and a token
    /// list in random combinations: part index in or out of range, no table,
    /// negative table entries, tokens that are the morph name, a bone name,
    /// or nothing known, empty tokens, trailing commas.
    #[test]
    fn rider_model_get_bone_mask_matches() {
        const MODEL: u32 = DATA + 0x1000;
        const TABLE: u32 = DATA + 0x1100;
        const PARTS: u32 = DATA + 0x1200;
        const NAMES: u32 = DATA + 0x2000;
        const TOKENS: u32 = DATA + 0x3000;
        const WORDS: [&str; 6] = ["head", "arm", "leg", "torso", "morph", "bogus"];
        check(
            ee::s10::f_00310ce8__generated,
            rider_model_get_bone_mask,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4170_0001, 0);
                c.pc = 0x0031_0ce8;
                oc417_put_str(c, BONE_MASK_STR_MORPH, "morph");
                c.bus.write32(MODEL + BONE_MASK_MODEL_PARTS, PARTS);
                c.bus
                    .write32(MODEL + BONE_MASK_MODEL_NUM_BONES, r.next_u32() % 40);
                c.bus.write32(
                    MODEL + BONE_MASK_MODEL_MAX_PART,
                    if seed % 9 == 0 { 1 } else { 3 },
                );
                c.bus.write32(
                    MODEL + BONE_MASK_MODEL_PART_INDEX,
                    if seed % 11 == 0 { 0 } else { TABLE },
                );
                for i in 0..4u32 {
                    let entry = if (seed / 3 + i) % 7 == 0 {
                        0xffff_ffff
                    } else {
                        i
                    };
                    c.bus.write32(TABLE + 4 * i, entry);
                    let part = PARTS + 0x58 * i;
                    c.bus
                        .write32(part + BONE_MASK_PART_FIRST_BONE, r.next_u32() % 30);
                    let morph = if (seed + i) % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 20
                    };
                    c.bus.write32(part + BONE_MASK_PART_MORPH, morph);
                    c.bus
                        .write32(part + BONE_MASK_PART_NAMES, NAMES + 0x140 * i);
                    c.bus
                        .write32(part + BONE_MASK_PART_NUM_BONES, 1 + r.next_u32() % 4);
                    for j in 0..4u32 {
                        let w = WORDS[((seed + i + j) % 5) as usize];
                        oc417_put_str(c, NAMES + 0x140 * i + BONE_MASK_NAME_STRIDE * j, w);
                    }
                }
                let mut tokens = String::new();
                if seed % 7 == 3 {
                    tokens.push(',');
                }
                for t in 0..(seed % 5) {
                    if t > 0 {
                        tokens.push(',');
                    }
                    tokens.push_str(WORDS[(r.next_u32() % 6) as usize]);
                }
                if seed % 8 == 5 {
                    tokens.push(',');
                }
                oc417_put_str(c, TOKENS, &tokens);
                c.g[A0][0] = MODEL as u64;
                c.g[A1][0] = match seed % 10 {
                    0 => 4,
                    1 => 0xffff_ffff_ffff_ffff,
                    n => (n % 4) as u64,
                };
                c.g[A2][0] = TOKENS as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // AnimDataBase_AllocChanCache

    /// Several part ids and channel counts; `operator new` faked: it returns
    /// the block at `DATA + 0x4000` (20 nodes of 0x24 bytes plus the ring
    /// links) for every allocation.
    #[test]
    fn anim_data_base_alloc_chan_cache_matches() {
        check(
            ee::s10::f_00310f18__generated,
            anim_data_base_alloc_chan_cache_2,
            200,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0004, 22);
                c.pc = 0x0031_0f18;
                oc417_force(c, FK_NEW_NAMED, DATA + 0x4000);
                c.g[A0][0] = (r.next_u32() % 40) as u64;
                c.g[A1][0] = (r.next_u32() % 12) as u64;
            },
        );
    }

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC427_CTRL;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC427_MAGIC;
    use std::ptr::addr_of_mut;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_queue_base;

    /// Pop the next queued value of callee `addr`.
    pub(crate) fn oc427_pop(c: &mut Cpu, addr: u32) -> Option<u64> {
        let q = oc427_queue_base(addr);
        let head = c.bus.read32(q);
        if head < c.bus.read32(q + 4) {
            c.bus.write32(q, head + 1);
            Some(c.bus.read64(q + 8 + 8 * head))
        } else {
            None
        }
    }

    /// A gated callee fake: logs the call like `fakes::fake`, then returns the
    /// next queued value; `NextToken` stores it as the parser's token
    /// instead, `GetView` fills the pose it was asked for.
    pub(crate) fn oc427_fake_call(c: &mut Cpu, addr: u32, ra: u32) {
        let a0 = c.g[A0][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match addr {
            0x0032_1af0 => {
                if let Some(token) = oc427_pop(c, addr) {
                    c.bus.write32(a0 + 0x98, token as u32);
                }
            }
            0x0016_73a0 => {
                for k in 0..8u32 {
                    c.bus
                        .write32(a0 + 4 * k, fakes::mix(n as u64 * 8 + k as u64) as u32);
                }
            }
            _ => {
                if let Some(v) = oc427_pop(c, addr) {
                    c.g[V0][0] = v;
                }
            }
        }
    }

    macro_rules! oc427_fakes {
        ($(($name:ident, $addr:expr, $shard:ident, $slot:ident, $gen:ident)),* $(,)?) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC427_CTRL) != OC427_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), ee::$shard::$gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    oc427_fake_call(c, $addr, ra);
                }
            )*

            /// Hooks the gated fakes into the callees' override slots (once per
            /// process; with the gate off they run what the slots held).
            pub(crate) fn oc427_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc427_fakes! {
        (oc427_fake_00283d28, 0x0028_3d28, s07, OVERRIDE_00283D28, f_00283d28__generated),
        (oc427_fake_00283d70, 0x0028_3d70, s07, OVERRIDE_00283D70, f_00283d70__generated),
        (oc427_fake_00277dd8, 0x0027_7dd8, s07, OVERRIDE_00277DD8, f_00277dd8__generated),
        (oc427_fake_002795d0, 0x0027_95d0, s07, OVERRIDE_002795D0, f_002795d0__generated),
        (oc427_fake_00279528, 0x0027_9528, s07, OVERRIDE_00279528, f_00279528__generated),
        (oc427_fake_002838e8, 0x0028_38e8, s07, OVERRIDE_002838E8, f_002838e8__generated),
        (oc427_fake_00273dc0, 0x0027_3dc0, s07, OVERRIDE_00273DC0, f_00273dc0__generated),
        (oc427_fake_00274d70, 0x0027_4d70, s07, OVERRIDE_00274D70, f_00274d70__generated),
        (oc427_fake_00274c20, 0x0027_4c20, s07, OVERRIDE_00274C20, f_00274c20__generated),
        (oc427_fake_002743c8, 0x0027_43c8, s07, OVERRIDE_002743C8, f_002743c8__generated),
        (oc427_fake_00273a60, 0x0027_3a60, s07, OVERRIDE_00273A60, f_00273a60__generated),
        (oc427_fake_00281c40, 0x0028_1c40, s07, OVERRIDE_00281C40, f_00281c40__generated),
        (oc427_fake_0027d2e8, 0x0027_d2e8, s07, OVERRIDE_0027D2E8, f_0027d2e8__generated),
        (oc427_fake_0027d2f0, 0x0027_d2f0, s07, OVERRIDE_0027D2F0, f_0027d2f0__generated),
        (oc427_fake_00116fa8, 0x0011_6fa8, s00, OVERRIDE_00116FA8, f_00116fa8__generated),
        (oc427_fake_0014dc80, 0x0014_dc80, s01, OVERRIDE_0014DC80, f_0014dc80__generated),
        (oc427_fake_0014dd58, 0x0014_dd58, s01, OVERRIDE_0014DD58, f_0014dd58__generated),
        (oc427_fake_00144bc0, 0x0014_4bc0, s01, OVERRIDE_00144BC0, f_00144bc0__generated),
        (oc427_fake_0014c6a0, 0x0014_c6a0, s01, OVERRIDE_0014C6A0, f_0014c6a0__generated),
        (oc427_fake_001567b8, 0x0015_67b8, s01, OVERRIDE_001567B8, f_001567b8__generated),
        (oc427_fake_001538e8, 0x0015_38e8, s01, OVERRIDE_001538E8, f_001538e8__generated),
        (oc427_fake_00146e98, 0x0014_6e98, s01, OVERRIDE_00146E98, f_00146e98__generated),
        (oc427_fake_00147398, 0x0014_7398, s01, OVERRIDE_00147398, f_00147398__generated),
        (oc427_fake_00145750, 0x0014_5750, s01, OVERRIDE_00145750, f_00145750__generated),
        (oc427_fake_0014a130, 0x0014_a130, s01, OVERRIDE_0014A130, f_0014a130__generated),
        (oc427_fake_001535c0, 0x0015_35c0, s01, OVERRIDE_001535C0, f_001535c0__generated),
        (oc427_fake_00176d68, 0x0017_6d68, s02, OVERRIDE_00176D68, f_00176d68__generated),
        (oc427_fake_00166640, 0x0016_6640, s02, OVERRIDE_00166640, f_00166640__generated),
        (oc427_fake_00166f90, 0x0016_6f90, s02, OVERRIDE_00166F90, f_00166f90__generated),
        (oc427_fake_001673a0, 0x0016_73a0, s02, OVERRIDE_001673A0, f_001673a0__generated),
        (oc427_fake_00317e30, 0x0031_7e30, s10, OVERRIDE_00317E30, f_00317e30__generated),
        (oc427_fake_0031c228, 0x0031_c228, s10, OVERRIDE_0031C228, f_0031c228__generated),
        (oc427_fake_0031c128, 0x0031_c128, s10, OVERRIDE_0031C128, f_0031c128__generated),
        (oc427_fake_00317e98, 0x0031_7e98, s10, OVERRIDE_00317E98, f_00317e98__generated),
        (oc427_fake_003177f0, 0x0031_77f0, s10, OVERRIDE_003177F0, f_003177f0__generated),
        (oc427_fake_00317d70, 0x0031_7d70, s10, OVERRIDE_00317D70, f_00317d70__generated),
        (oc427_fake_00321500, 0x0032_1500, s10, OVERRIDE_00321500, f_00321500__generated),
        (oc427_fake_00321af0, 0x0032_1af0, s10, OVERRIDE_00321AF0, f_00321af0__generated),
        (oc427_fake_00323268, 0x0032_3268, s10, OVERRIDE_00323268, f_00323268__generated),
        (oc427_fake_0031bf60, 0x0031_bf60, s10, OVERRIDE_0031BF60, f_0031bf60__generated),
        (oc427_fake_0031c040, 0x0031_c040, s10, OVERRIDE_0031C040, f_0031c040__generated),
        (oc427_fake_003215f8, 0x0032_15f8, s10, OVERRIDE_003215F8, f_003215f8__generated),
        (oc427_fake_003506d8, 0x0035_06d8, s11, OVERRIDE_003506D8, f_003506d8__generated),
        (oc427_fake_00416210, 0x0041_6210, s15, OVERRIDE_00416210, f_00416210__generated),
        (oc427_fake_0041605c, 0x0041_605c, s15, OVERRIDE_0041605C, f_0041605c__generated),
        (oc427_fake_0040d758, 0x0040_d758, s15, OVERRIDE_0040D758, f_0040d758__generated),
        (oc427_fake_0040da10, 0x0040_da10, s15, OVERRIDE_0040DA10, f_0040da10__generated),
        (oc427_fake_00123128, 0x0012_3128, s00, OVERRIDE_00123128, f_00123128__generated),
        (oc427_fake_002742e0, 0x0027_42e0, s07, OVERRIDE_002742E0, f_002742e0__generated),
        (oc427_fake_0031d790, 0x0031_d790, s10, OVERRIDE_0031D790, f_0031d790__generated),
        (oc427_fake_00317e50, 0x0031_7e50, s10, OVERRIDE_00317E50, f_00317e50__generated),
    }

    /// `RiderModel_AddModelRef` (a leaf): appends to the reference list.
    #[test]
    fn rider_model_add_model_ref_matches() {
        check(
            ee::s10::f_0030db70__generated,
            rider_model_add_model_ref,
            50,
            &|c, seed| {
                let mut r = Rng(0x30db_7000 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0030_db70;
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x60, r.next_u32() % 8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::intersect::riderspheretree::tests::OC430_CTRL;
    pub(crate) use crate::game::intersect::riderspheretree::tests::OC430_MAGIC;

    /// Channel-cache tables (0x4fe7a0 / 0x4fe860) for `AnimDataBase_FreeChanCaches`:
    /// a few of the 48 entries point at 20 records of 0x24 bytes whose object
    /// and block pointers are zero or non-zero.
    pub(crate) fn oc430_chan_caches(c: &mut Cpu, r: &mut Rng, seed: u32) {
        for i in 0..48u32 {
            let live = if seed % 4 == 0 {
                i < 2
            } else {
                r.next_u32() % 12 == 0
            };
            let base = DATA + 0x1000 + i * 0x300;
            c.bus
                .write32(0x004f_e7a0 + 4 * i, if live { base } else { 0 });
            c.bus.write32(0x004f_e860 + 4 * i, r.next_u32());
            if live {
                for j in 0..20u32 {
                    let rec = base + 0x24 * j;
                    let obj = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x20000 + 0x40 * (i * 20 + j)
                    };
                    c.bus.write32(rec, obj);
                    let blk = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(rec + 4, blk);
                }
            }
        }
    }

    /// `AnimDataBase_FreeChanCaches`: empty and live entries, records with
    /// and without object / block pointers; the frees are gated fakes.
    #[test]
    fn anim_data_base_free_chan_caches_matches() {
        oc430_install_fakes();
        covered(
            ee::s10::f_00311110__generated,
            anim_data_base_free_chan_caches,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_1110, 30);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0031_1110;
                oc430_chan_caches(c, &mut r, seed);
            },
        );
    }

    /// The same with event delivery at the loop back edges.
    #[test]
    fn anim_data_base_free_chan_caches_events_match() {
        oc430_install_fakes();
        check_events(
            ee::s10::f_00311110__generated,
            anim_data_base_free_chan_caches,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_1111, 0);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                oc430_chan_caches(c, &mut r, seed);
                let mode = schedule(c, &mut r, seed, 0x0031_1110);
                c.g[RA][0] = SENTINEL as u64;
                mode
            },
        );
    }

    #[test]
    fn rider_model_get_part_point_world_matches() {
        check(
            ee::s10::f_00310458__generated,
            rider_model_get_part_point_world,
            200,
            &|c, seed| {
                let mut r = Rng(0x0031_0458 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_0458;
                let model = DATA;
                let parts = DATA + 0x100;
                let hot = DATA + 0x400;
                let skel = DATA + 0x800;
                // numParts 0..3: the zero-part path and parts with ids 5, 7, 9.
                c.bus.write32(model + 0x8, seed % 4);
                c.bus.write32(model + 0xc, parts);
                c.bus.write32(model + 0x34, skel);
                for i in 0..3u32 {
                    let p = parts + i * 0x58;
                    c.bus.write32(p, 5 + 2 * i);
                    c.bus.write32(p + 0x4, r.next_u32() % 3);
                    c.bus.write32(p + 0x3c, hot);
                }
                for k in 0..4u32 {
                    let h = hot + k * 16;
                    c.bus.write32(h, r.next_u32());
                    c.bus.write32(h + 0x4, r.next_u32());
                    c.bus.write32(h + 0x8, r.next_u32());
                    c.bus.write32(h + 0xc, r.next_u32() % 4);
                }
                fill(c, &mut r, skel, 128);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = model as u64;
                // Found (ids 5, 7, 9) or missing (id 3).
                c.g[A2][0] = if seed % 5 == 0 {
                    3
                } else {
                    (5 + 2 * (seed % 3)) as u64
                };
                c.g[A3][0] = (seed % 4) as u64;
            },
        );
    }
}
