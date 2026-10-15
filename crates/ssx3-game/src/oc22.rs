//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Hand-written, typed replacements for small generated functions from the
//! world / world-cache / streamer / script-cache blocks, each with a
//! differential test against the generated body.

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_7790, world_block_pool_free_block, "WorldBlockPool_FreeBlock"),
    (0x003a_77b0, section_allocator_ctor, "SectionAllocator_ctor"),
    (0x003a_8448, c_world_cache_request_hull, "cWorldCache_RequestHull"),
    (0x003a_84c8, c_world_cache_expire_hull, "cWorldCache_ExpireHull"),
    (0x003a_84f8, c_world_cache_set_hull_pinned, "cWorldCache_SetHullPinned"),
    (0x003a_8618, c_world_cache_is_hull_resident, "cWorldCache_IsHullResident"),
    (0x003a_8638, c_world_cache_is_hull_free, "cWorldCache_IsHullFree"),
    (0x003a_8650, c_world_cache_set_hull_resident, "cWorldCache_SetHullResident"),
    (0x003a_9820, c_world_cache_streamer_get_section, "cWorldCache_StreamerGetSection"),
    (
        0x003a_9858,
        c_world_cache_streamer_is_section_resident,
        "cWorldCache_StreamerIsSectionResident",
    ),
    (
        0x003a_9890,
        c_world_cache_streamer_is_section_unloaded,
        "cWorldCache_StreamerIsSectionUnloaded",
    ),
    (0x003a_9a98, c_world_cache_streamer_get_hull, "cWorldCache_StreamerGetHull"),
    (0x003a_9ab0, c_world_cache_streamer_get_hull_count, "cWorldCache_StreamerGetHullCount"),
    (0x003a_ca38, c_wscript_cache_get_data, "cWScriptCache_GetData"),
    (0x003a_ca50, t_wscript_cache_ctor, "tWScriptCache_ctor"),
    (0x003a_d188, ssx_app_get_game_module, "SSXApp_GetGameModule"),
    (0x003a_d198, game_module_get_occlusion_man, "GameModule_GetOcclusionMan"),
    (0x003a_dc50, t_wscript_cache_get_table_entry, "tWScriptCache_GetTableEntry"),
    (0x003a_dce0, t_wscript_cache_get_process_count, "tWScriptCache_GetProcessCount"),
    (0x003a_dd10, t_wscript_cache_get_processes, "tWScriptCache_GetProcesses"),
    (0x003a_dd40, t_wscript_cache_get_mission_count, "tWScriptCache_GetMissionCount"),
    (0x003a_dd70, t_wscript_cache_get_missions, "tWScriptCache_GetMissions"),
    (0x003b_0340, av_subtitle_init, "AV_SubtitleInit"),
    (0x003b_0410, rcmp_system_ctor, "RCMP_SYSTEM_ctor"),
    (0x003b_0720, av_chunk_init, "AV_ChunkInit"),
    (0x003b_0738, rcmp_decoder_init_callbacks, "RCMP_DECODER_InitCallbacks"),
    (0x003b_0758, rcmp_make_callbacks, "RCMP_MakeCallbacks"),
    (0x003b_0948, ps2_sony_codec_internal_ctor, "PS2_SONY_CODEC_INTERNAL_ctor"),
    (
        0x003b_10d0,
        ps2_sony_codec_internal_pop_free_frame,
        "PS2_SONY_CODEC_INTERNAL_PopFreeFrame",
    ),
    (0x003b_1258, mpeg2_show_bits, "MPEG2_Show_Bits"),
    (
        0x003b_2440,
        mpeg2_update_temporal_reference_tacking_data,
        "MPEG2_Update_Temporal_Reference_Tacking_Data",
    ),
    (0x003b_2688, shp_palette_entries_for_bpp, "SHP_PaletteEntriesForBpp"),
    (0x003b_39d8, shp_get_bpp, "SHP_GetBpp"),
    (0x003b_3d00, shps_get_entry_name, "SHPS_GetEntryName"),
    (0x003b_3da8, shp_format_from_bpp, "SHP_FormatFromBpp"),
    (0x003b_42b0, shp_swizzle_addr_psmt4, "SHP_SwizzleAddrPSMT4"),
    (0x003b_4338, shp_swizzle_addr_psmt8, "SHP_SwizzleAddrPSMT8"),
    (0x003b_43c0, shp_swizzle_addr_psmct16, "SHP_SwizzleAddrPSMCT16"),
    (0x003b_4448, shp_swizzle_addr_psmct32, "SHP_SwizzleAddrPSMCT32"),
    (0x003b_5910, snd_add_tick_callback, "SND_AddTickCallback"),
    (0x003b_5a28, snd_add_event_callback, "SND_AddEventCallback"),
    (0x003b_5ba0, snd_mem_clamp_block, "SND_MemClampBlock"),
    (0x003b_5bd0, snd_mem_init_heap, "SND_MemInitHeap"),
    (0x003b_c5c8, snd_random, "SND_Random"),
    (0x003b_c930, sndenc_install_dts, "SNDENC_InstallDts"),
    (0x003c_0c18, snd_get_group_voice_range, "SND_GetGroupVoiceRange"),
    (0x003c_3500, snd_set_spu_reverb_mode, "SND_SetSpuReverbMode"),
    (0x003c_36c8, snd_set_spu_fx_volume, "SND_SetSpuFxVolume"),
    (0x003c_4af8, snd_spu_mem_clamp_block, "SND_SpuMemClampBlock"),
    (0x003c_4e50, sndi_spu_mem_set_base, "SNDI_SpuMemSetBase"),
];

// ---------------------------------------------------------------------------
// World block pool / section allocator.

