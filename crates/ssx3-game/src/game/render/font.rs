//! `src/render/font.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::gp_7 as gp;
pub(crate) use crate::game::fe::feasyncfile::gp_8 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::p as p_7;
pub(crate) use crate::game::fe::feridermanager::o760_lh;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::object::instancenode::ui_vector_lw;
pub(crate) use crate::game::object::movenode::AVALANCHE_GRMAN_GP;
use crate::game::object::movenode::IDENTITY_MATRIX;
pub(crate) use crate::game::object::railmodifier::reset_vtable_and_delete;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::render::lightman::LIGHT_MAN_INSTANCES;
pub(crate) use crate::game::render::particle::copy_mat;
use crate::game::render::particle::man;
pub(crate) use crate::game::render::particle::p_2 as p;
use crate::game::render::particle::renderer_fields;
#[cfg(test)]
use crate::game::render::particle::GM_FOG_MODE;
#[cfg(test)]
use crate::game::render::particle::GM_LIGHTS;
pub(crate) use crate::game::sound::bankmonitor::o760_add;
pub(crate) use crate::game::sound::bankmonitor::o760_addi;
pub(crate) use crate::game::sound::bankmonitor::o760_ea;
pub(crate) use crate::game::sound::bankmonitor::o760_lb;
pub(crate) use crate::game::sound::bankmonitor::o760_lw;
pub(crate) use crate::game::sound::bankmonitor::o760_sb;
pub(crate) use crate::game::sound::bankmonitor::o760_sw;
pub(crate) use crate::game::sound::bankmonitor::o760_sw0;
pub(crate) use crate::game::sound::bankmonitor::p as p_2;
pub(crate) use crate::game::wscript::wscriptman::p as p_3;
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
    (0x0039_1c48, font_draw_text_f, "Font_DrawTextF"),
    (
        0x0039_29f8,
        font_draw_text_centered_f,
        "Font_DrawTextCenteredF",
    ),
    (0x0039_2a60, font_draw_text_right, "Font_DrawTextRight"),
    (0x0039_2b40, font_draw_text_right_f, "Font_DrawTextRightF"),
    (0x0039_2ba8, font_get_char_width, "Font_GetCharWidth"),
    (0x0039_2c60, font_get_char_width_w, "Font_GetCharWidthW"),
    (0x0039_2d18, fx_trail_reset, "FXTrail_Reset"),
    (0x0039_2d90, fxtrail_add_point, "FXTrail_AddPoint"),
    (
        0x0039_2df0,
        renderer_build_sine_table,
        "Renderer_BuildSineTable",
    ),
    (
        0x0039_3fb8,
        graphics_man_set_screen_offset,
        "GraphicsMan_SetScreenOffset",
    ),
    (
        0x0039_3fd8,
        graphics_man_get_screen_offset_x,
        "GraphicsMan_GetScreenOffsetX",
    ),
    (
        0x0039_3ff0,
        graphics_man_get_screen_offset_y,
        "GraphicsMan_GetScreenOffsetY",
    ),
    (0x0039_42a0, graphics_man_v58, "GraphicsMan_v58"),
    (
        0x0039_4b38,
        grid_mesh_base_base_dtor,
        "GridMeshBase_BaseDtor",
    ),
    (0x0039_4b98, grid_mesh_base_call_v22, "GridMeshBase_CallV22"),
    (
        0x0039_4c98,
        light_man_init_instances,
        "LightMan_InitInstances",
    ),
    (
        0x0039_4ce8,
        light_man_free_instances,
        "LightMan_FreeInstances",
    ),
    (0x0039_4d18, light_man_next_id, "LightMan_NextId"),
    (0x0039_4ed0, defer_man_intern_state, "DeferMan_InternState"),
    (0x0039_5000, defer_man_find_bucket, "DeferMan_FindBucket"),
    (
        0x0039_50c0,
        ps2_graphics_man_copy_view_state,
        "PS2GraphicsMan_CopyViewState",
    ),
    (
        0x0039_5330,
        ps2_graphics_man_get_screen_width,
        "PS2GraphicsMan_GetScreenWidth",
    ),
    (
        0x0039_5350,
        ps2_graphics_man_get_screen_height,
        "PS2GraphicsMan_GetScreenHeight",
    ),
    (
        0x0039_54d0,
        ps2_graphics_man_set_lighting,
        "PS2GraphicsMan_SetLighting",
    ),
    (
        0x0039_56e8,
        ps2_graphics_man_push_matrix,
        "PS2GraphicsMan_PushMatrix",
    ),
    (
        0x0039_5c38,
        ps2_graphics_man_load_matrix,
        "PS2GraphicsMan_LoadMatrix",
    ),
    (
        0x0039_5c70,
        ps2_graphics_man_mult_matrix,
        "PS2GraphicsMan_MultMatrix",
    ),
    (
        0x0039_5cf0,
        ps2_graphics_man_load_identity,
        "PS2GraphicsMan_LoadIdentity",
    ),
    (
        0x0039_5d28,
        ps2_graphics_man_translate,
        "PS2GraphicsMan_Translate",
    ),
    (
        0x0039_5d60,
        ps2_graphics_man_rotate_axis,
        "PS2GraphicsMan_RotateAxis",
    ),
    (
        0x0039_6110,
        c_ps2_grid_mesh_set_scale,
        "cPS2GridMesh_SetScale",
    ),
    (
        0x0039_61a8,
        vfx_render_toggles_dtor,
        "VfxRenderToggles_dtor",
    ),
    (0x0039_64c8, vfx_plants_dtor, "VfxPlants_dtor"),
    (0x0039_6838, vfx_tube_dtor, "VfxTube_dtor"),
    (0x0039_68d0, rail_tube_set_point, "RailTube_SetPoint"),
    (0x0039_68e8, path_arrow_v01, "PathArrow_v01"),
    (0x0039_6928, detail_system_v01, "DetailSystem_v01"),
    (
        0x0039_6a00,
        vfx_visual_effects_test_dtor,
        "VfxVisualEffectsTest_dtor",
    ),
    (0x0039_6d30, vfx_screen_tint_dtor, "VfxScreenTint_dtor"),
    (
        0x0039_6128,
        tex_man_invalidate_bound,
        "TexMan_InvalidateBound",
    ),
    (
        0x0039_5f60,
        ps2_graphics_man_try_acquire_frame,
        "PS2GraphicsMan_TryAcquireFrame",
    ),
    (0x0039_1e30, font_draw_text_w, "Font_DrawTextW"),
    (0x0039_21f0, font_measure_text_w, "Font_MeasureTextW"),
    (0x0039_2430, font_find_line_break, "Font_FindLineBreak"),
    (
        0x0039_5f80,
        ps2_graphics_man_is_idle,
        "PS2GraphicsMan_IsIdle",
    ),
    (0x001d_c990, font_draw_text_varargs, "Font_DrawTextVarargs"),
    (
        0x001d_c9d0,
        font_draw_text_scaled_varargs,
        "Font_DrawTextScaledVarargs",
    ),
    (
        0x001d_ca40,
        font_draw_text_centered_varargs,
        "Font_DrawTextCenteredVarargs",
    ),
    (
        0x001d_ca80,
        font_draw_text_right_varargs,
        "Font_DrawTextRightVarargs",
    ),
    (
        0x0039_3048,
        renderer_static_init_destroy,
        "Renderer_StaticInitDestroy",
    ),
    (0x0039_4c68, light_man_dtor_wrapper, "LightMan_dtor_Wrapper"),
    (0x0039_4c88, light_man_get_instance, "LightMan_GetInstance"),
    (
        0x0039_4d10,
        light_man_reset_counter,
        "LightMan_ResetCounter",
    ),
    (0x0039_4d30, light_man_v08, "LightMan_v08"),
    (0x0039_4d38, light_man_v13, "LightMan_v13"),
    (0x0039_4d40, light_man_v14, "LightMan_v14"),
    (0x0039_4d48, light_man_v15, "LightMan_v15"),
    (
        0x0039_6108,
        c_ps2_grid_mesh_is_valid,
        "cPS2GridMesh_IsValid",
    ),
    (0x0039_70f8, renderer_global_ctors, "Renderer_GlobalCtors"),
    (0x0039_7118, renderer_global_dtors, "Renderer_GlobalDtors"),
    (0x0039_5268, ps2_graphics_man_dtor, "PS2GraphicsMan_dtor"),
    (
        0x0039_5318,
        ps2_graphics_man_get_display_mode_count,
        "PS2GraphicsMan_GetDisplayModeCount",
    ),
    (
        0x0039_5320,
        ps2_graphics_man_is_progressive,
        "PS2GraphicsMan_IsProgressive",
    ),
    (
        0x0039_5328,
        ps2_graphics_man_get_display_mode_index,
        "PS2GraphicsMan_GetDisplayModeIndex",
    ),
    (
        0x0039_5500,
        ps2_graphics_man_get_lighting,
        "PS2GraphicsMan_GetLighting",
    ),
    (0x0039_56e0, ps2_graphics_man_v98, "PS2GraphicsMan_v98"),
    (0x0039_5750, ps2_graphics_man_v33, "PS2GraphicsMan_v33"),
    (0x0039_5fa0, ps2_graphics_man_v68, "PS2GraphicsMan_v68"),
    (0x0039_5288, ps2_graphics_man_ctor, "PS2GraphicsMan_ctor"),
    (
        0x0039_53b0,
        ps2_graphics_man_get_fog_state,
        "PS2GraphicsMan_GetFogState",
    ),
    (
        0x0039_5420,
        ps2_graphics_man_clear_lights,
        "PS2GraphicsMan_ClearLights",
    ),
    (0x0039_5508, ps2_graphics_man_v110, "PS2GraphicsMan_v110"),
    (
        0x0039_5510,
        ps2_graphics_man_get_flip_count,
        "PS2GraphicsMan_GetFlipCount",
    ),
    (
        0x0039_5730,
        ps2_graphics_man_pop_matrix,
        "PS2GraphicsMan_PopMatrix",
    ),
    (
        0x0039_5c68,
        ps2_graphics_man_get_matrix,
        "PS2GraphicsMan_GetMatrix",
    ),
    (0x0039_1788, font_parse_glyphs, "Font_ParseGlyphs"),
    (0x0039_1a48, font_upload_texture, "Font_UploadTexture"),
    (0x0039_4d50, defer_man_ctor, "DeferMan_ctor"),
    (0x0039_7138, ui_engine_store_word, "UIEngine_StoreWord"),
    (
        0x0021_d160,
        font_draw_text_varargs_2,
        "Font_DrawTextVarargs_2",
    ),
    (
        0x0022_23e8,
        font_draw_text_centered_varargs_2,
        "Font_DrawTextCenteredVarargs_2",
    ),
    (0x0039_19e8, font_free, "Font_Free"),
    (0x0039_2680, font_find_line_break_w, "Font_FindLineBreakW"),
    (
        0x0039_2980,
        font_draw_text_centered_w,
        "Font_DrawTextCenteredW",
    ),
    (0x0039_2ad0, font_draw_text_right_w, "Font_DrawTextRightW"),
    (0x0039_2de8, fx_trail_set_flag_a4, "FXTrail_SetFlagA4"),
    (
        0x0039_6160,
        vfx_component_base_dtor,
        "VfxComponent_BaseDtor",
    ),
    (
        0x0039_3fa8,
        graphics_man_set_default_display_mode_00393fa8,
        "GraphicsMan_SetDefaultDisplayMode",
    ),
    (
        0x0039_3fb0,
        graphics_man_is_progressive_00393fb0,
        "GraphicsMan_IsProgressive",
    ),
    (0x0039_4008, graphics_man_v16_00394008, "GraphicsMan_v16"),
    (
        0x0039_4010,
        graphics_man_try_acquire_frame_00394010,
        "GraphicsMan_TryAcquireFrame",
    ),
    (
        0x0039_4018,
        graphics_man_is_idle_00394018,
        "GraphicsMan_IsIdle",
    ),
    (
        0x0039_42c0,
        graphics_man_release_texture_ref_003942c0,
        "GraphicsMan_ReleaseTextureRef",
    ),
    (
        0x0039_42c8,
        graphics_man_draw_transition_003942c8,
        "GraphicsMan_DrawTransition",
    ),
    (0x0039_42d0, graphics_man_v62_003942d0, "GraphicsMan_v62"),
    (
        0x0039_42d8,
        graphics_man_get_color_mask_003942d8,
        "GraphicsMan_GetColorMask",
    ),
    (0x0039_42e0, graphics_man_v65_003942e0, "GraphicsMan_v65"),
    (
        0x0039_45b8,
        graphics_man_clear_lights_003945b8,
        "GraphicsMan_ClearLights",
    ),
    (
        0x0039_45c0,
        graphics_man_add_light_003945c0,
        "GraphicsMan_AddLight",
    ),
    (0x0039_45c8, graphics_man_v74_003945c8, "GraphicsMan_v74"),
    (0x0039_45d0, graphics_man_v88_003945d0, "GraphicsMan_v88"),
    (
        0x0039_45d8,
        graphics_man_draw_shadow_volume_003945d8,
        "GraphicsMan_DrawShadowVolume",
    ),
    (
        0x0039_45e0,
        graphics_man_draw_board_wake_003945e0,
        "GraphicsMan_DrawBoardWake",
    ),
    (0x0039_45e8, graphics_man_v98_003945e8, "GraphicsMan_v98"),
    (
        0x0039_4610,
        graphics_man_draw_skinned_model_shadow_00394610,
        "GraphicsMan_DrawSkinnedModelShadow",
    ),
    (0x0039_4618, graphics_man_v102_00394618, "GraphicsMan_v102"),
    (0x0039_4620, graphics_man_v103_00394620, "GraphicsMan_v103"),
    (0x0039_4628, graphics_man_v104_00394628, "GraphicsMan_v104"),
    (0x0039_4630, graphics_man_v105_00394630, "GraphicsMan_v105"),
    (0x0039_4638, graphics_man_v106_00394638, "GraphicsMan_v106"),
    (0x0039_4640, graphics_man_v107_00394640, "GraphicsMan_v107"),
    (0x0039_4648, graphics_man_v108_00394648, "GraphicsMan_v108"),
    (0x0039_4650, graphics_man_v109_00394650, "GraphicsMan_v109"),
    (0x0039_4658, graphics_man_v110_00394658, "GraphicsMan_v110"),
    (0x0039_4660, graphics_man_v111_00394660, "GraphicsMan_v111"),
    (0x0039_4668, graphics_man_v112_00394668, "GraphicsMan_v112"),
    (0x0039_4670, graphics_man_v113_00394670, "GraphicsMan_v113"),
    (
        0x0039_4678,
        graphics_man_get_flip_count_00394678,
        "GraphicsMan_GetFlipCount",
    ),
    (
        0x0039_4680,
        graphics_man_update_lens_effects_00394680,
        "GraphicsMan_UpdateLensEffects",
    ),
    (
        0x0039_4688,
        graphics_man_add_view_00394688,
        "GraphicsMan_AddView",
    ),
    (
        0x0039_46e8,
        graphics_man_remove_interrupt_handlers_003946e8,
        "GraphicsMan_RemoveInterruptHandlers",
    ),
    (
        0x0039_4b68,
        grid_mesh_base_set_scale_00394b68,
        "GridMeshBase_SetScale",
    ),
    (
        0x0039_4b78,
        bezier_man_base_v04_00394b78,
        "BezierManBase_v04",
    ),
    (
        0x0039_4b80,
        bezier_man_base_v05_00394b80,
        "BezierManBase_v05",
    ),
    (
        0x0039_4b88,
        bezier_man_base_v19_00394b88,
        "BezierManBase_v19",
    ),
    (
        0x0039_4b90,
        bezier_man_base_v20_00394b90,
        "BezierManBase_v20",
    ),
    (
        0x0039_4bc0,
        ps2_bezier_man_set_lod_near_dist_00394bc0,
        "PS2BezierMan_SetLodNearDist",
    ),
    (
        0x0039_4bc8,
        ps2_bezier_man_set_lod_far_dist_00394bc8,
        "PS2BezierMan_SetLodFarDist",
    ),
    (
        0x0039_4bd0,
        ps2_bezier_man_get_lod_near_dist_00394bd0,
        "PS2BezierMan_GetLodNearDist",
    ),
    (
        0x0039_4bd8,
        ps2_bezier_man_get_lod_far_dist_00394bd8,
        "PS2BezierMan_GetLodFarDist",
    ),
    (0x0039_4be0, ps2_bezier_man_v10_00394be0, "PS2BezierMan_v10"),
    (0x0039_4be8, ps2_bezier_man_v11_00394be8, "PS2BezierMan_v11"),
    (0x0039_4bf0, ps2_bezier_man_v12_00394bf0, "PS2BezierMan_v12"),
    (0x0039_4bf8, ps2_bezier_man_v13_00394bf8, "PS2BezierMan_v13"),
    (0x0039_4c00, ps2_bezier_man_v14_00394c00, "PS2BezierMan_v14"),
    (0x0039_4c08, ps2_bezier_man_v15_00394c08, "PS2BezierMan_v15"),
    (0x0039_4c10, ps2_bezier_man_v16_00394c10, "PS2BezierMan_v16"),
    (0x0039_68c8, vfx_tube_render_003968c8, "VfxTube_Render"),
    (0x0039_6918, path_arrow_update_00396918, "PathArrow_Update"),
    (0x0039_6920, path_arrow_render_00396920, "PathArrow_Render"),
    (0x0039_6958, detail_system_v05_00396958, "DetailSystem_v05"),
    (
        0x0039_6b40,
        ps2_graphics_man_emit_lighting_packet,
        "PS2GraphicsMan_EmitLightingPacket",
    ),
];

/// `DeferMan` layout (offsets from the `DeferMan*`).
pub(crate) mod defer {

    /// Bucket chain heads, one word per pass (`FindBucket`).
    pub const BUCKETS: u32 = 0x6_78a0;

    /// State record chain heads, one word per pass (`InternState`).
    pub const STATE_HEADS: u32 = 0x6_74a0;

    /// The state record pool: `+0x1480` count, records from `+0x1484`.
    pub const STATE_POOL: u32 = 0x5_0000;

    pub const STATE_POOL_COUNT: u32 = 0x1480;

    pub const STATE_POOL_RECORDS: u32 = 0x5_1484;

    pub const STATE_RECORD_SIZE: u32 = 0x18;

    /// Bucket node: keys at +0/+4/+8, "not a leaf" at +0x14, next at +0x1c.
    pub const NODE_SKIP: u32 = 0x14;

    pub const NODE_NEXT: u32 = 0x1c;

    /// State record: four compared words, a fifth, next at +0x14.
    pub const STATE_NEXT: u32 = 0x14;
}

/// One saved view (`sPtView`).
pub(crate) mod vw {

    pub(crate) use crate::game::render::particle::vw::KEY;
    pub(crate) use crate::game::render::particle::vw::M120;
    pub(crate) use crate::game::render::particle::vw::M180;
    pub(crate) use crate::game::render::particle::vw::M1C0;
    pub(crate) use crate::game::render::particle::vw::M60;
    pub(crate) use crate::game::render::particle::vw::MA0;
    pub(crate) use crate::game::render::particle::vw::ME0;
    pub(crate) use crate::game::render::particle::vw::Q160;
    pub(crate) use crate::game::render::particle::vw::Q170;
    pub(crate) use crate::game::render::particle::vw::Q200;
    pub(crate) use crate::game::render::particle::vw::Q210;
    pub(crate) use crate::game::render::particle::vw::Q30;
    pub(crate) use crate::game::render::particle::vw::Q40;
    pub(crate) use crate::game::render::particle::vw::Q50;
    pub(crate) use crate::game::render::particle::vw::SCISSOR;
}

/// `FXTrail` (font flash trail) fields: the count, the `{x,y,z}` cursor, and
/// the two endpoint times.
pub(crate) mod trail {

    pub const COUNT: u32 = 0x8;

    pub const START: u32 = 0xa0;

    pub const END: u32 = 0xa4;
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    use super::{ee, Callee};
    pub(crate) use crate::game::fe::festateload::callee::EA_MEM_FREE;

    pub(crate) use crate::game::lib::sif::callee::MEMSET;

    pub(crate) use crate::game::render::lightman::callee::LIGHT_MAN_DTOR;

    pub(crate) use crate::game::render::particle::callee::RENDERER_STATIC_INIT_DESTROY;

    /// `Rand_GetDefaultState(state)`: copy the default random state.
    pub(crate) const RAND_GET_DEFAULT_STATE: Callee = Callee {
        addr: 0x0031_7930,
        f: ee::s10::f_00317930,
    };

    /// `SprStream_SetMode(stream, mode)`.
    pub(crate) const SPR_STREAM_SET_MODE: Callee = Callee {
        addr: 0x0038_f300,
        f: ee::s12::f_0038f300,
    };

    /// `Renderer_BuildSineTable(table)`.
    pub(crate) const RENDERER_BUILD_SINE_TABLE: Callee = Callee {
        addr: 0x0039_2df0,
        f: ee::s13::f_00392df0,
    };
    pub(crate) use crate::game::render::lightman::callee::MATH_SIN_COS;

    /// `GraphicsMan_dtor` (0x3695d8, s12): the base graphics manager
    /// destructor.
    pub(crate) const GRAPHICS_MAN_DTOR: Callee = Callee {
        addr: 0x0036_95d8,
        f: ee::s12::GraphicsMan_dtor,
    };

    /// `void Gs_UploadImage(...)`: uploads a pixel rectangle to the GS.
    pub(crate) const GS_UPLOAD_IMAGE: Callee = Callee {
        addr: 0x0038_3808,
        f: ee::s12::Gs_UploadImage,
    };

    /// `float Font_MeasureTextW(...)`: measures a wide string.
    pub(crate) const FONT_MEASURE_TEXT_W: Callee = Callee {
        addr: 0x0039_21f0,
        f: ee::s12::Font_MeasureTextW,
    };

    /// `void Font_DrawTextW(...)`: draws a wide string.
    pub(crate) const FONT_DRAW_TEXT_W: Callee = Callee {
        addr: 0x0039_1e30,
        f: ee::s12::Font_DrawTextW,
    };

    /// `void EA_MemFree(void *)`: frees a heap block.
    pub(crate) const EA_MEM_FREE_2: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::EA_MemFree,
    };
}

/// `gp`-relative globals used here.
pub(crate) mod g {}

// ---------------------------------------------------------------------------
// Deferred draw buckets.

/// `DeferManBucket *DeferMan_FindBucket(DeferMan *man, uint hash, uint key1,
/// uint pass, uint key2)` (0x395000): walk the chain of pass `pass & 0xff`
/// for the leaf bucket with keys `(hash, key1, key2)`; a hit is moved to the
/// head of the chain and returned, otherwise 0. Nodes with +0x14 set are
/// never matched.
pub fn defer_man_find_bucket(c: &mut Cpu) {
    c.cycles += 9;
    let man = abi::arg(c, 0);
    let slot_off = (abi::arg(c, 3) & 0xff) << 2;
    let head_slot = man.wrapping_add(defer::BUCKETS + slot_off);
    c.g[A3][0] = slot_off as u64;
    c.g[T3][0] = sx(head_slot);
    if Addr(head_slot).lw(c) == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[T4][0] = 1;
    c.g[A0][0] = sx(man.wrapping_add(defer::BUCKETS));
    loop {
        // t3: the link being followed; a1/a2/t0: the keys.
        let link = r32(c, T3);
        c.cycles += 5;
        let node = Addr(link).lw(c);
        let n = Addr(node as u32);
        let skip = n.off(defer::NODE_SKIP).lw(c);
        // (v1 as the compare chain leaves it, hit)
        let (v1, hit) = if skip != 0 {
            (node, false)
        } else {
            let key1 = n.off(4).lw(c);
            if key1 != c.g[A2][0] {
                c.cycles += 5;
                (0, false)
            } else {
                let key2 = n.off(8).lw(c);
                if key2 != c.g[T0][0] {
                    c.cycles += 7;
                    (0, false)
                } else {
                    c.cycles += 12;
                    let same = n.lw(c) == c.g[A1][0];
                    (same as u64, same)
                }
            }
        };
        c.cycles += 2;
        c.g[V1][0] = v1;
        c.g[T1][0] = node;
        c.g[T2][0] = hit as u64;
        if !hit {
            let next = n.off(defer::NODE_NEXT).lw(c);
            c.cycles += 2;
            c.g[T3][0] = sx((node as u32).wrapping_add(defer::NODE_NEXT));
            if next != 0 {
                if event_due(c) {
                    c.g[V0][0] = next;
                    deliver_event_here(c, 0x0039_5030);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        // Hit: unlink it and push it on the head of the chain.
        c.cycles += 4;
        let head_slot = r32(c, A0).wrapping_add(r32(c, A3));
        c.g[A3][0] = sx(head_slot);
        if node != Addr(head_slot).lw(c) {
            let next = n.off(defer::NODE_NEXT).lw(c);
            c.cycles += 6;
            c.g[A0][0] = next;
            Addr(link).sw(c, next);
            let old_head = Addr(head_slot).lw(c);
            c.g[V1][0] = old_head;
            n.off(defer::NODE_NEXT).sw(c, old_head);
            Addr(head_slot).sw(c, node);
        } else {
            c.cycles += 2;
        }
        c.g[V0][0] = node;
        abi::ret(c);
        return;
    }
}

/// `DeferStateRecord *DeferMan_InternState(DeferMan *man, const u32 state[5],
/// int pass)` (0x394ed0): share equal draw states. Look in the chain of
/// pass `pass & 0xff` for a record whose first four words equal `state`;
/// a hit is moved to the head and returned. Otherwise take the next pool
/// record, copy the five words into it, push it on the chain and return it.
pub fn defer_man_intern_state(c: &mut Cpu) {
    c.cycles += 10;
    let man = abi::arg(c, 0);
    let pass = c.g[A2][0] & 0xff;
    let slot_off = (pass as u32) << 2;
    let first_link = man.wrapping_add(defer::STATE_HEADS + slot_off);
    c.g[A3][0] = pass;
    c.g[V1][0] = slot_off as u64;
    c.g[T2][0] = c.g[A0][0];
    c.g[T3][0] = c.g[A1][0];
    c.g[T1][0] = sx(first_link);
    if Addr(first_link).lw(c) != 0 {
        c.cycles += 4;
        c.g[T5][0] = slot_off as u64;
        c.g[T4][0] = sx(man.wrapping_add(defer::STATE_HEADS));
        if intern_state_find(c) {
            return;
        }
    }
    intern_state_insert(c);
}

/// The chain walk of `DeferMan_InternState`: true when it found (and
/// returned) a record. Registers: t1 the link, t3 the state, t4 + t5 the
/// head slot.
pub(crate) fn intern_state_find(c: &mut Cpu) -> bool {
    loop {
        c.cycles += 4;
        let link = r32(c, T1);
        let rec = Addr(link).lw(c);
        c.g[A2][0] = rec;
        // Compare the four state words: a0/a1 walk record and state, t0
        // counts (both cursors stop on a difference).
        let mut rec_at = rec;
        let mut state_at = c.g[T3][0];
        let mut count = 0u64;
        let mut word;
        let same = loop {
            c.cycles += 4;
            word = Addr(rec_at as u32).lw(c);
            let want = Addr(state_at as u32).lw(c);
            count = sx((count as u32).wrapping_add(1));
            if word != want {
                c.cycles += 2;
                break false;
            }
            c.cycles += 4;
            state_at = sx((state_at as u32).wrapping_add(4));
            rec_at = sx((rec_at as u32).wrapping_add(4));
            if count < 4 {
                if event_due(c) {
                    c.g[V0][0] = 1;
                    c.g[V1][0] = word;
                    c.g[T0][0] = count;
                    c.g[A0][0] = rec_at;
                    c.g[A1][0] = state_at;
                    deliver_event_here(c, 0x0039_4f18);
                    (count, rec_at, state_at) = (c.g[T0][0], c.g[A0][0], c.g[A1][0]);
                }
                continue;
            }
            c.cycles += 1;
            break true;
        };
        c.cycles += 2;
        let head_slot = r32(c, T4).wrapping_add(r32(c, T5));
        c.g[V1][0] = word;
        c.g[T0][0] = count;
        c.g[A0][0] = rec_at;
        c.g[A1][0] = sx(head_slot);
        let r = Addr(rec as u32);
        if !same {
            c.cycles += 3;
            let next = r.off(defer::STATE_NEXT).lw(c);
            c.g[V0][0] = next;
            c.g[T1][0] = sx((rec as u32).wrapping_add(defer::STATE_NEXT));
            if next != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0039_4f08);
                }
                continue;
            }
            return false;
        }
        // Hit: move it to the head of the chain.
        c.cycles += 3;
        if rec != Addr(head_slot).lw(c) {
            let next = r.off(defer::STATE_NEXT).lw(c);
            c.cycles += 6;
            c.g[A0][0] = next;
            Addr(link).sw(c, next);
            let old_head = Addr(head_slot).lw(c);
            c.g[V1][0] = old_head;
            r.off(defer::STATE_NEXT).sw(c, old_head);
            Addr(head_slot).sw(c, rec);
        } else {
            c.cycles += 2;
        }
        c.g[V0][0] = rec;
        abi::ret(c);
        return true;
    }
}

/// The miss path of `DeferMan_InternState`: take pool record `count++`,
/// copy the five state words (the first four as two unaligned doublewords:
/// records are 0x18 bytes from +0x51484), push it on the chain.
pub(crate) fn intern_state_insert(c: &mut Cpu) {
    c.cycles += 31;
    let man = r32(c, T2);
    let state = r32(c, T3);
    let pool = man.wrapping_add(defer::STATE_POOL);
    let count_at = pool.wrapping_add(defer::STATE_POOL_COUNT);
    let count = Addr(count_at).lw(c);
    let rec_off = abi::mult(c, count as u32, defer::STATE_RECORD_SIZE);
    let slot_off = r32(c, A3) << 2;
    Addr(count_at).sw(c, sx((count as u32).wrapping_add(1)));
    let head_slot = man.wrapping_add(defer::STATE_HEADS).wrapping_add(slot_off);
    let rec = man.wrapping_add(rec_off.wrapping_add(defer::STATE_POOL_RECORDS));
    let s = Addr(state);
    let words01 = s.read_u64_unaligned(c);
    let words23 = s.off(8).read_u64_unaligned(c);
    let word4 = s.off(0x10).lw(c);
    Addr(rec).write_u64_unaligned(c, words01);
    Addr(rec).off(8).write_u64_unaligned(c, words23);
    Addr(rec).off(0x10).sw(c, word4);
    let old_head = Addr(head_slot).lw(c);
    Addr(rec).off(defer::STATE_NEXT).sw(c, old_head);
    Addr(head_slot).sw(c, sx(rec));
    c.g[V0][0] = sx(rec);
    c.g[V1][0] = old_head;
    c.g[A0][0] = sx(head_slot);
    c.g[A1][0] = sx(rec);
    c.g[A2][0] = word4;
    c.g[A3][0] = sx(slot_off);
    abi::ret(c);
}

/// `lq gpr, from; sq gpr, to` (a quadword moved through a GPR).
#[inline(always)]
pub(crate) fn copy_q(c: &mut Cpu, gpr: usize, from: u32, to: u32) {
    c.lq(gpr, from);
    c.sq(gpr, to);
}

/// `void PS2GraphicsMan_CopyViewState(sPtView* dst, const sPtView* src)`
/// (0x3950c0): copy one saved view (scissor/shorts, floats, matrices, quads,
/// key). Returns `dst` in v0 and the copied key in v1.
pub fn ps2_graphics_man_copy_view_state(c: &mut Cpu) {
    c.cycles += 106;
    let dst = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    c.g[A3][0] = sx(p(dst, vw::M60));
    c.g[A2][0] = sx(p(src, vw::M60));
    for &(off, fpr) in &[
        (0x0u32, 1usize),
        (0x4, 0),
        (0x8, 1),
        (0xc, 0),
        (0x10, 1),
        (0x14, 0),
        (0x18, 1),
        (0x1c, 0),
    ] {
        c.lwc1(fpr, p(src, off));
        c.swc1(fpr, p(dst, off));
    }
    c.g[V0][0] = Addr(src).off(0x20).lw(c);
    c.sw(p(dst, 0x20), c.g[V0][0]);
    for &(off, fpr) in &[(0x24u32, 0usize), (0x28, 1), (0x2c, 0)] {
        c.lwc1(fpr, p(src, off));
        c.swc1(fpr, p(dst, off));
    }
    copy_q(c, 2, p(src, vw::Q30), p(dst, vw::Q30));
    copy_q(c, 3, p(src, vw::Q40), p(dst, vw::Q40));
    copy_q(c, 2, p(src, vw::Q50), p(dst, vw::Q50));
    copy_mat(c, c.g[A2][0] as u32, c.g[A3][0] as u32);
    c.g[V1][0] = sx(p(dst, vw::MA0));
    c.g[V0][0] = sx(p(src, vw::MA0));
    copy_mat(c, c.g[V0][0] as u32, c.g[V1][0] as u32);
    c.g[A2][0] = sx(p(dst, vw::ME0));
    c.g[V0][0] = sx(p(src, vw::ME0));
    copy_mat(c, c.g[V0][0] as u32, c.g[A2][0] as u32);
    c.g[A3][0] = sx(p(dst, vw::M120));
    c.g[V0][0] = sx(p(src, vw::M120));
    copy_mat(c, c.g[V0][0] as u32, c.g[A3][0] as u32);
    c.lq(3, p(src, vw::Q160));
    c.g[A3][0] = sx(p(dst, vw::M180));
    c.g[A2][0] = sx(p(src, vw::M180));
    c.sq(3, p(dst, vw::Q160));
    c.lq(2, p(src, vw::Q170));
    c.sq(2, p(dst, vw::Q170));
    copy_mat(c, c.g[A2][0] as u32, c.g[A3][0] as u32);
    c.g[V1][0] = sx(p(dst, vw::M1C0));
    c.g[V0][0] = sx(p(src, vw::M1C0));
    copy_mat(c, c.g[V0][0] as u32, c.g[V1][0] as u32);
    c.lq(6, p(src, vw::Q200));
    c.g[V0][0] = c.g[A0][0];
    c.sq(6, p(dst, vw::Q200));
    c.lq(3, p(src, vw::Q210));
    c.sq(3, p(dst, vw::Q210));
    c.ldl(3, p(src, vw::SCISSOR + 7));
    c.ldr(3, p(src, vw::SCISSOR));
    c.sdl(3, p(dst, vw::SCISSOR + 7));
    c.sdr(3, p(dst, vw::SCISSOR));
    c.g[V1][0] = Addr(src).off(vw::KEY).lw(c);
    c.sw(p(dst, vw::KEY), c.g[V1][0]);
    c.pc = c.g[RA][0] as u32;
}

// ---------------------------------------------------------------------------
// Font / flash trails.

/// `void FXTrail_Reset(FXTrail *trail)` (0x392d18): clear the trail's count,
/// its `{x,y,z}` cursor and the two endpoint times.
pub fn fx_trail_reset(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x10);
    // The original builds a zeroed {x,y,z} on the stack and moves it to the
    // trail as one doubleword; keep the same stack traffic and registers.
    let tmp = frame.at(c, 0);
    Addr(tmp).sw(c, 0);
    Addr(tmp + 4).sw(c, 0);
    Addr(tmp + 8).sw(c, 0);
    let trail = abi::arg(c, 0);
    // (+0xc cleared too, before the copy.)
    c.sw(trail.wrapping_add(0xc), 0);
    c.ldl(V0, tmp + 7);
    c.ldr(V0, tmp);
    c.g[V1][0] = Addr(tmp + 8).lw(c);
    c.sdl(V0, trail.wrapping_add(7));
    c.sdr(V0, trail);
    c.sw(trail.wrapping_add(trail::COUNT), c.g[V1][0]);
    c.sw(trail.wrapping_add(trail::END), 0);
    c.sw(trail.wrapping_add(trail::START), 0);
    frame.close(c);
}

