//! `src/world/wscriptcache.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_4;
pub(crate) use crate::game::dirtysock::tagsunk::gp_3 as gp;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::gp_12 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_9 as lw_6;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_3;
pub(crate) use crate::game::fe::fepopup::at as at_5;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_7;
pub(crate) use crate::game::fe::fepopup::sw_4;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::festaterewards::lw_3;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_2;
pub(crate) use crate::game::object::flexrailnode::r32v;
pub(crate) use crate::game::object::flexrailnode::spa;
pub(crate) use crate::game::object::railmodifier::reset_vtable_and_delete;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::world::wscriptcache::callee::AV_PLAYER_INIT;
pub(crate) use crate::game::world::wscriptcache::callee::RCMP_DECODER_RELEASE_CODEC;
pub(crate) use crate::game::world::wscriptcache::callee::STREAM_FILE_RELEASE_CHUNK;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003b_0b40,
        ps2_sony_codec_internal_feed_bitstream,
        "PS2_SONY_CODEC_INTERNAL_FeedBitstream",
    ),
    (0x003a_c800, cwscriptcache_alloc, "cWScriptCache_Alloc"),
    (
        0x003a_ca10,
        cwscript_cache_set_data,
        "cWScriptCache_SetData",
    ),
    (
        0x003a_ca38,
        c_wscript_cache_get_data,
        "cWScriptCache_GetData",
    ),
    (0x003a_ca50, t_wscript_cache_ctor, "tWScriptCache_ctor"),
    (0x003a_ca70, wscript_cache_dtor, "tWScriptCache_dtor"),
    (0x003a_cc50, wscript_cache_set_data, "tWScriptCache_SetData"),
    (0x003a_cc90, t_w_script_cache_free, "tWScriptCache_Free"),
    (0x003a_d188, ssx_app_get_game_module, "SSXApp_GetGameModule"),
    (
        0x003a_d198,
        game_module_get_occlusion_man,
        "GameModule_GetOcclusionMan",
    ),
    (0x003a_d1a8, world_reset_lights, "World_ResetLights"),
    (
        0x003a_dc18,
        c_ps2_chunk_resolve_base_dtor,
        "cPS2ChunkResolve_BaseDtor",
    ),
    (
        0x003a_dc50,
        t_wscript_cache_get_table_entry,
        "tWScriptCache_GetTableEntry",
    ),
    (
        0x003a_dce0,
        t_wscript_cache_get_process_count,
        "tWScriptCache_GetProcessCount",
    ),
    (
        0x003a_dd10,
        t_wscript_cache_get_processes,
        "tWScriptCache_GetProcesses",
    ),
    (
        0x003a_dd40,
        t_wscript_cache_get_mission_count,
        "tWScriptCache_GetMissionCount",
    ),
    (
        0x003a_dd70,
        t_wscript_cache_get_missions,
        "tWScriptCache_GetMissions",
    ),
    (0x003a_de08, av_player_ctor_2, "AV_Player_ctor_2"),
    (0x003a_e6c8, av_player_decode_frame, "AV_Player_DecodeFrame"),
    (
        0x003a_e718,
        av_player_get_current_subtitle,
        "AV_Player_GetCurrentSubtitle",
    ),
    (
        0x003a_e860,
        av_player_is_audio_done,
        "AV_Player_IsAudioDone",
    ),
    (0x003a_e888, av_player_set_rate, "AV_Player_SetRate"),
    (0x003a_e938, av_player_pause, "AV_Player_Pause"),
    (0x003a_e958, av_player_resume, "AV_Player_Resume"),
    (0x003a_e978, av_player_set_volume, "AV_Player_SetVolume"),
    (
        0x003a_ece0,
        av_player_release_chunk,
        "AV_Player_ReleaseChunk",
    ),
    (
        0x003a_ed20,
        av_player_release_chunk_callback,
        "AV_Player_ReleaseChunkCallback",
    ),
    (0x003a_ed40, av_audio_init, "AV_Audio_Init"),
    (0x003a_ee30, av_audio_dtor, "AV_Audio_dtor"),
    (0x003a_ee98, av_audio_set_pitch, "AV_Audio_SetPitch"),
    (0x003a_eef8, av_audio_start, "AV_Audio_Start"),
    (0x003a_ef48, av_audio_set_volume, "AV_Audio_SetVolume"),
    (0x003a_efa8, av_audio_is_done, "AV_Audio_IsDone"),
    (0x003a_f020, av_free_subtitles, "AV_FreeSubtitles"),
    (
        0x003a_f258,
        av_find_subtitle_by_frame,
        "AV_FindSubtitleByFrame",
    ),
    (0x003a_f300, av_skip_whitespace, "AV_SkipWhitespace"),
    (0x003a_f378, av_str_icmp, "AV_StrICmp"),
    (0x003b_0340, av_subtitle_init, "AV_SubtitleInit"),
    (0x003b_0410, rcmp_system_ctor, "RCMP_SYSTEM_ctor"),
    (0x003b_0430, rcmp_decoder_ctor, "RCMP_DECODER_ctor"),
    (0x003b_04a0, rcmp_decoder_dtor, "RCMP_DECODER_dtor"),
    (0x003b_04f8, rcmp_decoder_set_codec, "RCMP_DECODER_SetCodec"),
    (
        0x003b_0538,
        rcmp_decoder_release_codec,
        "RCMP_DECODER_ReleaseCodec",
    ),
    (
        0x003b_0580,
        rcmp_decoder_get_frame_number,
        "RCMP_DECODER_GetFrameNumber",
    ),
    (
        0x003b_05c0,
        rcmp_decoder_get_frame_rate,
        "RCMP_DECODER_GetFrameRate",
    ),
    (
        0x003b_0600,
        rcmp_decoder_decode_frame,
        "RCMP_DECODER_DecodeFrame",
    ),
    (0x003b_06b0, rcmp_decoder_get_chunk, "RCMP_DECODER_GetChunk"),
    (0x003b_0720, av_chunk_init, "AV_ChunkInit"),
    (
        0x003b_0738,
        rcmp_decoder_init_callbacks,
        "RCMP_DECODER_InitCallbacks",
    ),
    (0x003b_0758, rcmp_make_callbacks, "RCMP_MakeCallbacks"),
    (
        0x003b_0770,
        rcmp_static_init_destroy,
        "RCMP_StaticInitDestroy",
    ),
    (
        0x003b_08e0,
        ps2_sony_codec_internal_free_frame,
        "PS2_SONY_CODEC_INTERNAL_FreeFrame",
    ),
    (
        0x003b_0948,
        ps2_sony_codec_internal_ctor,
        "PS2_SONY_CODEC_INTERNAL_ctor",
    ),
    (
        0x003b_0b10,
        ps2_sony_codec_internal_nodata_callback,
        "PS2_SONY_CODEC_INTERNAL_NodataCallback",
    ),
    (
        0x003b_0fa8,
        ps2_sony_codec_internal_v06,
        "PS2_SONY_CODEC_INTERNAL_v06",
    ),
    (
        0x003b_0fb8,
        ps2_sony_codec_internal_get_picture,
        "PS2_SONY_CODEC_INTERNAL_GetPicture",
    ),
    (
        0x003b_1050,
        ps2_sony_codec_internal_decode_frame,
        "PS2_SONY_CODEC_INTERNAL_DecodeFrame",
    ),
    (
        0x003b_10d0,
        ps2_sony_codec_internal_pop_free_frame,
        "PS2_SONY_CODEC_INTERNAL_PopFreeFrame",
    ),
    (
        0x003b_1140,
        ps2_sony_codec_internal_release_frame,
        "PS2_SONY_CODEC_INTERNAL_ReleaseFrame",
    ),
    (
        0x003b_11a0,
        ps2_sony_codec_internal_create,
        "PS2_SONY_CODEC_INTERNAL_Create",
    ),
    (0x003b_11f0, rcmp_is_mpeg_chunk_tag, "RCMP_IsMpegChunkTag"),
    (
        0x003b_1228,
        mpeg2_initialize_buffer,
        "MPEG2_Initialize_Buffer",
    ),
    (0x003b_1258, mpeg2_show_bits, "MPEG2_Show_Bits"),
    (0x003b_1290, mpeg2_flush_buffer, "MPEG2_Flush_Buffer"),
    (0x003b_1300, mpeg2_get_bits, "MPEG2_Get_Bits"),
    (0x003b_1340, mpeg2_flush_buffer32, "MPEG2_Flush_Buffer32"),
    (0x003b_13a8, mpeg2_get_bits32, "MPEG2_Get_Bits32"),
    (
        0x003b_1ab0,
        mpeg2_sequence_extension,
        "MPEG2_sequence_extension",
    ),
    (
        0x003b_1fc8,
        mpeg2_picture_display_extension,
        "MPEG2_picture_display_extension",
    ),
    (
        0x003b_2230,
        mpeg2_picture_spatial_scalable_extension,
        "MPEG2_picture_spatial_scalable_extension",
    ),
    (
        0x003b_22f8,
        mpeg2_picture_temporal_scalable_extension,
        "MPEG2_picture_temporal_scalable_extension",
    ),
    (
        0x003b_2318,
        mpeg2_extra_bit_information,
        "MPEG2_extra_bit_information",
    ),
    (
        0x003b_2440,
        mpeg2_update_temporal_reference_tacking_data,
        "MPEG2_Update_Temporal_Reference_Tacking_Data",
    ),
    (
        0x003b_2688,
        shp_palette_entries_for_bpp,
        "SHP_PaletteEntriesForBpp",
    ),
    (0x003b_26b8, shp_calc_alloc_size, "SHP_CalcAllocSize"),
    (0x003b_2b68, shp_free, "SHP_Free"),
    (
        0x003b_2e78,
        shp_decode_dxt_color_block,
        "SHP_DecodeDXTColorBlock",
    ),
    (0x003b_39d8, shp_get_bpp, "SHP_GetBpp"),
    (0x003b_3da8, shp_format_from_bpp, "SHP_FormatFromBpp"),
    (0x003b_42b0, shp_swizzle_addr_psmt4, "SHP_SwizzleAddrPSMT4"),
    (0x003b_4338, shp_swizzle_addr_psmt8, "SHP_SwizzleAddrPSMT8"),
    (
        0x003b_43c0,
        shp_swizzle_addr_psmct16,
        "SHP_SwizzleAddrPSMCT16",
    ),
    (
        0x003b_4448,
        shp_swizzle_addr_psmct32,
        "SHP_SwizzleAddrPSMCT32",
    ),
    (
        0x003a_d120,
        wscript_cache_get_object_slot,
        "tWScriptCache_GetObjectSlot",
    ),
    (
        0x003a_ccd8,
        t_wscript_cache_relocate,
        "tWScriptCache_Relocate",
    ),
    (
        0x003a_d230,
        world_chunk_resolve_hook,
        "World_ChunkResolveHook",
    ),
    (
        0x003a_dc80,
        t_w_script_cache_get_global_count,
        "tWScriptCache_GetGlobalCount",
    ),
    (
        0x003a_dcb0,
        w_script_cache_get_globals,
        "tWScriptCache_GetGlobals",
    ),
    (0x003a_c8f0, cwscript_cache_dtor, "cWScriptCache_dtor"),
    (0x003a_c980, cwscript_cache_update, "cWScriptCache_Update"),
    (0x003a_c9e0, cwscript_cache_load, "cWScriptCache_Load"),
    (
        0x003a_cad0,
        twscript_cache_poll_load,
        "tWScriptCache_PollLoad",
    ),
    (0x003a_cbb8, twscript_cache_load, "tWScriptCache_Load"),
    (0x003a_d250, world_return_self, "World_ReturnSelf"),
    (
        0x003a_d290,
        world_static_init_destroy,
        "World_StaticInitDestroy",
    ),
    (0x003a_dda0, world_global_ctors, "World_GlobalCtors"),
    (0x003b_2528, shp_normalize_bpp, "SHP_NormalizeBpp"),
    (0x003b_2558, shp_calc_image_size, "SHP_CalcImageSize"),
    (0x003b_27c8, shp_init_header, "SHP_InitHeader"),
    (0x003b_2a68, shp_create, "SHP_Create"),
    (
        0x003b_2b90,
        shp_decode_dxt_alpha_block,
        "SHP_DecodeDXTAlphaBlock",
    ),
    (0x003b_3308, shp_dxt_alpha_is_opaque, "SHP_DXTAlphaIsOpaque"),
    (0x003b_33b0, shp_decode_dxt, "SHP_DecodeDXT"),
    (0x003b_3578, shp_unpack, "SHP_Unpack"),
    (
        0x003b_38b8,
        shp_palette_format_for_bpp,
        "SHP_PaletteFormatForBpp",
    ),
    (0x003b_3a10, shp_find_long_name, "SHP_FindLongName"),
    (
        0x003b_3a50,
        shp_calc_pixel_data_size,
        "SHP_CalcPixelDataSize",
    ),
    (0x003b_3bb0, shp_get_block_size, "SHP_GetBlockSize"),
    (0x003b_3d40, shp_find_info_data, "SHP_FindInfoData"),
    (0x003b_4010, shp_get_pixel_ptr, "SHP_GetPixelPtr"),
    (0x003b_4218, shp_swizzle_addr, "SHP_SwizzleAddr"),
    (0x003b_4478, shp_calc_total_size, "SHP_CalcTotalSize"),
    (0x003b_4540, shp_relocate, "SHP_Relocate"),
    (0x003b_4690, shp_clone, "SHP_Clone"),
    (0x003b_4708, shp_is_palette_format, "SHP_IsPaletteFormat"),
    (0x003b_3e40, shp_calc_pixel_offset, "SHP_CalcPixelOffset"),
    (
        0x003b_09a0,
        ps2_sony_codec_internal_dtor,
        "PS2_SONY_CODEC_INTERNAL_dtor",
    ),
    (
        0x003b_0c58,
        ps2_sony_codec_internal_open,
        "PS2_SONY_CODEC_INTERNAL_Open",
    ),
    (0x003a_ddc0, av_player_ctor, "AV_Player_ctor"),
    (0x003a_de58, av_player_ctor_3, "AV_Player_ctor_3"),
    (0x003a_de90, av_player_ctor_4, "AV_Player_ctor_4"),
    (0x003a_dec8, av_player_init, "AV_Player_Init"),
    (0x003a_e300, av_player_shutdown, "AV_Player_Shutdown"),
    (0x003a_e450, av_player_start, "AV_Player_Start"),
    (0x003a_e780, av_player_is_frame_due, "AV_Player_IsFrameDue"),
    (0x003a_e9a0, av_player_get_time_ms, "AV_Player_GetTimeMs"),
    (0x003a_ead0, av_player_on_chunk, "AV_Player_OnChunk"),
    (
        0x003a_ecb8,
        av_player_get_chunk_callback,
        "AV_Player_GetChunkCallback",
    ),
    (0x003a_f0c8, av_create_subtitles, "AV_CreateSubtitles"),
    (0x003a_f428, av_parse_justify, "AV_ParseJustify"),
    (0x003a_f560, av_parse_int_field, "AV_ParseIntField"),
    (0x003a_f670, av_find_token, "AV_FindToken"),
    (0x003a_f7f0, av_parse_subtitles, "AV_ParseSubtitles"),
    (0x003b_03a8, av_subtitle_set_x, "AV_SubtitleSetX"),
    (0x003b_03b0, av_subtitle_set_y, "AV_SubtitleSetY"),
    (0x003b_03b8, av_subtitle_set_width, "AV_SubtitleSetWidth"),
    (0x003b_03c0, av_subtitle_set_height, "AV_SubtitleSetHeight"),
    (
        0x003b_03c8,
        av_subtitle_set_string_id,
        "AV_SubtitleSetStringId",
    ),
    (0x003b_03d0, av_subtitle_set_frame, "AV_SubtitleSetFrame"),
    (
        0x003b_03d8,
        av_subtitle_set_justify_x,
        "AV_SubtitleSetJustifyX",
    ),
    (
        0x003b_03e0,
        av_subtitle_set_justify_y,
        "AV_SubtitleSetJustifyY",
    ),
    (0x003b_03e8, av_subtitle_set_shape, "AV_SubtitleSetShape"),
    (0x003b_03f0, av_subtitle_set_color0, "AV_SubtitleSetColor0"),
    (0x003b_03f8, av_subtitle_set_color1, "AV_SubtitleSetColor1"),
    (0x003b_0400, av_subtitle_set_color2, "AV_SubtitleSetColor2"),
    (0x003b_0408, av_subtitle_set_color3, "AV_SubtitleSetColor3"),
    (0x003b_11e8, mpeg2_error, "MPEG2_Error"),
    (0x003b_1270, mpeg2_get_bits1, "MPEG2_Get_Bits1"),
    (0x003b_13d8, mpeg2_get_hdr, "MPEG2_Get_Hdr"),
    (0x003b_1498, mpeg2_next_start_code, "MPEG2_next_start_code"),
    (0x003b_14f0, mpeg2_sequence_header, "MPEG2_sequence_header"),
    (0x003b_17a8, mpeg2_picture_header, "MPEG2_picture_header"),
    (0x003b_1878, mpeg2_slice_header, "MPEG2_slice_header"),
    (
        0x003b_1988,
        mpeg2_extension_and_user_data,
        "MPEG2_extension_and_user_data",
    ),
    (
        0x003b_1cc0,
        mpeg2_sequence_display_extension,
        "MPEG2_sequence_display_extension",
    ),
    (
        0x003b_1d58,
        mpeg2_quant_matrix_extension,
        "MPEG2_quant_matrix_extension",
    ),
    (
        0x003b_1ee0,
        mpeg2_sequence_scalable_extension,
        "MPEG2_sequence_scalable_extension",
    ),
    (
        0x003b_20b8,
        mpeg2_picture_coding_extension,
        "MPEG2_picture_coding_extension",
    ),
    (0x003b_2360, mpeg2_marker_bit, "MPEG2_marker_bit"),
    (0x003b_2380, mpeg2_user_data, "MPEG2_user_data"),
    (
        0x003b_23a0,
        mpeg2_copyright_extension,
        "MPEG2_copyright_extension",
    ),
    (
        0x003a_dc48,
        c_ps2_chunk_resolve_v08_003adc48,
        "cPS2ChunkResolve_v08",
    ),
    (0x003b_0680, rcmp_decoder_reset, "RCMP_DECODER_Reset"),
    (
        0x003b_06f8,
        rcmp_decoder_release_chunk,
        "RCMP_DECODER_ReleaseChunk",
    ),
    (0x003b_07b8, rcmp_global_ctors, "RCMP_GlobalCtors"),
    (0x003b_07d8, rcmp_global_dtors, "RCMP_GlobalDtors"),
    (
        0x003b_0fa0,
        ps2_sony_codec_internal_v05_003b0fa0,
        "PS2_SONY_CODEC_INTERNAL_v05",
    ),
    (
        0x003b_16e0,
        mpeg2_group_of_pictures_header_2,
        "MPEG2_group_of_pictures_header",
    ),
    (0x0042_e278, rcmp_codec_dtor, "RCMP_CODEC_dtor"),
    (0x0042_e498, rcmp_system_dtor, "RCMP_SYSTEM_dtor"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::ai::ai::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_FLUSH_BUFFER32;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_SHOW_BITS;

    pub(crate) use crate::game::ai::rider::callee::CODEC_POP_FREE_FRAME;

    pub(crate) use crate::game::ai::rider::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_MARKER_BIT;

    pub(crate) use crate::game::be::bedbprofile::callee::SHP_CALC_IMAGE_SIZE;
    pub(crate) use crate::game::be::bedbprofile::callee::SHP_NORMALIZE_BPP;
    pub(crate) use crate::game::be::bedbprofile::callee::SHP_PALETTE_ENTRIES_FOR_BPP;

    pub(crate) use crate::game::be::beintmission::callee::MPEG2_ERROR;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::FLOAT_TO_UINT;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::RCMP_FRAME_NUMBER;

    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;

    pub(crate) use crate::game::fe::fepopup::callee::C003B1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003B2360;

    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_CACHE_SET_DATA;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::TWSCRIPT_CACHE_RELOCATE;

    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER as MPEG2_FLUSH_BUFFER_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS as MPEG2_SHOW_BITS_2;

    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;

    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE;

    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;

    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;

    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;

    pub const SCRIPT_SOUND_BANK_MAN_FIND_BANK: Callee = Callee {
        addr: 0x0028_36d8,
        f: ee::s07::f_002836d8,
    };

    pub const SSX_AUDIO_SYSTEM_PAUSE: Callee = Callee {
        addr: 0x0029_ce28,
        f: ee::s08::f_0029ce28,
    };

    pub const BANK_MONITOR_DELETE_INSTANCE: Callee = Callee {
        addr: 0x002a_d5f0,
        f: ee::s08::f_002ad5f0,
    };

    pub const BELIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };

    pub const BELIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };

    pub const BE_NEW_RACE_INT_GET_TRACK_INFO: Callee = Callee {
        addr: 0x0014_4bc0,
        f: ee::s01::f_00144bc0,
    };

    pub const SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5: Callee = Callee {
        addr: 0x002a_3fd8,
        f: ee::s08::f_002a3fd8,
    };

    pub const CROWD_AUDIO_FREE_SAMPLE_LIST: Callee = Callee {
        addr: 0x002a_4cf8,
        f: ee::s08::f_002a4cf8,
    };

    pub const CUT_SCENE_TRANSITION_MAN_BASE_CTOR: Callee = Callee {
        addr: 0x0035_4648,
        f: ee::s11::f_00354648,
    };

    pub const CUT_SCENE_TRANSITION_MAN_INIT: Callee = Callee {
        addr: 0x002e_aa28,
        f: ee::s09::f_002eaa28,
    };

    pub const WSCRIPT_MAN_COLLECT_SET_GET_COUNT: Callee = Callee {
        addr: 0x0030_c468,
        f: ee::s10::f_0030c468,
    };

    pub const WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS: Callee = Callee {
        addr: 0x0030_c4a8,
        f: ee::s10::f_0030c4a8,
    };

    pub const EA_MEM_FREE_2: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };

    pub const MDF_FILE_FREE: Callee = Callee {
        addr: 0x0031_49d0,
        f: ee::s10::f_003149d0,
    };

    pub const SPRINTF_2: Callee = Callee {
        addr: 0x0041_7828,
        f: ee::s15::f_00417828,
    };

    pub const INPUT_PREPROC_WRITE_STRING: Callee = Callee {
        addr: 0x0032_6308,
        f: ee::s10::f_00326308,
    };

    pub const CINSTANCE_NODE_CTOR_V: Callee = Callee {
        addr: 0x0034_fb90,
        f: ee::s11::f_0034fb90,
    };

    pub const FONT_FREE: Callee = Callee {
        addr: 0x0039_19e8,
        f: ee::s12::f_003919e8,
    };

    pub const UI_ANIM_APPLY: Callee = Callee {
        addr: 0x0039_75e0,
        f: ee::s13::f_003975e0,
    };

    pub const UI_ANIM_ADVANCE: Callee = Callee {
        addr: 0x0039_74b0,
        f: ee::s13::f_003974b0,
    };

    pub const SECTION_ALLOCATOR_FREE_ALL: Callee = Callee {
        addr: 0x003a_7818,
        f: ee::s13::f_003a7818,
    };

    pub const C_WORLD_CACHE_STREAMER_DTOR_STUB: Callee = Callee {
        addr: 0x003a_9180,
        f: ee::s13::f_003a9180,
    };

    pub const RCMP_SYSTEM_CTOR: Callee = Callee {
        addr: 0x003b_0410,
        f: ee::s13::f_003b0410,
    };

    pub const PS2_SONY_CODEC_INTERNAL_CTOR: Callee = Callee {
        addr: 0x003b_0948,
        f: ee::s13::f_003b0948,
    };

    pub const MPEG2_GET_BITS1: Callee = Callee {
        addr: 0x003b_1270,
        f: ee::s13::f_003b1270,
    };

    pub const MPEG2_FLUSH_BUFFER_3: Callee = Callee {
        addr: 0x003b_1290,
        f: ee::s13::f_003b1290,
    };

    pub const RIDER_SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fec8,
        f: ee::s00::f_0011fec8,
    };

    pub const RIDER_SET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe78,
        f: ee::s00::f_0011fe78,
    };

    pub const BE_SCORE_INT_GET_HIGH_SCORE_TABLE: Callee = Callee {
        addr: 0x0014_ab20,
        f: ee::s01::f_0014ab20,
    };

    pub const CHASE_CAMERA_ALGORITHM_CTOR_2: Callee = Callee {
        addr: 0x0016_2318,
        f: ee::s01::f_00162318,
    };

    pub const ACTIVE_CAMERA_TRIGGER_LIST_CLEAR: Callee = Callee {
        addr: 0x0016_c628,
        f: ee::s02::f_0016c628,
    };

    pub const CAMERA_TRIGGER_STACK_CLEAR: Callee = Callee {
        addr: 0x0016_d200,
        f: ee::s02::f_0016d200,
    };

    pub const CAMERA_RESET_CHASE_CONTROLLER_SWITCHES: Callee = Callee {
        addr: 0x0015_dc18,
        f: ee::s01::f_0015dc18,
    };

    pub const BOLT_FILE_CACHE_SHUTDOWN: Callee = Callee {
        addr: 0x0019_cf40,
        f: ee::s03::f_0019cf40,
    };

    pub const DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::f_00317e50,
    };

    /// `call_indirect` targets the generator left as table calls.
    pub const SND_BANK_MONITOR_MEM_SIZE: u32 = 0x003b_7b00;

    /// `PS2_SONY_CODEC_INTERNAL_FeedBitstream` (only used by
    /// `PS2_SONY_CODEC_INTERNAL_NodataCallback`).
    pub const FEED_BITSTREAM: Callee = Callee {
        addr: 0x003b_0b40,
        f: ee::s13::PS2_SONY_CODEC_INTERNAL_FeedBitstream,
    };
    pub(crate) use crate::game::fe::festatetrophyroom::callee::SPRINTF;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;

    pub(crate) use crate::game::lib::eafile::callee::EA_MEM_COPY;
    pub(crate) use crate::game::lib::libc::callee::STRLEN;

    /// `int SHP_CalcAllocSize(int w, int h, int bpp, int palBpp, int
    /// maxLevel, int extra1, int extra2)`: bytes of a whole image chain.
    pub const SHP_CALC_ALLOC_SIZE: Callee = Callee {
        addr: 0x003b_26b8,
        f: ee::s13::SHP_CalcAllocSize,
    };

    /// `void SHP_DecodeDXTColorBlock(u64* dst, int pitch, const u8* block)`.
    pub const SHP_DECODE_DXT_COLOR_BLOCK: Callee = Callee {
        addr: 0x003b_2e78,
        f: ee::s13::SHP_DecodeDXTColorBlock,
    };

    /// `SHP_DecodeDXTAlphaBlock`: a block's alpha into its pixels.
    pub const SHP_DECODE_DXT_ALPHA_BLOCK: Callee = Callee {
        addr: 0x003b_2b90,
        f: ee::s13::SHP_DecodeDXTAlphaBlock,
    };

    /// `SHP_DXTAlphaIsOpaque`: 1 when some DXT block is not fully opaque.
    pub const SHP_DXT_ALPHA_IS_OPAQUE: Callee = Callee {
        addr: 0x003b_3308,
        f: ee::s13::SHP_DXTAlphaIsOpaque,
    };

    /// `int SHP_CalcPixelDataSize(int format, int w, int h, int maxLevel,
    /// int swizzled)`.
    pub const SHP_CALC_PIXEL_DATA_SIZE: Callee = Callee {
        addr: 0x003b_3a50,
        f: ee::s13::SHP_CalcPixelDataSize,
    };

    /// `int RefPack_GetSizes(const u8* src, int* packedSize)`: the unpacked
    /// size of a RefPack stream (and its packed size).
    pub const REFPACK_GET_SIZES: Callee = Callee {
        addr: 0x003b_4d00,
        f: ee::s13::RefPack_GetSizes,
    };

    /// `const char* SHP_FindLongName(const SHPHeader*)`.
    pub const SHP_FIND_LONG_NAME: Callee = Callee {
        addr: 0x003b_3a10,
        f: ee::s13::SHP_FindLongName,
    };

    /// `char* SHPS_GetEntryName(SHPSheet*, int index, char* out)`.
    pub const SHPS_GET_ENTRY_NAME: Callee = Callee {
        addr: 0x003b_3d00,
        f: ee::s13::SHPS_GetEntryName,
    };

    /// `SHPHeader* SHPS_FindEntryByName(SHPSheet*, const char* name)`.
    pub const SHPS_FIND_ENTRY_BY_NAME: Callee = Callee {
        addr: 0x003b_3900,
        f: ee::s13::SHPS_FindEntryByName,
    };

    /// `uint SHP_SwizzleAddr(int x, int y, int w, int h, int bpp)`.
    pub const SHP_SWIZZLE_ADDR: Callee = Callee {
        addr: 0x003b_4218,
        f: ee::s13::SHP_SwizzleAddr,
    };

    /// `uint SHP_SwizzleAddrPSMT4(uint x, uint y, uint w)` and the 8, 16 and
    /// 32-bit forms: a pixel's bit offset in GS swizzled order.
    pub const SHP_SWIZZLE_ADDR_PSMT4: Callee = Callee {
        addr: 0x003b_42b0,
        f: ee::s13::SHP_SwizzleAddrPSMT4,
    };

    pub const SHP_SWIZZLE_ADDR_PSMT8: Callee = Callee {
        addr: 0x003b_4338,
        f: ee::s13::SHP_SwizzleAddrPSMT8,
    };

    pub const SHP_SWIZZLE_ADDR_PSMCT16: Callee = Callee {
        addr: 0x003b_43c0,
        f: ee::s13::SHP_SwizzleAddrPSMCT16,
    };

    pub const SHP_SWIZZLE_ADDR_PSMCT32: Callee = Callee {
        addr: 0x003b_4448,
        f: ee::s13::SHP_SwizzleAddrPSMCT32,
    };

    /// `int SHP_GetBlockSize(const SHPHeader* block)`.
    pub const SHP_GET_BLOCK_SIZE: Callee = Callee {
        addr: 0x003b_3bb0,
        f: ee::s13::SHP_GetBlockSize,
    };

    /// `bool SHP_IsPaletteFormat(int format)`.
    pub const SHP_IS_PALETTE_FORMAT: Callee = Callee {
        addr: 0x003b_4708,
        f: ee::s13::SHP_IsPaletteFormat,
    };

    /// `int SHP_CalcTotalSize(const SHPHeader* img)`.
    pub const SHP_CALC_TOTAL_SIZE: Callee = Callee {
        addr: 0x003b_4478,
        f: ee::s13::SHP_CalcTotalSize,
    };

    /// `void SHP_Relocate(SHPHeader* dst, const SHPHeader* src)`.
    pub const SHP_RELOCATE: Callee = Callee {
        addr: 0x003b_4540,
        f: ee::s13::SHP_Relocate,
    };

    /// `void* EA_MemMove(void* dst, const void* src, int n)`.
    pub const EA_MEM_MOVE: Callee = Callee {
        addr: 0x003e_6188,
        f: ee::s14::EA_MemMove,
    };

    pub(crate) use crate::game::world::streamman::EAC_UNPACK;

    /// `u8* SHP_GetPixelPtr(const SHPHeader*, int x, int y, int level)`.
    pub const SHP_GET_PIXEL_PTR: Callee = Callee {
        addr: 0x003b_4010,
        f: ee::s13::SHP_GetPixelPtr,
    };

    /// `SHP_DecodeDXT`: decodes a DXT image.
    pub const SHP_DECODE_DXT: Callee = Callee {
        addr: 0x003b_33b0,
        f: ee::s13::SHP_DecodeDXT,
    };

    /// `SHPHeader* SHP_Clone(const SHPHeader*)`.
    pub const SHP_CLONE: Callee = Callee {
        addr: 0x003b_4690,
        f: ee::s13::SHP_Clone,
    };

    /// `SHP_InitHeader`: writes an image chain in place.
    pub const SHP_INIT_HEADER: Callee = Callee {
        addr: 0x003b_27c8,
        f: ee::s13::SHP_InitHeader,
    };

    /// `int SHP_PaletteFormatForBpp(int bpp)`: the palette format for a
    /// palette entry depth.
    pub const SHP_PALETTE_FORMAT_FOR_BPP: Callee = Callee {
        addr: 0x003b_38b8,
        f: ee::s13::SHP_PaletteFormatForBpp,
    };

    /// `int SHP_FormatFromBpp(int bpp)`: the SHP pixel format for a bit depth.
    pub const SHP_FORMAT_FROM_BPP: Callee = Callee {
        addr: 0x003b_3da8,
        f: ee::s13::SHP_FormatFromBpp,
    };

    /// `int SHP_GetBpp(const SHPHeader*)`: bits per pixel of a header's format.
    pub const SHP_GET_BPP: Callee = Callee {
        addr: 0x003b_39d8,
        f: ee::s13::SHP_GetBpp,
    };

    pub(crate) use crate::game::bx::cubicspline::callee::MPEG2_GET_HDR;
    pub(crate) use crate::game::bx::cubicspline::callee::MPEG2_INITIALIZE_BUFFER;

    pub(crate) use crate::game::bx::cubicspline::callee::SCE_MPEG_ADD_CALLBACK;
    pub(crate) use crate::game::bx::cubicspline::callee::SCE_MPEG_CREATE;
    pub(crate) use crate::game::bx::cubicspline::callee::SCE_MPEG_INIT;
    pub(crate) use crate::game::bx::cubicspline::callee::SONY_CODEC_INIT_FRAME;

    pub(crate) use crate::game::mem::memstd::callee::SONY_CODEC_FREE_FRAME;

    pub(crate) const SND_IS_INITIALIZED: Callee = Callee {
        addr: 0x003b_5440,
        f: ee::s13::SND_IsInitialized,
    };

    pub(crate) const AV_FIND_TOKEN: Callee = Callee {
        addr: 0x003a_f670,
        f: ee::s13::AV_FindToken,
    };

    pub(crate) const AV_PARSE_INT_FIELD: Callee = Callee {
        addr: 0x003a_f560,
        f: ee::s13::AV_ParseIntField,
    };

    pub(crate) const AV_PARSE_JUSTIFY: Callee = Callee {
        addr: 0x003a_f428,
        f: ee::s13::AV_ParseJustify,
    };

    pub(crate) const AV_SUBTITLE_SET_JUSTIFY_X: Callee = Callee {
        addr: 0x003b_03d8,
        f: ee::s13::AV_SubtitleSetJustifyX,
    };

    pub(crate) const AV_SUBTITLE_SET_JUSTIFY_Y: Callee = Callee {
        addr: 0x003b_03e0,
        f: ee::s13::AV_SubtitleSetJustifyY,
    };

    pub(crate) const AV_SUBTITLE_SET_SHAPE: Callee = Callee {
        addr: 0x003b_03e8,
        f: ee::s13::AV_SubtitleSetShape,
    };

    pub(crate) const AV_SUBTITLE_SET_COLOR0: Callee = Callee {
        addr: 0x003b_03f0,
        f: ee::s13::AV_SubtitleSetColor0,
    };

    pub(crate) const AV_SUBTITLE_SET_COLOR1: Callee = Callee {
        addr: 0x003b_03f8,
        f: ee::s13::AV_SubtitleSetColor1,
    };

    pub(crate) const AV_SUBTITLE_SET_COLOR2: Callee = Callee {
        addr: 0x003b_0400,
        f: ee::s13::AV_SubtitleSetColor2,
    };

    pub(crate) const AV_SUBTITLE_SET_COLOR3: Callee = Callee {
        addr: 0x003b_0408,
        f: ee::s13::AV_SubtitleSetColor3,
    };

    /// `MPEG2_next_start_code` (`0x003b_1498`).
    pub(crate) const NEXT_START_CODE: Callee = Callee {
        addr: 0x003b_1498,
        f: ee::s13::MPEG2_next_start_code,
    };

    /// `MPEG2_Get_Bits32` (`0x003b_13a8`).
    pub(crate) const GET_BITS32: Callee = Callee {
        addr: 0x003b_13a8,
        f: ee::s13::MPEG2_Get_Bits32,
    };

    /// `MPEG2_sequence_header` (`0x003b_14f0`).
    pub(crate) const SEQUENCE_HEADER: Callee = Callee {
        addr: 0x003b_14f0,
        f: ee::s13::MPEG2_sequence_header,
    };

    /// `MPEG2_group_of_pictures_header` (`0x003b_16e0`).
    pub(crate) const GROUP_OF_PICTURES_HEADER: Callee = Callee {
        addr: 0x003b_16e0,
        f: ee::s13::MPEG2_group_of_pictures_header,
    };

    /// `MPEG2_picture_header` (`0x003b_17a8`).
    pub(crate) const PICTURE_HEADER: Callee = Callee {
        addr: 0x003b_17a8,
        f: ee::s13::MPEG2_picture_header,
    };

    /// `MPEG2_extension_and_user_data` (`0x003b_1988`).
    pub(crate) const EXTENSION_AND_USER_DATA: Callee = Callee {
        addr: 0x003b_1988,
        f: ee::s13::MPEG2_extension_and_user_data,
    };

    /// `MPEG2_sequence_extension` (`0x003b_1ab0`).
    pub(crate) const SEQUENCE_EXTENSION: Callee = Callee {
        addr: 0x003b_1ab0,
        f: ee::s13::MPEG2_sequence_extension,
    };

    /// `MPEG2_sequence_display_extension` (`0x003b_1cc0`).
    pub(crate) const SEQUENCE_DISPLAY_EXTENSION: Callee = Callee {
        addr: 0x003b_1cc0,
        f: ee::s13::MPEG2_sequence_display_extension,
    };

    /// `MPEG2_quant_matrix_extension` (`0x003b_1d58`).
    pub(crate) const QUANT_MATRIX_EXTENSION: Callee = Callee {
        addr: 0x003b_1d58,
        f: ee::s13::MPEG2_quant_matrix_extension,
    };

    /// `MPEG2_sequence_scalable_extension` (`0x003b_1ee0`).
    pub(crate) const SEQUENCE_SCALABLE_EXTENSION: Callee = Callee {
        addr: 0x003b_1ee0,
        f: ee::s13::MPEG2_sequence_scalable_extension,
    };

    /// `MPEG2_picture_display_extension` (`0x003b_1fc8`).
    pub(crate) const PICTURE_DISPLAY_EXTENSION: Callee = Callee {
        addr: 0x003b_1fc8,
        f: ee::s13::MPEG2_picture_display_extension,
    };

    /// `MPEG2_picture_coding_extension` (`0x003b_20b8`).
    pub(crate) const PICTURE_CODING_EXTENSION: Callee = Callee {
        addr: 0x003b_20b8,
        f: ee::s13::MPEG2_picture_coding_extension,
    };

    /// `MPEG2_picture_spatial_scalable_extension` (`0x003b_2230`).
    pub(crate) const PICTURE_SPATIAL_SCALABLE_EXTENSION: Callee = Callee {
        addr: 0x003b_2230,
        f: ee::s13::MPEG2_picture_spatial_scalable_extension,
    };

    /// `MPEG2_picture_temporal_scalable_extension` (`0x003b_22f8`).
    pub(crate) const PICTURE_TEMPORAL_SCALABLE_EXTENSION: Callee = Callee {
        addr: 0x003b_22f8,
        f: ee::s13::MPEG2_picture_temporal_scalable_extension,
    };

    /// `MPEG2_extra_bit_information` (`0x003b_2318`).
    pub(crate) const EXTRA_BIT_INFORMATION: Callee = Callee {
        addr: 0x003b_2318,
        f: ee::s13::MPEG2_extra_bit_information,
    };

    /// `MPEG2_user_data` (`0x003b_2380`).
    pub(crate) const USER_DATA: Callee = Callee {
        addr: 0x003b_2380,
        f: ee::s13::MPEG2_user_data,
    };

    /// `MPEG2_copyright_extension` (`0x003b_23a0`).
    pub(crate) const COPYRIGHT_EXTENSION: Callee = Callee {
        addr: 0x003b_23a0,
        f: ee::s13::MPEG2_copyright_extension,
    };

    /// `MPEG2_Update_Temporal_Reference_Tacking_Data` (`0x003b_2440`).
    pub(crate) const UPDATE_TEMPORAL_REFERENCE_TACKING_DATA: Callee = Callee {
        addr: 0x003b_2440,
        f: ee::s13::MPEG2_Update_Temporal_Reference_Tacking_Data,
    };
}

/// `gp`-relative globals used here.
pub(crate) mod g {}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {

    pub(crate) use crate::game::bx::bxstringctor::addr::MEM_MAN_FREE;
    pub(crate) use crate::game::bx::bxstringctor::addr::MEM_MAN_FREE_NOSIZE;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

    pub(crate) use crate::game::fe::festaterewards::table_callee::CODEC_DECODE_FRAME;
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
pub(crate) fn t_wscript_cache_field(c: &mut Cpu, offset: u32) {
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
#[cfg(test)]
pub(crate) const SHP_BPP_TABLE: u32 = 0x0044_c3d0;

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

pub(crate) const AV_PLAYER_SET_RATE: Callee = Callee {
    addr: 0x003a_e888,
    f: ee::s13::f_003ae888,
};

pub(crate) const AV_AUDIO_IS_DONE: Callee = Callee {
    addr: 0x003a_efa8,
    f: ee::s13::f_003aefa8,
};

pub(crate) const AV_AUDIO_SET_VOLUME: Callee = Callee {
    addr: 0x003a_ef48,
    f: ee::s13::f_003aef48,
};

pub(crate) const AV_AUDIO_SET_PITCH: Callee = Callee {
    addr: 0x003a_ee98,
    f: ee::s13::f_003aee98,
};

pub(crate) const AV_PLAYER_ON_CHUNK: Callee = Callee {
    addr: 0x003a_ead0,
    f: ee::s13::f_003aead0,
};

pub(crate) const AV_PLAYER_RELEASE_CHUNK: Callee = Callee {
    addr: 0x003a_ece0,
    f: ee::s13::f_003aece0,
};

pub(crate) const EATIMER_GET_TICKS: u32 = 0x003e_4e98;

pub(crate) const SND_LOCK: Callee = Callee {
    addr: 0x003b_58a0,
    f: ee::s13::f_003b58a0,
};

pub(crate) const SND_UNLOCK: Callee = Callee {
    addr: 0x003b_58d8,
    f: ee::s13::f_003b58d8,
};

pub(crate) const SNDSTRM_SET_PITCH: Callee = Callee {
    addr: 0x003b_7fb8,
    f: ee::s13::f_003b7fb8,
};

pub(crate) const SNDSTRM_SET_PREBUFFER: Callee = Callee {
    addr: 0x003b_7d80,
    f: ee::s13::f_003b7d80,
};

pub(crate) const SNDSTRM_SET_VOLUME: Callee = Callee {
    addr: 0x003b_7e70,
    f: ee::s13::f_003b7e70,
};

pub(crate) const SNDSTRM_GET_STATUS: Callee = Callee {
    addr: 0x003b_7b48,
    f: ee::s13::f_003b7b48,
};

pub(crate) const SNDSTRM_GET_REQUEST_STATUS: Callee = Callee {
    addr: 0x003b_7c40,
    f: ee::s13::f_003b7c40,
};

pub(crate) const AV_FIND_SUBTITLE_BY_FRAME: Callee = Callee {
    addr: 0x003a_f258,
    f: ee::s13::f_003af258,
};

/// `__fixunssfsi` (cross-shard, through the function table).
pub(crate) const FIXUNSSFSI: u32 = 0x0041_39f8;

/// The character-class table `AV_SkipWhitespace`/`AV_StrICmp` index with a
/// byte (`lb`): 0x4a0000 - 0x637f. Bit 3 is "whitespace", bit 0 "uppercase".
#[cfg(test)]
pub(crate) const CTYPE: u32 = 0x0049_9c81;

/// `void AV_SkipWhitespace(const char **str, int *count)` (0x3af300): advance
/// `*str` over leading whitespace (per the ctype table) and decrease
/// `*count` accordingly.
pub fn av_skip_whitespace(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A3][0] = 0;
    let empty = c.g[A2][0] == 0;
    c.g[T0][0] = Addr(c.g[A0][0] as u32).lw(c);
    if empty {
        c.cycles += 6;
        av_skip_whitespace_tail(c);
        return;
    }
    c.cycles += 8;
    c.g[V1][0] = 0x4a_0000;
    c.g[V0][0] = Addr(c.g[T0][0] as u32).lb(c);
    c.g[T2][0] = sx(0x4a_0000u32.wrapping_sub(0x637f));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
    c.g[V1][0] &= 8;
    let first_ws = c.g[V1][0] != 0;
    c.g[T1][0] = c.g[A2][0];
    if !first_ws {
        c.cycles += 6;
        av_skip_whitespace_tail(c);
        return;
    }
    c.cycles += 2;
    c.g[A2][0] = c.g[T0][0];
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
    loop {
        c.cycles += 3;
        c.g[V0][0] = (c.g[A3][0] < c.g[T1][0]) as u64;
        let done = c.g[V0][0] == 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        if done {
            break;
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lb(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
        c.g[V1][0] &= 8;
        if c.g[V1][0] == 0 {
            break;
        }
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        if event_due(c) {
            deliver_event_here(c, 0x003a_f338);
        }
    }
    c.cycles += 6;
    av_skip_whitespace_tail(c);
}

/// The shared epilogue: advance `*str` by the skipped count and subtract it
/// from `*count`.
pub(crate) fn av_skip_whitespace_tail(c: &mut Cpu) {
    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
    let ret = c.g[RA][0] as u32;
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    c.pc = ret;
}

/// `int AV_StrICmp(const char *a, const char *b)` (0x3af378): a case-insensitive
/// compare using the ctype table's "uppercase" bit (0); returns the accumulated
/// character difference.
pub fn av_str_icmp(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T5][0] = c.g[A0][0];
    c.g[T4][0] = c.g[A1][0];
    c.g[V0][0] = Addr(c.g[T5][0] as u32).lb(c);
    c.g[T3][0] = 0;
    let a_end = c.g[V0][0] == 0;
    c.g[T2][0] = 0;
    if a_end {
        c.cycles += 6;
        av_str_icmp_tail(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[T4][0] as u32).lb(c);
    let b_end = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[T4][0];
    if b_end {
        c.cycles += 6;
        av_str_icmp_tail(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 0x4a_0000;
    c.g[T1][0] = c.g[T5][0];
    c.g[T6][0] = sx(0x4a_0000u32.wrapping_sub(0x637f));
    loop {
        c.cycles += 16;
        c.g[A3][0] = Addr(c.g[A0][0] as u32).lb(c);
        c.g[T0][0] = Addr(c.g[T1][0] as u32).lb(c);
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T6][0] as u32));
        c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
        c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T6][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lbu(c);
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
        c.g[V1][0] &= 1;
        c.g[A0][0] &= 1;
        if c.g[V1][0] == 0 {
            c.g[A2][0] = c.g[A3][0];
        }
        if c.g[A0][0] == 0 {
            c.g[V0][0] = c.g[T0][0];
        }
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let diff = c.g[T3][0] == 0;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
        if !diff {
            c.cycles += 2;
            let ret = c.g[RA][0] as u32;
            c.g[V0][0] = c.g[T3][0];
            c.pc = ret;
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[T1][0] as u32).lb(c);
        let a_end = c.g[V0][0] == 0;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
        if a_end {
            break;
        }
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[T2][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
        let b_more = c.g[V1][0] != 0;
        c.g[A0][0] = c.g[V0][0];
        if !b_more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_f3a8);
        }
    }
    c.cycles += 6;
    av_str_icmp_tail(c);
}

/// The shared epilogue: return the differing character bytes.
pub(crate) fn av_str_icmp_tail(c: &mut Cpu) {
    c.g[V0][0] = sx((c.g[T5][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
    let ret = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.pc = ret;
}

// ---------------------------------------------------------------------------
// AV player / audio wrappers.

/// `bool AV_Player_IsAudioDone(AV_Player *this)` (0x3ae860): true when there
/// is no audio object, else `AV_Audio_IsDone(this->audio(+0))`.
pub fn av_player_is_audio_done(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = 1;
        if !none {
            c.cycles += 2;
            AV_AUDIO_IS_DONE.call(c, 0x003a_e87c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_Player_SetRate(AV_Player *this, uint rate)` (0x3ae888): clamp the
/// rate to 0x4000, apply it to the audio object's pitch, and update the
/// stream's tick bookkeeping.
pub fn av_player_set_rate(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = (c.g[A1][0] < 0x4001) as u64;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x4000;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let no_audio = c.g[A0][0] == 0;
        if c.g[V0][0] != 0 {
            c.g[S1][0] = c.g[A1][0];
        }
        if !no_audio {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            AV_AUDIO_SET_PITCH.call(c, 0x003a_e8b8)?;
        }
        c.cycles += 7;
        let stream = Addr(c.g[S0][0] as u32).off(0x5c).lw(c);
        c.g[S0][0] = stream;
        call_ptr(c, EATIMER_GET_TICKS, 0x003a_e8c0)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = Addr(stream as u32).off(0x10).lw(c);
        c.g[V0][0] = 0x1000;
        let not_default = c.g[A0][0] != c.g[V0][0];
        c.g[V0][0] = Addr(stream as u32).lw(c);
        if not_default {
            c.cycles += 10;
            c.g[V1][0] = Addr((stream as u32).wrapping_add(8)).ld(c);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            let prod = abi::multu(c, c.g[A0][0] as u32, c.g[V0][0] as u32);
            c.g[A2][0] = prod as u64;
            c.g[V0][0] = c.hi[0];
            c.g[A2][0] = (c.g[A2][0] << 32) >> 32;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] |= c.g[A2][0];
            c.g[V0][0] >>= 12;
        } else {
            c.cycles += 5;
            c.g[V1][0] = Addr((stream as u32).wrapping_add(8)).ld(c);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] = (c.g[V0][0] << 32) >> 32;
        }
        c.cycles += 10;
        c.g[V1][0] = c.g[V1][0].wrapping_add(c.g[V0][0]);
        Addr((c.g[S0][0] as u32).wrapping_add(8)).sd(c, c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.g[V0][0] = 0;
        c.sw(c.g[S0][0] as u32, c.g[A1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_Player_Pause(AV_Player *this)` (0x3ae938): `AV_Player_SetRate(this, 0)`.
pub fn av_player_pause(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        AV_PLAYER_SET_RATE.call(c, 0x003a_e948)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_Player_Resume(AV_Player *this)` (0x3ae958): `AV_Player_SetRate(this, 0x1000)`.
pub fn av_player_resume(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1000;
        AV_PLAYER_SET_RATE.call(c, 0x003a_e968)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Player_SetVolume(AV_Player *this, int volume)` (0x3ae978): 0 when
/// there is no audio object, else `AV_Audio_SetVolume(this->audio(+0), volume)`.
pub fn av_player_set_volume(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !none {
            c.cycles += 2;
            AV_AUDIO_SET_VOLUME.call(c, 0x003a_e994)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_Player_GetChunkCallback(AV_Player* this, void* user)` (0x3aecb8):
/// `AV_Player_OnChunk(*user, this)` (9 cycles, one 0x10 frame, `v0` holds
/// the player across the load).
pub fn av_player_get_chunk_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A1][0] = c.g[V0][0];
        AV_PLAYER_ON_CHUNK.call(c, 0x003a_ecd0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_ParseIntField(char** pp, int* plen, int* out)` (0x3af560): scan
/// `*pp` for a tab/LF within `*plen` bytes, NUL it, parse the head with
/// `strtol` base 10, shrink `*plen` past the consumed bytes, restore the
/// delimiter unless it landed exactly on the new end, and store the value.
/// Returns 1 when digits parsed, 0 when the head is empty.
pub fn av_parse_int_field(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = 0x9;
        frame.save_ra(c, 0x10);
        // `i` (S0) scans `*pp` (S2) for the first tab/LF; `delim` (S3) remembers
        // it (9 when the scan runs out). The NUL goes in the delay slot.
        'scan: {
            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
            let empty_len = c.g[V0][0] == 0;
            // Delay slot of the length test: stash `*out` on both paths.
            c.g[S4][0] = c.g[A2][0];
            if empty_len {
                break 'scan;
            }
            'found: {
                'first: {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(c.g[S2][0] as u32).lw(c);
                    c.g[A1][0] = c.g[V1][0];
                    c.g[V1][0] = Addr(c.g[V1][0] as u32).lbu(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x9));
                    c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        break 'first;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                    Addr(c.g[A1][0] as u32).sb(c, 0);
                    break 'found;
                }
                'search: loop {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    if c.g[V0][0] == 0 {
                        break 'scan;
                    }
                    c.cycles += 5;
                    c.g[V1][0] = Addr(c.g[A0][0] as u32).lbu(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x9));
                    c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_f5c0);
                        }
                        continue 'search;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                    Addr(c.g[A0][0] as u32).sb(c, 0);
                    break 'found;
                }
            }
            // The delimiter byte, sign-extended (0 when no delimiter was NULled,
            // 9/0xA for tab/LF).
            c.cycles += 1;
            c.g[S3][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
        }
        // Parse the (possibly NUL-shortened) head; `end` lands on the stack.
        c.cycles += 12;
        c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0xa;
        call_ptr(c, 0x0041_e9e0, 0x003a_f600)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V1][0] = c.lw(frame.at(c, 0));
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        let new_len = c.g[A0][0];
        Addr(c.g[S1][0] as u32).sw(c, new_len);
        if c.g[S0][0] != c.g[A0][0] {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            let delim = c.g[S3][0];
            Addr(c.g[V0][0] as u32).sb(c, delim);
        }
        // Empty head: 0, leaving `*out` alone. Otherwise store the value, 1.
        c.cycles += 4;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[V0][0] = c.lw(frame.at(c, 0));
        let at_end = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = 0x1;
        if !at_end {
            c.cycles += 4;
            c.g[V1][0] = c.g[A1][0] << 32;
            c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
            let r = c.g[V1][0];
            Addr(c.g[S4][0] as u32).sw(c, r);
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_ParseJustify(char* s, int len)` (0x3af428): NUL the first
/// tab/CR in `s` (remembering it), look `s` up in the five justification
/// names (`AV_StrICmp`, values from the table beside them), restore the
/// byte unless it sat exactly on the end, and return the value (-1 for no
/// match).
pub fn av_parse_justify(c: &mut Cpu) {
    let entry = c.pc;
    guest(c, |c| av_parse_justify_body(c, entry))
}

pub(crate) fn av_parse_justify_body(c: &mut Cpu, mut pc: u32) -> Flow {
    let frame = Frame { size: 0xc0 };
    loop {
        match pc {
            // Entry: spill registers, point at the name/value tables.
            0x003a_f428 => {
                c.cycles += 16;
                c.g[SP][0] = sx(sp(c).wrapping_sub(0xc0));
                frame.save(c, S3, 0x80);
                frame.save(c, S4, 0x70);
                c.g[S3][0] = 0;
                frame.save(c, S5, 0x60);
                c.g[S4][0] = c.g[A0][0];
                frame.save(c, S6, 0x50);
                c.g[S5][0] = c.g[A1][0];
                frame.save(c, S0, 0xb0);
                c.g[S6][0] = 0x9;
                frame.save(c, S1, 0xa0);
                frame.save(c, S2, 0x90);
                frame.save_ra(c, 0x40);
                c.g[A1][0] = 0x490000;
                c.g[A2][0] = 0x490000;
                pc = 0x003a_f468;
            }
            // Scan head: `s[i]` while `i < len`.
            0x003a_f468 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[S3][0] < c.g[S5][0]) as u64;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S3][0] as u32));
                if c.g[V0][0] == 0 {
                    pc = 0x003a_f494;
                } else {
                    pc = 0x003a_f474;
                }
            }
            // Tab or CR ends the scan (its byte is remembered, NULled next).
            0x003a_f474 => {
                c.cycles += 4;
                c.g[V0][0] = 0x9;
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lb(c);
                let is_tab = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0xd;
                if is_tab {
                    pc = 0x003a_f48c;
                } else {
                    pc = 0x003a_f484;
                }
            }
            // Any other byte keeps scanning.
            0x003a_f484 => {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003a_f468) {
                        return Ok(());
                    }
                    pc = 0x003a_f468;
                } else {
                    pc = 0x003a_f48c;
                }
            }
            // Found: NUL the delimiter, remember it.
            0x003a_f48c => {
                c.cycles += 2;
                Addr(c.g[A0][0] as u32).sb(c, 0);
                c.g[S6][0] = c.g[V1][0];
                pc = 0x003a_f494;
            }
            // Copy the five name pointers and values onto the stack.
            0x003a_f494 => {
                c.cycles += 27;
                c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5290));
                c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[A3][0] as u32);
                c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                c.sdl(2, frame.at(c, 0x7));
                c.sdr(2, frame.at(c, 0));
                c.sdl(3, frame.at(c, 0xf));
                c.sdr(3, frame.at(c, 0x8));
                c.sw(frame.at(c, 0x10), c.g[A0][0]);
                c.g[S1][0] = 0xffffffffffff0000;
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x52a8));
                c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[A1][0] as u32);
                c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                c.sdl(2, frame.at(c, 0x27));
                c.sdr(2, frame.at(c, 0x20));
                c.sdl(3, frame.at(c, 0x2f));
                c.sdr(3, frame.at(c, 0x28));
                c.sw(frame.at(c, 0x30), c.g[A0][0]);
                c.g[S1][0] |= 0xffff;
                c.g[S2][0] = 0;
                c.g[S0][0] = c.g[SP][0];
                pc = 0x003a_f504;
            }
            // Next table entry.
            0x003a_f500 => {
                c.cycles += 1;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                pc = 0x003a_f504;
            }
            // All five tried: give up with -1.
            0x003a_f504 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[S2][0] < 0x5) as u64;
                if c.g[V0][0] == 0 {
                    pc = 0x003a_f528;
                } else {
                    pc = 0x003a_f510;
                }
            }
            // Compare one table name; on mismatch advance to the next.
            0x003a_f510 => {
                c.cycles += 5;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A1][0] = c.g[S4][0];
                AV_STR_ICMP.call(c, 0x003a_f51c)?;
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if back_edge_unwinds(c, 0x003a_f500) {
                        return Ok(());
                    }
                    pc = 0x003a_f500;
                } else {
                    pc = 0x003a_f524;
                }
            }
            // Match: the value beside the name.
            0x003a_f524 => {
                c.cycles += 1;
                c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x20)).lw(c);
                pc = 0x003a_f528;
            }
            // Restore the delimiter unless it sat on the end.
            0x003a_f528 => {
                c.cycles += 2;
                let at_end = c.g[S3][0] == c.g[S5][0];
                c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S3][0] as u32));
                if !at_end {
                    c.cycles += 1;
                    let at = c.g[V0][0];
                    let delim = c.g[S6][0];
                    Addr(at as u32).sb(c, delim);
                    pc = 0x003a_f534;
                } else {
                    pc = 0x003a_f534;
                }
            }
            // Return the value.
            0x003a_f534 => {
                c.cycles += 11;
                c.g[V0][0] = c.g[S1][0];
                frame.restore(c, S0, 0xb0);
                frame.restore(c, S1, 0xa0);
                frame.restore(c, S2, 0x90);
                frame.restore(c, S3, 0x80);
                frame.restore(c, S4, 0x70);
                frame.restore(c, S5, 0x60);
                frame.restore(c, S6, 0x50);
                frame.restore_ra(c, 0x40);
                frame.close(c);
                return Ok(());
            }
            _ => {
                c.jump_out(pc, 0x003a_f428, 0x003a_f560);
                return Ok(());
            }
        }
    }
}

/// `void AV_FindToken(char** ps, int* plen, ...)` (0x3af670): skip leading
/// whitespace, then match `*ps` against the `nwords` (word, length) pairs:
/// on a full word match advance `*ps` past it, shrink `*plen`, and store the
/// word index; otherwise clear the outputs (`*ps` = 0, `*plen` = 0,
/// index = -1).
pub fn av_find_token(c: &mut Cpu) {
    let entry = c.pc;
    guest(c, |c| av_find_token_body(c, entry))
}

pub(crate) fn av_find_token_body(c: &mut Cpu, mut pc: u32) -> Flow {
    let frame = Frame { size: 0x80 };
    loop {
        match pc {
            // Entry: spill registers; empty input clears the outputs.
            0x003a_f670 => {
                c.cycles += 17;
                c.g[SP][0] = sx(sp(c).wrapping_sub(0x80));
                frame.save(c, S0, 0x70);
                frame.save(c, S1, 0x60);
                c.g[S0][0] = c.g[A1][0];
                frame.save(c, S2, 0x50);
                c.g[S1][0] = c.g[T0][0];
                frame.save(c, S3, 0x40);
                c.g[S2][0] = c.g[A0][0];
                frame.save(c, S4, 0x30);
                c.g[S3][0] = c.g[T1][0];
                frame.save(c, S5, 0x20);
                c.g[S4][0] = c.g[A3][0];
                frame.save_ra(c, 0x10);
                c.g[S5][0] = c.g[A2][0];
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                let empty = c.g[V0][0] == 0;
                c.sw(frame.at(c, 0), c.g[V0][0]);
                if !empty {
                    pc = 0x003a_f6e0;
                } else {
                    pc = 0x003a_f6b4;
                }
            }
            // Null input: `*ps` = 0, then the shared failure exit.
            0x003a_f6b4 => {
                c.cycles += 2;
                Addr(c.g[S2][0] as u32).sw(c, 0);
                pc = 0x003a_f7b8;
            }
            // Match: advance `*ps` past the word, shrink `*plen`, store the
            // word index; then the shared epilogue.
            0x003a_f6bc => {
                c.cycles += 9;
                c.g[V0][0] = c.lw(frame.at(c, 0));
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let start = c.g[V0][0];
                Addr(c.g[S2][0] as u32).sw(c, start);
                c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                let left = c.g[V1][0];
                Addr(c.g[S0][0] as u32).sw(c, left);
                let word = c.g[T3][0];
                Addr(c.g[S3][0] as u32).sw(c, word);
                pc = 0x003a_f7c8;
            }
            // Skip whitespace; an empty remainder fails.
            0x003a_f6e0 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S0][0];
                AV_SKIP_WHITESPACE.call(c, 0x003a_f6ec)?;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                let empty = c.g[V0][0] == 0;
                c.g[A3][0] = 0;
                if empty {
                    pc = 0x003a_f7b4;
                } else {
                    pc = 0x003a_f6f8;
                }
            }
            // Remember the (post-skip) length.
            0x003a_f6f8 => {
                c.cycles += 2;
                c.g[T5][0] = c.g[V0][0];
                pc = 0x003a_f700;
            }
            // Next start position; no words left means advance.
            0x003a_f700 => {
                c.cycles += 2;
                let no_words = c.g[S1][0] == 0;
                c.g[T3][0] = 0;
                if no_words {
                    pc = 0x003a_f7a4;
                } else {
                    pc = 0x003a_f708;
                }
            }
            // Point at this position's string, word list and length list.
            0x003a_f708 => {
                c.cycles += 4;
                c.g[T4][0] = c.lw(frame.at(c, 0));
                c.g[T1][0] = c.g[S4][0];
                c.g[T0][0] = c.g[S5][0];
                pc = 0x003a_f718;
            }
            // Next word: empty words match immediately.
            0x003a_f718 => {
                c.cycles += 4;
                c.g[T2][0] = Addr(c.g[T1][0] as u32).lw(c);
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T4][0] as u32));
                let empty_word = c.g[T2][0] == 0;
                c.g[A1][0] = 0;
                if empty_word {
                    pc = 0x003a_f788;
                } else {
                    pc = 0x003a_f728;
                }
            }
            // Past the end: this word cannot match here.
            0x003a_f728 => {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V0][0] = (c.g[A3][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
                    pc = 0x003a_f78c;
                } else {
                    pc = 0x003a_f738;
                }
            }
            // First bytes differ: this word cannot match here.
            0x003a_f738 => {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[A2][0] as u32).lb(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
                if c.g[V1][0] != c.g[A0][0] {
                    c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
                    pc = 0x003a_f78c;
                } else {
                    pc = 0x003a_f74c;
                }
            }
            // One more byte matches.
            0x003a_f74c => {
                c.cycles += 1;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                pc = 0x003a_f750;
            }
            // Whole word matched when the count reaches its length.
            0x003a_f750 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[A1][0] < c.g[T2][0]) as u64;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                if c.g[V0][0] == 0 {
                    pc = 0x003a_f788;
                } else {
                    pc = 0x003a_f75c;
                }
            }
            // Still inside the remainder: compare the next byte pair.
            0x003a_f75c => {
                c.cycles += 5;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
                    pc = 0x003a_f78c;
                } else {
                    pc = 0x003a_f770;
                }
            }
            // Equal so far: count the byte and keep matching.
            0x003a_f770 => {
                c.cycles += 6;
                c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[A2][0] as u32).lb(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
                if c.g[V1][0] == c.g[A0][0] {
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003a_f750) {
                        return Ok(());
                    }
                    pc = 0x003a_f750;
                } else {
                    pc = 0x003a_f788;
                }
            }
            // Length reached: recheck the count against this word's length.
            0x003a_f788 => {
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
                pc = 0x003a_f78c;
            }
            // Full match: consume the word. Otherwise try the next word.
            0x003a_f78c => {
                c.cycles += 2;
                let matched = c.g[A1][0] == c.g[V0][0];
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4));
                if matched {
                    if back_edge_unwinds(c, 0x003a_f6bc) {
                        return Ok(());
                    }
                    pc = 0x003a_f6bc;
                } else {
                    pc = 0x003a_f794;
                }
            }
            // Next word.
            0x003a_f794 => {
                c.cycles += 4;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[T3][0] < c.g[S1][0]) as u64;
                let more_words = c.g[V0][0] != 0;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4));
                if more_words {
                    if back_edge_unwinds(c, 0x003a_f718) {
                        return Ok(());
                    }
                    pc = 0x003a_f718;
                } else {
                    pc = 0x003a_f7a4;
                }
            }
            // Next start position.
            0x003a_f7a4 => {
                c.cycles += 4;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[A3][0] < c.g[T5][0]) as u64;
                let more = c.g[V0][0] != 0;
                if more {
                    if back_edge_unwinds(c, 0x003a_f700) {
                        return Ok(());
                    }
                    pc = 0x003a_f700;
                } else {
                    pc = 0x003a_f7b4;
                }
            }
            // No match anywhere: `*ps` = 0, then the shared failure exit.
            0x003a_f7b4 => {
                c.cycles += 1;
                Addr(c.g[S2][0] as u32).sw(c, 0);
                pc = 0x003a_f7b8;
            }
            // Failure exit: `*plen` = 0, index = -1.
            0x003a_f7b8 => {
                c.cycles += 4;
                c.g[V0][0] = 0xffffffffffff0000;
                Addr(c.g[S0][0] as u32).sw(c, 0);
                c.g[V0][0] |= 0xffff;
                let none = c.g[V0][0];
                Addr(c.g[S3][0] as u32).sw(c, none);
                pc = 0x003a_f7c8;
            }
            // Shared epilogue.
            0x003a_f7c8 => {
                c.cycles += 9;
                frame.restore(c, S0, 0x70);
                frame.restore(c, S1, 0x60);
                frame.restore(c, S2, 0x50);
                frame.restore(c, S3, 0x40);
                frame.restore(c, S4, 0x30);
                frame.restore(c, S5, 0x20);
                frame.restore_ra(c, 0x10);
                frame.close(c);
                return Ok(());
            }
            _ => {
                c.jump_out(pc, 0x003a_f670, 0x003a_f7ec);
                return Ok(());
            }
        }
    }
}

/// `int AV_Player_IsFrameDue(AV_Player* this)` (0x3ae780): the media time
/// plus the base at +0x40, scaled by the codec frame rate: 1 while the
/// scaled presentation time has not passed one frame, else 0. Negative
/// intermediate values take a halving path through the float conversion.
pub fn av_player_is_frame_due(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        AV_PLAYER_GET_TIME_MS.call(c, 0x003a_e798)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x64).lw(c);
        c.g[S0][0] = c.g[V0][0];
        RCMP_DECODER_GET_FRAME_RATE.call(c, 0x003a_e7a4)?;
        // Clock plus base; negative sums halve into the float conversion.
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let negative = (c.g[V0][0] as i64) < 0;
        c.mov_s(2, 0);
        if negative {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] &= 0x1;
            c.g[V0][0] |= c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
        } else {
            c.cycles += 4;
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
        }
        // Scale by the rate and the second threshold word.
        c.cycles += 8;
        c.g[AT][0] = 0x3a830000;
        c.g[AT][0] |= 0x126f;
        c.mtc1(1, 0);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x44).lw(c);
        c.mul_s(0, 1, 0);
        c.mul_s(0, 0, 2);
        let negative2 = (c.g[V0][0] as i64) < 0;
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x48));
        if negative2 {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] &= 0x1;
            c.g[V0][0] |= c.g[V1][0];
            c.mtc1(2, 2);
            c.cvt_s_w(2, 2);
            c.add_s(2, 2, 2);
        } else {
            c.cycles += 4;
            c.mtc1(2, 2);
            c.cvt_s_w(2, 2);
        }
        // Due when a full frame's worth has accumulated.
        c.cycles += 8;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x48));
        c.add_s(0, 2, 0);
        c.c_lt_s(1, 0);
        let over = c.fcc();
        c.g[V0][0] = 0;
        if !over {
            c.cycles += 1;
            c.g[V0][0] = 0x1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Player_GetTimeMs(AV_Player* this)` (0x3ae9a0): the media clock in
/// milliseconds (`accum * 1000 / tick_rate`, minus the drift offset). While
/// enabled, the sound locks are held to sample the stream status, and the
/// drift tracker folds the output-latency-corrected error back into the
/// offset, resetting it once it grows past 264 ms.
pub fn av_player_get_time_ms(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x5c).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0042_e508, 0x003a_e9c4)?;
        call_ptr(c, 0x003e_5008, 0x003a_e9cc)?;
        // The tick rate is re-tested here; both arms fall through.
        c.g[A0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x8));
        // Milliseconds from the 1000x-scaled accumulator.
        c.cycles += 12;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x4c).lw(c);
        c.g[V1][0] = c.g[A0][0] << 5;
        c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x54).lw(c);
        c.g[V1][0] = c.g[V1][0].wrapping_sub(c.g[A0][0]);
        c.g[V1][0] = c.g[V1][0] << 2;
        c.g[V1][0] = c.g[V1][0].wrapping_add(c.g[A0][0]);
        c.g[V1][0] = c.g[V1][0] << 35;
        c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
        c.divu(0, V1, V0);
        c.g[S2][0] = c.lo[0];
        let sound_off = c.g[A1][0] == 0;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[A2][0] as u32));
        if !sound_off {
            c.cycles += 19;
            SND_LOCK.call(c, 0x003a_ea10)?;
            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[A1][0] = sx(frame.at(c, 0x10));
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            SNDSTRM_GET_STATUS.call(c, 0x003a_ea20)?;
            c.g[A0][0] = c.lw(frame.at(c, 0x14));
            c.g[A1][0] = c.g[SP][0];
            SNDSTRM_GET_REQUEST_STATUS.call(c, 0x003a_ea2c)?;
            SND_UNLOCK.call(c, 0x003a_ea34)?;
            SND_GET_OUTPUT_LATENCY.call(c, 0x003a_ea3c)?;
            // Behind only when the stream's last position lags the sample.
            c.g[A0][0] = c.lw(frame.at(c, 0x4));
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x50).lw(c);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            let behind = (c.g[V1][0] as i64) < (c.g[V0][0] as i64);
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
            if behind {
                c.cycles += 15;
                c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x58).lw(c);
                c.g[A2][0] = u64::MAX;
                let fresh = c.g[V0][0];
                Addr(c.g[S1][0] as u32).off(0x50).sw(c, fresh);
                let nonneg = (c.g[A2][0] as i64) < (c.g[A0][0] as i64);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7));
                if nonneg {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
                let negative = (c.g[V0][0] as i64) < (c.g[A0][0] as i64);
                if negative {
                    c.g[V0][0] = c.g[A0][0];
                }
                let settled = (c.g[V0][0] as i64) < 265;
                let drift = c.g[A0][0];
                Addr(c.g[S1][0] as u32).off(0x58).sw(c, drift);
                if !settled {
                    c.cycles += 9;
                    let nonneg2 = (c.g[A2][0] as i64) < (c.g[A0][0] as i64);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7));
                    if nonneg2 {
                        c.g[V1][0] = c.g[A0][0];
                    }
                    Addr(c.g[S1][0] as u32).off(0x58).sw(c, 0);
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x4c).lw(c);
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    let offset = c.g[A0][0];
                    Addr(c.g[S1][0] as u32).off(0x4c).sw(c, offset);
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x40);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_SubtitleInit(AVSubtitle *self)` (0x3b0340).
pub(crate) const AV_SUBTITLE_INIT: Callee = Callee {
    addr: 0x003b_0340,
    f: ee::s13::f_003b0340,
};

/// `void AV_SubtitleSetX(AVSubtitle *self, int x)` (0x3b03a8).
pub(crate) const AV_SUBTITLE_SET_X: Callee = Callee {
    addr: 0x003b_03a8,
    f: ee::s13::f_003b03a8,
};

/// `void AV_SubtitleSetY(AVSubtitle *self, int y)` (0x3b03b0).
pub(crate) const AV_SUBTITLE_SET_Y: Callee = Callee {
    addr: 0x003b_03b0,
    f: ee::s13::f_003b03b0,
};

/// `void AV_SubtitleSetWidth(AVSubtitle *self, int width)` (0x3b03b8).
pub(crate) const AV_SUBTITLE_SET_WIDTH: Callee = Callee {
    addr: 0x003b_03b8,
    f: ee::s13::f_003b03b8,
};

/// `void AV_SubtitleSetHeight(AVSubtitle *self, int height)` (0x3b03c0).
pub(crate) const AV_SUBTITLE_SET_HEIGHT: Callee = Callee {
    addr: 0x003b_03c0,
    f: ee::s13::f_003b03c0,
};

/// `void AV_SubtitleSetStringId(AVSubtitle *self, int id)` (0x3b03c8).
pub(crate) const AV_SUBTITLE_SET_STRING_ID: Callee = Callee {
    addr: 0x003b_03c8,
    f: ee::s13::f_003b03c8,
};

/// `void AV_SubtitleSetFrame(AVSubtitle *self, int frame)` (0x3b03d0).
pub(crate) const AV_SUBTITLE_SET_FRAME: Callee = Callee {
    addr: 0x003b_03d0,
    f: ee::s13::f_003b03d0,
};

/// `void AV_Audio_dtor(AV_Audio *self, int flags)` (0x3aee30).
pub(crate) const AV_AUDIO_DTOR: Callee = Callee {
    addr: 0x003a_ee30,
    f: ee::s13::f_003aee30,
};

/// `void *AV_Audio_Init(AV_Audio *self, int a1, int a2)` (0x3aed40).
pub(crate) const AV_AUDIO_INIT: Callee = Callee {
    addr: 0x003a_ed40,
    f: ee::s13::f_003aed40,
};

/// `int AV_Audio_Start(AV_Audio *self)` (0x3aeef8).
pub(crate) const AV_AUDIO_START: Callee = Callee {
    addr: 0x003a_eef8,
    f: ee::s13::f_003aeef8,
};

/// `void RCMP_MakeCallbacks(...)` (0x3b0758).
pub(crate) const RCMP_MAKE_CALLBACKS: Callee = Callee {
    addr: 0x003b_0758,
    f: ee::s13::f_003b0758,
};

/// `RCMP_DECODER *RCMP_DECODER_ctor(RCMP_DECODER *self, void *key)`
/// (0x3b0430).
pub(crate) const RCMP_DECODER_CTOR: Callee = Callee {
    addr: 0x003b_0430,
    f: ee::s13::f_003b0430,
};

/// `int RCMP_DECODER_DecodeFrame(RCMP_DECODER *d, int a1)` (0x3b0600).
pub(crate) const RCMP_DECODER_DECODE_FRAME: Callee = Callee {
    addr: 0x003b_0600,
    f: ee::s13::f_003b0600,
};

/// `void *AV_ChunkInit(AV_Chunk *self)` (0x3b0720).
pub(crate) const AV_CHUNK_INIT: Callee = Callee {
    addr: 0x003b_0720,
    f: ee::s13::f_003b0720,
};

/// `void PS2_SONY_CODEC_INTERNAL_Create(void)` (0x3b11a0).
pub(crate) const PS2_SONY_CODEC_INTERNAL_CREATE: Callee = Callee {
    addr: 0x003b_11a0,
    f: ee::s13::f_003b11a0,
};

/// `void RCMP_DECODER_SetCodec(Decoder *d, Codec *codec, int a2)`
/// (0x3b04f8).
pub(crate) const RCMP_DECODER_SET_CODEC: Callee = Callee {
    addr: 0x003b_04f8,
    f: ee::s13::f_003b04f8,
};

/// `int RCMP_IsMpegChunkTag(uint tag)` (0x3b11f0).
pub(crate) const RCMP_IS_MPEG_CHUNK_TAG: Callee = Callee {
    addr: 0x003b_11f0,
    f: ee::s13::f_003b11f0,
};

/// `void StreamFile_SetPriority(int handle, int a, int b)` (0x3e0ad8).
pub(crate) const STREAM_FILE_SET_PRIORITY: Callee = Callee {
    addr: 0x003e_0ad8,
    f: ee::s14::f_003e0ad8,
};

/// `int AsyncFile_GetResult(int handle, void *out, int *size)` (0x3df9d8).
pub(crate) const ASYNC_FILE_GET_RESULT: Callee = Callee {
    addr: 0x003d_f9d8,
    f: ee::s14::f_003df9d8,
};

/// `int StreamFile_QueueFile(int h, int a, int b, int c)` (0x3e0d80).
pub(crate) const STREAM_FILE_QUEUE_FILE: Callee = Callee {
    addr: 0x003e_0d80,
    f: ee::s14::f_003e0d80,
};

/// `uint EATimer_GetTicks(void)` (0x3e4e98).
pub(crate) const EA_TIMER_GET_TICKS: Callee = Callee {
    addr: 0x003e_4e98,
    f: ee::s14::f_003e4e98,
};

/// `void RCMP_AV_MS_TIMER_Update(void *timer)` (0x42e508).
pub(crate) const RCMP_AV_MS_TIMER_UPDATE: Callee = Callee {
    addr: 0x0042_e508,
    f: ee::s15::f_0042e508,
};

/// `uint EATimer_GetTickRate(void)` (0x3e5008).
pub(crate) const EA_TIMER_GET_TICK_RATE: Callee = Callee {
    addr: 0x003e_5008,
    f: ee::s14::f_003e5008,
};

/// `void StreamFile_Destroy(int handle)` (0x3e0a28).
pub(crate) const STREAM_FILE_DESTROY: Callee = Callee {
    addr: 0x003e_0a28,
    f: ee::s14::f_003e0a28,
};

/// `int EAFile_GetSettings(void *stream)` (0x3de928).
pub(crate) const EA_FILE_GET_SETTINGS: Callee = Callee {
    addr: 0x003d_e928,
    f: ee::s14::f_003de928,
};

/// `void *StreamFile_GetChunk(int stream)` (0x3e12e0).
pub(crate) const STREAM_FILE_GET_CHUNK: Callee = Callee {
    addr: 0x003e_12e0,
    f: ee::s14::f_003e12e0,
};

/// `int EATimer_ServiceCallbacks(void *unused)` (0x3e5928).
pub(crate) const EA_TIMER_SERVICE_CALLBACKS: Callee = Callee {
    addr: 0x003e_5928,
    f: ee::s14::f_003e5928,
};

/// `int StreamFile_IsIdle(int stream)` (0x3e1530).
pub(crate) const STREAM_FILE_IS_IDLE: Callee = Callee {
    addr: 0x003e_1530,
    f: ee::s14::f_003e1530,
};

/// `void AV_CreateSubtitles(SubtitleList *list, int count, int x, int y,
/// int width, int height)` (0x3af0c8): allocate `count` 0x34-byte subtitle
/// records, run `AV_SubtitleInit` over each, then stamp every record's
/// geometry (`x`, `y`, `width`, `height`) and its string id (its index).
/// Record 0's frame is cleared as well. `width` arrives in `t0` (spilled to
/// the stack on entry), `height` in `t1`.
pub fn av_create_subtitles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0xb0);
        c.g[V1][0] = 0x51_0000;
        frame.save(c, 20, 0x60);
        c.g[T2][0] = sx(r32(c, V1).wrapping_sub(0x6bd0));
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, 16, 0xa0);
        c.g[A1][0] = 0x34;
        frame.save(c, 17, 0x90);
        let rec_bytes = abi::mult(c, r32(c, S4), r32(c, A1));
        c.g[A1][0] = sx(rec_bytes);
        frame.save(c, 18, 0x80);
        frame.save(c, 19, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 22, 0x40);
        c.g[A0][0] = 0x49_0000;
        frame.save(c, 23, 0x30);
        c.g[S6][0] = c.g[T1][0];
        frame.save(c, 30, 0x20);
        c.g[S7][0] = c.g[A3][0];
        frame.save(c, 21, 0x50);
        c.g[FP][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5260));
        c.sw(sp(c), c.g[T0][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
        c.g[V0][0] = Addr(r32(c, V1).wrapping_sub(0x6bd0)).lw(c);
        c.g[A2][0] = 0;
        c.g[T0][0] = Addr(r32(c, T2).wrapping_add(8)).lw(c);
        c.g[A3][0] = 0;
        let alloc = r32(c, V0);
        c.g[RA][0] = sx(0x003a_f140);
        c.g[S1][0] = sx(r32(c, S4).wrapping_sub(1));
        call_ptr(c, alloc, 0x003a_f140)?;
        c.g[S3][0] = sx(r32(c, V0).wrapping_add(0x10));
        c.sw(r32(c, V0), c.g[S4][0]);
        let no_records = c.g[S4][0] == 0;
        c.g[S0][0] = c.g[S3][0];
        if !no_records {
            c.cycles += 2;
            c.g[S5][0] = u64::MAX;
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003a_f164);
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                AV_SUBTITLE_INIT.call(c, 0x003a_f164)?;
                let more = c.g[S1][0] != c.g[S5][0];
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(SUBTITLE_RECORD as u32));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_f158);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 3;
        c.sw(r32(c, S2), c.g[S3][0]);
        let empty = c.g[S4][0] == 0;
        c.sw(r32(c, S2).wrapping_add(4), c.g[S4][0]);
        if !empty {
            c.cycles += 3;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x003a_f184);
            c.g[A1][0] = 0;
            AV_SUBTITLE_SET_FRAME.call(c, 0x003a_f184)?;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(4)).lw(c);
        let none = c.g[V0][0] == 0;
        c.g[S0][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[S1][0] = 0;
            loop {
                c.cycles += 25;
                c.g[A0][0] = Addr(r32(c, S2)).lw(c);
                c.g[A1][0] = c.g[FP][0];
                c.g[RA][0] = sx(0x003a_f1a8);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                AV_SUBTITLE_SET_X.call(c, 0x003a_f1a8)?;
                c.g[A0][0] = Addr(r32(c, S2)).lw(c);
                c.g[A1][0] = c.g[S7][0];
                c.g[RA][0] = sx(0x003a_f1b8);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                AV_SUBTITLE_SET_Y.call(c, 0x003a_f1b8)?;
                c.g[A0][0] = Addr(r32(c, S2)).lw(c);
                c.g[A1][0] = c.lw(sp(c));
                c.g[RA][0] = sx(0x003a_f1c8);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                AV_SUBTITLE_SET_WIDTH.call(c, 0x003a_f1c8)?;
                c.g[A0][0] = Addr(r32(c, S2)).lw(c);
                c.g[A1][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x003a_f1d8);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                AV_SUBTITLE_SET_HEIGHT.call(c, 0x003a_f1d8)?;
                c.g[A0][0] = Addr(r32(c, S2)).lw(c);
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[RA][0] = sx(0x003a_f1ec);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                AV_SUBTITLE_SET_STRING_ID.call(c, 0x003a_f1ec)?;
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(4)).lw(c);
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(SUBTITLE_RECORD as u32));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_f198);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 12;
        frame.restore(c, 16, 0xa0);
        frame.restore(c, 17, 0x90);
        frame.restore(c, 18, 0x80);
        frame.restore(c, 19, 0x70);
        frame.restore(c, 20, 0x60);
        frame.restore(c, 21, 0x50);
        frame.restore(c, 22, 0x40);
        frame.restore(c, 23, 0x30);
        frame.restore(c, 30, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_Player_Shutdown(AV_Player *this, int flags)` (0x3ae300): tear
/// down a player: release the decoder through its vtable, destroy the audio
/// object, free the timer, destroy both stream handles, apply the pending
/// chunk callbacks, and free the stream/timer slots. `flags & 1` frees the
/// player itself through the heap free slot.
pub fn av_player_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0x20);
        c.g[A2][0] = Addr(r32(c, S0).wrapping_add(0x64)).lw(c);
        let no_decoder = c.g[A2][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !no_decoder {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x18)).lw(c);
            c.g[A1][0] = 3;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(8)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
            let release = r32(c, V1);
            c.g[RA][0] = sx(0x003a_e33c);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, release, 0x003a_e33c)?;
            c.sw(r32(c, S0).wrapping_add(0x64), 0);
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x5c)).lw(c);
        } else {
            c.cycles += 4;
            c.g[RA][0] = sx(0x003a_e354);
            c.g[A1][0] = 3;
            AV_AUDIO_DTOR.call(c, 0x003a_e354)?;
            c.sw(r32(c, S0), 0);
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x5c)).lw(c);
        }
        c.cycles += 2;
        let no_timer = c.g[A0][0] == 0;
        c.g[V1][0] = 0x51_0000;
        if !no_timer {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, V1).wrapping_sub(0x6bcc)).lw(c);
            let free = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e370);
            call_ptr(c, free, 0x003a_e370)?;
            c.sw(r32(c, S0).wrapping_add(0x5c), 0);
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x003a_e37c);
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x30)).lw(c);
        STREAM_FILE_DESTROY.call(c, 0x003a_e37c)?;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x14)).lw(c);
        let one_handle = c.g[V0][0] == 0;
        c.sw(r32(c, S0).wrapping_add(0x30), 0);
        if !one_handle {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003a_e390);
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x34)).lw(c);
            STREAM_FILE_DESTROY.call(c, 0x003a_e390)?;
            c.sw(r32(c, S0).wrapping_add(0x34), 0);
        }
        c.cycles += 3;
        c.g[S1][0] = Addr(r32(c, S0).wrapping_add(0x20)).lw(c);
        let no_extra = c.g[S1][0] == 0;
        c.g[V0][0] = 0x20;
        if !no_extra {
            c.cycles += 7;
            c.g[A0][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x003a_e3ac);
            c.sw(sp(c), c.g[V0][0]);
            EA_FILE_GET_SETTINGS.call(c, 0x003a_e3ac)?;
            c.g[V0][0] = Addr(sp(c).wrapping_add(8)).lw(c);
            let apply = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e3b8);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, apply, 0x003a_e3b8)?;
            c.sw(r32(c, S0).wrapping_add(0x20), 0);
        }
        c.cycles += 3;
        c.g[S1][0] = Addr(r32(c, S0).wrapping_add(0x28)).lw(c);
        let no_extra2 = c.g[S1][0] == 0;
        c.g[V0][0] = 0x20;
        if !no_extra2 {
            c.cycles += 7;
            c.g[A0][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x003a_e3d4);
            c.sw(sp(c), c.g[V0][0]);
            EA_FILE_GET_SETTINGS.call(c, 0x003a_e3d4)?;
            c.g[V0][0] = Addr(sp(c).wrapping_add(8)).lw(c);
            let apply = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e3e0);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, apply, 0x003a_e3e0)?;
            c.sw(r32(c, S0).wrapping_add(0x28), 0);
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
        let no_free = c.g[A0][0] == 0;
        c.g[V1][0] = 0x51_0000;
        if !no_free {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, V1).wrapping_sub(0x6bcc)).lw(c);
            let free = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e3fc);
            call_ptr(c, free, 0x003a_e3fc)?;
            c.sw(r32(c, S0).wrapping_add(4), 0);
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
        let no_free2 = c.g[A0][0] == 0;
        c.g[V1][0] = 0x51_0000;
        if !no_free2 {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, V1).wrapping_sub(0x6bcc)).lw(c);
            let free = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e418);
            call_ptr(c, free, 0x003a_e418)?;
            c.sw(r32(c, S0).wrapping_add(8), 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 1;
        let keep = c.g[V0][0] == 0;
        c.g[V1][0] = 0x51_0000;
        if !keep {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, V1).wrapping_sub(0x6bcc)).lw(c);
            let free = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e434);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, free, 0x003a_e434)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Player_Start(AV_Player *this, int a1, int rate)` (0x3ae450):
/// (re)open the two stream handles, resolve the pending async reads, bring
/// up the audio object and the decoder, start the millisecond timer and set
/// the playback rate. Returns the first decoded frame number (`+0x74`).
pub fn av_player_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S4, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S2, 0x60);
        c.g[A1][0] = 0x97;
        frame.save(c, S3, 0x50);
        c.g[A2][0] = 0x33;
        frame.save_ra(c, 0x20);
        c.g[RA][0] = sx(0x003a_e48c);
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x34)).lw(c);
        STREAM_FILE_SET_PRIORITY.call(c, 0x003a_e48c)?;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x30)).lw(c);
        c.g[A1][0] = 0x97;
        c.g[RA][0] = sx(0x003a_e49c);
        c.g[A2][0] = 0x33;
        STREAM_FILE_SET_PRIORITY.call(c, 0x003a_e49c)?;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x24)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x2c)).lw(c);
        } else {
            c.cycles += 10;
            c.g[A2][0] = sx(sp(c).wrapping_add(0x10));
            c.g[RA][0] = sx(0x003a_e4b4);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x20));
            ASYNC_FILE_GET_RESULT.call(c, 0x003a_e4b4)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x30)).lw(c);
            c.g[A3][0] = 0;
            c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x20)).lw(c);
            c.g[RA][0] = sx(0x003a_e4c8);
            c.g[A2][0] = Addr(sp(c).wrapping_add(0x10)).lw(c);
            STREAM_FILE_QUEUE_FILE.call(c, 0x003a_e4c8)?;
            c.sw(r32(c, S1).wrapping_add(0x38), c.g[V0][0]);
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x2c)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x14)).lw(c);
        } else {
            c.cycles += 10;
            c.g[A2][0] = sx(sp(c).wrapping_add(0x14));
            c.g[RA][0] = sx(0x003a_e4e4);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x28));
            ASYNC_FILE_GET_RESULT.call(c, 0x003a_e4e4)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x34)).lw(c);
            c.g[A3][0] = 0;
            c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x28)).lw(c);
            c.g[RA][0] = sx(0x003a_e4f8);
            c.g[A2][0] = Addr(sp(c).wrapping_add(0x14)).lw(c);
            STREAM_FILE_QUEUE_FILE.call(c, 0x003a_e4f8)?;
            c.sw(r32(c, S1).wrapping_add(0x3c), c.g[V0][0]);
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x14)).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x1c)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
            c.sw(r32(c, S1).wrapping_add(0x3c), c.g[V0][0]);
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x1c)).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.sw(r32(c, S1).wrapping_add(0x54), 0);
            c.cycles += 2;
            c.g[S2][0] = 0x51_0000;
            c.g[S3][0] = 0x49_0000;
        } else {
            c.cycles += 22;
            c.g[T1][0] = 0x51_0000;
            c.g[S0][0] = 0x49_0000;
            c.g[V1][0] = sx(r32(c, T1).wrapping_sub(0x6bd0));
            c.g[V0][0] = Addr(r32(c, T1).wrapping_sub(0x6bd0)).lw(c);
            c.g[T0][0] = Addr(r32(c, V1).wrapping_add(8)).lw(c);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x5210));
            c.g[A1][0] = 0x14;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            let alloc = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e548);
            c.g[S2][0] = c.g[T1][0];
            call_ptr(c, alloc, 0x003a_e548)?;
            c.g[S3][0] = c.g[S0][0];
            c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x34)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003a_e55c);
            c.g[A2][0] = Addr(r32(c, S1).wrapping_add(0x3c)).lw(c);
            AV_AUDIO_INIT.call(c, 0x003a_e55c)?;
            c.sw(r32(c, S1), c.g[V0][0]);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
            c.g[V1][0] = !(0u64 | c.g[V1][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 31);
            c.sw(r32(c, S1).wrapping_add(0x54), c.g[V1][0]);
        }
        c.cycles += 5;
        c.sw(r32(c, S1).wrapping_add(0x64), 0);
        c.g[S0][0] = sx(r32(c, S2).wrapping_sub(0x6bd0));
        c.g[V0][0] = Addr(r32(c, S2).wrapping_sub(0x6bd0)).lw(c);
        let no_heap = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !no_heap {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        let no_callbacks = c.g[V1][0] == 0;
        c.g[A2][0] = 0x3b_0000;
        if !no_callbacks {
            c.cycles += 19;
            c.sw(r32(c, S1).wrapping_add(0x60), c.g[S1][0]);
            c.g[A3][0] = 0x3b_0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0x1348));
            c.g[A3][0] = sx(r32(c, A3).wrapping_sub(0x12e0));
            c.g[T0][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x003a_e5c0);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x60));
            RCMP_MAKE_CALLBACKS.call(c, 0x003a_e5c0)?;
            c.g[T0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
            c.g[A0][0] = sx(r32(c, S3).wrapping_add(0x5210));
            c.g[V0][0] = Addr(r32(c, S2).wrapping_sub(0x6bd0)).lw(c);
            c.g[A1][0] = 0x1c;
            c.g[A2][0] = 0;
            let alloc = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e5dc);
            c.g[A3][0] = 0;
            call_ptr(c, alloc, 0x003a_e5dc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003a_e5e8);
            c.g[A1][0] = c.g[SP][0];
            RCMP_DECODER_CTOR.call(c, 0x003a_e5e8)?;
            c.sw(r32(c, S1).wrapping_add(0x64), c.g[V0][0]);
        }
        c.cycles += 25;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x64)).lw(c);
        c.g[RA][0] = sx(0x003a_e5f8);
        c.g[A1][0] = 0;
        RCMP_DECODER_DECODE_FRAME.call(c, 0x003a_e5f8)?;
        c.sw(r32(c, S1).wrapping_add(0x74), c.g[V0][0]);
        c.g[A1][0] = 0x18;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
        c.g[V0][0] = Addr(r32(c, S2).wrapping_sub(0x6bd0)).lw(c);
        let timer_alloc = r32(c, V0);
        c.g[RA][0] = sx(0x003a_e618);
        c.g[A0][0] = sx(r32(c, S3).wrapping_add(0x5210));
        call_ptr(c, timer_alloc, 0x003a_e618)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003a_e624);
        c.sd(r32(c, S0).wrapping_add(8), 0);
        EA_TIMER_GET_TICKS.call(c, 0x003a_e624)?;
        c.g[V1][0] = 0x1000;
        c.sw(r32(c, S0), c.g[V0][0]);
        c.sw(r32(c, S0).wrapping_add(0x10), c.g[V1][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x003a_e63c);
        c.sw(r32(c, S1).wrapping_add(0x5c), c.g[S0][0]);
        RCMP_AV_MS_TIMER_UPDATE.call(c, 0x003a_e63c)?;
        c.g[RA][0] = sx(0x003a_e644);
        EA_TIMER_GET_TICK_RATE.call(c, 0x003a_e644)?;
        c.g[A0][0] = c.ld(r32(c, S0).wrapping_add(8));
        // Either way the timer block ends here: the original branches to the
        // same block exit whether the tick rate came back zero or not.
        c.cycles += 14;
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        c.g[V1][0] = c.g[A0][0] << 5;
        c.sw(r32(c, S1).wrapping_add(0x40), c.g[S5][0]);
        c.g[V1][0] = c.g[V1][0].wrapping_sub(c.g[A0][0]);
        c.sw(r32(c, S1).wrapping_add(0x50), 0);
        c.g[V1][0] = c.g[V1][0] << 2;
        c.sw(r32(c, S1).wrapping_add(0x58), 0);
        c.g[V1][0] = c.g[V1][0].wrapping_add(c.g[A0][0]);
        c.g[V1][0] = c.g[V1][0] << 35;
        c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
        c.divu(0, 3, 2);
        c.g[V1][0] = c.lo[0];
        let no_audio = c.g[A1][0] == 0;
        c.sw(r32(c, S1).wrapping_add(0x4c), c.g[V1][0]);
        if !no_audio {
            c.cycles += 2;
            c.g[RA][0] = sx(0x003a_e690);
            c.g[A0][0] = c.g[A1][0];
            AV_AUDIO_START.call(c, 0x003a_e690)?;
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x003a_e69c);
        c.g[A1][0] = 0x1000;
        AV_PLAYER_SET_RATE.call(c, 0x003a_e69c)?;
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x74)).lw(c);
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

/// `int AV_Player_OnChunk(AV_Player *this, void *decoder, void **out)`
/// (0x3aead0): poll the stream for the next chunk and hand it to the codec:
/// fetch a chunk, tag it (MPEG chunks get a codec object created and set),
/// release it, and repeat while the stream is idle. Returns through the save
/// slots; the out pointer receives the working chunk buffer.
pub fn av_player_on_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        c.g[S3][0] = 0;
        frame.save(c, S5, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S7, 0x30);
        c.g[S6][0] = 0x51_0000;
        frame.save(c, FP, 0x20);
        c.g[S7][0] = 0x3;
        frame.save(c, S0, 0xa0);
        c.g[FP][0] = 0x2;
        frame.save(c, S1, 0x90);
        frame.save_ra(c, 0x10);
        frame.save(c, S2, 0x80);
        c.g[S2][0] = Addr(r32(c, A0).wrapping_add(0x30)).lw(c);
        c.sw(r32(c, S5), 0);
        'poll: loop {
            'done: {
                'check: {
                    'chunk: {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x003a_eb28);
                        c.g[A0][0] = c.g[S2][0];
                        STREAM_FILE_GET_CHUNK.call(c, 0x003a_eb28)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x003a_eb34);
                        c.g[A0][0] = 0;
                        EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003a_eb34)?;
                        if c.g[S0][0] == 0 {
                            break 'chunk;
                        }
                        'filter: {
                            c.cycles += 15;
                            c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
                            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(1)).lbu(c);
                            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(2)).lbu(c);
                            c.g[V0][0] = sx(r32(c, V0) << 24);
                            c.g[V1][0] = sx(r32(c, V1) << 16);
                            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(3)).lbu(c);
                            c.g[A0][0] = sx(r32(c, A0) << 8);
                            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                            c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                            c.g[S1][0] = c.g[V0][0] | c.g[A1][0];
                            c.g[RA][0] = sx(0x003a_eb6c);
                            c.g[A0][0] = c.g[S1][0];
                            RCMP_IS_MPEG_CHUNK_TAG.call(c, 0x003a_eb6c)?;
                            if c.g[V0][0] != 0 {
                                c.g[S3][0] = c.g[S7][0];
                            }
                            let plain = c.g[S3][0] == 0;
                            c.g[V0][0] = 0x51_0000;
                            if plain {
                                break 'filter;
                            }
                            'init: {
                                c.cycles += 15;
                                c.g[A1][0] = 0xc;
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x6bd0));
                                c.g[A2][0] = 0;
                                c.g[T0][0] = Addr(r32(c, V0).wrapping_add(8)).lw(c);
                                c.g[V0][0] = 0x49_0000;
                                c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x5210));
                                c.g[V0][0] = Addr(r32(c, S6).wrapping_sub(0x6bd0)).lw(c);
                                let alloc = r32(c, V0);
                                c.g[RA][0] = sx(0x003a_eb9c);
                                c.g[A3][0] = 0;
                                call_ptr(c, alloc, 0x003a_eb9c)?;
                                c.g[RA][0] = sx(0x003a_eba4);
                                c.g[A0][0] = c.g[V0][0];
                                AV_CHUNK_INIT.call(c, 0x003a_eba4)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.sw(r32(c, S5), c.g[S1][0]);
                                let tag_mismatch = c.g[S3][0] != c.g[FP][0];
                                // Delay slot of the tag test: link the chunk
                                // on both paths.
                                c.sw(r32(c, S1), c.g[S0][0]);
                                if tag_mismatch {
                                    break 'init;
                                }
                                // Unreachable: S3 is 0 or 3 here (cleared on
                                // entry, set to 3 on an MPEG tag) while FP is
                                // 2, so the original's else arm never runs.
                                c.cycles += 13;
                                c.g[V1][0] = sx(r32(c, S0).wrapping_add(4));
                                c.g[A2][0] = Addr(r32(c, S0).wrapping_add(4)).lbu(c);
                                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(3)).lbu(c);
                                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(2)).lbu(c);
                                c.g[A1][0] = Addr(r32(c, V1).wrapping_add(1)).lbu(c);
                                c.g[V0][0] = sx(r32(c, V0) << 24);
                                c.g[A0][0] = sx(r32(c, A0) << 16);
                                c.sw(r32(c, S1).wrapping_add(4), c.g[S0][0]);
                                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                                c.g[A1][0] = sx(r32(c, A1) << 8);
                                c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                            }
                            c.cycles += 13;
                            c.g[V1][0] = sx(r32(c, S0).wrapping_add(4));
                            c.g[A3][0] = Addr(r32(c, S0).wrapping_add(4)).lbu(c);
                            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(3)).lbu(c);
                            c.g[A2][0] = sx(r32(c, S0).wrapping_add(8));
                            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(2)).lbu(c);
                            c.g[A1][0] = Addr(r32(c, V1).wrapping_add(1)).lbu(c);
                            c.g[V0][0] = sx(r32(c, V0) << 24);
                            c.g[A0][0] = sx(r32(c, A0) << 16);
                            c.sw(r32(c, S1).wrapping_add(4), c.g[A2][0]);
                            c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                            c.g[A1][0] = sx(r32(c, A1) << 8);
                            c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                            c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                            c.cycles += 4;
                            c.sw(r32(c, S1).wrapping_add(8), c.g[V0][0]);
                            c.g[V0][0] = Addr(r32(c, S4).wrapping_add(0x14)).lw(c);
                            let encoded = c.g[V0][0] != 0;
                            frame.restore(c, S0, 0xa0);
                            if encoded {
                                break 'check;
                            }
                            c.cycles += 2;
                            if c.g[S3][0] != c.g[S7][0] {
                                frame.restore(c, S1, 0x90);
                                break 'done;
                            }
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x003a_ec3c);
                            PS2_SONY_CODEC_INTERNAL_CREATE.call(c, 0x003a_ec3c)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x003a_ec4c);
                            c.g[A0][0] = c.g[S4][0];
                            RCMP_DECODER_SET_CODEC.call(c, 0x003a_ec4c)?;
                            frame.restore(c, S0, 0xa0);
                            if c.g[V0][0] == c.g[FP][0] {
                                break 'check;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x003a_ec5c);
                            c.g[A0][0] = c.g[S4][0];
                            RCMP_DECODER_RELEASE_CODEC.call(c, 0x003a_ec5c)?;
                            frame.restore(c, S0, 0xa0);
                            break 'check;
                        }
                        c.cycles += 5;
                        c.sw(sp(c), c.g[S1][0]);
                        c.g[A1][0] = c.g[S0][0];
                        c.sb(sp(c).wrapping_add(4), 0);
                        c.g[RA][0] = sx(0x003a_ec78);
                        c.g[A0][0] = c.g[S2][0];
                        STREAM_FILE_RELEASE_CHUNK.call(c, 0x003a_ec78)?;
                        break 'chunk;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003a_ec80);
                    c.g[A0][0] = c.g[S2][0];
                    STREAM_FILE_IS_IDLE.call(c, 0x003a_ec80)?;
                    let more = c.g[V0][0] == 0;
                    frame.restore(c, S0, 0xa0);
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_eb20);
                        }
                        continue 'poll;
                    }
                    break 'check;
                }
                c.cycles += 1;
                frame.restore(c, S1, 0x90);
            }
            break 'poll;
        }
        c.cycles += 10;
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

/// `void AV_Player_Init(AV_Player* self, int a1, int a2, int a3, int a4,
/// int a5, int a6, int a7, int a8)` (0x3adec8): zero the player, record
/// the constructor arguments, and — when the allocator object is up —
/// build the stream pair: two allocator results, two `StreamFile_Create`
/// streams, priorities, channel filters, then (by the `type` in `a7`) an
/// `AsyncFile_Load` pair, a `StreamFile_QueueRange` pair or a
/// `StreamFile_QueueFile` pair. With no audio (`a4 == 0`) only one
/// allocator result is kept. A dead allocator reports busy and returns.
/// The sound flag at +0x1c is `a8`, forced to 1 while SND is down.
pub fn av_player_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        c.g[S1][0] = c.g[T3][0];
        frame.save(c, S3, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S5, 0x50);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[T0][0];
        frame.save(c, S7, 0x30);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, FP, 0x20);
        c.g[S7][0] = c.g[T2][0];
        frame.save(c, S4, 0x60);
        c.g[FP][0] = c.g[T1][0];
        c.sw(frame.at(c, 0), c.g[A2][0]);
        c.g[S0][0] = c.lw(frame.at(c, 0xb0));
        frame.save_ra(c, 0x10);
        'b_3adf90: {
            c.g[S4][0] = 0x1;
            callee::SND_IS_INITIALIZED.call(c, 0x003a_df20)?;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = c.g[S4][0];
            }
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x78;
            call_ptr(c, 0x003e6448, 0x003a_df34)?;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[S1][0]);
            c.g[V1][0] = 0x510000;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), c.g[S0][0]);
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x20), 0u64);
            c.g[S0][0] = c.g[V1][0];
            c.sw((c.g[S2][0] as u32).wrapping_add(0x28), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x24), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x2c), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x44), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x5c), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x64), 0u64);
            c.sw(c.g[S2][0] as u32, 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x70), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x74), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[S6][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[S5][0]);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_3adf90;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[S4][0];
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = 0u64;
                }
                break 'b_3adf90;
            }
        }
        'b_3ae2c8: {
            'b_3adfa8: {
                c.cycles += 2;
                let t = c.g[A0][0] != 0u64;

                if t {
                    break 'b_3adfa8;
                } else {
                    c.cycles += 4;
                    call_ptr(c, 0x003e62b0, 0x003a_dfa0)?;
                    let t = true;
                    frame.restore(c, S0, 0xa0);
                    if t {
                        break 'b_3ae2c8;
                    } else {
                        break 'b_3adfa8;
                    }
                }
            }
            'b_3ae2c4: {
                'b_3ae280: {
                    'b_3ae270: {
                        'b_3ae178: {
                            c.cycles += 2;
                            let t = c.g[S5][0] == 0u64;
                            c.g[A0][0] = 0x490000;
                            if t {
                                break 'b_3ae178;
                            } else {
                                'b_3ae0c0: {
                                    'b_3ae0a8: {
                                        c.cycles += 55;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[S4][0]);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5218));
                                        c.g[A1][0] = c.g[S3][0];
                                        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[A2][0] = 0u64;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x6bd0));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, t, 0x003a_dfd0)?;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                        c.g[A0][0] = 0x490000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5230));
                                        c.g[A1][0] = c.g[S7][0];
                                        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[A2][0] = 0u64;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x6bd0));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, t, 0x003a_dff4)?;
                                        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                        c.g[T0][0] = c.g[S3][0];
                                        c.g[A0][0] = 0x2;
                                        c.g[A1][0] = 0x2;
                                        c.g[A2][0] = 0x1;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        call_ptr(c, 0x003e06d8, 0x003a_e010)?;
                                        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        c.g[T0][0] = c.g[S7][0];
                                        c.g[A0][0] = 0x2;
                                        c.g[A1][0] = 0x2;
                                        c.g[A2][0] = 0x1;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                                        call_ptr(c, 0x003e06d8, 0x003a_e02c)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                                        c.g[A1][0] = 0x98;
                                        c.g[A2][0] = 0x34;
                                        call_ptr(c, 0x003e0ad8, 0x003a_e040)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                        c.g[A1][0] = 0x98;
                                        c.g[A2][0] = 0x34;
                                        call_ptr(c, 0x003e0ad8, 0x003a_e050)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                        c.g[A1][0] = 0x1;
                                        c.g[A2][0] = 0xff;
                                        c.g[A3][0] = 0x4d;
                                        c.g[T0][0] = 0x1;
                                        call_ptr(c, 0x003e0948, 0x003a_e068)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                        c.g[A1][0] = 0x2;
                                        c.g[A2][0] = 0u64;
                                        c.g[A3][0] = 0u64;
                                        c.g[T0][0] = 0xfffffffffffffffe;
                                        call_ptr(c, 0x003e0948, 0x003a_e080)?;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                        if t {
                                            break 'b_3ae0a8;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A1][0] = 0x1;
                                            c.g[A2][0] = 0xffff;
                                            c.g[A3][0] = 0x4353;
                                            c.g[T0][0] = 0x1;
                                            call_ptr(c, 0x003e0948, 0x003a_e0a0)?;
                                            let t = true;
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                            if t {
                                                break 'b_3ae0c0;
                                            } else {
                                                break 'b_3ae0a8;
                                            }
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x1;
                                    c.g[A2][0] = 0xffff;
                                    c.g[A3][0] = 0x4353;
                                    c.g[T0][0] = 0xffffffffffffffff;
                                    call_ptr(c, 0x003e0948, 0x003a_e0bc)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                    break 'b_3ae0c0;
                                }
                                'b_3ae100: {
                                    c.cycles += 9;
                                    c.g[A1][0] = 0x2;
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    c.g[T0][0] = 0xfffffffffffffffe;
                                    call_ptr(c, 0x003e0948, 0x003a_e0d4)?;
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6bd0));
                                    if t {
                                        break 'b_3ae100;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.g[S5][0];
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        call_ptr(c, 0x003df690, 0x003a_e0f0)?;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                                        c.g[A0][0] = c.g[S6][0];
                                        let t = true;
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_3ae270;
                                        } else {
                                            break 'b_3ae100;
                                        }
                                    }
                                }
                                'b_3ae13c: {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[V0][0] = 0x2;
                                    if t {
                                        break 'b_3ae13c;
                                    } else {
                                        c.cycles += 13;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                        c.g[A1][0] = c.g[S5][0];
                                        c.g[A2][0] = 0u64;
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x003e0c88, 0x003a_e11c)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                        c.g[A1][0] = c.g[S6][0];
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                                        c.g[A2][0] = 0u64;
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x003e0c88, 0x003a_e134)?;
                                        let t = true;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                                        if t {
                                            break 'b_3ae2c4;
                                        } else {
                                            break 'b_3ae13c;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V1][0] != c.g[V0][0];
                                frame.restore(c, S0, 0xa0);
                                if t {
                                    break 'b_3ae2c8;
                                } else {
                                    c.cycles += 13;
                                    c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                                    c.g[A3][0] = 0u64;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                    call_ptr(c, 0x003e0d80, 0x003a_e158)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                    c.g[A2][0] = c.g[FP][0];
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A3][0] = 0u64;
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                                    call_ptr(c, 0x003e0d80, 0x003a_e170)?;
                                    let t = true;
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                                    if t {
                                        break 'b_3ae2c4;
                                    } else {
                                        break 'b_3ae178;
                                    }
                                }
                            }
                        }
                        'b_3ae25c: {
                            'b_3ae244: {
                                c.cycles += 44;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x14), 0u64);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5218));
                                c.g[A1][0] = c.g[S3][0];
                                c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[A2][0] = 0u64;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x6bd0));
                                let t = c.g[V0][0] as u32;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, t, 0x003a_e198)?;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                                c.g[A3][0] = c.g[V0][0];
                                c.g[T0][0] = c.g[S3][0];
                                c.g[A2][0] = 0x2;
                                c.g[A0][0] = 0x2;
                                c.g[A1][0] = 0x3;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                call_ptr(c, 0x003e06d8, 0x003a_e1b8)?;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.g[V0][0];
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                                call_ptr(c, 0x003e0bf8, 0x003a_e1c8)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                                c.g[A1][0] = 0x98;
                                c.g[A2][0] = 0x34;
                                call_ptr(c, 0x003e0ad8, 0x003a_e1dc)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                c.g[A1][0] = 0x98;
                                c.g[A2][0] = 0x34;
                                call_ptr(c, 0x003e0ad8, 0x003a_e1ec)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                c.g[A1][0] = 0x1;
                                c.g[A2][0] = 0xff;
                                c.g[A3][0] = 0x4d;
                                c.g[T0][0] = 0x1;
                                call_ptr(c, 0x003e0948, 0x003a_e204)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                c.g[A1][0] = 0x3;
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                c.g[T0][0] = 0xfffffffffffffffe;
                                call_ptr(c, 0x003e0948, 0x003a_e21c)?;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_3ae244;
                                } else {
                                    c.cycles += 7;
                                    c.g[A1][0] = 0x2;
                                    c.g[A2][0] = 0xffff;
                                    c.g[A3][0] = 0x4353;
                                    c.g[T0][0] = 0x2;
                                    call_ptr(c, 0x003e0948, 0x003a_e23c)?;
                                    let t = true;
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_3ae25c;
                                    } else {
                                        break 'b_3ae244;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A1][0] = 0x2;
                            c.g[A2][0] = 0xffff;
                            c.g[A3][0] = 0x4353;
                            c.g[T0][0] = 0xffffffffffffffff;
                            call_ptr(c, 0x003e0948, 0x003a_e258)?;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                            break 'b_3ae25c;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6bd0));
                        if t {
                            break 'b_3ae280;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S6][0];
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            break 'b_3ae270;
                        }
                    }
                    c.cycles += 4;
                    call_ptr(c, 0x003df690, 0x003a_e278)?;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    if t {
                        break 'b_3ae2c4;
                    } else {
                        break 'b_3ae280;
                    }
                }
                'b_3ae2a4: {
                    c.cycles += 2;
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = 0x2;
                    if t {
                        break 'b_3ae2a4;
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                        c.g[A1][0] = c.g[S6][0];
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x003e0c88, 0x003a_e29c)?;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                        if t {
                            break 'b_3ae2c4;
                        } else {
                            break 'b_3ae2a4;
                        }
                    }
                }
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0];
                frame.restore(c, S0, 0xa0);
                if t {
                    break 'b_3ae2c8;
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                    c.g[A3][0] = 0u64;
                    c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x003e0d80, 0x003a_e2c0)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                    break 'b_3ae2c4;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0xa0);
            break 'b_3ae2c8;
        }
        c.cycles += 11;
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

/// `int AV_ParseSubtitles(SubtitleList* list, const char* text, int len)`
/// (0x3af7f0): copy the 13 subtitle token spellings to the stack, count
/// the entries, allocate the record array (`0x34` bytes each), then walk
/// the text: split lines, match `FRAME`/`STRINGID`/`X`/`Y`/`WIDTH`/
/// `HEIGHT`/`JUSTIFY_X`/`JUSTIFY_Y`/`SHAPE`/`A`/`R`/`G`/`B` through the
/// jump table, parse each value with `AV_ParseIntField`/`AV_ParseJustify`
/// and store it with the matching `AV_SubtitleSet*` setter. Empty input
/// (or no token on any line) stores count 0 and returns. `pc` is read
/// from the machine so a resume after an unwound event or callee
/// re-enters at its recorded arm; there is no separate resume twin.
pub fn av_parse_subtitles(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003af7f0 => {
                    c.cycles += 64;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x240));
                    c.g[V0][0] = 0x490000;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x230));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x220));
                    c.g[S0][0] = c.g[SP][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x210));
                    c.g[S1][0] = 0u64;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x200));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                    c.g[S5][0] = c.g[A1][0];
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                    c.g[S4][0] = c.g[A2][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x154), c.g[V1][0]);
                    c.g[FP][0] = 0xffffffffffffffff;
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                    c.g[S7][0] = 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x1a0), c.g[RA][0]);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5340));
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(7, c.g[V1][0] as u32);
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.sdr(7, c.g[SP][0] as u32);
                    c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0xf));
                    c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x17));
                    c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x27));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x20));
                    c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x2f));
                    c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x28));
                    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x1f));
                    c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x18));
                    c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x27));
                    c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x2f));
                    c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A3][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x118), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x11c), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x120), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x124), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x128), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x12c), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x130), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x134), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x138), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x13c), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x140), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x144), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x148), c.g[A0][0]);
                    c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                    pc = 0x003af8f0;
                    continue;
                }
                0x003af8f0 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x00416810, 0x003a_f900)?;
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = (c.g[S1][0] < 0xdu64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x003a_f8f0) {
                            return Ok(());
                        }
                        pc = 0x003af8f0;
                        continue;
                    }
                    pc = 0x003af910;
                    continue;
                }
                0x003af910 => {
                    c.cycles += 5;
                    c.g[A2][0] = 0x1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x150), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[A2][0]);
                    let t = c.g[S4][0] == 0u64;
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x003af9b4;
                        continue;
                    }
                    pc = 0x003af924;
                    continue;
                }
                0x003af924 => {
                    c.cycles += 3;
                    c.g[S6][0] = 0xa;
                    c.g[S0][0] = c.g[S5][0];
                    pc = 0x003af930;
                    continue;
                }
                0x003af930 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    c.g[A3][0] = 0x9;
                    let t = c.g[V0][0] == c.g[A3][0];
                    c.g[S2][0] = c.g[S0][0];
                    if t {
                        pc = 0x003af950;
                        continue;
                    }
                    pc = 0x003af940;
                    continue;
                }
                0x003af940 => {
                    c.cycles += 4;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                    c.g[T1][0] = 0x1;

                    if c.g[T0][0] != c.g[T1][0] {
                        c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                        pc = 0x003af994;
                        continue;
                    }
                    pc = 0x003af950;
                    continue;
                }
                0x003af950 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffffffffffffffff;
                    let t = c.g[FP][0] != c.g[V0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), 0u64);
                    if t {
                        pc = 0x003af990;
                        continue;
                    }
                    pc = 0x003af95c;
                    continue;
                }
                0x003af95c => {
                    c.cycles += 13;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x84), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S2][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x84));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[T0][0] = 0xd;
                    c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x88));
                    callee::AV_FIND_TOKEN.call(c, 0x003a_f984)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x88));
                    c.g[V0][0] = !(0u64 | c.g[V0][0]);
                    if c.g[V0][0] != 0 {
                        c.g[FP][0] = c.g[S7][0];
                    }
                    pc = 0x003af990;
                    continue;
                }
                0x003af990 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                    pc = 0x003af994;
                    continue;
                }
                0x003af994 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != c.g[S6][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003af9a8;
                        continue;
                    }
                    pc = 0x003af99c;
                    continue;
                }
                0x003af99c => {
                    c.cycles += 3;
                    c.g[V1][0] = 0x1;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[V1][0]);
                    pc = 0x003af9a8;
                    continue;
                }
                0x003af9a8 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x003a_f930) {
                            return Ok(());
                        }
                        pc = 0x003af930;
                        continue;
                    }
                    pc = 0x003af9b4;
                    continue;
                }
                0x003af9b4 => {
                    c.cycles += 3;
                    c.g[S6][0] = 0xffffffffffffffff;
                    let t = c.g[FP][0] != c.g[S6][0];
                    c.g[T1][0] = sx((c.g[S7][0] as u32).wrapping_sub(c.g[FP][0] as u32));
                    if t {
                        pc = 0x003af9d8;
                        continue;
                    }
                    pc = 0x003af9c0;
                    continue;
                }
                0x003af9c0 => {
                    c.cycles += 3;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                    let t = true;
                    c.sw(c.g[S3][0] as u32, 0u64);
                    if t {
                        pc = 0x003b030c;
                        continue;
                    }
                    pc = 0x003af9cc;
                    continue;
                }
                0x003af9cc => {
                    c.cycles += 3;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[A0][0]);
                    if t {
                        pc = 0x003afb9c;
                        continue;
                    }
                    pc = 0x003af9d8;
                    continue;
                }
                0x003af9d8 => {
                    c.cycles += 18;
                    c.g[A1][0] = 0x34;
                    c.g[S7][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = 0x510000;
                    c.mult(0, 5, 23, 5);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
                    c.g[T0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = 0x490000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5260));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] as u32;
                    c.g[S0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x2));
                    call_ptr(c, t, 0x003a_fa10)?;
                    c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.sw(c.g[V0][0] as u32, c.g[S7][0]);
                    let t = c.g[S0][0] == c.g[S6][0];
                    c.g[S1][0] = c.g[S2][0];
                    if t {
                        pc = 0x003afa3c;
                        continue;
                    }
                    pc = 0x003afa20;
                    continue;
                }
                0x003afa20 => {
                    c.cycles += 2;
                    c.g[S6][0] = 0xffffffffffffffff;
                    pc = 0x003afa28;
                    continue;
                }
                0x003afa28 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    AV_SUBTITLE_INIT.call(c, 0x003a_fa34)?;
                    let t = c.g[S0][0] != c.g[S6][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x34));
                    if t {
                        if back_edge_unwinds(c, 0x003a_fa28) {
                            return Ok(());
                        }
                        pc = 0x003afa28;
                        continue;
                    }
                    pc = 0x003afa3c;
                    continue;
                }
                0x003afa3c => {
                    c.cycles += 5;
                    c.sw(c.g[S3][0] as u32, c.g[S2][0]);
                    c.g[S0][0] = 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[S7][0]);
                    let t = c.g[S4][0] == 0u64;
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x003afa8c;
                        continue;
                    }
                    pc = 0x003afa50;
                    continue;
                }
                0x003afa50 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lb(c.g[S5][0] as u32);
                    c.g[V0][0] = c.g[V0][0] ^ 0xa;
                    c.g[S2][0] = (c.g[V0][0] < 0x1u64) as u64;
                    let t = c.g[FP][0] == c.g[S2][0];
                    c.g[A2][0] = 0x1;
                    if t {
                        pc = 0x003afa90;
                        continue;
                    }
                    pc = 0x003afa64;
                    continue;
                }
                0x003afa64 => {
                    c.cycles += 1;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    pc = 0x003afa68;
                    continue;
                }
                0x003afa68 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    if t {
                        pc = 0x003afa8c;
                        continue;
                    }
                    pc = 0x003afa74;
                    continue;
                }
                0x003afa74 => {
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lb(c.g[V0][0] as u32);
                    c.g[V1][0] = c.g[V0][0] ^ 0xa;
                    if c.g[V1][0] == 0 {
                        c.g[S2][0] = c.g[A0][0];
                    }

                    if c.g[FP][0] != c.g[S2][0] {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        if back_edge_unwinds(c, 0x003a_fa68) {
                            return Ok(());
                        }
                        pc = 0x003afa68;
                        continue;
                    }
                    pc = 0x003afa8c;
                    continue;
                }
                0x003afa8c => {
                    c.cycles += 1;
                    c.g[A2][0] = 0x1;
                    pc = 0x003afa90;
                    continue;
                }
                0x003afa90 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[A2][0]);
                    if t {
                        pc = 0x003afb9c;
                        continue;
                    }
                    pc = 0x003afa9c;
                    continue;
                }
                0x003afa9c => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    pc = 0x003afaa0;
                    continue;
                }
                0x003afaa0 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x9;
                    c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                    let t = c.g[V1][0] == c.g[A0][0];
                    c.g[S2][0] = c.g[V0][0];
                    if t {
                        pc = 0x003afac0;
                        continue;
                    }
                    pc = 0x003afab0;
                    continue;
                }
                0x003afab0 => {
                    c.cycles += 4;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                    c.g[V0][0] = 0x1;

                    if c.g[A3][0] != c.g[V0][0] {
                        c.g[V1][0] = c.lb(c.g[S2][0] as u32);
                        pc = 0x003afb80;
                        continue;
                    }
                    pc = 0x003afac0;
                    continue;
                }
                0x003afac0 => {
                    c.cycles += 15;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x90), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8c), c.g[S2][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[T0][0] = 0xd;
                    c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x88));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), 0u64);
                    callee::AV_FIND_TOKEN.call(c, 0x003a_faec)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x88));
                    c.g[V0][0] = (c.g[V1][0] < 0xdu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afafc;
                    continue;
                }
                0x003afafc => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5380));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x003afb14 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x118), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb1c;
                    continue;
                }
                0x003afb1c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x11c), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb24;
                    continue;
                }
                0x003afb24 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x134), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb2c;
                    continue;
                }
                0x003afb2c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x138), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb34;
                    continue;
                }
                0x003afb34 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x120), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb3c;
                    continue;
                }
                0x003afb3c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x124), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb44;
                    continue;
                }
                0x003afb44 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x128), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb4c;
                    continue;
                }
                0x003afb4c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x12c), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb54;
                    continue;
                }
                0x003afb54 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x130), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb5c;
                    continue;
                }
                0x003afb5c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x13c), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb64;
                    continue;
                }
                0x003afb64 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x140), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb6c;
                    continue;
                }
                0x003afb6c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x144), c.g[S0][0]);
                    if t {
                        pc = 0x003afb78;
                        continue;
                    }
                    pc = 0x003afb74;
                    continue;
                }
                0x003afb74 => {
                    c.cycles += 1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x148), c.g[S0][0]);
                    pc = 0x003afb78;
                    continue;
                }
                0x003afb78 => {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = c.lb(c.g[S2][0] as u32);
                    pc = 0x003afb80;
                    continue;
                }
                0x003afb80 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xa;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x1;
                    if t {
                        if back_edge_unwinds(c, 0x003a_f9cc) {
                            return Ok(());
                        }
                        pc = 0x003af9cc;
                        continue;
                    }
                    pc = 0x003afb8c;
                    continue;
                }
                0x003afb8c => {
                    c.cycles += 4;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x003a_faa0) {
                            return Ok(());
                        }
                        pc = 0x003afaa0;
                        continue;
                    }
                    pc = 0x003afb9c;
                    continue;
                }
                0x003afb9c => {
                    c.cycles += 4;
                    c.g[S0][0] = 0u64;
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x003b030c;
                        continue;
                    }
                    pc = 0x003afbac;
                    continue;
                }
                0x003afbac => {
                    c.cycles += 19;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), 0u64);
                    c.g[S7][0] = 0xffffffffffffffff;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x160), 0u64);
                    c.g[S6][0] = 0x34;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x164), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x168), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x16c), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x170), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x174), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x178), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x17c), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x180), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x184), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x188), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18c), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x190), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x194), 0u64);
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    pc = 0x003afbf8;
                    continue;
                }
                0x003afbf8 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x9;
                    c.g[A0][0] = c.lb(c.g[V0][0] as u32);
                    let t = c.g[A0][0] == c.g[V1][0];
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                    if t {
                        pc = 0x003afc14;
                        continue;
                    }
                    pc = 0x003afc08;
                    continue;
                }
                0x003afc08 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[T0][0] != c.g[V0][0];
                    c.g[V0][0] = 0xa;
                    if t {
                        pc = 0x003b01a8;
                        continue;
                    }
                    pc = 0x003afc14;
                    continue;
                }
                0x003afc14 => {
                    c.cycles += 8;
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if c.g[T1][0] == 0 {
                        c.g[S1][0] = c.g[V0][0];
                    }
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x158), c.g[V0][0]);
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0u64;
                    if t {
                        pc = 0x003afc64;
                        continue;
                    }
                    pc = 0x003afc34;
                    continue;
                }
                0x003afc34 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    if t {
                        pc = 0x003afc50;
                        continue;
                    }
                    pc = 0x003afc3c;
                    continue;
                }
                0x003afc3c => {
                    c.cycles += 1;
                    pc = 0x003afc40;
                    continue;
                }
                0x003afc40 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = (c.g[V0][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003afc64;
                        continue;
                    }
                    pc = 0x003afc50;
                    continue;
                }
                0x003afc50 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x9));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if back_edge_unwinds(c, 0x003a_fc40) {
                            return Ok(());
                        }
                        pc = 0x003afc40;
                        continue;
                    }
                    pc = 0x003afc64;
                    continue;
                }
                0x003afc64 => {
                    c.cycles += 3;
                    c.g[FP][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let t = c.g[FP][0] == c.g[S4][0];
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    if t {
                        pc = 0x003afc7c;
                        continue;
                    }
                    pc = 0x003afc70;
                    continue;
                }
                0x003afc70 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x150), c.g[V1][0]);
                    c.sb(c.g[V0][0] as u32, 0u64);
                    pc = 0x003afc7c;
                    continue;
                }
                0x003afc7c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x118));
                    let t = c.g[S0][0] != c.g[A0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), 0u64);
                    if t {
                        pc = 0x003afce4;
                        continue;
                    }
                    pc = 0x003afc88;
                    continue;
                }
                0x003afc88 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x98), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x94), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x98));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x94));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x9c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_fcc8)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x194));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x9c));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    AV_SUBTITLE_SET_FRAME.call(c, 0x003a_fcdc)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afce4;
                    continue;
                }
                0x003afce4 => {
                    c.cycles += 3;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x11c));
                    let t = c.g[S0][0] != c.g[A3][0];
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x120));
                    if t {
                        pc = 0x003afd4c;
                        continue;
                    }
                    pc = 0x003afcf0;
                    continue;
                }
                0x003afcf0 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xa4), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa4));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_fd30)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x190));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa8));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    AV_SUBTITLE_SET_STRING_ID.call(c, 0x003a_fd44)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afd4c;
                    continue;
                }
                0x003afd4c => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[T1][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x124));
                    if t {
                        pc = 0x003afdb0;
                        continue;
                    }
                    pc = 0x003afd54;
                    continue;
                }
                0x003afd54 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xac), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xac));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xb4), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_fd94)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18c));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb4));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    AV_SUBTITLE_SET_WIDTH.call(c, 0x003a_fda8)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afdb0;
                    continue;
                }
                0x003afdb0 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[V1][0];
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x128));
                    if t {
                        pc = 0x003afe14;
                        continue;
                    }
                    pc = 0x003afdb8;
                    continue;
                }
                0x003afdb8 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xbc), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xb8), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xbc));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb8));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_fdf8)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x188));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    AV_SUBTITLE_SET_HEIGHT.call(c, 0x003a_fe0c)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afe14;
                    continue;
                }
                0x003afe14 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[A3][0];
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x12c));
                    if t {
                        pc = 0x003afe78;
                        continue;
                    }
                    pc = 0x003afe1c;
                    continue;
                }
                0x003afe1c => {
                    c.cycles += 5;
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    callee::AV_PARSE_JUSTIFY.call(c, 0x003a_fe28)?;
                    let t = c.g[V0][0] == c.g[S7][0];
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x003afe48;
                        continue;
                    }
                    pc = 0x003afe30;
                    continue;
                }
                0x003afe30 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x184));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    callee::AV_SUBTITLE_SET_JUSTIFY_X.call(c, 0x003a_fe40)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afe48;
                    continue;
                }
                0x003afe48 => {
                    c.cycles += 12;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V1][0] = ((c.g[S7][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x180));
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = 0u64;
                    }
                    c.mult(0, 3, 2, 22);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T1][0] as u32));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    callee::AV_SUBTITLE_SET_JUSTIFY_X.call(c, 0x003a_fe70)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afe78;
                    continue;
                }
                0x003afe78 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[A0][0];
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x130));
                    if t {
                        pc = 0x003afedc;
                        continue;
                    }
                    pc = 0x003afe80;
                    continue;
                }
                0x003afe80 => {
                    c.cycles += 5;
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    callee::AV_PARSE_JUSTIFY.call(c, 0x003a_fe8c)?;
                    let t = c.g[V0][0] == c.g[S7][0];
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x003afeac;
                        continue;
                    }
                    pc = 0x003afe94;
                    continue;
                }
                0x003afe94 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x17c));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    callee::AV_SUBTITLE_SET_JUSTIFY_Y.call(c, 0x003a_fea4)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afeac;
                    continue;
                }
                0x003afeac => {
                    c.cycles += 12;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V1][0] = ((c.g[S7][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x178));
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = 0u64;
                    }
                    c.mult(0, 3, 2, 22);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    callee::AV_SUBTITLE_SET_JUSTIFY_Y.call(c, 0x003a_fed4)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003afedc;
                    continue;
                }
                0x003afedc => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[A0][0];
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x134));
                    if t {
                        pc = 0x003aff40;
                        continue;
                    }
                    pc = 0x003afee4;
                    continue;
                }
                0x003afee4 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc8));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc4));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xcc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xcc), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_ff24)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x174));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xcc));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    callee::AV_SUBTITLE_SET_SHAPE.call(c, 0x003a_ff38)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003aff40;
                    continue;
                }
                0x003aff40 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[A3][0];
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                    if t {
                        pc = 0x003affa4;
                        continue;
                    }
                    pc = 0x003aff48;
                    continue;
                }
                0x003aff48 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd4));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_ff88)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x170));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd8));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    AV_SUBTITLE_SET_X.call(c, 0x003a_ff9c)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003affa4;
                    continue;
                }
                0x003affa4 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[T1][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x13c));
                    if t {
                        pc = 0x003b0008;
                        continue;
                    }
                    pc = 0x003affac;
                    continue;
                }
                0x003affac => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xdc), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xdc));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xe4), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003a_ffec)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    AV_SUBTITLE_SET_Y.call(c, 0x003b_0000)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003b0008;
                    continue;
                }
                0x003b0008 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[V1][0];
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x140));
                    if t {
                        pc = 0x003b006c;
                        continue;
                    }
                    pc = 0x003b0010;
                    continue;
                }
                0x003b0010 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xec), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xe8), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xec));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe8));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x24));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003b_0050)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    callee::AV_SUBTITLE_SET_COLOR0.call(c, 0x003b_0064)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003b006c;
                    continue;
                }
                0x003b006c => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[A3][0];
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x144));
                    if t {
                        pc = 0x003b00d0;
                        continue;
                    }
                    pc = 0x003b0074;
                    continue;
                }
                0x003b0074 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xf8), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xf4), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf4));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xfc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x25));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xfc), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003b_00b4)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x164));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xfc));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    callee::AV_SUBTITLE_SET_COLOR1.call(c, 0x003b_00c8)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003b00d0;
                    continue;
                }
                0x003b00d0 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[T1][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x148));
                    if t {
                        pc = 0x003b0134;
                        continue;
                    }
                    pc = 0x003b00d8;
                    continue;
                }
                0x003b00d8 => {
                    c.cycles += 23;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x100), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x104));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x108));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x26));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x108), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003b_0118)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x108));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    callee::AV_SUBTITLE_SET_COLOR2.call(c, 0x003b_012c)?;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003b0134;
                    continue;
                }
                0x003b0134 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[V1][0];
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    if t {
                        pc = 0x003b0194;
                        continue;
                    }
                    pc = 0x003b013c;
                    continue;
                }
                0x003b013c => {
                    c.cycles += 22;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.mult(0, 3, 3, 22);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x110), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10c));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x114));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x27));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x114), c.g[V0][0]);
                    callee::AV_PARSE_INT_FIELD.call(c, 0x003b_017c)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x15c));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x114));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    callee::AV_SUBTITLE_SET_COLOR3.call(c, 0x003b_0190)?;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                    pc = 0x003b0194;
                    continue;
                }
                0x003b0194 => {
                    c.cycles += 2;
                    let t = c.g[FP][0] == c.g[S4][0];
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        pc = 0x003b0300;
                        continue;
                    }
                    pc = 0x003b019c;
                    continue;
                }
                0x003b019c => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    let t = true;
                    c.sb(c.g[V0][0] as u32, c.g[A3][0]);
                    if t {
                        pc = 0x003b0300;
                        continue;
                    }
                    pc = 0x003b01a8;
                    continue;
                }
                0x003b01a8 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003b0300;
                        continue;
                    }
                    pc = 0x003b01b0;
                    continue;
                }
                0x003b01b0 => {
                    c.cycles += 84;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x15c));
                    c.mult(0, 6, 18, 22);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x34));
                    c.g[V1][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[T0][0]);
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[S2][0];
                    }
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x164));
                    c.mult(0, 2, 2, 22);
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x34));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x34));
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x34));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[A3][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x34));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x164), c.g[T0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x168), c.g[T1][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x16c), c.g[V1][0]);
                    c.g[S0][0] = 0u64;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x170));
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x174));
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x178));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x34));
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x17c));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x34));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x180));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x34));
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x34));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x170), c.g[A2][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x34));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x174), c.g[A3][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x17c), c.g[T1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x180), c.g[V1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x178), c.g[T0][0]);
                    c.ldl(4, (c.g[A1][0] as u32).wrapping_sub(0x2d));
                    c.ldr(4, (c.g[A1][0] as u32).wrapping_sub(0x34));
                    c.ldl(6, (c.g[A1][0] as u32).wrapping_sub(0x25));
                    c.ldr(6, (c.g[A1][0] as u32).wrapping_sub(0x2c));
                    c.ldl(7, (c.g[A1][0] as u32).wrapping_sub(0x1d));
                    c.ldr(7, (c.g[A1][0] as u32).wrapping_sub(0x24));
                    c.ldl(8, (c.g[A1][0] as u32).wrapping_sub(0x15));
                    c.ldr(8, (c.g[A1][0] as u32).wrapping_sub(0x1c));
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.sdr(4, c.g[V0][0] as u32);
                    c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x17));
                    c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x10));
                    c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x1f));
                    c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x18));
                    c.ldl(4, (c.g[A1][0] as u32).wrapping_sub(0xd));
                    c.ldr(4, (c.g[A1][0] as u32).wrapping_sub(0x14));
                    c.ldl(6, (c.g[A1][0] as u32).wrapping_sub(0x5));
                    c.ldr(6, (c.g[A1][0] as u32).wrapping_sub(0xc));
                    c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x4));
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x27));
                    c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x20));
                    c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x2f));
                    c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x28));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x30), c.g[A3][0]);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x188));
                    c.g[A3][0] = 0x1;
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x184));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x190));
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x34));
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x194));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x34));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x184), c.g[T1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x188), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x34));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18c), c.g[V1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x190), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x194), c.g[A2][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[A3][0]);
                    pc = 0x003b0300;
                    continue;
                }
                0x003b0300 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x003a_fbf8) {
                            return Ok(());
                        }
                        pc = 0x003afbf8;
                        continue;
                    }
                    pc = 0x003b030c;
                    continue;
                }
                0x003b030c => {
                    c.cycles += 12;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x230));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x220));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x210));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x200));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1a0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x240));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003af7f0, 0x003b033c);
                    return Ok(());
                }
            }
        }
    })
}

/// `void AV_Player_ReleaseChunkCallback(void *unused, void *user, void *chunk)`
/// (0x3aed20): `AV_Player_ReleaseChunk(*user, chunk)`.
pub fn av_player_release_chunk_callback(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A1][0] = c.g[A2][0];
        AV_PLAYER_RELEASE_CHUNK.call(c, 0x003a_ed34)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Audio_SetPitch(AV_Audio *this, int pitch)` (0x3aee98): 0 when the
/// stream id at +0xc is -1, else `SNDSTRM_SetPitch` under the sound lock.
pub fn av_audio_set_pitch(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        let has_stream = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if !has_stream {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 8;
            SND_LOCK.call(c, 0x003a_eecc)?;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
            c.g[A1][0] = c.g[S1][0];
            SNDSTRM_SET_PITCH.call(c, 0x003a_eed8)?;
            c.g[S0][0] = c.g[V0][0];
            SND_UNLOCK.call(c, 0x003a_eee0)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Audio_Start(AV_Audio *this)` (0x3aeef8): 0 when the prebuffer id
/// at +0x10 is -1, else `SNDSTRM_SetPrebuffer(stream, 0)` under the lock.
pub fn av_audio_start(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
        let has_prebuf = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        if has_prebuf {
            c.cycles += 8;
            SND_LOCK.call(c, 0x003a_ef20)?;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
            c.g[A1][0] = 0;
            SNDSTRM_SET_PREBUFFER.call(c, 0x003a_ef2c)?;
            c.g[S0][0] = c.g[V0][0];
            SND_UNLOCK.call(c, 0x003a_ef34)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Audio_SetVolume(AV_Audio *this, int volume)` (0x3aef48): 0 when the
/// stream id at +0xc is -1, else `SNDSTRM_SetVolume` under the lock.
pub fn av_audio_set_volume(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        let has_stream = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if !has_stream {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 8;
            SND_LOCK.call(c, 0x003a_ef7c)?;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
            c.g[A1][0] = c.g[S1][0];
            SNDSTRM_SET_VOLUME.call(c, 0x003a_ef88)?;
            c.g[S0][0] = c.g[V0][0];
            SND_UNLOCK.call(c, 0x003a_ef90)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The subtitle-record size `AV_FindSubtitleByFrame` indexes with; the frame
/// number is at +0x14, the count at +0x4 and the record base at +0.
pub(crate) const SUBTITLE_RECORD: usize = 0x34;

/// `void AV_FreeSubtitles(SubtitleList *list, uint flags)` (0x3af020): free
/// the block holding the subtitle records (`list->base(+0) - 0x10`, sized by
/// `count(+4) * 0x34`) and clear the list; when `flags & 1` also delete it.
pub fn av_free_subtitles(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        let no_base = c.g[A1][0] == 0;
        c.g[V0][0] = SUBTITLE_RECORD as u64;
        if !no_base {
            c.cycles += 6;
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_sub(0x10)).lw(c);
            c.g[A0][0] = c.g[A1][0];
            let prod = abi::mult(c, c.g[V1][0] as u32, c.g[V0][0] as u32);
            c.g[A2][0] = sx(prod);
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            let aligned = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = c.g[A0][0];
            if !aligned {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(SUBTITLE_RECORD as u32));
                loop {
                    c.cycles += 7;
                    let more = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(SUBTITLE_RECORD as u32));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_f060);
                    }
                }
            }
            c.cycles += 6;
            c.g[V1][0] = 0x51_0000;
            c.g[V1][0] = sx(0x51_0000u32.wrapping_sub(0x6bd0));
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, c.g[V0][0] as u32, 0x003a_f090)?;
            c.sw(c.g[S0][0] as u32, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        let delete_list = c.g[V0][0] != 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(4), 0);
        if delete_list {
            c.cycles += 4;
            c.g[V1][0] = 0x51_0000;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x6bcc)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, c.g[V0][0] as u32, 0x003a_f0b0)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Subtitle *AV_FindSubtitleByFrame(SubtitleList *list, uint frame)`
/// (0x3af258): binary-search the sorted records and return the nearest record
/// at or before `frame` (null for an empty list). FPU-free but uses `mult`.
pub fn av_find_subtitle_by_frame(c: &mut Cpu) {
    c.cycles += 4;
    let list = c.g[A0][0];
    c.g[V1][0] = list;
    c.g[V0][0] = Addr(list as u32).off(4).lw(c);
    let empty = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[V0][0];
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[T2][0] = Addr(list as u32).lw(c);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
    c.g[T1][0] = c.g[A0][0];
    c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
    c.g[A3][0] = u64::MAX;
    // count is non-zero here, so the `count < 1` exit below is unreachable.
    c.cycles += 2;
    c.g[T0][0] = SUBTITLE_RECORD as u64;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    loop {
        c.cycles += 9;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        let prod = abi::mult(c, c.g[A2][0] as u32, c.g[T0][0] as u32);
        c.g[V0][0] = sx(prod);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lw(c);
        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
        if c.g[V0][0] != 0 {
            c.g[A3][0] = c.g[A2][0];
        } else {
            c.cycles += 1;
            c.g[A0][0] = c.g[A2][0];
        }
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
        if c.g[V0][0] != 0 {
            break;
        }
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        if event_due(c) {
            deliver_event_here(c, 0x003a_f290);
        }
    }
    c.cycles += 11;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
    c.g[V1][0] = u64::MAX;
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(1));
    if c.g[V1][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[A1][0] = SUBTITLE_RECORD as u64;
    c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[V0][0] = c.g[A0][0];
    }
    let prod = abi::mult(c, c.g[V0][0] as u32, c.g[A1][0] as u32);
    c.g[V1][0] = sx(prod);
    let ret = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.pc = ret;
}

/// `Subtitle *AV_Player_GetCurrentSubtitle(AV_Player *this)` (0x3ae718): the
/// subtitle for the decoder's current frame (or the frame derived from the
/// rate at +0x48), null when the player has no subtitles.
pub fn av_player_get_current_subtitle(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x70).lw(c);
        let no_list = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if no_list {
            c.cycles += 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x74).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x44).lw(c);
                AV_FIND_SUBTITLE_BY_FRAME.call(c, 0x003a_e748)?;
            } else {
                c.cycles += 7;
                c.f[12] = Addr((c.g[S0][0] as u32).wrapping_add(0x48)).read_u32(c);
                call_ptr(c, FIXUNSSFSI, 0x003a_e758)?;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x70).lw(c);
                c.g[A1][0] = c.g[V0][0];
                AV_FIND_SUBTITLE_BY_FRAME.call(c, 0x003a_e764)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Audio_IsDone(AV_Audio *this)` (0x3aefa8): under the sound lock, ask
/// the stream for its status and current request; 0 while either is pending,
/// else true when the finished count at `sp+8` is below 1.
pub fn av_audio_is_done(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        SND_LOCK.call(c, 0x003a_efbc)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        c.g[A1][0] = sx(frame.at(c, 0x10));
        SNDSTRM_GET_STATUS.call(c, 0x003a_efc8)?;
        let mut early = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = Addr(frame.at(c, 0x14)).lw(c);
        if !early {
            c.cycles += 4;
            c.g[A1][0] = c.g[SP][0];
            SNDSTRM_GET_REQUEST_STATUS.call(c, 0x003a_efd8)?;
            early = (c.g[V0][0] as i64) < 0;
        }
        if early {
            c.cycles += 3;
            SND_UNLOCK.call(c, 0x003a_effc)?;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 5;
            SND_UNLOCK.call(c, 0x003a_efe8)?;
            c.g[V0][0] = Addr(frame.at(c, 8)).lw(c);
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DXT colour block.

/// 16-bit RGB555 to the output's RGBA1555 form: red into bits 10-14, green
/// into 5-9, blue into 0-4 and the alpha bit set.
pub(crate) fn to_1555(v: u64) -> u64 {
    let v = v as u32 & 0xffff;
    ((((v & 0x1f) << 10) | ((v >> 1) & 0x3e0) | (v >> 11)) | 0x8000) as u64
}

/// `void SHP_DecodeDXTColorBlock(u64 *dst, int pitch, const u8 *block)`
/// (0x3b2e78): decode one DXT1 colour block into four 16-byte pixels per
/// row, four rows `pitch` bytes apart.
///
/// The block's first word holds the two 565 endpoints. Their order decides
/// the palette: four blended entries when `c0 >= c1`, otherwise the average
/// of the pair (and transparent black). Each index byte then selects four
/// entries 2 bits at a time.
pub fn shp_decode_dxt_color_block(c: &mut Cpu) {
    c.cycles += 34;
    let frame = Frame::open(c, 0x50);
    frame.save(c, S0, 0x40);
    frame.save(c, S1, 0x30);
    frame.save(c, S2, 0x20);
    let stack = sp(c);
    let mut dst = abi::arg(c, 0);
    let pitch = abi::arg(c, 1);
    let block = abi::arg(c, 2);

    // Unpack the two endpoints to 24-bit RGB (the odd `c & 0xf8000000`
    // term is what the original adds to the second one).
    let cw = c.ld(block);
    let c0 = ((cw & 0x1f) << 3) | ((cw & 0x7e0) << 5) | ((cw & 0xf800) << 8);
    let c1 = ((cw >> 13) & 0xf8) | ((cw >> 11) & 0xfc00) | ((cw & 0xf800_0000) >> 8);
    let alpha = 0x40204u64;
    c.sd(stack + 8, c1 | alpha);
    c.sd(stack, c0 | alpha);

    if c0 < c1 {
        // Endpoint order says "four-colour with transparency": the third and
        // fourth entries are the averages of the two.
        c.cycles += 30;
        let (r0, g0, b0) = (c0 & 0xff, c0 & 0xff00, c0 & 0xff0000);
        let (r1, g1, b1) = (c1 & 0xff, c1 & 0xff00, c1 & 0xff0000);
        let sum = (r0 + r1) | ((g0 + g1) | (b0 + b1));
        c.sd(stack + 0x18, 0);
        c.sd(stack + 0x10, (sum >> 1) | 0xff00_0000);
    } else {
        // Three-colour mode: 1/3 and 2/3 blends of the endpoints.
        c.cycles += 72;
        let (r0, g0, b0) = (c0 & 0xff, c0 & 0xff00, c0 & 0xff0000);
        let (r1, g1, b1) = (c1 & 0xff, c1 & 0xff00, c1 & 0xff0000);
        let pal2 = (((5 * r0 + 3 * r1) >> 3) & 0xff)
            | (((5 * g0 + 3 * g1) >> 3) & 0xff00)
            | (((5 * b0 + 3 * b1) >> 3) & 0xff0000);
        let pal3 = ((((3 * r0 + 5 * r1) >> 3) & 0xff)
            | (((3 * g0 + 5 * g1) >> 3) & 0xff00)
            | (((3 * b0 + 5 * b1) >> 3) & 0xff0000))
            | 0xff00_0000;
        c.sd(stack + 0x10, pal2);
        c.sd(stack + 0x18, pal3);
    }
    let pal0 = c.ld(stack) | 0xff00_0000;
    let pal1 = c.ld(stack + 8) | 0xff00_0000;
    let pal2 = c.ld(stack + 0x10) | 0xff00_0000;
    c.sd(stack, pal0);
    c.sd(stack + 8, pal1);
    c.sd(stack + 0x10, pal2);

    // The 16-bit entries are re-encoded in the output's own (RGB555 with an
    // alpha bit) ordering; a zero entry (the transparent one) stays zero.
    c.cycles += 13;
    let mut conv = [0u64; 4];
    let t0 = c.lhu(stack);
    if t0 != 0 {
        c.cycles += 10;
        conv[0] = to_1555(t0);
    }
    c.cycles += 3;
    let t1 = c.lhu(stack + 8);
    c.sd(stack, conv[0]);
    if t1 != 0 {
        c.cycles += 10;
        conv[1] = to_1555(t1);
    }
    c.cycles += 4;
    let t2 = c.lhu(stack + 0x10);
    c.sd(stack + 8, conv[1]);
    if t2 != 0 {
        c.cycles += 10;
        conv[2] = to_1555(t2);
    }
    c.cycles += 3;
    let t3 = c.lhu(stack + 0x18);
    c.sd(stack + 0x10, conv[2]);
    if t3 != 0 {
        c.cycles += 10;
        conv[3] = to_1555(t3);
    }

    // Four rows; each index byte picks four palette entries, two bits each.
    c.cycles += 93;
    c.sd(stack + 0x18, conv[3]);
    for row in 0..4u32 {
        let b = c.lbu(block + 4 + row);
        let idx = [
            (b & 3) << 3,
            (b << 1) & 0x18,
            (b >> 1) & 0x18,
            (b >> 6) << 3,
        ];
        let p0 = c.ld(stack + idx[0] as u32);
        let p1 = c.ld(stack + idx[1] as u32);
        let p2 = c.ld(stack + idx[2] as u32);
        let p3 = c.ld(stack + idx[3] as u32);
        c.sd(dst, p0);
        c.sd(dst + 8, p1);
        c.sd(dst + 0x10, p2);
        c.g[V0][0] = p3;
        c.g[V1][0] = sx(stack + idx[3] as u32);
        c.sd(dst + 0x18, p3);
        dst = dst.wrapping_add(pitch);
    }

    frame.restore(c, S0, 0x40);
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t;
}

/// `void MPEG2_picture_temporal_scalable_extension(void)` (0x3b22f8):
/// `MPEG2_Error(D_00495608)` (the error handler is an empty stub).
pub fn mpeg2_picture_temporal_scalable_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x0049_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5608));
        callee::MPEG2_ERROR.call(c, 0x003b_230c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWScriptCache_SetData(cWScriptCache* self, int i, int value)`
/// (0x3aca10): write item `i` of the cache (`+0x4` base, 12 bytes each).
pub fn cwscript_cache_set_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc;
        frame.save_ra(c, 0);
        c.mult(0, V0, A1, V0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 4)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        callee::WSCRIPT_CACHE_SET_DATA.call(c, 0x003a_ca2c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SHP_Free(char* buf)` (0x3b2b68): NUL-terminate the buffer and hand
/// it to the shared-header free routine (`D_0044C458`, newlib `free`).
pub fn shp_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        Addr(abi::arg(c, 0)).sb(c, 0);
        c.g[V1][0] = 0x0045_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(0x0044_c458).lw(c);
        let target = c.g[V0][0] as u32;
        frame.save_ra(c, 0);
        call_ptr(c, target, 0x003b_2b84)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cPS2ChunkResolve` base-object destroying destructor (0x3adc18): the base
/// vtable is the object's third word.
pub fn c_ps2_chunk_resolve_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x003a_dc3c, 0x0049_5150, 0x8)
    })
}

// ---------------------------------------------------------------------------
// MPEG2 bitstream init / read.

/// `void MPEG2_Initialize_Buffer(void* data)` (0x3b1228): install `data` as the
/// bitstream buffer (`D_0050A088`), clear the bit accumulator and refill.
pub fn mpeg2_initialize_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V0][0] = 0x0051_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_sub(0x5f78)).lw(c);
        frame.save_ra(c, 0);
        Addr(r32(c, V1)).sw(c, c.g[A0][0]);
        Addr(r32(c, V1).wrapping_add(0x4)).sw(c, 0);
        c.g[A0][0] = 0;
        Addr(r32(c, V1).wrapping_add(0x8)).sw(c, 0);
        callee::MPEG2_FLUSH_BUFFER.call(c, 0x003b_124c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `unsigned int MPEG2_Get_Bits32(void)` (0x3b13a8): peek 32 bits, consume
/// them, and return the peeked value.
pub fn mpeg2_get_bits32(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x20;
        callee::MPEG2_SHOW_BITS.call(c, 0x003b_13bc)?;
        c.g[S0][0] = c.g[V0][0];
        callee::MPEG2_FLUSH_BUFFER32.call(c, 0x003b_13c4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int RCMP_IsMpegChunkTag(int tag)` (0x3b11f0): whether `tag` is one of the
/// two words of the MPEG chunk-tag table at `0x495448`.
pub fn rcmp_is_mpeg_chunk_tag(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x0049_0000;
    c.g[V1][0] = 0;
    c.g[A1][0] = sx(0x0049_5448);
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let mismatch = c.g[V0][0] != c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if !mismatch {
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[V1][0] < 2u64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003b_1200);
            }
            continue;
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
}

/// `void tWScriptCache_SetData(sWScriptCacheItem* e, int index, int value)`
/// (0x3ACC50): record the data value and type 2 / owner 1, then relocate the
/// item in the script cache.
pub fn wscript_cache_set_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        let item = c.g[A0][0];
        c.g[V0][0] = item;
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A1][0];
        let value = c.g[A2][0];
        Addr((item as u32).wrapping_add(8)).sw(c, value);
        c.g[V1][0] = 1;
        let tags = c.lw_r_2(A1, gp(c, 0x16c8));
        let _ = tags;
        c.g[A2][0] = 2;
        Addr(item as u32).sh(c, 2);
        Addr((item as u32).wrapping_add(2)).sh(c, 1);
        c.g[A2][0] = c.g[A3][0];
        callee::TWSCRIPT_CACHE_RELOCATE.call(c, 0x003a_cc80)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MPEG-2 decoder wrappers (s13): the codec is the object at `decoder + 0x14`
/// and the wrappers call one of its virtual methods.

/// `void RCMP_DECODER_SetCodec(Decoder* d, Codec* codec, int a2)`
/// (0x3b04f8): remember the codec and `a2`, then `codec->v02(codec, a2)`.
pub fn rcmp_decoder_set_codec(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let decoder = c.g[A0][0];
        c.g[V0][0] = decoder;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        // The arguments are moved, not truncated: `a0 = a1`.
        c.g[A0][0] = c.g[A1][0];
        let codec = c.g[A0][0];
        Addr((decoder as u32).wrapping_add(0x14)).sw(c, c.g[A0][0]);
        c.g[A1][0] = c.g[V0][0];
        Addr(decoder as u32).sw(c, c.g[A2][0]);
        let vt = Addr(codec as u32).lw(c);
        c.g[V1][0] = vt;
        let m = VSlot::read(c, vt, 0x10);
        c.g[A3][0] = m.delta;
        c.g[V0][0] = m.func;
        c.g[A0][0] = m.this(codec);
        crate::call::call_ptr(c, m.func as u32, 0x003b_0528)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int RCMP_DECODER_GetFrameNumber(Decoder* d)` (0x3b0580): the codec's
/// frame number, or 0 before a codec is set.
pub fn rcmp_decoder_get_frame_number(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let dec = crate::abi::arg(c, 0);
        let obj = lw(c, dec, 0x14);
        c.g[A1][0] = obj;
        let missing = obj == 0;
        c.g[V0][0] = 0;
        if missing {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 7;
            let vt = Addr(obj as u32).lw(c);
            c.g[V1][0] = vt;
            let m = VSlot::read(c, vt, 0x20);
            c.g[A0][0] = m.delta;
            c.g[V0][0] = m.func;
            c.g[A0][0] = m.this(obj);
            crate::call::call_ptr(c, m.func as u32, 0x003b_05a8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `float RCMP_DECODER_GetFrameRate(Decoder* d)` (0x3b05c0): the codec's
/// frame rate, or 0.0 before a codec is set (the result stays in `v0` when
/// there is none, as the original leaves it).
pub fn rcmp_decoder_get_frame_rate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let dec = crate::abi::arg(c, 0);
        let obj = lw(c, dec, 0x14);
        c.g[A1][0] = obj;
        let missing = obj == 0;
        frame.restore_ra(c, 0);
        if missing {
            c.cycles += 1;
            c.mtc1(0, 0);
        } else {
            c.cycles += 7;
            let vt = Addr(obj as u32).lw(c);
            c.g[V0][0] = vt;
            let m = VSlot::read(c, vt, 0x28);
            c.g[A0][0] = m.delta;
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(obj);
            crate::call::call_ptr(c, m.func as u32, 0x003b_05e8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `unsigned MPEG2_Get_Bits(reader* r)` (0x3b1300): peek at the next bits
/// (`MPEG2_Show_Bits`), then consume them (`MPEG2_Flush_Buffer`) and return
/// the peeked value.
pub fn mpeg2_get_bits(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MPEG2_SHOW_BITS_2.call(c, 0x003b_1318)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::MPEG2_FLUSH_BUFFER_2.call(c, 0x003b_1324)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* AV_Player_ReleaseChunk(AV_Player* self, Chunk* chunk /*a1*/)`
/// (0x3aece0): release the chunk's stream block, then run its destructor
/// through `D_00509434[0]`.
pub fn av_player_release_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x30).lw(c);
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        callee::STREAM_FILE_RELEASE_CHUNK.call(c, 0x003a_ecfc)?;
        let null_chunk = c.g[S0][0] == 0;
        c.g[V1][0] = 0x0051_0000;
        if !null_chunk {
            c.cycles += 3;
            c.g[V0][0] = Addr(0x0051_0000u32.wrapping_sub(0x6bcc)).lw(c);
            let func = r32(c, V0);
            c.g[RA][0] = sx(0x003a_ed10);
            c.g[A0][0] = c.g[S0][0];
            c.pc = func;
            call_indirect(c, func);
            if c.pc != 0x003a_ed10 {
                return Err(crate::call::Unwound);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        leave(c, 0x20);
        Ok(())
    })
}

/// `operator delete(void*)` (0x317e50).
pub(crate) const DELETE_2: Callee = Callee {
    addr: 0x0031_7e50,
    f: ee::s10::f_00317e50,
};

/// `a + off`.
#[inline(always)]
pub(crate) fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// `void RCMP_DECODER_ReleaseCodec(RCMP_DECODER* d)` (0x3b0538): detach
/// (`this, 3`) and drop the codec at `+0x14`.
pub fn rcmp_decoder_release_codec(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(0x14));
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, A2));
            let vt = r32(c, V0);
            c.g[A0][0] = c.lh(vt.wrapping_add(8));
            c.g[V1][0] = c.lw(vt.wrapping_add(0xc));
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x003b_0568)?;
            c.sw(r32(c, S0).wrapping_add(0x14), 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int RCMP_DECODER_GetChunk(RCMP_DECODER* d)` (0x3b06b0): return the pending
/// word if set, else call the decoder's `fn(self, arg, &out)` and return the
/// word it wrote.
pub fn rcmp_decoder_get_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        let this = r32(c, V0);
        c.g[V1][0] = c.lw(this);
        if c.g[V1][0] != 0 {
            c.sw(this, 0);
            c.cycles += 2;
            c.sw(sp(c), c.g[V1][0]);
            c.g[V0][0] = c.g[V1][0];
        } else {
            c.cycles += 6;
            c.g[A2][0] = sx(sp(c));
            c.g[V0][0] = c.lw(this.wrapping_add(8));
            let method = r32(c, V0);
            c.g[A1][0] = c.lw(this.wrapping_add(4));
            call_ptr(c, method, 0x003b_06d8)?;
            c.g[V0][0] = c.lw(sp(c));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void* RCMP_StaticInitDestroy(int init, int prio)` (0x3b0770): the one-time
/// RCMP static setup: with priority 0xffff, build the global RCMP system when
/// `init` is set, else point its slot at the destroy routine.
pub fn rcmp_static_init_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xffff;
        let not_static = c.g[A1][0] != c.g[V0][0];
        frame.save_ra(c, 0);
        if not_static {
            c.cycles += 1;
        } else {
            c.cycles += 2;
            let no_init = c.g[A0][0] == 0;
            c.g[A0][0] = 0x0051_0000;
            if no_init {
                c.cycles += 4;
                c.g[V0][0] = 0x0045_0000;
                c.g[V1][0] = 0x0051_0000;
                c.g[V0][0] = sx(0x0045_0000u32.wrapping_add(0x6890));
                let init_slot = 0x0051_0000u32.wrapping_sub(0x6bc4);
                c.sw(init_slot, c.g[V0][0]);
                c.cycles += 1;
            } else {
                c.cycles += 4;
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x6bd0));
                callee::RCMP_SYSTEM_CTOR.call(c, 0x003b_0790)?;
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* PS2_SONY_CODEC_INTERNAL_Create(void)` (0x3b11a0): allocate 0x80
/// bytes from the RCMP allocator, then run the codec constructor.
pub fn ps2_sony_codec_internal_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x0051_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x0051_0000u32.wrapping_sub(0x6bd0));
        c.g[V0][0] = c.lw(0x0050_9430);
        c.g[A0][0] = 0x0049_0000;
        c.g[T0][0] = c.lw(0x0050_9438);
        c.g[A2][0] = 0;
        c.g[A1][0] = 0x80;
        c.g[A3][0] = 0;
        let alloc = r32(c, V0);
        c.g[A0][0] = sx(0x0049_0000u32.wrapping_add(0x53e8));
        call_ptr(c, alloc, 0x003b_11d0)?;
        c.g[A0][0] = c.g[V0][0];
        callee::PS2_SONY_CODEC_INTERNAL_CTOR.call(c, 0x003b_11d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MPEG2_extra_bit_information(void)` (0x3b2318): count and discard the
/// extra-bit marker bytes (`while (Get_Bits1()) { Flush_Buffer(8); n++ }`).
/// Resumable: the loop head 0x3b2330 is an event resume entry.
pub fn mpeg2_extra_bit_information(c: &mut Cpu) {
    let entry = c.pc;
    guest(c, |c| mpeg2_extra_bits(c, entry))
}

pub(crate) fn mpeg2_extra_bits(c: &mut Cpu, mut pc: u32) -> Flow {
    const ENTRY: u32 = 0x003b_2318;
    const CHECK: u32 = 0x003b_2330;
    const PEEK: u32 = 0x003b_2338;
    const DONE: u32 = 0x003b_2348;
    let frame = Frame { size: 0x20 };
    loop {
        match pc {
            ENTRY => {
                c.cycles += 5;
                c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
                c.sq(S0, p(sp(c), 0x10));
                c.sd(sp(c), c.g[RA][0]);
                c.g[S0][0] = 0;
                // The prologue's degenerate branch goes straight to the peek;
                // 0x3b232c is only a resume entry.
                pc = PEEK;
            }
            0x003b_232c => {
                c.cycles += 1;
                pc = CHECK;
            }
            CHECK => {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                callee::MPEG2_FLUSH_BUFFER_3.call(c, PEEK)?;
                pc = PEEK;
            }
            PEEK => {
                c.cycles += 4;
                callee::MPEG2_GET_BITS1.call(c, 0x003b_2340)?;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = 8;
                if more {
                    if event_due(c) {
                        c.pc = CHECK;
                        if c.event() {
                            return Ok(());
                        }
                    }
                    pc = CHECK;
                } else {
                    pc = DONE;
                }
            }
            DONE => {
                c.cycles += 5;
                c.g[V0][0] = c.g[S0][0];
                frame.restore_ra(c, 0);
                frame.restore(c, S0, 0x10);
                frame.close(c);
                return Ok(());
            }
            _ => {
                c.jump_out(pc, ENTRY, 0x003b_235c);
                return Ok(());
            }
        }
    }
}

/// `void tWScriptCache_Free(entry)` (0x3acc90): free the cached script buffer
/// at `+8` unless it is a borrowed (static) buffer (`*(short*)(entry+2)`),
/// then clear the entry's flag word.
pub fn t_w_script_cache_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let entry = c.g[S0][0] as u32;
        c.g[A0][0] = Addr(entry).off(8).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(entry).off(2).lh(c);
            if c.g[V0][0] != 0 {
                Addr(entry).off(8).sw(c, 0);
            } else {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x003a_ccc0)?;
                Addr(entry).off(8).sw(c, 0);
            }
            c.cycles += 1;
        }
        Addr(entry).sh(c, 0);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AV player.

/// `int AV_Player_DecodeFrame(AV_Player* p, float time)` (0x3ae6c8): store
/// the requested time at +0x48, decode that frame of the stream at +0x64,
/// remember the decoded index at +0x74 and the stream's frame number at
/// +0x44; return the decoded index. 19 cycles plus the callees.
pub fn av_player_decode_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.mov_s(0, 12);
        frame.save(c, S0, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.swc1(0, p(self_, 0x48));
        call_ptr(c, callee::FLOAT_TO_UINT, 0x003a_e6e4)?;
        c.g[A0][0] = c.lw(p(self_, 0x64));
        c.g[A1][0] = c.g[V0][0];
        callee::RCMP_DECODE_FRAME.call(c, 0x003a_e6f0)?;
        c.g[A0][0] = c.lw(p(self_, 0x64));
        c.sw(p(self_, 0x74), c.g[V0][0]);
        callee::RCMP_FRAME_NUMBER.call(c, 0x003a_e6fc)?;
        c.sw(p(self_, 0x44), c.g[V0][0]);
        c.g[V0][0] = c.lw(p(self_, 0x74));
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* AV_Player_ctor_2(AV_Player* self, int a1, int a2, int a3, int t0,
/// int t1)` (0x3ade08): clear the player's state and forward the constructor
/// arguments to `AV_Player_Init`.
pub fn av_player_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.g[A3][0];
        frame.save(c, S0, 0x20);
        c.g[V1][0] = c.g[T0][0];
        frame.save_ra(c, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        Addr(frame.at(c, 0)).sw(c, c.g[T2][0]);
        c.g[A3][0] = c.g[A2][0];
        c.g[T3][0] = c.g[T1][0];
        Addr(p(this, 0x60)).sw(c, 0);
        c.g[T0][0] = c.g[V0][0];
        c.g[T2][0] = c.g[V1][0];
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x003a_de44);
        c.g[T1][0] = 0;
        callee::AV_PLAYER_INIT.call(c, 0x003a_de44)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void RCMP_DECODER_dtor(RCMP_DECODER* d, int flags)` (0x3b04a0): set the
/// vtable, release the codec and call the global destructor for bit 0.
pub fn rcmp_decoder_dtor(c: &mut Cpu) {
    c.cycles += 13;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0045_68a8);
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A1][0];
    c.sw(p_2(c, S1, 0x18), c.g[V0][0]);
    if callee::RCMP_DECODER_RELEASE_CODEC
        .call(c, 0x003b_04c8)
        .is_err()
    {
        return;
    }
    c.g[S0][0] &= 1;
    let keep = c.g[S0][0] != 0;
    c.g[V1][0] = 0x0051_0000;
    if keep {
        c.cycles += 3;
        c.g[V0][0] = Addr(p_2(c, V1, 0u32.wrapping_sub(0x6bcc))).lw(c);
        let target = abi::r32(c, V0);
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, target, 0x003b_04e0).is_err() {
            return;
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `PS2_SONY_CODEC_INTERNAL_v06` (0x003b0fa8): the codec singleton's float
/// (`D_00509508`).
pub fn ps2_sony_codec_internal_v06(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x0051_0000;
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6af8));
    abi::ret(c);
}

/// `PS2_SONY_CODEC_INTERNAL_ReleaseFrame` (0x003b1140): drop a reference to a
/// frame; the last reference unlinks it from the frame ring and appends it
/// to the codec's free list.
pub fn ps2_sony_codec_internal_release_frame(c: &mut Cpu) {
    c.cycles += 5;
    let codec = abi::arg(c, 0);
    let item = abi::arg(c, 1);
    c.g[T0][0] = c.g[A1][0];
    c.g[V0][0] = c.lw(p(item, 0x10));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    let still_used = (c.g[V0][0] as i64) > 0;
    c.sw(p(item, 0x10), c.g[V0][0]);
    if !still_used {
        c.cycles += 16;
        c.g[V1][0] = sx(item.wrapping_add(8));
        c.g[A2][0] = c.lw(p(item, 8));
        c.g[A1][0] = c.lw(p(r(c, V1), 4));
        c.g[A3][0] = 0xb;
        c.g[A0][0] = sx(codec.wrapping_add(0x20));
        c.g[V0][0] = 0;
        c.sw(r(c, A1), c.g[A2][0]);
        if c.g[T0][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        c.sw(p(r(c, A2), 4), c.g[A1][0]);
        c.sw(p(r(c, T0), 8), c.g[A3][0]);
        c.sw(p(r(c, V1), 4), c.g[A3][0]);
        c.g[A1][0] = c.lw(p(r(c, A0), 4));
        c.sw(r(c, A1), c.g[V0][0]);
        c.sw(p(r(c, A0), 4), c.g[V0][0]);
        c.sw(r(c, V0), c.g[A0][0]);
        c.sw(p(r(c, V0), 4), c.g[A1][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `PS2_SONY_CODEC_INTERNAL_NodataCallback` (0x003b0b10): the buy popup's
/// widget notification — hand it to the codec's bitstream feeder and report
/// success.
///
/// The test fixture keeps the callee on its short path (a chunk with no
/// bitstream data, freed through `AV_ChunkInit`, then one DMA chain entry);
/// the feeder's unconditional tail reaches `libmpeg_AddBsDmaChain`, whose
/// `FlushCache`/`DIntr`/`EIntr` calls are kernel no-ops on a fresh machine.
pub fn ps2_sony_codec_internal_nodata_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        // `func_003B0B40(a2, self, a1)`: the popup hands the notification to
        // the bitstream feeder and reports success.
        let ui = c.g[A0][0];
        let target = c.g[A1][0];
        let payload = c.g[A2][0];
        c.g[V0][0] = ui;
        c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
        c.g[V1][0] = target;
        c.g[A0][0] = payload;
        c.sd(r(c, SP), c.g[RA][0]);
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[V1][0];
        callee::FEED_BITSTREAM.call(c, 0x003b_0b30)?;
        c.g[RA][0] = c.ld(r(c, SP));
        c.g[V0][0] = 1;
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    })
}

/// `void tWScriptCache_dtor(tWScriptCache* self, int flags)` (0x3ACA70):
/// release the cached block at `+8` unless a reference is held, then delete
/// the object when bit 0 of `flags` is set.
pub fn wscript_cache_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(ea(c.g[S0][0], 8));
        let no_block = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if no_block {
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0] & 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lh(ea(c.g[S0][0], 2));
            let referenced = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S1][0] & 1;
            if !referenced {
                c.cycles += 2;
                call_ptr(c, addr::MEM_MAN_FREE_NOSIZE, 0x003a_caa4)?;
                c.cycles += 1;
                c.g[V0][0] = c.g[S1][0] & 1;
            }
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, addr::MEM_MAN_FREE, 0x003a_cab8)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Codec / effects.

/// `void PS2_SONY_CODEC_INTERNAL_FreeFrame(Frame* self, int flags)`
/// (0x3b08e0).
pub fn ps2_sony_codec_internal_free_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 4)).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            c.cycles += 3;
            call_ptr(c, 0x003b_2b68, 0x003b_0908)?;
            Addr(p(c.g[S0][0] as u32, 4)).sw(c, 0);
        }
        c.cycles += 5;
        c.g[V0][0] = 0x7;
        c.g[V1][0] = c.g[S1][0] & 1;
        Addr(p(c.g[S0][0] as u32, 0xc)).sw(c, c.g[V0][0]);
        let no_delete = c.g[V1][0] == 0;
        Addr(p(c.g[S0][0] as u32, 8)).sw(c, c.g[V0][0]);
        if !no_delete {
            c.cycles += 4;
            c.g[V1][0] = 0x0051_0000;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x6bcc)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, target, 0x003b_0930)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MPEG2_Flush_Buffer32(BitReader* r)` (0x3b1340): push bytes into the
/// bit buffer until it holds at least 25 bits.
pub fn mpeg2_flush_buffer32(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x0051_0000;
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0x5f78)).lw(c);
    c.g[T2][0] = c.g[V0][0];
    c.g[V1][0] = Addr(p(c.g[A0][0] as u32, 8)).lw(c);
    c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x20));
    c.g[V0][0] = ((c.g[T0][0] as i64) < 25) as u64;
    let no_loop = c.g[V0][0] == 0;
    Addr(p(c.g[A0][0] as u32, 4)).sw(c, 0);
    if !no_loop {
        c.cycles += 2;
        c.g[A3][0] = c.g[A0][0];
        c.g[T1][0] = 0x18;
        loop {
            c.cycles += 12;
            c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
            c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[T0][0] as u32));
            c.g[A0][0] = Addr(p(c.g[A3][0] as u32, 4)).lw(c);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(8));
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
            c.g[A2][0] = ((c.g[T0][0] as i64) < 25) as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A1][0] as u32) & 31));
            Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
            c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
            let more = c.g[A2][0] != 0;
            Addr(p(c.g[A3][0] as u32, 4)).sw(c, c.g[A0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_1368);
            }
        }
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[T2][0] as u32).wrapping_sub(0x5f78)).lw(c);
    let ra = c.g[RA][0] as u32;
    Addr(p(c.g[V0][0] as u32, 8)).sw(c, c.g[T0][0]);
    c.pc = ra;
}

/// `AV_Audio_dtor(self, flags)` 0x3aee30: close the stream, release the
/// sub-object and optionally delete the audio object.
pub fn av_audio_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg64(c, 0);
        let this32 = this as u32;
        frame.save_ra(c, 0);
        c.g[S0][0] = this;
        c.g[V0][0] = lw_at(c, this32, 4);
        let absent = c.g[V0][0] == 0;
        c.g[S1][0] = abi::arg64(c, 1);
        if !absent {
            c.cycles += 7;
            c.g[A0][0] = lw_at(c, this32, 0xc);
            call_ptr(c, 0x003b_7838, 0x003a_ee58)?;
            c.g[V1][0] = 0x510000;
            c.g[V0][0] = Addr(0x0050_9434).lw(c);
            let func = r32(c, V0);
            c.g[A0][0] = lw_at(c, this32, 4);
            call_ptr(c, func, 0x003a_ee68)?;
            Addr(p(this32, 4)).sw(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        let keep = c.g[V0][0] == 0;
        c.g[V1][0] = 0x510000;
        if !keep {
            c.cycles += 3;
            c.g[V0][0] = Addr(0x0050_9434).lw(c);
            let func = r32(c, V0);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, func, 0x003a_ee84)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RCMP_DECODER_ctor(self, key)` (0x3b0430): install the vtable, initialise
/// the callback table and copy the 16-byte key.
pub fn rcmp_decoder_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x450000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x68a8));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 4));
        Addr(p(c.g[S0][0] as u32, 0x18)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x003b_0738, 0x003b_045c)?;
        c.ldl(V0, p(c.g[S1][0] as u32, 7));
        c.ldr(V0, c.g[S1][0] as u32);
        c.ldl(V1, p(c.g[S1][0] as u32, 0xf));
        c.ldr(V1, p(c.g[S1][0] as u32, 8));
        c.sdl(V0, p(c.g[S0][0] as u32, 0xb));
        c.sdr(V0, p(c.g[S0][0] as u32, 4));
        c.sdl(V1, p(c.g[S0][0] as u32, 0x13));
        c.sdr(V1, p(c.g[S0][0] as u32, 0xc));
        c.g[V0][0] = c.g[S0][0];
        Addr(p(c.g[S0][0] as u32, 0x14)).sw(c, 0);
        Addr(c.g[S0][0] as u32).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MPEG2_Flush_Buffer(BitReader* unused, int n)` (0x3b1290): shift the
/// bit-reader buffer left by `n` bits and pull whole bytes in until it holds
/// at least 25 bits.
pub fn mpeg2_flush_buffer(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V1][0] = 0x510000;
    c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x5f78)).lw(c);
    c.g[T2][0] = c.g[V1][0];
    c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 4)).lw(c);
    c.g[V1][0] = Addr(p(c.g[A1][0] as u32, 8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A0][0] as u32) & 31));
    c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V1][0] = ((c.g[T0][0] as i64) < 25i64) as u64;
    let no_loop = c.g[V1][0] == 0;
    Addr(p(c.g[A1][0] as u32, 4)).sw(c, c.g[V0][0]);
    if !no_loop {
        c.cycles += 2;
        c.g[A3][0] = c.g[A1][0];
        c.g[T1][0] = 0x18;
        loop {
            c.cycles += 12;
            c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
            c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[T0][0] as u32));
            c.g[A0][0] = Addr(p(c.g[A3][0] as u32, 4)).lw(c);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(8));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lbu(c);
            c.g[A2][0] = ((c.g[T0][0] as i64) < 25i64) as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A1][0] as u32) & 31));
            Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
            c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
            let more = c.g[A2][0] != 0;
            Addr(p(c.g[A3][0] as u32, 4)).sw(c, c.g[A0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_12c0);
            }
        }
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[T2][0] as u32).wrapping_sub(0x5f78)).lw(c);
    let ra = c.g[RA][0] as u32;
    Addr(p(c.g[V0][0] as u32, 8)).sw(c, c.g[T0][0]);
    c.pc = ra;
}

/// `void RCMP_DECODER_DecodeFrame(RCMP_DECODER* d, void* arg)` (0x003b0600):
/// make sure a picture is queued (starting the decoder when needed), then
/// pass the frame to the current output's `+0x18` virtual method.
pub fn rcmp_decoder_decode_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        let queued = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !queued {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
                c.g[A2][0] = c.g[S0][0];
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x003b_063c)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            }
            c.cycles += 2;
            let no_picture = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if no_picture {
                c.cycles += 1;
                c.lq(S0, sp(c).wrapping_add(0x20));
                c.cycles += 4;
                c.lq(S1, sp(c).wrapping_add(0x10));
                c.g[RA][0] = c.ld(sp(c));
                c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
                abi::ret(c);
                return Ok(());
            }
        }
        c.cycles += 9;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x003b_0664)?;
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.cycles += 4;
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void PS2_SONY_CODEC_INTERNAL_DecodeFrame(Codec* c, int limit)`
/// (0x003b1050): while the MPEG stream is not at its end and pictures keep
/// arriving, decode and deliver them to the caller's `+0x30` method.
pub fn ps2_sony_codec_internal_decode_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A1][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0;
        loop {
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let target = c.g[V1][0] as u32;
                call_ptr(c, target, 0x003b_108c)?;
            }
            c.cycles += 4;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
            call_ptr(c, 0x0040_2b38, 0x003b_1094)?; // sceMpegIsEnd
            let ended = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if ended {
                c.cycles += 1;
                c.lq(S0, sp(c).wrapping_add(0x20));
                c.cycles += 4;
                c.lq(S1, sp(c).wrapping_add(0x10));
                c.g[RA][0] = c.ld(sp(c));
                c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
                abi::ret(c);
                return Ok(());
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003b_0fb8, 0x003b_10a4)?; // PS2_SONY_CODEC_INTERNAL_GetPicture
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = (c.g[V1][0] < c.g[S1][0]) as u64;
            c.g[A1][0] = c.g[V0][0];
            if c.g[V1][0] == 0 {
                c.cycles += 2;
                c.lq(S0, sp(c).wrapping_add(0x20));
                c.cycles += 4;
                c.lq(S1, sp(c).wrapping_add(0x10));
                c.g[RA][0] = c.ld(sp(c));
                c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
                abi::ret(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_1070);
            }
        }
    })
}

/// `void World_ResetLights()` (0x3ad1a8): clear every rider's nearby-light
/// list through the race manager.
pub fn world_reset_lights(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = lw_at_2(c, gw(c, -0x848));
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = lw_at_2(c, p(c.g[V0][0] as u32, 0x84));
        c.g[A0][0] = lw_at_2(c, p(c.g[V1][0] as u32, 0x90));
        call_ptr(c, 0x002f_5998, 0x003a_d1cc)?;
        c.g[V0][0] = lw_at_2(c, gw(c, -0x848));
        c.g[V1][0] = lw_at_2(c, p(c.g[V0][0] as u32, 0x84));
        c.g[S2][0] = lw_at_2(c, p(c.g[V1][0] as u32, 0xc));
        let none = c.g[S2][0] == 0;
        frame.restore(c, 16, 0x30);
        if !none {
            c.cycles += 3;
            c.g[V0][0] = lw_at_2(c, p(c.g[S2][0] as u32, 0x78));
            let no_riders = (c.g[V0][0] as i64) <= 0;
            c.g[S1][0] = 0;
            if !no_riders {
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
                c.g[A0][0] = lw_at_2(c, c.g[S0][0] as u32);
                loop {
                    c.cycles += 7;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                    callee::RIDER_CLEAR_NEARBY_LIGHTS.call(c, 0x003a_d204)?;
                    c.g[V0][0] = lw_at_2(c, p(r32(c, S2), 0x78));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.cycles += 1;
                        frame.restore(c, 16, 0x30);
                        break;
                    }
                    c.g[A0][0] = lw_at_2(c, c.g[S0][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_d1f8);
                    }
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWScriptCache_dtor(cWScriptCache* self, int flags)` (0x3ac8f0):
/// destroy the script nodes of the cache backwards, free the node array,
/// clear the global cache and, with bit 0 of `flags`, delete the object.
pub fn c_w_script_cache_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = flags;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(at_2(c, S1, 4));
        let empty = c.g[A1][0] == 0;
        c.g[V0][0] = 0xc;
        if !empty {
            c.cycles += 5;
            c.g[V1][0] = c.lw(r32(c, A1).wrapping_sub(0x10));
            c.mult(0, 4, 3, 2);
            c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = c.lw(at_2(c, S1, 4));
            } else {
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0xc));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, callee::WSCRIPT_CACHE_NODE_DTOR, 0x003a_c93c)?;
                    c.g[V0][0] = c.lw(at_2(c, S1, 4));
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0xc));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw(at_2(c, S1, 4));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_c930);
                    }
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x10));
            call_ptr(c, callee::OPERATOR_DELETE, 0x003a_c954)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 1;
        let no_free = c.g[V0][0] == 0;
        c.sw(gp_2(c, 0x1720), 0);
        if !no_free {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::OPERATOR_DELETE, 0x003a_c968)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `sWSItem* PS2_SONY_CODEC_INTERNAL_GetPicture(sWSPool* self, ...)`
/// (0x3b0fb8): pop the first free frame, decode into the work buffer and bump
/// the frame counter.
pub fn ps2_sony_codec_internal_get_picture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CODEC_POP_FREE_FRAME.call(c, 0x003b_0fd0)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = lw_3(c, S1, 0x4);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0xc).lw(c);
        c.g[V0][0] = c.g[V0][0] & 0x1000;
        if c.g[V0][0] == 0 {
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 2;
            c.g[V0][0] = lw_3(c, A1, 0x10);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
        c.cycles += 23;
        c.g[A0][0] = lw_3(c, S0, 0x8);
        c.g[A3][0] = u64::MAX;
        c.g[A2][0] = lw_3(c, S0, 0xc);
        c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf));
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A0][0];
        }
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
        c.mult(0, 2, 2, 6);
        c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
        if c.g[A3][0] != 0 {
            c.g[A2][0] = c.g[V0][0];
        }
        c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 4) as u32);
        call_ptr(c, table_callee::CODEC_DECODE_FRAME, 0x003b_1028)?;
        c.g[V1][0] = lw_3(c, S0, 0x10);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S1, 0x10);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        frame.restore_ra(c, 0);
        Addr(c.g[S0][0] as u32).off(0x10).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `MPEG2_picture_spatial_scalable_extension()` (0x3b2230): read the spatial
/// scalability fields (lower/upper layer, priority, horizontal and vertical
/// sub-sampling) from the MPEG-2 bit reader.
pub fn mpeg2_picture_spatial_scalable_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[V0][0] = 0x0051_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(0x0051_0000u32.wrapping_sub(0x5f78)).lw(c);
        c.g[A1][0] = 1;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0xa;
        Addr(c.g[V1][0] as u32).off(0x430).sw(c, c.g[A1][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_2250)?;

        c.g[V1][0] = 0x0051_0000;
        c.g[A0][0] = 0x0049_0000;
        Addr(0x0051_0000u32.wrapping_sub(0x6a08)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(0x0049_0000u32.wrapping_add(0x5598));
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_2264)?;

        c.g[A0][0] = 0xf;
        callee::MPEG2_GET_BITS.call(c, 0x003b_226c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[A0][0] = 0x0051_0000;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 16384) as u64;
        let small = c.g[V0][0] != 0;
        Addr(0x0051_0000u32.wrapping_sub(0x6a04)).sw(c, c.g[V1][0]);
        if !small {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x8000));
            Addr(0x0051_0000u32.wrapping_sub(0x6a04)).sw(c, c.g[V0][0]);
        }

        c.cycles += 10;
        c.g[A0][0] = 0x0049_0000;
        c.g[A0][0] = sx(0x0049_0000u32.wrapping_add(0x55d0));
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_2294)?;
        c.g[A0][0] = 0xf;
        callee::MPEG2_GET_BITS.call(c, 0x003b_229c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[A0][0] = 0x0051_0000;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 16384) as u64;
        let small = c.g[V0][0] != 0;
        Addr(0x0051_0000u32.wrapping_sub(0x6a00)).sw(c, c.g[V1][0]);
        if !small {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x8000));
            Addr(0x0051_0000u32.wrapping_sub(0x6a00)).sw(c, c.g[V0][0]);
        }

        for (bits, off, ret) in [
            (2u32, 0x69fc, 0x003b_22c0u32),
            (1, 0x69f8, 0x003b_22d0),
            (1, 0x69f4, 0x003b_22e0),
        ] {
            c.cycles += if off == 0x69fc { 15 } else { 0 };
            c.g[A0][0] = bits as u64;
            callee::MPEG2_GET_BITS.call(c, ret)?;
            c.g[V1][0] = 0x0051_0000;
            c.g[A0][0] = 1;
            Addr(0x0051_0000u32.wrapping_sub(off)).sw(c, c.g[V0][0]);
            if off == 0x69f4 {
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MPEG2_group_of_pictures_header()` (0x3b16e0): read the group-of-pictures
/// header fields from the bit reader.
pub fn mpeg2_group_of_pictures_header(c: &mut Cpu) {
    'b_3b171c: {
        c.cycles += 7;
        c.g[V0][0] = 0x510000;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5f78));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x69b8));
        let t = c.g[A0][0] != c.g[V1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        if t {
            break 'b_3b171c;
        } else {
            c.cycles += 8;
            c.g[V1][0] = 0x450000;
            c.g[A1][0] = 0x450000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x3c84));
            c.g[A0][0] = 0x450000;
            c.g[V1][0] = 0x1;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw((c.g[A0][0] as u32).wrapping_sub(0x3c80), c.g[V1][0]);
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x3c88), c.g[V0][0]);
            break 'b_3b171c;
        }
    }
    c.cycles += 35;
    c.g[RA][0] = sx(0x003b1724);
    c.g[A0][0] = 0x1;
    c.pc = 0x003b1300;
    ee::s13::MPEG2_Get_Bits(c);
    if c.pc != 0x003b1724 {
        return;
    }
    c.g[V1][0] = 0x510000;
    c.g[A0][0] = 0x5;
    c.g[RA][0] = sx(0x003b1734);
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x69d8), c.g[V0][0]);
    c.pc = 0x003b1300;
    ee::s13::MPEG2_Get_Bits(c);
    if c.pc != 0x003b1734 {
        return;
    }
    c.g[V1][0] = 0x510000;
    c.g[A0][0] = 0x6;
    c.g[RA][0] = sx(0x003b1744);
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x69d4), c.g[V0][0]);
    c.pc = 0x003b1300;
    ee::s13::MPEG2_Get_Bits(c);
    if c.pc != 0x003b1744 {
        return;
    }
    c.g[V1][0] = 0x510000;
    c.g[A0][0] = 0x490000;
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x69d0), c.g[V0][0]);
    c.g[RA][0] = sx(0x003b1758);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5468));
    c.pc = 0x003b2360;
    ee::s13::MPEG2_marker_bit(c);
    if c.pc != 0x003b1758 {
        return;
    }
    c.g[RA][0] = sx(0x003b1760);
    c.g[A0][0] = 0x6;
    c.pc = 0x003b1300;
    ee::s13::MPEG2_Get_Bits(c);
    if c.pc != 0x003b1760 {
        return;
    }
    c.g[V1][0] = 0x510000;
    c.g[A0][0] = 0x6;
    c.g[RA][0] = sx(0x003b1770);
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x69cc), c.g[V0][0]);
    c.pc = 0x003b1300;
    ee::s13::MPEG2_Get_Bits(c);
    if c.pc != 0x003b1770 {
        return;
    }
    c.g[V1][0] = 0x510000;
    c.g[A0][0] = 0x1;
    c.g[RA][0] = sx(0x003b1780);
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x69c8), c.g[V0][0]);
    c.pc = 0x003b1300;
    ee::s13::MPEG2_Get_Bits(c);
    if c.pc != 0x003b1780 {
        return;
    }
    c.g[V1][0] = 0x510000;
    c.g[A0][0] = 0x1;
    c.g[RA][0] = sx(0x003b1790);
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x69c4), c.g[V0][0]);
    c.pc = 0x003b1300;
    ee::s13::MPEG2_Get_Bits(c);
    if c.pc != 0x003b1790 {
        return;
    }
    c.g[V1][0] = 0x510000;
    c.g[RA][0] = sx(0x003b179c);
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x69c0), c.g[V0][0]);
    c.pc = 0x003b1988;
    ee::s13::MPEG2_extension_and_user_data(c);
    if c.pc != 0x003b179c {
        return;
    }
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t;
    return;
}

/// `cWScriptCache_Alloc(self, count)` (0x3ac800): resize the world script
/// cache's block array to `count` entries: destroy and release the old
/// array, allocate a new header and array, construct the entries, then
/// publish it and clear the used count.
pub fn cwscriptcache_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        let _this = abi::arg(c, 0);
        let _count = abi::arg(c, 1);
        frame.save(c, 17, 0x50);
        frame.save(c, 18, 0x40);
        frame.save(c, 19, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 16, 0x60);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, 20, 0x20);
        frame.save(c, 21, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw_4(c, p(c.g[S2][0] as u32, 4));
        let t = c.g[A1][0] == 0;
        c.g[S1][0] = 0x49_0000;
        if !t {
            // destroy the old array back to front, then free its header
            c.cycles += 6;
            c.g[V1][0] = lw_4(c, (c.g[A1][0] as u32).wrapping_sub(0x10));
            c.g[V0][0] = 0xc;
            c.mult(0, A0, V1, V0); // a0 = old_count * 12
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = lw_4(c, p(c.g[S2][0] as u32, 4));
            } else {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[RA][0] = sx(0x003a_c85c);
                    c.g[A0][0] = c.g[S0][0];
                    c.pc = 0x003a_ca70;
                    ee::s13::tWScriptCache_dtor(c);
                    if c.pc != 0x003a_c85c {
                        return Err(Unwound);
                    }
                    c.g[V0][0] = lw_4(c, p(c.g[S2][0] as u32, 4));
                    if c.g[V0][0] != c.g[S0][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_c850);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = lw_4(c, p(c.g[S2][0] as u32, 4));
                    break;
                }
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x003a_c874);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            c.pc = 0x0031_7e98;
            call_ptr(c, 0x0031_7e98, 0x003a_c874)?; // operator delete
        }
        // allocate and construct the new array
        c.cycles += 14;
        c.g[A0][0] = 0xc;
        c.sw(p(c.g[S2][0] as u32, 8), c.g[S3][0]);
        c.mult(0, A0, S3, A0); // a0 = count * 12
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5070));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
        c.g[RA][0] = sx(0x003a_c89c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        c.pc = 0x0031_7e30;
        call_ptr(c, 0x0031_7e30, 0x003a_c89c)?; // operator new
        c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.sw(c.g[V0][0] as u32, c.g[S3][0]);
        let t = c.g[S3][0] == 0;
        c.g[S0][0] = c.g[S4][0];
        if !t {
            c.cycles += 1;
            c.g[S3][0] = sx(0xffff_ffff);
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003a_c8bc);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                c.pc = 0x003a_ca50;
                ee::s13::tWScriptCache_ctor(c);
                if c.pc != 0x003a_c8bc {
                    return Err(Unwound);
                }
                let t = c.g[S1][0] != c.g[S3][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_c8b0);
                }
            }
        }
        c.cycles += 11;
        c.sw(c.g[S5][0] as u32, c.g[S4][0]);
        c.sw(p(c.g[S2][0] as u32, 0xc), 0);
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore(c, 20, 0x20);
        frame.restore(c, 21, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AV_Audio_Init(self, a1, a2)` (0x3aed40): prepare an audio stream cache
/// entry. If the stream module is not up yet, just poke the stream system
/// and return `self`; otherwise clear the entry, build the play parameters,
/// size and open the stream, queue the file, and mark the entry ready.
pub fn av_audio_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let _this = abi::arg(c, 0);
        frame.save(c, 17, 0x50);
        frame.save(c, 18, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 16, 0x60);
        c.sd(p(sp(c), 0x20), c.g[RA][0]);
        c.g[RA][0] = sx(0x003a_ed68);
        c.g[S3][0] = c.g[A2][0];
        c.pc = 0x003b_5440;
        ee::s13::SND_IsInitialized(c);
        if c.pc != 0x003a_ed68 {
            return Err(Unwound);
        }
        let ready = c.g[V0][0] != 0;
        c.g[V0][0] = sx(0xffff_ffff);
        if !ready {
            c.cycles += 4;
            call_ptr(c, 0x003e_62b0, 0x003a_ed78)?;
            c.g[V0][0] = c.g[S1][0];
        } else {
            c.cycles += 37;
            c.sw(p(c.g[S1][0] as u32, 4), 0);
            c.g[A0][0] = c.g[SP][0];
            c.sw(p(c.g[S1][0] as u32, 0xc), c.g[V0][0]);
            c.sw(p(c.g[S1][0] as u32, 0x10), c.g[V0][0]);
            c.g[RA][0] = sx(0x003a_ed94);
            c.pc = 0x003b_a020;
            ee::s13::SND_InitPlayParams(c);
            if c.pc != 0x003a_ed94 {
                return Err(Unwound);
            }
            c.g[A0][0] = 1;
            c.g[RA][0] = sx(0x003a_eda0);
            c.g[A1][0] = 0x1e;
            c.pc = 0x003b_7ac8;
            ee::s13::SNDSTRM_CalcMemSize(c);
            if c.pc != 0x003a_eda0 {
                return Err(Unwound);
            }
            c.g[V1][0] = 0x51_0000;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
            c.g[V0][0] = lw_4(c, (c.g[V1][0] as u32).wrapping_sub(0x6bd0));
            c.g[T0][0] = lw_4(c, p(c.g[A0][0] as u32, 8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = 0x49_0000;
            c.g[A2][0] = 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5250));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003a_edcc);
            c.g[A3][0] = 0;
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x003a_edcc {
                return Err(Unwound);
            }
            c.g[A1][0] = c.g[SP][0];
            c.g[T1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[T0][0] = c.g[V0][0];
            c.g[A2][0] = 1;
            c.g[A3][0] = 0x1e;
            c.g[RA][0] = sx(0x003a_edec);
            c.sw(p(c.g[S1][0] as u32, 4), c.g[V0][0]);
            c.pc = 0x003b_7e38;
            ee::s13::SNDSTRM_OpenEx(c);
            if c.pc != 0x003a_edec {
                return Err(Unwound);
            }
            c.g[A2][0] = c.g[S3][0];
            c.g[A0][0] = c.g[V0][0];
            c.sw(p(c.g[S1][0] as u32, 0xc), c.g[V0][0]);
            c.g[RA][0] = sx(0x003a_ee00);
            c.g[A1][0] = sx(0xffff_ffff);
            c.pc = 0x003b_7de8;
            ee::s13::SNDSTRM_QueueFile(c);
            if c.pc != 0x003a_ee00 {
                return Err(Unwound);
            }
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.sw(p(c.g[S1][0] as u32, 0x10), c.g[V0][0]);
            c.swc1(0, c.g[S1][0] as u32);
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        c.g[RA][0] = c.ld(p(sp(c), 0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `MPEG2_picture_display_extension(&bits)` (0x3b1fc8): read the picture
/// display extension: three picture-level fields and two marker bits per
/// picture, according to the state of the MPEG2 decoder globals, and return
/// how many pictures were read.
pub fn mpeg2_picture_display_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V0][0] = 0x51_0000;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = lw_4(c, (c.g[V0][0] as u32).wrapping_sub(0x6adc));
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        frame.save(c, 19, 0x20);
        frame.save(c, 20, 0x10);
        let t = c.g[V1][0] == 0;
        frame.save_ra(c, 0);
        let count;
        if t {
            // no decoder state: one picture
            c.cycles += 5;
            c.g[V0][0] = 0x51_0000;
            c.g[V1][0] = 3;
            c.g[A0][0] = lw_4(c, (c.g[V0][0] as u32).wrapping_sub(0x6a7c));
            let t = c.g[A0][0] == c.g[V1][0];
            c.g[V1][0] = 0x51_0000;
            if t {
                c.cycles += 3;
                c.g[A1][0] = 2;
                c.g[V0][0] = lw_4(c, (c.g[V1][0] as u32).wrapping_sub(0x6a68));
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = c.g[A0][0];
                }
            } else {
                c.cycles += 2;
                c.g[A1][0] = 1;
            }
            count = c.g[A1][0];
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x51_0000;
            c.g[V1][0] = lw_4(c, (c.g[V0][0] as u32).wrapping_sub(0x6a68));
            let t = c.g[V1][0] == 0;
            c.g[V0][0] = 0x51_0000;
            if t {
                c.cycles += 2;
                c.g[A1][0] = 1;
                count = c.g[A1][0];
            } else {
                c.cycles += 5;
                c.g[A1][0] = 2;
                c.g[A0][0] = lw_4(c, (c.g[V0][0] as u32).wrapping_sub(0x6a78));
                c.g[V1][0] = 3;
                if c.g[A0][0] != 0 {
                    c.g[A1][0] = c.g[V1][0];
                }
                count = c.g[A1][0];
            }
        }
        c.cycles += 2;
        let t = (c.g[A1][0] as i64) <= 0;
        c.g[V1][0] = 0x51_0000;
        if !t {
            c.cycles += 7;
            c.g[V0][0] = 0x51_0000;
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6a30));
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6a40));
            c.g[S0][0] = count;
            c.g[S4][0] = 0x49_0000;
            c.g[S3][0] = 0x49_0000;
            loop {
                c.cycles += 14;
                c.g[A0][0] = 0x10;
                c.g[RA][0] = sx(0x003b_206c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                c.pc = 0x003b_1300;
                ee::s13::MPEG2_Get_Bits(c);
                if c.pc != 0x003b_206c {
                    return Err(Unwound);
                }
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                c.g[RA][0] = sx(0x003b_2078);
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5538));
                c.pc = 0x003b_2360;
                ee::s13::MPEG2_marker_bit(c);
                if c.pc != 0x003b_2078 {
                    return Err(Unwound);
                }
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                c.g[RA][0] = sx(0x003b_2084);
                c.g[A0][0] = 0x10;
                c.pc = 0x003b_1300;
                ee::s13::MPEG2_Get_Bits(c);
                if c.pc != 0x003b_2084 {
                    return Err(Unwound);
                }
                c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                c.g[RA][0] = sx(0x003b_2090);
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5568));
                c.pc = 0x003b_2360;
                ee::s13::MPEG2_marker_bit(c);
                if c.pc != 0x003b_2090 {
                    return Err(Unwound);
                }
                let t = c.g[S0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_2060);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Texture / SHP.

/// `int SHP_CalcAllocSize(int w, int h, int bpp, int flags, int pad0,
/// int pad1, Vec4* mip)` (0x3b26b8): raw image size plus the mip-chain and
/// header overhead, rounded as the original does.
pub fn shp_calc_alloc_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x90));
        c.sq(16, spa(c, 0x80));
        c.sq(18, spa(c, 0x60));
        c.g[S0][0] = c.g[A2][0];
        c.sq(17, spa(c, 0x70));
        c.g[S2][0] = c.g[A0][0];
        c.sq(19, spa(c, 0x50));
        c.g[A0][0] = c.g[S0][0];
        c.sq(20, spa(c, 0x40));
        c.g[S3][0] = c.g[T0][0];
        c.sq(22, spa(c, 0x20));
        c.g[S1][0] = c.g[A1][0];
        c.sq(23, spa(c, 0x10));
        c.g[S4][0] = c.g[A3][0];
        c.sq(21, spa(c, 0x30));
        c.g[S6][0] = c.g[T1][0];
        c.sd(spa(c, 0), c.g[RA][0]);
        c.g[RA][0] = sx(0x003b_2704);
        c.g[S7][0] = c.g[T2][0];
        callee::SHP_NORMALIZE_BPP.call(c, 0x003b_2704)?;
        c.g[S5][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x003b_271c);
        c.g[A3][0] = c.g[S3][0];
        callee::SHP_CALC_IMAGE_SIZE.call(c, 0x003b_271c)?;
        c.g[S0][0] = sx(r32v(c, V0).wrapping_add(0x10));
        c.g[V0][0] = ((c.g[S5][0] as i64) < 9) as u64;
        let plain_bpp = c.g[V0][0] == 0;
        if !plain_bpp {
            c.cycles += 2;
            let no_flags = c.g[S4][0] == 0;
            c.g[V1][0] = 0x10;
            if !no_flags {
                // Palette alignment: keep 16-byte blocks, at least one.
                c.cycles += 3;
                c.g[V0][0] = c.g[S4][0] ^ 0xf;
                let no_blocks = c.g[S0][0] == 0;
                if c.g[V0][0] == 0 {
                    c.g[S4][0] = c.g[V1][0];
                }
                if !no_blocks {
                    c.cycles += 3;
                    c.g[V1][0] = sx(r32v(c, S0).wrapping_add(0xf));
                    c.g[V0][0] = 0xffff_ffff_ffff_fff0;
                    c.g[S0][0] = c.g[V1][0] & c.g[V0][0];
                }
                c.cycles += 13;
                c.g[RA][0] = sx(0x003b_2754);
                c.g[A0][0] = c.g[S5][0];
                callee::SHP_PALETTE_ENTRIES_FOR_BPP.call(c, 0x003b_2754)?;
                // v0 = v0 * flags (the palette bytes), rounded up to 8.
                c.mult(0, 2, 2, 20);
                c.g[A1][0] = sx(r32v(c, S0).wrapping_add(0xf));
                c.g[V1][0] = 0xffff_ffff_ffff_fff0;
                c.g[A0][0] = 0xffff_ffff_ffff_ffff;
                c.g[A1][0] = c.g[A1][0] & c.g[V1][0];
                c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[V1][0] = sx(r32v(c, V0).wrapping_add(7));
                if c.g[A0][0] != 0 {
                    c.g[V1][0] = c.g[V0][0];
                }
                c.g[V1][0] = sx(((r32v(c, V1) as i32) >> 3) as u32);
                c.g[V1][0] = sx(r32v(c, V1).wrapping_add(0x10));
                c.g[S0][0] = sx(r32v(c, A1).wrapping_add(r32v(c, V1)));
            }
        }
        // Mip chain and the two optional header blocks.
        c.cycles += 2;
        let no_mip = c.g[S6][0] == 0;
        c.g[V0][0] = sx(r32v(c, S0).wrapping_add(8));
        if !no_mip {
            c.cycles += 1;
            c.g[S0][0] = sx(r32v(c, V0).wrapping_add(r32v(c, S6)));
        }
        c.cycles += 2;
        let no_tail = c.g[S7][0] == 0;
        c.g[V0][0] = sx(r32v(c, S0).wrapping_add(0x10));
        if !no_tail {
            c.cycles += 1;
            c.g[S0][0] = sx(r32v(c, V0).wrapping_add(r32v(c, S7)));
        }
        c.cycles += 12;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, spa(c, 0x70));
        c.lq(16, spa(c, 0x80));
        c.lq(18, spa(c, 0x60));
        c.lq(19, spa(c, 0x50));
        c.lq(20, spa(c, 0x40));
        c.lq(21, spa(c, 0x30));
        c.lq(22, spa(c, 0x20));
        c.lq(23, spa(c, 0x10));
        c.g[RA][0] = c.ld(spa(c, 0));
        c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x90));
        abi::ret(c);
        Ok(())
    })
}

/// `PS2_SONY_CODEC_INTERNAL_FeedBitstream(self, stream)` (0x003b0b40): take a
/// chunk from the MPEG decoder, copy it into the bit reservoir, clear the ring
/// slots not covered by the chunk and hand it to `sceMpegAddBs`.
pub fn ps2_sony_codec_internal_feed_bitstream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0);
        let decoder = c.g[S1][0];
        c.g[A0][0] = lw_6(c, decoder, 0x28);
        Callee {
            addr: 0x003b_06b0,
            f: ee::s13::RCMP_DECODER_GetChunk,
        }
        .call(c, 0x003b_0b70)?;
        c.g[S3][0] = c.g[V0][0];
        if c.g[S3][0] != 0 {
            c.g[S2][0] = lw_6(c, c.g[S3][0], 0x4);
        }

        c.cycles += 2;
        c.g[V1][0] = 0xffff_ffff_ffff_fff0;
        if c.g[S2][0] == 0 {
            // No usable chunk: clear the first four ring slots.
            c.cycles += 3;
            c.g[S2][0] = 0x10;
            c.g[S0][0] = 0;
            c.g[A0][0] = sx(0xb701_0000);
            loop {
                c.cycles += 7;
                let dest = lw_6(c, decoder, 0x7c).wrapping_add(c.g[S0][0] & 0xffff_ffff);
                c.g[V0][0] = sx(dest as u32);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                c.g[V1][0] = ((c.g[S0][0] as i64) < 16) as u64;
                let more = c.g[V1][0] != 0;
                sw_3(c, c.g[V0][0], 0, c.g[A0][0]);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_0be8);
                }
            }
        } else {
            // Copy the chunk into the bit reservoir and clear the rest of the
            // ring it spans.
            c.cycles += 12;
            c.g[S0][0] = lw_6(c, c.g[S3][0], 0x8);
            c.g[A0][0] = lw_6(c, decoder, 0x7c);
            c.g[A1][0] = c.g[S2][0];
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(7));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(8));
            c.g[S2][0] = c.g[V0][0] & 0xffff_ffff_ffff_fff0;
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x003e_6574, 0x003b_0ba4)?;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = lw_6(c, decoder, 0x7c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    let more = c.g[V1][0] != 0;
                    sw_3(c, c.g[V0][0], 0, 0);
                    if !more {
                        c.cycles += 2;
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_0bb8);
                    }
                }
            }
        }

        c.cycles += 2;
        if c.g[S3][0] == 0 {
            c.g[V0][0] = lw_6(c, decoder, 0x14);
        } else {
            c.cycles += 4;
            c.g[A0][0] = lw_6(c, decoder, 0x28);
            c.g[A1][0] = c.g[S3][0];
            Callee {
                addr: 0x003b_06f8,
                f: ee::s13::RCMP_DECODER_ReleaseChunk,
            }
            .call(c, 0x003b_0c18)?;
            c.g[V0][0] = lw_6(c, decoder, 0x14);
        }

        c.cycles += 14;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = lw_6(c, decoder, 0x78);
        c.g[A2][0] = c.g[S2][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        sw_3(c, decoder, 0x14, c.g[V0][0]);
        call_ptr(c, 0x0040_29d0, 0x003b_0c34)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_003B1AB0(void)` (0x3b1ab0): decode an MPEG-2 sequence-extension
/// header: read the extension start code's fields (horizontal/vertical/
/// bit-rate extensions, the display-extension flags) into the sequence
/// globals and recompute the frame rate.
pub fn mpeg2_sequence_extension(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_3b1c28: {
        'b_3b1c0c: {
            c.cycles += 76;
            c.g[V0][0] = 0x510000;
            c.g[V1][0] = lw_7(c, r32(c, V0).wrapping_sub(0x5f78));
            c.g[A1][0] = 0x1;
            frame.save(c, S0, 0x60);
            c.g[A2][0] = 0x510000;
            frame.save(c, S1, 0x50);
            c.g[A0][0] = 0x8;
            frame.save(c, S2, 0x40);
            c.g[S0][0] = 0x510000;
            frame.save(c, S3, 0x30);
            c.g[S1][0] = 0x510000;
            frame.save(c, S4, 0x20);
            frame.save(c, S5, 0x10);
            sd(c, r32(c, SP), c.g[RA][0]);
            sw_4(c, at_5(c, V1, 0x420), c.g[A1][0]);
            sw_4(c, at_5(c, V1, 0x424), 0u64);
            c.g[RA][0] = sx(0x003b1afc);
            sw_4(c, r32(c, A2).wrapping_sub(0x6a24), 0u64);
            if callee::C003B1300.call(c, 0x003b1afc).is_err() {
                return;
            }
            c.g[A0][0] = 0x1;
            c.g[RA][0] = sx(0x003b1b08);
            sw_4(c, r32(c, S0).wrapping_sub(0x6ae0), c.g[V0][0]);
            if callee::C003B1300.call(c, 0x003b1b08).is_err() {
                return;
            }
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 0x2;
            c.g[RA][0] = sx(0x003b1b18);
            sw_4(c, r32(c, V1).wrapping_sub(0x6adc), c.g[V0][0]);
            if callee::C003B1300.call(c, 0x003b1b18).is_err() {
                return;
            }
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 0x2;
            c.g[RA][0] = sx(0x003b1b28);
            sw_4(c, r32(c, V1).wrapping_sub(0x6ad8), c.g[V0][0]);
            if callee::C003B1300.call(c, 0x003b1b28).is_err() {
                return;
            }
            c.g[A0][0] = 0x2;
            c.g[RA][0] = sx(0x003b1b34);
            c.g[S4][0] = c.g[V0][0];
            if callee::C003B1300.call(c, 0x003b1b34).is_err() {
                return;
            }
            c.g[A0][0] = 0xc;
            c.g[RA][0] = sx(0x003b1b40);
            c.g[S5][0] = c.g[V0][0];
            if callee::C003B1300.call(c, 0x003b1b40).is_err() {
                return;
            }
            c.g[A0][0] = 0x490000;
            c.g[S2][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003b1b50);
            c.g[A0][0] = sx(at_5(c, A0, 0x54b8));
            if callee::C003B2360.call(c, 0x003b1b50).is_err() {
                return;
            }
            c.g[RA][0] = sx(0x003b1b58);
            c.g[A0][0] = 0x8;
            if callee::C003B1300.call(c, 0x003b1b58).is_err() {
                return;
            }
            c.g[A0][0] = 0x1;
            c.g[RA][0] = sx(0x003b1b64);
            c.g[S3][0] = c.g[V0][0];
            if callee::C003B1300.call(c, 0x003b1b64).is_err() {
                return;
            }
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 0x2;
            c.g[RA][0] = sx(0x003b1b74);
            sw_4(c, r32(c, V1).wrapping_sub(0x6ad4), c.g[V0][0]);
            if callee::C003B1300.call(c, 0x003b1b74).is_err() {
                return;
            }
            c.g[A0][0] = 0x5;
            c.g[RA][0] = sx(0x003b1b80);
            sw_4(c, r32(c, S1).wrapping_sub(0x6ad0), c.g[V0][0]);
            if callee::C003B1300.call(c, 0x003b1b80).is_err() {
                return;
            }
            c.g[A1][0] = lw_7(c, r32(c, S1).wrapping_sub(0x6ad0));
            c.g[V1][0] = sx(at_5(c, V0, 0x1));
            c.mtc1(3, 2);
            c.cvt_s_w(2, 2);
            c.g[A2][0] = 0x510000;
            c.g[A1][0] = sx(at_5(c, A1, 0x1));
            c.g[A0][0] = lw_7(c, r32(c, A2).wrapping_sub(0x6af0));
            c.mtc1(5, 1);
            c.cvt_s_w(1, 1);
            c.g[V1][0] = 0x450000;
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x3c78));
            c.g[A0][0] = sx(r32(c, A0) << 2);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            c.g[A3][0] = lw_7(c, r32(c, S0).wrapping_sub(0x6ae0));
            c.div_s(1, 1, 2);
            c.lwc1(0, r32(c, A0));
            c.g[A1][0] = 0x510000;
            c.g[A2][0] = 0x510000;
            c.g[V1][0] = sx((r32(c, A3) as i32 >> 7) as u32);
            sw_4(c, r32(c, A1).wrapping_sub(0x6acc), c.g[V0][0]);
            c.g[V1][0] = c.g[V1][0] & 0x1;
            c.mul_s(0, 0, 1);
            let t = c.g[V1][0] == 0u64;
            c.swc1(0, r32(c, A2).wrapping_sub(0x6af8));
            if t {
                break 'b_3b1c0c;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.g[A3][0] & 0xf;
                c.g[V0][0] = 0x5;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[T4][0] = 0x510000;
                if t {
                    break 'b_3b1c28;
                } else {
                    c.cycles += 7;
                    c.g[A1][0] = 0x510000;
                    c.g[V1][0] = 0x85;
                    c.g[A0][0] = 0x510000;
                    c.g[V0][0] = 0x8;
                    sw_4(c, r32(c, A1).wrapping_sub(0x6b14), c.g[V1][0]);
                    let t = true;
                    sw_4(c, r32(c, A0).wrapping_sub(0x6b10), c.g[V0][0]);
                    if t {
                        break 'b_3b1c28;
                    } else {
                        break 'b_3b1c0c;
                    }
                }
            }
        }
        c.cycles += 7;
        c.g[A1][0] = c.g[A3][0] & 0xf;
        c.g[A0][0] = 0x510000;
        c.g[V0][0] = sx((r32(c, A3) as i32 >> 4) as u32);
        c.g[V1][0] = 0x510000;
        sw_4(c, r32(c, A0).wrapping_sub(0x6b14), c.g[V0][0]);
        sw_4(c, r32(c, V1).wrapping_sub(0x6b10), c.g[A1][0]);
        c.g[T4][0] = 0x510000;
        break 'b_3b1c28;
    }
    c.cycles += 37;
    c.g[V0][0] = sx(r32(c, S2) << 18);
    c.g[T0][0] = lw_7(c, r32(c, T4).wrapping_sub(0x6aec));
    c.g[T3][0] = 0x510000;
    c.g[T2][0] = 0x510000;
    c.g[V1][0] = lw_7(c, r32(c, T3).wrapping_sub(0x6b08));
    c.g[T0][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
    c.g[AT][0] = 0x43c80000;
    c.mtc1(1, 1);
    c.mtc1(8, 0);
    c.cvt_s_w(0, 0);
    c.g[A0][0] = lw_7(c, r32(c, T2).wrapping_sub(0x6b0c));
    c.g[T1][0] = 0x510000;
    c.g[V0][0] = sx(r32(c, S5) << 12);
    c.g[A3][0] = lw_7(c, r32(c, T1).wrapping_sub(0x6ae8));
    c.g[V1][0] = c.g[V1][0] & 0xfff;
    c.mul_s(0, 0, 1);
    c.g[A2][0] = sx(r32(c, S3) << 10);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[A1][0] = sx(r32(c, S4) << 12);
    c.g[A0][0] = c.g[A0][0] & 0xfff;
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, A2)));
    c.g[A1][0] = c.g[A1][0] | c.g[A0][0];
    c.g[V1][0] = 0x510000;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    sw_4(c, r32(c, T2).wrapping_sub(0x6b0c), c.g[A1][0]);
    sw_4(c, r32(c, T3).wrapping_sub(0x6b08), c.g[V0][0]);
    c.swc1(0, r32(c, V1).wrapping_sub(0x6afc));
    sw_4(c, r32(c, T1).wrapping_sub(0x6ae8), c.g[A3][0]);
    sw_4(c, r32(c, T4).wrapping_sub(0x6aec), c.g[T0][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_5(c, SP, 0x70));
    c.pc = t;
    return;
}

// ------------------------------------------------ tWScriptCache_GetObjectSlot

/// `int tWScriptCache_GetObjectSlot(sEntry* entry, Object* obj, int i)`
/// (0x3ad120): an object's cached script slot `i`. The object's script
/// (`obj+0x88`) carries an id (`script+8`, -1 = none) whose upper bits
/// (`id >> 8`) index the entry's slot table (`entry+8` -> table, rows of
/// 0x18 bytes at `table+0x1c`). Returns -1 when the object has no script,
/// the entry no table, or the script no id.
pub fn wscript_cache_get_object_slot(c: &mut Cpu) {
    const OBJ_SCRIPT: u32 = 0x88;
    const ENTRY_TABLE: u32 = 0x8;
    const SCRIPT_ID: u32 = 0x8;
    const TABLE_ROWS: u32 = 0x1c;
    const ROW_SIZE: u64 = 0x18;

    c.cycles += 3;
    let script = Addr(abi::arg(c, 1)).off(OBJ_SCRIPT).lw(c);
    c.g[V1][0] = script;
    if script != 0 {
        c.cycles += 3;
        let table = Addr(abi::arg(c, 0)).off(ENTRY_TABLE).lw(c);
        c.g[V0][0] = table;
        c.g[A0][0] = table;
        if table != 0 {
            c.cycles += 4;
            let id = Addr(script as u32).off(SCRIPT_ID).lw(c);
            c.g[V1][0] = id;
            if id != u64::MAX {
                c.cycles += 9;
                c.g[V0][0] = sx((id as u32) >> 8);
                let rows = Addr(table as u32).off(TABLE_ROWS).lw(c);
                c.g[A0][0] = rows;
                c.g[V1][0] = ROW_SIZE;
                c.mult(0, V0, V0, V1); // row index * 0x18 (also LO/HI)
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(rows as u32));
                let slot = (c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32);
                c.g[V1][0] = sx(slot);
                c.g[V0][0] = Addr(slot).lw(c);
                ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            ret(c);
            return;
        }
    }
    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    ret(c);
}

/// `void World_ChunkResolveHook(void)` (0x3ad230): forwards to
/// `LensFX_GetFlareSizeForRes` (0x2e27e8) through the function table.
pub fn world_chunk_resolve_hook(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Table(0x002e_27e8),
        0x003a_d240,
    );
}

// ---------------------------------------------------------------------------
// tWScriptCache_Relocate.

/// `tWScriptCache_Relocate`'s inline record lookup (the "resolve" the C++
/// reference calls): the id in `t0` is `slot | index << 8`; `*(s1)` is the
/// world cache whose slot table (+8) holds records; the record's table at
/// `table_off` maps `index` to `value << 8`; the result (`value << 2`, 0 when
/// absent) lands in `a2`. `head` is the cycle charge of the first block;
/// `load_cache` is whether that block also loads the cache pointer into `a2`.
/// `a3` is the running index.
pub(crate) fn oc420_cache_record(c: &mut Cpu, head: u64, table_off: u32, load_cache: bool) {
    c.cycles += head;
    if load_cache {
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
    }
    c.g[A1][0] = c.g[T0][0] & 0xff;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[A3][0] as u32) << 8);
    c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
    let present = c.g[V0][0] != 0;
    c.g[T0][0] = c.g[A1][0] | c.g[V1][0];
    if !present {
        c.cycles += 2;
        c.g[A2][0] = 0;
    } else {
        c.cycles += 8;
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(table_off)).lw(c);
        c.g[V1][0] = sx((c.g[T0][0] as u32) >> 8);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32) >> 8);
        let found = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 1;
        c.g[A2][0] = c.g[V0][0];
    }
}

/// `*(reg + off) += reg` where `reg` (`v1` or `a0`) holds the script data
/// base (reloaded from `entry + 8` first when `reload`): turns an offset
/// into a pointer.
pub(crate) fn oc420_rebase(c: &mut Cpu, entry: u32, reg: usize, off: u32, reload: bool) {
    if reload {
        c.g[reg][0] = Addr(entry.wrapping_add(8)).lw(c);
    }
    let base = c.g[reg][0] as u32;
    c.g[V0][0] = Addr(base.wrapping_add(off)).lw(c);
    c.g[V0][0] = sx(base.wrapping_add(c.g[V0][0] as u32));
    Addr(base.wrapping_add(off)).sw(c, c.g[V0][0]);
}

/// `void tWScriptCache_Relocate(Entry* e, World** wp, int index)`
/// (0x3accd8): turns a freshly loaded world script block (`e + 8`, header
/// `0x1000` and the world id at +0xc) from file offsets into pointers: the
/// ten table pointers, the entries of the +0x3c and +0x50 tables, then
/// records the script data refers to are resolved in the world cache (the
/// track records at +0x88/+0x8 for the +0x44 count, the type 1/3 table entries
/// at +0xc for the +0x4c count, the spline records at +0x28/+0x1c for the +0x54
/// count) and the two tables at +0x24/+0x20 and +0x2c/+0x28 are relocated.
/// A block with the wrong header or world is dropped (freed when the entry
/// owns it); the entry's state word becomes 3 either way.
pub fn t_wscript_cache_relocate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let entry = c.g[S0][0] as u32;
        c.g[A3][0] = Addr(entry.wrapping_add(8)).lw(c);
        c.g[S2][0] = c.g[A2][0];
        'finish: {
            'done: {
                if c.g[A3][0] == 0 {
                    break 'done;
                }
                c.cycles += 4;
                c.g[V1][0] = Addr(c.g[A3][0] as u32).lhu(c);
                c.g[V0][0] = 0x1000;
                let mut keep = false;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 6;
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(4)).lw(c);
                    if c.g[A1][0] == c.g[V1][0] {
                        c.g[V1][0] = Addr(entry.wrapping_add(8)).lw(c);
                        keep = true;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(entry.wrapping_add(2)).lh(c);
                    }
                } else {
                    c.g[V0][0] = Addr(entry.wrapping_add(2)).lh(c);
                }
                if !keep {
                    // Wrong block: free it when the entry owns it, and forget it.
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        Addr(entry.wrapping_add(8)).sw(c, 0);
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[A3][0];
                        call_ptr(c, 0x0031_7e98, 0x003a_cd3c)?;
                        Addr(entry.wrapping_add(8)).sw(c, 0);
                    }
                    c.cycles += 1;
                    c.g[V1][0] = Addr(entry.wrapping_add(8)).lw(c);
                }
                c.cycles += 2;
                let none = c.g[V1][0] == 0;
                c.g[V0][0] = 3;
                if none {
                    break 'finish;
                }
                // The ten table pointers.
                c.cycles += 44;
                c.g[A1][0] = 0;
                for (reg, off, reload) in [
                    (V1, 0x14u32, false),
                    (V1, 0x1c, true),
                    (A0, 0x24, true),
                    (V1, 0x2c, true),
                    (A0, 0x34, true),
                    (V1, 0x3c, true),
                    (A0, 0x40, true),
                    (V1, 0x48, true),
                    (A0, 0x50, true),
                    (V1, 0x58, true),
                ] {
                    oc420_rebase(c, entry, reg, off, reload);
                }
                c.g[A0][0] = Addr(entry.wrapping_add(8)).lw(c);
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x38)).lw(c);
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x3c)).lw(c);
                if !empty {
                    // Entries of the +0x3c table (count at +0x38).
                    c.cycles += 1;
                    loop {
                        c.cycles += 9;
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
                        c.g[A0][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x38)).lw(c);
                        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                        if !more {
                            c.cycles += 1;
                            c.g[A0][0] = Addr(entry.wrapping_add(8)).lw(c);
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_ce00);
                        }
                    }
                }
                // Entries of the +0x50 table (count at +0x4c) get the +0x40 base.
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4c)).lw(c);
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[A2][0] = 0;
                if !empty {
                    c.cycles += 1;
                    c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x50)).lw(c);
                    loop {
                        c.cycles += 12;
                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x40)).lw(c);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        Addr(c.g[V1][0] as u32).sw(c, c.g[A0][0]);
                        c.g[A0][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4c)).lw(c);
                        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x50)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_ce38);
                        }
                    }
                }
                // Records of the world cache for the +0x44 track entries.
                c.cycles += 7;
                c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                c.g[T0][0] = 0xffff_ffff_ffff_0000;
                c.g[A3][0] = 0;
                c.g[T0][0] |= 0xff00;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x44)).lw(c);
                let empty = (c.g[V1][0] as i64) <= 0;
                c.g[T0][0] |= c.g[S2][0];
                if !empty {
                    c.cycles += 3;
                    c.g[T1][0] = 0xffff_ffff_ffff_0000;
                    c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
                    loop {
                        oc420_cache_record(c, 8, 0x1c, false);
                        c.cycles += 22;
                        c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[A0][0] = sx((c.g[A3][0] as u32) << 1);
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lw(c);
                        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x50)).lw(c);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                        Addr((c.g[A2][0] as u32).wrapping_add(0x88)).sw(c, c.g[V1][0]);
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
                        c.g[V0][0] &= c.g[T1][0];
                        c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                        c.g[V0][0] |= c.g[V1][0];
                        c.g[V0][0] |= 2;
                        Addr((c.g[A2][0] as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
                        c.g[V1][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x44)).lw(c);
                        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_ce90);
                        }
                    }
                }
                // Table entries of type 1 and 3 point at records (+0xc), others at 0.
                c.cycles += 4;
                c.g[V1][0] = Addr(entry.wrapping_add(8)).lw(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4c)).lw(c);
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[A3][0] = 0;
                if !empty {
                    c.cycles += 4;
                    c.g[T2][0] = 1;
                    c.g[T1][0] = 3;
                    c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x50)).lw(c);
                    loop {
                        c.cycles += 6;
                        c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                        let mut resolve = true;
                        if c.g[V0][0] == c.g[T2][0] {
                            c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[T1][0] {
                                Addr((c.g[A2][0] as u32).wrapping_add(0xc)).sw(c, 0);
                                resolve = false;
                            } else {
                                c.cycles += 1;
                                c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
                            }
                        }
                        if resolve {
                            c.cycles += 8;
                            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                            c.g[V1][0] = c.g[A1][0] & 0xff;
                            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lw(c);
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
                            let present = c.g[V1][0] != 0;
                            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                            let mut found = false;
                            if present {
                                c.cycles += 7;
                                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x64)).lw(c);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                                found = c.g[V0][0] != 0;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            }
                            if !found {
                                c.cycles += 1;
                                c.g[V0][0] = 0;
                            }
                            c.cycles += 1;
                            Addr((c.g[A2][0] as u32).wrapping_add(0xc)).sw(c, c.g[V0][0]);
                        }
                        c.cycles += 6;
                        c.g[V1][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4c)).lw(c);
                        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x50)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_cf58);
                        }
                    }
                }
                // Spline records (+0x54 of them): two halfwords each from +0x58.
                c.cycles += 4;
                c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                c.g[T1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x54)).lw(c);
                let empty = (c.g[T1][0] as i64) <= 0;
                c.g[A3][0] = 0;
                if !empty {
                    c.cycles += 2;
                    c.g[T2][0] = 0xffff_ffff_ffff_0000;
                    loop {
                        oc420_cache_record(c, 9, 0x44, true);
                        c.cycles += 21;
                        c.g[A0][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x58)).lw(c);
                        c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.lwl(3, (c.g[V0][0] as u32).wrapping_add(3));
                        c.lwr(3, c.g[V0][0] as u32);
                        c.swl(3, sp(c).wrapping_add(3));
                        c.swr(3, sp(c));
                        c.g[V1][0] = Addr(sp(c).wrapping_add(2)).lhu(c);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
                        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
                        Addr((c.g[A2][0] as u32).wrapping_add(0x28)).sw(c, c.g[V1][0]);
                        c.g[V0][0] = Addr(sp(c)).lh(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 16);
                        c.g[V0][0] &= c.g[T2][0];
                        c.g[V0][0] |= c.g[V1][0];
                        let more = c.g[A1][0] != 0;
                        Addr((c.g[A2][0] as u32).wrapping_add(0x1c)).sw(c, c.g[V0][0]);
                        if !more {
                            c.cycles += 1;
                            c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_cff0);
                        }
                    }
                }
                // Relocate `+0x28` records (0x40 bytes each, field +8 at +0x2c).
                c.cycles += 3;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
                let empty = (c.g[V1][0] as i64) <= 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2c)).lw(c);
                if !empty {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[V1][0];
                    loop {
                        c.cycles += 7;
                        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                        c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        Addr((c.g[A0][0] as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
                        let more = c.g[A1][0] != 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_d0b0);
                        }
                    }
                }
                // Relocate the `+0x20` records (0x54 bytes each, field +8 at +0x24).
                c.cycles += 4;
                c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lw(c);
                let empty = (c.g[V1][0] as i64) <= 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
                if !empty {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[V1][0];
                    loop {
                        c.cycles += 7;
                        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                        c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        Addr((c.g[A0][0] as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
                        let more = c.g[A1][0] != 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x54));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_d0e0);
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 3;
        }
        c.cycles += 7;
        Addr(entry).sh(c, c.g[V0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int tWScriptCache_GetGlobals(tWScriptCacheEntry* self)` (0x3adcb0): the entry's global
/// script pointer (`+0x14` of its data at `+0x8`) when the entry has type 3 and data, else 0.
pub fn w_script_cache_get_globals(c: &mut Cpu) {
    'b_3adcd4: {
        c.cycles += 4;
        c.g[V0][0] = c.lh(c.g[A0][0] as u32);
        c.g[V1][0] = 0x3;
        let t = c.g[V0][0] != c.g[V1][0];
        if t {
            break 'b_3adcd4;
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
            let t = c.g[A0][0] == 0u64;
            if t {
                break 'b_3adcd4;
            } else {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                c.pc = t;
                return;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
}

/// `int tWScriptCache_GetGlobalCount(tWScriptCache *self)` (0x3adc80): when
/// the cache kind (`+0`, halfword) is 3 and its data (`+8`) exists, the data's
/// word at `+0x10`; else 0.
pub fn t_w_script_cache_get_global_count(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, A0)).lh(c);
    c.g[V1][0] = 3;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, A0).wrapping_add(8)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x10)).lw(c);
            ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// `void World_StaticInitDestroy(int prio, int init)` (0x3ad290). Does nothing
/// unless `prio == 0xFFFF` and the object pointer `a0` is non-zero; then it
/// rewrites the world's static tables: float4 rows at 0x509080..0x5093f0, the
/// small blocks at 0x4a5a70..0x4a5ae0 and 0x509400..0x509420, and the
/// GP-relative flags at gp+0x2980..gp+0x29f8. The table stores keep the
/// generated order, so every register (scratch ones included) and the cycle
/// count match the generated body; the three exits charge 25 / 23 / 609 cycles.
pub fn world_static_init_destroy(c: &mut Cpu) {
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
    c.g[V0][0] = 0xffff;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
    let other_prio = c.g[A1][0] != c.g[V0][0];
    c.sq(30, c.g[SP][0] as u32);
    if other_prio {
        return world_static_init_destroy_exit(c);
    }
    c.cycles += 2;
    let no_object = c.g[A0][0] == 0u64;
    c.g[V0][0] = 0x510000;
    if no_object {
        return world_static_init_destroy_exit(c);
    }
    c.cycles += 584;
    // The table stores, in generated order (no calls: initE/initD are inlined).
    c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x23ac));
    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x23a8));
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6f80));
    c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x23a4));
    c.g[T5][0] = 0x510000;
    c.g[T6][0] = 0x510000;
    c.g[T7][0] = 0x510000;
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x23a0));
    c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x6f50));
    c.swc1(9, (c.g[T5][0] as u32).wrapping_sub(0x6f70));
    c.g[S2][0] = 0x510000;
    c.swc1(10, (c.g[T6][0] as u32).wrapping_sub(0x6f60));
    c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6f30));
    c.swc1(11, (c.g[T7][0] as u32).wrapping_sub(0x6f50));
    c.g[S1][0] = 0x510000;
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x239c));
    c.g[S3][0] = 0x510000;
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc));
    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6f40));
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x2398));
    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6f20));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x2394));
    c.g[T3][0] = 0x510000;
    c.swc1(4, (c.g[S2][0] as u32).wrapping_sub(0x6f30));
    c.g[S6][0] = 0x510000;
    c.swc1(8, (c.g[S1][0] as u32).wrapping_sub(0x6f40));
    c.g[S2][0] = 0x510000;
    c.swc1(5, (c.g[S3][0] as u32).wrapping_sub(0x6f20));
    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6ea0));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2390));
    c.g[S3][0] = 0x510000;
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x238c));
    c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6e90));
    c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
    c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6f00));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    c.g[S3][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6e80));
    c.swc1(1, (c.g[T3][0] as u32).wrapping_sub(0x6f00));
    c.g[S6][0] = 0x510000;
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2388));
    c.g[T3][0] = 0x510000;
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 6);
    c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x6f70));
    c.swc1(3, (c.g[S6][0] as u32).wrapping_sub(0x6f10));
    c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x6f60));
    c.swc1(0, (c.g[T3][0] as u32).wrapping_sub(0x6f80));
    c.g[T2][0] = 0x510000;
    c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x6f10));
    c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
    c.g[T9][0] = 0x510000;
    c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x2384));
    c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_sub(0x6ef0));
    c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x2380));
    c.g[S4][0] = 0x510000;
    c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
    c.g[T7][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6ec0));
    c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x237c));
    c.g[S6][0] = 0x510000;
    c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x2378));
    c.g[T3][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6e70));
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.g[T8][0] = 0x510000;
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[S6][0] = 0x510000;
    c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_sub(0x6ee0));
    c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[S5][0] = 0x510000;
    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
    c.g[T6][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x6ed0));
    c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x2374));
    c.g[V1][0] = 0x510000;
    c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x2370));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6e30));
    c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
    c.g[S0][0] = 0x510000;
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x236c));
    c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
    c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
    c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = 0x510000;
    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6e10));
    c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6e00));
    c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
    c.g[S6][0] = 0x510000;
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
    c.g[FP][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6df0));
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
    c.g[S6][0] = 0x510000;
    c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = 0x510000;
    c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
    c.g[T1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6db0));
    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
    c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x2368));
    c.g[T0][0] = 0x510000;
    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.g[S7][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6dc0));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2364));
    c.g[A2][0] = 0x510000;
    c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6de0));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2360));
    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6dd0));
    c.swc1(2, (c.g[T9][0] as u32).wrapping_sub(0x6ef0));
    c.g[T0][0] = 0x510000;
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x235c));
    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6d80));
    c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
    c.g[T0][0] = 0x510000;
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2358));
    c.g[S6][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6d70));
    c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0xc));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x2354));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x2350));
    c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
    c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
    c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
    c.g[T4][0] = c.g[S0][0];
    c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x6eb0));
    c.swc1(0, (c.g[T8][0] as u32).wrapping_sub(0x6ee0));
    c.g[T2][0] = 0x510000;
    c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
    c.g[T0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x6d20));
    c.swc1(0, (c.g[S5][0] as u32).wrapping_sub(0x6ed0));
    c.g[T2][0] = 0x510000;
    c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x234c));
    c.swc1(0, (c.g[S4][0] as u32).wrapping_sub(0x6ec0));
    c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x6eb0));
    c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0xc));
    c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x6ea0));
    c.g[T4][0] = 0x510000;
    c.g[T2][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x6cf0));
    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
    c.g[T4][0] = 0x510000;
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2348));
    c.swc1(0, (c.g[T4][0] as u32).wrapping_sub(0x6e90));
    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
    c.g[T4][0] = 0x510000;
    c.swc1(0, (c.g[T4][0] as u32).wrapping_sub(0x6e80));
    c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
    c.g[T4][0] = 0x510000;
    c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
    c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
    c.g[T5][0] = 0x0;
    c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
    c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x22c8));
    c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0x4));
    c.g[T6][0] = 0x0;
    c.g[T7][0] = c.g[S0][0];
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2344));
    c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x6eb0));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2340));
    c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x2270));
    c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
    c.g[T6][0] = sx((c.g[T6][0] as u32) >> 3);
    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
    c.g[T7][0] = 0x510000;
    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
    c.g[T5][0] = sx((c.g[T5][0] as u32) >> 3);
    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
    c.g[S1][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x6c80));
    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
    c.g[S2][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x6c90));
    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
    c.g[T4][0] = 0x510000;
    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
    c.g[T7][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x6c60));
    c.g[S3][0] = 0x510000;
    c.g[T4][0] = 0x510000;
    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6e40));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x233c));
    c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0xc));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
    c.swc1(1, (c.g[T4][0] as u32).wrapping_sub(0x6e10));
    c.g[T4][0] = 0x510000;
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x2338));
    c.g[S3][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x6c50));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x2334));
    c.g[T4][0] = 0x510000;
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x2330));
    c.swc1(0, (c.g[T4][0] as u32).wrapping_sub(0x6e70));
    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
    c.g[T4][0] = 0x510000;
    c.swc1(0, (c.g[T4][0] as u32).wrapping_sub(0x6e60));
    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x6e60));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x232c));
    c.swc1(6, (c.g[T4][0] as u32).wrapping_add(0xc));
    c.g[T4][0] = 0x510000;
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2328));
    c.swc1(0, (c.g[T4][0] as u32).wrapping_sub(0x6e50));
    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x6e50));
    c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x2324));
    c.swc1(6, (c.g[T4][0] as u32).wrapping_add(0xc));
    c.g[T4][0] = 0x510000;
    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x2320));
    c.swc1(0, (c.g[T4][0] as u32).wrapping_sub(0x6e40));
    c.g[T4][0] = 0x510000;
    c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x231c));
    c.swc1(0, (c.g[T4][0] as u32).wrapping_sub(0x6e30));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = 0x510000;
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x2318));
    c.g[T4][0] = c.g[V0][0];
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6e20));
    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x6e20));
    c.g[V0][0] = 0x510000;
    c.swc1(5, (c.g[T4][0] as u32).wrapping_add(0x8));
    c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
    c.swc1(7, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[T4][0] = 0x0;
    c.swc1(8, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x2400));
    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
    c.g[T4][0] = sx((c.g[T4][0] as u32) >> 3);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6e00));
    c.g[V1][0] = 0x510000;
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = 0x14000000;
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6df0));
    c.g[T4][0] = c.g[T4][0] | c.g[V0][0];
    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = 0x510000;
    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6e50));
    c.g[T3][0] = 0x510000;
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x2314));
    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6e60));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x2310));
    c.swc1(6, (c.g[T3][0] as u32).wrapping_add(0x4));
    c.g[T6][0] = c.g[T6][0] | c.g[V0][0];
    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
    c.g[T5][0] = c.g[T5][0] | c.g[V0][0];
    c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[T3][0] = 0x510000;
    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6e40));
    c.g[V1][0] = 0x510000;
    c.swc1(6, (c.g[T3][0] as u32).wrapping_add(0x4));
    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6e20));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[T3][0] = 0x510000;
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = 0x510000;
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = 0x510000;
    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = 0x510000;
    c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
    c.g[T4][0] = c.g[T4][0] << 32;
    c.swc1(1, (c.g[T1][0] as u32).wrapping_add(0xc));
    c.g[T6][0] = c.g[T6][0] << 32;
    c.swc1(4, (c.g[FP][0] as u32).wrapping_add(0x8));
    c.g[T5][0] = c.g[T5][0] << 32;
    c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6de0));
    c.swc1(7, (c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = 0x510000;
    c.swc1(5, (c.g[A1][0] as u32).wrapping_add(0x8));
    c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x230c));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x2308));
    c.swc1(0, (c.g[T3][0] as u32).wrapping_sub(0x6dd0));
    c.swc1(8, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.g[T3][0] = 0x510000;
    c.swc1(6, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2304));
    c.g[A2][0] = c.g[A0][0];
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6dc0));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6da0));
    c.swc1(3, (c.g[S7][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.g[T3][0];
    c.swc1(0, (c.g[S7][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6d90));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6db0));
    c.swc1(9, (c.g[T1][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = 0x510000;
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6da0));
    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = 0x510000;
    c.swc1(0, (c.g[T3][0] as u32).wrapping_sub(0x6d90));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
    c.g[T3][0] = 0x510000;
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6d80));
    c.swc1(10, (c.g[A3][0] as u32).wrapping_add(0x4));
    c.swc1(11, (c.g[A3][0] as u32).wrapping_add(0x8));
    c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x2300));
    c.g[A3][0] = 0x510000;
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6d70));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6d10));
    c.swc1(12, (c.g[S7][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = 0x510000;
    c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.g[A2][0] = 0x510000;
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = 0x510000;
    c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.g[V0][0];
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x22fc));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6d50));
    c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0xc));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x22f8));
    c.g[S6][0] = c.g[T3][0];
    c.swc1(4, (c.g[A2][0] as u32).wrapping_sub(0x6d30));
    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6d60));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x22f4));
    c.g[A2][0] = c.g[A0][0];
    c.swc1(3, (c.g[T0][0] as u32).wrapping_add(0xc));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6d40));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x22f0));
    c.swc1(2, (c.g[A3][0] as u32).wrapping_add(0xc));
    c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
    c.g[A3][0] = 0x510000;
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x22ec));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6d30));
    c.swc1(0, (c.g[T3][0] as u32).wrapping_sub(0x6d60));
    c.swc1(5, (c.g[S6][0] as u32).wrapping_add(0x4));
    c.g[T3][0] = 0x510000;
    c.sw((c.g[S6][0] as u32).wrapping_add(0xc), 0u64);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6d50));
    c.g[S6][0] = 0x510000;
    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = c.g[S6][0];
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6d40));
    c.g[V1][0] = 0x510000;
    c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6d10));
    c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
    c.g[A0][0] = c.g[V1][0];
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x22e8));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6d00));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x22e4));
    c.g[A2][0] = 0x510000;
    c.swc1(0, (c.g[T3][0] as u32).wrapping_sub(0x6d20));
    c.g[A3][0] = 0x510000;
    c.swc1(7, (c.g[T0][0] as u32).wrapping_add(0x8));
    c.g[T3][0] = 0x510000;
    c.swc1(6, (c.g[T0][0] as u32).wrapping_add(0x4));
    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6d50));
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x22e0));
    c.g[T0][0] = 0x510000;
    c.swc1(0, (c.g[S6][0] as u32).wrapping_sub(0x6d10));
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6d60));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.g[S6][0] = 0x510000;
    c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x22dc));
    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6d40));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6d00));
    c.g[V0][0] = 0x510000;
    c.swc1(9, (c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6d30));
    c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
    c.g[V1][0] = 0x510000;
    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x22d8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6d10));
    c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x6cf0));
    c.swc1(10, (c.g[T2][0] as u32).wrapping_add(0x4));
    c.g[A2][0] = 0x510000;
    c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6ce0));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6cd0));
    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
    c.g[T0][0] = 0x510000;
    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x4));
    c.g[T3][0] = 0x510000;
    c.sw((c.g[S6][0] as u32).wrapping_add(0x8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
    c.g[S6][0] = c.g[T3][0];
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6cb0));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = 0x510000;
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[A3][0];
    c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6ce0));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x22d4));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = 0x510000;
    c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0xc));
    c.g[T2][0] = c.g[T0][0];
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x22d0));
    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x6cc0));
    c.swc1(2, (c.g[S2][0] as u32).wrapping_add(0xc));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6ca0));
    c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0xc));
    c.g[A2][0] = 0x510000;
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
    c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6cd0));
    c.g[A0][0] = 0x510000;
    c.swc1(0, (c.g[T0][0] as u32).wrapping_sub(0x6cc0));
    c.g[A3][0] = 0x510000;
    c.sw((c.g[T2][0] as u32).wrapping_add(0xc), 0u64);
    c.g[T0][0] = c.g[A3][0];
    c.swc1(0, (c.g[T3][0] as u32).wrapping_sub(0x6cb0));
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6c70));
    c.sw((c.g[S6][0] as u32).wrapping_add(0xc), 0u64);
    c.g[T2][0] = 0x510000;
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6ca0));
    c.g[T3][0] = 0x510000;
    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6ce0));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x22cc));
    c.g[S6][0] = 0x510000;
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6c90));
    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6cd0));
    c.swc1(3, (c.g[S2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = 0x510000;
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x22c8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6cc0));
    c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x6c80));
    c.g[V1][0] = 0x510000;
    c.swc1(4, (c.g[S1][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6cb0));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x22c4));
    c.g[A0][0] = 0x510000;
    c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6c70));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6ca0));
    c.swc1(5, (c.g[T0][0] as u32).wrapping_add(0x8));
    c.g[A2][0] = 0x1;
    c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
    c.g[A3][0] = 0x2;
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x22c0));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x22bc));
    c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x6c60));
    c.swc1(6, (c.g[T7][0] as u32).wrapping_add(0x4));
    c.g[T2][0] = 0x510000;
    c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x6c40));
    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
    c.swc1(12, (c.g[S6][0] as u32).wrapping_add(0x4));
    c.g[T3][0] = 0x510000;
    c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x8));
    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6c30));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.g[S6][0] = 0x510000;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
    c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
    c.g[S2][0] = 0x510000;
    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
    c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
    c.g[T0][0] = 0x3;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x2988), 0u64);
    c.g[T7][0] = 0x510000;
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x22b8));
    c.sb((c.g[GP][0] as u32).wrapping_add(0x2990), c.g[A2][0]);
    c.sb((c.g[GP][0] as u32).wrapping_add(0x2998), c.g[A3][0]);
    c.sb((c.g[GP][0] as u32).wrapping_add(0x29a0), c.g[T0][0]);
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[A1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2988));
    c.g[V1][0] = c.g[A3][0] & 0xff;
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x22b4));
    c.g[A2][0] = c.g[S6][0];
    c.swc1(2, (c.g[T2][0] as u32).wrapping_add(0xc));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6c20));
    c.swc1(1, (c.g[T3][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = c.g[T0][0] & 0xff;
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x22b0));
    c.g[A3][0] = 0x510000;
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x22ac));
    c.g[T0][0] = c.g[A3][0];
    c.swc1(0, (c.g[T7][0] as u32).wrapping_sub(0x6c50));
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6c10));
    c.swc1(3, (c.g[S3][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] | 0x100;
    c.swc1(4, (c.g[S3][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.g[V1][0] | 0x200;
    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
    c.g[A0][0] = c.g[A0][0] | 0x300;
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x22a8));
    c.g[S3][0] = 0x510000;
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x22a4));
    c.g[T7][0] = 0x6;
    c.swc1(0, (c.g[S2][0] as u32).wrapping_sub(0x6c40));
    c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x4));
    c.g[S2][0] = 0x7;
    c.swc1(6, (c.g[T2][0] as u32).wrapping_add(0x8));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x22a0));
    c.g[T2][0] = 0x4;
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x229c));
    c.swc1(0, (c.g[S3][0] as u32).wrapping_sub(0x6c30));
    c.swc1(7, (c.g[T3][0] as u32).wrapping_add(0x4));
    c.g[S3][0] = 0x8;
    c.swc1(8, (c.g[T3][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[S6][0] as u32).wrapping_sub(0x6c20));
    c.g[T3][0] = 0x5;
    c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
    c.g[S6][0] = 0x9;
    c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6c10));
    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
    c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x2988), c.g[A1][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x2990), c.g[V0][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x2998), c.g[V1][0]);
    c.g[V0][0] = 0xa;
    c.sw((c.g[A2][0] as u32).wrapping_add(0x4), 0u64);
    c.g[V1][0] = 0xb;
    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.sw((c.g[T0][0] as u32).wrapping_add(0x8), 0u64);
    c.g[A2][0] = 0xd;
    c.sw((c.g[GP][0] as u32).wrapping_add(0x2984), 0u64);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x2980), 0u64);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x29a0), c.g[A0][0]);
    c.sb((c.g[GP][0] as u32).wrapping_add(0x29a8), c.g[T2][0]);
    c.g[A0][0] = 0xc;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x29b0), c.g[T3][0]);
    c.sb((c.g[GP][0] as u32).wrapping_add(0x29b8), c.g[T7][0]);
    c.g[T1][0] = c.g[T2][0] & 0xff;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x29d8), c.g[V0][0]);
    c.g[T2][0] = c.g[T3][0] & 0xff;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x29e0), c.g[V1][0]);
    c.g[T3][0] = c.g[T7][0] & 0xff;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x29f0), c.g[A2][0]);
    c.g[A1][0] = c.g[V0][0] & 0xff;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x29c8), c.g[S3][0]);
    c.g[T7][0] = 0x510000;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x29e8), c.g[A0][0]);
    c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x6c00));
    c.sb((c.g[GP][0] as u32).wrapping_add(0x29c0), c.g[S2][0]);
    c.g[A0][0] = c.g[V1][0] & 0xff;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x29d0), c.g[S6][0]);
    c.g[A3][0] = c.g[S3][0] & 0xff;
    c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x29f0));
    c.g[T0][0] = c.g[S2][0] & 0xff;
    c.g[V1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x29e8));
    c.g[A2][0] = c.g[S6][0] & 0xff;
    c.sd((c.g[T7][0] as u32).wrapping_add(0x8), c.g[T4][0]);
    c.g[V0][0] = c.g[V0][0] | 0xd00;
    c.g[S3][0] = 0x510000;
    c.sw((c.g[GP][0] as u32).wrapping_add(0x29f0), c.g[V0][0]);
    c.g[V1][0] = c.g[V1][0] | 0xc00;
    c.g[T1][0] = c.g[T1][0] | 0x400;
    c.g[T2][0] = c.g[T2][0] | 0x500;
    c.g[T3][0] = c.g[T3][0] | 0x600;
    c.g[T0][0] = c.g[T0][0] | 0x700;
    c.g[A3][0] = c.g[A3][0] | 0x800;
    c.g[A2][0] = c.g[A2][0] | 0x900;
    c.g[A1][0] = c.g[A1][0] | 0xa00;
    c.g[A0][0] = c.g[A0][0] | 0xb00;
    c.g[S2][0] = 0x510000;
    c.g[S6][0] = c.g[S3][0];
    c.sw((c.g[GP][0] as u32).wrapping_add(0x29e8), c.g[V1][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x29a8), c.g[T1][0]);
    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6bf0));
    c.sw((c.g[GP][0] as u32).wrapping_add(0x29b0), c.g[T2][0]);
    c.g[V0][0] = 0x510000;
    c.sw((c.g[GP][0] as u32).wrapping_add(0x29b8), c.g[T3][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6be0));
    c.sw((c.g[GP][0] as u32).wrapping_add(0x29c0), c.g[T0][0]);
    c.g[V1][0] = 0x510000;
    c.sw((c.g[GP][0] as u32).wrapping_add(0x29c8), c.g[A3][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x29d0), c.g[A2][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x29d8), c.g[A1][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x29e0), c.g[A0][0]);
    c.sd((c.g[S2][0] as u32).wrapping_sub(0x6c00), 0u64);
    c.sd((c.g[S3][0] as u32).wrapping_sub(0x6bf0), 0u64);
    c.sd((c.g[S6][0] as u32).wrapping_add(0x8), c.g[T6][0]);
    c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T5][0]);
    c.sd((c.g[V1][0] as u32).wrapping_sub(0x6be0), 0u64);
    world_static_init_destroy_exit(c);
}

/// Common exit of [`world_static_init_destroy`]: the epilogue's 11 cycles,
/// restores s0-s7 and fp, pops the frame and returns through `ra`.
pub(crate) fn world_static_init_destroy_exit(c: &mut Cpu) {
    c.cycles += 11;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(30, c.g[SP][0] as u32);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    ret(c);
}

/// `EAC_GetUnpackedSize(...)` (0x3b4818).
pub(crate) const EAC_GET_UNPACKED_SIZE: Callee = Callee {
    addr: 0x003b_4818,
    f: ee::s13::EAC_GetUnpackedSize,
};

/// `EAC_Unpack(...)` (0x3b47f8).
pub(crate) const EAC_UNPACK: Callee = Callee {
    addr: 0x003b_47f8,
    f: ee::s13::EAC_Unpack,
};

/// `tWScriptCacheEntry_dtor` (0x3aca70).
pub(crate) const WSCRIPT_ENTRY_DTOR: Callee = Callee {
    addr: 0x003a_ca70,
    f: ee::s13::tWScriptCache_dtor,
};

/// `cWScriptCacheEntry_update` (0x3acad0).
pub(crate) const WSCRIPT_POLL_LOAD: Callee = Callee {
    addr: 0x003a_cad0,
    f: ee::s13::tWScriptCache_PollLoad,
};

/// `cWScriptCacheEntry_load` (0x3acbb8).
pub(crate) const WSCRIPT_ENTRY_LOAD: Callee = Callee {
    addr: 0x003a_cbb8,
    f: ee::s13::tWScriptCache_Load,
};

/// `World_StaticInitDestroy(on, prio)` (0x3ad290).
pub(crate) const WORLD_STATIC_INIT_DESTROY: Callee = Callee {
    addr: 0x003a_d290,
    f: ee::s13::World_StaticInitDestroy,
};

/// `cWScriptCache::~cWScriptCache(this, flags)` (0x3ac8f0): destroy the
/// entries from the last to the first (`tWScriptCache_dtor`), free the array
/// (`operator delete[]`, the cookie 0x10 below it), and free `this` when
/// bit 0 of `flags` is set.
pub fn cwscript_cache_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(0x4));
        let no_entries = c.g[A1][0] == 0;
        c.g[V0][0] = 0xc;
        if !no_entries {
            'arr: {
                c.cycles += 5;
                c.g[V1][0] = c.lw(r32(c, A1).wrapping_sub(0x10));
                c.mult(0, 4, 3, 2);
                c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                if c.g[A1][0] == c.g[S0][0] {
                    c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
                    break 'arr;
                }
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0xc));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    WSCRIPT_ENTRY_DTOR.call(c, 0x003a_c93c)?;
                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0xc));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_c930);
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
            }
            c.cycles += 2;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x003a_c954)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 0x1;
        let no_free = c.g[V0][0] == 0;
        c.sw((c.g[GP][0] as u32).wrapping_add(0x1720), 0);
        if !no_free {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x003a_c968)?;
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWScriptCache_update(cWScriptCache* self)` (0x3ac980): poll every
/// entry (`cWScriptCacheEntry_update`), 0xc bytes apart.
pub fn cwscript_cache_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x8));
        let none = c.g[V0][0] == 0;
        c.g[S0][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S2)));
                WSCRIPT_POLL_LOAD.call(c, 0x003a_c9b8)?;
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x8));
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_c9a8);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWScriptCache_load(cWScriptCache* self, int index, int fileNum)`
/// (0x3ac9e0): load entry `index` (0xc bytes each) from file `fileNum`.
pub fn cwscript_cache_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc;
        frame.save_ra(c, 0);
        c.mult(0, 3, 5, 2);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0x4));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        WSCRIPT_ENTRY_LOAD.call(c, 0x003a_ca00)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWScriptCacheEntry_update(entry* e)` (0x3acad0): an entry in state
/// 1 polls its resource (`0x3df980`); once it is ready it probes the data
/// (`0x3df9d8`), unpacks a new buffer when the size is positive and stores
/// the entry's state word (`sh`) as 1 or 2.
pub fn twscript_cache_poll_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 1;
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x30);
        c.g[S0][0] = c.lh(r32(c, S1));
        // The generated exits that skip the store reload s0 only on the
        // not-ready paths; the unpack-size exit leaves s0 as it is.
        let mut reload_s0 = true;
        'body: {
            if c.g[S0][0] != c.g[V0][0] {
                break 'body;
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
            call_ptr(c, 0x003d_f980, 0x003a_cafc)?;
            if c.g[V0][0] != c.g[S0][0] {
                break 'body;
            }
            c.cycles += 9;
            c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
            c.g[A1][0] = c.g[SP][0];
            c.sw(sp(c), 0);
            c.g[A2][0] = c.g[SP][0] | 0x4;
            c.sw(sp(c).wrapping_add(0x4), 0);
            call_ptr(c, 0x003d_f9d8, 0x003a_cb1c)?;
            c.g[A0][0] = c.lw(sp(c));
            c.g[V0][0] = c.lw(sp(c).wrapping_add(0x4));
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                let nonpositive = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = 0x2;
                if nonpositive {
                    // Leaves through the epilogue with s0 unreloaded.
                    reload_s0 = false;
                    break 'body;
                }
                'b94: {
                    'b6c: {
                        c.cycles += 4;
                        EAC_GET_UNPACKED_SIZE.call(c, 0x003a_cb38)?;
                        let no_size = (c.g[V0][0] as i64) <= 0;
                        c.g[A1][0] = 0x490000;
                        if no_size {
                            break 'b6c;
                        }
                        c.cycles += 11;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x5080));
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7e30, 0x003a_cb54)?;
                        c.g[A0][0] = c.lw(sp(c));
                        c.g[A1][0] = c.g[V0][0];
                        c.sw(r32(c, S1).wrapping_add(0x8), c.g[V0][0]);
                        EAC_UNPACK.call(c, 0x003a_cb64)?;
                        break 'b94;
                    }
                    c.cycles += 10;
                    c.g[A0][0] = c.lw(sp(c).wrapping_add(0x4));
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x5080));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7e30, 0x003a_cb80)?;
                    c.g[A2][0] = c.lw(sp(c).wrapping_add(0x4));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.lw(sp(c));
                    c.sw(r32(c, S1).wrapping_add(0x8), c.g[V0][0]);
                    call_ptr(c, 0x003e_6574, 0x003a_cb94)?;
                }
                c.cycles += 2;
                c.g[A0][0] = c.lw(sp(c));
                call_ptr(c, 0x0025_23a8, 0x003a_cb9c)?;
            }
            // Store the state word (`'b_3acb9c` exit).
            c.cycles += 2;
            c.sh(r32(c, S1), c.g[V0][0]);
        }
        if reload_s0 {
            frame.restore(c, 16, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cWScriptCacheEntry_load(entry* e, int fileNum)` (0x3acbb8): the
/// entry's file load: format the name, allocate 0x100 bytes for the data and
/// set the entry's state (`sh`) to 1 when the file is non-empty, else 2.
pub fn twscript_cache_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x4a_0000;
        frame.save(c, 17, 0x30);
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.sh(r32(c, S1).wrapping_add(0x2), 0);
        c.g[A1][0] = sx(r32(c, V0).wrapping_add(0x47c0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_86d0, 0x003a_cbe8)?;
        c.g[A2][0] = 0x4a_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x4818));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0031_8630, 0x003a_cbfc)?;
        c.g[A0][0] = c.lw(sp(c));
        c.g[A1][0] = 0x100;
        call_ptr(c, 0x003d_f690, 0x003a_cc08)?;
        c.sw(r32(c, S1).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x003a_cc18)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x003a_cc24)?;
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
        let non_empty = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = 1;
        if !non_empty {
            c.cycles += 1;
            c.g[V0][0] = 2;
        }
        c.cycles += 6;
        c.sh(r32(c, S1), c.g[V0][0]);
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void World_ReturnSelf(...)` (0x3ad250, `func_003AD250`): spills its
/// arguments to its frame and returns its first argument.
pub fn world_return_self(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x80);
    c.g[V0][0] = c.g[A0][0];
    let base = sp(c);
    c.sd(base.wrapping_add(0x48), c.g[A1][0]);
    c.sd(base.wrapping_add(0x50), c.g[A2][0]);
    c.sd(base.wrapping_add(0x58), c.g[A3][0]);
    c.sd(base.wrapping_add(0x60), c.g[T0][0]);
    c.sd(base.wrapping_add(0x68), c.g[T1][0]);
    c.sd(base.wrapping_add(0x70), c.g[T2][0]);
    c.sd(base.wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, base.wrapping_add(0x38));
    c.swc1(14, base.wrapping_add(0x3c));
    c.swc1(16, base.wrapping_add(0x40));
    c.swc1(18, base.wrapping_add(0x44));
    frame.close(c);
}

/// `void World_GlobalCtors(void)` (0x3adda0): runs the static init of the
/// world (`World_StaticInitDestroy(1, 0xffff)`).
pub fn world_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        WORLD_STATIC_INIT_DESTROY.call(c, 0x003a_ddb4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SHP image headers.
//
// An SHP image starts with a 16-byte header: the format byte at +0 (low
// seven bits the pixel format, bit 7 set when the pixels are RefPack
// compressed), the offset of the next block in the chain in the upper 24
// bits of the word at +0, the width (+4) and height (+6) as halves, and a
// flags word at +0xc (bits 28-31 the last mip level, bit 13 set when the
// pixels are swizzled, bit 12 when the pixel data lives at the offset
// stored at +0x10 instead of right after the header). Other blocks of a
// chain ("o", "i", "p", "|") have their own formats.

/// Header fields.
pub(crate) const SHP_WIDTH: u32 = 0x4;

pub(crate) const SHP_HEIGHT: u32 = 0x6;

pub(crate) const SHP_FLAGS: u32 = 0xc;

/// The pixel data (or, with [`SHP_FLAG_DATA_AT_OFFSET`], its offset).
pub(crate) const SHP_DATA: u32 = 0x10;

pub(crate) const SHP_HEADER_SIZE: u64 = 0x10;

/// Flags bit: the pixel data is at the offset stored at [`SHP_DATA`].
pub(crate) const SHP_FLAG_DATA_AT_OFFSET: u64 = 0x1000;

/// Format byte of an extra "o" block (unknown contents) and of the "i"
/// block that ends with the tag `"USE"`.
pub(crate) const SHP_BLOCK_O: u64 = 0x6f;

pub(crate) const SHP_BLOCK_I: u64 = 0x69;

pub(crate) const SHP_BLOCK_I_TAG: u64 = 0x0045_5355;

/// `int SHP_NormalizeBpp(int bpp)` (0x3b2528): the bit depth a requested
/// depth is stored at (0 means 32): the depth of the format
/// `SHP_FormatFromBpp` picks for it, read back through a one-byte header on
/// the stack (16 becomes 15, unknown depths 0).
pub fn shp_normalize_bpp(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x20;
        frame.save_ra(c, 0x10);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = c.g[V0][0];
        }
        callee::SHP_FORMAT_FROM_BPP.call(c, 0x003b_253c)?;
        // The format byte of a header at sp.
        let header = frame.at(c, 0);
        Addr(header).sb(c, c.g[V0][0]);
        c.g[A0][0] = c.g[SP][0];
        callee::SHP_GET_BPP.call(c, 0x003b_2548)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `SHP_CalcImageSize`'s entry, its mip loop's two heads (the next level,
/// and the level test reached after a 1-pixel-wide level) and its end.
pub(crate) const CALC_IMAGE_SIZE_ENTRY: u32 = 0x003b_2558;

pub(crate) const CALC_IMAGE_SIZE_NEXT_LEVEL: u32 = 0x003b_2600;

pub(crate) const CALC_IMAGE_SIZE_LEVEL: u32 = 0x003b_2604;

pub(crate) const CALC_IMAGE_SIZE_END: u32 = 0x003b_2684;

/// Largest pixel count (`w * h`) an image may have.
pub(crate) const SHP_MAX_PIXELS: u64 = 1_000_000_000;

/// `int SHP_CalcImageSize(int w, int h, int bpp, int maxLevel)` (0x3b2558):
/// the bytes of the pixel data of a `w` x `h` image at `bpp` (normalised; 15
/// counts as 16 bits) with mip levels 0..=`maxLevel`, each level's rows
/// rounded up to whole bytes and each level after the first starting on a 16
/// byte boundary; the chain stops early at a 1x1 level. 0 for a width or
/// height outside 1..=0x10000, more than 10^9 pixels or a depth with no
/// format. Resumable at the mip loop's two heads (registers hold the state:
/// s0 total, s1 h, s2 w, s3 bits per pixel, s4 maxLevel, t0 level).
pub fn shp_calc_image_size(c: &mut Cpu) {
    guest(c, |c| {
        let resume = match c.pc {
            CALC_IMAGE_SIZE_ENTRY => None,
            CALC_IMAGE_SIZE_NEXT_LEVEL | CALC_IMAGE_SIZE_LEVEL => Some(c.pc),
            pc => {
                c.jump_out(pc, CALC_IMAGE_SIZE_ENTRY, CALC_IMAGE_SIZE_END);
                return Ok(());
            }
        };
        let frame = Frame { size: 0x60 };
        'exit: {
            let mut head = match resume {
                Some(head) => head,
                None => {
                    c.cycles += 21;
                    let frame = Frame::open(c, 0x60);
                    frame.save(c, S2, 0x30);
                    frame.save(c, S1, 0x40);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S4, 0x10);
                    c.g[S1][0] = c.g[A1][0];
                    frame.save(c, S0, 0x50);
                    c.g[A0][0] = c.g[A2][0];
                    frame.save(c, S3, 0x20);
                    c.g[S4][0] = c.g[A3][0];
                    frame.save_ra(c, 0);
                    c.g[S3][0] = 0x10;
                    callee::SHP_NORMALIZE_BPP.call(c, 0x003b_258c)?;
                    c.g[S0][0] = 0;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] = 0xffff;
                    c.g[V0][0] = sx(r32(c, S2).wrapping_sub(1));
                    c.g[A0][0] = c.g[A1][0] ^ 0xf;
                    // Width outside 1..=0x10000 (unsigned w - 1 > 0xffff).
                    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    let bad_width = c.g[V1][0] != 0;
                    // 15 bits per pixel are stored in 16.
                    if c.g[A0][0] != 0 {
                        c.g[S3][0] = c.g[A1][0];
                    }
                    let known = 'check: {
                        if bad_width {
                            break 'check false;
                        }
                        c.cycles += 2;
                        let no_height = (c.g[S1][0] as i64) <= 0;
                        c.g[V0][0] = 0x10000;
                        if no_height {
                            break 'check false;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                        let too_tall = c.g[V0][0] != 0;
                        c.g[V0][0] = 0;
                        if too_tall {
                            break 'exit;
                        }
                        c.cycles += 6;
                        c.mult(0, V1, S2, S1);
                        c.g[V0][0] = SHP_MAX_PIXELS;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        let too_big = c.g[V0][0] != 0;
                        c.g[V0][0] = 0;
                        if too_big {
                            break 'exit;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[A1][0];
                        callee::SHP_FORMAT_FROM_BPP.call(c, 0x003b_25e0)?;
                        let known = c.g[V0][0] != 0;
                        c.g[T0][0] = 0;
                        known
                    };
                    if !known {
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        break 'exit;
                    }
                    c.cycles += 4;
                    c.g[T1][0] = 0;
                    c.g[T3][0] = !0xf;
                    c.g[T2][0] = u64::MAX;
                    CALC_IMAGE_SIZE_LEVEL
                }
            };
            // One mip level per pass: t0 the level, a1/a2 its width and
            // height (at least 1), s0 the running total.
            loop {
                if head == CALC_IMAGE_SIZE_NEXT_LEVEL {
                    c.cycles += 1;
                    c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                }
                c.cycles += 3;
                let level = r32(c, T0) & 31;
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[T0][0] as i64)) as u64;
                let past_last = c.g[V0][0] != 0;
                c.g[A1][0] = sx(((r32(c, S2) as i32) >> level) as u32);
                if past_last {
                    break;
                }
                c.cycles += 7;
                c.g[A2][0] = sx(((r32(c, S1) as i32) >> level) as u32);
                c.g[A3][0] = 1;
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[V1][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = c.g[A3][0];
                }
                let first = c.g[S0][0] == 0;
                if c.g[V1][0] == 0 {
                    c.g[A2][0] = c.g[A3][0];
                }
                if !first {
                    // Later levels start 16-byte aligned.
                    c.cycles += 2;
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(0xf));
                    c.g[S0][0] = c.g[V0][0] & c.g[T3][0];
                }
                // Row bytes (bits rounded up to bytes, as a signed divide by
                // 8), times the rows.
                c.cycles += 9;
                c.mult(0, V0, A1, S3);
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(7));
                c.g[V1][0] = ((c.g[T2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xe));
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.g[V0][0] = sx(((r32(c, V0) as i32) >> 3) as u32);
                c.mult(0, V1, V0, A2);
                let wide = c.g[A1][0] != c.g[A3][0];
                c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, S0)));
                if wide {
                    if back_edge_unwinds(c, CALC_IMAGE_SIZE_NEXT_LEVEL) {
                        return Ok(());
                    }
                    head = CALC_IMAGE_SIZE_NEXT_LEVEL;
                    continue;
                }
                c.cycles += 2;
                if c.g[A2][0] != c.g[A1][0] {
                    c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                    if back_edge_unwinds(c, CALC_IMAGE_SIZE_LEVEL) {
                        return Ok(());
                    }
                    head = CALC_IMAGE_SIZE_LEVEL;
                    continue;
                }
                // A 1x1 level ends the chain.
                break;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SHP_InitHeader(SHPHeader* img, int w, int h, int bpp, int palBpp,
/// int maxLevel, int extra1, int extra2)` (0x3b27c8): write an image chain
/// in place: the image header (cleared; the format for `bpp`, 0 meaning 32;
/// the size; the last mip level in the flags' top nibble) followed by
/// room for its pixels; for an indexed image (`bpp` < 9) with a palette
/// depth, a palette header at the next 16-byte boundary (one row of
/// `SHP_PaletteEntriesForBpp` entries) whose entries are a grey ramp with
/// alpha 0xff (`palBpp` >= 32) or all 0xff bytes; then an optional "o"
/// block of `extra1` bytes (+8 header) and an optional "i" block of
/// `extra2` bytes (+16 header, tagged "USE"). Each block's next-offset
/// links to the one after it.
pub fn shp_init_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S7, 0x30);
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S6, 0x40);
        frame.save(c, FP, 0x20);
        frame.save(c, S1, 0x90);
        frame.save(c, S5, 0x50);
        frame.save_ra(c, 0x10);
        // s0/s5 the image (s5: the block last linked), s7 bpp, s2 w, s3 h,
        // s4 maxLevel, s6 palette depth, fp extra1; extra2 at 0(sp).
        c.g[S7][0] = 0x20;
        c.g[S0][0] = c.g[A0][0];
        if c.g[A3][0] != 0 {
            c.g[S7][0] = c.g[A3][0];
        }
        c.g[A0][0] = c.g[S7][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A2][0];
        c.g[S4][0] = c.g[T1][0];
        c.g[S6][0] = c.g[T0][0];
        c.g[FP][0] = c.g[T2][0];
        c.g[S5][0] = c.g[S0][0];
        let extra2_slot = frame.at(c, 0);
        Addr(extra2_slot).sw(c, c.g[T3][0]);
        callee::SHP_FORMAT_FROM_BPP.call(c, 0x003b_2824)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = SHP_HEADER_SIZE;
        callee::EA_MEM_SET.call(c, 0x003b_2838)?;
        let image = Addr(r32(c, S0));
        c.g[V1][0] = image.off(SHP_FLAGS).lw(c);
        c.g[V0][0] = 0x0fff_ffff;
        c.g[A0][0] = sx(r32(c, S4) << 28);
        c.g[V1][0] &= c.g[V0][0];
        image.sb(c, c.g[S1][0]);
        c.g[V1][0] |= c.g[A0][0];
        image.off(SHP_WIDTH).sh(c, c.g[S2][0]);
        image.off(SHP_HEIGHT).sh(c, c.g[S3][0]);
        c.g[A0][0] = c.g[S2][0];
        image.off(SHP_FLAGS).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S7][0];
        callee::SHP_CALC_IMAGE_SIZE.call(c, 0x003b_2874)?;
        // s1: the end of the chain so far.
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(SHP_HEADER_SIZE as u32));
        c.g[V1][0] = ((c.g[S7][0] as i64) < 9) as u64;
        let indexed = c.g[V1][0] != 0;
        c.g[S1][0] = sx(r32(c, S5).wrapping_add(r32(c, V0)));
        if indexed {
            c.cycles += 2;
            let has_palette = c.g[S6][0] != 0;
            c.g[A0][0] = c.g[S7][0];
            if has_palette {
                shp_init_header_palette(c)?;
            }
        }
        c.cycles += 2;
        let has_extra1 = c.g[FP][0] != 0;
        c.g[V0][0] = sx(r32(c, S1).wrapping_sub(r32(c, S5)));
        if has_extra1 {
            c.cycles += 14;
            shp_link_block(c);
            c.g[S0][0] = sx(r32(c, FP).wrapping_add(8));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[S0][0];
            c.g[S5][0] = c.g[S1][0];
            callee::EA_MEM_SET.call(c, 0x003b_29d8)?;
            c.g[V0][0] = SHP_BLOCK_O;
            let block = Addr(r32(c, S1));
            block.off(4).sw(c, c.g[FP][0]);
            block.sb(c, c.g[V0][0]);
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
        }
        c.cycles += 3;
        let extra2_slot = Addr(frame.at(c, 0));
        c.g[V0][0] = extra2_slot.lw(c);
        let has_extra2 = c.g[V0][0] != 0;
        c.g[A0][0] = extra2_slot.lw(c);
        if has_extra2 {
            c.cycles += 16;
            c.g[V0][0] = sx(r32(c, S1).wrapping_sub(r32(c, S5)));
            c.g[A2][0] = sx(r32(c, A0).wrapping_add(SHP_HEADER_SIZE as u32));
            c.g[A1][0] = 0;
            shp_link_block(c);
            c.g[A0][0] = c.g[S1][0];
            callee::EA_MEM_SET.call(c, 0x003b_2a18)?;
            c.g[V0][0] = SHP_BLOCK_I_TAG;
            c.g[V1][0] = SHP_BLOCK_I;
            c.g[A0][0] = SHP_HEADER_SIZE;
            let block = Addr(r32(c, S1));
            block.off(SHP_DATA).sw(c, c.g[V0][0]);
            block.sb(c, c.g[V1][0]);
            block.off(SHP_HEIGHT).sh(c, c.g[A0][0]);
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

/// Link the block at s5 to the one starting `v0` bytes later: its format
/// byte with the offset in the upper 24 bits (`v1`, `v0` left as the
/// original leaves them).
pub(crate) fn shp_link_block(c: &mut Cpu) {
    let block = Addr(r32(c, S5));
    c.g[V1][0] = block.lbu(c);
    c.g[V0][0] = sx(r32(c, V0) << 8);
    c.g[V1][0] |= c.g[V0][0];
    block.sw(c, c.g[V1][0]);
}

/// `SHP_InitHeader`'s palette block (s1 the chain's end, s5 the image, s6
/// the palette depth, a0 the image depth): the palette header at the next
/// 16-byte boundary, `entries` x 1 in the palette format, its entries
/// filled, and s1 past them (s5 the palette).
pub(crate) fn shp_init_header_palette(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 27;
    // s3: bits per palette entry (15 is stored in 16).
    c.g[S3][0] = 0x10;
    callee::SHP_PALETTE_ENTRIES_FOR_BPP.call(c, 0x003b_2894)?;
    c.g[A0][0] = c.g[S6][0];
    c.g[S2][0] = c.g[V0][0];
    callee::SHP_PALETTE_FORMAT_FOR_BPP.call(c, 0x003b_28a0)?;
    c.g[V1][0] = sx(r32(c, S1).wrapping_sub(r32(c, S5)));
    c.g[A1][0] = !0xf;
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0xf));
    let image = Addr(r32(c, S5));
    c.g[A0][0] = image.lbu(c);
    c.g[V1][0] &= c.g[A1][0];
    c.g[A2][0] = c.g[S6][0] ^ 0xf;
    c.g[A1][0] = sx(r32(c, V1) << 8);
    c.g[S1][0] = sx(r32(c, S5).wrapping_add(r32(c, V1)));
    c.g[A0][0] |= c.g[A1][0];
    if c.g[A2][0] != 0 {
        c.g[S3][0] = c.g[S6][0];
    }
    image.sw(c, c.g[A0][0]);
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = SHP_HEADER_SIZE;
    callee::EA_MEM_SET.call(c, 0x003b_28e0)?;
    c.g[V0][0] = 1;
    let palette = Addr(r32(c, S1));
    palette.sb(c, c.g[S0][0]);
    palette.off(SHP_HEIGHT).sh(c, c.g[V0][0]);
    c.g[V1][0] = ((c.g[S6][0] as i64) < 32) as u64;
    let masked = c.g[V1][0] != 0;
    palette.off(SHP_WIDTH).sh(c, c.g[S2][0]);
    if masked {
        // All bytes 0xff: entries * bits / 8 (signed, rounded toward 0).
        c.cycles += 4;
        c.g[V0][0] = palette.off(SHP_FLAGS).lw(c) & SHP_FLAG_DATA_AT_OFFSET;
        let at_offset = c.g[V0][0] != 0;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(SHP_DATA));
        if at_offset {
            c.cycles += 2;
            c.g[V0][0] = palette.off(SHP_DATA).lw(c);
            c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
        }
        c.cycles += 9;
        c.mult(0, V1, S2, S3);
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = u64::MAX;
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.g[A2][0] = sx(r32(c, V1).wrapping_add(7));
        if c.g[V0][0] != 0 {
            c.g[A2][0] = c.g[V1][0];
        }
        c.g[A2][0] = sx(((r32(c, A2) as i32) >> 3) as u32);
        callee::EA_MEM_SET.call(c, 0x003b_2984)?;
        c.cycles += 1;
        c.mult(0, A0, S2, S3);
    } else {
        // A grey ramp: entry i is (i, i, i, 0xff).
        c.cycles += 4;
        c.g[V0][0] = palette.off(SHP_FLAGS).lw(c) & SHP_FLAG_DATA_AT_OFFSET;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = sx(r32(c, S1).wrapping_add(SHP_DATA));
        } else {
            c.cycles += 2;
            c.g[V0][0] = palette.off(SHP_DATA).lw(c);
            c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
        }
        c.cycles += 2;
        let empty = (c.g[S2][0] as i64) <= 0;
        c.g[A0][0] = 0;
        if empty {
            c.cycles += 1;
        } else {
            c.cycles += 2;
            c.g[V1][0] = c.g[V0][0];
            c.g[A1][0] = 0xff;
            loop {
                c.cycles += 8;
                let entry = Addr(r32(c, V1));
                entry.off(2).sb(c, c.g[A0][0]);
                entry.off(1).sb(c, c.g[A0][0]);
                entry.sb(c, c.g[A0][0]);
                entry.off(3).sb(c, c.g[A1][0]);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, SHP_INIT_HEADER_RAMP);
                }
            }
            c.cycles += 2;
        }
        c.mult(0, A0, S2, S3);
    }
    // s1 past the entries (bytes rounded toward 0, as a signed divide).
    c.cycles += 8;
    c.g[V1][0] = u64::MAX;
    c.g[S5][0] = c.g[S1][0];
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(7));
    if c.g[V1][0] != 0 {
        c.g[V0][0] = c.g[A0][0];
    }
    c.g[V0][0] = sx(((r32(c, V0) as i32) >> 3) as u32);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(SHP_HEADER_SIZE as u32));
    c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
    Ok(())
}

/// The grey-ramp loop's head in `SHP_InitHeader`.
pub(crate) const SHP_INIT_HEADER_RAMP: u32 = 0x003b_2920;

/// The allocator the SHP code calls through (`void* (*)(const char* name,
/// int size, int, int align, int heap)`; `MEM_AllocNamedAlignedHook` in the
/// image) and the matching free hook after it.
pub(crate) const SHP_ALLOC_HOOK: u32 = 0x0044_c454;

/// The allocator `SHP_Unpack` calls through (`void* (*)(const char* name,
/// int size, int)`).
pub(crate) const SHP_UNPACK_ALLOC_HOOK: u32 = 0x0044_c450;

/// `"SHP%dx%dx%d"`: the allocation name of a new image.
pub(crate) const SHP_NAME_FORMAT: u64 = 0x0049_56b0;

/// `SHPHeader* SHP_Create(int w, int h, int bpp, int palBpp, int maxLevel,
/// int heap, int extra1, int extra2)` (0x3b2a68): allocate an image chain
/// (`SHP_CalcAllocSize` bytes, 16-byte aligned, named `"SHP<w>x<h>x<bpp>"`,
/// through the allocator hook with `heap`) and lay it out with
/// `SHP_InitHeader`. Null when the size is 0 or the allocation fails.
pub fn shp_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S2, 0x90);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        // s2 w, s3 h, s4 bpp, s6 palBpp, s7 maxLevel, s5 heap, fp extra1;
        // extra2 at 0x10(sp); the name is built at 0(sp).
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A1][0];
        c.g[S4][0] = c.g[A2][0];
        c.g[S5][0] = c.g[T1][0];
        c.g[S6][0] = c.g[A3][0];
        c.g[S7][0] = c.g[T0][0];
        let extra2_slot = Addr(frame.at(c, 0x10));
        extra2_slot.sw(c, c.g[T3][0]);
        c.g[FP][0] = c.g[T2][0];
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save_ra(c, 0x20);
        c.g[T1][0] = c.g[FP][0];
        c.g[T2][0] = c.g[T3][0];
        callee::SHP_CALC_ALLOC_SIZE.call(c, 0x003b_2ac0)?;
        c.g[S1][0] = c.g[V0][0];
        let empty = c.g[S1][0] == 0;
        c.g[S0][0] = 0;
        if !empty {
            c.cycles += 18;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = SHP_NAME_FORMAT;
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S3][0];
            c.g[T0][0] = c.g[S4][0];
            callee::SPRINTF.call(c, 0x003b_2ae8)?;
            c.g[V1][0] = 0x45_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[V0][0] = Addr(SHP_ALLOC_HOOK).lw(c);
            c.g[T0][0] = c.g[S5][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = 0;
            c.g[A3][0] = 0x10;
            call_ptr(c, r32(c, V0), 0x003b_2b08)?;
            c.g[S0][0] = c.g[V0][0];
            let failed = c.g[S0][0] == 0;
            let extra2_slot = Addr(frame.at(c, 0x10));
            c.g[T3][0] = extra2_slot.lw(c);
            if !failed {
                c.cycles += 8;
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[T0][0] = c.g[S6][0];
                c.g[T1][0] = c.g[S7][0];
                c.g[T2][0] = c.g[FP][0];
                c.g[A0][0] = c.g[S0][0];
                callee::SHP_INIT_HEADER.call(c, 0x003b_2b34)?;
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `SHP_DecodeDXTAlphaBlock`'s entry and end, and the loop heads where it
/// checks events.
pub(crate) const DXT_ALPHA_ENTRY: u32 = 0x003b_2b90;

pub(crate) const DXT_ALPHA_END: u32 = 0x003b_2e74;

pub(crate) const DXT_ALPHA_EXPLICIT_ROW: u32 = 0x003b_2bc0;

pub(crate) const DXT_ALPHA_EXPLICIT_BYTE: u32 = 0x003b_2bc8;

pub(crate) const DXT_ALPHA_HALF: u32 = 0x003b_2db0;

pub(crate) const DXT_ALPHA_ROW: u32 = 0x003b_2dc0;

pub(crate) const DXT_ALPHA_PIXEL: u32 = 0x003b_2dc8;

/// The original's jump table on the 3-bit alpha index (one store per
/// palette register).
pub(crate) const DXT_ALPHA_INDEX_TABLE: u32 = 0x0049_56c0;

/// `SHP_DecodeDXT`'s block mode with explicit 4-bit alpha (DXT3).
pub(crate) const DXT_MODE_EXPLICIT_ALPHA: u64 = 3;

/// The jump-table function computed a target outside its own cases.
pub(crate) struct DxtJumpedOut;

/// `void SHP_DecodeDXTAlphaBlock(u64* dst, int pitch, const u8* src, int
/// mode)` (0x3b2b90): put a DXT block's alpha into the top byte of each of
/// its 4x4 destination pixels. Mode 3 (DXT3): 4-bit alphas, two per source
/// byte, widened to 8 bits, into 64-bit pixels (8 bytes apart; the word is
/// rewritten with the new byte sign-extended into its upper half). Other
/// modes (DXT5): two end alphas and 3-bit indices into the 8-entry palette
/// between them (alpha0 > alpha1: six interpolated values in sevenths;
/// otherwise four in fifths, then 0 and 0xff), one byte per 32-bit pixel.
/// `pitch` is the destination row stride. Resume entries go to the
/// generated twin.
pub fn shp_decode_dxt_alpha_block(c: &mut Cpu) {
    if c.pc != DXT_ALPHA_ENTRY {
        return ee::s13::f_003b2b90__entries(c);
    }
    c.cycles += 8;
    let frame = Frame::open(c, 0x20);
    // t0 the destination, t1 the source, s0 the pitch.
    c.g[T0][0] = c.g[A0][0];
    frame.save(c, S0, 0x10);
    c.g[T1][0] = c.g[A2][0];
    frame.save(c, S1, 0);
    c.g[V0][0] = DXT_MODE_EXPLICIT_ALPHA;
    let explicit = c.g[A3][0] == c.g[V0][0];
    c.g[S0][0] = c.g[A1][0];
    if explicit {
        dxt_explicit_alpha(c, frame);
    } else if dxt_interpolated_alpha(c, frame).is_err() {
        return;
    }
    c.cycles += 3;
    frame.restore(c, S1, 0);
    frame.close(c);
}

/// DXT3 alpha: four rows of two source bytes (t3 the RGB mask), each byte
/// two pixels.
pub(crate) fn dxt_explicit_alpha(c: &mut Cpu, frame: Frame) {
    c.cycles += 4;
    c.g[A0][0] = 0;
    c.g[T3][0] = 0x00ff_ffff;
    loop {
        c.cycles += 2;
        c.g[T2][0] = sx(r32(c, A0).wrapping_add(1));
        c.g[A3][0] = 0;
        loop {
            c.cycles += 23;
            c.g[V1][0] = Addr(r32(c, T1)).lbu(c);
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
            c.g[A0][0] = Addr(r32(c, T0)).ld(c);
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
            // Low nibble: the first pixel's alpha (n | n << 4).
            c.g[V0][0] = c.g[V1][0] & 0xf;
            c.g[A1][0] = sx(r32(c, V0) << 4);
            c.g[A0][0] &= c.g[T3][0];
            c.g[V0][0] |= c.g[A1][0];
            c.g[V1][0] &= 0xf0;
            c.g[V0][0] = sx(r32(c, V0) << 24);
            c.g[A2][0] = sx(r32(c, V1) >> 4);
            c.g[A0][0] |= c.g[V0][0];
            c.g[V1][0] |= c.g[A2][0];
            Addr(r32(c, T0)).sd(c, c.g[A0][0]);
            // High nibble: the second pixel's.
            c.g[V1][0] = sx(r32(c, V1) << 24);
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(8));
            c.g[A1][0] = (c.g[A3][0] < 2) as u64;
            c.g[V0][0] = Addr(r32(c, T0)).ld(c);
            c.g[V0][0] &= c.g[T3][0];
            c.g[V0][0] |= c.g[V1][0];
            Addr(r32(c, T0)).sd(c, c.g[V0][0]);
            let more = c.g[A1][0] != 0;
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(8));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, DXT_ALPHA_EXPLICIT_BYTE);
            }
        }
        // Next row: back over the row's 32 bytes, then one pitch on.
        c.cycles += 5;
        c.g[V1][0] = sx(r32(c, T0).wrapping_add(r32(c, S0)));
        c.g[A0][0] = c.g[T2][0];
        c.g[V0][0] = (c.g[A0][0] < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.g[T0][0] = sx(r32(c, V1).wrapping_sub(0x20));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, DXT_ALPHA_EXPLICIT_ROW);
        }
    }
    c.cycles += 2;
    frame.restore(c, S0, 0x10);
}

/// DXT5 alpha: the palette into registers (t6, t5, a3, a1, t1, t8, t9, t2
/// for indices 0..7), then two halves of two rows of four 3-bit indices
/// (a2, read 48 bits at a time from t7), each stored through the jump
/// table to the alpha byte at t4.
pub(crate) fn dxt_interpolated_alpha(c: &mut Cpu, frame: Frame) -> Result<(), DxtJumpedOut> {
    c.cycles += 8;
    c.g[T7][0] = c.g[T1][0];
    c.g[T4][0] = sx(r32(c, T0).wrapping_add(3));
    c.g[T6][0] = Addr(r32(c, T7)).lbu(c);
    c.g[T7][0] = sx(r32(c, T7).wrapping_add(1));
    c.g[T5][0] = Addr(r32(c, T7)).lbu(c);
    c.g[V0][0] = (c.g[T5][0] < c.g[T6][0]) as u64;
    let sevenths = c.g[V0][0] != 0;
    c.g[T7][0] = sx(r32(c, T7).wrapping_add(1));
    if sevenths {
        dxt_alpha_palette_sevenths(c);
    } else {
        dxt_alpha_palette_fifths(c);
    }
    c.cycles += 3;
    c.g[S1][0] = sx(r32(c, S0).wrapping_sub(0x10));
    c.g[V1][0] = 0;
    c.g[A2][0] = Addr(r32(c, T7)).ld(c);
    loop {
        c.cycles += 4;
        c.g[S0][0] = sx(r32(c, V1).wrapping_add(1));
        c.g[T7][0] = sx(r32(c, T7).wrapping_add(3));
        c.g[V1][0] = 0;
        loop {
            c.cycles += 2;
            c.g[T3][0] = sx(r32(c, V1).wrapping_add(1));
            c.g[T0][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.g[A2][0] & 7;
                c.g[A0][0] = c.g[V0][0] & 0xff;
                c.g[V0][0] = (c.g[A0][0] < 8) as u64;
                let in_table = c.g[V0][0] != 0;
                c.g[A2][0] >>= 3;
                if in_table {
                    c.cycles += 7;
                    c.g[V0][0] = DXT_ALPHA_INDEX_TABLE as u64;
                    c.g[A0][0] = sx(r32(c, A0) << 2);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = Addr(r32(c, A0)).lw(c);
                    let (alpha, cost) = match r32(c, V1) {
                        0x003b_2df8 => (T6, 2),
                        0x003b_2e00 => (T5, 2),
                        0x003b_2e08 => (A3, 2),
                        0x003b_2e10 => (A1, 2),
                        0x003b_2e18 => (T1, 2),
                        0x003b_2e20 => (T8, 2),
                        0x003b_2e28 => (T9, 2),
                        0x003b_2e30 => (T2, 1),
                        target => {
                            c.jump_out(target, DXT_ALPHA_ENTRY, DXT_ALPHA_END);
                            return Err(DxtJumpedOut);
                        }
                    };
                    c.cycles += cost;
                    Addr(r32(c, T4)).sb(c, c.g[alpha][0]);
                }
                c.cycles += 4;
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                c.g[V0][0] = ((c.g[T0][0] as i64) < 4) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T4][0] = sx(r32(c, T4).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, DXT_ALPHA_PIXEL);
                }
            }
            // Next row (s1: the pitch less the row's 16 bytes).
            c.cycles += 4;
            c.g[V1][0] = c.g[T3][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[T4][0] = sx(r32(c, T4).wrapping_add(r32(c, S1)));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, DXT_ALPHA_ROW);
            }
        }
        c.cycles += 4;
        c.g[V1][0] = c.g[S0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[A2][0] = Addr(r32(c, T7)).ld(c);
        if event_due(c) {
            deliver_event_here(c, DXT_ALPHA_HALF);
        }
    }
    c.cycles += 1;
    frame.restore(c, S0, 0x10);
    Ok(())
}

/// alpha0 > alpha1: indices 2..7 are (6a0 + a1 + 3) / 7 .. (a0 + 6a1 + 3) / 7
/// (a3, a1, t1, t8, t9, t2), from t6 = alpha0 and t5 = alpha1.
pub(crate) fn dxt_alpha_palette_sevenths(c: &mut Cpu) {
    c.cycles += 29;
    c.g[A3][0] = 6;
    c.g[T1][0] = sx(r32(c, T6) << 2);
    c.mult(0, V0, T5, A3);
    c.g[T3][0] = sx(r32(c, T5) << 1);
    c.g[A1][0] = sx(r32(c, T1).wrapping_add(r32(c, T6)));
    c.g[T0][0] = sx(r32(c, T6) << 1);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, T3)));
    c.g[A0][0] = sx(r32(c, T5) << 2);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(3));
    c.g[A2][0] = sx(r32(c, T0).wrapping_add(r32(c, T6)));
    c.g[T2][0] = sx(r32(c, V0).wrapping_add(r32(c, T6)));
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    c.mult(0, V0, T6, A3);
    c.g[T2][0] = sx(r32(c, T2).wrapping_add(3));
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(3));
    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, T5)));
    c.g[T0][0] = sx(r32(c, T0).wrapping_add(r32(c, V1)));
    c.g[A0][0] = 7;
    c.g[T0][0] = sx(r32(c, T0).wrapping_add(3));
    c.g[V1][0] = 7;
    c.g[A3][0] = sx(r32(c, V0).wrapping_add(r32(c, T5)));
    c.g[V0][0] = sx(r32(c, T3).wrapping_add(r32(c, T5)));
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(3));
    c.g[T1][0] = sx(r32(c, T1).wrapping_add(r32(c, V0)));
    c.g[V0][0] = 7;
    c.g[T1][0] = sx(r32(c, T1).wrapping_add(3));
    // Each sum / 7, kept as a byte.
    c.div(0, T2, V0);
    c.cycles += 18;
    c.g[T2][0] = c.lo[0];
    c.div(0, A3, V0);
    c.g[T2][0] &= 0xff;
    c.g[A3][0] = c.lo[0];
    c.div(0, A1, V0);
    c.g[A3][0] &= 0xff;
    c.g[A1][0] = c.lo[0];
    c.div(0, T1, V0);
    c.g[A1][0] &= 0xff;
    c.g[T1][0] = c.lo[0];
    c.div(0, A2, V0);
    c.g[T1][0] &= 0xff;
    c.g[A2][0] = c.lo[0];
    c.div(0, T0, V0);
    c.g[T8][0] = c.g[A2][0] & 0xff;
    c.g[T0][0] = c.lo[0];
    c.g[T9][0] = c.g[T0][0] & 0xff;
}

/// alpha0 <= alpha1: indices 2..5 are (4a0 + a1 + 2) / 5 .. (a0 + 4a1 + 2) / 5
/// (a3, a1, t1, t8), 6 is 0 (t9) and 7 is 0xff (t2).
pub(crate) fn dxt_alpha_palette_fifths(c: &mut Cpu) {
    c.cycles += 19;
    c.g[A3][0] = sx(r32(c, T5) << 1);
    c.g[A2][0] = sx(r32(c, T6) << 1);
    c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, T5)));
    c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, T6)));
    c.g[V1][0] = sx(r32(c, T5) << 2);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
    c.g[V1][0] = sx(r32(c, T6).wrapping_add(r32(c, V1)));
    c.g[V0][0] = 5;
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(2));
    c.g[A0][0] = sx(r32(c, T6) << 2);
    c.div(0, V1, V0);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T5)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(2));
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, A3)));
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(2));
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(2));
    c.g[T1][0] = 5;
    c.cycles += 15;
    c.g[A3][0] = 5;
    c.g[T0][0] = 5;
    c.g[T9][0] = 0;
    c.g[T2][0] = 0xff;
    c.g[V1][0] = c.lo[0];
    c.div(0, A0, V0);
    c.g[T8][0] = c.g[V1][0] & 0xff;
    c.g[A0][0] = c.lo[0];
    c.div(0, A1, V0);
    c.g[A3][0] = c.g[A0][0] & 0xff;
    c.g[A1][0] = c.lo[0];
    c.div(0, A2, V0);
    c.g[A1][0] &= 0xff;
    c.g[A2][0] = c.lo[0];
    c.g[T1][0] = c.g[A2][0] & 0xff;
}

/// `SHP_DXTAlphaIsOpaque`'s loop heads (mode 3, other modes).
pub(crate) const DXT_OPAQUE_EXPLICIT_LOOP: u32 = 0x003b_3338;

pub(crate) const DXT_OPAQUE_LOOP: u32 = 0x003b_3388;

/// Bytes per block in the buffers `SHP_DXTAlphaIsOpaque` scans.
pub(crate) const DXT_OPAQUE_BLOCK: u32 = 0x20;

/// `int SHP_DXTAlphaIsOpaque(const void* blocks, int size, int mode)`
/// (0x3b3308): despite the name, 1 when some of the `size / 16` blocks
/// (32 bytes apart) is not fully opaque, else 0. Mode 3 (DXT3) blocks are
/// opaque when their first two doublewords are both 0xffffffff; other modes
/// when the first half-word is 0xffff.
pub fn shp_dxt_alpha_is_opaque(c: &mut Cpu) {
    'transparent: {
        'opaque: {
            c.cycles += 3;
            c.g[V0][0] = DXT_MODE_EXPLICIT_ALPHA;
            let explicit = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = u64::MAX;
            // Blocks: size / 16, rounded toward 0.
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, A1).wrapping_add(0xf));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[A1][0];
            }
            c.g[V1][0] = sx(((r32(c, V1) as i32) >> 4) as u32);
            let empty = (c.g[V1][0] as i64) <= 0;
            if !explicit {
                c.g[A1][0] = 0;
                if empty {
                    break 'opaque;
                }
                c.cycles += 1;
                c.g[A2][0] = 0xffff;
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, A0)).lhu(c);
                    let differs = c.g[V0][0] != c.g[A2][0];
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    if differs {
                        break 'transparent;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(DXT_OPAQUE_BLOCK));
                    if !more {
                        break 'opaque;
                    }
                    if event_due(c) {
                        deliver_event_here(c, DXT_OPAQUE_LOOP);
                    }
                }
            }
            c.g[A2][0] = 0;
            if empty {
                break 'opaque;
            }
            c.cycles += 3;
            c.g[A3][0] = 0xffff_ffff;
            loop {
                c.cycles += 3;
                c.g[A1][0] = Addr(r32(c, A0)).ld(c);
                let differs = c.g[A1][0] != c.g[A3][0];
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
                if differs {
                    break 'transparent;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A0)).ld(c);
                let differs = c.g[V0][0] != c.g[A1][0];
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                if differs {
                    break 'transparent;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(DXT_OPAQUE_BLOCK - 8));
                if !more {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    ret(c);
                    return;
                }
                if event_due(c) {
                    deliver_event_here(c, DXT_OPAQUE_EXPLICIT_LOOP);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    ret(c);
}

/// `SHP_DecodeDXT`'s mode with interpolated alpha (DXT5).
pub(crate) const DXT_MODE_INTERPOLATED_ALPHA: u64 = 5;

/// How `SHP_DecodeDXT` walks the blocks: 16-byte blocks with their alpha
/// decoded too, 16-byte blocks colour only (all opaque), or 8-byte DXT1
/// blocks. `skip` holds the rest of a block row (w * 24, added to the
/// output after each one).
pub(crate) struct DxtRows {
    alpha: bool,
    block_bytes: u32,
    color_at: u32,
    pitch_shift: u32,
    skip: usize,
    setup: u64,
    row_head: u32,
    block_head: u32,
    color_ret: u32,
}

pub(crate) const DXT_ROWS_ALPHA: DxtRows = DxtRows {
    alpha: true,
    block_bytes: 0x10,
    color_at: 8,
    pitch_shift: 3,
    skip: FP,
    setup: 2,
    row_head: 0x003b_3430,
    block_head: 0x003b_3440,
    color_ret: 0x003b_3454,
};

pub(crate) const DXT_ROWS_OPAQUE: DxtRows = DxtRows {
    alpha: false,
    block_bytes: 0x10,
    color_at: 8,
    pitch_shift: 3,
    skip: S3,
    setup: 2,
    row_head: 0x003b_34a0,
    block_head: 0x003b_34b0,
    color_ret: 0x003b_34c4,
};

pub(crate) const DXT_ROWS_DXT1: DxtRows = DxtRows {
    alpha: false,
    block_bytes: 8,
    color_at: 0,
    pitch_shift: 2,
    skip: S3,
    setup: 3,
    row_head: 0x003b_3500,
    block_head: 0x003b_3510,
    color_ret: 0x003b_3524,
};

/// `void SHP_DecodeDXT(const u8* blocks, u64* out, int w, int h, int mode)`
/// (0x3b33b0): decode a `w` x `h` DXT image block by block (4x4 pixels,
/// rows of `w / 4` blocks). Modes 3 and 5 have 16-byte blocks (alpha, then
/// colour); when `SHP_DXTAlphaIsOpaque` finds the blocks all opaque only
/// the colour halves are decoded. Other modes are DXT1 (8-byte blocks).
pub fn shp_decode_dxt(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = DXT_MODE_EXPLICIT_ALPHA;
        frame.save(c, S4, 0x50);
        frame.save(c, S6, 0x30);
        // s2 the blocks, s1 the output, s4 w, s6 h, s7 the mode.
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S1, 0x80);
        c.g[S6][0] = c.g[A3][0];
        frame.save(c, S2, 0x70);
        c.mult(0, V1, S4, S6);
        frame.save(c, S7, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x90);
        c.g[S7][0] = c.g[T0][0];
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S5, 0x40);
        frame.save(c, FP, 0x10);
        let mut paired = c.g[S7][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if !paired {
            c.cycles += 3;
            c.g[V0][0] = DXT_MODE_INTERPOLATED_ALPHA;
            paired = c.g[S7][0] == c.g[V0][0];
        }
        let rows = if paired {
            c.cycles += 6;
            c.g[A1][0] = c.g[V1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S7][0];
            callee::SHP_DXT_ALPHA_IS_OPAQUE.call(c, 0x003b_3418)?;
            if c.g[V0][0] == 0 {
                &DXT_ROWS_OPAQUE
            } else {
                &DXT_ROWS_ALPHA
            }
        } else {
            &DXT_ROWS_DXT1
        };
        c.cycles += 2;
        let none = (c.g[S6][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if none || rows.block_bytes == 8 {
            if !none {
                shp_decode_dxt_rows(c, rows)?;
            }
            c.cycles += 1;
        } else {
            shp_decode_dxt_rows(c, rows)?;
            c.cycles += 2;
        }
        frame.restore(c, S0, 0x90);
        c.cycles += 11;
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

/// `SHP_DecodeDXT`'s block rows (v1 the row, s5 the next one, s0 the
/// column, both stepping by 4 pixels).
pub(crate) fn shp_decode_dxt_rows(c: &mut Cpu, rows: &DxtRows) -> crate::call::Flow {
    c.cycles += rows.setup;
    c.g[V0][0] = 0x18;
    c.mult(0, rows.skip, S4, V0);
    loop {
        c.cycles += 3;
        c.g[S5][0] = sx(r32(c, V1).wrapping_add(4));
        let empty = (c.g[S4][0] as i64) <= 0;
        c.g[S0][0] = 0;
        if !empty {
            c.cycles += 1;
            if rows.alpha {
                c.g[S3][0] = sx(r32(c, S4) << 3);
            }
            loop {
                if rows.alpha {
                    c.cycles += 14;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = sx(r32(c, S2).wrapping_add(rows.color_at));
                    c.g[A1][0] = c.g[S3][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                    callee::SHP_DECODE_DXT_COLOR_BLOCK.call(c, rows.color_ret)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A3][0] = c.g[S7][0];
                    callee::SHP_DECODE_DXT_ALPHA_BLOCK.call(c, 0x003b_3468)?;
                } else {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S1][0];
                    // DXT1's colour half is the block itself: a plain move.
                    c.g[A2][0] = if rows.color_at == 0 {
                        c.g[S2][0]
                    } else {
                        sx(r32(c, S2).wrapping_add(rows.color_at))
                    };
                    c.g[A1][0] = sx(r32(c, S4) << rows.pitch_shift);
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                    callee::SHP_DECODE_DXT_COLOR_BLOCK.call(c, rows.color_ret)?;
                }
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x20));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(rows.block_bytes));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, rows.block_head);
                }
            }
        }
        c.cycles += 4;
        c.g[V1][0] = c.g[S5][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S6][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, rows.skip)));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, rows.row_head);
        }
    }
}

/// Allocation names of `SHP_Unpack`'s copies: `"unShp"` (decompressed),
/// `"undxt"` (DXT decoded).
pub(crate) const SHP_UNPACK_NAME: u64 = 0x0049_56e0;

pub(crate) const SHP_UNDXT_NAME: u64 = 0x0049_56e8;

/// The format DXT images are decoded to (32-bit).
pub(crate) const SHP_FORMAT_32: u64 = 5;

/// The mip loop head of `SHP_Unpack`'s DXT decode.
pub(crate) const UNPACK_DXT_LEVEL: u32 = 0x003b_37c8;

/// Copy the 16-byte header at s7 to 0(sp) through two registers (the
/// original's unaligned `ldl`/`ldr`, `sdl`/`sdr` pairs).
pub(crate) fn shp_unpack_copy_header(c: &mut Cpu, lo: usize, hi: usize) {
    let (src, dst) = (r32(c, S7), sp(c));
    c.ldl(lo, src.wrapping_add(7));
    c.ldr(lo, src);
    c.ldl(hi, src.wrapping_add(0xf));
    c.ldr(hi, src.wrapping_add(8));
    c.sdl(lo, dst.wrapping_add(7));
    c.sdr(lo, dst);
    c.sdl(hi, dst.wrapping_add(0xf));
    c.sdr(hi, dst.wrapping_add(8));
}

/// `SHPHeader* SHP_Unpack(const SHPHeader* img)` (0x3b3578): a copy of the
/// image chain with its first image in a plain format. A RefPack-compressed
/// image (format bit 7) is decompressed (`EAC_Unpack`) into a new block
/// from the unpack allocator ("unShp"); a DXT image (0x60/0x61/0x62: DXT1,
/// 3, 5) is decoded level by level to 32 bits, its size rounded up to
/// whole 4x4 blocks ("undxt"); the rest of the chain is then copied after
/// it (`SHP_Relocate`). Any other image is cloned whole (`SHP_Clone`).
/// Null for null.
pub fn shp_unpack(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S7, 0x40);
        // s7 the source image, s1 the result; the rest of the chain's size
        // at 0x10(sp), the new first block's size at 0x14(sp).
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0xb0);
        c.g[S7][0] = c.g[S1][0];
        frame.save(c, S2, 0x90);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, FP, 0x30);
        frame.save_ra(c, 0x20);
        Addr(frame.at(c, 0x10)).sw(c, 0);
        Addr(frame.at(c, 0x14)).sw(c, 0);
        let none = c.g[S1][0] == 0;
        c.g[V0][0] = 0;
        'exit: {
            if none {
                break 'exit;
            }
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> 8) as u32);
            let last = c.g[V0][0] == 0;
            c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
            if last {
                c.cycles += 1;
            } else {
                c.cycles += 2;
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    callee::SHP_CALC_TOTAL_SIZE.call(c, 0x003b_35dc)?;
                    let tail_slot = frame.at(c, 0x10);
                    Addr(tail_slot).sw(c, c.g[V0][0]);
                    c.cycles += 1;
                }
            }
            c.g[V1][0] = Addr(r32(c, S1)).lbu(c);
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & SHP_FORMAT_COMPRESSED;
            let compressed = c.g[V0][0] != 0;
            c.g[V1][0] &= 0xff;
            // Whether the DXT level loop ran (it charges its own exit).
            let decoded_levels = 'copied: {
                if compressed {
                    c.cycles += 35;
                    shp_unpack_copy_header(c, V0, V1);
                    let header = Addr(sp(c));
                    c.g[V0][0] = header.lbu(c) & 0x7f;
                    c.g[A0][0] = c.g[SP][0];
                    header.sb(c, c.g[V0][0]);
                    callee::SHP_GET_BLOCK_SIZE.call(c, 0x003b_3624)?;
                    shp_unpack_alloc(c, frame, SHP_UNPACK_NAME, true, 0x003b_364c)?;
                    c.g[A0][0] = sx(r32(c, S7).wrapping_add(SHP_DATA));
                    c.g[A1][0] = sx(r32(c, S1).wrapping_add(SHP_DATA));
                    callee::EAC_UNPACK.call(c, 0x003b_366c)?;
                    let copy = Addr(r32(c, S1));
                    c.g[V1][0] = copy.lbu(c) & 0x7f;
                    copy.sb(c, c.g[V1][0]);
                    break 'copied false;
                }
                // fp: the DXT mode (1, 3, 5) or 0.
                c.cycles += 3;
                c.g[V0][0] = SHP_FORMAT_DXT1;
                let dxt1 = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = SHP_FORMAT_DXT1 + 1;
                if dxt1 {
                    c.cycles += 2;
                    c.g[FP][0] = 1;
                } else {
                    c.cycles += 2;
                    let dxt3 = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = c.g[V1][0] ^ (SHP_FORMAT_DXT1 + 2);
                    c.cycles += 2;
                    if dxt3 {
                        c.g[FP][0] = DXT_MODE_EXPLICIT_ALPHA;
                    } else {
                        c.g[FP][0] = DXT_MODE_INTERPOLATED_ALPHA;
                        if c.g[V0][0] != 0 {
                            c.g[FP][0] = 0;
                        }
                    }
                }
                c.cycles += 2;
                let plain = c.g[FP][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if plain {
                    c.cycles += 3;
                    let tail_slot = frame.at(c, 0x10);
                    Addr(tail_slot).sw(c, 0);
                    callee::SHP_CLONE.call(c, 0x003b_384c)?;
                    c.g[S1][0] = c.g[V0][0];
                    break 'copied false;
                }
                shp_unpack_decode_dxt(c, frame)?
            };
            if !decoded_levels {
                c.cycles += 1;
            }
            c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
            shp_unpack_link_tail(c, frame)?;
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `SHP_Unpack`'s new first block (v0 its size from `SHP_GetBlockSize`):
/// the size to 0x14(sp), a block of it plus the tail from the unpack
/// allocator into s1 (`name`), the source header copied in. The two callers
/// load the slots in opposite orders: the size into a2 and the tail into v1
/// (`size_in_a2`), or the other way round.
pub(crate) fn shp_unpack_alloc(
    c: &mut Cpu,
    frame: Frame,
    name: u64,
    size_in_a2: bool,
    back: u32,
) -> crate::call::Flow {
    let size_slot = Addr(frame.at(c, 0x14));
    let tail_slot = Addr(frame.at(c, 0x10));
    size_slot.sw(c, c.g[V0][0]);
    c.g[V1][0] = 0x45_0000;
    c.g[V0][0] = Addr(SHP_UNPACK_ALLOC_HOOK).lw(c);
    c.g[A0][0] = name;
    if size_in_a2 {
        c.g[A2][0] = size_slot.lw(c);
        c.g[V1][0] = tail_slot.lw(c);
        c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
    } else {
        c.g[A2][0] = tail_slot.lw(c);
        c.g[V1][0] = size_slot.lw(c);
        c.g[A1][0] = sx(r32(c, V1).wrapping_add(r32(c, A2)));
    }
    c.g[A2][0] = 0;
    call_ptr(c, r32(c, V0), back)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S7][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = SHP_HEADER_SIZE;
    callee::EA_MEM_COPY.call(c, back + 0x14)?;
    Ok(())
}

/// `SHP_Unpack`'s DXT decode (fp the mode): a 32-bit image of the size
/// rounded up to 4x4 blocks, then each level decoded (s2 the level, s4/s3
/// its size). True when the level loop ran (its exit charges differ).
pub(crate) fn shp_unpack_decode_dxt(
    c: &mut Cpu,
    frame: Frame,
) -> Result<bool, crate::call::Unwound> {
    c.cycles += 34;
    shp_unpack_copy_header(c, A0, A1);
    let header = Addr(sp(c));
    c.g[A1][0] = header.off(SHP_HEIGHT).lhu(c);
    c.g[A2][0] = u64::MAX;
    c.g[A0][0] = header.off(SHP_WIDTH).lhu(c);
    c.g[A1][0] = sx(((r32(c, A1) << 16) as i32 >> 16) as u32);
    c.g[A0][0] = sx(((r32(c, A0) << 16) as i32 >> 16) as u32);
    // w % 4 and h % 4 (signed).
    c.g[A3][0] = ((c.g[A2][0] as i64) < (c.g[A1][0] as i64)) as u64;
    c.g[A2][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
    c.g[V1][0] = sx(r32(c, A0).wrapping_add(3));
    c.g[V0][0] = sx(r32(c, A1).wrapping_add(3));
    if c.g[A2][0] != 0 {
        c.g[V1][0] = c.g[A0][0];
    }
    if c.g[A3][0] != 0 {
        c.g[V0][0] = c.g[A1][0];
    }
    c.g[V1][0] = sx(((r32(c, V1) as i32) >> 2) as u32);
    c.g[V0][0] = sx(((r32(c, V0) as i32) >> 2) as u32);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V1][0] = sx(r32(c, A0).wrapping_sub(r32(c, V1)));
    c.g[V0][0] = sx(r32(c, A1).wrapping_sub(r32(c, V0)));
    c.g[V1][0] = sx(((r32(c, V1) << 16) as i32 >> 16) as u32);
    c.g[A2][0] = sx(((r32(c, V0) << 16) as i32 >> 16) as u32);
    c.g[S5][0] = c.g[A0][0];
    let w_rest = c.g[V1][0] != 0;
    c.g[S6][0] = c.g[A1][0];
    if w_rest {
        c.cycles += 2;
        c.g[V0][0] = sx(r32(c, S5).wrapping_add(4));
        c.g[S5][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
    }
    c.cycles += 2;
    let h_rest = c.g[A2][0] != 0;
    c.g[V0][0] = sx(r32(c, S6).wrapping_add(4));
    if h_rest {
        c.cycles += 1;
        c.g[S6][0] = sx(r32(c, V0).wrapping_sub(r32(c, A2)));
    }
    // The new header: 32-bit, the rounded size.
    c.cycles += 31;
    c.g[S0][0] = SHP_FORMAT_32;
    c.g[A0][0] = c.g[SP][0];
    let header = Addr(sp(c));
    header.off(SHP_WIDTH).sh(c, c.g[S5][0]);
    c.g[S4][0] = c.g[S5][0];
    header.off(SHP_HEIGHT).sh(c, c.g[S6][0]);
    c.g[S3][0] = c.g[S6][0];
    header.sb(c, c.g[S0][0]);
    c.g[S2][0] = 0;
    callee::SHP_GET_BLOCK_SIZE.call(c, 0x003b_3770)?;
    shp_unpack_alloc(c, frame, SHP_UNDXT_NAME, false, 0x003b_3798)?;
    let copy = Addr(r32(c, S1));
    c.g[V0][0] = copy.off(SHP_FLAGS).lw(c);
    copy.sb(c, c.g[S0][0]);
    c.g[V0][0] = sx(r32(c, V0) >> 28);
    copy.off(SHP_WIDTH).sh(c, c.g[S5][0]);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    let no_levels = (c.g[V0][0] as i64) <= 0;
    copy.off(SHP_HEIGHT).sh(c, c.g[S6][0]);
    if no_levels {
        return Ok(false);
    }
    loop {
        c.cycles += 29;
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        callee::SHP_GET_PIXEL_PTR.call(c, 0x003b_37dc)?;
        c.g[A3][0] = c.g[S2][0];
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
        callee::SHP_GET_PIXEL_PTR.call(c, 0x003b_37f8)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[T0][0] = c.g[FP][0];
        callee::SHP_DECODE_DXT.call(c, 0x003b_3810)?;
        // The next level: half the size (at least 1).
        c.g[V0][0] = Addr(r32(c, S1)).off(SHP_FLAGS).lw(c);
        c.g[A2][0] = ((c.g[S4][0] as i64) < 2) as u64;
        c.g[A0][0] = sx(((r32(c, S4) as i32) >> 1) as u32);
        c.g[A1][0] = ((c.g[S3][0] as i64) < 2) as u64;
        c.g[V0][0] = sx(r32(c, V0) >> 28);
        c.g[V1][0] = sx(((r32(c, S3) as i32) >> 1) as u32);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        if c.g[A2][0] == 0 {
            c.g[S4][0] = c.g[A0][0];
        }
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        if c.g[A1][0] == 0 {
            c.g[S3][0] = c.g[V1][0];
        }
        if !more {
            c.cycles += 2;
            return Ok(true);
        }
        if event_due(c) {
            deliver_event_here(c, UNPACK_DXT_LEVEL);
        }
    }
}

/// `SHP_Unpack`'s tail (v0 the rest of the chain's size): when there is
/// one, link the new first block to it and copy the rest of the source
/// chain there (`SHP_Relocate`).
pub(crate) fn shp_unpack_link_tail(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 2;
    let no_tail = c.g[V0][0] == 0;
    c.g[A0][0] = Addr(frame.at(c, 0x14)).lw(c);
    if !no_tail {
        c.cycles += 10;
        let first = Addr(r32(c, S1));
        c.g[V1][0] = first.lbu(c);
        c.g[V0][0] = sx(r32(c, A0) << 8);
        c.g[V1][0] |= c.g[V0][0];
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        first.sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(r32(c, S7)).lw(c);
        c.g[V0][0] = sx(((r32(c, V0) as i32) >> 8) as u32);
        c.g[A1][0] = sx(r32(c, S7).wrapping_add(r32(c, V0)));
        if c.g[V0][0] == 0 {
            c.g[A1][0] = 0;
        }
        callee::SHP_RELOCATE.call(c, 0x003b_3884)?;
    }
    Ok(())
}

/// `SHP_PaletteFormatForBpp`'s entry and end, and its jump table on
/// `bpp - 15` (15..=32).
pub(crate) const PALETTE_FORMAT_ENTRY: u32 = 0x003b_38b8;

pub(crate) const PALETTE_FORMAT_END: u32 = 0x003b_3900;

pub(crate) const PALETTE_FORMAT_TABLE: u32 = 0x0049_56f0;

/// `int SHP_PaletteFormatForBpp(int bpp)` (0x3b38b8): the format of a
/// palette with `bpp`-bit entries: 0x20 for 15/16, 0x21 for 32, 0 otherwise
/// (also for 18 and 24). Resume entries go to the generated twin.
pub fn shp_palette_format_for_bpp(c: &mut Cpu) {
    if c.pc != PALETTE_FORMAT_ENTRY {
        return ee::s13::f_003b38b8__entries(c);
    }
    c.cycles += 4;
    c.g[V1][0] = sx(r32(c, A0).wrapping_sub(15));
    c.g[V0][0] = (c.g[V1][0] < 0x12) as u64;
    let in_table = c.g[V0][0] != 0;
    c.g[A1][0] = 0;
    if in_table {
        c.cycles += 7;
        c.g[V0][0] = PALETTE_FORMAT_TABLE as u64;
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        match r32(c, A0) {
            0x003b_38e4 => {
                c.cycles += 2;
                c.g[A1][0] = 0x20;
            }
            0x003b_38ec => {
                c.cycles += 2;
                c.g[A1][0] = 0;
            }
            0x003b_38f4 => {
                c.cycles += 1;
                c.g[A1][0] = 0x21;
            }
            0x003b_38f8 => {}
            target => {
                c.jump_out(target, PALETTE_FORMAT_ENTRY, PALETTE_FORMAT_END);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    ret(c);
}

/// Format byte of the block holding an image's long name (from +4).
pub(crate) const SHP_BLOCK_LONG_NAME: u64 = 0x70;

/// The loop head of `SHP_FindLongName`.
pub(crate) const FIND_LONG_NAME_LOOP: u32 = 0x003b_3a20;

/// `const char* SHP_FindLongName(const SHPHeader* img)` (0x3b3a10): walk
/// the block chain from `img` (itself included) to the first "p" block and
/// return its name (+4); null at the end of the chain.
pub fn shp_find_long_name(c: &mut Cpu) {
    c.cycles += 2;
    let none = c.g[A0][0] == 0;
    c.g[A1][0] = SHP_BLOCK_LONG_NAME;
    if !none {
        c.cycles += 2;
        c.g[V0][0] = Addr(r32(c, A0)).lbu(c);
        loop {
            c.cycles += 2;
            if c.g[V0][0] == c.g[A1][0] {
                c.cycles += 2;
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(4));
                ret(c);
                return;
            }
            // The next block: the offset in the upper 24 bits, 0 at the end.
            c.g[V0][0] = Addr(r32(c, A0)).lw(c);
            c.cycles += 6;
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> 8) as u32);
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
            c.g[A0][0] = 0;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = c.g[V1][0];
            }
            if c.g[A0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(r32(c, A0)).lbu(c);
            if event_due(c) {
                deliver_event_here(c, FIND_LONG_NAME_LOOP);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// Palette formats (0x20 with 16-bit, 0x21 with 32-bit entries) and the
/// DXT formats (0x60..0x62).
pub(crate) const SHP_FORMAT_PALETTE16: u64 = 0x20;

pub(crate) const SHP_FORMAT_DXT1: u64 = 0x60;

/// The mip loop head of `SHP_CalcPixelDataSize`.
pub(crate) const PIXEL_DATA_SIZE_LEVEL: u32 = 0x003b_3b20;

/// `int SHP_CalcPixelDataSize(int format, int w, int h, int maxLevel, int
/// swizzled)` (0x3b3a50): the bytes of an image's pixel data plus its
/// 16-byte header (0 when there are none). A palette (formats 0x20/0x21)
/// stored swizzled pads its entry count (16..23 extra entries per 32 to 24,
/// 8..15 by 8) and has no mip levels; other images sum levels
/// 0..=`maxLevel` (each at least 1x1, DXT levels rounded up to 4x4, each
/// level after the first 16-byte aligned) of rows rounded up to bytes.
pub fn shp_calc_pixel_data_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        // s1 the format, s0 w, s5 h, s4 maxLevel, s3 swizzled, s2 the total.
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x70);
        frame.save(c, S3, 0x40);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S4, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A3][0];
        frame.save(c, S2, 0x50);
        c.g[S5][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.g[T0][0];
        // A one-byte header on the stack for SHP_GetBpp.
        let header = frame.at(c, 0);
        Addr(header).sb(c, c.g[S1][0]);
        c.g[S2][0] = 0;
        callee::SHP_GET_BPP.call(c, 0x003b_3a94)?;
        // t3: bits per pixel (15 is stored in 16).
        c.g[T3][0] = 0x10;
        c.g[A0][0] = c.g[V0][0] ^ 0xf;
        c.g[V1][0] = sx(r32(c, S1).wrapping_sub(SHP_FORMAT_PALETTE16 as u32));
        c.g[V1][0] = (c.g[V1][0] < 2) as u64;
        let palette = c.g[V1][0] != 0;
        if c.g[A0][0] != 0 {
            c.g[T3][0] = c.g[V0][0];
        }
        'exit: {
            if palette {
                c.cycles += 2;
                let swizzled = c.g[S3][0] != 0;
                c.g[V1][0] = c.g[S0][0] & 0x1f;
                if swizzled {
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x10));
                    c.g[V0][0] = (c.g[V0][0] < 8) as u64;
                    let upper = c.g[V0][0] != 0;
                    c.g[V0][0] = !0x1f;
                    if upper {
                        c.cycles += 3;
                        c.g[V0][0] &= c.g[S0][0];
                        c.g[S0][0] = sx(r32(c, V0).wrapping_add(0x18));
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(8));
                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(8));
                        c.g[V1][0] = 7;
                        c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                        if c.g[V1][0] == 0 {
                            c.g[S0][0] = c.g[A0][0];
                        }
                    }
                    // Entries * bits / 8 (signed, rounded toward 0).
                    c.cycles += 7;
                    c.mult(0, A0, S0, T3);
                    c.g[V1][0] = u64::MAX;
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(7));
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[A0][0];
                    }
                    c.g[S2][0] = sx(((r32(c, V0) as i32) >> 3) as u32);
                    break 'exit;
                }
            }
            c.cycles += 8;
            c.g[V0][0] = sx(r32(c, S1).wrapping_sub(SHP_FORMAT_DXT1 as u32));
            c.g[A3][0] = 0;
            c.g[T4][0] = (c.g[V0][0] < 3) as u64;
            c.g[T2][0] = 1;
            c.g[T1][0] = 0;
            c.g[T0][0] = !3;
            c.g[T6][0] = !0xf;
            c.g[T5][0] = u64::MAX;
            // One mip level per pass (a3 the level, a0/a2 its size).
            loop {
                c.cycles += 7;
                let level = r32(c, A3) & 31;
                c.g[A0][0] = sx(((r32(c, S0) as i32) >> level) as u32);
                c.g[A2][0] = sx(((r32(c, S5) as i32) >> level) as u32);
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V1][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = c.g[T2][0];
                }
                let dxt = c.g[T4][0] != 0;
                if c.g[V1][0] == 0 {
                    c.g[A2][0] = c.g[T2][0];
                }
                if dxt {
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(3));
                    c.g[V1][0] = sx(r32(c, A2).wrapping_add(3));
                    c.g[A0][0] = c.g[V0][0] & c.g[T0][0];
                    c.g[A2][0] = c.g[V1][0] & c.g[T0][0];
                }
                c.cycles += 2;
                let first = c.g[S2][0] == 0;
                c.g[V0][0] = sx(r32(c, S2).wrapping_add(0xf));
                if !first {
                    c.cycles += 1;
                    c.g[S2][0] = c.g[V0][0] & c.g[T6][0];
                }
                c.cycles += 11;
                c.mult(0, V0, A0, T3);
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.g[A1][0] = (c.g[S4][0] < c.g[A3][0]) as u64;
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(7));
                c.g[V1][0] = ((c.g[T5][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xe));
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.g[V0][0] = sx(((r32(c, V0) as i32) >> 3) as u32);
                c.mult(0, V1, V0, A2);
                let done = c.g[A1][0] != 0;
                c.g[S2][0] = sx(r32(c, V1).wrapping_add(r32(c, S2)));
                if done {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, PIXEL_DATA_SIZE_LEVEL);
                }
            }
        }
        c.cycles += 11;
        c.g[V0][0] = sx(r32(c, S2).wrapping_add(SHP_HEADER_SIZE as u32));
        frame.restore(c, S0, 0x70);
        if c.g[S2][0] == 0 {
            c.g[V0][0] = 0;
        }
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

/// Block formats with their own size rules: the "i" block (+6 bit 4: a
/// size at +4, 0 meaning 0xf0), an "o" block (size at +4), the long-name
/// block (a string) and the "|" block (+4 counts 8-byte entries).
pub(crate) const SHP_BLOCK_PIPE: u64 = 0x7c;

/// Format bit 7: an image whose pixels are RefPack-compressed.
pub(crate) const SHP_FORMAT_COMPRESSED: u64 = 0x80;

/// `int SHP_GetBlockSize(const SHPHeader* block)` (0x3b3bb0): the bytes of
/// one block of an image chain. "o": its size + 8; "|": 8 per entry + 8;
/// "p": the long name + NUL + 4; "i": 16, or 16 + its size; images: the
/// header and pixel data (`SHP_CalcPixelDataSize` from the header's format,
/// size, mip levels and swizzle flag bit 13), or for compressed images the
/// RefPack stream's packed size + 16 (0 when its unpacked size is 0).
pub fn shp_get_block_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        // v1 the block.
        c.g[V1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[V0][0] = SHP_BLOCK_O;
        frame.save_ra(c, 0x10);
        let block = Addr(r32(c, V1));
        c.g[A0][0] = block.lbu(c);
        let o_block = c.g[A0][0] == c.g[V0][0];
        c.g[S0][0] = 0;
        'size: {
            if o_block {
                c.cycles += 3;
                c.g[V0][0] = block.off(4).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(8));
                break 'size;
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[A0][0] < SHP_BLOCK_LONG_NAME) as u64;
            let below = c.g[V0][0] != 0;
            c.g[V0][0] = SHP_BLOCK_I;
            if below {
                c.cycles += 2;
                if c.g[A0][0] == c.g[V0][0] {
                    c.g[V0][0] = block.off(6).lhu(c);
                    c.cycles += 3;
                    c.g[V0][0] &= 0x10;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = SHP_HEADER_SIZE;
                        break 'size;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = block.off(4).lhu(c);
                    c.g[V1][0] = 0xf0;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(SHP_HEADER_SIZE as u32));
                    break 'size;
                }
                c.cycles += 2;
            } else {
                c.cycles += 3;
                c.g[V0][0] = SHP_BLOCK_LONG_NAME;
                if c.g[A0][0] == c.g[V0][0] {
                    c.cycles += 4;
                    c.g[A0][0] = sx(r32(c, V1).wrapping_add(4));
                    callee::STRLEN.call(c, 0x003b_3c14)?;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(5));
                    break 'size;
                }
                c.cycles += 3;
                c.g[V0][0] = SHP_BLOCK_PIPE;
                if c.g[A0][0] == c.g[V0][0] {
                    c.g[V0][0] = block.off(4).lw(c);
                    c.cycles += 3;
                    c.g[V0][0] = sx(r32(c, V0) << 3);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(8));
                    break 'size;
                }
                c.cycles += 2;
            }
            // An image.
            c.g[V0][0] = block.lbu(c);
            c.cycles += 3;
            c.g[V0][0] &= SHP_FORMAT_COMPRESSED;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[V0][0] = block.off(SHP_FLAGS).lw(c) & SHP_FLAG_DATA_AT_OFFSET;
                let at_offset = c.g[V0][0] != 0;
                let packed_size = frame.at(c, 0);
                Addr(packed_size).sw(c, 0);
                if at_offset {
                    c.cycles += 3;
                    c.g[V0][0] = block.off(SHP_DATA).lw(c);
                    c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = sx(r32(c, V1).wrapping_add(SHP_DATA));
                }
                c.cycles += 4;
                c.g[A1][0] = c.g[SP][0];
                callee::REFPACK_GET_SIZES.call(c, 0x003b_3c88)?;
                let empty = c.g[V0][0] == 0;
                c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
                if !empty {
                    c.cycles += 2;
                    c.g[S0][0] = sx(r32(c, V0).wrapping_add(SHP_HEADER_SIZE as u32));
                }
            } else {
                c.g[A3][0] = block.off(SHP_FLAGS).lw(c);
                c.cycles += 8;
                c.g[A2][0] = block.off(SHP_HEIGHT).lh(c);
                c.g[T0][0] = sx(((r32(c, A3) as i32) >> 13) as u32);
                c.g[A0][0] = block.lbu(c);
                c.g[A1][0] = block.off(SHP_WIDTH).lh(c);
                c.g[A3][0] = sx(r32(c, A3) >> 28);
                c.g[T0][0] &= 1;
                callee::SHP_CALC_PIXEL_DATA_SIZE.call(c, 0x003b_3cb4)?;
                c.g[S0][0] = c.g[V0][0];
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The loop head of `SHP_FindInfoData`.
pub(crate) const FIND_INFO_DATA_LOOP: u32 = 0x003b_3d68;

/// "i" block flags (+6): bit 4 set when the block has data.
pub(crate) const SHP_INFO_HAS_DATA: u64 = 0x10;

/// `void* SHP_FindInfoData(const SHPHeader* img)` (0x3b3d40): walk the
/// block chain from `img` (itself included) to the first "i" block; its
/// data (+0x10) when its +6 flags have bit 4, else null (also when there is
/// no "i" block).
pub fn shp_find_info_data(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = c.g[A0][0];
    let none = c.g[A1][0] == 0;
    c.g[A2][0] = 0;
    if !none {
        c.cycles += 4;
        c.g[V1][0] = Addr(r32(c, A1)).lbu(c);
        c.g[V0][0] = SHP_BLOCK_I;
        let mut found = c.g[V1][0] == c.g[V0][0];
        if found {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A1)).off(SHP_HEIGHT).lhu(c);
            c.g[A2][0] = sx(r32(c, A1).wrapping_add(SHP_DATA));
        } else {
            c.g[V0][0] = Addr(r32(c, A1)).lw(c);
            loop {
                // The next block (a0, also a1), or the end.
                c.cycles += 6;
                c.g[A0][0] = 0;
                c.g[V0][0] = sx(((r32(c, V0) as i32) >> 8) as u32);
                c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.g[V1][0];
                }
                let end = c.g[A0][0] == 0;
                c.g[A1][0] = c.g[A0][0];
                if end {
                    break;
                }
                c.cycles += 4;
                let block = Addr(r32(c, A0));
                c.g[V1][0] = block.lbu(c);
                c.g[V0][0] = SHP_BLOCK_I;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 2;
                    c.g[V0][0] = block.off(SHP_HEIGHT).lhu(c);
                    c.g[A2][0] = sx(r32(c, A0).wrapping_add(SHP_DATA));
                    found = true;
                    break;
                }
                c.g[V0][0] = Addr(r32(c, A1)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, FIND_INFO_DATA_LOOP);
                }
            }
        }
        if found {
            c.cycles += 2;
            c.g[V0][0] &= SHP_INFO_HAS_DATA;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = 0;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    ret(c);
}

/// Flags bit 13: the pixels are stored in GS swizzled order.
pub(crate) const SHP_FLAG_SWIZZLED: u64 = 0x2000;

/// The level loop head of `SHP_GetPixelPtr`.
pub(crate) const PIXEL_PTR_LEVEL: u32 = 0x003b_40d8;

/// `u8* SHP_GetPixelPtr(const SHPHeader* img, int x, int y, int level)`
/// (0x3b4010): the address of pixel (`x`, `y`) of mip `level`. Levels are
/// skipped by their bit sizes (each 16-bit aligned, halving w and h down to
/// 1); DXT1 (0x60) images count in 4x4 blocks of 64 bits. Palettes stored
/// swizzled index their entries as the GS does (entries 8..15 and 16..23 of
/// each 32 swapped), other swizzled images go through `SHP_SwizzleAddr`;
/// otherwise rows are 16-bit aligned. Bits 15 count as 16.
pub fn shp_get_pixel_ptr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, FP, 0x20);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S7, 0x30);
        frame.save_ra(c, 0x10);
        // s0 the image, s6/s4 x, s5/s7 y, fp the level; s1/s2 w and h, s3
        // the levels' bits, t1 bits per pixel; the pixel data at 0(sp).
        c.g[S0][0] = c.g[A0][0];
        c.g[S5][0] = c.g[A2][0];
        c.g[S6][0] = c.g[A1][0];
        let image = Addr(r32(c, S0));
        c.g[V0][0] = image.off(SHP_FLAGS).lw(c) & SHP_FLAG_DATA_AT_OFFSET;
        let at_offset = c.g[V0][0] != 0;
        c.g[FP][0] = c.g[A3][0];
        if at_offset {
            c.cycles += 3;
            c.g[V0][0] = image.off(SHP_DATA).lw(c);
            c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        } else {
            c.cycles += 1;
            c.g[V0][0] = sx(r32(c, S0).wrapping_add(SHP_DATA));
        }
        c.cycles += 18;
        let data_slot = Addr(frame.at(c, 0));
        data_slot.sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = image.off(SHP_WIDTH).lh(c);
        c.g[S2][0] = image.off(SHP_HEIGHT).lh(c);
        c.g[S4][0] = c.g[S6][0];
        callee::SHP_GET_BPP.call(c, 0x003b_4080)?;
        c.g[S7][0] = c.g[S5][0];
        c.g[A3][0] = Addr(r32(c, S0)).lbu(c);
        c.g[A0][0] = c.g[V0][0] ^ 0xf;
        c.g[T1][0] = 0x10;
        c.g[A2][0] = 0x40;
        c.g[A1][0] = c.g[A3][0] & 0xff;
        if c.g[A0][0] != 0 {
            c.g[T1][0] = c.g[V0][0];
        }
        c.g[V1][0] = c.g[A1][0] ^ SHP_FORMAT_DXT1;
        c.g[S3][0] = 0;
        c.g[V0][0] = SHP_FORMAT_DXT1;
        let dxt1 = c.g[A1][0] == c.g[V0][0];
        if c.g[V1][0] == 0 {
            c.g[T1][0] = c.g[A2][0];
        }
        if dxt1 {
            // The size in 4x4 blocks.
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, S1).wrapping_add(3));
            c.g[V1][0] = sx(r32(c, S2).wrapping_add(3));
            c.g[S1][0] = sx(((r32(c, V0) as i32) >> 2) as u32);
            c.g[S2][0] = sx(((r32(c, V1) as i32) >> 2) as u32);
        }
        c.cycles += 3;
        c.g[T3][0] = Addr(r32(c, S0)).off(SHP_FLAGS).lw(c);
        let skip_levels = (c.g[FP][0] as i64) > 0;
        c.g[T2][0] = c.g[A3][0];
        if skip_levels {
            c.cycles += 3;
            c.g[A3][0] = c.g[FP][0];
            c.g[T0][0] = !0xf;
            loop {
                c.cycles += 13;
                c.mult(0, A0, S1, S2);
                c.g[V0][0] = sx(r32(c, S3).wrapping_add(0xf));
                c.g[S3][0] = c.g[V0][0] & c.g[T0][0];
                c.g[A1][0] = ((c.g[S1][0] as i64) < 2) as u64;
                c.g[V0][0] = sx(((r32(c, S1) as i32) >> 1) as u32);
                c.g[A2][0] = ((c.g[S2][0] as i64) < 2) as u64;
                c.g[V1][0] = sx(((r32(c, S2) as i32) >> 1) as u32);
                if c.g[A1][0] == 0 {
                    c.g[S1][0] = c.g[V0][0];
                }
                c.mult(0, A0, A0, T1);
                if c.g[A2][0] == 0 {
                    c.g[S2][0] = c.g[V1][0];
                }
                c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
                let more = c.g[A3][0] != 0;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, PIXEL_PTR_LEVEL);
                }
            }
        }
        c.cycles += 4;
        c.g[A3][0] = c.g[T2][0] & 0xff;
        c.g[V0][0] = SHP_FORMAT_DXT1;
        let dxt1 = c.g[A3][0] == c.g[V0][0];
        c.g[V0][0] = sx(r32(c, T2).wrapping_sub(SHP_FORMAT_PALETTE16 as u32));
        if dxt1 {
            // x and y in blocks (signed, rounded toward 0).
            c.cycles += 10;
            c.g[V0][0] = u64::MAX;
            c.g[A0][0] = sx(r32(c, S6).wrapping_add(3));
            c.g[A1][0] = ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
            c.g[V1][0] = sx(r32(c, S5).wrapping_add(3));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S6][0] as i64)) as u64;
            if c.g[A1][0] != 0 {
                c.g[V1][0] = c.g[S5][0];
            }
            if c.g[V0][0] != 0 {
                c.g[A0][0] = c.g[S6][0];
            }
            c.g[S7][0] = sx(((r32(c, V1) as i32) >> 2) as u32);
            c.g[S4][0] = sx(((r32(c, A0) as i32) >> 2) as u32);
            c.g[V0][0] = sx(r32(c, T2).wrapping_sub(SHP_FORMAT_PALETTE16 as u32));
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        let palette = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[T3][0] & SHP_FLAG_SWIZZLED;
        // v0: the pixel's bit offset in the data.
        'offset: {
            let swizzled = c.g[V0][0] != 0;
            c.cycles += 2;
            if palette {
                c.g[A2][0] = c.g[S4][0];
                if swizzled {
                    c.cycles += 8;
                    c.g[V1][0] = 0xf;
                    c.g[V0][0] = c.g[A2][0] & 0x1f;
                    c.g[A1][0] = c.g[A2][0] ^ 0x18;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(8));
                    c.g[A0][0] = 0x21;
                    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    let entries32 = c.g[A3][0] == c.g[A0][0];
                    if c.g[V1][0] == 0 {
                        c.g[A2][0] = c.g[A1][0];
                    }
                    if entries32 {
                        c.cycles += 2;
                        c.g[A2][0] = sx(r32(c, A2) << 5);
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.g[A3][0] ^ SHP_FORMAT_PALETTE16;
                        c.g[V0][0] = sx(r32(c, A2) << 4);
                        if c.g[V1][0] == 0 {
                            c.g[A2][0] = c.g[V0][0];
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A2][0];
                    break 'offset;
                }
            } else {
                c.g[A2][0] = c.g[S1][0];
                if swizzled {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S7][0];
                    c.g[A3][0] = c.g[S2][0];
                    c.g[T0][0] = c.g[T1][0];
                    callee::SHP_SWIZZLE_ADDR.call(c, 0x003b_41b0)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.cycles += 1;
                    c.g[V0][0] = sx(r32(c, A2).wrapping_add(r32(c, S3)));
                    break 'offset;
                }
            }
            // Linear: rows of w * bits, 16-bit aligned.
            c.cycles += 7;
            c.mult(0, V0, S1, T1);
            c.g[V1][0] = !0xf;
            c.mult(0, A0, S4, T1);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xf));
            c.g[V0][0] &= c.g[V1][0];
            c.mult(0, V0, S7, V0);
            c.g[A2][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.cycles += 1;
            c.g[V0][0] = sx(r32(c, A2).wrapping_add(r32(c, S3)));
        }
        c.cycles += 15;
        c.g[V1][0] = Addr(frame.at(c, 0)).lw(c);
        c.g[V0][0] = sx(((r32(c, V0) as i32) >> 3) as u32);
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
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

/// `SHP_SwizzleAddr`'s entry and end, and its jump table on `bpp - 4`
/// (4..=32).
pub(crate) const SWIZZLE_ADDR_ENTRY: u32 = 0x003b_4218;

pub(crate) const SWIZZLE_ADDR_END: u32 = 0x003b_42ac;

pub(crate) const SWIZZLE_ADDR_TABLE: u32 = 0x0049_5740;

/// `uint SHP_SwizzleAddr(int x, int y, int w, int h, int bpp)` (0x3b4218):
/// the bit offset of pixel (`x`, `y`) in a swizzled image of width `w`, by
/// the GS layout for the depth (PSMT4, PSMT8, PSMCT16 for 15 and 16,
/// PSMCT32); 0 for other depths. Resume entries go to the generated twin.
pub fn shp_swizzle_addr(c: &mut Cpu) {
    if c.pc != SWIZZLE_ADDR_ENTRY {
        return ee::s13::f_003b4218__entries(c);
    }
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = sx(r32(c, T0).wrapping_sub(4));
        frame.save_ra(c, 0);
        // a3: x (a0 is the jump table's scratch).
        c.g[A3][0] = c.g[A0][0];
        c.g[V0][0] = (c.g[V1][0] < 0x1d) as u64;
        let in_table = c.g[V0][0] != 0;
        c.g[T1][0] = 0;
        if in_table {
            c.cycles += 7;
            c.g[V0][0] = SWIZZLE_ADDR_TABLE as u64;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            let layout = match r32(c, A0) {
                0x003b_4250 => Some((callee::SHP_SWIZZLE_ADDR_PSMT4, 4, 0x003b_4258)),
                0x003b_4260 => Some((callee::SHP_SWIZZLE_ADDR_PSMT8, 4, 0x003b_4268)),
                0x003b_4270 => Some((callee::SHP_SWIZZLE_ADDR_PSMCT16, 4, 0x003b_4278)),
                0x003b_4280 => Some((callee::SHP_SWIZZLE_ADDR_PSMCT16, 4, 0x003b_4288)),
                0x003b_4290 => Some((callee::SHP_SWIZZLE_ADDR_PSMCT32, 3, 0x003b_4298)),
                0x003b_429c => None,
                target => {
                    c.jump_out(target, SWIZZLE_ADDR_ENTRY, SWIZZLE_ADDR_END);
                    return Ok(());
                }
            };
            if let Some((swizzle, cost, back)) = layout {
                c.cycles += cost;
                c.g[A0][0] = c.g[A3][0];
                swizzle.call(c, back)?;
                c.g[T1][0] = c.g[V0][0];
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[T1][0];
        frame.close(c);
        Ok(())
    })
}

/// The loop head of `SHP_CalcTotalSize`.
pub(crate) const CALC_TOTAL_SIZE_LOOP: u32 = 0x003b_44c0;

/// Where a block after the first starts, `offset` bytes into a chain whose
/// start is `pad` bytes below a 16-byte boundary (s4: -16, s5: pad - 16):
/// the next boundary after `offset`. Palette blocks get there another way
/// (`+ 0x1f`, then `pad - 16`): the same offset, one cycle more. Shared by
/// `SHP_CalcTotalSize` and `SHP_Relocate` (registers `offset`, `pad`,
/// `mask`, `pad16` and the format byte at `block`).
pub(crate) fn shp_align_next_block(
    c: &mut Cpu,
    block: usize,
    offset: usize,
    pad: usize,
    mask: usize,
    pad16: usize,
    ret_addr: u32,
) -> crate::call::Flow {
    c.cycles += 4;
    c.g[A0][0] = Addr(r32(c, block)).lbu(c);
    callee::SHP_IS_PALETTE_FORMAT.call(c, ret_addr)?;
    let palette = c.g[V0][0] != 0;
    c.g[V0][0] = sx(r32(c, offset).wrapping_sub(r32(c, pad)));
    if palette {
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x1f));
        c.g[V0][0] &= c.g[mask][0];
        c.g[offset][0] = sx(r32(c, V0).wrapping_add(r32(c, pad16)));
    } else {
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xf));
        c.g[V0][0] &= c.g[mask][0];
        c.g[offset][0] = sx(r32(c, V0).wrapping_add(r32(c, pad)));
    }
    Ok(())
}

/// The next block of a chain into `reg` (null at the end), from the
/// offset in the upper 24 bits of its first word (v1 the offset, v0 the
/// address).
pub(crate) fn shp_next_block(c: &mut Cpu, reg: usize) {
    c.g[V1][0] = sx(((r32(c, V1) as i32) >> 8) as u32);
    c.g[V0][0] = sx(r32(c, reg).wrapping_add(r32(c, V1)));
    c.g[reg][0] = 0;
    if c.g[V1][0] != 0 {
        c.g[reg][0] = c.g[V0][0];
    }
}

/// `int SHP_CalcTotalSize(const SHPHeader* img)` (0x3b4478): the bytes of
/// a whole block chain laid out as `SHP_Relocate` lays it out: each block
/// after the first starts at the next 16-byte boundary (relative to `img`'s
/// own alignment).
pub fn shp_calc_total_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        c.g[V1][0] = !0xf;
        frame.save(c, S0, 0x60);
        // s0 the block, s1 the size, s2 the pad to a 16-byte boundary, s3
        // the block's own size.
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(0xf));
        frame.save(c, S2, 0x40);
        c.g[V0][0] &= c.g[V1][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S5, 0x10);
        c.g[S2][0] = sx(r32(c, V0).wrapping_sub(r32(c, S0)));
        let none = c.g[S0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[S4][0] = !0xf;
            c.g[S5][0] = sx(r32(c, S2).wrapping_sub(0x10));
            loop {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                callee::SHP_GET_BLOCK_SIZE.call(c, 0x003b_44c8)?;
                let first = c.g[S1][0] == 0;
                c.g[S3][0] = c.g[V0][0];
                if !first {
                    shp_align_next_block(c, S0, S1, S2, S4, S5, 0x003b_44d8)?;
                }
                c.cycles += 7;
                c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                shp_next_block(c, S0);
                let more = c.g[S0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, S3)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, CALC_TOTAL_SIZE_LOOP);
                }
            }
        }
        c.cycles += 10;
        c.g[V0][0] = c.g[S1][0];
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

/// The loop head of `SHP_Relocate`.
pub(crate) const RELOCATE_LOOP: u32 = 0x003b_45a0;

/// `"EAGL%d"`: the text `SHP_Relocate` writes into a copied "i" block.
pub(crate) const SHP_INFO_TEXT_FORMAT: u32 = 0x0049_57b8;

/// The smallest "i" block that gets that text.
pub(crate) const SHP_INFO_TEXT_MIN_SIZE: i64 = 21;

/// `void SHP_Relocate(SHPHeader* dst, const SHPHeader* src)` (0x3b4540):
/// copy the block chain at `src` to `dst`, block by block at the offsets
/// `SHP_CalcTotalSize` counts, relinking each copied block's next offset
/// (the last one ends the chain). A copied "i" block of 21 bytes or more
/// gets `"EAGL<w>"` (its +4 half) written at +0x10.
pub fn shp_relocate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xa0);
        c.g[V1][0] = !0xf;
        frame.save(c, S0, 0x90);
        // s0 the source block, s7 dst, s3 the offset, s2 the copied block,
        // s1 the block's size, s4 the pad to a 16-byte boundary.
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(0xf));
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        c.g[V0][0] &= c.g[V1][0];
        frame.save(c, S7, 0x20);
        c.g[S3][0] = 0;
        frame.save(c, S1, 0x80);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S5, 0x40);
        c.g[S4][0] = sx(r32(c, V0).wrapping_sub(r32(c, S0)));
        frame.save(c, S6, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, FP, 0x10);
        let none = c.g[S0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 4;
            c.g[S5][0] = !0xf;
            c.g[S6][0] = sx(r32(c, S4).wrapping_sub(0x10));
            c.g[FP][0] = 0x49_0000;
            loop {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                callee::SHP_GET_BLOCK_SIZE.call(c, 0x003b_45a8)?;
                let first = c.g[S3][0] == 0;
                c.g[S1][0] = c.g[V0][0];
                if !first {
                    shp_align_next_block(c, S0, S3, S4, S5, S6, 0x003b_45b8)?;
                }
                // Link the previous copy to this one.
                c.cycles += 3;
                c.g[A0][0] = c.g[S2][0];
                let first_copy = c.g[A0][0] == 0;
                c.g[S2][0] = sx(r32(c, S7).wrapping_add(r32(c, S3)));
                if !first_copy {
                    c.cycles += 5;
                    let prev = Addr(r32(c, A0));
                    c.g[V1][0] = prev.lbu(c);
                    c.g[V0][0] = sx(r32(c, S2).wrapping_sub(r32(c, A0)));
                    c.g[V0][0] = sx(r32(c, V0) << 8);
                    c.g[V1][0] |= c.g[V0][0];
                    prev.sw(c, c.g[V1][0]);
                }
                c.cycles += 3;
                let copy = Addr(r32(c, S2));
                c.g[V0][0] = copy.lbu(c);
                let empty = c.g[S1][0] == 0;
                copy.sw(c, c.g[V0][0]);
                'next: {
                    if !empty {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S1][0];
                        callee::EA_MEM_MOVE.call(c, 0x003b_4618)?;
                        c.g[V0][0] = Addr(r32(c, S2)).lbu(c);
                        c.g[V1][0] = SHP_BLOCK_I;
                        if c.g[V0][0] == c.g[V1][0] {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S1][0] as i64) < SHP_INFO_TEXT_MIN_SIZE) as u64;
                            if c.g[V0][0] == 0 {
                                c.cycles += 4;
                                c.g[A2][0] = Addr(r32(c, S2)).off(SHP_WIDTH).lh(c);
                                c.g[A0][0] = sx(r32(c, S2).wrapping_add(SHP_DATA));
                                c.g[A1][0] =
                                    sx(r32(c, FP).wrapping_add(SHP_INFO_TEXT_FORMAT & 0xffff));
                                callee::SPRINTF.call(c, 0x003b_4644)?;
                            } else {
                                c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                                break 'next;
                            }
                        } else {
                            c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                            break 'next;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                }
                c.cycles += 6;
                shp_next_block(c, S0);
                let more = c.g[S0][0] != 0;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(r32(c, S1)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, RELOCATE_LOOP);
                }
            }
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

/// `SHPHeader* SHP_Clone(const SHPHeader* img)` (0x3b4690): a copy of the
/// whole chain in a new block from the allocator hook (`SHP_CalcTotalSize`
/// bytes, 16-byte aligned, named after the image's long name, flags
/// 0x400). The allocation is not checked.
pub fn shp_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x45_0000;
        callee::SHP_CALC_TOTAL_SIZE.call(c, 0x003b_46b0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::SHP_FIND_LONG_NAME.call(c, 0x003b_46bc)?;
        c.g[V1][0] = Addr(SHP_ALLOC_HOOK).lw(c);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0x10;
        c.g[T0][0] = 0x400;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, r32(c, V1), 0x003b_46d8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        callee::SHP_RELOCATE.call(c, 0x003b_46e8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SHP_IsPaletteFormat`'s entry and end, and its jump table on
/// `format - 0x20` (0x20..=0x38).
pub(crate) const IS_PALETTE_FORMAT_ENTRY: u32 = 0x003b_4708;

pub(crate) const IS_PALETTE_FORMAT_END: u32 = 0x003b_4740;

pub(crate) const IS_PALETTE_FORMAT_TABLE: u32 = 0x0049_57c0;

/// `bool SHP_IsPaletteFormat(int format)` (0x3b4708): whether the format
/// is a palette's (0x20-0x22, 0x24, 0x29, 0x2a, 0x2c-0x2e, 0x30-0x33,
/// 0x38). Resume entries go to the generated twin.
pub fn shp_is_palette_format(c: &mut Cpu) {
    if c.pc != IS_PALETTE_FORMAT_ENTRY {
        return ee::s13::f_003b4708__entries(c);
    }
    c.cycles += 4;
    c.g[V1][0] = sx(r32(c, A0).wrapping_sub(SHP_FORMAT_PALETTE16 as u32));
    c.g[V0][0] = (c.g[V1][0] < 0x19) as u64;
    let in_table = c.g[V0][0] != 0;
    c.g[A1][0] = 0;
    if in_table {
        c.cycles += 7;
        c.g[V0][0] = IS_PALETTE_FORMAT_TABLE as u64;
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        match r32(c, A0) {
            0x003b_4734 => {
                c.cycles += 1;
                c.g[A1][0] = 1;
            }
            0x003b_4738 => {}
            target => {
                c.jump_out(target, IS_PALETTE_FORMAT_ENTRY, IS_PALETTE_FORMAT_END);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    ret(c);
}

pub(crate) mod shp_header {

    /// Pixel format byte.
    pub const FORMAT: u32 = 0x0;

    /// Image width (halfword).
    pub const WIDTH: u32 = 0x4;

    /// Image height (halfword).
    pub const HEIGHT: u32 = 0x6;

    /// Flags (bit 0x2000 selects the swizzle call).
    pub const FLAGS: u32 = 0xc;
}

/// `uint SHP_CalcPixelOffset(header, x, y, level)` (0x3b3e40): the bit
/// offset of pixel `(x, y)` at mip `level` in GS swizzled order (through
/// `SHP_GetBpp`, shifting and masking for the small formats, the
/// `SHP_SwizzleAddr` call when flags ask for it).
pub fn shp_calc_pixel_offset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S5, 0x40);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S6, 0x30);
        c.g[S6][0] = c.g[S4][0];
        frame.save(c, S7, 0x20);
        c.g[S7][0] = c.g[S5][0];
        frame.save(c, FP, 0x10);
        c.g[FP][0] = c.g[A3][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S3][0] = 0u64;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(shp_header::WIDTH));
        c.g[RA][0] = sx(0x003b_3e94);
        c.g[S1][0] = c.lh((c.g[S2][0] as u32).wrapping_add(shp_header::HEIGHT));
        callee::SHP_GET_BPP.call(c, 0x003b_3e94)?;
        c.g[A3][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(shp_header::FORMAT));
        c.g[A0][0] = c.g[V0][0] ^ 0xf;
        c.g[T1][0] = 0x10;
        c.g[A2][0] = 0x40;
        c.g[A1][0] = c.g[A3][0] & 0xff;
        if c.g[A0][0] != 0 {
            c.g[T1][0] = c.g[V0][0];
        }
        c.g[V1][0] = c.g[A1][0] ^ 0x60;
        c.g[V0][0] = 0x60;
        let direct = c.g[A1][0] != c.g[V0][0];
        if c.g[V1][0] == 0 {
            c.g[T1][0] = c.g[A2][0];
        }
        if !direct {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3));
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3));
            c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
            c.g[S1][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
        }
        // Align the running offset down to 16 bytes, `fp` times.
        c.cycles += 3;
        c.g[T2][0] = c.g[A3][0];
        let skip = (c.g[FP][0] as i64) <= 0;
        c.g[T3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(shp_header::FLAGS));
        if !skip {
            c.cycles += 2;
            c.g[A3][0] = c.g[FP][0];
            c.g[T0][0] = 0xfffffffffffffff0;
            loop {
                c.cycles += 13;
                c.mult(0, 4, 16, 17);
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xf));
                c.g[S3][0] = c.g[V0][0] & c.g[T0][0];
                c.g[A1][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                c.g[V0][0] = sx(((c.g[S0][0] as u32) as i32 >> 1) as u32);
                c.g[A2][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                c.g[V1][0] = sx(((c.g[S1][0] as u32) as i32 >> 1) as u32);
                if c.g[A1][0] == 0 {
                    c.g[S0][0] = c.g[V0][0];
                }
                c.mult(0, 4, 4, 9);
                if c.g[A2][0] == 0 {
                    c.g[S1][0] = c.g[V1][0];
                }
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                let more = c.g[A3][0] != 0u64;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_3ee0);
                }
            }
        }
        // Shift and mask for the small formats.
        c.cycles += 4;
        c.g[A3][0] = c.g[T2][0] & 0xff;
        c.g[V0][0] = 0x60;
        let small = c.g[A3][0] != c.g[V0][0];
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x20));
        if small {
            // Larger formats keep the shifted offset.
        } else {
            c.cycles += 10;
            c.g[V0][0] = 0xffffffffffffffff;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x3));
            c.g[A1][0] = ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x3));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S4][0] as i64)) as u64;
            if c.g[A1][0] != 0 {
                c.g[V1][0] = c.g[S5][0];
            }
            if c.g[V0][0] != 0 {
                c.g[A0][0] = c.g[S4][0];
            }
            c.g[S7][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
            c.g[S6][0] = sx(((c.g[A0][0] as u32) as i32 >> 2) as u32);
            c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x20));
        }
        'b_3b3fe0: {
            'b_3b3fdc: {
                'b_3b3fc0: {
                    'b_3b3f9c: {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                        let wide = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.g[T3][0] & 0x2000;
                        if wide {
                            break 'b_3b3f9c;
                        }
                        c.cycles += 2;
                        let plain = c.g[V0][0] == 0u64;
                        c.g[A2][0] = c.g[S6][0];
                        if plain {
                            break 'b_3b3fc0;
                        }
                        c.cycles += 8;
                        'b_3b3f94: {
                            'b_3b3f88: {
                                c.g[V1][0] = 0xf;
                                c.g[V0][0] = c.g[A2][0] & 0x1f;
                                c.g[A1][0] = c.g[A2][0] ^ 0x18;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
                                c.g[A0][0] = 0x21;
                                c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                                let other = c.g[A3][0] != c.g[A0][0];
                                if c.g[V1][0] == 0 {
                                    c.g[A2][0] = c.g[A1][0];
                                }
                                if other {
                                    break 'b_3b3f88;
                                }
                                c.cycles += 2;
                                c.g[A2][0] = sx((c.g[A2][0] as u32) << 5);
                                break 'b_3b3f94;
                            }
                            c.cycles += 3;
                            c.g[V1][0] = c.g[A3][0] ^ 0x20;
                            c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
                            if c.g[V1][0] == 0 {
                                c.g[A2][0] = c.g[V0][0];
                            }
                            c.cycles += 2;
                            c.g[V0][0] = c.g[A2][0];
                            break 'b_3b3fe0;
                        }
                    }
                    c.cycles += 2;
                    let swizzled = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.g[S0][0];
                    if swizzled {
                        break 'b_3b3fc0;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A1][0] = c.g[S7][0];
                    c.g[A3][0] = c.g[S1][0];
                    c.g[T0][0] = c.g[T1][0];
                    callee::SHP_SWIZZLE_ADDR.call(c, 0x003b_3fb8)?;
                    c.g[A2][0] = c.g[V0][0];
                    break 'b_3b3fdc;
                }
                c.cycles += 7;
                c.mult(0, 2, 16, 9);
                c.g[V1][0] = 0xfffffffffffffff0;
                c.mult(0, 4, 22, 9);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.mult(0, 2, 23, 2);
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                break 'b_3b3fdc;
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S3][0] as u32));
            break 'b_3b3fe0;
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

/// `int PS2_SONY_CODEC_INTERNAL_Open(SonyCodec *ctx, Script *script,
/// Stream *stream)` (0x3b0c58): (re)open the MPEG decoder on `stream`:
/// free the old work and IPU buffers (video free hook `0x509434`), parse the
/// sequence header (`MPEG2_Initialize_Buffer` + `MPEG2_Get_Hdr` into the
/// decoder globals at `0x509648`; MPEG-1 streams get progressive defaults),
/// derive the macroblock-aligned frame size (interlaced MPEG-2 rounds the
/// height to 32), allocate and link `script->count` frames, the work area
/// (`w * h * 4.5 + 0x176c`) and the IPU buffer (0x32000, uncached alias at
/// `+0x7c`), then `sceMpegInit` / `sceMpegCreate` / `sceMpegAddCallback`
/// with the no-data callback. Returns 2.
pub fn ps2_sony_codec_internal_open(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, 18, 0x80);
        frame.save(c, 19, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 30, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, 16, 0xa0);
        c.g[FP][0] = 0x510000;
        frame.save(c, 17, 0x90);
        frame.save(c, 20, 0x60);
        frame.save(c, 21, 0x50);
        frame.save(c, 22, 0x40);
        frame.save(c, 23, 0x30);
        frame.save_ra(c, 0x10);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x28), c.g[A1][0]);
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x2c));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), 0u64);
        let no_work = c.g[A0][0] == 0u64;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x14), 0u64);
        if !no_work {
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6bd0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            let t = c.g[V0][0] as u32;
            call_ptr(c, t, 0x003b_0cb4)?;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x2c), 0u64);
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x78));
        let no_ipu = c.g[A0][0] == 0u64;
        c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6bd0));
        if !no_ipu {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            let t = c.g[V0][0] as u32;
            call_ptr(c, t, 0x003b_0cd0)?;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x78), 0u64);
        }
        // Parse the sequence header into the decoder globals.
        c.cycles += 13;
        c.g[S1][0] = 0x1;
        c.g[S0][0] = 0x510000;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[S1][0]);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x69b8));
        c.g[V0][0] = 0x510000;
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
        c.sw((c.g[V0][0] as u32).wrapping_sub(0x5f78), c.g[S0][0]);
        MPEG2_INITIALIZE_BUFFER.call(c, 0x003b_0cf4)?;
        MPEG2_GET_HDR.call(c, 0x003b_0cfc)?;
        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x420));
        let mpeg2 = c.g[T1][0] != 0u64;
        c.g[V1][0] = 0x510000;
        if !mpeg2 {
            // MPEG-1: progressive 4:2:0 frame-picture defaults.
            c.cycles += 13;
            c.g[A3][0] = 0x510000;
            c.g[T0][0] = 0x510000;
            c.g[A2][0] = 0x510000;
            c.g[V1][0] = 0x3;
            c.g[A0][0] = 0x510000;
            c.g[A1][0] = 0x510000;
            c.g[V0][0] = 0x5;
            c.sw((c.g[A2][0] as u32).wrapping_sub(0x6a7c), c.g[V1][0]);
            c.sw((c.g[A0][0] as u32).wrapping_sub(0x6a74), c.g[S1][0]);
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x6ab8), c.g[V0][0]);
            c.sw((c.g[A3][0] as u32).wrapping_sub(0x6adc), c.g[S1][0]);
            c.sw((c.g[T0][0] as u32).wrapping_sub(0x6a60), c.g[S1][0]);
            c.g[V1][0] = 0x510000;
        }
        // Macroblock columns (`(hsize + 15) / 16`) into 0x5094fc, rows into
        // V0 (stored at 0x509500 below).
        'mb_rows: {
            'rows_by_16: {
                'mpeg1: {
                    c.cycles += 14;
                    c.g[A3][0] = 0xffff_ffff_ffff_ffff;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6b0c));
                    c.g[A2][0] = 0x510000;
                    c.g[A1][0] = 0x510000;
                    c.g[T3][0] = c.g[A2][0];
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
                    c.g[T2][0] = c.g[A1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1e));
                    c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[A0][0];
                    }
                    c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6b00));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                    let t = c.g[T1][0] == 0u64;
                    c.sw((c.g[A2][0] as u32).wrapping_sub(0x6b04), c.g[V0][0]);
                    if t {
                        break 'mpeg1;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = 0x510000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x6adc));
                    let progressive = c.g[V1][0] != 0u64;
                    c.g[A0][0] = 0x510000;
                    if progressive {
                        break 'rows_by_16;
                    }
                    // Interlaced: rows = (vsize + 31) / 32 * 2.
                    c.cycles += 9;
                    c.g[V1][0] = 0x510000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6b08));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1f));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e));
                    c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[A0][0];
                    }
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    break 'mb_rows;
                }
                c.cycles += 1;
                c.g[A0][0] = 0x510000;
            }
            // rows = (vsize + 15) / 16.
            c.cycles += 7;
            c.g[V1][0] = 0xffff_ffff_ffff_ffff;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x6b08));
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1e));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[A1][0];
            }
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
        }
        // 0x3b0dc8: pixel sizes (and halves) into the globals and `ctx`.
        c.cycles += 22;
        c.sw(c.g[T0][0] as u32, c.g[V0][0]);
        c.g[A0][0] = 0x510000;
        c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_sub(0x6b04));
        c.g[A2][0] = 0x510000;
        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28));
        c.g[T0][0] = 0x510000;
        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_sub(0x6b00));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.sw((c.g[A0][0] as u32).wrapping_sub(0x6b2c), c.g[V0][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.sw((c.g[A2][0] as u32).wrapping_sub(0x6b28), c.g[V1][0]);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[A1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
        c.sw((c.g[T0][0] as u32).wrapping_sub(0x6b24), c.g[V0][0]);
        c.g[A0][0] = 0x510000;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x30));
        c.sw((c.g[A0][0] as u32).wrapping_sub(0x6b20), c.g[A1][0]);
        let no_frames = (c.g[V0][0] as i64) <= 0;
        c.sw(frame.at(c, 0), c.g[V1][0]);
        if !no_frames {
            c.cycles += 6;
            c.g[S4][0] = 0x510000;
            c.g[S3][0] = c.g[V0][0];
            c.g[S7][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6bd0));
            c.g[S6][0] = 0x490000;
            c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(0x20));
            // 0x3b0e38: allocate, init and push one frame on `ctx + 0x20`.
            loop {
                c.cycles += 22;
                c.g[T0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x53e8));
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x6bd0));
                c.g[A1][0] = 0x14;
                c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = 0u64;
                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                let t = c.g[V0][0] as u32;
                c.g[A3][0] = 0u64;
                call_ptr(c, t, 0x003b_0e5c)?;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S0][0];
                SONY_CODEC_INIT_FRAME.call(c, 0x003b_0e70)?;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = 0u64;
                }
                c.sw((c.g[S2][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.sw(c.g[V1][0] as u32, c.g[A0][0]);
                let more = c.g[S3][0] != 0u64;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S5][0]);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_0e38);
                }
            }
        }
        // 0x3b0e90: work area, IPU buffer, then the libmpeg decoder.
        c.cycles += 68;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
        c.g[S0][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6bd0));
        c.g[T1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = 0x490000;
        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x53f0));
        c.mult(0, 9, 9, 2);
        c.g[A2][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_sub(0x6bd0));
        c.g[T0][0] = c.g[T0][0] | 0x400;
        c.g[A3][0] = 0u64;
        c.g[A1][0] = sx((c.g[T1][0] as u32) << 3);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T1][0] as u32));
        c.g[V1][0] = sx((c.g[A1][0] as u32) >> 31);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 1) as u32);
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x176c));
        call_ptr(c, t, 0x003b_0ed8)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
        c.g[A1][0] = 0x30000;
        c.g[A0][0] = 0x490000;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5400));
        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_sub(0x6bd0));
        c.g[A2][0] = 0u64;
        c.g[T0][0] = c.g[T0][0] | 0x400;
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = c.g[A1][0] | 0x2000;
        call_ptr(c, t, 0x003b_0f04)?;
        c.g[V1][0] = 0xfff0000;
        c.g[A0][0] = 0x30000000;
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x78), c.g[V0][0]);
        c.g[V1][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x7c), c.g[V1][0]);
        call_ptr(c, SCE_MPEG_INIT, 0x003b_0f24)?;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lw(frame.at(c, 0));
        c.mult(0, 3, 3, 2);
        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x2c));
        c.g[A2][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 31);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 1) as u32);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x176c));
        call_ptr(c, SCE_MPEG_CREATE, 0x003b_0f54)?;
        c.g[A0][0] = c.lw(frame.at(c, 0));
        c.g[A2][0] = 0x3b0000;
        c.g[A3][0] = c.g[S2][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xb10));
        c.g[A1][0] = 0x1;
        call_ptr(c, SCE_MPEG_ADD_CALLBACK, 0x003b_0f6c)?;
        frame.restore(c, 16, 0xa0);
        c.g[V0][0] = 0x2;
        frame.restore(c, 17, 0x90);
        frame.restore(c, 18, 0x80);
        frame.restore(c, 19, 0x70);
        frame.restore(c, 20, 0x60);
        frame.restore(c, 21, 0x50);
        frame.restore(c, 22, 0x40);
        frame.restore(c, 23, 0x30);
        frame.restore(c, 30, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2_SONY_CODEC_INTERNAL_dtor(...)` (0x3b09a0): free the codec's
/// buffer through the video free hook, unlink and free both frame lists,
/// then the context itself.
pub fn ps2_sony_codec_internal_dtor(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003b_09a0 => {
                    c.cycles += 14;
                    let frame = Frame::open(c, 0x60);
                    frame.save(c, 16, 0x50);
                    c.g[V0][0] = 0x450000;
                    frame.save(c, 17, 0x40);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6850));
                    frame.save(c, 18, 0x30);
                    frame.save(c, 19, 0x20);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, 20, 0x10);
                    frame.save_ra(c, 0);
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                    let t = c.g[A0][0] == 0u64;
                    c.g[S1][0] = c.g[A1][0];
                    if t {
                        pc = 0x003b_0a08;
                        continue;
                    }
                    c.cycles += 5;
                    c.g[V1][0] = 0x510000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bcc));
                    let t = c.g[V0][0] as u32;
                    call_ptr(c, t, 0x003b_09e8)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0u64);
                    pc = 0x003b_0a08;
                    continue;
                }
                0x003b_0a08 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                    let t = c.g[A0][0] == 0u64;
                    c.g[V1][0] = 0x510000;
                    if t {
                        pc = 0x003b_0a68;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bcc));
                    let t = c.g[V0][0] as u32;
                    call_ptr(c, t, 0x003b_0a04)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
                    pc = 0x003b_0a68;
                    continue;
                }
                0x003b_0a68 => {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[S3][0] = c.g[V0][0];
                    c.g[S4][0] = c.g[S1][0] & 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x003b_0ab8;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[S1][0] = 0xb;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    pc = 0x003b_0a30;
                    continue;
                }
                0x003b_0a30 => {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                    c.sw(c.g[V0][0] as u32, c.g[S1][0]);
                    let t = c.g[V1][0] == 0u64;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                    if !t {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[V1][0];
                        c.g[A1][0] = 0x3;
                        SONY_CODEC_FREE_FRAME.call(c, 0x003b_0a5c)?;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_0a30);
                        }
                        pc = 0x003b_0a30;
                        continue;
                    }
                    pc = 0x003b_0ab8;
                    continue;
                }
                0x003b_0ab8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    let t = c.g[V0][0] == c.g[S2][0];
                    c.g[S1][0] = 0xb;
                    if t {
                        pc = 0x003b_0aec;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    pc = 0x003b_0a80;
                    continue;
                }
                0x003b_0a80 => {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                    c.sw(c.g[V0][0] as u32, c.g[S1][0]);
                    let t = c.g[V1][0] == 0u64;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                    if !t {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[V1][0];
                        c.g[A1][0] = 0x3;
                        SONY_CODEC_FREE_FRAME.call(c, 0x003b_0aac)?;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    if c.g[V0][0] != c.g[S2][0] {
                        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_0a80);
                        }
                        pc = 0x003b_0a80;
                        continue;
                    }
                    pc = 0x003b_0aec;
                    continue;
                }
                0x003b_0aec => {
                    c.cycles += 9;
                    c.g[V0][0] = 0x7;
                    c.g[V1][0] = 0x450000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6810));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    let t = c.g[S4][0] == 0u64;
                    c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                    if !t {
                        c.cycles += 4;
                        c.g[V1][0] = 0x510000;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bcc));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, t, 0x003b_0aec)?;
                    }
                    c.cycles += 8;
                    let frame = Frame { size: 0x60 };
                    frame.restore(c, 16, 0x50);
                    frame.restore(c, 17, 0x40);
                    frame.restore(c, 18, 0x30);
                    frame.restore(c, 19, 0x20);
                    frame.restore(c, 20, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003b_09a0, 0x003b_0af4);
                    return Ok(());
                }
            }
        }
    })
}

/// `void AV_SubtitleSetX(AVSubtitle* self, int x)` (0x3b03a8): store the x
/// field at +0x0.
pub fn av_subtitle_set_x(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetY(AVSubtitle* self, int y)` (0x3b03b0): store the y
/// field at +0x4.
pub fn av_subtitle_set_y(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x4).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetWidth(AVSubtitle* self, int w)` (0x3b03b8): store the
/// width field at +0x8.
pub fn av_subtitle_set_width(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x8).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetHeight(AVSubtitle* self, int h)` (0x3b03c0): store the
/// height field at +0xc.
pub fn av_subtitle_set_height(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0xc).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetStringId(AVSubtitle* self, int id)` (0x3b03c8): store
/// the string id field at +0x10.
pub fn av_subtitle_set_string_id(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x10).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetFrame(AVSubtitle* self, int frame)` (0x3b03d0): store
/// the frame field at +0x14.
pub fn av_subtitle_set_frame(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x14).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetJustifyX(AVSubtitle* self, int jx)` (0x3b03d8): store
/// the x-justify field at +0x18.
pub fn av_subtitle_set_justify_x(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x18).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetJustifyY(AVSubtitle* self, int jy)` (0x3b03e0): store
/// the y-justify field at +0x1c.
pub fn av_subtitle_set_justify_y(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x1c).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetShape(AVSubtitle* self, int shape)` (0x3b03e8): store
/// the shape field at +0x20.
pub fn av_subtitle_set_shape(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x20).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetColor0(AVSubtitle* self, int color)` (0x3b03f0):
/// store the first color byte at +0x24.
pub fn av_subtitle_set_color0(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x24).sb(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetColor1(AVSubtitle* self, int color)` (0x3b03f8):
/// store the second color byte at +0x25.
pub fn av_subtitle_set_color1(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x25).sb(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetColor2(AVSubtitle* self, int color)` (0x3b0400):
/// store the third color byte at +0x26.
pub fn av_subtitle_set_color2(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x26).sb(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetColor3(AVSubtitle* self, int color)` (0x3b0408):
/// store the fourth color byte at +0x27.
pub fn av_subtitle_set_color3(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x27).sb(c, c.g[A1][0]);
    abi::ret(c);
}

/// `int AV_StrICmp(const char*, const char*)` (0x3af378): case-insensitive
/// string compare, used here to match subtitle justification names.
pub(crate) const AV_STR_ICMP: Callee = Callee {
    addr: 0x003a_f378,
    f: ee::s13::f_003af378,
};

/// `void AV_SkipWhitespace(const char**, int*)` (0x3af300): advance over
/// leading whitespace, used here before token matching.
pub(crate) const AV_SKIP_WHITESPACE: Callee = Callee {
    addr: 0x003a_f300,
    f: ee::s13::f_003af300,
};

/// `float RCMP_DECODER_GetFrameRate(RCMP_DECODER*)` (0x3b05c0): the codec's
/// frame rate (0 when it has no object yet).
pub(crate) const RCMP_DECODER_GET_FRAME_RATE: Callee = Callee {
    addr: 0x003b_05c0,
    f: ee::s13::f_003b05c0,
};

/// `int AV_Player_GetTimeMs(AV_Player*)` (0x3ae9a0): the player's media
/// clock in milliseconds (ported below; called by the frame-due test).
pub(crate) const AV_PLAYER_GET_TIME_MS: Callee = Callee {
    addr: 0x003a_e9a0,
    f: ee::s13::f_003ae9a0,
};

/// `int SND_GetOutputLatency(void)` (0x3b5f60): nonzero path calls the
/// platform latency hook, but both arms return 0.
pub(crate) const SND_GET_OUTPUT_LATENCY: Callee = Callee {
    addr: 0x003b_5f60,
    f: ee::s13::f_003b5f60,
};

/// `void* AV_Player_ctor(AV_Player* self, int a1, int a2, int a3, int a4)`
/// (0x3addc0): clear the word at +0x60, then initialise with
/// `(a1, 0, a2, 0, 0, 0, a3, a4)` (the last word travels on the stack).
/// Returns `self`.
pub fn av_player_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[T3][0] = c.g[A3][0];
        frame.save(c, S0, 0x20);
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let stacked = c.g[T0][0];
        c.sw(frame.at(c, 0), stacked);
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x60).sw(c, 0);
        c.g[A2][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        AV_PLAYER_INIT.call(c, 0x003a_ddf4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void* AV_Player_ctor_3(AV_Player* self, ...)` (0x3ade58): clear the word
/// at +0x60 and forward all eight constructor arguments to
/// `AV_Player_Init` (the eighth arrives on the caller's stack). Returns
/// `self`.
pub fn av_player_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[V0][0] = Addr(frame.at(c, 0x30)).lw(c);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x60).sw(c, 0);
        let stacked = c.g[V0][0];
        c.sw(frame.at(c, 0), stacked);
        AV_PLAYER_INIT.call(c, 0x003a_de78)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void* AV_Player_ctor_4(AV_Player* self, ...)` (0x3ade90): clear the word
/// at +0x60, forward the stacked seventh argument, and initialise with a
/// constant 2 in the seventh register slot. Returns `self`.
pub fn av_player_ctor_4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let stacked = c.g[T3][0];
        c.sw(frame.at(c, 0), stacked);
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x60).sw(c, 0);
        c.g[T3][0] = 0x2;
        AV_PLAYER_INIT.call(c, 0x003a_deb0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void MPEG2_Error(void)` (0x3b11e8): the error stub is a bare return;
///
/// callers report through their own paths.
pub fn mpeg2_error(c: &mut Cpu) {
    c.cycles += 2;
    c.pc = c.g[RA][0] as u32;
}

/// `void MPEG2_user_data(void)` (0x3b2380): skip user data by scanning for
/// the next start code.
pub fn mpeg2_user_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::NEXT_START_CODE.call(c, 0x003b_2390)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 MPEG2_Get_Bits1(void)` (0x3b1270): one bit from the stream.
pub fn mpeg2_get_bits1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x1;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1280)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 MPEG2_marker_bit(void)` (0x3b2360): one marker bit from the stream.
pub fn mpeg2_marker_bit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x1;
        callee::MPEG2_GET_BITS.call(c, 0x003b_2370)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MPEG2_copyright_extension(void)` (0x3b23a0): parse the copyright
/// extension (flag, id, original/copy flags, reserved, number, marker
/// triplets) into the decoder globals.
pub fn mpeg2_copyright_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x1;
        callee::MPEG2_GET_BITS.call(c, 0x003b_23b0)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x8;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x69f0)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_23c0)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x69ec)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_23d0)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x7;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x69e8)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_23e0)?;
        c.g[A0][0] = 0x49_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5630));
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_23ec)?;
        c.g[A0][0] = 0x14;
        callee::MPEG2_GET_BITS.call(c, 0x003b_23f4)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x49_0000;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x69e4)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5658));
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_2408)?;
        c.g[A0][0] = 0x16;
        callee::MPEG2_GET_BITS.call(c, 0x003b_2410)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x49_0000;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x69e0)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5688));
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_2424)?;
        c.g[A0][0] = 0x16;
        callee::MPEG2_GET_BITS.call(c, 0x003b_242c)?;
        c.g[V1][0] = 0x51_0000;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x69dc)).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MPEG2_sequence_display_extension(void)` (0x3b1cc0): parse the
/// sequence display extension (format, colour flag, display size triple or
/// the plain tail, marker) into the decoder globals.
pub fn mpeg2_sequence_display_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x3;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1cd0)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ac8)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1ce0)?;
        c.g[V1][0] = 0x51_0000;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ac4)).sw(c, c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 12;
            c.g[A0][0] = 0x8;
            callee::MPEG2_GET_BITS.call(c, 0x003b_1cf4)?;
            c.g[V1][0] = 0x51_0000;
            c.g[A0][0] = 0x8;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6ac0)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_1d04)?;
            c.g[V1][0] = 0x51_0000;
            c.g[A0][0] = 0x8;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6abc)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_1d14)?;
            c.g[V1][0] = 0x51_0000;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6ab8)).sw(c, c.g[V0][0]);
        }
        c.cycles += 14;
        c.g[A0][0] = 0xe;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1d24)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x49_0000;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ab4)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x54d0));
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_1d38)?;
        c.g[A0][0] = 0xe;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1d40)?;
        c.g[V1][0] = 0x51_0000;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ab0)).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MPEG2_sequence_scalable_extension(void)` (0x3b1ee0): parse the
/// sequence scalable extension (layer selector, scalable mode nibble, …).
/// When the scalable mode is not 2 the plain tail runs; mode 2 with a
/// non-2 lower field reports through `MPEG2_Error`.
pub fn mpeg2_sequence_scalable_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = 0x2;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0x51_0000;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1ef8)?;
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x5f78)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = 0x4;
        Addr((c.g[V1][0] as u32).wrapping_add(0x424)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1f0c)?;
        c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x5f78)).lw(c);
        c.g[V1][0] = 0x51_0000;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6a24)).sw(c, c.g[V0][0]);
        c.g[A0][0] = 0x2;
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x424)).lw(c);
        if c.g[V0][0] != c.g[A0][0] {
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x5f78)).lw(c);
        } else {
            c.cycles += 28;
            c.g[A0][0] = 0xe;
            callee::MPEG2_GET_BITS.call(c, 0x003b_1f30)?;
            c.g[V1][0] = 0x51_0000;
            c.g[A0][0] = 0x49_0000;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a20)).sw(c, c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x54f0));
            callee::MPEG2_MARKER_BIT.call(c, 0x003b_1f44)?;
            c.g[A0][0] = 0xe;
            callee::MPEG2_GET_BITS.call(c, 0x003b_1f4c)?;
            c.g[V1][0] = 0x51_0000;
            c.g[A0][0] = 0x5;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a1c)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_1f5c)?;
            c.g[V1][0] = 0x51_0000;
            c.g[A0][0] = 0x5;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a18)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_1f6c)?;
            c.g[V1][0] = 0x51_0000;
            c.g[A0][0] = 0x5;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a14)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_1f7c)?;
            c.g[V1][0] = 0x51_0000;
            c.g[A0][0] = 0x5;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a10)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_1f8c)?;
            c.g[V1][0] = 0x51_0000;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a0c)).sw(c, c.g[V0][0]);
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x5f78)).lw(c);
        }
        c.cycles += 4;
        c.g[A0][0] = 0x4;
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x424)).lw(c);
        frame.restore(c, S0, 0x10);
        if c.g[V1][0] != c.g[A0][0] {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
        } else {
            c.cycles += 4;
            c.g[A0][0] = 0x49_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5510));
            callee::MPEG2_ERROR.call(c, 0x003b_1fb4)?;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
        }
        Ok(())
    })
}

/// `void MPEG2_picture_coding_extension(void)` (0x3b20b8): parse the
/// picture coding extension (f-codes, intra/predictive vectors, divisibility
/// flags, qscale type, intra-VLC, alternate scan) into the decoder globals.
pub fn mpeg2_picture_coding_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 68;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x4;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x51_0000;
        callee::MPEG2_GET_BITS.call(c, 0x003b_20d4)?;
        c.g[S0][0] = 0x51_0000;
        c.g[A0][0] = 0x4;
        Addr((c.g[S0][0] as u32).wrapping_sub(0x6a90)).sw(c, c.g[V0][0]);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6a90));
        callee::MPEG2_GET_BITS.call(c, 0x003b_20e8)?;
        c.g[A0][0] = 0x4;
        Addr((c.g[S0][0] as u32).wrapping_add(0x4)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_20f4)?;
        c.g[A0][0] = 0x4;
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_2100)?;
        c.g[A0][0] = 0x2;
        Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_210c)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x2;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6a80)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_211c)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6a7c)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_212c)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6a78)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_213c)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6a74)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_214c)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6a70)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_215c)?;
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_sub(0x5f78)).lw(c);
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_add(0x428)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_216c)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6a6c)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_217c)?;
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_sub(0x5f78)).lw(c);
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_add(0x42c)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_218c)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6a68)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_219c)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6a64)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_21ac)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6a60)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_21bc)?;
        c.g[V1][0] = 0x51_0000;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6a5c)).sw(c, c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 20;
            c.g[A0][0] = 0x1;
            callee::MPEG2_GET_BITS.call(c, 0x003b_21d0)?;
            c.g[V1][0] = 0x51_0000;
            c.g[A0][0] = 0x3;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a58)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_21e0)?;
            c.g[V1][0] = 0x51_0000;
            c.g[A0][0] = 0x1;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a54)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_21f0)?;
            c.g[V1][0] = 0x51_0000;
            c.g[A0][0] = 0x7;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a50)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_2200)?;
            c.g[V1][0] = 0x51_0000;
            c.g[A0][0] = 0x8;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a4c)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_2210)?;
            c.g[V1][0] = 0x51_0000;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a48)).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MPEG2_slice_header(void)` (0x3b1878): parse a slice header (slice
/// row from the vertical position or the stream, quantiser scale, extra
/// bit) and return the slice row in `v0`.
pub fn mpeg2_slice_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x51_0000;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0x5f78)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x420)).lw(c);
        c.g[S0][0] = c.g[V0][0];
        if c.g[A0][0] != 0 {
            c.cycles += 5;
            c.g[V1][0] = 0x51_0000;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x6b08)).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < 2801i64) as u64;
            c.g[S1][0] = 0u64;
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[A0][0] = 0x3;
                callee::MPEG2_GET_BITS.call(c, 0x003b_18b8)?;
                c.g[S1][0] = c.g[V0][0];
            }
        } else {
            c.cycles += 1;
            c.g[S1][0] = 0u64;
        }
        c.cycles += 5;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x5f78)).lw(c);
        c.g[A0][0] = 0x1;
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x424)).lw(c);
        if c.g[V1][0] == c.g[A0][0] {
            c.cycles += 4;
            c.g[A0][0] = 0x7;
            callee::MPEG2_GET_BITS.call(c, 0x003b_18e0)?;
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x5f78)).lw(c);
            Addr((c.g[V1][0] as u32).wrapping_add(0x434)).sw(c, c.g[V0][0]);
        }
        c.cycles += 7;
        c.g[A0][0] = 0x5;
        callee::MPEG2_GET_BITS.call(c, 0x003b_18f0)?;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x5f78)).lw(c);
        c.g[V1][0] = c.g[V0][0];
        if Addr((c.g[A0][0] as u32).wrapping_add(0x420)).lw(c) != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x428)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 1);
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x45_0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3d28));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lbu(c);
            }
        } else {
            c.g[V0][0] = c.g[V1][0];
        }
        c.cycles += 5;
        Addr((c.g[A0][0] as u32).wrapping_add(0x438)).sw(c, c.g[V0][0]);
        c.g[A0][0] = 0x1;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1930)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x5f78)).lw(c);
            c.cycles += 2;
            Addr((c.g[V0][0] as u32).wrapping_add(0x43c)).sw(c, 0u64);
            c.g[V0][0] = c.g[S1][0];
        } else {
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x5f78)).lw(c);
            c.cycles += 12;
            c.g[A0][0] = 0x1;
            callee::MPEG2_GET_BITS.call(c, 0x003b_1940)?;
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x5f78)).lw(c);
            c.g[A0][0] = 0x1;
            Addr((c.g[V1][0] as u32).wrapping_add(0x43c)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_1950)?;
            c.g[A0][0] = 0x6;
            callee::MPEG2_GET_BITS.call(c, 0x003b_1958)?;
            callee::EXTRA_BIT_INFORMATION.call(c, 0x003b_1960)?;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MPEG2_picture_header(void)` (0x3b17a8): parse a picture header
/// (temporal reference, type, VBV delay, full-pel/vector flags for
/// predictive pictures) then run the extra-bit, extension and temporal
/// tails.
pub fn mpeg2_picture_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x51_0000;
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0x5f78)).lw(c);
        c.g[A0][0] = 0xa;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0x51_0000;
        Addr((c.g[V1][0] as u32).wrapping_add(0x430)).sw(c, 0u64);
        callee::MPEG2_GET_BITS.call(c, 0x003b_17cc)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x3;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6aac)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_17dc)?;
        c.g[A0][0] = 0x10;
        Addr((c.g[S0][0] as u32).wrapping_sub(0x6aa8)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_17e8)?;
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x6aa8)).lw(c);
        c.g[A0][0] = 0x51_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
        c.g[V1][0] = (c.g[V1][0] < 0x2u64) as u64;
        Addr((c.g[A0][0] as u32).wrapping_sub(0x6aa4)).sw(c, c.g[V0][0]);
        if c.g[V1][0] != 0 {
            c.cycles += 8;
            c.g[A0][0] = 0x1;
            callee::MPEG2_GET_BITS.call(c, 0x003b_1808)?;
            c.g[V1][0] = 0x51_0000;
            c.g[A0][0] = 0x3;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6aa0)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_1818)?;
            c.g[V1][0] = 0x51_0000;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a9c)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x6aa8)).lw(c);
        c.g[V0][0] = 0x3;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 8;
            c.g[A0][0] = 0x1;
            callee::MPEG2_GET_BITS.call(c, 0x003b_1838)?;
            c.g[V1][0] = 0x51_0000;
            c.g[A0][0] = 0x3;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a98)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_1848)?;
            c.g[V1][0] = 0x51_0000;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6a94)).sw(c, c.g[V0][0]);
        }
        c.cycles += 10;
        callee::EXTRA_BIT_INFORMATION.call(c, 0x003b_1858)?;
        callee::EXTENSION_AND_USER_DATA.call(c, 0x003b_1860)?;
        callee::UPDATE_TEMPORAL_REFERENCE_TACKING_DATA.call(c, 0x003b_1868)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MPEG2_sequence_header(void)` (0x3b14f0): parse the sequence header
/// (size, aspect, rate, bitrate, VBV, constrained and quant-matrix flags)
/// into the decoder state, install the default matrices for any table not
/// loaded from the stream, then run the extension/user-data tail.
pub fn mpeg2_sequence_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x40);
        c.g[A0][0] = 0xc;
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = 0x51_0000;
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[S0][0];
        callee::MPEG2_GET_BITS.call(c, 0x003b_1514)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0xc;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6b0c)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1524)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x4;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6b08)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1534)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x4;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6af4)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1544)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x12;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6af0)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1554)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x49_0000;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6aec)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5450));
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_1568)?;
        c.g[A0][0] = 0xa;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1570)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ae8)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1580)?;
        c.g[V1][0] = 0x51_0000;
        c.g[A0][0] = 0x1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ae4)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1590)?;
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x5f78)).lw(c);
        Addr((c.g[V1][0] as u32).wrapping_add(0x410)).sw(c, c.g[V0][0]);
        if c.g[V0][0] == 0 {
            // No intra matrix in the stream: fill the default table.
            c.cycles += 5;
            c.g[V0][0] = 0x45_0000;
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx(0x45_0000 - 0x3d68);
            c.g[S0][0] = 0u64;
            loop {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < 64i64) as u64;
                Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_15f0);
                }
            }
        } else {
            // Intra matrix present: stream 64 bytes through the zigzag.
            c.cycles += 3;
            c.g[V0][0] = 0x45_0000;
            c.g[S0][0] = 0u64;
            c.g[S1][0] = sx(0x45_0000 - 0x3de8);
            loop {
                c.cycles += 11;
                c.g[A0][0] = 0x8;
                callee::MPEG2_GET_BITS.call(c, 0x003b_15b0)?;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_sub(0x5f78)).lw(c);
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lbu(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = ((c.g[S0][0] as i64) < 64i64) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                Addr((c.g[A1][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                if c.g[A0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_15a8);
                }
            }
            c.cycles += 2;
        }
        c.cycles += 5;
        c.g[A0][0] = 0x1;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1614)?;
        c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_sub(0x5f78)).lw(c);
        Addr((c.g[V1][0] as u32).wrapping_add(0x414)).sw(c, c.g[V0][0]);
        if c.g[V0][0] == 0 {
            // No non-intra matrix: fill the default 0x10 table.
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x110));
            c.g[S0][0] = 0x3fu64;
            c.g[V1][0] = 0x10;
            loop {
                c.cycles += 7;
                Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                if (c.g[S0][0] as i64) < 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_1670);
                }
            }
            c.cycles += 1;
            c.g[V1][0] = 0x51_0000;
        } else {
            // Non-intra matrix present: stream it, then fall out.
            c.cycles += 4;
            c.g[V0][0] = 0x45_0000;
            c.g[S0][0] = 0u64;
            c.g[S1][0] = sx(0x45_0000 - 0x3de8);
            loop {
                c.cycles += 11;
                c.g[A0][0] = 0x8;
                callee::MPEG2_GET_BITS.call(c, 0x003b_1638)?;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_sub(0x5f78)).lw(c);
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lbu(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = ((c.g[S0][0] as i64) < 64i64) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                Addr((c.g[A1][0] as u32).wrapping_add(0x110)).sw(c, c.g[V0][0]);
                if c.g[A0][0] == 0 {
                    c.cycles += 2;
                    c.g[V1][0] = 0x51_0000;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_1630);
                }
            }
        }
        // Rotate the four 64-word matrix windows, then the tail.
        c.cycles += 4;
        c.g[S0][0] = 0x3fu64;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x5f78)).lw(c);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x310));
        loop {
            c.cycles += 7;
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_sub(0x300)).lw(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_sub(0x200)).lw(c);
            Addr((c.g[A0][0] as u32).wrapping_sub(0x100)).sw(c, c.g[V1][0]);
            Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            if (c.g[S0][0] as i64) < 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_16a0);
            }
        }
        c.cycles += 8;
        callee::EXTENSION_AND_USER_DATA.call(c, 0x003b_16c4)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 MPEG2_Get_Hdr(void)` (0x3b13d8): scan for the next start code and
/// dispatch: sequence headers and pictures return through their parsers, a
/// group-of-pictures header continues the scan, `0x1b7` ends it with 0 and a
/// picture header ends it with 1.
pub fn mpeg2_get_hdr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0x1b8;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0x1b7;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0x100;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0x1b3;
        loop {
            c.cycles += 7;
            callee::NEXT_START_CODE.call(c, 0x003b_1408)?;
            callee::GET_BITS32.call(c, 0x003b_1410)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 436i64) as u64;
            if c.g[V1][0] == c.g[S3][0] {
                // Sequence header code: parse it, keep scanning.
                c.cycles += 4;
                callee::SEQUENCE_HEADER.call(c, 0x003b_1454)?;
                if event_due(c) {
                    deliver_event_here(c, 0x003b_1400);
                }
                continue;
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                // Below the extension range: picture code or keep scanning.
                c.cycles += 2;
                if c.g[V1][0] == c.g[S2][0] {
                    c.cycles += 3;
                    callee::PICTURE_HEADER.call(c, 0x003b_1474)?;
                    c.g[V0][0] = 0x1;
                    break;
                }
                c.cycles += 2;
                if event_due(c) {
                    deliver_event_here(c, 0x003b_1400);
                }
                continue;
            }
            c.cycles += 2;
            if c.g[V1][0] == c.g[S1][0] {
                // Sequence end: done, 0.
                c.g[V0][0] = 0u64;
                break;
            }
            c.cycles += 2;
            if c.g[V1][0] != c.g[S0][0] {
                c.cycles += 2;
                if event_due(c) {
                    deliver_event_here(c, 0x003b_1400);
                }
                continue;
            }
            // Group-of-pictures header: parse it, keep scanning.
            c.cycles += 4;
            callee::GROUP_OF_PICTURES_HEADER.call(c, 0x003b_1464)?;
            if event_due(c) {
                deliver_event_here(c, 0x003b_1400);
            }
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

/// `void MPEG2_quant_matrix_extension(void)` (0x3b1d58): parse the quant
/// matrix extension: four load flags, each followed by 64 streamed bytes
/// for the intra, non-intra and chroma tables when set.
pub fn mpeg2_quant_matrix_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[A0][0] = 0x1;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0x51_0000;
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[S0][0];
        callee::MPEG2_GET_BITS.call(c, 0x003b_1d7c)?;
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x5f78)).lw(c);
        Addr((c.g[V1][0] as u32).wrapping_add(0x410)).sw(c, c.g[V0][0]);
        if c.g[V0][0] == 0 {
            // No intra matrix: fall through to the non-intra flag below.
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x45_0000;
            c.g[S0][0] = 0u64;
            c.g[S2][0] = sx(0x45_0000 - 0x3de8);
            loop {
                c.cycles += 13;
                c.g[A0][0] = 0x8;
                callee::MPEG2_GET_BITS.call(c, 0x003b_1da0)?;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_sub(0x5f78)).lw(c);
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lbu(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A2][0] = ((c.g[S0][0] as i64) < 64i64) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = c.g[A0][0];
                Addr((c.g[A0][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                Addr((c.g[A1][0] as u32).wrapping_add(0x210)).sw(c, c.g[V0][0]);
                if c.g[A2][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_1d98);
                }
            }
        }
        c.cycles += 5;
        c.g[A0][0] = 0x1;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1dd4)?;
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_sub(0x5f78)).lw(c);
        Addr((c.g[V1][0] as u32).wrapping_add(0x414)).sw(c, c.g[V0][0]);
        if c.g[V0][0] == 0 {
            // No non-intra matrix.
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x45_0000;
            c.g[S0][0] = 0u64;
            c.g[S2][0] = sx(0x45_0000 - 0x3de8);
            loop {
                c.cycles += 13;
                c.g[A0][0] = 0x8;
                callee::MPEG2_GET_BITS.call(c, 0x003b_1df8)?;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_sub(0x5f78)).lw(c);
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lbu(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A2][0] = ((c.g[S0][0] as i64) < 64i64) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = c.g[A0][0];
                Addr((c.g[A0][0] as u32).wrapping_add(0x110)).sw(c, c.g[V0][0]);
                Addr((c.g[A1][0] as u32).wrapping_add(0x310)).sw(c, c.g[V0][0]);
                if c.g[A2][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_1df0);
                }
            }
        }
        c.cycles += 5;
        c.g[A0][0] = 0x1;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1e2c)?;
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_sub(0x5f78)).lw(c);
        Addr((c.g[V1][0] as u32).wrapping_add(0x418)).sw(c, c.g[V0][0]);
        if c.g[V0][0] == 0 {
            // No chroma intra matrix.
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x45_0000;
            c.g[S0][0] = 0u64;
            c.g[S2][0] = sx(0x45_0000 - 0x3de8);
            loop {
                c.cycles += 11;
                c.g[A0][0] = 0x8;
                callee::MPEG2_GET_BITS.call(c, 0x003b_1e50)?;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_sub(0x5f78)).lw(c);
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lbu(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = ((c.g[S0][0] as i64) < 64i64) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                Addr((c.g[A1][0] as u32).wrapping_add(0x210)).sw(c, c.g[V0][0]);
                if c.g[A0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_1e48);
                }
            }
        }
        c.cycles += 5;
        c.g[A0][0] = 0x1;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1e7c)?;
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_sub(0x5f78)).lw(c);
        Addr((c.g[V1][0] as u32).wrapping_add(0x41c)).sw(c, c.g[V0][0]);
        if c.g[V0][0] == 0 {
            // No chroma non-intra matrix.
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x45_0000;
            c.g[S0][0] = 0u64;
            c.g[S2][0] = sx(0x45_0000 - 0x3de8);
            loop {
                c.cycles += 11;
                c.g[A0][0] = 0x8;
                callee::MPEG2_GET_BITS.call(c, 0x003b_1ea0)?;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_sub(0x5f78)).lw(c);
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lbu(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = ((c.g[S0][0] as i64) < 64i64) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                Addr((c.g[A1][0] as u32).wrapping_add(0x310)).sw(c, c.g[V0][0]);
                if c.g[A0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_1e98);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MPEG2_next_start_code(void)` (0x3b1498): byte-align the stream
/// and flush bytes until the 24-bit prefix `0x000001` shows, looping
/// through the dispatcher heads on events.
pub fn mpeg2_next_start_code(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003b_1498 => {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.g[V0][0] = 0x51_0000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S0][0] = 0x1;
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0x5f78)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
                    c.g[A0][0] &= 0x7;
                    callee::MPEG2_FLUSH_BUFFER.call(c, 0x003b_14bc)?;
                    pc = 0x003b_14d0;
                    continue;
                }
                0x003b_14c4 => {
                    c.cycles += 1;
                    pc = 0x003b_14c8;
                    continue;
                }
                0x003b_14c8 => {
                    c.cycles += 2;
                    callee::MPEG2_FLUSH_BUFFER.call(c, 0x003b_14d0)?;
                    pc = 0x003b_14d0;
                    continue;
                }
                0x003b_14d0 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x18;
                    callee::MPEG2_SHOW_BITS.call(c, 0x003b_14d8)?;
                    c.g[A0][0] = 0x8;
                    if c.g[V0][0] != c.g[S0][0] {
                        if back_edge_unwinds(c, 0x003b_14c8) {
                            return Ok(());
                        }
                        pc = 0x003b_14c8;
                        continue;
                    }
                    pc = 0x003b_14e0;
                    continue;
                }
                0x003b_14e0 => {
                    c.cycles += 4;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = c.g[RA][0] as u32;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003b_1498, 0x003b_14f0);
                    return Ok(());
                }
            }
        }
    })
}

/// `void MPEG2_extension_and_user_data(void)` (0x3b1988): scan for the next
/// start code and dispatch extension ids 1..10 through the extension table
/// (other ids rescan); user data and anything else ends the scan.
pub fn mpeg2_extension_and_user_data(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003b_1988 => {
                    c.cycles += 2;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    pc = 0x003b_1990;
                    continue;
                }
                0x003b_1990 => {
                    c.cycles += 4;
                    callee::NEXT_START_CODE.call(c, 0x003b_1998)?;
                    pc = 0x003b_1a84;
                    continue;
                }
                0x003b_19a0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1b5;
                    if c.g[V1][0] == c.g[V0][0] {
                        pc = 0x003b_19ac;
                        continue;
                    }
                    pc = 0x003b_1a74;
                    continue;
                }
                0x003b_19ac => {
                    c.cycles += 8;
                    callee::MPEG2_FLUSH_BUFFER32.call(c, 0x003b_19b4)?;
                    c.g[A0][0] = 0x4;
                    callee::MPEG2_GET_BITS.call(c, 0x003b_19bc)?;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    c.g[V0][0] = 0x49_0000;
                    if c.g[V1][0] >= 0xau64 {
                        if back_edge_unwinds(c, 0x003b_1990) {
                            return Ok(());
                        }
                        pc = 0x003b_1990;
                        continue;
                    }
                    pc = 0x003b_19cc;
                    continue;
                }
                0x003b_19cc => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5490));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                    pc = c.g[A0][0] as u32;
                    continue;
                }
                0x003b_19e4 => {
                    c.cycles += 4;
                    callee::SEQUENCE_EXTENSION.call(c, 0x003b_19ec)?;
                    if back_edge_unwinds(c, 0x003b_1990) {
                        return Ok(());
                    }
                    pc = 0x003b_1990;
                    continue;
                }
                0x003b_19f4 => {
                    c.cycles += 4;
                    callee::SEQUENCE_DISPLAY_EXTENSION.call(c, 0x003b_19fc)?;
                    if back_edge_unwinds(c, 0x003b_1990) {
                        return Ok(());
                    }
                    pc = 0x003b_1990;
                    continue;
                }
                0x003b_1a04 => {
                    c.cycles += 4;
                    callee::QUANT_MATRIX_EXTENSION.call(c, 0x003b_1a0c)?;
                    if back_edge_unwinds(c, 0x003b_1990) {
                        return Ok(());
                    }
                    pc = 0x003b_1990;
                    continue;
                }
                0x003b_1a14 => {
                    c.cycles += 4;
                    callee::SEQUENCE_SCALABLE_EXTENSION.call(c, 0x003b_1a1c)?;
                    if back_edge_unwinds(c, 0x003b_1990) {
                        return Ok(());
                    }
                    pc = 0x003b_1990;
                    continue;
                }
                0x003b_1a24 => {
                    c.cycles += 4;
                    callee::PICTURE_DISPLAY_EXTENSION.call(c, 0x003b_1a2c)?;
                    if back_edge_unwinds(c, 0x003b_1990) {
                        return Ok(());
                    }
                    pc = 0x003b_1990;
                    continue;
                }
                0x003b_1a34 => {
                    c.cycles += 4;
                    callee::PICTURE_CODING_EXTENSION.call(c, 0x003b_1a3c)?;
                    if back_edge_unwinds(c, 0x003b_1990) {
                        return Ok(());
                    }
                    pc = 0x003b_1990;
                    continue;
                }
                0x003b_1a44 => {
                    c.cycles += 4;
                    callee::PICTURE_SPATIAL_SCALABLE_EXTENSION.call(c, 0x003b_1a4c)?;
                    if back_edge_unwinds(c, 0x003b_1990) {
                        return Ok(());
                    }
                    pc = 0x003b_1990;
                    continue;
                }
                0x003b_1a54 => {
                    c.cycles += 4;
                    callee::PICTURE_TEMPORAL_SCALABLE_EXTENSION.call(c, 0x003b_1a5c)?;
                    if back_edge_unwinds(c, 0x003b_1990) {
                        return Ok(());
                    }
                    pc = 0x003b_1990;
                    continue;
                }
                0x003b_1a64 => {
                    c.cycles += 4;
                    callee::COPYRIGHT_EXTENSION.call(c, 0x003b_1a6c)?;
                    if back_edge_unwinds(c, 0x003b_1990) {
                        return Ok(());
                    }
                    pc = 0x003b_1990;
                    continue;
                }
                0x003b_1a74 => {
                    c.cycles += 4;
                    callee::MPEG2_FLUSH_BUFFER32.call(c, 0x003b_1a7c)?;
                    callee::USER_DATA.call(c, 0x003b_1a84)?;
                    pc = 0x003b_1a84;
                    continue;
                }
                0x003b_1a84 => {
                    c.cycles += 6;
                    c.g[A0][0] = 0x20;
                    callee::MPEG2_SHOW_BITS.call(c, 0x003b_1a8c)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = 0x1b5;
                    c.g[V0][0] = 0x1b2;
                    if c.g[V1][0] == 0x1b5 {
                        if back_edge_unwinds(c, 0x003b_19ac) {
                            return Ok(());
                        }
                        pc = 0x003b_19ac;
                        continue;
                    }
                    pc = 0x003b_1a9c;
                    continue;
                }
                0x003b_1a9c => {
                    c.cycles += 2;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if c.g[V1][0] == c.g[V0][0] {
                        if back_edge_unwinds(c, 0x003b_19a0) {
                            return Ok(());
                        }
                        pc = 0x003b_19a0;
                        continue;
                    }
                    pc = 0x003b_1aa4;
                    continue;
                }
                0x003b_1aa4 => {
                    c.cycles += 2;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = c.g[RA][0] as u32;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003b_1988, 0x003b_1aac);
                    return Ok(());
                }
            }
        }
    })
}

/// `cPS2ChunkResolve_v08` (0x003a_dc48): straight-line leaf. Mechanical port of the generated body.
pub fn c_ps2_chunk_resolve_v08_003adc48(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `PS2_SONY_CODEC_INTERNAL_v05` (0x003b_0fa0): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn ps2_sony_codec_internal_v05_003b0fa0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

pub(crate) const MPEG2_GET_BITS: Callee = Callee {
    addr: 0x003b_1300,
    f: ee::s13::f_003b1300,
};

pub(crate) const MPEG2_MARKER_BIT: Callee = Callee {
    addr: 0x003b_2360,
    f: ee::s13::f_003b2360,
};

pub(crate) const MPEG2_EXTENSION_AND_USER_DATA: Callee = Callee {
    addr: 0x003b_1988,
    f: ee::s13::f_003b1988,
};

pub(crate) const RCMP_STATIC_INIT_DESTROY: Callee = Callee {
    addr: 0x003b_0770,
    f: ee::s13::f_003b0770,
};

/// `RCMP_DECODER_Reset` (0x3b0680): calls the method in slot `+0x34` of the
/// object at `self + 0x14` (its `this` is `obj + vt[+0x30]`, a 16-bit
/// offset), passing `a1` through.
pub fn rcmp_decoder_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x003b_06a0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `RCMP_DECODER_ReleaseChunk(self, blk)` (0x3b06f8): calls the function at
/// `self + 0xc` with `a0 = self`, `a1 = *(self + 4)`, `a2 = blk`.
pub fn rcmp_decoder_release_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let t = c.g[V1][0] as u32;
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        call_ptr(c, t, 0x003b_0714)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `RCMP_GlobalCtors` (0x3b07b8): `RCMP_StaticInitDestroy(1, 0xffff)`.
pub fn rcmp_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        RCMP_STATIC_INIT_DESTROY.call(c, 0x003b_07cc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `RCMP_GlobalDtors` (0x3b07d8): `RCMP_StaticInitDestroy(0, 0xffff)`.
pub fn rcmp_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0u64;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        RCMP_STATIC_INIT_DESTROY.call(c, 0x003b_07ec)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `MPEG2_group_of_pictures_header` (0x3b16e0): reads the GOP fields
/// through `MPEG2_Get_Bits` and `MPEG2_marker_bit`, then
/// `MPEG2_extension_and_user_data`. A first-GOP check (the word at
/// 0x510000 - 0x5f78 against 0x509648) also bumps the counter pair at
/// 0x450000 - 0x3c80 / 0x3c88 before the reads.
pub fn mpeg2_group_of_pictures_header_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = 0x510000;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5f78));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x69b8));
        let t = c.g[A0][0] != c.g[V1][0];
        frame.save_ra(c, 0);
        if !t {
            c.cycles += 8;
            c.g[V1][0] = 0x450000;
            c.g[A1][0] = 0x450000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x3c84));
            c.g[A0][0] = 0x450000;
            c.g[V1][0] = 0x1;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw((c.g[A0][0] as u32).wrapping_sub(0x3c80), c.g[V1][0]);
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x3c88), c.g[V0][0]);
        }
        c.cycles += 35;
        c.g[A0][0] = 0x1;
        MPEG2_GET_BITS.call(c, 0x003b_1724)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x5;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69d8), c.g[V0][0]);
        MPEG2_GET_BITS.call(c, 0x003b_1734)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x6;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69d4), c.g[V0][0]);
        MPEG2_GET_BITS.call(c, 0x003b_1744)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x490000;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69d0), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5468));
        MPEG2_MARKER_BIT.call(c, 0x003b_1758)?;
        c.g[A0][0] = 0x6;
        MPEG2_GET_BITS.call(c, 0x003b_1760)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x6;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69cc), c.g[V0][0]);
        MPEG2_GET_BITS.call(c, 0x003b_1770)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x1;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69c8), c.g[V0][0]);
        MPEG2_GET_BITS.call(c, 0x003b_1780)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x1;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69c4), c.g[V0][0]);
        MPEG2_GET_BITS.call(c, 0x003b_1790)?;
        c.g[V1][0] = 0x510000;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69c0), c.g[V0][0]);
        MPEG2_EXTENSION_AND_USER_DATA.call(c, 0x003b_179c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `RCMP_CODEC::~RCMP_CODEC(this, flags)` (0x42e278): stores the codec vtable
/// word (0x456810) at `this`, then, when bit 0 of `flags` is set, calls the
/// function at `D_00509434[0]`.
pub fn rcmp_codec_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x450000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6810));
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let call_hook = c.g[A1][0] != 0u64;
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        if call_hook {
            c.cycles += 4;
            c.g[V1][0] = 0x510000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bcc));
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0042_e2a4)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `RCMP_SYSTEM::~RCMP_SYSTEM(this, flags)` (0x42e498): stores the system
/// vtable word (0x456890) at `this + 0xc`, then deletes the object (through
/// the function table, 0x317e50) when bit 0 of `flags` is set.
pub fn rcmp_system_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x450000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6890));
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let delete_object = c.g[A1][0] != 0u64;
        c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        if delete_object {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0042_e4bc)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::game::render::shps::{SHPS_COUNT, SHPS_ENTRIES, SHPS_ENTRY_OFFSET, SHPS_ENTRY_SIZE};
    use crate::game::replay::replaycache::tests::oc420_hook_mem_free;

    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::ai::ai::tests::strict;
    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::ai::ai::tests::SEEDS as SEEDS_10;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS as SEEDS_7;
    pub(crate) use crate::game::camera::script::scriptcontroller::tests::SEEDS_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::pool;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::world;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_9;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_8;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3;

    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_5;

    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::fe::ovtemplatedialog::tests::ZERO_STUB;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_4;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_8;
    pub(crate) use crate::game::movie::movieplayer::tests::RET0;
    pub(crate) use crate::game::object::flexrailnode::tests::base_3 as base_7;
    pub(crate) use crate::game::object::flexrailnode::tests::check_all;
    pub(crate) use crate::game::object::flexrailnode::tests::floats;
    pub(crate) use crate::game::object::railmodifier::tests::destructor_setup;
    pub(crate) use crate::game::object::railmodifier::tests::h;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;

    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::ui::uivector::tests::start;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6;
    use ssx3_ee as ee;

    use ssx3_ee::s13;

    #[test]
    fn c_wscript_cache_get_data_matches() {
        check_lohi(
            ssx3_ee::s13::f_003aca38,
            c_wscript_cache_get_data,
            200,
            &|c, seed| {
                let mut r = Rng(0xe001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (r.next_u32() % 8) as u32;
                c.g[A1][0] = index as u64;
                c.bus.write32(DATA + 0x4, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + index * 0xc + 8, r.next_u32());
            },
        );
    }

    #[test]
    fn t_wscript_cache_ctor_matches() {
        check(
            ssx3_ee::s13::f_003aca50,
            t_wscript_cache_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0xe002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn ssx_app_get_game_module_matches() {
        check(
            ssx3_ee::s13::f_003ad188,
            ssx_app_get_game_module,
            200,
            &|c, seed| {
                let mut r = Rng(0xe003 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = DATA + 0x100;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, r.next_u32());
            },
        );
    }

    #[test]
    fn game_module_get_occlusion_man_matches() {
        check(
            ssx3_ee::s13::f_003ad198,
            game_module_get_occlusion_man,
            200,
            &|c, seed| {
                let mut r = Rng(0xe004 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = DATA + 0x100;
                let module = DATA + 0x200;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, module);
                c.bus.write32(module + 0x8c, r.next_u32());
            },
        );
    }

    #[test]
    fn t_wscript_cache_get_table_entry_matches() {
        check(
            ssx3_ee::s13::f_003adc50,
            t_wscript_cache_get_table_entry,
            200,
            &|c, seed| {
                let mut r = Rng(0xe005 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (r.next_u32() % 8) as u32;
                c.g[A1][0] = index as u64;
                let ty = if seed % 3 == 0 {
                    3u16
                } else {
                    r.next_u32() as u16
                };
                c.bus.write16(DATA, ty);
                let table = if seed % 4 == 0 { 0u32 } else { DATA + 0x100 };
                c.bus.write32(DATA + 0x8, table);
                c.bus.write32(table + 0x3c, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + index * 4, r.next_u32());
            },
        );
    }

    /// Shared setup for the four `tWScriptCache_Get*` field accessors.
    pub(crate) fn wscript_field_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0xe006 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        let ty = if seed % 3 == 0 {
            3u16
        } else {
            r.next_u32() as u16
        };
        c.bus.write16(DATA, ty);
        let table = if seed % 4 == 0 { 0u32 } else { DATA + 0x100 };
        c.bus.write32(DATA + 0x8, table);
        for off in [0x20u32, 0x24, 0x28, 0x2c] {
            c.bus.write32(table + off, r.next_u32());
        }
    }

    #[test]
    fn t_wscript_cache_get_process_count_matches() {
        check(
            ssx3_ee::s13::f_003adce0,
            t_wscript_cache_get_process_count,
            200,
            &wscript_field_setup,
        );
    }

    #[test]
    fn t_wscript_cache_get_processes_matches() {
        check(
            ssx3_ee::s13::f_003add10,
            t_wscript_cache_get_processes,
            200,
            &wscript_field_setup,
        );
    }

    #[test]
    fn t_wscript_cache_get_mission_count_matches() {
        check(
            ssx3_ee::s13::f_003add40,
            t_wscript_cache_get_mission_count,
            200,
            &wscript_field_setup,
        );
    }

    #[test]
    fn t_wscript_cache_get_missions_matches() {
        check(
            ssx3_ee::s13::f_003add70,
            t_wscript_cache_get_missions,
            200,
            &wscript_field_setup,
        );
    }

    #[test]
    fn av_subtitle_init_matches() {
        check(
            ssx3_ee::s13::f_003b0340,
            av_subtitle_init,
            100,
            &|c, seed| {
                let mut r = Rng(0xe007 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn rcmp_system_ctor_matches() {
        check(
            ssx3_ee::s13::f_003b0410,
            rcmp_system_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0xe008 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
            },
        );
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
        check(
            ssx3_ee::s13::f_003b0738,
            rcmp_decoder_init_callbacks,
            100,
            &|c, seed| {
                let mut r = Rng(0xe00a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn rcmp_make_callbacks_matches() {
        check(
            ssx3_ee::s13::f_003b0758,
            rcmp_make_callbacks,
            200,
            &|c, seed| {
                let mut r = Rng(0xe00b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[T0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ps2_sony_codec_internal_ctor_matches() {
        check(
            ssx3_ee::s13::f_003b0948,
            ps2_sony_codec_internal_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0xe00c ^ seed as u64);
                scramble(c, &mut r);
                let base = DATA + (seed % 4) * 8;
                c.g[A0][0] = base as u64;
            },
        );
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
        check(
            ssx3_ee::s13::f_003b1258,
            mpeg2_show_bits,
            200,
            &|c, seed| {
                let mut r = Rng(0xf002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | (r.next_u32() % 64) as u64;
                c.bus.write32(0x0050_a088, DATA + 0x100);
                c.bus.write32(DATA + 0x104, r.next_u32());
            },
        );
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
                c.bus.write32(
                    0x0050_9648,
                    if seed % 3 == 0 {
                        0x0050_9648
                    } else {
                        r.next_u32()
                    },
                );
                c.bus
                    .write32(0x0050_9554, if seed % 2 == 0 { 3 } else { r.next_u32() });
                c.bus
                    .write32(0x0044_c3c8, if seed % 4 == 0 { 0 } else { r.next_u32() });
                c.bus
                    .write32(0x0044_c380, if seed % 5 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn shp_palette_entries_for_bpp_matches() {
        check(
            ssx3_ee::s13::f_003b2688,
            shp_palette_entries_for_bpp,
            100,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn shp_get_bpp_matches() {
        check(ssx3_ee::s13::f_003b39d8, shp_get_bpp, 200, &|c, seed| {
            let mut r = Rng(0xf005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let b = r.next_u32() as u8;
            c.bus.write8(DATA, b);
            c.bus
                .write8(SHP_BPP_TABLE + (b & 0x7f) as u32, r.next_u32() as u8);
        });
    }

    #[test]
    fn shp_format_from_bpp_matches() {
        check(
            ssx3_ee::s13::f_003b3da8,
            shp_format_from_bpp,
            200,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn shp_swizzle_addr_psmt4_matches() {
        check_lohi(
            ssx3_ee::s13::f_003b42b0,
            shp_swizzle_addr_psmt4,
            200,
            &|c, seed| {
                let mut r = Rng(0xf008 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn shp_swizzle_addr_psmt8_matches() {
        check_lohi(
            ssx3_ee::s13::f_003b4338,
            shp_swizzle_addr_psmt8,
            200,
            &|c, seed| {
                let mut r = Rng(0xf009 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn shp_swizzle_addr_psmct16_matches() {
        check_lohi(
            ssx3_ee::s13::f_003b43c0,
            shp_swizzle_addr_psmct16,
            200,
            &|c, seed| {
                let mut r = Rng(0xf00a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn shp_swizzle_addr_psmct32_matches() {
        check_lohi(
            ssx3_ee::s13::f_003b4448,
            shp_swizzle_addr_psmct32,
            200,
            &|c, seed| {
                let mut r = Rng(0xf00b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    pub(crate) const SEEDS: u32 = 300;

    // ------------------------------------------------------ AV player/audio

    /// An `AV_Player` at DATA with a stream at DATA+0x100; `audio` adds an
    /// audio object at DATA+0x200 pointing at the stream.
    pub(crate) fn av_player(c: &mut Cpu, r: &mut Rng, audio: bool) -> u32 {
        let this = DATA;
        let stream = DATA + 0x100;
        c.g[A0][0] = this as u64;
        c.bus.write32(this + 0x5c, stream);
        c.bus.write32(stream, r.next_u32());
        c.bus.write32(
            stream + 0x10,
            if r.next_u32() % 2 == 0 {
                0x1000
            } else {
                r.next_u32() % 0x4000
            },
        );
        c.bus.write64(
            stream + 8,
            ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
        );
        c.bus.write32(this + 0x50, r.next_u32());
        c.bus.write32(this + 0x58, r.next_u32());
        if audio {
            let a = DATA + 0x200;
            c.bus.write32(this, a);
            c.bus.write32(a + 0xc, stream);
            c.bus.write32(a + 0x10, r.next_u32());
            c.bus
                .write64(a + 8, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
        } else {
            c.bus.write32(this, 0);
        }
        this
    }

    #[test]
    fn av_player_rate_wrappers_match() {
        for (gen, hand, salt) in [
            (
                ee::s13::f_003ae938 as fn(&mut Cpu),
                av_player_pause as fn(&mut Cpu),
                0xe938u64,
            ),
            (ee::s13::f_003ae958, av_player_resume, 0xe958),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                av_player(c, &mut r, seed % 2 == 0);
            });
        }
    }

    #[test]
    fn av_player_set_rate_matches() {
        check(
            ee::s13::f_003ae888,
            av_player_set_rate,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xe888, 0);
                av_player(c, &mut r, seed % 2 == 0);
                c.g[A1][0] = if seed % 3 == 0 {
                    0x4000
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    #[test]
    fn av_player_is_audio_done_matches() {
        check(
            ee::s13::f_003ae860,
            av_player_is_audio_done,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xe860, 0);
                av_player(c, &mut r, seed % 3 != 0);
            },
        );
    }

    #[test]
    fn av_player_set_volume_matches() {
        check(
            ee::s13::f_003ae978,
            av_player_set_volume,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xe978, 0);
                av_player(c, &mut r, seed % 3 != 0);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    /// An `AV_Audio` at DATA with a stream handle at `field` (or -1).
    pub(crate) fn av_audio(c: &mut Cpu, r: &mut Rng, field: u32, neg: bool) {
        let audio = DATA;
        c.g[A0][0] = audio as u64;
        c.bus
            .write32(audio + field, if neg { 0xffff_ffff } else { DATA + 0x100 });
        crate::difftest::fill(c, r, DATA + 0x100, 0x40);
    }

    #[test]
    fn av_audio_pitch_volume_match() {
        for (gen, hand, field, salt) in [
            (
                ee::s13::f_003aee98 as fn(&mut Cpu),
                av_audio_set_pitch as fn(&mut Cpu),
                0xcu32,
                0xee98u64,
            ),
            (ee::s13::f_003aef48, av_audio_set_volume, 0xc, 0xef48),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                av_audio(c, &mut r, field, seed % 3 == 0);
                c.g[A1][0] = sx(r.next_u32());
            });
        }
    }

    #[test]
    fn av_audio_start_matches() {
        check(ee::s13::f_003aeef8, av_audio_start, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xeef8, 0);
            av_audio(c, &mut r, 0x10, seed % 3 == 0);
        });
    }

    #[test]
    fn av_audio_is_done_matches() {
        check(ee::s13::f_003aefa8, av_audio_is_done, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xefa8, 0);
            av_audio(c, &mut r, 0xc, false);
        });
    }

    /// Subtitle records (0x34 bytes, frame at +0x14) at DATA+0x200, list head
    /// at DATA. Returns the list.
    pub(crate) fn subtitle_list(c: &mut Cpu, r: &mut Rng, n: u32) -> u32 {
        let list = DATA;
        let base = DATA + 0x200;
        c.bus.write32(list, base);
        c.bus.write32(list + 4, n);
        for i in 0..n {
            c.bus.write32(base + i * 0x34 + 0x14, i * 10);
            c.bus.write32(base + i * 0x34 + 0x18, r.next_u32());
        }
        list
    }

    #[test]
    fn av_free_subtitles_matches() {
        covered(ee::s13::f_003af020, av_free_subtitles, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xaf020, 1);
            let obj = DATA;
            let base = DATA + 0x100;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(base.wrapping_sub(0x10), seed % 4);
            c.bus.write32(obj, if seed % 5 == 0 { 0 } else { base });
            c.bus.write32(obj + 4, r.next_u32());
        });
    }

    #[test]
    fn av_find_subtitle_by_frame_matches() {
        check_lohi(
            ee::s13::f_003af258,
            av_find_subtitle_by_frame,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x3af258 ^ seed as u64);
                scramble(c, &mut r);
                let n = seed % 6;
                subtitle_list(c, &mut r, n);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = sx(r.next_u32() % 60);
            },
        );
    }

    #[test]
    fn av_player_get_current_subtitle_matches() {
        check(
            ee::s13::f_003ae718,
            av_player_get_current_subtitle,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0xe718, 0);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let list = if seed % 4 == 0 {
                    0
                } else {
                    subtitle_list(c, &mut r, seed % 5)
                };
                c.bus.write32(this + 0x70, list);
                c.bus.write32(
                    this + 0x74,
                    if seed % 2 == 0 { 0 } else { r.next_u32() % 50 },
                );
                c.bus.write32(this + 0x44, r.next_u32() % 50);
                c.bus.write32(this + 0x48, r.edge_f32());
            },
        );
    }

    #[test]
    fn av_player_release_chunk_callback_matches() {
        check(
            ee::s13::f_003aed20,
            av_player_release_chunk_callback,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xed20, 0);
                let player = DATA;
                let chunk = DATA + 0x300;
                let user = DATA + 0x500;
                c.bus.write32(user, player);
                c.bus.write32(player + 0x30, DATA + 0x100);
                c.bus.write32(chunk, r.next_u32());
                c.g[A1][0] = user as u64;
                c.g[A2][0] = if seed % 3 == 0 { 0 } else { chunk as u64 };
            },
        );
    }

    #[test]
    fn av_skip_whitespace_matches() {
        check(
            ee::s13::f_003af300,
            av_skip_whitespace,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x3af300 ^ seed as u64);
                scramble(c, &mut r);
                for ch in [0x20u32, 0x09, 0x0a, 0x0d, 0x0b, 0x0c] {
                    c.bus.write8(CTYPE + ch, 8);
                }
                let s = DATA + 0x1000;
                let ws = seed % 6;
                for i in 0..8u32 {
                    let ch = if i < ws {
                        [0x20u8, 0x09, 0x0a, 0x0d][(seed as usize + i as usize) % 4]
                    } else {
                        b'a' + (i as u8 % 26)
                    };
                    c.bus.write8(s + i, ch);
                }
                let count = if seed % 5 == 0 {
                    r.next_u32() % 8
                } else {
                    seed % 8
                };
                c.bus.write32(DATA, s);
                c.bus.write32(DATA + 4, count);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 4) as u64;
            },
        );
    }

    #[test]
    fn av_str_icmp_matches() {
        check(ee::s13::f_003af378, av_str_icmp, SEEDS, &|c, seed| {
            let mut r = Rng(0x3af378 ^ seed as u64);
            scramble(c, &mut r);
            for ch in b'A'..=b'Z' {
                c.bus.write8(CTYPE + ch as u32, 1);
            }
            let a = DATA + 0x1000;
            let b = DATA + 0x2000;
            let n = (seed % 5) as usize;
            for i in 0..n {
                let ch = [b'a', b'A', b'b', b'B', b'1', b'0'][(r.next_u32() as usize) % 6];
                c.bus.write8(a + i as u32, ch);
            }
            c.bus.write8(a + n as u32, 0);
            for i in 0..n {
                // Often mirror `a` with a case flip so the compare hits 0.
                let ch = if seed % 3 == 0 {
                    let x = c.bus.read8(a + i as u32);
                    if x.is_ascii_lowercase() {
                        x - 32
                    } else {
                        x
                    }
                } else {
                    [b'a', b'A', b'b', b'B', b'1', b'0'][(r.next_u32() as usize) % 6]
                };
                c.bus.write8(b + i as u32, ch);
            }
            c.bus.write8(b + n as u32, 0);
            c.g[A0][0] = a as u64;
            c.g[A1][0] = b as u64;
        });
    }

    pub(crate) const BLOCK: u32 = DATA + 0x2000;

    #[test]
    fn shp_decode_dxt_color_block_matches() {
        check(
            ee::s13::f_003b2e78__generated,
            shp_decode_dxt_color_block,
            SEEDS_2,
            &|c, seed| {
                let mut r = Rng(0x003b_2e78 ^ (seed as u64) << 11);
                scramble(c, &mut r);
                // Both endpoint orders (four-colour mode and three-colour mode).
                let (a, b) = match seed % 4 {
                    0 => (0x001f_u32, 0x7fff_u32),
                    1 => (0x7fff, 0x001f),
                    _ => (r.next_u32() & 0xffff, r.next_u32() & 0xffff),
                };
                let idx = r.next_u32();
                c.bus
                    .write64(BLOCK, a as u64 | (b as u64) << 16 | (idx as u64) << 32);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(0x40);
                c.g[A2][0] = sx(BLOCK);
            },
        );
    }

    #[test]
    fn mpeg2_picture_temporal_scalable_extension_matches() {
        crate::difftest::check(
            ee::s13::f_003b22f8__generated,
            mpeg2_picture_temporal_scalable_extension,
            30,
            &|c, seed| {
                machine(c, seed, 0x3b22_f8, 1);
            },
        );
    }

    #[test]
    fn cwscript_cache_set_data_matches() {
        check_lohi(
            s13::f_003aca10__generated,
            cwscript_cache_set_data,
            150,
            &|c, seed| {
                machine(c, seed, 0x3aca10, 1);
                clear(c, OBJ, 0x40);
                clear(c, CHILD, 0x80);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(seed % 5);
                c.g[A2][0] = sx(seed as u32 ^ 0x5a5a);
                c.bus.write32(OBJ + 4, CHILD);
            },
        );
    }

    #[test]
    fn shp_free_matches() {
        check(s13::f_003b2b68__generated, shp_free, 120, &|c, seed| {
            machine(c, seed, 0x3b2b68, 1);
            clear(c, OBJ, 0x80);
            // `D_0044C458` is newlib's `free` hook; the null slot is faked.
            c.bus.write32(0x0044_c458, 0);
            c.g[A0][0] = sx(OBJ + (seed as u32 & 0x3c));
        });
    }

    #[test]
    fn c_ps2_chunk_resolve_base_dtor_matches() {
        check(
            ee::s13::f_003adc18__generated,
            c_ps2_chunk_resolve_base_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3adc_18),
        );
    }

    #[test]
    fn mpeg2_initialize_buffer_matches() {
        check(
            ee::s13::f_003b1228__generated,
            mpeg2_initialize_buffer,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x3b12_28, 2);
                be_library(c);
                // The global bitstream descriptor and the data block it points at.
                c.bus.write32(0x0050_a088, DATA + 0x100);
                c.g[A0][0] = sx(DATA + 0x400);
            },
        );
    }

    #[test]
    fn mpeg2_get_bits32_matches() {
        check(
            ee::s13::f_003b13a8__generated,
            mpeg2_get_bits32,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b13_a8, 2);
                be_library(c);
                c.bus.write32(0x0050_a088, DATA + 0x100);
                c.bus.write32(DATA + 0x104, r.next_u32());
                // A bit count that covers both the "refill" loop and its exit.
                c.bus.write32(DATA + 0x108, r.next_u32() % 0x50);
            },
        );
    }

    // -----------------------------------------------------------------------
    // RCMP_IsMpegChunkTag.

    pub(crate) fn chunk_tag_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3b11_f0, 0);
        let tags = 0x0049_5448u32;
        let a = r.next_u32();
        let b = r.next_u32();
        c.bus.write32(tags, a);
        c.bus.write32(tags + 4, b);
        c.g[A0][0] = match seed % 3 {
            0 => a as u64,
            1 => b as u64,
            _ => (a ^ 0x1000) as u64,
        };
    }

    driven!(gen_chunk_tag, ssx3_ee::s13::f_003b11f0__generated);

    driven!(hand_chunk_tag, rcmp_is_mpeg_chunk_tag);

    #[test]
    fn rcmp_is_mpeg_chunk_tag_matches() {
        check(
            ssx3_ee::s13::f_003b11f0__generated,
            rcmp_is_mpeg_chunk_tag,
            48,
            &chunk_tag_setup,
        );
        check_events(gen_chunk_tag, hand_chunk_tag, 48, &|c, seed| {
            chunk_tag_setup(c, seed);
            let mut r = Rng(0x3b11_f0 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x003b_11f0);
            // The table has two entries: make the (single) back edge due.
            c.next_event = c.cycles + 5;
            mode
        });
    }

    #[test]
    fn wscript_cache_set_data_matches() {
        check(
            ssx3_ee::s13::f_003acc50__generated,
            wscript_cache_set_data,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x3acc_50, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (if seed % 2 == 0 { 0 } else { DATA + 0x1000 }) as u64;
            },
        );
    }

    // ---------------------------------------------------------------- MPEG-2

    #[test]
    fn rcmp_decoder_set_codec_matches() {
        check(
            ssx3_ee::s13::f_003b04f8__generated,
            rcmp_decoder_set_codec,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x01, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = 0x1000 + seed as u64;
                // `*(codec)` is the codec's vtable; slot 0x10 is its v02.
                let vt = vtable(c, &mut r, &[0x10]);
                c.bus.write32(DATA + 0x2000, vt);
            },
        );
    }

    #[test]
    fn rcmp_decoder_get_frame_number_matches() {
        check(
            ssx3_ee::s13::f_003b0580__generated,
            rcmp_decoder_get_frame_number,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x02, 1);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    let vt = vtable(c, &mut r, &[0x20]);
                    c.bus.write32(DATA + 0x14, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2000, vt);
                } else {
                    c.bus.write32(DATA + 0x14, 0);
                }
            },
        );
    }

    #[test]
    fn rcmp_decoder_get_frame_rate_matches() {
        check(
            ssx3_ee::s13::f_003b05c0__generated,
            rcmp_decoder_get_frame_rate,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x03, 1);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    let vt = vtable(c, &mut r, &[0x28]);
                    c.bus.write32(DATA + 0x14, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2000, vt);
                } else {
                    c.bus.write32(DATA + 0x14, 0);
                }
            },
        );
    }

    #[test]
    fn mpeg2_get_bits_matches() {
        check(
            ssx3_ee::s13::f_003b1300__generated,
            mpeg2_get_bits,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x04, 1);
                c.g[A0][0] = sx(DATA + 0x3000);
                // The peek/consume pair runs on the (zeroed) reader state; give
                // the shift count a value that keeps it in range.
                c.bus.write32(DATA + 0x3000, r.next_u32() % 32);
            },
        );
    }

    #[test]
    fn av_player_release_chunk_matches() {
        check(
            ee::s13::f_003aece0__generated,
            av_player_release_chunk,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x3aece0, 0);
                let self_at = DATA + 0x200;
                let chunk = DATA + 0x300;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { chunk as u64 };
                c.bus.write32(self_at + 0x30, DATA + 0x500);
                c.bus.write32(chunk, (seed % 6) as u32);
                c.bus.write32(0x0050_9434, FAKE);
            },
        );
    }

    #[test]
    fn rcmp_decoder_release_codec_matches() {
        covered(
            ee::s13::f_003b0538__generated,
            rcmp_decoder_release_codec,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_0538, 1);
                let obj = DATA + 0x1000;
                let codec = DATA + 0x2000;
                if seed % 2 != 0 {
                    let vt = vtable(c, &mut r, &[8]);
                    c.bus.write32(codec, vt);
                    c.bus.write32(obj + 0x14, codec);
                }
                c.g[A0][0] = sx(obj);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn rcmp_decoder_get_chunk_matches() {
        covered(
            ee::s13::f_003b06b0__generated,
            rcmp_decoder_get_chunk,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_06b0, 1);
                let obj = DATA + 0x1000;
                c.bus
                    .write32(obj, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(obj + 4, r.next_u32());
                c.bus.write32(obj + 8, FAKE);
                c.g[A0][0] = sx(obj);
            },
        );
    }

    #[test]
    fn rcmp_static_init_destroy_matches() {
        check(
            ee::s13::f_003b0770__generated,
            rcmp_static_init_destroy,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_0770, 0);
                c.g[A0][0] = (seed % 3) as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0xffff
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    #[test]
    fn ps2_sony_codec_internal_create_matches() {
        covered(
            ee::s13::f_003b11a0__generated,
            ps2_sony_codec_internal_create,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_11a0, 1);
                c.bus.write32(0x0050_9430, FAKE);
                c.bus.write32(0x0050_9438, r.next_u32());
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn mpeg2_extra_bit_information_matches() {
        check(
            ee::s13::f_003b2318__generated,
            mpeg2_extra_bit_information,
            20,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_2318, 0);
                // The global bit reader: an all-zero reservoir, so no extra bit is
                // ever present and the loop body (flush) is not entered.
                let reader = DATA + 0x9000;
                c.bus.write32(0x0050_a088, reader);
                c.bus.write32(reader, DATA + 0xa000);
                c.bus.write32(reader + 4, 0);
                c.bus.write32(reader + 8, 24);
                c.pc = 0x003b_2318;
                c.g[A0][0] = 0;
                let _ = r.next_u32();
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x3acc90 tWScriptCache_Free.

    #[test]
    fn t_w_script_cache_free_matches() {
        check(
            ee::s13::f_003acc90__generated,
            t_w_script_cache_free,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x3acc_90, 0);
                let entry = DATA + 0x100;
                c.g[A0][0] = sx(entry);
                if seed % 2 == 0 {
                    c.bus.write32(entry + 8, 0);
                } else {
                    c.bus.write32(entry + 8, DATA + 0x400);
                    c.bus.write16(entry + 2, 1); // borrowed: never freed
                }
                c.bus.write16(entry, 0xffff);
            },
        );
    }

    // ------------------------------------------------------------- AV player

    #[test]
    fn av_player_decode_frame_matches() {
        check(
            ee::s13::f_003ae6c8__generated,
            av_player_decode_frame,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x3ae6);
                let p = DATA;
                let stream = DATA + 0x200;
                c.bus.write32(p + 0x64, stream);
                c.bus.write32(p + 0x74, r.next_u32());
                c.bus.write32(p + 0x44, r.next_u32());
                c.bus.write32(stream + 0x1c, 1 + r.next_u32() % 4);
                fill(c, &mut r, stream, 0x40);
                c.g[A0][0] = sx(p);
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn av_player_ctor_2_matches() {
        check(
            ee::s13::f_003ade08__generated,
            av_player_ctor_2,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ade_08, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_2(c, this, 0x100);
                c.g[A0][0] = sx(this);
                for i in 1..6 {
                    c.g[[A1, A2, A3, T0, T1][i - 1]][0] = r.next_u32() as u64;
                }
            },
        );
    }

    /// The global "RCMP decoder delete" function pointer at 0x509434.
    pub(crate) const RCMP_DELETE: u32 = 0x0050_9434;

    #[test]
    fn rcmp_decoder_dtor_matches() {
        check(
            ssx3_ee::s13::f_003b04a0__generated,
            rcmp_decoder_dtor,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0xb04a0, 1);
                heap_init(c);
                c.bus.write32(RCMP_DELETE, FAKE);
                let this = alloc(c, 0x100);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    // ------------------------------------------------------------- codec

    #[test]
    fn ps2_sony_codec_internal_nodata_callback_matches() {
        // Keep the feeder on its short path: the chunk at `codec + 0x28` has
        // no bitstream data (`+0x4 == 0`), so it only fills the "no data"
        // words, releases the chunk and queues one DMA chain entry.
        check(
            ssx3_ee::s13::f_003b0b10__generated,
            ps2_sony_codec_internal_nodata_callback,
            60,
            &|c, seed| {
                let mut r = start(c, seed, 0xb0b10);
                let popup = DATA + 0x400; // arg0 -> the feeder's a1 (s4)
                let notif = DATA + 0x500; // arg1 -> the feeder's a2
                let codec = DATA + 0x600; // arg2 -> the feeder's a0 (s1)
                c.g[A0][0] = popup as u64;
                c.g[A1][0] = notif as u64;
                c.g[A2][0] = codec as u64;
                // The chunk holder and the chunk it returns.
                c.bus.write32(codec + 0x28, DATA + 0x700);
                c.bus.write32(DATA + 0x700, DATA + 0x800);
                c.bus.write32(DATA + 0x804, 0); // no bitstream data
                c.bus.write32(DATA + 0x70c, 0x003b_0720); // the chunk's free hook
                c.bus.write32(codec + 0x7c, DATA + 0x900); // "no data" fill target
                c.bus.write32(codec + 0x78, r.next_u32()); // DMA tag source
                c.bus.write32(codec + 0x14, r.next_u32() & 0xff); // the counter
                c.bus.write32(popup + 0x40, DATA + 0xa00); // the DMA chain base
                for i in 0..4 {
                    c.bus.write32(DATA + 0x900 + 4 * i, r.next_u32());
                    c.bus.write32(codec + 4 * i, r.next_u32());
                }
                c.bus.write32(DATA + 0x1280, r.next_u32());
                c.bus.write32(DATA + 0x1284, r.next_u32());
            },
        );
    }

    #[test]
    fn ps2_sony_codec_internal_v06_matches() {
        check(
            ssx3_ee::s13::f_003b0fa8__generated,
            ps2_sony_codec_internal_v06,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0xb0fa8);
                c.bus.write32(0x0050_9508, r.edge_f32());
            },
        );
    }

    #[test]
    fn ps2_sony_codec_internal_release_frame_matches() {
        check(
            ssx3_ee::s13::f_003b1140__generated,
            ps2_sony_codec_internal_release_frame,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0xb1140);
                let codec = DATA + 0x400;
                let item = DATA + 0x500;
                c.g[A0][0] = codec as u64;
                c.g[A1][0] = item as u64;
                c.bus
                    .write32(item + 0x10, if seed % 2 == 0 { 2 } else { 1 });
                c.bus.write32(item + 8, DATA + 0x600);
                c.bus.write32(item + 0xc, DATA + 0x640);
                c.bus.write32(codec + 0x20, DATA + 0x680);
                c.bus.write32(codec + 0x24, DATA + 0x700);
                for i in 0..4 {
                    c.bus.write32(DATA + 0x600 + 4 * i, r.next_u32());
                    c.bus.write32(DATA + 0x640 + 4 * i, r.next_u32());
                    c.bus.write32(DATA + 0x680 + 4 * i, r.next_u32());
                    c.bus.write32(DATA + 0x700 + 4 * i, r.next_u32());
                }
            },
        );
    }

    // ------------------------------------------------------- script engine

    #[test]
    fn wscript_cache_dtor_matches() {
        check(
            ee::s13::f_003aca70__generated,
            wscript_cache_dtor,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x003a_ca70, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 3) as u64;
                w(c, DATA + 8, if seed % 2 == 0 { 0 } else { DATA + 0x800 });
                h(c, DATA + 2, if seed % 3 == 0 { 1 } else { 0 });
            },
        );
    }

    #[test]
    fn ps2_sony_codec_internal_free_frame_matches() {
        check(
            ee::s13::f_003b08e0__generated,
            ps2_sony_codec_internal_free_frame,
            12,
            &|c, seed| {
                machine(c, seed, 0x003b_08e0, 2);
                be_library_2(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(DATA + 4, 0);
                c.bus.write32(0x0050_9434, FAKE);
            },
        );
    }

    #[test]
    fn mpeg2_flush_buffer32_matches() {
        check(
            ee::s13::f_003b1340__generated,
            mpeg2_flush_buffer32,
            120,
            &|c, seed| {
                let mut r = Rng(0x3b13_40e7 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(0x0050_a088, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, DATA + 0x1100);
                c.bus.write32(DATA + 0x1004, r.next_u32());
                c.bus.write32(DATA + 0x1008, 0x20 + (r.next_u32() % 0x40));
                for i in 0..8 {
                    c.bus.write8(DATA + 0x1100 + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn av_audio_dtor_matches() {
        check(
            ee::s13::f_003aee30__generated,
            av_audio_dtor,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x3aee30, 0);
                globals(c);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = if seed % 3 == 0 { 1 } else { 0 };
                w_2(c, OBJ_2 + 4, if seed % 2 == 0 { 0 } else { OBJ2 });
                w_2(c, OBJ_2 + 0xc, OBJ3);
                w_2(
                    c,
                    0x0050_9434,
                    if seed % 4 == 0 {
                        ZERO_STUB
                    } else {
                        fakes::FAKE
                    },
                );
            },
        );
    }

    #[test]
    fn rcmp_decoder_ctor_matches() {
        check(
            ee::s13::f_003b0430__generated,
            rcmp_decoder_ctor,
            6,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x003b_0430, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (DATA + 0x4000) as u64;
                for i in 0..4 {
                    w(c, DATA + 0x4000 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn mpeg2_flush_buffer_matches() {
        check(
            ee::s13::f_003b1290__generated,
            mpeg2_flush_buffer,
            120,
            &|c, seed| {
                let mut r = Rng(0x3b12_90e7 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                w(c, 0x0050_a088, DATA + 0x1000);
                w(c, DATA + 0x1000, DATA + 0x1100);
                w(c, DATA + 0x1004, r.next_u32());
                w(c, DATA + 0x1008, 0x20 + (r.next_u32() % 0x40));
                for i in 0..8 {
                    c.bus.write8(DATA + 0x1100 + i, r.next_u32() as u8);
                }
                c.g[A0][0] = (r.next_u32() % 0x30) as u64;
            },
        );
    }

    #[test]
    fn rcmp_decoder_decode_frame_matches() {
        check(
            ee::s13::f_003b0600__generated,
            rcmp_decoder_decode_frame,
            80,
            &|c, seed| {
                let _ = base_3(c, seed, 0x3b06, 2);
                let d = DATA;
                w(c, d + 0x14, if seed % 3 == 0 { 0 } else { DATA + 0x400 });
                w(c, d, (seed % 2) as u32);
                w(c, d + 4, seed);
                w(c, d + 8, if seed % 4 == 0 { 0 } else { RET0 });
                w(c, DATA + 0x400, DATA + 0x500);
                slot(c, DATA + 0x500, 0x18, FAKE);
                c.g[A0][0] = sx(d);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn ps2_sony_codec_internal_decode_frame_matches() {
        check(
            ee::s13::f_003b1050__generated,
            ps2_sony_codec_internal_decode_frame,
            40,
            &|c, seed| {
                let _ = base_3(c, seed, 0x3b10, 2);
                let codec = DATA;
                // `sceMpegIsEnd` reads `lw_2(lw_2(codec+0x40))`; GetPicture bumps
                // `codec+0x10`, so the loop ends after `limit` pictures.
                w(
                    c,
                    codec + 0x40,
                    if seed % 4 == 0 { 0 } else { DATA + 0x500 },
                );
                w(c, DATA + 0x500, if seed % 8 == 0 { 1 } else { 0 });
                w(c, codec + 0x10, 0);
                w(c, codec + 8, 0);
                w(c, codec + 0xc, 0);
                w(c, codec + 0x24, 0);
                c.g[A0][0] = sx(codec);
                // Keep the loop short: at most two decoded pictures.
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    // -----------------------------------------------------------------------
    // World and game states.

    #[test]
    fn world_reset_lights_matches() {
        check(
            ee::s13::f_003ad1a8__generated,
            world_reset_lights,
            SEEDS_8,
            &|c, seed| {
                base_4(c, seed, 0xd1a8, 0);
                let race = DATA + 0x9400;
                let lights = DATA + 0x800;
                let arr = DATA + 0x880;
                c.bus.write32(race + 0x90, lights);
                c.bus
                    .write32(race + 0xc, if seed % 5 == 0 { 0 } else { DATA + 0x900 });
                c.bus.write32(DATA + 0x900 + 0x78, (seed % 4) as u32);
                c.bus.write32(DATA + 0x900 + 0x28, arr);
                for i in 0..4u32 {
                    c.bus.write32(arr + 4 * i, DATA + 0x1000 + 0x40 * i);
                }
            },
        );
    }

    #[test]
    fn ps2_sony_codec_internal_get_picture_matches() {
        check_lohi(
            ee::s13::f_003b0fb8__generated,
            ps2_sony_codec_internal_get_picture,
            30,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x3b0fb8, 3);
                let pool_4 = DATA + 0x1000;
                c.g[A0][0] = pool_4 as u64;
                w(
                    c,
                    pool_4 + 0x8,
                    if seed % 3 == 0 {
                        0
                    } else {
                        16 * (1 + seed % 4)
                    },
                );
                w(
                    c,
                    pool_4 + 0xc,
                    if seed % 3 == 0 {
                        0
                    } else {
                        16 * (1 + seed % 4)
                    },
                );
                w(c, pool_4 + 0x10, r.next_u32());
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn mpeg2_picture_spatial_scalable_extension_matches() {
        check(
            ee::s13::f_003b2230__generated,
            mpeg2_picture_spatial_scalable_extension,
            SEEDS_9,
            &|c, seed| {
                let _ = machine(c, seed, 0x3b22, 0);
                game_globals(c);
            },
        );
    }

    #[test]
    fn cwscriptcache_alloc_matches() {
        strict(
            ee::s13::f_003ac800__generated,
            cwscriptcache_alloc,
            SEEDS_10,
            &|c, seed| {
                let mut r = base_6(c, seed, 0xc800, 0);
                let this = DATA + 0x100;
                let count = 1 + (seed % 8);
                if seed % 2 == 0 {
                    w(c, this + 4, 0);
                } else {
                    // an existing array of two zeroed entries behind a header
                    let mem = DATA + 0x2000;
                    w(c, mem, 2);
                    w(c, this + 4, mem + 0x10);
                    for k in 0..2u32 {
                        w(c, mem + 0x10 + 0xc * k, 0);
                        w(c, mem + 0x14 + 0xc * k, 0);
                        w(c, mem + 0x18 + 0xc * k, 0);
                    }
                }
                w(c, this + 8, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = count as u64;
            },
        );
    }

    #[test]
    fn av_audio_init_matches() {
        // Only the "stream module not up yet" path: the ready path opens a
        // stream through the IOP, which a test machine cannot do.
        strict(
            ee::s13::f_003aed40__generated,
            av_audio_init,
            SEEDS_10,
            &|c, seed| {
                let r = base_6(c, seed, 0xed40, 0);
                let _ = r;
                let this = DATA + 0x100;
                c.bus.write8(0x0050_aa5c, 0);
                w(c, 0x0045_0da4, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x200);
                c.g[A2][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn mpeg2_picture_display_extension_matches() {
        strict(
            ee::s13::f_003b1fc8__generated,
            mpeg2_picture_display_extension,
            SEEDS_10,
            &|c, seed| {
                let _r = base_6(c, seed, 0x1fc8, 0);
                // no decoder state: one picture, three fields, two marker bits
                w(c, 0x0050_9524, 0);
                w(c, 0x0050_9584, if seed % 3 == 0 { 3 } else { 0 });
                let st = DATA + 0x3000;
                w(c, 0x0050_9588, st);
                w(c, st, DATA + 0x4000);
                w(c, st + 4, 0);
                w(c, st + 8, 0);
                c.g[A0][0] = sx(DATA + 0x500);
            },
        );
    }

    #[test]
    fn shp_calc_alloc_size_matches() {
        check_all(
            ee::s13::f_003b26b8__generated,
            shp_calc_alloc_size,
            300,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x3b_26b8);
                let w = 1 + (r.next_u32() % 64);
                let h = 1 + (r.next_u32() % 64);
                let bpp = match seed % 4 {
                    0 => r.next_u32() % 8,
                    1 => 8 + r.next_u32() % 32,
                    2 => 4,
                    _ => 32,
                };
                let flags = match seed % 3 {
                    0 => 0,
                    1 => 0xf,
                    _ => r.next_u32() & 0xf0,
                };
                c.g[A0][0] = w as u64;
                c.g[A1][0] = h as u64;
                c.g[A2][0] = bpp as u64;
                c.g[A3][0] = flags as u64;
                c.g[T0][0] = (r.next_u32() % 0x40) as u64;
                c.g[T1][0] = if seed % 4 == 0 {
                    0
                } else {
                    (r.next_u32() % 0x40) as u64
                };
                c.g[T2][0] = sx(DATA);
                floats(c, &mut r, DATA, 4);
            },
        );
    }

    #[test]
    fn ps2_sony_codec_internal_feed_bitstream_matches() {
        check_all_2(
            ee::s13::f_003b0b40,
            ps2_sony_codec_internal_feed_bitstream,
            60,
            &|c, seed| {
                machine(c, seed, 0x3b0b, 0);
                let decoder = DATA + 0x2000;
                let stream = DATA + 0x3000;
                c.g[A0][0] = decoder as u64;
                c.g[A1][0] = stream as u64;
                c.bus.write32(decoder + 0x7c, DATA + 0x4000);
                c.bus.write32(decoder + 0x78, DATA + 0x5000);
                c.bus.write32(decoder + 0x28, DATA + 0x6000);
                if seed % 2 == 0 {
                    // No chunk pending.
                    c.bus.write32(decoder, 0);
                    c.bus.write32(decoder + 8, 0);
                } else {
                    // A chunk of 0x20 bytes at ring offset 0x20.
                    let chunk = DATA + 0x7000;
                    c.bus.write32(decoder, chunk);
                    c.bus.write32(chunk + 4, 0x20);
                    c.bus.write32(chunk + 8, 0x28);
                }
                c.bus.write32(DATA + 0x6000, 0);
            },
        );
    }

    #[test]
    fn mpeg2_sequence_extension_matches() {
        check_all_3(
            ee::s13::f_003b1ab0__generated,
            mpeg2_sequence_extension,
            20,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x3b1a_b0, 8);
                // The bit reader state: an 8-byte buffer with a few extension bits.
                c.bus.write32(0x0050_a088, DATA + 0x1_0000);
                put_words(c, DATA + 0x1_0000, &[r.next_u32(), r.next_u32()]);
                c.bus.write32(0x0050_9510, 1);
                c.bus.write32(DATA + 0x1_0000 + 0x420, 0);
                put_words(c, DATA + 0x1_0000 + 0x424, &[r.next_u32()]);
            },
        );
    }

    // ------------------------------------------------ tWScriptCache_GetObjectSlot

    /// `tWScriptCache_GetObjectSlot`: no script, no table, id -1, and real
    /// lookups of slot `i` in row `id >> 8`.
    #[test]
    fn wscript_cache_get_object_slot_matches() {
        check(
            ssx3_ee::s13::f_003ad120__generated,
            wscript_cache_get_object_slot,
            300,
            &|c, seed| {
                let mut r = Rng(0x3ad1_2000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003a_d120;
                let obj = DATA;
                let entry = DATA + 0x100;
                let script = DATA + 0x200;
                let table = DATA + 0x300;
                let rows = DATA + 0x1000;
                c.g[A0][0] = sx(entry);
                c.g[A1][0] = sx(obj);
                let i = r.next_u32() % 6;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | i as u64;
                c.bus
                    .write32(obj + 0x88, if seed % 5 == 0 { 0 } else { script });
                c.bus
                    .write32(entry + 0x8, if seed % 5 == 1 { 0 } else { table });
                let row = r.next_u32() % 8;
                let id = if seed % 5 == 2 {
                    0xffff_ffff
                } else {
                    row << 8 | (r.next_u32() & 0xff)
                };
                c.bus.write32(script + 0x8, id);
                c.bus.write32(table + 0x1c, rows);
                c.bus.write32(rows + row * 0x18 + i * 4, r.next_u32());
            },
        );
    }

    pub(crate) use crate::game::ai::ai::tests::OC420_CTRL;
    pub(crate) use crate::game::ai::ai::tests::OC420_MAGIC;

    /// The hook forwards to the flare-size lookup for assorted resolutions.
    #[test]
    fn world_chunk_resolve_hook_matches() {
        check(
            ee::s13::f_003ad230__generated,
            world_chunk_resolve_hook,
            70,
            &|c, seed| {
                machine(c, seed, 0x3a_d230, 0);
                c.g[A0][0] = [0x20u64, 0x10, 0x40, 0x30, 33, 0, u64::MAX][(seed % 7) as usize];
                c.pc = 0x003a_d230;
            },
        );
    }

    // ------------------------------------------------ tWScriptCache_Relocate

    /// A script block at `DATA + 0x1_0000`: header `0x1000` and world id,
    /// ten table offsets, 0..3 entries in each table (see the layout below),
    /// the world cache with three slot records (one empty) and objects that
    /// the records resolve to.
    pub(crate) fn oc420_relocate_setup(c: &mut Cpu, r: &mut Rng, seed: u32, kind: u32) {
        const D: u32 = DATA + 0x1_0000;
        let (entry, wp, cache, ymeta, slots) = (
            DATA + 0x1000,
            DATA + 0x1100,
            DATA + 0x1200,
            DATA + 0x1210,
            DATA + 0x1300,
        );
        let wid = 0x1234_5678u32;
        c.bus
            .write32(entry + 8, if seed % 16 == 15 && kind == 3 { 0 } else { D });
        c.bus.write16(entry + 2, if seed % 8 < 6 { 0 } else { 1 });
        c.bus.write32(wp, cache);
        c.bus.write32(cache, ymeta);
        c.bus.write32(ymeta + 4, wid);
        c.bus.write32(cache + 8, slots);
        c.bus.write16(D, if kind == 0 { 0x1001 } else { 0x1000 });
        c.bus
            .write32(D + 0xc, if kind == 1 { wid ^ 1 } else { wid });
        // Table offsets.
        for (off, v) in [
            (0x14u32, 0x400u32),
            (0x1c, 0x500),
            (0x24, 0x600),
            (0x2c, 0x800),
            (0x34, 0x900),
            (0x3c, 0xa00),
            (0x40, 0xb00),
            (0x48, 0xc00),
            (0x50, 0xd00),
            (0x58, 0xe00),
        ] {
            c.bus.write32(D + off, v);
        }
        let n4c = r.next_u32() % 4;
        let n44 = r.next_u32() % 4;
        c.bus.write32(D + 0x20, r.next_u32() % 3);
        c.bus.write32(D + 0x28, r.next_u32() % 3);
        c.bus.write32(D + 0x38, r.next_u32() % 4);
        c.bus.write32(D + 0x44, n44);
        c.bus.write32(D + 0x4c, n4c);
        c.bus.write32(D + 0x54, r.next_u32() % 3);
        // +0x3c: word offsets; +0x600/+0x800: records (field +8 an offset).
        for i in 0..4u32 {
            c.bus.write32(D + 0xa00 + 4 * i, 0x20 * i);
            c.bus
                .write16(D + 0xc00 + 2 * i, (r.next_u32() % n4c.max(1)) as u16);
            // +0x50 table: offsets from the +0x40 base to the entries.
            c.bus.write32(D + 0xd00 + 4 * i, 0x100 * i);
            let e = D + 0xb00 + 0x100 * i;
            c.bus
                .write32(e, [0u32, 1, 3, 2][(r.next_u32() % 4) as usize]);
            let slot = r.next_u32() % 4;
            c.bus.write32(e + 0xc, slot | (r.next_u32() % 4) << 8);
            c.bus
                .write32(D + 0x600 + 0x54 * i + 8, r.next_u32() % 0x100);
            c.bus
                .write32(D + 0x800 + 0x40 * i + 8, r.next_u32() % 0x100);
            c.bus.write32(e + 4, r.next_u32());
            // +0x58: two halfwords per spline.
            c.bus.write32(D + 0xe00 + 4 * i, r.next_u32());
        }
        // World cache records.
        for s in 0..3u32 {
            let rec = DATA + 0x1400 + 0x80 * s;
            c.bus.write32(slots + 4 * s, if s == 2 { 0 } else { rec });
            for (off, base) in [(0x1cu32, 0x1800u32), (0x44, 0x1900), (0x64, 0x1a00)] {
                let table = DATA + base + 0x20 * s;
                c.bus.write32(rec + off, table);
                for k in 0..4u32 {
                    let obj = DATA + 0x2000 + 0x100 * ((s + k) % 4);
                    let word = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        (obj >> 2) << 8
                    };
                    c.bus.write32(table + 4 * k, word);
                }
            }
        }
        c.g[A0][0] = sx(entry);
        c.g[A1][0] = sx(wp);
        c.g[A2][0] = [1u64, 0, 2, 5][(r.next_u32() % 4) as usize];
    }

    #[test]
    fn t_wscript_cache_relocate_matches() {
        use crate::difftest::events::{check_events, schedule};
        oc420_hook_mem_free();
        covered(
            ee::s13::f_003accd8__generated,
            t_wscript_cache_relocate,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_ccd8, 1);
                c.pc = 0x003a_ccd8;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                oc420_relocate_setup(c, &mut r, seed, seed % 4);
            },
        );
        check_events(
            ee::s13::f_003accd8__generated,
            t_wscript_cache_relocate,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_ccd9, 0);
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                oc420_relocate_setup(c, &mut r, seed, 2);
                schedule(c, &mut r, seed, 0x003a_ccd8)
            },
        );
    }

    pub(crate) use crate::game::ai::ai::tests::oc434_setup;
    pub(crate) use crate::game::ai::ai::tests::oc434_w;

    pub(crate) use crate::game::world::world::tests::w_2 as w_3;
    pub(crate) use crate::game::world::world::tests::BASE_2 as BASE;
    pub(crate) use crate::game::world::world::tests::SEEDS_4 as SEEDS_11;

    /// `tWScriptCache_GetGlobals`: a leaf; type 3 with and without data, other types.
    #[test]
    fn w_script_cache_get_globals_matches() {
        check(
            ee::s13::f_003adcb0__generated,
            w_script_cache_get_globals,
            24,
            &|c, seed| {
                let mut r = oc434_setup(c, seed, 0x3adcb0, 0, 0x003a_dcb0);
                let (entry, data) = (DATA + 0x100, DATA + 0x200);
                c.g[A0][0] = entry as u64;
                c.bus.write16(entry, if seed % 3 == 0 { 2 } else { 3 });
                oc434_w(c, entry + 8, if seed % 4 == 1 { 0 } else { data });
                oc434_w(c, data + 0x14, r.next_u32());
            },
        );
    }

    /// `tWScriptCache_GetGlobalCount`: kind 3 with data returns `data+0x10`;
    /// other kinds (also a sign-extended halfword) and missing data return 0.
    #[test]
    fn t_w_script_cache_get_global_count_matches() {
        check(
            ee::s13::f_003adc80__generated,
            t_w_script_cache_get_global_count,
            SEEDS_11,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3adc80, 0);
                let cache = BASE + 0x100;
                let data = BASE + 0x200;
                c.g[A0][0] = cache as u64;
                c.bus
                    .write16(cache, [3, 3, 3, 1, 0x8003][seed as usize % 5]);
                w_3(c, cache + 8, if seed % 3 == 0 { 0 } else { data });
                w_3(c, data + 0x10, r.next_u32());
            },
        );
    }

    pub(crate) use crate::game::world::worldview::tests::machine_2;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_8 as SEEDS_12;

    /// Every exit and the full table path: a1 != 0xFFFF exits early, a0 == 0
    /// exits after the prologue, the rest run the table stores.
    #[test]
    fn world_static_init_destroy_matches() {
        check(
            ee::s13::f_003ad290__generated,
            world_static_init_destroy,
            200,
            &|c, seed| {
                let mut r = Rng(0x3ad2_9000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                // The float pool the body loads from (gp-0x23ac and up).
                fill(c, &mut r, GAME_GP - 0x2400, 0x400);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 3 == 0 { 0x1234 } else { 0xffff };
                if seed % 7 == 0 {
                    c.g[A0][0] = 0;
                }
            },
        );
    }

    #[test]
    fn cwscript_cache_dtor_matches() {
        check(
            ee::s13::f_003ac8f0__generated,
            cwscript_cache_dtor,
            SEEDS_12,
            &|c, seed| {
                let mut r = machine_2(c, seed, 0x3ac8, 0x003a_c8f0);
                let cache = DATA + 0x100;
                // No array (the free paths are skipped) for the seeds whose
                // array would need the real allocator; the flag bit is set on
                // odd seeds.
                c.bus.write32(cache + 0x4, 0);
                c.bus.write32(cache + 0x8, r.next_u32());
                c.g[A0][0] = sx(cache);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn cwscript_cache_update_matches() {
        check(
            ee::s13::f_003ac980__generated,
            cwscript_cache_update,
            SEEDS_12,
            &|c, seed| {
                machine_2(c, seed, 0x3ac9, 0x003a_c980);
                let cache = DATA + 0x100;
                let entries = DATA + 0x200;
                c.bus.write32(cache + 0x4, entries);
                let count = seed % 4;
                c.bus.write32(cache + 0x8, count);
                for i in 0..count {
                    // state 0: the poll returns at once
                    c.bus.write16(entries + 0xc * i, 0);
                }
                c.g[A0][0] = sx(cache);
            },
        );
    }

    #[test]
    fn cwscript_cache_load_matches() {
        check(
            ee::s13::f_003ac9e0__generated,
            cwscript_cache_load,
            SEEDS_12,
            &|c, seed| {
                machine_2(c, seed, 0x3ac9, 0x003a_c9e0);
                let cache = DATA + 0x100;
                c.bus.write32(cache + 0x4, DATA + 0x200);
                c.g[A0][0] = sx(cache);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
            },
        );
    }

    #[test]
    fn twscript_cache_poll_load_matches() {
        check(
            ee::s13::f_003acad0__generated,
            twscript_cache_poll_load,
            SEEDS_12,
            &|c, seed| {
                machine_2(c, seed, 0x3aca, 0x003a_cad0);
                let entry = DATA + 0x100;
                // Entry states 0..=2: the generated code only works on state 1.
                c.bus.write16(entry, (seed % 3) as u16);
                c.bus.write32(entry + 4, DATA + 0x200);
                c.g[A0][0] = sx(entry);
            },
        );
    }

    #[test]
    fn twscript_cache_load_matches() {
        check(
            ee::s13::f_003acbb8__generated,
            twscript_cache_load,
            SEEDS_12,
            &|c, seed| {
                machine_2(c, seed, 0x3acb, 0x003a_cbb8);
                let entry = DATA + 0x100;
                c.g[A0][0] = sx(entry);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
            },
        );
    }

    #[test]
    fn world_return_self_matches() {
        check(
            ee::s13::f_003ad250__generated,
            world_return_self,
            SEEDS_12,
            &|c, seed| {
                machine_2(c, seed, 0x3ad2, 0x003a_d250);
            },
        );
    }

    #[test]
    fn world_global_ctors_matches() {
        check(
            ee::s13::f_003adda0__generated,
            world_global_ctors,
            SEEDS_12,
            &|c, seed| {
                machine_2(c, seed, 0x3add, 0x003a_dda0);
            },
        );
    }

    use crate::difftest::check_paths;
    use crate::difftest::check_paths_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::events::Events;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// A machine for the SHP tests: scrambled registers, the game's gp and
    /// its image (the bits-per-pixel table `SHP_GetBpp` reads), `pc` at the
    /// function's entry.
    pub(crate) fn shp_machine(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        let mut r = Rng(0x5e1f_7a3b_0c4d_9e21 ^ (seed as u64) << 12 ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        elf_image(c);
        c.pc = entry;
        r
    }

    /// Gate of the counted unwinds: magic, the index of the counted call
    /// that unwinds, the calls counted so far. Every fake below (and the
    /// cost fakes) counts its calls while the gate is set, so a seed can
    /// make the k-th counted call of a run unwind; otherwise they run the
    /// real body.
    pub(crate) const SHP_UNWIND_GATE: u32 = DATA + 0xf_eb00;

    pub(crate) const SHP_UNWIND_MAGIC: u32 = 0x5348_5057;

    /// Count a call; true (with `pc` at an unwind address) when it is the
    /// one the gate names.
    pub(crate) fn shp_unwind_at_count(c: &mut Cpu) -> bool {
        if c.bus.read32(SHP_UNWIND_GATE) != SHP_UNWIND_MAGIC {
            return false;
        }
        let n = c.bus.read32(SHP_UNWIND_GATE + 8);
        c.bus.write32(SHP_UNWIND_GATE + 8, n + 1);
        if n != c.bus.read32(SHP_UNWIND_GATE + 4) {
            return false;
        }
        c.pc = 0xdead_0000 | (n & 0xfff) << 4;
        true
    }

    /// A counting fake for a callee slot: the real body unless the count
    /// says unwind.
    macro_rules! shp_counting_fake {
        ($fake:ident, $prev:ident, $generated:path) => {
            pub(crate) static $prev: AtomicUsize = AtomicUsize::new(0);
            pub(crate) fn $fake(c: &mut Cpu) {
                if shp_unwind_at_count(c) {
                    return;
                }
                fakes::fall_back(c, &$prev, $generated)
            }
        };
    }

    shp_counting_fake!(
        shp_count_format_from_bpp,
        SHP_COUNT_PREV_FORMAT_FROM_BPP,
        ee::s13::f_003b3da8__generated
    );

    shp_counting_fake!(
        shp_count_get_bpp,
        SHP_COUNT_PREV_GET_BPP,
        ee::s13::f_003b39d8__generated
    );

    shp_counting_fake!(
        shp_count_palette_entries,
        SHP_COUNT_PREV_PALETTE_ENTRIES,
        ee::s13::f_003b2688__generated
    );

    shp_counting_fake!(
        shp_count_get_entry_name,
        SHP_COUNT_PREV_GET_ENTRY_NAME,
        ee::s13::f_003b3d00__generated
    );

    shp_counting_fake!(
        shp_count_psmt4,
        SHP_COUNT_PREV_PSMT4,
        ee::s13::f_003b42b0__generated
    );

    shp_counting_fake!(
        shp_count_psmt8,
        SHP_COUNT_PREV_PSMT8,
        ee::s13::f_003b4338__generated
    );

    shp_counting_fake!(
        shp_count_psmct16,
        SHP_COUNT_PREV_PSMCT16,
        ee::s13::f_003b43c0__generated
    );

    shp_counting_fake!(
        shp_count_psmct32,
        SHP_COUNT_PREV_PSMCT32,
        ee::s13::f_003b4448__generated
    );

    /// Hook every counting fake (and the cost fakes, which count too) and
    /// make the `target`-th counted call unwind.
    pub(crate) fn shp_unwind_setup(c: &mut Cpu, target: u32) {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B3DA8),
            shp_count_format_from_bpp,
            &SHP_COUNT_PREV_FORMAT_FROM_BPP,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B39D8),
            shp_count_get_bpp,
            &SHP_COUNT_PREV_GET_BPP,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B2688),
            shp_count_palette_entries,
            &SHP_COUNT_PREV_PALETTE_ENTRIES,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B3D00),
            shp_count_get_entry_name,
            &SHP_COUNT_PREV_GET_ENTRY_NAME,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B42B0),
            shp_count_psmt4,
            &SHP_COUNT_PREV_PSMT4,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B4338),
            shp_count_psmt8,
            &SHP_COUNT_PREV_PSMT8,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B43C0),
            shp_count_psmct16,
            &SHP_COUNT_PREV_PSMCT16,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B4448),
            shp_count_psmct32,
            &SHP_COUNT_PREV_PSMCT32,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6448),
            shp_init_header_fake_mem_set,
            &SHP_INIT_HEADER_PREV_MEM_SET,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            shp_fake_sprintf_alloc,
            &SHP_ALLOC_PREV_SPRINTF,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6574),
            shp_fake_mem_copy,
            &SHP_COPY_PREV_MEM_COPY,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B2E78),
            shp_decode_dxt_fake_color_block,
            &SHP_DECODE_DXT_PREV_COLOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004165A8),
            shps_fake_strcmp,
            &SHPS_PREV_STRCMP,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416810),
            shp_block_size_fake_strlen,
            &SHP_BLOCK_SIZE_PREV_STRLEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B47F8),
            shp_fake_eac_unpack,
            &SHP_UNPACK_PREV_EAC,
        );
        put_words(c, SHP_UNWIND_GATE, &[SHP_UNWIND_MAGIC, target, 0]);
    }

    /// Seeds past a test's `base` seeds are unwind seeds: fixture seed
    /// `fixtures[s % len]`, unwinding at counted call `s / len` (`s` the
    /// seed less `base`). Returns the fixture seed and the target.
    pub(crate) fn shp_unwind_seed(seed: u32, base: u32, fixtures: &[u32]) -> (u32, Option<u32>) {
        if seed < base {
            return (seed, None);
        }
        let s = seed - base;
        let n = fixtures.len() as u32;
        (fixtures[(s % n) as usize], Some(s / n))
    }

    /// Bit depths the SHP code knows (4, 8, 15/16, 32, 0 meaning 32) and
    /// one it does not (24).
    pub(crate) const SHP_TEST_DEPTHS: [u64; 7] = [0, 4, 8, 15, 16, 24, 32];

    /// Every known and unknown depth, and random ones (one path: the
    /// totals differ only by `SHP_FormatFromBpp`'s branch for the depth).
    #[test]
    fn shp_normalize_bpp_matches() {
        // Seeds 160..163 unwind at FormatFromBpp or GetBpp (4 and 8 bpp).
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let (seed, unwind) = shp_unwind_seed(seed, 160, &[1, 2]);
            let mut r = shp_machine(c, seed, 0x2528, 0x003b_2528);
            c.g[A0][0] = match seed % 8 {
                7 => r.next_u32() as u64,
                k => SHP_TEST_DEPTHS[k as usize],
            };
            if let Some(k) = unwind {
                shp_unwind_setup(c, k);
            }
        };
        // 16/15: 29, 4: 31, 0/24/32: 32, 8: 33.
        check_paths(
            ee::s13::f_003b2528__generated,
            shp_normalize_bpp,
            164,
            &[29, 31, 32, 33],
            setup,
        );
    }

    /// Size bits (sizes up to `1 << bits`) and max levels of the mip loop.
    pub(crate) const CALC_IMAGE_SIZE_BITS: [u32; 6] = [0, 1, 2, 4, 9, 16];

    pub(crate) const CALC_IMAGE_SIZE_LEVELS: [i32; 5] = [-1, 0, 1, 3, 17];

    /// Seeds that give every mip-loop sample `u` below 1260 (three per 8).
    pub(crate) const CALC_IMAGE_SIZE_SEEDS: u32 = 1260 / 3 * 8;

    /// `SHP_CalcImageSize` call state: the rejects (`seed % 8` 0-4: bad
    /// width, no height, too tall, too many pixels, unknown depth; depth
    /// `seed / 8 % 7`, the variant `seed / 56 % 2` or `% 4`: for the size
    /// rejects one of them is the limit itself, which passes) and the mip
    /// loop (5-7), numbering its seeds `u` = 0, 1, 2, ...: depth `u % 7`,
    /// width and height up to `1 << bits` with bits `u / 7 % 6` and
    /// `u / 42 % 6`, max level `u / 252 % 5`.
    pub(crate) fn calc_image_size_call(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = shp_machine(c, seed, 0x2558, CALC_IMAGE_SIZE_ENTRY);
        let depth = SHP_TEST_DEPTHS[(seed / 8 % 7) as usize];
        let mut max_level = -1;
        let (w, h, depth): (u32, u32, u64) = match seed % 8 {
            0 => ([0, 0x10001, 0, 0x10000][(seed / 56 % 4) as usize], 4, depth),
            1 => (8, if seed / 56 % 2 == 0 { 0 } else { 0xffff_fff0 }, depth),
            2 => (8, if seed / 56 % 2 == 0 { 0x10001 } else { 0x10000 }, depth),
            // 31250 x 32000 is exactly the most pixels.
            3 if seed / 56 % 2 == 0 => (0x8000, 0x8000, depth),
            3 => (31250, 32000, depth),
            4 => (8, 8, 24),
            k => {
                let u = seed / 8 * 3 + (k - 5);
                let depth = SHP_TEST_DEPTHS[(u % 7) as usize];
                let w_bits = CALC_IMAGE_SIZE_BITS[(u / 7 % 6) as usize];
                let h_bits = CALC_IMAGE_SIZE_BITS[(u / 42 % 6) as usize];
                max_level = CALC_IMAGE_SIZE_LEVELS[(u / 252 % 5) as usize];
                (
                    1 + r.next_u32() % (1 << w_bits),
                    1 + r.next_u32() % (1 << h_bits),
                    if depth == 24 { 4 } else { depth },
                )
            }
        };
        c.g[A0][0] = w as u64;
        c.g[A1][0] = sx(h);
        c.g[A2][0] = depth;
        c.g[A3][0] = max_level as i64 as u64;
        r
    }

    /// Paths claimed (4 bpp, NormalizeBpp 31, FormatFromBpp 13): bad width
    /// 62, no height 64, too tall 65, too many pixels 71, unknown depth (24,
    /// 32 + 14) 92; the fixed shapes of `calc_image_size_fixed`: maxLevel -1
    /// 96, 1x1 114, 2x2 with maxLevel 0 116, 2x1 136, 1x2 137.
    #[test]
    fn shp_calc_image_size_matches() {
        // The last three seeds unwind at each counted call of a 4x4 image
        // (NormalizeBpp's FormatFromBpp and GetBpp, then FormatFromBpp).
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let (seed, unwind) = shp_unwind_seed(seed, CALC_IMAGE_SIZE_SEEDS, &[4]);
            calc_image_size_call(c, seed);
            calc_image_size_fixed(c, seed);
            if let Some(k) = unwind {
                shp_unwind_setup(c, k);
            }
        };
        check_paths_lohi(
            ee::s13::f_003b2558__generated,
            shp_calc_image_size,
            CALC_IMAGE_SIZE_SEEDS + 3,
            &[62, 64, 65, 71, 92, 96, 114, 116, 136, 137],
            setup,
        );
    }

    /// For the unknown-depth seeds of depth digit 0 (`seed % 56 == 4`;
    /// that reject ignores the digit, so they repeat the others): a 4 bpp
    /// image of a fixed shape picked by `seed / 56 % 5`.
    pub(crate) fn calc_image_size_fixed(c: &mut Cpu, seed: u32) {
        if seed % 56 != 4 {
            return;
        }
        let (w, h, max_level): (u64, u64, u64) = match seed / 56 % 5 {
            0 => (4, 4, u64::MAX),
            1 => (1, 1, 5),
            2 => (2, 2, 0),
            3 => (2, 1, 5),
            _ => (1, 2, 5),
        };
        c.g[A0][0] = w;
        c.g[A1][0] = h;
        c.g[A2][0] = 4;
        c.g[A3][0] = max_level;
    }

    driven!(gen_shp_calc_image_size, ee::s13::f_003b2558__generated);

    driven!(hand_shp_calc_image_size, shp_calc_image_size);

    /// The mip loop with live events: the first back edge is past the event
    /// deadline, so unwinds land on the next-level head (images wider than
    /// one pixel) and on the level-test head (1-pixel-wide, taller images).
    #[test]
    fn shp_calc_image_size_events() {
        check_events(
            gen_shp_calc_image_size,
            hand_shp_calc_image_size,
            300,
            &|c, seed| {
                let mut r = calc_image_size_call(c, seed / 3 * 8 + 5 + seed / 3 % 3);
                if seed / 9 % 2 == 0 {
                    c.g[A0][0] = 1;
                    c.g[A1][0] = 0x40 + (r.next_u32() % 0x400) as u64;
                }
                c.g[A3][0] = 17;
                schedule(c, &mut r, seed, CALC_IMAGE_SIZE_ENTRY)
            },
        );
    }

    /// Where the SHP tests build images (16-byte aligned, inside the
    /// filled area).
    pub(crate) const SHP_TEST_IMAGE: u32 = DATA + 0x1230;

    /// Gate of the `EA_MemSet` fake below: magic, the image (whose header
    /// clear stays real), the palette data offset the fake leaves.
    pub(crate) const SHP_INIT_HEADER_GATE: u32 = DATA + 0xf_e400;

    pub(crate) const SHP_INIT_HEADER_MAGIC: u32 = 0x5348_5049;

    pub(crate) static SHP_INIT_HEADER_PREV_MEM_SET: AtomicUsize = AtomicUsize::new(0);

    /// `EA_MemSet` for `SHP_InitHeader`'s palette tests: with the gate set,
    /// a 16-byte clear of any block but the image instead leaves a header
    /// whose data is at the offset stored at gate+8 (flags bit 12), so the
    /// palette fill takes its "data elsewhere" arm.
    pub(crate) fn shp_init_header_fake_mem_set(c: &mut Cpu) {
        if shp_unwind_at_count(c) {
            return;
        }
        let gate = SHP_INIT_HEADER_GATE;
        if c.bus.read32(gate) != SHP_INIT_HEADER_MAGIC
            || c.g[A2][0] != SHP_HEADER_SIZE
            || r32(c, A0) == c.bus.read32(gate + 4)
        {
            return fakes::fall_back(
                c,
                &SHP_INIT_HEADER_PREV_MEM_SET,
                ee::s14::f_003e6448__generated,
            );
        }
        let block = r32(c, A0);
        let offset = c.bus.read32(gate + 8);
        put_words(c, block, &[0, 0, 0, SHP_FLAG_DATA_AT_OFFSET as u32, offset]);
        c.cycles += 4;
        c.pc = c.g[RA][0] as u32;
    }

    /// One `SHP_InitHeader` call: (w, h, bpp, palette bpp, maxLevel,
    /// extra1, extra2, palette header kept by the fake).
    pub(crate) type InitHeaderCall = (u32, u32, u32, u32, u32, u32, u32, bool);

    /// The shapes whose totals `shp_init_header_matches` claims.
    pub(crate) const INIT_HEADER_FIXED: [InitHeaderCall; 10] = [
        (1, 1, 32, 0, 0, 0, 0, false),
        (1, 1, 4, 0, 0, 0, 0, false),
        (1, 1, 4, 16, 0, 0, 0, false),
        (1, 1, 4, 32, 0, 0, 0, false),
        (1, 1, 2, 32, 0, 0, 0, false),
        (4, 1, 32, 0, 0, 0x18, 0x20, false),
        (1, 1, 4, 16, 0, 0, 0, true),
        (1, 1, 4, 32, 0, 0, 0, true),
        (1, 1, 4, 0xffff_fff8, 0, 0, 0, false),
        (1, 1, 4, 16, 0, 0x18, 0x20, false),
    ];

    /// Machine, filled image area and fake for an `SHP_InitHeader` call.
    pub(crate) fn init_header_setup(c: &mut Cpu, seed: u32, call: InitHeaderCall) -> Rng {
        let mut r = shp_machine(c, seed, 0x27c8, 0x003b_27c8);
        fill(c, &mut r, DATA + 0x1000, 0x800);
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6448),
            shp_init_header_fake_mem_set,
            &SHP_INIT_HEADER_PREV_MEM_SET,
        );
        let (w, h, bpp, palette, max_level, extra1, extra2, keep) = call;
        c.g[A0][0] = sx(SHP_TEST_IMAGE);
        c.g[A1][0] = sx(w);
        c.g[A2][0] = sx(h);
        c.g[A3][0] = sx(bpp);
        c.g[T0][0] = sx(palette);
        c.g[T1][0] = sx(max_level);
        c.g[T2][0] = sx(extra1);
        c.g[T3][0] = sx(extra2);
        if keep {
            put_words(
                c,
                SHP_INIT_HEADER_GATE,
                &[
                    SHP_INIT_HEADER_MAGIC,
                    SHP_TEST_IMAGE,
                    0x40 + (r.next_u32() % 4) * 0x10,
                ],
            );
        }
        r
    }

    /// Every arm: `seed % 10 == 0` the fixed shapes (`seed / 10 % 10`);
    /// otherwise the palette depth by `seed % 10` (none, out of range, 15,
    /// 16, 18, 24, 32, 64, -8), the depth by `seed / 10 % 9` (0, 2, 4, 8,
    /// 9, 16, 24, 32, -1), the kept palette header by `seed / 90 % 2`, random
    /// sizes, levels and extra blocks. Seeds 480..489 unwind at each of the
    /// ten counted calls of the palette-and-extras shape.
    pub(crate) fn init_header_call(c: &mut Cpu, seed: u32) {
        let (seed, unwind) = shp_unwind_seed(seed, 480, &[90]);
        let call = if seed % 10 == 0 {
            INIT_HEADER_FIXED[(seed / 10 % 10) as usize]
        } else {
            let mut r = Rng(0x27c8 ^ (seed as u64) << 20);
            const PALETTES: [u32; 9] = [0, 4, 15, 16, 18, 24, 32, 0x40, 0xffff_fff8];
            const DEPTHS: [u32; 9] = [0, 2, 4, 8, 9, 16, 24, 32, 0xffff_ffff];
            let extra = |r: &mut Rng| {
                if r.next_u32() % 2 == 0 {
                    0
                } else {
                    0x10 + r.next_u32() % 0x30
                }
            };
            (
                1 + r.next_u32() % 16,
                1 + r.next_u32() % 16,
                DEPTHS[(seed / 10 % 9) as usize],
                PALETTES[(seed % 10 - 1) as usize],
                (r.next_u32() % 6).wrapping_sub(1),
                extra(&mut r),
                extra(&mut r),
                seed / 90 % 2 == 1,
            )
        };
        init_header_setup(c, seed, call);
        if let Some(k) = unwind {
            shp_unwind_setup(c, k);
        }
    }

    /// Totals of the fixed shapes: 32 bpp (no palette) 235; 4 bpp without a
    /// palette 234, 0xff palette 393, grey ramp 467; 2 bpp (no entries) ramp
    /// 317; 32 bpp 4x1 with both extra blocks 373; the kept palette header
    /// (data at an offset) with 0xff bytes 358, with the ramp 432; a
    /// palette of -8-bit entries (negative byte counts) 357; 0xff palette
    /// and both extra blocks 529.
    #[test]
    fn shp_init_header_matches() {
        check_paths_lohi(
            ee::s13::f_003b27c8__generated,
            shp_init_header,
            490,
            &[235, 234, 393, 467, 317, 373, 358, 432, 357, 529],
            &|c, seed| init_header_call(c, seed),
        );
    }

    driven!(gen_shp_init_header, ee::s13::f_003b27c8__generated);

    driven!(hand_shp_init_header, shp_init_header);

    /// The grey-ramp loop (16 or 256 entries) with live events at its back
    /// edge (no other loop runs before it: a 1x1 image, 16-byte clears).
    #[test]
    fn shp_init_header_events() {
        check_events(
            gen_shp_init_header,
            hand_shp_init_header,
            240,
            &|c, seed| {
                let call = (
                    1,
                    1,
                    if seed / 3 % 2 == 0 { 4 } else { 8 },
                    if seed / 6 % 2 == 0 { 32 } else { 0x40 },
                    0,
                    0,
                    0,
                    seed / 12 % 2 == 1,
                );
                let mut r = init_header_setup(c, seed, call);
                schedule(c, &mut r, seed, 0x003b_27c8)
            },
        );
    }

    /// Gate of `shp_fake_sprintf_alloc`: magic, the block the allocator
    /// returns (0: fails), and which kind of call unwinds (1 a name, 2 an
    /// allocation, 3 an "i" text, else none).
    pub(crate) const SHP_ALLOC_GATE: u32 = DATA + 0xf_e500;

    pub(crate) const SHP_ALLOC_MAGIC: u32 = 0x5348_5043;

    pub(crate) static SHP_ALLOC_PREV_SPRINTF: AtomicUsize = AtomicUsize::new(0);

    /// `sprintf` for the SHP tests, also standing in for the allocator
    /// (the tests point the allocator hooks at `sprintf`), told apart by
    /// the return address. With the gate set: `SHP_Create`'s name costs 24
    /// cycles and returns 11; the allocations of `SHP_Create`, `SHP_Clone`
    /// and `SHP_Unpack` cost 9 and return the gate's block;
    /// `SHP_Relocate`'s "i" text costs 24 and returns 6 (nothing written).
    /// Any of them can unwind instead.
    pub(crate) fn shp_fake_sprintf_alloc(c: &mut Cpu) {
        if shp_unwind_at_count(c) {
            return;
        }
        let gate = SHP_ALLOC_GATE;
        let ra = c.g[RA][0] as u32;
        let kind = match ra {
            0x003b_2ae8 => 1,
            0x003b_2b08 | 0x003b_46d8 | 0x003b_364c | 0x003b_3798 => 2,
            0x003b_4644 => 3,
            _ => 0,
        };
        if c.bus.read32(gate) != SHP_ALLOC_MAGIC || kind == 0 {
            return fakes::fall_back(c, &SHP_ALLOC_PREV_SPRINTF, ee::s15::f_00417828__generated);
        }
        match kind {
            1 => {
                c.cycles += 24;
                c.g[V0][0] = 11;
            }
            2 => {
                c.cycles += 9;
                c.g[V0][0] = c.bus.read32(gate + 4) as u64;
            }
            _ => {
                c.cycles += 24;
                c.g[V0][0] = 6;
            }
        }
        c.g[V1][0] = 0x5348_5000 | kind as u64;
        c.pc = if c.bus.read32(gate + 8) == kind {
            0xdead_0000 | kind << 4
        } else {
            ra
        };
    }

    /// Hook `shp_fake_sprintf_alloc` and point both allocator hooks at
    /// `sprintf`; with `block` the gate is set (`unwind` as above).
    pub(crate) fn shp_alloc_setup(c: &mut Cpu, block: Option<u32>, unwind: u32) {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            shp_fake_sprintf_alloc,
            &SHP_ALLOC_PREV_SPRINTF,
        );
        c.bus.write32(SHP_ALLOC_HOOK, 0x0041_7828);
        c.bus.write32(SHP_UNPACK_ALLOC_HOOK, 0x0041_7828);
        if let Some(block) = block {
            put_words(c, SHP_ALLOC_GATE, &[SHP_ALLOC_MAGIC, block, unwind]);
        }
    }

    /// Gate of `shp_fake_mem_copy`.
    pub(crate) const SHP_COPY_GATE: u32 = DATA + 0xf_e900;

    pub(crate) const SHP_COPY_MAGIC: u32 = 0x5348_504d;

    pub(crate) static SHP_COPY_PREV_MEM_COPY: AtomicUsize = AtomicUsize::new(0);

    /// `EA_MemCopy` at a fixed cost (20 cycles) for the chain tests' path
    /// totals: copies the bytes front to back.
    pub(crate) fn shp_fake_mem_copy(c: &mut Cpu) {
        if shp_unwind_at_count(c) {
            return;
        }
        if c.bus.read32(SHP_COPY_GATE) != SHP_COPY_MAGIC {
            return fakes::fall_back(c, &SHP_COPY_PREV_MEM_COPY, ee::s14::f_003e6574__generated);
        }
        let (dst, src, n) = (r32(c, A0), r32(c, A1), r32(c, A2) as i32);
        for i in 0..n.max(0) as u32 {
            let b = c.bus.read8(src + i);
            c.bus.write8(dst + i, b);
        }
        c.cycles += 20;
        c.g[V0][0] = c.g[A0][0];
        c.pc = c.g[RA][0] as u32;
    }

    /// Hook `shp_fake_mem_copy`, gate set or not.
    pub(crate) fn shp_copy_setup(c: &mut Cpu, on: bool) {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6574),
            shp_fake_mem_copy,
            &SHP_COPY_PREV_MEM_COPY,
        );
        if on {
            c.bus.write32(SHP_COPY_GATE, SHP_COPY_MAGIC);
        }
    }

    /// `SHP_Create` call state: `seed % 6` picks a fixed shape (0: the size
    /// comes to 0; 1: the allocation fails; 2: 1x1 at 32 bpp) or random
    /// sizes, depths, palettes and extras (3-5); the allocation fails for
    /// `seed / 6 % 5 == 0`; the name or the allocation unwinds for
    /// `seed / 30 % 4` 1 or 2.
    pub(crate) fn create_call(c: &mut Cpu, seed: u32) {
        let (seed, unwind_at) = shp_unwind_seed(seed, 360, &[2]);
        let mut r = shp_machine(c, seed, 0x2a68, 0x003b_2a68);
        fill(c, &mut r, DATA + 0x1000, 0x800);
        let fails = seed % 6 == 1 || (seed % 6 >= 3 && seed / 6 % 5 == 0);
        let unwind = if seed % 6 < 3 { 0 } else { seed / 30 % 4 };
        shp_alloc_setup(c, Some(if fails { 0 } else { SHP_TEST_IMAGE }), unwind);
        // (w, h, bpp, palBpp, maxLevel, extra1, extra2)
        let args: [u32; 7] = match seed % 6 {
            0 => [1, 1, 32, 0, 0, 0, 0xffff_ffdc],
            1 | 2 => [1, 1, 32, 0, 0, 0, 0],
            _ => {
                const DEPTHS: [u32; 5] = [0, 4, 8, 16, 32];
                const PALETTES: [u32; 5] = [0, 15, 16, 24, 32];
                let extra = |r: &mut Rng| {
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        0x10 + r.next_u32() % 0x30
                    }
                };
                [
                    1 + r.next_u32() % 16,
                    1 + r.next_u32() % 16,
                    DEPTHS[(r.next_u32() % 5) as usize],
                    PALETTES[(r.next_u32() % 5) as usize],
                    r.next_u32() % 5,
                    extra(&mut r),
                    extra(&mut r),
                ]
            }
        };
        let [w, h, bpp, palette, max_level, extra1, extra2] = args;
        c.g[A0][0] = sx(w);
        c.g[A1][0] = sx(h);
        c.g[A2][0] = sx(bpp);
        c.g[A3][0] = sx(palette);
        c.g[T0][0] = sx(max_level);
        c.g[T1][0] = sx(r.next_u32());
        c.g[T2][0] = sx(extra1);
        c.g[T3][0] = sx(extra2);
        if let Some(k) = unwind_at {
            shp_unwind_setup(c, k);
        }
    }

    /// Totals: the size comes to 0 (extra2 -0x24 cancels a 1x1 32 bpp
    /// image) 232; the allocation fails 282; a 1x1 32 bpp image 525. Seeds
    /// 360..371 unwind at each counted call of the 1x1 image (inside
    /// CalcAllocSize, the name, the allocation, inside InitHeader).
    #[test]
    fn shp_create_matches() {
        check_paths(
            ee::s13::f_003b2a68__generated,
            shp_create,
            372,
            &[232, 282, 525],
            &|c, seed| create_call(c, seed),
        );
    }

    /// A DXT block and its destination: alpha ends (`seed / 3 % 2`: alpha0
    /// above alpha1 or not), indices all 0, all 7 or random (`seed / 6 %
    /// 3`), the source at an odd address, a filled destination and a
    /// random pitch. The index doublewords are loaded from `src + 2` and
    /// `src + 5`, which the bus rounds down to 8 bytes: the index pattern
    /// also goes into the three low bytes of each of those doublewords.
    pub(crate) fn dxt_alpha_block(c: &mut Cpu, r: &mut Rng, seed: u32, src: u32, dst: u32) {
        let (hi, lo) = (0x80 + r.next_u32() % 0x80, r.next_u32() % 0x80);
        let (a0, a1) = match seed / 3 % 2 {
            0 => (hi, lo),
            _ if seed / 18 % 3 == 0 => (lo, lo),
            _ => (lo, hi),
        };
        let mut block = [a0 as u8, a1 as u8, 0, 0, 0, 0, 0, 0];
        for b in block[2..].iter_mut() {
            *b = match seed / 6 % 3 {
                0 => 0,
                1 => 0xff,
                _ => r.next_u32() as u8,
            };
        }
        for (i, b) in block.iter().enumerate() {
            c.bus.write8(src + i as u32, *b);
        }
        for half in [src.wrapping_add(2) & !7, src.wrapping_add(5) & !7] {
            for i in 0..3 {
                let b = match seed / 6 % 3 {
                    0 => 0,
                    1 => 0xff,
                    _ => r.next_u32() as u8,
                };
                if half + i < src || half + i >= src + 2 {
                    c.bus.write8(half + i, b);
                }
            }
        }
        fill(c, r, dst, 0x100);
    }

    /// Mode 3 or DXT5 (`seed % 3`), both palettes, all index values.
    pub(crate) fn dxt_alpha_call(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = shp_machine(c, seed, 0x2b90, DXT_ALPHA_ENTRY);
        let (src, dst) = (DATA + 0x1235, DATA + 0x2008);
        dxt_alpha_block(c, &mut r, seed, src, dst);
        c.g[A0][0] = sx(dst);
        c.g[A1][0] = sx(0x20 + (r.next_u32() % 8) * 8);
        c.g[A2][0] = sx(src);
        c.g[A3][0] = match seed % 3 {
            0 => DXT_MODE_EXPLICIT_ALPHA,
            1 => 5,
            _ => (r.next_u32() % 3) as u64,
        };
        r
    }

    /// Totals: mode 3 229; DXT5 sevenths palette 398 with indices 0, 382
    /// with indices 7; fifths 385 / 369 (index 7 stores cost one cycle
    /// less). Random indices reach every jump-table case.
    #[test]
    fn shp_decode_dxt_alpha_block_matches() {
        check_paths_lohi(
            ee::s13::f_003b2b90__generated,
            shp_decode_dxt_alpha_block,
            216,
            &[229, 398, 382, 385, 369],
            &|c, seed| {
                dxt_alpha_call(c, seed);
            },
        );
    }

    driven!(
        gen_shp_decode_dxt_alpha_block,
        ee::s13::f_003b2b90__generated
    );

    driven!(hand_shp_decode_dxt_alpha_block, shp_decode_dxt_alpha_block);

    /// Events at every loop head (delivered; the loops do not unwind).
    /// `seed / 3 % 3`: random blocks with the default early deadline (the
    /// first pixel or byte back edge), or a DXT5 sevenths block with indices
    /// 0 whose deadline lands at the first row back edge (cycle 140 of the
    /// run: the pixel checks fall at 90, 108, 126, the row check at 148) or
    /// at the half back edge (220: last pixel check 204, half check 230).
    #[test]
    fn shp_decode_dxt_alpha_block_events() {
        check_events(
            gen_shp_decode_dxt_alpha_block,
            hand_shp_decode_dxt_alpha_block,
            216,
            &|c, seed| {
                let pick = seed / 3 % 3;
                let mut r = dxt_alpha_call(c, if pick == 0 { seed / 9 } else { 1 });
                let mode = schedule(c, &mut r, seed, DXT_ALPHA_ENTRY);
                if mode != Events::Off && pick != 0 {
                    c.next_event = c.cycles + if pick == 1 { 140 } else { 220 };
                }
                mode
            },
        );
    }

    /// `SHP_DXTAlphaIsOpaque` call: mode 3 or another (`seed % 2`); no
    /// blocks (sizes <= 15, negative ones too) or 1..4 of them (`seed / 2 %
    /// 4`, 0 meaning none); all opaque, or one block failing on its first
    /// or (mode 3) second word (`seed / 8 % 3`); `blocks` overrides the
    /// count.
    pub(crate) fn dxt_opaque_call(c: &mut Cpu, seed: u32, blocks: Option<u32>) -> Rng {
        let mut r = shp_machine(c, seed, 0x3308, 0x003b_3308);
        let buf = DATA + 0x1238;
        let explicit = seed % 2 == 0;
        let n = blocks.unwrap_or(seed / 2 % 4 + seed / 24 % 2);
        let size = match n {
            0 => [0, 7, 15, 0xffff_fffb, 0xffff_ff9c, 0xffff_ffff][(r.next_u32() % 6) as usize],
            _ => n * 16 + r.next_u32() % 16,
        };
        fill(c, &mut r, buf, 0x20 * (n + 1));
        for i in 0..n {
            let block = buf + i * 0x20;
            if explicit {
                c.bus.write64(block, 0xffff_ffff);
                c.bus.write64(block + 8, 0xffff_ffff);
            } else {
                c.bus.write16(block, 0xffff);
            }
        }
        if n > 0 {
            let block = buf + r.next_u32() % n * 0x20;
            let bit = 1u64 << (r.next_u32() % 64);
            match seed / 8 % 3 {
                0 => {}
                1 if explicit => c.bus.write64(block, 0xffff_ffff ^ bit),
                2 if explicit => c.bus.write64(block + 8, 0xffff_ffff ^ bit),
                _ => c
                    .bus
                    .write16(block, 0xffff ^ (1 << (r.next_u32() % 16)) as u16),
            }
        }
        c.g[A0][0] = sx(buf);
        c.g[A1][0] = sx(size);
        c.g[A2][0] = if explicit {
            DXT_MODE_EXPLICIT_ALPHA
        } else {
            (r.next_u32() % 3) as u64
        };
        r
    }

    /// Totals: no blocks 12; other modes, one opaque block 19, failing 15;
    /// mode 3, one opaque block 23, failing on the first word 17, on the
    /// second 20.
    #[test]
    fn shp_dxt_alpha_is_opaque_matches() {
        check_paths(
            ee::s13::f_003b3308__generated,
            shp_dxt_alpha_is_opaque,
            288,
            &[12, 19, 15, 23, 17, 20],
            &|c, seed| {
                dxt_opaque_call(c, seed, None);
            },
        );
    }

    driven!(gen_shp_dxt_alpha_is_opaque, ee::s13::f_003b3308__generated);

    driven!(hand_shp_dxt_alpha_is_opaque, shp_dxt_alpha_is_opaque);

    /// 40..63 blocks with live events at both loops' back edges.
    #[test]
    fn shp_dxt_alpha_is_opaque_events() {
        check_events(
            gen_shp_dxt_alpha_is_opaque,
            hand_shp_dxt_alpha_is_opaque,
            192,
            &|c, seed| {
                let mut r = dxt_opaque_call(c, seed / 3, Some(40 + seed % 24));
                schedule(c, &mut r, seed, 0x003b_3308)
            },
        );
    }

    /// Gate of `shp_decode_dxt_fake_color_block`.
    pub(crate) const SHP_DECODE_DXT_GATE: u32 = DATA + 0xf_e600;

    pub(crate) const SHP_DECODE_DXT_MAGIC: u32 = 0x5348_5044;

    pub(crate) static SHP_DECODE_DXT_PREV_COLOR: AtomicUsize = AtomicUsize::new(0);

    /// `SHP_DecodeDXTColorBlock` for `SHP_DecodeDXT`'s path totals: with
    /// the gate set it costs 10 cycles and leaves the block's first word
    /// xor the pitch at the output.
    pub(crate) fn shp_decode_dxt_fake_color_block(c: &mut Cpu) {
        if shp_unwind_at_count(c) {
            return;
        }
        if c.bus.read32(SHP_DECODE_DXT_GATE) != SHP_DECODE_DXT_MAGIC {
            return fakes::fall_back(
                c,
                &SHP_DECODE_DXT_PREV_COLOR,
                ee::s13::f_003b2e78__generated,
            );
        }
        let block = r32(c, A2);
        let v = c.bus.read32(block) ^ r32(c, A1);
        let out = r32(c, A0);
        c.bus.write32(out, v);
        c.cycles += 10;
        c.pc = c.g[RA][0] as u32;
    }

    /// `SHP_DecodeDXT` call: mode 3, 5 or DXT1 (`seed % 3`); the colour
    /// fake on (`seed / 3 % 2`); 4x4, no rows (h 0 or -4), empty rows (w 0),
    /// random sizes up to 16x16, -4 x -4 (a positive pixel count with no
    /// rows) or w = 0x80000001 x 16 (16 pixels by 32-bit wrap, empty rows)
    /// (`seed / 6 % 6`); opaque or transparent alpha (`seed / 36 % 2`; the
    /// first DXT5 block's alpha ends 0xc0 > 0x40 with indices 0).
    pub(crate) fn decode_dxt_call(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = shp_machine(c, seed, 0x33b0, 0x003b_33b0);
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B2E78),
            shp_decode_dxt_fake_color_block,
            &SHP_DECODE_DXT_PREV_COLOR,
        );
        let (blocks, out) = (DATA + 0x1230, DATA + 0x3008);
        fill(c, &mut r, blocks, 0x200);
        fill(c, &mut r, out, 0x300);
        if seed / 3 % 2 == 1 {
            c.bus.write32(SHP_DECODE_DXT_GATE, SHP_DECODE_DXT_MAGIC);
        }
        let mode = match seed % 3 {
            0 => DXT_MODE_EXPLICIT_ALPHA,
            1 => DXT_MODE_INTERPOLATED_ALPHA,
            _ => [0, 1, 2, 4, 6][(r.next_u32() % 5) as usize],
        };
        let (w, h): (u32, u32) = match seed / 6 % 6 {
            0 => (4, 4),
            1 => (
                4,
                if r.next_u32() % 2 == 0 {
                    0
                } else {
                    0xffff_fffc
                },
            ),
            2 => (0, 4),
            3 => (1 + r.next_u32() % 16, 1 + r.next_u32() % 16),
            4 => (0xffff_fffc, 0xffff_fffc),
            _ => (0x8000_0001, 16),
        };
        let opaque = seed / 36 % 2 == 0;
        for i in 0..16 {
            let block = blocks + i * 0x20;
            match (mode, opaque) {
                (DXT_MODE_EXPLICIT_ALPHA, true) => {
                    c.bus.write64(block, 0xffff_ffff);
                    c.bus.write64(block + 8, 0xffff_ffff);
                }
                (DXT_MODE_EXPLICIT_ALPHA, false) if i == 0 => {
                    c.bus.write64(block, 0x1234_5678_9abc_def0);
                }
                (DXT_MODE_INTERPOLATED_ALPHA, true) => c.bus.write16(block, 0xffff),
                (DXT_MODE_INTERPOLATED_ALPHA, false) if i == 0 => {
                    c.bus.write64(block, 0x40c0);
                }
                _ => {}
            }
        }
        c.g[A0][0] = sx(blocks);
        c.g[A1][0] = sx(out);
        c.g[A2][0] = sx(w);
        c.g[A3][0] = sx(h);
        c.g[T0][0] = mode;
        r
    }

    /// Totals with the colour fake: DXT1 4x4 66, no rows 36, empty rows
    /// 46; mode 3 4x4 opaque 92, transparent 320, no rows 51, empty rows 61;
    /// mode 5 4x4 opaque 91, transparent 490, no rows 54, empty rows 64;
    /// transparent -4 x -4 (alpha walk, no rows) mode 3 56, mode 5 57;
    /// transparent 0x80000001 x 16 (alpha walk, four empty rows) mode 3 87,
    /// mode 5 88, DXT1 67. Seeds 432..434 unwind at the first colour block
    /// of the alpha, opaque and DXT1 walks.
    #[test]
    fn shp_decode_dxt_matches() {
        check_paths_lohi(
            ee::s13::f_003b33b0__generated,
            shp_decode_dxt,
            435,
            &[
                66, 36, 46, 92, 320, 51, 61, 91, 490, 54, 64, 56, 57, 87, 88, 67,
            ],
            &|c, seed| {
                let (seed, unwind) = shp_unwind_seed(seed, 432, &[39, 3, 5]);
                decode_dxt_call(c, seed);
                if let Some(k) = unwind {
                    shp_unwind_setup(c, k);
                }
            },
        );
    }

    /// The source pointer with a non-zero upper half (the low word as in
    /// `decode_dxt_call`): the first colour block of a DXT1 image is passed
    /// the whole register, an alpha image's the sign-extended word + 8.
    #[test]
    fn shp_decode_dxt_wide_source_matches() {
        check_lohi(
            ee::s13::f_003b33b0__generated,
            shp_decode_dxt,
            216,
            &|c, seed| {
                decode_dxt_call(c, seed);
                c.g[A0][0] = c.g[A0][0] as u32 as u64 | 0x95e6_c37f << 32;
            },
        );
    }

    driven!(gen_shp_decode_dxt, ee::s13::f_003b33b0__generated);

    driven!(hand_shp_decode_dxt, shp_decode_dxt);

    /// Live events (`seed / 3 % 4`): random sizes with the default early
    /// deadline (0; events land in the block decoders and the walks); an
    /// 8x8 transparent mode 3 image (colour fake on) whose deadline lands
    /// at the alpha walk's block back edge (1: cycle 290 of the run; the
    /// first alpha block's last check is at 270, the block check at 303) or
    /// at its row back edge (2: 545; last alpha check 523, row check 560);
    /// a 4x8 opaque mode 3 image (one block per row, colour fake on) whose
    /// deadline lands at the opaque walk's row back edge (3: cycle 70; the
    /// only earlier check is DXTAlphaIsOpaque's at 46, the row check at 88).
    #[test]
    fn shp_decode_dxt_events() {
        check_events(gen_shp_decode_dxt, hand_shp_decode_dxt, 240, &|c, seed| {
            let pick = seed / 3 % 4;
            let s = seed / 12;
            let mut r = match pick {
                0 => decode_dxt_call(c, s % 6 + 18 + s / 6 % 2 * 36 + s / 12 * 72),
                3 => {
                    let r = decode_dxt_call(c, 3);
                    c.g[A3][0] = 8;
                    r
                }
                _ => {
                    let r = decode_dxt_call(c, 39);
                    c.g[A2][0] = 8;
                    c.g[A3][0] = 8;
                    r
                }
            };
            let mode = schedule(c, &mut r, seed, 0x003b_33b0);
            if mode != Events::Off && pick != 0 {
                c.next_event = c.cycles
                    + match pick {
                        1 => 290,
                        2 => 545,
                        _ => 70,
                    };
            }
            mode
        });
    }

    /// Every bit depth from 0 to 39 (each jump-table case) and random ones.
    #[test]
    fn shp_palette_format_for_bpp_matches() {
        check_paths(
            ee::s13::f_003b38b8__generated,
            shp_palette_format_for_bpp,
            200,
            &[6, 15, 14, 13],
            &|c, seed| {
                let mut r = shp_machine(c, seed, 0x38b8, PALETTE_FORMAT_ENTRY);
                c.g[A0][0] = if seed < 160 {
                    (seed % 40) as u64
                } else {
                    sx(r.next_u32())
                };
            },
        );
    }

    /// Entered at a jump-table case (0x3b38e4), the port hands the call to
    /// the generated twin.
    #[test]
    fn shp_palette_format_for_bpp_inner_entry_matches() {
        check(
            ee::s13::f_003b38b8__generated,
            shp_palette_format_for_bpp,
            8,
            &|c, seed| {
                shp_machine(c, seed, 0x38b9, 0x003b_38e4);
            },
        );
    }

    /// Where the shape-sheet tests put the sheet, the name looked up and
    /// the images (one 0x100-byte slot each).
    pub(crate) const SHPS_TEST_SHEET: u32 = DATA + 0x1230;

    pub(crate) const SHPS_TEST_NAME: u32 = DATA + 0x1101;

    pub(crate) const SHPS_TEST_IMAGES: u32 = SHPS_TEST_SHEET + 0x200;

    /// Write a NUL-terminated string.
    pub(crate) fn shps_put_str(c: &mut Cpu, at: u32, text: &[u8]) {
        for (i, b) in text.iter().chain(&[0]).enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
    }

    /// One entry's image: a chain of `blocks` blocks 0x20 apart (format
    /// bytes 0x05, 0x6f, ...), the one at `long_at` (if any) a "p" block
    /// with the long name `"long<i>"`.
    pub(crate) fn shps_put_image(c: &mut Cpu, i: u32, blocks: u32, long_at: Option<u32>) {
        let image = SHPS_TEST_IMAGES + i * 0x100;
        for k in 0..blocks {
            let block = image + k * 0x20;
            let format = if long_at == Some(k) {
                0x70
            } else {
                [0x05, 0x6f, 0x69, 0x21][k as usize % 4]
            };
            let next = if k + 1 < blocks { 0x20 } else { 0 };
            c.bus.write32(block, format | next << 8);
            if long_at == Some(k) {
                shps_put_str(c, block + 4, format!("long{i}").as_bytes());
            }
        }
    }

    /// A sheet of `count` entries named `"e<i>__"` (4 bytes), entry `i`'s
    /// image `1 + r % 3` blocks long with or without a long name; `name`
    /// the looked-up string.
    pub(crate) fn shps_put_sheet(c: &mut Cpu, r: &mut Rng, count: u32, name: &[u8]) {
        fill(c, r, SHPS_TEST_SHEET, 0x40);
        c.bus.write32(SHPS_TEST_SHEET + SHPS_COUNT, count);
        for i in 0..count.min(32) {
            let entry = SHPS_TEST_SHEET + SHPS_ENTRIES + i * SHPS_ENTRY_SIZE;
            for (k, b) in [b'e', b'0' + i as u8 % 10, b'_', b'_'].iter().enumerate() {
                c.bus.write8(entry + k as u32, *b);
            }
            c.bus.write32(entry + SHPS_ENTRY_OFFSET, 0x200 + i * 0x100);
            let blocks = 1 + r.next_u32() % 3;
            let long_at = if r.next_u32() % 2 == 0 {
                Some(r.next_u32() % blocks)
            } else {
                None
            };
            shps_put_image(c, i, blocks, long_at);
        }
        shps_put_str(c, SHPS_TEST_NAME, name);
    }

    /// Gate of `shps_fake_strcmp`.
    pub(crate) const SHPS_GATE: u32 = DATA + 0xf_e700;

    pub(crate) const SHPS_MAGIC: u32 = 0x5348_5053;

    pub(crate) static SHPS_PREV_STRCMP: AtomicUsize = AtomicUsize::new(0);

    /// `strcmp` at a fixed cost (12 cycles) for the shape-sheet path
    /// totals: 0 when the strings are equal, else 1.
    pub(crate) fn shps_fake_strcmp(c: &mut Cpu) {
        if shp_unwind_at_count(c) {
            return;
        }
        if c.bus.read32(SHPS_GATE) != SHPS_MAGIC {
            return fakes::fall_back(c, &SHPS_PREV_STRCMP, ee::s15::f_004165a8__generated);
        }
        let (a, b) = (r32(c, A0), r32(c, A1));
        let mut differs = 0;
        for i in 0..64 {
            let (x, y) = (c.bus.read8(a + i), c.bus.read8(b + i));
            if x != y {
                differs = 1;
                break;
            }
            if x == 0 {
                break;
            }
        }
        c.cycles += 12;
        c.g[V0][0] = differs;
        c.pc = c.g[RA][0] as u32;
    }

    /// `SHPS_FindEntryByName` / `SHPS_Find` call: the strcmp fake on
    /// (`seed % 2`); no entries (0 or negative), one entry that is itself
    /// a "p" block, one without a long name, or 2..6 random entries
    /// (`seed / 2 % 5`); the name matching an entry (its long name if it
    /// has one) or not (`seed / 10 % 2`).
    pub(crate) fn shps_find_call(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut r = shp_machine(c, seed, 0x3900, entry);
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004165A8),
            shps_fake_strcmp,
            &SHPS_PREV_STRCMP,
        );
        if seed % 2 == 1 {
            c.bus.write32(SHPS_GATE, SHPS_MAGIC);
        }
        let matches = seed / 10 % 2 == 0;
        let shape = seed / 2 % 5;
        let count = match shape {
            0 => [0, 0xffff_fffe][(r.next_u32() % 2) as usize],
            1 | 2 => 1,
            _ => 2 + r.next_u32() % 5,
        };
        shps_put_sheet(c, &mut r, count, b"nope");
        if shape == 1 {
            shps_put_image(c, 0, 1, Some(0));
        } else if shape == 2 {
            shps_put_image(c, 0, 1, None);
        }
        if matches && count < 32 && count > 0 {
            // The name of a random entry, as the search will see it.
            let i = r.next_u32() % count;
            let long = shps_test_long_name(c, SHPS_TEST_IMAGES + i * 0x100);
            let mut name = Vec::new();
            match long {
                Some(at) => {
                    for k in 0..16 {
                        let b = c.bus.read8(at + k);
                        if b == 0 {
                            break;
                        }
                        name.push(b);
                    }
                }
                None => name.extend_from_slice(&[b'e', b'0' + i as u8 % 10, b'_', b'_']),
            }
            shps_put_str(c, SHPS_TEST_NAME, &name);
        }
        c.g[A0][0] = sx(SHPS_TEST_SHEET);
        c.g[A1][0] = sx(SHPS_TEST_NAME);
        r
    }

    /// Where the test images' long names are (the fixture's own walk).
    pub(crate) fn shps_test_long_name(c: &mut Cpu, mut block: u32) -> Option<u32> {
        loop {
            let word = c.bus.read32(block);
            if word & 0xff == 0x70 {
                return Some(block + 4);
            }
            let next = (word as i32 >> 8) as u32;
            if next == 0 {
                return None;
            }
            block = block.wrapping_add(next);
        }
    }

    /// `SHP_FindLongName`: chains of 1..4 blocks (`seed / 2 % 4`) with the
    /// "p" block at some position or none (`seed / 8 % 4`); null for
    /// `seed % 16 == 15`.
    pub(crate) fn find_long_name_call(c: &mut Cpu, seed: u32, blocks: Option<u32>) -> Rng {
        let mut r = shp_machine(c, seed, 0x3a10, 0x003b_3a10);
        let blocks = blocks.unwrap_or(1 + seed / 2 % 4);
        let long_at = match seed / 8 % 4 {
            0 => None,
            k => Some((k - 1 + r.next_u32()) % blocks),
        };
        fill(c, &mut r, SHPS_TEST_IMAGES, 0x400);
        shps_put_image(c, 0, blocks, long_at);
        c.g[A0][0] = if seed % 16 == 15 {
            0
        } else {
            sx(SHPS_TEST_IMAGES)
        };
        r
    }

    /// Totals: null 4; found in the first block 8, the second 16; not
    /// found in one block 14, in two 22.
    #[test]
    fn shp_find_long_name_matches() {
        check_paths(
            ee::s13::f_003b3a10__generated,
            shp_find_long_name,
            256,
            &[4, 8, 16, 14, 22],
            &|c, seed| {
                find_long_name_call(c, seed, None);
            },
        );
    }

    driven!(gen_shp_find_long_name, ee::s13::f_003b3a10__generated);

    driven!(hand_shp_find_long_name, shp_find_long_name);

    /// Chains of 8..15 blocks with live events at the back edge.
    #[test]
    fn shp_find_long_name_events() {
        check_events(
            gen_shp_find_long_name,
            hand_shp_find_long_name,
            150,
            &|c, seed| {
                let mut r = find_long_name_call(c, seed / 3 % 4 * 8 + 2, Some(8 + seed % 8));
                schedule(c, &mut r, seed, 0x003b_3a10)
            },
        );
    }

    /// `SHP_CalcPixelDataSize` call: a palette (or 0x22), DXT (or 0x63),
    /// plain or random format (`seed % 4`); not swizzled, swizzled or random (`seed / 4 %
    /// 3`); levels 0, 1 or random (`seed / 12 % 3`); a width with 16..23
    /// extra entries, a random one, a negative one or 0x7fffffff (`seed /
    /// 36 % 4`).
    pub(crate) fn pixel_data_size_call(c: &mut Cpu, seed: u32, max_level: Option<u32>) -> Rng {
        let mut r = shp_machine(c, seed, 0x3a50, 0x003b_3a50);
        let format = match seed % 4 {
            0 => SHP_FORMAT_PALETTE16 + (r.next_u32() % 3) as u64,
            1 => SHP_FORMAT_DXT1 + (r.next_u32() % 4) as u64,
            2 => 1 + (r.next_u32() % 5) as u64,
            _ => (r.next_u32() & 0xff) as u64,
        };
        let swizzled = match seed / 4 % 3 {
            0 => 0,
            1 => 1,
            _ => sx(r.next_u32()),
        };
        let max_level = max_level.unwrap_or(match seed / 12 % 3 {
            0 => 0,
            1 => 1,
            _ => r.next_u32() % 16,
        });
        let w = match seed / 36 % 4 {
            0 => 16 + r.next_u32() % 8 + 32 * (r.next_u32() % 2),
            1 => r.next_u32() % 65,
            // Negative widths (a negative palette byte count).
            2 => 0u32.wrapping_sub(1 + r.next_u32() % 31),
            // Rows of more than 2^31 bits (a negative product).
            _ => 0x7fff_ffff,
        };
        c.g[A0][0] = format;
        c.g[A1][0] = sx(w);
        c.g[A2][0] = sx(r.next_u32() % 65);
        c.g[A3][0] = max_level as u64;
        c.g[T0][0] = swizzled;
        r
    }

    /// Totals: a swizzled palette with 16..23 extra entries 57, other
    /// widths 59; one level of a palette 71, a plain image 69, a DXT image
    /// 73; two levels of a plain image 90.
    #[test]
    fn shp_calc_pixel_data_size_matches() {
        check_paths_lohi(
            ee::s13::f_003b3a50__generated,
            shp_calc_pixel_data_size,
            289,
            &[57, 59, 71, 69, 73, 90],
            &|c, seed| {
                // Seed 288: a plain image unwinding at SHP_GetBpp.
                let (seed, unwind) = shp_unwind_seed(seed, 288, &[2]);
                pixel_data_size_call(c, seed, None);
                if let Some(k) = unwind {
                    shp_unwind_setup(c, k);
                }
            },
        );
    }

    /// Swizzled palettes (both formats, one or two levels) whose width
    /// leaves 24 or 15 extra entries: each one past an edge of the two
    /// compares on `w % 32`.
    #[test]
    fn shp_calc_pixel_data_size_swizzled_widths_matches() {
        check_lohi(
            ee::s13::f_003b3a50__generated,
            shp_calc_pixel_data_size,
            16,
            &|c, seed| {
                pixel_data_size_call(c, seed, Some(seed / 8 % 2));
                c.g[A0][0] = SHP_FORMAT_PALETTE16 + (seed % 2) as u64;
                c.g[A1][0] = [24, 15, 56, 47][(seed / 2 % 4) as usize];
                c.g[T0][0] = 1;
            },
        );
    }

    driven!(gen_shp_calc_pixel_data_size, ee::s13::f_003b3a50__generated);

    driven!(hand_shp_calc_pixel_data_size, shp_calc_pixel_data_size);

    /// 16 mip levels with live events at the loop's back edge.
    #[test]
    fn shp_calc_pixel_data_size_events() {
        check_events(
            gen_shp_calc_pixel_data_size,
            hand_shp_calc_pixel_data_size,
            150,
            &|c, seed| {
                let mut r = pixel_data_size_call(c, seed / 3 % 2 + 1 + seed / 6 * 4, Some(15));
                schedule(c, &mut r, seed, 0x003b_3a50)
            },
        );
    }

    /// Gate of `shp_block_size_fake_strlen`.
    pub(crate) const SHP_BLOCK_SIZE_GATE: u32 = DATA + 0xf_e800;

    pub(crate) const SHP_BLOCK_SIZE_MAGIC: u32 = 0x5348_504c;

    pub(crate) static SHP_BLOCK_SIZE_PREV_STRLEN: AtomicUsize = AtomicUsize::new(0);

    /// `strlen` at a fixed cost (9 cycles) for `SHP_GetBlockSize`'s path
    /// totals.
    pub(crate) fn shp_block_size_fake_strlen(c: &mut Cpu) {
        if shp_unwind_at_count(c) {
            return;
        }
        if c.bus.read32(SHP_BLOCK_SIZE_GATE) != SHP_BLOCK_SIZE_MAGIC {
            return fakes::fall_back(
                c,
                &SHP_BLOCK_SIZE_PREV_STRLEN,
                ee::s15::f_00416810__generated,
            );
        }
        let at = r32(c, A0);
        let mut n = 0;
        while n < 0x100 && c.bus.read8(at + n) != 0 {
            n += 1;
        }
        c.cycles += 9;
        c.g[V0][0] = n as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// A RefPack stream of `unpacked` bytes that ends at once (6 bytes).
    pub(crate) fn shp_put_refpack_header(c: &mut Cpu, at: u32, unpacked: u32) {
        let bytes = [
            0x10,
            0xfb,
            (unpacked >> 16) as u8,
            (unpacked >> 8) as u8,
            unpacked as u8,
            0xfc,
        ];
        for (i, b) in bytes.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
    }

    /// `SHP_GetBlockSize` call on one block (`seed % 9`): "o"; "i" without
    /// and with a size; "|"; "p" (strlen fake on for `seed / 9 % 2 == 0`);
    /// a 4x4 32-bit image; a compressed image (its stream at +0x10, or at
    /// an offset for `seed / 9 % 2 == 1`); one whose stream is empty; a
    /// random image header.
    pub(crate) fn block_size_call(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = shp_machine(c, seed, 0x3bb0, 0x003b_3bb0);
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416810),
            shp_block_size_fake_strlen,
            &SHP_BLOCK_SIZE_PREV_STRLEN,
        );
        let block = SHP_TEST_IMAGE;
        fill(c, &mut r, block, 0x40);
        let alt = seed / 9 % 2 == 1;
        let word0 = r.next_u32() & !0xff;
        let set_format = |c: &mut Cpu, format: u32| c.bus.write32(block, word0 | format);
        match seed % 9 {
            0 => set_format(c, SHP_BLOCK_O as u32),
            1 => {
                set_format(c, SHP_BLOCK_I as u32);
                c.bus.write16(block + 6, (r.next_u32() as u16) & !0x10);
            }
            2 => {
                set_format(c, SHP_BLOCK_I as u32);
                c.bus.write16(block + 6, (r.next_u32() as u16) | 0x10);
                c.bus
                    .write16(block + 4, if alt { 0 } else { r.next_u32() as u16 });
            }
            3 => set_format(c, SHP_BLOCK_PIPE as u32),
            4 => {
                set_format(c, SHP_BLOCK_LONG_NAME as u32);
                let len = r.next_u32() % 20;
                let name: Vec<u8> = (0..len).map(|i| b'a' + (i % 26) as u8).collect();
                shps_put_str(c, block + 4, &name);
                if !alt {
                    c.bus.write32(SHP_BLOCK_SIZE_GATE, SHP_BLOCK_SIZE_MAGIC);
                }
            }
            5 => {
                put_words(c, block, &[word0 | 5, 4 | 4 << 16, 0, 0]);
            }
            6 | 7 => {
                set_format(c, 0x85);
                let unpacked = if seed % 9 == 7 {
                    0
                } else {
                    1 + r.next_u32() % 0x1000
                };
                let flags = r.next_u32() & !(SHP_FLAG_DATA_AT_OFFSET as u32);
                if alt && seed % 9 == 6 {
                    c.bus
                        .write32(block + SHP_FLAGS, flags | SHP_FLAG_DATA_AT_OFFSET as u32);
                    c.bus.write32(block + SHP_DATA, 0x21);
                    shp_put_refpack_header(c, block + 0x21, unpacked);
                } else {
                    c.bus.write32(block + SHP_FLAGS, flags);
                    shp_put_refpack_header(c, block + SHP_DATA, unpacked);
                }
            }
            _ => {
                const FORMATS: [u32; 10] = [1, 2, 3, 4, 5, 0x20, 0x21, 0x60, 0x62, 0x7d];
                set_format(c, FORMATS[(r.next_u32() % 10) as usize]);
                c.bus.write16(block + 4, (r.next_u32() % 80) as u16);
                c.bus.write16(block + 6, (r.next_u32() % 80) as u16);
            }
        }
        c.g[A0][0] = sx(block);
        r
    }

    /// Totals: "o" 15; "i" 20 / 25; "|" 24; "p" (strlen fake) 31; a 4x4
    /// 32-bit image 100; compressed 77, with its stream at an offset 79,
    /// empty 75.
    #[test]
    fn shp_get_block_size_matches() {
        check_paths(
            ee::s13::f_003b3bb0__generated,
            shp_get_block_size,
            272,
            &[15, 20, 25, 24, 31, 100, 77, 79, 75],
            &|c, seed| {
                // Seeds 270, 271: unwinding at strlen ("p") and inside
                // SHP_CalcPixelDataSize (the 4x4 image).
                let (seed, unwind) = shp_unwind_seed(seed, 270, &[4, 5]);
                block_size_call(c, seed);
                if let Some(k) = unwind {
                    shp_unwind_setup(c, k);
                }
            },
        );
    }

    /// A block chain at `at`: one block per format, 0x20 apart, linked by
    /// their next offsets (the last ends it).
    pub(crate) fn shp_put_chain(c: &mut Cpu, at: u32, formats: &[u32]) {
        for (k, format) in formats.iter().enumerate() {
            let next = if k + 1 < formats.len() { 0x20 } else { 0 };
            c.bus.write32(at + 0x20 * k as u32, format | next << 8);
        }
    }

    /// `SHP_FindInfoData` call: null for `seed % 16 == 15`; chains of
    /// 1..4 blocks (`seed / 2 % 4`) with an "i" block at some position or
    /// none (`seed / 8 % 4`), its data flag set or not (`seed % 2`).
    pub(crate) fn find_info_data_call(c: &mut Cpu, seed: u32, blocks: Option<u32>) -> Rng {
        let mut r = shp_machine(c, seed, 0x3d40, 0x003b_3d40);
        let blocks = blocks.unwrap_or(1 + seed / 2 % 4);
        let info_at = match seed / 8 % 4 {
            0 => None,
            k => Some((k - 1 + r.next_u32()) % blocks),
        };
        fill(c, &mut r, SHP_TEST_IMAGE, 0x200);
        let formats: Vec<u32> = (0..blocks)
            .map(|k| {
                if Some(k) == info_at {
                    SHP_BLOCK_I as u32
                } else {
                    [0x05, 0x6f, 0x70, 0x7c][k as usize % 4]
                }
            })
            .collect();
        shp_put_chain(c, SHP_TEST_IMAGE, &formats);
        if let Some(k) = info_at {
            let flags = r.next_u32() as u16 & !(SHP_INFO_HAS_DATA as u16);
            let flags = if seed % 2 == 0 {
                flags | SHP_INFO_HAS_DATA as u16
            } else {
                flags
            };
            c.bus.write16(SHP_TEST_IMAGE + 0x20 * k + 6, flags);
        }
        c.g[A0][0] = if seed % 16 == 15 {
            0
        } else {
            sx(SHP_TEST_IMAGE)
        };
        r
    }

    /// Totals: null 5; the image itself an "i" block 14; one block, none
    /// 15; found second 23; two blocks, none 25.
    #[test]
    fn shp_find_info_data_matches() {
        check_paths(
            ee::s13::f_003b3d40__generated,
            shp_find_info_data,
            256,
            &[5, 14, 15, 23, 25],
            &|c, seed| {
                find_info_data_call(c, seed, None);
            },
        );
    }

    driven!(gen_shp_find_info_data, ee::s13::f_003b3d40__generated);

    driven!(hand_shp_find_info_data, shp_find_info_data);

    /// Chains of 8..15 blocks with live events at the back edge.
    #[test]
    fn shp_find_info_data_events() {
        check_events(
            gen_shp_find_info_data,
            hand_shp_find_info_data,
            150,
            &|c, seed| {
                let mut r = find_info_data_call(c, seed / 3 % 8 * 2, Some(8 + seed % 8));
                schedule(c, &mut r, seed, 0x003b_3d40)
            },
        );
    }

    /// `SHP_GetPixelPtr` call: 32-bit, DXT1, a palette (0x20/0x21) or 0x22,
    /// another depth or a random format (`seed % 5`); data at an offset or
    /// not (`seed / 5 % 2`); swizzled or not (`seed / 10 % 2`); level 0, 1
    /// or random -1..5 (`seed / 20 % 3`); random sizes and coordinates,
    /// negated for `seed / 60 % 2 == 1` (x = -1 for DXT1 with `seed / 120
    /// % 2 == 1`).
    pub(crate) fn pixel_ptr_call(c: &mut Cpu, seed: u32, level: Option<u32>) -> Rng {
        let mut r = shp_machine(c, seed, 0x4010, 0x003b_4010);
        let image = SHP_TEST_IMAGE;
        fill(c, &mut r, image, 0x10);
        let format = match seed % 5 {
            0 => 5,
            1 => SHP_FORMAT_DXT1 as u32,
            2 => SHP_FORMAT_PALETTE16 as u32 + r.next_u32() % 3,
            3 => 1 + r.next_u32() % 4,
            _ => r.next_u32() & 0xff,
        };
        let mut flags = r.next_u32() & !((SHP_FLAG_DATA_AT_OFFSET | SHP_FLAG_SWIZZLED) as u32);
        if seed / 5 % 2 == 1 {
            flags |= SHP_FLAG_DATA_AT_OFFSET as u32;
        }
        if seed / 10 % 2 == 1 {
            flags |= SHP_FLAG_SWIZZLED as u32;
        }
        let (w, h) = (1 + r.next_u32() % 256, 1 + r.next_u32() % 256);
        c.bus.write32(image, r.next_u32() & !0xff | format);
        c.bus.write16(image + SHP_WIDTH, w as u16);
        c.bus.write16(image + SHP_HEIGHT, h as u16);
        c.bus.write32(image + SHP_FLAGS, flags);
        let level = level.unwrap_or(match seed / 20 % 3 {
            0 => 0,
            1 => 1,
            _ => (r.next_u32() % 7).wrapping_sub(1),
        });
        // Negative coordinates for `seed / 60 % 2 == 1`.
        let (x, y) = (r.next_u32() % w, r.next_u32() % h);
        let (x, y) = if seed / 60 % 2 == 1 {
            (x.wrapping_neg(), y.wrapping_neg().wrapping_sub(1))
        } else {
            (x, y)
        };
        let x = if seed % 5 == 1 && seed / 60 % 2 == 1 && seed / 120 % 2 == 1 {
            u32::MAX
        } else {
            x
        };
        c.g[A0][0] = sx(image);
        c.g[A1][0] = sx(x);
        c.g[A2][0] = sx(y);
        c.g[A3][0] = sx(level);
        r
    }

    /// Totals: level 0 of a linear 32-bit image (or a linear palette) 79,
    /// with the data at an offset 81; level 1 95; DXT1 93; swizzled
    /// palettes of 32-bit entries 83, 16-bit 84; a swizzled 32-bit image
    /// 104.
    #[test]
    fn shp_get_pixel_ptr_matches() {
        check_paths_lohi(
            ee::s13::f_003b4010__generated,
            shp_get_pixel_ptr,
            362,
            &[79, 81, 95, 93, 83, 84, 104],
            &|c, seed| {
                // Seeds 360, 361: a swizzled 32-bit image unwinding at
                // SHP_GetBpp, then inside SHP_SwizzleAddr.
                let (seed, unwind) = shp_unwind_seed(seed, 360, &[10]);
                pixel_ptr_call(c, seed, None);
                if let Some(k) = unwind {
                    shp_unwind_setup(c, k);
                }
            },
        );
    }

    driven!(gen_shp_get_pixel_ptr, ee::s13::f_003b4010__generated);

    driven!(hand_shp_get_pixel_ptr, shp_get_pixel_ptr);

    /// 30..45 levels skipped with live events at the loop's back edge.
    #[test]
    fn shp_get_pixel_ptr_events() {
        check_events(
            gen_shp_get_pixel_ptr,
            hand_shp_get_pixel_ptr,
            150,
            &|c, seed| {
                let mut r = pixel_ptr_call(c, seed / 3, Some(30 + seed % 16));
                schedule(c, &mut r, seed, 0x003b_4010)
            },
        );
    }

    /// `SHP_SwizzleAddr`: every depth case of the jump table and others
    /// (`seed % 11`); for 16 bits the PSMCT16 arm by `seed / 11 % 2` (x in
    /// the first page of a wide image, or a narrow one).
    #[test]
    fn shp_swizzle_addr_matches() {
        check_paths(
            ee::s13::f_003b4218__generated,
            shp_swizzle_addr,
            225,
            &[11, 56, 55, 50, 51, 25, 18],
            &|c, seed| {
                // Seeds 220..224: unwinding in each layout (4, 8, 15, 16, 32).
                let (seed, unwind) = shp_unwind_seed(seed, 220, &[0, 1, 2, 3, 4]);
                let mut r = shp_machine(c, seed, 0x4218, SWIZZLE_ADDR_ENTRY);
                const DEPTHS: [u32; 10] = [4, 8, 15, 16, 32, 24, 5, 3, 33, 100];
                let bpp = match seed % 11 {
                    10 => r.next_u32(),
                    k => DEPTHS[k as usize],
                };
                let (x, w) = if seed / 11 % 2 == 0 {
                    (r.next_u32() % 64, 128 + r.next_u32() % 512)
                } else {
                    (r.next_u32() % 128, 1 + r.next_u32() % 127)
                };
                c.g[A0][0] = sx(x);
                c.g[A1][0] = sx(r.next_u32() % 512);
                c.g[A2][0] = sx(w);
                c.g[A3][0] = sx(r.next_u32() % 512);
                c.g[T0][0] = sx(bpp);
                if let Some(k) = unwind {
                    shp_unwind_setup(c, k);
                }
            },
        );
    }

    /// Entered at the jump-table dispatch (0x3b4234, case `seed % 0x1d`),
    /// the port hands the call to the generated twin.
    #[test]
    fn shp_swizzle_addr_inner_entry_matches() {
        check(
            ee::s13::f_003b4218__generated,
            shp_swizzle_addr,
            29,
            &|c, seed| {
                shp_machine(c, seed, 0x4219, 0x003b_4234);
                c.g[V1][0] = (seed % 0x1d) as u64;
                c.bus.write64(c.g[SP][0] as u32, crate::difftest::RA as u64);
            },
        );
    }

    /// One block of a test chain.
    #[derive(Clone, Copy)]
    pub(crate) enum TestBlock {
        /// An "o" block whose size field is the value (bytes: + 8).
        O(u32),
        /// An "i" block: size half, data flag.
        I(u16, bool),
        /// A palette (format 0x21) of w x h entries.
        Palette(u16, u16),
        /// A "p" block with a name of n letters.
        Long(u32),
        /// A "|" block of n entries.
        Pipe(u32),
    }

    /// Write `blocks` as a chain at `at`, 0x40 bytes apart.
    pub(crate) fn shp_put_test_chain(c: &mut Cpu, at: u32, blocks: &[TestBlock]) {
        for (k, block) in blocks.iter().enumerate() {
            let b = at + 0x40 * k as u32;
            let next = if k + 1 < blocks.len() { 0x40 << 8 } else { 0 };
            match *block {
                TestBlock::O(n) => put_words(c, b, &[SHP_BLOCK_O as u32 | next, n]),
                TestBlock::I(n, data) => {
                    c.bus.write32(b, SHP_BLOCK_I as u32 | next);
                    c.bus.write16(b + 4, n);
                    c.bus.write16(b + 6, if data { 0x13 } else { 0x2 });
                }
                TestBlock::Palette(w, h) => {
                    put_words(c, b, &[0x21 | next, w as u32 | (h as u32) << 16, 0, 0]);
                }
                TestBlock::Long(n) => {
                    c.bus.write32(b, SHP_BLOCK_LONG_NAME as u32 | next);
                    let name: Vec<u8> = (0..n).map(|i| b'a' + i as u8 % 26).collect();
                    shps_put_str(c, b + 4, &name);
                }
                TestBlock::Pipe(n) => put_words(c, b, &[SHP_BLOCK_PIPE as u32 | next, n]),
            }
        }
    }

    /// A random chain of 1..5 blocks (small sizes).
    pub(crate) fn shp_random_chain(r: &mut Rng, len: u32) -> Vec<TestBlock> {
        (0..len)
            .map(|_| match r.next_u32() % 5 {
                0 => TestBlock::O(r.next_u32() % 0x30),
                1 => TestBlock::I((r.next_u32() % 0x30) as u16, r.next_u32() % 2 == 0),
                2 => {
                    TestBlock::Palette(1 + (r.next_u32() % 8) as u16, 1 + (r.next_u32() % 2) as u16)
                }
                3 => TestBlock::Long(r.next_u32() % 12),
                _ => TestBlock::Pipe(r.next_u32() % 4),
            })
            .collect()
    }

    /// The fixed chains whose totals the chain tests claim, by `k`: one
    /// "o" block, two, an "o" then a palette, a palette alone.
    pub(crate) fn shp_fixed_chain(k: u32) -> Vec<TestBlock> {
        match k {
            0 => vec![TestBlock::O(0x10)],
            1 => vec![TestBlock::O(0x10), TestBlock::O(0x8)],
            2 => vec![TestBlock::O(0x10), TestBlock::Palette(4, 1)],
            _ => vec![TestBlock::Palette(4, 1)],
        }
    }

    /// `SHP_CalcTotalSize` call: null (`seed % 8 == 7`), a fixed chain
    /// (`seed % 8` 0..3) or a random one, at a 16-byte boundary or 8 past
    /// one (`seed / 8 % 2`).
    pub(crate) fn calc_total_size_call(c: &mut Cpu, seed: u32, len: Option<u32>) -> Rng {
        let mut r = shp_machine(c, seed, 0x4478, 0x003b_4478);
        let at = SHP_TEST_IMAGE + 8 * (seed / 8 % 2);
        fill(c, &mut r, SHP_TEST_IMAGE, 0x800);
        let chain = match (len, seed % 8) {
            (Some(n), _) => shp_random_chain(&mut r, n),
            (None, k @ 0..=3) => shp_fixed_chain(k),
            _ => {
                let n = 1 + r.next_u32() % 5;
                shp_random_chain(&mut r, n)
            }
        };
        shp_put_test_chain(c, at, &chain);
        c.g[A0][0] = if seed % 8 == 7 { 0 } else { sx(at) };
        r
    }

    /// Totals: null 25; one "o" block 54; two 93; "o" then a palette 189;
    /// a palette alone 141.
    #[test]
    fn shp_calc_total_size_matches() {
        check_paths(
            ee::s13::f_003b4478__generated,
            shp_calc_total_size,
            161,
            &[25, 54, 93, 189, 141],
            &|c, seed| {
                // Seed 160: the palette alone, unwinding inside its size.
                let (seed, unwind) = shp_unwind_seed(seed, 160, &[3]);
                calc_total_size_call(c, seed, None);
                if let Some(k) = unwind {
                    shp_unwind_setup(c, k);
                }
            },
        );
    }

    driven!(gen_shp_calc_total_size, ee::s13::f_003b4478__generated);

    driven!(hand_shp_calc_total_size, shp_calc_total_size);

    /// Chains of 10..20 blocks with live events at the back edge (and in
    /// the block-size callees).
    #[test]
    fn shp_calc_total_size_events() {
        check_events(
            gen_shp_calc_total_size,
            hand_shp_calc_total_size,
            150,
            &|c, seed| {
                let mut r = calc_total_size_call(c, seed / 3 % 2 * 8, Some(10 + seed % 11));
                schedule(c, &mut r, seed, 0x003b_4478)
            },
        );
    }

    /// Where the chain tests copy to.
    pub(crate) const SHP_TEST_COPY: u32 = DATA + 0x3008;

    /// `SHP_Relocate` call: null source (`seed % 8 == 7`); one or two empty
    /// "o" blocks (0, 1); one "o" block of 0x18 bytes (2); an "i" block with
    /// text room (3; exactly the 21 bytes for `seed / 32 % 2`) or without
    /// (4); a palette (5); random chains (6). The copy and
    /// text fakes on for `seed / 8 % 2 == 0`; the source 16-byte aligned or
    /// 8 past (`seed / 16 % 2`).
    pub(crate) fn relocate_call(c: &mut Cpu, seed: u32, len: Option<u32>) -> Rng {
        let mut r = shp_machine(c, seed, 0x4540, 0x003b_4540);
        let fakes_on = seed / 8 % 2 == 0;
        shp_copy_setup(c, fakes_on);
        shp_alloc_setup(c, if fakes_on { Some(0) } else { None }, 0);
        let at = SHP_TEST_IMAGE + 8 * (seed / 16 % 2);
        fill(c, &mut r, SHP_TEST_IMAGE, 0x800);
        fill(c, &mut r, SHP_TEST_COPY, 0x400);
        let chain = match (len, seed % 8) {
            (Some(n), _) => shp_random_chain(&mut r, n),
            (None, 0) => vec![TestBlock::O(0xffff_fff8)],
            (None, 1) => vec![TestBlock::O(0xffff_fff8), TestBlock::O(0xffff_fff8)],
            (None, 2) => vec![TestBlock::O(0x10)],
            (None, 3) if seed / 32 % 2 == 0 => vec![TestBlock::I(0x20, true)],
            (None, 3) => vec![TestBlock::I(5, true)],
            (None, 4) => vec![TestBlock::I(0x20, false)],
            (None, 5) => vec![TestBlock::Palette(4, 1)],
            _ => {
                let n = 1 + r.next_u32() % 5;
                shp_random_chain(&mut r, n)
            }
        };
        shp_put_test_chain(c, at, &chain);
        c.g[A0][0] = sx(SHP_TEST_COPY);
        c.g[A1][0] = if seed % 8 == 7 { 0 } else { sx(at) };
        r
    }

    /// Totals with the fakes: null 32; one empty block 68, two 105; one
    /// "o" block copied 109; an "i" block with its text 151, without 117;
    /// a palette 196.
    #[test]
    fn shp_relocate_matches() {
        check_paths(
            ee::s13::f_003b4540__generated,
            shp_relocate,
            228,
            &[32, 68, 105, 109, 151, 117, 196],
            &|c, seed| {
                // Seeds 224..227: the palette unwinding inside its size and
                // at its copy; the "i" block at its copy and at its text.
                let (seed, unwind) = shp_unwind_seed(seed, 224, &[5, 3]);
                relocate_call(c, seed, None);
                if let Some(k) = unwind {
                    shp_unwind_setup(c, k);
                }
            },
        );
    }

    driven!(gen_shp_relocate, ee::s13::f_003b4540__generated);

    driven!(hand_shp_relocate, shp_relocate);

    /// Chains of 6..12 blocks (copy and text fakes on) with live events at
    /// the back edge.
    #[test]
    fn shp_relocate_events() {
        check_events(gen_shp_relocate, hand_shp_relocate, 150, &|c, seed| {
            // The fakes on (a real sprintf could unwind mid-call).
            let mut r = relocate_call(c, seed / 3 % 2 * 16, Some(6 + seed % 7));
            schedule(c, &mut r, seed, 0x003b_4540)
        });
    }

    /// `SHP_Clone` call: null (`seed % 5 == 3`), one empty "o" block (0),
    /// one of 0x18 bytes (1), random chains (2), a palette (4); the copy
    /// fake on for `seed / 5 % 2 == 0` (the allocation always faked); the
    /// allocation unwinding for random chains with `seed / 10 % 3 == 1`.
    /// Seeds 150..153 unwind at each counted call of the palette (inside
    /// its total size, the allocation, inside its copy's size, the copy).
    pub(crate) fn clone_call(c: &mut Cpu, seed: u32) {
        let (seed, unwind_at) = shp_unwind_seed(seed, 150, &[4]);
        let mut r = shp_machine(c, seed, 0x4690, 0x003b_4690);
        let fakes_on = seed / 5 % 2 == 0;
        shp_copy_setup(c, fakes_on);
        let unwind = if seed % 5 == 2 && seed / 10 % 3 == 1 {
            2
        } else {
            0
        };
        shp_alloc_setup(c, Some(SHP_TEST_COPY), unwind);
        fill(c, &mut r, SHP_TEST_IMAGE, 0x800);
        fill(c, &mut r, SHP_TEST_COPY, 0x400);
        let chain = match seed % 5 {
            0 => vec![TestBlock::O(0xffff_fff8)],
            1 => vec![TestBlock::O(0x10)],
            4 => vec![TestBlock::Palette(4, 1)],
            _ => {
                let n = 1 + r.next_u32() % 5;
                shp_random_chain(&mut r, n)
            }
        };
        shp_put_test_chain(c, SHP_TEST_IMAGE, &chain);
        c.g[A0][0] = if seed % 5 == 3 { 0 } else { sx(SHP_TEST_IMAGE) };
        if let Some(k) = unwind_at {
            shp_unwind_setup(c, k);
        }
    }

    /// Totals with the fakes: null 99; one empty block 174; one copied
    /// 215; a palette 389.
    #[test]
    fn shp_clone_matches() {
        check_paths(
            ee::s13::f_003b4690__generated,
            shp_clone,
            154,
            &[99, 174, 215, 389],
            &|c, seed| clone_call(c, seed),
        );
    }

    /// Every format from 0x18 to 0x3f (each jump-table case) and random
    /// ones.
    #[test]
    fn shp_is_palette_format_matches() {
        check_paths(
            ee::s13::f_003b4708__generated,
            shp_is_palette_format,
            160,
            &[6, 14, 13],
            &|c, seed| {
                let mut r = shp_machine(c, seed, 0x4708, IS_PALETTE_FORMAT_ENTRY);
                c.g[A0][0] = if seed < 120 {
                    0x18 + (seed % 40) as u64
                } else {
                    sx(r.next_u32())
                };
            },
        );
    }

    /// Entered at a jump-table case (0x3b4734), the port hands the call to
    /// the generated twin.
    #[test]
    fn shp_is_palette_format_inner_entry_matches() {
        check(
            ee::s13::f_003b4708__generated,
            shp_is_palette_format,
            8,
            &|c, seed| {
                shp_machine(c, seed, 0x4709, 0x003b_4734);
            },
        );
    }

    /// Gate of `shp_fake_eac_unpack`.
    pub(crate) const SHP_UNPACK_GATE: u32 = DATA + 0xf_ea00;

    pub(crate) const SHP_UNPACK_MAGIC: u32 = 0x5348_5055;

    pub(crate) static SHP_UNPACK_PREV_EAC: AtomicUsize = AtomicUsize::new(0);

    /// `EAC_Unpack` for `SHP_Unpack`'s tests (the streams are not real):
    /// 30 cycles, the first source word xor a pattern to the destination.
    pub(crate) fn shp_fake_eac_unpack(c: &mut Cpu) {
        if shp_unwind_at_count(c) {
            return;
        }
        if c.bus.read32(SHP_UNPACK_GATE) != SHP_UNPACK_MAGIC {
            return fakes::fall_back(c, &SHP_UNPACK_PREV_EAC, ee::s13::f_003b47f8__generated);
        }
        let (src, dst) = (r32(c, A0), r32(c, A1));
        let v = c.bus.read32(src) ^ 0x5a5a_5a5a;
        c.bus.write32(dst, v);
        c.cycles += 30;
        c.pc = c.g[RA][0] as u32;
    }

    /// An image whose next offset reaches address 0 (the chain's "end"
    /// by wrap-around).
    pub(crate) const SHP_TEST_LOW_IMAGE: u32 = 0x007f_1230;

    /// `SHP_Unpack` call (`seed % 14`): null; one plain "o" block
    /// (cloned); a compressed 4x4 image; a DXT3 4x4 opaque image; a
    /// compressed image followed by an "o" block; a plain block whose next
    /// offset wraps to 0; random (DXT1/3/5 of random sizes and levels,
    /// compressed with a random tail, or a random plain chain); a DXT5
    /// image of -5 x -6 (negative sizes rounded to blocks); a compressed
    /// image whose new block is allocated 16 bytes below it, so the
    /// decompression overwrites the source's next offset with 0; the DXT3
    /// image with two levels; a compressed image followed by a 4x4 32-bit
    /// image; a DXT1 4x4 image; DXT1 images of 2x8 and 8x2 with four levels
    /// (the width, then the height, held at 1 for the last levels). The
    /// copy and colour-block fakes on for `seed / 14 % 2 == 0` (the
    /// allocator and `EAC_Unpack` always faked); the allocation unwinding
    /// for `seed / 28 % 4 == 1`.
    pub(crate) fn unpack_call(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = shp_machine(c, seed, 0x3578, 0x003b_3578);
        let kind = seed % 14;
        let fakes_on = seed / 14 % 2 == 0;
        shp_copy_setup(c, fakes_on);
        let unwind = if seed / 28 % 4 == 1 { 2 } else { 0 };
        let block = if kind == 8 {
            SHP_TEST_IMAGE - 0x10
        } else {
            SHP_TEST_COPY
        };
        shp_alloc_setup(c, Some(block), unwind);
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B47F8),
            shp_fake_eac_unpack,
            &SHP_UNPACK_PREV_EAC,
        );
        c.bus.write32(SHP_UNPACK_GATE, SHP_UNPACK_MAGIC);
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B2E78),
            shp_decode_dxt_fake_color_block,
            &SHP_DECODE_DXT_PREV_COLOR,
        );
        if fakes_on {
            c.bus.write32(SHP_DECODE_DXT_GATE, SHP_DECODE_DXT_MAGIC);
        }
        let image = SHP_TEST_IMAGE;
        fill(c, &mut r, image, 0x400);
        fill(c, &mut r, SHP_TEST_COPY, 0x800);
        // An image header: format, next offset, w, h, flags.
        let put_image =
            |c: &mut Cpu, at: u32, format: u32, next: u32, w: u32, h: u32, flags: u32| {
                put_words(c, at, &[format | next << 8, w | h << 16, 0, flags]);
            };
        let mut at = image;
        match kind {
            0 => {}
            1 => shp_put_test_chain(c, image, &[TestBlock::O(0x10)]),
            2 => put_image(c, image, 0x85, 0, 4, 4, 0),
            3 | 9 => {
                let levels = if kind == 9 { 1 << 28 } else { 0 };
                put_image(c, image, 0x61, 0, 4, 4, levels);
                c.bus.write64(image + 0x10, 0xffff_ffff);
                c.bus.write64(image + 0x18, 0xffff_ffff);
            }
            4 | 8 => {
                put_image(c, image, 0x85, 0x40, 4, 4, 0);
                put_words(c, image + 0x40, &[SHP_BLOCK_O as u32, 0x10]);
                if kind == 8 {
                    // EAC_Unpack's fake writes this word ^ 0x5a5a5a5a over
                    // the source header: format 0x5f, next offset 0.
                    c.bus.write32(image + 0x10, 0x5a5a_5a05);
                }
            }
            5 => {
                at = SHP_TEST_LOW_IMAGE;
                let next = SHP_TEST_LOW_IMAGE.wrapping_neg() & 0x00ff_ffff;
                put_words(c, at, &[SHP_BLOCK_O as u32 | next << 8, 0x10]);
            }
            7 => put_image(c, image, 0x62, 0, 0xfffb, 0xfffa, 0),
            10 => {
                put_image(c, image, 0x85, 0x40, 4, 4, 0);
                put_image(c, image + 0x40, 5, 0, 4, 4, 0);
            }
            11 => put_image(c, image, SHP_FORMAT_DXT1 as u32, 0, 4, 4, 0),
            12 => put_image(c, image, SHP_FORMAT_DXT1 as u32, 0, 2, 8, 3 << 28),
            13 => put_image(c, image, SHP_FORMAT_DXT1 as u32, 0, 8, 2, 3 << 28),
            _ => {
                let len = r.next_u32() % 3;
                let tail = shp_random_chain(&mut r, len);
                let next = if tail.is_empty() { 0 } else { 0x100 };
                let (w, h) = (1 + r.next_u32() % 16, 1 + r.next_u32() % 16);
                let flags = (r.next_u32() % 3) << 28 | r.next_u32() & SHP_FLAG_SWIZZLED as u32;
                match r.next_u32() % 3 {
                    0 => put_image(c, image, 0x60 + r.next_u32() % 3, next, w, h, flags),
                    1 => put_image(c, image, 0x80 | (1 + r.next_u32() % 5), next, w, h, flags),
                    _ => put_image(c, image, 1 + r.next_u32() % 5, next, w, h, flags),
                }
                shp_put_test_chain(c, image + 0x100, &tail);
            }
        }
        c.g[A0][0] = if kind == 0 { 0 } else { sx(at) };
        r
    }

    /// Totals with the fakes: null 29; a plain block cloned 268, the one
    /// whose next offset wraps to 0 269; a compressed image 235, with an
    /// "o" block after it 413, with a 4x4 image after it 583; a DXT3 image
    /// 526; DXT1 images: 4x4 512, the four-level 2x8 1579 and 8x2 1570.
    /// Seeds 448..461 unwind at each of the seven counted calls of the
    /// compressed-plus-image chain (the tail's size, the new block's size,
    /// the allocation, the header copy, the decompression, the tail's copy)
    /// and of the DXT3 image (its size, the allocation, the header copy, both
    /// pixel pointers, the colour block).
    #[test]
    fn shp_unpack_matches() {
        check_paths(
            ee::s13::f_003b3578__generated,
            shp_unpack,
            462,
            &[29, 268, 269, 235, 413, 583, 526, 512, 1579, 1570],
            &|c, seed| {
                let (seed, unwind) = shp_unwind_seed(seed, 448, &[10, 3]);
                unpack_call(c, seed);
                if let Some(k) = unwind {
                    shp_unwind_setup(c, k);
                }
            },
        );
    }

    /// DXT1/3/5 images (fakes on and off) with a width or a height of -1
    /// (0xffff), or source flags with bit 3 set (below the level count).
    #[test]
    fn shp_unpack_dxt_edges_matches() {
        check(
            ee::s13::f_003b3578__generated,
            shp_unpack,
            18,
            &|c, seed| {
                // Kind 11 (one DXT1 4x4 image), fakes by `seed % 2`.
                unpack_call(c, 11 + 14 * (seed % 2));
                let (w, h, flags) =
                    [(0xffff, 4, 0), (4, 0xffff, 0), (4, 4, 8)][(seed / 2 % 3) as usize];
                let format = SHP_FORMAT_DXT1 as u32 + seed / 6 % 3;
                put_words(c, SHP_TEST_IMAGE, &[format, w | h << 16, 0, flags]);
            },
        );
    }

    driven!(gen_shp_unpack, ee::s13::f_003b3578__generated);

    driven!(hand_shp_unpack, shp_unpack);

    /// Live events: random DXT, compressed and plain chains with the
    /// default early deadline (`seed / 3 % 2 == 0`; events land in the
    /// callees' loops), or the two-level DXT3 image with the deadline at
    /// cycle 400 of the run, between the size loop's check (185) and the
    /// level loop's back edge (530), so the event is delivered there.
    #[test]
    fn shp_unpack_events() {
        check_events(gen_shp_unpack, hand_shp_unpack, 150, &|c, seed| {
            let level_head = seed / 3 % 2 == 1;
            let s = seed / 6;
            let mut r = if level_head {
                unpack_call(c, 9)
            } else {
                unpack_call(c, 6 + s % 2 * 14 + s / 2 * 56)
            };
            let mode = schedule(c, &mut r, seed, 0x003b_3578);
            if mode != Events::Off && level_head {
                c.next_event = c.cycles + 400;
            }
            mode
        });
    }

    /// SHP header and coordinates for [`shp_calc_pixel_offset`]: format
    /// from `seed % 4` (0x20, 0x60, 0x40, 0x22), small widths/heights,
    /// coordinates covering the shift arms and -1, level count
    /// `seed / 6 % 3`. The game's image is loaded: `SHP_GetBpp` reads the
    /// real bits-per-pixel table (format 0x20 gives 15).
    pub(crate) const CALC_PIXEL_HEADER: u32 = DATA + 0x1234;

    pub(crate) fn calc_pixel_setup(c: &mut Cpu, seed: u32) -> Rng {
        let r = machine(c, seed, 0x3b_3e40, 0);
        elf_image(c);
        c.pc = 0x003b_3e40;
        // 0x20: palette-16 (bpp 15, the `^ 0xf` arm); 0x60: the shift
        // path; 0x40: plain; 0x22: the `< 2` wide boundary.
        let format = [0x20u32, 0x60, 0x40, 0x22][(seed % 4) as usize];
        c.bus
            .write8(CALC_PIXEL_HEADER + shp_header::FORMAT, format as u8);
        // Widths/heights 2 and 4 drive the `< 2` halving arms to the
        // boundary (the `== 2` / `<= 2` mutants change the decision there).
        c.bus.write16(
            CALC_PIXEL_HEADER + shp_header::WIDTH,
            [64u16, 2, 4][(seed / 4 % 3) as usize],
        );
        c.bus.write16(
            CALC_PIXEL_HEADER + shp_header::HEIGHT,
            [64u16, 4, 2][(seed / 12 % 3) as usize],
        );
        c.bus.write32(
            CALC_PIXEL_HEADER + shp_header::FLAGS,
            (seed / 3 % 2 * 0x2000) as u32,
        );
        c.g[A0][0] = sx(CALC_PIXEL_HEADER);
        // x/y: small coords, the `& 0x1f` high-bit arm (42), the exact-15
        // arm (23), and -1 (the `< -1` boundary the `<=` mutants need).
        c.g[A1][0] = match seed / 36 % 4 {
            0 => (seed / 9 % 48) as u64,
            1 => 23,
            2 => 42,
            _ => u64::MAX,
        };
        c.g[A2][0] = match seed / 7 % 4 {
            0 => (seed / 57 % 48) as u64,
            1 => 23,
            2 => 42,
            _ => u64::MAX,
        };
        c.g[A3][0] = (seed / 6 % 3) as u64;
        r
    }

    /// Formats with and without the shift path (incl. the 0x22 wide
    /// boundary), the 0x2000 flag on and off, widths/heights at the `< 2`
    /// boundary, shift-arm and -1 coordinates, levels 0..2.
    #[test]
    fn shp_calc_pixel_offset_matches() {
        check_lohi(
            ee::s13::f_003b3e40__generated,
            shp_calc_pixel_offset,
            200,
            &|c, seed| {
                calc_pixel_setup(c, seed);
            },
        );
    }

    use crate::difftest::compare;

    #[test]
    fn shp_calc_pixel_offset_events() {
        driven!(gen_calc, ee::s13::f_003b3e40__generated);
        driven!(hand_calc, shp_calc_pixel_offset);
        check_events(gen_calc, hand_calc, 60, &|c, seed| {
            calc_pixel_setup(c, seed / 3);
            schedule(
                c,
                &mut Rng(0x3b_3e40 ^ (seed as u64) << 8),
                seed,
                0x003b_3e40,
            )
        });
    }

    #[test]
    fn av_player_ctor_3_matches() {
        check(
            ee::s13::f_003ade58__generated,
            av_player_ctor_3,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ade_58, 0);
                alloc_ready(c);
                clear_3(c, AVPLAYER_THIS, 0x100);
                c.g[A0][0] = AVPLAYER_THIS as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A2][0] = if seed / 2 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A3][0] = if seed / 4 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T0][0] = if seed / 8 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T1][0] = if seed / 16 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T2][0] = if seed / 32 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T3][0] = if seed / 64 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                // The eighth argument arrives on the caller's stack.
                c.bus.write32(
                    crate::difftest::SP + 0x30,
                    if seed / 128 % 2 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn av_player_ctor_4_matches() {
        check(
            ee::s13::f_003ade90__generated,
            av_player_ctor_4,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ade_90, 0);
                alloc_ready(c);
                clear_3(c, AVPLAYER_THIS, 0x100);
                c.g[A0][0] = AVPLAYER_THIS as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A2][0] = if seed / 2 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A3][0] = if seed / 4 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T0][0] = if seed / 8 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T1][0] = if seed / 16 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T2][0] = if seed / 32 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T3][0] = if seed / 64 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    /// Gate for the stream-file fakes below: they answer only while this word
    /// holds `AVCHUNK_CTRL_KEY` (otherwise the slot body they replaced runs).
    pub(crate) const AVCHUNK_CTRL: u32 = DATA + 0x3000;

    pub(crate) const AVCHUNK_CTRL_KEY: u32 = 0x4156_5052;

    pub(crate) static AVCHUNK_PREV_GETCHUNK: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static AVCHUNK_PREV_ISIDLE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn avchunk_gated(c: &mut Cpu) -> bool {
        c.bus.read32(AVCHUNK_CTRL) == AVCHUNK_CTRL_KEY
    }

    /// `StreamFile_GetChunk`: no chunk queued.
    pub(crate) fn avchunk_fake_getchunk(c: &mut Cpu) {
        if !avchunk_gated(c) {
            return fakes::fall_back(c, &AVCHUNK_PREV_GETCHUNK, ee::s14::f_003e12e0__generated);
        }
        fakes::fake_call(c, 0x003e_12e0);
        c.g[V0][0] = 0;
    }

    /// `StreamFile_IsIdle`: the stream is idle, so the chunk loop exits.
    pub(crate) fn avchunk_fake_isidle(c: &mut Cpu) {
        if !avchunk_gated(c) {
            return fakes::fall_back(c, &AVCHUNK_PREV_ISIDLE, ee::s14::f_003e1530__generated);
        }
        fakes::fake_call(c, 0x003e_1530);
        c.g[V0][0] = 1;
    }

    pub(crate) fn avchunk_install_stream_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E12E0),
            avchunk_fake_getchunk,
            &AVCHUNK_PREV_GETCHUNK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E1530),
            avchunk_fake_isidle,
            &AVCHUNK_PREV_ISIDLE,
        );
    }

    /// Chunk-callback fixture pieces (all 16-aligned, nonzero low byte).
    pub(crate) const AVCHUNK_PLAYER: u32 = DATA + 0x1410;

    pub(crate) const AVCHUNK_HOLDER: u32 = DATA + 0x1450;

    pub(crate) const AVCHUNK_CHUNK: u32 = DATA + 0x1460;

    pub(crate) const AVCHUNK_STREAM: u32 = DATA + 0x1500;

    pub(crate) const AVCHUNK_MAGIC: u32 = DATA + 0x1510;

    pub(crate) const AVCHUNK_SCRATCH: u32 = DATA + 0x1700;

    #[test]
    fn av_player_get_chunk_callback_matches() {
        // A valid stream with no chunk queued: `GetChunk` returns 0 and
        // `IsIdle` reports idle, so `OnChunk` takes its short path. (Its
        // deeper arms belong to `OnChunk`'s own test below.)
        check(
            ee::s13::f_003aecb8__generated,
            av_player_get_chunk_callback,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aec_b8, 0);
                avchunk_install_stream_fakes();
                c.bus.write32(AVCHUNK_CTRL, AVCHUNK_CTRL_KEY);
                c.bus.write32(AVCHUNK_MAGIC, 0x4d52_5453);
                c.bus.write32(AVCHUNK_MAGIC + 0x38, 1);
                for i in 0..8 {
                    c.bus.write32(AVCHUNK_PLAYER + 4 * i, 0xdead_beef);
                    c.bus.write32(AVCHUNK_CHUNK + 4 * i, r.next_u32());
                }
                c.bus.write32(AVCHUNK_CHUNK + 0x30, AVCHUNK_STREAM);
                c.bus.write32(AVCHUNK_HOLDER, AVCHUNK_CHUNK);
                c.bus.write32(AVCHUNK_STREAM, AVCHUNK_MAGIC);
                c.bus.write32(AVCHUNK_STREAM + 0x8, 0);
                c.bus.write32(AVCHUNK_SCRATCH, 0xdead_beef);
                c.g[A0][0] = AVCHUNK_PLAYER as u64;
                c.g[A1][0] = AVCHUNK_HOLDER as u64;
                c.g[A2][0] = AVCHUNK_SCRATCH as u64;
            },
        );
    }

    /// Int-field fixture: the pointer, length and output words plus the
    /// 16-byte head scratch.
    pub(crate) const AVINT_PP: u32 = DATA + 0x1410;

    pub(crate) const AVINT_PLEN: u32 = DATA + 0x1414;

    pub(crate) const AVINT_OUT: u32 = DATA + 0x1418;

    pub(crate) const AVINT_STR: u32 = DATA + 0x1420;

    pub(crate) fn av_int_setup(c: &mut Cpu, seed: u32) {
        // `seed % 4`: 0 empty (returns 0, output untouched), 1 delimiter
        // mid-string (restore + value), 2 delimiter first (empty head,
        // returns 0), 3 digits-or-nothing to the end. A short length stops
        // the scan before the delimiter.
        machine(c, seed, 0x3af5_60, 0);
        for i in 0..8 {
            c.bus.write32(AVINT_STR + 4 * i, 0xdead_beef);
        }
        c.bus.write32(AVINT_PP, AVINT_STR);
        c.bus.write32(AVINT_OUT, 0xdead_beef);
        let (text, len): (&[u8], u32) = match seed % 4 {
            0 => (b"", 0),
            1 => (b"12\n34", 5),
            2 => (b"\n789", 4),
            _ => match seed / 4 % 3 {
                0 => (b"456", 3),
                1 => (b"-12", 3),
                _ => (b"abc", 3),
            },
        };
        for (i, &b) in text.iter().enumerate() {
            c.bus.write8(AVINT_STR + i as u32, b);
        }
        c.bus.write8(AVINT_STR + text.len() as u32, 0);
        let len = if seed % 4 == 1 && seed / 16 % 2 == 0 {
            2
        } else {
            len
        };
        c.bus.write32(AVINT_PLEN, 0xdead_beef);
        c.bus.write32(AVINT_PLEN, len);
        c.g[A0][0] = AVINT_PP as u64;
        c.g[A1][0] = AVINT_PLEN as u64;
        c.g[A2][0] = AVINT_OUT as u64;
    }

    #[test]
    fn av_parse_int_field_matches() {
        check(
            ee::s13::f_003af560__generated,
            av_parse_int_field,
            200,
            &av_int_setup,
        );
    }

    /// Vertical-tab bytes (0x0b): first-byte and mid-string VT hit the
    /// `(byte-9) < 0x2` class boundary (V0==2) that tab/LF/digits never do.
    pub(crate) fn av_int_vt_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x3af5_60, 0);
        for i in 0..8 {
            c.bus.write32(AVINT_STR + 4 * i, 0xdead_beef);
        }
        c.bus.write32(AVINT_PP, AVINT_STR);
        c.bus.write32(AVINT_OUT, 0xdead_beef);
        // VT first (empty-class hit at once), VT mid-string (scan finds
        // it), VT only (scan to end after VT? no: VT ends the scan).
        let (text, len): (&[u8], u32) = match seed % 3 {
            0 => (b"\x0b789", 4),
            1 => (b"45\x0b67", 5),
            _ => (b"\x0b", 1),
        };
        for (i, &b) in text.iter().enumerate() {
            c.bus.write8(AVINT_STR + i as u32, b);
        }
        c.bus.write8(AVINT_STR + text.len() as u32, 0);
        c.bus.write32(AVINT_PLEN, 0xdead_beef);
        c.bus.write32(AVINT_PLEN, len);
        c.g[A0][0] = AVINT_PP as u64;
        c.g[A1][0] = AVINT_PLEN as u64;
        c.g[A2][0] = AVINT_OUT as u64;
    }

    #[test]
    fn av_parse_int_field_vt_matches() {
        check(
            ee::s13::f_003af560__generated,
            av_parse_int_field,
            96,
            &av_int_vt_setup,
        );
    }

    /// Store-path fixture: the game image for `_strtol_r`'s ctype table at
    /// `0x499c81` (without it no byte is ever a digit and `at_end` stays
    /// true), then digit texts that all take the `!at_end` store in
    /// generated block `0x3af64c` (hand wscriptcache.rs:9577-9581).
    /// `seed % 4`: 0 "456", 1 "-7", 2 "0", 3 "12x" (stops mid-way, still
    /// stores 12). Outputs are prefilled `0xdead_beef` so every store is
    /// checked.
    pub(crate) fn av_int_store_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x3af5_60, 0);
        elf_image(c);
        for i in 0..8 {
            c.bus.write32(AVINT_STR + 4 * i, 0xdead_beef);
        }
        c.bus.write32(AVINT_PP, AVINT_STR);
        c.bus.write32(AVINT_OUT, 0xdead_beef);
        let (text, len): (&[u8], u32) = match seed % 4 {
            0 => (b"456", 3),
            1 => (b"-7", 2),
            2 => (b"0", 1),
            _ => (b"12x", 3),
        };
        for (i, &b) in text.iter().enumerate() {
            c.bus.write8(AVINT_STR + i as u32, b);
        }
        c.bus.write8(AVINT_STR + text.len() as u32, 0);
        c.bus.write32(AVINT_PLEN, 0xdead_beef);
        c.bus.write32(AVINT_PLEN, len);
        c.g[A0][0] = AVINT_PP as u64;
        c.g[A1][0] = AVINT_PLEN as u64;
        c.g[A2][0] = AVINT_OUT as u64;
    }

    #[test]
    fn av_parse_int_field_store_matches() {
        check(
            ee::s13::f_003af560__generated,
            av_parse_int_field,
            32,
            &av_int_store_setup,
        );
        // Stored-value proof that generated block `0x3af64c`'s `!at_end`
        // store ran on the hand body: each value differs from the
        // `0xdead_beef` prefill (including "0", which stores zero) and the
        // return is 1.
        for (seed, want) in [(0u32, 456u32), (1, (-7i32) as u32), (2, 0), (3, 12)] {
            let mut c = crate::difftest::fresh();
            av_int_store_setup(&mut c, seed);
            av_parse_int_field(&mut c);
            assert_eq!(c.bus.read32(AVINT_OUT), want, "seed {seed}");
            assert_eq!(c.g[V0][0], 1, "seed {seed} return");
        }
    }

    #[test]
    fn av_parse_int_field_events() {
        // The scan loop's bare event check delivers without unwinding, so
        // both sides stay in lockstep through the driver. `strtol` runs
        // real (leaf, no event checks) on both sides.
        driven!(av_int_gen_driven, ee::s13::f_003af560__generated);
        driven!(av_int_hand_driven, av_parse_int_field);
        check_events(av_int_gen_driven, av_int_hand_driven, 96, &|c, seed| {
            let mut r = machine(c, seed, 0x3af5_60, 0);
            av_int_setup(c, seed);
            c.pc = 0x003a_f560;
            schedule(c, &mut r, seed, 0x003a_f560)
        });
    }

    /// Justify fixture: the game-fixed name/value tables plus subject and
    /// dictionary scratch.
    pub(crate) const AVJUSTIFY_NAMES: u32 = 0x0049_5290;

    pub(crate) const AVJUSTIFY_VALS: u32 = 0x0049_52a8;

    pub(crate) const AVJUSTIFY_S: u32 = DATA + 0x1410;

    pub(crate) const AVJUSTIFY_DICT: u32 = DATA + 0x1500;

    pub(crate) const AVJUSTIFY_CTYPE: u32 = 0x0049_9c81;

    pub(crate) fn av_justify_setup(c: &mut Cpu, seed: u32) {
        // `seed % 6`: 0 empty (miss), 1 exact (first entry), 2 case-folded
        // (second entry), 3 tabbed (miss + restore), 4 CR (fourth entry +
        // restore), 5 unknown (miss, no restore).
        machine(c, seed, 0x3af4_28, 0);
        for ch in b'A'..=b'Z' {
            c.bus.write8(AVJUSTIFY_CTYPE + ch as u32, 1);
        }
        const WORDS: [&[u8]; 5] = [b"left", b"right", b"center", b"top", b"up"];
        const VALS: [u32; 5] = [10, 20, 30, 40, 50];
        for (i, w) in WORDS.iter().enumerate() {
            let at = AVJUSTIFY_DICT + i as u32 * 0x10;
            for (j, &b) in w.iter().enumerate() {
                c.bus.write8(at + j as u32, b);
            }
            c.bus.write8(at + w.len() as u32, 0);
            c.bus.write32(AVJUSTIFY_NAMES + 4 * i as u32, at);
            c.bus.write32(AVJUSTIFY_VALS + 4 * i as u32, VALS[i]);
        }
        for i in 0..8 {
            c.bus.write32(AVJUSTIFY_S + 4 * i, 0xdead_beef);
        }
        let (text, len): (&[u8], u32) = match seed % 6 {
            0 => (b"", 0),
            1 => (b"left", 4),
            2 => (b"RIGHT", 5),
            3 => (b"cen\tter", 7),
            4 => (b"top\r", 4),
            _ => (b"zzz", 3),
        };
        for (i, &b) in text.iter().enumerate() {
            c.bus.write8(AVJUSTIFY_S + i as u32, b);
        }
        c.bus.write8(AVJUSTIFY_S + text.len() as u32, 0);
        c.pc = 0x003a_f428;
        c.g[A0][0] = AVJUSTIFY_S as u64;
        c.g[A1][0] = len as u64;
    }

    #[test]
    fn av_parse_justify_matches() {
        check(
            ee::s13::f_003af428__generated,
            av_parse_justify,
            200,
            &av_justify_setup,
        );
    }

    #[test]
    fn av_parse_justify_paths() {
        // One total per arm (seed % 6): empty 182, exact 210, case-folded
        // 281, tabbed 348, CR 304, unknown 284. Inputs are fixed per arm,
        // so the `AV_StrICmp` costs are stable.
        require_paths(
            ee::s13::f_003af428__generated,
            200,
            &[182, 210, 281, 348, 304, 284],
            &av_justify_setup,
        );
    }

    #[test]
    fn av_parse_justify_events() {
        // Both back edges unwind through `drive` and resume at their heads;
        // `AV_StrICmp` runs real (leaf, no event checks) on both sides.
        driven!(av_justify_gen_driven, ee::s13::f_003af428__generated);
        driven!(av_justify_hand_driven, av_parse_justify);
        check_events(
            av_justify_gen_driven,
            av_justify_hand_driven,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3af4_28, 0);
                av_justify_setup(c, seed);
                c.pc = 0x003a_f428;
                schedule(c, &mut r, seed, 0x003a_f428)
            },
        );
    }

    /// Token fixture: subject/pointer/length/output words, the word and
    /// length lists, and dictionary scratch.
    pub(crate) const AVTOKEN_PS: u32 = DATA + 0x1410;

    pub(crate) const AVTOKEN_PLEN: u32 = DATA + 0x1414;

    pub(crate) const AVTOKEN_OUT: u32 = DATA + 0x1418;

    pub(crate) const AVTOKEN_WORDS: u32 = DATA + 0x1420;

    pub(crate) const AVTOKEN_LENS: u32 = DATA + 0x1430;

    pub(crate) const AVTOKEN_S: u32 = DATA + 0x1440;

    pub(crate) const AVTOKEN_DICT: u32 = DATA + 0x1500;

    pub(crate) const AVTOKEN_CTYPE: u32 = 0x0049_9c81;

    pub(crate) fn av_find_token_setup(c: &mut Cpu, seed: u32) {
        // `seed % 7`: 0 null subject, 1 empty remainder, 2 match at (0,0),
        // 3 empty-word match, 4 match at (3,1), 5 leading whitespace, 6 no
        // match (with and without words).
        machine(c, seed, 0x3af6_70, 0);
        for ch in [0x20u32, 0x09, 0x0a, 0x0d] {
            c.bus.write8(AVTOKEN_CTYPE + ch, 8);
        }
        for ch in b'A'..=b'Z' {
            c.bus.write8(AVTOKEN_CTYPE + ch as u32, 1);
        }
        const WORDS: [&[u8]; 3] = [b"alpha", b"beta", b""];
        for (i, w) in WORDS.iter().enumerate() {
            let at = AVTOKEN_DICT + i as u32 * 0x10;
            for (j, &b) in w.iter().enumerate() {
                c.bus.write8(at + j as u32, b);
            }
            c.bus.write8(at + w.len() as u32, 0);
            c.bus.write32(AVTOKEN_WORDS + 4 * i as u32, at);
            c.bus.write32(AVTOKEN_LENS + 4 * i as u32, w.len() as u32);
        }
        for i in 0..8 {
            c.bus.write32(AVTOKEN_S + 4 * i, 0xdead_beef);
        }
        let (text, len, nwords): (Option<&[u8]>, u32, u32) = match seed % 7 {
            0 => (None, 0, 3),
            1 => (Some(b""), 0, 2),
            2 => (Some(b"alpha rest"), 10, 3),
            3 => (Some(b"zz beta"), 7, 3),
            4 => (Some(b"zz beta"), 7, 2),
            5 => (Some(b"  alpha"), 7, 3),
            _ => (Some(b"zzz"), 3, if seed / 7 % 2 == 0 { 2 } else { 0 }),
        };
        if let Some(text) = text {
            for (i, &b) in text.iter().enumerate() {
                c.bus.write8(AVTOKEN_S + i as u32, b);
            }
            c.bus.write8(AVTOKEN_S + text.len() as u32, 0);
            c.bus.write32(AVTOKEN_PS, AVTOKEN_S);
        } else {
            c.bus.write32(AVTOKEN_PS, 0);
        }
        c.bus.write32(AVTOKEN_PLEN, 0xdead_beef);
        c.bus.write32(AVTOKEN_PLEN, len);
        c.bus.write32(AVTOKEN_OUT, 0xdead_beef);
        c.pc = 0x003a_f670;
        c.g[A0][0] = AVTOKEN_PS as u64;
        c.g[A1][0] = AVTOKEN_PLEN as u64;
        c.g[A2][0] = AVTOKEN_WORDS as u64;
        c.g[A3][0] = AVTOKEN_LENS as u64;
        c.g[T0][0] = nwords as u64;
        c.g[T1][0] = AVTOKEN_OUT as u64;
    }

    #[test]
    fn av_find_token_matches() {
        check(
            ee::s13::f_003af670__generated,
            av_find_token,
            200,
            &av_find_token_setup,
        );
    }

    #[test]
    fn av_find_token_paths() {
        // One total per arm (seed % 7): null 32, empty 47, match-(0,0) 143,
        // empty-word 112, match-(3,1) 292, whitespace 163, no-match 201
        // (two words) or 75 (no words). Inputs are fixed per arm, so the
        // `AV_SkipWhitespace` costs are stable.
        require_paths(
            ee::s13::f_003af670__generated,
            200,
            &[32, 47, 143, 112, 292, 163, 201, 75],
            &av_find_token_setup,
        );
    }

    #[test]
    fn av_find_token_events() {
        // The three loop heads (700/718/750) unwind through `drive` and
        // resume at their heads; 6bc is the match exit (no unwind; the
        // bare check covers it). `AV_SkipWhitespace` runs real (bare
        // check only, it never unwinds) on both sides.
        driven!(av_token_gen_driven, ee::s13::f_003af670__generated);
        driven!(av_token_hand_driven, av_find_token);
        check_events(av_token_gen_driven, av_token_hand_driven, 96, &|c, seed| {
            let mut r = machine(c, seed, 0x3af6_70, 0);
            av_find_token_setup(c, seed);
            c.pc = 0x003a_f670;
            schedule(c, &mut r, seed, 0x003a_f670)
        });
    }

    /// Frame-due fixture: player, timer and decoder scratch.
    pub(crate) const AVFRAMEDUE_PLAYER: u32 = DATA + 0x1410;

    pub(crate) const AVFRAMEDUE_TIMER: u32 = DATA + 0x1500;

    pub(crate) const AVFRAMEDUE_DECODER: u32 = DATA + 0x1540;

    /// The EATimer tick-rate word both time tests sample.
    pub(crate) const AVTIME_TICK_RATE: u32 = 0x0045_0dc8;

    #[test]
    fn av_player_is_frame_due_matches() {
        // The accumulator stays 0, so the sum sign comes from `base`:
        // -5 (negative), 0 and 1 (positive, under a frame), 5 (positive,
        // over a frame); the threshold word covers both float arms.
        check(
            ee::s13::f_003ae780__generated,
            av_player_is_frame_due,
            200,
            &|c, seed| {
                machine(c, seed, 0x3ae7_80, 0);
                c.bus.write32(AVTIME_TICK_RATE, 1000);
                for i in 0..0x20 {
                    c.bus.write32(AVFRAMEDUE_PLAYER + 4 * i, 0xdead_beef);
                }
                let base: u32 = [-5i32 as u32, 0, 1, 5][seed as usize % 4];
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x40, base);
                c.bus.write32(
                    AVFRAMEDUE_PLAYER + 0x44,
                    if seed / 4 % 2 == 0 { 0xffff_ffff } else { 1 },
                );
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x4c, 0);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x54, 0);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x58, 0);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x5c, AVFRAMEDUE_TIMER);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x64, AVFRAMEDUE_DECODER);
                for i in 0..8 {
                    c.bus.write32(AVFRAMEDUE_TIMER + 4 * i, 0xdead_beef);
                }
                c.bus.write64(AVFRAMEDUE_TIMER + 0x8, 0);
                c.bus.write32(AVFRAMEDUE_TIMER + 0x10, 0x1000);
                c.bus.write32(AVFRAMEDUE_DECODER + 0x14, 0);
                c.g[A0][0] = AVFRAMEDUE_PLAYER as u64;
            },
        );
    }

    /// Edge values: a base that zeroes the clock sum (X=0, halving
    /// boundary), a very negative base (X<0, halving taken), a zero
    /// threshold word (second-float boundary), and a huge accumulator
    /// (nonzero stored product, GetTimeMs >= 0x800000 so the stale-f2 bits
    /// are not a denormal). GetTimeMs returns 0xa92fc on the stock timer,
    /// so base = -0xa92fc zeroes X.
    #[test]
    fn av_player_is_frame_due_edges_matches() {
        check(
            ee::s13::f_003ae780__generated,
            av_player_is_frame_due,
            32,
            &|c, seed| {
                machine(c, seed, 0x3ae7_80, 0);
                c.bus.write32(AVTIME_TICK_RATE, 1000);
                for i in 0..0x20 {
                    c.bus.write32(AVFRAMEDUE_PLAYER + 4 * i, 0xdead_beef);
                }
                let (base, thresh, accum): (u32, u32, u64) = match seed % 4 {
                    0 => (0xa92fc_u32.wrapping_neg(), 1, 0),
                    1 => (0xf0000000, 0xffff_ffff, 0),
                    2 => (5, 0, 0),
                    _ => (5, 1, 0x900000),
                };
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x40, base);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x44, thresh);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x4c, 0);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x54, 0);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x58, 0);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x5c, AVFRAMEDUE_TIMER);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x64, AVFRAMEDUE_DECODER);
                for i in 0..8 {
                    c.bus.write32(AVFRAMEDUE_TIMER + 4 * i, 0xdead_beef);
                }
                c.bus.write64(AVFRAMEDUE_TIMER + 0x8, accum);
                c.bus.write32(AVFRAMEDUE_TIMER + 0x10, 0x1000);
                c.bus.write32(AVFRAMEDUE_DECODER + 0x14, 0);
                c.g[A0][0] = AVFRAMEDUE_PLAYER as u64;
            },
        );
    }

    /// Time fixture: player, timer and owner scratch plus the SND globals
    /// the status path samples (null stream table, latency flag byte).
    pub(crate) const AVTIME_PLAYER: u32 = DATA + 0x1410;

    pub(crate) const AVTIME_TIMER: u32 = DATA + 0x1500;

    pub(crate) const AVTIME_OWNER: u32 = DATA + 0x1540;

    pub(crate) const AVTIME_STREAM_COUNT: u32 = 0x0050_a92e;

    pub(crate) const AVTIME_STREAM_TABLE: u32 = 0x0050_a0a0;

    /// 0x510000 - 0x55a4: the flag byte `SND_GetOutputLatency` samples.
    pub(crate) const AVTIME_LATENCY: u32 = 0x0051_0000 - 0x55a4;

    #[test]
    fn av_player_get_time_ms_matches() {
        // `seed % 2` enables the sound block; `seed / 2 % 2` lags the
        // stream; `seed / 4 % 3` settles the drift or pushes it past the
        // reset; `seed / 12 % 2` drives both latency-flag arms.
        check_lohi(
            ee::s13::f_003ae9a0__generated,
            av_player_get_time_ms,
            200,
            &|c, seed| {
                machine(c, seed, 0x3ae9_a0, 0);
                c.bus.write32(AVTIME_TICK_RATE, 1000);
                c.bus.write32(AVTIME_STREAM_COUNT, 1);
                c.bus.write32(AVTIME_STREAM_TABLE, 0);
                c.bus.write8(AVTIME_LATENCY, (seed / 12 % 2) as u8);
                for i in 0..0x20 {
                    c.bus.write32(AVTIME_PLAYER + 4 * i, 0xdead_beef);
                }
                c.bus.write32(AVTIME_PLAYER, AVTIME_OWNER);
                let (drift, offset) = match seed / 4 % 3 {
                    0 => (0, 0),
                    1 => (100, 0),
                    _ => (0, 500),
                };
                c.bus.write32(AVTIME_PLAYER + 0x4c, offset);
                c.bus.write32(
                    AVTIME_PLAYER + 0x50,
                    if seed / 2 % 2 == 0 { 0 } else { 0xffff_ffff },
                );
                c.bus.write32(AVTIME_PLAYER + 0x54, (seed % 2) as u32);
                c.bus.write32(AVTIME_PLAYER + 0x58, drift);
                c.bus.write32(AVTIME_PLAYER + 0x5c, AVTIME_TIMER);
                for i in 0..8 {
                    c.bus.write32(AVTIME_TIMER + 4 * i, 0xdead_beef);
                }
                c.bus.write64(AVTIME_TIMER + 0x8, 0);
                c.bus.write32(AVTIME_TIMER + 0x10, 0x1000);
                c.bus.write32(AVTIME_OWNER + 0xc, 0);
                c.g[A0][0] = AVTIME_PLAYER as u64;
            },
        );
    }

    /// Drift-boundary values: drift -1 (the `nonneg` A2=-1 boundary),
    /// drift 7 and 8 (the (A0+7)>>3 shift boundary), and a lagging stream
    /// with drift that propagates past both shifts to the stored fields.
    /// Outputs are prefilled nonzero so every store is checked.
    #[test]
    fn av_player_get_time_ms_drift_matches() {
        check_lohi(
            ee::s13::f_003ae9a0__generated,
            av_player_get_time_ms,
            32,
            &|c, seed| {
                machine(c, seed, 0x3ae9_a0, 0);
                c.bus.write32(AVTIME_TICK_RATE, 1000);
                c.bus.write32(AVTIME_STREAM_COUNT, 1);
                c.bus.write32(AVTIME_STREAM_TABLE, 0);
                c.bus.write8(AVTIME_LATENCY, (seed % 2) as u8);
                for i in 0..0x20 {
                    c.bus.write32(AVTIME_PLAYER + 4 * i, 0xdead_beef);
                }
                c.bus.write32(AVTIME_PLAYER, AVTIME_OWNER);
                let drift: u32 = match seed % 4 {
                    0 => 0xffff_ffff,
                    1 => 7,
                    2 => 8,
                    _ => 100,
                };
                c.bus.write32(AVTIME_PLAYER + 0x4c, 0x1111_1111);
                c.bus.write32(AVTIME_PLAYER + 0x50, 0xffff_ffff);
                c.bus.write32(AVTIME_PLAYER + 0x54, 1);
                c.bus.write32(AVTIME_PLAYER + 0x58, drift);
                c.bus.write32(AVTIME_PLAYER + 0x5c, AVTIME_TIMER);
                for i in 0..8 {
                    c.bus.write32(AVTIME_TIMER + 4 * i, 0xdead_beef);
                }
                c.bus.write64(AVTIME_TIMER + 0x8, 0);
                c.bus.write32(AVTIME_TIMER + 0x10, 0x1000);
                c.bus.write32(AVTIME_OWNER + 0xc, 0);
                c.g[A0][0] = AVTIME_PLAYER as u64;
            },
        );
    }

    #[test]
    fn av_parse_int_field_paths() {
        // Arm totals, one per branch shape (the "456"/"abc" coincidence is
        // real: `strtol` costs the same digits-or-nothing run; RAM differs).
        require_paths(
            ee::s13::f_003af560__generated,
            200,
            &[411, 425, 431, 442, 440, 461],
            &av_int_setup,
        );
    }

    /// Resume frame base for the justify tests (entry opens 0xc0).
    pub(crate) const AVJUSTIFY_FRAME: u32 = DATA + 0x1600;

    /// Resume frame base for the token tests (entry opens 0x80).
    pub(crate) const AVTOKEN_FRAME: u32 = DATA + 0x1600;

    /// Direct-entry resume for `av_parse_justify`'s two used-form heads: the
    /// frame as the entry block leaves it, loop registers at mid-scan
    /// values, compared head by head against the generated body.
    #[test]
    fn av_parse_justify_resume_matches() {
        for head in [0x003a_f468u32, 0x003a_f500] {
            let r = compare(ee::s13::f_003af428__generated, av_parse_justify, &|c| {
                machine(c, 0x3af4_28, head as u64, 0);
                // Tables and dictionary, as in the matches fixture.
                for ch in b'A'..=b'Z' {
                    c.bus.write8(AVJUSTIFY_CTYPE + ch as u32, 1);
                }
                const WORDS: [&[u8]; 5] = [b"left", b"right", b"center", b"top", b"up"];
                for (i, w) in WORDS.iter().enumerate() {
                    let at = AVJUSTIFY_DICT + i as u32 * 0x10;
                    for (j, b) in w.iter().enumerate() {
                        c.bus.write8(at + j as u32, *b);
                    }
                    c.bus.write8(at + w.len() as u32, 0);
                    c.bus.write32(AVJUSTIFY_NAMES + 4 * i as u32, at);
                    c.bus.write32(AVJUSTIFY_VALS + 4 * i as u32, 10 * i as u32);
                }
                // Subject "cen\tter": the tab at index 3 ends the scan.
                for (i, b) in b"cen\tter".iter().enumerate() {
                    c.bus.write8(AVJUSTIFY_S + i as u32, *b);
                }
                c.bus.write8(AVJUSTIFY_S + 7, 0);
                // Frame as the entry block leaves it; saved slots get
                // distinctive garbage so a missed restore changes RAM.
                let sp = AVJUSTIFY_FRAME;
                c.g[SP][0] = sp as u64;
                for i in 0..0x30 {
                    c.bus.write32(sp + 4 * i as u32, 0x5a5a_5a00 + i as u32);
                }
                c.bus.write32(sp + 0x40, crate::difftest::RA);
                c.g[RA][0] = crate::difftest::RA as u64;
                if head == 0x003a_f468 {
                    // Mid-scan: index 2 of 7, tab remembered.
                    c.g[S3][0] = 2;
                    c.g[S5][0] = 7;
                    c.g[S4][0] = AVJUSTIFY_S as u64;
                    c.g[S6][0] = 9;
                } else {
                    // Mid-table: third entry; stack names/values forged as
                    // the copy block lays them out (names at +0x0, values
                    // at +0x20) with the dictionary's values; S1 is the
                    // entry block's -1 (no match found yet).
                    let names = DATA + 0x1700;
                    for i in 0..WORDS.len() {
                        let at = AVJUSTIFY_DICT + i as u32 * 0x10;
                        c.bus.write32(names + 4 * i as u32, at);
                        c.bus.write32(names + 0x20 + 4 * i as u32, 10 * i as u32);
                    }
                    c.g[S2][0] = 2;
                    c.g[S0][0] = names as u64;
                    c.g[S1][0] = u64::MAX;
                    c.g[S4][0] = AVJUSTIFY_S as u64;
                    c.g[S3][0] = 3;
                    c.g[S5][0] = 7;
                    c.g[S6][0] = 9;
                }
                c.pc = head;
            });
            assert!(r.hard.is_empty(), "head {head:#x}: {:#?}", r.hard);
        }
    }

    /// Shared direct-entry setup for `av_find_token`'s four used-form heads:
    /// the frame as the entry block leaves it, loop registers at mid-match
    /// values. Every head below runs to a full "alpha" match (718 enters
    /// with word 0 already rejected and matches word 1).
    fn av_find_token_head_setup(c: &mut Cpu, head: u32) {
        machine(c, 0x3af6_70, head as u64, 0);
        for ch in [0x20u32, 0x09, 0x0a, 0x0d] {
            c.bus.write8(AVTOKEN_CTYPE + ch, 8);
        }
        for ch in b'A'..=b'Z' {
            c.bus.write8(AVTOKEN_CTYPE + ch as u32, 1);
        }
        // One word, "alpha" (the 718 arm below adds a rejected word 0).
        const WORD: &[u8; 5] = b"alpha";
        let at = AVTOKEN_DICT;
        for (j, b) in WORD.iter().enumerate() {
            c.bus.write8(at + j as u32, *b);
        }
        c.bus.write8(at + WORD.len() as u32, 0);
        c.bus.write32(AVTOKEN_WORDS, at);
        c.bus.write32(AVTOKEN_LENS, 5);
        for (i, b) in b"alpha rest".iter().enumerate() {
            c.bus.write8(AVTOKEN_S + i as u32, *b);
        }
        c.bus.write8(AVTOKEN_S + 10, 0);
        c.bus.write32(AVTOKEN_PS, AVTOKEN_S);
        c.bus.write32(AVTOKEN_PLEN, 10);
        c.bus.write32(AVTOKEN_OUT, 0xdead_beef);
        // Frame as the entry block leaves it (subject pointer word
        // at +0x0, saved slots garbage).
        let sp = AVTOKEN_FRAME;
        c.g[SP][0] = sp as u64;
        c.bus.write32(sp, AVTOKEN_S);
        for i in 1..0x20 {
            c.bus.write32(sp + 4 * i as u32, 0x5a5a_5a00 + i as u32);
        }
        c.bus.write32(sp + 0x10, crate::difftest::RA);
        c.g[RA][0] = crate::difftest::RA as u64;
        c.g[S0][0] = AVTOKEN_PLEN as u64;
        c.g[S2][0] = AVTOKEN_PS as u64;
        c.g[S3][0] = AVTOKEN_OUT as u64;
        c.g[S4][0] = AVTOKEN_LENS as u64;
        c.g[S5][0] = AVTOKEN_WORDS as u64;
        c.g[T5][0] = 10;
        c.g[T1][0] = AVTOKEN_LENS as u64;
        c.g[T0][0] = AVTOKEN_WORDS as u64;
        c.g[T4][0] = AVTOKEN_S as u64;
        match head {
            // Match exit: start 0, five bytes matched, word 0.
            0x003a_f6bc => {
                c.g[A3][0] = 0;
                c.g[A1][0] = 5;
                c.g[T3][0] = 0;
                c.g[A0][0] = 0;
                c.g[A2][0] = 0;
            }
            // Next start: fresh scan of the full remainder.
            0x003a_f700 => {
                c.g[S1][0] = 1;
                c.g[T3][0] = 0;
                c.g[A3][0] = 0;
                c.g[A1][0] = 0;
                c.g[A0][0] = 0;
                c.g[A2][0] = 0;
            }
            // Next word: word 0 ("zzz") already tried and rejected,
            // word 1 ("alpha") matches at start 0 and runs to 0x3af6bc.
            0x003a_f718 => {
                let w1 = AVTOKEN_DICT + 0x10;
                for (j, b) in b"zzz\0".iter().enumerate() {
                    c.bus.write8(AVTOKEN_DICT + j as u32, *b);
                }
                for (j, b) in b"alpha\0".iter().enumerate() {
                    c.bus.write8(w1 + j as u32, *b);
                }
                c.bus.write32(AVTOKEN_WORDS, AVTOKEN_DICT);
                c.bus.write32(AVTOKEN_WORDS + 4, w1);
                c.bus.write32(AVTOKEN_LENS, 3);
                c.bus.write32(AVTOKEN_LENS + 4, 5);
                c.g[S1][0] = 2;
                c.g[T3][0] = 1;
                c.g[T1][0] = (AVTOKEN_LENS + 4) as u64;
                c.g[T0][0] = (AVTOKEN_WORDS + 4) as u64;
                c.g[A3][0] = 0;
                c.g[A1][0] = 0;
                c.g[A0][0] = 0;
                c.g[A2][0] = 0;
            }
            // Whole word matched when the count reaches its length:
            // three bytes in ("alp"), two to go ("ha").
            _ => {
                c.g[S1][0] = 1;
                c.g[T3][0] = 0;
                c.g[A3][0] = 0;
                c.g[A1][0] = 3;
                c.g[A2][0] = (AVTOKEN_S + 2) as u64;
                c.g[T2][0] = 5;
                c.g[V0][0] = 0;
                c.g[V1][0] = 0;
            }
        }
        c.pc = head;
    }

    /// Direct-entry resume for `av_find_token`'s four used-form heads,
    /// compared head by head.
    #[test]
    fn av_find_token_resume_matches() {
        for head in [0x003a_f6bcu32, 0x003a_f700, 0x003a_f718, 0x003a_f750] {
            let r = compare(ee::s13::f_003af670__generated, av_find_token, &|c| {
                av_find_token_head_setup(c, head)
            });
            assert!(r.hard.is_empty(), "head {head:#x}: {:#?}", r.hard);
        }
        // The 718 row must genuinely reach the match, not merely agree on
        // the failure path: word index 1 lands in [S3] (failure writes
        // 0xffff), remainder 5 in [S0], out pointer past "alpha" in [S2].
        {
            let mut c = crate::difftest::fresh();
            av_find_token_head_setup(&mut c, 0x003a_f718);
            av_find_token(&mut c);
            assert_eq!(c.bus.read32(AVTOKEN_OUT), 1);
            assert_eq!(c.bus.read32(AVTOKEN_PLEN), 5);
            assert_eq!(c.bus.read32(AVTOKEN_PS), AVTOKEN_S + 5);
        }
    }

    /// Fresh machine with the BELibrary/world singletons, the SND-not-ready
    /// flag (`lb(0x50aa5c)`) and the assert-not-busy word clear, the tick
    /// globals scribbled, and the wsAlloc/free slots (`gp - 0x6bd0/0x6bcc`)
    /// pointed at the test fake. Returns the RNG for fixture garbage.
    pub(crate) fn av_test_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x30_0000);
        c.bus.write32(GAME_GP - 0x848, DATA + 0x31_0000);
        c.bus.write8(0x0050_aa5c, 0);
        c.bus.write32(0x0045_0da4, 0);
        c.bus.write32(0x0045_0dcc, 0x1111_1111);
        c.bus.write32(0x0045_0dc8, 0);
        c.bus.write32(0x0050_9430, FAKE);
        c.bus.write32(0x0050_9434, FAKE);
        c.bus.write32(0x0050_9438, 0);
        r
    }

    /// Fill `words` words at `addr` with nonzero garbage.
    pub(crate) fn av_fill_words(c: &mut Cpu, r: &mut Rng, addr: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(addr + 4 * i, r.next_u32() | 0x0101_0101);
        }
    }

    /// Gate for the stream fakes below: they answer only while this word holds
    /// `AVSTREAM_CTRL_KEY` (otherwise the slot body they replaced runs).
    /// `AVSTREAM_CHUNKVAL` is what the queued-chunk fake returns: 0 for the
    /// empty arm, the forged chunk for the chunk arms. The stream-queue
    /// internals (`Validate`/mutex/list walk) stay on the other side of this
    /// boundary; everything downstream (tag dispatch, codec paths, release)
    /// runs real.
    pub(crate) const AVSTREAM_CTRL: u32 = DATA + 0x3100;

    pub(crate) const AVSTREAM_CTRL_KEY: u32 = 0x5354_5245;

    pub(crate) const AVSTREAM_CHUNKVAL: u32 = DATA + 0x3104;

    pub(crate) static AVSTREAM_PREV_ISIDLE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static AVSTREAM_PREV_GETCHUNK: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn avstream_gated(c: &mut Cpu) -> bool {
        c.bus.read32(AVSTREAM_CTRL) == AVSTREAM_CTRL_KEY
    }

    /// `StreamFile_IsIdle`: the stream is idle, so the chunk loop exits.
    /// `StreamFile_GetChunk` stays real: the fixtures forge a draining
    /// stream and the test proves it drains.
    pub(crate) fn avstream_fake_isidle(c: &mut Cpu) {
        if !avstream_gated(c) {
            return fakes::fall_back(c, &AVSTREAM_PREV_ISIDLE, ee::s14::f_003e1530__generated);
        }
        fakes::fake_call(c, 0x003e_1530);
        c.g[V0][0] = 1;
    }

    pub(crate) fn avstream_install_isidle_fake() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E1530),
            avstream_fake_isidle,
            &AVSTREAM_PREV_ISIDLE,
        );
    }

    /// `StreamFile_GetChunk`: the fixture-forged queue head.
    pub(crate) fn avstream_fake_getchunk(c: &mut Cpu) {
        if !avstream_gated(c) {
            return fakes::fall_back(c, &AVSTREAM_PREV_GETCHUNK, ee::s14::f_003e12e0__generated);
        }
        fakes::fake_call(c, 0x003e_12e0);
        c.g[V0][0] = c.bus.read32(AVSTREAM_CHUNKVAL) as u64;
    }

    pub(crate) fn avstream_install_getchunk_fake() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E12E0),
            avstream_fake_getchunk,
            &AVSTREAM_PREV_GETCHUNK,
        );
    }

    #[test]
    fn av_create_subtitles_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = av_test_base(c, seed, 0xaf0c8);
            let list = DATA + 0x1410;
            let count = seed % 4;
            av_fill_words(c, &mut r, list, 2);
            c.pc = 0x003a_f0c8;
            c.g[A0][0] = list as u64;
            c.g[A1][0] = count as u64;
            c.g[A2][0] = (0x111 + (seed / 4) % 5) as u64;
            c.g[A3][0] = (0x222 + (seed / 8) % 7) as u64;
            c.g[T0][0] = (0x333 + (seed / 16) % 9) as u64;
            c.g[T1][0] = (0x444 + (seed / 32) % 9) as u64;
        };
        check_paths(
            ee::s13::f_003af0c8__generated,
            av_create_subtitles,
            16,
            &[56, 135, 199, 269],
            &setup,
        );
        // The built list holds `count` records stamped with the geometry
        // and the record index as string id (record 0's frame is cleared).
        // The records land at the fake-alloc address, which could in theory
        // overlap the list head; `check` above already proves those seeds
        // equal, so only the shape asserts skip an overlapped seed.
        for seed in 0..8u32 {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            av_create_subtitles(&mut c);
            let list = DATA + 0x1410;
            let count = seed % 4;
            assert_eq!(c.bus.read32(list + 4), count, "seed {seed}");
            let base = c.bus.read32(list);
            if count == 0 {
                continue;
            }
            if base >= list.wrapping_sub(3 * 0x34) && base < list + 8 {
                continue;
            }
            let x = 0x111 + (seed / 4) % 5;
            let y = 0x222 + (seed / 8) % 7;
            let width = 0x333 + (seed / 16) % 9;
            let height = 0x444 + (seed / 32) % 9;
            for i in 0..count {
                let rec = base + i * 0x34;
                assert_eq!(c.bus.read32(rec), x, "seed {seed} rec {i} x");
                assert_eq!(c.bus.read32(rec + 4), y, "seed {seed} rec {i} y");
                assert_eq!(c.bus.read32(rec + 8), width, "seed {seed} rec {i} w");
                assert_eq!(c.bus.read32(rec + 0xc), height, "seed {seed} rec {i} h");
                assert_eq!(c.bus.read32(rec + 0x10), i, "seed {seed} rec {i} id");
                assert_eq!(
                    c.bus.read32(rec + 0x14),
                    if i == 0 { 0 } else { 0xffff_ffff },
                    "seed {seed} rec {i} frame"
                );
            }
        }
    }

    pub(crate) fn av_shutdown_setup(c: &mut Cpu, seed: u32) {
        let mut r = av_test_base(c, seed, 0xae300);
        let this = DATA + 0x1410;
        let arm = seed % 4;
        av_fill_words(c, &mut r, this, 0x1e);
        let decoder = DATA + 0x1510;
        let vtab = DATA + 0x1600;
        av_fill_words(c, &mut r, decoder, 8);
        av_fill_words(c, &mut r, vtab, 4);
        c.bus.write32(vtab + 8, 0);
        c.bus.write32(vtab + 0xc, FAKE);
        c.bus.write32(decoder + 0x18, vtab);
        let audio = DATA + 0x1700;
        av_fill_words(c, &mut r, audio, 4);
        c.bus.write32(audio + 4, 0);
        c.pc = 0x003a_e300;
        c.g[A0][0] = this as u64;
        c.g[A1][0] = (arm >= 2) as u64;
        c.bus
            .write32(this + 0x64, if arm >= 2 { decoder } else { 0 });
        c.bus.write32(this, if arm >= 2 { audio } else { 0 });
        c.bus.write32(this + 0x5c, if arm >= 2 { 0x77 } else { 0 });
        c.bus.write32(this + 0x30, 0);
        c.bus.write32(this + 0x34, 0);
        c.bus.write32(this + 0x14, if arm >= 1 { 0x99 } else { 0 });
        c.bus.write32(this + 0x20, if arm >= 2 { FAKE } else { 0 });
        c.bus.write32(this + 0x28, if arm >= 3 { FAKE } else { 0 });
        c.bus.write32(this + 0x4, if arm >= 2 { 0x55 } else { 0 });
        c.bus.write32(this + 0x8, if arm >= 3 { 0x66 } else { 0 });
        // No stack setup needed: each GetSettings block stores 0x20 at
        // sp first (so the length is always 32), and the settings copy
        // zeroes sp+8, so the chunk callback always goes through the
        // miss-hook fake at target 0.
    }

    #[test]
    fn av_player_shutdown_matches() {
        check_paths(
            ee::s13::f_003ae300__generated,
            av_player_shutdown,
            32,
            &[56, 75, 241, 345],
            &av_shutdown_setup,
        );
    }

    pub(crate) fn av_start_setup(c: &mut Cpu, seed: u32) {
        let mut r = av_test_base(c, seed, 0xae450);
        avstream_install_isidle_fake();
        c.bus.write32(AVSTREAM_CTRL, AVSTREAM_CTRL_KEY);
        let this = DATA + 0x1410;
        let arm = seed % 6;
        av_fill_words(c, &mut r, this, 0x1e);
        let stream = DATA + 0x1900;
        av_fill_words(c, &mut r, stream, 5);
        c.bus.write32(stream + 0x10, 0x1000);
        let hp1 = DATA + 0x1a10;
        let hp2 = DATA + 0x1a20;
        c.bus.write32(hp1, hp2);
        c.bus.write32(hp2, 0x1234_5678);
        let bad = arm == 1;
        c.pc = 0x003a_e450;
        c.g[A0][0] = this as u64;
        c.g[A1][0] = 0x700;
        c.g[A2][0] = 0x800;
        c.bus.write32(this + 0x34, if bad { hp1 } else { 0 });
        c.bus.write32(this + 0x30, if bad { hp1 } else { 0 });
        c.bus.write32(
            this + 0x24,
            match arm {
                2 => 5,
                _ => 0,
            },
        );
        c.bus.write32(
            this + 0x2c,
            match arm {
                3 => 9,
                _ => 0,
            },
        );
        c.bus.write32(this + 0x14, if arm == 5 { 0 } else { 1 });
        c.bus.write32(this + 0x1c, if arm >= 4 { 0 } else { 1 });
        c.bus.write32(this + 0x5c, stream);
        c.bus.write32(this, 0);
        c.bus.write32(0x0050_9430, if arm >= 4 { FAKE } else { 0 });
    }

    #[test]
    fn av_player_start_matches() {
        check_paths(
            ee::s13::f_003ae450__generated,
            av_player_start,
            36,
            &[236, 244, 383, 398, 760, 762],
            &av_start_setup,
        );
        // The two async blocks land in different fields: arm C resolves
        // handle 1 into +0x38, arm C2 resolves handle 2 into +0x3c. The
        // queue call returns slot 0 for the forged handles (not -1).
        for seed in [2u32, 8, 14, 20, 26, 32] {
            let mut c = crate::difftest::fresh();
            av_start_setup(&mut c, seed);
            assert_eq!(seed % 6, 2);
            av_player_start(&mut c);
            assert_eq!(c.bus.read32(DATA + 0x1410 + 0x38), 0);
        }
        for seed in [3u32, 9, 15, 21, 27, 33] {
            let mut c = crate::difftest::fresh();
            av_start_setup(&mut c, seed);
            assert_eq!(seed % 6, 3);
            av_player_start(&mut c);
            assert_eq!(c.bus.read32(DATA + 0x1410 + 0x3c), 0);
        }
        // Arms 4 and 5 take different sub-paths at the +0x14 test (arm 5
        // stores [+0x38] into +0x3c with a +3 charge; arm 4 skips it) and
        // the audio path downstream reads +0x3c, so their totals differ
        // (760 vs 762) even though the +0x3c word itself is clobbered
        // downstream on both arms.
    }

    /// Value-boundary fixtures (arm 5, which runs the audio-init path):
    /// a heap word at the wsAlloc free slot (so `no_heap` is false and the
    /// V0==0 boundary is hit) and a nonzero audio word (so `!(0&V1)`
    /// differs). Outputs are prefilled nonzero and checked by the RAM
    /// comparison. (Timer ticks stay 0: TIMER_UPDATE zeroes the fresh
    /// timer's +8 before it is read, so the shift mutants need a
    /// ticking timer, documented as a gap.)
    pub(crate) fn av_start_values_setup(c: &mut Cpu, seed: u32) {
        av_start_setup(c, seed);
        // Heap with a zero word at +4 (V0==0 boundary for the `<` test).
        c.bus.write32(0x0050_a430, DATA + 0x4000);
        c.bus.write32(DATA + 0x4000 + 4, 0);
        // Nonzero audio word with bit 31 set (arm-5 AudioInit returns a
        // stable address; bit 31 survives the >>31 sign test below).
        c.bus.write32(0x0083_a7d0 + 0xc, 0x8abc_def0);
    }

    #[test]
    fn av_player_start_values_matches() {
        check(
            ee::s13::f_003ae450__generated,
            av_player_start,
            36,
            &av_start_values_setup,
        );
    }

    pub(crate) fn av_onchunk_setup(c: &mut Cpu, seed: u32) {
        let mut r = av_test_base(c, seed, 0xaead0);
        avstream_install_isidle_fake();
        avstream_install_getchunk_fake();
        c.bus.write32(AVSTREAM_CTRL, AVSTREAM_CTRL_KEY);
        // A: empty stream; B1: plain chunk; B2: MPEG chunk, decoder
        // busy; B3: MPEG chunk, codec created and set.
        let arm = seed % 4;
        let player = DATA + 0x1410;
        let decoder = DATA + 0x1420;
        let out = DATA + 0x1430;
        let stream = DATA + 0x1440;
        let head = DATA + 0x1450;
        let chunk = DATA + 0x1460;
        av_fill_words(c, &mut r, player, 0x10);
        av_fill_words(c, &mut r, decoder, 0x10);
        av_fill_words(c, &mut r, stream, 5);
        av_fill_words(c, &mut r, head, 0x10);
        av_fill_words(c, &mut r, chunk, 8);
        c.bus.write32(player + 0x30, stream);
        c.bus.write32(stream, head);
        c.bus.write32(stream + 8, if arm == 0 { 0 } else { 0x10 });
        c.bus.write32(stream + 0xc, chunk);
        c.bus.write32(head, 0x4d52_5453);
        c.bus.write32(head + 4, 0);
        c.bus.write32(head + 0xc, 0);
        c.bus.write32(head + 0x30, 0);
        c.bus.write32(head + 0x34, 8);
        c.bus.write32(head + 0x38, 0);
        c.bus.write32(chunk + 4, if arm == 0 { 0 } else { 0x10 });
        // Arms 2/3 carry the real "MPCh" tag (bytes 4D 50 43 68 assemble to
        // 0x4D504368, the table's first word); arm 1's single nonzero byte
        // is not a table word and stays plain.
        const MPCH: [u8; 4] = [0x4d, 0x50, 0x43, 0x68];
        for i in 0..4u32 {
            c.bus.write8(
                chunk + i,
                if arm == 1 {
                    (i == 0) as u8
                } else if arm >= 2 {
                    MPCH[i as usize]
                } else {
                    0
                },
            );
        }
        c.bus.write32(
            decoder + 0x14,
            match arm {
                2 => 0x888,
                3 => 0,
                _ => 0x99,
            },
        );
        let sp = crate::difftest::SP - 0xb0;
        c.bus.write32(sp + 4, DATA + 0x1500);
        c.bus.write32(DATA + 0x1508, 0);
        c.bus
            .write32(AVSTREAM_CHUNKVAL, if arm == 0 { 0 } else { chunk });
        // The game's real MPEG tag table (orig/SLUS_207.72 at 0x495448):
        // "MPCh"/"MPCd" as OnChunk assembles them (b0<<24|b1<<16|b2<<8|b3).
        c.bus.write32(0x0049_5448, 0x4d50_4368);
        c.bus.write32(0x0049_544c, 0x4d50_4364);
        c.pc = 0x003a_ead0;
        c.g[A0][0] = player as u64;
        c.g[A1][0] = decoder as u64;
        c.g[A2][0] = out as u64;
    }

    #[test]
    fn av_player_on_chunk_matches() {
        check_paths(
            ee::s13::f_003aead0__generated,
            av_player_on_chunk,
            32,
            &[162, 227, 224, 328],
            &av_onchunk_setup,
        );
    }

    /// Second-tag bytes nonzero: chunk bytes 5-7 feed the codec-path tag
    /// assembly (`V0 = b7<<24|b6<<16|b5<<8|b4`), so the shift/operator
    /// mutants there change the stored tag. Bytes 0-3 keep the real MPCh
    /// tag so dispatch is unchanged (MPEG arms).
    pub(crate) fn av_onchunk_bytes_setup(c: &mut Cpu, seed: u32) {
        av_onchunk_setup(c, seed);
        let chunk = DATA + 0x1460;
        // Nonzero, seed-varying; byte 5 gets the high bit on some seeds.
        c.bus.write8(
            chunk + 5,
            (0x20 + seed * 37) as u8 | if seed % 3 == 0 { 0x80 } else { 0 },
        );
        c.bus.write8(chunk + 6, (0x30 + seed * 53) as u8);
        c.bus.write8(chunk + 7, (0x40 + seed * 71) as u8);
    }

    #[test]
    fn av_player_on_chunk_bytes_matches() {
        check(
            ee::s13::f_003aead0__generated,
            av_player_on_chunk,
            32,
            &av_onchunk_bytes_setup,
        );
    }

    /// The chunk loop's bare event check delivers without unwinding, so both
    /// sides stay in lockstep through the driver. `GetChunk`/`IsIdle` are
    /// gated fakes, `EATimer` runs real (bare check only); everything
    /// downstream of a queued chunk runs real.
    #[test]
    fn av_player_on_chunk_events() {
        driven!(av_onchunk_gen_driven, ee::s13::f_003aead0__generated);
        driven!(av_onchunk_hand_driven, av_player_on_chunk);
        check_events(
            av_onchunk_gen_driven,
            av_onchunk_hand_driven,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0xaead_00, 0);
                av_onchunk_setup(c, seed % 32);
                c.pc = 0x003a_ead0;
                schedule(c, &mut r, seed, 0x003a_ead0)
            },
        );
    }

    /// Both subtitle-list build loops deliver their bare event checks without
    /// unwinding. The allocator and record writes run real; the list head
    /// is prefilled nonzero.
    #[test]
    fn av_create_subtitles_events() {
        driven!(av_create_gen_driven, ee::s13::f_003af0c8__generated);
        driven!(av_create_hand_driven, av_create_subtitles);
        check_events(
            av_create_gen_driven,
            av_create_hand_driven,
            96,
            &|c, seed| {
                let mut r = av_test_base(c, seed, 0xaf0c_80);
                let list = DATA + 0x1410;
                av_fill_words(c, &mut r, list, 2);
                c.pc = 0x003a_f0c8;
                c.g[A0][0] = list as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = 0x111 as u64;
                c.g[A3][0] = 0x222 as u64;
                c.g[T0][0] = 0x333 as u64;
                c.g[T1][0] = 0x444 as u64;
                schedule(c, &mut r, seed, 0x003a_f0c8)
            },
        );
    }

    /// `AV_Player` field offsets, used by the fixture to prefill and read
    /// back the struct the port initializes.
    pub(crate) const AV_PLAYER_A1: u32 = 0x0c;

    pub(crate) const AV_PLAYER_A4: u32 = 0x10;

    pub(crate) const AV_PLAYER_ALLOC_A: u32 = 0x04;

    pub(crate) const AV_PLAYER_ALLOC_B: u32 = 0x08;

    pub(crate) const AV_PLAYER_HAS_AUDIO: u32 = 0x14;

    pub(crate) const AV_PLAYER_TYPE: u32 = 0x18;

    pub(crate) const AV_PLAYER_SND_FLAG: u32 = 0x1c;

    pub(crate) const AV_PLAYER_ASYNC_A: u32 = 0x24;

    pub(crate) const AV_PLAYER_ASYNC_B: u32 = 0x2c;

    pub(crate) const AV_PLAYER_STREAM_A: u32 = 0x30;

    pub(crate) const AV_PLAYER_STREAM_B: u32 = 0x34;

    pub(crate) const AV_PLAYER_FILE_A: u32 = 0x38;

    pub(crate) const AV_PLAYER_FILE_B: u32 = 0x3c;

    /// Gate of the stream/async callee fakes: a magic word in the machine's
    /// RAM turns a fake on (both sides of a diff get their own machine).
    /// With the gate off the fake falls back to the body the hook found.
    pub(crate) const AVTEST_MAGIC: u32 = 0x1403_1403;

    pub(crate) const AVTEST_CTRL: u32 = DATA + 0x20_0000;

    macro_rules! avtest_gated_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(AVTEST_CTRL) != AVTEST_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }
    avtest_gated_fake!(
        avtest_fake_stream_create,
        AVTEST_PREV_STREAM_CREATE,
        0x003e_06d8,
        ee::s14::f_003e06d8
    );

    avtest_gated_fake!(
        avtest_fake_get_channel,
        AVTEST_PREV_GET_CHANNEL,
        0x003e_0bf8,
        ee::s14::f_003e0bf8
    );

    avtest_gated_fake!(
        avtest_fake_set_priority,
        AVTEST_PREV_SET_PRIORITY,
        0x003e_0ad8,
        ee::s14::f_003e0ad8
    );

    avtest_gated_fake!(
        avtest_fake_set_channel_filter,
        AVTEST_PREV_SET_CHANNEL_FILTER,
        0x003e_0948,
        ee::s14::f_003e0948
    );

    avtest_gated_fake!(
        avtest_fake_queue_range,
        AVTEST_PREV_QUEUE_RANGE,
        0x003e_0c88,
        ee::s14::f_003e0c88
    );

    avtest_gated_fake!(
        avtest_fake_queue_file,
        AVTEST_PREV_QUEUE_FILE,
        0x003e_0d80,
        ee::s14::f_003e0d80
    );

    avtest_gated_fake!(
        avtest_fake_async_load,
        AVTEST_PREV_ASYNC_LOAD,
        0x003d_f690,
        ee::s14::f_003df690
    );

    /// Install the stream/async fakes once per process (later calls no-op).
    pub(crate) fn avplayer_hook_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E06D8),
            avtest_fake_stream_create,
            &AVTEST_PREV_STREAM_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E0BF8),
            avtest_fake_get_channel,
            &AVTEST_PREV_GET_CHANNEL,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E0AD8),
            avtest_fake_set_priority,
            &AVTEST_PREV_SET_PRIORITY,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E0948),
            avtest_fake_set_channel_filter,
            &AVTEST_PREV_SET_CHANNEL_FILTER,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E0C88),
            avtest_fake_queue_range,
            &AVTEST_PREV_QUEUE_RANGE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E0D80),
            avtest_fake_queue_file,
            &AVTEST_PREV_QUEUE_FILE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF690),
            avtest_fake_async_load,
            &AVTEST_PREV_ASYNC_LOAD,
        );
    }

    /// The player struct under test: 16-aligned (the memset cycle count
    /// assumes it) at a nonzero test address.
    pub(crate) const AVPLAYER_TEST: u32 = DATA + 0x100;

    /// Fixture for av_player_init: the struct prefilled nonzero, every init
    /// branch steered by an independent seed digit. EA_MemSet and
    /// EA_AssertNotBusy stay real (short bounded paths); the allocator
    /// vtable word points at the miss-hook fake; the stream/async callees
    /// hit the gated fakes above; the RAM comparison checks every stored
    /// field, and the two all-real arms pin exact cycle totals.
    pub(crate) fn avplayer_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x1403, 0);
        let snd_on = seed % 2;
        let alloc = seed / 2 % 3;
        let has_audio = seed / 6 % 2;
        let flag = seed / 12 % 3;
        let ty = seed / 36 % 4;
        for i in 0..0x78 / 4 {
            c.bus
                .write32(AVPLAYER_TEST + 4 * i as u32, r.next_u32() | 1);
        }
        for (i, off) in [
            AV_PLAYER_A1,
            AV_PLAYER_A4,
            AV_PLAYER_ALLOC_A,
            AV_PLAYER_ALLOC_B,
            AV_PLAYER_HAS_AUDIO,
            AV_PLAYER_TYPE,
            AV_PLAYER_SND_FLAG,
            AV_PLAYER_ASYNC_A,
            AV_PLAYER_ASYNC_B,
            AV_PLAYER_STREAM_A,
            AV_PLAYER_STREAM_B,
            AV_PLAYER_FILE_A,
            AV_PLAYER_FILE_B,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus
                .write32(AVPLAYER_TEST + off, 0x5A5A_5A00 + i as u32 + 1);
        }
        // SND on/off: `SND_IsInitialized` samples `lb(0x510000 - 0x55a4)`.
        // The sound flag at +0x1c is a8, or forced to 1 while SND is down.
        c.bus.write8(0x0050_aa5c, if snd_on == 0 { 0 } else { 1 });
        // Allocator object absent / present-but-idle / present.
        c.bus
            .write32(0x0050_9430, if alloc == 0 { 0 } else { fakes::FAKE });
        c.bus
            .write32(0x0050_9434, if alloc == 2 { 0x1234_5678 } else { 0 });
        c.g[A0][0] = AVPLAYER_TEST as u64;
        c.g[A1][0] = (r.next_u32() | 1) as u64;
        c.g[A2][0] = (r.next_u32() | 1) as u64;
        c.g[A3][0] = (r.next_u32() | 1) as u64;
        c.g[T0][0] = if has_audio == 0 {
            0
        } else {
            (r.next_u32() | 1) as u64
        };
        c.g[T1][0] = (r.next_u32() | 1) as u64;
        c.g[T2][0] = (r.next_u32() | 1) as u64;
        c.g[T3][0] = ty as u64;
        let a8 = match flag {
            0 => 0,
            1 => 1,
            _ => 0x1A2B_3C4D,
        };
        // The eighth argument lives at the caller's sp (the port opens a
        // 0xb0 frame first, then reads it back at sp+0xb0).
        c.bus.write32(c.g[SP][0] as u32, a8);
        c.bus.write32(AVTEST_CTRL, AVTEST_MAGIC);
    }

    /// AV_Player_Init (0x3adec8): the struct ends up identical on both sides
    /// on every arm (checked word for word through dirty-page RAM
    /// comparison). Arms 161 (no allocator object) and 164 (idle allocator)
    /// stay fully real, so their totals are exact; the stream arms run
    /// through the gated fakes (whose cost varies), so they are
    /// equality-checked but not path-listed.
    #[test]
    fn av_player_init_matches() {
        avplayer_hook_fakes();
        // Equality on every arm (300 seeds over the five steering digits).
        check(
            ee::s13::f_003adec8__generated,
            av_player_init,
            300,
            &avplayer_setup,
        );
        // The all-real early exits have exact totals: 160 (no allocator
        // object) and 163 (idle allocator). The stream arms run through
        // call-logging fakes whose `3 + (h & 7)` cost varies per seed, so
        // their totals scatter and are equality-checked above instead.
        require_paths(
            ee::s13::f_003adec8__generated,
            300,
            &[160, 163],
            &avplayer_setup,
        );
        // Arm-distinguishing RAM asserts on the port, seed by seed.
        for seed in 0..300u32 {
            let mut c = crate::difftest::fresh();
            avplayer_setup(&mut c, seed);
            let snd = seed % 2;
            let alloc = seed / 2 % 3;
            let audio = seed / 6 % 2;
            let flag = seed / 12 % 3;
            let ty = seed / 36 % 4;
            let a8 = match flag {
                0 => 0,
                1 => 1,
                _ => 0x1A2B_3C4D,
            };
            av_player_init(&mut c);
            let p = AVPLAYER_TEST;
            // SND down forces the flag to 1; SND up keeps a8.
            assert_eq!(
                c.bus.read32(p + 0x1c),
                if snd == 0 { 1 } else { a8 },
                "seed {seed} +0x1c"
            );
            // The audio flag (a4) is recorded at +0x10 and tested for the
            // no-audio path below.
            if audio == 0 {
                assert_eq!(c.bus.read32(p + 0x10), 0, "seed {seed} +0x10");
            } else {
                assert_ne!(c.bus.read32(p + 0x10), 0, "seed {seed} +0x10");
            }
            if alloc == 2 {
                // Allocator present: both stream pairs are built.
                assert_ne!(c.bus.read32(p + 0x30), 0, "seed {seed} +0x30");
                assert_ne!(c.bus.read32(p + 0x34), 0, "seed {seed} +0x34");
                // The type digit selects the queue pair; the call log shows
                // which one ran.
                let mut targets = Vec::new();
                let mut addr = crate::difftest::fakes::LOG_START;
                for _ in 0..16 {
                    let t = c.bus.read32(addr);
                    if t == 0 {
                        break;
                    }
                    targets.push(t);
                    addr += 0x220;
                }
                match ty {
                    0 => assert!(
                        targets.contains(&0x003e_0c88),
                        "seed {seed} queue-range {targets:x?}"
                    ),
                    1 => assert!(
                        targets.contains(&0x003d_f690),
                        "seed {seed} async-load {targets:x?}"
                    ),
                    2 => assert!(
                        targets.contains(&0x003e_0d80),
                        "seed {seed} queue-file {targets:x?}"
                    ),
                    _ => assert!(
                        !targets.contains(&0x003e_0c88)
                            && !targets.contains(&0x003d_f690)
                            && !targets.contains(&0x003e_0d80),
                        "seed {seed} no queue {targets:x?}"
                    ),
                }
            } else {
                // No/idle allocator: the early exit builds no streams.
                assert_eq!(c.bus.read32(p + 0x30), 0, "seed {seed} +0x30");
                assert_eq!(c.bus.read32(p + 0x34), 0, "seed {seed} +0x34");
                assert_eq!(c.bus.read32(p + 0x38), 0, "seed {seed} +0x38");
                assert_eq!(c.bus.read32(p + 0x3c), 0, "seed {seed} +0x3c");
            }
        }
    }

    /// The subtitle list struct, text buffer and record array for the
    /// parser test. The record array is the forced result of the one
    /// allocator call (first miss-hook call, index 0).
    pub(crate) const AVPARSE_LIST: u32 = DATA + 0x200;

    pub(crate) const AVPARSE_TEXT: u32 = DATA + 0x1000;

    pub(crate) const AVPARSE_RECORDS: u32 = DATA + 0x6_0000;

    /// Real-shaped subtitle text by shape: empty, one full entry, many
    /// entries, overlong lines, token-free junk. Numbers vary with the seed
    /// (frames/timings); every buffer is NUL-terminated past its length.
    pub(crate) fn avparse_subtitle_text(shape: u32, seed: u32) -> Vec<u8> {
        match shape {
            0 => Vec::new(),
            1 => format!(
                "FRAME\t{}\nSTRINGID\t{}\nX\t{}\nY\t{}\nWIDTH\t{}\nHEIGHT\t{}\nJUSTIFY_X\t0\nJUSTIFY_Y\t0\nSHAPE\t1\nA\t255\nR\t{}\nG\t{}\nB\t{}\n",
                seed % 100000,
                seed % 997,
                100 + seed % 400,
                50 + seed % 300,
                200 + seed % 200,
                60,
                (seed % 256) as u8,
                ((seed / 7) % 256) as u8,
                ((seed / 13) % 256) as u8,
            )
            .into_bytes(),
            2 => {
                let mut v = Vec::new();
                for i in 0..8u32 {
                    v.extend_from_slice(
                        format!("FRAME\t{}\nSTRINGID\t{}\n", seed % 50000 + i, i)
                            .as_bytes(),
                    );
                }
                v
            }
            3 => {
                let mut v = b"FRAME\t".to_vec();
                v.extend(std::iter::repeat(b'9').take(200));
                v.extend_from_slice(b"\nX\t");
                v.extend(std::iter::repeat(b'Z').take(250));
                v.extend_from_slice(b"\nSTRINGID\t7\n");
                v
            }
            _ => b"hello world\n\t \n\x01\x02\tend\nno keywords here\n".to_vec(),
        }
    }

    /// Fixture for av_parse_subtitles: game image for the token table, jump
    /// table and strlen targets; bounded NUL-terminated text; a writable
    /// record array sized for the entry counts above. Every loop in the
    /// parser is bounded by the text length or the entry count, so every
    /// shape terminates on both sides. The RAM comparison checks the parsed
    /// tables (record count, per-record fields, strings), not only the
    /// return value.
    pub(crate) fn avparse_subtitle_setup(c: &mut Cpu, seed: u32) {
        fakes::machine(c, seed, 0x03af, 0);
        elf_image(c);
        let text = avparse_subtitle_text(seed % 5, seed);
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(AVPARSE_TEXT + i as u32, *b);
        }
        c.bus.write8(AVPARSE_TEXT + text.len() as u32, 0);
        for i in 0..64 * SUBTITLE_RECORD / 4 {
            c.bus.write32(AVPARSE_RECORDS + 4 * i as u32, 0xDEAD_BEEF);
        }
        c.bus.write32(fakes::LOG_RESULTS, AVPARSE_RECORDS);
        c.bus.write32(AVPARSE_LIST, 0xA5A5_A5A5);
        c.bus.write32(AVPARSE_LIST + 4, 0x5A5A_5A5A);
        c.bus.write32(0x0050_9430, fakes::FAKE);
        c.g[A0][0] = AVPARSE_LIST as u64;
        c.g[A1][0] = AVPARSE_TEXT as u64;
        c.g[A2][0] = text.len() as u64;
        c.pc = 0x003a_f7f0;
    }

    #[test]
    fn av_parse_subtitles_matches() {
        check(
            ee::s13::f_003af7f0__generated,
            av_parse_subtitles,
            300,
            &avparse_subtitle_setup,
        );
    }

    /// Event delivery without unwinds: a Deliver-only schedule on a running
    /// guest thread, so every loop back edge (top-level and nested) delivers
    /// without ever unwinding. Unwind seeds are deliberately excluded here:
    /// an unwind inside a nested `AV_FindToken`/`AV_ParseJustify` call leaves
    /// `drive` re-entering the top function at the nested head, which the
    /// top-level `match pc` cannot resume (the gccrt-fix-1 precedent for
    /// `check_eh_spec`). Live unwinds land only in the shape-0 test below;
    /// every used-form head is also entered directly by the resume test, and
    /// the nested functions carry their own driven Unwind tests below.
    #[test]
    fn av_parse_subtitles_events_matches() {
        driven!(avparse_gen_driven, ee::s13::f_003af7f0__generated);
        driven!(avparse_hand_driven, av_parse_subtitles);
        check_events(avparse_gen_driven, avparse_hand_driven, 96, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x03af_0000 + seed as u64, 0);
            elf_image(c);
            let text = avparse_subtitle_text(seed % 5, seed);
            for (i, b) in text.iter().enumerate() {
                c.bus.write8(AVPARSE_TEXT + i as u32, *b);
            }
            c.bus.write8(AVPARSE_TEXT + text.len() as u32, 0);
            for i in 0..64 * SUBTITLE_RECORD / 4 {
                c.bus.write32(AVPARSE_RECORDS + 4 * i as u32, 0xDEAD_BEEF);
            }
            c.bus.write32(fakes::LOG_RESULTS, AVPARSE_RECORDS);
            c.bus.write32(AVPARSE_LIST, 0xA5A5_A5A5);
            c.bus.write32(AVPARSE_LIST + 4, 0x5A5A_5A5A);
            c.bus.write32(0x0050_9430, fakes::FAKE);
            c.g[A0][0] = AVPARSE_LIST as u64;
            c.g[A1][0] = AVPARSE_TEXT as u64;
            c.g[A2][0] = text.len() as u64;
            c.g[RA][0] = crate::difftest::RA as u64;
            // Stock schedule for the running thread, then force Deliver:
            // no VBlank, so events deliver without ever unwinding.
            let _ = schedule(c, &mut r, seed, 0x003a_f7f0);
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .sched
                .next_vblank = 1 << 40;
            c.next_event = c.cycles + (r.next_u32() % 48) as u64;
            crate::difftest::events::Events::Deliver
        });
    }

    /// The eight loop-event heads of av_parse_subtitles: the back edges the
    /// generated body checks (`c.pc = HEAD; if c.event() { return; }`). Each
    /// entry below is one resume head exercised by direct entry.
    pub(crate) const AVPARSE_RESUME_HEADS: [u32; 8] = [
        0x003a_f8f0, // token-table copy loop
        0x003a_f930, // first token scan
        0x003a_f9cc, // next entry
        0x003a_fa28, // subtitle-record init loop
        0x003a_fa68, // line token scan
        0x003a_faa0, // per-byte token scan
        0x003a_fbf8, // entry line scan
        0x003a_fc40, // end-of-line scan
    ];

    /// Fixture for one direct-entry resume head: the stack frame in place
    /// (token pointers and lengths copied, slots cleared, RA saved), the
    /// list/text/records of the normal fixture, and the registers the arm
    /// reads set to mid-loop values. Terminates on both sides like the
    /// normal fixture (every loop stays bounded by the text or the count).
    pub(crate) fn avparse_resume_setup(c: &mut Cpu, seed: u32, head: u32) {
        let mut r = fakes::machine(c, seed, 0x03af_1000 + head as u64, 0);
        elf_image(c);
        let text = avparse_subtitle_text(1, seed);
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(AVPARSE_TEXT + i as u32, *b);
        }
        c.bus.write8(AVPARSE_TEXT + text.len() as u32, 0);
        for i in 0..64 * SUBTITLE_RECORD / 4 {
            c.bus.write32(AVPARSE_RECORDS + 4 * i as u32, 0xDEAD_BEEF);
        }
        c.bus.write32(fakes::LOG_RESULTS, AVPARSE_RECORDS);
        c.bus.write32(AVPARSE_LIST, AVPARSE_RECORDS + 0x10);
        c.bus.write32(AVPARSE_LIST + 4, 5);
        c.bus.write32(0x0050_9430, fakes::FAKE);
        // Stack frame as the entry arm leaves it.
        let sp = crate::difftest::SP - 0x240;
        c.g[SP][0] = sp as u64;
        for i in 0..0x240 / 4 {
            c.bus.write32(sp + 4 * i as u32, r.next_u32() | 1);
        }
        for i in 0..13u32 {
            let ptr = c.bus.read32(0x0049_5340 + 4 * i);
            c.bus.write32(sp + 4 * i, ptr);
            let mut len = 0u32;
            while c.bus.read8(ptr + len) != 0 {
                len += 1;
            }
            c.bus.write32(sp + 0x40 + 4 * i, len);
        }
        c.bus.write32(sp + 0x154, sp + 0x40);
        let resume_len = c.bus.read32(0x0049_5340 + 0x30);
        c.bus.write32(sp + 0x30, resume_len);
        for (i, slot) in [
            0x118u32, 0x11c, 0x120, 0x124, 0x128, 0x12c, 0x130, 0x134, 0x138, 0x13c, 0x140, 0x144,
            0x148,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus.write32(sp + slot, i as u32);
        }
        for off in [
            0x15cu32, 0x160, 0x164, 0x168, 0x16c, 0x170, 0x174, 0x178, 0x17c, 0x180, 0x184, 0x188,
            0x18c, 0x190, 0x194,
        ] {
            c.bus.write32(sp + off, 0);
        }
        c.bus.write32(sp + 0x14c, 1);
        c.bus.write32(sp + 0x150, 0);
        c.bus.write32(sp + 0x158, 1);
        c.bus.write32(sp + 0x88, 0);
        c.bus.write32(sp + 0x1a0, crate::difftest::RA);
        c.g[RA][0] = crate::difftest::RA as u64;
        // Canonical mid-parse registers; each head below overrides its own.
        c.g[S0][0] = 0;
        c.g[S1][0] = 0;
        c.g[S2][0] = 0;
        c.g[S3][0] = AVPARSE_LIST as u64;
        c.g[S4][0] = text.len() as u64;
        c.g[S5][0] = AVPARSE_TEXT as u64;
        c.g[S6][0] = 0xa;
        c.g[S7][0] = 0;
        c.g[FP][0] = u64::MAX;
        c.g[A0][0] = 0;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        match head {
            0x003a_f8f0 => {
                c.g[S0][0] = 0x0049_5340 + 0x18;
                c.g[S1][0] = 6;
                c.g[S2][0] = (sp + 0x40 + 0x18) as u64;
            }
            0x003a_f930 => {
                c.g[S0][0] = (AVPARSE_TEXT + 2) as u64;
                c.g[S1][0] = 2;
            }
            0x003a_f9cc => {
                c.g[S1][0] = 4;
                c.g[A0][0] = 0x003a_fb14;
            }
            0x003a_fa28 => {
                c.g[S1][0] = (AVPARSE_RECORDS + 0x10) as u64;
                c.g[S0][0] = 1;
                c.g[S6][0] = u64::MAX;
            }
            0x003a_fa68 => {
                c.g[S1][0] = 0;
            }
            0x003a_faa0 => {
                c.g[S1][0] = 2;
                c.g[V0][0] = (AVPARSE_TEXT + 2) as u64;
                c.g[S0][0] = 0;
                c.g[FP][0] = 0;
            }
            0x003a_fbf8 => {
                c.g[S0][0] = 0;
                c.g[S1][0] = 2;
                c.g[V0][0] = (AVPARSE_TEXT + 2) as u64;
            }
            _ => {
                // 0x3afc40: end-of-line scan from the line start.
                c.g[S1][0] = 0;
                c.g[A0][0] = 0;
                c.g[V1][0] = AVPARSE_TEXT as u64;
            }
        }
        c.pc = head;
    }

    /// One direct-entry resume per used-form loop head: the same frame and
    /// register state on both sides, compared to equality (registers,
    /// cycles, RAM). Together with the events test above this pins all
    /// eight back edges with and without delivery.
    #[test]
    fn av_parse_subtitles_resume_matches() {
        for head in AVPARSE_RESUME_HEADS {
            let r = compare(ee::s13::f_003af7f0__generated, av_parse_subtitles, &|c| {
                avparse_resume_setup(c, 0xC0DE, head)
            });
            assert!(r.hard.is_empty(), "head {head:#x}: {:#?}", r.hard);
        }
    }

    /// Token-per-line subtitle entries: line 0 is a FRAME filler (so phase 1
    /// matches and allocates), line 1 holds each of the 13 known tokens, so
    /// the phase-2 forward search records its slot and phase 3 dispatches
    /// its field handler. JUSTIFY_X/Y use real justification names (hit
    /// path); the rest use distinctive numbers checked field by field below.
    pub(crate) const AVPARSE_TOKENS: [&str; 13] = [
        "FRAME",
        "STRINGID",
        "X",
        "Y",
        "WIDTH",
        "HEIGHT",
        "JUSTIFY_X",
        "JUSTIFY_Y",
        "SHAPE",
        "A",
        "R",
        "G",
        "B",
    ];

    pub(crate) fn avparse_token_first_text(which: usize, seed: u32) -> Vec<u8> {
        let num = 100 + (seed % 400);
        // JUSTIFY_X/Y alternate hit and miss names by seed so both the
        // known-name and default paths run.
        let val_of = |tok: &str| match tok {
            "JUSTIFY_X" if seed % 2 == 0 => "left".to_string(),
            "JUSTIFY_Y" if seed % 2 == 1 => "TOP".to_string(),
            "JUSTIFY_X" | "JUSTIFY_Y" => "BOGUS".to_string(),
            _ => format!("{num}"),
        };
        // Line 0 is FRAME (phase-1 match); line 1 is token `which`.
        let mut v = Vec::new();
        v.extend_from_slice(format!("FRAME\t{num}\n").as_bytes());
        v.extend_from_slice(
            format!(
                "{}\t{}\n",
                AVPARSE_TOKENS[which],
                val_of(AVPARSE_TOKENS[which])
            )
            .as_bytes(),
        );
        v
    }

    pub(crate) fn avparse_token_first_setup(c: &mut Cpu, seed: u32) {
        fakes::machine(c, seed, 0x03af, 0);
        elf_image(c);
        // which = seed % 13 rotates the line-1 token through all 13 kinds.
        let text = avparse_token_first_text(seed as usize % 13, seed);
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(AVPARSE_TEXT + i as u32, *b);
        }
        c.bus.write8(AVPARSE_TEXT + text.len() as u32, 0);
        for i in 0..64 * SUBTITLE_RECORD / 4 {
            c.bus.write32(AVPARSE_RECORDS + 4 * i as u32, 0xDEAD_BEEF);
        }
        c.bus.write32(fakes::LOG_RESULTS, AVPARSE_RECORDS);
        c.bus.write32(AVPARSE_LIST, 0xA5A5_A5A5);
        c.bus.write32(AVPARSE_LIST + 4, 0x5A5A_5A5A);
        c.bus.write32(0x0050_9430, fakes::FAKE);
        c.g[A0][0] = AVPARSE_LIST as u64;
        c.g[A1][0] = AVPARSE_TEXT as u64;
        c.g[A2][0] = text.len() as u64;
        c.pc = 0x003a_f7f0;
    }

    #[test]
    fn av_parse_subtitles_token_first_matches() {
        check(
            ee::s13::f_003af7f0__generated,
            av_parse_subtitles,
            130,
            &avparse_token_first_setup,
        );
    }

    /// Field-by-field proof that the token-first entries dispatch the right
    /// handler: record 0 carries line 1's parsed value in its token's
    /// field (INIT defaults are 32/350/600/50/-1/-1/2/2/0 with zero colors).
    #[test]
    fn av_parse_subtitles_token_first_fields() {
        // (token index, record word, expected value). JUSTIFY_X hits on
        // even seeds (left=3), JUSTIFY_Y on odd seeds (TOP=1); other seeds
        // take the default path.
        for k in 0..13usize {
            let seed = match k {
                6 => 0u32,
                7 => 1u32,
                _ => 1u32,
            };
            let mut c = crate::difftest::fresh();
            avparse_subtitle_setup(&mut c, seed);
            let text = avparse_token_first_text(k, seed);
            for (i, b) in text.iter().enumerate() {
                c.bus.write8(AVPARSE_TEXT + i as u32, *b);
            }
            c.bus.write8(AVPARSE_TEXT + text.len() as u32, 0);
            c.g[A2][0] = text.len() as u64;
            av_parse_subtitles(&mut c);
            let rec = AVPARSE_RECORDS + 0x10;
            let mut w = |o: u32| c.bus.read32(rec + o);
            let num = 100 + (seed % 400);
            match k {
                0 => assert_eq!(w(0x14), num, "FRAME field"),
                1 => assert_eq!(w(0x10), num, "STRINGID field"),
                2 => assert_eq!(w(0x00), num, "X field"),
                3 => assert_eq!(w(0x04), num, "Y field"),
                4 => assert_eq!(w(0x08), num, "WIDTH field"),
                5 => assert_eq!(w(0x0c), num, "HEIGHT field"),
                6 => assert_eq!(w(0x18), 3, "JUSTIFY_X left"),
                7 => assert_eq!(w(0x1c), 1, "JUSTIFY_Y TOP"),
                8 => assert_eq!(w(0x20), num, "SHAPE field"),
                9 => assert_eq!(c.bus.read8(rec + 0x24), num as u8, "A color byte"),
                10 => assert_eq!(c.bus.read8(rec + 0x25), num as u8, "R color byte"),
                11 => assert_eq!(c.bus.read8(rec + 0x26), num as u8, "G color byte"),
                _ => assert_eq!(c.bus.read8(rec + 0x27), num as u8, "B color byte"),
            }
        }
    }

    /// Boundary shapes: no trailing LF (S1==S4 at the scan ends), a VT byte
    /// (whitespace class 0x0b), a leading LF (entry-scan loop iterates), and
    /// a tab-first line. Each pins a loop bound or byte class the five main
    /// shapes never hit.
    pub(crate) fn avparse_boundary_text(which: u32, seed: u32) -> Vec<u8> {
        let num = 100 + (seed % 400);
        match which % 4 {
            0 => format!("FRAME\t{num}\nX\t{num}").into_bytes(),
            1 => format!("FRAME\t{num}\nX\x0b{num}\n").into_bytes(),
            2 => format!("\nFRAME\t{num}\nX\t{num}\n").into_bytes(),
            _ => format!("\t{num}\nFRAME\t{num}\n").into_bytes(),
        }
    }

    pub(crate) fn avparse_boundary_setup(c: &mut Cpu, seed: u32) {
        fakes::machine(c, seed, 0x03af, 0);
        elf_image(c);
        let text = avparse_boundary_text(seed % 4, seed);
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(AVPARSE_TEXT + i as u32, *b);
        }
        c.bus.write8(AVPARSE_TEXT + text.len() as u32, 0);
        for i in 0..64 * SUBTITLE_RECORD / 4 {
            c.bus.write32(AVPARSE_RECORDS + 4 * i as u32, 0xDEAD_BEEF);
        }
        c.bus.write32(fakes::LOG_RESULTS, AVPARSE_RECORDS);
        c.bus.write32(AVPARSE_LIST, 0xA5A5_A5A5);
        c.bus.write32(AVPARSE_LIST + 4, 0x5A5A_5A5A);
        c.bus.write32(0x0050_9430, fakes::FAKE);
        c.g[A0][0] = AVPARSE_LIST as u64;
        c.g[A1][0] = AVPARSE_TEXT as u64;
        c.g[A2][0] = text.len() as u64;
        c.pc = 0x003a_f7f0;
    }

    #[test]
    fn av_parse_subtitles_boundary_matches() {
        check(
            ee::s13::f_003af7f0__generated,
            av_parse_subtitles,
            120,
            &avparse_boundary_setup,
        );
    }

    /// First unwind on each of the eight loop-event heads: a tuned VBlank
    /// time per head (calibrated: f8f0 ~200, f930 ~1500, fa28 ~2600,
    /// fa68 ~3300 on leading-LF text, faa0 ~3100, f9cc ~3900, fc40 ~4000,
    /// fbf8 ~4500), with the per-head resume count asserted. VBlank is
    /// consumed by the first unwind, so each driven side resumes exactly
    /// once at that head (combined {head: 2}).
    #[test]
    fn av_parse_subtitles_first_unwind_per_head() {
        use crate::difftest::events::{compare_schedule, take_resumes};
        driven!(avparse_gen_first, ee::s13::f_003af7f0__generated);
        driven!(avparse_hand_first, av_parse_subtitles);
        // (head, VBlank offset, leading-LF text?).
        const CASES: [(u32, u64, bool); 8] = [
            (0x003a_f8f0, 200, false),
            (0x003a_f930, 1500, false),
            (0x003a_f9cc, 3900, false),
            (0x003a_fa28, 2600, false),
            (0x003a_fa68, 3300, true),
            (0x003a_faa0, 3100, false),
            (0x003a_fbf8, 4500, false),
            (0x003a_fc40, 4000, false),
        ];
        for (head, toff, leading_lf) in CASES {
            let setup = |c: &mut Cpu| {
                avparse_subtitle_setup(c, 1);
                if leading_lf {
                    c.bus.write8(AVPARSE_TEXT, b'\n');
                    let text = avparse_subtitle_text(1, 1);
                    for (i, b) in text.iter().enumerate() {
                        c.bus.write8(AVPARSE_TEXT + 1 + i as u32, *b);
                    }
                    c.bus.write8(AVPARSE_TEXT + 1 + text.len() as u32, 0);
                    c.g[A2][0] = (text.len() + 1) as u64;
                }
                let saved = (c.g[A0], c.g[A1], c.g[31]);
                ssx3_ps2::kernel::start_main_thread(c, 0x003a_f7f0, 0, 0);
                (c.g[A0], c.g[A1], c.g[31]) = saved;
                c.pc = 0x003a_f7f0;
                c.cycles = 10000;
                c.next_event = 10000;
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .sched
                    .next_vblank = 10000 + toff;
            };
            take_resumes();
            let r = crate::difftest::compare_with(
                avparse_gen_first,
                avparse_hand_first,
                &setup,
                |a, b, hard| {
                    compare_schedule(a, b, hard);
                },
            );
            assert!(r.hard.is_empty(), "head {head:#x}: {:#?}", r.hard);
            let resumes = take_resumes();
            let mut expected = std::collections::BTreeMap::new();
            expected.insert(head, 2);
            assert_eq!(resumes, expected, "head {head:#x}");
        }
    }
    use crate::difftest::require_paths;

    pub(crate) use crate::game::bx::cubicspline::tests::dtor_fixture;
    pub(crate) use crate::game::bx::cubicspline::tests::dtor_list;

    pub(crate) use crate::game::mem::memstd::tests::norm_miss;

    // ---------- PS2_SONY_CODEC_INTERNAL_Open ----------

    pub(crate) fn sony_open_norm_miss(c: &mut Cpu, target: u32) {
        let before = c.cycles;
        fakes::fake_call(c, target);
        c.cycles = before + 4;
    }

    /// Gate of the codec-open fakes below, and the switch that makes the
    /// `InitFrame` fake return null (the `movz` arm of the frame push).
    pub(crate) const SONY_OPEN_CTRL: u32 = DATA + 0x7_e000;

    pub(crate) const SONY_OPEN_MAGIC: u32 = 0x5c0d_ec01;

    pub(crate) const SONY_OPEN_NULL_FRAME: u32 = DATA + 0x7_e004;

    /// A gated fake: with the gate on, a logged fake call at a fixed 4-cycle
    /// cost; otherwise the slot's previous body.
    macro_rules! sony_open_fake {
        ($fake:ident, $prev:ident, $addr:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(SONY_OPEN_CTRL) != SONY_OPEN_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let before = c.cycles;
                fakes::fake_call(c, $addr);
                c.cycles = before + 4;
            }
        };
    }

    sony_open_fake!(
        sony_open_fake_init_buffer,
        SONY_OPEN_PREV_INIT_BUFFER,
        0x003b_1228,
        ee::s13::f_003b1228__generated
    );

    sony_open_fake!(
        sony_open_fake_get_hdr,
        SONY_OPEN_PREV_GET_HDR,
        0x003b_13d8,
        ee::s13::f_003b13d8__generated
    );

    sony_open_fake!(
        sony_open_fake_mpeg_init,
        SONY_OPEN_PREV_MPEG_INIT,
        0x0040_2708,
        ee::s15::f_00402708__generated
    );

    sony_open_fake!(
        sony_open_fake_mpeg_create,
        SONY_OPEN_PREV_MPEG_CREATE,
        0x0040_27b8,
        ee::s15::f_004027b8__generated
    );

    sony_open_fake!(
        sony_open_fake_add_callback,
        SONY_OPEN_PREV_ADD_CALLBACK,
        0x0040_2c08,
        ee::s15::f_00402c08__generated
    );

    pub(crate) static SONY_OPEN_PREV_INIT_FRAME: AtomicUsize = AtomicUsize::new(0);

    /// `PS2_SONY_CODEC_INTERNAL_InitFrame` with the gate on: a logged fake
    /// (4 cycles) returning the fake's pointer, or null when the fixture
    /// asks for it.
    pub(crate) fn sony_open_fake_init_frame(c: &mut Cpu) {
        if c.bus.read32(SONY_OPEN_CTRL) != SONY_OPEN_MAGIC {
            return fakes::fall_back(
                c,
                &SONY_OPEN_PREV_INIT_FRAME,
                ee::s13::f_003b07f8__generated,
            );
        }
        let before = c.cycles;
        fakes::fake_call(c, 0x003b_07f8);
        c.cycles = before + 4;
        if c.bus.read32(SONY_OPEN_NULL_FRAME) != 0 {
            c.g[V0][0] = 0;
        }
    }

    pub(crate) fn sony_open_hook() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B1228),
            sony_open_fake_init_buffer,
            &SONY_OPEN_PREV_INIT_BUFFER,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B13D8),
            sony_open_fake_get_hdr,
            &SONY_OPEN_PREV_GET_HDR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B07F8),
            sony_open_fake_init_frame,
            &SONY_OPEN_PREV_INIT_FRAME,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402708),
            sony_open_fake_mpeg_init,
            &SONY_OPEN_PREV_MPEG_INIT,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004027B8),
            sony_open_fake_mpeg_create,
            &SONY_OPEN_PREV_MPEG_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402C08),
            sony_open_fake_add_callback,
            &SONY_OPEN_PREV_ADD_CALLBACK,
        );
    }

    pub(crate) const SONY_OPEN_CTX: u32 = DATA + 0x5_0000;

    pub(crate) const SONY_OPEN_CFG: u32 = DATA + 0x5_1000;

    pub(crate) const SONY_OPEN_DESC: u32 = DATA + 0x5_2000;

    /// Decoder globals the open writes (prefilled so every store shows).
    pub(crate) const SONY_OPEN_OUT_GLOBALS: [u32; 12] = [
        0x0050_94d4,
        0x0050_94d8,
        0x0050_94dc,
        0x0050_94e0,
        0x0050_94fc,
        0x0050_9500,
        0x0050_9524,
        0x0050_9548,
        0x0050_9584,
        0x0050_958c,
        0x0050_95a0,
        0x0050_a088,
    ];

    /// Codec-open fixture. `seed % 3`: 0 = MPEG-1 (`0x509a68` = 0, the
    /// defaults arm and rows/16 via 0x3b0da8), 1 = MPEG-2 progressive
    /// (`0x509524` != 0, rows/16 via 0x3b0dac), 2 = MPEG-2 interlaced
    /// (`0x509524` = 0, rows/32*2); `seed / 3 % 4` bit 0 = old work buffer
    /// (`+0x2c`), bit 1 = old IPU buffer (`+0x78`); `seed / 12 % 4` frames;
    /// `seed / 48` negative h/v sizes (the `movn` rounding); every fifth seed
    /// `InitFrame` returns null.
    pub(crate) fn sony_open_fixture(c: &mut Cpu, seed: u32) {
        let mode = seed % 3;
        let frees = (seed / 3) % 4;
        let frames = (seed / 12) % 4;
        let negative = (seed / 48) % 2 == 1;
        for i in 0..0x30u32 {
            c.bus.write32(SONY_OPEN_CTX + 4 * i, 0x5555_0000 + i);
        }
        c.bus.write32(
            SONY_OPEN_CTX + 0x2c,
            if frees & 1 != 0 { DATA + 0x5_3000 } else { 0 },
        );
        c.bus.write32(
            SONY_OPEN_CTX + 0x78,
            if frees & 2 != 0 { DATA + 0x5_3100 } else { 0 },
        );
        c.bus.write32(SONY_OPEN_CTX + 0x20, SONY_OPEN_CTX + 0x20);
        c.bus.write32(SONY_OPEN_CTX + 0x24, SONY_OPEN_CTX + 0x20);
        for i in 0..8u32 {
            c.bus.write32(SONY_OPEN_CFG + 4 * i, 0x6666_0000 + i);
        }
        c.bus.write32(SONY_OPEN_CFG + 0x10, frames);
        c.bus.write32(SONY_OPEN_DESC, 0x4d4d_0000 + seed);
        c.bus.write32(SONY_OPEN_DESC + 4, 0x5eed_0000 + seed);
        // The video alloc/free hooks (both the fake) and the alloc flags.
        c.bus.write32(0x0050_9430, FAKE);
        c.bus.write32(0x0050_9434, FAKE);
        c.bus.write32(0x0050_9438, 0x0bad_0000 | seed);
        for (i, a) in SONY_OPEN_OUT_GLOBALS.into_iter().enumerate() {
            c.bus.write32(a, 0x7a7a_0000 + i as u32);
        }
        c.bus
            .write32(0x0050_9a68, if mode == 0 { 0 } else { 1 + seed });
        c.bus
            .write32(0x0050_9524, if mode == 2 { 0 } else { 0x10 + seed });
        let (h, v) = if negative {
            (
                (-(0x21 + (seed % 7) as i32)) as u32,
                (-(0x13 + (seed % 5) as i32)) as u32,
            )
        } else {
            (0x140 + seed, 0xe0 + seed)
        };
        c.bus.write32(0x0050_94f4, h);
        c.bus.write32(0x0050_94f8, v);
        c.bus.write32(SONY_OPEN_CTRL, SONY_OPEN_MAGIC);
        c.bus.write32(SONY_OPEN_NULL_FRAME, (seed % 5 == 4) as u32);
        c.g[A0][0] = SONY_OPEN_CTX as u64;
        c.g[A1][0] = SONY_OPEN_CFG as u64;
        c.g[A2][0] = SONY_OPEN_DESC as u64;
        c.pc = 0x003b_0c58;
    }

    /// `PS2_SONY_CODEC_INTERNAL_Open`: all three header modes, both frees,
    /// 0..3 frames. Totals: 167 + {11 progressive, 13 interlaced, 21 MPEG-1}
    /// + 9 (work free) + 8 (IPU free) + [frames > 0](6 + 30 * frames).
    #[test]
    fn ps2_sony_codec_internal_open_matches() {
        sony_open_hook();
        check_paths(
            ee::s13::f_003b0c58__generated,
            ps2_sony_codec_internal_open,
            96,
            // 178 progressive; 180 interlaced; 187 progressive + work free;
            // 196 MPEG-1 + IPU free; 205 MPEG-1 + both frees; 214 progressive
            // + 1 frame; 274 progressive + 3 frames.
            &[178, 180, 187, 196, 205, 214, 274],
            &|c, seed| {
                machine(c, seed, 0x003b_0c58, 0);
                c.hooks.miss = sony_open_norm_miss;
                sony_open_fixture(c, seed);
            },
        );
        fakes::covered(
            ee::s13::f_003b0c58__generated,
            ps2_sony_codec_internal_open,
            96,
            &|c, seed| {
                machine(c, seed, 0x003b_0c59, 9);
                c.hooks.miss = sony_open_norm_miss;
                sony_open_fixture(c, seed);
            },
        );
    }

    #[test]
    fn ps2_sony_codec_internal_dtor_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x003b_09a0, 0);
            c.hooks.miss = norm_miss;
            dtor_fixture(c, seed);
        };
        check(
            ee::s13::f_003b09a0__generated,
            ps2_sony_codec_internal_dtor,
            72,
            &setup,
        );
        // The 72 seeds are every combination of the five choices; pinned: no
        // lists with each word/self-free mix (43, 51, 60, 68), and both
        // lists two deep with everything freed (235).
        require_paths(
            ee::s13::f_003b09a0__generated,
            72,
            &[43, 51, 60, 68, 235],
            &setup,
        );
    }

    driven!(dtor_gen, ee::s13::f_003b09a0__generated);

    driven!(dtor_hand, ps2_sony_codec_internal_dtor);

    /// The dtor list-teardown loops under delivered events (the freed
    /// frames go through the leaf `FreeFrame`). The first loop's head
    /// (0x3b0a30) is structured: the port must carry on, never resume
    /// there (the second head is covered by
    /// `ps2_sony_codec_internal_dtor_second_loop_carries_on`).
    #[test]
    fn ps2_sony_codec_internal_dtor_events() {
        let resumes = check_events(dtor_gen, dtor_hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x003b_09a0, 0);
            c.hooks.miss = norm_miss;
            // Both lists live (two nodes each), both words freed.
            dtor_fixture(c, 3 + 8 * 2 + 24 * 2 + 4 * (seed % 2));
            schedule(c, &mut r, seed, 0x003b_09a0)
        });
        assert!(
            resumes.get(&0x003b_0a30).is_none(),
            "unwound at structured head 0x3b0a30: {resumes:x?}"
        );
    }

    driven!(dtor_edge2_gen, ee::s13::f_003b09a0__generated);
    driven!(dtor_edge2_hand, ps2_sony_codec_internal_dtor);

    /// The dtor's second loop at 0x3b0a80 never sees an unwind
    /// under `schedule` (delivery fires in the first loop), so this test
    /// lengthens the first list (12 nodes) and sweeps the VBlank past it.
    /// Fails on the `back_edge_unwinds` body.
    #[test]
    fn ps2_sony_codec_internal_dtor_second_loop_carries_on() {
        for k in [400u64, 600] {
            let resumes = check_events(dtor_edge2_gen, dtor_edge2_hand, 4, &|c, seed| {
                machine(c, seed, 0x003b_09a0, 0);
                c.hooks.miss = norm_miss;
                dtor_fixture(c, 3 + 8 * 2 + 24 * 2 + 4 * (seed % 2));
                const CTX: u32 = DATA + 0x100;
                dtor_list(c, CTX + 0x18, DATA + 0x1000, 12);
                dtor_list(c, CTX + 0x20, DATA + 0x1800, 6);
                let saved = (c.g[A0], c.g[A1], c.g[31]);
                ssx3_ps2::kernel::start_main_thread(c, 0x003b_09a0, 0, 0);
                (c.g[A0], c.g[A1], c.g[31]) = saved;
                c.pc = 0x003b_09a0;
                c.cycles = 1000;
                c.next_event = 1000 + k;
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .sched
                    .next_vblank = 1000 + k;
                Events::Unwind
            });
            assert!(
                resumes.get(&0x003b_0a80).is_none(),
                "k={k}: unwound at structured head 0x3b0a80: {resumes:x?}"
            );
        }
    }
    use crate::game::wscript::wscriptman::tests::clear_3;

    /// Base of the scratch subtitle structs. Word-aligned with a nonzero low
    /// byte, so a wrong base or offset changes RAM; every word starts as
    /// garbage the stored value must overwrite exactly once.
    pub(crate) const AV_SUBTITLE_SLOTS: u32 = DATA + 0x1410;

    pub(crate) fn av_subtitle_setup(c: &mut Cpu, seed: u32, slot: u32, value: u32) {
        let mut r = Rng(0x0c40 ^ (seed as u64) ^ ((slot as u64) << 32));
        scramble(c, &mut r);
        for i in 0..16 {
            c.bus
                .write32(AV_SUBTITLE_SLOTS + slot * 0x40 + i * 4, 0xdead_beef);
        }
        c.g[A0][0] = (AV_SUBTITLE_SLOTS + slot * 0x40) as u64;
        c.g[A1][0] = (value ^ r.next_u32()) as u64;
    }

    #[test]
    fn av_subtitle_set_x_matches() {
        check(
            ssx3_ee::s13::f_003b03a8__generated,
            av_subtitle_set_x,
            100,
            &|c, seed| av_subtitle_setup(c, seed, 0, 0x1111_1111),
        );
    }

    #[test]
    fn av_subtitle_set_y_matches() {
        check(
            ssx3_ee::s13::f_003b03b0__generated,
            av_subtitle_set_y,
            100,
            &|c, seed| av_subtitle_setup(c, seed, 1, 0x2222_2222),
        );
    }

    #[test]
    fn av_subtitle_set_width_matches() {
        check(
            ssx3_ee::s13::f_003b03b8__generated,
            av_subtitle_set_width,
            100,
            &|c, seed| av_subtitle_setup(c, seed, 2, 0x3333_3333),
        );
    }

    #[test]
    fn av_subtitle_set_height_matches() {
        check(
            ssx3_ee::s13::f_003b03c0__generated,
            av_subtitle_set_height,
            100,
            &|c, seed| av_subtitle_setup(c, seed, 3, 0x4444_4444),
        );
    }

    #[test]
    fn av_subtitle_set_string_id_matches() {
        check(
            ssx3_ee::s13::f_003b03c8__generated,
            av_subtitle_set_string_id,
            100,
            &|c, seed| av_subtitle_setup(c, seed, 4, 0x5555_5555),
        );
    }

    #[test]
    fn av_subtitle_set_frame_matches() {
        check(
            ssx3_ee::s13::f_003b03d0__generated,
            av_subtitle_set_frame,
            100,
            &|c, seed| av_subtitle_setup(c, seed, 5, 0x6666_6666),
        );
    }

    #[test]
    fn av_subtitle_set_justify_x_matches() {
        check(
            ssx3_ee::s13::f_003b03d8__generated,
            av_subtitle_set_justify_x,
            100,
            &|c, seed| av_subtitle_setup(c, seed, 6, 0x7777_7777),
        );
    }

    #[test]
    fn av_subtitle_set_justify_y_matches() {
        check(
            ssx3_ee::s13::f_003b03e0__generated,
            av_subtitle_set_justify_y,
            100,
            &|c, seed| av_subtitle_setup(c, seed, 7, 0x1111_8888),
        );
    }

    #[test]
    fn av_subtitle_set_shape_matches() {
        check(
            ssx3_ee::s13::f_003b03e8__generated,
            av_subtitle_set_shape,
            100,
            &|c, seed| av_subtitle_setup(c, seed, 8, 0x2222_9999),
        );
    }

    #[test]
    fn av_subtitle_set_color0_matches() {
        check(
            ssx3_ee::s13::f_003b03f0__generated,
            av_subtitle_set_color0,
            100,
            &|c, seed| av_subtitle_setup(c, seed, 9, 0xaa),
        );
    }

    #[test]
    fn av_subtitle_set_color1_matches() {
        check(
            ssx3_ee::s13::f_003b03f8__generated,
            av_subtitle_set_color1,
            100,
            &|c, seed| av_subtitle_setup(c, seed, 10, 0xbb),
        );
    }

    #[test]
    fn av_subtitle_set_color2_matches() {
        check(
            ssx3_ee::s13::f_003b0400__generated,
            av_subtitle_set_color2,
            100,
            &|c, seed| av_subtitle_setup(c, seed, 11, 0xcc),
        );
    }

    #[test]
    fn av_subtitle_set_color3_matches() {
        check(
            ssx3_ee::s13::f_003b0408__generated,
            av_subtitle_set_color3,
            100,
            &|c, seed| av_subtitle_setup(c, seed, 12, 0xdd),
        );
    }

    /// Player base for the ctor tests: 16-aligned, nonzero low byte, cleared
    /// like the sibling ctor_2 test (`AV_Player_Init` dereferences it).
    pub(crate) const AVPLAYER_THIS: u32 = DATA + 0x1410;

    #[test]
    fn av_player_ctor_matches() {
        check(
            ee::s13::f_003addc0__generated,
            av_player_ctor,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3add_c0, 0);
                alloc_ready(c);
                clear_3(c, AVPLAYER_THIS, 0x100);
                c.g[A0][0] = AVPLAYER_THIS as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A2][0] = if seed / 2 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A3][0] = if seed / 4 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T0][0] = if seed / 8 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    #[test]
    fn mpeg2_error_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = Rng(0x003b_11e8 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            scramble(c, &mut r);
        };
        check(ee::s13::f_003b11e8__generated, mpeg2_error, 200, setup);
        require_paths(ee::s13::f_003b11e8__generated, 200, &[2], setup);
    }

    #[test]
    fn mpeg2_user_data_matches() {
        let stream: &[u8] = &[
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = Rng(0x003b_2380 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            scramble(c, &mut r);
            c.bus.write32(DATA + 0x1000, DATA + 0x2000);
            c.bus.write32(DATA + 0x1004, 0x0000_0000);
            c.bus.write32(DATA + 0x1008, 32);
            for (i, b) in stream.iter().enumerate().skip(4) {
                c.bus.write8(DATA + 0x2000 + (i - 4) as u32, *b);
            }
            let mut w = 0u32;
            for i in 0..4 {
                w = (w << 8) | stream[i] as u32;
            }
            c.bus.write32(DATA + 0x1004, w);
            c.bus.write32(0x51_0000 - 0x5f78, DATA + 0x1000);
            c.pc = 0x003b_2380;
        };
        check(ee::s13::f_003b2380__generated, mpeg2_user_data, 200, setup);
        require_paths(ee::s13::f_003b2380__generated, 200, &[84], setup);
    }

    /// Bitstream fixture shared by the MPEG2 header/extension ports: the
    /// hardware bit-buffer struct lives at `DATA + 0x1000` (byte pointer to
    /// `DATA + 0x2000`, buffered word + valid-bit count planted by the
    /// caller), the globals the parsers store to (`0x509400..0x510000`) and
    /// the quant-table bytes they load (`0x44c200..0x44c300`) are prefilled
    /// with distinctive non-zero patterns.
    pub(crate) fn mpeg2_bitstream_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x003b_0000 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
        scramble(c, &mut r);
        let mut w = 0x509400u32;
        while w < 0x510000 {
            c.bus.write32(w, 0xa5a5_0000 | (w & 0xffff));
            w += 4;
        }
        let mut b = 0x44c200u32;
        let mut k = seed.wrapping_mul(0x85eb_ca6b).wrapping_add(0xc2b2_ae35);
        while b < 0x44c300 {
            k = k.wrapping_mul(0x85eb_ca6b).wrapping_add(0xc2b2_ae35);
            c.bus.write8(b, ((k >> 13) as u8) | 0x11);
            b += 1;
        }
        c.bus.write32(DATA + 0x1000, DATA + 0x2000);
        c.bus
            .write32(DATA + 0x1004, 0x1234_5678 ^ seed.wrapping_mul(0x1f3b_52d5));
        c.bus.write32(DATA + 0x1008, 0x100);
        c.bus.write32(0x51_0000 - 0x5f78, DATA + 0x1000);
        get_bits_spy_on(c);
    }

    /// Gate word of the `MPEG2_Get_Bits` spy ([`get_bits_spy_on`]); the
    /// call count follows it, the log starts at `+0x10`.
    pub(crate) const GET_BITS_SPY: u32 = 0x0098_0000;
    const GET_BITS_SPY_MAGIC: u32 = 0x5350_5931;
    static GET_BITS_SPY_PREV: AtomicUsize = AtomicUsize::new(0);

    fn get_bits_spy(c: &mut Cpu) {
        if c.bus.read32(GET_BITS_SPY) == GET_BITS_SPY_MAGIC {
            let n = c.bus.read32(GET_BITS_SPY + 4);
            let p = GET_BITS_SPY + 0x10 + (n % 0x1000) * 0x18;
            for (i, r) in [S0, S1, S2, SP, RA, A0].into_iter().enumerate() {
                c.bus.write32(p + 4 * i as u32, c.g[r][0] as u32);
            }
            c.bus.write32(GET_BITS_SPY + 4, n + 1);
        }
        fakes::fall_back(c, &GET_BITS_SPY_PREV, ee::s13::f_003b1300__generated)
    }

    /// Log `s0`-`s2`, `sp`, `ra` and `a0` at every `MPEG2_Get_Bits` entry
    /// (then run the real body), so a register the callee saves on its
    /// stack is compared at each call even when a later call overwrites
    /// that stack word with the same value on both sides.
    pub(crate) fn get_bits_spy_on(c: &mut Cpu) {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B1300),
            get_bits_spy,
            &GET_BITS_SPY_PREV,
        );
        c.bus.write32(GET_BITS_SPY, GET_BITS_SPY_MAGIC);
    }

    /// Stream-driven fixture: the whole stream is the byte list (MSB-first),
    /// with the first 4 bytes preloaded as the buffered word and the valid
    /// count at 32, so the first start-code scan starts byte-aligned. The
    /// decoder struct past the bit buffer (the matrices at `+0x10..+0x410`
    /// and the flags after them) is prefilled distinctive non-zero, and the
    /// game image supplies the zigzag and default-matrix tables.
    pub(crate) fn mpeg2_stream_setup(c: &mut Cpu, seed: u32, bytes: &[u8]) {
        mpeg2_bitstream_setup(c, seed);
        assert!(bytes.len() >= 4);
        let mut w = 0u32;
        for i in 0..4 {
            w = (w << 8) | bytes[i] as u32;
        }
        c.bus.write32(DATA + 0x1004, w);
        c.bus.write32(DATA + 0x1008, 32);
        for (i, b) in bytes.iter().enumerate().skip(4) {
            c.bus.write8(DATA + 0x2000 + (i - 4) as u32, *b);
        }
        let mut o = 0x1010u32;
        while o < 0x1800 {
            c.bus.write32(DATA + o, 0x5a5a_0000 | o);
            o += 4;
        }
        mpeg2_game_tables(c);
    }

    /// MSB-first bit packer for stream fixtures: `(width, value)` fields,
    /// zero-padded to a byte, then `tail` (a start code the parser's
    /// extension scan stops at) and 16 zero bytes of slack.
    pub(crate) fn mpeg2_bits(fields: &[(u32, u32)], tail: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let (mut acc, mut n) = (0u64, 0u32);
        for &(w, v) in fields {
            acc = acc << w | (v as u64 & ((1u64 << w) - 1));
            n += w;
            while n >= 8 {
                out.push((acc >> (n - 8)) as u8);
                n -= 8;
            }
        }
        if n > 0 {
            out.push((acc << (8 - n)) as u8);
        }
        out.extend_from_slice(tail);
        out.extend_from_slice(&[0; 16]);
        out
    }

    /// The real game tables the header parsers read (zigzag scan at
    /// `0x44c218`, default intra matrix at `0x44c298`), over the
    /// distinctive fill of [`mpeg2_bitstream_setup`].
    pub(crate) fn mpeg2_game_tables(c: &mut Cpu) {
        let keep: Vec<u32> = (0x509400u32..0x510000)
            .step_by(4)
            .map(|w| c.bus.read32(w))
            .collect();
        elf_image(c);
        for (i, w) in (0x509400u32..0x510000).step_by(4).enumerate() {
            c.bus.write32(w, keep[i]);
        }
    }

    /// Refill rows for the header leaves: a seed-hashed stream (MSB first)
    /// with `count` valid bits in the buffered word and the bits past them
    /// zero, so `MPEG2_Flush_Buffer` refills inside the leaf and leaves the
    /// buffered word in `A0`. `fill` replaces every stream byte when given
    /// (`0x00`/`0xff` steer the leaves' flag arms).
    pub(crate) fn mpeg2_refill_setup(c: &mut Cpu, seed: u32, count: u32, fill: Option<u8>) {
        assert!((25..=32).contains(&count));
        let mut r = Rng((0x003b_1290 ^ (seed as u64).wrapping_mul(0x9e37_79b9)) | 1);
        let bytes: Vec<u8> = (0..48)
            .map(|_| fill.unwrap_or_else(|| r.next_u32() as u8))
            .collect();
        mpeg2_stream_setup(c, seed, &bytes);
        let w = c.bus.read32(DATA + 0x1004);
        c.bus
            .write32(DATA + 0x1004, w & (u64::MAX << (32 - count)) as u32);
        c.bus.write32(DATA + 0x1008, count);
    }

    /// Every row of `setup` refills in `generated`: the byte pointer at
    /// `DATA + 0x1000` ends past the stream start.
    pub(crate) fn require_refill(
        generated: ssx3_ee_rt::funcs::GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, generated);
            assert!(
                c.bus.read32(DATA + 0x1000) > DATA + 0x2000,
                "seed {seed}: no refill"
            );
        }
    }

    #[test]
    fn mpeg2_get_bits1_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_bitstream_setup(c, seed);
            // Top bit steered by seed: 1 on even seeds, 0 on odd.
            c.bus.write32(
                DATA + 0x1004,
                if seed % 2 == 0 {
                    0x8000_0001
                } else {
                    0x7fff_fffe
                },
            );
            c.bus.write32(DATA + 0x1008, 0x100);
        };
        check(ee::s13::f_003b1270__generated, mpeg2_get_bits1, 200, setup);
        require_paths(ee::s13::f_003b1270__generated, 200, &[41], setup);
        // Refill rows: a 1-bit read with the count at 25 refills, so the
        // buffered word comes from the stream; top bit steered by seed.
        let refill: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_refill_setup(c, seed, 25, Some(if seed % 2 == 0 { 0x80 } else { 0x7f }));
        };
        check(ee::s13::f_003b1270__generated, mpeg2_get_bits1, 64, refill);
        require_paths(ee::s13::f_003b1270__generated, 64, &[55], refill);
        require_refill(ee::s13::f_003b1270__generated, 64, refill);
    }

    #[test]
    fn mpeg2_marker_bit_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_bitstream_setup(c, seed);
            c.bus.write32(
                DATA + 0x1004,
                if seed % 2 == 0 {
                    0x8000_0000
                } else {
                    0x0000_0000
                },
            );
            c.bus.write32(DATA + 0x1008, 0x100);
        };
        check(ee::s13::f_003b2360__generated, mpeg2_marker_bit, 200, setup);
        require_paths(ee::s13::f_003b2360__generated, 200, &[41], setup);
        // Refill rows: a 1-bit read with the count at 25 refills, so the
        // buffered word comes from the stream; top bit steered by seed.
        let refill: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_refill_setup(c, seed, 25, Some(if seed % 2 == 0 { 0x80 } else { 0x00 }));
        };
        check(ee::s13::f_003b2360__generated, mpeg2_marker_bit, 64, refill);
        require_paths(ee::s13::f_003b2360__generated, 64, &[55], refill);
        require_refill(ee::s13::f_003b2360__generated, 64, refill);
    }

    #[test]
    fn mpeg2_copyright_extension_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_bitstream_setup(c, seed);
            // All-zero vs all-one stream: every stored field flips.
            let fill = if seed % 2 == 0 {
                0x0000_0000
            } else {
                0xffff_ffff
            };
            c.bus.write32(DATA + 0x1004, fill);
            c.bus.write32(DATA + 0x1008, 0x100);
        };
        check(
            ee::s13::f_003b23a0__generated,
            mpeg2_copyright_extension,
            200,
            setup,
        );
        require_paths(ee::s13::f_003b23a0__generated, 200, &[401], setup);
        // Refill rows: real stream, count 25-32 from the seed, all-zero,
        // all-one or hashed bytes (`seed % 4`), so the reads refill.
        let refill: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let fill = [Some(0x00), Some(0xff), None, None][(seed % 4) as usize];
            mpeg2_refill_setup(c, seed, 25 + seed / 4 % 8, fill);
        };
        check(
            ee::s13::f_003b23a0__generated,
            mpeg2_copyright_extension,
            64,
            refill,
        );
        require_paths(
            ee::s13::f_003b23a0__generated,
            64,
            &[531, 533, 543, 545],
            refill,
        );
        require_refill(ee::s13::f_003b23a0__generated, 64, refill);
    }

    #[test]
    fn mpeg2_sequence_display_extension_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_bitstream_setup(c, seed);
            // Colour-description flag is the first bit read after the
            // 3-bit format: all-zero skips the size triple, all-one runs it.
            let fill = if seed % 2 == 0 {
                0x0000_0000
            } else {
                0xffff_ffff
            };
            c.bus.write32(DATA + 0x1004, fill);
            c.bus.write32(DATA + 0x1008, 0x100);
        };
        check(
            ee::s13::f_003b1cc0__generated,
            mpeg2_sequence_display_extension,
            200,
            setup,
        );
        require_paths(ee::s13::f_003b1cc0__generated, 200, &[202, 316], setup);
        // Refill rows: real stream, count 25-32 from the seed, all-zero,
        // all-one or hashed bytes (`seed % 4`), so the reads refill.
        let refill: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let fill = [Some(0x00), Some(0xff), None, None][(seed % 4) as usize];
            mpeg2_refill_setup(c, seed, 25 + seed / 4 % 8, fill);
        };
        check(
            ee::s13::f_003b1cc0__generated,
            mpeg2_sequence_display_extension,
            64,
            refill,
        );
        require_paths(
            ee::s13::f_003b1cc0__generated,
            64,
            &[254, 256, 258, 268, 410, 412, 414, 424],
            refill,
        );
        require_refill(ee::s13::f_003b1cc0__generated, 64, refill);
    }

    #[test]
    fn mpeg2_sequence_scalable_extension_matches() {
        // A real stream with the valid count at 32: the reads are 2, 4, 14,
        // marker, 14 and four 5-bit fields, so the 14-bit reads and the
        // first and third 5-bit reads refill, and the fourth 5-bit call
        // follows a refill (its `A0` load matters). Scalable mode in the
        // first 2 bits, steered by seed: mode 1 (+1 stored) takes the long
        // tail, 0 and 2 take the plain tail, and 3 reports through
        // MPEG2_Error.
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let h = seed.wrapping_mul(0x9e37_79b9) ^ 0x5bd1_e995;
            let mode = [1u32, 0, 3, 2][(seed % 4) as usize];
            let f = [
                (2, mode),
                (4, h),
                (14, h >> 4),
                (1, 1),
                (14, h >> 18),
                (5, h >> 7),
                (5, h >> 12),
                (5, h >> 17),
                (5, h >> 22),
                (16, h.rotate_left(11)),
                (16, h.rotate_left(23)),
            ];
            mpeg2_stream_setup(c, seed, &mpeg2_bits(&f, &[]));
        };
        check(
            ee::s13::f_003b1ee0__generated,
            mpeg2_sequence_scalable_extension,
            200,
            setup,
        );
        require_paths(ee::s13::f_003b1ee0__generated, 200, &[93, 99, 446], setup);
        // The plain tail and the error arm read 6 bits and never refill;
        // every long-tail row (seed % 4 == 0) does.
        require_refill(ee::s13::f_003b1ee0__generated, 50, &|c, seed| {
            setup(c, seed * 4)
        });
    }

    #[test]
    fn mpeg2_picture_coding_extension_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_bitstream_setup(c, seed);
            // The composite-display flag is the last bit read: all-zero
            // skips the extra block, all-one runs it.
            let fill = if seed % 2 == 0 {
                0x0000_0000
            } else {
                0xffff_ffff
            };
            c.bus.write32(DATA + 0x1004, fill);
            c.bus.write32(DATA + 0x1008, 0x100);
        };
        check(
            ee::s13::f_003b20b8__generated,
            mpeg2_picture_coding_extension,
            200,
            setup,
        );
        require_paths(ee::s13::f_003b20b8__generated, 200, &[617, 807], setup);
        // Refill rows: real stream, count 25-32 from the seed, all-zero,
        // all-one or hashed bytes (`seed % 4`), so the reads refill.
        let refill: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let fill = [Some(0x00), Some(0xff), None, None][(seed % 4) as usize];
            mpeg2_refill_setup(c, seed, 25 + seed / 4 % 8, fill);
        };
        check(
            ee::s13::f_003b20b8__generated,
            mpeg2_picture_coding_extension,
            64,
            refill,
        );
        require_paths(
            ee::s13::f_003b20b8__generated,
            64,
            &[659, 673, 891, 905],
            refill,
        );
        require_refill(ee::s13::f_003b20b8__generated, 64, refill);
    }

    #[test]
    fn mpeg2_slice_header_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_bitstream_setup(c, seed);
            let state = DATA + 0x1000;
            // Matrix-present flag, width class, quant-matrix-valid flag and
            // the second flag word, each steered by its own seed digit.
            c.bus
                .write32(state + 0x420, if seed % 2 == 0 { 0 } else { 0xabcd_1234 });
            c.bus
                .write32(0x51_0000 - 0x6b08, if seed / 2 % 2 == 0 { 0 } else { 3000 });
            c.bus
                .write32(state + 0x424, if seed / 4 % 2 == 0 { 0 } else { 1 });
            c.bus.write32(
                state + 0x428,
                if seed / 8 % 2 == 0 { 0 } else { 0x1234_abcd },
            );
            let fill = if seed / 16 % 2 == 0 {
                0x0000_0000
            } else {
                0xffff_ffff
            };
            c.bus.write32(DATA + 0x1004, fill);
            c.bus.write32(DATA + 0x1008, 0x100);
        };
        check(
            ee::s13::f_003b1878__generated,
            mpeg2_slice_header,
            64,
            setup,
        );
        require_paths(
            ee::s13::f_003b1878__generated,
            64,
            &[
                102, 110, 113, 140, 148, 151, 186, 189, 389, 397, 400, 413, 416, 427, 435, 438,
            ],
            setup,
        );
        // Refill rows: the same flag digits on a real stream (count 25-32,
        // zero/one/hashed bytes by `seed / 16`), so the reads refill.
        let refill: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let state = DATA + 0x1000;
            let fill = [Some(0x00), Some(0xff), None, None][(seed / 16 % 4) as usize];
            mpeg2_refill_setup(c, seed, 25 + seed.wrapping_mul(5) % 8, fill);
            c.bus
                .write32(state + 0x420, if seed % 2 == 0 { 0 } else { 0xabcd_1234 });
            c.bus
                .write32(0x51_0000 - 0x6b08, if seed / 2 % 2 == 0 { 0 } else { 3000 });
            c.bus
                .write32(state + 0x424, if seed / 4 % 2 == 0 { 0 } else { 1 });
            c.bus.write32(
                state + 0x428,
                if seed / 8 % 2 == 0 { 0 } else { 0x1234_abcd },
            );
        };
        check(
            ee::s13::f_003b1878__generated,
            mpeg2_slice_header,
            64,
            refill,
        );
        require_paths(
            ee::s13::f_003b1878__generated,
            64,
            &[
                116, 124, 127, 154, 162, 165, 168, 176, 179, 214, 217, 297, 305, 343, 349, 371,
                395, 491, 508, 519, 527, 571, 642, 667, 856, 3379, 3387, 3390, 3401, 3409, 3412,
                3425, 3428, 3447, 3450, 3453,
            ],
            refill,
        );
        require_refill(ee::s13::f_003b1878__generated, 64, refill);
    }

    #[test]
    fn mpeg2_picture_header_matches() {
        // Temporal/vbv layouts: field bits, extra stop bit, zero pad, then
        // a byte-placed 00 00 01 00 tail the extension scan finds.
        let streams: [&[u8]; 3] = [
            &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
            &[
                0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
            &[
                0x00, 0x00, 0x00, 0x1c, 0x40, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
        ];
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_stream_setup(c, seed, streams[(seed % 3) as usize]);
        };
        check(
            ee::s13::f_003b17a8__generated,
            mpeg2_picture_header,
            60,
            setup,
        );
        require_paths(ee::s13::f_003b17a8__generated, 60, &[354, 428], setup);
    }

    #[test]
    fn mpeg2_next_start_code_matches() {
        // Pad variants: the marker sits 8/16/24 bits out, so the scan takes
        // 1/2/3 iterations.
        let streams: [&[u8]; 3] = [
            &[
                0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
            &[
                0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
            &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
        ];
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_stream_setup(c, seed, streams[(seed % 3) as usize]);
            c.pc = 0x003b_1498;
        };
        check(
            ee::s13::f_003b1498__generated,
            mpeg2_next_start_code,
            60,
            setup,
        );
        require_paths(ee::s13::f_003b1498__generated, 60, &[77, 116, 155], setup);
    }

    #[test]
    fn mpeg2_get_hdr_matches() {
        // Four header sequences: SEQ->GOP->PIC (returns 1), GOP->PIC
        // (returns 1), PIC directly (returns 1), sequence end (returns 0).
        let seq_gop_pic: &[u8] = &[
            0x00, 0x00, 0x00, 0x01, 0xb3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let gop_pic: &[u8] = &[
            0x00, 0x00, 0x00, 0x01, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
        ];
        let pic: &[u8] = &[
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let end: &[u8] = &[0x00, 0x00, 0x00, 0x01, 0xb7];
        let streams: [&[u8]; 4] = [seq_gop_pic, gop_pic, pic, end];
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_stream_setup(c, seed, streams[(seed % 4) as usize]);
        };
        check(ee::s13::f_003b13d8__generated, mpeg2_get_hdr, 60, setup);
        require_paths(
            ee::s13::f_003b13d8__generated,
            60,
            &[184, 502, 1151, 3285],
            setup,
        );
    }

    /// A sequence-error code (0x1b4, the last code below the extension
    /// range) ahead of the picture: the scan files it with the other
    /// unknown codes, past the end/GOP tests, and keeps scanning.
    #[test]
    fn mpeg2_get_hdr_error_code() {
        let err_pic: &[u8] = &[
            0x00, 0x00, 0x00, 0x01, 0xb4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_stream_setup(c, seed, err_pic);
        };
        check(ee::s13::f_003b13d8__generated, mpeg2_get_hdr, 8, setup);
        require_paths(ee::s13::f_003b13d8__generated, 8, &[790], setup);
    }

    /// Every picture coding type 1-4 (I, P, B, D) after the temporal
    /// reference: P and B read the forward vector fields, B the backward
    /// ones too, I and D neither; then the extra-bit stop and a picture
    /// start code the extension scan stops at.
    #[test]
    fn mpeg2_picture_header_coding_types() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let pct = 1 + seed % 4;
            let h = seed.wrapping_mul(0x9e37_79b9);
            let mut f = vec![(10, h >> 7), (3, pct), (16, h >> 13)];
            if pct == 2 || pct == 3 {
                f.push((1, h >> 3));
                f.push((3, 1 + h % 7));
            }
            if pct == 3 {
                f.push((1, h >> 5));
                f.push((3, 1 + (h >> 9) % 7));
            }
            f.push((1, 0));
            let bytes = mpeg2_bits(&f, &[0x00, 0x00, 0x01, 0x00]);
            mpeg2_stream_setup(c, seed, &bytes);
        };
        check(
            ee::s13::f_003b17a8__generated,
            mpeg2_picture_header,
            32,
            setup,
        );
        let mut seen = std::collections::BTreeMap::new();
        for seed in 0..4 {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s13::f_003b17a8__generated);
            seen.insert(1 + seed % 4, c.cycles - before);
        }
        // I and D read no vector fields; P reads one pair, B two.
        assert_eq!(seen[&1], seen[&4], "I vs D: {seen:?}");
        assert!(seen[&2] > seen[&1] && seen[&3] > seen[&2], "{seen:?}");
    }

    /// Vertical size 2800 and 2801 around the slice header's 2800 limit:
    /// above it the slice vertical position extension (3 bits) is read.
    #[test]
    fn mpeg2_slice_header_size_limit() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_bitstream_setup(c, seed);
            let state = DATA + 0x1000;
            c.bus.write32(state + 0x420, 0xabcd_1234);
            c.bus.write32(0x51_0000 - 0x6b08, 2800 + seed % 2);
            c.bus.write32(state + 0x424, seed / 2 % 2);
            c.bus.write32(state + 0x428, 0);
            c.bus.write32(DATA + 0x1004, 0);
            c.bus.write32(DATA + 0x1008, 0x100);
        };
        check(
            ee::s13::f_003b1878__generated,
            mpeg2_slice_header,
            16,
            setup,
        );
        let total = |seed: u32| {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s13::f_003b1878__generated);
            c.cycles - before
        };
        assert!(total(1) > total(0), "2801 must read the extension bits");
    }

    /// A sequence header after its start code: size, aspect, rate,
    /// bitrate, marker, VBV and constrained fields from seed hashes, the
    /// load-intra flag at bit 62 and the load-non-intra flag after it
    /// steered by `seed % 4`, each loaded matrix 64 non-zero bytes, then a
    /// picture start code the extension scan stops at.
    pub(crate) fn mpeg2_seq_header_stream(seed: u32) -> Vec<u8> {
        let (intra, non_intra) = (seed & 1, seed >> 1 & 1);
        let h = seed.wrapping_mul(0x9e37_79b9) | 0x0101_0101;
        let mut f = vec![
            (12, h >> 4),
            (12, h >> 12),
            (4, h >> 24),
            (4, h >> 28),
            (18, h.rotate_left(7)),
            (1, 1),
            (10, h >> 9),
            (1, h >> 3),
            (1, intra),
        ];
        let mut k = 0u32;
        while k < 64 * intra {
            f.push((8, (seed.wrapping_mul(0x85eb_ca6b) >> k % 24) | 0x11 | k));
            k += 1;
        }
        f.push((1, non_intra));
        let mut k = 0u32;
        while k < 64 * non_intra {
            f.push((
                8,
                (seed.wrapping_mul(0xc2b2_ae35) >> k % 24) | 0x21 | k << 1,
            ));
            k += 1;
        }
        mpeg2_bits(&f, &[0x00, 0x00, 0x01, 0x00])
    }

    /// The four matrix arms (default/streamed intra x default/streamed
    /// non-intra).
    #[test]
    fn mpeg2_sequence_header_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_stream_setup(c, seed, &mpeg2_seq_header_stream(seed));
        };
        check(
            ee::s13::f_003b14f0__generated,
            mpeg2_sequence_header,
            60,
            setup,
        );
        require_paths(
            ee::s13::f_003b14f0__generated,
            60,
            &[1928, 5256, 5258, 8586],
            setup,
        );
    }

    /// A quant-matrix extension after its id nibble: the four load flags
    /// (intra, non-intra, chroma intra, chroma non-intra) from the low four
    /// seed bits, each loaded matrix 64 distinct non-zero bytes (the 64th,
    /// stored in the loop's exit delay slot, included), then a picture
    /// start code.
    pub(crate) fn mpeg2_quant_stream(seed: u32) -> Vec<u8> {
        let mut f = Vec::new();
        let mut m = 0u32;
        while m < 4 {
            let on = seed >> m & 1;
            f.push((1, on));
            let mut k = 0u32;
            while k < 64 * on {
                f.push((
                    8,
                    (seed.wrapping_mul(0x27d4_eb2f) >> (k + m) % 24) | 0x41 | k << 1,
                ));
                k += 1;
            }
            m += 1;
        }
        mpeg2_bits(&f, &[0x00, 0x00, 0x01, 0x00])
    }

    /// All 16 load-flag combinations.
    #[test]
    fn mpeg2_quant_matrix_extension_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg2_stream_setup(c, seed, &mpeg2_quant_stream(seed));
        };
        check(
            ee::s13::f_003b1d58__generated,
            mpeg2_quant_matrix_extension,
            64,
            setup,
        );
        require_paths(
            ee::s13::f_003b1d58__generated,
            64,
            &[169, 3949, 4077, 7729, 7857, 7985, 11637, 11765, 15545],
            setup,
        );
    }

    #[test]
    fn mpeg2_extension_and_user_data_matches() {
        // Extension id in the top nibble after the B5 prefix (`seed % 11`):
        // 0 and 6 skip, 1-5 and 7-10 call their parser; every variant then
        // runs the B2 user-data arm and stops at the 00 00 01 00 tail. The
        // per-id totals below show each id took its own arm.
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let id = (seed % 11) as u8;
            let mut bytes = [0u8; 94];
            bytes[1] = 0x00;
            bytes[2] = 0x00;
            bytes[3] = 0x01;
            bytes[4] = 0xb5;
            bytes[5] = id << 4;
            bytes[70] = 0x00;
            bytes[71] = 0x00;
            bytes[72] = 0x01;
            bytes[73] = 0xb2;
            bytes[82] = 0x00;
            bytes[83] = 0x00;
            bytes[84] = 0x01;
            bytes[85] = 0x00;
            mpeg2_stream_setup(c, seed, &bytes);
            c.pc = 0x003b_1988;
        };
        check(
            ee::s13::f_003b1988__generated,
            mpeg2_extension_and_user_data,
            66,
            setup,
        );
        let mut per_id = std::collections::BTreeMap::new();
        for id in 0..11u32 {
            let mut c = crate::difftest::fresh();
            setup(&mut c, id);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s13::f_003b1988__generated);
            per_id.insert(id, c.cycles - before);
        }
        let arms: std::collections::BTreeSet<u64> = per_id.values().copied().collect();
        assert_eq!(arms.len(), 11, "per-id totals {per_id:?}");
        require_paths(
            ee::s13::f_003b1988__generated,
            66,
            &[
                3205, 3591, 3315, 3384, 3354, 3283, 3211, 3406, 3732, 3392, 3225,
            ],
            setup,
        );
    }

    /// [`schedule`] the entry with the event deadline
    /// `step * (seed / 3 / variants % steps)` cycles into the run, so the
    /// event seeds walk the deadline across every loop of the body. With
    /// `unwind` false the unwind seeds only deliver: `drive` re-enters one
    /// call level, and these bodies' callees loop two levels down (the bit
    /// reader's refill under `Get_Bits`).
    fn mpeg2_events_at(
        c: &mut Cpu,
        seed: u32,
        entry: u32,
        (variants, step, steps): (u32, u64, u32),
        unwind: bool,
    ) -> Events {
        let mut r = Rng(0x003b_e000 ^ seed as u64);
        let mut mode = schedule(c, &mut r, seed, entry);
        if mode == Events::Unwind && !unwind {
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .sched
                .next_vblank = 1 << 40;
            mode = Events::Deliver;
        }
        if mode != Events::Off {
            c.next_event = c.cycles + step * (seed / 3 / variants % steps) as u64;
        }
        mode
    }

    driven!(gen_mpeg2_sequence_header, ee::s13::f_003b14f0__generated);
    driven!(hand_mpeg2_sequence_header, mpeg2_sequence_header);

    /// Live events in the two matrix loops of each kind and the window
    /// rotation (loop heads 0x3b15a8, 0x3b15f0, 0x3b1630, 0x3b1670,
    /// 0x3b16a0) and in the bit reader's refill loops.
    #[test]
    fn mpeg2_sequence_header_events() {
        check_events(
            gen_mpeg2_sequence_header,
            hand_mpeg2_sequence_header,
            192,
            &|c, seed| {
                mpeg2_stream_setup(c, seed, &mpeg2_seq_header_stream(seed / 3));
                mpeg2_events_at(c, seed, 0x003b_14f0, (4, 500, 16), false)
            },
        );
    }

    driven!(
        gen_mpeg2_quant_matrix_extension,
        ee::s13::f_003b1d58__generated
    );
    driven!(
        hand_mpeg2_quant_matrix_extension,
        mpeg2_quant_matrix_extension
    );

    /// Live events in the four matrix loops (heads 0x3b1d98, 0x3b1df0,
    /// 0x3b1e48, 0x3b1e98).
    #[test]
    fn mpeg2_quant_matrix_extension_events() {
        check_events(
            gen_mpeg2_quant_matrix_extension,
            hand_mpeg2_quant_matrix_extension,
            384,
            &|c, seed| {
                mpeg2_stream_setup(c, seed, &mpeg2_quant_stream(seed / 3));
                mpeg2_events_at(c, seed, 0x003b_1d58, (16, 1000, 8), false)
            },
        );
    }

    driven!(gen_mpeg2_next_start_code, ee::s13::f_003b1498__generated);
    driven!(hand_mpeg2_next_start_code, mpeg2_next_start_code);

    /// Live events in the byte scan: 4-11 zero bytes before the marker.
    #[test]
    fn mpeg2_next_start_code_events() {
        check_events(
            gen_mpeg2_next_start_code,
            hand_mpeg2_next_start_code,
            192,
            &|c, seed| {
                let mut bytes = vec![0u8; 4 + (seed / 3 % 8) as usize];
                bytes.extend_from_slice(&[0x00, 0x00, 0x01, 0xb5]);
                bytes.extend_from_slice(&[0; 16]);
                mpeg2_stream_setup(c, seed, &bytes);
                mpeg2_events_at(c, seed, 0x003b_1498, (8, 40, 8), true)
            },
        );
    }

    /// The `MPEG2_Get_Hdr` header sequences of [`mpeg2_get_hdr_matches`].
    fn mpeg2_get_hdr_stream(k: u32) -> Vec<u8> {
        let seq_gop_pic: &[u8] = &[
            0x00, 0x00, 0x00, 0x01, 0xb3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let gop_pic: &[u8] = &[
            0x00, 0x00, 0x00, 0x01, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
        ];
        let pic: &[u8] = &[
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let end: &[u8] = &[0x00, 0x00, 0x00, 0x01, 0xb7];
        [seq_gop_pic, gop_pic, pic, end][(k % 4) as usize].to_vec()
    }

    driven!(gen_mpeg2_get_hdr, ee::s13::f_003b13d8__generated);
    driven!(hand_mpeg2_get_hdr, mpeg2_get_hdr);

    /// Live events in the header scan and the parsers it calls.
    #[test]
    fn mpeg2_get_hdr_events() {
        check_events(gen_mpeg2_get_hdr, hand_mpeg2_get_hdr, 192, &|c, seed| {
            mpeg2_stream_setup(c, seed, &mpeg2_get_hdr_stream(seed / 3));
            mpeg2_events_at(c, seed, 0x003b_13d8, (4, 200, 16), false)
        });
    }

    /// The `MPEG2_extension_and_user_data` stream of
    /// [`mpeg2_extension_and_user_data_matches`] for extension `id`.
    fn mpeg2_extension_stream(id: u32) -> [u8; 94] {
        let mut bytes = [0u8; 94];
        bytes[3] = 0x01;
        bytes[4] = 0xb5;
        bytes[5] = (id as u8) << 4;
        bytes[72] = 0x01;
        bytes[73] = 0xb2;
        bytes[84] = 0x01;
        bytes
    }

    driven!(
        gen_mpeg2_extension_and_user_data,
        ee::s13::f_003b1988__generated
    );
    driven!(
        hand_mpeg2_extension_and_user_data,
        mpeg2_extension_and_user_data
    );

    /// Live events in the extension/user-data loop (heads 0x3b1990,
    /// 0x3b19a0, 0x3b19ac) for every extension id.
    #[test]
    fn mpeg2_extension_and_user_data_events() {
        check_events(
            gen_mpeg2_extension_and_user_data,
            hand_mpeg2_extension_and_user_data,
            264,
            &|c, seed| {
                mpeg2_stream_setup(c, seed, &mpeg2_extension_stream(seed / 3 % 11));
                mpeg2_events_at(c, seed, 0x003b_1988, (11, 400, 8), false)
            },
        );
    }

    pub(crate) use crate::game::ui::uivector::tests::leaf_fill;

    /// `cPS2ChunkResolve_v08` (0x003a_dc48) against the generated body.
    #[test]
    fn c_ps2_chunk_resolve_v08_003adc48_matches() {
        check(
            ee::s13::f_003adc48__generated,
            c_ps2_chunk_resolve_v08_003adc48,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3adc48, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2_SONY_CODEC_INTERNAL_v05` (0x003b_0fa0) against the generated body.
    #[test]
    fn ps2_sony_codec_internal_v05_003b0fa0_matches() {
        check(
            ee::s13::f_003b0fa0__generated,
            ps2_sony_codec_internal_v05_003b0fa0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b0fa0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// Gate words of the fakes below (one per fake, in each machine's own RAM).
    pub(crate) const OC1583_MAGIC: u32 = 0x1583_0001;

    pub(crate) const OC1583_STATIC_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC1583_BITS_CTRL: u32 = DATA + 0x20_1000;

    pub(crate) const OC1583_MARKER_CTRL: u32 = DATA + 0x20_2000;

    pub(crate) const OC1583_EXT_CTRL: u32 = DATA + 0x20_3000;

    pub(crate) const OC1583_LOCK_CTRL: u32 = DATA + 0x20_4000;

    pub(crate) const OC1583_UNLOCK_CTRL: u32 = DATA + 0x20_5000;

    /// The GOP first-picture cell (0x510000 - 0x5f78).
    pub(crate) const OC1583_GOP_CELL: u32 = 0x0050_a088;

    pub(crate) static OC1583_PREV_STATIC: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1583_PREV_BITS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1583_PREV_MARKER: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1583_PREV_EXT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1583_PREV_LOCK: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1583_PREV_UNLOCK: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1583_fake_static_init(c: &mut Cpu) {
        if c.bus.read32(OC1583_STATIC_CTRL) != OC1583_MAGIC {
            return fakes::fall_back(c, &OC1583_PREV_STATIC, ee::s13::f_003b0770__generated);
        }
        fakes::fake_call(c, 0x003b_0770);
    }

    pub(crate) fn oc1583_fake_get_bits(c: &mut Cpu) {
        if c.bus.read32(OC1583_BITS_CTRL) != OC1583_MAGIC {
            return fakes::fall_back(c, &OC1583_PREV_BITS, ee::s13::f_003b1300__generated);
        }
        fakes::fake_call(c, 0x003b_1300);
    }

    pub(crate) fn oc1583_fake_marker(c: &mut Cpu) {
        if c.bus.read32(OC1583_MARKER_CTRL) != OC1583_MAGIC {
            return fakes::fall_back(c, &OC1583_PREV_MARKER, ee::s13::f_003b2360__generated);
        }
        fakes::fake_call(c, 0x003b_2360);
    }

    pub(crate) fn oc1583_fake_ext(c: &mut Cpu) {
        if c.bus.read32(OC1583_EXT_CTRL) != OC1583_MAGIC {
            return fakes::fall_back(c, &OC1583_PREV_EXT, ee::s13::f_003b1988__generated);
        }
        fakes::fake_call(c, 0x003b_1988);
    }

    pub(crate) fn oc1583_fake_lock(c: &mut Cpu) {
        if c.bus.read32(OC1583_LOCK_CTRL) != OC1583_MAGIC {
            return fakes::fall_back(c, &OC1583_PREV_LOCK, ee::s13::f_003b58a0__generated);
        }
        fakes::fake_call(c, 0x003b_58a0);
    }

    pub(crate) fn oc1583_fake_unlock(c: &mut Cpu) {
        if c.bus.read32(OC1583_UNLOCK_CTRL) != OC1583_MAGIC {
            return fakes::fall_back(c, &OC1583_PREV_UNLOCK, ee::s13::f_003b58d8__generated);
        }
        fakes::fake_call(c, 0x003b_58d8);
    }

    pub(crate) fn oc1583_install() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B0770),
            oc1583_fake_static_init,
            &OC1583_PREV_STATIC,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B1300),
            oc1583_fake_get_bits,
            &OC1583_PREV_BITS,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B2360),
            oc1583_fake_marker,
            &OC1583_PREV_MARKER,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B1988),
            oc1583_fake_ext,
            &OC1583_PREV_EXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58A0),
            oc1583_fake_lock,
            &OC1583_PREV_LOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58D8),
            oc1583_fake_unlock,
            &OC1583_PREV_UNLOCK,
        );
    }

    /// `RCMP_DECODER_Reset`: the object's method (slot +0x34, `this` offset
    /// from +0x30) is a logged fake; the unwind index covers the return path.
    #[test]
    fn rcmp_decoder_reset_matches() {
        check(
            ee::s13::f_003b0680__generated,
            rcmp_decoder_reset,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003b_0680, 1);
                let this = fakes::ptr(&mut r);
                let obj = fakes::ptr(&mut r);
                let vt = fakes::vtable(c, &mut r, &[0x30]);
                c.bus.write32(obj, vt);
                c.bus.write32(this + 0x14, obj);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    /// `RCMP_DECODER_ReleaseChunk`: the function at `self + 0xc` is a logged
    /// fake (the miss hook); `a1` comes from `self + 4`.
    #[test]
    fn rcmp_decoder_release_chunk_matches() {
        check(
            ee::s13::f_003b06f8__generated,
            rcmp_decoder_release_chunk,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003b_06f8, 1);
                let this = fakes::ptr(&mut r);
                c.bus.write32(this + 0xc, fakes::FAKE);
                c.bus.write32(this + 4, r.next_u32());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    /// `RCMP_GlobalCtors` / `RCMP_GlobalDtors`: the static-init callee is a
    /// gated logged fake, so its two arguments are compared.
    #[test]
    fn rcmp_global_ctors_matches() {
        oc1583_install();
        check(
            ee::s13::f_003b07b8__generated,
            rcmp_global_ctors,
            200,
            &|c, seed| {
                machine(c, seed, 0x003b_07b8, 1);
                c.bus.write32(OC1583_STATIC_CTRL, OC1583_MAGIC);
            },
        );
    }

    #[test]
    fn rcmp_global_dtors_matches() {
        oc1583_install();
        check(
            ee::s13::f_003b07d8__generated,
            rcmp_global_dtors,
            200,
            &|c, seed| {
                machine(c, seed, 0x003b_07d8, 1);
                c.bus.write32(OC1583_STATIC_CTRL, OC1583_MAGIC);
            },
        );
    }

    /// `MPEG2_group_of_pictures_header`: both arms of the first-GOP check,
    /// and the three callees as logged fakes (one may unwind).
    #[test]
    fn mpeg2_group_of_pictures_header_matches() {
        oc1583_install();
        check(
            ee::s13::f_003b16e0__generated,
            mpeg2_group_of_pictures_header_2,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003b_16e0, 9);
                c.bus.write32(OC1583_BITS_CTRL, OC1583_MAGIC);
                c.bus.write32(OC1583_MARKER_CTRL, OC1583_MAGIC);
                c.bus.write32(OC1583_EXT_CTRL, OC1583_MAGIC);
                let cell = if seed % 2 == 0 {
                    0x510000 - 0x69b8
                } else {
                    r.next_u32()
                };
                c.bus.write32(OC1583_GOP_CELL, cell);
            },
        );
    }

    /// Gate word of the `__builtin_delete` fake below: its own address in the
    /// machine's RAM (beyond the fakes' call log), armed with `OC1597_MAGIC`.
    pub(crate) const OC1597_MAGIC: u32 = 0x1597_0001;

    pub(crate) const OC1597_DELETE_CTRL: u32 = DATA + 0x5e_0000;

    pub(crate) static OC1597_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);

    /// `__builtin_delete`: a logged no-op call while its gate is armed.
    pub(crate) fn oc1597_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC1597_DELETE_CTRL) != OC1597_MAGIC {
            return fakes::fall_back(c, &OC1597_PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        fakes::fake_call(c, 0x0031_7e50);
    }

    pub(crate) fn oc1597_install_delete() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1597_fake_delete,
            &OC1597_PREV_DELETE,
        );
    }

    #[test]
    fn rcmp_codec_dtor_matches() {
        check(
            ee::s15::f_0042e278__generated,
            rcmp_codec_dtor,
            200,
            &|c, seed| {
                oc1597_install_delete();
                let mut r = machine(c, seed, 0x0042_e278, 1);
                c.bus.write32(OC1597_DELETE_CTRL, OC1597_MAGIC);
                // D_00509434[0] is the delete fake: a call through the table.
                c.bus.write32(0x0050_9434, 0x0031_7e50);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = ((r.next_u32() & !1) | (seed & 1)) as u64;
            },
        );
    }

    #[test]
    fn rcmp_system_dtor_matches() {
        check(
            ee::s15::f_0042e498__generated,
            rcmp_system_dtor,
            200,
            &|c, seed| {
                oc1597_install_delete();
                let mut r = machine(c, seed, 0x0042_e498, 1);
                c.bus.write32(OC1597_DELETE_CTRL, OC1597_MAGIC);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = ((r.next_u32() & !1) | (seed & 1)) as u64;
            },
        );
    }
}