/// `void WorldBlockPool_FreeBlock(WorldBlockPool* pool, void* block)`
/// (0x3a7790): push `block` onto the pool's free list at +0x10, recording the
/// old head at block+0x14 and bumping the free count at pool+0x8.
pub fn world_block_pool_free_block(c: &mut Cpu) {
    c.cycles += 8;
    let pool = Addr(abi::arg(c, 0));
    let block = Addr(abi::arg(c, 1));
    block.off(0x10).sw(c, 0);
    c.g[V0][0] = pool.off(0x10).lw(c);
    block.off(0x14).sw(c, c.g[V0][0]);
    pool.off(0x10).sw(c, c.g[A1][0]);
    c.g[V0][0] = pool.off(0x8).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    pool.off(0x8).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `SectionAllocator* SectionAllocator_ctor(SectionAllocator* self, void*
/// begin, void* end)` (0x3a77b0): initialise the allocator's first/current/
/// end pointers and clear its counters.
pub fn section_allocator_ctor(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    this.off(0x4).sw(c, c.g[A2][0]);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, c.g[A1][0]);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World cache hulls.

/// `void cWorldCache_RequestHull(cWorldCache* self, int index)` (0x3a8448):
/// claim the hull slot `index` (0x18 bytes each, table at +0x3f0), fill in the
/// free-list linkage and set the slot's reference count to 1 (0 stays 0).
pub fn c_world_cache_request_hull(c: &mut Cpu) {
    c.cycles += 16;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 0x18;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.mult(0, A1, A1, V0);
    c.g[A3][0] = sx(self_.wrapping_add(0x3f0));
    c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
    c.g[V0][0] = sx(self_.wrapping_add(c.g[A1][0] as u32));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x400), c.g[A2][0]);
    c.g[V1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[V0][0];
    c.swc1(12, (c.g[V1][0] as u32).wrapping_add(0x3fc));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x3f4), 0);
    c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
    c.g[V0][0] = ((c.g[V1][0] as i64) < 4) as u64;
    if c.g[V0][0] == 0 {
        // Slot already had a count >= 4: leave it alone unless exactly 4.
        c.g[V0][0] = 0x4;
        c.cycles += 2;
        let is_four = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x3;
        if is_four {
            c.cycles += 1;
            c.sw(c.g[A3][0] as u32, c.g[V0][0]);
        } else {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
    } else {
        c.cycles += 2;
        if (c.g[V1][0] as i64) > 0 {
            // 1..3: already in use.
        } else {
            c.cycles += 2;
            let is_zero = c.g[V1][0] == 0;
            c.g[V0][0] = 0x1;
            if is_zero {
                c.cycles += 2;
                c.sw(c.g[A3][0] as u32, c.g[V0][0]);
            } else {
                c.cycles += 2;
            }
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void cWorldCache_ExpireHull(cWorldCache* self, int index)` (0x3a84c8):
/// release the hull slot `index`: clear its in-use flag and refresh its
/// last-use stamp from the global frame counter.
pub fn c_world_cache_expire_hull(c: &mut Cpu) {
    c.cycles += 11;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 0x18;
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.mult(0, A1, A1, V0);
    c.g[V1][0] = sx(self_.wrapping_add(c.g[A1][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x3f4), 0);
    c.g[A1][0] = c.g[V1][0];
    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x1c).lw(c);
    c.g[V1][0] = Addr(self_.wrapping_add(0x1bf0)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.sw((c.g[A1][0] as u32).wrapping_add(0x400), c.g[V0][0]);
    abi::ret(c);
}

/// `void cWorldCache_SetHullPinned(cWorldCache* self, int index, int pinned)`
/// (0x3a84f8): store `pinned` in the hull slot's flag word.
pub fn c_world_cache_set_hull_pinned(c: &mut Cpu) {
    c.cycles += 5;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 0x18;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx(self_.wrapping_add(c.g[V1][0] as u32));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x3f4), c.g[A2][0]);
    abi::ret(c);
}

/// `bool cWorldCache_IsHullResident(cWorldCache* self, int index)` (0x3a8618):
/// true when the hull slot's ref count is exactly 3.
pub fn c_world_cache_is_hull_resident(c: &mut Cpu) {
    c.cycles += 7;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 0x18;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx(self_.wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x3f0).lw(c);
    c.g[V0][0] = c.g[V0][0] ^ 0x3;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}

/// `bool cWorldCache_IsHullFree(cWorldCache* self, int index)` (0x3a8638):
/// true when the hull slot's ref count is zero.
pub fn c_world_cache_is_hull_free(c: &mut Cpu) {
    c.cycles += 6;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 0x18;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx(self_.wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x3f0).lw(c);
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}

/// `void cWorldCache_SetHullResident(cWorldCache* self, int index)`
/// (0x3a8650): mark the hull slot resident (ref count 3).
pub fn c_world_cache_set_hull_resident(c: &mut Cpu) {
    c.cycles += 6;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 0x18;
    c.g[V1][0] = 0x3;
    c.mult(0, A2, A1, V0);
    c.g[A0][0] = sx(self_.wrapping_add(c.g[A2][0] as u32));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x3f0), c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World cache streamer sections.

/// `Section* cWorldCache_StreamerGetSection(Streamer* self, int index)`
/// (0x3a9820): look the section up in the table at self+8; records are 0x58
/// bytes.
pub fn c_world_cache_streamer_get_section(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = self_.off(0x4).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x58;
        if in_range {
            c.cycles += 4;
            c.g[V1][0] = self_.off(0x8).lw(c);
            c.mult(0, V0, A1, V0);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `bool cWorldCache_StreamerIsSectionResident(Streamer* self, int index)`
/// (0x3a9858): true when the section's state at +0x14 is 3 or 4.
pub fn c_world_cache_streamer_is_section_resident(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = self_.off(0x4).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
        if in_range {
            c.cycles += 5;
            c.g[V1][0] = sx(self_.0.wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(5));
            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `bool cWorldCache_StreamerIsSectionUnloaded(Streamer* self, int index)`
/// (0x3a9890): true when the section's state at +0x14 is zero.
pub fn c_world_cache_streamer_is_section_unloaded(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = self_.off(0x4).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
        if in_range {
            c.cycles += 4;
            c.g[V1][0] = sx(self_.0.wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
            c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `Hull* cWorldCache_StreamerGetHull(Streamer* self, int index)`
/// (0x3a9a98): base of the hull array at self+0x10 plus index*0x44.
pub fn c_world_cache_streamer_get_hull(c: &mut Cpu) {
    c.cycles += 5;
    let self_ = abi::arg(c, 0);
    c.g[V1][0] = 0x44;
    c.g[V0][0] = Addr(self_.wrapping_add(0x10)).lw(c);
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}

/// `int cWorldCache_StreamerGetHullCount(Streamer* self)` (0x3a9ab0): hull
/// count at the streamer's owner + 0x10.
pub fn c_world_cache_streamer_get_hull_count(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = abi::arg(c, 0);
    c.g[V1][0] = Addr(self_.wrapping_add(0x4)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// WScript cache.

/// `void* cWScriptCache_GetData(cWScriptCache* self, int index)` (0x3aca38):
/// data pointer of entry `index` in the table at self+4 (records 0xc bytes).
pub fn c_wscript_cache_get_data(c: &mut Cpu) {
    c.cycles += 6;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 0xc;
    c.g[V1][0] = Addr(self_.wrapping_add(4)).lw(c);
    c.mult(0, A1, A1, V0);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x8).lw(c);
    abi::ret(c);
}

/// `tWScriptCache* tWScriptCache_ctor(tWScriptCache* self)` (0x3aca50):
/// clear the type/count/tail fields and set the head to -1.
pub fn t_wscript_cache_ctor(c: &mut Cpu) {
    c.cycles += 7;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = u64::MAX;
    self_.sh(c, 0);
    self_.off(0x4).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    self_.off(0x8).sw(c, 0);
    self_.off(0x2).sh(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// App / game module globals.

/// `GameModule* SSXApp_GetGameModule(void)` (0x3ad188): the world singleton's
/// game module at +0x84.
pub fn ssx_app_get_game_module(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
    abi::ret(c);
}

/// `void* GameModule_GetOcclusionMan(void)` (0x3ad198): the occlusion manager
/// at +0x8c of the world singleton's +0x84 module.
pub fn game_module_get_occlusion_man(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x8c).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// WScript cache accessors.

/// `void* tWScriptCache_GetTableEntry(tWScriptCache* self, int index)`
/// (0x3adc50): the table at self+8 (only when the cache type is 3), entry
/// `index`, whose array hangs off +0x3c.
pub fn t_wscript_cache_get_table_entry(c: &mut Cpu) {
    c.cycles += 4;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V1][0] = self_.lh(c);
    c.g[V0][0] = 0x3;
    if c.g[V1][0] == c.g[V0][0] {
        c.g[V0][0] = self_.off(0x8).lw(c);
        c.cycles += 5;
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x3c).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// Shared body of the `tWScriptCache_Get*` accessors (0x3adce0..0x3add70):
/// valid only when the cache type is 3 and it has a table; otherwise null.
fn t_wscript_cache_field(c: &mut Cpu, offset: u32) {
    c.cycles += 4;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = self_.lh(c);
    c.g[V1][0] = 0x3;
    if c.g[V0][0] != c.g[V1][0] {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A0][0] = self_.off(0x8).lw(c);
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(offset).lw(c);
    abi::ret(c);
}

/// `int tWScriptCache_GetProcessCount(tWScriptCache* self)` (0x3adce0).
pub fn t_wscript_cache_get_process_count(c: &mut Cpu) {
    t_wscript_cache_field(c, 0x28);
}

/// `void* tWScriptCache_GetProcesses(tWScriptCache* self)` (0x3add10).
pub fn t_wscript_cache_get_processes(c: &mut Cpu) {
    t_wscript_cache_field(c, 0x2c);
}

/// `int tWScriptCache_GetMissionCount(tWScriptCache* self)` (0x3add40).
pub fn t_wscript_cache_get_mission_count(c: &mut Cpu) {
    t_wscript_cache_field(c, 0x20);
}

/// `void* tWScriptCache_GetMissions(tWScriptCache* self)` (0x3add70).
pub fn t_wscript_cache_get_missions(c: &mut Cpu) {
    t_wscript_cache_field(c, 0x24);
}

// ---------------------------------------------------------------------------
// AV subtitle / RCMP codec.

/// `void AV_SubtitleInit(AVSubtitle* self)` (0x3b0340): fill in the default
/// subtitle geometry/state fields.
pub fn av_subtitle_init(c: &mut Cpu) {
    c.cycles += 26;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x20;
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = 0xff;
    c.g[V1][0] |= 0xffff;
    c.g[T1][0] = 0x2;
    c.g[A2][0] = 0x15e;
    c.g[A3][0] = 0x258;
    c.g[T0][0] = 0x32;
    self_.sw(c, c.g[V0][0]);
    self_.off(0x4).sw(c, c.g[A2][0]);
    c.g[V0][0] = c.g[A0][0];
    self_.off(0x8).sw(c, c.g[A3][0]);
    self_.off(0xc).sw(c, c.g[T0][0]);
    self_.off(0x14).sw(c, c.g[V1][0]);
    self_.off(0x27).sb(c, c.g[A1][0]);
    self_.off(0x1c).sw(c, c.g[T1][0]);
    self_.off(0x10).sw(c, c.g[V1][0]);
    self_.off(0x24).sb(c, c.g[A1][0]);
    self_.off(0x25).sb(c, c.g[A1][0]);
    self_.off(0x26).sb(c, c.g[A1][0]);
    self_.off(0x18).sw(c, c.g[T1][0]);
    self_.off(0x20).sw(c, 0);
    self_.off(0x28).sw(c, 0);
    self_.off(0x2c).sw(c, 0);
    self_.off(0x30).sw(c, 0);
    abi::ret(c);
}

/// `RCMP_SYSTEM* RCMP_SYSTEM_ctor(RCMP_SYSTEM* self)` (0x3b0410): initialise
/// the RCMP system: vtable-ish constant at +0xc, counters cleared.
pub fn rcmp_system_ctor(c: &mut Cpu) {
    c.cycles += 7;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V1][0] = 0x45_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6890));
    self_.sw(c, 0);
    self_.off(0xc).sw(c, c.g[V1][0]);
    self_.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `AV_Chunk* AV_ChunkInit(AV_Chunk* self)` (0x3b0720): clear the chunk's
/// state words.
pub fn av_chunk_init(c: &mut Cpu) {
    c.cycles += 5;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    self_.off(0x4).sw(c, 0);
    self_.off(0x8).sw(c, 0);
    self_.sw(c, 0);
    abi::ret(c);
}

/// `void* RCMP_DECODER_InitCallbacks(RCMP_DECODER* self)` (0x3b0738): set the
/// initial state to 2 and clear the callback pointers.
pub fn rcmp_decoder_init_callbacks(c: &mut Cpu) {
    c.cycles += 7;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x2;
    self_.sw(c, 0);
    self_.off(0xc).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    self_.off(0x4).sw(c, 0);
    self_.off(0x8).sw(c, 0);
    abi::ret(c);
}

/// `RCMP_CALLBACKS* RCMP_MakeCallbacks(RCMP_CALLBACKS* self, void* open,
/// void* close, void* read, void* arg)` (0x3b0758): fill in the four
/// callback slots.
pub fn rcmp_make_callbacks(c: &mut Cpu) {
    c.cycles += 6;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0xc).sw(c, c.g[T0][0]);
    c.g[V0][0] = c.g[A0][0];
    self_.sw(c, c.g[A1][0]);
    self_.off(0x4).sw(c, c.g[A2][0]);
    self_.off(0x8).sw(c, c.g[A3][0]);
    abi::ret(c);
}

/// `PS2_SONY_CODEC_INTERNAL* PS2_SONY_CODEC_INTERNAL_ctor(...)`
/// (0x3b0948): initialise the codec's queues/vtable pointer chain.
pub fn ps2_sony_codec_internal_ctor(c: &mut Cpu) {
    c.cycles += 22;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x45_0000;
    c.g[A2][0] = 0x3;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6850));
    self_.off(0x18).sw(c, c.g[A2][0]);
    self_.sw(c, c.g[V0][0]);
    c.g[V1][0] = sx(self_.0.wrapping_add(0x18));
    self_.off(0x18).sw(c, c.g[V1][0]);
    c.g[A1][0] = sx(self_.0.wrapping_add(0x20));
    self_.off(0x18).sw(c, c.g[V1][0]);
    c.g[V0][0] = c.g[A0][0];
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
    self_.off(0x18).sw(c, c.g[V1][0]);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
    self_.off(0x20).sw(c, c.g[A2][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
    self_.off(0x20).sw(c, c.g[A1][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    self_.off(0x20).sw(c, c.g[A1][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    self_.off(0x2c).sw(c, 0);
    self_.off(0x78).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// SONY codec frame queues.

/// `void* PS2_SONY_CODEC_INTERNAL_PopFreeFrame(...)` (0x3b10d0):
/// unlink and return the first free frame from the list head at +0x20, or null
/// when the list is empty.
pub fn ps2_sony_codec_internal_pop_free_frame(c: &mut Cpu) {
    c.cycles += 5;
    let self_ = Addr(abi::arg(c, 0));
    c.g[A2][0] = c.g[A0][0];
    c.g[T1][0] = self_.off(0x20).lw(c);
    c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(0x20));
    let empty = c.g[T1][0] == c.g[V0][0];
    c.g[A3][0] = 0xb;
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 20;
    c.g[A1][0] = Addr(c.g[T1][0] as u32).lw(c);
    c.g[V0][0] = Addr(c.g[T1][0] as u32).off(0x4).lw(c);
    c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub(8));
    if c.g[T1][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x18));
    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
    c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V0][0]);
    if c.g[A0][0] == 0 {
        c.g[V1][0] = 0;
    }
    c.sw((c.g[T1][0] as u32).wrapping_add(4), c.g[A3][0]);
    c.g[T0][0] = 1;
    c.sw(c.g[T1][0] as u32, c.g[A3][0]);
    c.g[V0][0] = c.g[A0][0];
    c.g[A1][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
    c.sw((c.g[A2][0] as u32).wrapping_add(4), c.g[V1][0]);
    c.sw(c.g[V1][0] as u32, c.g[A2][0]);
    c.sw((c.g[V1][0] as u32).wrapping_add(4), c.g[A1][0]);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// MPEG2 bit reader.

/// `uint MPEG2_Show_Bits(int n)` (0x3b1258): peek the top `n` bits of the bit
/// buffer: buffer word at `[0x510000-0x5f78] + 4` shifted right by `-n & 31`.
pub fn mpeg2_show_bits(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x51_0000;
    c.g[A0][0] = sx(0u32.wrapping_sub(abi::arg(c, 0)));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5f78));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> ((c.g[A0][0] as u32) & 31));
    abi::ret(c);
}

/// `void MPEG2_Update_Temporal_Reference_Tacking_Data(void)` (0x3b2440):
/// refresh the MPEG2 temporal-reference tracking counters.
pub fn mpeg2_update_temporal_reference_tacking_data(c: &mut Cpu) {
    'b_3b2520: {
        c.cycles += 6;
        c.g[V0][0] = 0x51_0000;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5f78));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x69b8));
        let t = c.g[A0][0] != c.g[V1][0];
        c.g[V0][0] = 0x51_0000;
        if t {
            break 'b_3b2520;
        } else {
            'b_3b24cc: {
                c.cycles += 6;
                c.g[A0][0] = 0x3;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x6aa8));
                c.g[A1][0] = 0x51_0000;
                c.g[A3][0] = 0x45_0000;
                let t = c.g[V1][0] == c.g[A0][0];
                c.g[A2][0] = 0x45_0000;
                if t {
                    break 'b_3b24cc;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = 0x45_0000;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x6aac));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x3c34));
                    let t = c.g[V1][0] == c.g[A0][0];
                    c.g[T0][0] = c.g[V0][0];
                    if t {
                        break 'b_3b24cc;
                    } else {
                        'b_3b249c: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x3c38));
                            let t = c.g[V0][0] == 0;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_sub(0x3c88));
                            if t {
                                break 'b_3b249c;
                            } else {
                                c.cycles += 3;
                                c.sw((c.g[A3][0] as u32).wrapping_sub(0x3c38), 0);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x400));
                                c.sw((c.g[A2][0] as u32).wrapping_sub(0x3c88), c.g[V0][0]);
                                break 'b_3b249c;
                            }
                        }
                        'b_3b24c4: {
                            'b_3b24c0: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x6aac));
                                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0;
                                c.g[V1][0] = 0x45_0000;
                                if t {
                                    break 'b_3b24c0;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x3c80));
                                    let t = c.g[V0][0] != 0;
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x6aac));
                                    if t {
                                        break 'b_3b24c4;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V0][0] = 0x1;
                                        c.sw((c.g[A3][0] as u32).wrapping_sub(0x3c38), c.g[V0][0]);
                                        break 'b_3b24c0;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x6aac));
                            break 'b_3b24c4;
                        }
                        c.cycles += 2;
                        c.sw((c.g[V1][0] as u32).wrapping_sub(0x3c80), 0);
                        c.sw((c.g[T0][0] as u32).wrapping_sub(0x3c34), c.g[V0][0]);
                        break 'b_3b24cc;
                    }
                }
            }
            'b_3b250c: {
                'b_3b2508: {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_sub(0x3c88));
                    c.g[V1][0] = 0x51_0000;
                    c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x6aac));
                    c.g[A2][0] = c.g[V1][0];
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x3c38));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    let t = c.g[V0][0] == 0;
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x5f68), c.g[A0][0]);
                    if t {
                        break 'b_3b2508;
                    } else {
                        c.cycles += 5;
                        c.g[V1][0] = 0x45_0000;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x3c34));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0;
                        c.g[A1][0] = 0x45_0000;
                        if t {
                            break 'b_3b250c;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x400));
                            c.sw((c.g[A2][0] as u32).wrapping_sub(0x5f68), c.g[V0][0]);
                            break 'b_3b2508;
                        }
                    }
                }
                c.cycles += 1;
                c.g[A1][0] = 0x45_0000;
                break 'b_3b250c;
            }
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_sub(0x5f68));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x3c84));
            c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[A0][0];
            }
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x3c84), c.g[V0][0]);
            break 'b_3b2520;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// SHP (image format) helpers.

