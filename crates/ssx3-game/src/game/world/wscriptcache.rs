//! `src/world/wscriptcache.cpp`: hand-written replacements (moved from the
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
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_4;
pub(crate) use crate::game::dirtysock::tags2::gp_3 as gp;
pub(crate) use crate::game::fe::feasyncfile::a;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::at as at_4;
pub(crate) use crate::game::fe::feasyncfile::gp_12 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_5;
pub(crate) use crate::game::fe::feasyncfile::lw_9 as lw_6;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::p_7 as p_3;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw;
pub(crate) use crate::game::fe::feasyncfile::sw_6 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_3;
pub(crate) use crate::game::fe::fepopup::at as at_5;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_7;
pub(crate) use crate::game::fe::fepopup::sw_4;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_3;
pub(crate) use crate::game::fe::festaterewards::lw_3;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_2;
pub(crate) use crate::game::object::flexrailnode::r32v;
pub(crate) use crate::game::object::flexrailnode::spa;
pub(crate) use crate::game::object::railmodifier::reset_vtable_and_delete;
pub(crate) use crate::game::object::railmodifier::DELETE;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::sound::bankmonitor::leave;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
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
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::ai::callee::AI_RESET_RIDERS;
    pub(crate) use crate::game::ai::ai::callee::AI_SET_RIDER_ORDER_MODE;
    pub(crate) use crate::game::ai::ai::callee::BE_COPY_PROFILE_CHARACTERS;
    pub(crate) use crate::game::ai::ai::callee::INTC_RESTORE_REGS;
    pub(crate) use crate::game::ai::ai::callee::INTC_SAVE_REGS;
    pub(crate) use crate::game::ai::ai::callee::IRRADIANCE_FIND_INDEX;
    pub(crate) use crate::game::ai::ai::callee::IRRADIANCE_GET_ENTRY;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_FLUSH_BUFFER32;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::ai::ai::callee::RIDER_FIND_NEARBY_LIGHTS;
    pub(crate) use crate::game::ai::ai::callee::WORLD_CACHE_FREE;
    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_COUNT as BUDDY_MAN_GET_ROSTER_COUNT_2;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_ENTRY as BUDDY_MAN_GET_ROSTER_ENTRY_2;
    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;
    pub(crate) use crate::game::ai::rider::callee::CODEC_POP_FREE_FRAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_ALL_FIELDS_FILLED;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_BACKUP_ACCOUNT;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_ACCOUNT_NAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_CONFIRM_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_EMAIL;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_INFORM;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_SHARE_INFO;
    pub(crate) use crate::game::ai::rider::callee::CROWD_MAN_GROUP_FREE;
    pub(crate) use crate::game::ai::rider::callee::GAME_UI_OPEN_SCREEN;
    pub(crate) use crate::game::ai::rider::callee::IN_SEQ_PLAY;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_GET_LANGUAGE;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_ACCOUNT_MEM_POPUP;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_DISCONNECT_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_GET_LIST_INDEX;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_OPEN_VIEW_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::ai::rider::callee::NET_ACCOUNT_MEM_CTOR;
    pub(crate) use crate::game::ai::rider::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_6;
    pub(crate) use crate::game::ai::rider::callee::RAIL_MAN_ENABLE_RAIL;
    pub(crate) use crate::game::ai::rider::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_START_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHAR_STATS;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHECKING_CARD_POPUP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_BASE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_EDGE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_TRICK_METER_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::ai::rider::callee::START_CARD_SCAN;
    pub(crate) use crate::game::ai::rider::callee::STREAMER_GET_SECTION_COUNT;
    pub(crate) use crate::game::ai::rider::callee::SYNC_PROCESS_DTOR;
    pub(crate) use crate::game::ai::rider::callee::TEX_MAN_EVICT_TEXTURE;
    pub(crate) use crate::game::ai::rider::callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_LOAD;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_RESET;
    pub(crate) use crate::game::ai::rider::callee::WSCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::be::bedbprofile::callee::BE_GET_EVENT_RECORD_INDEX as BE_GET_EVENT_RECORD_INDEX_2;
    pub(crate) use crate::game::be::bedbprofile::callee::CLOSEST_POINT_ON_SEGMENT;
    pub(crate) use crate::game::be::bedbprofile::callee::SHP_CALC_IMAGE_SIZE;
    pub(crate) use crate::game::be::bedbprofile::callee::SHP_NORMALIZE_BPP;
    pub(crate) use crate::game::be::bedbprofile::callee::SHP_PALETTE_ENTRIES_FOR_BPP;
    pub(crate) use crate::game::be::bedbprofile::callee::WORLD_TRIGGER_FIND_FREE;
    pub(crate) use crate::game::be::bedbprofile::callee::WORLD_TRIGGER_FIND_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BUY_ATTRIB_UPDATE_BARS;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_EMPTY_STUB;
    pub(crate) use crate::game::be::beintmission::callee::CAREER_STATS_SHOW_HIGHLIGHTS;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::MEMCARD_SCAN_STEP;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_COUNT;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_NTH;
    pub(crate) use crate::game::be::beintmission::callee::MPEG2_ERROR;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_ON_WIDGET_ACTION;
    pub(crate) use crate::game::be::beintmission::callee::REWARDS_FILE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::UBER_TRICK_REFRESH;
    pub(crate) use crate::game::be::beintmission::callee::UITEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::be::beintmission::callee::VIEW_POPUP_SHOW_MESSAGE;
    pub(crate) use crate::game::be::beintmission::callee::WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED as KEYBOARD_POPUP_IS_KEY_ENABLED_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2 as REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2 as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_CHARACTER_2 as BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4 as BE_LIBRARY_GET_INTERFACE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_MISSION_INT_GET_SESSION_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_SLOT_3 as BE_NEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_GROUP_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_CTOR_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_RECV;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_SEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_SSX_WORLD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_ALLOC_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_FREE_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEAD_NODE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2 as FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_LOAD_UPDATE_PLAYER_RANKING;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LIGHT_MAN_EMPTY_HOOK_D;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_CARD_STATE_BASE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4 as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_12 as NET_FE_STATE_ON_CHILD_EVENT_7;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_13 as NET_FE_STATE_ON_CHILD_EVENT_8;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3 as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_GET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BUILD_WORLD_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_UPDATE_BOARD_LEAN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2 as SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_FLUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_TREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE as UI_ANIM_ADVANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY as UI_ANIM_APPLY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2 as UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2 as UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD as UI_OBJECT_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2 as UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2 as UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF_ASCII_FMT;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C0017f500;
    pub(crate) use crate::game::fe::fepopup::callee::C0019e2b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0019f878;
    pub(crate) use crate::game::fe::fepopup::callee::C001bb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001c84a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001c8568;
    pub(crate) use crate::game::fe::fepopup::callee::C001c9b68;
    pub(crate) use crate::game::fe::fepopup::callee::C001cb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbc48;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbce8;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbfc0;
    pub(crate) use crate::game::fe::fepopup::callee::C001ce560;
    pub(crate) use crate::game::fe::fepopup::callee::C001d32e0;
    pub(crate) use crate::game::fe::fepopup::callee::C001e1a30;
    pub(crate) use crate::game::fe::fepopup::callee::C001e3388;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91f8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e9290;
    pub(crate) use crate::game::fe::fepopup::callee::C001f10f8;
    pub(crate) use crate::game::fe::fepopup::callee::C0020e900;
    pub(crate) use crate::game::fe::fepopup::callee::C00224c78;
    pub(crate) use crate::game::fe::fepopup::callee::C00224da0;
    pub(crate) use crate::game::fe::fepopup::callee::C00224df0;
    pub(crate) use crate::game::fe::fepopup::callee::C002259a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00227f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00231f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00238bf8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026d4d8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026db88;
    pub(crate) use crate::game::fe::fepopup::callee::C0026e448;
    pub(crate) use crate::game::fe::fepopup::callee::C0026f4a0;
    pub(crate) use crate::game::fe::fepopup::callee::C002702f8;
    pub(crate) use crate::game::fe::fepopup::callee::C002703f0;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_SAVE_DATA_SIZE;
    pub(crate) use crate::game::fe::festateload::callee::BOLT_FILE_CACHE_INIT;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SELECT_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::EA_MEM_FREE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_UPDATE;
    pub(crate) use crate::game::fe::festateload::callee::I_SIGNAL_SEMA;
    pub(crate) use crate::game::fe::festateload::callee::MEM_CARD_BASE_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V42;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V43;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_HANDLE_EVENT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_METRIX_SET_SPIN_AMOUNT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_PREVIEW_APPLY_TRANSFORM;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_RESUME;
    pub(crate) use crate::game::fe::festateload::callee::STRLEN_COPY;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_TRIGGER_MANAGER_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_CACHE_SET_DATA;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_GOTO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_JUMP_TO_TASK;
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_ACCOUNT_NAME;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_PASSWORD;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::EVENT_SELECT_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_ASYNC_MANAGER_CANCEL;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_MOVE_DOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MEM_CARD_STATE_BASE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER as MPEG2_FLUSH_BUFFER_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS as MPEG2_SHOW_BITS_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR as UI_STATE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2 as UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING as UI_TEXT_SET_STRING_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_3;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION as MENU_SYS_PAGE_VALIDATE_SELECTION_2;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_4;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3 as SCRIPT_ENGINE_LIST_STOP_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2 as UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR as ACTIVE_CAMERA_TRIGGER_LIST_CLEAR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2 as SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2 as SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;
    pub(crate) use crate::game::sound::bankmonitor::callee::AUDIO_RAND;
    pub(crate) use crate::game::sound::bankmonitor::callee::BANK_MONITOR_TIMER_KILL;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4 as RIDER_METRIX_CLEAR_PEAK_A4_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_LIST_ADD_CHOICE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SECTION_ALLOCATOR_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::SNDSTRM_SET_PREBUFFER;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SONG_INSTANCE_GET_REQUEST_STATUS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SPEECH_SYSTEM_STOP_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::TEX_MAN_FREE_TEXTURE;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::sound::bankmonitor::callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_MAN_CALL_TASK_FUNC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_REMOVE;
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND as CROWD_AUDIO_INSTANCE_STOP_SOUND_2;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED as EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;

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
}

