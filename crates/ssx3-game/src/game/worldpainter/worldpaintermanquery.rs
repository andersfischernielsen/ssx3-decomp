//! `src/worldpainter/worldpaintermanquery.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
pub(crate) use crate::game::ui::uiengine::WPQ_ALLOC;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
use crate::game::worldpainter::wpigd::replay_frame_call;
use crate::game::worldpainter::wpigd::REPLAY_FRAME_READ_SLOT;
use crate::game::worldpainter::wpigd::REPLAY_FRAME_WRITE_SLOT;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use Oc1040Comp::Bits;
use Oc1040Comp::Pool;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002c_1460,
        world_painter_query_base_dtor,
        "WorldPainterQuery_BaseDtor",
    ),
    (0x002c_16a8, quad_tree_node_set_leaf, "QuadTreeNode_SetLeaf"),
    (
        0x002c_16b8,
        quad_tree_node_set_children,
        "QuadTreeNode_SetChildren",
    ),
    (
        0x002c_17c0,
        quad_tree_init_free_list,
        "QuadTree_InitFreeList",
    ),
    (
        0x002c_18e8,
        quad_tree_free_children,
        "QuadTree_FreeChildren",
    ),
    (
        0x002c_1560,
        twpigd_sun_get_elevation,
        "tWPIGD_Sun_GetElevation",
    ),
    (0x002c_1568, twpigd_sun_get_azimuth, "tWPIGD_Sun_GetAzimuth"),
    (
        0x002c_0a58,
        world_painter_query_create,
        "WorldPainterQuery_Create",
    ),
    (
        0x002c_0b70,
        world_painter_query_static_init_destroy,
        "WorldPainterQuery_StaticInitDestroy",
    ),
    (
        0x002c_1688,
        world_painter_query_global_ctors,
        "WorldPainterQuery_GlobalCtors",
    ),
    (
        0x002b_c500,
        t_wpigd_base_get_music_trigger08_002bc500,
        "tWPIGD_Base_GetMusicTrigger08",
    ),
    (
        0x002b_c508,
        t_wpigd_base_get_music_trigger10_002bc508,
        "tWPIGD_Base_GetMusicTrigger10",
    ),
    (
        0x002b_c510,
        t_wpigd_base_get_mix08_002bc510,
        "tWPIGD_Base_GetMix08",
    ),
    (
        0x002b_c518,
        t_wpigd_base_get_ambience08_002bc518,
        "tWPIGD_Base_GetAmbience08",
    ),
    (
        0x002b_c520,
        t_wpigd_base_get_speech08_002bc520,
        "tWPIGD_Base_GetSpeech08",
    ),
    (
        0x002b_c528,
        t_wpigd_base_get_camera08_002bc528,
        "tWPIGD_Base_GetCamera08",
    ),
    (
        0x002b_c530,
        t_wpigd_base_get_fog08_002bc530,
        "tWPIGD_Base_GetFog08",
    ),
    (
        0x002b_c538,
        t_wpigd_base_get_fog10_002bc538,
        "tWPIGD_Base_GetFog10",
    ),
    (
        0x002b_c540,
        t_wpigd_base_get_fog18_002bc540,
        "tWPIGD_Base_GetFog18",
    ),
    (
        0x002b_c548,
        t_wpigd_base_get_fog20_002bc548,
        "tWPIGD_Base_GetFog20",
    ),
    (
        0x002b_c550,
        t_wpigd_base_get_fog28_002bc550,
        "tWPIGD_Base_GetFog28",
    ),
    (
        0x002b_c558,
        t_wpigd_base_get_fog30_002bc558,
        "tWPIGD_Base_GetFog30",
    ),
    (
        0x002b_c560,
        t_wpigd_base_get_light_glow08_002bc560,
        "tWPIGD_Base_GetLightGlow08",
    ),
    (
        0x002b_c568,
        t_wpigd_base_get_light_glow10_002bc568,
        "tWPIGD_Base_GetLightGlow10",
    ),
    (
        0x002b_c570,
        t_wpigd_base_get_light_glow18_002bc570,
        "tWPIGD_Base_GetLightGlow18",
    ),
    (
        0x002b_c578,
        t_wpigd_base_get_light_glow20_002bc578,
        "tWPIGD_Base_GetLightGlow20",
    ),
    (
        0x002b_c580,
        t_wpigd_base_get_light_glow28_002bc580,
        "tWPIGD_Base_GetLightGlow28",
    ),
    (
        0x002b_c588,
        t_wpigd_base_get_light_glow30_002bc588,
        "tWPIGD_Base_GetLightGlow30",
    ),
    (
        0x002b_c590,
        t_wpigd_base_get_light_glow38_002bc590,
        "tWPIGD_Base_GetLightGlow38",
    ),
    (
        0x002b_c598,
        t_wpigd_base_get_screen_tint_red_002bc598,
        "tWPIGD_Base_GetScreenTintRed",
    ),
    (
        0x002b_c5a0,
        t_wpigd_base_get_screen_tint_green_002bc5a0,
        "tWPIGD_Base_GetScreenTintGreen",
    ),
    (
        0x002b_c5a8,
        t_wpigd_base_get_screen_tint_blue_002bc5a8,
        "tWPIGD_Base_GetScreenTintBlue",
    ),
    (
        0x002b_c5b0,
        t_wpigd_base_get_screen_fill_red_002bc5b0,
        "tWPIGD_Base_GetScreenFillRed",
    ),
    (
        0x002b_c5b8,
        t_wpigd_base_get_screen_fill_green_002bc5b8,
        "tWPIGD_Base_GetScreenFillGreen",
    ),
    (
        0x002b_c5c0,
        t_wpigd_base_get_screen_fill_blue_002bc5c0,
        "tWPIGD_Base_GetScreenFillBlue",
    ),
    (
        0x002b_c5c8,
        t_wpigd_base_get_sky_box08_002bc5c8,
        "tWPIGD_Base_GetSkyBox08",
    ),
    (
        0x002b_c5d0,
        t_wpigd_base_get_sun_elevation_002bc5d0,
        "tWPIGD_Base_GetSunElevation",
    ),
    (
        0x002b_c5d8,
        t_wpigd_base_get_sun_azimuth_002bc5d8,
        "tWPIGD_Base_GetSunAzimuth",
    ),
    (
        0x002b_c5e0,
        t_wpigd_base_get_sun_red_002bc5e0,
        "tWPIGD_Base_GetSunRed",
    ),
    (
        0x002b_c5e8,
        t_wpigd_base_get_sun_green_002bc5e8,
        "tWPIGD_Base_GetSunGreen",
    ),
    (
        0x002b_c5f0,
        t_wpigd_base_get_sun_blue_002bc5f0,
        "tWPIGD_Base_GetSunBlue",
    ),
    (
        0x002b_c5f8,
        t_wpigd_base_get_sun_intensity_002bc5f8,
        "tWPIGD_Base_GetSunIntensity",
    ),
    (
        0x002b_c600,
        t_wpigd_base_get_sun_texture_002bc600,
        "tWPIGD_Base_GetSunTexture",
    ),
    (
        0x002b_c608,
        t_wpigd_base_get_sun_flare_intensity_002bc608,
        "tWPIGD_Base_GetSunFlareIntensity",
    ),
    (
        0x002b_c610,
        t_wpigd_base_get_sun_size_002bc610,
        "tWPIGD_Base_GetSunSize",
    ),
    (
        0x002b_c618,
        t_wpigd_base_get_surface08_002bc618,
        "tWPIGD_Base_GetSurface08",
    ),
    (
        0x002b_c620,
        t_wpigd_base_get_irradiance_name0_002bc620,
        "tWPIGD_Base_GetIrradianceName0",
    ),
    (
        0x002b_c628,
        t_wpigd_base_get_irradiance_name1_002bc628,
        "tWPIGD_Base_GetIrradianceName1",
    ),
    (
        0x002b_c630,
        t_wpigd_base_get_irradiance_name2_002bc630,
        "tWPIGD_Base_GetIrradianceName2",
    ),
    (
        0x002b_c638,
        t_wpigd_base_get_irradiance_name3_002bc638,
        "tWPIGD_Base_GetIrradianceName3",
    ),
    (
        0x002b_c640,
        t_wpigd_base_get_lighting48_002bc640,
        "tWPIGD_Base_GetLighting48",
    ),
    (
        0x002b_c648,
        t_wpigd_base_get_lighting50_002bc648,
        "tWPIGD_Base_GetLighting50",
    ),
    (
        0x002b_c650,
        t_wpigd_base_get_weather08_002bc650,
        "tWPIGD_Base_GetWeather08",
    ),
    (
        0x002b_c658,
        t_wpigd_base_get_weather10_002bc658,
        "tWPIGD_Base_GetWeather10",
    ),
    (
        0x002b_c660,
        t_wpigd_base_get_weather18_002bc660,
        "tWPIGD_Base_GetWeather18",
    ),
    (
        0x002b_c668,
        t_wpigd_base_get_weather20_002bc668,
        "tWPIGD_Base_GetWeather20",
    ),
    (
        0x002b_c670,
        t_wpigd_base_get_weather28_002bc670,
        "tWPIGD_Base_GetWeather28",
    ),
    (
        0x002b_c678,
        t_wpigd_base_get_weather30_002bc678,
        "tWPIGD_Base_GetWeather30",
    ),
    (
        0x002b_c680,
        t_wpigd_base_get_weather38_002bc680,
        "tWPIGD_Base_GetWeather38",
    ),
    (
        0x002b_c688,
        t_wpigd_base_get_weather40_002bc688,
        "tWPIGD_Base_GetWeather40",
    ),
    (
        0x002b_c690,
        t_wpigd_base_get_weather48_002bc690,
        "tWPIGD_Base_GetWeather48",
    ),
    (
        0x002b_c698,
        t_wpigd_base_get_weather50_002bc698,
        "tWPIGD_Base_GetWeather50",
    ),
    (
        0x002b_c6a0,
        t_wpigd_base_get_weather58_002bc6a0,
        "tWPIGD_Base_GetWeather58",
    ),
    (
        0x002b_c6a8,
        t_wpigd_base_get_weather60_002bc6a8,
        "tWPIGD_Base_GetWeather60",
    ),
    (
        0x002b_c6b0,
        t_wpigd_base_get_weather68_002bc6b0,
        "tWPIGD_Base_GetWeather68",
    ),
    (
        0x002b_c6b8,
        t_wpigd_base_get_weather70_002bc6b8,
        "tWPIGD_Base_GetWeather70",
    ),
    (
        0x002b_c6c0,
        t_wpigd_base_get_weather78_002bc6c0,
        "tWPIGD_Base_GetWeather78",
    ),
    (
        0x002b_c6c8,
        t_wpigd_base_get_weather80_002bc6c8,
        "tWPIGD_Base_GetWeather80",
    ),
    (
        0x002b_c6d0,
        t_wpigd_base_get_weather88_002bc6d0,
        "tWPIGD_Base_GetWeather88",
    ),
    (
        0x002b_c6d8,
        t_wpigd_base_get_weather90_002bc6d8,
        "tWPIGD_Base_GetWeather90",
    ),
    (
        0x002b_c6e0,
        t_wpigd_base_get_weather98_002bc6e0,
        "tWPIGD_Base_GetWeather98",
    ),
    (
        0x002b_c6e8,
        t_wpigd_base_get_danger08_002bc6e8,
        "tWPIGD_Base_GetDanger08",
    ),
    (
        0x002b_c6f0,
        t_wpigd_base_get_danger10_002bc6f0,
        "tWPIGD_Base_GetDanger10",
    ),
    (0x002b_cd68, wp_igd_ambience_blend, "tWPIGD_Ambience_Blend"),
    (0x002b_ce08, wp_igd_speech_blend, "tWPIGD_Speech_Blend"),
    (0x002b_d968, wp_igd_danger_blend, "tWPIGD_Danger_Blend"),
    (
        0x002b_da28,
        t_wpigd_music_trigger_v67_002bda28,
        "tWPIGD_MusicTrigger_v67",
    ),
    (
        0x002b_da30,
        t_wpigd_ambience_v67_002bda30,
        "tWPIGD_Ambience_v67",
    ),
    (
        0x002b_da38,
        t_wpigd_camera_v67_002bda38,
        "tWPIGD_Camera_v67",
    ),
    (
        0x002b_da40,
        t_wpigd_danger_v67_002bda40,
        "tWPIGD_Danger_v67",
    ),
    (0x002b_da48, t_wpigd_fog_v67_002bda48, "tWPIGD_Fog_v67"),
    (
        0x002b_da50,
        t_wpigd_light_glow_v67_002bda50,
        "tWPIGD_LightGlow_v67",
    ),
    (
        0x002b_da58,
        t_wpigd_lighting_v67_002bda58,
        "tWPIGD_Lighting_v67",
    ),
    (0x002b_da60, t_wpigd_mix_v67_002bda60, "tWPIGD_Mix_v67"),
    (
        0x002b_da68,
        t_wpigd_screen_tint_v67_002bda68,
        "tWPIGD_ScreenTint_v67",
    ),
    (
        0x002b_da70,
        t_wpigd_sky_box_v67_002bda70,
        "tWPIGD_SkyBox_v67",
    ),
    (
        0x002b_da78,
        t_wpigd_speech_v67_002bda78,
        "tWPIGD_Speech_v67",
    ),
    (0x002b_da80, t_wpigd_sun_v67_002bda80, "tWPIGD_Sun_v67"),
    (
        0x002b_da88,
        t_wpigd_surface_v67_002bda88,
        "tWPIGD_Surface_v67",
    ),
    (
        0x002b_da90,
        t_wpigd_weather_v67_002bda90,
        "tWPIGD_Weather_v67",
    ),
    (
        0x002b_dac8,
        t_wpigd_mix_is_equal_002bdac8,
        "tWPIGD_Mix_IsEqual",
    ),
    (
        0x002b_dae0,
        t_wpigd_ambience_is_equal_002bdae0,
        "tWPIGD_Ambience_IsEqual",
    ),
    (
        0x002b_daf8,
        t_wpigd_speech_is_equal_002bdaf8,
        "tWPIGD_Speech_IsEqual",
    ),
    (
        0x002b_e0b0,
        t_wpigd_music_trigger_set_defaults_002be0b0,
        "tWPIGD_MusicTrigger_SetDefaults",
    ),
    (
        0x002b_e0c8,
        t_wpigd_mix_set_defaults_002be0c8,
        "tWPIGD_Mix_SetDefaults",
    ),
    (
        0x002b_e0d8,
        t_wpigd_ambience_set_defaults_002be0d8,
        "tWPIGD_Ambience_SetDefaults",
    ),
    (
        0x002b_e0e8,
        t_wpigd_speech_set_defaults_002be0e8,
        "tWPIGD_Speech_SetDefaults",
    ),
    (
        0x002b_e0f8,
        t_wpigd_camera_set_defaults_002be0f8,
        "tWPIGD_Camera_SetDefaults",
    ),
    (
        0x002b_e108,
        t_wpigd_fog_set_defaults_002be108,
        "tWPIGD_Fog_SetDefaults",
    ),
    (
        0x002b_e140,
        t_wpigd_light_glow_set_defaults_002be140,
        "tWPIGD_LightGlow_SetDefaults",
    ),
    (
        0x002b_e170,
        t_wpigd_screen_tint_set_defaults_002be170,
        "tWPIGD_ScreenTint_SetDefaults",
    ),
    (
        0x002b_e198,
        t_wpigd_sky_box_set_defaults_002be198,
        "tWPIGD_SkyBox_SetDefaults",
    ),
    (
        0x002b_e1a8,
        t_wpigd_sun_set_defaults_002be1a8,
        "tWPIGD_Sun_SetDefaults",
    ),
    (
        0x002b_e1e0,
        t_wpigd_surface_set_defaults_002be1e0,
        "tWPIGD_Surface_SetDefaults",
    ),
    (
        0x002b_e2c8,
        t_wpigd_danger_set_defaults_002be2c8,
        "tWPIGD_Danger_SetDefaults",
    ),
    (
        0x002b_e2d8,
        twpigd_music_trigger_load_state,
        "tWPIGD_MusicTrigger_LoadState",
    ),
    (0x002b_e518, twpigd_fog_load_state, "tWPIGD_Fog_LoadState"),
    (
        0x002b_e698,
        twpigd_light_glow_load_state,
        "tWPIGD_LightGlow_LoadState",
    ),
    (
        0x002b_e850,
        twpigd_screen_tint_load_state,
        "tWPIGD_ScreenTint_LoadState",
    ),
    (0x002b_ea38, twpigd_sun_load_state, "tWPIGD_Sun_LoadState"),
    (
        0x002b_ecc8,
        twpigd_lighting_load_state,
        "tWPIGD_Lighting_LoadState",
    ),
    (
        0x002b_ee48,
        twpigd_weather_load_state,
        "tWPIGD_Weather_LoadState",
    ),
    (
        0x002b_f2a0,
        twpigd_danger_load_state,
        "tWPIGD_Danger_LoadState",
    ),
    (
        0x002b_f340,
        twpigd_music_trigger_save_state,
        "tWPIGD_MusicTrigger_SaveState",
    ),
    (0x002b_f580, twpigd_fog_save_state, "tWPIGD_Fog_SaveState"),
    (
        0x002b_f700,
        light_glow_save_state,
        "tWPIGD_LightGlow_SaveState",
    ),
    (
        0x002b_f8b8,
        screen_tint_save_state,
        "tWPIGD_ScreenTint_SaveState",
    ),
    (0x002b_faa0, twpigd_sun_save_state, "tWPIGD_Sun_SaveState"),
    (
        0x002b_fd30,
        lighting_save_state,
        "tWPIGD_Lighting_SaveState",
    ),
    (0x002c_0308, danger_save_state, "tWPIGD_Danger_SaveState"),
    (
        0x002c_0b30,
        world_painter_query_debug_printf,
        "WorldPainterQuery_DebugPrintf",
    ),
    (0x002c_1458, t_wpigd_base_v67_002c1458, "tWPIGD_Base_v67"),
    (
        0x002c_1490,
        t_wpigd_music_trigger_get08_002c1490,
        "tWPIGD_MusicTrigger_Get08",
    ),
    (
        0x002c_1498,
        t_wpigd_music_trigger_get10_002c1498,
        "tWPIGD_MusicTrigger_Get10",
    ),
    (0x002c_14a0, t_wpigd_mix_get08_002c14a0, "tWPIGD_Mix_Get08"),
    (
        0x002c_14a8,
        t_wpigd_ambience_get08_002c14a8,
        "tWPIGD_Ambience_Get08",
    ),
    (
        0x002c_14b0,
        t_wpigd_speech_get08_002c14b0,
        "tWPIGD_Speech_Get08",
    ),
    (
        0x002c_14b8,
        t_wpigd_camera_get08_002c14b8,
        "tWPIGD_Camera_Get08",
    ),
    (0x002c_14c0, t_wpigd_fog_get08_002c14c0, "tWPIGD_Fog_Get08"),
    (0x002c_14c8, t_wpigd_fog_get10_002c14c8, "tWPIGD_Fog_Get10"),
    (0x002c_14d0, t_wpigd_fog_get18_002c14d0, "tWPIGD_Fog_Get18"),
    (0x002c_14d8, t_wpigd_fog_get20_002c14d8, "tWPIGD_Fog_Get20"),
    (0x002c_14e0, t_wpigd_fog_get28_002c14e0, "tWPIGD_Fog_Get28"),
    (0x002c_14e8, t_wpigd_fog_get30_002c14e8, "tWPIGD_Fog_Get30"),
    (
        0x002c_14f0,
        t_wpigd_light_glow_get08_002c14f0,
        "tWPIGD_LightGlow_Get08",
    ),
    (
        0x002c_14f8,
        t_wpigd_light_glow_get10_002c14f8,
        "tWPIGD_LightGlow_Get10",
    ),
    (
        0x002c_1500,
        t_wpigd_light_glow_get18_002c1500,
        "tWPIGD_LightGlow_Get18",
    ),
    (
        0x002c_1508,
        t_wpigd_light_glow_get20_002c1508,
        "tWPIGD_LightGlow_Get20",
    ),
    (
        0x002c_1510,
        t_wpigd_light_glow_get28_002c1510,
        "tWPIGD_LightGlow_Get28",
    ),
    (
        0x002c_1518,
        t_wpigd_light_glow_get30_002c1518,
        "tWPIGD_LightGlow_Get30",
    ),
    (
        0x002c_1520,
        t_wpigd_light_glow_get38_002c1520,
        "tWPIGD_LightGlow_Get38",
    ),
    (
        0x002c_1528,
        t_wpigd_screen_tint_get_tint_red_002c1528,
        "tWPIGD_ScreenTint_GetTintRed",
    ),
    (
        0x002c_1530,
        t_wpigd_screen_tint_get_tint_green_002c1530,
        "tWPIGD_ScreenTint_GetTintGreen",
    ),
    (
        0x002c_1538,
        t_wpigd_screen_tint_get_tint_blue_002c1538,
        "tWPIGD_ScreenTint_GetTintBlue",
    ),
    (
        0x002c_1540,
        t_wpigd_screen_tint_get_fill_red_002c1540,
        "tWPIGD_ScreenTint_GetFillRed",
    ),
    (
        0x002c_1548,
        t_wpigd_screen_tint_get_fill_green_002c1548,
        "tWPIGD_ScreenTint_GetFillGreen",
    ),
    (
        0x002c_1550,
        t_wpigd_screen_tint_get_fill_blue_002c1550,
        "tWPIGD_ScreenTint_GetFillBlue",
    ),
    (
        0x002c_1558,
        t_wpigd_sky_box_get08_002c1558,
        "tWPIGD_SkyBox_Get08",
    ),
    (
        0x002c_1570,
        t_wpigd_sun_get_red_002c1570,
        "tWPIGD_Sun_GetRed",
    ),
    (
        0x002c_1578,
        t_wpigd_sun_get_green_002c1578,
        "tWPIGD_Sun_GetGreen",
    ),
    (
        0x002c_1580,
        t_wpigd_sun_get_blue_002c1580,
        "tWPIGD_Sun_GetBlue",
    ),
    (
        0x002c_1588,
        t_wpigd_sun_get_intensity_002c1588,
        "tWPIGD_Sun_GetIntensity",
    ),
    (
        0x002c_1590,
        t_wpigd_sun_get_texture_002c1590,
        "tWPIGD_Sun_GetTexture",
    ),
    (
        0x002c_1598,
        t_wpigd_sun_get_flare_intensity_002c1598,
        "tWPIGD_Sun_GetFlareIntensity",
    ),
    (
        0x002c_15a0,
        t_wpigd_sun_get_size_002c15a0,
        "tWPIGD_Sun_GetSize",
    ),
    (
        0x002c_15a8,
        t_wpigd_surface_get08_002c15a8,
        "tWPIGD_Surface_Get08",
    ),
    (
        0x002c_15b0,
        t_wpigd_lighting_get_irradiance_name0_002c15b0,
        "tWPIGD_Lighting_GetIrradianceName0",
    ),
    (
        0x002c_15b8,
        t_wpigd_lighting_get_irradiance_name1_002c15b8,
        "tWPIGD_Lighting_GetIrradianceName1",
    ),
    (
        0x002c_15c0,
        t_wpigd_lighting_get28_002c15c0,
        "tWPIGD_Lighting_Get28",
    ),
    (
        0x002c_15c8,
        t_wpigd_lighting_get38_002c15c8,
        "tWPIGD_Lighting_Get38",
    ),
    (
        0x002c_15d0,
        t_wpigd_lighting_get48_002c15d0,
        "tWPIGD_Lighting_Get48",
    ),
    (
        0x002c_15d8,
        t_wpigd_lighting_get50_002c15d8,
        "tWPIGD_Lighting_Get50",
    ),
    (
        0x002c_15e0,
        t_wpigd_weather_get08_002c15e0,
        "tWPIGD_Weather_Get08",
    ),
    (
        0x002c_15e8,
        t_wpigd_weather_get10_002c15e8,
        "tWPIGD_Weather_Get10",
    ),
    (
        0x002c_15f0,
        t_wpigd_weather_get18_002c15f0,
        "tWPIGD_Weather_Get18",
    ),
    (
        0x002c_15f8,
        t_wpigd_weather_get20_002c15f8,
        "tWPIGD_Weather_Get20",
    ),
    (
        0x002c_1600,
        t_wpigd_weather_get28_002c1600,
        "tWPIGD_Weather_Get28",
    ),
    (
        0x002c_1608,
        t_wpigd_weather_get30_002c1608,
        "tWPIGD_Weather_Get30",
    ),
    (
        0x002c_1610,
        t_wpigd_weather_get38_002c1610,
        "tWPIGD_Weather_Get38",
    ),
    (
        0x002c_1618,
        t_wpigd_weather_get40_002c1618,
        "tWPIGD_Weather_Get40",
    ),
    (
        0x002c_1620,
        t_wpigd_weather_get48_002c1620,
        "tWPIGD_Weather_Get48",
    ),
    (
        0x002c_1628,
        t_wpigd_weather_get50_002c1628,
        "tWPIGD_Weather_Get50",
    ),
    (
        0x002c_1630,
        t_wpigd_weather_get58_002c1630,
        "tWPIGD_Weather_Get58",
    ),
    (
        0x002c_1638,
        t_wpigd_weather_get60_002c1638,
        "tWPIGD_Weather_Get60",
    ),
    (
        0x002c_1640,
        t_wpigd_weather_get68_002c1640,
        "tWPIGD_Weather_Get68",
    ),
    (
        0x002c_1648,
        t_wpigd_weather_get70_002c1648,
        "tWPIGD_Weather_Get70",
    ),
    (
        0x002c_1650,
        t_wpigd_weather_get78_002c1650,
        "tWPIGD_Weather_Get78",
    ),
    (
        0x002c_1658,
        t_wpigd_weather_get80_002c1658,
        "tWPIGD_Weather_Get80",
    ),
    (
        0x002c_1660,
        t_wpigd_weather_get88_002c1660,
        "tWPIGD_Weather_Get88",
    ),
    (
        0x002c_1668,
        t_wpigd_weather_get90_002c1668,
        "tWPIGD_Weather_Get90",
    ),
    (
        0x002c_1670,
        t_wpigd_weather_get98_002c1670,
        "tWPIGD_Weather_Get98",
    ),
    (
        0x002c_1678,
        t_wpigd_danger_get08_002c1678,
        "tWPIGD_Danger_Get08",
    ),
    (
        0x002c_1680,
        t_wpigd_danger_get10_002c1680,
        "tWPIGD_Danger_Get10",
    ),
];