/// `int SHP_PaletteEntriesForBpp(int bpp)` (0x3b2688): 16 for 4bpp, 256 for
/// 8bpp, 0 otherwise.
pub fn shp_palette_entries_for_bpp(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x4;
    let is4 = c.g[A0][0] == c.g[V0][0];
    c.g[V0][0] = 0x8;
    if is4 {
        c.cycles += 2;
        c.g[V0][0] = 0x10;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let is8 = c.g[A0][0] != c.g[V0][0];
    c.g[V0][0] = 0x100;
    if is8 {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 2;
    }
    abi::ret(c);
}

/// Bytes-per-pixel table used by `SHP_GetBpp`.
const SHP_BPP_TABLE: u32 = 0x0044_c3d0;

/// `int SHP_GetBpp(const SHPHeader* h)` (0x3b39d8): byte at the header's
/// `bpp` field indexed into the BPP table.
pub fn shp_get_bpp(c: &mut Cpu) {
    c.cycles += 7;
    let hdr = Addr(abi::arg(c, 0));
    c.g[V1][0] = hdr.lbu(c);
    c.g[V0][0] = 0x45_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3c30));
    c.g[V1][0] &= 0x7f;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lbu(c);
    abi::ret(c);
}

/// `char* SHPS_GetEntryName(SHPSheet* sheet, int index, char* out)`
/// (0x3b3d00): copy the entry's name pointer (or "" when out of range) into
/// `out` (an unaligned 4-byte sequence via `swr`/`swl`).
pub fn shps_get_entry_name(c: &mut Cpu) {
    c.cycles += 5;
    let sheet = abi::arg(c, 0);
    let out = c.g[A2][0];
    c.g[T1][0] = Addr(sheet).off(0x8).lw(c);
    c.g[T3][0] = sx((c.g[A1][0] as u32) << 3);
    c.g[AT][0] = (c.g[A1][0] < c.g[T1][0]) as u64;
    let out_of_range = c.g[AT][0] == 0;
    c.g[T3][0] = sx(sheet.wrapping_add(c.g[T3][0] as u32));
    if out_of_range {
        c.cycles += 5;
        c.g[V0][0] = 0;
        c.swr(V0, out as u32);
        c.swl(V0, (out as u32).wrapping_add(3));
        c.g[V0][0] = c.g[A2][0];
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V0][0] = Addr(c.g[T3][0] as u32).off(0x10).lw(c);
    c.swr(V0, out as u32);
    c.swl(V0, (out as u32).wrapping_add(3));
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

/// `int SHP_FormatFromBpp(int bpp)` (0x3b3da8): map a palette bit-depth to the
/// SHP texture format id, or 0 when unknown.
pub fn shp_format_from_bpp(c: &mut Cpu) {
    'b_3b3e38: {
        'b_3b3e34: {
            'b_3b3e30: {
                'b_3b3e28: {
                    'b_3b3e20: {
                        'b_3b3e18: {
                            'b_3b3dd8: {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A0][0] as i64) < 17) as u64;
                                let t = c.g[V0][0] == 0;
                                c.g[V1][0] = 0;
                                if t {
                                    break 'b_3b3dd8;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A0][0] as i64) < 15) as u64;
                                    let t = c.g[V0][0] == 0;
                                    c.g[V0][0] = 0x4;
                                    if t {
                                        break 'b_3b3e28;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[A0][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x8;
                                        if t {
                                            break 'b_3b3e18;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A0][0] == c.g[V0][0];
                                            c.g[V0][0] = c.g[V1][0];
                                            if t {
                                                break 'b_3b3e20;
                                            } else {
                                                c.cycles += 2;
                                                break 'b_3b3e38;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = 0x22b;
                            let t = c.g[A0][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[A0][0] as i64) < 556) as u64;
                            if t {
                                break 'b_3b3e28;
                            } else {
                                'b_3b3dfc: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0;
                                    c.g[V0][0] = 0x20;
                                    if t {
                                        break 'b_3b3dfc;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[A0][0] == c.g[V0][0];
                                        c.g[V0][0] = c.g[V1][0];
                                        if t {
                                            break 'b_3b3e30;
                                        } else {
                                            c.cycles += 2;
                                            break 'b_3b3e38;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x613;
                                let t = c.g[A0][0] == c.g[V0][0];
                                c.g[V0][0] = 0x22b8;
                                if t {
                                    break 'b_3b3e28;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A0][0] == c.g[V0][0];
                                    c.g[V0][0] = c.g[V1][0];
                                    if t {
                                        break 'b_3b3e30;
                                    } else {
                                        c.cycles += 2;
                                        break 'b_3b3e38;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[V1][0] = 0x1;
                        break 'b_3b3e34;
                    }
                    c.cycles += 2;
                    c.g[V1][0] = 0x2;
                    break 'b_3b3e34;
                }
                c.cycles += 2;
                c.g[V1][0] = 0x3;
                break 'b_3b3e34;
            }
            c.cycles += 1;
            c.g[V1][0] = 0x5;
            break 'b_3b3e34;
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[V1][0];
        break 'b_3b3e38;
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// SHP swizzle address helpers.

/// `uint SHP_SwizzleAddrPSMT4(uint x, uint y, uint base)` (0x3b42b0).
pub fn shp_swizzle_addr_psmt4(c: &mut Cpu) {
    c.cycles += 34;
    c.g[T0][0] = c.g[A1][0];
    c.g[T1][0] = c.g[A0][0];
    c.g[V1][0] = c.g[T1][0] & 0x60;
    c.g[V0][0] = c.g[T0][0] & 0xc;
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
    c.g[A0][0] = c.g[T0][0] & 0x1;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = sx(((c.g[T0][0] as u32) as i32 >> 7) as u32);
    c.g[A0][0] = c.g[T0][0] & 0x4;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
    c.g[A1][0] = c.g[T0][0] & 0x70;
    c.g[A3][0] = c.g[T0][0] & 0x2;
    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 1);
    c.mult(0, A2, T0, A2);
    c.g[V0][0] = c.g[T1][0] & 0x7;
    c.g[V0][0] = c.g[V0][0] ^ c.g[A3][0];
    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 1) as u32);
    c.g[V0][0] = c.g[V0][0] ^ c.g[A0][0];
    c.g[V1][0] = sx(((c.g[T1][0] as u32) as i32 >> 7) as u32);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
    c.g[A0][0] = sx((c.g[A2][0] as u32) >> 31);
    c.g[V0][0] = c.g[T1][0] & 0x18;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 1) as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T1][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 5);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `uint SHP_SwizzleAddrPSMT8(uint x, uint y, uint base)` (0x3b4338).
pub fn shp_swizzle_addr_psmt8(c: &mut Cpu) {
    c.cycles += 33;
    c.g[A3][0] = c.g[A1][0];
    c.g[T1][0] = c.g[A0][0];
    c.g[V1][0] = c.g[A3][0] & 0x3c;
    c.g[A0][0] = c.g[A3][0] & 0x1;
    c.g[V0][0] = sx(((c.g[A3][0] as u32) as i32 >> 6) as u32);
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[T0][0] = c.g[A3][0] & 0x2;
    c.g[A1][0] = c.g[A3][0] & 0x4;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = c.g[T1][0] & 0x7;
    c.mult(0, A2, A3, A2);
    c.g[V0][0] = sx((c.g[T0][0] as u32) << 1);
    c.g[A0][0] = c.g[A0][0] ^ c.g[V0][0];
    c.g[V1][0] = c.g[T1][0] & 0x70;
    c.g[A0][0] = c.g[A0][0] ^ c.g[A1][0];
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
    c.g[A1][0] = sx(((c.g[T1][0] as u32) as i32 >> 7) as u32);
    c.g[V0][0] = c.g[T1][0] & 0x8;
    c.g[A3][0] = sx((c.g[A2][0] as u32) >> 31);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
    c.g[T1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 1) as u32);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.g[T0][0] = sx((c.g[T0][0] as u32) << 2);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 5);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `uint SHP_SwizzleAddrPSMCT16(uint x, uint y, uint base)` (0x3b43c0).
pub fn shp_swizzle_addr_psmct16(c: &mut Cpu) {
    'b_3b4428: {
        'b_3b4410: {
            c.cycles += 15;
            c.g[A3][0] = c.g[A0][0];
            c.g[V1][0] = c.g[A1][0] & 0x7;
            c.g[V0][0] = c.g[A3][0] & 0x30;
            c.g[T2][0] = sx(((c.g[A1][0] as u32) as i32 >> 6) as u32);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.g[T0][0] = sx(((c.g[A3][0] as u32) as i32 >> 6) as u32);
            c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[T1][0] = sx(((c.g[A2][0] as u32) as i32 >> 7) as u32);
            c.g[A0][0] = c.g[A3][0] & 0x8;
            c.g[A1][0] = c.g[A1][0] & 0x38;
            c.g[V0][0] = c.g[A3][0] & 0x7;
            c.g[T4][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[V1][0] = ((c.g[T0][0] as i64) < (c.g[T1][0] as i64)) as u64;
            let t = c.g[V1][0] == 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            if t {
                break 'b_3b4410;
            } else {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[T0][0] as u32) << 6);
                c.g[V1][0] = sx((c.g[T2][0] as u32) << 6);
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T3][0] as u32));
                break 'b_3b4428;
            }
        }
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[T1][0] as u32));
        c.g[A0][0] = sx((c.g[T2][0] as u32) << 6);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
        c.g[V1][0] = sx((c.g[T3][0] as u32).wrapping_add(0x20));
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        break 'b_3b4428;
    }
    c.cycles += 8;
    c.mult(0, V0, A1, A2);
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
    c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
    abi::ret(c);
}