/// `void GridMeshBase_CallV22(GridMeshBase* self, ...)` (0x394b98): call slot
/// 0x16 of the mesh object at `+0x4` with the remaining arguments.
pub fn grid_mesh_base_call_v22(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[T2][0] = Addr(at(abi::arg(c, 0), 4)).lw(c);
        let obj = c.g[T2][0] as u32;
        c.g[V1][0] = Addr(at(obj, 0xb0)).lh(c);
        c.g[V0][0] = Addr(at(obj, 0xb4)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0039_4bb4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void LightMan_FreeInstances(LightMan* self)` (0x394ce8): free the
/// instance array at `+0x4`, if any.
pub fn light_man_free_instances(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 4)).lw(c);
        let list = c.g[A0][0];
        frame.restore_ra(c, 0);
        if list != 0 {
            c.cycles += 3;
            call_ptr(c, callee::EA_MEM_FREE, 0x0039_4d04)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `GridMeshBase` base-object destroying destructor (0x394b38): the base
/// vtable is the second word of the object.
pub fn grid_mesh_base_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_4b5c, 0x0049_3938, 0x4)
    })
}

/// `VfxRenderToggles` destroying destructor (0x3961a8).
pub fn vfx_render_toggles_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_61cc, 0x0048_8680, 0)
    })
}

/// `VfxPlants` destroying destructor (0x3964c8).
pub fn vfx_plants_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_64ec, 0x0048_8680, 0)
    })
}

/// `VfxTube` destroying destructor (0x396838).
pub fn vfx_tube_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_685c, 0x0048_8680, 0)
    })
}

/// `PathArrow` destroying destructor (0x3968e8).
pub fn path_arrow_v01(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_690c, 0x0048_8680, 0)
    })
}

/// `DetailSystem` destroying destructor (0x396928).
pub fn detail_system_v01(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_694c, 0x0048_8680, 0)
    })
}

/// `VfxVisualEffectsTest` destroying destructor (0x396a00).
pub fn vfx_visual_effects_test_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_6a24, 0x0048_8680, 0)
    })
}

/// `VfxScreenTint` destroying destructor (0x396d30).
pub fn vfx_screen_tint_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0039_6d54, 0x0048_8680, 0)
    })
}

// ---------------------------------------------------------------------------
// FX trail.

/// `void FXTrail_AddPoint(FXTrail* t, const Quat* rot, const Vec* pos)`
/// (0x392d90): append one point — 12 bytes at `t+0x10+12*count` and the
/// quaternion at `t+0x50+16*count` — then bump the count. 19 cycles.
pub fn fxtrail_add_point(c: &mut Cpu) {
    c.cycles += 19;
    let t = r32(c, A0);
    c.g[V0][0] = c.lw(p_2(t, 0xc));
    c.g[V1][0] = 0xc;
    c.mult(0, A3, V0, V1);
    c.g[V0][0] = sx(r32(c, A3).wrapping_add(t));
    let pos = r32(c, A2);
    c.ldl(8, p_2(pos, 7));
    c.ldr(8, pos);
    c.g[T1][0] = c.lw(p_2(pos, 8));
    let slot = r32(c, V0);
    c.sdl(8, p_2(slot, 0x17));
    c.sdr(8, p_2(slot, 0x10));
    c.sw(p_2(slot, 0x18), c.g[T1][0]);
    c.g[V1][0] = c.lw(p_2(t, 0xc));
    let rot = r32(c, A1);
    c.lq(6, rot);
    c.g[V1][0] = sx(r32(c, V1) << 4);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(t));
    c.sq(6, p_2(r32(c, V1), 0x50));
    c.g[V0][0] = c.lw(p_2(r32(c, A0), 0xc));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    c.sw(p_2(t, 0xc), c.g[V0][0]);
    abi::ret(c);
}

/// `void LightMan_InitInstances(LightMan* self, int n)` (0x394c98): allocate
/// the `n + 0x32` instance table.
pub fn light_man_init_instances(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[V1][0] = sx(r32(c, V0).wrapping_add(0x32));
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p_3(this, 0xc)).sw(c, 0);
        c.g[A1][0] = 0x49_1fa0; // D_00491FA0
        Addr(p_3(this, 0x10)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(r32(c, V1) << 2);
        Addr(p_3(this, 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x0039_4cd4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0039_4cd4)?;
        Addr(p_3(this, 4)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Graphics manager / font helpers.

/// `void GraphicsMan_SetScreenOffset(self, x, y)` (0x393fb8): store the
/// integer arguments as the screen-offset floats.
pub fn graphics_man_set_screen_offset(c: &mut Cpu) {
    c.cycles += 7;
    c.mtc1(5, 0);
    c.cvt_s_w(0, 0);
    c.mtc1(6, 1);
    c.cvt_s_w(1, 1);
    let this = Addr(abi::arg(c, 0));
    c.swc1(0, this.0);
    c.swc1(1, this.off(0x4).0);
    abi::ret(c);
}

/// `int GraphicsMan_GetScreenOffsetX(self)` (0x393fd8): truncate the float at
/// `+0`.
pub fn graphics_man_get_screen_offset_x(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    c.lwc1(0, this.0);
    c.cvt_w_s(1, 0);
    c.mfc1(2, 1);
    abi::ret(c);
}

/// `int GraphicsMan_GetScreenOffsetY(self)` (0x393ff0): truncate the float at
/// `+4`.
pub fn graphics_man_get_screen_offset_y(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    c.lwc1(0, this.off(0x4).0);
    c.cvt_w_s(1, 0);
    c.mfc1(2, 1);
    abi::ret(c);
}

/// `self GraphicsMan_v58(self)` (0x3942a0): initialise a colour/scale object
/// to `{1.0f, 1.0f, 0, 1.0f}` and return it.
pub fn graphics_man_v58(c: &mut Cpu) {
    c.cycles += 8;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.g[V0][0] = c.g[A0][0];
    let this = Addr(abi::arg(c, 0));
    this.off(0x8).sw(c, 0);
    c.swc1(0, this.off(0xc).0);
    c.swc1(0, this.0);
    c.swc1(0, this.off(0x4).0);
    abi::ret(c);
}

/// `void* LightMan_NextId(self)` (0x394d18): `base + index` and increment the
/// index (`+0xc`).
pub fn light_man_next_id(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0xc).lw(c);
    c.g[V0][0] = this.off(0x8).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    let next = c.g[V1][0];
    this.off(0xc).sw(c, next);
    abi::ret(c);
}

/// `int PS2GraphicsMan_GetScreenWidth(self)` (0x395330): the current font
/// entry's `+0x88` (entry size 0xa4).
pub fn ps2_graphics_man_get_screen_width(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0x59c8).lw(c);
    c.g[A1][0] = 0xa4;
    c.g[V0][0] = this.off(0x59cc).lw(c);
    c.mult(0, V1, V1, A1);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x88).lw(c);
    abi::ret(c);
}

/// `int PS2GraphicsMan_GetScreenHeight(self)` (0x395350): the current font
/// entry's `+0x8c`.
pub fn ps2_graphics_man_get_screen_height(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0x59c8).lw(c);
    c.g[A1][0] = 0xa4;
    c.g[V0][0] = this.off(0x59cc).lw(c);
    c.mult(0, V1, V1, A1);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x8c).lw(c);
    abi::ret(c);
}

/// `void PS2GraphicsMan_SetLighting(self, src)` (0x3954d0): copy the 0xa0-byte
/// lighting block to `self + 0x6bb0`, a quadword at a time.
pub fn ps2_graphics_man_set_lighting(c: &mut Cpu) {
    c.cycles += 2;
    let dst = (c.g[A0][0] as u32).wrapping_add(0x6bb0);
    c.g[A0][0] = sx(dst);
    let end = (c.g[A1][0] as u32).wrapping_add(0xa0);
    c.g[V0][0] = sx(end);
    loop {
        // Reload the loop state from the registers: an event delivered at
        // the back edge may have switched the guest thread.
        let src = c.g[A1][0] as u32;
        let dst = c.g[A0][0] as u32;
        c.cycles += 7;
        c.lq(3, src);
        c.lq(6, src.wrapping_add(0x10));
        c.sq(3, dst);
        c.sq(6, dst.wrapping_add(0x10));
        c.g[A1][0] = sx(src.wrapping_add(0x20));
        let more = c.g[A1][0] != c.g[V0][0];
        c.g[A0][0] = sx(dst.wrapping_add(0x20));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_54d8);
        }
    }
}

/// `void PS2GraphicsMan_PushMatrix(self)` (0x3956e8): copy the current matrix
/// to `top + 0x40` and advance the stack (`+0x13e4`) and depth (`+0x13e0`).
pub fn ps2_graphics_man_push_matrix(c: &mut Cpu) {
    c.cycles += 17;
    let this = Addr(abi::arg(c, 0));
    c.g[A1][0] = this.off(0x13e4).lw(c);
    let src = c.g[A1][0] as u32;
    c.g[V0][0] = sx(src.wrapping_add(0x40));
    let dst = c.g[V0][0] as u32;
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
    c.g[V1][0] = this.off(0x13e4).lw(c);
    c.g[V0][0] = this.off(0x13e0).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    let top = c.g[V1][0];
    this.off(0x13e4).sw(c, top);
    let depth = c.g[V0][0];
    this.off(0x13e0).sw(c, depth);
    abi::ret(c);
}

/// `void PS2GraphicsMan_LoadMatrix(self, m)` (0x395c38): copy `m` over the
/// current matrix and drop the cached matrix flag (`+0x6b90`).
pub fn ps2_graphics_man_load_matrix(c: &mut Cpu) {
    c.cycles += 11;
    let this = Addr(abi::arg(c, 0));
    let m = abi::arg(c, 1) as u32;
    c.g[V0][0] = this.off(0x13e4).lw(c);
    let dst = c.g[V0][0] as u32;
    c.lqc2(1, m);
    c.lqc2(2, m.wrapping_add(0x10));
    c.lqc2(3, m.wrapping_add(0x20));
    c.lqc2(4, m.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
    this.off(0x6b90).sw(c, 0);
    abi::ret(c);
}

/// `void PS2GraphicsMan_MultMatrix(self, m)` (0x395c70): current = current * m
/// (four VU0 dot products).
pub fn ps2_graphics_man_mult_matrix(c: &mut Cpu) {
    c.cycles += 31;
    let this = Addr(abi::arg(c, 0));
    let m = abi::arg(c, 1) as u32;
    c.g[V0][0] = this.off(0x13e4).lw(c);
    let dst = c.g[V0][0] as u32;
    c.lqc2(4, dst);
    c.lqc2(5, dst.wrapping_add(0x10));
    c.lqc2(6, dst.wrapping_add(0x20));
    c.lqc2(7, dst.wrapping_add(0x30));
    c.lqc2(8, m);
    c.lqc2(9, m.wrapping_add(0x10));
    c.lqc2(10, m.wrapping_add(0x20));
    c.lqc2(11, m.wrapping_add(0x30));
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
    c.sqc2(12, dst);
    c.sqc2(13, dst.wrapping_add(0x10));
    c.sqc2(14, dst.wrapping_add(0x20));
    c.sqc2(15, dst.wrapping_add(0x30));
    this.off(0x6b90).sw(c, 0);
    abi::ret(c);
}

/// `void PS2GraphicsMan_LoadIdentity(self)` (0x395cf0): copy the identity
/// matrix at `0x4ff1a0` over the current matrix.
pub fn ps2_graphics_man_load_identity(c: &mut Cpu) {
    c.cycles += 13;
    c.g[V0][0] = 0x50_0000;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0x13e4).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
    let identity = c.g[V0][0] as u32;
    let dst = c.g[V1][0] as u32;
    c.lqc2(1, identity);
    c.lqc2(2, identity.wrapping_add(0x10));
    c.lqc2(3, identity.wrapping_add(0x20));
    c.lqc2(4, identity.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
    this.off(0x6b90).sw(c, 0);
    abi::ret(c);
}

/// `void PS2GraphicsMan_Translate(self, v)` (0x395d28): translate the current
/// matrix by the vector at `v` (VU0 dot product into row 3).
pub fn ps2_graphics_man_translate(c: &mut Cpu) {
    c.cycles += 13;
    let this = Addr(abi::arg(c, 0));
    let v = abi::arg(c, 1) as u32;
    c.g[V0][0] = this.off(0x13e4).lw(c);
    let dst = c.g[V0][0] as u32;
    c.lqc2(4, dst);
    c.lqc2(5, dst.wrapping_add(0x10));
    c.lqc2(6, dst.wrapping_add(0x20));
    c.lqc2(7, dst.wrapping_add(0x30));
    c.lqc2(8, v);
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, dst.wrapping_add(0x30));
    this.off(0x6b90).sw(c, 0);
    abi::ret(c);
}

/// `void cPS2GridMesh_SetScale(self, range)` (0x396110): store the range and
/// the reciprocal scale `32767 / range`.
pub fn c_ps2_grid_mesh_set_scale(c: &mut Cpu) {
    c.cycles += 5;
    let gp = c.g[GP][0] as u32;
    c.lwc1(0, gp.wrapping_sub(0x254c));
    let this = Addr(abi::arg(c, 0));
    c.swc1(12, this.off(0x194).0);
    c.div_s(0, 0, 12);
    c.swc1(0, this.off(0x198).0);
    abi::ret(c);
}

/// `void RailTube_SetPoint(self, index, point)` (0x3968d0): copy the quadword
/// `point` into the tube's point array (`ptr + index * 0x10`).
pub fn rail_tube_set_point(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(0x8).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
    let point = abi::arg(c, 2) as u32;
    c.lq(3, point);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let dst = c.g[A1][0] as u32;
    c.sq(3, dst);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Fonts and HUD.

/// `void Font_DrawTextF(void* font, float x, float y, const char* fmt, ...)`
/// (0x391c48).
pub fn font_draw_text_f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(S0, p_4(c.g[SP][0] as u32, 0x20));
        c.sq(S1, p_4(c.g[SP][0] as u32, 0x10));
        c.g[S0][0] = 0x0050_0000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x0ca0));
        c.swc1(21, p_4(c.g[SP][0] as u32, 0x38));
        c.swc1(20, p_4(c.g[SP][0] as u32, 0x30));
        c.g[S1][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.mov_s(20, 12);
        c.mov_s(21, 13);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_86c8, 0x0039_1c7c)?;
        c.g[A0][0] = c.g[S1][0];
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0039_1cb0, 0x0039_1c90)?;
        c.lq(S0, p_4(c.g[SP][0] as u32, 0x20));
        c.lq(S1, p_4(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(21, p_4(c.g[SP][0] as u32, 0x38));
        c.lwc1(20, p_4(c.g[SP][0] as u32, 0x30));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `Font_DrawTextCenteredF(font, fmt, ap, x, y)` 0x3929f8: format and draw a
/// centred string.
pub fn font_draw_text_centered_f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        let font = abi::arg64(c, 0);
        let fmt = abi::arg64(c, 1);
        let ap = abi::arg64(c, 2);
        c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
        frame.save(c, S0, 0x20);
        c.swc1(21, p_5(sp(c), 0x38));
        c.g[S0][0] = font;
        c.swc1(20, p_5(sp(c), 0x30));
        c.g[A0][0] = sp(c) as u64;
        frame.save_ra(c, 0x10);
        c.mov_s(20, 12);
        c.mov_s(21, 13);
        c.sw(sp(c), c.g[V0][0]);
        c.g[A1][0] = fmt;
        c.g[A2][0] = ap;
        call_ptr(c, 0x0031_91c0, 0x0039_2a28)?;
        c.g[A1][0] = Addr(sp(c)).lw(c);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_2908, 0x0039_2a3c)?;
        c.g[A0][0] = sp(c) as u64;
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0039_2a48)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        c.lwc1(21, p_5(sp(c), 0x38));
        c.lwc1(20, p_5(sp(c), 0x30));
        frame.close(c);
        Ok(())
    })
}

/// `Font_DrawTextRightF(font, fmt, ap, x, y)` 0x392b40: format and draw a
/// right-aligned string.
pub fn font_draw_text_right_f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        let font = abi::arg64(c, 0);
        let fmt = abi::arg64(c, 1);
        let ap = abi::arg64(c, 2);
        c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
        frame.save(c, S0, 0x20);
        c.swc1(21, p_5(sp(c), 0x38));
        c.g[S0][0] = font;
        c.swc1(20, p_5(sp(c), 0x30));
        c.g[A0][0] = sp(c) as u64;
        frame.save_ra(c, 0x10);
        c.mov_s(20, 12);
        c.mov_s(21, 13);
        c.sw(sp(c), c.g[V0][0]);
        c.g[A1][0] = fmt;
        c.g[A2][0] = ap;
        call_ptr(c, 0x0031_91c0, 0x0039_2b70)?;
        c.g[A1][0] = Addr(sp(c)).lw(c);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_2a60, 0x0039_2b84)?;
        c.g[A0][0] = sp(c) as u64;
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0039_2b90)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        c.lwc1(21, p_5(sp(c), 0x38));
        c.lwc1(20, p_5(sp(c), 0x30));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rendering and fonts.

/// `Font_DrawTextRight(font, text, x, y)` (0x392a60): measure the string,
/// subtract its width from x and draw it.
pub fn font_draw_text_right(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(21, 13);
        c.mov_s(20, 12);
        c.lwc1(12, p_6(c.g[S0][0] as u32, 0x38));
        c.lwc1(13, p_6(c.g[S0][0] as u32, 0x3c));
        call_ptr(c, 0x0039_1fb0, 0x0039_2a9c)?;
        c.sub_s(12, 20, 0);
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(13, 21);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_1cb0, 0x0039_2ab0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `Renderer_BuildSineTable(self)` (0x392df0): fill 640 floats with
/// `sin(i * 2*pi/512)`.
pub fn renderer_build_sine_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x10);
        frame.save_fpr(c, 20, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[S2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        c.lwc1(20, gp_2(c, -0x2694));
        loop {
            c.cycles += 9;
            c.mtc1(S1, 12);
            c.cvt_s_w(12, 12);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.mul_s(12, 12, 20);
            call_ptr(c, 0x0031_bf60, 0x0039_2e2c)?;
            c.swc1(0, c.g[S0][0] as u32);
            c.g[V0][0] = ((c.g[S1][0] as i64) < 640i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0039_2e18);
                }
            } else {
                c.cycles += 8;
                c.g[V0][0] = c.g[S2][0];
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.restore_fpr(c, 20, 0x40);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Fonts.

/// `float Font_GetCharWidth(sGlyphTable* t, char c)` (0x392ba8): binary
/// search the glyph table for `c`; a found glyph's width, else 0.0f.
pub fn font_get_char_width(c: &mut Cpu) {
    c.cycles += 7;
    let font = c.g[A0][0];
    c.g[V1][0] = font;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[A3][0] = Addr(p_7(font as u32, 0xc)).lw(c);
    c.g[A0][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A3][0] as i64)) as u64;
    let below_range = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    'done: {
        if below_range {
            break 'done;
        }
        'search: {
            c.cycles += 4;
            c.g[A1][0] = Addr(p_7(font as u32, 0x10)).lw(c);
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[T0][0] = Addr(font as u32).lw(c);
                break 'search;
            }
            c.cycles += 7;
            c.g[V0][0] = 0xc;
            c.g[V1][0] = Addr(p_7(font as u32, 4)).lw(c);
            c.mult(0, 4, 4, 2);
            c.mult(0, 2, 7, 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            break 'done;
        }
        c.cycles += 4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A3][0] as u32));
        c.g[T2][0] = Addr(p_7(c.g[V1][0] as u32, 4)).lw(c);
        c.g[T1][0] = 0xc;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        loop {
            c.cycles += 6;
            c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.mult(0, 6, 7, 9);
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T2][0] as u32));
            c.g[A2][0] = c.lhu(c.g[V0][0] as u32);
            let hit = c.g[A0][0] == c.g[A2][0];
            if hit {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            } else {
                c.cycles += 1;
                c.g[T0][0] = c.g[A3][0];
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[T0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T0][0] as u32));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = Addr(p_7(c.g[V1][0] as u32, 8)).lw(c);
                break 'done;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_2c00);
            }
        }
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(p_7(c.g[V0][0] as u32, 8)).lb(c);
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
    } else {
        c.cycles += 3;
        c.mtc1(0, 0);
    }
    c.pc = c.g[RA][0] as u32;
}

/// `float Font_GetCharWidthW(sGlyphTable* t, unsigned short c)` (0x392c60):
/// the same lookup for wide characters.
pub fn font_get_char_width_w(c: &mut Cpu) {
    c.cycles += 6;
    let font = c.g[A0][0];
    c.g[V1][0] = font;
    c.g[A3][0] = Addr(p_7(font as u32, 0xc)).lw(c);
    c.g[A0][0] = c.g[A1][0] & 0xffff;
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A3][0] as i64)) as u64;
    let below_range = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    'done: {
        if below_range {
            break 'done;
        }
        'search: {
            c.cycles += 4;
            c.g[A2][0] = Addr(p_7(font as u32, 0x10)).lw(c);
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[A1][0] = Addr(font as u32).lw(c);
                break 'search;
            }
            c.cycles += 7;
            c.g[V0][0] = 0xc;
            c.g[V1][0] = Addr(p_7(font as u32, 4)).lw(c);
            c.mult(0, 4, 4, 2);
            c.mult(0, 2, 7, 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            break 'done;
        }
        c.cycles += 5;
        c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A3][0] as u32));
        c.g[T2][0] = Addr(p_7(c.g[V1][0] as u32, 4)).lw(c);
        c.g[T1][0] = 0xc;
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        loop {
            c.cycles += 6;
            c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.mult(0, 6, 7, 9);
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T2][0] as u32));
            c.g[A2][0] = c.lhu(c.g[V0][0] as u32);
            let hit = c.g[A0][0] == c.g[A2][0];
            if hit {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            } else {
                c.cycles += 1;
                c.g[A1][0] = c.g[A3][0];
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = Addr(p_7(c.g[V1][0] as u32, 8)).lw(c);
                break 'done;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_2cb8);
            }
        }
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(p_7(c.g[V0][0] as u32, 8)).lb(c);
        c.cycles += 4;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
    } else {
        c.cycles += 3;
        c.mtc1(0, 0);
    }
    c.pc = c.g[RA][0] as u32;
}

/// `void PS2GraphicsMan_RotateAxis(GraphicsMan* self, const float* ang)`
/// (0x395d60): build the rotation matrix for the three angles through
/// `Math_SinCos` and transform the manager's `+0x13e4` matrix in place, then
/// clear `+0x6b90`.
pub fn ps2_graphics_man_rotate_axis(c: &mut Cpu) {
    c.cycles += 94;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
    c.sq(16, c.pa(SP, 0x80));
    c.sq(17, c.pa(SP, 0x70));
    c.g[S0][0] = c.g[A1][0];
    c.std(SP, 0x50, c.g[RA][0]);
    c.g[S1][0] = c.g[A0][0];
    c.sq(18, c.pa(SP, 0x60));
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S2][0] = c.ldw_2(S1, 0x13e4);
    c.g[RA][0] = sx(0x00395d8c);
    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x44));
    c.pc = 0x0031be50;
    call_indirect(c, 0x0031be50);
    if c.pc != 0x00395d8c {
        return;
    }
    c.lwc1(11, c.pa(SP, 0x44));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 5);
    c.lwc1(4, c.pa(S0, 0x8));
    c.sub_s(1, 5, 11);
    c.lwc1(0, c.pa(S0, 0));
    c.lwc1(6, c.pa(S0, 0x4));
    c.lwc1(8, c.pa(SP, 0x40));
    c.mul_s(2, 1, 4);
    c.swc1(5, c.pa(SP, 0x3c));
    c.mul_s(3, 1, 0);
    c.stw_2(SP, 0x30, 0u64);
    c.mul_s(1, 1, 6);
    c.stw_2(SP, 0x34, 0u64);
    c.mul_s(13, 8, 4);
    c.stw_2(SP, 0x38, 0u64);
    c.mul_s(12, 8, 0);
    c.stw_2(SP, 0xc, 0u64);
    c.mul_s(9, 2, 4);
    c.stw_2(SP, 0x1c, 0u64);
    c.mul_s(10, 3, 4);
    c.stw_2(SP, 0x2c, 0u64);
    c.mul_s(5, 2, 0);
    c.mul_s(7, 3, 0);
    c.mul_s(4, 1, 4);
    c.mul_s(0, 1, 0);
    c.mul_s(8, 8, 6);
    c.mul_s(2, 2, 6);
    c.mul_s(3, 3, 6);
    c.mul_s(1, 1, 6);
    c.add_s(5, 5, 8);
    c.sub_s(0, 0, 13);
    c.sub_s(2, 2, 12);
    c.add_s(9, 9, 11);
    c.swc1(5, c.pa(SP, 0x8));
    c.add_s(7, 7, 11);
    c.swc1(0, c.pa(SP, 0x4));
    c.add_s(3, 3, 13);
    c.swc1(2, c.pa(SP, 0x18));
    c.sub_s(10, 10, 8);
    c.swc1(9, c.pa(SP, 0x28));
    c.add_s(1, 1, 11);
    c.swc1(7, c.pa(SP, 0));
    c.add_s(4, 4, 12);
    c.swc1(3, c.pa(SP, 0x10));
    c.swc1(10, c.pa(SP, 0x20));
    c.swc1(1, c.pa(SP, 0x14));
    c.swc1(4, c.pa(SP, 0x24));
    c.lqc2(4, c.pa(S2, 0));
    c.lqc2(5, c.pa(S2, 0x10));
    c.lqc2(6, c.pa(S2, 0x20));
    c.lqc2(7, c.pa(S2, 0x30));
    c.lqc2(8, c.pa(SP, 0));
    c.lqc2(9, c.pa(SP, 0x10));
    c.lqc2(10, c.pa(SP, 0x20));
    c.lqc2(11, c.pa(SP, 0x30));
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
    c.sqc2(12, c.pa(S2, 0));
    c.sqc2(13, c.pa(S2, 0x10));
    c.sqc2(14, c.pa(S2, 0x20));
    c.sqc2(15, c.pa(S2, 0x30));
    c.stw_2(S1, 0x6b90, 0u64);
    c.lq(16, c.pa(SP, 0x80));
    c.lq(17, c.pa(SP, 0x70));
    c.lq(18, c.pa(SP, 0x60));
    c.g[RA][0] = c.ldd(SP, 0x50);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = ra;
    return;
}

// ---------------------------------------------------------------------------
// 0x396128 TexMan_InvalidateBound

/// `void TexMan_InvalidateBound(TexMan* self)` (0x396128): forget which
/// textures are bound: both bound-texture slots (+0x1f4c, +0x1f50, cleared
/// from the top down) and the bound palette (+0x1f54) become -1.
pub fn tex_man_invalidate_bound(c: &mut Cpu) {
    const BOUND_SLOTS: u32 = 0x1f4c;
    const BOUND_PALETTE: u32 = 0x1f54;
    let this = r32(c, A0);
    c.cycles += 4;
    c.g[A1][0] = u64::MAX;
    c.g[V1][0] = 1;
    c.g[V0][0] = sx(this.wrapping_add(BOUND_SLOTS + 4));
    loop {
        // 0x396138
        c.cycles += 7;
        c.sw(r32(c, V0), c.g[A1][0]);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_6138);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    c.sw(this.wrapping_add(BOUND_PALETTE), c.g[V0][0]);
    ret(c);
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_TryAcquireFrame (0x395f60, font.cpp)

/// `PS2GraphicsMan_AcquireFrameBuffer` (reached through the function table).
pub(crate) const OC264_GM_ACQUIRE_FRAME_BUFFER: u32 = 0x0038_26e0;

/// `bool PS2GraphicsMan_TryAcquireFrame(PS2GraphicsMan* self)` (0x395f60):
/// forwards to `PS2GraphicsMan_AcquireFrameBuffer(self)` (claims the
/// current frame buffer if it is free) and returns its result.
pub fn ps2_graphics_man_try_acquire_frame(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Table(OC264_GM_ACQUIRE_FRAME_BUFFER),
        0x0039_5f70,
    );
}

// ---------------------------------------------------------------------------
// Font_MeasureTextW (0x3921f0)

/// `cFont` glyph lookup: glyph count (+0), glyph table (+4, 12-byte
/// entries keyed by their first u16), default glyph (+8), first and last
/// directly indexed character (+0xc, +0x10), shadow offset (+0x28, +0x2c).
pub(crate) const FONT_GLYPH_COUNT: u32 = 0x0;

pub(crate) const FONT_GLYPHS: u32 = 0x4;

pub(crate) const FONT_DEFAULT_GLYPH: u32 = 0x8;

pub(crate) const FONT_FIRST_CHAR: u32 = 0xc;

pub(crate) const FONT_LAST_DIRECT: u32 = 0x10;

pub(crate) const FONT_SHADOW: u32 = 0x28;

/// The glyph of character `v1` (font a0, next character pointer left in a1,
/// result in t2), as `Font_MeasureTextW` inlines it: none below the first
/// character, a direct index up to the last direct one, else a binary search
/// of the table (default glyph if absent). Registers as the original leaves
/// them (a2 = the string pointer on entry).
pub(crate) fn font_find_glyph_inline(c: &mut Cpu, measure_head: u32) -> u64 {
    let font = c.g[A0][0] as u32;
    let ch = c.g[V1][0];
    c.cycles += 4;
    c.g[T2][0] = Addr(font + FONT_FIRST_CHAR).lw(c);
    let below = (ch as i64) < (c.g[T2][0] as i64);
    c.g[V0][0] = below as u64;
    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(2));
    if below {
        c.cycles += 2;
        c.g[T2][0] = 0;
        return 0;
    }
    c.cycles += 4;
    c.g[A1][0] = Addr(font + FONT_LAST_DIRECT).lw(c);
    let searched = (c.g[A1][0] as i64) < (ch as i64);
    c.g[V0][0] = searched as u64;
    if !searched {
        // Direct: glyphs + (ch - first) * 12.
        c.cycles += 7;
        c.mult(0, V1, V1, T4);
        c.mult(0, T2, T2, T4);
        let glyphs = Addr(font + FONT_GLYPHS).lw(c);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(2));
        c.g[V0][0] = sx((glyphs as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[T2][0] as u32));
        return c.g[T2][0];
    }
    // Binary search over [lo = last - first, hi = count).
    c.g[T2][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[T2][0] as u32));
    c.cycles += 5;
    c.g[T0][0] = Addr(font + FONT_GLYPH_COUNT).lw(c);
    c.g[T6][0] = Addr(font + FONT_GLYPHS).lw(c);
    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(2));
    c.g[T1][0] = 0xc;
    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[T0][0] as u32));
    loop {
        c.cycles += 6;
        c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        c.mult(0, A2, A3, T1);
        let entry = (c.g[A2][0] as u32).wrapping_add(c.g[T6][0] as u32);
        c.g[V0][0] = sx(entry);
        c.g[A2][0] = Addr(entry).lhu(c);
        if ch == c.g[A2][0] {
            c.g[T2][0] = sx(entry);
            return c.g[T2][0];
        }
        c.cycles += 3;
        let lower = (ch as i64) < (c.g[A2][0] as i64);
        c.g[V0][0] = lower as u64;
        if !lower {
            c.g[T2][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        } else {
            c.cycles += 1;
            c.g[T0][0] = c.g[A3][0];
        }
        c.cycles += 3;
        let more = (c.g[T2][0] as i64) < (c.g[T0][0] as i64);
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[T0][0] as u32));
        if !more {
            c.cycles += 1;
            c.g[T2][0] = Addr(font + FONT_DEFAULT_GLYPH).lw(c);
            return c.g[T2][0];
        }
        if event_due(c) {
            deliver_event_here(c, measure_head);
        }
    }
}

/// `float Font_MeasureTextW(cFont* font, const u16* str, Rect* out, int mono,
/// float sx, float sy)` (0x3921f0): the bounding box (x0, y0, x1, y1) of a
/// wide string drawn at the origin with scale (sx, sy), widened by the
/// font's shadow offset; returns its x1. `out` may be null (a stack rect).
pub fn font_measure_text_w(c: &mut Cpu) {
    c.cycles += 13;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    c.g[T5][0] = c.g[A3][0];
    c.g[T3][0] = if c.g[A2][0] != 0 {
        c.g[A2][0]
    } else {
        c.g[SP][0]
    };
    let rc = c.g[T3][0] as u32;
    Addr(rc).sw(c, 0);
    c.g[A2][0] = c.g[A1][0];
    Addr(rc + 4).sw(c, 0);
    Addr(rc + 8).sw(c, 0);
    Addr(rc + 0xc).sw(c, 0);
    c.lwc1(6, rc); // x = 0
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lhu(c);
    c.mov_s(7, 6); // y = 0
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[T4][0] = 0xc;
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lhu(c);
        loop {
            let glyph = font_find_glyph_inline(c, 0x0039_2288);
            c.cycles += 2;
            c.g[A2][0] = c.g[A1][0];
            if glyph != 0 {
                let g = glyph as u32;
                c.cycles += 31;
                c.g[V0][0] = Addr(g + 9).lb(c);
                c.lwc1(2, rc);
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.g[V1][0] = Addr(g + 8).lb(c);
                c.lwc1(3, rc + 4);
                c.mtc1(V1, 1);
                c.cvt_s_w(1, 1);
                c.lwc1(4, rc + 0xc);
                c.mul_s(0, 0, 12);
                c.mul_s(5, 1, 12); // advance * sx
                c.add_s(0, 6, 0);
                c.min_s(0, 0, 2);
                c.swc1(0, rc); // x0 = min(x + xoff * sx, x0)
                c.g[V0][0] = Addr(g + 0xa).lb(c);
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(0, 0, 13);
                c.add_s(0, 7, 0);
                c.min_s(0, 0, 3);
                c.swc1(0, rc + 4); // y0 = min(y + yoff * sy, y0)
                let yoff = Addr(g + 0xa).lb(c);
                c.g[V1][0] = Addr(g + 3).lbu(c);
                c.g[V0][0] = sx((yoff as u32).wrapping_add(c.g[V1][0] as u32));
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(0, 0, 13);
                c.add_s(0, 7, 0);
                c.max_s(0, 0, 4);
                c.swc1(0, rc + 0xc); // y1 = max(y + (yoff + h) * sy, y1)
                if c.g[T5][0] != 0 {
                    // Monospaced: x1 = max(x + advance, x1).
                    c.cycles += 3;
                    c.add_s(0, 6, 5);
                    c.lwc1(1, rc + 8);
                    c.mov_s(2, 0);
                } else {
                    // x1 = max(x + (xoff + w) * sx, x1).
                    c.cycles += 10;
                    c.g[V1][0] = Addr(g + 2).lbu(c);
                    c.add_s(2, 6, 5);
                    let xoff = Addr(g + 9).lb(c);
                    c.lwc1(1, rc + 8);
                    c.g[V0][0] = sx((xoff as u32).wrapping_add(c.g[V1][0] as u32));
                    c.mtc1(V0, 0);
                    c.cvt_s_w(0, 0);
                    c.mul_s(0, 0, 12);
                    c.add_s(0, 6, 0);
                }
                c.cycles += 4;
                c.max_s(0, 0, 1);
                c.swc1(0, rc + 8);
                c.mov_s(6, 2); // x += advance
                c.g[A2][0] = c.g[A1][0];
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lhu(c);
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lhu(c);
            if event_due(c) {
                deliver_event_here(c, 0x0039_2230);
            }
        }
    }
    // x1 += x0, y1 += y0; then the shadow offset.
    let font = c.g[A0][0] as u32;
    c.cycles += 14;
    c.lwc1(2, rc + 8);
    c.lwc1(0, rc + 0xc);
    c.lwc1(4, rc);
    c.lwc1(1, rc + 4);
    c.add_s(3, 2, 4);
    c.add_s(0, 0, 1);
    c.f[2] = 0;
    c.swc1(3, rc + 8);
    c.swc1(0, rc + 0xc);
    c.lwc1(0, font + FONT_SHADOW);
    c.c_lt_s(0, 2);
    if !c.fcc() {
        c.add_s(0, 3, 0);
    } else {
        c.cycles += 4;
        c.sub_s(1, 4, 0);
        c.swc1(1, rc);
        c.lwc1(0, font + FONT_SHADOW);
        c.sub_s(0, 3, 0);
    }
    c.cycles += 7;
    c.swc1(0, rc + 8);
    c.lwc1(1, font + FONT_SHADOW + 4);
    c.f[0] = 0;
    c.c_lt_s(1, 0);
    if !c.fcc() {
        c.lwc1(0, rc + 0xc);
        c.cycles += 2;
        c.add_s(0, 0, 1);
        c.swc1(0, rc + 0xc);
    } else {
        c.cycles += 8;
        c.lwc1(0, rc + 4);
        c.lwc1(2, rc + 0xc);
        c.sub_s(0, 0, 1);
        c.swc1(0, rc + 4);
        c.lwc1(1, font + FONT_SHADOW + 4);
        c.sub_s(2, 2, 1);
        c.swc1(2, rc + 0xc);
    }
    c.cycles += 3;
    c.lwc1(0, rc + 8);
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Font_DrawTextW (0x391e30)

/// `cFont` drawing fields: offset (+0x18), UV scale (+0x20), shadow offset
/// (+0x28), scale (+0x38), colour (+0x40), shadow colour (+0x50), texture
/// id (+0x60).
pub(crate) const FONT_OFFSET: u32 = 0x18;

pub(crate) const FONT_UV_SCALE: u32 = 0x20;

pub(crate) const FONT_SCALE: u32 = 0x38;

pub(crate) const FONT_TEXTURE: u32 = 0x60;

/// GrMan's text draw (vtable `+0x288`): font, string, colour at
/// `font + colour`, position (f12, f13), scale, UV scale, z = 0
/// (`sp + 8`). Registers as `Font_DrawTextW` sets them.
pub(crate) fn font_draw_text_w_call(c: &mut Cpu, colour: u32, ret: u32) -> crate::call::Flow {
    let grman = c.g[S3][0] as u32;
    let vt = c.g[V0][0] as u32;
    let font = c.g[S2][0] as u32;
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S4][0];
    c.g[A3][0] = sx(font.wrapping_add(colour));
    let delta = Addr(vt + 0x288).lh(c);
    let func = Addr(vt + 0x28c).lw(c);
    c.g[A0][0] = sx(grman.wrapping_add(delta as u32));
    c.lwc1(14, font + FONT_SCALE);
    c.lwc1(15, font + FONT_SCALE + 4);
    c.lwc1(16, font + FONT_UV_SCALE);
    c.lwc1(17, font + FONT_UV_SCALE + 4);
    let z = sp(c) + 8;
    c.lwc1(18, z);
    if colour == 0x40 {
        c.g[V1][0] = func;
    } else {
        c.g[T0][0] = func;
    }
    call_ptr(c, func as u32, ret)
}

/// `void Font_DrawTextW(cFont* font, const u16* str, float x, float y)`
/// (0x391e30): bind the font texture, draw the string at (x, y) + offset *
/// scale through GrMan (vtable `+0x288`); with a shadow offset, first draw
/// the shadow (shadow colour, one layer lower) shifted by it.
pub fn font_draw_text_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S3, 0x30);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0x10);
        let font = c.g[S2][0] as u32;
        let gp = c.g[GP][0] as u32;
        let grman = Addr(gp.wrapping_add(AVALANCHE_GRMAN_GP)).lw(c);
        c.g[S3][0] = grman;
        let tex = Addr(font + FONT_TEXTURE).lw(c);
        c.g[V1][0] = tex;
        let ds = ui_vector_lw(c, grman, 0xe84);
        c.g[V0][0] = ds;
        Addr((ds as u32).wrapping_add(0x10)).sh(c, tex);
        let pos = frame.at(c, 0);
        Addr(pos + 8).sw(c, 0);
        // pos = (x + offset.x * scale.x, y + offset.y * scale.y, 0, 0).
        c.lwc1(2, font + FONT_SCALE);
        c.lwc1(0, font + FONT_OFFSET);
        c.lwc1(1, font + FONT_OFFSET + 4);
        c.lwc1(3, font + FONT_SCALE + 4);
        c.mul_s(0, 0, 2);
        c.lwc1(2, font + FONT_SHADOW);
        c.mul_s(1, 1, 3);
        c.lwc1(4, pos + 8);
        c.add_s(12, 12, 0);
        Addr(pos + 0xc).sw(c, 0);
        c.c_eq_s(2, 4);
        c.add_s(13, 13, 1);
        c.swc1(12, pos);
        let mut shadow = !c.fcc();
        c.swc1(13, pos + 4);
        if !shadow {
            c.cycles += 5;
            c.lwc1(0, font + FONT_SHADOW + 4);
            c.c_eq_s(0, 4);
            shadow = !c.fcc();
        }
        if shadow {
            // Shadow: draw-state layer (bits 5..9 of word +8) minus one.
            c.cycles += 40;
            let ds = ui_vector_lw(c, grman, 0xe84);
            c.g[T0][0] = ds;
            c.g[S1][0] = 0xffff_ffff_ffff_fc1f;
            let w = ui_vector_lw(c, ds, 8);
            c.g[S0][0] = sx(((w & 0x3e0) as u32) >> 5);
            let layer = sx((c.g[S0][0] as u32).wrapping_sub(1));
            let positive = (-1i64) < (layer as i64);
            c.g[A0][0] = positive as u64;
            let layer = if positive { layer } else { 0 };
            c.g[V0][0] = sx((layer as u32) << 5) & 0x3e0;
            c.g[V1][0] = (w & 0xffff_ffff_ffff_fc1f) | c.g[V0][0];
            Addr((ds as u32).wrapping_add(8)).sw(c, c.g[V1][0]);
            c.g[V0][0] = ui_vector_lw(c, grman, 0x10d8);
            c.lwc1(0, pos);
            c.lwc1(12, font + FONT_SHADOW);
            c.lwc1(1, pos + 4);
            c.lwc1(13, font + FONT_SHADOW + 4);
            c.add_s(12, 0, 12);
            c.add_s(13, 1, 13);
            font_draw_text_w_call(c, 0x50, 0x0039_1f3c)?;
            let ds = ui_vector_lw(c, c.g[S3][0], 0xe84);
            c.g[V1][0] = ds;
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 5);
            let w = ui_vector_lw(c, ds, 8);
            c.g[V0][0] = (w & c.g[S1][0]) | c.g[S0][0];
            Addr((ds as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
        }
        let grman = c.g[S3][0];
        c.g[V0][0] = ui_vector_lw(c, grman, 0x10d8);
        c.cycles += 22;
        let pos = frame.at(c, 0);
        c.lwc1(12, pos);
        c.lwc1(13, pos + 4);
        font_draw_text_w_call(c, 0x40, 0x0039_1f90)?;
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

// ---------------------------------------------------------------------------
// Small wrappers and leaves

/// `bool PS2GraphicsMan_IsIdle(PS2GraphicsMan* self)` (0x395f80): forwards
/// to `PS2GraphicsMan_IsRenderIdle` (0x382730, through the table).
pub fn ps2_graphics_man_is_idle(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Table(0x0038_2730),
        0x0039_5f90,
    );
}

// ---------------------------------------------------------------------------
// Font_FindLineBreak

/// Font: horizontal shadow offset (float; its magnitude shortens the line).
pub(crate) const FONT_FIND_LINE_BREAK_SHADOW_X: u32 = 0x28;

/// Font: lowest character of the direct glyph range.
pub(crate) const FONT_FIND_LINE_BREAK_LO: u32 = 0xc;

/// Font: highest character of the direct glyph range.
pub(crate) const FONT_FIND_LINE_BREAK_HI: u32 = 0x10;

/// Font: glyph count (the end of the sorted extra glyphs).
pub(crate) const FONT_FIND_LINE_BREAK_COUNT: u32 = 0x0;

/// Font: glyph table (12-byte entries: `u16 code` at +0, `u8 w` at +2,
/// `s8 advance` at +8, `s8 x offset` at +9).
pub(crate) const FONT_FIND_LINE_BREAK_TABLE: u32 = 0x4;

/// Font: the glyph used for a character missing from the table.
pub(crate) const FONT_FIND_LINE_BREAK_DEFAULT: u32 = 0x8;

/// The "line is full" exit of `Font_FindLineBreak`: end the copied line
/// after the last word (`dst[lastSpace + 1] = 0`, v0 = that address - 1),
/// then skip the spaces after it in the source; returns (v0) the start of
/// the next line. `extra` is the cost of the branch that led here, `head`
/// the skip loop's pc.
pub(crate) fn font_find_line_break_skip_spaces(c: &mut Cpu, extra: u64, head: u32) {
    c.cycles += extra;
    c.g[A0][0] = sx(r32(c, T8).wrapping_add(r32(c, T6)));
    let at = r32(c, V0).wrapping_add(1);
    c.sb(at, 0);
    c.g[V1][0] = 0x20;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
    loop {
        c.cycles += 7;
        c.g[V0][0] = c.lb(r32(c, A0));
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            return;
        }
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `char* Font_FindLineBreak(cFont* font, const char* str, char* dst, int max,
/// float scale, float width)` (0x392430): copy `str` into `dst` until the
/// scaled glyph widths reach `width` (less the shadow offset) or `max - 1`
/// characters. When the line is full it breaks after the last whole word
/// and returns where the next line starts (spaces skipped); if a single word
/// does not fit it breaks inside it and returns the rest; if everything fits
/// it returns 0. Glyphs come from the direct range `lo..=hi`, else a binary
/// search of the sorted extra glyphs (the default glyph when missing), and
/// none below `lo`.
///
/// Registers as in the original: t0 str, t8 start, t5 n, t7 k, t6 last
/// word end (-1: none yet), f3 width so far, f4 pending advance, f13 the
/// width limit.
pub fn font_find_line_break(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x40));
    let frame = r32(c, SP);
    c.g[T3][0] = c.g[A0][0];
    c.sq(16, frame + 0x30);
    c.g[T0][0] = c.g[A1][0];
    c.sq(17, frame + 0x20);
    c.sq(18, frame + 0x10);
    c.sq(19, frame);
    let font = r32(c, T3);
    // width -= |shadow x|, clamped at 0.
    c.mtc1(0, 1);
    c.lwc1(0, font.wrapping_add(FONT_FIND_LINE_BREAK_SHADOW_X));
    c.abs_s(0, 0);
    c.sub_s(13, 13, 0);
    c.c_lt_s(13, 1);
    if c.fcc() {
        c.mov_s(13, 1);
    }
    'ret: {
        'tail: {
            c.cycles += 8;
            c.mov_s(3, 1);
            c.g[V0][0] = c.lb(r32(c, T0));
            c.g[T8][0] = c.g[T0][0];
            c.mov_s(4, 3);
            c.g[T6][0] = u64::MAX;
            c.g[T5][0] = 0;
            let empty = c.g[V0][0] == 0;
            c.g[T7][0] = 0;
            if empty {
                break 'tail;
            }
            c.cycles += 5;
            c.g[S1][0] = sx(r32(c, A3).wrapping_sub(1));
            c.g[S0][0] = 0xc;
            c.g[S2][0] = u64::MAX;
            c.g[V0][0] = c.lb(r32(c, T0));
            c.g[V0][0] ^= 0x20;
            loop {
                // A space: break here if the line is full, else remember the
                // end of the word before it.
                c.cycles += 2;
                let is_space = c.g[V0][0] == 0;
                c.g[T9][0] = c.lbu(r32(c, T0));
                if is_space {
                    c.cycles += 4;
                    c.c_le_s(13, 3);
                    let full = c.fcc();
                    c.g[V0][0] = sx(r32(c, T6).wrapping_add(r32(c, A2)));
                    if full {
                        font_find_line_break_skip_spaces(c, 4, 0x0039_24c8);
                        break 'ret;
                    }
                    c.cycles += 2;
                    let first = (c.g[T7][0] as i64) <= 0;
                    c.g[V0][0] = sx(r32(c, T7).wrapping_add(r32(c, T8)));
                    if !first {
                        c.cycles += 4;
                        c.g[A0][0] = sx(r32(c, T7).wrapping_sub(1));
                        c.g[V1][0] = c.lb(r32(c, V0).wrapping_sub(1));
                        c.g[V1][0] ^= 0x20;
                        if c.g[V1][0] != 0 {
                            c.g[T6][0] = c.g[A0][0];
                        }
                    }
                }
                // The glyph (t2; 0 = none); t1 = &dst[n].
                'glyph: {
                    c.cycles += 5;
                    c.g[V1][0] = c.lbu(r32(c, T0));
                    c.g[T2][0] = c.lw(font.wrapping_add(FONT_FIND_LINE_BREAK_LO));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
                    let below = c.g[V0][0] != 0;
                    c.g[T1][0] = sx(r32(c, A2).wrapping_add(r32(c, T5)));
                    if below {
                        c.cycles += 2;
                        c.g[T2][0] = 0;
                        break 'glyph;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(font.wrapping_add(FONT_FIND_LINE_BREAK_HI));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        // Direct range: table + (ch - lo) * 12.
                        c.cycles += 7;
                        c.mult(0, V1, V1, S0);
                        c.mult(0, T2, T2, S0);
                        c.g[V0][0] = c.lw(font.wrapping_add(FONT_FIND_LINE_BREAK_TABLE));
                        c.g[T1][0] = sx(r32(c, A2).wrapping_add(r32(c, T5)));
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                        c.g[T2][0] = sx(r32(c, V0).wrapping_sub(r32(c, T2)));
                        break 'glyph;
                    }
                    // Above the range: binary search glyphs hi - lo .. count.
                    c.g[A3][0] = c.lw(font.wrapping_add(FONT_FIND_LINE_BREAK_COUNT));
                    c.cycles += 5;
                    c.g[T2][0] = sx(r32(c, A0).wrapping_sub(r32(c, T2)));
                    c.g[S3][0] = c.lw(font.wrapping_add(FONT_FIND_LINE_BREAK_TABLE));
                    c.g[T1][0] = sx(r32(c, A2).wrapping_add(r32(c, T5)));
                    c.g[T4][0] = 0xc;
                    c.g[V0][0] = sx(r32(c, T2).wrapping_add(r32(c, A3)));
                    loop {
                        c.cycles += 6;
                        c.g[A1][0] = sx((r32(c, V0) as i32 >> 1) as u32);
                        c.mult(0, A0, A1, T4);
                        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
                        c.g[A0][0] = c.lhu(r32(c, V0));
                        if c.g[V1][0] == c.g[A0][0] {
                            c.g[T2][0] = c.g[V0][0];
                            break 'glyph;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            c.g[T2][0] = sx(r32(c, A1).wrapping_add(1));
                        } else {
                            c.cycles += 1;
                            c.g[A3][0] = c.g[A1][0];
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V0][0] = sx(r32(c, T2).wrapping_add(r32(c, A3)));
                        if !more {
                            c.cycles += 1;
                            c.g[T2][0] = c.lw(font.wrapping_add(FONT_FIND_LINE_BREAK_DEFAULT));
                            break 'glyph;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0039_2560);
                        }
                    }
                }
                // right = (xoff + w) * scale; width += pend + right;
                // pend = advance * scale - right.
                c.cycles += 2;
                if c.g[T2][0] != 0 {
                    c.cycles += 13;
                    let glyph = r32(c, T2);
                    c.g[V0][0] = c.lb(glyph.wrapping_add(9));
                    c.g[V1][0] = c.lbu(glyph.wrapping_add(2));
                    c.g[A0][0] = c.lb(glyph.wrapping_add(8));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.mtc1(V0, 0);
                    c.cvt_s_w(0, 0);
                    c.mtc1(A0, 1);
                    c.cvt_s_w(1, 1);
                    c.mul_s(0, 0, 12);
                    c.mul_s(1, 1, 12);
                    c.add_s(2, 4, 0);
                    c.sub_s(4, 1, 0);
                    c.add_s(3, 3, 2);
                }
                // Copy the character, unless the line is full with no word
                // break yet: then break inside the word.
                c.cycles += 4;
                c.c_le_s(13, 3);
                let dst = r32(c, T1);
                if !c.fcc() {
                    let ch = c.g[T9][0];
                    c.sb(dst, ch);
                } else {
                    c.cycles += 2;
                    if c.g[T6][0] == c.g[S2][0] {
                        c.sb(dst, 0);
                        c.cycles += 2;
                        c.g[V0][0] = c.g[T0][0];
                        break 'ret;
                    }
                    c.cycles += 1;
                    let ch = c.g[T9][0];
                    c.sb(dst, ch);
                }
                c.cycles += 4;
                c.g[T5][0] = sx(r32(c, T5).wrapping_add(1));
                c.g[T7][0] = sx(r32(c, T7).wrapping_add(1));
                let at_max = c.g[T5][0] == c.g[S1][0];
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                if at_max {
                    break 'tail;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(r32(c, T0));
                let more = c.g[V0][0] != 0;
                c.g[V0][0] ^= 0x20;
                if !more {
                    break 'tail;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_24a0);
                }
            }
        }
        // End of the string (or max): break after the last word if full.
        c.cycles += 4;
        c.c_le_s(13, 3);
        let full = c.fcc();
        c.g[V0][0] = sx(r32(c, T6).wrapping_add(r32(c, A2)));
        if full {
            font_find_line_break_skip_spaces(c, 5, 0x0039_2630);
            break 'ret;
        }
        c.cycles += 3;
        c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, T5)));
        c.g[V0][0] = 0;
        let end = r32(c, V1);
        c.sb(end, 0);
    }
    c.cycles += 6;
    let frame = r32(c, SP);
    c.lq(16, frame + 0x30);
    c.lq(17, frame + 0x20);
    c.lq(18, frame + 0x10);
    c.lq(19, frame);
    c.g[SP][0] = sx(frame.wrapping_add(0x40));
    ret(c);
}