// ---------------------------------------------------------------------------
// Quad tree.

/// `void QuadTreeNode_SetLeaf(QuadTreeNode* n, uint a, uint b)` (0x2c16a8):
/// store `b` at +0 and `a` at +4.
pub fn quad_tree_node_set_leaf(c: &mut Cpu) {
    c.cycles += 3;
    let n = Addr(abi::arg(c, 0));
    n.sw(c, c.g[A2][0]);
    n.off(0x4).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void QuadTreeNode_SetChildren(QuadTreeNode* n, uint c0, uint c1, uint c2,
/// uint c3)` (0x2c16b8): update the node's low half-words from the child
/// indices (each doubled, low bit from the node).
pub fn quad_tree_node_set_children(c: &mut Cpu) {
    c.cycles += 23;
    let n = Addr(abi::arg(c, 0));
    c.g[T1][0] = n.off(0x2).lhu(c);
    c.g[A1][0] &= 0xffff;
    c.g[V1][0] = n.off(0x4).lhu(c);
    c.g[A2][0] &= 0xffff;
    c.g[V0][0] = n.off(0x6).lhu(c);
    c.g[A3][0] &= 0xffff;
    c.g[T0][0] &= 0xffff;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
    c.g[T1][0] &= 0x1;
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 1);
    c.g[V1][0] &= 0x1;
    c.g[T0][0] = sx((c.g[T0][0] as u32) << 1);
    c.g[V0][0] &= 0x1;
    c.g[A2][0] |= c.g[T1][0];
    c.g[A3][0] |= c.g[V1][0];
    c.g[T0][0] |= c.g[V0][0];
    c.g[A1][0] |= 0x1;
    n.off(0x6).sh(c, c.g[T0][0]);
    n.sh(c, c.g[A1][0]);
    n.off(0x2).sh(c, c.g[A2][0]);
    n.off(0x4).sh(c, c.g[A3][0]);
    abi::ret(c);
}

/// `void QuadTree_InitFreeList(QuadTree* q)` (0x2c17c0): point the free-list
/// head and tail at the embedded list node at +0x28.
pub fn quad_tree_init_free_list(c: &mut Cpu) {
    c.cycles += 4;
    let q = Addr(abi::arg(c, 0));
    c.g[V0][0] = sx(q.0.wrapping_add(0x28));
    q.off(0x24).sw(c, c.g[V0][0]);
    q.off(0x20).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void QuadTree_FreeChildren(QuadTree* q, uint i0, uint i1, uint i2, uint
/// i3)` (0x2c18e8): splice the free-list node at +0x20, unlink the four child
/// entries and decrement the count at +0xc.
pub fn quad_tree_free_children(c: &mut Cpu) {
    c.cycles += 25;
    let q = Addr(abi::arg(c, 0));
    c.g[T1][0] = q.off(0x20).lw(c);
    c.g[A1][0] &= 0xffff;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
    c.g[A2][0] &= 0xffff;
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A3][0] &= 0xffff;
    Addr(c.g[V0][0] as u32).sh(c, c.g[A2][0]);
    c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
    Addr(c.g[V1][0] as u32).sh(c, c.g[A3][0]);
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[T0][0] &= 0xffff;
    Addr(c.g[V0][0] as u32).sh(c, c.g[T0][0]);
    c.g[T0][0] = sx((c.g[T0][0] as u32) << 3);
    c.g[V0][0] = q.off(0x24).lw(c);
    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
    Addr(c.g[T1][0] as u32).sh(c, c.g[V1][0]);
    c.g[A2][0] = q.off(0x24).lw(c);
    Addr(c.g[A2][0] as u32).sh(c, c.g[A1][0]);
    c.g[V0][0] = q.off(0xc).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    q.off(0xc).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `WorldPainterQuery_BaseDtor` (0x2c1460): vtable at `+0x4`.
pub fn world_painter_query_base_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x4, 0x0048_5d30, 0x002c_1484);
}

/// `float* tWPIGD_Sun::GetElevation()` (0x2c1560): `&this->elevation`
/// (+0x08).
pub fn twpigd_sun_get_elevation(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x8));
    ret(c);
}

/// `float* tWPIGD_Sun::GetAzimuth()` (0x2c1568): `&this->azimuth` (+0x10).
pub fn twpigd_sun_get_azimuth(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x10));
    ret(c);
}

/// Return address after the allocation in `WorldPainterQuery_Create`.
pub(crate) const WPQ_ALLOC_RET: u32 = 0x002c_0a80;

/// Return address after `WorldPainterQuery_ctor` in `WorldPainterQuery_Create`.
pub(crate) const WPQ_CTOR_RET: u32 = 0x002c_0a8c;

/// `WorldPainterQuery_ctor` (0x2c0408).
pub(crate) const WPQ_CTOR: u32 = 0x002c_0408;

/// `WorldPainterQuery* cWorldPainterMan_getQuery(void* self, cWorldPainterManArg* arg)`
/// (0x2c0a58): allocates a 0x18-byte `WorldPainterQuery` (named by a string in
/// the image), constructs it from the word `*arg` points at, and returns it in
/// `v0`. The frame keeps `s0` (the `arg` pointer) and `ra` across the calls.
pub fn world_painter_query_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A3][0] = 0;
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        // EA_MemAlloc(0x18, name): a0 = size, a1 = name string.
        c.g[A0][0] = 0x18;
        c.g[A1][0] = sx(0x0048_3de8);
        call_ptr(c, WPQ_ALLOC, WPQ_ALLOC_RET)?;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: WPQ_CTOR,
            f: ee::s08::f_002c0408,
        }
        .call(c, WPQ_CTOR_RET)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// One float component of a static colour: a constant, or a word of the gp
/// constant pool (`gp - n`, the `.lit4` values the compiler hoisted).
#[derive(Clone, Copy)]
pub(crate) enum Oc1040Comp {
    Bits(u32),
    Pool(i32),
}

/// Address of the first `sColor` (16 bytes each, 56 of them).
pub(crate) const OC1040_COLOR_BASE: u32 = 0x004d_4638;

pub(crate) const OC1040_COLOR_COUNT: usize = 56;