/// `uint SHP_SwizzleAddrPSMCT32(uint x, uint y, uint base)` (0x3b4448).
pub fn shp_swizzle_addr_psmct32(c: &mut Cpu) {
    c.cycles += 4;
    c.mult(0, V1, A1, A2);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Sound library (SND_*).

/// `void SND_AddTickCallback(void (*cb)())` (0x3b5910): append `cb` to the
/// per-tick callback table at 0x50a8e8 (only when the subsystem is enabled).
pub fn snd_add_tick_callback(c: &mut Cpu) {
    'b_3b5940: {
        c.cycles += 5;
        c.g[V1][0] = 0x51_0000;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x174).lb(c);
        if c.g[V0][0] == 0 {
            break 'b_3b5940;
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x177).lb(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x184), c.g[A0][0]);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x177).lbu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        Addr(c.g[A1][0] as u32).off(0x177).sb(c, c.g[V1][0]);
        break 'b_3b5940;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SND_AddEventCallback(void (*cb)())` (0x3b5a28): append `cb` to the
/// per-event callback table at 0x50a8e8.
pub fn snd_add_event_callback(c: &mut Cpu) {
    'b_3b5a58: {
        c.cycles += 5;
        c.g[V1][0] = 0x51_0000;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x174).lb(c);
        if c.g[V0][0] == 0 {
            break 'b_3b5a58;
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x179).lb(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c8), c.g[A0][0]);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x179).lbu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        Addr(c.g[A1][0] as u32).off(0x179).sb(c, c.g[V1][0]);
        break 'b_3b5a58;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SND_MemClampBlock(uint* addr, uint* size)` (0x3b5ba0): clamp `size`