/// The gcc varargs forwarder of the font draw calls: spills `a2..t3` and
/// `f14, f16, f18` to the caller's varargs area (the format string now at
/// `sp + 0x40`) and tail-calls `draw`, returning to `ret`.
pub(crate) fn draw_varargs(c: &mut Cpu, draw: u32, ret: u32) -> Flow {
    c.cycles += 16;
    let frame = Frame::open(c, 0x70);
    let s = sp(c);
    let a2 = c.g[A2][0];
    c.sd(s.wrapping_add(0x40), a2);
    frame.save_ra(c, 0);
    c.g[A2][0] = sx(s.wrapping_add(0x40));
    let spill = [c.g[A3][0], c.g[T0][0], c.g[T1][0], c.g[T2][0], c.g[T3][0]];
    for (i, v) in spill.into_iter().enumerate() {
        c.sd(s.wrapping_add(0x48 + 8 * i as u32), v);
    }
    c.swc1(14, s.wrapping_add(0x34));
    c.swc1(16, s.wrapping_add(0x38));
    c.swc1(18, s.wrapping_add(0x3c));
    call_ptr(c, draw, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `Font_DrawTextVarargs` (0x1dc990): forwards to `0x391c48`.
pub fn font_draw_text_varargs(c: &mut Cpu) {
    guest(c, |c| draw_varargs(c, 0x0039_1c48, 0x001d_c9c4));
}

/// `Font_DrawTextCenteredVarargs` (0x1dca40): forwards to `0x3929f8`.
pub fn font_draw_text_centered_varargs(c: &mut Cpu) {
    guest(c, |c| draw_varargs(c, 0x0039_29f8, 0x001d_ca74));
}

/// `Font_DrawTextRightVarargs` (0x1dca80): forwards to `0x392b40`.
pub fn font_draw_text_right_varargs(c: &mut Cpu) {
    guest(c, |c| draw_varargs(c, 0x0039_2b40, 0x001d_cab4));
}

/// `Font_DrawTextScaledVarargs` (0x1dc9d0): the object at `a0` holds the
/// scale (f0 = +0x34 times f15, f1 = +0x30 times f14); both are stored back
/// at +0x38/+0x3c, `0x391c48` draws, and the 8 bytes at +0x30 are copied to
/// +0x38 through v0.
pub fn font_draw_text_scaled_varargs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let s = sp(c);
        let a2 = c.g[A2][0];
        c.sd(s.wrapping_add(0x40), a2);
        c.g[A2][0] = sx(s.wrapping_add(0x40));
        let obj = c.g[S0][0] as u32;
        c.lwc1(1, obj.wrapping_add(0x30));
        c.lwc1(0, obj.wrapping_add(0x34));
        c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[14])).0;
        let a3 = c.g[A3][0];
        c.sd(s.wrapping_add(0x48), a3);
        c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[15])).0;
        let spill = [c.g[T0][0], c.g[T1][0], c.g[T2][0], c.g[T3][0]];
        for (i, v) in spill.into_iter().enumerate() {
            c.sd(s.wrapping_add(0x50 + 8 * i as u32), v);
        }
        c.swc1(16, s.wrapping_add(0x38));
        c.swc1(18, s.wrapping_add(0x3c));
        c.swc1(1, obj.wrapping_add(0x38));
        c.swc1(0, obj.wrapping_add(0x3c));
        call_ptr(c, 0x0039_1c48, 0x001d_ca20)?;
        let obj = c.g[S0][0] as u32;
        c.ldl(2, obj.wrapping_add(0x37));
        c.ldr(2, obj.wrapping_add(0x30));
        c.sdl(2, obj.wrapping_add(0x3f));
        c.sdr(2, obj.wrapping_add(0x38));
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

pub(crate) mod gmesh {

    pub(crate) use crate::game::render::particle::gmesh::VALID;
}

pub(crate) const LIGHT_MAN_NEXT_INDEX: u32 = 0xc;

/// `void LightMan_dtor_Wrapper(LightMan* self, int flags)` (0x394c68): the
/// vtable's destructor slot; forwards to `LightMan::~LightMan`.
pub fn light_man_dtor_wrapper(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(callee::LIGHT_MAN_DTOR), 0x0039_4c78);
}