/// The 56 static world-painter colours, 4 floats each, in address order.
pub(crate) static OC1040_COLORS: [[Oc1040Comp; 4]; OC1040_COLOR_COUNT] = [
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f000000),
    ], // 0x4d4638
    [
        Pool(-0x427c),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
    ], // 0x4d4648
    [
        Pool(-0x4278),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
    ], // 0x4d4658
    [Pool(-0x4274), Pool(-0x4254), Pool(-0x4250), Pool(-0x4270)], // 0x4d4668
    [
        Pool(-0x426c),
        Pool(-0x424c),
        Pool(-0x4248),
        Bits(0x00000000),
    ], // 0x4d4678
    [
        Pool(-0x4268),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
    ], // 0x4d4688
    [Pool(-0x4264), Pool(-0x4244), Pool(-0x4240), Pool(-0x4260)], // 0x4d4698
    [
        Pool(-0x425c),
        Pool(-0x423c),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d46a8
    [
        Pool(-0x4258),
        Pool(-0x4238),
        Pool(-0x4228),
        Bits(0x00000000),
    ], // 0x4d46b8
    [Pool(-0x4234), Pool(-0x4224), Pool(-0x4220), Pool(-0x4230)], // 0x4d46c8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d46d8
    [
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d46e8
    [
        Bits(0x3f800000),
        Pool(-0x422c),
        Pool(-0x421c),
        Pool(-0x422c),
    ], // 0x4d46f8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4708
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4718
    [
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d4728
    [
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d4738
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4748
    [
        Bits(0x3f800000),
        Bits(0x3f000000),
        Bits(0x3f800000),
        Bits(0x3f000000),
    ], // 0x4d4758
    [
        Bits(0x3f800000),
        Bits(0x3f000000),
        Bits(0x3f800000),
        Bits(0x3f000000),
    ], // 0x4d4768
    [
        Bits(0x3f800000),
        Bits(0x3f000000),
        Bits(0x3f800000),
        Bits(0x3f000000),
    ], // 0x4d4778
    [
        Bits(0x3f800000),
        Pool(-0x4210),
        Pool(-0x420c),
        Pool(-0x4218),
    ], // 0x4d4788
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Pool(-0x4208),
        Bits(0x00000000),
    ], // 0x4d4798
    [
        Pool(-0x4214),
        Pool(-0x4204),
        Pool(-0x4200),
        Bits(0x00000000),
    ], // 0x4d47a8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d47b8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Pool(-0x41e8),
        Bits(0x00000000),
    ], // 0x4d47c8
    [
        Bits(0x3f800000),
        Pool(-0x41e4),
        Pool(-0x41e0),
        Bits(0x00000000),
    ], // 0x4d47d8
    [
        Bits(0x3f800000),
        Pool(-0x41dc),
        Pool(-0x41d8),
        Pool(-0x41fc),
    ], // 0x4d47e8
    [
        Bits(0x3f800000),
        Pool(-0x41d0),
        Pool(-0x41d4),
        Bits(0x3f800000),
    ], // 0x4d47f8
    [
        Bits(0x3f800000),
        Pool(-0x41f4),
        Bits(0x3f800000),
        Pool(-0x41f8),
    ], // 0x4d4808
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4818
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4828
    [
        Bits(0x3f800000),
        Pool(-0x41f0),
        Pool(-0x41ec),
        Bits(0x00000000),
    ], // 0x4d4838
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4848
    [
        Bits(0x3f800000),
        Pool(-0x41bc),
        Bits(0x3f800000),
        Bits(0x00000000),
    ], // 0x4d4858
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4868
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d4878
    [
        Pool(-0x41cc),
        Bits(0x00000000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d4888
    [
        Bits(0x3f800000),
        Pool(-0x41b8),
        Pool(-0x41b4),
        Pool(-0x41c8),
    ], // 0x4d4898
    [
        Bits(0x3f800000),
        Pool(-0x41b0),
        Bits(0x3f800000),
        Pool(-0x41c4),
    ], // 0x4d48a8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Pool(-0x41ac),
        Bits(0x00000000),
    ], // 0x4d48b8
    [
        Bits(0x3f800000),
        Pool(-0x41a8),
        Bits(0x3f800000),
        Pool(-0x41c0),
    ], // 0x4d48c8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d48d8
    [
        Bits(0x3f800000),
        Pool(-0x41d0),
        Bits(0x3f800000),
        Pool(-0x41d0),
    ], // 0x4d48e8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d48f8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d4908
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d4918
    [
        Bits(0x3f800000),
        Pool(-0x419c),
        Bits(0x3f800000),
        Pool(-0x41a4),
    ], // 0x4d4928
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Pool(-0x4198),
        Pool(-0x41a0),
    ], // 0x4d4938
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Pool(-0x4194),
        Bits(0x00000000),
    ], // 0x4d4948
    [
        Bits(0x3f800000),
        Pool(-0x4190),
        Pool(-0x418c),
        Bits(0x00000000),
    ], // 0x4d4958
    [
        Bits(0x3f800000),
        Pool(-0x4180),
        Pool(-0x417c),
        Bits(0x00000000),
    ], // 0x4d4968
    [
        Bits(0x3f800000),
        Pool(-0x4178),
        Pool(-0x4174),
        Pool(-0x4188),
    ], // 0x4d4978
    [
        Bits(0x3f800000),
        Pool(-0x4170),
        Pool(-0x416c),
        Pool(-0x4184),
    ], // 0x4d4988
    [
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x3f800000),
        Bits(0x00000000),
    ], // 0x4d4998
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d49a8
];

/// The gp-relative stores that follow (`is_byte`, gp offset, value): the
/// region-type table (the id as a byte, then replicated in a word), in the
/// order of the generated body.
pub(crate) static OC1040_GP_STORES: &[(bool, u32, u32)] = &[
    (true, 0x2300, 0x3),
    (true, 0x22e8, 0x0),
    (true, 0x22f0, 0x1),
    (true, 0x22f8, 0x2),
    (false, 0x22e8, 0x0),
    (false, 0x22f0, 0x101),
    (false, 0x22f8, 0x202),
    (false, 0x22e4, 0x0),
    (false, 0x22e0, 0x0),
    (false, 0x2300, 0x303),
    (true, 0x2308, 0x4),
    (true, 0x2310, 0x5),
    (true, 0x2340, 0xb),
    (true, 0x2350, 0xd),
    (true, 0x2318, 0x6),
    (true, 0x2320, 0x7),
    (true, 0x2328, 0x8),
    (true, 0x2330, 0x9),
    (true, 0x2338, 0xa),
    (true, 0x2348, 0xc),
    (false, 0x2308, 0x404),
    (false, 0x2310, 0x505),
    (false, 0x2318, 0x606),
    (false, 0x2320, 0x707),
    (false, 0x2328, 0x808),
    (false, 0x2330, 0x909),
    (false, 0x2338, 0xa0a),
    (false, 0x2340, 0xb0b),
    (false, 0x2348, 0xc0c),
    (false, 0x2350, 0xd0d),
];

/// FPU registers the generated body leaves behind (last `lwc1`/`mtc1`).
pub(crate) static OC1040_FINAL_F: &[(usize, Oc1040Comp)] = &[
    (0, Bits(0x3f800000)),
    (1, Pool(-0x4184)),
    (2, Pool(-0x4188)),
    (3, Pool(-0x4180)),
    (4, Pool(-0x417c)),
    (5, Pool(-0x4178)),
    (6, Pool(-0x4174)),
    (7, Pool(-0x4170)),
    (8, Pool(-0x416c)),
    (9, Pool(-0x41ac)),
    (10, Pool(-0x41a8)),
    (11, Pool(-0x41ec)),
    (12, Pool(-0x41d0)),
    (13, Pool(-0x424c)),
    (14, Pool(-0x4248)),
    (15, Pool(-0x4244)),
    (16, Pool(-0x4240)),
    (17, Pool(-0x4254)),
    (18, Pool(-0x4250)),
    (19, Pool(-0x4238)),
];

/// `void* WorldPainterQuery_StaticInitDestroy(int initialize, int priority)`
/// (0x2c0b70): the compiler's static-init/destroy function of the world
/// painter query unit. Only for `priority == 0xffff` and `initialize != 0` it
/// constructs the 56 static `sColor`s (0x4d4638..0x4d49b8), clears the null
/// name field and builds the 14 `tWorldPainterRegionType`s (ids 0..13, byte
/// and replicated-word forms at gp+0x22e0..0x2350). Every other call does
/// nothing. The pool constants are loaded at run time, as the original does,
/// and the scratch registers it leaves behind are reproduced.
pub fn world_painter_query_static_init_destroy(c: &mut Cpu) {
    c.cycles += 12;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x90));
    c.g[V0][0] = 0xffff;
    // sq s0..s7 at 0x80..0x10(sp)
    for i in 0..8usize {
        let a = sp(c).wrapping_add(0x80 - 0x10 * i as u32);
        c.sq(16 + i, a);
    }
    let priority_other = c.g[A1][0] != c.g[V0][0];
    let a = sp(c);
    c.sq(30, a);
    if !priority_other {
        c.cycles += 2;
        let initialize = c.g[A0][0] != 0;
        c.g[T5][0] = 0x4d0000;
        if initialize {
            oc1040_construct_statics(c);
        }
    }
    c.cycles += 11;
    for i in 0..8usize {
        let a = sp(c).wrapping_add(0x80 - 0x10 * i as u32);
        c.lq(16 + i, a);
    }
    let a = sp(c);
    c.lq(30, a);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x90));
    c.pc = ra;
}

/// The initialising branch of [`world_painter_query_static_init_destroy`].
pub(crate) fn oc1040_construct_statics(c: &mut Cpu) {
    c.cycles += 544;
    let gp = c.g[GP][0] as u32;
    let comp = |c: &mut Cpu, k: Oc1040Comp| match k {
        Bits(b) => b,
        Pool(off) => c.bus.read32(gp.wrapping_add(off as u32)),
    };
    for (i, color) in OC1040_COLORS.iter().enumerate() {
        for (j, k) in color.iter().enumerate() {
            let v = comp(c, *k);
            Addr(OC1040_COLOR_BASE + 16 * i as u32 + 4 * j as u32).sw(c, v as u64);
        }
    }
    for &(is_byte, off, v) in OC1040_GP_STORES {
        let a = Addr(gp.wrapping_add(off));
        if is_byte {
            a.sb(c, v as u64);
        } else {
            a.sw(c, v as u64);
        }
    }
    for &(n, k) in OC1040_FINAL_F {
        c.f[n] = comp(c, k);
    }
    // scratch registers left by the generated body (s0..s7/fp are restored)
    c.g[AT][0] = 0x3f000000;
    c.g[T0][0] = 0x404;
    c.g[T1][0] = 0x707;
    c.g[T2][0] = 0x505;
    c.g[T3][0] = 0x606;
    c.g[T4][0] = 0x4d48b8;
    c.g[T5][0] = 0x6;
    c.g[T6][0] = 0x4d4948;
    c.g[T7][0] = 0x4d4938;
    c.g[T8][0] = 0x4d0000;
    c.g[T9][0] = 0x4d0000;
    c.g[A0][0] = 0xb0b;
    c.g[A1][0] = 0xa0a;
    c.g[A2][0] = 0x909;
    c.g[A3][0] = 0x808;
    c.g[V0][0] = 0xd0d;
    c.g[V1][0] = 0xc0c;
}

/// `WorldPainterQuery_StaticInitDestroy` (0x2c0b70).
pub(crate) const WPQ_STATIC_INIT_DESTROY: Callee = Callee {
    addr: 0x002c_0b70,
    f: ee::s08::WorldPainterQuery_StaticInitDestroy,
};

/// Global constructor of the painter-query module (0x2c1688): the static init
/// `WorldPainterQuery_StaticInitDestroy(1, 0xffff)`.
pub fn world_painter_query_global_ctors(c: &mut Cpu) {
    guest(c, world_painter_query_global_ctors_body);
}