/// so that `addr + size` does not pass the heap end.
pub fn snd_mem_clamp_block(c: &mut Cpu) {
    'b_3b5bc8: {
        c.cycles += 9;
        c.g[V0][0] = 0x51_0000;
        c.g[A0][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x54e4));
        c.g[V0][0] = Addr(abi::arg(c, 1)).lw(c);
        c.g[V1][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        if t {
            break 'b_3b5bc8;
        }
        c.cycles += 1;
        c.sw(abi::arg(c, 1), c.g[V0][0]);
        break 'b_3b5bc8;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SND_MemInitHeap(uintptr_t base, uint size)` (0x3b5bd0): carve a block
/// header out of `[base, base+size)` and link it into the heap list.
pub fn snd_mem_init_heap(c: &mut Cpu) {
    c.cycles += 28;
    let base = abi::arg(c, 0);
    let size = abi::arg(c, 1);
    c.g[A2][0] = 0x51_0000;
    c.g[V0][0] = sx(base.wrapping_add(size));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5718));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(8));
    c.sw((c.g[A2][0] as u32).wrapping_add(0x234), base as u64);
    c.g[A3][0] = 0xffff_ffff_ffff_0000;
    c.sw(base.wrapping_add(8), size as u64);
    c.g[A3][0] |= 0xfff0;
    c.g[A0][0] = sx(base.wrapping_add(0x18));
    c.g[A1][0] = sx(size.wrapping_sub(0xf));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x234));
    c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x20));
    c.sw((c.g[V1][0] as u32).wrapping_add(4), c.g[V0][0]);
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x234));
    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x234));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x234));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x234));
    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T0][0]);
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x234));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
    abi::ret(c);
}

/// `uint SND_Random(void)` (0x3bc5c8): xorshift-style PRNG state update at
/// 0x50ac50; returns the new state.
pub fn snd_random(c: &mut Cpu) {
    'b_3bc600: {
        'b_3bc5f8: {
            c.cycles += 9;
            c.g[V1][0] = 0x51_0000;
            c.g[T2][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x53b0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = (c.g[A3][0] < c.g[V0][0]) as u64;
            let t = c.g[V0][0] != 0;
            c.g[T1][0] = 0;
            if t {
                break 'b_3bc5f8;
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[A3][0] < c.g[A0][0]) as u64;
            let t = c.g[V0][0] == 0;
            c.g[T0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x53b0));
            if t {
                break 'b_3bc600;
            }
            break 'b_3bc5f8;
        }
        c.cycles += 2;
        c.g[T1][0] = 0x1;
        c.g[T0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x53b0));
        break 'b_3bc600;
    }
    'b_3bc6ac: {
        c.cycles += 24;
        c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_sub(0x53b0));
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw((c.g[T0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
        c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
        c.g[T1][0] = (c.g[A3][0] < c.g[V1][0]) as u64;
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.sw((c.g[T0][0] as u32).wrapping_add(0xc), c.g[A3][0]);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x14));
        c.g[T1][0] = (c.g[A3][0] < c.g[A1][0]) as u64;
        c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
        c.sw((c.g[T0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[T1][0] = (c.g[A3][0] < c.g[A2][0]) as u64;
        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T1][0] as u32));
        let t = c.g[V1][0] != 0;
        c.sw((c.g[T2][0] as u32).wrapping_sub(0x53b0), c.g[A3][0]);
        if t {
            break 'b_3bc6ac;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let t = c.g[V0][0] != 0;
        c.sw((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        if t {
            break 'b_3bc6ac;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let t = c.g[V0][0] != 0;
        c.sw((c.g[T0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        if t {
            break 'b_3bc6ac;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let t = c.g[V0][0] != 0;
        c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        if t {
            break 'b_3bc6ac;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let t = c.g[V0][0] != 0;
        c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        if t {
            break 'b_3bc6ac;
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        c.sw((c.g[T2][0] as u32).wrapping_sub(0x53b0), c.g[V0][0]);
        c.g[A3][0] = c.g[V0][0];
        break 'b_3bc6ac;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    abi::ret(c);
}

/// `void SNDENC_InstallDts(void)` (0x3bc930): publish the three default DTS
/// callbacks into the SND globals.
pub fn sndenc_install_dts(c: &mut Cpu) {
    c.cycles += 13;
    c.g[V0][0] = 0x3c_0000;
    c.g[V1][0] = 0x3c_0000;
    c.g[A0][0] = 0x3c_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1360));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x12d0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xf10));
    c.g[A1][0] = 0x51_0000;
    c.g[A2][0] = 0x51_0000;
    c.g[A3][0] = 0x51_0000;
    c.sw((c.g[A1][0] as u32).wrapping_sub(0x37c0), c.g[V0][0]);
    c.sw((c.g[A2][0] as u32).wrapping_sub(0x37bc), c.g[V1][0]);
    c.sw((c.g[A3][0] as u32).wrapping_sub(0x37b8), c.g[A0][0]);
    abi::ret(c);
}

/// `void SND_GetGroupVoiceRange(int group, int* lo, int* hi)` (0x3c0c18):
/// report the voice range of a group, or the global range for group 0.
pub fn snd_get_group_voice_range(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0] & 0x8;
    let t8 = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[A0][0] & 0x100;
    if !t8 {
        c.cycles += 5;
        c.sw(abi::arg(c, 1), 0);
        c.g[V0][0] = 0x51_0000;
        c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x56e8));
        c.sw(abi::arg(c, 2), c.g[V1][0]);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let t100 = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[A0][0] & 0x4;
    if !t100 {
        c.cycles += 9;
        c.g[V1][0] = 0x51_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lbu(c);
        c.sw(abi::arg(c, 1), c.g[V0][0]);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x2f).lbu(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lbu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sw(abi::arg(c, 2), c.g[V0][0]);
        abi::ret(c);
        return;
    }
    'b_3c0c88: {
        c.cycles += 2;
        let t4 = c.g[V0][0] == 0;
        c.g[V1][0] = 0x51_0000;
        if t4 {
            break 'b_3c0c88;
        }
        c.cycles += 7;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x2f).lbu(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lbu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sw(abi::arg(c, 1), c.g[V0][0]);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x17e).lh(c);
        c.sw(abi::arg(c, 2), c.g[A0][0]);
        break 'b_3c0c88;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SND_SetSpuReverbMode(int mode)` (0x3c3500): map an API reverb mode to
/// the SPU's internal reverb preset and program it.
pub fn snd_set_spu_reverb_mode(c: &mut Cpu) {
    'b_3c35a0: {
        'b_3c3518: {
            c.cycles += 4;
            c.g[A3][0] = c.g[A0][0];
            c.g[V0][0] = (c.g[A3][0] < 0x2u64) as u64;
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = 0x5;
            if t {
                break 'b_3c3518;
            }
            c.cycles += 2;
            c.g[A3][0] = 0;
            break 'b_3c35a0;
        }
        'b_3c3528: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0xa;
            if t {
                break 'b_3c3528;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x1;
            break 'b_3c35a0;
        }
        'b_3c3538: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0x14;
            if t {
                break 'b_3c3538;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x2;
            break 'b_3c35a0;
        }
        'b_3c3548: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0x1e;
            if t {
                break 'b_3c3548;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x3;
            break 'b_3c35a0;
        }
        'b_3c3558: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0x28;
            if t {
                break 'b_3c3558;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x4;
            break 'b_3c35a0;
        }
        'b_3c3568: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0x32;
            if t {
                break 'b_3c3568;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x5;
            break 'b_3c35a0;
        }
        'b_3c3578: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0x64;
            if t {
                break 'b_3c3578;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x6;
            break 'b_3c35a0;
        }
        'b_3c3588: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = 0x6e;
            if t {
                break 'b_3c3588;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x9;
            break 'b_3c35a0;
        }
        'b_3c3598: {
            c.cycles += 2;
            let t = c.g[A3][0] != c.g[V0][0];
            c.g[V1][0] = 0x7;
            if t {
                break 'b_3c3598;
            }
            c.cycles += 2;
            c.g[A3][0] = 0x8;
            break 'b_3c35a0;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A3][0] ^ 0x78;
        if c.g[V0][0] == 0 {
            c.g[A3][0] = c.g[V1][0];
        }
        break 'b_3c35a0;
    }
    c.cycles += 22;
    c.g[V0][0] = 0x45_0000;
    c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x22b8));
    c.g[A2][0] = 0x51_0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = 0x20_0000;
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5300));
    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcb0));
    c.g[V1][0] = 0x51_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 6) as u32);
    c.sh((c.g[A2][0] as u32).wrapping_add(0x1a9e), c.g[A0][0]);
    c.sb((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x224).lbu(c);
    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcb0));
    c.sb((c.g[A0][0] as u32).wrapping_add(0x11), c.g[V0][0]);
    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcb0));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x228).lbu(c);
    c.sb((c.g[A1][0] as u32).wrapping_add(0x12), c.g[V0][0]);
    abi::ret(c);
}

/// `void SND_SetSpuFxVolume(int flags)` (0x3c36c8): scale and store the SPU
/// effect volume (only when the global volume pointer is set and the effect
/// bit is present).
pub fn snd_set_spu_fx_volume(c: &mut Cpu) {
    'b_3c3720: {
        c.cycles += 5;
        c.g[V0][0] = 0x51_0000;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5300));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcb0));
        let t = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[A1][0] & 0x8;
        if t {
            break 'b_3c3720;
        }
        c.cycles += 2;
        let t = c.g[V0][0] == 0;
        c.g[V1][0] = 0x51_0000;
        if t {
            break 'b_3c3720;
        }
        c.cycles += 15;
        c.g[AT][0] = 0x46ff_0000;
        c.g[AT][0] |= 0xfe00;
        c.mtc1(AT, 1);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x54f8));
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
        c.mul_s(0, 0, 1);
        c.cvt_w_s(2, 0);
        c.mfc1(V0, 2);
        c.sh((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcb0));
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(V0, 1);
        c.sh((c.g[A0][0] as u32).wrapping_add(0x16), c.g[V0][0]);
        break 'b_3c3720;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SND_SpuMemClampBlock(uint* addr, uint* size)` (0x3c4af8): clamp the