/// `gp`-relative globals used here.
pub(crate) mod g {
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::bx::bxstring2::addr::BANK_MONITOR_TIMER_RESET;
    pub(crate) use crate::game::bx::bxstring2::addr::BE_INTERFACE_LOOKUP;
    pub(crate) use crate::game::bx::bxstring2::addr::CBE_GET_BE;
    pub(crate) use crate::game::bx::bxstring2::addr::CBE_GET_INTERFACE;
    pub(crate) use crate::game::bx::bxstring2::addr::FMODF;
    pub(crate) use crate::game::bx::bxstring2::addr::MC_MANAGER_GET_INFO;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_FREE;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_FREE_NOSIZE;
    pub(crate) use crate::game::bx::bxstring2::addr::SPEECH_BANK_FREE_CALLBACK;
    pub(crate) use crate::game::bx::bxstring2::addr::SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::addr::UI_ENGINE_TEARDOWN;
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::STRLEN;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::AI_SET_AI_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::APP_MAN_SET_NEXT_MODULE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BENEW_PLAYER_INT_GET_CHAR_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_TEXT_FROM_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUDDY_PRODUCT_COMPARE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::CODEC_DECODE_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_BIG_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_SMALL_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::festaterewards::table_callee::MUSIC_SLOT_RELEASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CHEAT_CHARS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CONCEPT_ARTS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_GET_BASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_POSTERS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TOYS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TRADING_CARDS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_VIDEOS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_REMOVE_SCRIPT_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SCRIPT_CLEANUP;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SET_SCRIPT_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SSX_AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRCPY;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRLEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_GET_STRING4;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_GET_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OPEN_FINISH_SCREEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_ADD_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_STATE_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_RICH_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UPLOAD_TEXTURE_PIXELS;
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

/// `void AV_Player_GetChunkCallback(AV_Player *this, void *user)` (0x3aecb8):
/// `AV_Player_OnChunk(*user, this)`.
pub fn av_player_get_chunk_callback(c: &mut Cpu) {
    call::guest(c, |c| {
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
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
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
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
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

/// `void MPEG2_sequence_display_extension(void)` (0x3b1cc0): read the display
/// extension bit fields into the decoder's parameter block.
pub fn mpeg2_sequence_display_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 3;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1cd0)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ac8)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1ce0)?;
        c.g[V1][0] = 0x510000;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ac4)).sw(c, c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 12;
            c.g[A0][0] = 8;
            callee::MPEG2_GET_BITS.call(c, 0x003b_1cf4)?;
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 8;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6ac0)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_1d04)?;
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 8;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6abc)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_1d14)?;
            c.g[V1][0] = 0x510000;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6ab8)).sw(c, c.g[V0][0]);
        }
        c.cycles += 14;
        c.g[A0][0] = 0xe;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1d24)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x490000;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ab4)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x54d0));
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_1d38)?;
        c.g[A0][0] = 0xe;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1d40)?;
        c.g[V1][0] = 0x510000;
        frame.restore_ra(c, 0);
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ab0)).sw(c, c.g[V0][0]);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MPEG2_copyright_extension(void)` (0x3b23a0): parse the copyright
/// extension's fields out of the current bitstream into the decoder globals.
pub fn mpeg2_copyright_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 1;
        callee::MPEG2_GET_BITS_2.call(c, 0x003b_23b0)?;
        c.g[A0][0] = 8;
        Addr(0x0050_9610).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS_2.call(c, 0x003b_23c0)?;
        c.g[A0][0] = 1;
        Addr(0x0050_9614).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS_2.call(c, 0x003b_23d0)?;
        c.g[A0][0] = 7;
        Addr(0x0050_9618).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS_2.call(c, 0x003b_23e0)?;
        c.g[A0][0] = 0x0049_5630;
        callee::MPEG2_MARKER_BIT_2.call(c, 0x003b_23ec)?;
        c.g[A0][0] = 0x14;
        callee::MPEG2_GET_BITS_2.call(c, 0x003b_23f4)?;
        c.g[A0][0] = 0x0049_5658;
        Addr(0x0050_961c).sw(c, c.g[V0][0]);
        callee::MPEG2_MARKER_BIT_2.call(c, 0x003b_2408)?;
        c.g[A0][0] = 0x16;
        callee::MPEG2_GET_BITS_2.call(c, 0x003b_2410)?;
        c.g[A0][0] = 0x0049_5688;
        Addr(0x0050_9620).sw(c, c.g[V0][0]);
        callee::MPEG2_MARKER_BIT_2.call(c, 0x003b_2424)?;
        c.g[A0][0] = 0x16;
        callee::MPEG2_GET_BITS_2.call(c, 0x003b_242c)?;
        frame.restore_ra(c, 0);
        Addr(0x0050_9624).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `int SHP_DXTAlphaIsOpaque(const void* buf, int size, int type)` (0x3b3308):
/// scan 16-byte DXT blocks for a non-opaque alpha pattern.
pub fn shp_dxt_alpha_is_opaque(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 3;
    let other = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = sx(0xffff_ffff);
    if other {
        c.cycles += 6;
        c.g[V1][0] = sx(r32(c, A1).wrapping_add(0xf));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[A1][0];
        }
        c.g[V1][0] = sx(((r32(c, V1) as i32) >> 4) as u32);
        let empty = (c.g[V1][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if empty {
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
            return;
        }
        c.cycles += 1;
        c.g[A2][0] = 0xffff;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A0)).lhu(c);
            let mismatch = c.g[V0][0] != c.g[A2][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            if mismatch {
                c.cycles += 3;
                c.g[V0][0] = 0;
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x20));
            if !more {
                c.cycles += 2;
                c.g[V0][0] = 1;
                abi::ret(c);
                return;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_3388);
            }
        }
    }
    c.cycles += 6;
    c.g[V1][0] = sx(r32(c, A1).wrapping_add(0xf));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[A1][0];
    }
    c.g[V1][0] = sx(((r32(c, V1) as i32) >> 4) as u32);
    let empty = (c.g[V1][0] as i64) <= 0;
    c.g[A2][0] = 0;
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A3][0] = sx(0xffff);
    loop {
        c.cycles += 3;
        c.g[A1][0] = Addr(r32(c, A0)).ld(c);
        let mismatch = c.g[A1][0] != c.g[A3][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
        if mismatch {
            c.cycles += 3;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A0)).ld(c);
        let mismatch = c.g[V0][0] != c.g[A1][0];
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
        if mismatch {
            c.cycles += 3;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003b_3338);
        }
    }
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
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
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
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
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
        let this = abi::arg(c, 0);
        let count = abi::arg(c, 1);
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
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
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
        let this = abi::arg(c, 0);
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
        let mut count;
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

/// `SHP_Create(a, b, c, d, e, f, g, h)` (0x3b2a68): format a texture header
/// into a stack buffer, allocate the calculated size, and initialise the
/// header; returns the new texture or null.
pub fn shp_create(c: &mut Cpu) {
    const CALC_ALLOC_SIZE: Callee = Callee {
        addr: 0x003b_26b8,
        f: ee::s13::SHP_CalcAllocSize,
    };
    const INIT_HEADER: Callee = Callee {
        addr: 0x003b_27c8,
        f: ee::s13::SHP_InitHeader,
    };
    const SPRINTF: u32 = 0x0041_7828;

    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, 18, 0x90);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 19, 0x80);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, 20, 0x70);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, 21, 0x60);
        c.g[S5][0] = c.g[T1][0];
        frame.save(c, 22, 0x50);
        c.g[S6][0] = c.g[A3][0];
        frame.save(c, 23, 0x40);
        c.g[S7][0] = c.g[T0][0];
        sw_2(c, frame.at(c, 0x10), c.g[T3][0]);
        c.g[FP][0] = c.g[T2][0];
        frame.save(c, 16, 0xb0);
        frame.save(c, 17, 0xa0);
        frame.save_ra(c, 0x20);
        c.g[T1][0] = c.g[FP][0];
        c.g[T2][0] = c.g[T3][0];
        CALC_ALLOC_SIZE.call(c, 0x003b_2ac0)?;
        c.g[S1][0] = c.g[V0][0];
        let none = c.g[S1][0] == 0;
        c.g[S0][0] = 0;
        if !none {
            c.cycles += 18;
            c.g[A1][0] = 0x0049_56b0;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S3][0];
            c.g[T0][0] = c.g[S4][0];
            call_ptr(c, SPRINTF, 0x003b_2ae8)?;
            c.g[V1][0] = 0x0045_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[V0][0] = lw_5(c, (c.g[V1][0] as u32).wrapping_sub(0x3bac));
            c.g[T0][0] = c.g[S5][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = 0;
            c.g[A3][0] = 0x10;
            call_ptr(c, c.g[V0][0] as u32, 0x003b_2b08)?;
            c.g[S0][0] = c.g[V0][0];
            let failed = c.g[S0][0] == 0;
            c.g[T3][0] = lw_5(c, frame.at(c, 0x10));
            if !failed {
                c.cycles += 8;
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[T0][0] = c.g[S6][0];
                c.g[T1][0] = c.g[S7][0];
                c.g[T2][0] = c.g[FP][0];
                c.g[A0][0] = c.g[S0][0];
                INIT_HEADER.call(c, 0x003b_2b34)?;
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0xa0);
        frame.restore(c, 16, 0xb0);
        frame.restore(c, 18, 0x90);
        frame.restore(c, 19, 0x80);
        frame.restore(c, 20, 0x70);
        frame.restore(c, 21, 0x60);
        frame.restore(c, 22, 0x50);
        frame.restore(c, 23, 0x40);
        frame.restore(c, 30, 0x30);
        frame.restore_ra(c, 0x20);
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
            if callee::C003b1300.call(c, 0x003b1afc).is_err() {
                return;
            }
            c.g[A0][0] = 0x1;
            c.g[RA][0] = sx(0x003b1b08);
            sw_4(c, r32(c, S0).wrapping_sub(0x6ae0), c.g[V0][0]);
            if callee::C003b1300.call(c, 0x003b1b08).is_err() {
                return;
            }
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 0x2;
            c.g[RA][0] = sx(0x003b1b18);
            sw_4(c, r32(c, V1).wrapping_sub(0x6adc), c.g[V0][0]);
            if callee::C003b1300.call(c, 0x003b1b18).is_err() {
                return;
            }
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 0x2;
            c.g[RA][0] = sx(0x003b1b28);
            sw_4(c, r32(c, V1).wrapping_sub(0x6ad8), c.g[V0][0]);
            if callee::C003b1300.call(c, 0x003b1b28).is_err() {
                return;
            }
            c.g[A0][0] = 0x2;
            c.g[RA][0] = sx(0x003b1b34);
            c.g[S4][0] = c.g[V0][0];
            if callee::C003b1300.call(c, 0x003b1b34).is_err() {
                return;
            }
            c.g[A0][0] = 0xc;
            c.g[RA][0] = sx(0x003b1b40);
            c.g[S5][0] = c.g[V0][0];
            if callee::C003b1300.call(c, 0x003b1b40).is_err() {
                return;
            }
            c.g[A0][0] = 0x490000;
            c.g[S2][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003b1b50);
            c.g[A0][0] = sx(at_5(c, A0, 0x54b8));
            if callee::C003b2360.call(c, 0x003b1b50).is_err() {
                return;
            }
            c.g[RA][0] = sx(0x003b1b58);
            c.g[A0][0] = 0x8;
            if callee::C003b1300.call(c, 0x003b1b58).is_err() {
                return;
            }
            c.g[A0][0] = 0x1;
            c.g[RA][0] = sx(0x003b1b64);
            c.g[S3][0] = c.g[V0][0];
            if callee::C003b1300.call(c, 0x003b1b64).is_err() {
                return;
            }
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 0x2;
            c.g[RA][0] = sx(0x003b1b74);
            sw_4(c, r32(c, V1).wrapping_sub(0x6ad4), c.g[V0][0]);
            if callee::C003b1300.call(c, 0x003b1b74).is_err() {
                return;
            }
            c.g[A0][0] = 0x5;
            c.g[RA][0] = sx(0x003b1b80);
            sw_4(c, r32(c, S1).wrapping_sub(0x6ad0), c.g[V0][0]);
            if callee::C003b1300.call(c, 0x003b1b80).is_err() {
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fakes::*;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::ai::ai::tests::strict;
    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::ai::ai::tests::SEEDS as SEEDS_10;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_7;
    pub(crate) use crate::game::camera::script::scriptcontroller::tests::SEEDS_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::globals_3 as globals_5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::pool_4 as pool_5;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_8;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_3;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_5;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals_4;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init_2;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_4;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::run;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA_SIZE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::MEM_HEAP;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::ui::uivector::tests::start;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6;
    use ssx3_ee as ee;
    use ssx3_ee::s00;
    use ssx3_ee::s01;
    use ssx3_ee::s02;
    use ssx3_ee::s03;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;
    use ssx3_ee_rt::funcs::GuestFn;

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
                        [0x20u8, 0x09, 0x0a, 0x0d][((seed as usize + i as usize) % 4)]
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
                let mut r = base_6(c, seed, 0xed40, 0);
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
}
