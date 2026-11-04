//! `src/render/particle.cpp`: hand-written replacements (moved from the
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
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::bx::memblockman::gp_2;
pub(crate) use crate::game::camera::camera::gp_4;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gpa as gpa_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::ofs;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::gpa;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_5;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_4;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::main::gamemode::lh_r;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::render::graphicsman::vslot;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::scripter::ssxscriptengine::gp_5 as gp_3;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p_3;
pub(crate) use crate::game::sound::bankmonitor::add32 as add32_2;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::ui::uivector::r;
pub(crate) use crate::game::visualfx::boardwakefx::copy_render_state;
pub(crate) use crate::game::visualfx::boardwakefx::ldf_2;
pub(crate) use crate::game::visualfx::boardwakefx::lh;
pub(crate) use crate::game::visualfx::boardwakefx::lw;
pub(crate) use crate::game::worldpainter::quadtree::create_with;
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
    (0x0037_10d0, dynamic_emitter_spawn, "DynamicEmitter_Spawn"),
    (0x0037_1318, dynamic_emitter_load, "DynamicEmitter_Load"),
    (0x0037_14b8, emitter_ctor, "mEmitter_ctor"),
    (0x0037_14f8, memitter_dtor, "mEmitter_dtor"),
    (
        0x0037_1600,
        memitter_reset_particles,
        "mEmitter_ResetParticles",
    ),
    (0x0037_1688, m_emitter_render, "mEmitter_Render"),
    (
        0x0037_17c0,
        dynamic_emitter_spawn_at,
        "DynamicEmitter_SpawnAt",
    ),
    (0x0037_2520, rail_tube_ctor, "RailTube_ctor"),
    (0x0037_2660, rail_tube_dtor, "RailTube_dtor"),
    (0x0037_2aa0, path_arrow_ctor, "PathArrow_ctor"),
    (
        0x0037_2b30,
        bezier_patch_cache_ctor,
        "BezierPatchCache_ctor",
    ),
    (
        0x0037_4a88,
        bezier_patch_cache_mark_used,
        "BezierPatchCache_MarkUsed",
    ),
    (0x0037_4c90, bezier_man_base_ctor, "BezierManBase_ctor"),
    (0x0037_4ca8, bezier_man_base_dtor, "BezierManBase_dtor"),
    (
        0x0037_4ce0,
        boader_shadow_set_params,
        "BoaderShadow_SetParams",
    ),
    (
        0x0037_4d00,
        boader_shadow_compute_projection,
        "BoaderShadow_ComputeProjection",
    ),
    (0x0037_5890, boader_shadow_submit, "BoaderShadow_Submit"),
    (0x0037_58f8, boader_shadow_ctor, "BoaderShadow_ctor"),
    (0x0037_5918, boader_shadow_dtor, "BoaderShadow_dtor"),
    (
        0x0037_5990,
        boader_shadow_alloc_texture,
        "BoaderShadow_AllocTexture",
    ),
    (0x0037_59e8, detail_system_ctor, "DetailSystem_ctor"),
    (
        0x0037_5a08,
        ps2_graphics_man_create,
        "PS2GraphicsMan_Create",
    ),
    (
        0x0037_6468,
        ps2_graphics_man_select_matching_display_mode,
        "PS2GraphicsMan_SelectMatchingDisplayMode",
    ),
    (
        0x0037_6560,
        ps2_graphics_man_get_display_mode_info,
        "PS2GraphicsMan_GetDisplayModeInfo",
    ),
    (
        0x0037_6578,
        ps2_graphics_man_set_default_display_mode,
        "PS2GraphicsMan_SetDefaultDisplayMode",
    ),
    (
        0x0037_65b8,
        ps2_graphics_man_push_view,
        "PS2GraphicsMan_PushView",
    ),
    (
        0x0037_6768,
        ps2_graphics_man_pop_view,
        "PS2GraphicsMan_PopView",
    ),
    (
        0x0037_6a70,
        ps2_graphics_man_set_viewport,
        "PS2GraphicsMan_SetViewport",
    ),
    (
        0x0037_6b90,
        ps2_graphics_man_get_viewport,
        "PS2GraphicsMan_GetViewport",
    ),
    (
        0x0037_6c10,
        ps2_graphics_man_get_fov,
        "PS2GraphicsMan_GetFov",
    ),
    (
        0x0037_6c28,
        ps2_graphics_man_get_near_far_aspect,
        "PS2GraphicsMan_GetNearFarAspect",
    ),
    (0x0037_7278, ps2_graphics_man_v27, "PS2GraphicsMan_v27"),
    (
        0x0037_7950,
        ps2_graphics_man_set_fog_mode,
        "PS2GraphicsMan_SetFogMode",
    ),
    (
        0x0037_79e0,
        ps2_graphics_man_set_fog_params,
        "PS2GraphicsMan_SetFogParams",
    ),
    (
        0x0037_a540,
        ps2_graphics_man_create_grid_mesh,
        "PS2GraphicsMan_CreateGridMesh",
    ),
    (
        0x0037_c570,
        ps2_graphics_man_set_brightness_rgb,
        "PS2GraphicsMan_SetBrightnessRGB",
    ),
    (
        0x0037_c720,
        ps2_graphics_man_set_brightness_alpha,
        "PS2GraphicsMan_SetBrightnessAlpha",
    ),
    (
        0x0037_c7c0,
        ps2_graphics_man_reset_texture_slots,
        "PS2GraphicsMan_ResetTextureSlots",
    ),
    (
        0x0037_c830,
        ps2_graphics_man_free_all_textures,
        "PS2GraphicsMan_FreeAllTextures",
    ),
    (
        0x0037_c8a0,
        ps2_graphics_man_set_tex_alloc_flags,
        "PS2GraphicsMan_SetTexAllocFlags",
    ),
    (
        0x0037_caf8,
        ps2_graphics_man_load_texture,
        "PS2GraphicsMan_LoadTexture",
    ),
    (
        0x0037_cb50,
        ps2_graphics_man_free_texture,
        "PS2GraphicsMan_FreeTexture",
    ),
    (
        0x0037_cb90,
        ps2_graphics_man_is_texture_valid,
        "PS2GraphicsMan_IsTextureValid",
    ),
    (0x0037_cbe8, ps2_graphics_man_v59, "PS2GraphicsMan_v59"),
    (0x0037_cc30, ps2_graphics_man_v60, "PS2GraphicsMan_v60"),
    (
        0x0037_d758,
        ps2_graphics_man_add_light,
        "PS2GraphicsMan_AddLight",
    ),
    (
        0x0037_d908,
        ps2_graphics_man_get_texture_width,
        "PS2GraphicsMan_GetTextureWidth",
    ),
    (
        0x0037_d938,
        ps2_graphics_man_get_texture_height,
        "PS2GraphicsMan_GetTextureHeight",
    ),
    (
        0x0037_d968,
        ps2_graphics_man_update_view_matrices,
        "PS2GraphicsMan_UpdateViewMatrices",
    ),
    (
        0x0037_de88,
        ps2_graphics_man_clip_test_box,
        "PS2GraphicsMan_ClipTestBox",
    ),
    (
        0x0037_df88,
        ps2graphics_man_cull_against_lights,
        "PS2GraphicsMan_CullAgainstLights",
    ),
    (
        0x0037_e040,
        ps2_graphics_man_emit_model_matrix2,
        "PS2GraphicsMan_EmitModelMatrix2",
    ),
    (
        0x0037_e098,
        ps2_graphics_man_emit_model_lighting,
        "PS2GraphicsMan_EmitModelLighting",
    ),
    (0x0038_0380, ps2_graphics_man_v97, "PS2GraphicsMan_v97"),
    (
        0x0038_2650,
        renderer_vif1_dmac_handler,
        "Renderer_Vif1DmacHandler",
    ),
    (
        0x0038_2688,
        ps2_graphics_man_on_vif1_dma_end,
        "PS2GraphicsMan_OnVif1DmaEnd",
    ),
    (0x0038_4d98, line_strip_init, "LineStrip_Init"),
    (0x0038_4dc0, line_strip_close, "LineStrip_Close"),
    (0x0038_4e50, line_strip_finish, "LineStrip_Finish"),
    (0x0038_4fd0, line_strip_add_vertex, "LineStrip_AddVertex"),
    (0x0038_52e8, c_ps2_grid_mesh_v03, "cPS2GridMesh_v03"),
    (
        0x0038_5a38,
        cps2_grid_mesh_pack_position,
        "cPS2GridMesh_PackPosition",
    ),
    (
        0x0038_5aa0,
        cps2_grid_mesh_pack_color,
        "cPS2GridMesh_PackColor",
    ),
    (0x0038_5ba0, cps2_grid_mesh_pack_uv, "cPS2GridMesh_PackUV"),
    (0x0038_5be0, c_ps2_grid_mesh_v07, "cPS2GridMesh_v07"),
    (
        0x0038_6688,
        ps2_graphics_man_add_view,
        "PS2GraphicsMan_AddView",
    ),
    (0x0038_6cf0, ps2_graphics_man_v118, "PS2GraphicsMan_v118"),
    (
        0x0038_6dd0,
        ps2_graphics_man_draw_shadow_edge_pass,
        "PS2GraphicsMan_DrawShadowEdgePass",
    ),
    (0x0038_89f0, ps2_graphics_man_v58, "PS2GraphicsMan_v58"),
    (
        0x0038_9098,
        ps2_graphics_man_set_clear_color,
        "PS2GraphicsMan_SetClearColor",
    ),
    (
        0x0038_9118,
        ps2_graphics_man_remove_interrupt_handlers,
        "PS2GraphicsMan_RemoveInterruptHandlers",
    ),
    (0x0038_9558, vec3_add_to, "Vec3_AddTo"),
    (0x0038_9590, irradiance_add_scaled, "Irradiance_AddScaled"),
    (
        0x0038_9620,
        irradiance_add_scaled_w,
        "Irradiance_AddScaledW",
    ),
    (0x0038_97a0, irradiance_scale, "Irradiance_Scale"),
    (0x0038_9c38, irradiance_build_sh, "Irradiance_BuildSH"),
    (
        0x0038_a530,
        irradiance_spot_light_vector,
        "Irradiance_SpotLightVector",
    ),
    (
        0x0038_a618,
        irradiance_point_light_vector,
        "Irradiance_PointLightVector",
    ),
    (
        0x0038_a6a8,
        irradiance_add_dynamic_light,
        "Irradiance_AddDynamicLight",
    ),
    (
        0x0038_ac20,
        irradiance_get_entry_by_name,
        "Irradiance_GetEntryByName",
    ),
    (
        0x0037_4180,
        bezier_patch_cache_tess_positions,
        "BezierPatchCache_TessPositions",
    ),
    (
        0x0037_4518,
        bezier_patch_cache_tess_positions_alt,
        "BezierPatchCache_TessPositionsAlt",
    ),
];

/// Same-shard direct callees used above.
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE as RIDER_GET_MAIN_STATE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST as UI_LIST_FIRST_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING as UI_TEXT_SET_STRING_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_3 as EA_MEM_FREE;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_NEXT_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_PREV_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_NODE_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILD_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_GET_RIDER_PREVIEW_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUTSYNC_IS_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_HEAP_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_ACTIVATE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_FIND_ENABLED_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED_3 as MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_FIND_BANK_SLOT as PATH_PLAT_FIND_BANK_SLOT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_GRID_MESH_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BASE_RESET_FX;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_CATEGORY_TITLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_BUY_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_EQUIP_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_GET_PATH_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::START_SCREEN_CTOR;
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
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_5;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE as CPS2_MEM_CARD_SET_STATE_2;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET as MEMSET_3;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_3;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF as SPRINTF_2;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY as STRCPY_3;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII as USTR_TO_ASCII_2;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::movie::movieplayer::callee::AIR_APPLY_WIND;
    pub(crate) use crate::game::movie::movieplayer::callee::AIR_PREDICTOR_UPDATE;
    pub(crate) use crate::game::movie::movieplayer::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::movie::movieplayer::callee::AUDIO_RESET_BOOST_PITCH;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_CHAR_ATTRIB_FACTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILD_OCCLUDER_PLANES;
    pub(crate) use crate::game::movie::movieplayer::callee::COLL_SEGMENT_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::MATH_DET3X3;
    pub(crate) use crate::game::movie::movieplayer::callee::OVERLAP_LIST_FIND_CLOSEST;
    pub(crate) use crate::game::movie::movieplayer::callee::PACK_POSITION;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL128_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL16_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL256_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL32_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL512_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL64_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL8_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::QSORT;
    pub(crate) use crate::game::movie::movieplayer::callee::REMOVE_HUD_MESSAGE;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_GET_SUBSTATE;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_ORIENTATION_IMPLICIT;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_UPDATE_ORIENTATION;
    pub(crate) use crate::game::movie::movieplayer::callee::SET_TIMED_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::SHOW_HUD_NUMBER;
    pub(crate) use crate::game::movie::movieplayer::callee::VU0_UPLOAD_OCCLUDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;
    pub(crate) use crate::game::sound::bankmonitor::callee::AUDIO_RAND;
    pub(crate) use crate::game::sound::bankmonitor::callee::BANK_MONITOR_TIMER_KILL;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4;
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
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
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
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::CBXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::C_WORLD_SOUND_UNLINK_EMITTERS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EA_MEM_COPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EVENT_SELECT_HANDLE_INPUT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::GRAPHICS_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::LOBBY_MAN_REQUEST_STATS_PAGE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MC_MANAGER_GET_INSTANCE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MENU_SYS_GET_INPUT_CODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_SET_TRACK_CONTROL_VALUE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_DUMMY_OBJ_READ_FIELDS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SSX_APP_BOOT_LOAD;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_UPDATE_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STREAM_BUFFER_GET_STATUS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::TEX_MAN_SET_ALLOC_FLAGS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_NODE_UNLINK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_STATE_MGR_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_RESET_SECTIONS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_INFO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_NAME;
    pub(crate) use crate::game::worldpainter::quadtree::callee::AI_VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::BUILTIN_DELETE_2;
    pub(crate) use crate::game::worldpainter::quadtree::callee::CONSOLE_CONFIG_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EMITTER_RENDER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_ON_VIF1_DMA_END;
    pub(crate) use crate::game::worldpainter::quadtree::callee::INTC_REMOVE_HANDLER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_SET_STATE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MENU_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MOD_OBJ_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::PS2_SOUND_MANAGER_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SNDSTRM_GET_REQUEST_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_FADE_OUT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_GET_STREAM_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_POST_EVENT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SPEECH_INSTANCE_STOP_STREAM;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STATS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STRCPY;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_SET_FILE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_STOP;
    pub(crate) use crate::game::worldpainter::quadtree::callee::VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_GET_SECTION_SCRIPT_FUNC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_OBJ_CTOR;

    pub const DYNAMIC_EMITTER_SPAWN: Callee = Callee {
        addr: 0x0037_10d0,
        f: ee::s12::DynamicEmitter_Spawn,
    };

    /// `rand()`-style float source the emitter calls (cross-shard, through
    /// the function table).
    pub const RANDOM: u32 = 0x0031_77f0;

    pub const POINT_BOX_DISTANCE: Callee = Callee {
        addr: 0x003a_9e50,
        f: ee::s13::cWorldCache_PointBoxDistance,
    };

    pub const IS_HULL_RESIDENT: Callee = Callee {
        addr: 0x003a_8618,
        f: ee::s13::cWorldCache_IsHullResident,
    };

    pub const REQUEST_HULL: Callee = Callee {
        addr: 0x003a_8448,
        f: ee::s13::cWorldCache_RequestHull,
    };

    pub const STREAMER_UPDATE_VIEW_NODE: Callee = Callee {
        addr: 0x003a_a028,
        f: ee::s13::cWorldCache_StreamerUpdateViewNode,
    };

    pub const HUD_DRAW_PANEL_SEGMENT: Callee = Callee {
        addr: 0x0021_ea00,
        f: ee::s05::HUD_DrawPanelSegment,
    };

    /// HUD helpers (cross-shard, through the function table).
    pub const HUD_GET_ELEM_COLOR: u32 = 0x001e_91a8;

    pub const HUD_GET_ELEM_POS: u32 = 0x001e_91f8;

    pub const HUD_GET_ELEM_SIZE: u32 = 0x001e_9220;

    pub const HUD_ALIGN_ELEM_POS: u32 = 0x001f_10f8;

    pub const HUD_DRAW_SPRITE: u32 = 0x001f_1190;

    pub const FONT_DRAW_TEXT: u32 = 0x0039_1cb0;

    pub const SPATIAL_KEY_FROM_AABB: Callee = Callee {
        addr: 0x0032_8f28,
        f: ee::s10::SpatialKey_FromAABB,
    };

    pub const SPATIAL_HASH_REMOVE: Callee = Callee {
        addr: 0x0032_84b8,
        f: ee::s10::SpatialHash_Remove,
    };

    pub const SPATIAL_HASH_INSERT: Callee = Callee {
        addr: 0x0032_8c20,
        f: ee::s10::SpatialHash_Insert,
    };

    pub const SPATIAL_HASH_MOVE: Callee = Callee {
        addr: 0x0032_8808,
        f: ee::s10::SpatialHash_Move,
    };

    pub const DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };

    pub const SPOT_LIGHT_VECTOR: Callee = Callee {
        addr: 0x0038_a530,
        f: ee::s12::Irradiance_SpotLightVector,
    };

    pub const POINT_LIGHT_VECTOR: Callee = Callee {
        addr: 0x0038_a618,
        f: ee::s12::Irradiance_PointLightVector,
    };

    pub const ADD_DIRECTIONAL: Callee = Callee {
        addr: 0x0038_9520,
        f: ee::s12::Irradiance_AddDirectional,
    };

    pub const SAMPLE_SURFACE_COLOURS: Callee = Callee {
        addr: 0x002e_db20,
        f: ee::s09::WorldPainter_SampleSurfaceColours,
    };

    pub const LERP_VIEW_COLOURS: Callee = Callee {
        addr: 0x002e_d338,
        f: ee::s09::WorldPainter_LerpViewColours,
    };

    pub const BLEND_VIEW_COLOURS: Callee = Callee {
        addr: 0x002e_d1d0,
        f: ee::s09::WorldPainter_BlendViewColours,
    };

    pub const GET_IRRADIANCE0: Callee = Callee {
        addr: 0x002e_e318,
        f: ee::s09::WorldPainter_GetIrradiance0,
    };

    pub const GET_IRRADIANCE1: Callee = Callee {
        addr: 0x002e_e340,
        f: ee::s09::WorldPainter_GetIrradiance1,
    };

    pub const GET_IRRADIANCE2: Callee = Callee {
        addr: 0x002e_e368,
        f: ee::s09::WorldPainter_GetIrradiance2,
    };

    pub const COMPUTE_COLOUR_FACTOR: Callee = Callee {
        addr: 0x002e_df00,
        f: ee::s09::WorldPainter_ComputeColourFactor,
    };

    pub const GET_LIGHTING28: Callee = Callee {
        addr: 0x002e_eff0,
        f: ee::s09::WorldPainter_GetLighting28,
    };

    /// Cross-shard (function table).
    pub const SECTION_GET_EVENT_WORLD_SECTION: u32 = 0x0022_e098;

    pub const RIDER_GET_MAIN_STATE: u32 = 0x0011_fe98;

    pub const IRRADIANCE_SCALE: u32 = 0x0038_97a0;

    pub const IRRADIANCE_SCALE_IN_PLACE: u32 = 0x0038_9810;

    pub const IRRADIANCE_ADD: u32 = 0x0038_9730;

    /// Same shard: a `jal` to the generated alias.
    pub const UPDATE_VIEW_MATRICES: Callee = Callee {
        addr: 0x0037_d968,
        f: ee::s12::PS2GraphicsMan_UpdateViewMatrices,
    };

    pub const INTERN_MATRIX_RECORD: Callee = Callee {
        addr: 0x0036_45b8,
        f: ee::s12::DeferMan_InternMatrixRecord,
    };

    pub const SET_SCISSOR: Callee = Callee {
        addr: 0x0036_4b88,
        f: ee::s12::DeferMan_SetScissor,
    };

    pub const SET_SCISSOR_CLAMPED: Callee = Callee {
        addr: 0x0037_6938,
        f: ee::s12::PS2GraphicsMan_SetScissorClamped,
    };

    /// Cross-shard: a `jal` the generator emitted as `call_indirect`.
    pub const COPY_VIEW_STATE: u32 = 0x0039_50c0;

    // Graphics (shard 12).
    pub const TEX_MAN_SET_ALLOC_FLAGS_2: Callee = Callee {
        addr: 0x0036_6fe0,
        f: ee::s12::f_00366fe0,
    };

    pub const INI_PARSER_SKIP_TO_EOL: Callee = Callee {
        addr: 0x002a_7a90,
        f: ee::s08::IniParser_SkipToEol,
    };

    pub const BANK_MONITOR_INSTANCE_PAUSE: Callee = Callee {
        addr: 0x002a_cac8,
        f: ee::s08::BankMonitorInstance_Pause,
    };

    pub const SONG_INSTANCE_GET_STREAM_HANDLE: Callee = Callee {
        addr: 0x002b_2488,
        f: ee::s08::SongInstance_GetStreamHandle,
    };

    pub const SONG_INSTANCE_GET_MS_TO_NEXT_BEAT: Callee = Callee {
        addr: 0x002b_2718,
        f: ee::s08::SongInstance_GetMsToNextBeat,
    };

    pub const CPS2_MEM_CARD_SET_STATE: Callee = Callee {
        addr: 0x002c_48c0,
        f: ee::s08::cPS2MemCard_SetState,
    };

    pub const MENU_SYS_ITEM_CTOR: Callee = Callee {
        addr: 0x002c_a258,
        f: ee::s09::MenuSysItem_ctor,
    };

    pub const MENU_SYS_ITEM_DRAW_ROW_2: Callee = Callee {
        addr: 0x002c_a988,
        f: ee::s09::MenuSysItem_DrawRow,
    };

    pub const CUT_SCENE_TRANSITION_SAVE_VIEWPORT: Callee = Callee {
        addr: 0x002e_aa28,
        f: ee::s09::cCutSceneTransitionMan_SaveViewport,
    };

    pub const EA_MEM_ALLOC_NAMED_2: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::EA_MemAllocNamed,
    };

    pub const AVA_SPLINE_MODIFIER_CTOR: Callee = Callee {
        addr: 0x0035_7750,
        f: ee::s11::AvaSplineModifier_ctor,
    };

    pub const POSITION_MODIFIER_CTOR: Callee = Callee {
        addr: 0x0035_6eb8,
        f: ee::s11::PositionModifier_ctor,
    };

    pub const MOD_OBJ_SET_TRANSFORM_MODIFIER: Callee = Callee {
        addr: 0x0035_54b0,
        f: ee::s11::ModObj_SetTransformModifier,
    };

    pub const ANIM_OBJECT_BASE_DTOR: Callee = Callee {
        addr: 0x0034_dba8,
        f: ee::s11::AnimObject_BaseDtor,
    };

    pub const BUILTIN_DELETE_4: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };

    pub const TEX_MAN_LOAD_TEXTURE: Callee = Callee {
        addr: 0x0036_7440,
        f: ee::s12::TexMan_LoadTexture,
    };

    pub const UI_LIST_RESET: Callee = Callee {
        addr: 0x0039_7930,
        f: ee::s13::UIList_Reset,
    };

    pub const UI_LIST_PUSH_BACK_2: Callee = Callee {
        addr: 0x0039_7910,
        f: ee::s13::UIList_PushBack,
    };

    pub const CPS2_CHUNK_RESOLVE_DTOR_STUB: Callee = Callee {
        addr: 0x003a_ae60,
        f: ee::s13::cPS2ChunkResolve_DtorStub,
    };

    pub const RIDER_METRIX_ON_MISSION_COMPLETED: Callee = Callee {
        addr: 0x0011_a110,
        f: ee::s00::RiderMetrix_OnMissionCompleted,
    };

    pub const RIDER_METRIX_AWARD_POINTS: Callee = Callee {
        addr: 0x0011_9ef8,
        f: ee::s00::RiderMetrix_AwardPoints,
    };

    pub const RIDER_BOARD_PRESS_SUB_SETTLE_SLEW: Callee = Callee {
        addr: 0x0013_13a8,
        f: ee::s00::RiderBoardPressSub_SettleSlew,
    };

    pub const RIDER_BOARD_PRESS_SUB_FINISH_PRESS: Callee = Callee {
        addr: 0x0013_1428,
        f: ee::s00::RiderBoardPressSub_FinishPress,
    };

    pub const RIDER_ADD_BOOST: Callee = Callee {
        addr: 0x0010_e098,
        f: ee::s00::Rider_AddBoost,
    };

    pub const RIDER_METRIX_FINISH_BOARD_PRESS: Callee = Callee {
        addr: 0x0011_9a38,
        f: ee::s00::RiderMetrix_FinishBoardPress,
    };

    pub const ALGORITHM_LIST_DTOR: Callee = Callee {
        addr: 0x0015_c940,
        f: ee::s01::AlgorithmList_dtor,
    };

    pub const CHASE_CAM_SET_ALGORITHM_IMMEDIATE: Callee = Callee {
        addr: 0x0015_d050,
        f: ee::s01::ChaseCamController_SetAlgorithmImmediate,
    };

    pub const CHASE_CAMERA_ALGORITHM_CTOR: Callee = Callee {
        addr: 0x0016_2318,
        f: ee::s01::ChaseCameraAlgorithm_ctor,
    };

    pub const CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS: Callee = Callee {
        addr: 0x0018_1bd0,
        f: ee::s02::cFEStateCharSelect_UpdateAttributeBars,
    };

    pub const BXSTRING_ASSIGN_CSTR: Callee = Callee {
        addr: 0x0031_84e8,
        f: ee::s10::cBXString_assign_cstr,
    };

    pub const BXSTRING_ASSIGN_COPY: Callee = Callee {
        addr: 0x0031_83d8,
        f: ee::s10::cBXString_AssignCopy,
    };

    pub const BXSTRING_CONCAT_IN_PLACE: Callee = Callee {
        addr: 0x0031_8830,
        f: ee::s10::cBXString_ConcatInPlace,
    };

    pub const CROWD_MAN3D_GROUP_REMOVE_SECTION: Callee = Callee {
        addr: 0x0034_4898,
        f: ee::s11::CrowdMan3d_GroupRemoveSection,
    };

    pub const WORLD_OBJ_SAVE: Callee = Callee {
        addr: 0x0034_fe90,
        f: ee::s11::WorldObj_Save,
    };

    pub const FLEX_BRIDGE_SPRINGS_SAVE: Callee = Callee {
        addr: 0x0035_45d8,
        f: ee::s11::FlexBridge_SpringsSave,
    };

    /// Library entry points reached through the function table.
    pub const BELIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

    pub const BELIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const BEREWARD_ON_MISSION_COMPLETED: u32 = 0x0015_9b08;

    pub const BEREWARD_ON_COLLECTIBLE_COLLECTED: u32 = 0x0015_99a0;

    pub const BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT: u32 = 0x0014_7488;

    pub const BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX: u32 = 0x0014_4cc0;

    pub const SND_LOCK: u32 = 0x003b_58a0;

    pub const SND_UNLOCK: u32 = 0x003b_58d8;

    pub const SND_SPU_MEM_SET_BASE: u32 = 0x003b_5b20;

    pub const LOBBY_OCCUPANT_POP: u32 = 0x002c_c460;

    pub const LOBBY_OCCUPANT_SELECT: u32 = 0x002c_c3b8;

    pub const SNDSTRM_SET_PITCH: u32 = 0x003b_7fb8;

    pub const PATH_SET_PAUSED: u32 = 0x003d_1038;

    pub const PATH_POST_EVENT: u32 = 0x003d_16f0;

    pub const SCE_MC_SEEK: u32 = 0x0040_9f28;

    pub const SCE_MC_READ: u32 = 0x0040_a090;

    pub const SCE_MC_WRITE: u32 = 0x0040_a1a8;

    pub const SCE_MC_MKDIR: u32 = 0x0040_9e38;

    pub const SCE_MC_DELETE: u32 = 0x0040_aa68;

    pub const ISIGNAL_SEMA: u32 = 0x0042_3dd0;

    pub const STRLEN: u32 = 0x0041_6810;

    pub const WORLD_CREATE_DEAD_NODE: u32 = 0x002d_1cc0;
}

// ---------------------------------------------------------------------------
// Lights.

/// Point light fields read by `WorldLightMan_LightDistance`.
pub(crate) mod light {

    pub const RADIUS: u32 = 0x1c;

    pub const DIR: u32 = 0x2c;

    pub const POS: u32 = 0x38;
}

// ---------------------------------------------------------------------------
// Dynamic emitters.

/// `DynamicEmitter` fields.
pub(crate) mod emitter {

    /// Particle lifetime step and accumulated time (`+0x10 += +0x14 * dt`,
    /// wrapped to 0 at `+0xc`).
    pub const PERIOD: u32 = 0x0c;

    pub const TIME: u32 = 0x10;

    pub const RATE: u32 = 0x14;

    pub const ENABLED: u32 = 0x174;

    pub const CAPACITY: u32 = 0x178;

    /// Ring cursor (counts down, wraps to `CAPACITY - 1`).
    pub const CURSOR: u32 = 0x17c;

    pub const POSITIONS: u32 = 0x1a0;

    pub const VELOCITIES: u32 = 0x1a4;

    pub const DEFAULT_POS: u32 = 0x1b0;

    pub const DEFAULT_VEL: u32 = 0x1c0;

    pub const DEFAULT_COLOUR: u32 = 0x1d0;

    pub const LIVE: u32 = 0x1e0;

    pub const COLOURS: u32 = 0x204;
}

// ---------------------------------------------------------------------------
// Texture sampling.

/// Texture record fields (`PS2GraphicsMan` texture table, `+0x18f4`).
pub(crate) mod texture {

    /// Pixel storage mode: 0 32-bit, 0xa 8-bit indexed, 0xc 4-bit indexed.
    pub const PSM: u32 = 0x0c;

    pub const PIXELS: u32 = 0x1c;

    pub const CLUT: u32 = 0x20;

    /// The GS TEX0 doubleword (TW at bit 26, TH at bit 30).
    pub const TEX0: u32 = 0x38;
}

/// Dynamic light fields.
pub(crate) mod dyn_light {

    pub const KIND: u32 = 0x10;

    pub const INTENSITY: u32 = 0x14;

    pub const RANGE: u32 = 0x1c;

    pub const COLOUR: u32 = 0x20;

    /// Cone/radius falloff exponent kind (`lb`): 1 linear, 2 square, 3 cube.
    pub const SHAPE: u32 = 0x64;

    /// Distance falloff exponent (`lb`).
    pub const EXPONENT: u32 = 0x65;

    pub const INNER: u32 = 0x5c;

    pub const OUTER: u32 = 0x60;
}

// ---------------------------------------------------------------------------
// Layout of the graphics manager (`PS2GraphicsMan`, the particle/view
// manager these functions belong to).

pub(crate) mod man {

    /// Display object used for virtual queries (`lh +0x50/+0x58` slots).
    pub const DISPLAY: u32 = 0x10d8;

    /// Index of the current view in `views[]`.
    pub const CUR_VIEW: u32 = 0x10dc;

    /// Pointer to the camera matrix the view matrices are built from.
    pub const CAMERA: u32 = 0x13e4;

    /// The defer manager (scissor and matrix records).
    pub const DEFER: u32 = 0x18f0;

    /// World matrix (projection `WORLD * CAMERA` goes to `VIEW_PROJ`).
    pub const WORLD: u32 = 0x6af0;

    pub const VIEW_PROJ: u32 = 0x6b30;

    /// Scissor rectangle, key, and the two view-space depths.
    pub const SCISSOR: u32 = 0x5900;

    pub const F5908: u32 = 0x5908;

    pub const F590C: u32 = 0x590c;

    pub const F5910: u32 = 0x5910;

    pub const Q5920: u32 = 0x5920;

    /// Matrices: camera copy, local, product, view matrix.
    pub const CAM_MAT: u32 = 0x5780;

    pub const LOCAL_MAT: u32 = 0x57c0;

    pub const CAM_MAT2: u32 = 0x5800;

    pub const MAT5860: u32 = 0x5860;

    pub const VIEW_MAT: u32 = 0x58a0;

    /// Quads pushed with the view state.
    pub const Q6B70: u32 = 0x6b70;

    pub const Q6B80: u32 = 0x6b80;

    pub const Q5840: u32 = 0x5840;

    pub const Q5850: u32 = 0x5850;

    pub const Q58E0: u32 = 0x58e0;

    pub const Q58F0: u32 = 0x58f0;

    /// `k + depth` values written by `PS2GraphicsMan_v27`.
    pub const F5858: u32 = 0x5858;

    pub const F58E8: u32 = 0x58e8;

    pub const F58F8: u32 = 0x58f8;

    /// Viewport clamp mode and top/bottom ratios.
    pub const CLAMP_MODE: u32 = 0x6b94;

    pub const CLAMP_TOP: u32 = 0x6b98;

    pub const CLAMP_BOTTOM: u32 = 0x6b9c;

    /// Set when the view matrices are current.
    pub const TEX_READY: u32 = 0x6b90;

    /// Drawing mode (0x31 selects the large depth bias).
    pub const MODE: u32 = 0x5a40;

    /// `sPtView views[]`.
    pub const VIEWS: u32 = 0x6d20;

    pub const VIEW_STRIDE: u32 = 0x230;
}

/// One saved view (`sPtView`).
pub(crate) mod vw {

    pub const DEPTH: u32 = 0x1c;

    pub const F24: u32 = 0x24;

    pub const F28: u32 = 0x28;

    pub const F2C: u32 = 0x2c;

    pub const Q30: u32 = 0x30;

    pub const Q40: u32 = 0x40;

    pub const Q50: u32 = 0x50;

    pub const M60: u32 = 0x60;

    pub const MA0: u32 = 0xa0;

    pub const ME0: u32 = 0xe0;

    pub const M120: u32 = 0x120;

    pub const Q160: u32 = 0x160;

    pub const Q170: u32 = 0x170;

    pub const M180: u32 = 0x180;

    pub const M1C0: u32 = 0x1c0;

    pub const Q200: u32 = 0x200;

    pub const Q210: u32 = 0x210;

    /// Scissor rectangle (four shorts) and the defer-record key.
    pub const SCISSOR: u32 = 0x220;

    pub const KEY: u32 = 0x228;
}

/// The game's render-context singleton (`gp+0x2a90`, `D_004A5B80`): its
/// vtable (`+0x10d8`), the render-state stack pointer (`+0xe84`) and the
/// texture-id table (`+0xf50`).
#[allow(dead_code)]
pub(crate) mod ctx {
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::TEX_IDS;
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::TOP;
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::VTABLE;
}

/// `sAmCtx` (`D_004A289C`) used by `Rider_RenderModel`, and the world
/// pointer (`gp-0x848`) whose `+0x84 -> +0x84 -> +0x14` is the active rider.
#[allow(dead_code)]
pub(crate) mod globals {
    pub(crate) use crate::game::visualfx::boardwakefx::globals::AM_CTX;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::BONE_MATRICES;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::BONE_MATRICES_SHADOW;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::EMITTER_BLEND;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::FLAG_MANAGER;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::FLAG_STATE;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::FLAG_TABLE;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::IDENTITY_MAT;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::WORLD;
}

// ---------------------------------------------------------------------------
// Grid mesh.

/// `cPS2GridMesh` fields used when packing vertices.
pub(crate) mod grid_mesh {

    /// Number of quads across a row.
    pub const WIDTH: u32 = 0x190;

    /// Scale applied to every position.
    pub const SCALE: u32 = 0x198;

    /// Which row table (`+0x1b4` / `+0x1b8`) is current.
    pub const TAB_INDEX: u32 = 0x1a0;

    /// Set to 1 once the mesh has been packed.
    pub const PACKED: u32 = 0x1a4;

    /// Row tables (2 x pointers) and the row count.
    pub const ROW_TABS: u32 = 0x1b4;

    pub const ROWS: u32 = 0x1bc;
}

/// `Cell`/`BoaderShadow` object layout (the fields this function touches).
pub(crate) mod shadow {

    /// Input vector `b` normalised into place: `{x, y, z, 0}`.
    pub const VEC_B: u32 = 0x00;

    /// Reference vector `a`.
    pub const VEC_A: u32 = 0x0c;

    /// Normalised direction `dir`.
    pub const DIR: u32 = 0x18;

    /// `b . dir` (the dot product of the two normalised vectors).
    pub const DOT: u32 = 0x24;

    /// Result matrices (4x4 each, 16-byte aligned).
    pub const MAT_B: u32 = 0x28;

    pub const MAT_C: u32 = 0x34;

    pub const LIGHT: u32 = 0x50;

    pub const PROJECTION: u32 = 0x90;

    pub const VIEW: u32 = 0xd0;

    pub const BONES: u32 = 0x110;

    /// Six indices into the bone table, consumed by the gather loop.
    pub const BONE_INDEX: u32 = 0x17c;

    /// Depth scale / bias inputs (integers).
    pub const SCALE_IN: u32 = 0x174;

    pub const BIAS_IN: u32 = 0x178;

    pub const SCALE: u32 = 0x150;

    pub const DEPTH: u32 = 0x158;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::STRLEN;
}

// ---------------------------------------------------------------------------
// Vector math.

/// `void Vec3_AddTo(Vec3 *dst, const Vec3 *src)` (0x389558): `dst += src`,
/// component by component, in the original's load/store order (so overlapping
/// vectors behave identically).
pub fn vec3_add_to(c: &mut Cpu) {
    let dst = Addr(abi::arg(c, 0));
    let src = Addr(abi::arg(c, 1));
    c.cycles += 13;
    let s0 = src.read_fpu(c);
    let d0 = dst.read_fpu(c);
    let d1 = dst.off(0x4).read_fpu(c);
    let r0 = d0 + s0;
    let d2 = dst.off(0x8).read_fpu(c);
    dst.write_fpu(c, r0);
    let s1 = src.off(0x4).read_fpu(c);
    let r1 = d1 + s1;
    dst.off(0x4).write_fpu(c, r1);
    let s2 = src.off(0x8).read_fpu(c);
    let r2 = d2 + s2;
    dst.off(0x8).write_fpu(c, r2);
    // Scratch registers the original leaves behind (f0/f1 are compared).
    abi::set_fpr(c, 0, s2);
    abi::set_fpr(c, 1, s1);
    abi::set_fpr(c, 2, r1);
    abi::set_fpr(c, 3, r2);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Bezier cache / border shadow

/// `BezierPatchCache_ctor(BezierPatchCache *self)` (0x372b30): clear the
/// three-word list at +0xc.
pub fn bezier_patch_cache_ctor(c: &mut Cpu) {
    c.cycles += 17;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    let this = Addr(abi::arg(c, 0));
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    c.g[V0][0] = c.g[A0][0];
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `void BoaderShadow_SetParams(BoaderShadow *s, uint a, uint b, uint c,
/// uint d, uint e)` (0x374ce0): store the five words at +0x190 (first) and
/// +0x17c..+0x18c (the rest, in order).
pub fn boader_shadow_set_params(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    this.off(0x190).sw(c, c.g[T2][0]);
    this.off(0x17c).sw(c, c.g[A1][0]);
    this.off(0x180).sw(c, c.g[A2][0]);
    this.off(0x184).sw(c, c.g[A3][0]);
    this.off(0x188).sw(c, c.g[T0][0]);
    this.off(0x18c).sw(c, c.g[T1][0]);
    abi::ret(c);
}

/// `BoaderShadow_ctor(BoaderShadow *s)` (0x3758f8): initialise the shadow
/// (count -1, extent 0x80) and leave `self` in v0.
pub fn boader_shadow_ctor(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] = 0x80;
    let this = Addr(abi::arg(c, 0));
    this.off(0x170).sw(c, c.g[V0][0]);
    this.off(0x178).sw(c, c.g[V1][0]);
    c.g[V0][0] = c.g[A0][0];
    this.off(0x174).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Detail system

/// `DetailSystem_ctor(DetailSystem *s)` (0x3759e8): install the vtable at
/// 0x493000.
pub fn detail_system_ctor(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x49_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(0x49_0000u32.wrapping_add(0x3000));
    Addr(abi::arg(c, 0)).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Line strips

/// `void LineStrip_Init(LineStrip *s, uint *buffer)` (0x384d98): store the
/// buffer pointer, its first word tagged with 0x30000000, and clear the counts.
pub fn line_strip_init(c: &mut Cpu) {
    c.cycles += 9;
    let this = Addr(abi::arg(c, 0));
    let buffer = abi::arg64(c, 1);
    this.sw(c, buffer);
    c.g[A2][0] = 0x3000_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = Addr(buffer as u32).lw(c);
    this.off(0x8).sw(c, 0);
    c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
    this.off(0xc).sw(c, 0);
    this.off(0x4).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void LineStrip_Close(LineStrip *s)` (0x384dc0): optionally append a
/// closing vertex, then write the 0x30-byte primitive header into the buffer.
pub fn line_strip_close(c: &mut Cpu) {
    c.cycles += 5;
    let t0 = c.g[A0][0] as u32;
    c.g[T0][0] = c.g[A0][0];
    c.g[V0][0] = Addr(t0).off(0x10).lw(c);
    c.g[V0][0] = c.g[V0][0] & 0x1;
    if c.g[V0][0] == 0 {
        c.g[V1][0] = Addr(t0).off(0xc).lw(c);
    } else {
        c.cycles += 5;
        c.g[V0][0] = Addr(t0).off(0x4).lw(c);
        c.sd(c.g[V0][0] as u32, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
        Addr(t0).off(0x4).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(t0).off(0xc).lw(c);
    }
    c.cycles += 25;
    c.g[V0][0] = 0x1400_0000;
    c.g[V0][0] = c.g[V0][0] << 32;
    c.g[V0][0] = c.g[V0][0] | 0x1;
    c.g[A1][0] = 0x4a;
    c.g[A2][0] = 0xd000;
    c.g[A2][0] = c.g[A2][0] << 46;
    c.sd(c.g[V1][0] as u32, c.g[V0][0]);
    c.g[A3][0] = 0x521;
    c.g[V0][0] = Addr(t0).off(0xc).lw(c);
    c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0);
    c.g[V1][0] = Addr(t0).off(0xc).lw(c);
    c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
    c.g[A0][0] = Addr(t0).off(0xc).lw(c);
    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), 0);
    c.g[V0][0] = Addr(t0).off(0x10).lw(c);
    c.g[A0][0] = Addr(t0).off(0xc).lw(c);
    c.g[V0][0] = c.g[V0][0] | 0x8000;
    c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
    c.g[V1][0] = Addr(t0).off(0xc).lw(c);
    c.sd((c.g[V1][0] as u32).wrapping_add(0x28), c.g[A3][0]);
    c.g[V0][0] = Addr(t0).off(0xc).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
    Addr(t0).off(0xc).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void LineStrip_AddVertex(LineStrip *s, void *vtx, void *colour)`
/// (0x384fd0): pack the vertex fields (position, colour, UV) into the buffer,
/// scaling by the colour at `colour + 0xc`.
pub fn line_strip_add_vertex(c: &mut Cpu) {
    c.cycles += 53;
    let a0 = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    c.g[V1][0] = Addr(a1).off(0x14).lwu(c);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = Addr(a1).off(0x10).lwu(c);
    c.g[A3][0] = Addr(a1).off(0x18).lwu(c);
    c.g[V1][0] = c.g[V1][0] << 8;
    c.g[T0][0] = Addr(a1).off(0x1c).lwu(c);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V1][0] = Addr(a2).off(0xc).lw(c);
    c.g[A3][0] = c.g[A3][0] << 16;
    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
    c.g[T0][0] = c.g[T0][0] << 24;
    c.g[A3][0] = Addr(a0).off(0x4).lw(c);
    c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
    c.g[V1][0] = c.g[V1][0] << 32;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd(c.g[A3][0] as u32, c.g[V0][0]);
    let colour = Addr(a2).off(0xc).read_fpu(c);
    c.f[0] = colour.0;
    let x = Addr(a1).read_fpu(c);
    c.f[2] = x.0;
    let y = Addr(a1).off(0x4).read_fpu(c);
    c.f[1] = y.0;
    let scaled_x = x * colour;
    c.f[2] = scaled_x.0;
    c.g[A3][0] = Addr(a0).off(0x4).lw(c);
    let scaled_y = y * colour;
    c.f[1] = scaled_y.0;
    Addr(c.g[SP][0] as u32).write_fpu(c, scaled_x);
    c.g[A1][0] = abi::mfc1(scaled_y);
    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
    c.g[V0][0] = c.g[A1][0] << 32;
    c.g[V0][0] = c.g[V1][0] | c.g[V0][0];
    c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    let colour = Addr(a2).off(0xc).read_fpu(c);
    c.f[1] = colour.0;
    let w = Addr(a2).off(0x8).read_fpu(c);
    c.f[0] = w.0;
    let z = Addr(a2).off(0x4).read_fpu(c);
    c.f[2] = z.0;
    let raw_w = Addr(a2).read_fpu(c);
    c.f[3] = raw_w.0;
    let scaled_w = w * colour;
    c.f[0] = scaled_w.0;
    let zi = z.to_i32();
    c.f[1] = zi as u32;
    c.g[V0][0] = sx(zi as u32);
    c.g[A2][0] = Addr(a0).off(0x4).lw(c);
    let wi = raw_w.to_i32();
    c.f[1] = wi as u32;
    c.g[A1][0] = sx(wi as u32);
    let packed = scaled_w.to_i32();
    c.f[1] = packed as u32;
    c.g[V1][0] = sx(packed as u32);
    c.g[V0][0] = c.g[V0][0] << 16;
    c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
    c.g[V1][0] = c.g[V1][0] << 32;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    c.g[V1][0] = Addr(a0).off(0x4).lw(c);
    c.g[V0][0] = Addr(a0).off(0x10).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x18));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    Addr(a0).off(0x4).sw(c, c.g[V1][0]);
    Addr(a0).off(0x10).sw(c, c.g[V0][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Grid mesh packing

/// `void cPS2GridMesh_PackPosition(cPS2GridMesh *m, int base, int x, int y,
/// const float *pos)` (0x385a38): scale the three floats at `pos` by the
/// matrix scale at `this + 0x198` and store the three 16-bit results at
/// `base + (x*3<<1) + y*2`... (`base + x*6 + base_offset`).
pub fn cps2_grid_mesh_pack_position(c: &mut Cpu) {
    c.cycles += 26;
    let a0 = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    let a3 = c.g[A3][0] as u32;
    let scale = Addr(a0).off(0x198).read_fpu(c);
    c.f[3] = scale.0;
    c.g[V0][0] = sx(a2 << 1);
    let z = Addr(a3).off(0x8).read_fpu(c);
    c.f[0] = z.0;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(a2));
    let x = Addr(a3).read_fpu(c);
    c.f[1] = x.0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    let y = Addr(a3).off(0x4).read_fpu(c);
    c.f[2] = y.0;
    let zs = z * scale;
    c.f[0] = zs.0;
    let xs = x * scale;
    c.f[1] = xs.0;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(a1));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    let ys = y * scale;
    c.f[2] = ys.0;
    let sp = c.g[SP][0] as u32;
    Addr(sp).off(0x8).write_fpu(c, zs);
    let zi = zs.to_i32();
    c.f[3] = zi as u32;
    c.g[A1][0] = sx(zi as u32);
    let xi = xs.to_i32();
    c.f[0] = xi as u32;
    c.g[A0][0] = sx(xi as u32);
    Addr(sp).write_fpu(c, xs);
    let yi = ys.to_i32();
    c.f[0] = yi as u32;
    c.g[V1][0] = sx(yi as u32);
    Addr(sp).off(0x4).write_fpu(c, ys);
    let base = c.g[V0][0] as u32;
    Addr(base).off(0x4).sh(c, c.g[A1][0]);
    Addr(base).sh(c, c.g[A0][0]);
    Addr(base).off(0x2).sh(c, c.g[V1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

/// `void cPS2GridMesh_PackUV(uint *dst, int base, int index, const float *uv)`
/// (0x385ba0): scale the two floats at `uv` by 4096 and store them as 16-bit
/// at `base + index*4`.
pub fn cps2_grid_mesh_pack_uv(c: &mut Cpu) {
    c.cycles += 15;
    c.g[AT][0] = 0x4580_0000;
    c.f[1] = c.g[AT][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    let a3 = c.g[A3][0] as u32;
    let dst = a1.wrapping_add(a2.wrapping_shl(2));
    c.g[A2][0] = sx((a2 << 2).wrapping_add(a1));
    let u = Addr(a3).read_fpu(c);
    let scaled_u = u * FpuF32(0x4580_0000);
    c.f[0] = scaled_u.0;
    let ui = scaled_u.to_i32();
    c.f[2] = ui as u32;
    c.g[V0][0] = sx(ui as u32);
    Addr(dst).sh(c, c.g[V0][0]);
    let v = Addr(a3).off(0x4).read_fpu(c);
    let scaled_v = v * FpuF32(0x4580_0000);
    c.f[0] = scaled_v.0;
    let vi = scaled_v.to_i32();
    c.f[1] = vi as u32;
    c.g[V0][0] = sx(vi as u32);
    Addr(dst).off(0x2).sh(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Irradiance

/// `bool Irradiance_SpotLightVector(Vec3 *out, Vec3 *surf, const Light *l,
/// float *atten, float *spot)` (0x38a530): compute the normalised direction and
/// scaled colour, returning false when the surface is outside the spot cone.
pub fn irradiance_spot_light_vector(c: &mut Cpu) {
    c.cycles += 20;
    let a0 = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    let a3 = c.g[A3][0] as u32;
    let t0 = c.g[T0][0] as u32;
    let t1 = c.g[T1][0] as u32;
    c.f[2] = Addr(a2).off(0x38).read_fpu(c).0;
    c.f[1] = Addr(a0).read_fpu(c).0;
    c.f[3] = Addr(a2).off(0x3c).read_fpu(c).0;
    c.f[0] = Addr(a0).off(0x4).read_fpu(c).0;
    c.f[8] = (FpuF32(c.f[2]) - FpuF32(c.f[1])).0;
    c.f[2] = Addr(a2).off(0x40).read_fpu(c).0;
    c.f[7] = (FpuF32(c.f[3]) - FpuF32(c.f[0])).0;
    c.f[4] = Addr(a0).off(0x8).read_fpu(c).0;
    c.f[1] = (FpuF32(c.f[8]) * FpuF32(c.f[8])).0;
    c.f[0] = Addr(a2).off(0x1c).read_fpu(c).0;
    c.f[6] = (FpuF32(c.f[2]) - FpuF32(c.f[4])).0;
    c.f[3] = (FpuF32(c.f[7]) * FpuF32(c.f[7])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[0])).0;
    c.f[2] = (FpuF32(c.f[6]) * FpuF32(c.f[6])).0;
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[3])).0;
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[2])).0;
    abi::set_fcc(c, FpuF32(c.f[0]).lt(FpuF32(c.f[1])));
    c.g[V0][0] = 0;
    if c.fcc() {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.f[0] = c.g[0][0] as u32;
    abi::set_fcc(c, FpuF32(c.f[1]).eq_s(FpuF32(c.f[0])));
    if c.fcc() {
        c.cycles += 4;
        c.g[AT][0] = 0x3f80_0000;
        c.f[4] = c.g[1][0] as u32;
        c.f[5] = c.f[4];
        c.f[8] = (FpuF32(c.f[8]) * FpuF32(c.f[4])).0;
    } else {
        c.cycles += 6;
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = c.g[1][0] as u32;
        c.f[5] = FpuF32(c.f[1]).sqrt().0;
        c.f[4] = FpuF32(c.f[0]).div(FpuF32(c.f[5])).0;
        c.f[8] = (FpuF32(c.f[8]) * FpuF32(c.f[4])).0;
    }
    c.cycles += 22;
    c.f[0] = Addr(a2).off(0x2c).read_fpu(c).0;
    c.f[7] = (FpuF32(c.f[7]) * FpuF32(c.f[4])).0;
    c.f[2] = Addr(a2).off(0x30).read_fpu(c).0;
    c.f[6] = (FpuF32(c.f[6]) * FpuF32(c.f[4])).0;
    c.f[1] = Addr(a2).off(0x34).read_fpu(c).0;
    c.f[0] = (FpuF32(c.f[8]) * FpuF32(c.f[0])).0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[3] = c.g[1][0] as u32;
    c.f[2] = (FpuF32(c.f[7]) * FpuF32(c.f[2])).0;
    c.g[V0][0] = 0x1;
    c.f[1] = (FpuF32(c.f[6]) * FpuF32(c.f[1])).0;
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[2])).0;
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
    c.f[0] = (-FpuF32(c.f[0])).0;
    Addr(t1).write_fpu(c, FpuF32(c.f[0]));
    Addr(a3).write_fpu(c, FpuF32(c.f[5]));
    Addr(t0).write_fpu(c, FpuF32(c.f[4]));
    Addr(a1).off(0xc).write_fpu(c, FpuF32(c.f[3]));
    Addr(a1).write_fpu(c, FpuF32(c.f[8]));
    Addr(a1).off(0x4).write_fpu(c, FpuF32(c.f[7]));
    Addr(a1).off(0x8).write_fpu(c, FpuF32(c.f[6]));
    abi::ret(c);
}

/// `bool Irradiance_PointLightVector(Vec3 *out, Vec3 *surf, const Light *l,
/// float *atten, float *dist)` (0x38a618): normalise the surface direction and
/// scale the colour, returning false when the surface is too far away.
pub fn irradiance_point_light_vector(c: &mut Cpu) {
    c.cycles += 19;
    let a0 = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    let a3 = c.g[A3][0] as u32;
    let t0 = c.g[T0][0] as u32;
    c.f[2] = Addr(a2).off(0x38).read_fpu(c).0;
    c.f[1] = Addr(a0).read_fpu(c).0;
    c.f[3] = Addr(a2).off(0x3c).read_fpu(c).0;
    c.f[0] = Addr(a0).off(0x4).read_fpu(c).0;
    c.f[6] = (FpuF32(c.f[2]) - FpuF32(c.f[1])).0;
    c.f[2] = Addr(a0).off(0x8).read_fpu(c).0;
    c.f[5] = (FpuF32(c.f[3]) - FpuF32(c.f[0])).0;
    c.f[4] = Addr(a2).off(0x40).read_fpu(c).0;
    c.f[1] = (FpuF32(c.f[6]) * FpuF32(c.f[6])).0;
    c.f[0] = Addr(a2).off(0x1c).read_fpu(c).0;
    c.f[4] = (FpuF32(c.f[4]) - FpuF32(c.f[2])).0;
    c.f[3] = (FpuF32(c.f[5]) * FpuF32(c.f[5])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[0])).0;
    c.f[2] = (FpuF32(c.f[4]) * FpuF32(c.f[4])).0;
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[3])).0;
    c.f[2] = (FpuF32(c.f[1]) + FpuF32(c.f[2])).0;
    abi::set_fcc(c, FpuF32(c.f[0]).lt(FpuF32(c.f[2])));
    c.g[V0][0] = 0;
    if c.fcc() {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 15;
    c.g[AT][0] = 0x3f80_0000;
    c.f[1] = c.g[1][0] as u32;
    c.f[0] = FpuF32(c.f[2]).sqrt().0;
    Addr(a3).write_fpu(c, FpuF32(c.f[0]));
    c.g[V0][0] = 0x1;
    c.f[0] = FpuF32(c.f[1]).div(FpuF32(c.f[0])).0;
    c.f[6] = (FpuF32(c.f[6]) * FpuF32(c.f[0])).0;
    Addr(t0).write_fpu(c, FpuF32(c.f[0]));
    c.f[5] = (FpuF32(c.f[5]) * FpuF32(c.f[0])).0;
    Addr(a1).off(0xc).write_fpu(c, FpuF32(c.f[1]));
    c.f[4] = (FpuF32(c.f[4]) * FpuF32(c.f[0])).0;
    Addr(a1).write_fpu(c, FpuF32(c.f[6]));
    Addr(a1).off(0x4).write_fpu(c, FpuF32(c.f[5]));
    Addr(a1).off(0x8).write_fpu(c, FpuF32(c.f[4]));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Small register helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// The address `reg + off`.
#[inline(always)]
pub(crate) fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lwc1 f, a`, returning the value.
#[inline(always)]
pub(crate) fn ldf(c: &mut Cpu, f: usize, a: u32) -> FpuF32 {
    c.lwc1(f, a);
    FpuF32(c.f[f])
}

/// FPU register `f`.
#[inline(always)]
pub(crate) fn fr(c: &Cpu, f: usize) -> FpuF32 {
    FpuF32(c.f[f])
}

/// Set FPU register `f`.
#[inline(always)]
pub(crate) fn setf(c: &mut Cpu, f: usize, v: FpuF32) {
    c.f[f] = v.0;
}

/// `lui at, hi(bits); mtc1 at, f`: a float constant through `at`.
#[inline(always)]
pub(crate) fn fconst(c: &mut Cpu, f: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = bits as u64;
    c.f[f] = bits;
    FpuF32(bits)
}

/// `c.lt.s a, b` leaving the condition bit.
#[inline(always)]
pub(crate) fn lt(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.lt(b);
    abi::set_fcc(c, r);
    r
}

/// `c.le.s a, b`.
#[inline(always)]
pub(crate) fn le(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.le(b);
    abi::set_fcc(c, r);
    r
}

/// `c.eq.s a, b`.
#[inline(always)]
pub(crate) fn eq(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.eq_s(b);
    abi::set_fcc(c, r);
    r
}

/// `x + y` of two sign-extended words (`addu`).
#[inline(always)]
pub(crate) fn add32(x: u64, y: u64) -> u64 {
    sx((x as u32).wrapping_add(y as u32))
}

/// `sll x, n` (32-bit, sign-extended).
#[inline(always)]
pub(crate) fn shl32(x: u64, n: u32) -> u64 {
    sx((x as u32) << n)
}

/// `sra x, n`.
#[inline(always)]
pub(crate) fn sra32(x: u64, n: u32) -> u64 {
    sx(((x as u32) as i32 >> n) as u32)
}

/// `srl x, n`.
#[inline(always)]
pub(crate) fn srl32(x: u64, n: u32) -> u64 {
    sx((x as u32) >> n)
}

/// `lq gpr, from; sq gpr, to` (a quadword moved through a GPR).
#[inline(always)]
pub(crate) fn copy_q(c: &mut Cpu, gpr: usize, from: u32, to: u32) {
    c.lq(gpr, from);
    c.sq(gpr, to);
}

/// `lqc2 vf3, a; lqc2 vf4, b; vadd vf5, vf3, vf4; sqc2 vf5, out`.
#[inline(always)]
pub(crate) fn vadd(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `lqc2 vf4, v; qmtc2 rt, vf3; vmulx vf5, vf4, vf3x; sqc2 vf5, out`:
/// a vector scaled by the float bits the original left in GPR `rt`.
#[inline(always)]
pub(crate) fn vscale(c: &mut Cpu, v: u32, rt: usize, out: u32) {
    c.lqc2(4, v);
    c.qmtc2(rt, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
}

/// The common epilogue tail: `addiu sp, sp, size; jr ra` with `ra` as it is.
#[inline(always)]
pub(crate) fn leave(c: &mut Cpu, size: u32) {
    let ra = r32(c, RA);
    c.g[SP][0] = sx(sp(c).wrapping_add(size));
    c.pc = ra;
}

/// `v0 = cursor * 4 + colours`: the colour slot of the current particle
/// (v0/v1 as the original leaves them).
#[inline(always)]
pub(crate) fn colour_slot(c: &mut Cpu, e: u32) -> u32 {
    c.g[V0][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V1][0] = c.lw(p(e, emitter::COLOURS));
    c.g[V0][0] = shl32(c.g[V0][0], 2);
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
    r32(c, V0)
}

/// One colour channel to a byte: `f3 = 0 <= v*255 ? min(v*255, 255) : 0`
/// (f0 = v*255, f1 = 255, f2 = 0 as the original leaves them).
#[inline(always)]
pub(crate) fn clamp_channel(c: &mut Cpu, v: FpuF32) {
    let scaled = v * fr(c, 1);
    setf(c, 0, scaled);
    let zero = fr(c, 2);
    let pos = le(c, zero, scaled);
    setf(c, 3, zero);
    if pos {
        c.cycles += 1;
        let m = scaled.min(fr(c, 1));
        setf(c, 3, m);
    }
}

/// `void DynamicEmitter_SpawnAt(Emitter* e, const Vec4* pos, const Vec4*
/// vel, const Vec4* colour /*a3*/, int flag /*t0*/, float dt /*f12*/)`
/// (0x3717c0): store the colour (or the emitter's default) as RGBA bytes
/// in the current particle's slot, then spawn it (`DynamicEmitter_Spawn`).
pub fn dynamic_emitter_spawn_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[T1][0] = c.g[A0][0];
        let e = r32(c, T1);
        frame.save_ra(c, 0);
        if c.g[A3][0] != 0 {
            c.cycles += 3;
            copy_q(c, V0, r32(c, A3), p(e, emitter::DEFAULT_COLOUR));
        } else {
            c.cycles += 1;
            c.g[A3][0] = sx(p(e, emitter::DEFAULT_COLOUR));
        }
        let colour = r32(c, A3);
        c.cycles += 9;
        let alpha = ldf(c, 0, p(colour, 0xc));
        fconst(c, 1, 0x437f_0000);
        setf(c, 2, FpuF32::ZERO);
        clamp_channel(c, alpha);
        // (byte offset in the slot, component of the colour to clamp next)
        for (byte, next) in [(3u32, 0u32), (0, 4), (1, 8)] {
            c.cycles += 13;
            c.cvt_w_s(0, 3);
            c.mfc1(A0, 0);
            let slot = colour_slot(c, e);
            let v = c.g[A0][0];
            c.sb(p(slot, byte), v);
            let comp = ldf(c, 0, p(colour, next));
            clamp_channel(c, comp);
        }
        c.cycles += 13;
        c.cvt_w_s(0, 3);
        c.mfc1(A3, 0);
        c.g[V0][0] = c.lw(p(e, emitter::CURSOR));
        c.g[V1][0] = c.lw(p(e, emitter::COLOURS));
        c.g[A0][0] = c.g[T1][0];
        c.g[V0][0] = shl32(c.g[V0][0], 2);
        c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
        let slot = r32(c, V0);
        let v = c.g[A3][0];
        c.sb(p(slot, 2), v);
        c.g[A3][0] = c.g[T0][0];
        callee::DYNAMIC_EMITTER_SPAWN.call(c, 0x0037_18d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DynamicEmitter_Spawn(Emitter* e, const Vec4* pos, const Vec4* vel,
/// int jitter /*a3*/, float dt /*f12*/)` (0x3710d0): if enabled, write the
/// particle at the ring cursor (position, velocity with w 0, or a random
/// [1, 2) when `jitter`), keep the live count, move the cursor back and
/// advance the emitter's clock.
pub fn dynamic_emitter_spawn(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save_fpr(c, 20, 0x30);
        c.g[S0][0] = c.g[A0][0];
        let e = r32(c, S0);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(p(e, emitter::ENABLED));
        c.mov_s(20, 12);
        if c.g[V0][0] != 0 {
            spawn_particle(c, e)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// The body of [`dynamic_emitter_spawn`] for an enabled emitter.
pub(crate) fn spawn_particle(c: &mut Cpu, e: u32) -> Flow {
    // Was the slot's previous particle alive (velocity w > 0)?
    c.cycles += 10;
    c.g[V0][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V1][0] = c.lw(p(e, emitter::VELOCITIES));
    c.g[V0][0] = shl32(c.g[V0][0], 4);
    setf(c, 1, FpuF32::ZERO);
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
    let age = ldf(c, 0, at(c, V0, 0xc));
    let alive = lt(c, FpuF32::ZERO, age);
    c.g[A0][0] = 1;
    if !alive {
        c.cycles += 1;
        c.g[A0][0] = 0;
    }
    // Live count: a reused live slot without jitter -1, a dead slot with
    // jitter +1.
    c.cycles += 2;
    let mut store = false;
    if c.g[A0][0] != 0 {
        c.cycles += 2;
        if c.g[A3][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(e, emitter::LIVE));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            store = true;
        }
    } else {
        c.cycles += 2;
        if c.g[A3][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(p(e, emitter::LIVE));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            store = true;
        }
    }
    if store {
        c.cycles += 1;
        let v = c.g[V0][0];
        c.sw(p(e, emitter::LIVE), v);
    }
    // Default position / velocity, or copy the given ones as the defaults.
    c.cycles += 2;
    if c.g[A1][0] == 0 {
        c.g[A1][0] = sx(p(e, emitter::DEFAULT_POS));
    } else {
        c.cycles += 2;
        copy_q(c, V0, r32(c, A1), p(e, emitter::DEFAULT_POS));
    }
    c.cycles += 2;
    if c.g[A2][0] == 0 {
        c.g[A2][0] = sx(p(e, emitter::DEFAULT_VEL));
    } else {
        c.cycles += 2;
        copy_q(c, V0, r32(c, A2), p(e, emitter::DEFAULT_VEL));
    }
    // positions[cursor] = pos, w = 0; velocities[cursor] = vel.
    c.cycles += 22;
    c.g[V1][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V0][0] = c.lw(p(e, emitter::POSITIONS));
    c.lq(A0, r32(c, A1));
    c.g[V1][0] = shl32(c.g[V1][0], 4);
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.sq(A0, r32(c, V1));
    c.g[V0][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V1][0] = c.lw(p(e, emitter::POSITIONS));
    c.g[V0][0] = shl32(c.g[V0][0], 4);
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
    c.sw(at(c, V0, 0xc), 0);
    c.g[V1][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V0][0] = c.lw(p(e, emitter::VELOCITIES));
    c.g[V1][0] = shl32(c.g[V1][0], 4);
    c.lq(A0, r32(c, A2));
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.sq(A0, r32(c, V1));
    c.g[V0][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V1][0] = c.lw(p(e, emitter::VELOCITIES));
    c.g[V0][0] = shl32(c.g[V0][0], 4);
    let jitter = c.g[A3][0] != 0;
    c.g[S1][0] = add32(c.g[V0][0], c.g[V1][0]);
    let vel = r32(c, S1);
    if jitter {
        // w = a float in [1, 2) from the random mantissa, minus 1, plus 1
        // (as the original computes it).
        c.cycles += 14;
        call_ptr(c, callee::RANDOM, 0x0037_11d0)?;
        c.g[V1][0] = 0x007f_ffff;
        c.g[A1][0] = 0x3f80_0000;
        let one = fconst(c, 1, 0x3f80_0000);
        c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | c.g[A1][0];
        let r = FpuF32(r32(c, V0));
        setf(c, 2, r);
        let age = r - one + one;
        setf(c, 0, age);
        c.swc1(0, p(vel, 0xc));
    } else {
        c.cycles += 1;
        c.sw(p(vel, 0xc), 0);
    }
    // cursor-- (wrap to capacity - 1).
    c.cycles += 4;
    c.g[V0][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
    let v = c.g[V0][0];
    c.sw(p(e, emitter::CURSOR), v);
    if (v as i64) < 0 {
        c.cycles += 3;
        c.g[V0][0] = c.lw(p(e, emitter::CAPACITY));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let v = c.g[V0][0];
        c.sw(p(e, emitter::CURSOR), v);
    }
    // time += rate * dt, back to 0 once (int)time reaches the period.
    c.cycles += 10;
    let rate = ldf(c, 0, p(e, emitter::RATE));
    let time = ldf(c, 1, p(e, emitter::TIME));
    let step = rate * fr(c, 20);
    setf(c, 0, step);
    c.g[V1][0] = c.lw(p(e, emitter::PERIOD));
    let time = time + step;
    setf(c, 1, time);
    c.cvt_w_s(0, 1);
    c.mfc1(V0, 0);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.swc1(1, p(e, emitter::TIME));
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.sw(p(e, emitter::TIME), 0);
    }
    Ok(())
}

/// Head of the texel loop (event checks).
pub(crate) const V58_TEXELS: u32 = 0x0038_8cd8;

/// Head of the (empty) counting loop before it.
pub(crate) const V58_SPIN: u32 = 0x0038_8cb0;

/// `sext(x)`: `dsll32; dsra32` (sign-extend the low word).
#[inline(always)]
pub(crate) fn sext(x: u64) -> u64 {
    sx(x as u32)
}

/// `1 << ((ld tex0 >> shift) & 0xf)` as the original computes it, leaving
/// v0 = the exponent and `reg` = the size. `tex0` is the doubleword
/// (a3 or a fresh `ld`).
#[inline(always)]
pub(crate) fn tex_size(c: &mut Cpu, tex0: u64, shift: u32, one: usize, reg: usize) -> u64 {
    c.g[V0][0] = sext((tex0 >> shift) & 0xf);
    let n = c.g[V0][0] as u32 & 31;
    c.g[reg][0] = sx((c.g[one][0] as u32) << n);
    c.g[reg][0]
}

/// `if (v < 0) v = 0; else if (size < v) v = size;` on GPR `reg` (`size`
/// in v1), with the original's cycles.
#[inline(always)]
pub(crate) fn clamp_index(c: &mut Cpu, reg: usize) {
    if (c.g[reg][0] as i64) < 0 {
        c.cycles += 1;
        c.g[reg][0] = 0;
    } else {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[reg][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[reg][0] = c.g[V1][0];
        }
    }
}

/// `Colour* PS2GraphicsMan_v58(Colour* out, PS2GraphicsMan* gm, int tex,
/// float u /*f12*/, float v /*f13*/)` (0x3889f0): bilinear sample of
/// texture `tex` at (u, v) (wrapped to [0, 1)), as four floats 0..255
/// scaled by a constant (gp-0x2730); white when the texture slot is empty.
/// Returns `out`.
pub fn ps2_graphics_man_v58(c: &mut Cpu) {
    c.cycles += 8;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0xe0));
    c.g[A2][0] = shl32(c.g[A2][0], 2);
    let sp = sp(c);
    c.sq(S0, p(sp, 0xd0));
    c.g[V0][0] = c.lw(at(c, A1, 0x18f4));
    c.g[V0][0] = add32(c.g[V0][0], c.g[A2][0]);
    c.g[T6][0] = c.lw(at(c, V0, 8));
    c.g[S0][0] = c.g[A0][0];
    let out = r32(c, S0);
    if c.g[T6][0] == 0 {
        c.cycles += 7;
        fconst(c, 0, 0x3f80_0000);
        c.sw(p(out, 8), 0);
        c.swc1(0, p(out, 0xc));
        c.swc1(0, out);
        c.swc1(0, p(out, 4));
    } else {
        v58_sample(c, sp, out);
    }
    c.cycles += 4;
    c.g[V0][0] = c.g[S0][0];
    c.lq(S0, p(sp, 0xd0));
    leave(c, 0xe0);
}

/// The texel coordinates: `f5`/`f12` = u*w -/+ 0.5 and `f13`/`f1` = v*h
/// -/+ 0.5, each clamped to [0, size]; their integer parts (clamped again)
/// at sp+0, 4, 8, 0xc.
pub(crate) fn v58_coords(c: &mut Cpu, sp: u32) {
    let tex = r32(c, T6);
    c.cycles += 40;
    c.g[V1][0] = c.ld(p(tex, texture::TEX0));
    c.cvt_w_s(0, 12);
    c.mfc1(A1, 0);
    c.cvt_w_s(0, 13);
    c.mfc1(A2, 0);
    c.g[A0][0] = 1;
    c.g[V0][0] = c.g[V1][0] >> 26;
    c.g[A3][0] = c.g[V1][0];
    c.mtc1(A1, 0);
    c.cvt_s_w(0, 0);
    c.g[V0][0] = sext(c.g[V0][0] & 0xf);
    c.mtc1(A2, 1);
    c.cvt_s_w(1, 1);
    c.g[V0][0] = sx(1u32 << (c.g[V0][0] as u32 & 31));
    c.g[V1][0] = sext((c.g[V1][0] >> 30) & 0xf);
    // fractional parts
    let fu = fr(c, 12) - fr(c, 0);
    setf(c, 12, fu);
    c.mtc1(V0, 2);
    c.cvt_s_w(2, 2);
    let fv = fr(c, 13) - fr(c, 1);
    setf(c, 13, fv);
    c.g[A0][0] = sx(1u32 << (c.g[V1][0] as u32 & 31));
    let half = fconst(c, 0, 0x3f00_0000);
    c.mtc1(A0, 1);
    c.cvt_s_w(1, 1);
    let width = fr(c, 2);
    let su = fu * width;
    c.g[T8][0] = c.lw(p(tex, texture::PIXELS));
    setf(c, 3, FpuF32::ZERO);
    let sv = fv * fr(c, 1);
    let u0 = su - half;
    setf(c, 5, u0);
    setf(c, 12, su + half);
    setf(c, 1, sv + half);
    let neg = lt(c, u0, FpuF32::ZERO);
    setf(c, 13, sv - half);
    if neg {
        c.cycles += 2;
        setf(c, 5, FpuF32::ZERO);
    } else {
        c.cycles += 4;
        if lt(c, width, u0) {
            setf(c, 5, width);
        }
    }
    // u1 in [0, w]
    c.cycles += 5;
    setf(c, 0, FpuF32::ZERO);
    if lt(c, fr(c, 12), FpuF32::ZERO) {
        setf(c, 12, FpuF32::ZERO);
    } else {
        c.cycles += 12;
        let tex0 = c.g[A3][0];
        c.g[V1][0] = 1;
        tex_size(c, tex0, 26, V1, V1);
        c.mtc1(V1, 0);
        c.cvt_s_w(0, 0);
        if lt(c, fr(c, 0), fr(c, 12)) {
            c.mov_s(12, 0);
        }
        c.cycles += 1;
        setf(c, 0, FpuF32::ZERO);
    }
    // v0 in [0, h]
    c.cycles += 4;
    if lt(c, fr(c, 13), fr(c, 0)) {
        c.mov_s(13, 0);
    } else {
        c.cycles += 12;
        let tex0 = c.g[A3][0];
        c.g[V1][0] = 1;
        tex_size(c, tex0, 30, V1, V1);
        c.mtc1(V1, 0);
        c.cvt_s_w(0, 0);
        if lt(c, fr(c, 0), fr(c, 13)) {
            c.mov_s(13, 0);
        }
    }
    // v1 in [0, h]
    c.cycles += 5;
    setf(c, 0, FpuF32::ZERO);
    if lt(c, fr(c, 1), FpuF32::ZERO) {
        c.mov_s(1, 0);
        c.cycles += 1;
        c.g[V0][0] = c.g[A3][0] >> 26;
    } else {
        c.cycles += 12;
        let tex0 = c.g[A3][0];
        c.g[V1][0] = 1;
        tex_size(c, tex0, 30, V1, V1);
        c.mtc1(V1, 0);
        c.cvt_s_w(0, 0);
        let over = lt(c, fr(c, 0), fr(c, 1));
        c.g[V0][0] = c.g[A3][0] >> 26;
        if over {
            c.cycles += 1;
            c.mov_s(1, 0);
            c.cycles += 1;
            c.g[V0][0] = c.g[A3][0] >> 26;
        }
    }
    // Integer coordinates, clamped to the size: x0, x1, y0, y1.
    c.cycles += 8;
    c.cvt_w_s(0, 5);
    c.mfc1(A0, 0);
    c.g[V0][0] = sext(c.g[V0][0] & 0xf);
    c.g[V1][0] = sx(1u32 << (c.g[V0][0] as u32 & 31));
    clamp_index(c, A0);
    for (store_off, f, reg, shift) in [(0u32, 12usize, A1, 26u32), (4, 13, A0, 30), (8, 1, A0, 30)]
    {
        c.cycles += 11;
        let prev = if store_off == 4 { A1 } else { A0 };
        let v = c.g[prev][0];
        c.sw(p(sp, store_off), v);
        if store_off == 8 {
            c.g[V1][0] = 1;
            c.cvt_w_s(0, f);
            c.mfc1(reg, 0);
        } else {
            c.cvt_w_s(0, f);
            c.mfc1(reg, 0);
            c.g[V1][0] = 1;
        }
        let tex0 = c.ld(p(r32(c, T6), texture::TEX0));
        c.g[V0][0] = tex0;
        tex_size(c, tex0, shift, V1, V1);
        clamp_index(c, reg);
    }
}

/// The body of [`ps2_graphics_man_v58`] for a present texture.
pub(crate) fn v58_sample(c: &mut Cpu, sp: u32, out: u32) {
    v58_coords(c, sp);
    c.cycles += 5;
    c.g[A2][0] = 0x7fff_ffff;
    let v = c.g[A0][0];
    c.sw(p(sp, 0xc), v);
    c.g[V0][0] = 3;
    c.g[V1][0] = u64::MAX;
    // A counting loop with an empty body (four passes).
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let more = c.g[V0][0] != c.g[V1][0];
        c.g[T7][0] = 0;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, V58_SPIN);
        }
    }
    // The four texels (x0,y0), (x0,y1), (x1,y0), (x1,y1) as floats at
    // sp+0x10.. (t5).
    c.cycles += 3;
    c.g[T9][0] = 1;
    c.g[T5][0] = sx(p(sp, 0x10));
    loop {
        v58_texel(c, sp);
        c.cycles += 21;
        let colour = c.g[A2][0];
        c.g[A0][0] = sra32(colour, 8);
        c.g[V0][0] = srl32(colour, 24);
        c.g[V1][0] &= 0xff;
        c.g[A0][0] &= 0xff;
        c.g[A1][0] = colour & 0xff;
        c.mtc1(V0, 3);
        c.cvt_s_w(3, 3);
        c.g[T7][0] = add32(c.g[T7][0], 1);
        c.mtc1(V1, 2);
        c.cvt_s_w(2, 2);
        c.g[V0][0] = ((c.g[T7][0] as i64) < 4) as u64;
        c.mtc1(A0, 0);
        c.cvt_s_w(0, 0);
        c.mtc1(A1, 1);
        c.cvt_s_w(1, 1);
        let dst = r32(c, T5);
        c.swc1(3, p(dst, 0xc));
        c.swc1(2, dst);
        c.swc1(0, p(dst, 4));
        c.swc1(1, p(dst, 8));
        c.g[T5][0] = sx(p(dst, 0x10));
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, V58_TEXELS);
        }
    }
    v58_blend(c, sp, out);
}

/// Fetch texel `t7` (x from sp+0/4, y from sp+8/0xc) into a2 (0xAARRGGBB
/// style word) and v1 (`a2 >> 16`). Unknown formats leave a2 as it was.
pub(crate) fn v58_texel(c: &mut Cpu, sp: u32) {
    let tex = r32(c, T6);
    c.cycles += 11;
    let i = c.g[T7][0];
    c.g[V0][0] = i & 1;
    c.g[V1][0] = sra32(i, 1);
    c.g[V0][0] = add32(c.g[V0][0], 2);
    c.g[V1][0] = shl32(c.g[V1][0], 2);
    c.g[V0][0] = shl32(c.g[V0][0], 2);
    c.g[A0][0] = add32(sp as u64, c.g[V1][0]);
    c.g[A1][0] = add32(sp as u64, c.g[V0][0]);
    c.g[T3][0] = c.lw(p(tex, texture::PSM));
    c.g[T4][0] = c.lw(r32(c, A0));
    c.g[T2][0] = c.lw(r32(c, A1));
    let psm = c.g[T3][0];
    if psm == 0 {
        // 32-bit: pixels[y * w + x].
        c.cycles += 11;
        let tex0 = c.ld(p(tex, texture::TEX0));
        tex_size(c, tex0, 26, T9, V0);
        c.mult(0, V1, T2, V0);
        c.g[V0][0] = add32(c.g[V1][0], c.g[T4][0]);
        c.g[V0][0] = shl32(c.g[V0][0], 2);
        c.g[V0][0] = add32(c.g[V0][0], c.g[T8][0]);
    } else {
        c.cycles += 3;
        c.g[V0][0] = 0xa;
        if psm == 0xa {
            v58_texel_psmt8(c, tex);
        } else {
            c.g[V0][0] = 0xc;
            c.cycles += 2;
            let t4 = psm != c.g[V0][0];
            c.g[V1][0] = sra32(c.g[A2][0], 16);
            if t4 {
                return;
            }
            v58_texel_psmt4(c, tex);
        }
    }
    c.cycles += 2;
    c.g[A2][0] = c.lw(r32(c, V0));
    c.g[V1][0] = sra32(c.g[A2][0], 16);
}

/// 8-bit indexed texel: the swizzled byte address, then the CLUT entry
/// address in v0.
pub(crate) fn v58_texel_psmt8(c: &mut Cpu, tex: u32) {
    c.cycles += 52;
    c.g[V0][0] = c.ld(p(tex, texture::TEX0));
    let y = c.g[T2][0];
    c.g[A0][0] = y & 0x3c;
    c.g[A1][0] = y & 1;
    c.g[V1][0] = sra32(y, 6);
    c.g[V0][0] >>= 26;
    c.g[A0][0] = sra32(c.g[A0][0], 1);
    c.g[V0][0] &= 0xf;
    c.g[A0][0] = add32(c.g[A0][0], c.g[A1][0]);
    c.g[A3][0] = y & 2;
    c.g[V1][0] = shl32(c.g[V1][0], 5);
    c.g[T0][0] = y & 4;
    c.g[V0][0] = sext(c.g[V0][0]);
    c.g[T2][0] = add32(c.g[V1][0], c.g[A0][0]);
    c.g[V0][0] = sx((c.g[T9][0] as u32) << (c.g[V0][0] as u32 & 31));
    c.mult(0, V0, T2, V0);
    c.g[A0][0] = shl32(c.g[A3][0], 1);
    let x = c.g[T4][0];
    c.g[V1][0] = x & 7;
    c.g[A2][0] = x & 0x70;
    c.g[V1][0] ^= c.g[A0][0];
    c.g[A2][0] = sra32(c.g[A2][0], 1);
    c.g[V1][0] ^= c.g[T0][0];
    c.g[A1][0] = sra32(x, 7);
    c.g[A2][0] = add32(c.g[A2][0], c.g[V1][0]);
    c.g[A0][0] = srl32(c.g[V0][0], 31);
    c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
    c.g[V1][0] = x & 8;
    c.g[A1][0] = shl32(c.g[A1][0], 6);
    c.g[V0][0] = sra32(c.g[V0][0], 1);
    c.g[T4][0] = add32(c.g[A1][0], c.g[A2][0]);
    c.g[V1][0] = shl32(c.g[V1][0], 1);
    c.g[V0][0] = add32(c.g[V0][0], c.g[T4][0]);
    c.g[A3][0] = shl32(c.g[A3][0], 2);
    c.g[V0][0] = shl32(c.g[V0][0], 5);
    c.g[V1][0] = add32(c.g[V1][0], c.g[A3][0]);
    c.g[V1][0] = add32(c.g[V0][0], c.g[V1][0]);
    c.g[A2][0] = c.lw(p(tex, texture::CLUT));
    c.g[V0][0] = sra32(c.g[V1][0], 3);
    c.g[V0][0] = add32(c.g[T8][0], c.g[V0][0]);
    // The CLUT index with bits 3 and 4 swapped (CSM1 layout).
    c.g[A1][0] = c.lbu(r32(c, V0));
    let idx = c.g[A1][0];
    c.g[A0][0] = idx & 8;
    c.g[V0][0] = idx & 7;
    c.g[V1][0] = idx & 0x10;
    c.g[A0][0] = shl32(c.g[A0][0], 1);
    c.g[V0][0] |= c.g[A0][0];
    c.g[V1][0] = srl32(c.g[V1][0], 1);
    c.g[V0][0] |= c.g[V1][0];
    c.g[A1][0] = idx & 0xe0;
    c.g[V0][0] |= c.g[A1][0];
    c.g[V0][0] = shl32(c.g[V0][0], 2);
    c.g[V0][0] = add32(c.g[V0][0], c.g[A2][0]);
}

/// 4-bit indexed texel: the swizzled nibble, then the CLUT entry address
/// in v0.
pub(crate) fn v58_texel_psmt4(c: &mut Cpu, tex: u32) {
    c.cycles += 42;
    let x = c.g[T4][0];
    let y = c.g[T2][0];
    c.g[A0][0] = x & 0x60;
    c.g[V0][0] = y & 0xc;
    c.g[V1][0] = c.ld(p(tex, texture::TEX0));
    c.g[V0][0] = sra32(c.g[V0][0], 1);
    c.g[A0][0] = sra32(c.g[A0][0], 2);
    c.g[A1][0] = y & 1;
    c.g[A0][0] = add32(c.g[A0][0], c.g[V0][0]);
    c.g[V1][0] >>= 26;
    c.g[V0][0] = sra32(y, 7);
    c.g[V1][0] &= 0xf;
    c.g[A0][0] = add32(c.g[A0][0], c.g[A1][0]);
    c.g[V0][0] = shl32(c.g[V0][0], 5);
    c.g[A3][0] = y & 0x70;
    c.g[T0][0] = y & 2;
    c.g[T1][0] = y & 4;
    c.g[V1][0] = sext(c.g[V1][0]);
    c.g[T2][0] = add32(c.g[V0][0], c.g[A0][0]);
    c.g[V1][0] = sx((c.g[T9][0] as u32) << (c.g[V1][0] as u32 & 31));
    c.mult(0, V1, T2, V1);
    c.g[T0][0] = shl32(c.g[T0][0], 1);
    c.g[V0][0] = x & 7;
    c.g[A1][0] = sra32(x, 7);
    c.g[V0][0] ^= c.g[T0][0];
    c.g[A3][0] = sra32(c.g[A3][0], 1);
    c.g[V0][0] ^= c.g[T1][0];
    c.g[A2][0] = x & 0x18;
    c.g[A0][0] = srl32(c.g[V1][0], 31);
    c.g[A3][0] = add32(c.g[A3][0], c.g[V0][0]);
    c.g[V1][0] = add32(c.g[V1][0], c.g[A0][0]);
    c.g[A1][0] = shl32(c.g[A1][0], 6);
    c.g[T4][0] = add32(c.g[A1][0], c.g[A3][0]);
    c.g[V1][0] = sra32(c.g[V1][0], 1);
    c.g[V1][0] = add32(c.g[V1][0], c.g[T4][0]);
    c.g[A2][0] = add32(c.g[A2][0], c.g[T0][0]);
    c.g[V1][0] = shl32(c.g[V1][0], 5);
    c.g[V1][0] = add32(c.g[V1][0], c.g[A2][0]);
    c.g[V0][0] = sra32(c.g[V1][0], 3);
    c.g[V0][0] = add32(c.g[T8][0], c.g[V0][0]);
    c.g[V1][0] &= 4;
    let high = c.g[V1][0] != 0;
    c.g[V0][0] = c.lbu(r32(c, V0));
    if high {
        c.cycles += 2;
        c.g[V0][0] = srl32(c.g[V0][0], 4);
    } else {
        c.cycles += 1;
        c.g[V0][0] &= 0xf;
    }
    c.cycles += 3;
    c.g[V1][0] = c.lw(p(tex, texture::CLUT));
    c.g[V0][0] = shl32(c.g[V0][0], 2);
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
}

/// Bilinear blend of the four texels (VU0, through the stack) and the
/// scaled result into `out` (unaligned doubleword copies).
pub(crate) fn v58_blend(c: &mut Cpu, sp: u32, out: u32) {
    c.cycles += 86;
    c.cvt_w_s(0, 5);
    c.mfc1(V1, 0);
    let one = fconst(c, 3, 0x3f80_0000);
    c.cvt_w_s(0, 13);
    c.mfc1(V0, 0);
    let scale = ldf(c, 4, gp(c, -0x2730));
    c.mtc1(V1, 2);
    c.cvt_s_w(2, 2);
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    let fx = fr(c, 5) - fr(c, 2);
    setf(c, 2, fx);
    let fy = fr(c, 13) - fr(c, 1);
    setf(c, 1, fy);
    let gx = one - fx;
    setf(c, 0, gx);
    let gy = one - fy;
    setf(c, 3, gy);
    setf(c, 5, fx * fy);
    setf(c, 1, gx * fy);
    setf(c, 0, gx * gy);
    // w00 * t00
    c.mfc1(V0, 0);
    vscale(c, p(sp, 0x10), V0, p(sp, 0x90));
    setf(c, 2, fx * gy);
    copy_q(c, V0, p(sp, 0x90), p(sp, 0x80));
    // + w01 * t01
    c.mfc1(V1, 1);
    vscale(c, p(sp, 0x20), V1, p(sp, 0xa0));
    copy_q(c, V0, p(sp, 0xa0), p(sp, 0x90));
    vadd(c, p(sp, 0x80), p(sp, 0x90), p(sp, 0xa0));
    copy_q(c, V0, p(sp, 0xa0), p(sp, 0x70));
    // + w10 * t10
    c.mfc1(V1, 2);
    vscale(c, p(sp, 0x30), V1, p(sp, 0xb0));
    copy_q(c, V0, p(sp, 0xb0), p(sp, 0xa0));
    vadd(c, p(sp, 0x70), p(sp, 0xa0), p(sp, 0xb0));
    copy_q(c, V0, p(sp, 0xb0), p(sp, 0x60));
    // + w11 * t11
    c.mfc1(V1, 5);
    vscale(c, p(sp, 0x40), V1, p(sp, 0xc0));
    copy_q(c, V0, p(sp, 0xc0), p(sp, 0xb0));
    vadd(c, p(sp, 0x60), p(sp, 0xb0), p(sp, 0xc0));
    c.lq(V0, p(sp, 0xc0));
    c.sq(V0, p(sp, 0x10));
    c.sq(V0, p(sp, 0x50));
    // Scaled, with the lanes reversed (w, z, y, x) into sp+0x50.
    let w = ldf(c, 1, p(sp, 0x1c));
    let z = ldf(c, 2, p(sp, 0x18));
    let y = ldf(c, 3, p(sp, 0x14));
    setf(c, 1, w * scale);
    let x = ldf(c, 0, p(sp, 0x10));
    setf(c, 2, z * scale);
    setf(c, 3, y * scale);
    setf(c, 0, x * scale);
    c.swc1(1, p(sp, 0x50));
    c.swc1(2, p(sp, 0x54));
    c.swc1(3, p(sp, 0x58));
    c.swc1(0, p(sp, 0x5c));
    c.ldl(V0, p(sp, 0x57));
    c.ldr(V0, p(sp, 0x50));
    c.ldl(V1, p(sp, 0x5f));
    c.ldr(V1, p(sp, 0x58));
    c.sdl(V0, p(out, 7));
    c.sdr(V0, out);
    c.sdl(V1, p(out, 0xf));
    c.sdr(V1, p(out, 8));
}

// ---------------------------------------------------------------------------
// Irradiance.

pub(crate) const IR_ENTRY: u32 = 0x0038_a6a8;

pub(crate) const IR_END: u32 = 0x0038_abf8;

/// Heads of the two integer-power loops (resume entries).
pub(crate) const IR_POW_OUTER: u32 = 0x0038_a900;

pub(crate) const IR_POW_INNER: u32 = 0x0038_aa20;

/// `switch (exponent)` jump tables (.rodata) of the two falloff branches.
pub(crate) const IR_TABLE_OUTER: u32 = 0x0049_2e30;

pub(crate) const IR_TABLE_INNER: u32 = 0x0049_2e50;

/// `void Irradiance_AddDynamicLight(Irradiance* irr, const Vec3* pos,
/// const DynLight* light)` (0x38a6a8): spot (kind 1) and point (kind 2)
/// lights as a directional contribution (`Irradiance_AddDirectional`)
/// scaled by the angular and distance falloffs (integer powers through a
/// `switch` on the exponent, or a square-and-multiply loop), clamped to
/// [0, 5]. Dispatcher-shaped in the original: re-entered at the power
/// loops' heads after an unwind.
pub fn irradiance_add_dynamic_light(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x60 };
        match c.pc {
            IR_ENTRY => ir_entry(c, frame),
            IR_POW_OUTER => {
                if !pow_loop(c, 1, IR_POW_OUTER) {
                    return Ok(());
                }
                ir_outer_done(c, frame)
            }
            IR_POW_INNER => {
                if !pow_loop(c, 3, IR_POW_INNER) {
                    return Ok(());
                }
                ir_inner_done(c, frame)
            }
            pc => {
                c.jump_out(pc, IR_ENTRY, IR_END);
                Ok(())
            }
        }
    })
}

pub(crate) fn ir_entry(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 10;
    Frame::open(c, 0x60);
    c.g[V0][0] = 1;
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    c.g[S0][0] = c.g[A2][0];
    frame.save_ra(c, 0x30);
    c.g[S1][0] = c.g[A0][0];
    c.g[V1][0] = c.lw(at(c, S0, dyn_light::KIND));
    let spot = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = c.g[A1][0];
    if spot {
        return ir_spot(c, frame);
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
    if c.g[V0][0] != 0 {
        frame.restore(c, S0, 0x50);
        ir_exit(c, frame);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = 2;
    let point = c.g[V1][0] == c.g[V0][0];
    c.g[A1][0] = c.g[SP][0];
    if point {
        return ir_point(c, frame);
    }
    c.cycles += 2;
    frame.restore(c, S0, 0x50);
    ir_exit(c, frame);
    Ok(())
}

/// The epilogue (s0 restored before).
pub(crate) fn ir_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 4;
    frame.restore(c, S1, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
}

/// No contribution: `lq s0` and out.
pub(crate) fn ir_abort(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    frame.restore(c, S0, 0x50);
    ir_exit(c, frame);
}

/// The cone/range term into f2 (1 when the range is below a threshold or
/// the angle term within it, else `1 - (angle - threshold) * k`), then
/// f3 = the second vector term * 100, f1 = 1, v1 = the shape byte; true
/// when f3 < 1 (c.lt.s).
pub(crate) fn ir_cone(c: &mut Cpu, k_range: i32, k_angle: i32, k_slope: i32) -> bool {
    let light = r32(c, S0);
    let sp = sp(c);
    c.cycles += 6;
    let range = ldf(c, 1, p(light, dyn_light::RANGE));
    let min_range = ldf(c, 0, gp(c, k_range));
    let short = lt(c, range, min_range);
    let angle = ldf(c, 2, p(sp, 0x20));
    let mut excess = None;
    if !short {
        c.cycles += 5;
        let start = ldf(c, 0, gp(c, k_angle));
        if !le(c, angle, start) {
            let e = angle - start;
            setf(c, 0, e);
            excess = Some(e);
        }
    }
    match excess {
        None => {
            c.cycles += 4;
            fconst(c, 2, 0x3f80_0000);
        }
        Some(e) => {
            c.cycles += 6;
            let slope = ldf(c, 1, gp(c, k_slope));
            let one = fconst(c, 2, 0x3f80_0000);
            let d = e * slope;
            setf(c, 0, d);
            setf(c, 2, one - d);
        }
    }
    let t = ldf(c, 3, p(sp, 0x24));
    c.cycles += 9;
    let hundred = fconst(c, 0, 0x42c8_0000);
    let one = fconst(c, 1, 0x3f80_0000);
    let t = t * hundred;
    setf(c, 3, t);
    let small = lt(c, t, one);
    c.g[V1][0] = c.lb(p(light, dyn_light::SHAPE));
    small
}

/// `v1 = exponent byte; v0 = v1 < 5 (unsigned); v0 = 0x490000`: the
/// switch head. Some(target) for a table case.
pub(crate) fn ir_switch(c: &mut Cpu, table: u32) -> Option<u32> {
    c.cycles += 4;
    c.g[V1][0] = c.lb(at(c, S0, dyn_light::EXPONENT));
    c.g[V0][0] = (c.g[V1][0] < 5) as u64;
    let in_table = c.g[V0][0] != 0;
    c.g[V0][0] = 0x0049_0000;
    if !in_table {
        return None;
    }
    c.cycles += 6;
    c.g[V1][0] = shl32(c.g[V1][0], 2);
    c.g[V0][0] = sx(table);
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.g[A0][0] = c.lw(r32(c, V1));
    Some(r32(c, A0))
}

/// f2 = colour.z (+0x28), a2 = sp+0x10.
#[inline(always)]
pub(crate) fn ir_colour_z(c: &mut Cpu) {
    ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
    c.g[A2][0] = sx(p(sp(c), 0x10));
}

/// f6, f4 = colour.x, colour.y.
#[inline(always)]
pub(crate) fn ir_colour_xy(c: &mut Cpu) {
    ldf(c, 6, at(c, S0, dyn_light::COLOUR));
    ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
}

/// Square-and-multiply `f[acc] *= f0^v1` from a loop head (v1 > 0). False
/// when an event at the head asked to unwind.
pub(crate) fn pow_loop(c: &mut Cpu, acc: usize, head: u32) -> bool {
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0] & 1;
        if c.g[V0][0] != 0 {
            setf(c, acc, fr(c, acc) * fr(c, 0));
        }
        c.cycles += 3;
        c.g[V1][0] = sra32(c.g[V1][0], 1);
        let more = c.g[V1][0] != 0;
        setf(c, 0, fr(c, 0) * fr(c, 0));
        if !more {
            return true;
        }
        if back_edge_unwinds(c, head) {
            return false;
        }
    }
}

/// Spot light: direction/attenuation vector, the cone term, the shape
/// power, then the distance falloff on one side of the inner radius.
pub(crate) fn ir_spot(c: &mut Cpu, frame: Frame) -> Flow {
    let sp = sp(c);
    c.cycles += 8;
    c.g[A1][0] = c.g[SP][0];
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = sx(p(sp, 0x20));
    c.g[T0][0] = sx(p(sp, 0x24));
    c.g[T1][0] = sx(p(sp, 0x28));
    callee::SPOT_LIGHT_VECTOR.call(c, 0x0038_a708)?;
    let lit = c.g[V0][0] != 0;
    let dist = ldf(c, 1, p(sp, 0x28));
    if !lit {
        ir_abort(c, frame);
        return Ok(());
    }
    c.cycles += 6;
    let light = r32(c, S0);
    let outer = ldf(c, 0, p(light, dyn_light::OUTER));
    setf(c, 7, dist);
    let inside = le(c, outer, dist);
    setf(c, 8, outer);
    if !inside {
        ir_abort(c, frame);
        return Ok(());
    }
    // Shape power of the cone term into f1 (1 by default).
    let small = ir_cone(c, -0x26e4, -0x26e0, -0x26dc);
    let mut loaded = false;
    if small {
        c.cycles += 3;
        c.g[V0][0] = 2;
        let square = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
        if square {
            c.cycles += 2;
            setf(c, 1, fr(c, 3) * fr(c, 3));
        } else {
            c.cycles += 2;
            let high = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if high {
                c.cycles += 3;
                c.g[V0][0] = 3;
                if c.g[V1][0] == c.g[V0][0] {
                    let sq = fr(c, 3) * fr(c, 3);
                    setf(c, 0, sq);
                    c.cycles += 2;
                    setf(c, 1, sq * fr(c, 3));
                } else {
                    c.cycles += 2;
                    c.cycles += 2;
                    fconst(c, 1, 0x3f80_0000);
                }
            } else {
                c.cycles += 2;
                if c.g[V1][0] == c.g[V0][0] {
                    c.mov_s(1, 3);
                } else {
                    c.cycles += 2;
                    ldf(c, 0, p(light, dyn_light::INTENSITY));
                    loaded = true;
                }
            }
        }
    }
    if !loaded {
        c.cycles += 1;
        ldf(c, 0, p(light, dyn_light::INTENSITY));
    }
    // f5 = shape * intensity * cone; which side of the inner radius?
    c.cycles += 6;
    let inner = ldf(c, 3, p(light, dyn_light::INNER));
    let scaled = fr(c, 1) * fr(c, 0);
    setf(c, 0, scaled);
    let beyond = le(c, inner, fr(c, 7));
    setf(c, 1, inner);
    setf(c, 5, scaled * fr(c, 2));
    if beyond {
        ir_spot_outer(c, frame)
    } else {
        ir_spot_inner(c, frame)
    }
}

/// Beyond the inner radius: f1 = dist^exponent, f5 *= f1.
pub(crate) fn ir_spot_outer(c: &mut Cpu, frame: Frame) -> Flow {
    match ir_switch(c, IR_TABLE_OUTER) {
        Some(0x0038_a834) => {
            c.cycles += 5;
            ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
            fconst(c, 1, 0x3f80_0000);
            c.g[A2][0] = sx(p(sp(c), 0x10));
            c.cycles += 3;
            ir_colour_xy(c);
        }
        Some(t @ (0x0038_a848 | 0x0038_a850)) => {
            if t == 0x0038_a848 {
                c.cycles += 2;
                c.mov_s(1, 7);
            } else {
                c.cycles += 1;
                setf(c, 1, fr(c, 7) * fr(c, 7));
            }
            c.cycles += 5;
            ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
            ldf(c, 6, at(c, S0, dyn_light::COLOUR));
            c.g[A2][0] = sx(p(sp(c), 0x10));
            ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
        }
        Some(t @ (0x0038_a868 | 0x0038_a884)) => {
            c.cycles += if t == 0x0038_a868 { 7 } else { 8 };
            let d = fr(c, 7);
            let mut v = d * d;
            setf(c, 0, v);
            ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
            ldf(c, 6, at(c, S0, dyn_light::COLOUR));
            c.g[A2][0] = sx(p(sp(c), 0x10));
            ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
            if t == 0x0038_a884 {
                v = v * d;
                setf(c, 0, v);
            }
            setf(c, 1, v * d);
        }
        Some(t) => {
            c.jump_out(t, IR_ENTRY, IR_END);
            return Ok(());
        }
        None => {
            // General exponent: 0 when dist == 0, else dist^n (1/dist for
            // negative n) by square-and-multiply.
            c.cycles += 5;
            setf(c, 1, FpuF32::ZERO);
            let zero = eq(c, fr(c, 7), FpuF32::ZERO);
            c.mov_s(0, 7);
            if zero {
                c.cycles += 2;
                ir_colour_z(c);
                c.cycles += 3;
                ir_colour_xy(c);
            } else {
                c.cycles += 2;
                let negative = (c.g[V1][0] as i64) < 0;
                ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
                if negative {
                    c.cycles += 4;
                    let one = fconst(c, 0, 0x3f80_0000);
                    c.g[V1][0] = sx(0u32.wrapping_sub(r32(c, V1)));
                    setf(c, 0, one.div(fr(c, 7)));
                }
                c.cycles += 6;
                c.g[A2][0] = sx(p(sp(c), 0x10));
                fconst(c, 1, 0x3f80_0000);
                ldf(c, 6, at(c, S0, dyn_light::COLOUR));
                let done = c.g[V1][0] == 0;
                ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
                if !done {
                    c.cycles += 1;
                    if !pow_loop(c, 1, IR_POW_OUTER) {
                        return Ok(());
                    }
                }
            }
        }
    }
    ir_outer_done(c, frame)
}

/// `f5 *= f1`, then the clamp and the contribution.
pub(crate) fn ir_outer_done(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    setf(c, 5, fr(c, 5) * fr(c, 1));
    ir_contribute(c, frame)
}

/// Within the inner radius: f3 = inner^exponent, then f5 *= f3 * (dist -
/// outer) / (inner - outer).
pub(crate) fn ir_spot_inner(c: &mut Cpu, frame: Frame) -> Flow {
    match ir_switch(c, IR_TABLE_INNER) {
        Some(t @ (0x0038_a948 | 0x0038_a95c)) => {
            if t == 0x0038_a948 {
                c.cycles += 5;
                ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
                fconst(c, 3, 0x3f80_0000);
                c.g[A2][0] = sx(p(sp(c), 0x10));
            } else {
                c.cycles += 2;
                ir_colour_z(c);
            }
            c.cycles += 3;
            ir_colour_xy(c);
        }
        Some(t @ (0x0038_a970 | 0x0038_a988 | 0x0038_a9a4)) => {
            let r = fr(c, 3);
            c.cycles += match t {
                0x0038_a970 => 6,
                0x0038_a988 => 7,
                _ => 8,
            };
            if t == 0x0038_a970 {
                setf(c, 3, r * r);
            } else {
                setf(c, 0, r * r);
            }
            ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
            ldf(c, 6, at(c, S0, dyn_light::COLOUR));
            c.g[A2][0] = sx(p(sp(c), 0x10));
            ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
            if t == 0x0038_a9a4 {
                setf(c, 0, fr(c, 0) * r);
            }
            if t != 0x0038_a970 {
                setf(c, 3, fr(c, 0) * r);
            }
        }
        Some(t) => {
            c.jump_out(t, IR_ENTRY, IR_END);
            return Ok(());
        }
        None => {
            c.cycles += 5;
            setf(c, 2, FpuF32::ZERO);
            let zero = eq(c, fr(c, 1), FpuF32::ZERO);
            c.mov_s(0, 1);
            if zero {
                c.cycles += 6;
                c.mov_s(0, 2);
                ldf(c, 6, at(c, S0, dyn_light::COLOUR));
                ir_colour_z(c);
                ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
                c.cycles += 1;
                c.mov_s(3, 0);
                return ir_falloff(c, frame);
            }
            c.cycles += 2;
            let negative = (c.g[V1][0] as i64) < 0;
            ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
            if negative {
                c.cycles += 4;
                let one = fconst(c, 0, 0x3f80_0000);
                c.g[V1][0] = sx(0u32.wrapping_sub(r32(c, V1)));
                setf(c, 0, one.div(fr(c, 1)));
            }
            c.cycles += 6;
            c.g[A2][0] = sx(p(sp(c), 0x10));
            fconst(c, 3, 0x3f80_0000);
            ldf(c, 6, at(c, S0, dyn_light::COLOUR));
            let done = c.g[V1][0] == 0;
            ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
            if !done && !pow_loop(c, 3, IR_POW_INNER) {
                return Ok(());
            }
            return ir_inner_done(c, frame);
        }
    }
    ir_falloff(c, frame)
}

/// After the inner power loop: `f0 = f3; f3 = f0`, then the falloff.
pub(crate) fn ir_inner_done(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.mov_s(0, 3);
    c.cycles += 1;
    c.mov_s(3, 0);
    ir_falloff(c, frame)
}

/// `f5 *= f3 * (dist - outer) / (inner - outer)`.
pub(crate) fn ir_falloff(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    let span = fr(c, 1) - fr(c, 8);
    setf(c, 1, span);
    let d = fr(c, 7) - fr(c, 8);
    setf(c, 0, d);
    let f5 = fr(c, 5) * fr(c, 3);
    let ratio = d.div(span);
    setf(c, 0, ratio);
    setf(c, 5, f5 * ratio);
    ir_contribute(c, frame)
}

/// Clamp f5 to [0, 5] and add the colour scaled by it as a directional
/// light (direction at sp+0).
pub(crate) fn ir_contribute(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    setf(c, 0, FpuF32::ZERO);
    let five = fconst(c, 1, 0x40a0_0000);
    let k = fr(c, 5);
    if le(c, FpuF32::ZERO, k) {
        setf(c, 0, k.min(five));
    }
    c.cycles += 12;
    let k = fr(c, 0);
    setf(c, 5, k);
    c.g[A0][0] = c.g[S1][0];
    let sp = sp(c);
    setf(c, 2, fr(c, 2) * k);
    c.g[A1][0] = c.g[SP][0];
    setf(c, 1, fr(c, 6) * k);
    setf(c, 0, fr(c, 4) * k);
    c.swc1(2, p(sp, 0x18));
    c.swc1(1, p(sp, 0x10));
    c.swc1(0, p(sp, 0x14));
    callee::ADD_DIRECTIONAL.call(c, 0x0038_aa98)?;
    frame.restore(c, S0, 0x50);
    ir_exit(c, frame);
    Ok(())
}

/// Point light: vector, the cone/range term, the shape power, clamp to
/// [0, 5], contribution.
pub(crate) fn ir_point(c: &mut Cpu, frame: Frame) -> Flow {
    let sp = sp(c);
    c.cycles += 6;
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = sx(p(sp, 0x20));
    c.g[T0][0] = sx(p(sp, 0x24));
    callee::POINT_LIGHT_VECTOR.call(c, 0x0038_aab0)?;
    if c.g[V0][0] == 0 {
        frame.restore(c, S0, 0x50);
        ir_exit(c, frame);
        return Ok(());
    }
    let light = r32(c, S0);
    let small = ir_cone(c, -0x26d8, -0x26d4, -0x26d0);
    // Shape power into f3 (1 by default); f0 = intensity.
    let mut unit = !small;
    let mut loaded = false;
    if small {
        c.cycles += 3;
        c.g[V0][0] = 2;
        let square = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
        if square {
            c.cycles += 2;
            setf(c, 3, fr(c, 3) * fr(c, 3));
        } else {
            c.cycles += 2;
            let high = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if high {
                c.cycles += 3;
                c.g[V0][0] = 3;
                if c.g[V1][0] == c.g[V0][0] {
                    let sq = fr(c, 3) * fr(c, 3);
                    setf(c, 0, sq);
                    c.cycles += 2;
                    setf(c, 3, sq * fr(c, 3));
                } else {
                    c.cycles += 2;
                    unit = true;
                }
            } else {
                c.cycles += 2;
                if c.g[V1][0] == c.g[V0][0] {
                    ldf(c, 0, p(light, dyn_light::INTENSITY));
                    loaded = true;
                } else {
                    c.cycles += 2;
                    unit = true;
                }
            }
        }
    }
    if unit {
        c.cycles += 2;
        fconst(c, 3, 0x3f80_0000);
    }
    if !loaded {
        c.cycles += 1;
        ldf(c, 0, p(light, dyn_light::INTENSITY));
    }
    c.cycles += 9;
    setf(c, 1, FpuF32::ZERO);
    let scaled = fr(c, 3) * fr(c, 0);
    setf(c, 0, scaled);
    let five = fconst(c, 3, 0x40a0_0000);
    let k = scaled * fr(c, 2);
    setf(c, 5, k);
    let positive = le(c, FpuF32::ZERO, k);
    setf(c, 4, FpuF32::ZERO);
    if positive {
        c.cycles += 1;
        setf(c, 4, k.min(five));
    }
    c.cycles += 14;
    let k = fr(c, 4);
    let cz = ldf(c, 2, p(light, dyn_light::COLOUR + 8));
    setf(c, 5, k);
    let cx = ldf(c, 1, p(light, dyn_light::COLOUR));
    c.g[A0][0] = c.g[S1][0];
    let cy = ldf(c, 0, p(light, dyn_light::COLOUR + 4));
    setf(c, 2, cz * k);
    setf(c, 1, cx * k);
    c.g[A1][0] = c.g[SP][0];
    setf(c, 0, cy * k);
    c.g[A2][0] = sx(p(sp, 0x10));
    c.swc1(2, p(sp, 0x18));
    c.swc1(1, p(sp, 0x10));
    c.swc1(0, p(sp, 0x14));
    callee::ADD_DIRECTIONAL.call(c, 0x0038_abe4)?;
    ir_abort(c, frame);
    Ok(())
}

// ---------------------------------------------------------------------------
// Small helpers.

/// `x + off` on a 32-bit guest address.
#[inline(always)]
pub(crate) fn p_2(x: u32, off: u32) -> u32 {
    x.wrapping_add(off)
}

#[inline(always)]
pub(crate) fn fmul(c: &mut Cpu, fd: usize, fs: usize, ft: usize) {
    let v = fr(c, fs) * fr(c, ft);
    setf(c, fd, v);
}

#[inline(always)]
pub(crate) fn fadd(c: &mut Cpu, fd: usize, fs: usize, ft: usize) {
    let v = fr(c, fs) + fr(c, ft);
    setf(c, fd, v);
}

#[inline(always)]
pub(crate) fn fsub(c: &mut Cpu, fd: usize, fs: usize, ft: usize) {
    let v = fr(c, fs) - fr(c, ft);
    setf(c, fd, v);
}

#[inline(always)]
pub(crate) fn f_lt(c: &mut Cpu, fs: usize, ft: usize) -> bool {
    let r = fr(c, fs).lt(fr(c, ft));
    abi::set_fcc(c, r);
    r
}

#[inline(always)]
pub(crate) fn f_le(c: &mut Cpu, fs: usize, ft: usize) -> bool {
    let r = fr(c, fs).le(fr(c, ft));
    abi::set_fcc(c, r);
    r
}

#[inline(always)]
pub(crate) fn f_eq(c: &mut Cpu, fs: usize, ft: usize) -> bool {
    let r = fr(c, fs).eq_s(fr(c, ft));
    abi::set_fcc(c, r);
    r
}

/// A 4x4 matrix copied through `vf1`..`vf4`.
#[inline(always)]
pub(crate) fn copy_mat(c: &mut Cpu, from: u32, to: u32) {
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, p_2(from, 0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, p_2(to, 0x10 * i));
    }
}

/// `dst = m1 * m2` the way gcc emitted it: VU0 multiply into `tmp`, then
/// `tmp -> sp+0 -> dst` (the intermediate quadwords are visible in RAM).
#[inline(always)]
pub(crate) fn mul_mat(c: &mut Cpu, dst: u32, m1: u32, m2: u32, tmp: u32) {
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, p_2(m1, 0x10 * i));
    }
    for i in 0..4u32 {
        c.lqc2(8 + i as usize, p_2(m2, 0x10 * i));
    }
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
    for i in 0..4u32 {
        c.sqc2(12 + i as usize, p_2(tmp, 0x10 * i));
    }
    let scratch = sp(c);
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, p_2(tmp, 0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, p_2(scratch, 0x10 * i));
    }
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, p_2(scratch, 0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, p_2(dst, 0x10 * i));
    }
}

// ---------------------------------------------------------------------------
// View matrices and depth.

/// `void PS2GraphicsMan_UpdateViewMatrices(PS2GraphicsMan* g)` (0x37d968):
/// mark the matrices current, rebuild the view-projection matrix and the two
/// dependent products, then hand the new view matrix to the defer manager.
pub fn ps2_graphics_man_update_view_matrices(c: &mut Cpu) {
    c.cycles += 159;
    let frame = Frame::open(c, 0x90);
    let gfx = abi::arg(c, 0);
    let sp0 = sp(c);
    c.g[V0][0] = 1;
    frame.save_ra(c, 0x80);
    c.g[A2][0] = sx(p_2(sp0, 0x40));
    c.sw(p_2(gfx, man::TEX_READY), c.g[V0][0]);
    c.g[V1][0] = sx(p_2(gfx, man::WORLD));
    c.g[V0][0] = Addr(gfx).off(man::CAMERA).lw(c);
    let tmp = c.g[A2][0] as u32;
    let world = c.g[V1][0] as u32;
    let cam = c.g[V0][0] as u32;
    // view-projection = world * camera
    mul_mat(c, p_2(gfx, man::VIEW_PROJ), world, cam, tmp);
    c.g[A1][0] = sx(p_2(gfx, man::CAM_MAT));
    c.g[V0][0] = Addr(gfx).off(man::CAMERA).lw(c);
    let cam = c.g[V0][0] as u32;
    let cam_mat = c.g[A1][0] as u32;
    copy_mat(c, cam, cam_mat);
    c.g[V1][0] = sx(p_2(gfx, man::LOCAL_MAT));
    let local = c.g[V1][0] as u32;
    let cam_mat = c.g[A1][0] as u32;
    mul_mat(c, p_2(gfx, man::CAM_MAT2), local, cam_mat, tmp);
    c.g[V1][0] = sx(p_2(gfx, man::MAT5860));
    let m5860 = c.g[V1][0] as u32;
    let cam_mat = c.g[A1][0] as u32;
    mul_mat(c, p_2(gfx, man::VIEW_MAT), m5860, cam_mat, tmp);
    c.g[V0][0] = sx(p_2(gfx, man::VIEW_MAT));
    c.g[RA][0] = sx(0x0037_dbd8);
    c.g[A0][0] = Addr(gfx).off(man::DEFER).lw(c);
    if callee::INTERN_MATRIX_RECORD.call(c, 0x0037_dbd8).is_err() {
        return;
    }
    frame.restore_ra(c, 0x80);
    frame.close(c);
}

/// `void PS2GraphicsMan_v27(PS2GraphicsMan* g, const Vec4* pos, float dist)`
/// (0x377278): project `pos` (and `pos + dist * camera-direction`) through the
/// view matrix, derive the view depth, and when it differs from the saved
/// depth, store `k + depth` (k = 32767.5, or 8388467.5 in mode 0x31) and mark
/// the matrices stale.
pub fn ps2_graphics_man_v27(c: &mut Cpu) {
    c.cycles += 16;
    let frame = Frame::open(c, 0xb0);
    let gfx = abi::arg(c, 0);
    let pos = abi::arg(c, 1);
    let sp0 = sp(c);
    c.g[V1][0] = 0x230;
    frame.save(c, S0, 0x90);
    frame.save(c, S1, 0x80);
    c.g[S0][0] = abi::arg64(c, 0);
    frame.save(c, S2, 0x70);
    frame.save_fpr(c, 20, 0xa0);
    c.g[S2][0] = abi::arg64(c, 1);
    frame.save_ra(c, 0x60);
    c.mov_s(20, 12);
    c.g[V0][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    c.g[A0][0] = Addr(gfx).off(man::TEX_READY).lw(c);
    c.mult(0, 2, 2, 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(man::VIEWS));
    let ready = c.g[A0][0];
    c.g[S1][0] = sx((gfx).wrapping_add(c.g[V0][0] as u32));
    if ready == 0 {
        c.cycles += 2;
        c.g[RA][0] = sx(0x0037_72c0);
        c.g[A0][0] = c.g[S0][0];
        if callee::UPDATE_VIEW_MATRICES.call(c, 0x0037_72c0).is_err() {
            return;
        }
    }
    let view = c.g[S1][0] as u32;

    // The depth offset: `f1` becomes 0 when the point is at the camera or
    // the projection ratio leaves [-1, 1].
    c.cycles += 5;
    c.mtc1(0, 3);
    let at_camera = f_eq(c, 20, 3);
    c.g[V1][0] = sx(p_2(gfx, man::VIEW_MAT));
    let mut computed = false;
    if !at_camera {
        c.cycles += 56;
        c.g[V0][0] = Addr(gfx).off(man::CAMERA).lw(c);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 4);
        c.lwc1(2, p_2(c.g[V0][0] as u32, 0x28));
        c.lwc1(0, p_2(c.g[V0][0] as u32, 0x8));
        c.lwc1(1, p_2(c.g[V0][0] as u32, 0x18));
        c.swc1(0, p_2(sp0, 0x0));
        c.swc1(1, p_2(sp0, 0x4));
        c.swc1(2, p_2(sp0, 0x8));
        c.swc1(3, p_2(sp0, 0xc));
        // view * pos
        c.lqc2(8, pos);
        c.lqc2(4, p_2(gfx, man::VIEW_MAT));
        c.lqc2(5, p_2(gfx, man::VIEW_MAT + 0x10));
        c.lqc2(6, p_2(gfx, man::VIEW_MAT + 0x20));
        c.lqc2(7, p_2(gfx, man::VIEW_MAT + 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, p_2(sp0, 0x20));
        c.lq(2, p_2(sp0, 0x20));
        c.sq(2, p_2(sp0, 0x10));
        // pos + dist * (camera direction)
        c.mfc1(4, 20);
        c.lqc2(4, p_2(sp0, 0x0));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, p_2(sp0, 0x50));
        c.lq(2, p_2(sp0, 0x50));
        c.sq(2, p_2(sp0, 0x40));
        c.lqc2(3, pos);
        c.lqc2(4, p_2(sp0, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p_2(sp0, 0x50));
        c.lq(2, p_2(sp0, 0x50));
        c.sq(2, p_2(sp0, 0x30));
        // view * (pos + dist * dir)
        c.lqc2(8, p_2(sp0, 0x30));
        c.lqc2(4, p_2(gfx, man::VIEW_MAT));
        c.lqc2(5, p_2(gfx, man::VIEW_MAT + 0x10));
        c.lqc2(6, p_2(gfx, man::VIEW_MAT + 0x20));
        c.lqc2(7, p_2(gfx, man::VIEW_MAT + 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, p_2(sp0, 0x50));
        c.lwc1(1, p_2(sp0, 0x18));
        c.lwc1(0, p_2(sp0, 0x1c));
        c.lq(2, p_2(sp0, 0x50));
        c.div_s(2, 1, 0);
        c.sq(2, p_2(sp0, 0x20));
        c.lwc1(1, p_2(sp0, 0x28));
        c.lwc1(0, p_2(sp0, 0x2c));
        c.c_le_s(4, 2);
        c.div_s(1, 1, 0);
        if c.fcc() {
            c.cycles += 5;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            if f_le(c, 2, 0) {
                c.cycles += 3;
                if f_le(c, 4, 1) {
                    c.cycles += 3;
                    if f_le(c, 1, 0) {
                        c.cycles += 4;
                        fsub(c, 1, 1, 2);
                        c.lwc1(0, p_2(gfx, man::F58E8));
                        fmul(c, 1, 0, 1);
                        computed = true;
                    }
                }
            }
        }
    }
    if !computed {
        c.cycles += 1;
        c.mtc1(0, 1);
    }

    // Compare with the depth saved in the view; rebuild when it changed.
    c.cycles += 5;
    c.lwc1(0, p_2(view, vw::DEPTH));
    if f_eq(c, 1, 0) {
        c.lq(16, p_2(sp0, 0x90));
    } else {
        c.cycles += 6;
        c.swc1(1, p_2(view, vw::DEPTH));
        c.g[V1][0] = 0x31;
        let gp = c.g[GP][0] as u32;
        c.lwc1(2, gp.wrapping_sub(0x279c));
        c.g[V0][0] = Addr(gfx).off(man::MODE).lw(c);
        if c.g[V0][0] != c.g[V1][0] {
            fadd(c, 1, 2, 1);
        } else {
            c.cycles += 2;
            let gp = c.g[GP][0] as u32;
            c.lwc1(2, gp.wrapping_sub(0x2798));
            fadd(c, 1, 2, 1);
        }
        c.cycles += 6;
        c.swc1(1, p_2(gfx, man::F58F8));
        c.lwc1(0, p_2(view, vw::DEPTH));
        c.sw(p_2(gfx, man::TEX_READY), 0);
        fadd(c, 0, 2, 0);
        c.swc1(0, p_2(gfx, man::F5858));
        c.lq(16, p_2(sp0, 0x90));
    }
    c.cycles += 6;
    c.lq(17, p_2(sp0, 0x80));
    c.lq(18, p_2(sp0, 0x70));
    c.g[RA][0] = c.ld(p_2(sp0, 0x60));
    c.lwc1(20, p_2(sp0, 0xa0));
    c.g[SP][0] = sx(p_2(sp0, 0xb0));
    c.pc = c.g[RA][0] as u32;
}

// ---------------------------------------------------------------------------
// The view stack.

/// `void PS2GraphicsMan_PushView(PS2GraphicsMan* g)` (0x3765b8): save the
/// current view state (scissor, key, depths, matrices, quads) into
/// `views[curView]`, bump the index, and copy that state into the new view.
pub fn ps2_graphics_man_push_view(c: &mut Cpu) {
    c.cycles += 108;
    let frame = Frame::open(c, 0x10);
    let gfx = abi::arg(c, 0);
    c.g[T1][0] = 0x230;
    frame.save_ra(c, 0);
    c.g[A1][0] = 0x60000;
    c.g[A1][0] |= 0x8000;
    c.g[T0][0] = sx(p_2(gfx, man::WORLD));
    c.g[A2][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    c.mult(0, 6, 6, 9);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(man::VIEWS));
    c.g[A2][0] = sx((gfx).wrapping_add(c.g[A2][0] as u32));
    let view = c.g[A2][0] as u32;
    c.ldl(2, p_2(gfx, man::SCISSOR + 7));
    c.ldr(2, p_2(gfx, man::SCISSOR));
    c.sdl(2, p_2(view, vw::SCISSOR + 7));
    c.sdr(2, p_2(view, vw::SCISSOR));
    c.g[A3][0] = sx(p_2(view, vw::M60));
    c.g[V0][0] = Addr(gfx).off(man::DEFER).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1cc4).lw(c);
    c.sw(p_2(view, vw::KEY), c.g[V1][0]);
    c.lwc1(0, p_2(gfx, man::F590C));
    c.swc1(0, p_2(view, vw::F24));
    c.lwc1(1, p_2(gfx, man::F5908));
    c.swc1(1, p_2(view, vw::F28));
    copy_mat(c, c.g[T0][0] as u32, c.g[A3][0] as u32);
    c.lq(3, p_2(gfx, man::Q6B70));
    c.g[A3][0] = sx(p_2(view, vw::MA0));
    c.g[A1][0] = sx(p_2(gfx, man::CAM_MAT));
    c.sq(3, p_2(view, vw::Q30));
    c.lq(2, p_2(gfx, man::Q6B80));
    c.sq(2, p_2(view, vw::Q40));
    copy_mat(c, c.g[A1][0] as u32, c.g[A3][0] as u32);
    c.g[V1][0] = sx(p_2(view, vw::ME0));
    c.g[V0][0] = sx(p_2(gfx, man::LOCAL_MAT));
    copy_mat(c, c.g[V0][0] as u32, c.g[V1][0] as u32);
    c.g[A1][0] = sx(p_2(view, vw::M120));
    c.g[V0][0] = sx(p_2(gfx, man::CAM_MAT2));
    copy_mat(c, c.g[V0][0] as u32, c.g[A1][0] as u32);
    c.lq(3, p_2(gfx, man::Q5840));
    c.g[T0][0] = sx(p_2(view, vw::M180));
    c.g[A3][0] = sx(p_2(gfx, man::MAT5860));
    c.sq(3, p_2(view, vw::Q160));
    c.lq(2, p_2(gfx, man::Q5850));
    c.sq(2, p_2(view, vw::Q170));
    copy_mat(c, c.g[A3][0] as u32, c.g[T0][0] as u32);
    c.g[A1][0] = sx(p_2(view, vw::M1C0));
    c.g[V0][0] = sx(p_2(gfx, man::VIEW_MAT));
    copy_mat(c, c.g[V0][0] as u32, c.g[A1][0] as u32);
    c.lq(3, p_2(gfx, man::Q58E0));
    c.g[A1][0] = c.g[A2][0];
    c.sq(3, p_2(view, vw::Q200));
    c.lq(2, p_2(gfx, man::Q58F0));
    c.sq(2, p_2(view, vw::Q210));
    c.lwc1(0, p_2(gfx, man::F5910));
    c.swc1(0, p_2(view, vw::F2C));
    c.lq(2, p_2(gfx, man::Q5920));
    c.sq(2, p_2(view, vw::Q50));
    c.g[V1][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.mult(0, 9, 3, 9);
    c.sw(p_2(gfx, man::CUR_VIEW), c.g[V1][0]);
    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(man::VIEWS));
    c.g[RA][0] = sx(0x0037_675c);
    c.g[A0][0] = sx((gfx).wrapping_add(c.g[T1][0] as u32));
    if call_ptr(c, callee::COPY_VIEW_STATE, 0x0037_675c).is_err() {
        return;
    }
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void PS2GraphicsMan_PopView(PS2GraphicsMan* g)` (0x376768): pop the view
/// index, restore the saved view state from `views[curView]` and clear the
/// "matrices current" flag.
pub fn ps2_graphics_man_pop_view(c: &mut Cpu) {
    c.cycles += 20;
    let frame = Frame::open(c, 0x30);
    c.g[V1][0] = 0x230;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = abi::arg64(c, 0);
    frame.save_ra(c, 0);
    c.g[A0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(man::CUR_VIEW).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    c.mult(0, 3, 2, 3);
    c.sw(p_2(c.g[S0][0] as u32, man::CUR_VIEW), c.g[V0][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(man::VIEWS));
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let gfx = c.g[S0][0] as u32;
    let view = c.g[S1][0] as u32;
    c.ldl(2, p_2(view, vw::SCISSOR + 7));
    c.ldr(2, p_2(view, vw::SCISSOR));
    c.sdl(2, p_2(gfx, man::SCISSOR + 7));
    c.sdr(2, p_2(gfx, man::SCISSOR));
    c.g[A2][0] = Addr(view).off(vw::KEY).lw(c);
    c.g[A1][0] = sx(p_2(gfx, man::SCISSOR));
    if c.g[A2][0] == c.g[A0][0] {
        c.cycles += 4;
        c.g[RA][0] = sx(0x0037_67c0);
        c.g[A0][0] = Addr(gfx).off(man::DEFER).lw(c);
        if callee::SET_SCISSOR.call(c, 0x0037_67c0).is_err() {
            return;
        }
        c.lwc1(1, p_2(view, vw::F24));
    } else {
        c.cycles += 6;
        c.g[V1][0] = Addr(gfx).off(man::DEFER).lw(c);
        c.g[V0][0] = 0x60000;
        c.g[V0][0] |= 0x8000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(p_2(c.g[V1][0] as u32, 0x1cc4), c.g[A2][0]);
        c.lwc1(1, p_2(view, vw::F24));
    }
    c.cycles += 85;
    c.g[A0][0] = sx(p_2(gfx, man::WORLD));
    c.g[V0][0] = sx(p_2(view, vw::M60));
    c.swc1(1, p_2(gfx, man::F590C));
    c.lwc1(0, p_2(view, vw::F28));
    c.swc1(0, p_2(gfx, man::F5908));
    copy_mat(c, c.g[V0][0] as u32, c.g[A0][0] as u32);
    c.lq(3, p_2(view, vw::Q30));
    c.g[A1][0] = sx(p_2(gfx, man::CAM_MAT));
    c.g[A0][0] = sx(p_2(view, vw::MA0));
    c.sq(3, p_2(gfx, man::Q6B70));
    c.lq(2, p_2(view, vw::Q40));
    c.sq(2, p_2(gfx, man::Q6B80));
    copy_mat(c, c.g[A0][0] as u32, c.g[A1][0] as u32);
    c.g[V1][0] = sx(p_2(gfx, man::LOCAL_MAT));
    c.g[V0][0] = sx(p_2(view, vw::ME0));
    copy_mat(c, c.g[V0][0] as u32, c.g[V1][0] as u32);
    c.g[A0][0] = sx(p_2(gfx, man::CAM_MAT2));
    c.g[V0][0] = sx(p_2(view, vw::M120));
    copy_mat(c, c.g[V0][0] as u32, c.g[A0][0] as u32);
    c.lq(3, p_2(view, vw::Q160));
    c.g[A2][0] = sx(p_2(gfx, man::MAT5860));
    c.g[A1][0] = sx(p_2(view, vw::M180));
    c.sq(3, p_2(gfx, man::Q5840));
    c.lq(2, p_2(view, vw::Q170));
    c.sq(2, p_2(gfx, man::Q5850));
    copy_mat(c, c.g[A1][0] as u32, c.g[A2][0] as u32);
    c.g[A0][0] = sx(p_2(gfx, man::VIEW_MAT));
    c.g[V1][0] = sx(p_2(view, vw::M1C0));
    copy_mat(c, c.g[V1][0] as u32, c.g[A0][0] as u32);
    c.lq(2, p_2(view, vw::Q200));
    frame.restore_ra(c, 0);
    c.sq(2, p_2(gfx, man::Q58E0));
    c.lq(3, p_2(view, vw::Q210));
    c.sq(3, p_2(gfx, man::Q58F0));
    c.lwc1(0, p_2(view, vw::F2C));
    c.swc1(0, p_2(gfx, man::F5910));
    c.lq(2, p_2(view, vw::Q50));
    c.sw(p_2(gfx, man::TEX_READY), 0);
    c.sq(2, p_2(gfx, man::Q5920));
    c.lq(16, p_2(sp(c), 0x20));
    c.lq(17, p_2(sp(c), 0x10));
    frame.close(c);
}

/// `void PS2GraphicsMan_SetViewport(PS2GraphicsMan* g, float x, float y,
/// float w, float h)` (0x376a70): clamp `y`/`h` by the display height in
/// clamp modes 0..2, store the rectangle in the current view, and queue the
/// scissor.
pub fn ps2_graphics_man_set_viewport(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x10);
    frame.save_fpr(c, 23, 0x38);
    c.g[S0][0] = abi::arg64(c, 0);
    frame.save_fpr(c, 22, 0x30);
    c.mov_s(23, 14);
    frame.save_fpr(c, 21, 0x28);
    c.mov_s(22, 12);
    frame.save_fpr(c, 20, 0x20);
    c.mov_s(21, 15);
    frame.save_ra(c, 0);
    c.mov_s(20, 13);
    let gfx = abi::arg(c, 0);
    c.g[V1][0] = Addr(gfx).off(man::DISPLAY).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[RA][0] = sx(0x0037_6ab4);
    c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
    if call_ptr(c, target, 0x0037_6ab4).is_err() {
        return;
    }
    c.g[V1][0] = Addr(gfx).off(man::CLAMP_MODE).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
    let mut clamped = true;
    if c.g[V0][0] == 0 {
        clamped = false;
    } else {
        c.cycles += 2;
        if (c.g[V1][0] as i64) < 0 {
            clamped = false;
        }
    }
    if !clamped {
        c.g[V1][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    } else {
        c.cycles += 8;
        c.mtc1(4, 1);
        c.cvt_s_w(1, 1);
        c.lwc1(0, p_2(gfx, man::CLAMP_TOP));
        fmul(c, 0, 0, 1);
        if f_lt(c, 20, 0) {
            c.mov_s(20, 0);
        }
        c.cycles += 6;
        c.lwc1(0, p_2(gfx, man::CLAMP_BOTTOM));
        fmul(c, 0, 0, 1);
        if f_lt(c, 0, 21) {
            c.mov_s(21, 0);
        }
        c.cycles += 1;
        c.g[V1][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    }
    c.cycles += 33;
    c.g[A1][0] = 0x230;
    c.mov_s(12, 22);
    c.mov_s(13, 20);
    c.mult(0, 2, 3, 5);
    c.mov_s(14, 23);
    c.mov_s(15, 21);
    c.g[A0][0] = c.g[S0][0];
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
    c.swc1(22, p_2(c.g[V1][0] as u32, man::VIEWS));
    c.g[V0][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    c.mult(0, 3, 2, 5);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
    c.swc1(20, p_2(c.g[V0][0] as u32, man::VIEWS + 0x4));
    c.g[V1][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    c.mult(0, 2, 3, 5);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
    c.swc1(23, p_2(c.g[V1][0] as u32, man::VIEWS + 0x8));
    c.g[V0][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    c.mult(0, 3, 2, 5);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
    c.g[RA][0] = sx(0x0037_6b64);
    c.swc1(21, p_2(c.g[V0][0] as u32, man::VIEWS + 0xc));
    if callee::SET_SCISSOR_CLAMPED.call(c, 0x0037_6b64).is_err() {
        return;
    }
    c.g[A0][0] = Addr(gfx).off(man::DEFER).lw(c);
    c.g[RA][0] = sx(0x0037_6b70);
    c.g[A1][0] = sx(p_2(gfx, man::SCISSOR));
    if callee::SET_SCISSOR.call(c, 0x0037_6b70).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 23, 0x38);
    frame.restore_fpr(c, 22, 0x30);
    frame.restore_fpr(c, 21, 0x28);
    frame.restore_fpr(c, 20, 0x20);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_CullAgainstLights.

/// `unsigned int PS2GraphicsMan_CullAgainstLights(PS2GraphicsMan* self,
/// const Vec4* a, const Vec4* b)` (0x37df88): every light item (0xa0 bytes at
/// `self+0x1400`, count at `self+0x13ec`) is tested against the segment
/// `a`-`b` by the method in the vtable at `self+0x10d8` (entry 93); the
/// result is a bitmask of the items the method rejected (it returned != 1).
pub fn ps2graphics_man_cull_against_lights(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = 0;
        frame.save(c, S5, 0x20);
        c.g[S3][0] = 0;
        frame.save(c, S6, 0x10);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S4, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(self_.wrapping_add(0x13ec));
        let t = c.g[V0][0] == 0;
        c.g[S6][0] = c.g[A1][0];
        if !t {
            c.cycles += 2;
            c.g[S4][0] = 1;
            c.g[S1][0] = sx(self_.wrapping_add(0x1400));
            loop {
                c.cycles += 10;
                let vt = c.lw(self_.wrapping_add(0x10d8));
                c.g[V1][0] = vt;
                c.g[A1][0] = c.g[S6][0];
                c.g[A2][0] = c.g[S5][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[A0][0] = Addr((vt as u32).wrapping_add(0x2e8)).lh(c);
                let func = c.lw((vt as u32).wrapping_add(0x2ec));
                c.g[V0][0] = func;
                c.g[RA][0] = sx(0x0037_dff0);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, func as u32, 0x0037_dff0)?;
                let kept = c.g[V0][0] == c.g[S4][0];
                c.g[V0][0] = sx(r32(c, S4) << (r32(c, S2) & 31));
                if !kept {
                    c.cycles += 1;
                    c.g[S3][0] = c.g[S3][0] | c.g[V0][0];
                }
                c.cycles += 5;
                c.g[V0][0] = c.lw(self_.wrapping_add(0x13ec));
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0xa0));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0037_dfd0);
                }
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// mEmitter_Render (0x371688).

/// `void func_00371688(Emitter* e, int blend)` (0x371688): if the emitter
/// has a texture (`+0x174 != 0`, layer texture `+0x1e0 != 0`, and a valid
/// `texIds[base + trunc(frame)]`), push a render state: blend mode
/// `D_0044B420[kind]<<2` in `flagsA`, `blend<<5` in `flagsB`, that texture,
/// the emitter's own render state copied on top, and draw the emitter through
/// slot 84.
pub fn m_emitter_render(c: &mut Cpu) {
    c.cycles += 7;
    let frame = Frame::open(c, 0x20);
    let e = r32(c, A0);
    c.g[T0][0] = c.g[A0][0];
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[A3][0] = c.g[A1][0];
    guest(c, |c| {
        'body: {
            let visible = lw(c, p(e, 0x174));
            c.g[V0][0] = visible;
            if visible == 0 {
                break 'body;
            }
            c.cycles += 3;
            let layered = lw(c, p(e, 0x1e0));
            c.g[V0][0] = layered;
            let context = lw(c, gp(c, 0x2a90)) as u32;
            c.g[S0][0] = sx(context);
            if layered == 0 {
                break 'body;
            }
            c.cycles += 10;
            let _u = ldf_2(c, 0, p(e, 0x10));
            c.g[V0][0] = lw(c, p(e, 0x4));
            c.cvt_w_s(1, 0);
            c.mfc1(V1, 1); // v1 = trunc(frame)
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(context.wrapping_add(r32(c, V0)));
            let tex = lw(c, p(r32(c, V0), ctx::TEX_IDS));
            c.g[T1][0] = tex;
            let t = (tex as i64) < 0;
            c.g[V0][0] = sx(0x45_0000);
            if t {
                break 'body;
            }
            c.cycles += 53;
            let kind = lw(c, p(e, 0x8));
            c.g[V1][0] = kind;
            c.g[V0][0] = sx(0x45_0000u32.wrapping_sub(0x4be0)); // &D_0044B420
            let flags_a = lw(c, p(e, 0x1e8));
            c.g[A1][0] = flags_a;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            let flags_b = lw(c, p(e, 0x1ec));
            c.g[A2][0] = flags_b;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = sx(!0x7cu32);
            let layer = lw(c, p(r32(c, V1), 0));
            c.g[V0][0] = layer;
            c.g[A3][0] = sx(r32(c, A3) << 5);
            c.g[V1][0] = sx(!0x3e0u32);
            c.g[A1][0] = flags_a & c.g[A0][0];
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[A2][0] = flags_b & c.g[V1][0];
            c.g[A3][0] = c.g[A3][0] & 0x3e0;
            c.g[V0][0] = c.g[V0][0] & 0x7c;
            c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
            c.g[A2][0] = c.g[A2][0] | c.g[A3][0];
            let a1 = c.g[A1][0];
            Addr(p(e, 0x1e8)).sw(c, a1);
            let a2 = c.g[A2][0];
            Addr(p(e, 0x1ec)).sw(c, a2);
            c.g[A1][0] = c.g[T0][0];
            let tex = c.g[T1][0];
            c.sh(p(e, 0x1f4), tex);
            // ctx->top[1] = ctx->top[0]; ctx->top++.
            let top = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V0][0] = sx(top);
            copy_render_state(c, V1, A0, A2, top, top);
            let top = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(top);
            c.g[V0][0] = sx(p(top, 0x14));
            let new_top = c.g[V0][0];
            Addr(p(context, ctx::TOP)).sw(c, new_top);
            // *ctx->top = e->rs.
            copy_render_state(c, V0, A0, A2, p(e, 0x1e4), top);
            let vt = lw(c, p(context, ctx::VTABLE)) as u32;
            c.g[V0][0] = sx(vt);
            let (delta, f) = vslot(c, vt, 0x2a0);
            c.g[A0][0] = delta;
            c.g[V1][0] = f;
            let target = f as u32;
            c.g[A0][0] = sx(context.wrapping_add(delta as u32));
            call_ptr(c, target, 0x0037_17a0)?;
            let top = lw(c, p(context, ctx::TOP));
            c.g[V0][0] = top;
            c.g[V0][0] = sx((top as u32).wrapping_sub(0x14));
            let top = c.g[V0][0];
            Addr(p(context, ctx::TOP)).sw(c, top);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `v = sx(v + n)` on a GPR (an `addiu`/`daddiu`).
#[inline(always)]
pub(crate) fn addi(c: &mut Cpu, reg: usize, n: u32) {
    c.g[reg][0] = sx(r32(c, reg).wrapping_add(n));
}

// ---------------------------------------------------------------------------
// Shadow quads.

/// The gfx manager singleton (`D_004A5B80`, `gp+0x2a90`).
pub(crate) const GFX_MAN: i32 = 0x2a90;

pub(crate) const GFX_MAN_VTBL: u32 = 0x10d8;

/// `void BoaderShadow_Submit(Shadow* self)` (0x375890): submit the quad
/// `(*(self+0xc), *(self+0x10), *(self+0x14), 1)` with the two vertices at
/// +0x90/+0xd0 and the texture at +0x170 through the gfx manager's virtual
/// `v67`, whose signature is `(Vec4 colour /*a1*/, void* uv /*a2*/,
/// void* pos /*a3*/, int tex /*t0*/, float alpha /*f12*/)`.
pub fn boader_shadow_submit(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.lw(gp(c, GFX_MAN));
        frame.save_ra(c, 0x10);
        c.g[V1][0] = c.g[A0][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 3);
        c.g[A1][0] = c.g[SP][0];
        c.g[T1][0] = c.lw(at(c, V0, GFX_MAN_VTBL));
        c.g[A2][0] = sx(r32(c, V1).wrapping_add(0x90));
        c.lwc1(2, at(c, V1, 0xc));
        c.mov_s(12, 3);
        c.lwc1(0, at(c, V1, 0x10));
        c.g[A3][0] = sx(r32(c, V1).wrapping_add(0xd0));
        c.lwc1(1, at(c, V1, 0x14));
        c.g[A0][0] = c.lh(at(c, T1, 0x218));
        let sp = sp(c);
        c.swc1(2, sp);
        c.swc1(0, p(sp, 0x4));
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.swc1(1, p(sp, 0x8));
        c.swc1(3, p(sp, 0xc));
        c.g[V0][0] = c.lw(at(c, T1, 0x21c));
        let target = r32(c, V0);
        c.g[T0][0] = c.lw(at(c, V1, 0x170));
        call_ptr(c, target, 0x0037_58ec)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bezier patch cache.

/// `void BezierPatchCache_MarkUsed(Cache* self, int count, Item* items /*a2*/,
/// int idx /*a3*/, int bit /*t0*/, int stride /*t1*/)` (0x374a88): set bit
/// `bit` (and the two low bits) in the `+4` word of each item's block in
/// `self->lists[idx]`, skipping items whose short index at +0xc is negative.
pub fn bezier_patch_cache_mark_used(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x1;
    c.g[V0][0] = sx(1u32 << (r32(c, T0) & 31));
    let empty = (c.g[A1][0] as i64) <= 0;
    c.g[T0][0] = c.g[V0][0] | 0xc;
    if !empty {
        c.cycles += 4;
        c.g[V1][0] = sx(r32(c, A3) << 2);
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x18));
        c.g[A3][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[T1][0] = 0xc;
        loop {
            c.cycles += 3;
            c.g[A0][0] = c.lh(at(c, A2, 0xc));
            let skipped = (c.g[A0][0] as i64) < 0;
            c.mult(0, A0, A0, T1);
            if !skipped {
                c.cycles += 5;
                c.g[V1][0] = c.lw(r32(c, A3));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.g[V0][0] = c.lw(at(c, V1, 0x4));
                c.g[V0][0] |= c.g[T0][0];
                c.sw(at(c, V1, 0x4), c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1));
            let more = c.g[A1][0] != 0;
            addi(c, A2, 0x10);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0037_4aa8);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Irradiance.

/// `void Irradiance_AddScaledW(Vec4* dst, const Vec4* src, float dt /*f12*/)`
/// (0x389620): `dst->w += src->w * dt` for each of the ten vectors.
pub fn irradiance_add_scaled_w(c: &mut Cpu) {
    c.cycles += 4;
    addi(c, A1, 0xc);
    addi(c, A0, 0xc);
    c.g[V0][0] = 0x9;
    loop {
        c.cycles += 9;
        c.lwc1(1, r32(c, A1));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
        c.lwc1(0, r32(c, A0));
        addi(c, A1, 0x10);
        c.mul_s(1, 1, 12);
        c.add_s(0, 0, 1);
        c.swc1(0, r32(c, A0));
        let more = (c.g[V0][0] as i64) >= 0;
        addi(c, A0, 0x10);
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_9630);
        }
    }
}

/// `void cPS2GridMesh_v03(cPS2GridMesh* self, const Vec3* verts)`
/// (0x3852e8): pack every row of the mesh's current vertex grid into its
/// output stream (two vertices per quad), then mark the mesh packed.
pub fn c_ps2_grid_mesh_v03(c: &mut Cpu) {
    c.cycles += 18;
    let frame = Frame::open(c, 0xb0);
    frame.save(c, S2, 0x80);
    frame.save(c, S7, 0x30);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S0, 0xa0);
    c.g[S7][0] = c.g[A1][0];
    frame.save(c, S1, 0x90);
    c.g[A1][0] = 0;
    frame.save(c, S3, 0x70);
    frame.save(c, S4, 0x60);
    frame.save(c, S5, 0x50);
    frame.save(c, S6, 0x40);
    frame.save(c, FP, 0x20);
    frame.save_ra(c, 0x10);
    let this = r32(c, S2);
    c.g[V1][0] = c.lw(p(this, grid_mesh::TAB_INDEX));
    c.g[V0][0] = c.lw(p(this, grid_mesh::ROWS));
    let no_rows = (c.g[V0][0] as i64) <= 0;
    c.g[V1][0] = (c.g[V1][0] < 1) as u64;
    if !no_rows {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx(this.wrapping_add(grid_mesh::ROW_TABS));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[FP][0] = 0xc;
        c.sw(sp(c), c.g[V0][0]);
        c.g[V0][0] = c.lhu(p(this, grid_mesh::WIDTH));
        loop {
            c.cycles += 10;
            c.g[A0][0] = sx((c.g[A1][0] as u32) << 4);
            c.g[S6][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[A2][0] = c.lw(sp(c));
            c.mult(0, 5, 5, 2);
            c.g[S5][0] = 0;
            c.g[V1][0] = c.lw(r32(c, A2));
            c.g[S3][0] = 0;
            c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            let empty_row = c.g[V0][0] == 0;
            c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            if !empty_row {
                c.cycles += 4;
                c.mult(0, 2, 7, 30);
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                c.mult(0, 2, 5, 30);
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                loop {
                    c.cycles += 17;
                    c.g[A1][0] = c.lw(p(r32(c, S4), 0x04));
                    c.g[A2][0] = c.g[S5][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S2][0];
                    if callee::PACK_POSITION.call(c, 0x0038_5394).is_err() {
                        return;
                    }
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                    c.g[A1][0] = c.lw(p(r32(c, S4), 0x04));
                    c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(1));
                    c.g[A3][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S2][0];
                    if callee::PACK_POSITION.call(c, 0x0038_53ac).is_err() {
                        return;
                    }
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(2));
                    c.g[V0][0] = c.lhu(p(this, grid_mesh::WIDTH));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x0c));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x0c));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0038_5380);
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = c.lw(p(this, grid_mesh::ROWS));
            c.g[A1][0] = c.g[S6][0];
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lhu(p(this, grid_mesh::WIDTH));
            if event_due(c) {
                deliver_event_here(c, 0x0038_5348);
            }
        }
    }
    c.cycles += 14;
    c.g[V0][0] = 1;
    frame.restore(c, S0, 0xa0);
    c.sw(p(this, grid_mesh::PACKED), c.g[V0][0]);
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
}

/// `D_00493820`: the vtable `BezierManBase`'s members point at.
pub(crate) const BEZIER_MAN_VTABLE: u32 = 0x0049_3820;

/// `BezierManBase* BezierManBase_ctor(BezierManBase* b)` (0x374c90): installs
/// the vtable at +4, publishes the object at `D_004A4248` and returns it.
pub fn bezier_man_base_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x49_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(BEZIER_MAN_VTABLE);
    let selfp = c.g[A0][0] as u32;
    c.sw(selfp.wrapping_add(4), c.g[V1][0]);
    c.sw(gp_2(c, 0x1158), c.g[A0][0]);
    abi::ret(c);
}

/// `void PS2GraphicsMan_SetTexAllocFlags(this, int flags)` (0x0037c8a0):
/// forward to the texture manager at `+0x18f4`.
pub fn ps2_graphics_man_set_tex_alloc_flags(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x18f4)).lw(c),
        callee::TEX_MAN_SET_ALLOC_FLAGS,
        0x0037_c8b0,
    )
}

/// `void PS2GraphicsMan_v118(this)` (0x00386cf0): `GraphicsMan_EmptyHook`.
pub fn ps2_graphics_man_v118(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |_| {},
        callee::GRAPHICS_MAN_EMPTY_HOOK,
        0x0038_6d00,
    )
}

// ---------------------------------------------------------------------------
// cPS2GridMesh_Draw / Irradiance_GetEntryByName / cWorldCache_Reset.

/// `void cPS2GridMesh_v07(GridMesh* self, int a, void* b, int c)` (0x385be0):
/// `self->v08(a, **(short**)((char*)b + 4), c)` — the draw call, with the
/// mesh's second sub-object read out of `self + 4`.
pub fn c_ps2_grid_mesh_v07(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(4)).lw(c);
        c.g[T0][0] = Addr(r32(c, A0).wrapping_add(4)).lw(c);
        c.g[A2][0] = Addr(r32(c, V0)).lh(c);
        c.g[V0][0] = Addr(r32(c, T0).wrapping_add(0x40)).lh(c);
        c.g[V1][0] = Addr(r32(c, T0).wrapping_add(0x44)).lw(c);
        let method = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        call_ptr(c, method, 0x0038_5c04)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `IrradianceEntry* Irradiance_GetEntryByName(IrradianceSet* s, char* name)`
/// (0x38ac20): `GetEntry(s, FindIndex(s, name))`.
pub fn irradiance_get_entry_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::IRRADIANCE_FIND_INDEX.call(c, 0x0038_ac34)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        callee::IRRADIANCE_GET_ENTRY.call(c, 0x0038_ac40)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The deleting-destructor prologue: install the class vtable, clear the
/// class's singleton global, then `delete this` when `flags & 1`.
pub(crate) fn deleting_dtor(
    c: &mut Cpu,
    ret: u32,
    vtable_slot: u32,
    vtable: u32,
    gp_slot: u32,
) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    let obj = c.g[A0][0];
    c.g[V0][0] = sx(vtable);
    frame.save_ra(c, 0);
    Addr(obj as u32).off(vtable_slot).sw(c, c.g[V0][0]);
    c.g[A1][0] &= 1;
    let owned = c.g[A1][0] != 0;
    c.sw(r32(c, GP).wrapping_add(gp_slot), 0);
    if owned {
        c.cycles += 2;
        callee::BUILTIN_DELETE_2.call(c, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `BezierManBase::~BezierManBase()` (0x374ca8): the bezier manager base
/// destructor (`operator delete` when the low flag bit is set).
pub fn bezier_man_base_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0037_4cd0, 4, 0x0049_3820, 0x1158))
}

/// `void* PS2GraphicsMan_Create()` (0x375a08): allocate the 0x75e0-byte
/// graphics manager and construct it.
pub fn ps2_graphics_man_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 0x75e0, 0x0049_2878, 0, 0x0037_5a28, |c, ret| {
            call_ptr(c, callee::GRAPHICS_MAN_CTOR, ret)
        })
    })
}

/// `int Renderer_Vif1DmacHandler(int cause, PS2GraphicsMan* man)` (0x382650):
/// the VIF1 DMA-end interrupt handler: finish the DMA transfer and re-enable
/// interrupts; any other cause is a fatal assertion in the original.
pub fn renderer_vif1_dmac_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        let dma_end = c.g[A0][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if dma_end {
            c.cycles += 8;
            c.g[A0][0] = c.g[A1][0];
            callee::GRAPHICS_ON_VIF1_DMA_END.call(c, 0x0038_266c)?;
            c.ei();
            frame.restore_ra(c, 0);
            c.g[V0][0] = 1;
            frame.close(c);
        } else {
            c.cycles += 1;
            c.brk(0x0038_2660, 0);
        }
        Ok(())
    })
}

/// `mEmitter* mEmitter_ctor(mEmitter* self)` (0x3714B8): the dynamic-emitter
/// constructor, vtable `0x4930D0` and empty particle lists.
pub fn emitter_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::DYNAMIC_EMITTER_CTOR.call(c, 0x0037_14cc)?;
        c.g[V1][0] = sx(0x0049_30d0);
        c.g[V0][0] = c.g[S0][0];
        Addr((this as u32).wrapping_add(0x200)).sw(c, 0);
        Addr((this as u32).wrapping_add(0x1f8)).sw(c, 0x0049_30d0);
        Addr((this as u32).wrapping_add(0x204)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Graphics / UI.

/// `void PS2GraphicsMan_SetDefaultDisplayMode(PS2GraphicsMan* self, u32 mode
/// /*a1*/)` (0x376578): call virtual slot 0xf2 with the default 0x200x0x1c0
/// (0x18x0x20x0x18) screen layout and `mode`.
pub fn ps2_graphics_man_set_default_display_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        c.g[T2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x200;
        c.g[A2][0] = 0x1c0;
        c.g[A3][0] = 0x18;
        c.g[V1][0] = Addr(abi::arg(c, 0)).off(0x10d8).lw(c);
        c.g[T0][0] = 0x20;
        c.g[T1][0] = Addr(r32(c, V1)).off(0x3d0).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x3d4).lw(c);
        c.g[A0][0] = add32_2(c.g[A0][0], c.g[T1][0]);
        let func = r32(c, V0);
        c.g[RA][0] = sx(0x0037_65ac);
        c.g[T1][0] = 0x18;
        c.pc = func;
        call_indirect(c, func);
        if c.pc != 0x0037_65ac {
            return Err(crate::call::Unwound);
        }
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}

/// `void PS2GraphicsMan_FreeTexture(PS2GraphicsMan* self, int index)`
/// (0x37cb50): free texture `index` through the texture manager at `+0x18f4`
/// and clear the slot at `+0x18f8 + index*4`.
pub fn ps2_graphics_man_free_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S1)).off(0x18f4).lw(c);
        callee::TEX_MAN_FREE_TEXTURE.call(c, 0x0037_cb70)?;
        c.g[S0][0] = sx((r32(c, S0) as u32) << 2);
        frame.restore_ra(c, 0);
        c.g[S1][0] = add32_2(c.g[S1][0], c.g[S0][0]);
        c.sw(p(r32(c, S1), 0x18f8), 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        leave(c, 0x30);
        Ok(())
    })
}

/// `void PS2GraphicsMan_v59(void* self, int* out, int a2, int a3)` (0x37cbe8):
/// ask slot 0x68 of the particle manager at `+0x10d8` for `(a2, a3, 0)` and
/// store the result in `out[1]`.
pub fn ps2_graphics_man_v59(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[T0][0] = c.lw(r32(c, A0).wrapping_add(0x10d8));
        c.g[A2][0] = c.g[A3][0];
        c.g[A3][0] = 0;
        let vt = r32(c, T0);
        c.g[V1][0] = c.lh(vt.wrapping_add(0x1a0));
        c.g[V0][0] = c.lw(vt.wrapping_add(0x1a4));
        let method = r32(c, V0);
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(r32(c, V1)));
        call_ptr(c, method, 0x0037_cc18)?;
        c.sw(r32(c, S0).wrapping_add(4), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Graphics managers.

/// `void PS2GraphicsMan_ResetTextureSlots(this, flags)` (0x37c7c0): clear the
/// 0x1f40-byte particle slot array at `+0x3838` and pass `flags` to the
/// texture allocator.
pub fn ps2_graphics_man_reset_texture_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x1f40;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3838));
        c.g[A1][0] = 0;
        call_ptr(c, 0x003e_6448, 0x0037_c7e8)?;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x18f4)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        TEX_MAN_SET_ALLOC_FLAGS_2.call(c, 0x0037_c7f4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2GraphicsMan_RemoveInterruptHandlers(this)` (0x389118): detach the
/// two interrupt/DMA handlers registered for the graphics manager.
pub fn ps2_graphics_man_remove_interrupt_handlers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x2;
        call_ptr(c, 0x0042_4880, 0x0038_9130)?;
        c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x5ac0)).lw(c);
        c.g[A0][0] = 0x2;
        call_ptr(c, 0x0042_3aa0, 0x0038_913c)?;
        c.g[A0][0] = 0x1;
        call_ptr(c, 0x0042_4950, 0x0038_9144)?;
        c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x5ac4)).lw(c);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0042_3ad0, 0x0038_9150)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Emitter teardown.

/// `void mEmitter_dtor(mEmitter* self, int flags)` (0x3714f8): install the
/// class vtable at +0x1f8, release the emitter's data and destroy the
/// dynamic-emitter base. 19 cycles plus the callees.
pub fn memitter_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0049_30d0);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.sw(p(self_, 0x1f8), c.g[V0][0]);
        c.g[A1][0] = 0;
        callee::MEMITTER_FREE_DATA.call(c, 0x0037_1524)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::DYNAMIC_EMITTER_DTOR.call(c, 0x0037_1530)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2GraphicsMan_SetBrightnessAlpha(void* self, float a)`
/// (0x37c720): store the brightness ramp for `1 - clamp(a, 1)`, scaled to
/// 128 steps.
pub fn ps2_graphics_man_set_brightness_alpha(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.mtc1(0, 0); // f0 = 0
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.c_le_s(0, 12); // 0 <= a
        frame.save_ra(c, 0);
        c.g[AT][0] = 0x3f80_0000; // 1.0
        c.mtc1(1, 1);
        let below = !c.fcc();
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        if !below {
            c.cycles += 1;
            c.min_s(0, 12, 1);
        }
        c.cycles += 10;
        c.sub_s(0, 1, 0); // 1 - clamp
        c.g[AT][0] = 0x4300_0000; // 128.0
        c.mtc1(1, 12);
        c.g[RA][0] = sx(0x0037_c75c);
        c.mul_s(12, 0, 12);
        call_ptr(c, 0x0041_39f8, 0x0037_c75c)?;
        Addr(p(this, 0x59f8)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BoaderShadow_AllocTexture(BoaderShadow* s)` (0x375990): if the
/// texture handle at `+0x170` is negative (not yet allocated), allocate it
/// through the graphics manager's `v52` with the stored size/format.
pub fn boader_shadow_alloc_texture(c: &mut Cpu) {
    c.cycles += 7;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[V0][0] = Addr(p_3(c, S0, 0x170)).lw(c);
    if (c.g[V0][0] as i64) < 0 {
        c.cycles += 11;
        c.g[T0][0] = Addr(gp_3(c, 0x2a90)).lw(c);
        c.g[A3][0] = 2;
        c.g[A1][0] = Addr(p_3(c, S0, 0x174)).lw(c);
        c.g[V1][0] = Addr(p_3(c, T0, 0x10d8)).lw(c);
        c.g[A2][0] = Addr(p_3(c, S0, 0x178)).lw(c);
        c.g[A0][0] = Addr(p_3(c, V1, 0x1a0)).lh(c);
        c.g[V0][0] = Addr(p_3(c, V1, 0x1a4)).lw(c);
        let target = abi::r32(c, V0);
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if call_ptr(c, target, 0x0037_59d0).is_err() {
            return;
        }
        c.sw(p_3(c, S0, 0x170), c.g[V0][0]);
    }
    c.cycles += 3;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `addu reg, delta`: a vtable's adjusted `this`.
#[inline(always)]
pub(crate) fn this_with_delta(obj: u64, delta: u64) -> u64 {
    abi::addu(obj, delta)
}

/// `void* PS2GraphicsMan_SelectMatchingDisplayMode(PS2GraphicsMan* self)`
/// (0x376468): pick the first display mode the platform accepts, else 0.
pub fn ps2_graphics_man_select_matching_display_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = Addr(this).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x3d8).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x3dc).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0037_648c);
        c.g[A0][0] = this_with_delta(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, target, 0x0037_648c)?;
        if (c.g[V0][0] as i64) < 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.g[V1][0] = Addr(r32(c, S0)).off(0x10d8).lw(c);
            c.cycles += 5;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = Addr(r32(c, V1)).off(0x20).lh(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x24).lw(c);
            let target = r32(c, V0);
            c.g[RA][0] = sx(0x0037_64b0);
            c.g[A0][0] = this_with_delta(c.g[S0][0], c.g[A0][0]);
            call_ptr(c, target, 0x0037_64b0)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PS2GraphicsMan_LoadTexture(self, a1, a2, a3, a4, a5, a6, a7)`
/// (0x37caf8): forward to the texture manager and mark the returned slot used.
pub fn ps2_graphics_man_load_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = Addr(frame.at(c, 0x48)).lw(c);
        c.g[V0][0] = Addr(frame.at(c, 0x40)).lw(c);
        c.g[T4][0] = Addr(frame.at(c, 0x50)).lw(c);
        c.g[T5][0] = Addr(frame.at(c, 0x58)).lw(c);
        c.g[A0][0] = Addr(r32(c, S0)).off(0x18f4).lw(c);
        Addr(frame.at(c, 0x8)).sw(c, c.g[V1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[V0][0]);
        Addr(frame.at(c, 0x10)).sw(c, c.g[T4][0]);
        c.g[RA][0] = sx(0x0037_cb30);
        Addr(frame.at(c, 0x18)).sw(c, c.g[T5][0]);
        callee::TEX_MAN_LOAD_TEXTURE.call(c, 0x0037_cb30)?;
        c.g[V1][0] = sx(r32(c, V0) << 2);
        c.g[A0][0] = 1;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
        frame.restore_ra(c, 0x20);
        Addr(r32(c, S0)).off(0x3838).sw(c, c.g[A0][0]);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2GraphicsMan_EmitModelMatrix2(PS2GraphicsMan* self, Quad** pp,
/// const Matrix* m)` (0x37e040): append a model-matrix packet to the
/// graphics packet stream.
pub fn ps2_graphics_man_emit_model_matrix2(c: &mut Cpu) {
    c.cycles += 22;
    c.g[V0][0] = Addr(r32(c, A1)).lw(c);
    c.por(3, 0, 0);
    c.g[V1][0] = 0x1000_0000;
    c.g[V1][0] |= 0x5;
    c.g[A2][0] = 0x6c04_0000;
    c.g[A2][0] |= 0x8005;
    c.g[A2][0] <<= 32;
    c.sq(3, r32(c, V0));
    c.g[A3][0] = sx(r32(c, V0).wrapping_add(0x20));
    c.sd(p(r32(c, V0), 0x18), c.g[A2][0]);
    c.sd(p(r32(c, V0), 0x10), 0);
    c.lqc2(1, r32(c, A0));
    c.lqc2(2, p(r32(c, A0), 0x10));
    c.lqc2(3, p(r32(c, A0), 0x20));
    c.lqc2(4, p(r32(c, A0), 0x30));
    c.sqc2(1, r32(c, A3));
    c.sqc2(2, p(r32(c, A3), 0x10));
    c.sqc2(3, p(r32(c, A3), 0x20));
    c.sqc2(4, p(r32(c, A3), 0x30));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x60));
    c.sw(r32(c, A1), c.g[V0][0]);
    abi::ret(c);
}

/// `void PS2GraphicsMan_OnVif1DmaEnd(PS2GraphicsMan* self)` (0x382688): run
/// the DMA-end state machine (start, finish or idle).
pub fn ps2_graphics_man_on_vif1_dma_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = 1;
        c.g[V0][0] = Addr(r32(c, A1)).off(0x5a8c).lw(c);
        let idle = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 2;
        if !idle {
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, A1)).off(0x5ac8).lw(c);
            Addr(r32(c, A1)).off(0x5a8c).sw(c, c.g[V0][0]);
            c.g[RA][0] = sx(0x0038_26b4);
            call_ptr(c, callee::ISIGNAL_SEMA, 0x0038_26b4)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(r32(c, A1)).off(0x5a8c).lw(c);
        c.g[V0][0] = 3;
        let not_running = c.g[V1][0] != c.g[V0][0];
        frame.restore_ra(c, 0);
        if !not_running {
            c.cycles += 3;
            c.g[V0][0] = 4;
            Addr(r32(c, A1)).off(0x5a8c).sw(c, c.g[V0][0]);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `PS2GraphicsMan` fields.
pub(crate) const GM_DISPLAY_MODE: u32 = 0x10dc;

pub(crate) const GM_VIEWPORTS: u32 = 0x6d20;

pub(crate) const GM_VIEWPORT_STRIDE: u32 = 0x230;

pub(crate) const GM_FOV: u32 = 0x6d38;

pub(crate) const GM_FOG_MODE: u32 = 0x6b94;

pub(crate) const GM_FOG_NEAR: u32 = 0x6b98;

pub(crate) const GM_FOG_FAR: u32 = 0x6b9c;

pub(crate) const GM_FOG_COLOR_A: u32 = 0x6ba0;

pub(crate) const GM_FOG_COLOR_B: u32 = 0x6ba4;

pub(crate) const GM_TEXTURES: u32 = 0x18f4;

pub(crate) const GM_LIGHTS: u32 = 0x13ec;

pub(crate) const GM_LIGHT_STRIDE: u32 = 0xa0;

pub(crate) const GM_VIEW_MATRICES: u32 = 0x6af0;

pub(crate) const GM_VIEW_ENTRIES: u32 = 0x5930;

pub(crate) const GM_VIEW_PAIRS: u32 = 0x59b0;

pub(crate) const GM_VIEW_PAIR_SRC: u32 = 0x5900;

pub(crate) const GM_VIEW_COUNT: u32 = 0x59c0;

pub(crate) const GM_PHASE2_TEX: u32 = 0x1440;

pub(crate) const GM_PHASE2_VTX: u32 = 0x1400;

pub(crate) const GM_PHASE2_Q: u32 = 0x13f0;

pub(crate) const GM_PHASE2_F: u32 = 0x1480;

pub(crate) const GM_PHASE2_I: u32 = 0x1484;

/// `lui at, bits; mtc1 at, f` — a float constant through `at`.
#[inline(always)]
pub(crate) fn fconst_2(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = bits as u64;
    c.f[f] = bits;
}

// ---------------------------------------------------------------------------
// Graphics manager.

/// `PS2GraphicsMan_GetDisplayModeInfo` (0x00376560): the display-mode record
/// for mode `idx` (0xa4 bytes each) after the pointer at `+0x59cc`.
pub fn ps2_graphics_man_get_display_mode_info(c: &mut Cpu) {
    c.cycles += 5;
    let gm = abi::arg(c, 0);
    let idx = abi::arg(c, 1);
    c.g[V1][0] = 0xa4;
    c.g[V0][0] = c.lw(p(gm, 0x59cc));
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = add32(c.g[V0][0], c.g[A1][0]);
    abi::ret(c);
}

/// `PS2GraphicsMan_GetViewport` (0x00376b90): copy viewport `mode`'s four
/// floats (0x230-byte records at `+0x6d20`) to `out` as integers converted
/// back to floats. The manager is `a1` and `out` is `a0`, as the original
/// reads them.
pub fn ps2_graphics_man_get_viewport(c: &mut Cpu) {
    c.cycles += 31;
    let out = abi::arg(c, 0);
    let gm = abi::arg(c, 1);
    c.g[V1][0] = c.lw(p(gm, GM_DISPLAY_MODE));
    c.g[A2][0] = GM_VIEWPORT_STRIDE as u64;
    c.g[V0][0] = c.g[A0][0];
    c.mult(0, V1, V1, A2);
    c.g[V1][0] = add32(c.g[V1][0], GM_VIEWPORTS as u64);
    c.g[A1][0] = add32(gm as u64, c.g[V1][0]);
    let vp = r(c, A1);
    c.lwc1(0, p(vp, 0xc));
    c.lwc1(2, vp);
    c.cvt_w_s(1, 0);
    c.mfc1(7, 1);
    c.lwc1(3, p(vp, 4));
    c.lwc1(1, p(vp, 8));
    c.cvt_w_s(0, 2);
    c.mfc1(3, 0);
    c.cvt_w_s(0, 3);
    c.mfc1(5, 0);
    c.cvt_w_s(0, 1);
    c.mfc1(6, 0);
    c.mtc1(3, 0);
    c.cvt_s_w(0, 0);
    c.mtc1(5, 1);
    c.cvt_s_w(1, 1);
    c.mtc1(6, 2);
    c.cvt_s_w(2, 2);
    c.mtc1(7, 3);
    c.cvt_s_w(3, 3);
    c.swc1(0, out);
    c.swc1(1, p(out, 4));
    c.swc1(2, p(out, 8));
    c.swc1(3, p(out, 0xc));
    abi::ret(c);
}

/// `PS2GraphicsMan_GetFov` (0x00376c10): the field of view of the current
/// display mode (float at `+0x6d38` of its viewport record), returned in `f0`.
pub fn ps2_graphics_man_get_fov(c: &mut Cpu) {
    c.cycles += 6;
    let gm = abi::arg(c, 0);
    c.g[V0][0] = c.lw(p(gm, GM_DISPLAY_MODE));
    c.g[V1][0] = GM_VIEWPORT_STRIDE as u64;
    c.mult(0, A1, V0, V1);
    c.g[V0][0] = add32(c.g[A1][0], c.g[A0][0]);
    c.lwc1(0, p(r(c, V0), GM_FOV));
    abi::ret(c);
}

/// `PS2GraphicsMan_GetNearFarAspect` (0x00376c28): the z/x/y of display mode
/// `mode`'s near/far/aspect record into `a`, `b`, `c`.
pub fn ps2_graphics_man_get_near_far_aspect(c: &mut Cpu) {
    c.cycles += 12;
    let gm = abi::arg(c, 0);
    let (out_a, out_b, out_c) = (abi::arg(c, 1), abi::arg(c, 2), abi::arg(c, 3));
    c.g[V0][0] = c.lw(p(gm, GM_DISPLAY_MODE));
    c.g[V1][0] = GM_VIEWPORT_STRIDE as u64;
    c.mult(0, V0, V0, V1);
    c.g[V0][0] = add32(c.g[V0][0], GM_VIEWPORTS as u64);
    c.g[A0][0] = add32(gm as u64, c.g[V0][0]);
    let rp = r(c, A0);
    c.lwc1(0, p(rp, 0x18));
    c.swc1(0, out_a);
    c.lwc1(1, p(rp, 0x10));
    c.swc1(1, out_b);
    c.lwc1(0, p(rp, 0x14));
    c.swc1(0, out_c);
    abi::ret(c);
}

/// `PS2GraphicsMan_SetFogMode` (0x00377950): store the fog mode and set its
/// three fog colour scales/offsets. Modes 1 and 2 have their own triples;
/// mode 0 and everything else falls back to the default white.
pub fn ps2_graphics_man_set_fog_mode(c: &mut Cpu) {
    c.cycles += 3;
    let gm = abi::arg(c, 0);
    let mode = abi::arg64(c, 1);
    c.g[V0][0] = 1;
    let is_one = mode == c.g[V0][0];
    c.sw(p(gm, GM_FOG_MODE), mode);
    if is_one {
        // Mode 1: 0.125 towards 0.75.
        c.cycles += 7;
        fconst_2(c, 0, 0x3f40_0000);
        fconst_2(c, 1, 0x3e00_0000);
        c.swc1(0, p(gm, GM_FOG_COLOR_B));
        c.swc1(1, p(gm, GM_FOG_NEAR));
        c.cycles += 3;
        c.swc1(0, p(gm, GM_FOG_FAR));
        c.swc1(0, p(gm, GM_FOG_COLOR_A));
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = ((mode as i64) < 2i64) as u64;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = 2;
        c.cycles += 2;
        if mode == c.g[V0][0] {
            // Mode 2: near 0, far/alpha 1, 0.75 in the colour slot.
            c.cycles += 9;
            fconst_2(c, 1, 0x3f80_0000);
            fconst_2(c, 0, 0x3f40_0000);
            c.swc1(1, p(gm, GM_FOG_COLOR_B));
            c.swc1(0, p(gm, GM_FOG_COLOR_A));
            c.sw(p(gm, GM_FOG_NEAR), 0);
            c.swc1(1, p(gm, GM_FOG_FAR));
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    if mode == 0 {
        c.sw(p(gm, GM_FOG_NEAR), 0);
        c.cycles += 3;
        fconst_2(c, 0, 0x3f80_0000);
        c.swc1(0, p(gm, GM_FOG_COLOR_B));
        c.cycles += 3;
        c.swc1(0, p(gm, GM_FOG_FAR));
        c.swc1(0, p(gm, GM_FOG_COLOR_A));
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `PS2GraphicsMan_SetFogParams` (0x003779e0): `*(self + 0x6b94) = *src`
/// (20 bytes: the fog record).
pub fn ps2_graphics_man_set_fog_params(c: &mut Cpu) {
    c.cycles += 12;
    let gm = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    c.ldl(2, p(src, 7));
    c.ldr(2, src);
    c.ldl(3, p(src, 0xf));
    c.ldr(3, p(src, 8));
    c.g[A2][0] = c.lw(p(src, 0x10));
    c.sdl(2, p(gm, GM_FOG_MODE + 7));
    c.sdr(2, p(gm, GM_FOG_MODE));
    c.sdl(3, p(gm, GM_FOG_MODE + 0xf));
    c.sdr(3, p(gm, GM_FOG_MODE + 8));
    c.sw(p(gm, GM_FOG_MODE + 0x10), c.g[A2][0]);
    abi::ret(c);
}

/// `PS2GraphicsMan_IsTextureValid` (0x0037cb90): negative indices are always
/// valid (the default texture); indices past 1999 are not; otherwise the
/// table entry must be non-null.
pub fn ps2_graphics_man_is_texture_valid(c: &mut Cpu) {
    c.cycles += 2;
    let gm = abi::arg(c, 0);
    let idx = abi::arg64(c, 1);
    let negative = (idx as i64) < 0;
    c.g[V1][0] = c.lw(p(gm, GM_TEXTURES));
    if negative {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = ((idx as i64) < 2000i64) as u64;
    let in_range = c.g[V0][0] != 0;
    c.g[V0][0] = sx((idx as u32) << 2);
    if !in_range {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = c.lw(p(r(c, V0), 8));
    c.g[V0][0] = (0u64 < c.g[V1][0]) as u64;
    abi::ret(c);
}

/// `PS2GraphicsMan_AddLight` (0x0037d758): append a light (quad, two
/// matrices, a float, an int) to the table at `+0x13ec`/`+0x13f0`.
pub fn ps2_graphics_man_add_light(c: &mut Cpu) {
    c.cycles += 42;
    let gm = abi::arg(c, 0);
    let quad = abi::arg(c, 1);
    let m0 = abi::arg(c, 2);
    let m1 = abi::arg(c, 3);
    c.g[V1][0] = c.lw(p(gm, GM_LIGHTS));
    c.g[T2][0] = GM_LIGHT_STRIDE as u64;
    c.lq(9, quad);
    c.mult(0, V0, V1, T2);
    c.g[V1][0] = add32(c.g[V0][0], gm as u64);
    c.sq(9, p(r(c, V1), GM_PHASE2_Q));
    c.g[V0][0] = c.lw(p(gm, GM_LIGHTS));
    c.mult(0, V1, V0, T2);
    c.g[V0][0] = add32(c.g[V1][0], gm as u64);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(GM_PHASE2_VTX));
    c.lqc2(1, m0);
    c.lqc2(2, p(m0, 0x10));
    c.lqc2(3, p(m0, 0x20));
    c.lqc2(4, p(m0, 0x30));
    let vtx = r(c, V0);
    c.sqc2(1, vtx);
    c.sqc2(2, p(vtx, 0x10));
    c.sqc2(3, p(vtx, 0x20));
    c.sqc2(4, p(vtx, 0x30));
    c.g[V1][0] = c.lw(p(gm, GM_LIGHTS));
    c.mult(0, V0, V1, T2);
    c.g[V1][0] = add32(c.g[V0][0], gm as u64);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(GM_PHASE2_TEX));
    c.lqc2(1, m1);
    c.lqc2(2, p(m1, 0x10));
    c.lqc2(3, p(m1, 0x20));
    c.lqc2(4, p(m1, 0x30));
    let tex = r(c, V1);
    c.sqc2(1, tex);
    c.sqc2(2, p(tex, 0x10));
    c.sqc2(3, p(tex, 0x20));
    c.sqc2(4, p(tex, 0x30));
    c.g[V0][0] = c.lw(p(gm, GM_LIGHTS));
    c.mult(0, V1, V0, T2);
    c.g[V0][0] = add32(c.g[V1][0], gm as u64);
    c.swc1(12, p(r(c, V0), GM_PHASE2_F));
    c.g[V1][0] = c.lw(p(gm, GM_LIGHTS));
    c.mult(0, V0, V1, T2);
    c.g[V1][0] = add32(c.g[V0][0], gm as u64);
    c.sw(p(r(c, V1), GM_PHASE2_I), c.g[T0][0]);
    c.g[V0][0] = c.lw(p(gm, GM_LIGHTS));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.sw(p(gm, GM_LIGHTS), c.g[V0][0]);
    abi::ret(c);
}

/// The `tex0` T-bit field (`>> shift, & 0xf`) of texture `idx`, as
/// `1 << field`: the texture's width/height in pixels.
pub(crate) fn texture_size(c: &mut Cpu, shift: u32) {
    c.cycles += 12;
    let gm = abi::arg(c, 0);
    let idx = abi::arg(c, 1);
    c.g[V0][0] = c.lw(p(gm, GM_TEXTURES));
    c.g[A1][0] = sx((idx as u32) << 2);
    c.g[A0][0] = 1;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = c.lw(p(r(c, V0), 8));
    c.g[V0][0] = c.ld(p(r(c, V1), 0x38));
    c.g[V0][0] >>= shift;
    c.g[V0][0] &= 0xf;
    c.g[V0][0] <<= 32;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
    c.g[V0][0] = sx((c.g[A0][0] as u32) << ((c.g[V0][0] as u32) & 31));
    abi::ret(c);
}

/// `PS2GraphicsMan_GetTextureWidth` (0x0037d908): the width (`tex0.TW`) of
/// texture `idx`.
pub fn ps2_graphics_man_get_texture_width(c: &mut Cpu) {
    texture_size(c, 26);
}

/// `PS2GraphicsMan_GetTextureHeight` (0x0037d938): the height
/// (`tex0.TH`) of texture `idx`.
pub fn ps2_graphics_man_get_texture_height(c: &mut Cpu) {
    texture_size(c, 30);
}

/// `PS2GraphicsMan_ClipTestBox` (0x0037de88): run the VU0 box-clip
/// microprogram at `D_570 >> 3` on the two points (`a1`, `a2`) and the clip
/// matrix (`a3`), then report 1 for the far side, 2 for the near side, 0 for
/// "no clip".
pub fn ps2_graphics_man_clip_test_box(c: &mut Cpu) {
    c.cycles += 18;
    let a = abi::arg(c, 1);
    let b = abi::arg(c, 2);
    let m = abi::arg(c, 3);
    c.g[V0][0] = 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x570));
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
    c.lqc2(15, a);
    c.lqc2(16, b);
    c.lqc2(10, m);
    c.lqc2(11, p(m, 0x10));
    c.lqc2(12, p(m, 0x20));
    c.lqc2(13, p(m, 0x30));
    c.ctc2(V0, 27);
    c.vu0().vnop();
    c.vu0().vnop();
    c.vcallmsr();
    c.cfc2(4, 1);
    c.cfc2(3, 2);
    let far = c.g[V1][0] != 0;
    c.g[V0][0] = 2;
    if far {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `PS2GraphicsMan_v97` (0x00380380): project an object's four corner
/// texcoords/colours into `pos` (3 floats each), `uv` (2 floats each) and
/// `col` (4 floats each), scaling the model's corner table by its scale.
///
/// `obj` (+0x80 model, +0x98 extra offset), `idx` (corner base), `pos`,
/// `uv`, `col` are passed in `a1`, `a2`, `a3`, `t0`, `t1` as the original
/// reads them.
pub fn ps2_graphics_man_v97(c: &mut Cpu) {
    c.cycles += 42;
    let obj = abi::arg(c, 1);
    let pos = abi::arg(c, 3);
    let col = abi::arg64(c, 5);
    c.g[T2][0] = c.lw(p(obj, 0x80)); // model
    c.g[A2][0] = sx((abi::arg(c, 2) as u32) << 4);
    c.g[T3][0] = col;
    c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x20));
    c.g[V0][0] = c.lw(p(r(c, T2), 8)); // corner table offset
    c.g[T4][0] = 3;
    c.g[T1][0] = c.lw(p(obj, 0x98)); // extra offset
    c.g[A2][0] = add32(c.g[A2][0], c.g[V0][0]);
    c.g[A1][0] = c.lw(p(r(c, T2), 0x24)); // vertex base offset
    c.g[V0][0] = c.lw(p(r(c, A2), 4));
    c.lwc1(3, gp_4(c, -0x2760)); // 3.0517578e-5
    c.g[A0][0] = c.lw(p(r(c, V0), 0x20));
    c.g[AT][0] = 0x3980_0000;
    c.mtc1(1, 5); // f5 = 1/4096
    c.g[V1][0] = c.lw(r(c, A0));
    c.lwc1(4, gp_4(c, -0x275c)); // 0.032258
    c.g[V0][0] = c.lw(p(r(c, V1), 4));
    c.g[T1][0] = add32(c.g[V0][0], c.g[T1][0]); // data
    c.g[V0][0] = add32(c.g[V0][0], c.g[A1][0]);
    c.g[V1][0] = c.lw(p(r(c, T1), 0x14)); // colour record
    c.g[A0][0] = c.lw(p(r(c, V0), 4)); // vertex source
    c.ldl(2, p(r(c, T2), 0x1f));
    c.ldr(2, p(r(c, T2), 0x18)); // scale x, y
    c.g[A1][0] = c.lw(p(r(c, T2), 0x20)); // scale z
    c.sdl(2, p(r(c, SP), 0x17));
    c.sdr(2, p(r(c, SP), 0x10));
    c.sw(p(r(c, SP), 0x18), c.g[A1][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
    let vbase = r(c, A0);
    c.g[A1][0] = sx(vbase.wrapping_add(0xa0)); // uv source
    c.lwc1(0, p(r(c, SP), 0x10));
    c.g[A0][0] = sx(vbase.wrapping_add(0x80)); // texcoord source
    c.lwc1(1, p(r(c, SP), 0x14));
    c.lwc1(2, p(r(c, SP), 0x18));
    c.mul_s(0, 0, 3);
    c.mul_s(1, 1, 3);
    c.mul_s(2, 2, 3);
    c.swc1(0, r(c, SP)); // k.x
    c.mov_s(6, 0);
    c.mov_s(3, 1); // k.y
    c.swc1(1, p(r(c, SP), 4));
    c.swc1(2, p(r(c, SP), 8));
    c.mov_s(1, 2); // k.z
    let mut pos = pos;
    loop {
        c.cycles += 58;
        // Position: the corner shorts scaled into world units.
        c.g[V0][0] = c.lh(r(c, A1));
        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(1));
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 6);
        c.swc1(0, pos);
        c.g[V0][0] = c.lh(p(r(c, A1), 2));
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 3);
        c.swc1(0, p(pos, 4));
        c.g[V0][0] = c.lh(p(r(c, A1), 4));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(6));
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 1);
        c.swc1(0, p(pos, 8));
        pos = pos.wrapping_add(0xc);
        c.g[A3][0] = sx(pos);
        // UV: the texcoord shorts scaled to 0..1.
        c.g[V0][0] = c.lh(r(c, A0));
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 5);
        c.swc1(0, r(c, T0));
        c.g[V0][0] = c.lh(p(r(c, A0), 2));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 5);
        c.swc1(0, p(r(c, T0), 4));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(8));
        // Colour: the pack16 record's four channels unpacked.
        c.g[V0][0] = c.lhu(r(c, V1));
        c.g[V0][0] &= 0x1f;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 4);
        c.swc1(0, p(r(c, T3), 4));
        c.g[V0][0] = c.lhu(r(c, V1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
        c.g[V0][0] &= 0x1f;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 4);
        c.swc1(0, p(r(c, T3), 8));
        c.g[V0][0] = c.lhu(r(c, V1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 10);
        c.g[V0][0] &= 0x1f;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 4);
        c.swc1(0, p(r(c, T3), 0xc));
        c.g[V0][0] = c.lhu(r(c, V1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(2));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 15);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.swc1(0, r(c, T3));
        let more = (c.g[T4][0] as i64) >= 0;
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x10));
        if !more {
            c.cycles += 2;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_0428);
        }
    }
}

/// `PS2GraphicsMan_AddView` (0x00386688): copy view `idx`'s two matrices
/// from `+0x6af0` into the array at `+0x5930`, copy its pair word, and bump
/// the count.
pub fn ps2_graphics_man_add_view(c: &mut Cpu) {
    c.cycles += 21;
    let gm = abi::arg(c, 0);
    let idx = abi::arg(c, 1);
    c.g[V0][0] = sx((idx as u32) << 6);
    c.g[A2][0] = sx(gm.wrapping_add(GM_VIEW_MATRICES));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(gm));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(GM_VIEW_ENTRIES));
    let src = r(c, A2);
    let dst = r(c, V0);
    c.lqc2(1, src);
    c.lqc2(2, p(src, 0x10));
    c.lqc2(3, p(src, 0x20));
    c.lqc2(4, p(src, 0x30));
    c.sqc2(1, dst);
    c.sqc2(2, p(dst, 0x10));
    c.sqc2(3, p(dst, 0x20));
    c.sqc2(4, p(dst, 0x30));
    c.g[V1][0] = sx((idx as u32) << 3);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(gm));
    c.g[A1][0] = sx((idx as u32).wrapping_add(1));
    c.ldl(2, p(gm, GM_VIEW_PAIR_SRC + 7));
    c.ldr(2, p(gm, GM_VIEW_PAIR_SRC));
    c.sdl(2, p(r(c, V1), GM_VIEW_PAIRS + 7));
    c.sdr(2, p(r(c, V1), GM_VIEW_PAIRS));
    c.sw(p(gm, GM_VIEW_COUNT), c.g[A1][0]);
    abi::ret(c);
}

/// `void PS2GraphicsMan_v60(void* self, VEntry* item)` (0x37cc30): unlock the
/// texture's VRAM and notify the texture manager.
pub fn ps2_graphics_man_v60(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xffff_ffff_ffff_ffff;
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 4)).lw(c);
        if c.g[A1][0] != c.g[S2][0] {
            c.cycles += 9;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x18f4)).lw(c);
            call_ptr(c, 0x0036_7b60, 0x0037_cc60)?;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x10d8)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 4)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x190)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x194)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(p(c.g[S0][0] as u32, c.g[A0][0] as u32));
            call_ptr(c, target, 0x0037_cc78)?;
            Addr(p(c.g[S1][0] as u32, 4)).sw(c, c.g[S2][0]);
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        c.cycles += 6;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RailTube_dtor(RailTube* self, int flags)` (0x372660).
pub fn rail_tube_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3068));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
        let no_buf = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_buf {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x0037_2694)?;
        }
        c.cycles += 5;
        c.g[V0][0] = 0x0049_0000;
        c.g[V1][0] = c.g[S1][0] & 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7980));
        let keep = c.g[V1][0] == 0;
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0037_26b0)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `DynamicEmitter_Load(emitter, data)` 0x371318: let the data object's
/// loader fill a local parameter block, then copy the position, range and
/// count fields into the emitter.
pub fn dynamic_emitter_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        let data64 = abi::arg64(c, 1);
        let data = data64 as u32;
        c.g[A3][0] = data64;
        frame.save(c, S0, 0x30);
        c.g[A1][0] = sp(c) as u64;
        c.g[A2][0] = 0x14;
        frame.save_ra(c, 0x20);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        let src = lw_at(c, data, 0) as u32;
        c.g[V1][0] = src as u64;
        c.g[A0][0] = Addr(p(src, 0x10)).lh(c);
        c.g[V0][0] = lw_at(c, src, 0x14);
        let func = r32(c, V0);
        c.g[A0][0] = sx(data.wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x0037_1348)?;
        let base = sp(c);
        c.lwc1(0, base);
        c.lwc1(1, p(base, 4));
        c.g[V1][0] = lw_at(c, base, 0xc);
        c.g[A0][0] = lw_at(c, base, 8);
        c.g[V0][0] = lw_at(c, base, 0x10);
        c.swc1(0, this);
        Addr(p(this, 0x17c)).sw(c, c.g[V0][0]);
        c.swc1(1, p(this, 0x28));
        Addr(p(this, 0x170)).sw(c, c.g[V1][0]);
        Addr(p(this, 0x174)).sw(c, c.g[A0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// One of the three identical `vu0` matrix products: load the two matrices
/// from `a` and `b` into VF4..VF11, multiply rows into VF12..VF15 and store
/// them at `out`. The COP2 operations are issued in the original's order so
/// VF/ACC and the VU0 flags end exactly as they do there.
pub(crate) fn transform_matrix(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(4, a);
    c.lqc2(5, p(a, 0x10));
    c.lqc2(6, p(a, 0x20));
    c.lqc2(7, p(a, 0x30));
    c.lqc2(8, b);
    c.lqc2(9, p(b, 0x10));
    c.lqc2(10, p(b, 0x20));
    c.lqc2(11, p(b, 0x30));
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
    c.sqc2(12, out);
    c.sqc2(13, p(out, 0x10));
    c.sqc2(14, p(out, 0x20));
    c.sqc2(15, p(out, 0x30));
}

/// A 4x4 matrix moved through the quad registers (the original does
/// `lqc2`/`sqc2` twice instead of a plain `lq`/`sq` to keep the quad pipeline).
pub(crate) fn copy_matrix(c: &mut Cpu, from: u32, to: u32) {
    c.lqc2(1, from);
    c.lqc2(2, p(from, 0x10));
    c.lqc2(3, p(from, 0x20));
    c.lqc2(4, p(from, 0x30));
    c.sqc2(1, to);
    c.sqc2(2, p(to, 0x10));
    c.sqc2(3, p(to, 0x20));
    c.sqc2(4, p(to, 0x30));
}

/// `void BoaderShadow_ComputeProjection(void* obj, Vec3* a, Vec3* b,
/// Vec3* dir, Mat4* bones, int f)` (0x374d00).
///
/// `a0`/`t1` is the shadow object, `a1` a reference direction, `a2` the
/// position vector, `a3` the shadow direction, `t0` the bone matrix table.
/// The int argument is unused (the original immediately reuses `t1` for the
/// object pointer; the fixed count of six bone entries is hard-coded).
pub fn boader_shadow_compute_projection(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0037_4d00);

    // ---------------------------------------------------------------
    // Prologue and normalisation of the `b` vector (`a2`) into the frame
    // scratch vector at `sp+0`.
    // ---------------------------------------------------------------
    c.cycles += 30;
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x190);
    c.g[SP][0] = sx(sp);
    c.mtc1(0, 3); // f3 = 0.0
    c.sq(16, p(sp, 0x120));
    c.g[T1][0] = c.g[A0][0];
    c.sq(17, p(sp, 0x110));
    c.swc1(31, p(sp, 0x188));
    c.swc1(30, p(sp, 0x180));
    c.swc1(29, p(sp, 0x178));
    c.swc1(28, p(sp, 0x170));
    c.swc1(27, p(sp, 0x168));
    c.swc1(26, p(sp, 0x160));
    c.swc1(25, p(sp, 0x158));
    c.swc1(24, p(sp, 0x150));
    c.swc1(23, p(sp, 0x148));
    c.swc1(22, p(sp, 0x140));
    c.swc1(21, p(sp, 0x138));
    c.swc1(20, p(sp, 0x130));
    let obj = c.g[T1][0] as u32;

    c.lwc1(4, c.g[A2][0] as u32);
    c.lwc1(5, p(c.g[A2][0] as u32, 0x4));
    c.mul_s(0, 4, 4);
    c.lwc1(6, p(c.g[A2][0] as u32, 0x8));
    c.mul_s(1, 5, 5);
    c.mul_s(2, 6, 6);
    c.add_s(0, 0, 1);
    c.add_s(0, 0, 2);
    c.sqrt_s(1, 0);
    c.c_eq_s(1, 3);
    if c.fcc() {
        // Zero length: keep the raw x component, leave y/z from `a2`.
        c.swc1(4, sp);
        c.cycles += 2;
        c.swc1(5, p(sp, 0x4));
        c.swc1(6, p(sp, 0x8));
    } else {
        c.cycles += 10;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.div_s(0, 0, 1);
        c.mul_s(1, 6, 0);
        c.mul_s(2, 4, 0);
        c.mul_s(0, 5, 0);
        c.swc1(1, p(sp, 0x8));
        c.swc1(2, sp);
        c.swc1(0, p(sp, 0x4));
    }

    // ---------------------------------------------------------------
    // Copy that vector to `obj+0`, then normalise the direction `a3` into
    // the frame and set `v0 = obj+0x18` for the next pass.
    // ---------------------------------------------------------------
    c.cycles += 21;
    c.ldl(2, p(sp, 0x7));
    c.ldr(2, sp);
    c.g[V1][0] = c.lw(p(sp, 0x8));
    c.sdl(2, p(obj, 0x7));
    c.sdr(2, obj);
    c.sw(p(obj, 0x8), c.g[V1][0]);
    c.mtc1(0, 3);
    c.g[V0][0] = sx(p(obj, 0x18));
    c.lwc1(4, c.g[A3][0] as u32);
    c.lwc1(5, p(c.g[A3][0] as u32, 0x4));
    c.mul_s(0, 4, 4);
    c.lwc1(6, p(c.g[A3][0] as u32, 0x8));
    c.mul_s(1, 5, 5);
    c.mul_s(2, 6, 6);
    c.add_s(0, 0, 1);
    c.add_s(0, 0, 2);
    c.sqrt_s(1, 0);
    c.c_eq_s(1, 3);
    if c.fcc() {
        c.swc1(4, sp);
        c.cycles += 2;
        c.swc1(5, p(sp, 0x4));
        c.swc1(6, p(sp, 0x8));
    } else {
        c.cycles += 10;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.div_s(0, 0, 1);
        c.mul_s(1, 6, 0);
        c.mul_s(2, 4, 0);
        c.mul_s(0, 5, 0);
        c.swc1(1, p(sp, 0x8));
        c.swc1(2, sp);
        c.swc1(0, p(sp, 0x4));
    }

    // ---------------------------------------------------------------
    // Direction to `obj+0x18`, reference vector `a1` to `obj+0x0c` and to
    // the scratch vector (negated), and the dot product into `obj+0x24`.
    // `v0` becomes 1 when `b` is exactly parallel to `dir`.
    // ---------------------------------------------------------------
    c.cycles += 44;
    let dir = p(obj, shadow::DIR);
    c.ldl(7, p(sp, 0x7));
    c.ldr(7, sp);
    c.g[T3][0] = c.lw(p(sp, 0x8));
    c.sdl(7, p(dir, 0x7));
    c.sdr(7, dir);
    c.sw(p(dir, 0x8), c.g[T3][0]);
    c.mtc1(0, 2); // f2 = 0.0
    c.g[V0][0] = 0;
    c.lwc1(3, p(c.g[A1][0] as u32, 0x8));
    c.lwc1(0, c.g[A1][0] as u32);
    c.lwc1(1, p(c.g[A1][0] as u32, 0x4));
    c.swc1(3, p(sp, 0x8));
    c.swc1(0, sp);
    c.swc1(1, p(sp, 0x4));
    c.swc1(2, p(sp, 0x18));
    c.swc1(2, p(sp, 0x10));
    c.swc1(2, p(sp, 0x14));
    c.ldl(5, p(sp, 0x7));
    c.ldr(5, sp);
    c.g[A2][0] = c.lw(p(sp, 0x8));
    c.sdl(5, p(obj, 0x13));
    c.sdr(5, p(obj, 0x0c));
    c.sw(p(obj, 0x14), c.g[A2][0]);
    let vec_a = p(obj, shadow::VEC_A);
    c.lwc1(7, p(dir, 0x0));
    c.lwc1(8, p(dir, 0x4));
    c.neg_s(3, 7);
    c.lwc1(9, p(dir, 0x8));
    c.neg_s(5, 8);
    c.lwc1(0, p(vec_a, 0x0));
    c.lwc1(1, p(vec_a, 0x4));
    c.neg_s(6, 9);
    c.mul_s(0, 3, 0);
    c.lwc1(2, p(vec_a, 0x8));
    c.mul_s(1, 5, 1);
    c.lwc1(4, p(obj, shadow::VEC_B));
    c.mul_s(2, 6, 2);
    c.swc1(3, sp);
    c.c_eq_s(4, 7);
    c.swc1(5, p(sp, 0x4));
    c.add_s(0, 0, 1);
    c.swc1(6, p(sp, 0x8));
    c.add_s(0, 0, 2);
    let different = !c.fcc();
    c.swc1(0, p(obj, shadow::DOT));
    if !different {
        // b.x == dir.x: try the remaining components.
        c.cycles += 4;
        c.lwc1(0, p(obj, shadow::VEC_B + 0x4));
        c.c_eq_s(0, 8);
        if c.fcc() {
            c.cycles += 5;
            c.lwc1(0, p(obj, shadow::VEC_B + 0x8));
            c.c_eq_s(0, 9);
            if c.fcc() {
                c.g[V0][0] = 0x1;
            }
        }
    }

    // ---------------------------------------------------------------
    // If `b` is not parallel to `dir`, check it against the negated `dir`;
    // when `b` is parallel to neither axis, bias `obj+0` by a global. `f5`
    // carries `obj+0x1c` into the first cross product below.
    // ---------------------------------------------------------------
    c.cycles += 2;
    let mut bias_b = false;
    if c.g[V0][0] != 0 {
        c.lwc1(0, p(obj, shadow::VEC_B));
        bias_b = true;
    } else {
        c.cycles += 13;
        c.lwc1(0, p(dir, 0x0));
        c.g[V0][0] = 0;
        c.lwc1(2, p(dir, 0x4));
        c.lwc1(3, p(dir, 0x8));
        c.neg_s(0, 0);
        c.lwc1(1, p(obj, shadow::VEC_B));
        c.neg_s(2, 2);
        c.neg_s(3, 3);
        c.c_eq_s(1, 0);
        c.swc1(0, sp);
        c.swc1(2, p(sp, 0x4));
        let different = !c.fcc();
        c.swc1(3, p(sp, 0x8));
        if !different {
            c.cycles += 4;
            c.lwc1(0, p(obj, shadow::VEC_B + 0x4));
            c.c_eq_s(0, 2);
            if c.fcc() {
                c.cycles += 5;
                c.lwc1(0, p(obj, shadow::VEC_B + 0x8));
                c.c_eq_s(0, 3);
                if c.fcc() {
                    c.g[V0][0] = 0x1;
                }
            }
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lwc1(5, p(dir, 0x4));
        } else {
            c.cycles += 1;
            c.lwc1(0, p(obj, shadow::VEC_B));
            bias_b = true;
        }
    }
    if bias_b {
        c.cycles += 4;
        c.lwc1(1, p(c.g[GP][0] as u32, 0xffff_d808)); // gp - 0x27f8
        c.add_s(0, 0, 1);
        c.swc1(0, p(obj, shadow::VEC_B));
        c.lwc1(5, p(dir, 0x4));
    }

    // ---------------------------------------------------------------
    // First cross product (`dir` x `b`), normalised into the frame vector
    // at `sp+0`; `v0 = obj+0x34` receives it in the next block.
    // ---------------------------------------------------------------
    c.cycles += 29;
    c.g[V0][0] = sx(p(obj, shadow::MAT_C));
    c.lwc1(4, p(obj, 0x4));
    c.lwc1(1, p(obj, 0x20));
    c.lwc1(0, p(obj, 0x8));
    c.lwc1(6, p(dir, 0x0));
    c.mul_s(7, 4, 1);
    c.lwc1(2, p(obj, 0x0));
    c.mul_s(3, 0, 5);
    c.mul_s(0, 0, 6);
    c.mul_s(1, 2, 1);
    c.sub_s(7, 7, 3);
    c.mul_s(2, 2, 5);
    c.sub_s(8, 0, 1);
    c.mul_s(4, 4, 6);
    c.swc1(7, p(sp, 0x20));
    c.mul_s(0, 7, 7);
    c.mul_s(3, 8, 8);
    c.swc1(8, p(sp, 0x24));
    c.sub_s(2, 2, 4);
    c.mtc1(0, 5); // f5 = 0.0
    c.add_s(0, 0, 3);
    c.mul_s(1, 2, 2);
    c.swc1(2, p(sp, 0x28));
    c.add_s(0, 0, 1);
    c.sqrt_s(1, 0);
    c.c_eq_s(1, 5);
    if c.fcc() {
        c.swc1(7, sp);
        c.cycles += 2;
        c.swc1(8, p(sp, 0x4));
        c.swc1(2, p(sp, 0x8));
    } else {
        c.cycles += 10;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.div_s(0, 0, 1);
        c.mul_s(1, 2, 0);
        c.mul_s(2, 7, 0);
        c.mul_s(0, 8, 0);
        c.swc1(1, p(sp, 0x8));
        c.swc1(2, sp);
        c.swc1(0, p(sp, 0x4));
    }

    // ---------------------------------------------------------------
    // `obj+0x34` gets that vector; a second cross product (`dir` x the
    // `obj+0x1c..` plane row) lands in the frame at `sp+0x10..0x18` and is
    // normalised into `sp+0`, then written to `obj+0x28` below.
    // ---------------------------------------------------------------
    c.cycles += 36;
    let mat_c = p(obj, shadow::MAT_C);
    c.ldl(3, p(sp, 0x7));
    c.ldr(3, sp);
    c.g[A0][0] = c.lw(p(sp, 0x8));
    c.sdl(3, p(mat_c, 0x7));
    c.sdr(3, mat_c);
    c.sw(p(mat_c, 0x8), c.g[A0][0]);
    c.mtc1(0, 8); // f8 = 0.0
    c.g[V0][0] = sx(p(obj, shadow::MAT_B));
    c.lwc1(5, p(obj, 0x38));
    c.lwc1(4, p(obj, 0x1c));
    c.lwc1(1, p(obj, 0x3c));
    c.lwc1(0, p(obj, 0x20));
    c.lwc1(6, p(obj, 0x34));
    c.mul_s(7, 4, 1);
    c.lwc1(2, p(dir, 0x0));
    c.mul_s(3, 0, 5);
    c.mul_s(0, 0, 6);
    c.mul_s(1, 2, 1);
    c.mul_s(2, 2, 5);
    c.sub_s(7, 7, 3);
    c.sub_s(5, 0, 1);
    c.mul_s(4, 4, 6);
    c.mul_s(0, 7, 7);
    c.swc1(7, p(sp, 0x10));
    c.mul_s(3, 5, 5);
    c.swc1(5, p(sp, 0x14));
    c.sub_s(2, 2, 4);
    c.add_s(0, 0, 3);
    c.mul_s(1, 2, 2);
    c.swc1(2, p(sp, 0x18));
    c.add_s(0, 0, 1);
    c.sqrt_s(1, 0);
    c.c_eq_s(1, 8);
    if c.fcc() {
        c.swc1(7, sp);
        c.cycles += 2;
        c.swc1(5, p(sp, 0x4));
        c.swc1(2, p(sp, 0x8));
    } else {
        c.cycles += 10;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.div_s(0, 0, 1);
        c.mul_s(1, 2, 0);
        c.mul_s(2, 7, 0);
        c.mul_s(0, 5, 0);
        c.swc1(1, p(sp, 0x8));
        c.swc1(2, sp);
        c.swc1(0, p(sp, 0x4));
    }

    // ---------------------------------------------------------------
    // `obj+0x28` gets the last vector. Set up the basis (rows built from the
    // two cross products and the direction), the light matrix and the six
    // bone gathers.
    // ---------------------------------------------------------------
    c.cycles += 100;
    let mat_b = p(obj, shadow::MAT_B);
    c.ldl(7, p(sp, 0x7));
    c.ldr(7, sp);
    c.g[T3][0] = c.lw(p(sp, 0x8));
    c.sdl(7, p(mat_b, 0x7));
    c.sdr(7, mat_b);
    c.g[T5][0] = sx(p(sp, 0x50));
    c.sw(p(mat_b, 0x8), c.g[T3][0]);
    c.g[T4][0] = sx(p(obj, shadow::LIGHT));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 18); // f18 = 1.0
    c.g[V1][0] = 0x004a_0000;
    c.lwc1(17, p(obj, shadow::VEC_A));
    c.g[A0][0] = 0x004a_0000;
    c.lwc1(15, p(obj, 0x10));
    c.mov_s(31, 18);
    c.lwc1(5, p(obj, 0x34));
    c.g[T3][0] = sx(p(sp, 0xd0));
    c.lwc1(7, p(obj, 0x38));
    c.g[T6][0] = sx(p(sp, 0x90));
    c.lwc1(8, p(obj, 0x28));
    c.mul_s(3, 17, 5);
    c.lwc1(9, p(obj, 0x2c));
    c.mul_s(12, 15, 7);
    c.mul_s(2, 17, 8);
    c.lwc1(13, p(obj, 0x14));
    c.mul_s(14, 15, 9);
    c.lwc1(0, p(obj, 0x3c));
    c.lwc1(1, p(obj, 0x30));
    c.add_s(3, 3, 12);
    c.swc1(5, p(obj, 0x50));
    c.mov_s(28, 5);
    c.lwc1(10, p(dir, 0x0));
    c.mul_s(12, 13, 1);
    c.lwc1(11, p(dir, 0x4));
    c.mul_s(5, 13, 0);
    c.add_s(2, 2, 14);
    c.lwc1(4, p(obj, 0x20));
    c.mul_s(16, 15, 11);
    c.swc1(0, p(obj, 0x58));
    c.mul_s(6, 17, 10);
    c.mov_s(22, 0);
    c.add_s(2, 2, 12);
    c.swc1(1, p(obj, 0x68));
    c.add_s(3, 3, 5);
    c.swc1(10, p(obj, 0x70));
    c.swc1(11, p(obj, 0x74));
    c.mul_s(0, 13, 4);
    c.swc1(7, p(obj, 0x54));
    c.add_s(6, 6, 16);
    c.swc1(8, p(obj, 0x60));
    c.mov_s(21, 1);
    c.swc1(9, p(obj, 0x64));
    c.mov_s(24, 10);
    c.swc1(2, p(sp, 0x14));
    c.add_s(6, 6, 0);
    c.swc1(3, p(sp, 0x10));
    c.mov_s(23, 11);
    c.swc1(18, p(obj, 0x8c));
    c.mov_s(27, 7);
    c.swc1(4, p(obj, 0x78));
    c.mov_s(26, 8);
    c.sw(p(obj, 0x80), 0u64);
    c.mov_s(25, 9);
    c.sw(p(obj, 0x84), 0u64);
    c.mov_s(20, 4);
    c.sw(p(obj, 0x88), 0u64);
    c.g[A2][0] = sx(p(obj, shadow::PROJECTION));
    c.swc1(17, p(sp, 0x30));
    c.g[T8][0] = sx(p(obj, shadow::VIEW));
    c.swc1(15, p(sp, 0x34));
    c.g[T2][0] = sx(p(obj, shadow::BONES));
    c.swc1(13, p(sp, 0x38));
    c.g[A1][0] = sx(p(obj, shadow::BONE_INDEX));
    c.swc1(18, p(sp, 0x3c));
    c.g[A3][0] = 0x5;
    c.swc1(6, p(sp, 0x18));
    c.lwc1(16, p(obj, 0x88));
    c.swc1(18, p(sp, 0x1c));
    c.lwc1(11, p(c.g[V1][0] as u32, 0x431c));
    c.mov_s(30, 16);
    c.lq(2, p(sp, 0x10));
    c.mov_s(29, 16);
    c.lwc1(10, p(c.g[A0][0] as u32, 0x4320));
    c.mov_s(14, 11);
    c.sq(2, sp);
    c.mov_s(12, 11);
    c.mov_s(13, 10);
    c.mov_s(15, 10);
    c.lwc1(0, p(sp, 0x0));
    c.lwc1(1, p(sp, 0x4));
    c.lwc1(2, p(sp, 0x8));
    c.neg_s(0, 0);
    c.neg_s(1, 1);
    c.neg_s(2, 2);
    c.swc1(0, p(obj, 0x5c));
    c.mov_s(19, 0);
    c.swc1(1, p(obj, 0x6c));
    c.mov_s(18, 1);
    c.swc1(2, p(obj, 0x7c));
    c.mov_s(17, 2);

    // Six bone entries: index from `obj+0x17c..`, entry stride 0x40, row
    // taken from `+0x30` and transformed by the current basis.
    'gather: loop {
        c.cycles += 48;
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.lq(2, p(c.g[V1][0] as u32, 0x30));
        c.sq(2, p(sp, 0x40));
        c.lwc1(1, p(sp, 0x40));
        c.lwc1(0, p(sp, 0x44));
        c.mul_s(6, 1, 30);
        c.lwc1(5, p(sp, 0x48));
        c.mul_s(8, 0, 27);
        c.lwc1(7, p(sp, 0x4c));
        c.mul_s(2, 0, 25);
        c.mul_s(9, 0, 16);
        c.mul_s(4, 1, 28);
        c.mul_s(3, 1, 26);
        c.mul_s(0, 0, 23);
        c.mul_s(1, 1, 24);
        c.add_s(4, 4, 8);
        c.add_s(3, 3, 2);
        c.add_s(1, 1, 0);
        c.mul_s(0, 5, 22);
        c.mul_s(2, 5, 21);
        c.mul_s(8, 5, 29);
        c.mul_s(5, 5, 20);
        c.add_s(6, 6, 9);
        c.add_s(4, 4, 0);
        c.add_s(3, 3, 2);
        c.add_s(1, 1, 5);
        c.mul_s(0, 7, 19);
        c.mul_s(5, 7, 31);
        c.mul_s(2, 7, 18);
        c.add_s(6, 6, 8);
        c.mul_s(7, 7, 17);
        c.add_s(4, 4, 0);
        c.add_s(3, 3, 2);
        c.add_s(1, 1, 7);
        c.add_s(6, 6, 5);
        c.swc1(4, p(sp, 0x20));
        c.swc1(3, p(sp, 0x24));
        c.swc1(1, p(sp, 0x28));
        c.swc1(6, p(sp, 0x2c));
        c.lq(2, p(sp, 0x20));
        c.sq(2, p(sp, 0x10));
        c.lwc1(0, p(sp, 0x10));
        c.c_lt_s(0, 14);
        if c.fcc() {
            c.mov_s(14, 0);
        }
        c.cycles += 4;
        c.c_lt_s(13, 0);
        if c.fcc() {
            c.mov_s(13, 0);
        }
        c.cycles += 5;
        c.lwc1(0, p(sp, 0x14));
        c.c_lt_s(0, 11);
        if c.fcc() {
            c.mov_s(11, 0);
        }
        c.cycles += 4;
        c.c_lt_s(10, 0);
        if c.fcc() {
            c.mov_s(10, 0);
        }
        c.cycles += 5;
        c.lwc1(0, p(sp, 0x18));
        c.c_lt_s(0, 12);
        if c.fcc() {
            c.mov_s(12, 0);
        }
        c.cycles += 4;
        c.c_lt_s(15, 0);
        if c.fcc() {
            c.mov_s(15, 0);
        }
        c.cycles += 3;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
        let more = (c.g[A3][0] as i64) >= 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0037_5270);
            }
            continue 'gather;
        }
        break;
    }

    // ---------------------------------------------------------------
    // Epilogue: the three projection passes and the depth/scale results.
    // ---------------------------------------------------------------
    c.cycles += 318;
    c.g[AT][0] = 0x428c_0000;
    c.mtc1(1, 0); // f0 = 70.0
    c.g[V0][0] = c.g[T6][0];
    c.lwc1(5, p(c.g[GP][0] as u32, 0xffff_d80c)); // gp - 0x27f4
    c.add_s(2, 10, 0);
    c.lwc1(4, p(c.g[GP][0] as u32, 0xffff_d810)); // gp - 0x27f0
    c.sub_s(3, 14, 0);
    c.swc1(5, p(sp, 0x78));
    c.add_s(1, 13, 0);
    c.swc1(4, p(sp, 0x7c));
    c.sub_s(0, 11, 0);
    c.swc1(2, p(obj, 0x48));
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(1, 6); // f6 = 2.0
    c.add_s(5, 1, 3);
    c.swc1(1, p(obj, 0x44));
    c.add_s(4, 2, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 12); // f12 = 1.0
    c.sub_s(1, 1, 3);
    c.swc1(3, p(obj, 0x40));
    c.sub_s(2, 2, 0);
    c.swc1(0, p(obj, 0x4c));
    c.neg_s(5, 5);
    c.sw(p(sp, 0x54), 0u64);
    c.neg_s(4, 4);
    c.sw(p(sp, 0x58), 0u64);
    c.sw(p(sp, 0x60), 0u64);
    c.div_s(5, 5, 1);
    c.sw(p(sp, 0x68), 0u64);
    c.div_s(4, 4, 2);
    c.sw(p(sp, 0x70), 0u64);
    c.div_s(1, 6, 1);
    c.sw(p(sp, 0x74), 0u64);
    c.div_s(2, 6, 2);
    c.sw(p(sp, 0x80), 0u64);
    c.sw(p(sp, 0x84), 0u64);
    c.sw(p(sp, 0x88), 0u64);
    c.swc1(5, p(sp, 0x5c));
    c.swc1(4, p(sp, 0x6c));
    c.swc1(1, p(sp, 0x50));
    c.swc1(2, p(sp, 0x64));
    c.swc1(12, p(sp, 0x8c));
    let (t4, t5, t3) = (c.g[T4][0] as u32, c.g[T5][0] as u32, c.g[T3][0] as u32);
    let v0 = c.g[V0][0] as u32;
    let a2 = c.g[A2][0] as u32;
    transform_matrix(c, t4, t5, t3);
    copy_matrix(c, t3, v0);
    copy_matrix(c, v0, a2);

    // Second pass: light direction rows.
    c.lwc1(3, p(obj, 0x4c));
    c.lwc1(0, p(obj, 0x48));
    c.lwc1(2, p(obj, 0x40));
    c.lwc1(1, p(obj, 0x44));
    c.sub_s(0, 0, 3);
    c.neg_s(5, 2);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 4); // f4 = -1.0
    c.sub_s(1, 1, 2);
    c.sw(p(sp, 0x54), 0u64);
    c.div_s(3, 3, 0);
    c.sw(p(sp, 0x58), 0u64);
    c.div_s(4, 4, 0);
    c.sw(p(sp, 0x60), 0u64);
    c.div_s(5, 5, 1);
    c.sw(p(sp, 0x68), 0u64);
    c.div_s(1, 12, 1);
    c.sw(p(sp, 0x70), 0u64);
    c.sw(p(sp, 0x74), 0u64);
    c.sw(p(sp, 0x78), 0u64);
    c.add_s(3, 3, 12);
    c.swc1(12, p(sp, 0x7c));
    c.swc1(4, p(sp, 0x64));
    c.swc1(5, p(sp, 0x5c));
    c.swc1(1, p(sp, 0x50));
    c.swc1(3, p(sp, 0x6c));
    c.sw(p(sp, 0x80), 0u64);
    c.sw(p(sp, 0x84), 0u64);
    c.sw(p(sp, 0x88), 0u64);
    c.swc1(12, p(sp, 0x8c));
    let v0 = c.g[V0][0] as u32;
    let t8 = c.g[T8][0] as u32;
    transform_matrix(c, t4, t5, t3);
    copy_matrix(c, t3, v0);
    copy_matrix(c, v0, t8);

    // Third pass: view matrix.
    c.lwc1(1, p(obj, 0x40));
    c.lwc1(0, p(obj, 0x44));
    c.lwc1(2, p(obj, 0x48));
    c.lwc1(4, p(obj, 0x4c));
    c.add_s(3, 0, 1);
    c.sub_s(0, 0, 1);
    c.g[AT][0] = 0xffff_ffff_c000_0000;
    c.mtc1(1, 5); // f5 = -2.0
    c.add_s(1, 2, 4);
    c.swc1(12, p(sp, 0x8c));
    c.sub_s(2, 2, 4);
    c.sw(p(sp, 0x54), 0u64);
    c.neg_s(3, 3);
    c.sw(p(sp, 0x58), 0u64);
    c.div_s(6, 6, 0);
    c.sw(p(sp, 0x60), 0u64);
    c.div_s(1, 1, 2);
    c.sw(p(sp, 0x68), 0u64);
    c.div_s(3, 3, 0);
    c.sw(p(sp, 0x70), 0u64);
    c.div_s(5, 5, 2);
    c.sw(p(sp, 0x74), 0u64);
    c.sw(p(sp, 0x78), 0u64);
    c.swc1(12, p(sp, 0x7c));
    c.swc1(6, p(sp, 0x50));
    c.swc1(1, p(sp, 0x6c));
    c.swc1(3, p(sp, 0x5c));
    c.swc1(5, p(sp, 0x64));
    c.sw(p(sp, 0x80), 0u64);
    c.sw(p(sp, 0x84), 0u64);
    c.sw(p(sp, 0x88), 0u64);
    let t2 = c.g[T2][0] as u32;
    transform_matrix(c, t4, t5, t3);
    copy_matrix(c, t3, v0);
    copy_matrix(c, v0, t2);

    // Depth scale and the six gathered bone rows.
    c.lwc1(1, p(obj, shadow::SCALE_IN));
    c.cvt_s_w(1, 1);
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(1, 2); // f2 = 0.5
    c.lwc1(0, p(obj, shadow::BIAS_IN));
    c.cvt_s_w(0, 0);
    c.g[AT][0] = 0x4500_0000;
    c.mtc1(1, 3); // f3 = 2048.0
    c.sw(p(obj, 0x16c), 0u64);
    c.mul_s(1, 1, 2);
    c.swc1(3, p(obj, 0x164));
    c.mul_s(0, 0, 2);
    c.sw(p(obj, shadow::DEPTH), 0u64);
    c.sw(p(obj, shadow::DEPTH + 0x4), 0u64);
    c.swc1(1, p(obj, shadow::SCALE));
    c.swc1(0, p(obj, shadow::SCALE + 0x4));
    c.swc1(3, p(obj, 0x160));
    c.sw(p(obj, 0x168), 0u64);

    // Transpose each of the three matrices in place (MMI halfword shuffles).
    let proj = c.g[A2][0] as u32;
    c.lq(9, proj);
    c.lq(8, p(proj, 0x10));
    c.lq(7, p(proj, 0x20));
    c.lq(11, p(proj, 0x30));
    c.pextlw(5, 8, 9);
    c.pextuw(4, 8, 9);
    c.pextlw(3, 11, 7);
    c.pextuw(2, 11, 7);
    c.pcpyld(9, 3, 5);
    c.pcpyud(8, 5, 3);
    c.pcpyld(7, 2, 4);
    c.pcpyud(11, 4, 2);
    c.sq(9, proj);
    c.sq(8, p(proj, 0x10));
    c.sq(7, p(proj, 0x20));
    c.sq(11, p(proj, 0x30));

    let bones = c.g[T2][0] as u32;
    c.lq(17, bones);
    c.lq(16, p(bones, 0x10));
    c.lq(15, p(bones, 0x20));
    c.lq(14, p(bones, 0x30));
    c.pextlw(13, 16, 17);
    c.pextuw(12, 16, 17);
    c.pextlw(11, 14, 15);
    c.pextuw(2, 14, 15);
    c.pcpyld(17, 11, 13);
    c.pcpyud(16, 13, 11);
    c.pcpyld(15, 2, 12);
    c.pcpyud(14, 12, 2);
    c.sq(17, bones);
    c.sq(16, p(bones, 0x10));
    c.sq(15, p(bones, 0x20));
    c.sq(14, p(bones, 0x30));

    let view = c.g[T8][0] as u32;
    c.lq(9, view);
    c.lq(8, p(view, 0x10));
    c.lq(7, p(view, 0x20));
    c.lq(6, p(view, 0x30));
    c.pextlw(5, 8, 9);
    c.pextuw(4, 8, 9);
    c.pextlw(3, 6, 7);
    c.pextuw(2, 6, 7);
    c.pcpyld(9, 3, 5);
    c.pcpyud(8, 5, 3);
    c.pcpyld(7, 2, 4);
    c.pcpyud(6, 4, 2);
    c.sq(9, view);
    c.sq(8, p(view, 0x10));
    c.sq(7, p(view, 0x20));
    c.sq(6, p(view, 0x30));

    // Restore the callee-saved registers and the frame.
    c.lq(16, p(sp, 0x120));
    c.lq(17, p(sp, 0x110));
    c.lwc1(31, p(sp, 0x188));
    c.lwc1(30, p(sp, 0x180));
    c.lwc1(29, p(sp, 0x178));
    c.lwc1(28, p(sp, 0x170));
    c.lwc1(27, p(sp, 0x168));
    c.lwc1(26, p(sp, 0x160));
    c.lwc1(25, p(sp, 0x158));
    c.lwc1(24, p(sp, 0x150));
    c.lwc1(23, p(sp, 0x148));
    c.lwc1(22, p(sp, 0x140));
    c.lwc1(21, p(sp, 0x138));
    c.lwc1(20, p(sp, 0x130));
    c.g[SP][0] = sx(p(sp, 0x190));
    abi::ret(c);
}

/// `void Irradiance_Scale(Vec4* out, const Vec4* in, float scale)` (0x3897a0):
/// scale each of ten vectors by `scale`, first into a stack buffer (through
/// VU0, as the original does), then copy the buffer to `out`.
pub fn irradiance_scale(c: &mut Cpu) {
    c.cycles += 4;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
    c.g[A2][0] = 9;
    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
    c.g[V1][0] = c.g[SP][0];
    loop {
        c.cycles += 12;
        c.mfc1(2, 12);
        c.lqc2(4, c.g[A1][0] as u32);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, p_4(c.g[SP][0] as u32, 0xb0));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        lqsp(c, 2, 0xb0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        sqsp(c, 2, 0xa0);
        c.sq(2, c.g[V1][0] as u32);
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_97b0);
        }
    }
    c.cycles += 2;
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = c.g[SP][0];
    loop {
        c.cycles += 7;
        c.lq(5, c.g[V0][0] as u32);
        c.lq(6, p_4(c.g[V0][0] as u32, 0x10));
        c.sq(5, c.g[V1][0] as u32);
        c.sq(6, p_4(c.g[V1][0] as u32, 0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
        let more = c.g[V0][0] != c.g[A3][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_97e8);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
    c.pc = ra;
}

/// `void PS2GraphicsMan_FreeAllTextures(PS2GraphicsMan* m)` (0x37c830): free
/// the first 2000 non-null texture slots through virtual slot 50.
pub fn ps2_graphics_man_free_all_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 18, 0x10);
        sqsp(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        sqsp(c, 17, 0x20);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18f8));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        c.g[A1][0] = 0;
        loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            let empty = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !empty {
                c.cycles += 5;
                c.g[V0][0] = Addr(p_4(r32(c, S2), 0x10d8)).lw(c);
                c.g[A0][0] = Addr(p_4(c.g[V0][0] as u32, 0x190)).lh(c);
                c.g[V1][0] = Addr(p_4(c.g[V0][0] as u32, 0x194)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = abi::addu(c.g[S2][0], c.g[A0][0]);
                call::call_ptr(c, target, 0x0037_c87c)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 2000) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0037_c858);
            }
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `void BoaderShadow_dtor(self, flags)` (0x375918): release the texture held
/// at `+0x170` (unlock the vram, then free the surface through its vtable)
/// and free the object when `flags & 1`.
pub fn boader_shadow_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = lw_r(c, S0, 0x170);
        let none = (c.g[A1][0] as i64) < 0;
        c.g[V0][0] = lw_r(c, GP, 0x2a90);
        if !none {
            c.cycles += 11;
            c.g[A0][0] = lw_r(c, V0, 0x18f4);
            callee::TEX_MAN_UNLOCK_VRAM.call(c, 0x0037_5944)?;
            c.g[A2][0] = lw_r(c, GP, 0x2a90);
            c.g[A1][0] = lw_r(c, S0, 0x170);
            c.g[V0][0] = lw_r(c, A2, 0x10d8);
            c.g[A0][0] = lh_r(c, V0, 0x190);
            c.g[V1][0] = lw_r(c, V0, 0x194);
            let target = low(c, V1);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0037_5960)?;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            sw_r(c, S0, 0x170, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] == 0 {
            c.lq(16, sp_2(c).wrapping_add(0x20));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::BUILTIN_DELETE_5.addr, 0x0037_597c)?;
            c.lq(16, sp_2(c).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void* PathArrow_ctor(self)` (0x00372aa0): set the arrow's vtable and
/// initialise the global 8-entry arrow table once.
pub fn path_arrow_ctor(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = sx(0x490000);
    c.g[V1][0] = Addr(gp(c, 0x114c)).lw(c);
    c.g[V0][0] = sx(0x490000u32.wrapping_add(0x3038));
    let done = c.g[V1][0] != 0;
    Addr(at(c, A0, 0)).sw(c, c.g[V0][0]);
    if done {
        c.cycles += 2;
        let ra = word(c, RA);
        c.g[V0][0] = c.g[A0][0];
        c.pc = ra;
        return;
    }
    c.cycles += 11;
    c.g[V0][0] = sx(0x500000);
    c.g[T0][0] = Addr(gp(c, 0x1124)).lw(c);
    c.g[V1][0] = sx(0x500000u32.wrapping_add(0x4560));
    c.g[A3][0] = Addr(gp(c, 0x1118)).lw(c);
    c.g[A2][0] = Addr(gp(c, 0x111c)).lw(c);
    c.g[T1][0] = sx(word(c, V1).wrapping_add(0x180));
    c.g[A1][0] = Addr(gp(c, 0x1120)).lw(c);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    Addr(at(c, V1, 0x1c)).sw(c, c.g[T0][0]);
    loop {
        c.cycles += 11;
        Addr(at(c, V1, 0x10)).sw(c, c.g[A3][0]);
        Addr(at(c, V1, 0x14)).sw(c, c.g[A2][0]);
        Addr(at(c, V1, 0x18)).sw(c, c.g[A1][0]);
        c.swc1(0, at(c, V1, 0x8));
        Addr(at(c, V1, 0xc)).sw(c, 0);
        Addr(at(c, V1, 0)).sw(c, 0);
        Addr(at(c, V1, 0x4)).sw(c, 0);
        c.g[V1][0] = sx(word(c, V1).wrapping_add(0x30));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            Addr(at(c, V1, 0x1c)).sw(c, c.g[T0][0]);
            if event_due(c) {
                deliver_event_here(c, 0x0037_2ae0);
            }
            continue;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        Addr(gp(c, 0x114c)).sw(c, c.g[V0][0]);
        break;
    }
    c.cycles += 2;
    let ra = word(c, RA);
    c.g[V0][0] = c.g[A0][0];
    c.pc = ra;
}

/// `void PS2GraphicsMan_SetClearColor(PS2GraphicsMan* g, float* rgb)`
/// (0x00389098): scale each channel by 255, truncate to an integer and pack
/// them into the clear colour word (`+0x6ae0`).
pub fn ps2_graphics_man_set_clear_color(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.swc1(20, sp(c).wrapping_add(0x40));
        c.g[AT][0] = 0x437f_0000;
        c.mtc1(1, 20);
        c.lwc1(12, c.g[S1][0] as u32);
        c.mul_s(12, 12, 20);
        call_ptr(c, 0x0041_39f8, 0x0038_90cc)?; // __fixunssfsi
        c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(4));
        c.g[S0][0] = c.g[V0][0];
        c.mul_s(12, 12, 20);
        call_ptr(c, 0x0041_39f8, 0x0038_90dc)?; // __fixunssfsi
        c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(8));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
        c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
        c.mul_s(12, 12, 20);
        call_ptr(c, 0x0041_39f8, 0x0038_90f0)?; // __fixunssfsi
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
        c.g[RA][0] = c.ld(sp(c));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x6ae0), c.g[S0][0]);
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.lwc1(20, sp(c).wrapping_add(0x40));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        abi::ret(c);
        Ok(())
    })
}

/// `void Irradiance_BuildSH(Irradiance* irr, Light* sun)` (0x00389c38): clear
/// the coefficients, add the sky's ambient entry scaled by 1.0, then add the
/// sun's camera-relative light when there is one.
pub fn irradiance_build_sh(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0038_9260, 0x0038_9c54)?; // Irradiance_Clear
        c.g[V0][0] = sx(0x0050_0000);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x47f8));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(4));
        if (c.g[V1][0] as i64) > 0 {
            c.cycles += 9;
            c.g[A1][0] = c.lw(gpa(c, 0x12d4));
            call_ptr(c, 0x0038_abf8, 0x0038_9c70)?; // Irradiance_GetEntry
            c.g[A2][0] = sx(0x0050_4810);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0038_9590, 0x0038_9c8c)?; // Irradiance_AddScaled
        }
        c.cycles += 2;
        let no_sun = c.g[S1][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_sun {
            c.cycles += 4;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0038_9cb8, 0x0038_9ca4)?; // Irradiance_AddCameraRelativeLight
        }
        c.cycles += 5;
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Emitters and graphics.

/// `void mEmitter_ResetParticles(mEmitter* self)` (0x371600): reset the base
/// emitter, then clear the packed colour of every particle (the scratch
/// colour word at_2 `gp+0x28a8` is copied unaligned into each particle slot).
pub fn memitter_reset_particles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x20);
        let sp0 = spv(c);
        c.sq(16, at_2(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        callee::DYNAMIC_EMITTER_RESET.call(c, 0x0037_1614)?;
        let this = c.g[S0][0];
        c.g[V0][0] = lw_at_2(c, this, 0x178);
        let count = c.g[V0][0] as i64;
        c.g[A0][0] = 0;
        if count > 0 {
            c.cycles += 2;
            c.g[A1][0] = 1;
            let a = gpa_2(c, 0x28ac);
            c.g[V0][0] = lw_at_2(c, a as u64, 0);
            loop {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = lw_at_2(c, this, 0x204);
                } else {
                    c.cycles += 6;
                    let g = gpa_2(c, 0x28a8);
                    c.sb(g, 0);
                    c.sb(at_2(g, 1), 0);
                    c.sb(at_2(g, 2), 0);
                    c.sb(at_2(g, 3), 0);
                    let ga = gpa_2(c, 0x28ac);
                    c.sw(ga, c.g[A1][0]);
                    c.g[V0][0] = lw_at_2(c, this, 0x204);
                }
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[A0][0] = sx(at_2(c.g[A0][0] as u32, 1));
                c.g[V1][0] = sx(at_2(c.g[V1][0] as u32, c.g[V0][0] as u32));
                let t0 = gpa_2(c, 0x28a8);
                c.g[T0][0] = sx(t0);
                c.lwl(2, at_2(t0, 3));
                c.lwr(2, t0);
                let dst = c.g[V1][0] as u32;
                c.swl(2, at_2(dst, 3));
                c.swr(2, dst);
                c.g[V0][0] = lw_at_2(c, this, 0x178);
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                let ga = gpa_2(c, 0x28ac);
                c.g[V0][0] = lw_at_2(c, ga as u64, 0);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0037_1628);
                }
            }
        }
        c.cycles += 4;
        c.lq(16, at_2(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `void PS2GraphicsMan_EmitModelLighting(void* dst, void* world, u32 model,
/// void** out_dl)` (0x37e098): write a lighting packet (fixed 0x60-byte
/// header, the model's three light quadwords) and append one quadword per
/// light to the display list at_2 `*out_dl`.
pub fn ps2_graphics_man_emit_model_lighting(c: &mut Cpu) {
    c.cycles += 24;
    c.g[T0][0] = lw_at_2(c, c.g[A3][0], 0);
    c.g[V0][0] = 0x1000_000f;
    c.g[V1][0] = 0x6c03_8009;
    c.g[V1][0] = c.g[V1][0] << 32;
    c.g[A1][0] = 0x6c0a_800f;
    c.g[A1][0] = c.g[A1][0] << 32;
    let dl = c.g[T0][0] as u32;
    c.sd(dl, c.g[V0][0]);
    c.g[T1][0] = 9;
    c.sd(at_2(dl, 0x18), c.g[V1][0]);
    c.sd(at_2(dl, 8), 0);
    c.sd(at_2(dl, 0x10), 0);
    let model = c.g[A2][0] as u32;
    c.lq(2, model);
    c.sq(2, at_2(dl, 0x20));
    c.lq(3, at_2(model, 0x10));
    c.sq(3, at_2(dl, 0x30));
    c.lq(2, at_2(model, 0x20));
    c.sd(at_2(dl, 0x58), c.g[A1][0]);
    c.sq(2, at_2(dl, 0x40));
    c.sd(at_2(dl, 0x50), 0);
    c.g[T0][0] = sx(at_2(dl, 0x60));
    c.g[V1][0] = c.g[T0][0];
    loop {
        c.cycles += 7;
        let src = c.g[A0][0] as u32;
        c.lq(2, src);
        c.g[T1][0] = sx(at_2(c.g[T1][0] as u32, 0xffff_ffff));
        c.g[A0][0] = sx(at_2(src, 0x10));
        c.sq(2, c.g[V1][0] as u32);
        let more = (c.g[T1][0] as i64) >= 0;
        c.g[V1][0] = sx(at_2(c.g[V1][0] as u32, 0x10));
        if !more {
            c.cycles += 3;
            c.g[T0][0] = sx(at_2(c.g[T0][0] as u32, 0xa0));
            let ra = c.g[RA][0] as u32;
            c.sw(c.g[A3][0] as u32, c.g[T0][0]);
            c.pc = ra;
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0037_e0f8);
        }
    }
}

// ---------------------------------------------------------------------------
// PS2 graphics manager.

/// `void PS2GraphicsMan_Shutdown(PS2GraphicsMan* self)` (0x376268): tear down
/// the GS/DMA channels, the texture manager and the interrupt table.
pub fn ps2_graphics_man_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 2;
        call_ptr(c, 0x0042_4880, 0x0037_6280)?;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x5ac0)).lw(c);
        c.g[A0][0] = 2;
        call_ptr(c, 0x0042_3aa0, 0x0037_628c)?;
        c.g[A0][0] = 1;
        call_ptr(c, 0x0042_4950, 0x0037_6294)?;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x5ac4)).lw(c);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0042_3ad0, 0x0037_62a0)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x5ad0)).lw(c);
        call_ptr(c, 0x0042_3bf0, 0x0037_62a8)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x5ad0)).lw(c);
        call_ptr(c, 0x0042_3bb0, 0x0037_62b0)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x5ac8)).lw(c);
        call_ptr(c, 0x0042_3db0, 0x0037_62b8)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x5acc)).lw(c);
        call_ptr(c, 0x0042_3db0, 0x0037_62c0)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x18f0)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0037_62c8)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x18f4)).lw(c);
        call_ptr(c, 0x0036_7360, 0x0037_62d0)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x18f4)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0037_62d8)?;
        c.g[A0][0] = 0x0050_0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x59d8));
        call_ptr(c, 0x0036_1f40, 0x0037_62e4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Line strips.

/// `void LineStrip_Begin(LineStrip* ctx, void* v, void* pos)` (0x3850a8):
/// close the current strip, then start a new batch of vertices at `ctx->p`.
pub fn line_strip_begin(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[S2][0] = c.g[A2][0];
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.g[V1][0] = sx(r32(c, V0).wrapping_add(0x20));
            Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
            Addr(p(r32(c, S0), 4)).sw(c, c.g[V1][0]);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
            if c.g[V0][0] == 0 {
                c.g[V1][0] = Addr(p(r32(c, S0), 4)).lw(c);
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                callee::LINE_STRIP_CLOSE.call(c, 0x0038_50f8)?;
                c.cycles += 1;
                c.g[V1][0] = Addr(p(r32(c, S0), 4)).lw(c);
            }
        }
        c.cycles += 14;
        c.g[A0][0] = c.g[S0][0];
        Addr(p(r32(c, S0), 0x10)).sw(c, 0);
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x30));
        c.g[A2][0] = c.g[S2][0];
        Addr(p(r32(c, S0), 4)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V1][0]);
        callee::LINE_STRIP_ADD_VERTEX.call(c, 0x0038_511c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* RailTube_ctor(RailTube* self, void* owner)` (0x372520): set the
/// vtable and owner; the first time, fill the 262-entry rail-vertex ring
/// with the default rail colours.
pub fn rail_tube_ctor(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x0049_0000;
    c.g[V1][0] = c.lw(gp(c, 0x113c));
    c.g[V0][0] = sx(p(r32(c, V0), 0x3068));
    c.sw(at(c, A0, 4), c.g[A1][0]);
    c.sw(r32(c, A0), c.g[V0][0]);
    let ready = c.g[V1][0] != 0;
    c.sw(at(c, A0, 8), 0);
    if !ready {
        c.cycles += 9;
        c.g[V0][0] = 0x0050_0000;
        c.g[T0][0] = c.lw(gp(c, 0x10f0));
        c.g[A3][0] = c.lw(gp(c, 0x10f4));
        c.g[V1][0] = sx(p(r32(c, V0), 0x1440));
        c.g[A2][0] = c.lw(gp(c, 0x10f8));
        c.g[A1][0] = 0;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[T1][0] = 0xff;
        loop {
            c.cycles += 9;
            c.sw(at(c, V1, 0x1c), c.g[T1][0]);
            c.g[V0][0] = c.g[A1][0] & 1;
            c.sw(at(c, V1, 0x10), c.g[T0][0]);
            c.sw(at(c, V1, 0x14), c.g[A3][0]);
            c.sw(at(c, V1, 0x18), c.g[A2][0]);
            c.swc1(0, at(c, V1, 8));
            c.sw(at(c, V1, 0xc), 0);
            let even = c.g[V0][0] == 0;
            c.sw(r32(c, V1), 0);
            if even {
                c.cycles += 1;
                c.sw(at(c, V1, 4), 0);
            } else {
                c.cycles += 2;
                c.swc1(0, at(c, V1, 4));
            }
            c.cycles += 4;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            c.g[V0][0] = ((c.g[A1][0] as i64) < 262) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(at(c, V1, 0x30));
            if !more {
                c.cycles += 2;
                c.g[V0][0] = 1;
                c.sw(gp(c, 0x113c), c.g[V0][0]);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0037_2560);
            }
        }
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.g[V0][0] = c.g[A0][0];
    c.pc = ra;
}

/// `void Irradiance_AddScaled(Vec4* a, const Vec4* b, const Vec4* c, float s)`
/// (0x389590): `a[i] += b[i] * (s * c)` with the components rotated
/// (x by s*c.y, y by s*c.z, z by s*c.w, w by s*c.x) for ten vertices.
pub fn irradiance_add_scaled(c: &mut Cpu) {
    c.cycles += 10;
    c.lwc1(1, at(c, A2, 0xc));
    c.g[V0][0] = 9;
    c.lwc1(2, r32(c, A2));
    c.mul_s(7, 12, 1);
    c.lwc1(0, at(c, A2, 4));
    c.lwc1(1, at(c, A2, 8));
    c.mul_s(6, 12, 2);
    c.mul_s(5, 12, 0);
    c.mul_s(12, 12, 1);
    loop {
        c.cycles += 24;
        c.lwc1(0, r32(c, A1));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.lwc1(1, r32(c, A0));
        c.mul_s(0, 0, 5);
        c.lwc1(2, at(c, A0, 4));
        c.lwc1(3, at(c, A0, 8));
        c.lwc1(4, at(c, A0, 0xc));
        c.add_s(1, 1, 0);
        c.swc1(1, r32(c, A0));
        c.lwc1(0, at(c, A1, 4));
        c.mul_s(0, 0, 12);
        c.add_s(2, 2, 0);
        c.swc1(2, at(c, A0, 4));
        c.lwc1(0, at(c, A1, 8));
        c.mul_s(0, 0, 7);
        c.add_s(3, 3, 0);
        c.swc1(3, at(c, A0, 8));
        c.lwc1(0, at(c, A1, 0xc));
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
        c.mul_s(0, 0, 6);
        c.add_s(4, 4, 0);
        c.swc1(4, at(c, A0, 0xc));
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
        if !more {
            c.cycles += 2;
            let ra = r32(c, RA);
            c.pc = ra;
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_95b8);
        }
    }
}

// ---------------------------------------------------------------------------
// Renderer.

/// `void PS2GraphicsMan_DrawShadowEdgePass(self, strips, int n, a3, a4, a5)`
/// (0x386dd0): emit every strip block with at least two vertices.
pub fn ps2_graphics_man_draw_shadow_edge_pass(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[T1][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[T0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A3][0];
        c.g[S5][0] = c.g[A0][0];
        let none = (c.g[S1][0] as i64) <= 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 1;
            loop {
                c.cycles += 4;
                c.g[A3][0] = Addr(p(r32(c, S0), 8)).lw(c);
                c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                } else {
                    c.cycles += 10;
                    c.g[A1][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A2][0] = Addr(p(r32(c, S0), 4)).lw(c);
                    c.g[T0][0] = c.g[S4][0];
                    c.lwc1(12, p(r32(c, S0), 0xc));
                    c.g[T1][0] = c.g[S3][0];
                    c.lwc1(13, p(r32(c, S0), 0x10));
                    c.g[T2][0] = c.g[S2][0];
                    call_ptr(c, 0x0038_7ec0, 0x00386e44)?;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                }
                c.cycles += 2;
                let more = (c.g[S1][0] as i64) > 0;
                c.g[S0][0] = sx(p(r32(c, S0), 0x14));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0038_6e10);
                }
            }
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

/// `PS2GraphicsMan_SetBrightnessRGB(this, rgb)` (0x37c570): store the three
/// channels, each clamped to [0, 1] and scaled to 0..128, into the manager's
/// brightness words.
pub fn ps2_graphics_man_set_brightness_rgb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let this = abi::arg(c, 0);
        let rgb = abi::arg(c, 1);
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = abi::arg64(c, 1);
        frame.save_fpr(c, 21, 0x38);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save_fpr(c, 20, 0x30);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x40);

        c.g[AT][0] = 0;
        c.mtc1(0, 21);
        c.lwc1(0, rgb);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20);
        c.c_le_s(21, 0);
        let negative = !c.fcc();
        c.mov_s(12, 21);
        if !negative {
            c.cycles += 1;
            c.min_s(12, 0, 20);
        }

        c.cycles += 10;
        c.g[AT][0] = 0x4300_0000;
        c.mtc1(1, 22);
        c.mul_s(12, 12, 22);
        call_ptr(c, addr::__FIXUNSSFSI, 0x0037_c5c8)?;
        Addr(this).off(0x59ec).sw(c, c.g[V0][0]);

        c.lwc1(0, rgb.wrapping_add(4));
        c.c_le_s(21, 0);
        let negative = !c.fcc();
        c.mov_s(12, 21);
        if !negative {
            c.cycles += 1;
            c.min_s(12, 0, 20);
        }

        c.cycles += 8;
        c.mul_s(12, 12, 22);
        call_ptr(c, addr::__FIXUNSSFSI, 0x0037_c5ec)?;
        Addr(this).off(0x59f0).sw(c, c.g[V0][0]);

        c.lwc1(0, rgb.wrapping_add(8));
        c.c_le_s(21, 0);
        let negative = !c.fcc();
        c.mov_s(12, 21);
        if !negative {
            c.cycles += 1;
            c.min_s(12, 0, 20);
        }

        c.cycles += 11;
        c.mul_s(12, 12, 22);
        call_ptr(c, addr::__FIXUNSSFSI, 0x0037_c610)?;
        Addr(this).off(0x59f4).sw(c, c.g[V0][0]);

        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `cPS2GridMesh *PS2GraphicsMan_CreateGridMesh(PS2GraphicsMan *self,
/// int a1, int a2, u32 flags)` (0x37a540): allocate and construct a grid mesh;
/// when the `0x40000000` flag is set, initialise its vertex range too.
pub fn ps2_graphics_man_create_grid_mesh(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = 0x0049_28d8;
        c.g[A2][0] = c.g[S0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x1cc;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0037_a580)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        callee::PS2_GRID_MESH_CTOR.call(c, 0x0037_a598)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = 0x4000_0000;
        c.g[S0][0] &= c.g[V0][0];
        let no_range = c.g[S0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !no_range {
            c.cycles += 2;
            let mut no_mesh = false;
            if c.g[S1][0] != 0 {
                c.g[V1][0] = Addr(p_5(c.g[S1][0], 0x4)).lw(c);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                no_mesh = true;
            }
            if !no_mesh {
                c.cycles += 6;
                c.g[A0][0] = Addr(p_5(c.g[V1][0], 0x10)).lh(c);
                c.g[V0][0] = Addr(p_5(c.g[V1][0], 0x14)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0037_a5cc)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.g[S1][0];
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(p_5(c.g[S1][0], 0x4)).lw(c);
                    c.g[A1][0] = 0x3;
                    c.g[A0][0] = Addr(p_5(c.g[V0][0], 0x8)).lh(c);
                    c.g[V1][0] = Addr(p_5(c.g[V0][0], 0xc)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0037_a5ec)?;
                    c.g[V0][0] = 0;
                }
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

/// `void func_00385AA0(void* self, u16* buf, int idx, const float* c)`
/// (0x385aa0): pack `c[1]`, `c[2]`, `c[3]`, `c[0]` (clamped to 0..1 and scaled
/// by 32) into the 16-bit PS2 colour word `buf[idx]`: `a << 15 | b << 10 |
/// g << 5 | r`, each channel saturated to 31 (alpha becomes 1 when set).
pub fn cps2_grid_mesh_pack_color(c: &mut Cpu) {
    c.cycles += 10;
    c.mtc1(0, 2); // f2 = 0.0
    c.g[T3][0] = c.g[A1][0]; // buf
    c.lwc1(1, p(c.g[A3][0] as u32, 4)); // f1 = c[1]
    c.g[T2][0] = c.g[A2][0]; // idx
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3); // f3 = 1.0
    c.c_le_s(2, 1); // 0 <= c[1]
    c.mov_s(0, 2); // f0 = 0.0
    if c.fcc() {
        c.cycles += 1;
        c.min_s(0, 1, 3); // f0 = min(c[1], 1.0)
    }

    c.cycles += 10;
    c.g[AT][0] = 0x4200_0000;
    c.mtc1(1, 5); // f5 = 32.0
    c.mov_s(4, 2); // f4 = 0.0
    c.lwc1(1, p(c.g[A3][0] as u32, 8)); // f1 = c[2]
    c.mul_s(0, 0, 5); // f0 *= 32
    c.c_le_s(2, 1);
    c.cvt_w_s(6, 0); // f6 = (int)f0
    c.mfc1(2, 6); // v0 = f6
    let t = !c.fcc();
    c.g[T1][0] = c.g[V0][0] & 0xff; // r
    if !t {
        c.cycles += 1;
        c.min_s(4, 1, 3); // f4 = min(c[2], 1.0)
    }

    c.cycles += 8;
    c.mul_s(0, 4, 5); // f0 = f4 * 32
    c.lwc1(1, p(c.g[A3][0] as u32, 0xc)); // f1 = c[3]
    c.mov_s(4, 2);
    c.c_le_s(2, 1);
    c.cvt_w_s(6, 0);
    c.mfc1(2, 6);
    let t = !c.fcc();
    c.g[T0][0] = c.g[V0][0] & 0xff; // g
    if !t {
        c.cycles += 1;
        c.min_s(4, 1, 3);
    }

    c.cycles += 8;
    c.mul_s(0, 4, 5);
    c.lwc1(1, c.g[A3][0] as u32); // f1 = c[0]
    c.mov_s(4, 2);
    c.c_le_s(2, 1);
    c.cvt_w_s(2, 0);
    c.mfc1(2, 2);
    let t = !c.fcc();
    c.g[A3][0] = c.g[V0][0] & 0xff; // b
    if !t {
        c.cycles += 1;
        c.min_s(4, 1, 3);
    }

    c.cycles += 23;
    c.mul_s(0, 4, 5); // f0 = f4 * 32
    c.g[A1][0] = 0x1f;
    c.g[V1][0] = (c.g[A3][0] < 0x20) as u64;
    c.g[V0][0] = 0x1;
    if c.g[V1][0] == 0 {
        c.g[A3][0] = c.g[A1][0];
    }
    c.g[A0][0] = (c.g[T0][0] < 0x20) as u64;
    c.cvt_w_s(1, 0);
    c.mfc1(3, 1); // t3 = (int)(clamp(c[0]) * 32)
    if c.g[A0][0] == 0 {
        c.g[T0][0] = c.g[A1][0];
    }
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 10);
    c.g[T0][0] = sx((c.g[T0][0] as u32) << 5);
    c.g[A2][0] = (c.g[T1][0] < 0x20) as u64;
    c.g[A0][0] = sx((c.g[T2][0] as u32) << 1);
    c.g[V1][0] = c.g[V1][0] & 0xff;
    if c.g[A2][0] == 0 {
        c.g[T1][0] = c.g[A1][0];
    }
    if c.g[V1][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T3][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 15);
    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
    c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
    c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
    c.sh(c.g[A0][0] as u32, c.g[V0][0]);
    abi::ret(c);
}

/// `void LineStrip_Finish(PktBuilder* self, int flags)` (0x384e50): close
/// the strip's packet: flush the open block, emit the two-quadword draw kick
/// from the current node (batching under 2600 vertices), advance the
/// cursors, and optionally release the builder's packet.
pub fn line_strip_finish(c: &mut Cpu) {
    'b_384fa8: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, c.pa(SP, 0x20));
        c.sq(17, c.pa(SP, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.std(SP, 0, c.g[RA][0]);
        c.g[V0][0] = c.ldw_2(S0, 0x8);
        let t = c.g[V0][0] == 0u64;
        c.g[S1][0] = c.g[A1][0];
        if t {
            break 'b_384fa8;
        } else {
            'b_384e88: {
                c.cycles += 3;
                c.g[V0][0] = c.ldw_2(S0, 0xc);
                if c.g[V0][0] == 0u64 {
                    c.g[A0][0] = c.ldw_2(S0, 0x8);
                    break 'b_384e88;
                } else {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00384e84);
                    c.pc = 0x00384dc0;
                    call_indirect(c, 0x00384dc0);
                    if c.pc != 0x00384e84 {
                        return;
                    }
                    c.g[A0][0] = c.ldw_2(S0, 0x8);
                    break 'b_384e88;
                }
            }
            'b_384f94: {
                c.cycles += 34;
                c.g[A3][0] = 0x10000000;
                c.g[V0][0] = c.ldw_2(S0, 0x4);
                c.g[A2][0] = 0x50000000;
                c.g[A1][0] = 0xfff0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V1][0] = c.g[V1][0] | c.g[A3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
                c.std(A0, 0, c.g[V1][0]);
                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V1][0] = c.ldw_2(S0, 0x8);
                c.std(V1, 0x8, 0u64);
                c.g[A0][0] = c.ldw_2(S0, 0x8);
                c.std(A0, 0x10, 0u64);
                c.g[V1][0] = c.ldw_2(S0, 0x8);
                c.std(V1, 0x18, c.g[V0][0]);
                c.g[V0][0] = c.ldw_2(S0, 0x4);
                c.g[A0][0] = c.ldw_2(S0, 0x8);
                c.g[V1][0] = c.ldw_2(GP, 0x2a90);
                c.g[T1][0] = c.g[V0][0] & c.g[A1][0];
                c.g[T2][0] = c.g[A0][0] & c.g[A1][0];
                c.stw_2(S0, 0x4, c.g[T1][0]);
                c.stw_2(S0, 0x8, c.g[T2][0]);
                c.g[A3][0] = c.ldw_2(V1, 0x18f0);
                c.g[V0][0] = c.ldw_2(A3, 0);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[T0][0] = c.ldw_2(V1, 0xe84);
                if t {
                    break 'b_384f94;
                } else {
                    c.cycles += 33;
                    c.g[A1][0] = 0x60000;
                    c.g[A0][0] = c.ldw_2(T0, 0x8);
                    c.g[A1][0] = c.g[A1][0] | 0x8000;
                    c.g[V1][0] = 0xffffffffffffffe0;
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                    c.g[V0][0] = c.ldw_2(A1, 0x1cc4);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                    c.g[V1][0] = 0x30000000;
                    c.g[V0][0] = c.g[V0][0] & 0x1f;
                    c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
                    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                    c.stw_2(T0, 0x8, c.g[A0][0]);
                    c.g[V1][0] = c.ldw_2(A3, 0);
                    c.g[A0][0] = c.ldw_2(A1, 0x1cc0);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.stw_2(A3, 0, c.g[V1][0]);
                    c.ldl(3, c.pa(T0, 0x7));
                    c.ldr(3, c.pa(T0, 0));
                    c.ldl(5, c.pa(T0, 0xf));
                    c.ldr(5, c.pa(T0, 0x8));
                    c.g[A2][0] = c.ldw_2(T0, 0x10);
                    c.sdl(3, c.pa(V0, 0x7));
                    c.sdr(3, c.pa(V0, 0));
                    c.sdl(5, c.pa(V0, 0xf));
                    c.sdr(5, c.pa(V0, 0x8));
                    c.stw_2(V0, 0x10, c.g[A2][0]);
                    c.stw_2(V0, 0x14, c.g[T2][0]);
                    c.stw_2(V0, 0x18, c.g[T1][0]);
                    c.sth_2(V0, 0x1c, c.g[A0][0]);
                    c.sth_2(V0, 0x1e, 0u64);
                    break 'b_384f94;
                }
            }
            c.cycles += 5;
            c.g[V0][0] = c.ldw_2(S0, 0x4);
            c.g[V1][0] = c.ldw_2(S0, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            c.stw_2(S0, 0x4, c.g[V0][0]);
            c.stw_2(V1, 0, c.g[V0][0]);
            break 'b_384fa8;
        }
    }
    'b_384fc0: {
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] == 0u64 {
            c.lq(16, c.pa(SP, 0x20));
            break 'b_384fc0;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x00384fbc);
            c.g[A0][0] = c.g[S0][0];
            c.pc = 0x00317e50;
            call_indirect(c, 0x00317e50);
            if c.pc != 0x00384fbc {
                return;
            }
            c.lq(16, c.pa(SP, 0x20));
            break 'b_384fc0;
        }
    }
    c.cycles += 4;
    c.lq(17, c.pa(SP, 0x10));
    c.g[RA][0] = c.ldd(SP, 0);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = ra;
    return;
}

/// `PS2GraphicsMan_FindDisplayMode` (0x003762f8): find the index of the
/// display-mode texture matching `(width, height, bpp, clutBpp, fmt, id)` in
/// the manager's texture list (`self+0x59c4` count, `self+0x59cc` array of
/// 0xa4-byte records), or -1.  Each of the three format fields must be
/// supported by the entry: bpp selects `psm` (+0x90), clutBpp selects
/// `clutPsm` (+0x94) and fmt 24 selects `fmt` (+0x98).
pub fn ps2_graphics_man_find_display_mode(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    let this = c.g[A0][0] as u32;
    c.g[T4][0] = 0u64;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[T1][0]; // fmt
    frame.save(c, S2, 0x0);
    c.g[S1][0] = c.g[A2][0]; // height
    c.g[S2][0] = c.g[A1][0]; // width
    c.g[V0][0] = c.lw(this.wrapping_add(0x59c4));
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[V1][0] = c.lw(this.wrapping_add(0x59cc));
    // `i` in T4, the current record in V1, the record count in T1.
    if empty {
        c.cycles += 1;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    } else {
        c.cycles += 7;
        c.g[T1][0] = c.g[V0][0];
        c.g[T7][0] = 0x18;
        c.g[A2][0] = (c.g[A3][0] < 0x19u64) as u64; // bpp < 25
        c.g[T9][0] = 0x10;
        c.g[T8][0] = 0x20;
        c.g[T5][0] = 0x1;
        c.g[A1][0] = (c.g[T0][0] < 0x19u64) as u64; // clutBpp < 25
        loop {
            let rec = c.g[V1][0] as u32;
            c.cycles += 3;
            c.g[V0][0] = c.lw(rec.wrapping_add(0x88)); // width
            if c.g[V0][0] != c.g[S2][0] {
                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(rec.wrapping_add(0x8c)); // height
                if c.g[V0][0] != c.g[S1][0] {
                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(rec.wrapping_add(0xa0)); // id
                    if c.g[V0][0] != c.g[T2][0] {
                        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
                    } else {
                        // ok1: the entry's pixel format supports `bpp`.
                        c.cycles += 4;
                        c.g[T3][0] = 0u64;
                        c.g[A0][0] = 0u64;
                        let fmt_24 = c.g[A3][0] == c.g[T7][0];
                        c.g[T6][0] = 0u64;
                        'bpp: {
                            if fmt_24 {
                                c.cycles += 2;
                                c.g[T3][0] = c.g[T5][0];
                                if c.g[V0][0] != 0 {
                                    c.g[T3][0] = 0;
                                }
                                break 'bpp;
                            }
                            if c.g[A2][0] != 0 {
                                c.cycles += 2;
                                c.cycles += 2;
                                if c.g[A3][0] == c.g[T9][0] {
                                    c.g[V0][0] = c.lw(rec.wrapping_add(0x90));
                                    c.cycles += 3;
                                    c.g[T3][0] = c.g[T5][0];
                                    c.g[V0][0] = c.g[V0][0] ^ 0x2;
                                    if c.g[V0][0] != 0 {
                                        c.g[T3][0] = 0;
                                    }
                                    c.cycles += 2;
                                    c.g[T3][0] = c.g[T5][0];
                                    if c.g[V0][0] != 0 {
                                        c.g[T3][0] = 0;
                                    }
                                    break 'bpp;
                                }
                                c.cycles += 2;
                                break 'bpp;
                            }
                            c.cycles += 2;
                            c.cycles += 2;
                            if c.g[A3][0] == c.g[T8][0] {
                                c.g[V0][0] = c.lw(rec.wrapping_add(0x90));
                                c.cycles += 4;
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                if c.g[V0][0] == 0 {
                                    c.g[T3][0] = c.g[T5][0];
                                }
                            } else {
                                c.cycles += 2;
                            }
                        }
                        // ok2: the entry's CLUT format supports `clutBpp`.
                        c.cycles += 2;
                        let clut_24 = c.g[T0][0] == c.g[T7][0];
                        if clut_24 {
                            c.g[V0][0] = c.lw(rec.wrapping_add(0x94));
                            c.cycles += 3;
                            c.g[A0][0] = c.g[T5][0];
                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                            c.cycles += 1;
                            if c.g[V0][0] != 0 {
                                c.g[A0][0] = 0;
                            }
                        } else if c.g[A1][0] != 0 {
                            c.cycles += 2;
                            c.cycles += 2;
                            if c.g[T0][0] == c.g[T9][0] {
                                c.g[V0][0] = c.lw(rec.wrapping_add(0x94));
                                c.cycles += 3;
                                c.g[A0][0] = c.g[T5][0];
                                c.g[V0][0] = c.g[V0][0] ^ 0x2;
                                c.cycles += 1;
                                if c.g[V0][0] != 0 {
                                    c.g[A0][0] = 0;
                                }
                            } else {
                                c.cycles += 2;
                            }
                        } else {
                            c.cycles += 2;
                            c.cycles += 2;
                            if c.g[T0][0] == c.g[T8][0] {
                                c.g[V0][0] = c.lw(rec.wrapping_add(0x94));
                                c.cycles += 1;
                                c.g[A0][0] = c.g[T5][0];
                                c.cycles += 1;
                                if c.g[V0][0] != 0 {
                                    c.g[A0][0] = 0;
                                }
                            } else {
                                c.cycles += 2;
                            }
                        }
                        // ok3: only fmt 24 constrains the texture format.
                        c.cycles += 2;
                        if c.g[S0][0] == c.g[T7][0] {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw(rec.wrapping_add(0x98));
                            c.g[T6][0] = c.g[T5][0];
                            c.g[V0][0] = c.g[V0][0] ^ 0x31;
                            if c.g[V0][0] != 0 {
                                c.g[T6][0] = 0;
                            }
                        }
                        c.cycles += 2;
                        if c.g[T3][0] == 0 {
                            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
                        } else {
                            c.cycles += 2;
                            if c.g[A0][0] == 0 {
                                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
                            } else {
                                c.cycles += 2;
                                let found = c.g[T6][0] != 0;
                                c.g[V0][0] = c.g[T4][0];
                                if found {
                                    // Return the index.
                                    c.cycles += 5;
                                    frame.restore(c, S0, 0x20);
                                    frame.restore(c, S1, 0x10);
                                    frame.restore(c, S2, 0x0);
                                    frame.restore_ra(c, 0);
                                    frame.close(c);
                                    return;
                                }
                                c.cycles += 1;
                                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
                            }
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[T1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xa4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0037_6340);
            }
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore(c, S2, 0x0);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Bézier patch cache tessellation.

/// Load the patch's 4x4 grid of control points (16 quads from `p`) into
/// vf1..vf16.
pub(crate) fn load_control_points(c: &mut Cpu, p: u32) {
    for k in 0..16u32 {
        c.lqc2(1 + k as usize, p.wrapping_add(0x10 * k));
    }
}

/// The curve of one parameter row: weights from the basis entry at_2 `w`
/// (vf17) applied to each column of control points, giving the curve's
/// four control points in vf18..vf21.
pub(crate) fn basis_row(c: &mut Cpu, w: u32) {
    c.lqc2(17, w);
    for (k, out) in (18u8..=21).enumerate() {
        let col = 1 + 4 * k as u8;
        c.vu0().mula(0b1111, col, Src::Bc(17, X));
        c.vu0().madda(0b1111, col + 1, Src::Bc(17, Y));
        c.vu0().madda(0b1111, col + 2, Src::Bc(17, Z));
        c.vu0().madd(0b1111, out, col + 3, Src::Bc(17, W));
    }
}

/// A point of the row curve (vf18..vf21): weights from the basis entry at_2
/// `w` (vf22); the point in vf23.
pub(crate) fn curve_point(c: &mut Cpu, w: u32) {
    c.lqc2(22, w);
    c.vu0().mula(0b1111, 18, Src::Bc(22, X));
    c.vu0().madda(0b1111, 19, Src::Bc(22, Y));
    c.vu0().madda(0b1111, 20, Src::Bc(22, Z));
    c.vu0().madd(0b1111, 23, 21, Src::Bc(22, W));
}

/// The end of a row (both tessellators): `v1 = v0 (row + 1)`, `v0 = row
/// < n`; true when another row follows, `v0 = row << 4` then.
pub(crate) fn next_row(c: &mut Cpu, n_reg: usize) -> bool {
    c.cycles += 4;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[n_reg][0] as i64)) as u64;
    let more = c.g[V0][0] != 0;
    c.g[V0][0] = sx(r32(c, V1) << 4);
    more
}

/// `void BezierPatchCache_TessPositions(BezierPatchCache* cache,
/// BezierPatch* patch, Vec3* out, int lod)` (0x374180): evaluate the bicubic
/// patch (control points at_2 patch+0x40) on the LOD's `n x n` parameter grid
/// (basis table `cache->basis[lod]` at_2 +0x190, `n = cache->count[lod]` at_2
/// +0x19c, one quad of weights per parameter) and write the positions as
/// packed vec3s, row by row. The quad goes through a 16-byte stack slot
/// (`ldl/ldr` + `lw`, then `sdl/sdr` + `sw`).
pub fn bezier_patch_cache_tess_positions(c: &mut Cpu) {
    const ROW_HEAD: u32 = 0x0037_41e8;
    const POINT_HEAD: u32 = 0x0037_4240;
    c.cycles += 25;
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, A3)));
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x40));
    c.g[A0][0] = c.g[V0][0];
    c.g[T0][0] = c.lw(at(c, V0, 0x190)); // basis
    c.g[A3][0] = c.lw(at(c, A0, 0x19c)); // n
    load_control_points(c, r32(c, A1));
    c.g[V1][0] = 0;
    if (c.g[A3][0] as i64) > 0 {
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, V1) << 4);
        loop {
            c.cycles += 20;
            c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
            basis_row(c, r32(c, V0));
            let empty = (c.g[A3][0] as i64) <= 0;
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
            if !empty {
                c.cycles += 2;
                c.g[A0][0] = c.g[T0][0];
                c.g[V1][0] = c.g[A3][0];
                loop {
                    c.cycles += 16;
                    curve_point(c, r32(c, A0));
                    let tmp = sp(c);
                    c.sqc2(23, tmp);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.ldl(A1, tmp.wrapping_add(7));
                    c.ldr(A1, tmp);
                    c.g[T1][0] = c.lw(tmp.wrapping_add(8));
                    let out = r32(c, A2);
                    c.sdl(A1, out.wrapping_add(7));
                    c.sdr(A1, out);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                    c.sw(out.wrapping_add(8), c.g[T1][0]);
                    let more = c.g[V1][0] != 0;
                    c.g[A2][0] = sx(out.wrapping_add(0xc));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, POINT_HEAD);
                    }
                }
            }
            if !next_row(c, A3) {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, ROW_HEAD);
            }
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    ret(c);
}

/// `void BezierPatchCache_TessPositionsAlt(BezierPatchCache* cache,
/// BezierPatch* patch, Vec3* out, int lod)` (0x374518): two passes over the
/// LOD's `n x n` grid (`n` at_2 +0x19c, value basis at_2 +0x190, derivative
/// basis at_2 +0x310, per LOD). Pass 1 writes one tangent (derivative-basis
/// rows, value-basis points) to `out`; pass 2 evaluates the other tangent
/// (value-basis rows, derivative-basis points) and replaces each entry
/// with the normalised cross product of the two (left unnormalised when
/// its length is 0): the patch normals.
pub fn bezier_patch_cache_tess_positions_alt(c: &mut Cpu) {
    const DU_ROW_HEAD: u32 = 0x0037_4590;
    const DU_POINT_HEAD: u32 = 0x0037_45e8;
    const DV_ROW_HEAD: u32 = 0x0037_4648;
    const DV_POINT_HEAD: u32 = 0x0037_46b0;
    c.cycles += 28;
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, A3)));
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x40));
    c.g[V0][0] = c.g[V1][0];
    c.g[T2][0] = c.lw(at(c, V1, 0x310)); // derivative basis
    c.g[A0][0] = c.g[V0][0];
    c.g[T0][0] = c.lw(at(c, V0, 0x19c)); // n
    c.g[T1][0] = c.lw(at(c, A0, 0x190)); // value basis
    c.g[A3][0] = c.g[A2][0];
    load_control_points(c, r32(c, A1));
    c.g[V1][0] = 0;
    // Pass 1: d/du into out.
    if (c.g[T0][0] as i64) > 0 {
        c.cycles += 2;
        c.g[V0][0] = sx(r32(c, V1) << 4);
        loop {
            c.cycles += 20;
            c.g[V0][0] = sx(r32(c, T2).wrapping_add(r32(c, V0)));
            basis_row(c, r32(c, V0));
            let empty = (c.g[T0][0] as i64) <= 0;
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
            if !empty {
                c.cycles += 2;
                c.g[A0][0] = c.g[T1][0];
                c.g[V1][0] = c.g[T0][0];
                loop {
                    c.cycles += 16;
                    curve_point(c, r32(c, A0));
                    let tmp = sp(c);
                    c.sqc2(23, tmp);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.ldl(A1, tmp.wrapping_add(7));
                    c.ldr(A1, tmp);
                    c.g[T3][0] = c.lw(tmp.wrapping_add(8));
                    let out = r32(c, A3);
                    c.sdl(A1, out.wrapping_add(7));
                    c.sdr(A1, out);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                    c.sw(out.wrapping_add(8), c.g[T3][0]);
                    let more = c.g[V1][0] != 0;
                    c.g[A3][0] = sx(out.wrapping_add(0xc));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, DU_POINT_HEAD);
                    }
                }
            }
            if !next_row(c, T0) {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, DU_ROW_HEAD);
            }
        }
    }
    // Pass 2: d/dv, crossed with d/du, normalised.
    c.cycles += 3;
    c.g[A3][0] = c.g[A2][0];
    c.g[V1][0] = 0;
    if (c.g[T0][0] as i64) > 0 {
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, V1) << 4);
        loop {
            c.cycles += 20;
            c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, V0)));
            basis_row(c, r32(c, V0));
            let empty = (c.g[T0][0] as i64) <= 0;
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
            if !empty {
                c.cycles += 6;
                c.mtc1(0, 9); // f9 = 0
                c.g[A0][0] = c.g[T2][0];
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 8); // f8 = 1
                c.g[V1][0] = c.g[T0][0];
                loop {
                    c.cycles += 34;
                    curve_point(c, r32(c, A0));
                    normal_at(c);
                    c.cycles += 10;
                    let tmp = sp(c);
                    c.ldl(A1, tmp.wrapping_add(0x17));
                    c.ldr(A1, tmp.wrapping_add(0x10));
                    c.g[A2][0] = c.lw(tmp.wrapping_add(0x18));
                    let out = r32(c, A3);
                    c.sdl(A1, out.wrapping_add(7));
                    c.sdr(A1, out);
                    c.sw(out.wrapping_add(8), c.g[A2][0]);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    let more = c.g[V1][0] != 0;
                    c.g[A3][0] = sx(out.wrapping_add(0xc));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, DV_POINT_HEAD);
                    }
                }
            }
            if !next_row(c, T0) {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, DV_ROW_HEAD);
            }
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
    ret(c);
}

/// Pass 2 of `TessPositionsAlt` at_2 one grid entry: `a = out[i]` (d/du),
/// `b = vf23` (d/dv, stored at_2 sp+0); `n = a × b` at_2 sp+0x20, then
/// `n / |n|` (or `n` when `|n| == 0`) at_2 sp+0x10..0x18 (f9 = 0, f8 = 1).
pub(crate) fn normal_at(c: &mut Cpu) {
    let tmp = sp(c);
    let out = r32(c, A3);
    c.sqc2(23, tmp);
    c.lwc1(1, out.wrapping_add(8)); // a.z
    c.lwc1(6, out); // a.x
    c.lwc1(4, tmp.wrapping_add(4)); // b.y
    c.lwc1(0, tmp.wrapping_add(8)); // b.z
    c.lwc1(3, out.wrapping_add(4)); // a.y
    c.mul_s(7, 4, 1);
    c.lwc1(2, tmp); // b.x
    c.mul_s(5, 0, 6);
    c.mul_s(0, 0, 3);
    c.mul_s(1, 2, 1);
    c.mul_s(2, 2, 3);
    c.sub_s(7, 7, 0); // f7 = b.y a.z - b.z a.y
    c.sub_s(5, 5, 1); // f5 = b.z a.x - b.x a.z
    c.mul_s(4, 4, 6);
    c.mul_s(0, 7, 7);
    c.swc1(7, tmp.wrapping_add(0x20));
    c.mul_s(3, 5, 5);
    c.swc1(5, tmp.wrapping_add(0x24));
    c.sub_s(2, 2, 4); // f2 = b.x a.y - b.y a.x
    c.add_s(0, 0, 3);
    c.mul_s(1, 2, 2);
    c.swc1(2, tmp.wrapping_add(0x28));
    c.add_s(0, 0, 1);
    c.sqrt_s(0, 0);
    c.c_eq_s(0, 9);
    if c.fcc() {
        c.swc1(7, tmp.wrapping_add(0x10));
        c.cycles += 2;
        c.swc1(5, tmp.wrapping_add(0x14));
        c.swc1(2, tmp.wrapping_add(0x18));
    } else {
        c.cycles += 8;
        c.div_s(0, 8, 0); // 1 / |n|
        c.mul_s(1, 2, 0);
        c.mul_s(2, 7, 0);
        c.mul_s(0, 5, 0);
        c.swc1(1, tmp.wrapping_add(0x18));
        c.swc1(2, tmp.wrapping_add(0x10));
        c.swc1(0, tmp.wrapping_add(0x14));
    }
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
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as TEST_SP;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::ai::ai::tests::generic;
    pub(crate) use crate::game::ai::ai::tests::globals_8 as globals_5;
    pub(crate) use crate::game::ai::ai::tests::slot as slot_3;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::ai::tests::wf;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS_4;
    pub(crate) use crate::game::be::beintplayer::tests::base_10 as base_8;
    pub(crate) use crate::game::be::beintplayer::tests::globals_5 as globals_4;
    pub(crate) use crate::game::be::beintplayer::tests::strict;
    pub(crate) use crate::game::be::beintplayer::tests::SEEDS_7 as SEEDS_12;
    pub(crate) use crate::game::be::belibrary::tests::alloc_ready;
    pub(crate) use crate::game::be::belibrary::tests::load;
    pub(crate) use crate::game::be::belibrary::tests::SRC;
    pub(crate) use crate::game::bx::bxstring2::tests::alloc_base;
    pub(crate) use crate::game::bx::bxstring2::tests::run as run_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::base as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_11;
    pub(crate) use crate::game::fe::feasyncfile::tests::VT;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_2;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_8;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::input::inputparse::tests::EMPTY;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_3;
    pub(crate) use crate::game::main::gamemode::tests::h;
    pub(crate) use crate::game::main::gamemode::tests::slot as slot_4;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_9;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_4;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::ui::uivector::tests::start;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::ctx_at;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::put_ctx;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::BASE;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::CTX;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS_3 as SEEDS_2;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::TEX;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::TOP;
    pub(crate) use crate::game::worldpainter::quadtree::tests::alloc_machine;
    pub(crate) use crate::game::worldpainter::quadtree::tests::heap_init;
    pub(crate) use crate::game::worldpainter::quadtree::tests::pool_allocator;
    pub(crate) use crate::game::worldpainter::quadtree::tests::run;
    pub(crate) use crate::game::worldpainter::quadtree::tests::MEM_HEAP;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::load as load_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn vec3_add_to_matches() {
        check(
            ssx3_ee::s12::f_00389558__generated,
            vec3_add_to,
            300,
            &|c, seed| {
                let mut r = Rng(0x9e37_79b9 ^ seed as u64);
                scramble(c, &mut r);
                let dst = DATA;
                // Cover distinct, exactly-alias and partially-overlapping vectors.
                let src = match seed % 3 {
                    0 => DATA,
                    1 => DATA + 4,
                    _ => DATA + 0x100,
                };
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = src as u64;
                for i in 0..3 {
                    c.bus.write32(dst + i * 4, r.edge_f32());
                }
                for i in 0..3 {
                    c.bus.write32(src + i * 4, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn bezier_patch_cache_ctor_matches() {
        check(
            ssx3_ee::s12::f_00372b30,
            bezier_patch_cache_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x372b_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
                for k in 0..3 {
                    c.bus.write32(DATA + 0xc + k * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn boader_shadow_set_params_matches() {
        check(
            ssx3_ee::s12::f_00374ce0,
            boader_shadow_set_params,
            200,
            &|c, seed| {
                let mut r = Rng(0x374c_e000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
                c.g[A2][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
                c.g[A3][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
                c.g[T0][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
                c.g[T1][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
                c.g[T2][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
            },
        );
    }

    #[test]
    fn boader_shadow_ctor_matches() {
        check(
            ssx3_ee::s12::f_003758f8,
            boader_shadow_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x3758_f800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
                for k in 0..4 {
                    c.bus.write32(DATA + 0x170 + k * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn detail_system_ctor_matches() {
        check(
            ssx3_ee::s12::f_003759e8,
            detail_system_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x3759_e800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn line_strip_init_matches() {
        check(
            ssx3_ee::s12::f_00384d98,
            line_strip_init,
            200,
            &|c, seed| {
                let mut r = Rng(0x384d_9800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.bus.write32(DATA + 0x100, r.next_u32());
            },
        );
    }

    #[test]
    fn line_strip_close_matches() {
        check(
            ssx3_ee::s12::f_00384dc0,
            line_strip_close,
            200,
            &|c, seed| {
                let mut r = Rng(0x384d_c000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let buf = DATA + 0x400;
                c.bus.write32(DATA + 0xc, buf);
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x10, 0);
                } else {
                    c.bus.write32(DATA + 0x10, 1);
                    c.bus.write32(DATA + 0x4, buf);
                }
                c.bus.write32(buf + 0x30, r.next_u32());
            },
        );
    }

    #[test]
    fn line_strip_add_vertex_matches() {
        check(
            ssx3_ee::s12::f_00384fd0,
            line_strip_add_vertex,
            200,
            &|c, seed| {
                let mut r = Rng(0x384f_d000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                c.bus.write32(DATA + 0x4, DATA + 0x800);
                c.bus.write32(DATA + 0x10, r.next_u32() & 0xff);
                for k in 0..8 {
                    c.bus.write32(DATA + 0x100 + k * 4, r.edge_f32());
                }
                for k in 0..4 {
                    c.bus.write32(DATA + 0x200 + k * 4, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn cps2_grid_mesh_pack_position_matches() {
        check(
            ssx3_ee::s12::f_00385a38,
            cps2_grid_mesh_pack_position,
            200,
            &|c, seed| {
                let mut r = Rng(0x385a_3800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.g[A3][0] = (DATA + 0x2000) as u64;
                c.bus.write32(DATA + 0x198, r.edge_f32());
                for k in 0..3 {
                    c.bus.write32(DATA + 0x2000 + k * 4, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn cps2_grid_mesh_pack_uv_matches() {
        check(
            ssx3_ee::s12::f_00385ba0,
            cps2_grid_mesh_pack_uv,
            200,
            &|c, seed| {
                let mut r = Rng(0x385b_a000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.g[A3][0] = (DATA + 0x2000) as u64;
                for k in 0..2 {
                    c.bus.write32(DATA + 0x2000 + k * 4, r.edge_f32());
                }
            },
        );
    }

    /// Shared setup for the two irradiance functions: `a0` a surface point,
    /// `a2` the light, `a1` the output vector.
    pub(crate) fn irradiance_setup(
        c: &mut Cpu,
        seed: u32,
        diffs_are_zero: bool,
        radius_zero: bool,
    ) {
        let mut r = Rng(0x38a0_0000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = (DATA + 0x100) as u64;
        c.g[A1][0] = (DATA + 0x300) as u64;
        c.g[A2][0] = (DATA + 0x200) as u64;
        c.g[A3][0] = (DATA + 0x400) as u64;
        c.g[T0][0] = (DATA + 0x500) as u64;
        c.g[T1][0] = (DATA + 0x600) as u64;
        for k in 0..3 {
            c.bus.write32(DATA + 0x100 + k * 4, r.edge_f32());
        }
        for k in 0..3 {
            let v = if diffs_are_zero {
                c.bus.read32(DATA + 0x100 + k * 4)
            } else {
                r.edge_f32()
            };
            c.bus.write32(DATA + 0x200 + 0x38 + k * 4, v);
        }
        c.bus.write32(
            DATA + 0x200 + 0x1c,
            if radius_zero { 0 } else { r.edge_f32() },
        );
        for off in [0x2c, 0x30, 0x34] {
            c.bus.write32(DATA + 0x200 + off, r.edge_f32());
        }
    }

    #[test]
    fn irradiance_spot_light_vector_matches() {
        check(
            ssx3_ee::s12::f_0038a530,
            irradiance_spot_light_vector,
            200,
            &|c, seed| match seed % 4 {
                0 => irradiance_setup(c, seed, false, true),
                1 => irradiance_setup(c, seed, true, true),
                _ => irradiance_setup(c, seed, false, false),
            },
        );
    }

    #[test]
    fn irradiance_point_light_vector_matches() {
        check(
            ssx3_ee::s12::f_0038a618,
            irradiance_point_light_vector,
            200,
            &|c, seed| match seed % 3 {
                0 => irradiance_setup(c, seed, false, true),
                _ => irradiance_setup(c, seed, false, false),
            },
        );
    }

    pub(crate) const SEEDS: u32 = 250;

    pub(crate) fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// A float in [lo, hi).
    pub(crate) fn uf(r: &mut Rng, lo: f32, hi: f32) -> f32 {
        lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)
    }

    /// Scrambled registers and the game's gp.
    pub(crate) fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    pub(crate) fn put_vec3(c: &mut Cpu, r: &mut Rng, at: u32, lo: f32, hi: f32) {
        for k in 0..3 {
            c.bus.write32(at + 4 * k, f(uf(r, lo, hi)));
        }
    }

    pub(crate) fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// An emitter at `e` with four-slot rings.
    pub(crate) fn put_emitter(c: &mut Cpu, r: &mut Rng, e: u32, seed: u32) {
        c.bus.write32(e + 0x174, if seed % 7 == 0 { 0 } else { 1 });
        c.bus.write32(e + 0x178, 4);
        c.bus.write32(e + 0x17c, r.next_u32() % 4);
        c.bus.write32(e + 0x1a0, DATA + 0x1000);
        c.bus.write32(e + 0x1a4, DATA + 0x2000);
        c.bus.write32(e + 0x204, DATA + 0x3000);
        c.bus.write32(e + 0xc, 1 + r.next_u32() % 4);
        c.bus.write32(e + 0x10, f(uf(r, 0.0, 4.0)));
        c.bus.write32(e + 0x14, f(uf(r, 0.0, 2.0)));
        c.bus.write32(e + 0x1e0, r.next_u32() % 8);
        fill(c, r, e + 0x1b0, 8);
        for k in 0..4 {
            c.bus.write32(e + 0x1d0 + 4 * k, f(uf(r, -0.5, 1.5)));
        }
        for i in 0..4 {
            fill(c, r, DATA + 0x2000 + 0x10 * i, 3);
            c.bus
                .write32(DATA + 0x2000 + 0x10 * i + 0xc, f(uf(r, -1.0, 1.0)));
        }
    }

    #[test]
    fn dynamic_emitter_spawn_matches() {
        check(
            ee::s12::f_003710d0__generated,
            dynamic_emitter_spawn,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x3710);
                let e = DATA;
                put_emitter(c, &mut r, e, seed);
                fill(c, &mut r, DATA + 0x400, 8);
                c.g[A0][0] = sx(e);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x400) };
                c.g[A2][0] = if seed % 5 == 0 { 0 } else { sx(DATA + 0x410) };
                c.g[A3][0] = (seed / 2 % 2) as u64;
                c.f[12] = f(uf(&mut r, 0.0, 3.0));
            },
        );
    }

    #[test]
    fn dynamic_emitter_spawn_at_matches() {
        check(
            ee::s12::f_003717c0__generated,
            dynamic_emitter_spawn_at,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x3717);
                let e = DATA;
                put_emitter(c, &mut r, e, seed);
                fill(c, &mut r, DATA + 0x400, 8);
                for k in 0..4 {
                    c.bus
                        .write32(DATA + 0x420 + 4 * k, f(uf(&mut r, -0.5, 1.5)));
                }
                c.g[A0][0] = sx(e);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x400) };
                c.g[A2][0] = if seed % 5 == 0 { 0 } else { sx(DATA + 0x410) };
                c.g[A3][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x420) };
                c.g[T0][0] = (seed / 2 % 2) as u64;
                c.f[12] = f(uf(&mut r, 0.0, 3.0));
            },
        );
    }

    #[test]
    fn ps2_graphics_man_v58_matches() {
        check(
            ee::s12::f_003889f0__generated,
            ps2_graphics_man_v58,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x3889);
                let (out, gm, table, tex, clut, pixels) = (
                    DATA,
                    DATA + 0x100,
                    DATA + 0x2000,
                    DATA + 0x2100,
                    DATA + 0x2400,
                    DATA + 0x4000,
                );
                c.bus.write32(gm + 0x18f4, table);
                let idx = r.next_u32() % 4;
                c.bus
                    .write32(table + 8 + 4 * idx, if seed % 9 == 0 { 0 } else { tex });
                let psm = [0u32, 0xa, 0xc, 0x13][(seed % 4) as usize];
                c.bus.write32(tex + 0xc, psm);
                c.bus.write32(tex + 0x1c, pixels);
                c.bus.write32(tex + 0x20, clut);
                let (tw, th) = (1 + r.next_u32() % 6, 1 + r.next_u32() % 6);
                let tex0 = (r.next_u32() as u64 & 0x03ff_ffff)
                    | (tw as u64) << 26
                    | (th as u64) << 30
                    | (r.next_u32() as u64) << 34;
                c.bus.write64(tex + 0x38, tex0);
                fill(c, &mut r, pixels, 0x1000);
                fill(c, &mut r, clut, 0x100);
                c.bus.write32(gp_at(-0x2730), f(uf(&mut r, 0.0, 2.0)));
                c.f[12] = if seed % 11 == 0 {
                    f(0.0)
                } else {
                    f(uf(&mut r, -0.3, 1.3))
                };
                c.f[13] = if seed % 13 == 0 {
                    f(1.0)
                } else {
                    f(uf(&mut r, -0.3, 1.3))
                };
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(gm);
                c.g[A2][0] = idx as u64;
            },
        );
    }

    #[test]
    fn irradiance_add_dynamic_light_matches() {
        check(
            ee::s12::f_0038a6a8__generated,
            irradiance_add_dynamic_light,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38a6, 0);
                put_words(
                    c,
                    IR_TABLE_OUTER,
                    &[
                        0x0038_a834,
                        0x0038_a848,
                        0x0038_a850,
                        0x0038_a868,
                        0x0038_a884,
                    ],
                );
                put_words(
                    c,
                    IR_TABLE_INNER,
                    &[
                        0x0038_a948,
                        0x0038_a95c,
                        0x0038_a970,
                        0x0038_a988,
                        0x0038_a9a4,
                    ],
                );
                let (irr, pos, light) = (DATA, DATA + 0x400, DATA + 0x500);
                let kind = [1u32, 2, 0, 3, 1, 2][(seed % 6) as usize];
                c.bus.write32(light + 0x10, kind);
                put_vec3(c, &mut r, pos, -20.0, 20.0);
                put_vec3(c, &mut r, light, -20.0, 20.0);
                c.bus.write32(light + 0xc, f(1.0));
                fill(c, &mut r, irr, 0x40);
                c.bus.write32(light + 0x14, f(uf(&mut r, 0.0, 3.0)));
                c.bus.write32(light + 0x18, f(uf(&mut r, -1.0, 1.0)));
                c.bus.write32(light + 0x1c, f(uf(&mut r, 0.0, 2.0)));
                put_vec3(c, &mut r, light + 0x20, 0.0, 1.0);
                put_vec3(c, &mut r, light + 0x2c, -1.0, 1.0);
                fill(c, &mut r, light + 0x38, 8);
                for k in 0..8 {
                    c.bus
                        .write32(light + 0x40 + 4 * k, f(uf(&mut r, -30.0, 30.0)));
                }
                let outer = uf(&mut r, 0.0, 10.0);
                c.bus.write32(light + 0x60, f(outer));
                c.bus
                    .write32(light + 0x5c, f(outer + uf(&mut r, 0.0, 30.0)));
                c.bus.write8(light + 0x64, (r.next_u32() % 5) as u8);
                let exponent = match seed % 5 {
                    0 => (r.next_u32() % 5) as u8,
                    1 => (r.next_u32() % 4) as u8,
                    2 => 5 + (r.next_u32() % 6) as u8,
                    3 => (0u8).wrapping_sub(1 + (r.next_u32() % 4) as u8),
                    _ => (r.next_u32() % 3) as u8,
                };
                c.bus.write8(light + 0x65, exponent);
                for off in [0x26e4, 0x26e0, 0x26dc, 0x26d8, 0x26d4, 0x26d0] {
                    c.bus.write32(gp_at(-off), f(uf(&mut r, 0.0, 1.0)));
                }
                c.g[A0][0] = sx(irr);
                c.g[A1][0] = sx(pos);
                c.g[A2][0] = sx(light);
            },
        );
    }

    pub(crate) fn base_2(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 8) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// The test vtable (`DATA + 0x3000`); entries are `{delta: 0, fn}`.
    pub(crate) fn vt() -> u32 {
        DATA + 0x3000
    }

    pub(crate) fn slot(c: &mut Cpu, off: u32, target: u32) {
        c.bus.write16(vt() + off, 0);
        c.bus.write32(vt() + off + 4, target);
    }

    /// A DeferMan stand-in at `DATA + 0x8000` with empty record lists.
    pub(crate) fn put_defer(c: &mut Cpu) -> u32 {
        let d = DATA + 0x8000;
        let rec = d + 0x68000;
        c.bus.write32(rec + 0x1cc4, 0);
        c.bus.write32(rec + 0x1cc8, 0);
        c.bus.write32(rec + 0x2ed0, 0);
        d
    }

    pub(crate) fn identity(c: &mut Cpu, at: u32) {
        for i in 0..4u32 {
            for k in 0..4u32 {
                let v = if i == k { f(1.0) } else { 0 };
                c.bus.write32(at + 0x10 * i + 4 * k, v);
            }
        }
    }

    #[test]
    fn ps2_graphics_man_update_view_matrices_matches() {
        check(
            ee::s12::f_0037d968__generated,
            ps2_graphics_man_update_view_matrices,
            SEEDS,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x37_d968);
                let gfx = DATA;
                let cam = DATA + 0x1000;
                let defer = put_defer(c);
                for i in 0..4u32 {
                    for k in 0..4u32 {
                        c.bus.write32(cam + 0x10 * i + 4 * k, r.edge_f32());
                        c.bus
                            .write32(gfx + man::WORLD + 0x10 * i + 4 * k, r.edge_f32());
                        c.bus
                            .write32(gfx + man::LOCAL_MAT + 0x10 * i + 4 * k, r.edge_f32());
                        c.bus
                            .write32(gfx + man::MAT5860 + 0x10 * i + 4 * k, r.edge_f32());
                    }
                }
                c.bus.write32(gfx + man::CAMERA, cam);
                c.bus.write32(gfx + man::DEFER, defer);
                c.g[A0][0] = sx(gfx);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_push_view_matches() {
        check(
            ee::s12::f_003765b8__generated,
            ps2_graphics_man_push_view,
            200,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x37_65b8);
                let gfx = DATA;
                let defer = put_defer(c);
                fill(c, &mut r, gfx + man::CAM_MAT, 0x200);
                fill(c, &mut r, gfx + man::WORLD, 0xd0);
                fill(c, &mut r, gfx + man::VIEWS, 0x460);
                c.bus.write32(gfx + man::CUR_VIEW, (seed % 3) as u32);
                c.bus.write32(gfx + man::DEFER, defer);
                c.bus.write32(defer + 0x68000 + 0x1cc4, r.next_u32());
                c.g[A0][0] = sx(gfx);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_pop_view_matches() {
        check(
            ee::s12::f_00376768__generated,
            ps2_graphics_man_pop_view,
            200,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x37_6768);
                let gfx = DATA;
                let defer = put_defer(c);
                fill(c, &mut r, gfx + man::CAM_MAT, 0x200);
                fill(c, &mut r, gfx + man::WORLD, 0xd0);
                fill(c, &mut r, gfx + man::VIEWS, 0x460);
                let cur = 1 + (seed % 2) as u32;
                c.bus.write32(gfx + man::CUR_VIEW, cur);
                let view = gfx + man::VIEWS + (cur - 1) * man::VIEW_STRIDE;
                let key = if seed % 3 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32()
                };
                c.bus.write32(view + vw::KEY, key);
                c.bus.write32(gfx + man::DEFER, defer);
                c.bus.write32(defer + 0x68000 + 0x1cc4, 0);
                c.g[A0][0] = sx(gfx);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_viewport_matches() {
        covered(
            ee::s12::f_00376a70__generated,
            ps2_graphics_man_set_viewport,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37_6a70, 4);
                let gfx = DATA;
                let defer = put_defer(c);
                fill(c, &mut r, gfx + man::CAM_MAT, 0x200);
                fill(c, &mut r, gfx + man::WORLD, 0xd0);
                fill(c, &mut r, gfx + man::VIEWS, 0x460);
                slot(c, 0x50, FAKE);
                slot(c, 0x58, FAKE + 0x10);
                c.bus.write32(gfx + man::DISPLAY, vt());
                c.bus.write32(gfx + man::CUR_VIEW, (seed % 2) as u32);
                c.bus.write32(gfx + man::DEFER, defer);
                let mode = match seed % 4 {
                    0 => 0,
                    1 => 2,
                    2 => 3,
                    _ => 0xffff_ffff,
                };
                c.bus.write32(gfx + man::CLAMP_MODE, mode);
                c.bus.write32(gfx + man::CLAMP_TOP, f(0.75));
                c.bus.write32(gfx + man::CLAMP_BOTTOM, f(0.5));
                for i in 0..4 {
                    c.f[12 + i] = r.edge_f32();
                }
                c.g[A0][0] = sx(gfx);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_v27_matches() {
        check(
            ee::s12::f_00377278__generated,
            ps2_graphics_man_v27,
            200,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x37_7278);
                let gfx = DATA;
                let pos = DATA + 0x1000;
                let cam = DATA + 0x2000;
                let defer = put_defer(c);
                // The camera's third column scales the point along the view's z.
                identity(c, cam);
                c.bus.write32(cam + 0x28, f(0.25));
                identity(c, gfx + man::MAT5860);
                identity(c, gfx + man::WORLD);
                identity(c, gfx + man::LOCAL_MAT);
                identity(c, gfx + man::VIEW_MAT);
                c.bus.write32(gfx + man::VIEW_MAT + 0x28, f(0.25));
                let (z, dist) = if seed % 7 == 3 {
                    (5.0f32, 1.0f32)
                } else {
                    match seed % 4 {
                        0 => (0.0, 0.0),
                        1 => (0.0, 1.0),
                        2 => (0.0, 2.0),
                        _ => (0.0, 1.0),
                    }
                };
                c.bus.write32(pos, 0);
                c.bus.write32(pos + 4, 0);
                c.bus.write32(pos + 8, f(z));
                c.bus.write32(pos + 0xc, f(1.0));
                c.bus.write32(gfx + man::CAMERA, cam);
                c.bus.write32(gfx + man::DEFER, defer);
                let idx = (seed % 2) as u32;
                c.bus.write32(gfx + man::CUR_VIEW, idx);
                c.bus
                    .write32(gfx + man::MODE, if seed % 3 == 0 { 0x31 } else { 0x10 });
                c.bus
                    .write32(gfx + man::TEX_READY, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(gfx + man::F58E8, f(100.0));
                c.bus.write32(GAME_GP - 0x279c, f(32767.5));
                c.bus.write32(GAME_GP - 0x2798, f(8388467.5));
                let d = if z == 0.0 { 6.25 * dist } else { 0.0 };
                let depth = if seed % 2 == 0 { f(d) } else { r.edge_f32() };
                let view = gfx + man::VIEWS + idx * man::VIEW_STRIDE;
                c.bus.write32(view + vw::DEPTH, depth);
                c.g[A0][0] = sx(gfx);
                c.g[A1][0] = sx(pos);
                c.f[12] = f(dist);
            },
        );
    }

    // PS2GraphicsMan_CullAgainstLights ------------------------------------

    pub(crate) const CULL: u32 = DATA;

    pub(crate) const CULL_VT: u32 = DATA + 0x2000;

    #[test]
    fn ps2graphics_man_cull_against_lights_matches() {
        covered(
            ssx3_ee::s12::f_0037df88__generated,
            ps2graphics_man_cull_against_lights,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37df88, 1);
                c.g[A0][0] = CULL as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = (DATA + 0x900) as u64;
                let count = (seed % 4) as u32;
                c.bus.write32(CULL + 0x13ec, count);
                if count != 0 {
                    c.bus.write32(CULL + 0x10d8, CULL_VT);
                    c.bus.write16(CULL_VT + 0x2e8, 0);
                    c.bus.write32(CULL_VT + 0x2ec, FAKE);
                }
                for i in 0..4 {
                    for k in 0..4 {
                        c.bus
                            .write32(CULL + 0x1400 + 0xa0 * i + 4 * k, r.next_u32());
                    }
                }
            },
        );
    }

    #[test]
    fn m_emitter_render_matches() {
        check(
            ee::s12::f_00371688__generated,
            m_emitter_render,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1688, 1);
                put_ctx(c, &mut r, &[0x2a0]);
                let e = BASE;
                c.bus.write32(e + 0x174, (seed % 5 != 0) as u32);
                c.bus.write32(e + 0x1e0, (seed % 3 != 0) as u32);
                c.bus.write32(e + 4, 0); // base
                c.bus.write32(e + 8, r.next_u32() % 8); // kind
                c.bus
                    .write32(e + 0x10, if seed % 4 == 0 { f(1.5) } else { 0 });
                for i in 0..5 {
                    c.bus
                        .write32(globals::EMITTER_BLEND + 4 * i, r.next_u32() % 32);
                }
                for i in 0..3u32 {
                    let tex = if seed % 7 == 3 && i == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 0x1000
                    };
                    c.bus.write32(TEX + 4 * i, tex);
                }
                c.bus.write32(e + 0x1e8, r.next_u32());
                c.bus.write32(e + 0x1ec, r.next_u32());
                for i in 0..5 {
                    c.bus.write32(e + 0x1e4 + 4 * i, r.next_u32());
                }
                c.bus.write32(e + 0x1f4, r.next_u32());
                c.g[A0][0] = sx(e);
                c.g[A1][0] = sx(seed % 4);
            },
        );
    }

    #[test]
    fn bezier_patch_cache_mark_used_matches() {
        check(
            ssx3_ee::s12::f_00374a88__generated,
            bezier_patch_cache_mark_used,
            200,
            &|c, seed| {
                let mut r = Rng(0x2f19_a4c3 ^ seed as u64);
                scramble(c, &mut r);
                let cache = DATA;
                let items = DATA + 0x1000;
                let lists = DATA + 0x2000;
                let idx = r.next_u32() % 4;
                let bit = r.next_u32() % 32;
                let count = seed % 5;
                for i in 0..4u32 {
                    let v: i32 = match (seed + i) % 4 {
                        0 => 0,
                        1 => -((r.next_u32() % 4) as i32) - 1,
                        2 => 3,
                        _ => (r.next_u32() % 3) as i32,
                    };
                    c.bus.write16(items + i * 0x10 + 0xc, v as u16);
                }
                c.bus.write32(cache + 0x18 + idx * 4, lists);
                for k in 0..4u32 {
                    c.bus.write32(lists + k * 0xc + 4, r.next_u32());
                }
                c.g[A0][0] = sx(cache);
                c.g[A1][0] = sx(count);
                c.g[A2][0] = sx(items);
                c.g[A3][0] = sx(idx);
                c.g[T0][0] = sx(bit);
                c.g[T1][0] = 0xc;
            },
        );
    }

    #[test]
    fn boader_shadow_submit_matches() {
        covered(
            ssx3_ee::s12::f_00375890__generated,
            boader_shadow_submit,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 83, 1);
                let g = DATA + 0x200;
                c.bus.write32(GAME_GP + 0x2a90, g);
                let vt = vtable(c, &mut r, &[0x218]);
                c.bus.write32(g + 0x10d8, vt);
                let s = DATA + 0x600;
                c.bus.write32(s + 0xc, r.f32(100.0));
                c.bus.write32(s + 0x10, r.f32(100.0));
                c.bus.write32(s + 0x14, r.f32(100.0));
                c.bus.write32(s + 0x170, r.next_u32());
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn irradiance_add_scaled_w_matches() {
        check(
            ssx3_ee::s12::f_00389620__generated,
            irradiance_add_scaled_w,
            200,
            &|c, seed| {
                let mut r = Rng(0x7a3f_19d5 ^ (seed as u64) << 5);
                scramble(c, &mut r);
                let src = DATA + 0x1000;
                let dst = DATA + 0x2000;
                for i in 0..10u32 {
                    for lane in 0..3u32 {
                        c.bus.write32(src + i * 0x10 + lane * 4, r.next_u32());
                        c.bus.write32(dst + i * 0x10 + lane * 4, r.next_u32());
                    }
                    c.bus.write32(src + i * 0x10 + 0xc, r.edge_f32());
                    c.bus.write32(dst + i * 0x10 + 0xc, r.edge_f32());
                }
                c.g[A0][0] = sx(dst);
                c.g[A1][0] = sx(src);
                c.f[12] = if seed % 3 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(2.0)
                };
            },
        );
    }

    #[test]
    fn c_ps2_grid_mesh_v03_matches() {
        check(
            ee::s12::f_003852e8__generated,
            c_ps2_grid_mesh_v03,
            120,
            &|c, seed| {
                let mut r = Rng(0x3852_e8 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                let verts = DATA + 0x4000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(verts);
                let w = seed % 4;
                let rows = (seed / 4) % 3;
                let tab = if seed % 2 == 0 {
                    DATA + 0x1000
                } else {
                    DATA + 0x2000
                };
                c.bus.write16(this + grid_mesh::WIDTH, w as u16);
                c.bus.write32(this + grid_mesh::SCALE, 1.0f32.to_bits());
                c.bus
                    .write32(this + grid_mesh::TAB_INDEX, (seed % 2) as u32);
                c.bus.write32(this + grid_mesh::PACKED, 0);
                c.bus.write32(this + grid_mesh::ROWS, rows);
                c.bus.write32(this + grid_mesh::ROW_TABS, tab);
                c.bus.write32(this + grid_mesh::ROW_TABS + 4, tab);
                for i in 0..rows as u32 {
                    c.bus.write32(tab + i * 16, r.next_u32());
                    c.bus.write32(tab + i * 16 + 4, DATA + 0x3000 + i * 0x100);
                }
                for i in 0..16 {
                    c.bus.write32(verts + i * 12, r.edge_f32());
                    c.bus.write32(verts + i * 12 + 4, r.edge_f32());
                    c.bus.write32(verts + i * 12 + 8, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn bezier_man_base_ctor_matches() {
        check(
            ee::s12::f_00374c90__generated,
            bezier_man_base_ctor,
            SEEDS_3,
            &|c, seed| {
                let mut r = rng(0x0037_4c90, seed);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA + 0x100);
                c.bus.write32(DATA + 0x104, r.next_u32());
                c.bus.write32(0x004a_4248, r.next_u32());
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_tex_alloc_flags_matches() {
        check(
            ee::s12::f_0037c8a0__generated,
            ps2_graphics_man_set_tex_alloc_flags,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37_c8a0, 0);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x18f4, DATA + 0x600);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ps2_graphics_man_v118_matches() {
        check(
            ee::s12::f_00386cf0__generated,
            ps2_graphics_man_v118,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38_6cf0, 0);
                c.g[A0][0] = ptr(&mut r) as u64;
            },
        );
    }

    #[test]
    fn c_ps2_grid_mesh_v07_matches() {
        check(
            ee::s12::f_00385be0__generated,
            c_ps2_grid_mesh_v07,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x385b_e0, 2);
                be_library(c);
                let mesh = DATA + 0x100;
                let sub = DATA + 0x200;
                c.g[A0][0] = sx(mesh);
                c.g[A1][0] = 0;
                c.g[A2][0] = sx(DATA + 0x300);
                // `b + 4` -> a pointer whose first short is the `this` delta.
                c.bus.write32(DATA + 0x304, DATA + 0x500);
                c.bus.write16(DATA + 0x500, r.next_u32() as u16);
                // `self + 4` -> the sub-object whose `+0x40`/`+0x44` are used.
                c.bus.write32(mesh + 4, sub);
                c.bus.write16(sub + 0x40, (r.next_u32() & 0xff) as u16);
                c.bus
                    .write32(sub + 0x44, if seed % 3 == 0 { 0 } else { fakes::FAKE });
            },
        );
    }

    #[test]
    fn irradiance_get_entry_by_name_matches() {
        check(
            ee::s12::f_0038ac20__generated,
            irradiance_get_entry_by_name,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x38ac_20, 2);
                be_library(c);
                let set = DATA + 0x100;
                c.g[A0][0] = sx(set);
                c.g[A1][0] = sx(DATA + 0x300);
                c.bus.write32(set + 4, 3);
                c.bus.write32(set + 8, DATA + 0x1000);
                c.bus.write32(set + 0xc, DATA + 0x200);
                let names: [&[u8]; 3] = [b"aaa\0".as_slice(), b"bbb\0", b"ccc\0"];
                for (i, name) in names.iter().enumerate() {
                    for (j, b) in name.iter().enumerate() {
                        c.bus.write8(DATA + 0x200 + (i as u32) * 8 + j as u32, *b);
                    }
                }
                let want: &[u8] = if seed % 2 == 0 { b"bbb\0" } else { b"zzz\0" };
                for (j, b) in want.iter().enumerate() {
                    c.bus.write8(DATA + 0x300 + j as u32, *b);
                }
            },
        );
    }

    /// A real block of `size` bytes from the main heap (for the deleting
    /// destructors, whose `operator delete` must see a valid chunk).
    pub(crate) fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0032_0058,
            ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    #[test]
    fn bezier_man_base_dtor_matches() {
        check(
            ee::s12::f_00374ca8__generated,
            bezier_man_base_dtor,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x374c_a8, 0);
                pool_allocator(c);
                heap_init(c);
                let obj = heap_block(c, 0x80);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx((seed & 3) as u32);
                c.bus.write32(GAME_GP + 0x1158, r.next_u32());
            },
        );
    }

    #[test]
    fn ps2_graphics_man_create_matches() {
        check(
            ee::s12::f_00375a08__generated,
            ps2_graphics_man_create,
            30,
            &|c, seed| {
                alloc_machine(c, seed, 0x375a_08);
            },
        );
    }

    #[test]
    fn renderer_vif1_dmac_handler_matches() {
        check(
            ee::s12::f_00382650__generated,
            renderer_vif1_dmac_handler,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3826_50, 0);
                // Only cause 1 is testable: any other cause is a `break` (fatal).
                c.g[A0][0] = 1;
                c.g[A1][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x5a8c, (seed % 4) as u32);
                fill(c, &mut r, DATA + 0x1000, 0x40);
            },
        );
    }

    #[test]
    fn emitter_ctor_matches() {
        check(
            ssx3_ee::s12::f_003714b8__generated,
            emitter_ctor,
            60,
            &|c, seed| {
                machine(c, seed, 0x3714_b8, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// A block from the test heap (`alloc_ready` must have run first).
    pub(crate) fn heap_block_2(c: &mut Cpu, size: u32) -> u32 {
        const MEM_HEAP: u32 = 0x004f_f250;
        load(
            c,
            0x0032_0058,
            ssx3_ee::s10::f_00320058,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    // ------------------------------------------------------------------
    // Graphics / UI.

    #[test]
    fn ps2_graphics_man_set_default_display_mode_matches() {
        check(
            ee::s12::f_00376578__generated,
            ps2_graphics_man_set_default_display_mode,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x376578, 0);
                let self_at = DATA + 0x200;
                let vt = vtable(c, &mut r, &[0x3d0]);
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(self_at + 0x10d8, vt);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_free_texture_matches() {
        check(
            ee::s12::f_0037cb50__generated,
            ps2_graphics_man_free_texture,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x37cb50, 0);
                alloc_ready(c);
                let self_at = DATA + 0x200;
                let texman = DATA + 0x300;
                let idx = seed % 4;
                // The texture record is a real (heap) block, so the delete at
                // the end of the free path is legitimate, and its `+0x28` /
                // `+0x2c` / `+0` say "no VRAM block": the VRAM heap (zeroed on
                // a test machine) is then never walked.
                let rec = heap_block_2(c, 0x400);
                for off in [
                    0u32, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x30, 0x34,
                ] {
                    c.bus.write32(rec + off, 0);
                }
                c.bus.write32(rec + 0x28, 0xffff_ffff);
                c.bus.write32(rec + 0x2c, 0xffff_ffff);
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = idx as u64;
                c.bus.write32(self_at + 0x18f4, texman);
                c.bus.write32(self_at + 0x18f8 + 4 * idx, seed % 5);
                c.bus.write32(texman + 8 + 4 * idx, rec);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_v59_matches() {
        covered(
            ee::s12::f_0037cbe8__generated,
            ps2_graphics_man_v59,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37_cbe8, 1);
                let self_ = DATA + 0x1000;
                let out = DATA + 0x2000;
                let vt = vtable(c, &mut r, &[0x1a0]);
                c.bus.write32(self_ + 0x10d8, vt);
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(out);
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x37c7c0 PS2GraphicsMan_ResetTextureSlots.

    #[test]
    fn ps2_graphics_man_reset_texture_slots_matches() {
        check(
            ee::s12::f_0037c7c0__generated,
            ps2_graphics_man_reset_texture_slots,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x37c7_c0, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = (seed as u64) << 4;
                c.bus.write32(DATA + 0x1000 + 0x18f4, DATA + 0x9000);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x389118 PS2GraphicsMan_RemoveInterruptHandlers.

    #[test]
    fn ps2_graphics_man_remove_interrupt_handlers_matches() {
        check(
            ee::s12::f_00389118__generated,
            ps2_graphics_man_remove_interrupt_handlers,
            8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3891_18, 0);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x5ac0, r.next_u32());
                c.bus.write32(this + 0x5ac4, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------------- emitters

    #[test]
    fn memitter_dtor_matches() {
        check(
            ee::s12::f_003714f8__generated,
            memitter_dtor,
            60,
            &|c, seed| {
                base_3(c, seed, 0x3714);
                let flags = if seed % 5 == 0 { 1u32 } else { 0 };
                // The delete path frees `self`: use a small (pooled) address.
                let obj = if flags == 1 { 0x100 } else { DATA };
                c.bus.write32(obj + 0x204, 0);
                c.bus.write32(obj + 0x1a0, 0);
                c.bus.write32(obj + 0x1a4, 0);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(flags);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_brightness_alpha_matches() {
        check(
            ee::s12::f_0037c720__generated,
            ps2_graphics_man_set_brightness_alpha,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37c7_20, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x4000);
                c.g[A0][0] = sx(this);
                c.f[12] = match seed % 4 {
                    0 => 0xbf80_0000, // -1
                    1 => 0x3f80_0000, // 1
                    2 => 0x4000_0000, // 2
                    _ => r.edge_f32(),
                };
            },
        );
    }

    #[test]
    fn boader_shadow_alloc_texture_matches() {
        check(
            ssx3_ee::s12::f_00375990__generated,
            boader_shadow_alloc_texture,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x75990, 1);
                let mgr = DATA + 0x100;
                let shadow = DATA + 0x200;
                c.bus.write32(GAME_GP + 0x2a90, mgr);
                let vt = vtable(c, &mut r, &[0x1a0]);
                c.bus.write32(mgr + 0x10d8, vt);
                c.bus.write32(
                    shadow + 0x170,
                    if seed % 2 == 0 { 0xffff_ffff } else { 0x1234 },
                );
                c.bus.write32(shadow + 0x174, r.next_u32() % 0x400);
                c.bus.write32(shadow + 0x178, r.next_u32() % 4);
                c.g[A0][0] = shadow as u64;
            },
        );
    }

    pub(crate) const SEEDS_7: u32 = 100;

    /// Scrambled state, fake hooks, a zeroed BELibrary dummy at `gp-0x1f04`.
    pub(crate) fn common(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        clear(c, DATA + 0xb000, 0x100);
        c.bus.write32(gp_at(-0x1f04), DATA + 0xb000);
        r
    }

    /// A vtable_2 whose chosen slots are `{delta, FAKE+k}` entries.
    pub(crate) fn vtable_2(c: &mut Cpu, at: u32, slots: &[(u32, i16, u32)]) {
        for &(off, delta, target) in slots {
            c.bus.write16(at + off, delta as u16);
            c.bus.write32(at + off + 4, target);
        }
    }

    #[test]
    fn ps2_graphics_man_select_matching_display_mode_matches() {
        check(
            ee::s12::f_00376468__generated,
            ps2_graphics_man_select_matching_display_mode,
            SEEDS_7,
            &|c, seed| {
                let _ = common(c, seed, 0x376468, 2);
                let this = DATA + 0x100;
                let vt = DATA + 0x300;
                clear(c, this, 0x200);
                clear(c, vt, 0x500);
                c.bus.write32(this + 0x10d8, vt);
                vtable_2(c, vt, &[(0x3d8, 0, FAKE), (0x20, 0, FAKE + 0x10)]);
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn ps2_graphics_man_load_texture_matches() {
        check(
            ee::s12::f_0037caf8__generated,
            ps2_graphics_man_load_texture,
            20,
            &|c, seed| {
                let _ = common(c, seed, 0x37caf8, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                c.g[A3][0] = (seed % 5) as u64;
            },
        );
    }

    #[test]
    fn ps2_graphics_man_emit_model_matrix2_matches() {
        check(
            ee::s12::f_0037e040__generated,
            ps2_graphics_man_emit_model_matrix2,
            SEEDS_7,
            &|c, seed| {
                let mut r = base(c, seed, 0x37e040);
                let m = DATA + 0x200;
                let pp = DATA + 0x180;
                clear(c, DATA, 0x400);
                put_words(
                    c,
                    m,
                    &[r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                c.bus.write32(pp, DATA + 0x300);
                c.g[A0][0] = sx(m);
                c.g[A1][0] = sx(pp);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_on_vif1_dma_end_matches() {
        check(
            ee::s12::f_00382688__generated,
            ps2_graphics_man_on_vif1_dma_end,
            SEEDS_7,
            &|c, seed| {
                let _ = common(c, seed, 0x382688, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x5b00 / 4);
                c.bus.write32(this + 0x5a8c, (seed % 5) as u32);
                c.g[A0][0] = sx(this);
            },
        );
    }

    // -------------------------------------------------- graphics manager

    #[test]
    fn ps2_graphics_man_get_display_mode_info_matches() {
        check_lohi(
            ssx3_ee::s12::f_00376560__generated,
            ps2_graphics_man_get_display_mode_info,
            150,
            &|c, seed| {
                let mut r = start(c, seed, 0x59cc);
                let gm = DATA + 0x400;
                c.g[A0][0] = gm as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.bus.write32(gm + 0x59cc, DATA + 0x800);
                for i in 0..8 {
                    c.bus
                        .write32(DATA + 0x800 + 0xa4 * (seed % 5) + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_get_viewport_matches() {
        check_lohi(
            ssx3_ee::s12::f_00376b90__generated,
            ps2_graphics_man_get_viewport,
            150,
            &|c, seed| {
                let mut r = start(c, seed, 0x6d20);
                let gm = DATA + 0x400;
                let mode = seed % 4;
                // `out` is a0, the manager (mode + viewport records) is a1.
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = gm as u64;
                c.bus.write32(gm + 0x10dc, mode);
                for i in 0..4 {
                    c.bus
                        .write32(gm + 0x6d20 + 0x230 * mode + 4 * i, r.edge_f32());
                }
                for i in 0..4 {
                    c.bus.write32(DATA + 0x0fc + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_get_fov_matches() {
        check_lohi(
            ssx3_ee::s12::f_00376c10__generated,
            ps2_graphics_man_get_fov,
            150,
            &|c, seed| {
                let mut r = start(c, seed, 0x6d38);
                let gm = DATA + 0x400;
                let mode = seed % 4;
                c.g[A0][0] = gm as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.bus.write32(gm + 0x10dc, mode);
                c.bus.write32(gm + 0x6d38 + 0x230 * mode, r.edge_f32());
            },
        );
    }

    #[test]
    fn ps2_graphics_man_get_near_far_aspect_matches() {
        check_lohi(
            ssx3_ee::s12::f_00376c28__generated,
            ps2_graphics_man_get_near_far_aspect,
            150,
            &|c, seed| {
                let mut r = start(c, seed, 0x6c28);
                let gm = DATA + 0x400;
                let mode = seed % 4;
                c.g[A0][0] = gm as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                c.g[A3][0] = (DATA + 0x300) as u64;
                c.bus.write32(gm + 0x10dc, mode);
                for i in 0..3 {
                    c.bus
                        .write32(gm + 0x6d20 + 0x230 * mode + 0x10 + 4 * i, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_fog_mode_matches() {
        check(
            ssx3_ee::s12::f_00377950__generated,
            ps2_graphics_man_set_fog_mode,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x77950);
                let gm = DATA + 0x400;
                c.g[A0][0] = gm as u64;
                c.g[A1][0] = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 5,
                    _ => u64::MAX, // -1
                };
                for i in 0..5 {
                    c.bus.write32(gm + 0x6b94 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_fog_params_matches() {
        check(
            ssx3_ee::s12::f_003779e0__generated,
            ps2_graphics_man_set_fog_params,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x779e0);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                for i in 0..5 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
                }
                for i in 0..6 {
                    c.bus.write32(DATA + 0x3f4 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_is_texture_valid_matches() {
        check(
            ssx3_ee::s12::f_0037cb90__generated,
            ps2_graphics_man_is_texture_valid,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x7cb90);
                let gm = DATA + 0x400;
                c.g[A0][0] = gm as u64;
                let idx = match seed % 4 {
                    0 => u64::MAX, // -1
                    1 => 2000,
                    2 => 2,
                    _ => seed as u64 % 4,
                };
                c.g[A1][0] = idx;
                c.bus.write32(gm + 0x18f4, DATA + 0x1000);
                for i in 0..4 {
                    c.bus.write32(
                        DATA + 0x1000 + 4 * i,
                        if seed % 2 == 0 {
                            0
                        } else {
                            DATA + 0x2000 + 0x10 * i
                        },
                    );
                    c.bus
                        .write32(DATA + 0x2008 + 0x10 * i, (seed + i) as u32 % 3);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_add_light_matches() {
        check_lohi(
            ssx3_ee::s12::f_0037d758__generated,
            ps2_graphics_man_add_light,
            100,
            &|c, seed| {
                let mut r = start(c, seed, 0x7d758);
                let gm = DATA + 0x400;
                c.g[A0][0] = gm as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                c.g[A3][0] = (DATA + 0x300) as u64;
                c.bus.write32(gm + 0x13ec, seed % 3);
                for i in 0..4 {
                    c.bus.write128(
                        DATA + 0x100 + 0x10 * i,
                        (r.next_u32() as u128) << 96 | 0x1111,
                    );
                }
                for i in 0..16 {
                    c.bus.write32(DATA + 0x200 + 4 * i, r.next_u32());
                    c.bus.write32(DATA + 0x300 + 4 * i, r.next_u32());
                }
                for i in 0..0x200 {
                    c.bus.write32(DATA + 0x13f0 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_get_texture_width_matches() {
        check(
            ssx3_ee::s12::f_0037d908__generated,
            ps2_graphics_man_get_texture_width,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x7d908);
                let gm = DATA + 0x400;
                c.g[A0][0] = gm as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(gm + 0x18f4, DATA + 0x1000);
                for i in 0..4 {
                    c.bus.write32(
                        DATA + 0x1000 + 4 * i,
                        if seed % 3 == 0 {
                            0
                        } else {
                            DATA + 0x2000 + 0x40 * i
                        },
                    );
                    c.bus
                        .write32(DATA + 0x2000 + 0x40 * i + 8, DATA + 0x3000 + 0x40 * i);
                    c.bus.write64(
                        DATA + 0x3000 + 0x40 * i + 0x38,
                        (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    );
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_get_texture_height_matches() {
        check(
            ssx3_ee::s12::f_0037d938__generated,
            ps2_graphics_man_get_texture_height,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x7d938);
                let gm = DATA + 0x400;
                c.g[A0][0] = gm as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(gm + 0x18f4, DATA + 0x1000);
                for i in 0..4 {
                    c.bus.write32(
                        DATA + 0x1000 + 4 * i,
                        if seed % 3 == 0 {
                            0
                        } else {
                            DATA + 0x2000 + 0x40 * i
                        },
                    );
                    c.bus
                        .write32(DATA + 0x2000 + 0x40 * i + 8, DATA + 0x3000 + 0x40 * i);
                    c.bus.write64(
                        DATA + 0x3000 + 0x40 * i + 0x38,
                        (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    );
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_clip_test_box_matches() {
        // The VU0 microprogram at 0x570 (`box_clip`) runs for real; it is
        // cheap, but 40 seeds keep the test well under the budget.
        check(
            ssx3_ee::s12::f_0037de88__generated,
            ps2_graphics_man_clip_test_box,
            40,
            &|c, seed| {
                let mut r = start(c, seed, 0x7de88);
                c.g[A0][0] = (DATA + 0x000) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                c.g[A3][0] = (DATA + 0x300) as u64;
                put_vec(
                    c,
                    DATA + 0x100,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    DATA + 0x200,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                for i in 0..4 {
                    put_vec(
                        c,
                        DATA + 0x300 + 0x10 * i,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    );
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_v97_matches() {
        check(
            ssx3_ee::s12::f_00380380__generated,
            ps2_graphics_man_v97,
            100,
            &|c, seed| {
                let mut r = start(c, seed, 0x380380);
                let obj = DATA + 0x400;
                c.g[A0][0] = 0;
                c.g[A1][0] = obj as u64;
                c.g[A2][0] = (seed % 2) as u64;
                c.g[A3][0] = (DATA + 0x5000) as u64;
                c.g[T0][0] = (DATA + 0x6000) as u64;
                c.g[T1][0] = (DATA + 0x7000) as u64;
                // obj -> model (+0x80), extra offset (+0x98).
                c.bus.write32(obj + 0x80, DATA + 0x8000);
                c.bus.write32(obj + 0x98, 0x20);
                // Model: corner-table offset, vertex base, scale.
                c.bus.write32(DATA + 0x8000 + 8, 0x100);
                c.bus.write32(DATA + 0x8000 + 0x24, 0x180);
                for i in 0..3 {
                    c.bus.write32(DATA + 0x8000 + 0x18 + 4 * i, r.edge_f32());
                }
                // Corner entry -> node -> vertex data (indices 0 and 1).
                c.bus.write32(DATA + 0x8100 + 4, DATA + 0x8200);
                c.bus.write32(DATA + 0x8110 + 4, DATA + 0x8200);
                c.bus.write32(DATA + 0x8200 + 0x20, DATA + 0x8300);
                c.bus.write32(DATA + 0x8300, DATA + 0x8400);
                c.bus.write32(DATA + 0x8400 + 4, DATA + 0x8500);
                // Vertex block: colour record pointer at +0x14 (data = +0x8520),
                // and the texcoord base at +0x4 of DATA+0x8500+0x180.
                c.bus.write32(DATA + 0x8520 + 0x14, DATA + 0x8600);
                c.bus.write32(DATA + 0x8680 + 4, DATA + 0x8780);
                for i in 0..4 {
                    for j in 0..3 {
                        c.bus
                            .write16(DATA + 0x8820 + 2 * (3 * i + j), r.next_u32() as u16);
                        c.bus
                            .write16(DATA + 0x8800 + 2 * (2 * i + j), r.next_u32() as u16);
                    }
                }
                for i in 0..8 {
                    c.bus.write16(DATA + 0x8600 + 2 * i, r.next_u32() as u16);
                }
                // The two gp_4 constants.
                c.bus.write32((GAME_GP as i32 - 0x2760) as u32, 0x3780_0000);
                c.bus.write32((GAME_GP as i32 - 0x275c) as u32, 0x3d04_2108);
                // Outputs (positions, uv, colours).
                for i in 0..16 {
                    c.bus.write32(DATA + 0x4ff0 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_add_view_matches() {
        check(
            ssx3_ee::s12::f_00386688__generated,
            ps2_graphics_man_add_view,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x386688);
                let gm = DATA + 0x400;
                c.g[A0][0] = gm as u64;
                c.g[A1][0] = (seed % 4) as u64;
                for i in 0..16 {
                    c.bus.write32(gm + 0x6af0 + 4 * i, r.next_u32());
                }
                for i in 0..4 {
                    c.bus.write32(gm + 0x58fc + 4 * i, r.next_u32());
                }
                for i in 0..0x40 {
                    c.bus.write32(gm + 0x5930 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn rail_tube_dtor_matches() {
        check(
            ee::s12::f_00372660__generated,
            rail_tube_dtor,
            8,
            &|c, seed| {
                machine(c, seed, 0x0037_2660, 1);
                be_library_2(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 8, 0);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_v60_matches() {
        check(
            ee::s12::f_0037cc30__generated,
            ps2_graphics_man_v60,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0037_cc30, 2);
                be_library_2(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.bus.write32(
                    DATA + 0x1004,
                    if seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() & 0x7fff
                    },
                );
                c.bus.write32(DATA + 0x18f4, 0);
                c.bus.write32(DATA + 0x10d8, DATA + 0x2000);
                c.bus.write16(DATA + 0x2000 + 0x190, 0);
                c.bus.write32(DATA + 0x2000 + 0x194, FAKE);
            },
        );
    }

    #[test]
    fn dynamic_emitter_load_matches() {
        check(
            ee::s12::f_00371318__generated,
            dynamic_emitter_load,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x371318, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                w_2(c, OBJ2, VT);
                slot_2(c, VT, 0x10, 0, fakes::FAKE);
                w_2(c, OBJ2 + 0x100, seed);
            },
        );
    }

    pub(crate) fn put_vec_2(c: &mut Cpu, addr: u32, x: u32, y: u32, z: u32) {
        c.bus.write32(addr, x);
        c.bus.write32(addr + 4, y);
        c.bus.write32(addr + 8, z);
    }

    /// `BoaderShadow_ComputeProjection`: object at `DATA` (holds the input
    /// vectors after the first pass), the three vector arguments, a
    /// six-entry bone table, and the globals/constants it reads. The seed
    /// picks zero, parallel, antiparallel and random input vectors so every
    /// zero-length and parallel branch is taken.
    #[test]
    fn boader_shadow_compute_projection_matches() {
        check(
            ssx3_ee::s12::f_00374d00__generated,
            boader_shadow_compute_projection,
            240,
            &|c, seed| {
                let mut r = Rng(0x374d_00 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x27f8), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x27f4), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x27f0), r.edge_f32());
                // Fixed data the original loads by absolute address.
                c.bus.write32(0x004a_431c, r.edge_f32());
                c.bus.write32(0x004a_4320, r.edge_f32());

                let obj = DATA;
                let a1 = DATA + 0x400;
                let a2 = DATA + 0x410;
                let a3 = DATA + 0x420;
                let bones = DATA + 0x800;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(a1);
                c.g[A2][0] = sx(a2);
                c.g[A3][0] = sx(a3);
                c.g[T0][0] = sx(bones);

                // Junk in every object field first, so a wrong offset in the
                // port cannot read a neighbour that happens to be zero too.
                fill(c, &mut r, obj, 0x68);

                // `b` (a2), `dir` (a3) and the reference vector (a1). The
                // structured seeds walk every parallel / antiparallel branch
                // (and the cycle charges they take); the rest are random.
                const ONE: u32 = 0x3f80_0000;
                const TWO: u32 = 0x4000_0000;
                const NEG_ONE: u32 = 0xbf80_0000;
                let (bx, by, bz, dx, dy, dz) = match seed % 9 {
                    0 => (0, 0, 0, 0, 0, 0),
                    // b == dir: the first parallel test sets v0.
                    1 => (ONE, ONE, ONE, ONE, ONE, ONE),
                    // b == -dir: only the second parallel test (and its +1) runs.
                    2 => (NEG_ONE, NEG_ONE, NEG_ONE, ONE, ONE, ONE),
                    // x equal (both zero), y different: the +4-only path of
                    // the first test (and of the second test's first compare).
                    3 => (0, ONE, TWO, 0, ONE, ONE),
                    // x opposite, y equal: the +4-only path of the second test.
                    4 => (NEG_ONE, ONE, ONE, ONE, ONE, ONE),
                    // x/y equal, z different: the third comparison fails.
                    5 => (ONE, ONE, NEG_ONE, ONE, ONE, ONE),
                    6 => (r.edge_f32(), r.edge_f32(), r.edge_f32(), 0, 0, 0),
                    7 => (0, 0, 0, r.edge_f32(), r.edge_f32(), r.edge_f32()),
                    _ => (
                        r.edge_f32(),
                        r.edge_f32(),
                        r.edge_f32(),
                        r.edge_f32(),
                        r.edge_f32(),
                        r.edge_f32(),
                    ),
                };
                put_vec_2(c, a2, bx, by, bz);
                put_vec_2(c, a3, dx, dy, dz);
                put_vec_2(c, a1, r.edge_f32(), r.edge_f32(), r.edge_f32());

                // Six bone indices and the integer depth inputs of the object.
                for i in 0..6u32 {
                    c.bus.write32(obj + 0x17c + i * 4, i);
                }
                let depth = if seed % 2 == 0 {
                    r.next_u32()
                } else {
                    r.next_u32() % 0x800
                };
                c.bus.write32(obj + 0x174, depth);
                c.bus.write32(obj + 0x178, r.next_u32() % 0x800);

                // The bone table (six 0x40-byte entries, row read at +0x30).
                fill(c, &mut r, bones, 0x80);
            },
        );
    }

    #[test]
    fn irradiance_scale_matches() {
        check(
            ssx3_ee::s12::f_003897a0__generated,
            irradiance_scale,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3897a0, 0);
                let out = DATA + 0x1000;
                let src = DATA + 0x2000;
                c.g[A0][0] = out as u64;
                c.g[A1][0] = src as u64;
                c.f[12] = r.edge_f32();
                for i in 0..10u32 {
                    for k in 0..4u32 {
                        w32(c, src + i * 0x10 + k * 4, r.next_u32());
                    }
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_free_all_textures_matches() {
        check(
            ssx3_ee::s12::f_0037c830__generated,
            ps2_graphics_man_free_all_textures,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37c830, 0);
                let m = DATA + 0x1000;
                let vt = DATA + 0x5000;
                c.g[A0][0] = m as u64;
                w32(c, m + 0x10d8, vt);
                slot_3(c, vt, 0x190, 0, EMPTY);
                for i in 0..2000u32 {
                    let v = if (seed as u32 + i) % 37 == 0 {
                        r.next_u32() | 1
                    } else {
                        0
                    };
                    w32(c, m + 0x18f8 + 4 * i, v);
                }
            },
        );
    }

    #[test]
    fn boader_shadow_dtor_matches() {
        check(
            ee::s12::f_00375918__generated,
            boader_shadow_dtor,
            SEEDS_9,
            &|c, seed| {
                machine(c, seed, 0x375918, 2);
                be_library_3(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, obj + 0x170, if seed % 3 == 0 { 0xffff_ffff } else { 5 });
                let mgr = DATA + 0x2000;
                w(c, GAME_GP.wrapping_add(0x2a90), mgr);
                w(c, mgr + 0x18f4, DATA + 0x2100);
                let vt = DATA + 0x2200;
                w(c, mgr + 0x10d8, vt);
                slot_4(c, vt, 0x190, 0x30, FAKE);
            },
        );
    }

    #[test]
    fn path_arrow_ctor_matches() {
        check(
            ee::s12::f_00372aa0__generated,
            path_arrow_ctor,
            12,
            &|c, seed| {
                let _ = base_4(c, seed, 0x372a_a0, 0);
                w(c, GAME_GP.wrapping_add(0x114c), (seed % 2) as u32);
                for (i, off) in [0x1118u32, 0x111c, 0x1120, 0x1124].into_iter().enumerate() {
                    w(c, GAME_GP.wrapping_add(off), 0x11_0000 + i as u32);
                }
                c.g[A0][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_clear_color_matches() {
        check(
            ee::s12::f_00389098__generated,
            ps2_graphics_man_set_clear_color,
            80,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x3890, 2);
                for k in 0..3 {
                    let v = match seed % 4 {
                        0 => r.edge_f32(),
                        1 => ((seed % 2) as f32).to_bits(),
                        _ => r.f32(1.5),
                    };
                    w(c, DATA + 0x200 + 4 * k, v);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn irradiance_build_sh_matches() {
        check(
            ee::s12::f_00389c38__generated,
            irradiance_build_sh,
            50,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x389c, 3);
                fill(c, &mut r, DATA, 0x400);
                // The sky's ambient entry (a fixed global) exists for some seeds.
                w(c, 0x0050_47fc, if seed % 3 == 0 { 0 } else { 1 });
                w(c, GAME_GP.wrapping_add(0x12d4), DATA + 0x200);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x300) };
            },
        );
    }

    #[test]
    fn memitter_reset_particles_matches() {
        check(
            ee::s12::f_00371600__generated,
            memitter_reset_particles,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x371600, 0);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x178, seed % 4);
                c.bus.write32(DATA + 0x204, DATA + 0x2000);
                c.bus.write32(DATA + 0x1a0, DATA + 0x3000);
                c.bus.write32(DATA + 0x1a4, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_emit_model_lighting_matches() {
        check(
            ee::s12::f_0037e098__generated,
            ps2_graphics_man_emit_model_lighting,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x37e098, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = sx(DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn rail_tube_ctor_matches() {
        check(
            ee::s12::f_00372520__generated,
            rail_tube_ctor,
            SEEDS_10,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3725, 0);
                env(c);
                w32(c, gpat(0x113c), (seed % 3 == 0) as u32);
                w32(c, gpat(0x10f0), r.next_u32());
                w32(c, gpat(0x10f4), r.next_u32());
                w32(c, gpat(0x10f8), r.next_u32());
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x800);
            },
        );
    }

    #[test]
    fn irradiance_add_scaled_matches() {
        check(
            ee::s12::f_00389590__generated,
            irradiance_add_scaled,
            SEEDS_10,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3895, 0);
                env(c);
                let (a, b, cc) = (DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
                for i in 0..10u32 {
                    for k in 0..4u32 {
                        w32(c, a + 0x10 * i + 4 * k, r.edge_f32());
                        w32(c, b + 0x10 * i + 4 * k, r.edge_f32());
                    }
                }
                for k in 0..4u32 {
                    w32(c, cc + 4 * k, r.edge_f32());
                }
                c.g[A0][0] = sx(a);
                c.g[A1][0] = sx(b);
                c.g[A2][0] = sx(cc);
                c.f[12] = r.f32(4.0);
            },
        );
    }

    pub(crate) const STRIPS: u32 = DATA + 0x2000;

    // ---------------------------------------------------------------- renderer

    #[test]
    fn ps2_graphics_man_draw_shadow_edge_pass_matches() {
        check(
            ee::s12::f_00386dd0__generated,
            ps2_graphics_man_draw_shadow_edge_pass,
            8,
            &|c, seed| {
                let _ = base_6(c, seed, 0x386dd0, 3);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = STRIPS as u64;
                c.g[A2][0] = (1 + seed % 3) as u64;
                c.g[A3][0] = 0x11;
                c.g[T0][0] = 0x22;
                c.g[T1][0] = 0x33;
                for i in 0..3u32 {
                    let s = STRIPS + 0x14 * i;
                    w(c, s, SRC);
                    w(c, s + 4, SRC + 0x100);
                    w(c, s + 8, if (seed + i) % 3 == 0 { 1 } else { 2 });
                    wf(c, s + 0xc, 0.5);
                    wf(c, s + 0x10, 0.75);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_brightness_rgb_matches() {
        check(
            ee::s12::f_0037c570__generated,
            ps2_graphics_man_set_brightness_rgb,
            SEEDS_11,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37c5, 0);
                game_globals(c);
                let this = DATA;
                let rgb = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = rgb as u64;
                match seed % 4 {
                    0 => {
                        c.bus.write32(rgb, 0x3f00_0000);
                        c.bus.write32(rgb + 4, 0x4000_0000);
                        c.bus.write32(rgb + 8, 0xbf80_0000);
                    }
                    1 => {
                        c.bus.write32(rgb, r.edge_f32());
                        c.bus.write32(rgb + 4, r.edge_f32());
                        c.bus.write32(rgb + 8, r.edge_f32());
                    }
                    _ => {
                        c.bus.write32(rgb, r.f32(1.5));
                        c.bus.write32(rgb + 4, r.f32(1.5));
                        c.bus.write32(rgb + 8, r.f32(1.5));
                    }
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_create_grid_mesh_matches() {
        check(
            ee::s12::f_0037a540__generated,
            ps2_graphics_man_create_grid_mesh,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x37a5);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                // No 0x40000000 flag: the constructor alone runs.
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn cps2_grid_mesh_pack_color_matches() {
        strict(
            ee::s12::f_00385aa0__generated,
            cps2_grid_mesh_pack_color,
            SEEDS_12,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x5aa0, 0);
                let colour = DATA + 0x1000;
                let buf = DATA + 0x2000;
                let idx = (r.next_u32() % 8) as u32;
                // clamped and out-of-range channels on both sides
                for k in 0..4 {
                    let v = match (seed as usize + k) % 4 {
                        0 => 0u32,
                        1 => 0x3f80_0000,
                        2 => r.f32(2.0),
                        _ => (r.next_u32() & 0x807f_ffff) | 0x3f00_0000,
                    };
                    w(c, colour + 4 * k as u32, v);
                }
                c.g[A0][0] = sx(DATA + 0x3000);
                c.g[A1][0] = sx(buf);
                c.g[A2][0] = sx(idx);
                c.g[A3][0] = sx(colour);
            },
        );
    }

    #[test]
    fn line_strip_finish_matches() {
        same(
            ssx3_ee::s12::f_00384e50__generated,
            line_strip_finish,
            200,
            &|c, seed| {
                let mut r = Rng(0x384e50 ^ seed as u64);
                generic(c, seed, 0x20);
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(s + 4, DATA + 0x1000);
                c.bus.write32(s + 8, DATA + 0x1400);
                c.bus.write32(s + 0xc, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(s + 0, DATA + 0x1800);
                let m = DATA + 0x1c00;
                c.bus.write32(GAME_GP + 0x2a90, m);
                for off in [0u32, 0x8, 0xe84, 0x18f0, 0x1cc0, 0x1cc4] {
                    c.bus.write32(m + off, (r.next_u32() % 64) as u32);
                }
                for i in 0..0x40u32 {
                    c.bus.write32(DATA + 0x1000 + 4 * i, r.next_u32());
                }
            },
        );
    }
    use crate::difftest::fakes::LOG_COUNT;
    pub(crate) use crate::game::main::ssxapp::tests::f as f_2;
    use ssx3_ee_rt::cpu::{reg::*, sx};
    use ssx3_ee_rt::Cpu;

    // ---------------------------------------------------- patch tessellation

    /// A patch cache with the LOD's basis tables and count, a patch with
    /// control points (all zero for some seeds: zero normals), an output
    /// grid. Returns nothing; sets a0..a3.
    pub(crate) fn tess_setup(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let cache = DATA;
        let patch = DATA + 0x1000;
        let value = DATA + 0x2000;
        let deriv = DATA + 0x3000;
        let out = DATA + 0x8000;
        let lod = r.next_u32() % 3;
        for l in 0..3 {
            c.bus.write32(cache + 0x190 + 4 * l, value + 0x100 * l);
            c.bus.write32(cache + 0x310 + 4 * l, deriv + 0x100 * l);
            let n = match (seed / 3) % 6 {
                0 => 0,
                1 => 0xffff_fffe,
                k => k,
            };
            c.bus.write32(cache + 0x19c + 4 * l, n);
        }
        for i in 0..(3 * 0x40) {
            c.bus.write32(value + 4 * i, f_2(r, 1.0));
            c.bus.write32(deriv + 4 * i, f_2(r, 3.0));
        }
        let zero = seed % 5 == 0;
        for i in 0..64 {
            let v = if zero {
                0
            } else if i % 7 == 0 {
                r.edge_f32()
            } else {
                f_2(r, 50.0)
            };
            c.bus.write32(patch + 0x40 + 4 * i, v);
        }
        fill(c, r, out, 80);
        c.g[A0][0] = sx(cache);
        c.g[A1][0] = sx(patch);
        c.g[A2][0] = sx(out);
        c.g[A3][0] = lod as u64 | (r.next_u32() as u64) << 32;
    }

    #[test]
    fn bezier_patch_cache_tess_positions_matches() {
        check_events(
            ssx3_ee::s12::f_00374180__generated,
            super::bezier_patch_cache_tess_positions,
            240,
            &|c, seed| {
                let mut r = Rng(0x7465_7373 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                tess_setup(c, &mut r, seed);
                schedule(c, &mut r, seed, 0x0037_4180)
            },
        );
    }

    #[test]
    fn bezier_patch_cache_tess_positions_alt_matches() {
        check_events(
            ssx3_ee::s12::f_00374518__generated,
            super::bezier_patch_cache_tess_positions_alt,
            240,
            &|c, seed| {
                let mut r = Rng(0x616c_7474 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                tess_setup(c, &mut r, seed);
                schedule(c, &mut r, seed, 0x0037_4518)
            },
        );
    }
}