pub(crate) fn world_painter_query_global_ctors_body(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    let f = Frame::open(c, 0x10);
    c.g[A0][0] = 1;
    f.save_ra(c, 0);
    c.g[A1][0] = 0xffff;
    WPQ_STATIC_INIT_DESTROY.call(c, 0x002c_169c)?;
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `tWPIGD_Base_GetMusicTrigger08` (0x002b_c500): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_music_trigger08_002bc500(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetMusicTrigger10` (0x002b_c508): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_music_trigger10_002bc508(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetMix08` (0x002b_c510): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_mix08_002bc510(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetAmbience08` (0x002b_c518): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_ambience08_002bc518(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSpeech08` (0x002b_c520): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_speech08_002bc520(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetCamera08` (0x002b_c528): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_camera08_002bc528(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetFog08` (0x002b_c530): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_fog08_002bc530(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetFog10` (0x002b_c538): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_fog10_002bc538(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetFog18` (0x002b_c540): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_fog18_002bc540(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetFog20` (0x002b_c548): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_fog20_002bc548(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetFog28` (0x002b_c550): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_fog28_002bc550(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetFog30` (0x002b_c558): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_fog30_002bc558(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLightGlow08` (0x002b_c560): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow08_002bc560(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLightGlow10` (0x002b_c568): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow10_002bc568(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLightGlow18` (0x002b_c570): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow18_002bc570(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLightGlow20` (0x002b_c578): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow20_002bc578(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLightGlow28` (0x002b_c580): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow28_002bc580(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLightGlow30` (0x002b_c588): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow30_002bc588(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLightGlow38` (0x002b_c590): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow38_002bc590(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetScreenTintRed` (0x002b_c598): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_screen_tint_red_002bc598(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetScreenTintGreen` (0x002b_c5a0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_screen_tint_green_002bc5a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetScreenTintBlue` (0x002b_c5a8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_screen_tint_blue_002bc5a8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetScreenFillRed` (0x002b_c5b0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_screen_fill_red_002bc5b0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetScreenFillGreen` (0x002b_c5b8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_screen_fill_green_002bc5b8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetScreenFillBlue` (0x002b_c5c0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_screen_fill_blue_002bc5c0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSkyBox08` (0x002b_c5c8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sky_box08_002bc5c8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunElevation` (0x002b_c5d0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_elevation_002bc5d0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunAzimuth` (0x002b_c5d8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_azimuth_002bc5d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunRed` (0x002b_c5e0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_red_002bc5e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunGreen` (0x002b_c5e8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_green_002bc5e8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunBlue` (0x002b_c5f0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_blue_002bc5f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunIntensity` (0x002b_c5f8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_intensity_002bc5f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunTexture` (0x002b_c600): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_texture_002bc600(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunFlareIntensity` (0x002b_c608): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_flare_intensity_002bc608(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunSize` (0x002b_c610): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_size_002bc610(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSurface08` (0x002b_c618): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_surface08_002bc618(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetIrradianceName0` (0x002b_c620): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_irradiance_name0_002bc620(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetIrradianceName1` (0x002b_c628): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_irradiance_name1_002bc628(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetIrradianceName2` (0x002b_c630): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_irradiance_name2_002bc630(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetIrradianceName3` (0x002b_c638): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_irradiance_name3_002bc638(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLighting48` (0x002b_c640): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_lighting48_002bc640(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLighting50` (0x002b_c648): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_lighting50_002bc648(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather08` (0x002b_c650): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather08_002bc650(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather10` (0x002b_c658): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather10_002bc658(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather18` (0x002b_c660): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather18_002bc660(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather20` (0x002b_c668): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather20_002bc668(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather28` (0x002b_c670): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather28_002bc670(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather30` (0x002b_c678): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather30_002bc678(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather38` (0x002b_c680): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather38_002bc680(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather40` (0x002b_c688): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather40_002bc688(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather48` (0x002b_c690): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather48_002bc690(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather50` (0x002b_c698): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather50_002bc698(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather58` (0x002b_c6a0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather58_002bc6a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather60` (0x002b_c6a8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather60_002bc6a8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather68` (0x002b_c6b0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather68_002bc6b0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather70` (0x002b_c6b8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather70_002bc6b8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather78` (0x002b_c6c0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather78_002bc6c0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather80` (0x002b_c6c8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather80_002bc6c8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather88` (0x002b_c6d0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather88_002bc6d0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather90` (0x002b_c6d8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather90_002bc6d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather98` (0x002b_c6e0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather98_002bc6e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetDanger08` (0x002b_c6e8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_danger08_002bc6e8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetDanger10` (0x002b_c6f0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_danger10_002bc6f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_MusicTrigger_v67` (0x002b_da28): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_music_trigger_v67_002bda28(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Ambience_v67` (0x002b_da30): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_ambience_v67_002bda30(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Camera_v67` (0x002b_da38): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_camera_v67_002bda38(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Danger_v67` (0x002b_da40): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_danger_v67_002bda40(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Fog_v67` (0x002b_da48): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_v67_002bda48(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_LightGlow_v67` (0x002b_da50): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_v67_002bda50(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Lighting_v67` (0x002b_da58): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_v67_002bda58(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Mix_v67` (0x002b_da60): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_mix_v67_002bda60(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_ScreenTint_v67` (0x002b_da68): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_v67_002bda68(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_SkyBox_v67` (0x002b_da70): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sky_box_v67_002bda70(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Speech_v67` (0x002b_da78): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_speech_v67_002bda78(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Sun_v67` (0x002b_da80): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_v67_002bda80(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Surface_v67` (0x002b_da88): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_surface_v67_002bda88(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Weather_v67` (0x002b_da90): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_v67_002bda90(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Mix_IsEqual` (0x002b_dac8): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn t_wpigd_mix_is_equal_002bdac8(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    ret(c);
}

/// `tWPIGD_Ambience_IsEqual` (0x002b_dae0): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn t_wpigd_ambience_is_equal_002bdae0(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    ret(c);
}

/// `tWPIGD_Speech_IsEqual` (0x002b_daf8): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn t_wpigd_speech_is_equal_002bdaf8(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    ret(c);
}

/// `tWPIGD_MusicTrigger_SetDefaults` (0x002b_e0b0): straight-line leaf: 3 store(s). Mechanical port of the generated body.
pub fn t_wpigd_music_trigger_set_defaults_002be0b0(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0xffffffffffffffff;
    c.sw(c.g[A0][0] as u32, 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    ret(c);
}

/// `tWPIGD_Mix_SetDefaults` (0x002b_e0c8): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn t_wpigd_mix_set_defaults_002be0c8(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0xffffffffffffffff;
    c.sw(c.g[A0][0] as u32, 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    ret(c);
}

/// `tWPIGD_Ambience_SetDefaults` (0x002b_e0d8): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn t_wpigd_ambience_set_defaults_002be0d8(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0xffffffffffffffff;
    c.sw(c.g[A0][0] as u32, 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    ret(c);
}

/// `tWPIGD_Speech_SetDefaults` (0x002b_e0e8): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn t_wpigd_speech_set_defaults_002be0e8(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0xffffffffffffffff;
    c.sw(c.g[A0][0] as u32, 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    ret(c);
}

/// `tWPIGD_Camera_SetDefaults` (0x002b_e0f8): straight-line leaf: 1 load(s), 2 store(s). Mechanical port of the generated body.
pub fn t_wpigd_camera_set_defaults_002be0f8(c: &mut Cpu) {
    c.cycles += 4;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x42a8));
    c.sw(c.g[A0][0] as u32, 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Fog_SetDefaults` (0x002b_e108): straight-line leaf: 5 load(s), 7 store(s). Mechanical port of the generated body.
pub fn t_wpigd_fog_set_defaults_002be108(c: &mut Cpu) {
    c.cycles += 13;
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x42a4));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x42a0));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x429c));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4298));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4294));
    c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x18));
    c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0x28));
    c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.sw(c.g[A0][0] as u32, 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
    ret(c);
}

/// `tWPIGD_LightGlow_SetDefaults` (0x002b_e140): straight-line leaf: 8 store(s). Mechanical port of the generated body.
pub fn t_wpigd_light_glow_set_defaults_002be140(c: &mut Cpu) {
    c.cycles += 11;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    c.sw(c.g[A0][0] as u32, 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x28));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x18));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x30), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x38), 0u64);
    ret(c);
}

/// `tWPIGD_ScreenTint_SetDefaults` (0x002b_e170): straight-line leaf: 7 store(s). Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_set_defaults_002be170(c: &mut Cpu) {
    c.cycles += 10;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    c.sw(c.g[A0][0] as u32, 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x18));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x20), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x28), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x30), 0u64);
    ret(c);
}

/// `tWPIGD_SkyBox_SetDefaults` (0x002b_e198): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn t_wpigd_sky_box_set_defaults_002be198(c: &mut Cpu) {
    c.cycles += 3;
    c.sw(c.g[A0][0] as u32, 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
    ret(c);
}

/// `tWPIGD_Sun_SetDefaults` (0x002b_e1a8): straight-line leaf: 10 store(s). Mechanical port of the generated body.
pub fn t_wpigd_sun_set_defaults_002be1a8(c: &mut Cpu) {
    c.cycles += 13;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    c.sw(c.g[A0][0] as u32, 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x48));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x18));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x28));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x38), 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x40));
    ret(c);
}

/// `tWPIGD_Surface_SetDefaults` (0x002b_e1e0): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn t_wpigd_surface_set_defaults_002be1e0(c: &mut Cpu) {
    c.cycles += 5;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    c.sw(c.g[A0][0] as u32, 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Danger_SetDefaults` (0x002b_e2c8): straight-line leaf: 3 store(s). Mechanical port of the generated body.
pub fn t_wpigd_danger_set_defaults_002be2c8(c: &mut Cpu) {
    c.cycles += 4;
    c.sw(c.g[A0][0] as u32, 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), 0u64);
    ret(c);
}

/// `WorldPainterQuery_DebugPrintf` (0x002c_0b30): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn world_painter_query_debug_printf(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.g[V0][0] = c.g[A0][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x44));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
    ret(c);
}

/// `tWPIGD_Base_v67` (0x002c_1458): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_v67_002c1458(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_MusicTrigger_Get08` (0x002c_1490): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_music_trigger_get08_002c1490(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_MusicTrigger_Get10` (0x002c_1498): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_music_trigger_get10_002c1498(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `tWPIGD_Mix_Get08` (0x002c_14a0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_mix_get08_002c14a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Ambience_Get08` (0x002c_14a8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_ambience_get08_002c14a8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Speech_Get08` (0x002c_14b0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_speech_get08_002c14b0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Camera_Get08` (0x002c_14b8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_camera_get08_002c14b8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Fog_Get08` (0x002c_14c0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_get08_002c14c0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Fog_Get10` (0x002c_14c8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_get10_002c14c8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `tWPIGD_Fog_Get18` (0x002c_14d0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_get18_002c14d0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    ret(c);
}

/// `tWPIGD_Fog_Get20` (0x002c_14d8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_get20_002c14d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `tWPIGD_Fog_Get28` (0x002c_14e0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_get28_002c14e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `tWPIGD_Fog_Get30` (0x002c_14e8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_get30_002c14e8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `tWPIGD_LightGlow_Get08` (0x002c_14f0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get08_002c14f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_LightGlow_Get10` (0x002c_14f8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get10_002c14f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `tWPIGD_LightGlow_Get18` (0x002c_1500): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get18_002c1500(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    ret(c);
}

/// `tWPIGD_LightGlow_Get20` (0x002c_1508): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get20_002c1508(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `tWPIGD_LightGlow_Get28` (0x002c_1510): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get28_002c1510(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `tWPIGD_LightGlow_Get30` (0x002c_1518): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get30_002c1518(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `tWPIGD_LightGlow_Get38` (0x002c_1520): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get38_002c1520(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
    ret(c);
}

/// `tWPIGD_ScreenTint_GetTintRed` (0x002c_1528): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_get_tint_red_002c1528(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_ScreenTint_GetTintGreen` (0x002c_1530): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_get_tint_green_002c1530(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `tWPIGD_ScreenTint_GetTintBlue` (0x002c_1538): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_get_tint_blue_002c1538(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    ret(c);
}

/// `tWPIGD_ScreenTint_GetFillRed` (0x002c_1540): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_get_fill_red_002c1540(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `tWPIGD_ScreenTint_GetFillGreen` (0x002c_1548): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_get_fill_green_002c1548(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `tWPIGD_ScreenTint_GetFillBlue` (0x002c_1550): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_get_fill_blue_002c1550(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `tWPIGD_SkyBox_Get08` (0x002c_1558): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sky_box_get08_002c1558(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Sun_GetRed` (0x002c_1570): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_red_002c1570(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    ret(c);
}

/// `tWPIGD_Sun_GetGreen` (0x002c_1578): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_green_002c1578(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `tWPIGD_Sun_GetBlue` (0x002c_1580): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_blue_002c1580(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `tWPIGD_Sun_GetIntensity` (0x002c_1588): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_intensity_002c1588(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `tWPIGD_Sun_GetTexture` (0x002c_1590): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_texture_002c1590(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
    ret(c);
}

/// `tWPIGD_Sun_GetFlareIntensity` (0x002c_1598): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_flare_intensity_002c1598(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    ret(c);
}

/// `tWPIGD_Sun_GetSize` (0x002c_15a0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_size_002c15a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
    ret(c);
}

/// `tWPIGD_Surface_Get08` (0x002c_15a8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_surface_get08_002c15a8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Lighting_GetIrradianceName0` (0x002c_15b0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_get_irradiance_name0_002c15b0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Lighting_GetIrradianceName1` (0x002c_15b8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_get_irradiance_name1_002c15b8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    ret(c);
}

/// `tWPIGD_Lighting_Get28` (0x002c_15c0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_get28_002c15c0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `tWPIGD_Lighting_Get38` (0x002c_15c8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_get38_002c15c8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
    ret(c);
}

/// `tWPIGD_Lighting_Get48` (0x002c_15d0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_get48_002c15d0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
    ret(c);
}

/// `tWPIGD_Lighting_Get50` (0x002c_15d8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_get50_002c15d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
    ret(c);
}

/// `tWPIGD_Weather_Get08` (0x002c_15e0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get08_002c15e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Weather_Get10` (0x002c_15e8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get10_002c15e8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `tWPIGD_Weather_Get18` (0x002c_15f0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get18_002c15f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    ret(c);
}

/// `tWPIGD_Weather_Get20` (0x002c_15f8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get20_002c15f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `tWPIGD_Weather_Get28` (0x002c_1600): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get28_002c1600(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `tWPIGD_Weather_Get30` (0x002c_1608): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get30_002c1608(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `tWPIGD_Weather_Get38` (0x002c_1610): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get38_002c1610(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
    ret(c);
}

/// `tWPIGD_Weather_Get40` (0x002c_1618): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get40_002c1618(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    ret(c);
}

/// `tWPIGD_Weather_Get48` (0x002c_1620): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get48_002c1620(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
    ret(c);
}

/// `tWPIGD_Weather_Get50` (0x002c_1628): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get50_002c1628(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
    ret(c);
}

/// `tWPIGD_Weather_Get58` (0x002c_1630): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get58_002c1630(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58));
    ret(c);
}

/// `tWPIGD_Weather_Get60` (0x002c_1638): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get60_002c1638(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
    ret(c);
}

/// `tWPIGD_Weather_Get68` (0x002c_1640): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get68_002c1640(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x68));
    ret(c);
}

/// `tWPIGD_Weather_Get70` (0x002c_1648): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get70_002c1648(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x70));
    ret(c);
}

/// `tWPIGD_Weather_Get78` (0x002c_1650): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get78_002c1650(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x78));
    ret(c);
}

/// `tWPIGD_Weather_Get80` (0x002c_1658): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get80_002c1658(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x80));
    ret(c);
}

/// `tWPIGD_Weather_Get88` (0x002c_1660): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get88_002c1660(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x88));
    ret(c);
}

/// `tWPIGD_Weather_Get90` (0x002c_1668): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get90_002c1668(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x90));
    ret(c);
}

/// `tWPIGD_Weather_Get98` (0x002c_1670): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get98_002c1670(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x98));
    ret(c);
}

/// `tWPIGD_Danger_Get08` (0x002c_1678): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_danger_get08_002c1678(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Danger_Get10` (0x002c_1680): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_danger_get10_002c1680(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// The shared body of the `tWPIGD_*_LoadState/SaveState` replay-frame
/// walkers: `stream->vt[vt_fn](stream + vt_adj, field, 4)` for each of
/// `fields` four-byte fields at `self + 8, +0x10, ...`, every field followed
/// by a four-byte transfer of the word at `self + 0`. Calls return to
/// `first_ret + 0x1c * n`.
pub(crate) fn wp_ingame_stream_fields(
    c: &mut Cpu,
    fields: u32,
    first_ret: u32,
    vt_adj_off: u32,
    vt_fn_off: u32,
) -> Flow {
    let frame = Frame::open(c, 0x30);
    c.g[A2][0] = 0x4;
    frame.save(c, 16, 0x20);
    frame.save(c, 17, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    for call in 0..2 * fields {
        // Even calls move field `call / 2` (self + 8 * (n + 1)), odd calls
        // the word at self + 0.
        c.g[A1][0] = if call % 2 == 0 {
            sx((c.g[S1][0] as u32).wrapping_add(8 * (call / 2 + 1)))
        } else {
            c.g[S1][0]
        };
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(vt_adj_off));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(vt_fn_off));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, first_ret + 0x1c * call)?;
    }
    frame.restore(c, 16, 0x20);
    frame.restore(c, 17, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `tWPIGD_Sun_LoadState(self, stream)` (0x2bea38,
/// `tWPInGameData_Sun_readFromReplayFrame`): read the 9 sun fields (pitch,
/// yaw, colour, intensity, type, lens...) from the replay stream (vtable
/// +0x10 offset adjust, +0x14 read function), 4 bytes each.
pub fn twpigd_sun_load_state(c: &mut Cpu) {
    c.cycles += 137;
    guest(c, |c| {
        wp_ingame_stream_fields(c, 9, 0x002b_ea6c, 0x10, 0x14)
    });
}

/// `tWPIGD_Weather_LoadState(self, stream)` (0x2bee48,
/// `tWPInGameData_Weather_readFromReplayFrame`): the 19 weather fields.
pub fn twpigd_weather_load_state(c: &mut Cpu) {
    c.cycles += 277;
    guest(c, |c| {
        wp_ingame_stream_fields(c, 19, 0x002b_ee7c, 0x10, 0x14)
    });
}

/// `tWPIGD_Sun_SaveState(self, stream)` (0x2bfaa0,
/// `tWPInGameData_Sun_writeToReplayFrame`): the same 9 sun fields written to
/// the stream (vtable +0x8 offset adjust, +0xc write function).
pub fn twpigd_sun_save_state(c: &mut Cpu) {
    c.cycles += 137;
    guest(c, |c| wp_ingame_stream_fields(c, 9, 0x002b_fad4, 0x8, 0xc));
}

/// `base + off` as the 32-bit address the generated code computes.
#[inline(always)]
pub(crate) fn wp_field(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// The virtual call of a blend: the method at `vt + 0x218` (`lh` delta, `lw`
/// target) is called with `this + delta`, returning to `ret`.
pub(crate) fn wp_blend_call(c: &mut Cpu, this: u64, ret: u32) -> Flow {
    let vt = c.g[V1][0];
    let slot = VSlot::read(c, vt, 0x218);
    c.g[A0][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(this);
    call_ptr(c, slot.func as u32, ret)
}

/// The bank blend shared by `tWPIGD_Ambience_Blend` and `tWPIGD_Speech_Blend`
/// (the two generated bodies are the same apart from their return addresses):
/// the blend factor is squared, the object's method is called (`first_ret`),
/// then the integer field at +8 becomes `b * rhs + (1 - b) * self` (`b` the
/// squared factor), the field at +0xc takes `rhs`'s value, and the method is
/// called again (`second_ret`).
pub(crate) fn wp_igd_bank_blend(c: &mut Cpu, first_ret: u32, second_ret: u32) -> Flow {
    c.cycles += 39;
    let frame = Frame::open(c, 0x40);
    frame.save(c, 16, 0x20);
    frame.save(c, 17, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    frame.save_fpr(c, 20, 0x30);
    c.mov_s(20, 12);
    c.g[V1][0] = Addr(wp_field(c.g[S0][0], 4)).lw(c);
    c.mul_s(20, 20, 20);
    let this = c.g[S0][0];
    wp_blend_call(c, this, first_ret)?;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    let rhs_level = wp_field(c.g[S1][0], 4);
    c.lwc1(1, rhs_level);
    c.cvt_s_w(1, 1);
    let self_bank = wp_field(c.g[S0][0], 8);
    c.lwc1(2, self_bank);
    c.cvt_s_w(2, 2);
    c.g[V1][0] = Addr(wp_field(c.g[S0][0], 4)).lw(c);
    c.sub_s(0, 0, 20);
    c.mul_s(20, 20, 1);
    c.mul_s(0, 0, 2);
    c.add_s(20, 20, 0);
    c.cvt_w_s(0, 20);
    let self_bank = wp_field(c.g[S0][0], 8);
    c.swc1(0, self_bank);
    c.g[V0][0] = Addr(wp_field(c.g[S1][0], 4)).lw(c);
    let stored = c.g[V0][0];
    Addr(wp_field(c.g[S0][0], 0xc)).sw(c, stored);
    let this = c.g[S0][0];
    wp_blend_call(c, this, second_ret)?;
    frame.restore(c, 16, 0x20);
    frame.restore(c, 17, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x30);
    frame.close(c);
    Ok(())
}

/// `void tWPIGD_Ambience_Blend(self, rhs, float blend)` (0x2bcd68).
pub fn wp_igd_ambience_blend(c: &mut Cpu) {
    guest(c, |c| wp_igd_bank_blend(c, 0x002b_cda0, 0x002b_cdec));
}

/// `void tWPIGD_Speech_Blend(self, rhs, float blend)` (0x2bce08): the same
/// blend as the ambience one, with its own call sites.
pub fn wp_igd_speech_blend(c: &mut Cpu) {
    guest(c, |c| wp_igd_bank_blend(c, 0x002b_ce40, 0x002b_ce8c));
}

/// `void tWPIGD_Danger_Blend(self, rhs, float blend)` (0x2bd968): squares the
/// blend factor, calls the method, then blends the level (integer, rounded
/// toward zero), the point rate (float) and the last-level field; the rate
/// blends `b * rhs + (1 - b) * self` with the squared factor.
pub fn wp_igd_danger_blend(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(20, 12);
        c.g[V1][0] = Addr(wp_field(c.g[S0][0], 4)).lw(c);
        c.mul_s(20, 20, 20);
        let this = c.g[S0][0];
        wp_blend_call(c, this, 0x002b_d9a0)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        let rhs_level = wp_field(c.g[S1][0], 4);
        c.lwc1(0, rhs_level);
        c.cvt_s_w(0, 0);
        let self_level = wp_field(c.g[S0][0], 8);
        c.lwc1(1, self_level);
        c.cvt_s_w(1, 1);
        let self_rate = wp_field(c.g[S0][0], 0x10);
        c.lwc1(3, self_rate);
        c.sub_s(2, 2, 20);
        c.g[V1][0] = Addr(wp_field(c.g[S0][0], 4)).lw(c);
        c.mul_s(0, 20, 0);
        c.mul_s(1, 2, 1);
        c.mul_s(2, 2, 3);
        c.add_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        let self_level = wp_field(c.g[S0][0], 8);
        c.swc1(1, self_level);
        c.g[V0][0] = Addr(wp_field(c.g[S1][0], 4)).lw(c);
        let last_level = c.g[V0][0];
        Addr(wp_field(c.g[S0][0], 0xc)).sw(c, last_level);
        let rhs_rate = wp_field(c.g[S1][0], 8);
        c.lwc1(0, rhs_rate);
        c.mul_s(20, 20, 0);
        c.add_s(20, 20, 2);
        let self_rate = wp_field(c.g[S0][0], 0x10);
        c.swc1(20, self_rate);
        let rhs_rate = wp_field(c.g[S1][0], 8);
        c.lwc1(0, rhs_rate);
        let last_rate = wp_field(c.g[S0][0], 0x14);
        c.swc1(0, last_rate);
        let this = c.g[S0][0];
        wp_blend_call(c, this, 0x002b_da0c)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}

/// One replay-frame transfer of a state function: `(off, size, ret)` moves
/// `size` bytes at `self + off` (`off` 0 is `self` itself) through the frame's
/// vtable, and `ret` is the address the call returns to.
/// The shared body of the eight `tWPIGD_*_LoadState` and `SaveState` functions
/// below: `s0` = the replay frame, `s1` = the object, one virtual call per row.
pub(crate) fn wpigd_field_transfers(
    c: &mut Cpu,
    slot_off: u32,
    cycles: u64,
    fields: &[(u32, u64, u32)],
) -> Flow {
    c.cycles += cycles;
    let frame = Frame::open(c, 0x30);
    c.g[A2][0] = fields[0].1;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    for &(off, size, ret_to) in fields {
        c.g[A1][0] = if off == 0 {
            c.g[S1][0]
        } else {
            sx((c.g[S1][0] as u32).wrapping_add(off))
        };
        c.g[A2][0] = size;
        replay_frame_call(c, slot_off, ret_to)?;
    }
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `tWPInGameData_MusicTrigger_readFromReplayFrame` (0x002b_e2d8): reads the trigger's type, travel distance, value and travel distance again.
pub fn twpigd_music_trigger_load_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_MusicTrigger_LoadState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
        (0x8, 4, 0x002b_e30c),
        (0x0, 4, 0x002b_e328),
        (0x10, 4, 0x002b_e344),
        (0x0, 4, 0x002b_e360),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_READ_SLOT, 39, FIELDS)
    });
}

/// `tWPInGameData_Fog_readFromReplayFrame` (0x002b_e518): reads the fog density, near and far distances and the RGB colour, each followed by the base word.
pub fn twpigd_fog_load_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_Fog_LoadState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
        (0x8, 4, 0x002b_e54c),
        (0x0, 4, 0x002b_e568),
        (0x10, 4, 0x002b_e584),
        (0x0, 4, 0x002b_e5a0),
        (0x18, 4, 0x002b_e5bc),
        (0x0, 4, 0x002b_e5d8),
        (0x20, 4, 0x002b_e5f4),
        (0x0, 4, 0x002b_e610),
        (0x28, 4, 0x002b_e62c),
        (0x0, 4, 0x002b_e648),
        (0x30, 4, 0x002b_e664),
        (0x0, 4, 0x002b_e680),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_READ_SLOT, 95, FIELDS)
    });
}