/// `void** LightMan_GetInstance(LightMan* self, int id)` (0x394c88): the
/// address of slot `id` of the instance table at +0x4 (`a1` is left as
/// `id * 4`).
pub fn light_man_get_instance(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(LIGHT_MAN_INSTANCES).lw(c);
    c.g[A1][0] = sx(r32(c, A1) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
    ret(c);
}

/// `void LightMan_ResetCounter(LightMan* self)` (0x394d10): next index
/// (+0xc) = 0.
pub fn light_man_reset_counter(c: &mut Cpu) {
    c.cycles += 2;
    let this = Addr(abi::arg(c, 0));
    this.off(LIGHT_MAN_NEXT_INDEX).sw(c, 0);
    ret(c);
}

/// `void LightMan_v08(LightMan* self)` (0x394d30): empty virtual method.
pub fn light_man_v08(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void* PS2GraphicsMan_GetFogState(gfx)` (0x3953b0): the fog state block
/// embedded at `gfx+0x6b94`.
pub fn ps2_graphics_man_get_fog_state(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6b94));
}

/// `void PS2GraphicsMan_ClearLights(gfx)` (0x395420): zero the light count
/// at `gfx+0x13ec`.
pub fn ps2_graphics_man_clear_lights(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
    let gfx = c.g[A0][0] as u32;
    Addr(gfx.wrapping_add(0x13ec)).sw(c, 0);
}

/// `void PS2GraphicsMan_v110(gfx)` (0x395508): empty virtual slot.
pub fn ps2_graphics_man_v110(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void LightMan_v13(LightMan* self)` (0x394d38): empty virtual method.
pub fn light_man_v13(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void LightMan_v14(LightMan* self)` (0x394d40): empty virtual method.
pub fn light_man_v14(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void LightMan_v15(LightMan* self)` (0x394d48): empty virtual method.
pub fn light_man_v15(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// Renderer_StaticInitDestroy: the static objects of the renderer unit.

/// The priority the unit's constructors run at (gcc's default, 0xffff).
pub(crate) const RENDERER_INIT_PRIORITY: u64 = 0xffff;

pub(crate) const RENDERER_INIT_FRAME: u32 = 0x120;

/// Callee-saved registers and their frame slots.
pub(crate) const RENDERER_INIT_SAVED: [(usize, u32); 9] = [
    (S0, 0xf0),
    (S1, 0xe0),
    (S2, 0xd0),
    (S3, 0xc0),
    (S4, 0xb0),
    (S5, 0xa0),
    (S6, 0x90),
    (S7, 0x80),
    (FP, 0x70),
];

pub(crate) const RENDERER_INIT_RA_SLOT: u32 = 0x60;

pub(crate) const RENDERER_INIT_SAVED_FPRS: [(usize, u32); 4] =
    [(23, 0x118), (22, 0x110), (21, 0x108), (20, 0x100)];

/// Stack slots the compiled code keeps values in (outside the saved area).
pub(crate) const RENDERER_INIT_MASK_SLOT: u32 = 0x10;

pub(crate) const RENDERER_INIT_COUNT_SLOT: u32 = 0x14;

pub(crate) const RENDERER_INIT_CURSOR_SLOT: u32 = 0x58;

/// High half of every static's address (`lui 0x50`).
pub(crate) const RENDERER_STATICS_HI: u64 = 0x50_0000;

/// 56 vec4 constants (unknown use; mostly w = 0 or 1).
pub(crate) const RENDERER_VEC4S_A: u32 = 0x0050_10a0;

/// A 16-byte block cleared with memset (other units copy it as a template).
pub(crate) const RENDERER_CLEARED_BLOCK: u32 = 0x0050_1420;

/// Two halfwords set to -1, right after the cleared block.
pub(crate) const RENDERER_HALF_PAIR: u32 = 0x0050_1430;

/// Two halfwords set to -1 (the compiled code addresses them absolutely; at
/// the game's gp this is gp+0x2920).
pub(crate) const RENDERER_GLOBAL_HALF_PAIR: u32 = 0x004a_5a10;

/// Five vec4 constants (unknown use).
pub(crate) const RENDERER_VEC4S_B: u32 = 0x0050_46e0;

/// A 4x4 matrix: x * 0.5, y * -0.5, then + (0.5, 0.5, 1, 1).
pub(crate) const RENDERER_HALF_SCALE_MATRIX: u32 = 0x0050_4760;

/// A 4x4 axis swap: x' = -y, y' = z, z' = x.
pub(crate) const RENDERER_AXIS_MATRIX: u32 = 0x0050_47a0;

/// A random state (0x18 bytes) filled by `Rand_GetDefaultState`.
pub(crate) const RENDERER_RAND_STATE: u32 = 0x0050_47e0;

/// Two words cleared after the random state.
pub(crate) const RENDERER_CLEARED_PAIR: u32 = 0x0050_47f8;

/// Two vec4 (1, 1, 1, 1).
pub(crate) const RENDERER_ONES: u32 = 0x0050_4810;

/// A copy of the all-ones entry 10 of [`RENDERER_VEC4S_A`].
pub(crate) const RENDERER_ONES_COPY: u32 = 0x0050_4830;

pub(crate) const RENDERER_ONES_SOURCE: u32 = RENDERER_VEC4S_A + 0xa0;

/// Four records (stride 0x60) of twelve VU microcode entry addresses.
pub(crate) const RENDERER_MICROCODE_TABLES: u32 = 0x0050_4840;

/// A second copy of the half-scale matrix.
pub(crate) const RENDERER_HALF_SCALE_MATRIX_2: u32 = 0x0050_49c0;

/// The renderer's scratchpad stream, set to mode 3.
pub(crate) const RENDERER_SPR_STREAM: u32 = 0x0050_4f80;

pub(crate) const RENDERER_SPR_STREAM_MODE: u64 = 3;

/// 640 floats filled by `Renderer_BuildSineTable`.
pub(crate) const RENDERER_SINE_TABLE: u32 = 0x0050_4fb8;

/// Two vec4 after the sine table.
pub(crate) const RENDERER_VEC4S_C: u32 = 0x0050_59b8;

/// gp-relative globals: two records of words (bit masks and sizes;
/// unknown use) and two flags.
pub(crate) const RENDERER_GP_WORDS: [(u32, u32); 24] = [
    (0x28bc, 0xc),
    (0x28d4, 0x0180_0000),
    (0x28d8, 0x0040_0000),
    (0x28dc, 0x0030_0000),
    (0x28e4, 0x7c),
    (0x28b8, 3),
    (0x28c0, 0),
    (0x28c4, 0),
    (0x28c8, 0),
    (0x28cc, 0),
    (0x28d0, 0),
    (0x28e0, 0x000f_f000),
    (0x28e8, 0),
    (0x28ec, 3),
    (0x28f4, 0x3e0),
    (0x28f8, 0x3c0),
    (0x28fc, 0xc00),
    (0x2900, 0x000f_f000),
    (0x2904, 0x1f),
    (0x2910, 0x3fff_ffff),
    (0x2908, 0),
    (0x290c, 0),
    (0x2914, 0),
    (0x2918, 0),
];

/// The second record's mask, which the compiled code passes through a stack
/// slot (stored at `gp+0x28f0`).
pub(crate) const RENDERER_GP_MASK: u64 = 0x1fff_fc00;

pub(crate) const RENDERER_GP_MASK_WORD: u32 = 0x28f0;

/// A word set to -1 (`gp+0x2928`).
pub(crate) const RENDERER_GP_FLAG: u32 = 0x2928;

/// A word cleared after the stream mode is set (`gp+0x2930`).
pub(crate) const RENDERER_GP_STREAM_WORD: u32 = 0x2930;

pub(crate) const RENDERER_ONE: u32 = 0x3f80_0000;

pub(crate) const RENDERER_HALF: u32 = 0x3f00_0000;

pub(crate) const RENDERER_NEG_HALF: u32 = 0xbf00_0000;

pub(crate) const RENDERER_NEG_ONE: u32 = 0xbf80_0000;

/// One float word of a static table: a constant, or the literal at
/// `gp - offset`.
#[derive(Clone, Copy)]
pub(crate) enum RendererWord {
    One,
    Half,
    NegHalf,
    NegOne,
    Zero,
    Lit(u32),
}

pub(crate) const RENDERER_VEC4S_A_INIT: [[RendererWord; 4]; 56] = {
    use RendererWord::{Half, Lit, One, Zero};
    [
        [One, One, One, Half],
        [Lit(0x2690), One, One, Zero],
        [Lit(0x268c), One, One, Zero],
        [Lit(0x2688), Lit(0x2668), Lit(0x2664), Lit(0x2684)],
        [Lit(0x2680), Lit(0x2660), Lit(0x265c), Zero],
        [Lit(0x267c), One, One, Zero],
        [Lit(0x2678), Lit(0x2658), Lit(0x2654), Lit(0x2674)],
        [Lit(0x2670), Lit(0x2650), One, One],
        [Lit(0x266c), Lit(0x264c), Lit(0x263c), Zero],
        [Lit(0x2648), Lit(0x2638), Lit(0x2634), Lit(0x2644)],
        [One, One, One, One],
        [One, Zero, Zero, Zero],
        [One, Lit(0x2640), Lit(0x2630), Lit(0x2640)],
        [One, One, One, One],
        [One, One, One, One],
        [One, Zero, Zero, Zero],
        [One, Zero, Zero, Zero],
        [One, One, One, One],
        [One, Half, One, Half],
        [One, Half, One, Half],
        [One, Half, One, Half],
        [One, Lit(0x2624), Lit(0x2620), Lit(0x262c)],
        [One, One, Lit(0x261c), Zero],
        [Lit(0x2628), Lit(0x2618), Lit(0x2614), Zero],
        [One, One, One, One],
        [One, One, Lit(0x2608), Zero],
        [One, Lit(0x2604), Lit(0x2600), Zero],
        [One, Lit(0x25fc), Lit(0x25f8), Lit(0x2610)],
        [One, Lit(0x25e4), Lit(0x25f4), One],
        [One, Lit(0x25f0), One, Lit(0x260c)],
        [One, One, One, One],
        [One, One, One, One],
        [One, Lit(0x25ec), Lit(0x25e8), Zero],
        [One, One, One, One],
        [One, Lit(0x25d0), One, Zero],
        [One, One, One, One],
        [One, One, Zero, Zero],
        [Lit(0x25e0), Zero, Zero, Zero],
        [One, Lit(0x25cc), Lit(0x25c8), Lit(0x25dc)],
        [One, Lit(0x25c4), One, Lit(0x25d8)],
        [One, One, Lit(0x25c0), Zero],
        [One, Lit(0x25bc), One, Lit(0x25d4)],
        [One, One, One, One],
        [One, Lit(0x25e4), One, Lit(0x25e4)],
        [One, One, Zero, Zero],
        [One, One, Zero, Zero],
        [One, One, Zero, Zero],
        [One, Lit(0x25b0), One, Lit(0x25b8)],
        [One, One, Lit(0x25ac), Lit(0x25b4)],
        [One, One, Lit(0x25a8), Zero],
        [One, Lit(0x25a4), Lit(0x25a0), Zero],
        [One, Lit(0x2594), Lit(0x2590), Zero],
        [One, Lit(0x258c), Lit(0x2588), Lit(0x259c)],
        [One, Lit(0x2584), Lit(0x2580), Lit(0x2598)],
        [One, Zero, One, Zero],
        [One, One, Zero, Zero],
    ]
};

pub(crate) const RENDERER_VEC4S_B_INIT: [[RendererWord; 4]; 5] = {
    use RendererWord::{Lit, One};
    [
        [Lit(0x257c), Lit(0x2574), Lit(0x2570), One],
        [Lit(0x2574), Lit(0x2574), Lit(0x2570), One],
        [Lit(0x257c), Lit(0x2574), Lit(0x2570), One],
        [Lit(0x2574), Lit(0x2574), Lit(0x2570), One],
        [One, Lit(0x256c), Lit(0x2568), Lit(0x2578)],
    ]
};

pub(crate) const RENDERER_HALF_SCALE_INIT: [[RendererWord; 4]; 4] = {
    use RendererWord::{Half, NegHalf, One, Zero};
    [
        [Half, Zero, Zero, Zero],
        [Zero, NegHalf, Zero, Zero],
        [Zero, Zero, Zero, Zero],
        [Half, Half, One, One],
    ]
};

pub(crate) const RENDERER_AXIS_INIT: [[RendererWord; 4]; 4] = {
    use RendererWord::{NegOne, One, Zero};
    [
        [Zero, Zero, One, Zero],
        [NegOne, Zero, Zero, Zero],
        [Zero, One, Zero, Zero],
        [Zero, Zero, Zero, One],
    ]
};

pub(crate) const RENDERER_ONES_INIT: [[RendererWord; 4]; 2] = {
    use RendererWord::One;
    [[One, One, One, One], [One, One, One, One]]
};

pub(crate) const RENDERER_VEC4S_C_INIT: [[RendererWord; 4]; 2] = {
    use RendererWord::{Lit, One};
    [[One, One, One, One], [Lit(0x2564), One, One, One]]
};

/// VU microcode entry points as byte offsets in the microprogram (stored as
/// instruction addresses, offset >> 3), one row per record.
pub(crate) const RENDERER_MICROCODE_ENTRIES: [[u32; 12]; 4] = [
    [
        0x610, 0x610, 0x6b8, 0x790, 0x868, 0x8a0, 0x8d8, 0x910, 0x948, 0x980, 0x948, 0x980,
    ],
    [
        0x648, 0x648, 0x700, 0x7d8, 0x868, 0x8a0, 0x8d8, 0x910, 0x948, 0x980, 0x948, 0x980,
    ],
    [
        0x680, 0x680, 0x748, 0x820, 0x868, 0x8a0, 0x8d8, 0x910, 0x948, 0x980, 0x948, 0x980,
    ],
    [
        0x930, 0x930, 0x9a0, 0xa38, 0xb00, 0xb68, 0xc48, 0xc48, 0xcf0, 0xcf0, 0xcf0, 0xcf0,
    ],
];

pub(crate) const RENDERER_MICROCODE_STRIDE: u32 = 0x60;

/// Entry addresses the compiled code spills to its frame while building the
/// records: `sw` slots, then `sd` slots (offset >> 3 each).
pub(crate) const RENDERER_MICROCODE_WORD_SPILLS: [(u32, u32); 10] = [
    (0x18, 0x610),
    (0x1c, 0x6b8),
    (0x20, 0x868),
    (0x30, 0x910),
    (0x40, 0x930),
    (0x44, 0xa38),
    (0x48, 0xb00),
    (0x4c, 0xb68),
    (0x50, 0xc48),
    (0x54, 0xc48),
];

pub(crate) const RENDERER_MICROCODE_DWORD_SPILLS: [(u32, u32); 2] = [(0x28, 0x868), (0x38, 0x910)];

/// A (1, 1, 1) the compiled code also writes at the bottom of its frame.
pub(crate) const RENDERER_ONES_SPILL: [u32; 3] = [0x0, 0x4, 0x8];

/// Registers as the first scheduled block leaves them (addresses inside
/// the tables, the stored values).
pub(crate) const RENDERER_AFTER_TABLES_GPRS: [(usize, u64); 25] = [
    (V0, 0x3e0),
    (V1, 0xc00),
    (A0, u64::MAX),
    (A1, 0x3c0),
    (A2, 0x1f),
    (A3, 0xc),
    (T0, 0x0050_1270),
    (T1, 0x0050_1260),
    (T2, 0x0180_0000),
    (T3, 0x0040_0000),
    (T4, 0x000f_f000),
    (T5, 0x0030_0000),
    (T6, 0x7c),
    (T7, 3),
    (T8, 0x0050_11e0),
    (S0, 0x0050_13a0),
    (S1, 0x0050_13f0),
    (S2, 0x0050_1370),
    (S4, RENDERER_STATICS_HI),
    (S5, RENDERER_STATICS_HI),
    (S6, RENDERER_STATICS_HI),
    (S7, RENDERER_STATICS_HI),
    (FP, 0x3fff_ffff),
    (AT, RENDERER_HALF as u64),
    (S3, RENDERER_GP_MASK),
];

/// FPU registers the first block leaves loaded: (register, literal offset
/// below gp); f0 holds 1.0.
pub(crate) const RENDERER_AFTER_TABLES_FPRS: [(usize, u32); 19] = [
    (1, 0x2598),
    (2, 0x259c),
    (3, 0x2594),
    (4, 0x2590),
    (5, 0x258c),
    (6, 0x2588),
    (7, 0x2584),
    (8, 0x2580),
    (9, 0x25c0),
    (10, 0x25bc),
    (11, 0x25e8),
    (12, 0x25e4),
    (13, 0x2660),
    (14, 0x265c),
    (15, 0x2658),
    (16, 0x2654),
    (17, 0x2668),
    (18, 0x2664),
    (19, 0x264c),
];

/// Registers after the five-vector block.
pub(crate) const RENDERER_AFTER_VEC4S_B_GPRS: [(usize, u64); 11] = [
    (V0, 0x0050_4710),
    (V1, 0x0050_4720),
    (A0, u64::MAX),
    (A1, 0x0050_46e0),
    (A2, 0x0050_46f0),
    (A3, 0x0050_4700),
    (T0, 2),
    (T1, u64::MAX),
    (T4, RENDERER_STATICS_HI),
    (T5, RENDERER_STATICS_HI),
    (AT, RENDERER_ONE as u64),
];

pub(crate) const RENDERER_AFTER_VEC4S_B_FPRS: [(usize, u32); 6] = [
    (1, 0x2578),
    (2, 0x2574),
    (3, 0x2570),
    (4, 0x257c),
    (5, 0x2568),
    (6, 0x256c),
];

/// Registers at the `Rand_GetDefaultState` call: the matrix addresses and
/// the (zero) high halves of microcode addresses used after the call.
pub(crate) const RENDERER_AT_RAND_GPRS: [(usize, u64); 9] = [
    (V0, RENDERER_HALF_SCALE_MATRIX as u64),
    (V1, RENDERER_AXIS_MATRIX as u64),
    (A0, RENDERER_RAND_STATE as u64),
    (S0, 0),
    (S1, 0),
    (S2, 0),
    (S3, 0x910),
    (S6, 0),
    (AT, 0xffff_ffff_bf00_0000),
];

/// Registers after the microcode records (record addresses and entry
/// values; V0/V1 are the first loop's counters).
pub(crate) const RENDERER_AFTER_MICROCODE_GPRS: [(usize, u64); 25] = [
    (A0, 0x0050_4900),
    (A1, 0x0050_4960),
    (A2, 0x0050_48a0),
    (A3, 0x980 >> 3),
    (S0, 0x680 >> 3),
    (S1, 0x820 >> 3),
    (S2, 0x748 >> 3),
    (S3, 0xc48 >> 3),
    (S4, RENDERER_STATICS_HI),
    (S5, 0x930),
    (S6, 0x9a0 >> 3),
    (S7, 0x820),
    (FP, 0x930 >> 3),
    (T0, 0x948 >> 3),
    (T1, 0xcf0 >> 3),
    (T2, 0xcf0 >> 3),
    (T3, 0x910 >> 3),
    (T4, 0xa38 >> 3),
    (T5, 0xb00 >> 3),
    (T6, 0xb68 >> 3),
    (T7, 0xc48 >> 3),
    (T8, 0x680 >> 3),
    (T9, 0x680),
    (V0, 5),
    (V1, 4),
];

/// Back-edge heads of the loops (event checks).
pub(crate) const RENDERER_HEAD_GLOBAL_HALVES: u32 = 0x0039_38f8;

pub(crate) const RENDERER_HEAD_HALVES: u32 = 0x0039_3930;

pub(crate) const RENDERER_HEAD_ARRAY_8: u32 = 0x0039_3968;

pub(crate) const RENDERER_HEAD_ARRAY_3: u32 = 0x0039_3a38;

pub(crate) const RENDERER_HEAD_GRID_ROWS: u32 = 0x0039_3e80;

pub(crate) const RENDERER_HEAD_GRID_COLS: u32 = 0x0039_3e88;

pub(crate) const RENDERER_HEAD_ARRAY_8B: u32 = 0x0039_3ec0;

pub(crate) const RENDERER_HEAD_ARRAY_8C: u32 = 0x0039_3ee8;

pub(crate) fn renderer_gp(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

pub(crate) fn renderer_word(c: &mut Cpu, w: RendererWord) -> u64 {
    match w {
        RendererWord::One => RENDERER_ONE as u64,
        RendererWord::Half => RENDERER_HALF as u64,
        RendererWord::NegHalf => RENDERER_NEG_HALF as u64,
        RendererWord::NegOne => RENDERER_NEG_ONE as u64,
        RendererWord::Zero => 0,
        RendererWord::Lit(off) => Addr(renderer_gp(c).wrapping_sub(off)).lw(c),
    }
}

/// Write `table` as consecutive vec4 at `base`.
pub(crate) fn renderer_write_vec4s(c: &mut Cpu, base: u32, table: &[[RendererWord; 4]]) {
    for (i, vec) in table.iter().enumerate() {
        for (k, &w) in vec.iter().enumerate() {
            let v = renderer_word(c, w);
            Addr(base + 0x10 * i as u32 + 4 * k as u32).sw(c, v);
        }
    }
}

pub(crate) fn renderer_set_gprs(c: &mut Cpu, regs: &[(usize, u64)]) {
    for &(r, v) in regs {
        c.g[r][0] = v;
    }
}

/// `lwc1 f, -off(gp)` for each entry.
pub(crate) fn renderer_load_lits(c: &mut Cpu, regs: &[(usize, u32)]) {
    for &(f, off) in regs {
        let a = renderer_gp(c).wrapping_sub(off);
        c.lwc1(f, a);
    }
}

/// `void Renderer_StaticInitDestroy(int initialize, int priority)`
/// (0x393048): gcc's static constructor/destructor body for the renderer
/// unit, called by `Renderer_GlobalCtors`/`Renderer_GlobalDtors`. Only the
/// construct pass at priority 0xffff does anything (the destroy pass has
/// nothing to destroy): it fills the unit's constant vec4 tables, matrices
/// and gp-relative words, sets two halfword pairs to -1, clears a block,
/// copies the default random state, writes four records of VU microcode
/// entry addresses, runs the (empty) constructor loops of five static
/// arrays, sets the scratchpad stream to mode 3 and builds the sine table.
pub fn renderer_static_init_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, RENDERER_INIT_FRAME);
        c.g[V0][0] = RENDERER_INIT_PRIORITY;
        for &(r, off) in &RENDERER_INIT_SAVED {
            frame.save(c, r, off);
        }
        frame.save_ra(c, RENDERER_INIT_RA_SLOT);
        for &(f, off) in &RENDERER_INIT_SAVED_FPRS {
            frame.save_fpr(c, f, off);
        }
        'construct: {
            if c.g[A1][0] != RENDERER_INIT_PRIORITY {
                break 'construct;
            }
            c.cycles += 2;
            c.g[T2][0] = RENDERER_STATICS_HI;
            if c.g[A0][0] == 0 {
                break 'construct;
            }
            renderer_construct_statics(c, frame)?;
        }
        c.cycles += 16;
        for &(r, off) in &RENDERER_INIT_SAVED {
            frame.restore(c, r, off);
        }
        frame.restore_ra(c, RENDERER_INIT_RA_SLOT);
        for &(f, off) in &RENDERER_INIT_SAVED_FPRS {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

/// The construct pass of [`renderer_static_init_destroy`].
pub(crate) fn renderer_construct_statics(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 537;
    let gp = renderer_gp(c);
    renderer_write_vec4s(c, RENDERER_VEC4S_A, &RENDERER_VEC4S_A_INIT);
    for &(off, v) in &RENDERER_GP_WORDS {
        Addr(gp.wrapping_add(off)).sw(c, v as u64);
    }
    Addr(frame.at(c, RENDERER_INIT_MASK_SLOT)).sw(c, RENDERER_GP_MASK);
    Addr(frame.at(c, RENDERER_INIT_CURSOR_SLOT)).sw(c, RENDERER_GLOBAL_HALF_PAIR as u64);
    Addr(frame.at(c, RENDERER_INIT_COUNT_SLOT)).sw(c, 0);
    let mask = Addr(frame.at(c, RENDERER_INIT_MASK_SLOT)).lw(c);
    Addr(gp.wrapping_add(RENDERER_GP_MASK_WORD)).sw(c, mask);
    renderer_set_gprs(c, &RENDERER_AFTER_TABLES_GPRS);
    c.f[0] = RENDERER_ONE;
    renderer_load_lits(c, &RENDERER_AFTER_TABLES_FPRS);

    // The global halfword pair, cursor and count kept in the frame.
    loop {
        c.cycles += 9;
        c.g[A3][0] = Addr(frame.at(c, RENDERER_INIT_CURSOR_SLOT)).lw(c);
        Addr(c.g[A3][0] as u32).sh(c, c.g[A0][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(2));
        c.g[T2][0] = Addr(frame.at(c, RENDERER_INIT_COUNT_SLOT)).lw(c);
        Addr(frame.at(c, RENDERER_INIT_CURSOR_SLOT)).sw(c, c.g[A3][0]);
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
        c.g[V0][0] = (c.g[T2][0] < 2) as u64;
        Addr(frame.at(c, RENDERER_INIT_COUNT_SLOT)).sw(c, c.g[T2][0]);
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, RENDERER_HEAD_GLOBAL_HALVES);
        }
    }

    // The halfword pair after the cleared block.
    c.cycles += 5;
    c.g[V0][0] = RENDERER_STATICS_HI;
    c.g[A1][0] = 0;
    c.g[A0][0] = sx(RENDERER_CLEARED_BLOCK);
    c.g[A2][0] = u64::MAX;
    c.g[V1][0] = sx(RENDERER_HALF_PAIR);
    loop {
        c.cycles += 7;
        Addr(c.g[V1][0] as u32).sh(c, c.g[A2][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[V0][0] = (c.g[A1][0] < 2) as u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(2));
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, RENDERER_HEAD_HALVES);
        }
    }

    c.cycles += 7;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x10;
    callee::MEMSET.call(c, 0x0039_3958)?;

    // Empty constructor loop of an 8-element static array.
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = 7;
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[T3][0] = RENDERER_STATICS_HI;
        if c.g[V0][0] == c.g[V1][0] {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, RENDERER_HEAD_ARRAY_8);
        }
    }

    c.cycles += 45;
    Addr(gp.wrapping_add(RENDERER_GP_FLAG)).sw(c, u64::MAX);
    renderer_write_vec4s(c, RENDERER_VEC4S_B, &RENDERER_VEC4S_B_INIT);
    renderer_set_gprs(c, &RENDERER_AFTER_VEC4S_B_GPRS);
    c.f[0] = RENDERER_ONE;
    renderer_load_lits(c, &RENDERER_AFTER_VEC4S_B_FPRS);

    // Empty constructor loop of a 3-element static array.
    loop {
        c.cycles += 7;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
        c.g[T6][0] = RENDERER_STATICS_HI;
        if c.g[T0][0] == c.g[T1][0] {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, RENDERER_HEAD_ARRAY_3);
        }
    }

    // The two matrices (f20-f23 keep 0, 1, 0.5 and -0.5 past the call).
    c.cycles += 267;
    c.f[22] = RENDERER_HALF;
    c.f[21] = RENDERER_ONE;
    c.f[20] = 0;
    c.f[0] = RENDERER_NEG_ONE;
    c.f[23] = RENDERER_NEG_HALF;
    renderer_write_vec4s(c, RENDERER_HALF_SCALE_MATRIX, &RENDERER_HALF_SCALE_INIT);
    renderer_write_vec4s(c, RENDERER_AXIS_MATRIX, &RENDERER_AXIS_INIT);
    renderer_set_gprs(c, &RENDERER_AT_RAND_GPRS);
    callee::RAND_GET_DEFAULT_STATE.call(c, 0x0039_3b24)?;

    renderer_write_microcode_records(c, frame);

    // Empty constructor loops of a 6x6 static array.
    loop {
        c.cycles += 2;
        c.g[V0][0] = 5;
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.g[S6][0] = u64::MAX;
            if c.g[V0][0] == c.g[S6][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, RENDERER_HEAD_GRID_COLS);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0];
        let more = c.g[V0][0] != c.g[S6][0];
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, RENDERER_HEAD_GRID_ROWS);
        }
    }

    // Empty constructor loops of two 8-element static arrays.
    c.cycles += 4;
    c.g[V0][0] = 7;
    c.g[A0][0] = u64::MAX;
    c.g[V1][0] = 6;
    loop {
        c.cycles += 8;
        c.g[V0][0] = c.g[V1][0];
        let more = c.g[V0][0] != c.g[A0][0];
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, RENDERER_HEAD_ARRAY_8B);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 7;
    c.g[V1][0] = u64::MAX;
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[A1][0] = RENDERER_SPR_STREAM_MODE;
        if c.g[V0][0] == c.g[V1][0] {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, RENDERER_HEAD_ARRAY_8C);
        }
    }

    // The scratchpad stream (from mode 0), then the sine table.
    c.cycles += 24;
    c.g[V0][0] = RENDERER_STATICS_HI;
    c.g[A0][0] = sx(RENDERER_SPR_STREAM);
    Addr(RENDERER_SPR_STREAM).sw(c, 0);
    callee::SPR_STREAM_SET_MODE.call(c, 0x0039_3f14)?;
    Addr(renderer_gp(c).wrapping_add(RENDERER_GP_STREAM_WORD)).sw(c, 0);
    c.g[A0][0] = sx(RENDERER_SINE_TABLE);
    callee::RENDERER_BUILD_SINE_TABLE.call(c, 0x0039_3f24)?;

    renderer_write_vec4s(c, RENDERER_VEC4S_C, &RENDERER_VEC4S_C_INIT);
    c.g[AT][0] = RENDERER_ONE as u64;
    c.f[0] = RENDERER_ONE;
    let lit = renderer_gp(c).wrapping_sub(0x2564);
    c.lwc1(1, lit);
    c.g[V0][0] = sx(RENDERER_VEC4S_C);
    c.g[V1][0] = sx(RENDERER_VEC4S_C + 0x10);
    c.g[A0][0] = RENDERER_STATICS_HI;
    c.g[A1][0] = RENDERER_STATICS_HI;
    Ok(())
}

/// After `Rand_GetDefaultState`: the cleared pair, the ones vectors and
/// their copy, the four microcode records, the second half-scale matrix,
/// and the frame slots and registers the compiled code leaves.
pub(crate) fn renderer_write_microcode_records(c: &mut Cpu, frame: Frame) {
    Addr(RENDERER_CLEARED_PAIR + 4).sw(c, 0);
    Addr(RENDERER_CLEARED_PAIR).sw(c, 0);
    renderer_write_vec4s(c, RENDERER_ONES, &RENDERER_ONES_INIT);
    for off in [0, 8] {
        let v = Addr(RENDERER_ONES_SOURCE + off).ld(c);
        Addr(RENDERER_ONES_COPY + off).sd(c, v);
    }
    for (i, record) in RENDERER_MICROCODE_ENTRIES.iter().enumerate() {
        let base = RENDERER_MICROCODE_TABLES + RENDERER_MICROCODE_STRIDE * i as u32;
        for (k, &entry) in record.iter().enumerate() {
            Addr(base + 8 * k as u32).sd(c, (entry >> 3) as u64);
        }
    }
    renderer_write_vec4s(c, RENDERER_HALF_SCALE_MATRIX_2, &RENDERER_HALF_SCALE_INIT);
    for &(off, entry) in &RENDERER_MICROCODE_WORD_SPILLS {
        Addr(frame.at(c, off)).sw(c, (entry >> 3) as u64);
    }
    for &(off, entry) in &RENDERER_MICROCODE_DWORD_SPILLS {
        Addr(frame.at(c, off)).sd(c, (entry >> 3) as u64);
    }
    for off in RENDERER_ONES_SPILL {
        Addr(frame.at(c, off)).sw(c, RENDERER_ONE as u64);
    }
    renderer_set_gprs(c, &RENDERER_AFTER_MICROCODE_GPRS);
}

/// `bool cPS2GridMesh_IsValid(cPS2GridMesh* self)` (0x396108, vtable
/// +0x14): whether the constructor's allocation succeeded (`VALID`).
pub fn c_ps2_grid_mesh_is_valid(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(r32(c, A0)).off(gmesh::VALID).lw(c);
    ret(c);
}

/// `Renderer_GlobalCtors` / `Renderer_GlobalDtors`: one call of
/// `Renderer_StaticInitDestroy(initialize, 0xffff)` returning to `ret_to`.
pub(crate) fn renderer_global_xtors(c: &mut Cpu, initialize: u64, ret_to: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = initialize;
        frame.save_ra(c, 0);
        c.g[A1][0] = RENDERER_INIT_PRIORITY;
        RENDERER_STATIC_INIT_DESTROY.call(c, ret_to)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PS2GraphicsMan_GetFlipCount(gfx)` (0x395510): the frame flip counter
/// at `gfx+0x5a74`.
pub fn ps2_graphics_man_get_flip_count(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
    let gfx = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(gfx.wrapping_add(0x5a74)).lw(c);
}

/// `PS2GraphicsMan* PS2GraphicsMan_ctor(PS2GraphicsMan* self)` (0x395288):
/// run the base-class constructor (`cGraphicsMan_ctor`, 0x3691f8), set the
/// vtable word at `self+0x10d8` (0x493260), burn the two short counted
/// loops the compiler left (12 and 4 passes, no stores in the generated
/// body) and return `self`.
pub fn ps2_graphics_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0039_529c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0036_91f8, 0x0039_529c)?;
        c.g[V0][0] = 0x49_0000;
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3260));
        c.g[V1][0] = 0xb;
        let vtable = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x10d8)).sw(c, vtable);
        'l_3952b0: loop {
            c.cycles += 7;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let more = c.g[V1][0] != c.g[A0][0];
            c.g[V0][0] = 0x7;
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0039_52b0);
                }
                continue 'l_3952b0;
            } else {
                break 'l_3952b0;
            }
        }
        c.cycles += 7;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 0x1;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 0x1;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 0x3;
        c.g[V1][0] = u64::MAX;
        'l_3952e8: loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let more = c.g[V0][0] != c.g[V1][0];
            frame.restore_ra(c, 0);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0039_52e8);
                }
                continue 'l_3952e8;
            } else {
                break 'l_3952e8;
            }
        }
        c.cycles += 4;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Renderer_GlobalCtors()` (0x3970f8): run the renderer unit's static
/// constructors (`Renderer_StaticInitDestroy(1, 0xffff)`).
pub fn renderer_global_ctors(c: &mut Cpu) {
    renderer_global_xtors(c, 1, 0x0039_710c);
}

/// `void Renderer_GlobalDtors()` (0x397118): run the renderer unit's static
/// destructors (`Renderer_StaticInitDestroy(0, 0xffff)`).
pub fn renderer_global_dtors(c: &mut Cpu) {
    renderer_global_xtors(c, 0, 0x0039_712c);
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan fields (besides `GM_*` in `game::render::particle`).

/// The 0xa0-byte lighting block (`PS2GraphicsMan_SetLighting` copies it in).
pub(crate) const GM_LIGHTING: u32 = 0x6bb0;

// ---------------------------------------------------------------------------
// Constructor and destructor.

/// `PS2GraphicsMan::~PS2GraphicsMan(int flags)` (0x395268): forwards to the
/// base `GraphicsMan_dtor` (which resets the vtable, clears the manager
/// global and frees the object when `flags & 1`).
pub fn ps2_graphics_man_dtor(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(callee::GRAPHICS_MAN_DTOR),
        0x0039_5278,
    );
}

// ---------------------------------------------------------------------------
// Small accessors.

/// `int PS2GraphicsMan::GetDisplayModeCount()` (0x395318): the number of
/// display modes (`+0x59c4`).
pub fn ps2_graphics_man_get_display_mode_count(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(renderer_fields::MODE_COUNT).lw(c);
    ret(c);
}

/// `int PS2GraphicsMan::IsProgressive()` (0x395320): the progressive-scan
/// flag (`+0x5a4c`).
pub fn ps2_graphics_man_is_progressive(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(renderer_fields::PROGRESSIVE).lw(c);
    ret(c);
}

/// `int PS2GraphicsMan::GetDisplayModeIndex()` (0x395328): the current
/// display mode (`+0x59c8`).
pub fn ps2_graphics_man_get_display_mode_index(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(renderer_fields::CUR_MODE).lw(c);
    ret(c);
}

/// `void* PS2GraphicsMan::GetLighting()` (0x395500): the address of the
/// lighting block (`this + 0x6bb0`).
pub fn ps2_graphics_man_get_lighting(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(GM_LIGHTING));
    ret(c);
}

/// `void PS2GraphicsMan::v98(int value)` (0x3956e0): store `value` in the
/// unknown word at `+0x6d14`.
pub fn ps2_graphics_man_v98(c: &mut Cpu) {
    c.cycles += 2;
    let value = c.g[A1][0];
    Addr(abi::arg(c, 0))
        .off(renderer_fields::F6D14)
        .sw(c, value);
    ret(c);
}

/// `void PS2GraphicsMan::v68()` (0x395fa0): an empty virtual method.
pub fn ps2_graphics_man_v68(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_v33 (0x395750): view matrix from a position and angles.

/// `v33`'s three rotation axes (static vec4s set at boot), in the order the
/// angles in f12, f13 and f14 use them.
pub(crate) const V33_AXIS_1: u32 = 0x004f_f140;

pub(crate) const V33_AXIS_2: u32 = 0x004f_f160;

pub(crate) const V33_AXIS_3: u32 = 0x004f_f150;

/// The base `v33` keeps in s0 across its calls and reads the axes relative
/// to.
pub(crate) const V33_AXES_BASE: u32 = 0x0050_0000;

/// `v33`'s stack: the scratch matrix (rows at 0..0x40, also the translation
/// vector), the sine/cosine pairs `Math_SinCos` writes for each angle, and
/// the saved registers.
pub(crate) const V33_FRAME: u32 = 0xe0;

pub(crate) const V33_SCRATCH: u32 = 0x0;

pub(crate) const V33_SIN_COS_1: u32 = 0x40;

pub(crate) const V33_SIN_COS_2: u32 = 0x48;

pub(crate) const V33_SIN_COS_3: u32 = 0x50;

pub(crate) const V33_SAVED_RA: u32 = 0x60;

/// One rotation of `v33`: from the sine and cosine at `sp+sin_cos` and
/// `sp+sin_cos+4` and the axis (x at `axis_x`, y and z at `axis+4` and
/// `axis+8`), build the axis-angle rotation matrix in the scratch matrix
/// (`t = 1 - cos`; `r[i][j] = t*a[j]*a[i] + cos` on the diagonal, `-/+ sin*a[k]`
/// off it), then multiply the current matrix by it in place. The FPU
/// registers are left as the original's straight-line code leaves them.
pub(crate) fn v33_rotate_about_axis(
    c: &mut Cpu,
    frame: Frame,
    sin_cos: u32,
    axis: u32,
    axis_x: u32,
    matrix: u32,
) {
    let cos = Addr(frame.at(c, sin_cos + 4)).read_fpu(c);
    let z = Addr(axis).off(8).read_fpu(c);
    let t = FpuF32(c.f[20]) - cos;
    let x = Addr(axis_x).read_fpu(c);
    let y = Addr(axis).off(4).read_fpu(c);
    let sin = Addr(frame.at(c, sin_cos)).read_fpu(c);

    let (tz, tx, ty) = (t * z, t * x, t * y);
    let (sin_z, sin_x) = (sin * z, sin * x);
    let (tzz, txz, tzx, txx) = (tz * z, tx * z, tz * x, tx * x);
    let (tyz, tyx) = (ty * z, ty * x);
    let sin_y = sin * y;
    let (tzy, txy, tyy) = (tz * y, tx * y, ty * y);

    let m02 = tzx + sin_y;
    let m01 = tyx - sin_z;
    let m12 = tzy - sin_x;
    let m22 = tzz + cos;
    let m00 = txx + cos;
    let m10 = txy + sin_z;
    let m20 = txz - sin_y;
    let m11 = tyy + cos;
    let m21 = tyz + sin_x;

    // Rows 0-2 with w = 0, row 3 = (0, 0, 0, 1.0 from f20).
    let rows = [
        [m00, m01, m02, FpuF32::ZERO],
        [m10, m11, m12, FpuF32::ZERO],
        [m20, m21, m22, FpuF32::ZERO],
        [FpuF32::ZERO, FpuF32::ZERO, FpuF32::ZERO, FpuF32(c.f[20])],
    ];
    let scratch = frame.at(c, V33_SCRATCH);
    for (i, row) in rows.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            Addr(scratch + 0x10 * i as u32 + 4 * j as u32).write_fpu(c, *v);
        }
    }
    for (reg, v) in [
        (0, m01),
        (1, m11),
        (2, m12),
        (3, m21),
        (4, m10),
        (5, m02),
        (6, y),
        (7, m00),
        (8, sin_y),
        (9, m22),
        (10, m20),
        (11, cos),
        (12, sin_x),
        (13, sin_z),
    ] {
        c.f[reg] = v.0;
    }

    // matrix = scratch * matrix (row i = sum over j of scratch[i][j] * row j).
    c.lqc2(4, matrix);
    c.lqc2(5, matrix.wrapping_add(0x10));
    c.lqc2(6, matrix.wrapping_add(0x20));
    c.lqc2(7, matrix.wrapping_add(0x30));
    for i in 0..4u32 {
        c.lqc2(8 + i as usize, scratch + 0x10 * i);
    }
    for i in 0..4u8 {
        let (row, out) = (8 + i, 12 + i);
        c.vu0().mula(0b1111, 4, Src::Bc(row, X));
        c.vu0().madda(0b1111, 5, Src::Bc(row, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(row, Z));
        c.vu0().madd(0b1111, out, 7, Src::Bc(row, W));
    }
    for i in 0..4u32 {
        c.sqc2(12 + i as usize, matrix.wrapping_add(0x10 * i));
    }
}

/// `void PS2GraphicsMan::v33(const Vec4* pos, float rx, float ry, float rz)`
/// (0x395750): build the view matrix in the current matrix: the identity
/// with its y and z rows swapped, rotated by `-rx`, `-ry` and `-rz` about
/// the three static axes (one `Math_SinCos` each), then translated by
/// `(-pos.x, -pos.y, -pos.z, pos.w)`; marks the view matrices stale.
pub fn ps2_graphics_man_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 313;
        let frame = Frame::open(c, V33_FRAME);
        frame.save(c, S2, 0x90);
        frame.save(c, S4, 0x70);
        frame.save_fpr(c, 22, 0xd0);
        frame.save_fpr(c, 21, 0xc8);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S3, 0x80);
        frame.save_ra(c, V33_SAVED_RA);
        frame.save_fpr(c, 20, 0xc0);
        c.g[S2][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        c.mov_s(22, 14);
        c.mov_s(21, 13);

        // Current matrix = identity, then swap the y and z rows:
        // row 1 = (0, 0, 1, 0), row 2 = (0, 1, 0, 0).
        let this = Addr(c.g[S2][0] as u32);
        c.g[V1][0] = sx(IDENTITY_MATRIX);
        c.g[V0][0] = this.off(man::CAMERA).lw(c);
        let (identity, top) = (c.g[V1][0] as u32, c.g[V0][0] as u32);
        for i in 0..4u32 {
            c.lqc2(1 + i as usize, identity + 0x10 * i);
        }
        for i in 0..4u32 {
            c.sqc2(1 + i as usize, top.wrapping_add(0x10 * i));
        }
        c.g[A1][0] = this.off(man::CAMERA).lw(c);
        c.neg_s(12, 12);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 20); // f20 = 1.0
        c.g[A0][0] = sx(frame.at(c, V33_SIN_COS_1));
        Addr(c.g[A1][0] as u32).off(0x14).sw(c, 0);
        c.g[S0][0] = sx(V33_AXES_BASE);
        c.g[A1][0] = sx(frame.at(c, V33_SIN_COS_1 + 4));
        c.g[S1][0] = sx(V33_AXIS_1);
        c.g[V1][0] = this.off(man::CAMERA).lw(c);
        c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x24));
        c.g[V0][0] = this.off(man::CAMERA).lw(c);
        c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = this.off(man::CAMERA).lw(c);
        Addr(c.g[V1][0] as u32).off(0x28).sw(c, 0);
        c.g[S3][0] = this.off(man::CAMERA).lw(c);

        // Rotate by -rx about the first axis (s1; its x read through s0).
        callee::MATH_SIN_COS.call(c, 0x0039_57f8)?;
        let axis = c.g[S1][0] as u32;
        let axis_x = (c.g[S0][0] as u32).wrapping_sub(V33_AXES_BASE - V33_AXIS_1);
        let matrix = c.g[S3][0] as u32;
        v33_rotate_about_axis(c, frame, V33_SIN_COS_1, axis, axis_x, matrix);

        // Rotate by -ry about the second axis.
        c.neg_s(12, 21);
        c.g[S1][0] = Addr(c.g[S2][0] as u32).off(man::CAMERA).lw(c);
        c.g[A0][0] = sx(frame.at(c, V33_SIN_COS_2));
        c.g[A1][0] = sx(frame.at(c, V33_SIN_COS_2 + 4));
        c.g[S0][0] = sx(V33_AXES_BASE);
        callee::MATH_SIN_COS.call(c, 0x0039_5938)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(V33_AXES_BASE - V33_AXIS_2));
        let axis = c.g[V0][0] as u32;
        let matrix = c.g[S1][0] as u32;
        v33_rotate_about_axis(c, frame, V33_SIN_COS_2, axis, axis, matrix);

        // Rotate by -rz about the third axis.
        c.neg_s(12, 22);
        c.g[S1][0] = Addr(c.g[S2][0] as u32).off(man::CAMERA).lw(c);
        c.g[A0][0] = sx(frame.at(c, V33_SIN_COS_3));
        c.g[A1][0] = sx(frame.at(c, V33_SIN_COS_3 + 4));
        c.g[S0][0] = sx(V33_AXES_BASE);
        callee::MATH_SIN_COS.call(c, 0x0039_5a7c)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(V33_AXES_BASE - V33_AXIS_3));
        let axis = c.g[V0][0] as u32;
        let matrix = c.g[S1][0] as u32;
        v33_rotate_about_axis(c, frame, V33_SIN_COS_3, axis, axis, matrix);

        // Translate by v = pos * -1.0 with its w negated back.
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 0); // f0 = -1.0
        c.mfc1(V1, 0);
        c.lqc2(4, c.g[S4][0] as u32);
        c.qmtc2(V1, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        let v = frame.at(c, V33_SCRATCH);
        c.sqc2(5, v + 0x10);
        c.lq(V0, v + 0x10);
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(man::CAMERA).lw(c);
        c.sq(V0, v);
        c.lwc1(0, v + 0xc);
        c.neg_s(0, 0);
        c.swc1(0, v + 0xc);
        // Row 3 of the current matrix = v transformed by it.
        let top = c.g[V1][0] as u32;
        c.lqc2(4, top);
        c.lqc2(5, top.wrapping_add(0x10));
        c.lqc2(6, top.wrapping_add(0x20));
        c.lqc2(7, top.wrapping_add(0x30));
        c.lqc2(8, v);
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, top.wrapping_add(0x30));
        Addr(c.g[S2][0] as u32).off(man::TEX_READY).sw(c, 0);

        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore_ra(c, V33_SAVED_RA);
        frame.restore_fpr(c, 22, 0xd0);
        frame.restore_fpr(c, 21, 0xc8);
        frame.restore_fpr(c, 20, 0xc0);
        frame.close(c);
        Ok(())
    })
}

/// `PS2GraphicsMan::PopMatrix(this)` (0x395730): drop the top matrix of the
/// view stack (`top -= 0x40`, `depth -= 1`) and clear the stack-dirty word.
pub fn ps2_graphics_man_pop_matrix(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi_a0(c);
    c.g[V0][0] = Addr(this).off(0x13e4).lw(c);
    c.g[V1][0] = Addr(this).off(0x13e0).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x40));
    Addr(this).off(0x6b90).sw(c, 0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
    let v0 = c.g[V0][0];
    Addr(this).off(0x13e4).sw(c, v0);
    let t = c.g[RA][0] as u32;
    let v1 = c.g[V1][0];
    Addr(this).off(0x13e0).sw(c, v1);
    c.pc = t;
}

/// `float* PS2GraphicsMan::GetMatrix(this)` (0x395c68): the current top
/// matrix of the view stack.
pub fn ps2_graphics_man_get_matrix(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = Addr(abi_a0(c)).off(0x13e4).lw(c);
    c.pc = t;
}

#[inline]
pub(crate) fn abi_a0(c: &Cpu) -> u32 {
    c.g[A0][0] as u32
}