/// requested SPU-memory block against the three size limits, adjusting both
/// the address and the size.
pub fn snd_spu_mem_clamp_block(c: &mut Cpu) {
    'b_3c4b30: {
        c.cycles += 7;
        c.g[V0][0] = 0x51_0000;
        c.g[A2][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5300));
        c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x1a9a).lhu(c);
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(abi::arg(c, 1)).lw(c);
            break 'b_3c4b30;
        }
        c.cycles += 7;
        c.sw(abi::arg(c, 0), c.g[V1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
        c.g[V0][0] = Addr(abi::arg(c, 1)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.sw(abi::arg(c, 1), c.g[V0][0]);
        c.g[A2][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[V0][0] = Addr(abi::arg(c, 1)).lw(c);
        break 'b_3c4b30;
    }
    'b_3c4b48: {
        c.cycles += 5;
        c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x1a9c).lhu(c);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
        if !t {
            c.cycles += 1;
            c.sw(abi::arg(c, 1), c.g[V0][0]);
        }
        break 'b_3c4b48;
    }
    'b_3c4b68: {
        c.cycles += 7;
        c.g[A0][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[V0][0] = Addr(abi::arg(c, 1)).lw(c);
        c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x1a9e).lhu(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        if !t {
            c.cycles += 1;
            c.sw(abi::arg(c, 1), c.g[V0][0]);
        }
        break 'b_3c4b68;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SNDI_SpuMemSetBase(int base, int end)` (0x3c4e50): program the SPU
/// memory window, deriving the clamped base from the requested address.
pub fn sndi_spu_mem_set_base(c: &mut Cpu) {
    'b_3c4eb4: {
        'b_3c4e78: {
            'b_3c4e68: {
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                let t = c.g[A0][0] != c.g[V0][0];
                c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(0x3f));
                if t {
                    break 'b_3c4e68;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x51_0000;
                c.g[A0][0] = 0x141;
                break 'b_3c4e78;
            }
            c.cycles += 4;
            c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
            c.g[V1][0] = ((c.g[A0][0] as i64) < 321) as u64;
            let t = c.g[V1][0] != 0;
            c.g[V0][0] = 0x51_0000;
            if t {
                break 'b_3c4eb4;
            }
            break 'b_3c4e78;
        }
        'b_3c4ec0: {
            'b_3c4e9c: {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5300));
                c.g[A2][0] = c.g[V0][0];
                c.g[V0][0] = u64::MAX;
                let t = c.g[A1][0] != c.g[V0][0];
                c.sh((c.g[V1][0] as u32).wrapping_add(0x1a9a), c.g[A0][0]);
                if t {
                    break 'b_3c4e9c;
                }
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5300));
                c.g[V0][0] = 0x8000;
                c.sh((c.g[V1][0] as u32).wrapping_add(0x1a9c), c.g[V0][0]);
                break 'b_3c4ec0;
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3f));
            c.g[V0][0] = 0x8000;
            c.g[A1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5300));
            if !t {
                break 'b_3c4eb4;
            }
            c.cycles += 1;
            c.sh((c.g[V0][0] as u32).wrapping_add(0x1a9c), c.g[A1][0]);
            break 'b_3c4ec0;
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0xffff_ffff_ffff_fff8;
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn world_block_pool_free_block_matches() {
        check(ssx3_ee::s13::f_003a7790, world_block_pool_free_block, 200, &|c, seed| {
            let mut r = Rng(0x1111 ^ seed as u64);
            scramble(c, &mut r);
            let pool = if seed % 3 == 0 { DATA } else { DATA + 0x100 };
            let block = if seed % 3 == 0 { DATA + 0x100 } else { DATA };
            c.g[A0][0] = pool as u64;
            c.g[A1][0] = block as u64;
            c.bus.write32(pool + 0x10, r.next_u32());
            c.bus.write32(pool + 0x8, r.next_u32());
            c.bus.write32(block + 0x10, r.next_u32());
        });
    }

    #[test]
    fn section_allocator_ctor_matches() {
        check(ssx3_ee::s13::f_003a77b0, section_allocator_ctor, 200, &|c, seed| {
            let mut r = Rng(0x2222 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        });
    }

    #[test]
    fn c_world_cache_request_hull_matches() {
        check_lohi(ssx3_ee::s13::f_003a8448, c_world_cache_request_hull, 200, &|c, seed| {
            let mut r = Rng(0x3333 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            let index = (r.next_u32() % 6) as u32;
            c.g[A1][0] = index as u64;
            let owner = DATA + 0x1000;
            c.bus.write32(GAME_GP + 0x2a74, owner);
            c.bus.write32(owner + 0x1c, r.next_u32());
            let slot = DATA + 0x3f0 + index * 0x18;
            let count = match seed % 6 {
                0 => 4u32,
                1 => 0,
                2 => 1,
                3 => r.next_u32() % 3 + 1,
                4 => 0xffff_ffff,
                _ => r.next_u32() | 0x8000_0000,
            };
            c.bus.write32(slot, count);
        });
    }

    #[test]
    fn c_world_cache_expire_hull_matches() {
        check_lohi(ssx3_ee::s13::f_003a84c8, c_world_cache_expire_hull, 200, &|c, seed| {
            let mut r = Rng(0x4444 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            let index = (r.next_u32() % 6) as u32;
            c.g[A1][0] = index as u64;
            let owner = DATA + 0x1000;
            c.bus.write32(GAME_GP + 0x2a74, owner);
            c.bus.write32(owner + 0x1c, r.next_u32());
            c.bus.write32(DATA + 0x1bf0, r.next_u32());
        });
    }

    #[test]
    fn c_world_cache_set_hull_pinned_matches() {
        check_lohi(ssx3_ee::s13::f_003a84f8, c_world_cache_set_hull_pinned, 200, &|c, seed| {
            let mut r = Rng(0x5555 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 6) as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        });
    }

    #[test]
    fn c_world_cache_is_hull_resident_matches() {
        check_lohi(ssx3_ee::s13::f_003a8618, c_world_cache_is_hull_resident, 200, &|c, seed| {
            let mut r = Rng(0x6666 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (r.next_u32() % 6) as u32;
            c.g[A1][0] = index as u64;
            let count = if seed % 3 == 0 { 3u32 } else { r.next_u32() };
            c.bus.write32(DATA + 0x3f0 + index * 0x18, count);
        });
    }

    #[test]
    fn c_world_cache_is_hull_free_matches() {
        check_lohi(ssx3_ee::s13::f_003a8638, c_world_cache_is_hull_free, 200, &|c, seed| {
            let mut r = Rng(0x7777 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (r.next_u32() % 6) as u32;
            c.g[A1][0] = index as u64;
            let count = if seed % 3 == 0 { 0u32 } else { r.next_u32() };
            c.bus.write32(DATA + 0x3f0 + index * 0x18, count);
        });
    }

    #[test]
    fn c_world_cache_set_hull_resident_matches() {
        check_lohi(ssx3_ee::s13::f_003a8650, c_world_cache_set_hull_resident, 200, &|c, seed| {
            let mut r = Rng(0x8888 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 6) as u64;
        });
    }

    #[test]
    fn c_world_cache_streamer_get_section_matches() {
        check(ssx3_ee::s13::f_003a9820, c_world_cache_streamer_get_section, 200, &|c, seed| {
            let mut r = Rng(0x9999 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (r.next_u32() % 8) as u64;
            c.g[A1][0] = index;
            let count = r.next_u32() % 8;
            let head = if seed % 5 == 0 { 0 } else { DATA + 0x100 };
            c.bus.write32(DATA + 0x4, head);
            c.bus.write32(DATA + 0x8, DATA + 0x1000);
            c.bus.write32(head + 0x8, count);
        });
    }

    #[test]
    fn c_world_cache_streamer_is_section_resident_matches() {
        check(
            ssx3_ee::s13::f_003a9858,
            c_world_cache_streamer_is_section_resident,
            200,
            &|c, seed| {
                let mut r = Rng(0xaaaa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (r.next_u32() % 8) as u32;
                c.g[A1][0] = index as u64;
                let count = r.next_u32() % 8;
                let head = if seed % 5 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(DATA + 0x4, head);
                c.bus.write32(head + 0x8, count);
                c.bus.write32(DATA + index * 8 + 0x14, if seed % 2 == 0 { 3 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn c_world_cache_streamer_is_section_unloaded_matches() {
        check(
            ssx3_ee::s13::f_003a9890,
            c_world_cache_streamer_is_section_unloaded,
            200,
            &|c, seed| {
                let mut r = Rng(0xbbbb ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (r.next_u32() % 8) as u32;
                c.g[A1][0] = index as u64;
                let count = r.next_u32() % 8;
                let head = if seed % 5 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(DATA + 0x4, head);
                c.bus.write32(head + 0x8, count);
                c.bus.write32(DATA + index * 8 + 0x14, if seed % 2 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn c_world_cache_streamer_get_hull_matches() {
        check_lohi(ssx3_ee::s13::f_003a9a98, c_world_cache_streamer_get_hull, 200, &|c, seed| {
            let mut r = Rng(0xcccc ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 8) as u64;
            c.bus.write32(DATA + 0x10, r.next_u32());
        });
    }

    #[test]
    fn c_world_cache_streamer_get_hull_count_matches() {
        check(ssx3_ee::s13::f_003a9ab0, c_world_cache_streamer_get_hull_count, 200, &|c, seed| {
            let mut r = Rng(0xdddd ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let owner = DATA + 0x100;
            c.bus.write32(DATA + 0x4, owner);
            c.bus.write32(owner + 0x10, r.next_u32());
        });
    }

    #[test]
    fn c_wscript_cache_get_data_matches() {
        check_lohi(ssx3_ee::s13::f_003aca38, c_wscript_cache_get_data, 200, &|c, seed| {
            let mut r = Rng(0xe001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (r.next_u32() % 8) as u32;
            c.g[A1][0] = index as u64;
            c.bus.write32(DATA + 0x4, DATA + 0x1000);
            c.bus.write32(DATA + 0x1000 + index * 0xc + 8, r.next_u32());
        });
    }

    #[test]
    fn t_wscript_cache_ctor_matches() {
        check(ssx3_ee::s13::f_003aca50, t_wscript_cache_ctor, 100, &|c, seed| {
            let mut r = Rng(0xe002 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn ssx_app_get_game_module_matches() {
        check(ssx3_ee::s13::f_003ad188, ssx_app_get_game_module, 200, &|c, seed| {
            let mut r = Rng(0xe003 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = DATA + 0x100;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, r.next_u32());
        });
    }

    #[test]
    fn game_module_get_occlusion_man_matches() {
        check(ssx3_ee::s13::f_003ad198, game_module_get_occlusion_man, 200, &|c, seed| {
            let mut r = Rng(0xe004 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = DATA + 0x100;
            let module = DATA + 0x200;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, module);
            c.bus.write32(module + 0x8c, r.next_u32());
        });
    }

    #[test]
    fn t_wscript_cache_get_table_entry_matches() {
        check(ssx3_ee::s13::f_003adc50, t_wscript_cache_get_table_entry, 200, &|c, seed| {
            let mut r = Rng(0xe005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (r.next_u32() % 8) as u32;
            c.g[A1][0] = index as u64;
            let ty = if seed % 3 == 0 { 3u16 } else { r.next_u32() as u16 };
            c.bus.write16(DATA, ty);
            let table = if seed % 4 == 0 { 0u32 } else { DATA + 0x100 };
            c.bus.write32(DATA + 0x8, table);
            c.bus.write32(table + 0x3c, DATA + 0x200);
            c.bus.write32(DATA + 0x200 + index * 4, r.next_u32());
        });
    }

    /// Shared setup for the four `tWScriptCache_Get*` field accessors.
    fn wscript_field_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0xe006 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        let ty = if seed % 3 == 0 { 3u16 } else { r.next_u32() as u16 };
        c.bus.write16(DATA, ty);
        let table = if seed % 4 == 0 { 0u32 } else { DATA + 0x100 };
        c.bus.write32(DATA + 0x8, table);
        for off in [0x20u32, 0x24, 0x28, 0x2c] {
            c.bus.write32(table + off, r.next_u32());
        }
    }

    #[test]
    fn t_wscript_cache_get_process_count_matches() {
        check(ssx3_ee::s13::f_003adce0, t_wscript_cache_get_process_count, 200, &wscript_field_setup);
    }

    #[test]
    fn t_wscript_cache_get_processes_matches() {
        check(ssx3_ee::s13::f_003add10, t_wscript_cache_get_processes, 200, &wscript_field_setup);
    }

    #[test]
    fn t_wscript_cache_get_mission_count_matches() {
        check(ssx3_ee::s13::f_003add40, t_wscript_cache_get_mission_count, 200, &wscript_field_setup);
    }

    #[test]
    fn t_wscript_cache_get_missions_matches() {
        check(ssx3_ee::s13::f_003add70, t_wscript_cache_get_missions, 200, &wscript_field_setup);
    }

    #[test]
    fn av_subtitle_init_matches() {
        check(ssx3_ee::s13::f_003b0340, av_subtitle_init, 100, &|c, seed| {
            let mut r = Rng(0xe007 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn rcmp_system_ctor_matches() {
        check(ssx3_ee::s13::f_003b0410, rcmp_system_ctor, 100, &|c, seed| {
            let mut r = Rng(0xe008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn av_chunk_init_matches() {
        check(ssx3_ee::s13::f_003b0720, av_chunk_init, 100, &|c, seed| {
            let mut r = Rng(0xe009 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn rcmp_decoder_init_callbacks_matches() {
        check(ssx3_ee::s13::f_003b0738, rcmp_decoder_init_callbacks, 100, &|c, seed| {
            let mut r = Rng(0xe00a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn rcmp_make_callbacks_matches() {
        check(ssx3_ee::s13::f_003b0758, rcmp_make_callbacks, 200, &|c, seed| {
            let mut r = Rng(0xe00b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[T0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        });
    }

    #[test]
    fn ps2_sony_codec_internal_ctor_matches() {
        check(ssx3_ee::s13::f_003b0948, ps2_sony_codec_internal_ctor, 200, &|c, seed| {
            let mut r = Rng(0xe00c ^ seed as u64);
            scramble(c, &mut r);
            let base = DATA + (seed % 4) * 8;
            c.g[A0][0] = base as u64;
        });
    }

    #[test]
    fn ps2_sony_codec_internal_pop_free_frame_matches() {
        check(
            ssx3_ee::s13::f_003b10d0,
            ps2_sony_codec_internal_pop_free_frame,
            200,
            &|c, seed| {
                let mut r = Rng(0xf001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                if seed % 3 == 0 {
                    // Empty list: the head points back at itself.
                    c.bus.write32(DATA + 0x20, DATA + 0x20);
                } else {
                    let head = DATA + 0x200;
                    c.bus.write32(DATA + 0x20, head);
                    c.bus.write32(head, DATA + 0x300); // lw(T1) = next
                    c.bus.write32(head + 4, DATA + 0x400); // lw(T1+4) = prev
                    c.bus.write32(DATA + 0x1c, DATA + 0x500); // lw(A2+4)
                    c.bus.write32(DATA + 0x208, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn mpeg2_show_bits_matches() {
        check(ssx3_ee::s13::f_003b1258, mpeg2_show_bits, 200, &|c, seed| {
            let mut r = Rng(0xf002 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | (r.next_u32() % 64) as u64;
            c.bus.write32(0x0050_a088, DATA + 0x100);
            c.bus.write32(DATA + 0x104, r.next_u32());
        });
    }

    #[test]
    fn mpeg2_update_temporal_reference_tacking_data_matches() {
        check(
            ssx3_ee::s13::f_003b2440,
            mpeg2_update_temporal_reference_tacking_data,
            200,
            &|c, seed| {
                let mut r = Rng(0xf003 ^ seed as u64);
                scramble(c, &mut r);
                for a in [
                    0x0050_9648u32,
                    0x0050_9558,
                    0x0050_9554,
                    0x0044_c3cc,
                    0x0044_c3c8,
                    0x0044_c378,
                    0x0044_c380,
                    0x0044_c37c,
                    0x0050_a098,
                ] {
                    c.bus.write32(a, r.next_u32());
                }
                // Exercise the first-word gate and the zero branches.
                c.bus.write32(0x0050_9648, if seed % 3 == 0 { 0x0050_9648 } else { r.next_u32() });
                c.bus.write32(0x0050_9554, if seed % 2 == 0 { 3 } else { r.next_u32() });
                c.bus.write32(0x0044_c3c8, if seed % 4 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(0x0044_c380, if seed % 5 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn shp_palette_entries_for_bpp_matches() {
        check(ssx3_ee::s13::f_003b2688, shp_palette_entries_for_bpp, 100, &|c, seed| {
            let mut r = Rng(0xf004 ^ seed as u64);
            scramble(c, &mut r);
            let v = match seed % 6 {
                0 => 4u32,
                1 => 8,
                2 => 0,
                3 => 16,
                4 => 0xffff_ffff,
                _ => r.next_u32(),
            };
            c.g[A0][0] = v as u64;
        });
    }

    #[test]
    fn shp_get_bpp_matches() {
        check(ssx3_ee::s13::f_003b39d8, shp_get_bpp, 200, &|c, seed| {
            let mut r = Rng(0xf005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let b = r.next_u32() as u8;
            c.bus.write8(DATA, b);
            c.bus.write8(SHP_BPP_TABLE + (b & 0x7f) as u32, r.next_u32() as u8);
        });
    }

    #[test]
    fn shps_get_entry_name_matches() {
        check(ssx3_ee::s13::f_003b3d00, shps_get_entry_name, 200, &|c, seed| {
            let mut r = Rng(0xf006 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (r.next_u32() % 8) as u32;
            c.g[A1][0] = index as u64;
            let count = r.next_u32() % 8;
            c.bus.write32(DATA + 8, count);
            c.bus.write32(DATA + index * 8 + 0x10, r.next_u32());
            let out = DATA + 0x300 + (seed % 4);
            c.g[A2][0] = out as u64;
            c.bus.write32(out & !3, r.next_u32());
        });
    }

    #[test]
    fn shp_format_from_bpp_matches() {
        check(ssx3_ee::s13::f_003b3da8, shp_format_from_bpp, 200, &|c, seed| {
            let mut r = Rng(0xf007 ^ seed as u64);
            scramble(c, &mut r);
            let v = match seed % 12 {
                0 => 0u32,
                1 => 4,
                2 => 8,
                3 => 15,
                4 => 16,
                5 => 17,
                6 => 32,
                7 => 0x22b,
                8 => 0x556,
                9 => 0x613,
                10 => 0x22b8,
                _ => r.next_u32(),
            };
            c.g[A0][0] = v as u64;
        });
    }

    #[test]
    fn shp_swizzle_addr_psmt4_matches() {
        check_lohi(ssx3_ee::s13::f_003b42b0, shp_swizzle_addr_psmt4, 200, &|c, seed| {
            let mut r = Rng(0xf008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn shp_swizzle_addr_psmt8_matches() {
        check_lohi(ssx3_ee::s13::f_003b4338, shp_swizzle_addr_psmt8, 200, &|c, seed| {
            let mut r = Rng(0xf009 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn shp_swizzle_addr_psmct16_matches() {
        check_lohi(ssx3_ee::s13::f_003b43c0, shp_swizzle_addr_psmct16, 200, &|c, seed| {
            let mut r = Rng(0xf00a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn shp_swizzle_addr_psmct32_matches() {
        check_lohi(ssx3_ee::s13::f_003b4448, shp_swizzle_addr_psmct32, 200, &|c, seed| {
            let mut r = Rng(0xf00b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn snd_add_tick_callback_matches() {
        check(ssx3_ee::s13::f_003b5910, snd_add_tick_callback, 200, &|c, seed| {
            let mut r = Rng(0x1001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            let base = 0x0050_a8e8u32;
            c.bus.write8(base + 0x174, if seed % 3 == 0 { 0 } else { 1 });
            let idx = if seed % 2 == 0 { r.next_u32() & 0x1f } else { r.next_u32() | 0x80 };
            c.bus.write8(base + 0x177, idx as u8);
        });
    }

    #[test]
    fn snd_add_event_callback_matches() {
        check(ssx3_ee::s13::f_003b5a28, snd_add_event_callback, 200, &|c, seed| {
            let mut r = Rng(0x1002 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            let base = 0x0050_a8e8u32;
            c.bus.write8(base + 0x174, if seed % 3 == 0 { 0 } else { 1 });
            let idx = if seed % 2 == 0 { r.next_u32() & 0x1f } else { r.next_u32() | 0x80 };
            c.bus.write8(base + 0x179, idx as u8);
        });
    }

    #[test]
    fn snd_mem_clamp_block_matches() {
        check(ssx3_ee::s13::f_003b5ba0, snd_mem_clamp_block, 200, &|c, seed| {
            let mut r = Rng(0x1003 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 4) as u64;
            let a0 = r.next_u32() & 0x7fff_ffff;
            let sz = r.next_u32() & 0x7fff_ffff;
            c.bus.write32(DATA, a0);
            c.bus.write32(DATA + 4, sz);
            let st = DATA + 0x100;
            c.bus.write32(0x0050_ab1c, st);
            let lim = if seed % 2 == 0 {
                a0.wrapping_add(sz).wrapping_sub(1)
            } else {
                a0.wrapping_add(sz).wrapping_add(1)
            };
            c.bus.write32(st + 0xc, lim);
        });
    }

    #[test]
    fn snd_mem_init_heap_matches() {
        check(ssx3_ee::s13::f_003b5bd0, snd_mem_init_heap, 200, &|c, seed| {
            let mut r = Rng(0x1004 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA + 0x200) as u64;
            c.g[A1][0] = (r.next_u32() % 0x1000) as u64;
            c.bus.write32(0x0050_ab1c, DATA + 0x400);
        });
    }

    #[test]
    fn snd_random_matches() {
        check(ssx3_ee::s13::f_003bc5c8, snd_random, 200, &|c, seed| {
            let mut r = Rng(0x1005 ^ seed as u64);
            scramble(c, &mut r);
            let t0 = 0x0050_ac50u32;
            c.bus.write32(t0, r.next_u32());
            for off in [0x4u32, 0x8, 0xc, 0x10, 0x14] {
                c.bus.write32(t0 + off, r.next_u32());
            }
            if seed % 4 == 0 {
                // Force a carry in the low word sum.
                c.bus.write32(t0 + 0x10, 0xffff_fff0);
                c.bus.write32(t0 + 0x14, 0xffff_fff0);
            }
            if seed % 3 == 0 {
                // Force every counter-increment branch to wrap.
                for off in [0x4u32, 0x8, 0xc, 0x10, 0x14] {
                    c.bus.write32(t0 + off, 0xffff_ffff);
                }
            }
        });
    }

    #[test]
    fn sndenc_install_dts_matches() {
        check(ssx3_ee::s13::f_003bc930, sndenc_install_dts, 50, &|c, seed| {
            let mut r = Rng(0x1006 ^ seed as u64);
            scramble(c, &mut r);
        });
    }

    #[test]
    fn snd_get_group_voice_range_matches() {
        check(ssx3_ee::s13::f_003c0c18, snd_get_group_voice_range, 200, &|c, seed| {
            let mut r = Rng(0x1007 ^ seed as u64);
            scramble(c, &mut r);
            let group = match seed % 8 {
                0 => 0u32,
                1 => 8,
                2 => 4,
                3 => 0x100,
                4 => 0x104,
                5 => 0x108,
                6 => 0x10c,
                _ => r.next_u32(),
            };
            c.g[A0][0] = group as u64;
            c.g[A1][0] = (DATA + 0x300) as u64;
            c.g[A2][0] = (DATA + 0x304) as u64;
            let base = 0x0050_a8e8u32;
            c.bus.write8(base + 0x2f, r.next_u32() as u8);
            c.bus.write8(base + 0x30, r.next_u32() as u8);
            c.bus.write16(base + 0x17e, r.next_u32() as u16);
            c.bus.write8(0x0050_a918, r.next_u32() as u8);
        });
    }

    #[test]
    fn snd_set_spu_reverb_mode_matches() {
        check(ssx3_ee::s13::f_003c3500, snd_set_spu_reverb_mode, 200, &|c, seed| {
            let mut r = Rng(0x1008 ^ seed as u64);
            scramble(c, &mut r);
            let mode = [0u32, 1, 5, 10, 20, 30, 40, 50, 100, 110, 120, 7, 99][(seed % 13) as usize];
            c.g[A0][0] = mode as u64;
            for k in 0..130u32 {
                c.bus.write32(0x0044_dd48 + k * 4, r.next_u32());
            }
            c.bus.write32(0x0050_b9b0, DATA + 0x400);
            c.bus.write8(0x0050_a8e8 + 0x224, r.next_u32() as u8);
            c.bus.write8(0x0050_a8e8 + 0x228, r.next_u32() as u8);
            c.bus.write16(0x0050_ad00 + 0x1a9e, r.next_u32() as u16);
        });
    }

    #[test]
    fn snd_set_spu_fx_volume_matches() {
        check(ssx3_ee::s13::f_003c36c8, snd_set_spu_fx_volume, 200, &|c, seed| {
            let mut r = Rng(0x1009 ^ seed as u64);
            scramble(c, &mut r);
            let flags = if seed % 3 == 0 { r.next_u32() & !8 } else { r.next_u32() | 8 };
            c.g[A1][0] = flags as u64;
            let ptr = if seed % 5 == 0 { 0 } else { DATA + 0x400 };
            c.bus.write32(0x0050_b9b0, ptr);
            c.bus.write32(0x0050_ab08 + 0xc, r.edge_f32());
            c.bus.write32(0x0050_ab08 + 0x10, r.edge_f32());
        });
    }

    #[test]
    fn snd_spu_mem_clamp_block_matches() {
        check(ssx3_ee::s13::f_003c4af8, snd_spu_mem_clamp_block, 200, &|c, seed| {
            let mut r = Rng(0x100a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 4) as u64;
            let a0 = r.next_u32() & 0xffff;
            let sz = r.next_u32() & 0xffff;
            c.bus.write32(DATA, a0);
            c.bus.write32(DATA + 4, sz);
            c.bus.write16(0x0050_ad00 + 0x1a9a, r.next_u32() as u16);
            c.bus.write16(0x0050_ad00 + 0x1a9c, r.next_u32() as u16);
            c.bus.write16(0x0050_ad00 + 0x1a9e, r.next_u32() as u16);
        });
    }

    #[test]
    fn sndi_spu_mem_set_base_matches() {
        check(ssx3_ee::s13::f_003c4e50, sndi_spu_mem_set_base, 200, &|c, seed| {
            let mut r = Rng(0x100b ^ seed as u64);
            scramble(c, &mut r);
            let a0 = match seed % 5 {
                0 => 0xffff_ffff_ffff_ffffu64,
                1 => 0,
                2 => 0x1234,
                3 => 0x7fff,
                _ => (r.next_u32() as u64) | ((r.next_u32() as u64) << 32),
            };
            c.g[A0][0] = a0;
            let a1 = match seed % 4 {
                0 => 0xffff_ffff_ffff_ffffu64,
                1 => 0,
                2 => 0x1_0000,
                _ => (r.next_u32() as u64) | ((r.next_u32() as u64) << 32),
            };
            c.g[A1][0] = a1;
            c.bus.write16(0x0050_ad00 + 0x1a9a, r.next_u32() as u16);
            c.bus.write16(0x0050_ad00 + 0x1a9c, r.next_u32() as u16);
        });
    }
}