/// `tWPInGameData_LightGlow_readFromReplayFrame` (0x002b_e698): reads the light glow's cutoff, scale, copy/source/frame intensities and blend values.
pub fn twpigd_light_glow_load_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_LightGlow_LoadState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
        (0x8, 4, 0x002b_e6cc),
        (0x0, 4, 0x002b_e6e8),
        (0x10, 4, 0x002b_e704),
        (0x0, 4, 0x002b_e720),
        (0x18, 4, 0x002b_e73c),
        (0x0, 4, 0x002b_e758),
        (0x20, 4, 0x002b_e774),
        (0x0, 4, 0x002b_e790),
        (0x28, 4, 0x002b_e7ac),
        (0x0, 4, 0x002b_e7c8),
        (0x30, 4, 0x002b_e7e4),
        (0x0, 4, 0x002b_e800),
        (0x38, 4, 0x002b_e81c),
        (0x0, 4, 0x002b_e838),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_READ_SLOT, 109, FIELDS)
    });
}

/// `tWPInGameData_ScreenTint_readFromReplayFrame` (0x002b_e850): reads the screen tint's scale and offset for R, G and B.
pub fn twpigd_screen_tint_load_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_ScreenTint_LoadState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
        (0x8, 4, 0x002b_e884),
        (0x0, 4, 0x002b_e8a0),
        (0x10, 4, 0x002b_e8bc),
        (0x0, 4, 0x002b_e8d8),
        (0x18, 4, 0x002b_e8f4),
        (0x0, 4, 0x002b_e910),
        (0x20, 4, 0x002b_e92c),
        (0x0, 4, 0x002b_e948),
        (0x28, 4, 0x002b_e964),
        (0x0, 4, 0x002b_e980),
        (0x30, 4, 0x002b_e99c),
        (0x0, 4, 0x002b_e9b8),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_READ_SLOT, 95, FIELDS)
    });
}

/// `tWPInGameData_Lighting_readFromReplayFrame` (0x002b_ecc8): reads the four 8-byte irradiance names and the intensity offsets.
pub fn twpigd_lighting_load_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_Lighting_LoadState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
        (0x8, 8, 0x002b_ecfc),
        (0x0, 4, 0x002b_ed18),
        (0x18, 8, 0x002b_ed34),
        (0x0, 4, 0x002b_ed50),
        (0x28, 8, 0x002b_ed6c),
        (0x0, 4, 0x002b_ed88),
        (0x38, 8, 0x002b_eda4),
        (0x0, 4, 0x002b_edc0),
        (0x48, 4, 0x002b_eddc),
        (0x0, 4, 0x002b_edf8),
        (0x50, 4, 0x002b_ee14),
        (0x0, 4, 0x002b_ee30),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_READ_SLOT, 95, FIELDS)
    });
}

/// `tWPInGameData_Danger_readFromReplayFrame` (0x002b_f2a0): reads the danger level, travel distance and point rate.
pub fn twpigd_danger_load_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_Danger_LoadState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
        (0x8, 4, 0x002b_f2d4),
        (0x0, 4, 0x002b_f2f0),
        (0x10, 4, 0x002b_f30c),
        (0x0, 4, 0x002b_f328),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_READ_SLOT, 39, FIELDS)
    });
}

/// `tWPInGameData_MusicTrigger_writeToReplayFrame` (0x002b_f340): writes the trigger's type, travel distance, value and travel distance again.
pub fn twpigd_music_trigger_save_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_MusicTrigger_SaveState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
        (0x8, 4, 0x002b_f374),
        (0x0, 4, 0x002b_f390),
        (0x10, 4, 0x002b_f3ac),
        (0x0, 4, 0x002b_f3c8),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_WRITE_SLOT, 39, FIELDS)
    });
}

/// `tWPInGameData_Fog_writeToReplayFrame` (0x002b_f580): writes the fog density, near and far distances and the RGB colour, each followed by the base word.
pub fn twpigd_fog_save_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_Fog_SaveState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
        (0x8, 4, 0x002b_f5b4),
        (0x0, 4, 0x002b_f5d0),
        (0x10, 4, 0x002b_f5ec),
        (0x0, 4, 0x002b_f608),
        (0x18, 4, 0x002b_f624),
        (0x0, 4, 0x002b_f640),
        (0x20, 4, 0x002b_f65c),
        (0x0, 4, 0x002b_f678),
        (0x28, 4, 0x002b_f694),
        (0x0, 4, 0x002b_f6b0),
        (0x30, 4, 0x002b_f6cc),
        (0x0, 4, 0x002b_f6e8),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_WRITE_SLOT, 95, FIELDS)
    });
}

/// One `frame->write(self + off, size)` of a replay save: the call through the
/// frame's write slot (the `this` adjustment at +8 of its vtable, the function
/// at +0xc), returning to `ret_to`. Offset 0 passes the object pointer itself.
pub(crate) fn replay_write(c: &mut Cpu, off: u32, size: u64, ret_to: u32) -> Flow {
    c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
    let vtbl = c.g[V1][0] as u32;
    c.g[A1][0] = if off == 0 {
        c.g[S1][0]
    } else {
        sx((c.g[S1][0] as u32).wrapping_add(off))
    };
    c.g[A2][0] = size;
    c.g[A0][0] = Addr(vtbl.wrapping_add(0x8)).lh(c);
    c.g[V0][0] = Addr(vtbl.wrapping_add(0xc)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_to)
}