#[inline]
pub(crate) fn o760_lhu(c: &mut Cpu, base: usize, off: u32) -> u64 {
    let a = o760_ea(c, base, off);
    Addr(a).lhu(c)
}

#[inline]
pub(crate) fn o760_lbu(c: &mut Cpu, base: usize, off: u32) -> u64 {
    let a = o760_ea(c, base, off);
    Addr(a).lbu(c)
}

#[inline]
pub(crate) fn o760_sh(c: &mut Cpu, base: usize, off: u32, val: usize) {
    let a = o760_ea(c, base, off);
    let v = c.g[val][0];
    Addr(a).sh(c, v);
}

/// `void Font_ParseGlyphs(Font* font, const FontFile* file)` (0x391788,
/// reference `cFont_linkFont`): builds the font's glyph table from a font
/// file. Allocates `count` 12-byte glyph records (`font+4`), copies the 11
/// meaningful bytes of each record from the file, extends the run of
/// consecutive codes `font+0x10`, points `font+8` at the glyph of code 0xac,
/// remembers the last glyph of code 0x20 (space) and gives a glyph of code
/// 0xa0 (no-break space) that space's metrics, tracks the tallest glyph
/// extent (`font+0x14`), then uploads the font texture.
pub fn font_parse_glyphs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = 0xc;
        c.g[A1][0] = 0x4a0000;
        c.g[V1][0] = o760_lb(c, S1, 0x10);
        c.g[A1][0] = o760_addi(c, A1, 0x46f8);
        c.g[A3][0] = o760_lb(c, S1, 0x11);
        c.mtc1(3, 0);
        c.cvt_s_w(0, 0);
        c.g[V0][0] = o760_lw(c, S1, 0x14);
        c.mtc1(7, 1);
        c.cvt_s_w(1, 1);
        c.g[S2][0] = o760_add(c, S1, V0); // the file's glyph records
        c.g[A3][0] = 0;
        let a = frame.at(c, 0);
        c.swc1(0, a);
        let a = frame.at(c, 4);
        c.swc1(1, a);
        let a = frame.at(c, 7);
        c.ldl(2, a);
        let a = frame.at(c, 0);
        c.ldr(2, a);
        let a = o760_ea(c, S0, 0x1f);
        c.sdl(2, a);
        let a = o760_ea(c, S0, 0x18);
        c.sdr(2, a);
        c.g[V0][0] = o760_lh(c, S1, 0xa); // glyph count
        c.mult(0, 0, 2, 4);
        c.g[A0][0] = c.lo[0];
        o760_sw(c, S0, 0, V0);
        call_ptr(c, 0x0031_7e30, 0x0039_1800)?; // alloc(count * 12)
        o760_sw(c, S0, 0x4, V0);
        c.g[T2][0] = 0;
        o760_sw0(c, S0, 0x14);
        c.g[T1][0] = 0;
        c.g[V1][0] = o760_lw(c, S0, 0);
        c.g[V0][0] = o760_lhu(c, S2, 0);
        o760_sw(c, S0, 0x10, V0);
        let no_glyphs = (c.g[V1][0] as i64) <= 0;
        o760_sw(c, S0, 0xc, V0);
        'glyphs: {
            if no_glyphs {
                break 'glyphs;
            }
            c.cycles += 5;
            c.g[T4][0] = 0xac;
            c.g[T3][0] = 0xa0;
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = 0;
            loop {
                // Copy the 11 bytes of glyph t1.
                'copy: {
                    c.cycles += 33;
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lhu(c, A3, 0);
                    c.g[V0][0] = o760_add(c, T0, V0);
                    o760_sh(c, V0, 0, A0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A3, 0x2);
                    c.g[V1][0] = o760_add(c, T0, V1);
                    o760_sb(c, V1, 0x2, A0);
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A3, 0x3);
                    c.g[V0][0] = o760_add(c, T0, V0);
                    o760_sb(c, V0, 0x3, A0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lhu(c, A3, 0x4);
                    c.g[V1][0] = o760_add(c, T0, V1);
                    o760_sh(c, V1, 0x4, A0);
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lhu(c, A3, 0x6);
                    c.g[V0][0] = o760_add(c, T0, V0);
                    o760_sh(c, V0, 0x6, A0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A3, 0x8);
                    c.g[V1][0] = o760_add(c, T0, V1);
                    o760_sb(c, V1, 0x8, A0);
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A3, 0x9);
                    c.g[V0][0] = o760_add(c, T0, V0);
                    o760_sb(c, V0, 0x9, A0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[V0][0] = o760_lbu(c, A3, 0xa);
                    c.g[V1][0] = o760_add(c, T0, V1);
                    let first = c.g[T1][0] == 0;
                    o760_sb(c, V1, 0xa, V0);
                    if first {
                        break 'copy;
                    }
                    // Extend the run of consecutive codes (font+0x10).
                    c.cycles += 7;
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[V1][0] = o760_lw(c, S0, 0x10);
                    c.g[V0][0] = o760_add(c, T0, V0);
                    c.g[V0][0] = o760_lhu(c, V0, 0);
                    c.g[V1][0] = o760_addi(c, V1, 1);
                    if c.g[V0][0] == c.g[V1][0] {
                        o760_sw(c, S0, 0x10, V0);
                    }
                }
                // Code 0xac: font+8 points at its record; code 0x20: remember the index (t2).
                'space: {
                    c.cycles += 8;
                    c.g[A1][0] = 0xc;
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.mult(0, 6, 9, 5);
                    c.g[A0][0] = o760_add(c, A2, V0);
                    c.g[V1][0] = o760_lhu(c, A0, 0);
                    c.g[V0][0] = c.g[V1][0] ^ 0x20;
                    let not_0xac = c.g[V1][0] != c.g[T4][0];
                    if c.g[V0][0] == 0 {
                        c.g[T2][0] = c.g[T1][0];
                    }
                    if not_0xac {
                        break 'space;
                    }
                    c.cycles += 1;
                    o760_sw(c, S0, 0x8, A0);
                }
                // A no-break space (0xa0) takes the metrics of the last space (record t2).
                'nbsp: {
                    c.cycles += 5;
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_add(c, A2, V0);
                    c.g[V1][0] = o760_lhu(c, A0, 0);
                    if c.g[V1][0] != c.g[T3][0] {
                        c.g[V1][0] = o760_lw(c, S0, 0x4);
                        break 'nbsp;
                    }
                    c.cycles += 29;
                    c.mult(0, 2, 10, 5);
                    c.g[A1][0] = o760_add(c, V0, S2);
                    c.g[V0][0] = o760_lbu(c, A1, 0x2);
                    o760_sb(c, A0, 0x2, V0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A1, 0x3);
                    c.g[V1][0] = o760_add(c, A2, V1);
                    o760_sb(c, V1, 0x3, A0);
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lhu(c, A1, 0x4);
                    c.g[V0][0] = o760_add(c, A2, V0);
                    o760_sh(c, V0, 0x4, A0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lhu(c, A1, 0x6);
                    c.g[V1][0] = o760_add(c, A2, V1);
                    o760_sh(c, V1, 0x6, A0);
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A1, 0x8);
                    c.g[V0][0] = o760_add(c, A2, V0);
                    o760_sb(c, V0, 0x8, A0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A1, 0x9);
                    c.g[V1][0] = o760_add(c, A2, V1);
                    o760_sb(c, V1, 0x9, A0);
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[V1][0] = o760_lbu(c, A1, 0xa);
                    c.g[V0][0] = o760_add(c, A2, V0);
                    o760_sb(c, V0, 0xa, V1);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                }
                // Track the greatest glyph extent (y offset + height) in font+0x14.
                {
                    c.cycles += 8;
                    c.g[A0][0] = o760_lw(c, S0, 0x14);
                    c.g[V1][0] = o760_add(c, A2, V1);
                    c.g[A1][0] = o760_lb(c, V1, 0xa);
                    c.g[V0][0] = o760_lbu(c, V1, 0x3);
                    c.g[V0][0] = o760_add(c, V0, A1);
                    c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        o760_sw(c, S0, 0x14, V0);
                    }
                }
                c.cycles += 6;
                c.g[V0][0] = o760_lw(c, S0, 0);
                c.g[T1][0] = o760_addi(c, T1, 1);
                c.g[A3][0] = o760_addi(c, A3, 0xc);
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T0][0] = o760_addi(c, T0, 0xc);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_1838);
                    }
                    continue;
                }
                break 'glyphs;
            }
        }
        c.cycles += 10;
        c.g[A1][0] = o760_lw(c, S1, 0x1c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = o760_add(c, S1, A1);
        Callee {
            addr: 0x0039_1a48,
            f: ee::s12::Font_UploadTexture,
        }
        .call(c, 0x0039_19cc)?;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Font_UploadTexture(Font* self, FontTexHdr* img)` (0x391a48,
/// `cFont::downloadTexture`): works out the power-of-two texture height,
/// stores the UV scale (`1/height`, `1/width`) in the font, builds a 4-bit
/// pixel buffer (`w * height / 2` bytes, copied from the header's shape data)
/// and a 16-entry grey-alpha palette, uploads both through the graphics
/// manager (`AddTexture1`) and frees the temporaries.
pub fn font_upload_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S0][0] = c.g[A1][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S1][0] = 0x1;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S4][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.g[S5][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
        c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x6));
        c.g[S1][0] = sx((c.g[S1][0] as u32) << 1);
        'l_391a80: loop {
            c.cycles += 7;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
            if c.g[V0][0] == 0u64 {
                c.g[S1][0] = sx((c.g[S1][0] as u32) << 1);
                if event_due(c) {
                    deliver_event_here(c, 0x00391a80);
                }
                continue 'l_391a80;
            } else {
                'b_391b58: {
                    c.cycles += 45;
                    c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.mtc1(17, 1);
                    c.cvt_s_w(1, 1);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.g[A1][0] = 0x490000;
                    c.mtc1(2, 2);
                    c.cvt_s_w(2, 2);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2f40));
                    c.div_s(1, 0, 1);
                    c.g[A2][0] = 0x100;
                    c.g[A3][0] = 0u64;
                    c.div_s(0, 0, 2);
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x27));
                    c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x27));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.mult(0, 4, 4, 17);
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 1) as u32);
                    call_ptr(c, 0x00317e30, 0x00391af8)?;
                    c.g[A1][0] = 0x490000;
                    c.g[A3][0] = 0u64;
                    c.g[A2][0] = 0x100;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2f50));
                    c.g[A0][0] = 0x40;
                    call_ptr(c, 0x00317e30, 0x00391b14)?;
                    c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = 0u64;
                    c.mult(0, 6, 6, 17);
                    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 1) as u32);
                    call_ptr(c, 0x003e6448, 0x00391b30)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x40;
                    call_ptr(c, 0x003e6448, 0x00391b40)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x1000;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_391b58;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        break 'b_391b58;
                    }
                }
                c.cycles += 12;
                c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[S3][0];
                c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x6));
                c.mult(0, 6, 6, 2);
                c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 1) as u32);
                call_ptr(c, 0x003e6574, 0x00391b70)?;
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x64));
                c.g[T0][0] = 0xffffffffffffff80;
                c.g[A3][0] = 0xf;
                c.g[T1][0] = 0u64;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.g[S2][0];
                'l_391b88: loop {
                    'b_391ba8: {
                        c.cycles += 8;
                        c.g[V0][0] = sx((c.g[A1][0] as u32) << 7);
                        c.sb(c.g[A0][0] as u32, c.g[T0][0]);
                        c.div(0, 2, 7);
                        c.sb((c.g[A0][0] as u32).wrapping_add(0x1), c.g[T0][0]);
                        c.sb((c.g[A0][0] as u32).wrapping_add(0x2), c.g[T0][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        if c.g[A3][0] == c.g[T1][0] {
                            break 'b_391ba8;
                        } else {
                            break 'b_391ba8;
                        }
                    }
                    c.cycles += 5;
                    c.g[V1][0] = ((c.g[A1][0] as i64) < 16i64) as u64;
                    c.g[V0][0] = c.lo[0];
                    c.sb((c.g[A0][0] as u32).wrapping_add(0x3), c.g[V0][0]);
                    let t = c.g[V1][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00391b88);
                        }
                        continue 'l_391b88;
                    } else {
                        'b_391c10: {
                            c.cycles += 19;
                            c.g[T3][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10d8));
                            c.g[V0][0] = 0x1;
                            c.g[A3][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0xffffffffffffffff;
                            c.g[A0][0] = c.lh((c.g[T3][0] as u32).wrapping_add(0x178));
                            c.g[T0][0] = c.g[S1][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                            c.g[A1][0] = c.g[S3][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.sw(c.g[SP][0] as u32, c.g[S2][0]);
                            c.g[T1][0] = 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[T2][0] = 0xb;
                            c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x17c));
                            let t = c.g[V0][0] as u32;
                            c.g[T3][0] = 0x1;
                            call_ptr(c, t, 0x00391c00)?;
                            let t = c.g[S3][0] == 0u64;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x60), c.g[V0][0]);
                            if t {
                                break 'b_391c10;
                            } else {
                                c.cycles += 2;
                                c.g[A0][0] = c.g[S3][0];
                                call_ptr(c, 0x00317e98, 0x00391c10)?;
                                break 'b_391c10;
                            }
                        }
                        'b_391c24: {
                            c.cycles += 2;
                            let t = c.g[S2][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                            if t {
                                break 'b_391c24;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x00317e98, 0x00391c20)?;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                break 'b_391c24;
                            }
                        }
                        c.cycles += 8;
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `DeferMan* DeferMan_ctor(DeferMan* self)` (0x394d50).
///
/// Constructs the deferred-work manager (`new (self) sFontCache`): fills
/// three tables of `-1` halfword pairs (2599 records of 0x80 bytes from
/// `+0x80`, 1023 records of 0x18 bytes from `+0x51484`, one 0x10-byte record
/// at `+0x69ca8`), each record followed by `memset(rec, 0, 0x10)`, then
/// runs two empty counting loops (0x800 and 12 iterations). Returns `self`.
#[allow(unused_labels, unused_assignments, unused_parens, unreachable_code)]
pub fn defer_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.g[V0][0] = 0xa27;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x80));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = 0xffffffffffffffff;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S4][0] = 0xffffffffffffffff;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        'l_394d80: loop {
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x80));
            c.g[A1][0] = 0u64;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            'l_394d90: loop {
                c.cycles += 7;
                c.sh(c.g[V1][0] as u32, c.g[S2][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00394d90);
                    }
                    continue 'l_394d90;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00394db8);
                    c.g[A2][0] = 0x10;
                    call_ptr(c, 0x00416210, 0x00394db8)?;
                    c.g[V0][0] = c.g[S1][0];
                    let t = c.g[V0][0] != c.g[S4][0];
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00394d80);
                        }
                        continue 'l_394d80;
                    } else {
                        c.cycles += 7;
                        c.g[V0][0] = 0x50000;
                        c.g[V1][0] = 0x3ff;
                        c.g[V0][0] = c.g[V0][0] | 0x1484;
                        c.g[S2][0] = 0xffffffffffffffff;
                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[S4][0] = 0xffffffffffffffff;
                        'l_394de0: loop {
                            c.cycles += 4;
                            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = 0u64;
                            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                            'l_394df0: loop {
                                c.cycles += 7;
                                c.sh(c.g[V1][0] as u32, c.g[S2][0]);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00394df0);
                                    }
                                    continue 'l_394df0;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0u64;
                                    c.g[RA][0] = sx(0x00394e18);
                                    c.g[A2][0] = 0x10;
                                    call_ptr(c, 0x00416210, 0x00394e18)?;
                                    c.g[V1][0] = c.g[S1][0];
                                    let t = c.g[V1][0] != c.g[S4][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00394de0);
                                        }
                                        continue 'l_394de0;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x7ff;
                                        c.g[V1][0] = 0xffffffffffffffff;
                                        'l_394e30: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                            let t = c.g[V0][0] != c.g[V1][0];
                                            c.g[A1][0] = 0u64;
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00394e30);
                                                }
                                                continue 'l_394e30;
                                            } else {
                                                c.cycles += 5;
                                                c.g[V0][0] = 0x60000;
                                                c.g[V0][0] = c.g[V0][0] | 0x9ca8;
                                                c.g[A2][0] = 0xffffffffffffffff;
                                                c.g[A0][0] = sx((c.g[S3][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[V1][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x10));
                                                'l_394e60: loop {
                                                    c.cycles += 7;
                                                    c.sh(c.g[V1][0] as u32, c.g[A2][0]);
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                    c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x2));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x00394e60);
                                                        }
                                                        continue 'l_394e60;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[A1][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00394e88);
                                                        c.g[A2][0] = 0x10;
                                                        call_ptr(c, 0x00416210, 0x00394e88)?;
                                                        c.g[V1][0] = 0xffffffffffffffff;
                                                        c.g[V0][0] = 0xb;
                                                        'l_394e90: loop {
                                                            c.cycles += 7;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            let t = c.g[V0][0] != c.g[V1][0];
                                                            c.lq(
                                                                17,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x40),
                                                            );
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x00394e90,
                                                                    );
                                                                }
                                                                continue 'l_394e90;
                                                            } else {
                                                                c.cycles += 8;
                                                                c.g[V0][0] = c.g[S3][0];
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x50),
                                                                );
                                                                c.lq(
                                                                    18,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.lq(
                                                                    19,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lq(
                                                                    20,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.g[RA][0] =
                                                                    c.ld(c.g[SP][0] as u32);
                                                                let t = c.g[RA][0] as u32;
                                                                c.g[SP][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x60));
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
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    })
}

/// `void UIEngine_StoreWord(int* dst, int value)` (0x397138): `*dst = value`.
pub fn ui_engine_store_word(c: &mut Cpu) {
    c.cycles += 2;
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    ret(c);
}

pub(crate) mod font_res {

    /// Glyph count (binary-search upper bound).
    pub const COUNT: u32 = 0x0;

    /// Glyph table (sorted by code above `last`).
    pub const TABLE: u32 = 0x4;

    /// Default glyph (code not found).
    pub const DEF: u32 = 0x8;

    /// First directly indexed code.
    pub const FIRST: u32 = 0xc;

    /// Last directly indexed code.
    pub const LAST: u32 = 0x10;

    /// Horizontal padding subtracted from the width.
    pub const PAD_X: u32 = 0x28;
}

pub(crate) mod glyph {

    /// Advance width (signed byte).
    pub const ADV: u32 = 0x8;

    /// Left bearing (signed byte).
    pub const XOFF: u32 = 0x9;

    /// Pixel width (byte).
    pub const WIDTH: u32 = 0x2;
}

/// `unsigned short *Font_FindLineBreakW(font, str, dst, max, sx, width)`
/// (0x392680): copies into `dst` as much of `str` as fits in `width`,
/// breaking at the last space; returns the next line's start (0 when the
/// whole string fit).
pub fn font_find_line_break_w(c: &mut Cpu) {
    c.cycles += 17;
    let frame = Frame::open(c, 0x60);
    c.g[T3][0] = c.g[A0][0];
    let font = r32(c, T3);
    frame.save(c, S0, 0x50);
    c.g[T0][0] = c.g[A1][0];
    frame.save(c, S1, 0x40);
    frame.save(c, S2, 0x30);
    frame.save(c, S3, 0x20);
    frame.save(c, S4, 0x10);
    frame.save(c, S5, 0);
    c.mtc1(0, 1);
    c.lwc1(0, font.wrapping_add(font_res::PAD_X));
    c.abs_s(0, 0);
    c.sub_s(13, 13, 0);
    c.c_lt_s(13, 1);
    if c.fcc() {
        c.mov_s(13, 1);
    }
    'b_3928e4: {
        'b_3928e0: {
            'b_3928d4: {
                'b_3928cc: {
                    'b_392884: {
                        c.cycles += 8;
                        c.mov_s(3, 1);
                        c.g[V0][0] = c.lhu(r32(c, T0));
                        c.g[S1][0] = c.g[T0][0];
                        c.mov_s(4, 3);
                        c.g[T8][0] = 0xffffffffffffffff;
                        c.g[T5][0] = 0u64;
                        let empty = c.g[V0][0] == 0u64;
                        c.g[T9][0] = 0u64;
                        if empty {
                            break 'b_392884;
                        }
                        c.cycles += 7;
                        c.g[S3][0] = sx(r32(c, A3).wrapping_sub(0x1));
                        c.g[S2][0] = 0xc;
                        c.g[S4][0] = 0xffffffffffffffff;
                        c.g[T7][0] = sx(r32(c, T0).wrapping_sub(0x2));
                        c.g[T6][0] = c.g[A2][0];
                        c.g[V0][0] = c.lhu(r32(c, T0));
                        'scan: loop {
                            'b_392774: {
                                // A space continues to the width check below;
                                // any other code goes to the glyph lookup.
                                c.cycles += 4;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[V0][0] = c.g[V0][0] ^ 0x20;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V1][0] = c.lhu(r32(c, T0));
                                    break 'b_392774;
                                } else {
                                    c.cycles += 4;
                                    c.c_le_s(13, 3);
                                    let room = !c.fcc();
                                    c.g[V0][0] = sx((c.g[T8][0] as u32) << 1);
                                    if !room {
                                        // The line is full: terminate it at the
                                        // last space and return past the spaces.
                                        c.cycles += 6;
                                        c.g[A1][0] = 0x20;
                                        c.g[V1][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(r32(c, A2)));
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.sh((c.g[V1][0] as u32).wrapping_add(0x2), 0u64);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                                        'spaces: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] = c.lhu(c.g[A0][0] as u32);
                                            if c.g[V0][0] == c.g[A1][0] {
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x2));
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0039_2738);
                                                }
                                                continue 'spaces;
                                            }
                                            c.cycles += 2;
                                            c.g[V0][0] = c.g[A0][0];
                                            break 'b_3928e4;
                                        }
                                    }
                                }
                                // The last-space bookkeeping.
                                c.cycles += 2;
                                let fresh = (c.g[T9][0] as i64) <= 0;
                                c.g[V1][0] = sx((c.g[T9][0] as u32).wrapping_sub(0x1));
                                if !fresh {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lhu(c.g[T7][0] as u32);
                                    c.g[V0][0] = c.g[V0][0] ^ 0x20;
                                    if c.g[V0][0] != 0 {
                                        c.g[T8][0] = c.g[V1][0];
                                    }
                                }
                                c.cycles += 1;
                                c.g[V1][0] = c.lhu(r32(c, T0));
                            }
                            // Glyph lookup: direct below `last`, binary
                            // search above it, the default glyph on a miss
                            // (leaves the glyph in `t2`, 0 for none).
                            c.cycles += 4;
                            c.g[T2][0] = c.lw(font.wrapping_add(font_res::FIRST));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
                            c.g[T4][0] = sx((c.g[T5][0] as u32) << 1);
                            if c.g[V0][0] == 0u64 {
                                c.cycles += 4;
                                c.g[A0][0] = c.lw(font.wrapping_add(font_res::LAST));
                                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    // Above `last`: binary search.
                                    c.g[A3][0] = c.lw(font.wrapping_add(font_res::COUNT));
                                    c.cycles += 6;
                                    c.g[T2][0] =
                                        sx((c.g[A0][0] as u32).wrapping_sub(c.g[T2][0] as u32));
                                    c.g[S5][0] = c.lw(font.wrapping_add(font_res::TABLE));
                                    c.g[T4][0] = sx((c.g[T5][0] as u32) << 1);
                                    c.g[T1][0] = 0xc;
                                    c.g[V0][0] =
                                        sx((c.g[T2][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                    'search: loop {
                                        c.cycles += 6;
                                        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                        c.mult(0, 4, 5, 9);
                                        c.g[V0][0] =
                                            sx((c.g[A0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                                        c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                        if c.g[V1][0] == c.g[A0][0] {
                                            c.g[T2][0] = c.g[V0][0];
                                            break;
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[T2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        } else {
                                            c.cycles += 1;
                                            c.g[A3][0] = c.g[A1][0];
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[T2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                                        let searching = c.g[V0][0] != 0u64;
                                        c.g[V0][0] =
                                            sx((c.g[T2][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                        if searching {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0039_27d0);
                                            }
                                            continue 'search;
                                        }
                                        c.cycles += 1;
                                        c.g[T2][0] = c.lw(font.wrapping_add(font_res::DEF));
                                        break;
                                    }
                                } else {
                                    // Direct index into the table.
                                    c.cycles += 7;
                                    c.mult(0, 3, 3, 18);
                                    c.mult(0, 10, 10, 18);
                                    c.g[V0][0] = c.lw(font.wrapping_add(font_res::TABLE));
                                    c.g[T4][0] = sx((c.g[T5][0] as u32) << 1);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[T2][0] =
                                        sx((c.g[V0][0] as u32).wrapping_sub(c.g[T2][0] as u32));
                                }
                            } else {
                                // Below `first`: no glyph.
                                c.cycles += 2;
                                c.g[T2][0] = 0u64;
                            }
                            // Advance the pen over the glyph.
                            c.cycles += 2;
                            if c.g[T2][0] != 0u64 {
                                c.cycles += 13;
                                c.g[V0][0] = c.lb((c.g[T2][0] as u32).wrapping_add(glyph::XOFF));
                                c.g[V1][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(glyph::WIDTH));
                                c.g[A0][0] = c.lb((c.g[T2][0] as u32).wrapping_add(glyph::ADV));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.mtc1(4, 1);
                                c.cvt_s_w(1, 1);
                                c.mul_s(0, 0, 12);
                                c.mul_s(1, 1, 12);
                                c.add_s(2, 4, 0);
                                c.sub_s(4, 1, 0);
                                c.add_s(3, 3, 2);
                            }
                            // Emit the code when the line still has room.
                            c.cycles += 4;
                            c.c_le_s(13, 3);
                            if !c.fcc() {
                                c.sh(r32(c, T6), c.g[S0][0]);
                            } else {
                                c.cycles += 2;
                                let unseen = c.g[T8][0] == c.g[S4][0];
                                c.g[V1][0] = sx((c.g[T4][0] as u32).wrapping_add(r32(c, A2)));
                                if unseen {
                                    break 'b_3928cc;
                                }
                                c.cycles += 1;
                                c.sh(r32(c, T6), c.g[S0][0]);
                            }
                            // Next code.
                            c.cycles += 6;
                            c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x1));
                            c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x2));
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x2));
                            c.g[T9][0] = sx((c.g[T9][0] as u32).wrapping_add(0x1));
                            let full = c.g[T5][0] == c.g[S3][0];
                            c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x2));
                            if full {
                                break 'b_392884;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lhu(r32(c, T0));
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lhu(r32(c, T0));
                                if event_due(c) {
                                    deliver_event_here(c, 0x0039_2700);
                                }
                                continue 'scan;
                            }
                            break 'b_392884;
                        }
                    }
                    // The string ended: trim at the last space when the
                    // line is full.
                    c.cycles += 4;
                    c.c_le_s(13, 3);
                    let full = !c.fcc();
                    c.g[V0][0] = sx((c.g[T8][0] as u32) << 1);
                    if full {
                        break 'b_3928d4;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = 0x20;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A2)));
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sh((c.g[V1][0] as u32).wrapping_add(0x2), 0u64);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                    'tail_spaces: loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.lhu(c.g[A0][0] as u32);
                        if c.g[V0][0] == c.g[A1][0] {
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                            if event_due(c) {
                                deliver_event_here(c, 0x0039_28a8);
                            }
                            continue 'tail_spaces;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = c.g[A0][0];
                        break 'b_3928e4;
                    }
                }
                // No space seen: the line ends here.
                c.cycles += 2;
                c.g[V0][0] = c.g[T0][0];
                break 'b_3928e0;
            }
            // Nothing fit past the break: terminate the copy.
            c.cycles += 3;
            c.g[V1][0] = sx((c.g[T5][0] as u32) << 1);
            c.g[V0][0] = 0u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(r32(c, A2)));
        }
        // Store the terminator, return the pointer.
        c.cycles += 1;
        c.sh(c.g[V1][0] as u32, 0u64);
    }
    c.cycles += 8;
    frame.restore(c, S0, 0x50);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore(c, S5, 0);
    frame.close(c);
}

/// `void Font_DrawTextCenteredW(font, str, ...)` (0x392980): centres `str`
/// with `Font_MeasureTextW`, then draws it with `Font_DrawTextW`.
pub fn font_draw_text_centered_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0u64;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0u64;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(21, 12);
        c.mov_s(20, 13);
        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x0039_29bc);
        c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x3c));
        callee::FONT_MEASURE_TEXT_W.call(c, 0x0039_29bc)?;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 1);
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(13, 20);
        c.g[A1][0] = c.g[S1][0];
        c.mul_s(0, 0, 1);
        c.g[RA][0] = sx(0x0039_29dc);
        c.sub_s(12, 21, 0);
        callee::FONT_DRAW_TEXT_W.call(c, 0x0039_29dc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void Font_DrawTextRightW(font, str, ...)` (0x392ad0): right-aligns `str`
/// with `Font_MeasureTextW`, then draws it with `Font_DrawTextW`.
pub fn font_draw_text_right_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0u64;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0u64;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(21, 13);
        c.mov_s(20, 12);
        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x0039_2b0c);
        c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x3c));
        callee::FONT_MEASURE_TEXT_W.call(c, 0x0039_2b0c)?;
        c.sub_s(12, 20, 0);
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(13, 21);
        c.g[RA][0] = sx(0x0039_2b20);
        c.g[A1][0] = c.g[S1][0];
        callee::FONT_DRAW_TEXT_W.call(c, 0x0039_2b20)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void Font_Free(font)` (0x3919e8): frees the font's heap block (unless
/// its free flag is clear), zeroes the header, then runs the renderer's
/// per-font teardown hook unless the font's flags are negative.
pub fn font_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        if c.g[A0][0] == 0u64 {
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0039_1a0c);
            callee::EA_MEM_FREE_2.call(c, 0x0039_1a0c)?;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
        }
        c.cycles += 4;
        c.sw(c.g[S0][0] as u32, 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        let negative = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
        if !negative {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x190));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x194));
            let teardown = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039_1a34);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.pc = teardown;
            call_indirect(c, teardown);
            if c.pc != 0x0039_1a34 {
                return Err(crate::call::Unwound);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Font_DrawTextVarargs_2(...)` (0x21d160): forwards to
/// `Font_DrawTextF` through the varargs spill area.
pub fn font_draw_text_varargs_2(c: &mut Cpu) {
    guest(c, |c| {
        crate::game::render::font::draw_varargs(c, 0x0039_1c48, 0x0021_d194)
    })
}

/// `void Font_DrawTextCenteredVarargs_2(...)` (0x2223e8): forwards to
/// `Font_DrawTextCenteredF` through the varargs spill area.
pub fn font_draw_text_centered_varargs_2(c: &mut Cpu) {
    guest(c, |c| {
        crate::game::render::font::draw_varargs(c, 0x0039_29f8, 0x0022_241c)
    })
}

/// `void FXTrail_SetFlagA4(FXTrail* self, u32 flag)` (0x392de8): store the
/// flag word at `+0xa4` of the trail.
pub fn fx_trail_set_flag_a4(c: &mut Cpu) {
    c.cycles += 2;
    let flag = c.g[A1][0];
    Addr(abi::arg(c, 0).wrapping_add(0xa4)).sw(c, flag);
    abi::ret(c);
}

/// `VfxComponent` base destructor (0x396160): set the base vtable (0x488680)
/// at the object, then free the object when bit 0 of the flags (`a1`) is set
/// (`operator_delete`, 0x317e50).
pub fn vfx_component_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x7980));
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let obj = Addr(abi::arg(c, 0));
        let vtbl = c.g[V0][0];
        obj.sw(c, vtbl);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0039_6184)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Effective address `base + off` for a 64-bit register base (32-bit wrap).
#[inline]
pub(crate) fn render_ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `EmitLightingPacket(PS2GraphicsMan* gm)` (0x396b40, the type-6 lighting
/// packet): leaf. Writes header words at the render-state record (`gm+0xe84`),
/// copies ten 16-byte blocks from `gm+0x6bb0` to the list at `gm+0x5a00 |
/// 0x3000_0000`, writes the list tail, and stores the new list pointer back
/// at `gm+0x5a00`. Mechanical port of the generated body.
pub fn ps2_graphics_man_emit_lighting_packet(c: &mut Cpu) {
    c.cycles += 38;
    c.g[T5][0] = c.g[A0][0];
    c.g[A1][0] = !0x3c0; // 0xffff_ffff_ffff_fc3f
    c.g[V1][0] = Addr(render_ea(c.g[T5][0], 0xe84)).lw(c);
    c.g[A0][0] = 0x3fff_0000;
    c.g[A0][0] |= 0xffff;
    c.g[A2][0] = !0x3fff_ffff; // 0xffff_ffff_c000_0000
    let rs = c.g[V1][0] as u32;
    c.g[V0][0] = Addr(rs).lw(c);
    c.g[T0][0] = 0x3000_0000;
    c.g[T1][0] = 0x6000_0000;
    c.g[T1][0] |= 0xc;
    c.g[T2][0] = 0x8000;
    c.g[T2][0] <<= 45;
    c.g[V0][0] &= c.g[A1][0];
    // A3 = 0x6c0a_0007_0100_0101 built from halves.
    c.g[A3][0] = 0x6c0a_0000;
    c.g[A3][0] |= 0x7;
    c.g[A3][0] <<= 16;
    c.g[A3][0] |= 0x100;
    c.g[A3][0] <<= 16;
    c.g[A3][0] |= 0x101;
    c.g[V0][0] |= 0x180;
    c.g[T3][0] = sx(render_ea(c.g[T5][0], 0x6bb0));
    let v0 = c.g[V0][0];
    Addr(rs).sw(c, v0);
    c.g[T4][0] = 0x9;
    c.g[A1][0] = Addr(render_ea(c.g[T5][0], 0xe84)).lw(c);
    c.g[V1][0] = Addr(render_ea(c.g[T5][0], 0x5a00)).lw(c);
    c.g[V0][0] = Addr(render_ea(c.g[A1][0], 0xc)).lw(c);
    c.g[V1][0] &= c.g[A0][0];
    c.g[V0][0] &= c.g[A2][0];
    c.g[V0][0] |= c.g[V1][0];
    let v0 = c.g[V0][0];
    Addr(render_ea(c.g[A1][0], 0xc)).sw(c, v0);
    c.g[V1][0] = Addr(render_ea(c.g[T5][0], 0x5a00)).lw(c);
    c.g[A1][0] = c.g[V1][0] | c.g[T0][0];
    let a1 = render_ea(c.g[A1][0], 0);
    let t1 = c.g[T1][0];
    Addr(a1).sd(c, t1);
    let t2 = c.g[T2][0];
    Addr(a1.wrapping_add(0x10)).sd(c, t2);
    let a3 = c.g[A3][0];
    Addr(a1.wrapping_add(0x18)).sd(c, a3);
    Addr(a1.wrapping_add(0x8)).sd(c, 0);
    c.g[A1][0] = sx(a1.wrapping_add(0x20));
    c.g[V1][0] = c.g[A1][0];
    // Ten 16-byte blocks: T3 walks the source, V1 the destination.
    loop {
        c.cycles += 7;
        let src = render_ea(c.g[T3][0], 0);
        c.lq(2, src);
        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x1));
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x10));
        let dst = render_ea(c.g[V1][0], 0);
        c.sq(2, dst);
        let more = (c.g[T4][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0039_6bd8);
            }
            continue;
        }
        // Tail: the 16-byte tag at a1+0xa0 and the list end at a1+0xa8.
        c.cycles += 14;
        c.g[V0][0] = 0x0;
        c.g[A0][0] = 0x1400_0000;
        c.g[V0][0] = sx(render_ea(c.g[V0][0], 0x1150));
        Addr(render_ea(c.g[A1][0], 0xa0)).sd(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
        c.g[V1][0] = 0x0fff_0000;
        c.g[V0][0] |= c.g[A0][0];
        c.g[V1][0] |= 0xffff;
        c.g[V0][0] <<= 32;
        let end = c.g[V0][0];
        Addr(render_ea(c.g[A1][0], 0xa8)).sd(c, end);
        c.g[A1][0] = sx(render_ea(c.g[A1][0], 0xb0));
        c.g[V1][0] = c.g[A1][0] & c.g[V1][0];
        let t = c.g[RA][0] as u32;
        let list = c.g[V1][0];
        Addr(render_ea(c.g[T5][0], 0x5a00)).sw(c, list);
        c.pc = t;
        return;
    }
}

/// `GraphicsMan_SetDefaultDisplayMode` (0x0039_3fa8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_set_default_display_mode_00393fa8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_IsProgressive` (0x0039_3fb0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_is_progressive_00393fb0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `GraphicsMan_v16` (0x0039_4008): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v16_00394008(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_TryAcquireFrame` (0x0039_4010): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_try_acquire_frame_00394010(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `GraphicsMan_IsIdle` (0x0039_4018): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_is_idle_00394018(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `GraphicsMan_ReleaseTextureRef` (0x0039_42c0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_release_texture_ref_003942c0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_DrawTransition` (0x0039_42c8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_draw_transition_003942c8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v62` (0x0039_42d0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v62_003942d0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_GetColorMask` (0x0039_42d8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_get_color_mask_003942d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xed8));
    ret(c);
}

/// `GraphicsMan_v65` (0x0039_42e0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v65_003942e0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_ClearLights` (0x0039_45b8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_clear_lights_003945b8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_AddLight` (0x0039_45c0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_add_light_003945c0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v74` (0x0039_45c8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v74_003945c8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v88` (0x0039_45d0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v88_003945d0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_DrawShadowVolume` (0x0039_45d8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_draw_shadow_volume_003945d8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_DrawBoardWake` (0x0039_45e0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_draw_board_wake_003945e0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v98` (0x0039_45e8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v98_003945e8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_DrawSkinnedModelShadow` (0x0039_4610): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_draw_skinned_model_shadow_00394610(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v102` (0x0039_4618): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v102_00394618(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v103` (0x0039_4620): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v103_00394620(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v104` (0x0039_4628): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v104_00394628(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v105` (0x0039_4630): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v105_00394630(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v106` (0x0039_4638): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v106_00394638(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v107` (0x0039_4640): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v107_00394640(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v108` (0x0039_4648): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v108_00394648(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v109` (0x0039_4650): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v109_00394650(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v110` (0x0039_4658): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v110_00394658(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v111` (0x0039_4660): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v111_00394660(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v112` (0x0039_4668): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v112_00394668(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v113` (0x0039_4670): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v113_00394670(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_GetFlipCount` (0x0039_4678): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_get_flip_count_00394678(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `GraphicsMan_UpdateLensEffects` (0x0039_4680): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_update_lens_effects_00394680(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_AddView` (0x0039_4688): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_add_view_00394688(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_RemoveInterruptHandlers` (0x0039_46e8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_remove_interrupt_handlers_003946e8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GridMeshBase_SetScale` (0x0039_4b68): straight-line leaf. Mechanical port of the generated body.
pub fn grid_mesh_base_set_scale_00394b68(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BezierManBase_v04` (0x0039_4b78): straight-line leaf. Mechanical port of the generated body.
pub fn bezier_man_base_v04_00394b78(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BezierManBase_v05` (0x0039_4b80): straight-line leaf. Mechanical port of the generated body.
pub fn bezier_man_base_v05_00394b80(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BezierManBase_v19` (0x0039_4b88): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn bezier_man_base_v19_00394b88(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, c.g[A0][0] as u32);
    ret(c);
}

/// `BezierManBase_v20` (0x0039_4b90): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn bezier_man_base_v20_00394b90(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, c.g[A0][0] as u32);
    ret(c);
}

/// `PS2BezierMan_SetLodNearDist` (0x0039_4bc0): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_set_lod_near_dist_00394bc0(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0x33c));
    ret(c);
}

/// `PS2BezierMan_SetLodFarDist` (0x0039_4bc8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_set_lod_far_dist_00394bc8(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0x340));
    ret(c);
}

/// `PS2BezierMan_GetLodNearDist` (0x0039_4bd0): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_get_lod_near_dist_00394bd0(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x33c));
    ret(c);
}

/// `PS2BezierMan_GetLodFarDist` (0x0039_4bd8): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_get_lod_far_dist_00394bd8(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x340));
    ret(c);
}

/// `PS2BezierMan_v10` (0x0039_4be0): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v10_00394be0(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x354), c.g[A1][0]);
    ret(c);
}

/// `PS2BezierMan_v11` (0x0039_4be8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v11_00394be8(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x344), c.g[A1][0]);
    ret(c);
}

/// `PS2BezierMan_v12` (0x0039_4bf0): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v12_00394bf0(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x34c), c.g[A1][0]);
    ret(c);
}

/// `PS2BezierMan_v13` (0x0039_4bf8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v13_00394bf8(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x350), c.g[A1][0]);
    ret(c);
}

/// `PS2BezierMan_v14` (0x0039_4c00): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v14_00394c00(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x344));
    ret(c);
}

/// `PS2BezierMan_v15` (0x0039_4c08): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v15_00394c08(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34c));
    ret(c);
}

/// `PS2BezierMan_v16` (0x0039_4c10): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v16_00394c10(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x350));
    ret(c);
}

/// `VfxTube_Render` (0x0039_68c8): straight-line leaf. Mechanical port of the generated body.
pub fn vfx_tube_render_003968c8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PathArrow_Update` (0x0039_6918): straight-line leaf. Mechanical port of the generated body.
pub fn path_arrow_update_00396918(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PathArrow_Render` (0x0039_6920): straight-line leaf. Mechanical port of the generated body.
pub fn path_arrow_render_00396920(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DetailSystem_v05` (0x0039_6958): straight-line leaf. Mechanical port of the generated body.
pub fn detail_system_v05_00396958(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    pub(crate) use crate::game::sound::bankmanager::tests::oc762_install_fakes;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc760_install_fakes;
    pub(crate) use crate::game::visualfx::lensfx::tests::oc780_install_fakes;

    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fill;
    use crate::difftest::put_vec;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    use crate::difftest::SP;

    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2;
    pub(crate) use crate::game::be::beintoption::tests::slot;
    pub(crate) use crate::game::be::beintoption::tests::VT;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::pool;

    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;

    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::object::railmodifier::tests::destructor_setup;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::render::particle::tests::base_2 as base;
    pub(crate) use crate::game::render::particle::tests::SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_4;
    use ssx3_ee as ee;

    use ssx3_ee::s13;

    // ------------------------------------------------------------ DeferMan

    /// The chain of a random pass for `DeferMan_FindBucket`: `n` nodes from
    /// DATA+0x1000 (0x40 apart); the searched keys in a1/a2/t0. Node `hit`
    /// matches; the others differ in one key, are skipped (+0x14 set) or
    /// are random.
    pub(crate) fn bucket_chain(c: &mut Cpu, r: &mut Rng, n: u32, hit: Option<u32>) {
        let man = DATA;
        let pass = r.next_u32();
        let (hash, key1, key2) = (r.next_u32(), r.next_u32(), r.next_u32());
        c.g[A0][0] = sx(man);
        c.g[A1][0] = sx(hash);
        c.g[A2][0] = sx(key1);
        c.g[A3][0] = (r.next_u32() as u64) << 32 | pass as u64;
        c.g[T0][0] = sx(key2);
        let node = |i: u32| DATA + 0x1000 + i * 0x40;
        let slot_at = man + defer::BUCKETS + (pass & 0xff) * 4;
        c.bus.write32(slot_at, if n == 0 { 0 } else { node(0) });
        for i in 0..n {
            let at = node(i);
            fill(c, r, at, 0x10);
            let (mut h, mut k1, mut k2, mut skip) = (hash, key1, key2, 0);
            if hit != Some(i) {
                match r.next_u32() % 5 {
                    0 => h ^= 1 << (r.next_u32() % 32),
                    1 => k1 = r.next_u32(),
                    2 => k2 = r.next_u32(),
                    3 => skip = r.next_u32() | 1,
                    _ => (h, k1, k2) = (r.next_u32(), r.next_u32(), r.next_u32()),
                }
            }
            c.bus.write32(at, h);
            c.bus.write32(at + 4, k1);
            c.bus.write32(at + 8, k2);
            c.bus.write32(at + defer::NODE_SKIP, skip);
            c.bus.write32(
                at + defer::NODE_NEXT,
                if i + 1 < n { node(i + 1) } else { 0 },
            );
        }
    }

    #[test]
    fn defer_man_find_bucket_matches() {
        check(
            ssx3_ee::s13::f_00395000__generated,
            defer_man_find_bucket,
            300,
            &|c, seed| {
                let mut r = Rng(0x3950_0000 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                let n = [0, 1, 2, 3, 5, 8][(seed % 6) as usize];
                let hit = if n == 0 || seed % 7 == 0 {
                    None
                } else {
                    Some(r.next_u32() % n)
                };
                bucket_chain(c, &mut r, n, hit);
            },
        );
        check_events(
            ssx3_ee::s13::f_00395000__generated,
            defer_man_find_bucket,
            90,
            &|c, seed| {
                let mut r = Rng(0x3950_e000 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                let events = schedule(c, &mut r, seed, 0x0039_5000);
                let n = 4 + r.next_u32() % 6;
                let hit = if seed % 4 == 0 {
                    None
                } else {
                    Some(n - 1 - r.next_u32() % 2)
                };
                bucket_chain(c, &mut r, n, hit);
                events
            },
        );
    }

    /// The state chain of a random pass for `DeferMan_InternState`: `n`
    /// records from DATA+0x1000 (0x20 apart), the state at DATA+0x800 or
    /// +0x804 (unaligned doublewords), the pool count random. Record `hit`
    /// equals the state; others share a prefix of 0..3 words or are random.
    pub(crate) fn state_chain(c: &mut Cpu, r: &mut Rng, n: u32, hit: Option<u32>) {
        let man = DATA;
        let pass = r.next_u32();
        let state = DATA + 0x800 + 4 * (r.next_u32() % 2);
        fill(c, r, state, 5);
        c.g[A0][0] = sx(man);
        c.g[A1][0] = sx(state);
        c.g[A2][0] = (r.next_u32() as u64) << 32 | pass as u64;
        let count = r.next_u32() % 64;
        c.bus
            .write32(man + defer::STATE_POOL + defer::STATE_POOL_COUNT, count);
        let rec = |i: u32| DATA + 0x1000 + i * 0x20;
        let slot_at = man + defer::STATE_HEADS + (pass & 0xff) * 4;
        c.bus.write32(slot_at, if n == 0 { 0 } else { rec(0) });
        for i in 0..n {
            let at = rec(i);
            fill(c, r, at, 8);
            if hit == Some(i) || r.next_u32() % 2 == 0 {
                let words = if hit == Some(i) { 4 } else { r.next_u32() % 4 };
                for w in 0..words {
                    let v = c.bus.read32(state + 4 * w);
                    c.bus.write32(at + 4 * w, v);
                }
            }
            c.bus.write32(
                at + defer::STATE_NEXT,
                if i + 1 < n { rec(i + 1) } else { 0 },
            );
        }
    }

    #[test]
    fn defer_man_intern_state_matches() {
        check_lohi(
            ssx3_ee::s13::f_00394ed0__generated,
            defer_man_intern_state,
            300,
            &|c, seed| {
                let mut r = Rng(0x394e_d000 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                let n = [0, 1, 2, 3, 5, 8][(seed % 6) as usize];
                let hit = if n == 0 || seed % 7 == 0 {
                    None
                } else {
                    Some(r.next_u32() % n)
                };
                state_chain(c, &mut r, n, hit);
            },
        );
        check_events(
            ssx3_ee::s13::f_00394ed0__generated,
            defer_man_intern_state,
            90,
            &|c, seed| {
                let mut r = Rng(0x394e_e000 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                let events = schedule(c, &mut r, seed, 0x0039_4ed0);
                let n = 3 + r.next_u32() % 5;
                let hit = if seed % 4 == 0 {
                    None
                } else {
                    Some(n - 1 - r.next_u32() % 2)
                };
                state_chain(c, &mut r, n, hit);
                events
            },
        );
    }

    #[test]
    fn ps2_graphics_man_copy_view_state_matches() {
        check(
            ee::s13::f_003950c0__generated,
            ps2_graphics_man_copy_view_state,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x39_50c0);
                let dst = DATA;
                let src = DATA + 0x4000;
                fill(c, &mut r, src, 0x100);
                fill(c, &mut r, dst, 0x100);
                c.g[A0][0] = sx(dst);
                c.g[A1][0] = sx(src);
            },
        );
    }

    // --------------------------------------------------------- font FX trails

    #[test]
    fn fx_trail_reset_matches() {
        check(
            ee::s13::f_00392d18__generated,
            fx_trail_reset,
            SEEDS_2,
            &|c, seed| {
                let mut r = Rng(0x0039_2d18 ^ seed as u64);
                scramble(c, &mut r);
                let trail = DATA + 0x1000;
                c.g[A0][0] = trail as u64;
                // Old contents the reset must clear: the {x,y,z} cursor, the two
                // endpoint times, and the scratch doubleword on the stack.
                fill(c, &mut r, trail, 3);
                fill(c, &mut r, trail + 0xa0, 2);
                fill(c, &mut r, SP - 0x10, 4);
            },
        );
    }

    #[test]
    fn grid_mesh_base_call_v22_matches() {
        covered(
            s13::f_00394b98__generated,
            grid_mesh_base_call_v22,
            120,
            &|c, seed| {
                machine(c, seed, 0x394b98, 1);
                clear(c, OBJ, 0x40);
                clear(c, VT, 0x200);
                slot(c, VT, 0xb0);
                c.bus.write32(OBJ + 4, VT);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(seed as u32);
                c.g[A2][0] = sx(seed.wrapping_mul(3));
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn light_man_free_instances_matches() {
        check(
            s13::f_00394ce8__generated,
            light_man_free_instances,
            120,
            &|c, seed| {
                machine(c, seed, 0x394ce8, 1);
                clear(c, OBJ, 0x40);
                c.g[A0][0] = sx(OBJ);
                let list = if seed % 3 == 0 { 0 } else { CHILD };
                c.bus.write32(OBJ + 4, list);
            },
        );
    }

    #[test]
    fn grid_mesh_base_base_dtor_matches() {
        check(
            ee::s13::f_00394b38__generated,
            grid_mesh_base_base_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x394b_38),
        );
    }

    #[test]
    fn vfx_render_toggles_dtor_matches() {
        check(
            ee::s13::f_003961a8__generated,
            vfx_render_toggles_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3961_a8),
        );
    }

    #[test]
    fn vfx_plants_dtor_matches() {
        check(
            ee::s13::f_003964c8__generated,
            vfx_plants_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3964_c8),
        );
    }

    #[test]
    fn vfx_tube_dtor_matches() {
        check(
            ee::s13::f_00396838__generated,
            vfx_tube_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3968_38),
        );
    }

    #[test]
    fn path_arrow_v01_matches() {
        check(
            ee::s13::f_003968e8__generated,
            path_arrow_v01,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3968_e8),
        );
    }

    #[test]
    fn detail_system_v01_matches() {
        check(
            ee::s13::f_00396928__generated,
            detail_system_v01,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3969_28),
        );
    }

    #[test]
    fn vfx_visual_effects_test_dtor_matches() {
        check(
            ee::s13::f_00396a00__generated,
            vfx_visual_effects_test_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x396a_00),
        );
    }

    #[test]
    fn vfx_screen_tint_dtor_matches() {
        check(
            ee::s13::f_00396d30__generated,
            vfx_screen_tint_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x396d_30),
        );
    }

    // ------------------------------------------------------------ FX trail

    #[test]
    fn fxtrail_add_point_matches() {
        check(
            ee::s13::f_00392d90__generated,
            fxtrail_add_point,
            80,
            &|c, seed| {
                let mut r = Rng(0x392d_7a11 ^ seed as u64);
                scramble(c, &mut r);
                let t = DATA;
                c.bus.write32(t + 0xc, r.next_u32() % 4);
                fill(c, &mut r, t + 0x10, 32);
                fill(c, &mut r, t + 0x50, 64);
                let rot = DATA + 0x200;
                let pos = DATA + 0x300;
                fill(c, &mut r, rot, 4);
                fill(c, &mut r, pos, 4);
                c.g[A0][0] = sx(t);
                c.g[A1][0] = sx(rot);
                c.g[A2][0] = sx(pos);
            },
        );
    }

    #[test]
    fn light_man_init_instances_matches() {
        check(
            ee::s13::f_00394c98__generated,
            light_man_init_instances,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x394c_98, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_2(c, this, 0x20);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 16) as u64;
            },
        );
    }

    /// Write `words` at `addr`.
    pub(crate) fn words(c: &mut Cpu, addr: u32, v: &[u32]) {
        for (i, w) in v.iter().enumerate() {
            c.bus.write32(addr + 4 * i as u32, *w);
        }
    }

    pub(crate) fn rand_vec(c: &mut Cpu, r: &mut Rng, addr: u32) {
        put_vec(
            c,
            addr,
            [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
        );
    }

    #[test]
    fn graphics_man_set_screen_offset_matches() {
        check(
            ssx3_ee::s13::f_00393fb8__generated,
            graphics_man_set_screen_offset,
            160,
            &|c, seed| {
                let mut r = Rng(0x393f_b800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
                words(c, DATA, &[r.next_u32(), r.next_u32()]);
            },
        );
    }

    #[test]
    fn graphics_man_get_screen_offset_x_matches() {
        check(
            ssx3_ee::s13::f_00393fd8__generated,
            graphics_man_get_screen_offset_x,
            200,
            &|c, seed| {
                let mut r = Rng(0x393f_d800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.edge_f32());
            },
        );
    }

    #[test]
    fn graphics_man_get_screen_offset_y_matches() {
        check(
            ssx3_ee::s13::f_00393ff0__generated,
            graphics_man_get_screen_offset_y,
            200,
            &|c, seed| {
                let mut r = Rng(0x393f_f000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 4, r.edge_f32());
            },
        );
    }

    #[test]
    fn graphics_man_v58_matches() {
        check(
            ssx3_ee::s13::f_003942a0__generated,
            graphics_man_v58,
            160,
            &|c, seed| {
                let mut r = Rng(0x3942_a000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                words(
                    c,
                    DATA,
                    &[r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
            },
        );
    }

    #[test]
    fn light_man_next_id_matches() {
        check(
            ssx3_ee::s13::f_00394d18__generated,
            light_man_next_id,
            200,
            &|c, seed| {
                let mut r = Rng(0x394d_1800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 8, r.next_u32());
                c.bus.write32(DATA + 0xc, r.next_u32() % 100);
            },
        );
    }

    pub(crate) fn screen_entry(c: &mut Cpu, r: &mut Rng, off: u32) {
        c.g[A0][0] = DATA as u64;
        let idx = r.next_u32() % 3;
        let entry = DATA + 0x400;
        c.bus.write32(DATA + 0x59c8, idx);
        c.bus.write32(DATA + 0x59cc, entry);
        c.bus.write32(entry + idx * 0xa4 + off, r.next_u32());
        c.bus.write32(entry + idx * 0xa4 + 0x88, r.next_u32());
        c.bus.write32(entry + idx * 0xa4 + 0x8c, r.next_u32());
    }

    #[test]
    fn ps2_graphics_man_get_screen_width_matches() {
        check(
            ssx3_ee::s13::f_00395330__generated,
            ps2_graphics_man_get_screen_width,
            160,
            &|c, seed| {
                let mut r = Rng(0x3953_3000 ^ seed as u64);
                scramble(c, &mut r);
                screen_entry(c, &mut r, 0x88);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_get_screen_height_matches() {
        check(
            ssx3_ee::s13::f_00395350__generated,
            ps2_graphics_man_get_screen_height,
            160,
            &|c, seed| {
                let mut r = Rng(0x3953_5000 ^ seed as u64);
                scramble(c, &mut r);
                screen_entry(c, &mut r, 0x8c);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_lighting_matches() {
        check_events(
            ssx3_ee::s13::f_003954d0__generated,
            ps2_graphics_man_set_lighting,
            120,
            &|c, seed| {
                let mut r = Rng(0x3954_d000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let src = DATA + 0x800;
                c.g[A1][0] = src as u64;
                for i in 0..10 {
                    rand_vec(c, &mut r, src + i * 0x10);
                }
                let mode = schedule(c, &mut r, seed, 0x0039_54d0);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn ps2_graphics_man_push_matrix_matches() {
        check(
            ssx3_ee::s13::f_003956e8__generated,
            ps2_graphics_man_push_matrix,
            160,
            &|c, seed| {
                let mut r = Rng(0x3956_e800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let top = DATA + 0x1000;
                c.bus.write32(DATA + 0x13e4, top);
                c.bus.write32(DATA + 0x13e0, r.next_u32() % 16);
                for i in 0..4 {
                    rand_vec(c, &mut r, top + i * 0x10);
                    rand_vec(c, &mut r, top + 0x40 + i * 0x10);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_load_matrix_matches() {
        check(
            ssx3_ee::s13::f_00395c38__generated,
            ps2_graphics_man_load_matrix,
            160,
            &|c, seed| {
                let mut r = Rng(0x395c_3800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let top = DATA + 0x1000;
                let m = DATA + 0x2000;
                c.bus.write32(DATA + 0x13e4, top);
                c.bus.write32(DATA + 0x6b90, r.next_u32());
                for i in 0..4 {
                    rand_vec(c, &mut r, top + i * 0x10);
                    rand_vec(c, &mut r, m + i * 0x10);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_mult_matrix_matches() {
        check(
            ssx3_ee::s13::f_00395c70__generated,
            ps2_graphics_man_mult_matrix,
            160,
            &|c, seed| {
                let mut r = Rng(0x395c_7000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let top = DATA + 0x1000;
                let m = DATA + 0x2000;
                c.bus.write32(DATA + 0x13e4, top);
                c.bus.write32(DATA + 0x6b90, r.next_u32());
                for i in 0..4 {
                    rand_vec(c, &mut r, top + i * 0x10);
                    rand_vec(c, &mut r, m + i * 0x10);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_load_identity_matches() {
        check(
            ssx3_ee::s13::f_00395cf0__generated,
            ps2_graphics_man_load_identity,
            160,
            &|c, seed| {
                let mut r = Rng(0x395c_f000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let top = DATA + 0x1000;
                c.bus.write32(DATA + 0x13e4, top);
                c.bus.write32(DATA + 0x6b90, r.next_u32());
                // The .lit4 identity the original copies.
                put_vec(c, 0x004f_f1a0, [0x3f80_0000, 0, 0, 0]);
                put_vec(c, 0x004f_f1b0, [0, 0x3f80_0000, 0, 0]);
                put_vec(c, 0x004f_f1c0, [0, 0, 0x3f80_0000, 0]);
                put_vec(c, 0x004f_f1d0, [0, 0, 0, 0x3f80_0000]);
                for i in 0..4 {
                    rand_vec(c, &mut r, top + i * 0x10);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_translate_matches() {
        check(
            ssx3_ee::s13::f_00395d28__generated,
            ps2_graphics_man_translate,
            160,
            &|c, seed| {
                let mut r = Rng(0x395d_2800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let top = DATA + 0x1000;
                let v = DATA + 0x2000;
                c.bus.write32(DATA + 0x13e4, top);
                c.bus.write32(DATA + 0x6b90, r.next_u32());
                for i in 0..4 {
                    rand_vec(c, &mut r, top + i * 0x10);
                }
                rand_vec(c, &mut r, v);
            },
        );
    }

    #[test]
    fn c_ps2_grid_mesh_set_scale_matches() {
        check(
            ssx3_ee::s13::f_00396110__generated,
            c_ps2_grid_mesh_set_scale,
            200,
            &|c, seed| {
                let mut r = Rng(0x3961_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP - 0x254c, 32767.0f32.to_bits());
                c.f[12] = if seed % 4 == 0 { 0 } else { r.edge_f32() };
                c.bus.write32(DATA + 0x194, r.next_u32());
                c.bus.write32(DATA + 0x198, r.next_u32());
            },
        );
    }

    #[test]
    fn rail_tube_set_point_matches() {
        check(
            ssx3_ee::s13::f_003968d0__generated,
            rail_tube_set_point,
            160,
            &|c, seed| {
                let mut r = Rng(0x3968_d000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.bus.write32(DATA + 8, DATA + 0x1000);
                rand_vec(c, &mut r, DATA + 0x2000);
                for i in 0..4 {
                    rand_vec(c, &mut r, DATA + 0x1000 + i * 0x10);
                }
            },
        );
    }

    #[test]
    fn font_draw_text_f_matches() {
        check(
            ee::s12::f_00391c48__generated,
            font_draw_text_f,
            4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0039_1c48, 2);
                be_library_2(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
            },
        );
    }

    #[test]
    fn font_draw_text_centered_f_matches() {
        check(
            ee::s13::f_003929f8__generated,
            font_draw_text_centered_f,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x3929f8, 0);
                globals(c);
                // The empty-string sentinel: the local BXString's destructor then
                // skips the refcount path.
                w(c, GAME_GP + 0xda0, 0x0048_dcd4);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = 0;
                w(c, OBJ2, 0);
                c.f[12] = if seed % 2 == 0 {
                    0x3f80_0000
                } else {
                    0xc000_0000
                };
                c.f[13] = 0x4040_0000;
            },
        );
    }

    #[test]
    fn font_draw_text_right_f_matches() {
        check(
            ee::s13::f_00392b40__generated,
            font_draw_text_right_f,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x392b40, 0);
                globals(c);
                w(c, GAME_GP + 0xda0, 0x0048_dcd4);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = 0;
                w(c, OBJ2, 0);
                c.f[12] = if seed % 2 == 0 {
                    0x3f80_0000
                } else {
                    0xc000_0000
                };
                c.f[13] = 0x4040_0000;
            },
        );
    }

    // --------------------------------------------------------- rendering

    #[test]
    fn font_draw_text_right_matches() {
        check(
            ee::s13::f_00392a60__generated,
            font_draw_text_right,
            4,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x0039_2a60, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (DATA + 0x4000) as u64;
                c.bus.write8(DATA + 0x4000, b't');
                c.bus.write8(DATA + 0x4001, 0);
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                w_2(c, OBJ + 0x38, r.edge_f32());
                w_2(c, OBJ + 0x3c, r.edge_f32());
            },
        );
    }

    #[test]
    fn renderer_build_sine_table_matches() {
        check(
            ee::s13::f_00392df0__generated,
            renderer_build_sine_table,
            4,
            &|c, seed| {
                let mut r = Rng(0x392d_f0e1 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                w_2(c, GAME_GP.wrapping_sub(0x2694), 0x3c49_0fdb);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    // -------------------------------------------------------------------- fonts

    /// A glyph table whose count/max fields let both the "outside the range"
    /// and the search paths run: `+0xc` count, `+0x10` max code, `+0` first
    /// glyph, `+4` glyph table pointer, `+8` width table pointer.
    pub(crate) fn font_fixture(c: &mut Cpu, seed: u32, salt: u64, wide: bool) {
        let _ = base_3(c, seed, salt, 0);
        c.g[A0][0] = OBJ as u64;
        let ch = if wide {
            (seed % 0x60) as u64
        } else {
            ((seed % 0x60) as i8 as i32) as u32 as u64
        };
        c.g[A1][0] = ch;
        w_3(c, OBJ + 0xc, if seed % 5 == 0 { 0 } else { 100 });
        w_3(c, OBJ + 0x10, if seed % 3 == 0 { 0x10 } else { 0x200 });
        w_3(c, OBJ, OBJ2_2);
        w_3(c, OBJ + 4, OBJ2_2);
        w_3(c, OBJ + 8, OBJ3);
        // A sorted little glyph table at OBJ2_2 (codes) and widths at OBJ3.
        for i in 0..0x20u32 {
            c.bus.write16(OBJ2_2 + i * 2, (i * 2) as u16);
            c.bus.write8(OBJ3 + i, (i as u8).wrapping_add(1));
        }
    }

    #[test]
    fn font_get_char_width_matches() {
        check(
            ee::s13::f_00392ba8__generated,
            font_get_char_width,
            12,
            &|c, seed| font_fixture(c, seed, 0x0039_2ba8, false),
        );
    }

    #[test]
    fn font_get_char_width_w_matches() {
        check(
            ee::s13::f_00392c60__generated,
            font_get_char_width_w,
            12,
            &|c, seed| font_fixture(c, seed, 0x0039_2c60, true),
        );
    }

    #[test]
    fn ps2_graphics_man_rotate_axis_matches() {
        same(
            ssx3_ee::s13::f_00395d60__generated,
            ps2_graphics_man_rotate_axis,
            200,
            &|c, seed| {
                let mut r = Rng(0x395d60 ^ seed as u64);
                scramble(c, &mut r);
                let m = DATA + 0x100;
                let man = DATA + 0x2000;
                c.g[A0][0] = man as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.bus.write32(man + 0x13e4, m);
                put_vec(
                    c,
                    m,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    m + 0x10,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    m + 0x20,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    m + 0x30,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                for i in 0..4 {
                    c.bus.write32(DATA + 0x300 + 4 * i, r.edge_f32());
                }
                c.f[12] = r.edge_f32();
            },
        );
    }

    /// TexMan_InvalidateBound: the texture manager at a random address with
    /// random bound slots. Events are delivered in the two-slot loop.
    #[test]
    fn tex_man_invalidate_bound_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        use crate::difftest::fakes::machine;
        use crate::difftest::{fill, DATA};
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x0039_6128, 0);
            let this = DATA + (r.next_u32() & 0x7_fff0);
            fill(c, &mut r, this + 0x1f40, 8);
            c.g[A0][0] = sx(this);
            let mode = schedule(c, &mut r, seed, 0x0039_6128);
            // The loop's only back edge comes 11 cycles in: make the event
            // seeds' event due by then.
            if mode != Events::Off {
                c.next_event = c.cycles + (r.next_u32() % 12) as u64;
            }
            mode
        };
        check_events(
            ee::s13::f_00396128__generated,
            tex_man_invalidate_bound,
            200,
            &setup,
        );
    }

    /// PS2GraphicsMan_TryAcquireFrame: the current frame buffer's state
    /// free (0), claimed (1) or busy (2); the real AcquireFrameBuffer runs.
    #[test]
    fn ps2_graphics_man_try_acquire_frame_matches() {
        check(
            ee::s13::f_00395f60__generated,
            ps2_graphics_man_try_acquire_frame,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x39_5f60, 0);
                c.pc = 0x0039_5f60;
                let this = DATA + 0x1_0000;
                let cur = seed % 2;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x5a10, cur);
                c.bus.write32(this + 0x5a90 + 4 * cur, (seed / 2) % 3);
            },
        );
    }

    pub(crate) use crate::game::object::instancenode::tests::oc383_floats;
    pub(crate) use crate::game::object::movenode::tests::oc383_vtable;
    pub(crate) use crate::game::seg::seg_2fe810::tests::oc383_machine;

    /// A font at `font`: 16 directly indexed glyphs 'A'..='P', then 8
    /// searched ones (keys 0x60, 0x62, .. 0x6e), random metrics, shadow.
    pub(crate) fn oc383_font(c: &mut Cpu, r: &mut Rng, font: u32, glyphs: u32) {
        c.bus.write32(font + FONT_GLYPH_COUNT, 24);
        c.bus.write32(font + FONT_GLYPHS, glyphs);
        c.bus.write32(font + FONT_FIRST_CHAR, 0x41);
        c.bus.write32(font + FONT_LAST_DIRECT, 0x50);
        for i in 0..24u32 {
            let e = glyphs + 12 * i;
            fill(c, r, e, 3);
            let key = if i < 16 {
                0x41 + i
            } else {
                0x60 + 2 * (i - 16)
            };
            c.bus.write16(e, key as u16);
        }
        let default = if r.next_u32() & 1 == 0 {
            0
        } else {
            glyphs + 12 * 5
        };
        c.bus.write32(font + FONT_DEFAULT_GLYPH, default);
        c.bus.write32(font + FONT_SHADOW, r.f32(3.0));
        c.bus.write32(font + FONT_SHADOW + 4, r.f32(3.0));
    }

    /// A wide string of up to 8 characters around the font's ranges.
    pub(crate) fn oc383_put_wstr(c: &mut Cpu, r: &mut Rng, at: u32) -> u32 {
        let len = r.next_u32() % 9;
        for i in 0..len {
            let k = r.next_u32();
            let ch = match k % 5 {
                0 => 0x30 + (k >> 8) % 0x11,
                1 | 2 => 0x41 + (k >> 8) % 16,
                3 => 0x5f + (k >> 8) % 0x12,
                _ => 0x100 + (k >> 8) % 0x100,
            };
            c.bus.write16(at + 2 * i, ch as u16);
        }
        c.bus.write16(at + 2 * len, 0);
        len
    }

    #[test]
    fn font_measure_text_w_matches() {
        check(
            ee::s12::f_003921f0__generated,
            font_measure_text_w,
            300,
            &|c, seed| {
                let mut r = oc383_machine(c, seed, 0x21f0, 0x0039_21f0);
                oc383_font(c, &mut r, DATA, DATA + 0x100);
                oc383_put_wstr(c, &mut r, DATA + 0x400);
                fill(c, &mut r, DATA + 0x800, 4);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x800) };
                c.g[A3][0] = match seed % 4 {
                    0 | 1 => 0,
                    2 => 1,
                    _ => r.next_u32() as u64,
                };
                c.f[12] = r.f32(2.0);
                c.f[13] = r.f32(2.0);
            },
        );
    }

    #[test]
    fn font_draw_text_w_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s12::f_00391e30__generated,
            font_draw_text_w,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e30, 2);
                c.pc = 0x0039_1e30;
                let font = DATA;
                fill(c, &mut r, font, 0x20);
                oc383_floats(c, &mut r, font + FONT_OFFSET, 4, 5.0);
                oc383_floats(c, &mut r, font + FONT_SCALE, 2, 3.0);
                let (sx_, sy_) = match seed % 4 {
                    0 => (0, 0),
                    1 => (r.f32(3.0), 0),
                    2 => (0, r.f32(3.0)),
                    _ => (0x8000_0000, r.f32(3.0)),
                };
                c.bus.write32(font + FONT_SHADOW, sx_);
                c.bus.write32(font + FONT_SHADOW + 4, sy_);
                let grman = DATA + 0x5000;
                c.bus.write32(GAME_GP + AVALANCHE_GRMAN_GP, grman);
                c.bus.write32(grman + 0xe84, DATA + 0x6000);
                fill(c, &mut r, DATA + 0x6000, 8);
                if seed % 5 == 0 {
                    // Layer 0: the shadow stays at layer 0.
                    let w = c.bus.read32(DATA + 0x6008) & !0x3e0;
                    c.bus.write32(DATA + 0x6008, w);
                }
                c.bus.write32(grman + 0x10d8, DATA + 0x5800);
                oc383_vtable(c, &mut r, DATA + 0x5800, &[0x288]);
                c.g[A0][0] = sx(font);
                c.g[A1][0] = sx(DATA + 0x400);
                c.f[12] = r.f32(300.0);
                c.f[13] = r.f32(300.0);
            },
        );
    }

    /// PS2GraphicsMan_IsIdle: the real IsRenderIdle on a busy or idle
    /// graphics manager.
    #[test]
    fn ps2_graphics_man_is_idle_matches() {
        check(
            ee::s13::f_00395f80__generated,
            ps2_graphics_man_is_idle,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x395f80, 0);
                c.pc = 0x0039_5f80;
                c.g[A0][0] = sx(DATA);
                let busy = if seed % 2 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(DATA + 0x5a8c, busy);
            },
        );
    }

    /// A font at DATA: direct glyphs for 0x20..=0x60, sorted extra glyphs
    /// (odd codes from 0x61) after them, a default glyph (or none), and a
    /// text of words, space runs and characters below/above the range.
    pub(crate) fn font_find_line_break_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0039_2430 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
        scramble(c, &mut r);
        c.pc = 0x0039_2430;
        let font = DATA;
        let table = DATA + 0x400;
        let text = DATA + 0x2000;
        let out = DATA + 0x3000;
        let (lo, hi) = (0x20u32, 0x60u32);
        let direct = hi - lo + 1;
        let count = direct + 12;
        c.bus.write32(font + FONT_FIND_LINE_BREAK_COUNT, count);
        c.bus.write32(font + FONT_FIND_LINE_BREAK_TABLE, table);
        let default = if seed % 4 == 0 { 0 } else { table + 0x900 };
        c.bus.write32(font + FONT_FIND_LINE_BREAK_DEFAULT, default);
        c.bus.write32(font + FONT_FIND_LINE_BREAK_LO, lo);
        c.bus.write32(font + FONT_FIND_LINE_BREAK_HI, hi);
        c.bus
            .write32(font + FONT_FIND_LINE_BREAK_SHADOW_X, r.f32(3.0));
        for i in 0..=count {
            let g = table + i * 12;
            let code = if i < direct {
                lo + i
            } else {
                0x61 + 2 * (i - direct)
            };
            c.bus.write16(g, code as u16);
            c.bus.write8(g + 2, (r.next_u32() % 12) as u8);
            c.bus.write8(g + 8, (r.next_u32() % 16) as u8);
            c.bus.write8(g + 9, ((r.next_u32() % 7) as i8 - 2) as u8);
        }
        c.bus.write8(table + 0x900 + 2, 5);
        c.bus.write8(table + 0x900 + 8, 6);
        c.bus.write8(table + 0x900 + 9, 0xff);
        let len = if seed % 13 == 0 { 0 } else { r.next_u32() % 48 };
        const CHARS: &[u8] = b"  abcdeAB  ,.xyz\x10\x7a\x63\x64\x90\xff qrs   ";
        for i in 0..len {
            let ch = CHARS[(r.next_u32() as usize) % CHARS.len()];
            c.bus.write8(text + i, ch);
        }
        c.bus.write8(text + len, 0);
        for i in 0..64 {
            c.bus.write8(out + i, 0xcc);
        }
        c.g[A0][0] = sx(font);
        c.g[A1][0] = sx(text);
        c.g[A2][0] = sx(out);
        c.g[A3][0] = sx(match seed % 5 {
            0 => 0xffff_ffff,
            1 => 6,
            _ => 1 + r.next_u32() % 60,
        });
        c.f[12] = 0.5f32.to_bits().wrapping_add(r.next_u32() % 0x0080_0000);
        c.f[13] = match seed % 6 {
            0 => 1000.0f32.to_bits(),
            1 => 1.0f32.to_bits(),
            _ => ((r.next_u32() % 200) as f32).to_bits(),
        };
    }

    /// `Font_FindLineBreak` (0x392430): every exit (fits, word break at a
    /// space, at the end, inside an over-long word, max reached).
    #[test]
    fn font_find_line_break_matches() {
        check_lohi(
            ee::s12::f_00392430__generated,
            font_find_line_break,
            300,
            &font_find_line_break_fixture,
        );
    }

    pub(crate) use crate::game::fe::debugfe::tests::oc505_install_fakes;
    pub(crate) use crate::game::fe::debugfe::tests::oc505_machine;

    #[test]
    fn font_draw_text_varargs_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dc990__generated,
            font_draw_text_varargs,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dc9_90, 1);
                c.pc = 0x001d_c990;
            },
        );
    }

    #[test]
    fn font_draw_text_centered_varargs_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dca40__generated,
            font_draw_text_centered_varargs,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dca_40, 1);
                c.pc = 0x001d_ca40;
            },
        );
    }

    #[test]
    fn font_draw_text_right_varargs_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dca80__generated,
            font_draw_text_right_varargs,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dca_80, 1);
                c.pc = 0x001d_ca80;
            },
        );
    }

    #[test]
    fn font_draw_text_scaled_varargs_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dc9d0__generated,
            font_draw_text_scaled_varargs,
            60,
            &|c, seed| {
                let mut r = oc505_machine(c, seed, 0x1dc9_d0, 1);
                c.pc = 0x001d_c9d0;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.bus.write32(DATA + 0x130, r.f32(100.0));
                c.bus.write32(DATA + 0x134, r.edge_f32());
            },
        );
    }

    use crate::difftest::check_paths;
    use crate::difftest::events::driven;

    use crate::difftest::elf_image;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    pub(crate) use crate::game::render::lightman::tests::light_man_empty_method_check;
    pub(crate) use crate::game::render::particle::tests::GRID_MESH_AT;

    #[test]
    fn light_man_v08_matches() {
        light_man_empty_method_check(ee::s13::f_00394d30__generated, light_man_v08, 0x394d30);
    }

    #[test]
    fn light_man_v13_matches() {
        light_man_empty_method_check(ee::s13::f_00394d38__generated, light_man_v13, 0x394d38);
    }

    #[test]
    fn light_man_v14_matches() {
        light_man_empty_method_check(ee::s13::f_00394d40__generated, light_man_v14, 0x394d40);
    }

    #[test]
    fn light_man_v15_matches() {
        light_man_empty_method_check(ee::s13::f_00394d48__generated, light_man_v15, 0x394d48);
    }

    /// Gate of the destructor fake of the `LightMan_dtor_Wrapper` test.
    pub(crate) const LIGHT_MAN_DTOR_GATE: u32 = DATA + 0x9_0010;

    pub(crate) const LIGHT_MAN_DTOR_MAGIC: u32 = 0x4454_4f52;

    pub(crate) static LIGHT_MAN_DTOR_PREV: AtomicUsize = AtomicUsize::new(0);

    /// `LightMan::~LightMan` for the wrapper's test: unwinds (pc away from
    /// the return address, nothing else changed) when the gate is set, else
    /// the real destructor.
    pub(crate) fn light_man_dtor_wrapper_fake_dtor(c: &mut Cpu) {
        if c.bus.read32(LIGHT_MAN_DTOR_GATE) != LIGHT_MAN_DTOR_MAGIC {
            return fakes::fall_back(c, &LIGHT_MAN_DTOR_PREV, ee::s12::f_0038ae28__generated);
        }
        c.pc = 0xdead_0000;
    }

    /// `LightMan_dtor_Wrapper`: a light manager of random words, the real
    /// destructor (vtable, gp+0x2a9c cleared) without the delete flag (the
    /// real delete would free a random block): 7 + 11 cycles; or
    /// (`seed / 4 % 2`) a gated fake destructor that unwinds: 7.
    #[test]
    fn light_man_dtor_wrapper_matches() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038AE28),
            light_man_dtor_wrapper_fake_dtor,
            &LIGHT_MAN_DTOR_PREV,
        );
        check_paths(
            ee::s13::f_00394c68__generated,
            light_man_dtor_wrapper,
            100,
            &[18, 7],
            &|c, seed| {
                let mut r = Rng(0x0039_4c68 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                fill(c, &mut r, DATA, 0x100);
                c.bus.write32(GAME_GP + 0x2a9c, r.next_u32());
                c.g[A0][0] = sx(DATA + 0x234 + (seed % 4) * 0x20);
                c.g[A1][0] &= !1;
                if seed / 4 % 2 == 1 {
                    c.bus.write32(LIGHT_MAN_DTOR_GATE, LIGHT_MAN_DTOR_MAGIC);
                }
                c.pc = 0x0039_4c68;
            },
        );
    }

    /// `LightMan_GetInstance`: random table pointers and ids (negative too,
    /// junk in the upper words).
    #[test]
    fn light_man_get_instance_matches() {
        check_paths(
            ee::s13::f_00394c88__generated,
            light_man_get_instance,
            100,
            &[4],
            &|c, seed| {
                let mut r = Rng(0x0039_4c88 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                fill(c, &mut r, DATA, 0x100);
                c.g[A0][0] = sx(DATA + 0x124 + (seed % 4) * 0x20);
            },
        );
    }

    /// `LightMan_ResetCounter`: a light manager of random words; only +0xc
    /// is cleared.
    #[test]
    fn light_man_reset_counter_matches() {
        check_paths(
            ee::s13::f_00394d10__generated,
            light_man_reset_counter,
            100,
            &[2],
            &|c, seed| {
                let mut r = Rng(0x0039_4d10 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                fill(c, &mut r, DATA, 0x100);
                c.g[A0][0] = sx(DATA + 0x234 + (seed % 4) * 0x20);
            },
        );
    }

    pub(crate) const RENDERER_INIT_ENTRY: u32 = 0x0039_3048;

    /// Gate of this test's callee fakes (a word in the machine's RAM).
    pub(crate) const RENDERER_INIT_FAKE_CTRL: u32 = DATA + 0x2_0000;

    pub(crate) const RENDERER_INIT_FAKE_MAGIC: u32 = 0x5245_4e49;

    /// Cycles each fake charges (a fixed cost, so the path totals are known).
    pub(crate) const RENDERER_INIT_FAKE_COST: u64 = 5;

    /// Run `fakes::fake` for `target` when the gate is set (fixed cost),
    /// else what the slot held before.
    pub(crate) fn renderer_init_fake_call(
        c: &mut Cpu,
        target: u32,
        prev: &AtomicUsize,
        generated: fn(&mut Cpu),
    ) {
        if c.bus.read32(RENDERER_INIT_FAKE_CTRL) != RENDERER_INIT_FAKE_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        let ra = c.g[RA][0] as u32;
        let start = c.cycles;
        fakes::fake(c, target, ra);
        c.cycles = start + RENDERER_INIT_FAKE_COST;
    }

    pub(crate) fn renderer_init_fake_memset(c: &mut Cpu) {
        renderer_init_fake_call(
            c,
            0x0041_6210,
            fakes::prev_cell(renderer_init_fake_memset),
            ee::s15::f_00416210__generated,
        )
    }

    /// `EA_MemCopy`, the copy inside `Rand_GetDefaultState` (which has no
    /// slot of its own).
    pub(crate) fn renderer_init_fake_ea_mem_copy(c: &mut Cpu) {
        renderer_init_fake_call(
            c,
            0x003e_6574,
            fakes::prev_cell(renderer_init_fake_ea_mem_copy),
            ee::s14::f_003e6574__generated,
        )
    }

    pub(crate) fn renderer_init_fake_spr_stream_set_mode(c: &mut Cpu) {
        renderer_init_fake_call(
            c,
            0x0038_f300,
            fakes::prev_cell(renderer_init_fake_spr_stream_set_mode),
            ee::s12::f_0038f300__generated,
        )
    }

    pub(crate) fn renderer_init_fake_build_sine_table(c: &mut Cpu) {
        renderer_init_fake_call(
            c,
            0x0039_2df0,
            fakes::prev_cell(renderer_init_fake_build_sine_table),
            ee::s13::f_00392df0__generated,
        )
    }

    pub(crate) fn renderer_init_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416210),
            renderer_init_fake_memset,
            fakes::prev_cell(renderer_init_fake_memset),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6574),
            renderer_init_fake_ea_mem_copy,
            fakes::prev_cell(renderer_init_fake_ea_mem_copy),
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038F300),
            renderer_init_fake_spr_stream_set_mode,
            fakes::prev_cell(renderer_init_fake_spr_stream_set_mode),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00392DF0),
            renderer_init_fake_build_sine_table,
            fakes::prev_cell(renderer_init_fake_build_sine_table),
        );
    }

    /// The callees a seed runs.
    #[derive(Clone, Copy, PartialEq)]
    pub(crate) enum RendererInitCallees {
        /// The real memset, Rand_GetDefaultState, SprStream_SetMode and
        /// Renderer_BuildSineTable (the game image supplies their tables).
        Real,
        /// Fakes; fake call `unwind_at` (0 memset, 1 EA_MemCopy, 2
        /// SprStream_SetMode, 3 Renderer_BuildSineTable) unwinds.
        Faked { unwind_at: u32 },
    }

    /// Scrambled registers (and the game image for real callees), every
    /// output area and frame slot pre-filled with random words, a random
    /// literal pool at gp-0x2690..gp-0x2560 (so a misplaced literal shows),
    /// and the arguments.
    pub(crate) fn renderer_init_setup(
        c: &mut Cpu,
        seed: u32,
        initialize: u64,
        priority: u64,
        callees: RendererInitCallees,
    ) -> Rng {
        if callees == RendererInitCallees::Real {
            elf_image(c);
        }
        let mut r = machine(c, seed, 0x0039_3048, 0);
        fill(
            c,
            &mut r,
            RENDERER_VEC4S_A,
            (0x0050_1440 - RENDERER_VEC4S_A) / 4,
        );
        fill(
            c,
            &mut r,
            RENDERER_VEC4S_B,
            (0x0050_5a00 - RENDERER_VEC4S_B) / 4,
        );
        fill(c, &mut r, GAME_GP + 0x28b0, 0x24);
        let frame_base = crate::difftest::SP - RENDERER_INIT_FRAME;
        fill(c, &mut r, frame_base, RENDERER_INIT_FRAME / 4);
        fill(c, &mut r, GAME_GP - 0x2690, 0x4c);
        if let RendererInitCallees::Faked { unwind_at } = callees {
            c.bus
                .write32(RENDERER_INIT_FAKE_CTRL, RENDERER_INIT_FAKE_MAGIC);
            c.bus.write32(fakes::LOG_UNWIND_AT, unwind_at);
        }
        c.g[A0][0] = initialize;
        c.g[A1][0] = priority;
        c.pc = RENDERER_INIT_ENTRY;
        r
    }

    /// Renderer_StaticInitDestroy. Digits: `seed % 4` picks the pass
    /// (0: another priority, 1: destroy pass, 2-3: construct pass);
    /// `seed / 4 % 6` the callees (0 real, 1 fakes, 2-5 fake call 0-3
    /// unwinds); `seed / 24 % 3` the argument values (priorities 0, 0xfffe
    /// and 0x1_0000_ffff; initialize 1, 0x1_0000_0000 and a negative one).
    ///
    /// Paths (cycles): other priority 33; destroy pass 35; construct with
    /// fakes 1466; memset unwinds 605; EA_MemCopy (in
    /// Rand_GetDefaultState) unwinds 1008; SprStream_SetMode unwinds 1445;
    /// Renderer_BuildSineTable unwinds 1450. Real-callee seeds take a path
    /// of their own.
    #[test]
    fn renderer_static_init_destroy_matches() {
        renderer_init_install_fakes();
        check_paths(
            ee::s13::f_00393048__generated,
            renderer_static_init_destroy,
            288,
            &[33, 35, 605, 1008, 1445, 1450, 1466],
            &|c, seed| {
                let pass = seed % 4;
                let callees = match seed / 4 % 6 {
                    0 => RendererInitCallees::Real,
                    1 => RendererInitCallees::Faked {
                        unwind_at: u32::MAX,
                    },
                    n => RendererInitCallees::Faked { unwind_at: n - 2 },
                };
                let value = (seed / 24 % 3) as usize;
                let (initialize, priority) = match pass {
                    0 => (1, [0, 0xfffe, 0x1_0000_ffff][value]),
                    1 => (0, RENDERER_INIT_PRIORITY),
                    _ => (
                        [1, 0x1_0000_0000, 0xffff_ffff_8000_0000][value],
                        RENDERER_INIT_PRIORITY,
                    ),
                };
                renderer_init_setup(c, seed, initialize, priority, callees);
            },
        );
    }

    driven!(renderer_init_gen_driven, ee::s13::f_00393048__generated);

    driven!(renderer_init_hand_driven, renderer_static_init_destroy);

    /// Cycles after entry at which the event falls due, one per loop head
    /// (with fakes): the global halfword loop (565), the halfword loop
    /// (586), the 8-array loop (612..654), the 3-array loop (713, 720), the
    /// 6x6 inner (1017..) and outer (1055) loops, and the two 8-array loops
    /// (1302..1350, 1367..1409).
    pub(crate) const RENDERER_INIT_EVENT_AT: [u64; 8] =
        [560, 580, 630, 715, 1030, 1050, 1320, 1390];

    /// Renderer_StaticInitDestroy (construct pass, fakes that return) with
    /// an event due at each loop head in turn (`seed / 3 % 8`): delivered
    /// or asking to unwind (the structured loops carry on either way).
    #[test]
    fn renderer_static_init_destroy_events_match() {
        renderer_init_install_fakes();
        check_events(
            renderer_init_gen_driven,
            renderer_init_hand_driven,
            144,
            &|c, seed| {
                let mut r = renderer_init_setup(
                    c,
                    seed / 3,
                    1,
                    RENDERER_INIT_PRIORITY,
                    RendererInitCallees::Faked {
                        unwind_at: u32::MAX,
                    },
                );
                let mode = schedule(c, &mut r, seed, RENDERER_INIT_ENTRY);
                if mode != Events::Off {
                    c.next_event = c.cycles + RENDERER_INIT_EVENT_AT[(seed / 3 % 8) as usize];
                }
                mode
            },
        );
    }

    /// `cPS2GridMesh_IsValid` against the generated body (one path, 2 cycles).
    #[test]
    fn c_ps2_grid_mesh_is_valid_matches() {
        check_paths(
            ee::s13::f_00396108__generated,
            c_ps2_grid_mesh_is_valid,
            100,
            &[2],
            &|c, seed| {
                let mut r = machine(c, seed, 0x39_6108, 0);
                c.pc = 0x0039_6108;
                fill(c, &mut r, GRID_MESH_AT, 0x74);
                if seed % 2 == 0 {
                    c.bus.write32(GRID_MESH_AT + gmesh::VALID, 0);
                }
                c.g[A0][0] = sx(GRID_MESH_AT);
            },
        );
    }

    /// `Renderer_GlobalCtors` against the generated body: the real
    /// `Renderer_StaticInitDestroy(1, 0xffff)` runs on the game image.
    #[test]
    fn renderer_global_ctors_matches() {
        check(
            ee::s13::f_003970f8__generated,
            renderer_global_ctors,
            100,
            &|c, seed| {
                machine(c, seed, 0x39_70f8, 0);
                elf_image(c);
                c.pc = 0x0039_70f8;
            },
        );
    }

    /// `Renderer_GlobalDtors` against the generated body: the real
    /// `Renderer_StaticInitDestroy(0, 0xffff)` takes its short path (43).
    #[test]
    fn renderer_global_dtors_matches() {
        check_paths(
            ee::s13::f_00397118__generated,
            renderer_global_dtors,
            100,
            &[43],
            &|c, seed| {
                machine(c, seed, 0x39_7118, 0);
                c.pc = 0x0039_7118;
            },
        );
    }

    use crate::difftest::fresh;
    use crate::difftest::require_paths;

    // -- Constructor and destructor -----------------------------------------

    /// Gate of this module's test fakes (a magic word in the machine's RAM).
    pub(crate) const PS2GM_FAKE_GATE: u32 = DATA + 0x8_0000;

    pub(crate) const PS2GM_FAKE_MAGIC: u32 = 0x5047_4d64;

    pub(crate) static PS2GM_DTOR_PREV_BASE_DTOR: AtomicUsize = AtomicUsize::new(0);

    /// `GraphicsMan_dtor` for `PS2GraphicsMan_dtor`'s test: the logging fake
    /// (may unwind) when the gate is set, else the body it replaced.
    pub(crate) fn ps2_graphics_man_dtor_fake_base_dtor(c: &mut Cpu) {
        if c.bus.read32(PS2GM_FAKE_GATE) != PS2GM_FAKE_MAGIC {
            return fakes::fall_back(
                c,
                &PS2GM_DTOR_PREV_BASE_DTOR,
                ee::s12::f_003695d8__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0036_95d8, ra);
    }

    /// `PS2GraphicsMan_dtor`: the real `GraphicsMan_dtor` on its no-delete
    /// path (`flags & 1 == 0`, 7 + 11 = 18 cycles), the fake returning, or
    /// the fake unwinding (`seed % 3`; the object address from `seed / 3`).
    #[test]
    fn ps2_graphics_man_dtor_matches() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_003695D8),
            ps2_graphics_man_dtor_fake_base_dtor,
            &PS2GM_DTOR_PREV_BASE_DTOR,
        );
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x0039_5268, 1);
            let this = DATA + 0x1_0000 + 0x10 * (seed / 3 % 16);
            fill(c, &mut r, this + 0x10c0, 16);
            fill(c, &mut r, GAME_GP + 0x2a80, 8);
            if seed % 3 != 0 {
                c.bus.write32(PS2GM_FAKE_GATE, PS2GM_FAKE_MAGIC);
            }
            let unwind_at = if seed % 3 == 2 { 0 } else { u32::MAX };
            c.bus.write32(fakes::LOG_UNWIND_AT, unwind_at);
            c.g[A0][0] = sx(this);
            c.g[A1][0] &= !1;
            c.pc = 0x0039_5268;
        };
        fakes::covered(
            ee::s13::f_00395268__generated,
            ps2_graphics_man_dtor,
            120,
            &setup,
        );
        require_paths(ee::s13::f_00395268__generated, 120, &[18], &setup);
    }

    pub(crate) const PS2GM_CTOR_THIS: u32 = DATA + 0x1_0030;

    /// A graphics manager to construct: its first 0x1300 bytes (base
    /// fields, the vtable word and the render-slot arrays) filled with
    /// random words.
    pub(crate) fn ps2_graphics_man_ctor_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = Rng(0x3952_8800 ^ (seed as u64) << 12);
        scramble(c, &mut r);
        c.g[GP][0] = sx(GAME_GP);
        fill(c, &mut r, PS2GM_CTOR_THIS, 0x1300 / 4);
        c.g[A0][0] = sx(PS2GM_CTOR_THIS);
        c.pc = 0x0039_5288;
        r
    }

    /// Cycles the base `GraphicsMan_ctor` takes on the fixture.
    pub(crate) fn ps2_graphics_man_ctor_base_cycles(seed: u32) -> u64 {
        let mut c = fresh();
        ps2_graphics_man_ctor_fixture(&mut c, seed);
        c.pc = 0x0036_91f8;
        let before = c.cycles;
        ee::s12::GraphicsMan_ctor(&mut c);
        assert_eq!(c.pc, crate::difftest::RA, "GraphicsMan_ctor did not return");
        c.cycles - before
    }

    /// The constructor's own charges: 10 + 12*7 + 7 + 4*7 + 4.
    pub(crate) const PS2GM_CTOR_OWN_CYCLES: u64 = 133;

    #[test]
    fn ps2_graphics_man_ctor_matches_gfx1() {
        let total = PS2GM_CTOR_OWN_CYCLES + ps2_graphics_man_ctor_base_cycles(0);
        check_paths(
            ee::s13::f_00395288__generated,
            crate::game::render::font::ps2_graphics_man_ctor,
            100,
            &[total],
            &|c, seed| {
                ps2_graphics_man_ctor_fixture(c, seed);
            },
        );
    }

    driven!(gen_ps2_graphics_man_ctor, ee::s13::f_00395288__generated);

    driven!(
        hand_ps2_graphics_man_ctor_gfx1,
        crate::game::render::font::ps2_graphics_man_ctor
    );

    /// Events at the back edges of both counting loops (0x3952b0 and
    /// 0x3952e8): the first event is due after the base constructor has
    /// returned, somewhere inside the two loops.
    #[test]
    fn ps2_graphics_man_ctor_events_gfx1() {
        let base = ps2_graphics_man_ctor_base_cycles(0);
        check_events(
            gen_ps2_graphics_man_ctor,
            hand_ps2_graphics_man_ctor_gfx1,
            150,
            &|c, seed| {
                let mut r = ps2_graphics_man_ctor_fixture(c, seed / 3);
                let mode = schedule(c, &mut r, seed, 0x0039_5288);
                if mode != Events::Off {
                    // Loop back edges are at +7..+77 and +98..+112 after the
                    // base constructor returns.
                    c.next_event = c.cycles + 10 + base + (r.next_u32() % 113) as u64;
                }
                mode
            },
        );
    }

    // -- Small accessors -----------------------------------------------------

    /// A graphics manager at a varied word address with varied non-zero
    /// words around `field` (both signs), `a0` carrying a random upper
    /// half, and `pc` at `entry`. Returns `this`.
    pub(crate) fn ps2_graphics_man_field_fixture(
        c: &mut Cpu,
        seed: u32,
        field: u32,
        entry: u32,
    ) -> u32 {
        let mut r = Rng(entry as u64 ^ (seed as u64) << 24 ^ field as u64);
        scramble(c, &mut r);
        let this = DATA + 0x4_0000 + 4 * (seed % 61);
        for i in 0..12 {
            let w = r.next_u32() | 0x0100_0100;
            c.bus.write32(this + field - 0x18 + 4 * i, w);
        }
        c.g[A0][0] = (r.next_u32() as u64) << 32 | this as u64;
        c.pc = entry;
        this
    }

    #[test]
    fn ps2_graphics_man_get_display_mode_count_matches() {
        check_paths(
            ee::s13::f_00395318__generated,
            ps2_graphics_man_get_display_mode_count,
            100,
            &[2],
            &|c, seed| {
                ps2_graphics_man_field_fixture(c, seed, renderer_fields::MODE_COUNT, 0x0039_5318);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_is_progressive_matches() {
        check_paths(
            ee::s13::f_00395320__generated,
            ps2_graphics_man_is_progressive,
            100,
            &[2],
            &|c, seed| {
                ps2_graphics_man_field_fixture(c, seed, renderer_fields::PROGRESSIVE, 0x0039_5320);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_get_display_mode_index_matches() {
        check_paths(
            ee::s13::f_00395328__generated,
            ps2_graphics_man_get_display_mode_index,
            100,
            &[2],
            &|c, seed| {
                ps2_graphics_man_field_fixture(c, seed, renderer_fields::CUR_MODE, 0x0039_5328);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_get_fog_state_matches_gfx1() {
        check_paths(
            ee::s13::f_003953b0__generated,
            crate::game::render::font::ps2_graphics_man_get_fog_state,
            100,
            &[2],
            &|c, seed| {
                ps2_graphics_man_field_fixture(c, seed, GM_FOG_MODE, 0x0039_53b0);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_clear_lights_matches_gfx1() {
        check_paths(
            ee::s13::f_00395420__generated,
            crate::game::render::font::ps2_graphics_man_clear_lights,
            100,
            &[2],
            &|c, seed| {
                ps2_graphics_man_field_fixture(c, seed, GM_LIGHTS, 0x0039_5420);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_get_lighting_matches() {
        check_paths(
            ee::s13::f_00395500__generated,
            ps2_graphics_man_get_lighting,
            100,
            &[2],
            &|c, seed| {
                ps2_graphics_man_field_fixture(c, seed, GM_LIGHTING, 0x0039_5500);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_v110_matches_gfx1() {
        check_paths(
            ee::s13::f_00395508__generated,
            crate::game::render::font::ps2_graphics_man_v110,
            100,
            &[2],
            &|c, seed| {
                ps2_graphics_man_field_fixture(c, seed, 0x40, 0x0039_5508);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_get_flip_count_matches_gfx1() {
        check_paths(
            ee::s13::f_00395510__generated,
            crate::game::render::font::ps2_graphics_man_get_flip_count,
            100,
            &[2],
            &|c, seed| {
                ps2_graphics_man_field_fixture(c, seed, renderer_fields::FLIP_COUNT, 0x0039_5510);
            },
        );
    }

    /// `a1` keeps scramble's random upper half (only the low word is stored).
    #[test]
    fn ps2_graphics_man_v98_matches() {
        check_paths(
            ee::s13::f_003956e0__generated,
            ps2_graphics_man_v98,
            100,
            &[2],
            &|c, seed| {
                ps2_graphics_man_field_fixture(c, seed, renderer_fields::F6D14, 0x0039_56e0);
            },
        );
    }

    /// Top pointer, depth and the view-valid word all random non-zero.
    #[test]
    fn ps2_graphics_man_pop_matrix_matches_gfx1() {
        check_paths(
            ee::s13::f_00395730__generated,
            crate::game::render::font::ps2_graphics_man_pop_matrix,
            100,
            &[8],
            &|c, seed| {
                let this = ps2_graphics_man_field_fixture(c, seed, man::CAMERA, 0x0039_5730);
                let mut r = Rng(0x3957_3000 ^ seed as u64);
                for i in 0..8 {
                    let w = r.next_u32() | 0x0100_0100;
                    c.bus.write32(this + man::TEX_READY - 0x10 + 4 * i, w);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_get_matrix_matches_gfx1() {
        check_paths(
            ee::s13::f_00395c68__generated,
            crate::game::render::font::ps2_graphics_man_get_matrix,
            100,
            &[2],
            &|c, seed| {
                ps2_graphics_man_field_fixture(c, seed, man::CAMERA, 0x0039_5c68);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_v68_matches() {
        check_paths(
            ee::s13::f_00395fa0__generated,
            ps2_graphics_man_v68,
            100,
            &[2],
            &|c, seed| {
                ps2_graphics_man_field_fixture(c, seed, 0x40, 0x0039_5fa0);
            },
        );
    }

    // -- v33 ---------------------------------------------------------------

    pub(crate) static PS2GM_V33_PREV_SIN_COS: AtomicUsize = AtomicUsize::new(0);

    /// `Math_SinCos` for `v33`'s test: the logging fake (may unwind; it
    /// writes no sine or cosine) when the gate is set, else the body it
    /// replaced.
    pub(crate) fn ps2_graphics_man_v33_fake_sin_cos(c: &mut Cpu) {
        if c.bus.read32(PS2GM_FAKE_GATE) != PS2GM_FAKE_MAGIC {
            return fakes::fall_back(c, &PS2GM_V33_PREV_SIN_COS, ee::s10::f_0031be50__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_be50, ra);
    }

    /// `v33` on random data: the manager at DATA+0x10000 with its matrix
    /// stack top at one of four 16-byte-aligned slots, random floats in the
    /// identity source, the three axes, the position and the stack frame.
    /// `seed % 5`: the real `Math_SinCos` (its constants from the game
    /// image), the fake returning from all three calls, or the fake
    /// unwinding at call 0, 1 or 2 (each call's return entered).
    /// `seed / 5 % 3 == 0`: edge-case angles, else within +-7 rad.
    #[test]
    fn ps2_graphics_man_v33_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0031BE50),
            ps2_graphics_man_v33_fake_sin_cos,
            &PS2GM_V33_PREV_SIN_COS,
        );
        fakes::covered(
            ee::s13::f_00395750__generated,
            ps2_graphics_man_v33,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0039_5750, 3);
                elf_image(c);
                let calls = seed % 5;
                if calls != 0 {
                    c.bus.write32(PS2GM_FAKE_GATE, PS2GM_FAKE_MAGIC);
                }
                let unwind_at = if calls >= 2 { calls - 2 } else { u32::MAX };
                c.bus.write32(fakes::LOG_UNWIND_AT, unwind_at);
                let this = DATA + 0x1_0000;
                fill(c, &mut r, this + renderer_fields::MATRIX_DEPTH - 0x10, 12);
                fill(c, &mut r, this + man::TEX_READY - 0x10, 8);
                let top = DATA + 0x3_0010 + 0x40 * (seed / 15 % 4);
                c.bus.write32(this + man::CAMERA, top);
                for at in (DATA + 0x3_0000..DATA + 0x3_0200).step_by(4) {
                    let v = r.f32(4.0);
                    c.bus.write32(at, v);
                }
                for base in [IDENTITY_MATRIX, V33_AXIS_1, V33_AXIS_2, V33_AXIS_3] {
                    let words = if base == IDENTITY_MATRIX { 16 } else { 4 };
                    for i in 0..words {
                        let v = r.f32(1.5);
                        c.bus.write32(base + 4 * i, v);
                    }
                }
                let pos = DATA + 0x2_0000 + 0x10 * (r.next_u32() % 8);
                for i in 0..4 {
                    let v = r.f32(500.0);
                    c.bus.write32(pos + 4 * i, v);
                }
                for f in 12..15 {
                    c.f[f] = if seed / 5 % 3 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(7.0)
                    };
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pos);
                c.pc = 0x0039_5750;
            },
        );
    }
    pub(crate) use crate::game::render::particle::tests::oc690_gfx_leaf;
    pub(crate) use crate::game::render::particle::tests::GFX;
    pub(crate) use crate::game::render::particle::tests::SEEDS_13 as SEEDS_5;

    /// `GetFogState`, `ClearLights`, `v110`, `GetFlipCount`: leaf accessors.
    #[test]
    fn ps2_graphics_man_leaf_accessors_match() {
        let fog: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _ = oc690_gfx_leaf(c, seed, 0x690_0005, 0x0039_53b0);
        };
        check(
            ee::s13::f_003953b0__generated,
            ps2_graphics_man_get_fog_state,
            SEEDS_5,
            fog,
        );
        let lights: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _ = oc690_gfx_leaf(c, seed, 0x690_0006, 0x0039_5420);
        };
        check(
            ee::s13::f_00395420__generated,
            ps2_graphics_man_clear_lights,
            SEEDS_5,
            lights,
        );
        let v110: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _ = oc690_gfx_leaf(c, seed, 0x690_0007, 0x0039_5508);
        };
        check(
            ee::s13::f_00395508__generated,
            ps2_graphics_man_v110,
            SEEDS_5,
            v110,
        );
        let flips: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _ = oc690_gfx_leaf(c, seed, 0x690_0008, 0x0039_5510);
        };
        check(
            ee::s13::f_00395510__generated,
            ps2_graphics_man_get_flip_count,
            SEEDS_5,
            flips,
        );
    }

    /// `PS2GraphicsMan_ctor`: the real base constructor (0x3691f8: short
    /// memset calls on `gfx`) and the two delay loops. Events are not
    /// scheduled (see the report).
    #[test]
    fn ps2_graphics_man_ctor_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _ = machine(c, seed, 0x690_0009, 0);
            c.pc = 0x0039_5288;
            c.g[A0][0] = GFX as u64;
        };
        check(
            ee::s13::f_00395288__generated,
            ps2_graphics_man_ctor,
            16,
            setup,
        );
    }

    /// `PopMatrix`: any stack pointer and depth (the arithmetic is 32-bit).
    #[test]
    fn ps2_graphics_man_pop_matrix_matches() {
        check(
            ee::s13::f_00395730__generated,
            ps2_graphics_man_pop_matrix,
            100,
            &|c, seed| {
                let mut r = Rng(0x0039_5730 ^ ((seed as u64) << 8));
                scramble(c, &mut r);
                c.pc = 0x0039_5730;
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x13e4, r.next_u32());
                c.bus.write32(this + 0x13e0, r.next_u32());
                c.bus.write32(this + 0x6b90, r.next_u32());
            },
        );
    }

    /// `GetMatrix`: returns the top-of-stack pointer.
    #[test]
    fn ps2_graphics_man_get_matrix_matches() {
        check(
            ee::s13::f_00395c68__generated,
            ps2_graphics_man_get_matrix,
            100,
            &|c, seed| {
                let mut r = Rng(0x0039_5c68 ^ ((seed as u64) << 8));
                scramble(c, &mut r);
                c.pc = 0x0039_5c68;
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x13e4, r.next_u32());
            },
        );
    }

    pub(crate) use crate::game::fe::debugfe::tests::OC798_OBJ;
    pub(crate) use crate::game::fe::feridermanager::tests::OC760_WORLD;
    pub(crate) use crate::game::fe::festatecharselect::tests::oc762_force;

    pub(crate) use crate::game::fe::festatecharselect::tests::FX_SELF_2 as FX_SELF;

    pub(crate) use crate::game::sound::bankmonitor::tests::OC760_CTRL;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC760_MAGIC;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC760_OBJ;
    pub(crate) use crate::game::sound::soundsys::tests::oc780_arm;

    pub(crate) use crate::game::worldpainter::worldpainterman::tests::oc762_machine;

    pub(crate) const OC760_FILE: u32 = DATA + 0x22_0000;

    pub(crate) const OC760_VT: u32 = DATA + 0x24_0000;

    /// Build a font file with `n` glyph records and a texture header, the
    /// world object the texture upload reads, and the fakes' gate.
    #[test]
    fn font_parse_glyphs_matches() {
        check(
            ee::s12::f_00391788__generated,
            font_parse_glyphs,
            200,
            &|c, seed| {
                oc760_install_fakes();
                let mut r = machine(c, seed, 0x39_1788, 12);
                c.bus.write32(OC760_CTRL, OC760_MAGIC);
                let font = OC760_OBJ;
                let file = OC760_FILE;
                let n = 1 + seed % 7;
                c.g[A0][0] = sx(font);
                c.g[A1][0] = sx(file);
                c.bus.write16(file + 0xa, n as u16);
                c.bus.write8(file + 0x10, r.next_u32() as u8);
                c.bus.write8(file + 0x11, r.next_u32() as u8);
                c.bus.write32(file + 0x14, 0x100); // glyph records
                c.bus.write32(file + 0x1c, 0x400); // texture header
                for i in 0..n {
                    let g = file + 0x100 + 12 * i;
                    // Codes: space, no-break space, the 0xac fallback, consecutive, random.
                    let code = match r.next_u32() % 5 {
                        0 => 0x20,
                        1 => 0xa0,
                        2 => 0xac,
                        3 => i,
                        _ => r.next_u32() & 0xffff,
                    };
                    c.bus.write16(g, code as u16);
                    for b in 2..11 {
                        // Heights and offsets: bytes, some negative.
                        c.bus.write8(g + b, r.next_u32() as u8);
                    }
                }
                // The texture header.
                let tex = file + 0x400;
                c.bus.write16(tex + 4, 8 + 2 * (r.next_u32() & 7) as u16);
                c.bus.write16(tex + 6, 1 + (r.next_u32() & 0x3f) as u16);
                c.bus
                    .write32(tex + 0xc, if seed % 2 == 0 { 0x1000 } else { 0 });
                c.bus.write32(tex + 0x10, 0x40);
                // gp+0x2a90: the world; world+0x10d8: its vtable block.
                c.bus.write32(GAME_GP + 0x2a90, OC760_WORLD);
                c.bus.write32(OC760_WORLD + 0x10d8, OC760_VT);
                c.bus.write16(OC760_VT + 0x178, r.next_u32() as u16 & 0xff);
                c.bus.write32(OC760_VT + 0x17c, fakes::FAKE);
            },
        );
    }

    pub(crate) const FX_NAME: u32 = DATA + 0x21_5000;

    /// `Font_UploadTexture`: image heights 1..200 (so 1 to 8 doublings),
    /// widths 2..64, with and without the 0x1000 "offset" flag; the two
    /// buffer allocations return a pointer or 0 (forced), skipping the
    /// matching `free`; the graphics-manager virtual call and the memory
    /// helpers are fakes, some of which unwind.
    #[test]
    fn font_upload_texture_matches() {
        oc762_install_fakes();
        covered(
            ssx3_ee::s12::f_00391a48__generated,
            font_upload_texture,
            300,
            &|c, seed| {
                let mut r = oc762_machine(c, seed, 0x0039_1a48, 7);
                fill(c, &mut r, FX_SELF, 0x40);
                let h = [1u32, 2, 3, 16, 31, 64, 100, 200, 0xffff][(seed % 9) as usize];
                let w = 2 + 2 * (r.next_u32() % 32);
                c.bus.write16(FX_NAME + 4, w as u16);
                c.bus.write16(FX_NAME + 6, h as u16);
                c.bus.write32(
                    FX_NAME + 0xc,
                    if seed % 2 == 0 {
                        0x1000
                    } else {
                        r.next_u32() & !0x1000
                    },
                );
                c.bus.write32(FX_NAME + 0x10, 0x20 + (r.next_u32() & 0x3c));
                if seed % 7 == 3 {
                    oc762_force(c, 0x0031_7e30, 0, 0);
                }
                if seed % 5 == 1 {
                    oc762_force(c, 0x0031_7e30, 1, 0);
                }
                c.g[A0][0] = sx(FX_SELF);
                c.g[A1][0] = sx(FX_NAME);
            },
        );
    }

    /// `DeferMan_ctor`: 3600 `memset` calls (faked, may unwind) between the
    /// record loops.
    #[test]
    fn defer_man_ctor_matches() {
        oc780_install_fakes();
        fakes::covered(
            ee::s13::f_00394d50__generated,
            defer_man_ctor,
            40,
            &|c, seed| {
                machine(c, seed, 0x780e, 3600);
                oc780_arm(c);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    /// `UIEngine_StoreWord`: stores `a1` at `a0`.
    #[test]
    fn ui_engine_store_word_matches() {
        check(
            ee::s13::f_00397138__generated,
            ui_engine_store_word,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x798a, 0);
                c.pc = 0x0039_7138;
                c.g[RA][0] = sx(crate::difftest::RA);
                c.g[A0][0] = sx(OC798_OBJ + 4 * (seed & 7));
                c.g[A1][0] = r.next_u32() as u64 | ((seed as u64) << 40);
            },
        );
    }

    use crate::difftest::fakes::FAKE;

    /// Font object, string and destination for
    /// [`font_find_line_break_w`]: a glyph table the seed walks (region
    /// prefilled non-zero so the byte-addressed direct-index reads exercise
    /// the pen advance), six string kinds from `seed / 7 % 6` (spaced,
    /// unspaced, tail-searched, default-glyph, and below-`first` codes that
    /// take the no-glyph arm), and `s6` pinned inside the destination so the
    /// old `a2`-clobber body writes visibly wrong.
    pub(crate) const FIND_FONT: u32 = DATA + 0x1234;

    pub(crate) const FIND_STR: u32 = DATA + 0x20_0000;

    pub(crate) const FIND_DST: u32 = DATA + 0x21_0000;

    pub(crate) const FIND_GLYPHS: u32 = DATA + 0x22_0000;

    /// Fixture for [`font_find_line_break_w`]: direct-index glyphs for the
    /// codes the string uses, a sorted tail with the default glyph, and a
    /// width the seed steers (0, tight, roomy).
    pub(crate) fn find_line_break_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = machine(c, seed, 0x39_2680, 0);
        c.pc = 0x0039_2680;
        // Glyph range 0x20..0x60 direct, sorted tail above.
        // Total records: 0x40 direct + 4 tail. (4 degenerated the binary
        // search above `last` to a single probe, hiding its mutants.)
        c.bus.write32(FIND_FONT + font_res::COUNT, 0x40 + 4);
        c.bus.write32(FIND_FONT + font_res::TABLE, FIND_GLYPHS);
        c.bus
            .write32(FIND_FONT + font_res::DEF, FIND_GLYPHS + 0x100);
        c.bus.write32(FIND_FONT + font_res::FIRST, 0x20);
        c.bus.write32(FIND_FONT + font_res::LAST, 0x5f);
        c.bus
            .write32(FIND_FONT + font_res::PAD_X, f32::to_bits(2.0));
        fill(c, &mut r, FIND_GLYPHS, 0x120);
        for i in 0..0x40u32 {
            let g = FIND_GLYPHS + 16 * i;
            c.bus.write16(g, (0x20 + i) as u16);
            c.bus.write8(g + glyph::WIDTH, 8);
            c.bus.write8(g + glyph::ADV, 9);
            c.bus.write8(g + glyph::XOFF, 1);
        }
        for i in 0..4u32 {
            let g = FIND_GLYPHS + 16 * (0x40 + i);
            c.bus.write16(g, (0x70 + 2 * i) as u16);
            c.bus.write8(g + glyph::WIDTH, 8);
            c.bus.write8(g + glyph::ADV, 9);
            c.bus.write8(g + glyph::XOFF, 1);
        }
        // The default glyph's metrics differ from every table glyph's, so
        // a wrong search direction (hit vs default) changes the pen.
        let def = FIND_GLYPHS + 0x100;
        c.bus.write16(def, 0x7fu16);
        c.bus.write8(def + glyph::WIDTH, 3);
        c.bus.write8(def + glyph::ADV, 4);
        c.bus.write8(def + glyph::XOFF, 0xfb);
        // String: seed-picked codes with spaces, then NUL. Kind 5 carries
        // non-zero codes below `first` (0x01) mid-string: they pass the scan
        // head (only NUL exits there) and take the no-glyph arm. Kind 6
        // carries 0x5f (direct arm) and 0x60 (search arm) so the
        // `LAST < V1` mutants' two sides differ. Kind 7 is content then
        // consecutive spaces: a tight width breaks the line at the second
        // space, using the `^ 0x20` bookkeeping the mutant changes.
        let kind = seed / 7 % 8;
        let codes: [u16; 12] = match kind {
            0 => [0x41, 0x42, 0x20, 0x43, 0x44, 0x20, 0x45, 0x46, 0, 0, 0, 0],
            1 => [0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0, 0, 0, 0],
            2 => [
                0x70, 0x72, 0x20, 0x74, 0x76, 0x20, 0x41, 0x20, 0x42, 0, 0, 0,
            ],
            3 => [0x20, 0x20, 0x41, 0x42, 0x20, 0, 0, 0, 0, 0, 0, 0],
            4 => [0x7f, 0x41, 0x20, 0x42, 0, 0, 0, 0, 0, 0, 0, 0],
            5 => [0x41, 0x01, 0x42, 0x20, 0x43, 0x01, 0x44, 0, 0, 0, 0, 0],
            6 => [0x5f, 0x41, 0x60, 0x20, 0x42, 0, 0, 0, 0, 0, 0, 0],
            _ => [0x41, 0x20, 0x20, 0x20, 0x20, 0x20, 0, 0, 0, 0, 0, 0],
        };
        for (i, &code) in codes.iter().enumerate() {
            c.bus.write16(FIND_STR + 2 * i as u32, code);
        }
        fill(c, &mut r, FIND_DST, 0x40);
        c.g[A0][0] = sx(FIND_FONT);
        c.g[A1][0] = sx(FIND_STR);
        c.g[A2][0] = sx(FIND_DST);
        // Distinct from `a2` but inside the prefilled destination, so the
        // clobber writes to a valid, observed address instead of faulting.
        c.g[S6][0] = sx(FIND_DST + 0x40);
        c.g[A3][0] = 8 + (seed % 3) as u64;
        c.f[12] = f32::to_bits(1.0);
        c.f[13] = f32::to_bits([0.0, 30.0, 500.0][(seed / 3 % 3) as usize]);
        r
    }

    /// Widths 0, tight and roomy over spaced, unspaced, tail-searched,
    /// default-glyph, below-`first`, boundary-code (0x5f/0x60) and
    /// consecutive-space strings; `max` 8..10.
    #[test]
    fn font_find_line_break_w_matches() {
        check_lohi(
            ee::s12::f_00392680__generated,
            font_find_line_break_w,
            200,
            &|c, seed| {
                find_line_break_setup(c, seed);
            },
        );
    }

    #[test]
    fn font_find_line_break_w_events() {
        driven!(gen_find, ee::s12::f_00392680__generated);
        driven!(hand_find, font_find_line_break_w);
        check_events(gen_find, hand_find, 60, &|c, seed| {
            find_line_break_setup(c, seed / 3);
            schedule(
                c,
                &mut Rng(0x39_2680 ^ (seed as u64) << 8),
                seed,
                0x0039_2680,
            )
        });
    }

    /// Gates for the two font draw wrappers: measure and draw are faked
    /// (override slots); the tests check the FPU setup and forwarding.
    pub(crate) const FONT_DRAW_CTRL: u32 = DATA + 0x32_0000;

    pub(crate) const FONT_DRAW_MAGIC: u32 = 0x0c13_7203;

    pub(crate) static FONT_DRAW_PREV_MEASURE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static FONT_DRAW_PREV_DRAW: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn fake_measure_text_w(c: &mut Cpu) {
        if c.bus.read32(FONT_DRAW_CTRL) != FONT_DRAW_MAGIC {
            return fakes::fall_back(c, &FONT_DRAW_PREV_MEASURE, ee::s12::f_003921f0__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn fake_draw_text_w(c: &mut Cpu) {
        if c.bus.read32(FONT_DRAW_CTRL) != FONT_DRAW_MAGIC {
            return fakes::fall_back(c, &FONT_DRAW_PREV_DRAW, ee::s12::f_00391e30__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn font_draw_hook() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_003921F0),
            fake_measure_text_w,
            &FONT_DRAW_PREV_MEASURE,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391E30),
            fake_draw_text_w,
            &FONT_DRAW_PREV_DRAW,
        );
    }

    /// Font object with measure/draw sizes, edge-float register inputs.
    pub(crate) fn font_draw_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = machine(c, seed, entry as u64, 0);
        c.bus.write32(FONT_DRAW_CTRL, FONT_DRAW_MAGIC);
        const OBJ: u32 = DATA + 0x1234;
        const STR: u32 = DATA + 0x20_0000;
        fill(c, &mut r, OBJ, 0x40);
        fill(c, &mut r, STR, 0x40);
        c.g[A0][0] = sx(OBJ);
        c.g[A1][0] = sx(STR);
        c.f[12] = r.next_u32();
        c.f[13] = r.next_u32();
        c.pc = entry;
    }

    #[test]
    fn font_draw_text_centered_w_matches() {
        font_draw_hook();
        check(
            ee::s13::f_00392980__generated,
            font_draw_text_centered_w,
            120,
            &|c, seed| {
                font_draw_setup(c, seed, 0x0039_2980);
            },
        );
    }

    #[test]
    fn font_draw_text_right_w_matches() {
        font_draw_hook();
        check(
            ee::s13::f_00392ad0__generated,
            font_draw_text_right_w,
            120,
            &|c, seed| {
                font_draw_setup(c, seed, 0x0039_29ad0);
            },
        );
    }

    /// Fixture for [`font_free`]: the free flag from `seed % 2`, the
    /// teardown sign from `seed / 2 % 2`; the heap free is faked (override
    /// slot), the teardown hook is the miss-hook fake.
    pub(crate) const FONT_FREE_OBJ: u32 = DATA + 0x1234;

    pub(crate) const FONT_FREE_TABLE: u32 = DATA + 0x20_0000;

    pub(crate) fn font_free_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = machine(c, seed, 0x39_19e8, 1);
        c.pc = 0x0039_19e8;
        c.g[A0][0] = sx(FONT_FREE_OBJ);
        fill(c, &mut r, FONT_FREE_OBJ + 8, 0x18);
        // After the fill: it spans +8..+0x68 and used to clobber both
        // words below, so the `< 0` boundary (flags 0) never occurred.
        c.bus.write32(FONT_FREE_OBJ + 4, (seed % 2) as u32 * 0x2410);
        c.bus.write32(
            FONT_FREE_OBJ + 0x60,
            if seed / 2 % 2 == 0 { 0 } else { 0x8000_0001 },
        );
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP + 0x2a90, FONT_FREE_TABLE);
        c.bus
            .write32(FONT_FREE_TABLE + 0x10d8, FONT_FREE_TABLE + 0x100);
        c.bus.write16(FONT_FREE_TABLE + 0x100 + 0x190, 0x30);
        c.bus.write32(FONT_FREE_TABLE + 0x100 + 0x194, FAKE);
        r
    }

    pub(crate) static FONT_FREE_PREV: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn fake_ea_mem_free(c: &mut Cpu) {
        if c.bus.read32(FONT_DRAW_CTRL) != FONT_DRAW_MAGIC {
            return fakes::fall_back(c, &FONT_FREE_PREV, ee::s10::f_00317e98__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// The free arm, the skip arm, and the negative/hook teardown arms;
    /// hook unwinds cover the unwind return.
    #[test]
    fn font_free_matches() {
        font_draw_hook();
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            fake_ea_mem_free,
            &FONT_FREE_PREV,
        );
        covered(
            ee::s12::f_003919e8__generated,
            font_free,
            200,
            &|c, seed| {
                c.bus.write32(FONT_DRAW_CTRL, FONT_DRAW_MAGIC);
                font_free_setup(c, seed);
            },
        );
    }

    /// Gates for the varargs forwarders: the draw targets are faked
    /// (override slots); the tests check the spill layout and forwarding.
    pub(crate) static VARARGS_PREV_F: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static VARARGS_PREV_CENTERED: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn fake_draw_text_f(c: &mut Cpu) {
        if c.bus.read32(FONT_DRAW_CTRL) != FONT_DRAW_MAGIC {
            return fakes::fall_back(c, &VARARGS_PREV_F, ee::s12::f_00391c48__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn fake_draw_text_centered_f(c: &mut Cpu) {
        if c.bus.read32(FONT_DRAW_CTRL) != FONT_DRAW_MAGIC {
            return fakes::fall_back(c, &VARARGS_PREV_CENTERED, ee::s13::f_003929f8__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn varargs_hook() {
        font_draw_hook();
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391C48),
            fake_draw_text_f,
            &VARARGS_PREV_F,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003929F8),
            fake_draw_text_centered_f,
            &VARARGS_PREV_CENTERED,
        );
    }

    #[test]
    fn font_draw_text_varargs_2_matches() {
        varargs_hook();
        check(
            ee::s05::f_0021d160__generated,
            font_draw_text_varargs_2,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x21_d160, 1);
                c.bus.write32(FONT_DRAW_CTRL, FONT_DRAW_MAGIC);
                c.g[A0][0] = sx(DATA + 0x1234);
                c.g[A1][0] = sx(DATA + 0x20_0000);
                c.pc = 0x0021_d160;
            },
        );
    }

    #[test]
    fn font_draw_text_centered_varargs_2_matches() {
        varargs_hook();
        check(
            ee::s05::f_002223e8__generated,
            font_draw_text_centered_varargs_2,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x22_23e8, 1);
                c.bus.write32(FONT_DRAW_CTRL, FONT_DRAW_MAGIC);
                c.g[A0][0] = sx(DATA + 0x1234);
                c.g[A1][0] = sx(DATA + 0x20_0000);
                c.pc = 0x0022_23e8;
            },
        );
    }

    /// Gate word and log cell of the operator_delete fake (unique to this module).
    pub(crate) const OC1072_GATE: u32 = DATA + 0x4000;

    pub(crate) const OC1072_LOG: u32 = DATA + 0x4004;

    pub(crate) const OC1072_MAGIC: u32 = 0x1072_0396;

    pub(crate) static OC1072_DELETE_PREV: AtomicUsize = AtomicUsize::new(0);

    /// Gated stand-in for `operator_delete` (0x317e50): when the gate is
    /// set it logs its argument and returns; otherwise the real body runs.
    pub(crate) fn oc1072_delete_fake(c: &mut Cpu) {
        if c.bus.read32(OC1072_GATE) != OC1072_MAGIC {
            return fakes::fall_back(c, &OC1072_DELETE_PREV, ee::s10::f_00317e50__generated);
        }
        let p = c.g[A0][0] as u32;
        c.bus.write32(OC1072_LOG, p);
        abi::ret(c);
    }

    #[test]
    fn fx_trail_set_flag_a4_matches() {
        check(
            ssx3_ee::s13::f_00392de8__generated,
            fx_trail_set_flag_a4,
            200,
            &|c, seed| {
                let mut r = Rng(0x392d_e800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(r.next_u32());
                c.bus.write32(DATA + 0xa4, r.next_u32());
            },
        );
    }

    #[test]
    fn vfx_component_base_dtor_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1072_delete_fake,
            &OC1072_DELETE_PREV,
        );
        check(
            ssx3_ee::s13::f_00396160__generated,
            vfx_component_base_dtor,
            200,
            &|c, seed| {
                let mut r = Rng(0x3961_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                // Bit 0 of the flags picks the delete path, the rest is junk.
                let flags = (r.next_u32() & !1) | (seed & 1);
                c.g[A1][0] = sx(flags);
                c.bus.write32(OC1072_GATE, OC1072_MAGIC);
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    /// Lighting list fixture: the gm record at DATA, its render-state record
    /// at DATA+0x1000, the source table at gm+0x6bb0 and the list pointer at
    /// gm+0x5a00 (destination in the 0x3000_0000 window).
    pub(crate) fn emit_lighting_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x396b_40 ^ (seed as u64) << 9);
        scramble(c, &mut r);
        c.pc = 0x0039_6b40;
        let gm = DATA;
        c.bus.write32(gm + 0xe84, DATA + 0x1000);
        c.bus
            .write32(gm + 0x5a00, DATA + 0x9000 + (seed % 4) * 0x100);
        fill(c, &mut r, DATA + 0x1000, 0x10);
        fill(c, &mut r, gm + 0x6bb0, 0x40);
        fill(c, &mut r, DATA + 0x9000, 0x80);
        c.g[A0][0] = sx(gm);
    }

    #[test]
    fn ps2_graphics_man_emit_lighting_packet_matches() {
        check(
            ee::s13::f_00396b40__generated,
            ps2_graphics_man_emit_lighting_packet,
            200,
            &|c, seed| emit_lighting_setup(c, seed),
        );
    }

    /// `GraphicsMan_SetDefaultDisplayMode` (0x0039_3fa8) against the generated body.
    #[test]
    fn graphics_man_set_default_display_mode_00393fa8_matches() {
        check(
            ee::s13::f_00393fa8__generated,
            graphics_man_set_default_display_mode_00393fa8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x393fa8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_IsProgressive` (0x0039_3fb0) against the generated body.
    #[test]
    fn graphics_man_is_progressive_00393fb0_matches() {
        check(
            ee::s13::f_00393fb0__generated,
            graphics_man_is_progressive_00393fb0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x393fb0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v16` (0x0039_4008) against the generated body.
    #[test]
    fn graphics_man_v16_00394008_matches() {
        check(
            ee::s13::f_00394008__generated,
            graphics_man_v16_00394008,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394008, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// An edge-case 64-bit register value (0, 1, -1, small, random).
    #[allow(dead_code)]
    pub(crate) fn leaf_val(r: &mut Rng) -> u64 {
        match r.next_u32() % 8 {
            0 => 0,
            1 => 1,
            2 => u64::MAX,
            3 => (r.next_u32() % 64) as u64,
            4 => sx(r.next_u32()),
            5 => r.next_u32() as u64,
            6 => 0x8000_0000,
            _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        }
    }

    /// Fill `size` bytes at `start` with zero / small / random words, or, when
    /// `chase` is set, with pointers (16-byte aligned) into `ptr_lo..ptr_lo + 0x200`.
    #[allow(dead_code)]
    pub(crate) fn leaf_fill(
        c: &mut Cpu,
        r: &mut Rng,
        start: u32,
        size: u32,
        chase: bool,
        ptr_lo: u32,
    ) {
        for i in 0..size / 4 {
            let w = if chase {
                ptr_lo + 16 * (r.next_u32() % 32)
            } else {
                match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.next_u32() % 16,
                    _ => r.next_u32(),
                }
            };
            c.bus.write32(start + 4 * i, w);
        }
    }

    /// `GraphicsMan_TryAcquireFrame` (0x0039_4010) against the generated body.
    #[test]
    fn graphics_man_try_acquire_frame_00394010_matches() {
        check(
            ee::s13::f_00394010__generated,
            graphics_man_try_acquire_frame_00394010,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394010, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_IsIdle` (0x0039_4018) against the generated body.
    #[test]
    fn graphics_man_is_idle_00394018_matches() {
        check(
            ee::s13::f_00394018__generated,
            graphics_man_is_idle_00394018,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394018, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_ReleaseTextureRef` (0x0039_42c0) against the generated body.
    #[test]
    fn graphics_man_release_texture_ref_003942c0_matches() {
        check(
            ee::s13::f_003942c0__generated,
            graphics_man_release_texture_ref_003942c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3942c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_DrawTransition` (0x0039_42c8) against the generated body.
    #[test]
    fn graphics_man_draw_transition_003942c8_matches() {
        check(
            ee::s13::f_003942c8__generated,
            graphics_man_draw_transition_003942c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3942c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v62` (0x0039_42d0) against the generated body.
    #[test]
    fn graphics_man_v62_003942d0_matches() {
        check(
            ee::s13::f_003942d0__generated,
            graphics_man_v62_003942d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3942d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_GetColorMask` (0x0039_42d8) against the generated body.
    #[test]
    fn graphics_man_get_color_mask_003942d8_matches() {
        check(
            ee::s13::f_003942d8__generated,
            graphics_man_get_color_mask_003942d8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3942d8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v65` (0x0039_42e0) against the generated body.
    #[test]
    fn graphics_man_v65_003942e0_matches() {
        check(
            ee::s13::f_003942e0__generated,
            graphics_man_v65_003942e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3942e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_ClearLights` (0x0039_45b8) against the generated body.
    #[test]
    fn graphics_man_clear_lights_003945b8_matches() {
        check(
            ee::s13::f_003945b8__generated,
            graphics_man_clear_lights_003945b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_AddLight` (0x0039_45c0) against the generated body.
    #[test]
    fn graphics_man_add_light_003945c0_matches() {
        check(
            ee::s13::f_003945c0__generated,
            graphics_man_add_light_003945c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v74` (0x0039_45c8) against the generated body.
    #[test]
    fn graphics_man_v74_003945c8_matches() {
        check(
            ee::s13::f_003945c8__generated,
            graphics_man_v74_003945c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v88` (0x0039_45d0) against the generated body.
    #[test]
    fn graphics_man_v88_003945d0_matches() {
        check(
            ee::s13::f_003945d0__generated,
            graphics_man_v88_003945d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_DrawShadowVolume` (0x0039_45d8) against the generated body.
    #[test]
    fn graphics_man_draw_shadow_volume_003945d8_matches() {
        check(
            ee::s13::f_003945d8__generated,
            graphics_man_draw_shadow_volume_003945d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_DrawBoardWake` (0x0039_45e0) against the generated body.
    #[test]
    fn graphics_man_draw_board_wake_003945e0_matches() {
        check(
            ee::s13::f_003945e0__generated,
            graphics_man_draw_board_wake_003945e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v98` (0x0039_45e8) against the generated body.
    #[test]
    fn graphics_man_v98_003945e8_matches() {
        check(
            ee::s13::f_003945e8__generated,
            graphics_man_v98_003945e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_DrawSkinnedModelShadow` (0x0039_4610) against the generated body.
    #[test]
    fn graphics_man_draw_skinned_model_shadow_00394610_matches() {
        check(
            ee::s13::f_00394610__generated,
            graphics_man_draw_skinned_model_shadow_00394610,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394610, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v102` (0x0039_4618) against the generated body.
    #[test]
    fn graphics_man_v102_00394618_matches() {
        check(
            ee::s13::f_00394618__generated,
            graphics_man_v102_00394618,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394618, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v103` (0x0039_4620) against the generated body.
    #[test]
    fn graphics_man_v103_00394620_matches() {
        check(
            ee::s13::f_00394620__generated,
            graphics_man_v103_00394620,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394620, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v104` (0x0039_4628) against the generated body.
    #[test]
    fn graphics_man_v104_00394628_matches() {
        check(
            ee::s13::f_00394628__generated,
            graphics_man_v104_00394628,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394628, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v105` (0x0039_4630) against the generated body.
    #[test]
    fn graphics_man_v105_00394630_matches() {
        check(
            ee::s13::f_00394630__generated,
            graphics_man_v105_00394630,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394630, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v106` (0x0039_4638) against the generated body.
    #[test]
    fn graphics_man_v106_00394638_matches() {
        check(
            ee::s13::f_00394638__generated,
            graphics_man_v106_00394638,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394638, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v107` (0x0039_4640) against the generated body.
    #[test]
    fn graphics_man_v107_00394640_matches() {
        check(
            ee::s13::f_00394640__generated,
            graphics_man_v107_00394640,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394640, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v108` (0x0039_4648) against the generated body.
    #[test]
    fn graphics_man_v108_00394648_matches() {
        check(
            ee::s13::f_00394648__generated,
            graphics_man_v108_00394648,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394648, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v109` (0x0039_4650) against the generated body.
    #[test]
    fn graphics_man_v109_00394650_matches() {
        check(
            ee::s13::f_00394650__generated,
            graphics_man_v109_00394650,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394650, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v110` (0x0039_4658) against the generated body.
    #[test]
    fn graphics_man_v110_00394658_matches() {
        check(
            ee::s13::f_00394658__generated,
            graphics_man_v110_00394658,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394658, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v111` (0x0039_4660) against the generated body.
    #[test]
    fn graphics_man_v111_00394660_matches() {
        check(
            ee::s13::f_00394660__generated,
            graphics_man_v111_00394660,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394660, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v112` (0x0039_4668) against the generated body.
    #[test]
    fn graphics_man_v112_00394668_matches() {
        check(
            ee::s13::f_00394668__generated,
            graphics_man_v112_00394668,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394668, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v113` (0x0039_4670) against the generated body.
    #[test]
    fn graphics_man_v113_00394670_matches() {
        check(
            ee::s13::f_00394670__generated,
            graphics_man_v113_00394670,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394670, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_GetFlipCount` (0x0039_4678) against the generated body.
    #[test]
    fn graphics_man_get_flip_count_00394678_matches() {
        check(
            ee::s13::f_00394678__generated,
            graphics_man_get_flip_count_00394678,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394678, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_UpdateLensEffects` (0x0039_4680) against the generated body.
    #[test]
    fn graphics_man_update_lens_effects_00394680_matches() {
        check(
            ee::s13::f_00394680__generated,
            graphics_man_update_lens_effects_00394680,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394680, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_AddView` (0x0039_4688) against the generated body.
    #[test]
    fn graphics_man_add_view_00394688_matches() {
        check(
            ee::s13::f_00394688__generated,
            graphics_man_add_view_00394688,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394688, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_RemoveInterruptHandlers` (0x0039_46e8) against the generated body.
    #[test]
    fn graphics_man_remove_interrupt_handlers_003946e8_matches() {
        check(
            ee::s13::f_003946e8__generated,
            graphics_man_remove_interrupt_handlers_003946e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3946e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GridMeshBase_SetScale` (0x0039_4b68) against the generated body.
    #[test]
    fn grid_mesh_base_set_scale_00394b68_matches() {
        check(
            ee::s13::f_00394b68__generated,
            grid_mesh_base_set_scale_00394b68,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394b68, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BezierManBase_v04` (0x0039_4b78) against the generated body.
    #[test]
    fn bezier_man_base_v04_00394b78_matches() {
        check(
            ee::s13::f_00394b78__generated,
            bezier_man_base_v04_00394b78,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394b78, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BezierManBase_v05` (0x0039_4b80) against the generated body.
    #[test]
    fn bezier_man_base_v05_00394b80_matches() {
        check(
            ee::s13::f_00394b80__generated,
            bezier_man_base_v05_00394b80,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394b80, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BezierManBase_v19` (0x0039_4b88) against the generated body.
    #[test]
    fn bezier_man_base_v19_00394b88_matches() {
        check(
            ee::s13::f_00394b88__generated,
            bezier_man_base_v19_00394b88,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394b88, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BezierManBase_v20` (0x0039_4b90) against the generated body.
    #[test]
    fn bezier_man_base_v20_00394b90_matches() {
        check(
            ee::s13::f_00394b90__generated,
            bezier_man_base_v20_00394b90,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394b90, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_SetLodNearDist` (0x0039_4bc0) against the generated body.
    #[test]
    fn ps2_bezier_man_set_lod_near_dist_00394bc0_matches() {
        check(
            ee::s13::f_00394bc0__generated,
            ps2_bezier_man_set_lod_near_dist_00394bc0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394bc0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_SetLodFarDist` (0x0039_4bc8) against the generated body.
    #[test]
    fn ps2_bezier_man_set_lod_far_dist_00394bc8_matches() {
        check(
            ee::s13::f_00394bc8__generated,
            ps2_bezier_man_set_lod_far_dist_00394bc8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394bc8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_GetLodNearDist` (0x0039_4bd0) against the generated body.
    #[test]
    fn ps2_bezier_man_get_lod_near_dist_00394bd0_matches() {
        check(
            ee::s13::f_00394bd0__generated,
            ps2_bezier_man_get_lod_near_dist_00394bd0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394bd0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_GetLodFarDist` (0x0039_4bd8) against the generated body.
    #[test]
    fn ps2_bezier_man_get_lod_far_dist_00394bd8_matches() {
        check(
            ee::s13::f_00394bd8__generated,
            ps2_bezier_man_get_lod_far_dist_00394bd8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394bd8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_v10` (0x0039_4be0) against the generated body.
    #[test]
    fn ps2_bezier_man_v10_00394be0_matches() {
        check(
            ee::s13::f_00394be0__generated,
            ps2_bezier_man_v10_00394be0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394be0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_v11` (0x0039_4be8) against the generated body.
    #[test]
    fn ps2_bezier_man_v11_00394be8_matches() {
        check(
            ee::s13::f_00394be8__generated,
            ps2_bezier_man_v11_00394be8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394be8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_v12` (0x0039_4bf0) against the generated body.
    #[test]
    fn ps2_bezier_man_v12_00394bf0_matches() {
        check(
            ee::s13::f_00394bf0__generated,
            ps2_bezier_man_v12_00394bf0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394bf0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_v13` (0x0039_4bf8) against the generated body.
    #[test]
    fn ps2_bezier_man_v13_00394bf8_matches() {
        check(
            ee::s13::f_00394bf8__generated,
            ps2_bezier_man_v13_00394bf8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394bf8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_v14` (0x0039_4c00) against the generated body.
    #[test]
    fn ps2_bezier_man_v14_00394c00_matches() {
        check(
            ee::s13::f_00394c00__generated,
            ps2_bezier_man_v14_00394c00,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394c00, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_v15` (0x0039_4c08) against the generated body.
    #[test]
    fn ps2_bezier_man_v15_00394c08_matches() {
        check(
            ee::s13::f_00394c08__generated,
            ps2_bezier_man_v15_00394c08,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394c08, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_v16` (0x0039_4c10) against the generated body.
    #[test]
    fn ps2_bezier_man_v16_00394c10_matches() {
        check(
            ee::s13::f_00394c10__generated,
            ps2_bezier_man_v16_00394c10,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394c10, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `VfxTube_Render` (0x0039_68c8) against the generated body.
    #[test]
    fn vfx_tube_render_003968c8_matches() {
        check(
            ee::s13::f_003968c8__generated,
            vfx_tube_render_003968c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3968c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PathArrow_Update` (0x0039_6918) against the generated body.
    #[test]
    fn path_arrow_update_00396918_matches() {
        check(
            ee::s13::f_00396918__generated,
            path_arrow_update_00396918,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x396918, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PathArrow_Render` (0x0039_6920) against the generated body.
    #[test]
    fn path_arrow_render_00396920_matches() {
        check(
            ee::s13::f_00396920__generated,
            path_arrow_render_00396920,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x396920, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DetailSystem_v05` (0x0039_6958) against the generated body.
    #[test]
    fn detail_system_v05_00396958_matches() {
        check(
            ee::s13::f_00396958__generated,
            detail_system_v05_00396958,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x396958, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