/// The save-state body shared by the four `tWPInGameData_*::writeToReplayFrame`
/// functions: `entry_cycles` charged at entry, then one [`replay_write`] per
/// row `(field offset, size, return address)`. The rows alternate a field with
/// the object's first word (offset 0), which the C++ writes after each field.
pub(crate) fn write_replay_frame(
    c: &mut Cpu,
    entry_cycles: u64,
    writes: &[(u32, u64, u32)],
) -> Flow {
    c.cycles += entry_cycles;
    let frame = Frame::open(c, 0x30);
    c.g[A2][0] = 0x4;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    for &(off, size, ret_to) in writes {
        replay_write(c, off, size, ret_to)?;
    }
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `tWPInGameData_LightGlow::writeToReplayFrame` (0x2bf700): the seven light-glow
/// words (cutoff, cutoff scale, copy/source/frame intensity, blend 2 and 3).
pub fn light_glow_save_state(c: &mut Cpu) {
    guest(c, |c| write_replay_frame(c, 109, LIGHT_GLOW_WRITES))
}

pub(crate) const LIGHT_GLOW_WRITES: &[(u32, u64, u32)] = &[
    (0x8, 0x4, 0x002b_f734), // m_LightGlowCutoff
    (0, 0x4, 0x002b_f750),
    (0x10, 0x4, 0x002b_f76c), // m_LightGlowCutoffScale
    (0, 0x4, 0x002b_f788),
    (0x18, 0x4, 0x002b_f7a4), // m_LightGlowCopyIntensity
    (0, 0x4, 0x002b_f7c0),
    (0x20, 0x4, 0x002b_f7dc), // m_LightGlowSourceIntensity
    (0, 0x4, 0x002b_f7f8),
    (0x28, 0x4, 0x002b_f814), // m_LightGlowFrameIntensity
    (0, 0x4, 0x002b_f830),
    (0x30, 0x4, 0x002b_f84c), // m_LightGlowBlend2
    (0, 0x4, 0x002b_f868),
    (0x38, 0x4, 0x002b_f884), // m_LightGlowBlend3
    (0, 0x4, 0x002b_f8a0),
];

/// `tWPInGameData_ScreenTint::writeToReplayFrame` (0x2bf8b8): the scale and offset
/// of the red, green and blue screen tint.
pub fn screen_tint_save_state(c: &mut Cpu) {
    guest(c, |c| write_replay_frame(c, 95, SCREEN_TINT_WRITES))
}

pub(crate) const SCREEN_TINT_WRITES: &[(u32, u64, u32)] = &[
    (0x8, 0x4, 0x002b_f8ec), // m_ScreenTintScaleR
    (0, 0x4, 0x002b_f908),
    (0x10, 0x4, 0x002b_f924), // m_ScreenTintScaleG
    (0, 0x4, 0x002b_f940),
    (0x18, 0x4, 0x002b_f95c), // m_ScreenTintScaleB
    (0, 0x4, 0x002b_f978),
    (0x20, 0x4, 0x002b_f994), // m_ScreenTintOffsetR
    (0, 0x4, 0x002b_f9b0),
    (0x28, 0x4, 0x002b_f9cc), // m_ScreenTintOffsetG
    (0, 0x4, 0x002b_f9e8),
    (0x30, 0x4, 0x002b_fa04), // m_ScreenTintOffsetB
    (0, 0x4, 0x002b_fa20),
];

/// `tWPInGameData_Lighting::writeToReplayFrame` (0x2bfd30): the four irradiance
/// texture names (8 bytes each) and the ambient and rim light offsets.
pub fn lighting_save_state(c: &mut Cpu) {
    guest(c, |c| write_replay_frame(c, 95, LIGHTING_WRITES))
}

pub(crate) const LIGHTING_WRITES: &[(u32, u64, u32)] = &[
    (0x8, 0x8, 0x002b_fd64), // m_LightIrradianceName
    (0, 0x4, 0x002b_fd80),
    (0x18, 0x8, 0x002b_fd9c), // m_ShadowIrradianceName
    (0, 0x4, 0x002b_fdb8),
    (0x28, 0x8, 0x002b_fdd4), // m_TunnelIrradianceName
    (0, 0x4, 0x002b_fdf0),
    (0x38, 0x8, 0x002b_fe0c), // m_ObjectIrradianceName
    (0, 0x4, 0x002b_fe28),
    (0x48, 0x4, 0x002b_fe44), // m_AmbientLightIntensityOffset
    (0, 0x4, 0x002b_fe60),
    (0x50, 0x4, 0x002b_fe7c), // m_RimLightIntensityOffset
    (0, 0x4, 0x002b_fe98),
];

/// `tWPInGameData_Danger::writeToReplayFrame` (0x2c0308): the level, the travel
/// distance (offset 0) and the point rate.
pub fn danger_save_state(c: &mut Cpu) {
    guest(c, |c| write_replay_frame(c, 39, DANGER_WRITES))
}

pub(crate) const DANGER_WRITES: &[(u32, u64, u32)] = &[
    (0x8, 0x4, 0x002c_033c),  // m_Level
    (0, 0x4, 0x002c_0358),    // m_wpTravelDistance (the C++ writes it twice)
    (0x10, 0x4, 0x002c_0374), // m_PointRate
    (0, 0x4, 0x002c_0390),
];

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;

    use ssx3_ee as ee;

    #[test]
    fn quad_tree_node_set_leaf_matches() {
        check(
            ssx3_ee::s08::f_002c16a8,
            quad_tree_node_set_leaf,
            50,
            &|c, seed| {
                let mut r = Rng(0x16a8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn quad_tree_node_set_children_matches() {
        check(
            ssx3_ee::s08::f_002c16b8,
            quad_tree_node_set_children,
            200,
            &|c, seed| {
                let mut r = Rng(0x16b8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for (reg, v) in [
                    (A1, r.next_u32()),
                    (A2, r.next_u32()),
                    (A3, r.next_u32()),
                    (T0, r.next_u32()),
                ] {
                    c.g[reg][0] = ((v as u64) << 32) | v as u64;
                }
                for k in 0..4u32 {
                    c.bus.write16(DATA + 2 * k, r.next_u32() as u16);
                }
            },
        );
    }

    #[test]
    fn quad_tree_init_free_list_matches() {
        check(
            ssx3_ee::s08::f_002c17c0,
            quad_tree_init_free_list,
            50,
            &|c, seed| {
                let mut r = Rng(0x17c0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + (seed % 8)) as u64;
            },
        );
    }

    #[test]
    fn quad_tree_free_children_matches() {
        check(
            ssx3_ee::s08::f_002c18e8,
            quad_tree_free_children,
            200,
            &|c, seed| {
                let mut r = Rng(0x18e8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for reg in [A1, A2, A3, T0] {
                    c.g[reg][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                }
                c.bus.write32(DATA + 0x20, DATA + 0x400);
                c.bus.write32(DATA + 0x24, DATA + 0x600);
                c.bus.write32(DATA + 0xc, r.next_u32());
                c.bus.write32(DATA + 0x400, r.next_u32());
                c.bus.write32(DATA + 0x600, r.next_u32() % 8);
            },
        );
    }

    // ------------------------------------------------------- menu

    #[test]
    fn world_painter_query_base_dtor_matches() {
        dtor_case(
            ee::s08::f_002c1460__generated,
            world_painter_query_base_dtor,
            0x2c1460,
        );
    }

    #[test]
    fn twpigd_sun_get_elevation_matches() {
        check(
            ssx3_ee::s08::f_002c1560__generated,
            super::twpigd_sun_get_elevation,
            100,
            &|c, seed| {
                let mut r = Rng(0x2c15_60 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn twpigd_sun_get_azimuth_matches() {
        check(
            ssx3_ee::s08::f_002c1568__generated,
            super::twpigd_sun_get_azimuth,
            100,
            &|c, seed| {
                let mut r = Rng(0x2c15_68 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;

    pub(crate) use crate::game::ui::uiengine::tests::oc726_hook;
    pub(crate) use crate::game::ui::uiengine::tests::CTRL;
    pub(crate) use crate::game::ui::uiengine::tests::CTRL_MAGIC;
    pub(crate) use crate::game::ui::uiengine::tests::SEEDS_10 as SEEDS;

    #[test]
    fn world_painter_query_create_matches() {
        oc726_hook();
        check(
            ee::s08::f_002c0a58__generated,
            world_painter_query_create,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 726, 1);
                let arg = fakes::ptr(&mut r);
                // The constructor skips its type switch when byte 1 of the word
                // is at least 0xe (it then makes no calls of its own).
                let word = ((0x0e + (r.next_u32() & 0xfff)) << 8) | (r.next_u32() & 0xff);
                c.bus.write32(arg, word);
                c.g[A1][0] = arg as u64;
                c.pc = 0x002c_0a58;
                c.bus.write32(CTRL, CTRL_MAGIC);
            },
        );
    }

    use crate::difftest::check_paths;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::worldpainter::quadtree::tests::oc1052_machine;

    /// Pool offsets (relative to gp) read by the colours and final FPRs.
    pub(crate) const OC1040_POOL: &[i32] = &[
        -0x416c, -0x4170, -0x4174, -0x4178, -0x417c, -0x4180, -0x4184, -0x4188, -0x418c, -0x4190,
        -0x4194, -0x4198, -0x419c, -0x41a0, -0x41a4, -0x41a8, -0x41ac, -0x41b0, -0x41b4, -0x41b8,
        -0x41bc, -0x41c0, -0x41c4, -0x41c8, -0x41cc, -0x41d0, -0x41d4, -0x41d8, -0x41dc, -0x41e0,
        -0x41e4, -0x41e8, -0x41ec, -0x41f0, -0x41f4, -0x41f8, -0x41fc, -0x4200, -0x4204, -0x4208,
        -0x420c, -0x4210, -0x4214, -0x4218, -0x421c, -0x4220, -0x4224, -0x4228, -0x422c, -0x4230,
        -0x4234, -0x4238, -0x423c, -0x4240, -0x4244, -0x4248, -0x424c, -0x4250, -0x4254, -0x4258,
        -0x425c, -0x4260, -0x4264, -0x4268, -0x426c, -0x4270, -0x4274, -0x4278, -0x427c,
    ];

    /// Three paths: wrong priority (23 cycles), `initialize == 0` (25) and
    /// the full init (569), with random bit patterns in the constant pool.
    #[test]
    fn world_painter_query_static_init_destroy_matches() {
        check_paths(
            ee::s08::f_002c0b70__generated,
            world_painter_query_static_init_destroy,
            90,
            &[23, 25, 569],
            &|c, seed| {
                let mut r = Rng(0x6c0b_7000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                for off in OC1040_POOL {
                    let v = r.next_u32();
                    c.bus.write32(GAME_GP.wrapping_add(*off as u32), v);
                }
                let (a0, a1) = match seed % 3 {
                    0 => (1 + (r.next_u32() as u64 & 0xff), r.next_u32() as u64),
                    1 => (0, 0xffff),
                    _ => (1 + (r.next_u32() as u64 & 0xff), 0xffff),
                };
                c.g[A0][0] = a0;
                c.g[A1][0] = a1;
                c.g[RA][0] = crate::difftest::RA as u64;
            },
        );
    }

    #[test]
    fn world_painter_query_global_ctors_matches() {
        check(
            ee::s08::f_002c1688__generated,
            world_painter_query_global_ctors,
            200,
            &|c, seed| {
                oc1052_machine(c, seed, 0x1052_0003);
            },
        );
    }

    use crate::difftest::fakes::covered;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fill;

    /// `tWPIGD_Base_GetMusicTrigger08` (0x002b_c500) against the generated body.
    #[test]
    fn t_wpigd_base_get_music_trigger08_002bc500_matches() {
        check(
            ee::s08::f_002bc500__generated,
            t_wpigd_base_get_music_trigger08_002bc500,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc500, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetMusicTrigger10` (0x002b_c508) against the generated body.
    #[test]
    fn t_wpigd_base_get_music_trigger10_002bc508_matches() {
        check(
            ee::s08::f_002bc508__generated,
            t_wpigd_base_get_music_trigger10_002bc508,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc508, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetMix08` (0x002b_c510) against the generated body.
    #[test]
    fn t_wpigd_base_get_mix08_002bc510_matches() {
        check(
            ee::s08::f_002bc510__generated,
            t_wpigd_base_get_mix08_002bc510,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc510, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetAmbience08` (0x002b_c518) against the generated body.
    #[test]
    fn t_wpigd_base_get_ambience08_002bc518_matches() {
        check(
            ee::s08::f_002bc518__generated,
            t_wpigd_base_get_ambience08_002bc518,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc518, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSpeech08` (0x002b_c520) against the generated body.
    #[test]
    fn t_wpigd_base_get_speech08_002bc520_matches() {
        check(
            ee::s08::f_002bc520__generated,
            t_wpigd_base_get_speech08_002bc520,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc520, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetCamera08` (0x002b_c528) against the generated body.
    #[test]
    fn t_wpigd_base_get_camera08_002bc528_matches() {
        check(
            ee::s08::f_002bc528__generated,
            t_wpigd_base_get_camera08_002bc528,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc528, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetFog08` (0x002b_c530) against the generated body.
    #[test]
    fn t_wpigd_base_get_fog08_002bc530_matches() {
        check(
            ee::s08::f_002bc530__generated,
            t_wpigd_base_get_fog08_002bc530,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc530, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetFog10` (0x002b_c538) against the generated body.
    #[test]
    fn t_wpigd_base_get_fog10_002bc538_matches() {
        check(
            ee::s08::f_002bc538__generated,
            t_wpigd_base_get_fog10_002bc538,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc538, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetFog18` (0x002b_c540) against the generated body.
    #[test]
    fn t_wpigd_base_get_fog18_002bc540_matches() {
        check(
            ee::s08::f_002bc540__generated,
            t_wpigd_base_get_fog18_002bc540,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc540, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetFog20` (0x002b_c548) against the generated body.
    #[test]
    fn t_wpigd_base_get_fog20_002bc548_matches() {
        check(
            ee::s08::f_002bc548__generated,
            t_wpigd_base_get_fog20_002bc548,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc548, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetFog28` (0x002b_c550) against the generated body.
    #[test]
    fn t_wpigd_base_get_fog28_002bc550_matches() {
        check(
            ee::s08::f_002bc550__generated,
            t_wpigd_base_get_fog28_002bc550,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc550, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetFog30` (0x002b_c558) against the generated body.
    #[test]
    fn t_wpigd_base_get_fog30_002bc558_matches() {
        check(
            ee::s08::f_002bc558__generated,
            t_wpigd_base_get_fog30_002bc558,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc558, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLightGlow08` (0x002b_c560) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow08_002bc560_matches() {
        check(
            ee::s08::f_002bc560__generated,
            t_wpigd_base_get_light_glow08_002bc560,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc560, 0);
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

    /// `tWPIGD_Base_GetLightGlow10` (0x002b_c568) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow10_002bc568_matches() {
        check(
            ee::s08::f_002bc568__generated,
            t_wpigd_base_get_light_glow10_002bc568,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc568, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLightGlow18` (0x002b_c570) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow18_002bc570_matches() {
        check(
            ee::s08::f_002bc570__generated,
            t_wpigd_base_get_light_glow18_002bc570,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc570, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLightGlow20` (0x002b_c578) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow20_002bc578_matches() {
        check(
            ee::s08::f_002bc578__generated,
            t_wpigd_base_get_light_glow20_002bc578,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc578, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLightGlow28` (0x002b_c580) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow28_002bc580_matches() {
        check(
            ee::s08::f_002bc580__generated,
            t_wpigd_base_get_light_glow28_002bc580,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc580, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLightGlow30` (0x002b_c588) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow30_002bc588_matches() {
        check(
            ee::s08::f_002bc588__generated,
            t_wpigd_base_get_light_glow30_002bc588,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc588, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLightGlow38` (0x002b_c590) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow38_002bc590_matches() {
        check(
            ee::s08::f_002bc590__generated,
            t_wpigd_base_get_light_glow38_002bc590,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc590, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetScreenTintRed` (0x002b_c598) against the generated body.
    #[test]
    fn t_wpigd_base_get_screen_tint_red_002bc598_matches() {
        check(
            ee::s08::f_002bc598__generated,
            t_wpigd_base_get_screen_tint_red_002bc598,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc598, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetScreenTintGreen` (0x002b_c5a0) against the generated body.
    #[test]
    fn t_wpigd_base_get_screen_tint_green_002bc5a0_matches() {
        check(
            ee::s08::f_002bc5a0__generated,
            t_wpigd_base_get_screen_tint_green_002bc5a0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5a0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetScreenTintBlue` (0x002b_c5a8) against the generated body.
    #[test]
    fn t_wpigd_base_get_screen_tint_blue_002bc5a8_matches() {
        check(
            ee::s08::f_002bc5a8__generated,
            t_wpigd_base_get_screen_tint_blue_002bc5a8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5a8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetScreenFillRed` (0x002b_c5b0) against the generated body.
    #[test]
    fn t_wpigd_base_get_screen_fill_red_002bc5b0_matches() {
        check(
            ee::s08::f_002bc5b0__generated,
            t_wpigd_base_get_screen_fill_red_002bc5b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetScreenFillGreen` (0x002b_c5b8) against the generated body.
    #[test]
    fn t_wpigd_base_get_screen_fill_green_002bc5b8_matches() {
        check(
            ee::s08::f_002bc5b8__generated,
            t_wpigd_base_get_screen_fill_green_002bc5b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetScreenFillBlue` (0x002b_c5c0) against the generated body.
    #[test]
    fn t_wpigd_base_get_screen_fill_blue_002bc5c0_matches() {
        check(
            ee::s08::f_002bc5c0__generated,
            t_wpigd_base_get_screen_fill_blue_002bc5c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSkyBox08` (0x002b_c5c8) against the generated body.
    #[test]
    fn t_wpigd_base_get_sky_box08_002bc5c8_matches() {
        check(
            ee::s08::f_002bc5c8__generated,
            t_wpigd_base_get_sky_box08_002bc5c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunElevation` (0x002b_c5d0) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_elevation_002bc5d0_matches() {
        check(
            ee::s08::f_002bc5d0__generated,
            t_wpigd_base_get_sun_elevation_002bc5d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunAzimuth` (0x002b_c5d8) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_azimuth_002bc5d8_matches() {
        check(
            ee::s08::f_002bc5d8__generated,
            t_wpigd_base_get_sun_azimuth_002bc5d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunRed` (0x002b_c5e0) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_red_002bc5e0_matches() {
        check(
            ee::s08::f_002bc5e0__generated,
            t_wpigd_base_get_sun_red_002bc5e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunGreen` (0x002b_c5e8) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_green_002bc5e8_matches() {
        check(
            ee::s08::f_002bc5e8__generated,
            t_wpigd_base_get_sun_green_002bc5e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunBlue` (0x002b_c5f0) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_blue_002bc5f0_matches() {
        check(
            ee::s08::f_002bc5f0__generated,
            t_wpigd_base_get_sun_blue_002bc5f0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5f0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunIntensity` (0x002b_c5f8) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_intensity_002bc5f8_matches() {
        check(
            ee::s08::f_002bc5f8__generated,
            t_wpigd_base_get_sun_intensity_002bc5f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunTexture` (0x002b_c600) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_texture_002bc600_matches() {
        check(
            ee::s08::f_002bc600__generated,
            t_wpigd_base_get_sun_texture_002bc600,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc600, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunFlareIntensity` (0x002b_c608) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_flare_intensity_002bc608_matches() {
        check(
            ee::s08::f_002bc608__generated,
            t_wpigd_base_get_sun_flare_intensity_002bc608,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc608, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunSize` (0x002b_c610) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_size_002bc610_matches() {
        check(
            ee::s08::f_002bc610__generated,
            t_wpigd_base_get_sun_size_002bc610,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc610, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSurface08` (0x002b_c618) against the generated body.
    #[test]
    fn t_wpigd_base_get_surface08_002bc618_matches() {
        check(
            ee::s08::f_002bc618__generated,
            t_wpigd_base_get_surface08_002bc618,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc618, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetIrradianceName0` (0x002b_c620) against the generated body.
    #[test]
    fn t_wpigd_base_get_irradiance_name0_002bc620_matches() {
        check(
            ee::s08::f_002bc620__generated,
            t_wpigd_base_get_irradiance_name0_002bc620,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc620, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetIrradianceName1` (0x002b_c628) against the generated body.
    #[test]
    fn t_wpigd_base_get_irradiance_name1_002bc628_matches() {
        check(
            ee::s08::f_002bc628__generated,
            t_wpigd_base_get_irradiance_name1_002bc628,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc628, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetIrradianceName2` (0x002b_c630) against the generated body.
    #[test]
    fn t_wpigd_base_get_irradiance_name2_002bc630_matches() {
        check(
            ee::s08::f_002bc630__generated,
            t_wpigd_base_get_irradiance_name2_002bc630,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc630, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetIrradianceName3` (0x002b_c638) against the generated body.
    #[test]
    fn t_wpigd_base_get_irradiance_name3_002bc638_matches() {
        check(
            ee::s08::f_002bc638__generated,
            t_wpigd_base_get_irradiance_name3_002bc638,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc638, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLighting48` (0x002b_c640) against the generated body.
    #[test]
    fn t_wpigd_base_get_lighting48_002bc640_matches() {
        check(
            ee::s08::f_002bc640__generated,
            t_wpigd_base_get_lighting48_002bc640,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc640, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLighting50` (0x002b_c648) against the generated body.
    #[test]
    fn t_wpigd_base_get_lighting50_002bc648_matches() {
        check(
            ee::s08::f_002bc648__generated,
            t_wpigd_base_get_lighting50_002bc648,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc648, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather08` (0x002b_c650) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather08_002bc650_matches() {
        check(
            ee::s08::f_002bc650__generated,
            t_wpigd_base_get_weather08_002bc650,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc650, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather10` (0x002b_c658) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather10_002bc658_matches() {
        check(
            ee::s08::f_002bc658__generated,
            t_wpigd_base_get_weather10_002bc658,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc658, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather18` (0x002b_c660) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather18_002bc660_matches() {
        check(
            ee::s08::f_002bc660__generated,
            t_wpigd_base_get_weather18_002bc660,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc660, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather20` (0x002b_c668) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather20_002bc668_matches() {
        check(
            ee::s08::f_002bc668__generated,
            t_wpigd_base_get_weather20_002bc668,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc668, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather28` (0x002b_c670) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather28_002bc670_matches() {
        check(
            ee::s08::f_002bc670__generated,
            t_wpigd_base_get_weather28_002bc670,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc670, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather30` (0x002b_c678) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather30_002bc678_matches() {
        check(
            ee::s08::f_002bc678__generated,
            t_wpigd_base_get_weather30_002bc678,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc678, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather38` (0x002b_c680) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather38_002bc680_matches() {
        check(
            ee::s08::f_002bc680__generated,
            t_wpigd_base_get_weather38_002bc680,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc680, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather40` (0x002b_c688) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather40_002bc688_matches() {
        check(
            ee::s08::f_002bc688__generated,
            t_wpigd_base_get_weather40_002bc688,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc688, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather48` (0x002b_c690) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather48_002bc690_matches() {
        check(
            ee::s08::f_002bc690__generated,
            t_wpigd_base_get_weather48_002bc690,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc690, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather50` (0x002b_c698) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather50_002bc698_matches() {
        check(
            ee::s08::f_002bc698__generated,
            t_wpigd_base_get_weather50_002bc698,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc698, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather58` (0x002b_c6a0) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather58_002bc6a0_matches() {
        check(
            ee::s08::f_002bc6a0__generated,
            t_wpigd_base_get_weather58_002bc6a0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6a0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather60` (0x002b_c6a8) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather60_002bc6a8_matches() {
        check(
            ee::s08::f_002bc6a8__generated,
            t_wpigd_base_get_weather60_002bc6a8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6a8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather68` (0x002b_c6b0) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather68_002bc6b0_matches() {
        check(
            ee::s08::f_002bc6b0__generated,
            t_wpigd_base_get_weather68_002bc6b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather70` (0x002b_c6b8) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather70_002bc6b8_matches() {
        check(
            ee::s08::f_002bc6b8__generated,
            t_wpigd_base_get_weather70_002bc6b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather78` (0x002b_c6c0) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather78_002bc6c0_matches() {
        check(
            ee::s08::f_002bc6c0__generated,
            t_wpigd_base_get_weather78_002bc6c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather80` (0x002b_c6c8) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather80_002bc6c8_matches() {
        check(
            ee::s08::f_002bc6c8__generated,
            t_wpigd_base_get_weather80_002bc6c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather88` (0x002b_c6d0) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather88_002bc6d0_matches() {
        check(
            ee::s08::f_002bc6d0__generated,
            t_wpigd_base_get_weather88_002bc6d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather90` (0x002b_c6d8) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather90_002bc6d8_matches() {
        check(
            ee::s08::f_002bc6d8__generated,
            t_wpigd_base_get_weather90_002bc6d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather98` (0x002b_c6e0) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather98_002bc6e0_matches() {
        check(
            ee::s08::f_002bc6e0__generated,
            t_wpigd_base_get_weather98_002bc6e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetDanger08` (0x002b_c6e8) against the generated body.
    #[test]
    fn t_wpigd_base_get_danger08_002bc6e8_matches() {
        check(
            ee::s08::f_002bc6e8__generated,
            t_wpigd_base_get_danger08_002bc6e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetDanger10` (0x002b_c6f0) against the generated body.
    #[test]
    fn t_wpigd_base_get_danger10_002bc6f0_matches() {
        check(
            ee::s08::f_002bc6f0__generated,
            t_wpigd_base_get_danger10_002bc6f0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6f0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_MusicTrigger_v67` (0x002b_da28) against the generated body.
    #[test]
    fn t_wpigd_music_trigger_v67_002bda28_matches() {
        check(
            ee::s08::f_002bda28__generated,
            t_wpigd_music_trigger_v67_002bda28,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda28, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Ambience_v67` (0x002b_da30) against the generated body.
    #[test]
    fn t_wpigd_ambience_v67_002bda30_matches() {
        check(
            ee::s08::f_002bda30__generated,
            t_wpigd_ambience_v67_002bda30,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda30, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Camera_v67` (0x002b_da38) against the generated body.
    #[test]
    fn t_wpigd_camera_v67_002bda38_matches() {
        check(
            ee::s08::f_002bda38__generated,
            t_wpigd_camera_v67_002bda38,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda38, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Danger_v67` (0x002b_da40) against the generated body.
    #[test]
    fn t_wpigd_danger_v67_002bda40_matches() {
        check(
            ee::s08::f_002bda40__generated,
            t_wpigd_danger_v67_002bda40,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda40, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_v67` (0x002b_da48) against the generated body.
    #[test]
    fn t_wpigd_fog_v67_002bda48_matches() {
        check(
            ee::s08::f_002bda48__generated,
            t_wpigd_fog_v67_002bda48,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda48, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_v67` (0x002b_da50) against the generated body.
    #[test]
    fn t_wpigd_light_glow_v67_002bda50_matches() {
        check(
            ee::s08::f_002bda50__generated,
            t_wpigd_light_glow_v67_002bda50,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda50, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_v67` (0x002b_da58) against the generated body.
    #[test]
    fn t_wpigd_lighting_v67_002bda58_matches() {
        check(
            ee::s08::f_002bda58__generated,
            t_wpigd_lighting_v67_002bda58,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda58, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Mix_v67` (0x002b_da60) against the generated body.
    #[test]
    fn t_wpigd_mix_v67_002bda60_matches() {
        check(
            ee::s08::f_002bda60__generated,
            t_wpigd_mix_v67_002bda60,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda60, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_v67` (0x002b_da68) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_v67_002bda68_matches() {
        check(
            ee::s08::f_002bda68__generated,
            t_wpigd_screen_tint_v67_002bda68,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda68, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_SkyBox_v67` (0x002b_da70) against the generated body.
    #[test]
    fn t_wpigd_sky_box_v67_002bda70_matches() {
        check(
            ee::s08::f_002bda70__generated,
            t_wpigd_sky_box_v67_002bda70,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda70, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Speech_v67` (0x002b_da78) against the generated body.
    #[test]
    fn t_wpigd_speech_v67_002bda78_matches() {
        check(
            ee::s08::f_002bda78__generated,
            t_wpigd_speech_v67_002bda78,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda78, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_v67` (0x002b_da80) against the generated body.
    #[test]
    fn t_wpigd_sun_v67_002bda80_matches() {
        check(
            ee::s08::f_002bda80__generated,
            t_wpigd_sun_v67_002bda80,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda80, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Surface_v67` (0x002b_da88) against the generated body.
    #[test]
    fn t_wpigd_surface_v67_002bda88_matches() {
        check(
            ee::s08::f_002bda88__generated,
            t_wpigd_surface_v67_002bda88,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda88, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_v67` (0x002b_da90) against the generated body.
    #[test]
    fn t_wpigd_weather_v67_002bda90_matches() {
        check(
            ee::s08::f_002bda90__generated,
            t_wpigd_weather_v67_002bda90,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda90, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Mix_IsEqual` (0x002b_dac8) against the generated body.
    #[test]
    fn t_wpigd_mix_is_equal_002bdac8_matches() {
        check(
            ee::s08::f_002bdac8__generated,
            t_wpigd_mix_is_equal_002bdac8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2bdac8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                leaf_fill(c, &mut r, 0x0080_5000, 0x2000, false, 0x0080_5000);
                c.g[A1][0] = sx(0x0080_5000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Ambience_IsEqual` (0x002b_dae0) against the generated body.
    #[test]
    fn t_wpigd_ambience_is_equal_002bdae0_matches() {
        check(
            ee::s08::f_002bdae0__generated,
            t_wpigd_ambience_is_equal_002bdae0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2bdae0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                leaf_fill(c, &mut r, 0x0080_5000, 0x2000, false, 0x0080_5000);
                c.g[A1][0] = sx(0x0080_5000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Speech_IsEqual` (0x002b_daf8) against the generated body.
    #[test]
    fn t_wpigd_speech_is_equal_002bdaf8_matches() {
        check(
            ee::s08::f_002bdaf8__generated,
            t_wpigd_speech_is_equal_002bdaf8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2bdaf8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                leaf_fill(c, &mut r, 0x0080_5000, 0x2000, false, 0x0080_5000);
                c.g[A1][0] = sx(0x0080_5000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_MusicTrigger_SetDefaults` (0x002b_e0b0) against the generated body.
    #[test]
    fn t_wpigd_music_trigger_set_defaults_002be0b0_matches() {
        check(
            ee::s08::f_002be0b0__generated,
            t_wpigd_music_trigger_set_defaults_002be0b0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be0b0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Mix_SetDefaults` (0x002b_e0c8) against the generated body.
    #[test]
    fn t_wpigd_mix_set_defaults_002be0c8_matches() {
        check(
            ee::s08::f_002be0c8__generated,
            t_wpigd_mix_set_defaults_002be0c8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be0c8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Ambience_SetDefaults` (0x002b_e0d8) against the generated body.
    #[test]
    fn t_wpigd_ambience_set_defaults_002be0d8_matches() {
        check(
            ee::s08::f_002be0d8__generated,
            t_wpigd_ambience_set_defaults_002be0d8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be0d8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Speech_SetDefaults` (0x002b_e0e8) against the generated body.
    #[test]
    fn t_wpigd_speech_set_defaults_002be0e8_matches() {
        check(
            ee::s08::f_002be0e8__generated,
            t_wpigd_speech_set_defaults_002be0e8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be0e8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Camera_SetDefaults` (0x002b_e0f8) against the generated body.
    #[test]
    fn t_wpigd_camera_set_defaults_002be0f8_matches() {
        check(
            ee::s08::f_002be0f8__generated,
            t_wpigd_camera_set_defaults_002be0f8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be0f8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x6000, false, 0x0080_62b0);
                c.g[A0][0] = sx(0x0080_62b0 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_SetDefaults` (0x002b_e108) against the generated body.
    #[test]
    fn t_wpigd_fog_set_defaults_002be108_matches() {
        check(
            ee::s08::f_002be108__generated,
            t_wpigd_fog_set_defaults_002be108,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be108, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x6000, false, 0x0080_62b0);
                c.g[A0][0] = sx(0x0080_62b0 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_SetDefaults` (0x002b_e140) against the generated body.
    #[test]
    fn t_wpigd_light_glow_set_defaults_002be140_matches() {
        check(
            ee::s08::f_002be140__generated,
            t_wpigd_light_glow_set_defaults_002be140,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be140, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_SetDefaults` (0x002b_e170) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_set_defaults_002be170_matches() {
        check(
            ee::s08::f_002be170__generated,
            t_wpigd_screen_tint_set_defaults_002be170,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be170, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_SkyBox_SetDefaults` (0x002b_e198) against the generated body.
    #[test]
    fn t_wpigd_sky_box_set_defaults_002be198_matches() {
        check(
            ee::s08::f_002be198__generated,
            t_wpigd_sky_box_set_defaults_002be198,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be198, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_SetDefaults` (0x002b_e1a8) against the generated body.
    #[test]
    fn t_wpigd_sun_set_defaults_002be1a8_matches() {
        check(
            ee::s08::f_002be1a8__generated,
            t_wpigd_sun_set_defaults_002be1a8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be1a8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Surface_SetDefaults` (0x002b_e1e0) against the generated body.
    #[test]
    fn t_wpigd_surface_set_defaults_002be1e0_matches() {
        check(
            ee::s08::f_002be1e0__generated,
            t_wpigd_surface_set_defaults_002be1e0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be1e0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Danger_SetDefaults` (0x002b_e2c8) against the generated body.
    #[test]
    fn t_wpigd_danger_set_defaults_002be2c8_matches() {
        check(
            ee::s08::f_002be2c8__generated,
            t_wpigd_danger_set_defaults_002be2c8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be2c8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldPainterQuery_DebugPrintf` (0x002c_0b30) against the generated body.
    #[test]
    fn world_painter_query_debug_printf_matches() {
        check(
            ee::s08::f_002c0b30__generated,
            world_painter_query_debug_printf,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c0b30, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[A1][0] = leaf_val(&mut r);
                c.g[A2][0] = leaf_val(&mut r);
                c.g[A3][0] = leaf_val(&mut r);
                c.g[T0][0] = leaf_val(&mut r);
                c.g[T1][0] = leaf_val(&mut r);
                c.g[T2][0] = leaf_val(&mut r);
                c.g[T3][0] = leaf_val(&mut r);
                c.f[12] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.f[16] = r.edge_f32();
                c.f[18] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_v67` (0x002c_1458) against the generated body.
    #[test]
    fn t_wpigd_base_v67_002c1458_matches() {
        check(
            ee::s08::f_002c1458__generated,
            t_wpigd_base_v67_002c1458,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2c1458, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_MusicTrigger_Get08` (0x002c_1490) against the generated body.
    #[test]
    fn t_wpigd_music_trigger_get08_002c1490_matches() {
        check(
            ee::s08::f_002c1490__generated,
            t_wpigd_music_trigger_get08_002c1490,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1490, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_MusicTrigger_Get10` (0x002c_1498) against the generated body.
    #[test]
    fn t_wpigd_music_trigger_get10_002c1498_matches() {
        check(
            ee::s08::f_002c1498__generated,
            t_wpigd_music_trigger_get10_002c1498,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1498, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Mix_Get08` (0x002c_14a0) against the generated body.
    #[test]
    fn t_wpigd_mix_get08_002c14a0_matches() {
        check(
            ee::s08::f_002c14a0__generated,
            t_wpigd_mix_get08_002c14a0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14a0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Ambience_Get08` (0x002c_14a8) against the generated body.
    #[test]
    fn t_wpigd_ambience_get08_002c14a8_matches() {
        check(
            ee::s08::f_002c14a8__generated,
            t_wpigd_ambience_get08_002c14a8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14a8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Speech_Get08` (0x002c_14b0) against the generated body.
    #[test]
    fn t_wpigd_speech_get08_002c14b0_matches() {
        check(
            ee::s08::f_002c14b0__generated,
            t_wpigd_speech_get08_002c14b0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14b0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Camera_Get08` (0x002c_14b8) against the generated body.
    #[test]
    fn t_wpigd_camera_get08_002c14b8_matches() {
        check(
            ee::s08::f_002c14b8__generated,
            t_wpigd_camera_get08_002c14b8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14b8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_Get08` (0x002c_14c0) against the generated body.
    #[test]
    fn t_wpigd_fog_get08_002c14c0_matches() {
        check(
            ee::s08::f_002c14c0__generated,
            t_wpigd_fog_get08_002c14c0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14c0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_Get10` (0x002c_14c8) against the generated body.
    #[test]
    fn t_wpigd_fog_get10_002c14c8_matches() {
        check(
            ee::s08::f_002c14c8__generated,
            t_wpigd_fog_get10_002c14c8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14c8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_Get18` (0x002c_14d0) against the generated body.
    #[test]
    fn t_wpigd_fog_get18_002c14d0_matches() {
        check(
            ee::s08::f_002c14d0__generated,
            t_wpigd_fog_get18_002c14d0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14d0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_Get20` (0x002c_14d8) against the generated body.
    #[test]
    fn t_wpigd_fog_get20_002c14d8_matches() {
        check(
            ee::s08::f_002c14d8__generated,
            t_wpigd_fog_get20_002c14d8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14d8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_Get28` (0x002c_14e0) against the generated body.
    #[test]
    fn t_wpigd_fog_get28_002c14e0_matches() {
        check(
            ee::s08::f_002c14e0__generated,
            t_wpigd_fog_get28_002c14e0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14e0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_Get30` (0x002c_14e8) against the generated body.
    #[test]
    fn t_wpigd_fog_get30_002c14e8_matches() {
        check(
            ee::s08::f_002c14e8__generated,
            t_wpigd_fog_get30_002c14e8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14e8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get08` (0x002c_14f0) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get08_002c14f0_matches() {
        check(
            ee::s08::f_002c14f0__generated,
            t_wpigd_light_glow_get08_002c14f0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14f0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get10` (0x002c_14f8) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get10_002c14f8_matches() {
        check(
            ee::s08::f_002c14f8__generated,
            t_wpigd_light_glow_get10_002c14f8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14f8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get18` (0x002c_1500) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get18_002c1500_matches() {
        check(
            ee::s08::f_002c1500__generated,
            t_wpigd_light_glow_get18_002c1500,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1500, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get20` (0x002c_1508) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get20_002c1508_matches() {
        check(
            ee::s08::f_002c1508__generated,
            t_wpigd_light_glow_get20_002c1508,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1508, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get28` (0x002c_1510) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get28_002c1510_matches() {
        check(
            ee::s08::f_002c1510__generated,
            t_wpigd_light_glow_get28_002c1510,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1510, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get30` (0x002c_1518) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get30_002c1518_matches() {
        check(
            ee::s08::f_002c1518__generated,
            t_wpigd_light_glow_get30_002c1518,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1518, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get38` (0x002c_1520) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get38_002c1520_matches() {
        check(
            ee::s08::f_002c1520__generated,
            t_wpigd_light_glow_get38_002c1520,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1520, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_GetTintRed` (0x002c_1528) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_get_tint_red_002c1528_matches() {
        check(
            ee::s08::f_002c1528__generated,
            t_wpigd_screen_tint_get_tint_red_002c1528,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1528, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_GetTintGreen` (0x002c_1530) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_get_tint_green_002c1530_matches() {
        check(
            ee::s08::f_002c1530__generated,
            t_wpigd_screen_tint_get_tint_green_002c1530,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1530, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_GetTintBlue` (0x002c_1538) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_get_tint_blue_002c1538_matches() {
        check(
            ee::s08::f_002c1538__generated,
            t_wpigd_screen_tint_get_tint_blue_002c1538,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1538, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_GetFillRed` (0x002c_1540) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_get_fill_red_002c1540_matches() {
        check(
            ee::s08::f_002c1540__generated,
            t_wpigd_screen_tint_get_fill_red_002c1540,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1540, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_GetFillGreen` (0x002c_1548) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_get_fill_green_002c1548_matches() {
        check(
            ee::s08::f_002c1548__generated,
            t_wpigd_screen_tint_get_fill_green_002c1548,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1548, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_GetFillBlue` (0x002c_1550) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_get_fill_blue_002c1550_matches() {
        check(
            ee::s08::f_002c1550__generated,
            t_wpigd_screen_tint_get_fill_blue_002c1550,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1550, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_SkyBox_Get08` (0x002c_1558) against the generated body.
    #[test]
    fn t_wpigd_sky_box_get08_002c1558_matches() {
        check(
            ee::s08::f_002c1558__generated,
            t_wpigd_sky_box_get08_002c1558,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1558, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetRed` (0x002c_1570) against the generated body.
    #[test]
    fn t_wpigd_sun_get_red_002c1570_matches() {
        check(
            ee::s08::f_002c1570__generated,
            t_wpigd_sun_get_red_002c1570,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1570, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetGreen` (0x002c_1578) against the generated body.
    #[test]
    fn t_wpigd_sun_get_green_002c1578_matches() {
        check(
            ee::s08::f_002c1578__generated,
            t_wpigd_sun_get_green_002c1578,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1578, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetBlue` (0x002c_1580) against the generated body.
    #[test]
    fn t_wpigd_sun_get_blue_002c1580_matches() {
        check(
            ee::s08::f_002c1580__generated,
            t_wpigd_sun_get_blue_002c1580,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1580, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetIntensity` (0x002c_1588) against the generated body.
    #[test]
    fn t_wpigd_sun_get_intensity_002c1588_matches() {
        check(
            ee::s08::f_002c1588__generated,
            t_wpigd_sun_get_intensity_002c1588,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1588, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetTexture` (0x002c_1590) against the generated body.
    #[test]
    fn t_wpigd_sun_get_texture_002c1590_matches() {
        check(
            ee::s08::f_002c1590__generated,
            t_wpigd_sun_get_texture_002c1590,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1590, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetFlareIntensity` (0x002c_1598) against the generated body.
    #[test]
    fn t_wpigd_sun_get_flare_intensity_002c1598_matches() {
        check(
            ee::s08::f_002c1598__generated,
            t_wpigd_sun_get_flare_intensity_002c1598,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1598, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetSize` (0x002c_15a0) against the generated body.
    #[test]
    fn t_wpigd_sun_get_size_002c15a0_matches() {
        check(
            ee::s08::f_002c15a0__generated,
            t_wpigd_sun_get_size_002c15a0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15a0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Surface_Get08` (0x002c_15a8) against the generated body.
    #[test]
    fn t_wpigd_surface_get08_002c15a8_matches() {
        check(
            ee::s08::f_002c15a8__generated,
            t_wpigd_surface_get08_002c15a8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15a8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_GetIrradianceName0` (0x002c_15b0) against the generated body.
    #[test]
    fn t_wpigd_lighting_get_irradiance_name0_002c15b0_matches() {
        check(
            ee::s08::f_002c15b0__generated,
            t_wpigd_lighting_get_irradiance_name0_002c15b0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15b0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_GetIrradianceName1` (0x002c_15b8) against the generated body.
    #[test]
    fn t_wpigd_lighting_get_irradiance_name1_002c15b8_matches() {
        check(
            ee::s08::f_002c15b8__generated,
            t_wpigd_lighting_get_irradiance_name1_002c15b8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15b8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_Get28` (0x002c_15c0) against the generated body.
    #[test]
    fn t_wpigd_lighting_get28_002c15c0_matches() {
        check(
            ee::s08::f_002c15c0__generated,
            t_wpigd_lighting_get28_002c15c0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15c0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_Get38` (0x002c_15c8) against the generated body.
    #[test]
    fn t_wpigd_lighting_get38_002c15c8_matches() {
        check(
            ee::s08::f_002c15c8__generated,
            t_wpigd_lighting_get38_002c15c8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15c8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_Get48` (0x002c_15d0) against the generated body.
    #[test]
    fn t_wpigd_lighting_get48_002c15d0_matches() {
        check(
            ee::s08::f_002c15d0__generated,
            t_wpigd_lighting_get48_002c15d0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15d0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_Get50` (0x002c_15d8) against the generated body.
    #[test]
    fn t_wpigd_lighting_get50_002c15d8_matches() {
        check(
            ee::s08::f_002c15d8__generated,
            t_wpigd_lighting_get50_002c15d8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15d8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get08` (0x002c_15e0) against the generated body.
    #[test]
    fn t_wpigd_weather_get08_002c15e0_matches() {
        check(
            ee::s08::f_002c15e0__generated,
            t_wpigd_weather_get08_002c15e0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15e0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get10` (0x002c_15e8) against the generated body.
    #[test]
    fn t_wpigd_weather_get10_002c15e8_matches() {
        check(
            ee::s08::f_002c15e8__generated,
            t_wpigd_weather_get10_002c15e8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15e8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get18` (0x002c_15f0) against the generated body.
    #[test]
    fn t_wpigd_weather_get18_002c15f0_matches() {
        check(
            ee::s08::f_002c15f0__generated,
            t_wpigd_weather_get18_002c15f0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15f0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get20` (0x002c_15f8) against the generated body.
    #[test]
    fn t_wpigd_weather_get20_002c15f8_matches() {
        check(
            ee::s08::f_002c15f8__generated,
            t_wpigd_weather_get20_002c15f8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15f8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get28` (0x002c_1600) against the generated body.
    #[test]
    fn t_wpigd_weather_get28_002c1600_matches() {
        check(
            ee::s08::f_002c1600__generated,
            t_wpigd_weather_get28_002c1600,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1600, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get30` (0x002c_1608) against the generated body.
    #[test]
    fn t_wpigd_weather_get30_002c1608_matches() {
        check(
            ee::s08::f_002c1608__generated,
            t_wpigd_weather_get30_002c1608,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1608, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get38` (0x002c_1610) against the generated body.
    #[test]
    fn t_wpigd_weather_get38_002c1610_matches() {
        check(
            ee::s08::f_002c1610__generated,
            t_wpigd_weather_get38_002c1610,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1610, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get40` (0x002c_1618) against the generated body.
    #[test]
    fn t_wpigd_weather_get40_002c1618_matches() {
        check(
            ee::s08::f_002c1618__generated,
            t_wpigd_weather_get40_002c1618,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1618, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get48` (0x002c_1620) against the generated body.
    #[test]
    fn t_wpigd_weather_get48_002c1620_matches() {
        check(
            ee::s08::f_002c1620__generated,
            t_wpigd_weather_get48_002c1620,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1620, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get50` (0x002c_1628) against the generated body.
    #[test]
    fn t_wpigd_weather_get50_002c1628_matches() {
        check(
            ee::s08::f_002c1628__generated,
            t_wpigd_weather_get50_002c1628,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1628, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get58` (0x002c_1630) against the generated body.
    #[test]
    fn t_wpigd_weather_get58_002c1630_matches() {
        check(
            ee::s08::f_002c1630__generated,
            t_wpigd_weather_get58_002c1630,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1630, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get60` (0x002c_1638) against the generated body.
    #[test]
    fn t_wpigd_weather_get60_002c1638_matches() {
        check(
            ee::s08::f_002c1638__generated,
            t_wpigd_weather_get60_002c1638,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1638, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get68` (0x002c_1640) against the generated body.
    #[test]
    fn t_wpigd_weather_get68_002c1640_matches() {
        check(
            ee::s08::f_002c1640__generated,
            t_wpigd_weather_get68_002c1640,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1640, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get70` (0x002c_1648) against the generated body.
    #[test]
    fn t_wpigd_weather_get70_002c1648_matches() {
        check(
            ee::s08::f_002c1648__generated,
            t_wpigd_weather_get70_002c1648,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1648, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get78` (0x002c_1650) against the generated body.
    #[test]
    fn t_wpigd_weather_get78_002c1650_matches() {
        check(
            ee::s08::f_002c1650__generated,
            t_wpigd_weather_get78_002c1650,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1650, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get80` (0x002c_1658) against the generated body.
    #[test]
    fn t_wpigd_weather_get80_002c1658_matches() {
        check(
            ee::s08::f_002c1658__generated,
            t_wpigd_weather_get80_002c1658,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1658, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get88` (0x002c_1660) against the generated body.
    #[test]
    fn t_wpigd_weather_get88_002c1660_matches() {
        check(
            ee::s08::f_002c1660__generated,
            t_wpigd_weather_get88_002c1660,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1660, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get90` (0x002c_1668) against the generated body.
    #[test]
    fn t_wpigd_weather_get90_002c1668_matches() {
        check(
            ee::s08::f_002c1668__generated,
            t_wpigd_weather_get90_002c1668,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1668, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get98` (0x002c_1670) against the generated body.
    #[test]
    fn t_wpigd_weather_get98_002c1670_matches() {
        check(
            ee::s08::f_002c1670__generated,
            t_wpigd_weather_get98_002c1670,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1670, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Danger_Get08` (0x002c_1678) against the generated body.
    #[test]
    fn t_wpigd_danger_get08_002c1678_matches() {
        check(
            ee::s08::f_002c1678__generated,
            t_wpigd_danger_get08_002c1678,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1678, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Danger_Get10` (0x002c_1680) against the generated body.
    #[test]
    fn t_wpigd_danger_get10_002c1680_matches() {
        check(
            ee::s08::f_002c1680__generated,
            t_wpigd_danger_get10_002c1680,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1680, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// Replay-frame walkers: a stream whose vtable holds the offset adjust
    /// and the (fake) transfer function; unwinds at random calls.
    pub(crate) fn oc1531_stream_fixture(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        calls: u32,
        adj_off: u32,
        fn_off: u32,
    ) {
        const OBJ: u32 = DATA + 0x1000;
        const STREAM: u32 = DATA + 0x1100;
        const VT: u32 = DATA + 0x1200;
        let mut r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(STREAM, VT);
        c.bus.write16(VT + adj_off, r.next_u32() as u16);
        c.bus.write32(VT + fn_off, fakes::FAKE + 0x10 * (seed % 4));
        fill(c, &mut r, OBJ, 8);
        c.g[A0][0] = sx(OBJ);
        c.g[A1][0] = sx(STREAM);
    }

    #[test]
    fn twpigd_sun_load_state_matches() {
        fakes::covered(
            ee::s08::f_002bea38__generated,
            twpigd_sun_load_state,
            120,
            &|c, seed| oc1531_stream_fixture(c, seed, 0x002b_ea38, 18, 0x10, 0x14),
        );
    }

    #[test]
    fn twpigd_weather_load_state_matches() {
        fakes::covered(
            ee::s08::f_002bee48__generated,
            twpigd_weather_load_state,
            120,
            &|c, seed| oc1531_stream_fixture(c, seed, 0x002b_ee48, 38, 0x10, 0x14),
        );
    }

    #[test]
    fn twpigd_sun_save_state_matches() {
        fakes::covered(
            ee::s08::f_002bfaa0__generated,
            twpigd_sun_save_state,
            120,
            &|c, seed| oc1531_stream_fixture(c, seed, 0x002b_faa0, 18, 0x8, 0xc),
        );
    }

    /// A blend fixture: `self` at DATA+0x100 (method table at +4, bank at
    /// +8, level at +0xc, rate at +0x10), `rhs` at DATA+0x200 (level at +4,
    /// rate at +8), the blend factor in f12, and a method table at DATA+0x300
    /// whose slot +0x218 is a delta with a fake target (outside the table, so
    /// the miss hook logs the call).
    pub(crate) fn wp_blend_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x1550_b1, 2);
        let this = DATA + 0x100;
        let rhs = DATA + 0x200;
        let vt = DATA + 0x300;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(rhs);
        c.f[12] = if seed % 5 == 0 {
            r.edge_f32()
        } else {
            r.f32(1.5)
        };
        c.bus.write32(this + 4, vt);
        c.bus.write32(this + 8, r.next_u32() % 2001);
        c.bus.write32(this + 0xc, r.next_u32() % 2001);
        c.bus.write32(this + 0x10, r.f32(10.0));
        c.bus.write32(rhs + 4, r.next_u32() % 2001);
        c.bus.write32(rhs + 8, r.f32(10.0));
        c.bus.write16(vt + 0x218, (r.next_u32() & 0xfffc) as u16);
        c.bus.write32(vt + 0x21c, fakes::FAKE + 0x10 * (seed % 2));
    }

    #[test]
    fn wp_igd_ambience_blend_matches() {
        check(
            ee::s08::f_002bcd68__generated,
            wp_igd_ambience_blend,
            300,
            &|c, seed| wp_blend_setup(c, seed),
        );
    }

    #[test]
    fn wp_igd_speech_blend_matches() {
        check(
            ee::s08::f_002bce08__generated,
            wp_igd_speech_blend,
            300,
            &|c, seed| wp_blend_setup(c, seed),
        );
    }

    #[test]
    fn wp_igd_danger_blend_matches() {
        check(
            ee::s08::f_002bd968__generated,
            wp_igd_danger_blend,
            300,
            &|c, seed| wp_blend_setup(c, seed),
        );
    }

    /// The reader object for a state function: a replay frame whose vtable has
    /// the transfer slot `{delta, FAKE}` at `slot_off`, and an object at
    /// `DATA + 0x2200`. `calls` is the number of transfers, so the fake's
    /// unwind index can fall on any of them.
    pub(crate) fn oc1551_state_setup(c: &mut Cpu, seed: u32, slot_off: u32, calls: u32, salt: u64) {
        let mut r = fakes::machine(c, seed, salt, calls);
        let vt = DATA + 0x2000;
        let delta = (r.next_u32() as i16 as i32 & !3) as u16;
        c.bus.write16(vt + slot_off, delta);
        c.bus.write32(vt + slot_off + 4, fakes::FAKE);
        let frame = DATA + 0x2100;
        c.bus.write32(frame, vt);
        c.g[A0][0] = sx(DATA + 0x2200 + 0x10 * (seed % 4));
        c.g[A1][0] = sx(frame);
    }

    #[test]
    fn twpigd_music_trigger_load_state_matches() {
        fakes::covered(
            ee::s08::f_002be2d8__generated,
            twpigd_music_trigger_load_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_READ_SLOT, 4, 0x1551_e2d8),
        );
    }

    #[test]
    fn twpigd_fog_load_state_matches() {
        fakes::covered(
            ee::s08::f_002be518__generated,
            twpigd_fog_load_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_READ_SLOT, 12, 0x1551_e518),
        );
    }

    #[test]
    fn twpigd_light_glow_load_state_matches() {
        fakes::covered(
            ee::s08::f_002be698__generated,
            twpigd_light_glow_load_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_READ_SLOT, 14, 0x1551_e698),
        );
    }

    #[test]
    fn twpigd_screen_tint_load_state_matches() {
        fakes::covered(
            ee::s08::f_002be850__generated,
            twpigd_screen_tint_load_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_READ_SLOT, 12, 0x1551_e850),
        );
    }

    #[test]
    fn twpigd_lighting_load_state_matches() {
        fakes::covered(
            ee::s08::f_002becc8__generated,
            twpigd_lighting_load_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_READ_SLOT, 12, 0x1551_ecc8),
        );
    }

    #[test]
    fn twpigd_danger_load_state_matches() {
        fakes::covered(
            ee::s08::f_002bf2a0__generated,
            twpigd_danger_load_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_READ_SLOT, 4, 0x1551_f2a0),
        );
    }

    #[test]
    fn twpigd_music_trigger_save_state_matches() {
        fakes::covered(
            ee::s08::f_002bf340__generated,
            twpigd_music_trigger_save_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_WRITE_SLOT, 4, 0x1551_f340),
        );
    }

    #[test]
    fn twpigd_fog_save_state_matches() {
        fakes::covered(
            ee::s08::f_002bf580__generated,
            twpigd_fog_save_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_WRITE_SLOT, 12, 0x1551_f580),
        );
    }

    /// The replay frame: its first word is the vtable of the write slot.
    pub(crate) const FRAME: u32 = DATA + 0x20_0000;

    /// The object being saved, the ISTR name and block.
    pub(crate) const OBJ: u32 = DATA + 0x20_1000;

    pub(crate) const SALT: u64 = 0x1552_0000;

    /// A save-state fixture: `a0` is the object, `a1` the replay frame whose
    /// first word is a vtable with one write slot (a logged fake call).
    pub(crate) fn replay_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = machine(c, seed, SALT, calls);
        let vt = vtable(c, &mut r, &[0x8]);
        c.bus.write32(FRAME, vt);
        c.g[A0][0] = sx(OBJ);
        c.g[A1][0] = sx(FRAME);
    }

    #[test]
    fn light_glow_save_state_matches() {
        covered(
            ee::s08::f_002bf700__generated,
            light_glow_save_state,
            200,
            &|c, seed| replay_fixture(c, seed, 14),
        );
    }

    #[test]
    fn screen_tint_save_state_matches() {
        covered(
            ee::s08::f_002bf8b8__generated,
            screen_tint_save_state,
            200,
            &|c, seed| replay_fixture(c, seed, 12),
        );
    }

    #[test]
    fn lighting_save_state_matches() {
        covered(
            ee::s08::f_002bfd30__generated,
            lighting_save_state,
            200,
            &|c, seed| replay_fixture(c, seed, 12),
        );
    }

    #[test]
    fn danger_save_state_matches() {
        covered(
            ee::s08::f_002c0308__generated,
            danger_save_state,
            200,
            &|c, seed| replay_fixture(c, seed, 4),
        );
    }
}
