//! `src/visualfx/worldlightman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ai::stream_write_member;
pub(crate) use crate::game::ai::rider::fcc_lt;
pub(crate) use crate::game::ai::rider::fr_3 as fr_2;
pub(crate) use crate::game::ai::rider::gp_lit;
pub(crate) use crate::game::ai::rider::set_fr;
pub(crate) use crate::game::be::bedbreward::BUILTIN_DELETE_ADDR;
pub(crate) use crate::game::fe::feasyncfile::lw_9 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_3;
pub(crate) use crate::game::intersect::worldsphtree::forward_to_arg_vtable;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::render::lightman::addu;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::render::particle::eq;
pub(crate) use crate::game::render::particle::fconst;
pub(crate) use crate::game::render::particle::ldf;
pub(crate) use crate::game::render::particle::lt;
pub(crate) use crate::game::render::particle::p;
pub(crate) use crate::game::render::particle::setf;
pub(crate) use crate::game::visualfx::boardwakefx::rider_wrapper_ctor;
pub(crate) use crate::game::visualfx::boardwakefx::RiderWrapperCtor;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
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
        0x002f_5298,
        world_light_man_alloc_overlap_list,
        "WorldLightMan_AllocOverlapList",
    ),
    (
        0x0011_1830,
        rider_base_get_main_state_name,
        "RiderBase_GetMainStateName",
    ),
    (
        0x0011_1860,
        rider_base_get_sub_state_name,
        "RiderBase_GetSubStateName",
    ),
    (0x002d_9b40, avalanche_destroy_all, "Avalanche_DestroyAll"),
    (0x002d_aa78, c_board_spark_fx_reset, "cBoardSparkFX_Reset"),
    (
        0x002d_bf80,
        c_ps2_fog_particle_man_base_ctor,
        "cPS2FogParticleMan_BaseCtor",
    ),
    (0x002e_8560, c_board_trail_fx_reset, "cBoardTrailFX_Reset"),
    (
        0x002f_53b0,
        world_light_man_free_overlap_lists,
        "WorldLightMan_FreeOverlapLists",
    ),
    (
        0x002f_5998,
        world_light_man_reset_regions,
        "WorldLightMan_ResetRegions",
    ),
    (
        0x002f_59d0,
        world_light_man_evict_oldest_instance,
        "WorldLightMan_EvictOldestInstance",
    ),
    (
        0x002f_5b68,
        world_light_man_find_strongest_lights,
        "WorldLightMan_FindStrongestLights",
    ),
    (
        0x002f_5d30,
        world_light_man_light_contribution,
        "WorldLightMan_LightContribution",
    ),
    (
        0x002f_6168,
        world_light_man_light_distance,
        "WorldLightMan_LightDistance",
    ),
    (
        0x002f_6908,
        rider_base_render_cheat_code,
        "RiderBase_RenderCheatCode",
    ),
    (
        0x002f_6a90,
        rider_base_load_cheat_code,
        "RiderBase_LoadCheatCode",
    ),
    (
        0x002f_6b28,
        rider_base_clear_cheat_code_events,
        "RiderBase_ClearCheatCodeEvents",
    ),
    (
        0x002f_6b38,
        rider_base_get_cheat_code_events,
        "RiderBase_GetCheatCodeEvents",
    ),
    (
        0x002f_7a68,
        board_spark_emitter_ctor,
        "BoardSparkEmitter_ctor",
    ),
    (
        0x002f_7be0,
        ps2_visual_effects_man_is_handle_null,
        "PS2VisualEffectsMan_IsHandleNull",
    ),
    (0x002f_7bf0, vfx_component_dtor, "VfxComponent_dtor"),
    (
        0x002f_7f00,
        vfx_power_up_fx_toggles_dtor,
        "VfxPowerUpFXToggles_dtor",
    ),
    (
        0x002f_7f30,
        c_ps2_fog_particle_man_base_dtor,
        "cPS2FogParticleMan_BaseDtor",
    ),
    (
        0x002f_7f60,
        c_ps2_fog_particle_man_dtor,
        "cPS2FogParticleMan_dtor",
    ),
    (
        0x002f_7f90,
        vfx_splash_toggles_dtor,
        "VfxSplashToggles_dtor",
    ),
    (0x002f_8000, transition_effect_dtor, "TransitionEffect_dtor"),
    (
        0x002f_8030,
        transition_effect_fade_dtor,
        "TransitionEffect_FadeDtor",
    ),
    (0x002f_80d8, transition_wipe_dtor, "TransitionWipe_dtor"),
    (
        0x002f_8108,
        transition_effect_random_dtor,
        "TransitionEffect_RandomDtor",
    ),
    (
        0x002f_8138,
        vfx_snow_fall_toggles_dtor,
        "VfxSnowfallToggles_dtor",
    ),
    (0x002f_8188, vfx_board_trail_dtor, "VfxBoardTrail_dtor"),
    (0x002f_81f0, vfx_light_glow_dtor, "VfxLightGlow_dtor"),
    (0x002f_8220, vfx_depth_fog_dtor, "VfxDepthFog_dtor"),
    (0x002f_8250, vfx_height_fog_dtor, "VfxHeightFog_dtor"),
    (0x002f_8280, vfx_shadow_dtor, "VfxShadow_dtor"),
    (0x002f_82b0, vfx_snow_surface_dtor, "VfxSnowSurface_dtor"),
    (0x002f_82e0, vfx_flags_dtor, "VfxFlags_dtor"),
    (
        0x002f_8310,
        visual_effects_man_dtor,
        "VisualEffectsMan_dtor",
    ),
    (
        0x002f_8340,
        aivisual_effects_man_dtor,
        "AIVisualEffectsMan_dtor",
    ),
    (
        0x002f_1150,
        rider_base_update_attack_fx,
        "RiderBase_UpdateAttackFX",
    ),
    (
        0x002f_6518,
        rider_base_update_cheat_code,
        "RiderBase_UpdateCheatCode",
    ),
    (
        0x002f_5400,
        world_light_man_get_instance_lighting,
        "WorldLightMan_GetInstanceLighting",
    ),
    (
        0x002f_6910,
        rider_base_match_cheat_code,
        "RiderBase_MatchCheatCode",
    ),
    (
        0x002f_6a58,
        rider_base_save_cheat_code,
        "RiderBase_SaveCheatCode",
    ),
    (
        0x002f_5a70,
        world_light_man_get_overlap_region,
        "WorldLightMan_GetOverlapRegion",
    ),
    (
        0x002f_5af0,
        world_light_man_add_point_lights,
        "WorldLightMan_AddPointLights",
    ),
    (
        0x002d_c190,
        ps2_fog_particle_man_v02,
        "cPS2FogParticleMan_v02",
    ),
    (
        0x002f_7ba8,
        rider_base_reset_attack_fx_thunk,
        "RiderBase_ResetAttackFXThunk",
    ),
    (
        0x002f_7ed0,
        rider_base_reset_power_up_trail_thunk,
        "RiderBase_ResetPowerUpTrailThunk",
    ),
    (
        0x002f_6238,
        rider_base_ctor_cheat_code,
        "RiderBase_CtorCheatCode",
    ),
    (
        0x002f_64e8,
        rider_base_reset_cheat_code,
        "RiderBase_ResetCheatCode",
    ),
    (
        0x002f_64f0,
        rider_base_clear_cheat_code,
        "RiderBase_ClearCheatCode",
    ),
    (
        0x002f_83b0,
        attached_emitter_type_data_init_part1,
        "AttachedEmitterTypeData_InitPart1",
    ),
    (
        0x002f_9838,
        attached_emitter_type_data_init_part2,
        "AttachedEmitterTypeData_InitPart2",
    ),
    (
        0x002f_8ff8,
        oc817_attached_emitter_type_data_free_part1,
        "AttachedEmitterTypeData_FreePart1",
    ),
    (
        0x002f_a5f8,
        oc815_attached_emitter_type_data_free_part2,
        "AttachedEmitterTypeData_FreePart2",
    ),
    (
        0x002f_6388,
        rider_base_dtor_cheat_code,
        "RiderBase_DtorCheatCode",
    ),
    (
        0x002f_6b88,
        visual_effects_man_static_init_destroy,
        "VisualEffectsMan_StaticInitDestroy",
    ),
    (
        0x002f_7ab8,
        cboard_wake_fx_reset_thunk,
        "cBoardWakeFX_ResetThunk",
    ),
    (
        0x002f_7ae0,
        cboard_spark_fx_reset_thunk,
        "cBoardSparkFX_ResetThunk",
    ),
    (
        0x002f_7b08,
        cboard_trail_fx_reset_thunk,
        "cBoardTrailFX_ResetThunk",
    ),
    (0x002f_7b30, cang_er_fx_reset_thunk, "cAngerFX_ResetThunk"),
    (
        0x002f_7b58,
        cplayer_pulse_fx_reset_thunk,
        "cPlayerPulseFX_ResetThunk",
    ),
    (
        0x002f_7b80,
        cair_streamer_fx_reset_thunk,
        "cAirStreamerFX_ResetThunk",
    ),
    (0x002f_7c38, vfx_avalanche_ctor, "VfxAvalanche_ctor"),
    (
        0x002f_7ea8,
        c_speed_boost_streamer_fx_reset_thunk,
        "cSpeedBoostStreamerFX_ResetThunk",
    ),
    (
        0x002f_8060,
        transition_effect_fade_ctor,
        "TransitionEffect_FadeCtor",
    ),
    (
        0x002f_81b8,
        c_cut_scene_transition_man_dtor,
        "cCutSceneTransitionMan_dtor",
    ),
    (
        0x002f_8370,
        visual_effects_man_global_ctors,
        "VisualEffectsMan_GlobalCtors",
    ),
    (
        0x002f_8390,
        visual_effects_man_global_dtors,
        "VisualEffectsMan_GlobalDtors",
    ),
    (
        0x002f_9040,
        attached_emitter_type_data_static_init1,
        "AttachedEmitterTypeData_StaticInit1",
    ),
    (
        0x002f_9818,
        attached_emitter_type_data_global_ctors1,
        "AttachedEmitterTypeData_GlobalCtors1",
    ),
    (
        0x002f_a640,
        attached_emitter_type_data_static_init2,
        "AttachedEmitterTypeData_StaticInit2",
    ),
    (
        0x002f_ae18,
        attached_emitter_type_data_global_ctors2,
        "AttachedEmitterTypeData_GlobalCtors2",
    ),
];

// ---------------------------------------------------------------------------
// Lights.

/// Point light fields read by `WorldLightMan_LightDistance`.
pub(crate) mod light {
    pub(crate) use crate::game::render::particle::light::DIR;
    pub(crate) use crate::game::render::particle::light::POS;
    pub(crate) use crate::game::render::particle::light::RADIUS;

    pub const ENTRY: u32 = 0x002f_5d30;

    pub const END: u32 = 0x002f_6164;

    /// Loop heads of the two `cone^exponent` loops (resume points).
    pub const POWI_INNER: u32 = 0x002f_5f48;

    pub const POWI_PENUMBRA: u32 = 0x002f_6018;

    /// 0 ambient, 1 spot / point (cone), 2 directional-like (no cone).
    pub const TYPE: u32 = 0x10;

    pub const INTENSITY: u32 = 0x14;

    pub const SCALE: u32 = 0x18;

    pub const RANGE: u32 = 0x1c;

    /// Cosines of the inner and outer cone angles.
    pub const COS_INNER: u32 = 0x5c;

    pub const COS_OUTER: u32 = 0x60;

    /// Distance falloff power (s8, 1..3) and cone exponent (s8).
    pub const FALLOFF: u32 = 0x64;

    pub const EXPONENT: u32 = 0x65;

    /// The exponent switches' jump tables (`.rodata`) and their targets.
    pub const INNER_TABLE_AT: u32 = 0x0048_7cf0;

    pub const PENUMBRA_TABLE_AT: u32 = 0x0048_7d10;

    pub const INNER_TABLE: [u32; 5] = [
        0x002f_5eb8,
        0x002f_5ec8,
        0x002f_5ed4,
        0x002f_5ee0,
        0x002f_5ef0,
    ];

    pub const PENUMBRA_TABLE: [u32; 5] = [
        0x002f_5f90,
        0x002f_5fa4,
        0x002f_5fa0,
        0x002f_5fac,
        0x002f_5fbc,
    ];
}

// ---------------------------------------------------------------------------
// Callees and field offsets.

pub(crate) mod callee {

    pub(crate) use crate::game::render::lightman::callee::LIGHT_CONTRIBUTION;
}

/// `WorldLightMan` fields.
pub(crate) mod lights {

    pub const COUNT: u32 = 0x210;

    pub const ARRAY: u32 = 0x214;

    /// `sLight`: type at +8, intensity at +0x18.
    pub const TYPE: u32 = 0x8;
}

// ---------------------------------------------------------------------------
// Rider base state names.

/// `const char *RiderBase_GetMainStateName(RiderBase *self)` (0x111830):
/// index `self->+0xde0` into the 7-entry main-state name table, or a default.
pub fn rider_base_get_main_state_name(c: &mut Cpu) {
    const MAIN_STATE: u32 = 0xde0;
    const NAMES: u32 = 0x0045_9d70; // 0x460000 - 0x6290
    const DEFAULT: u32 = 0x0045_6c48; // 0x450000 + 0x6c48
    c.cycles += 4;
    let this = abi::arg(c, 0);
    let state = Addr(this).off(MAIN_STATE).lw(c);
    c.g[V1][0] = state;
    let in_range = state < 7;
    c.g[V0][0] = in_range as u64;
    if !in_range {
        c.cycles += 2;
        c.g[V0][0] = sx(DEFAULT);
        c.pc = c.g[RA][0] as u32;
    } else {
        c.cycles += 6;
        c.g[V1][0] = sx((state as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(NAMES));
        let ret = c.g[RA][0] as u32;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.pc = ret;
    }
}

/// `const char *RiderBase_GetSubStateName(RiderBase *self)` (0x111860):
/// index `self->+0xde4` into the 15-entry sub-state name table, or a default.
pub fn rider_base_get_sub_state_name(c: &mut Cpu) {
    const SUB_STATE: u32 = 0xde4;
    const NAMES: u32 = 0x0045_9dc8; // 0x460000 - 0x6238
    const DEFAULT: u32 = 0x0045_6c48; // 0x450000 + 0x6c48
    c.cycles += 4;
    let this = abi::arg(c, 0);
    let state = Addr(this).off(SUB_STATE).lw(c);
    c.g[V1][0] = state;
    let in_range = state < 15;
    c.g[V0][0] = in_range as u64;
    if !in_range {
        c.cycles += 2;
        c.g[V0][0] = sx(DEFAULT);
        c.pc = c.g[RA][0] as u32;
    } else {
        c.cycles += 6;
        c.g[V1][0] = sx((state as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(NAMES));
        let ret = c.g[RA][0] as u32;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.pc = ret;
    }
}

/// `void cPS2FogParticleMan_BaseCtor(cPS2FogParticleMan *this)` (0x2dbf80):
/// clear the first word and install the vtable at +4.
pub fn c_ps2_fog_particle_man_base_ctor(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    c.g[V1][0] = 0x49_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7cf8));
    Addr(this).sw(c, 0);
    Addr(this).off(0x4).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Rider cheat events.

/// `void RiderBase_ClearCheatCodeEvents(Rider *rider)` (0x2f6b28): clear the
/// low cheat-event bits of the byte at +7.
pub fn rider_base_clear_cheat_code_events(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x7).lbu(c);
    c.g[V0][0] = c.g[V0][0] & 0x8f;
    Addr(this).off(0x7).sb(c, c.g[V0][0]);
    abi::ret(c);
}

/// `int RiderBase_GetCheatCodeEvents(Rider *rider)` (0x2f6b38): the high
/// cheat-event bits of the byte at +7.
pub fn rider_base_get_cheat_code_events(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x7).lbu(c);
    c.g[V0][0] = c.g[V0][0] & 0x70;
    abi::ret(c);
}

/// `void AIVisualEffectsMan_dtor(AIVisualEffectsMan *this, uint flags)`
/// (0x2f8340): set the vtable at +4 to 0x487d78 and, when `flags & 1`, delete
/// `this`.
pub fn aivisual_effects_man_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x48_0000u32.wrapping_add(0x7d78));
        c.g[A1][0] &= 1;
        let delete_this = c.g[A1][0] != 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[V0][0]);
        if delete_this {
            c.cycles += 2;
            call_ptr(c, BUILTIN_DELETE_ADDR, 0x002f_8364)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WorldLightMan_LightDistance` (0x2f6168, s09).
pub(crate) const LIGHT_DISTANCE: Callee = Callee {
    addr: 0x002f_6168,
    f: ssx3_ee::s09::WorldLightMan_LightDistance,
};

/// An FPU register as a float.
#[inline]
pub(crate) fn fr(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

/// `float WorldLightMan_LightContribution(WorldLightMan* m, Vec3* point,
/// Light* light)` (0x2f5d30): the light's intensity at a point. Ambient
/// lights give their scale (+0x18); a cone light (type 1) gives
/// `scale * intensity * falloff * attenuation * cone` when the point is
/// inside the outer cone, with the cone factor `cos^exp` inside the inner
/// cone and `inner^exp * (cos - outer) / (inner - outer)` in the penumbra;
/// type 2 gives `scale * intensity * falloff * attenuation`. The distance,
/// its inverse and the cone cosine come from `WorldLightMan_LightDistance`
/// (into the stack frame). Other types give 0.
///
/// A dispatcher: the `^exp` loops can unwind and are re-entered at their
/// heads with the state in v1, f0..f6 and s0.
pub fn world_light_man_light_contribution(c: &mut Cpu) {
    guest(c, |c| match c.pc {
        light::ENTRY => light_contribution(c),
        light::POWI_INNER => {
            light_powi(c, 1, light::POWI_INNER)?;
            inner_cone_finish(c);
            light_epilogue(c);
            Ok(())
        }
        light::POWI_PENUMBRA => {
            light_powi(c, 3, light::POWI_PENUMBRA)?;
            c.cycles += 1;
            c.f[0] = c.f[3];
            c.cycles += 1;
            c.f[3] = c.f[0];
            penumbra_finish(c);
            light_epilogue(c);
            Ok(())
        }
        pc => {
            c.jump_out(pc, light::ENTRY, light::END);
            Ok(())
        }
    })
}

/// The body from the entry: false results end in `f0 = 0`.
pub(crate) fn light_contribution(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = 1;
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0x10);
    c.g[S0][0] = c.g[A2][0];
    let l = Addr(c.g[S0][0] as u32);
    let kind = l.off(light::TYPE).lw(c);
    c.g[V1][0] = kind;
    c.g[V0][0] = ((kind as i64) < 2) as u64;
    let lit = if kind == 1 {
        cone_light(c, l)?
    } else {
        c.cycles += 2;
        if (kind as i64) < 2 {
            c.cycles += 2;
            if kind == 0 {
                c.f[0] = l.off(light::SCALE).read_u32(c);
                true
            } else {
                c.cycles += 2;
                false
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 3;
            if kind == 2 {
                plain_light(c, l)?
            } else {
                c.cycles += 2;
                false
            }
        }
    };
    if !lit {
        c.cycles += 1;
        c.f[0] = 0;
    }
    light_epilogue(c);
    Ok(())
}

/// 0x2f6154: restore s0 and ra, return.
pub(crate) fn light_epilogue(c: &mut Cpu) {
    c.cycles += 4;
    let frame = Frame { size: 0x30 };
    frame.restore(c, S0, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

/// `WorldLightMan_LightDistance(m, point, light, &dist, &invDist,
/// cosOut)` with the outputs in the frame (sp, sp|4, `cos_at`).
pub(crate) fn light_distance(c: &mut Cpu, cos_at: u64, ret: u32) -> Flow {
    c.cycles += 7;
    let sp = c.g[SP][0];
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = sp;
    c.g[T0][0] = sp | 4;
    c.g[T1][0] = cos_at;
    LIGHT_DISTANCE.call(c, ret)
}

/// Distance attenuation into f4: 1 when the light's range (+0x1c) is
/// below the threshold at gp-`at[0]` or the distance (sp+0) is within
/// gp-`at[1]`, else `1 - (d - start) * slope(gp-`at[2]`)`. Then the
/// inverse distance scaled by 100 into f1 and its test against 1 (`near`);
/// f2 = 1.0, v1 = the falloff power.
pub(crate) fn light_attenuation(c: &mut Cpu, l: Addr, at: [u32; 3]) -> bool {
    let gp = c.g[GP][0] as u32;
    let stack = Addr(c.g[SP][0] as u32);
    c.cycles += 6;
    let range = l.off(light::RANGE).read_fpu(c);
    let small = Addr(gp.wrapping_sub(at[0])).read_fpu(c);
    c.f[1] = range.0;
    c.f[0] = small.0;
    let no_range = range.lt(small);
    abi::set_fcc(c, no_range);
    let d = stack.read_fpu(c);
    c.f[2] = d.0;
    let over = if no_range {
        None
    } else {
        c.cycles += 5;
        let start = Addr(gp.wrapping_sub(at[1])).read_fpu(c);
        c.f[0] = start.0;
        let within = d.le(start);
        abi::set_fcc(c, within);
        if within {
            None
        } else {
            Some(d - start)
        }
    };
    c.g[AT][0] = FpuF32::ONE.0 as u64;
    match over {
        None => {
            c.cycles += 4;
            c.f[4] = FpuF32::ONE.0;
        }
        Some(over) => {
            c.cycles += 6;
            let slope = Addr(gp.wrapping_sub(at[2])).read_fpu(c);
            c.f[1] = slope.0;
            c.f[2] = FpuF32::ONE.0;
            c.f[4] = (FpuF32::ONE - over * slope).0;
        }
    }
    let inv = stack.off(4).read_fpu(c);
    c.f[0] = inv.0;
    c.cycles += 9;
    let x = inv * FpuF32(0x42c8_0000);
    c.f[1] = x.0;
    c.f[2] = FpuF32::ONE.0;
    let near = x.lt(FpuF32::ONE);
    abi::set_fcc(c, near);
    c.g[V1][0] = l.off(light::FALLOFF).lb(c);
    near
}

/// Type 1: a cone light. False when outside the cone or out of reach.
pub(crate) fn cone_light(c: &mut Cpu, l: Addr) -> Result<bool, Unwound> {
    let cos_at = c.g[SP][0] | 8;
    light_distance(c, cos_at, 0x002f_5d8c)?;
    let stack = Addr(c.g[SP][0] as u32);
    let cos = stack.off(8).read_fpu(c);
    c.f[1] = cos.0;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    c.cycles += 6;
    let cos_outer = l.off(light::COS_OUTER).read_fpu(c);
    c.f[0] = cos_outer.0;
    c.f[5] = cos.0;
    c.f[6] = cos_outer.0;
    let in_cone = cos_outer.le(cos);
    abi::set_fcc(c, in_cone);
    if !in_cone {
        return Ok(false);
    }
    let near = light_attenuation(c, l, [0x38b0, 0x38ac, 0x38a8]);
    // falloff in f1: (100 / d)^power when near, else 1
    let mode = c.g[V1][0];
    let x = fr(c, 1);
    let mut falloff_one = !near;
    let mut intensity_loaded = false;
    if near {
        c.cycles += 3;
        c.g[V0][0] = ((mode as i64) < 3) as u64;
        if mode == 2 {
            c.cycles += 2;
            c.f[1] = (x * x).0;
        } else {
            c.cycles += 2;
            let low = (mode as i64) < 3;
            c.g[V0][0] = 1;
            if low {
                c.cycles += 2;
                if mode == 1 {
                    c.f[0] = l.off(light::INTENSITY).read_u32(c);
                    intensity_loaded = true;
                } else {
                    c.cycles += 2;
                    falloff_one = true;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 3;
                if mode == 3 {
                    let x2 = x * x;
                    c.f[0] = x2.0;
                    c.cycles += 2;
                    c.f[1] = (x2 * x).0;
                } else {
                    c.cycles += 2;
                    falloff_one = true;
                }
            }
        }
    }
    if falloff_one {
        c.cycles += 2;
        c.f[1] = FpuF32::ONE.0;
    }
    if !intensity_loaded {
        c.cycles += 1;
        c.f[0] = l.off(light::INTENSITY).read_u32(c);
    }
    // f2 = falloff * intensity * attenuation; f1 = inner cosine
    c.cycles += 6;
    let cos_inner = l.off(light::COS_INNER).read_fpu(c);
    c.f[3] = cos_inner.0;
    let lit = fr(c, 1) * fr(c, 0);
    c.f[0] = lit.0;
    let inside = cos_inner.le(cos);
    abi::set_fcc(c, inside);
    c.f[1] = cos_inner.0;
    c.f[2] = (lit * fr(c, 4)).0;
    let exponent = l.off(light::EXPONENT).lb(c);
    if inside {
        // cone = cos^exp (f1)
        c.cycles += 4;
        c.g[V1][0] = exponent;
        c.g[V0][0] = 0x48_0000;
        if exponent < 5 {
            c.cycles += 6;
            let e = exponent as usize;
            abi::set_word(c, V1, ((e as u32) << 2).wrapping_add(light::INNER_TABLE_AT));
            abi::set_word(c, V0, light::INNER_TABLE_AT);
            abi::set_word(c, A0, light::INNER_TABLE[e]);
            let cos = fr(c, 5);
            match e {
                0 => {
                    c.cycles += 4;
                    c.g[AT][0] = FpuF32::ONE.0 as u64;
                    c.f[1] = FpuF32::ONE.0;
                }
                1 => {
                    c.cycles += 3;
                    c.f[1] = cos.0;
                }
                2 => {
                    c.cycles += 3;
                    c.f[1] = (cos * cos).0;
                }
                3 => {
                    c.cycles += 4;
                    let p = cos * cos;
                    c.f[0] = p.0;
                    c.f[1] = (p * cos).0;
                }
                _ => {
                    c.cycles += 5;
                    let p = cos * cos * cos;
                    c.f[0] = p.0;
                    c.f[1] = (p * cos).0;
                }
            }
            c.f[4] = l.off(light::SCALE).read_u32(c);
        } else {
            c.cycles += 5;
            c.f[1] = 0;
            let cos = fr(c, 5);
            let zero = cos.eq_s(FpuF32::ZERO);
            abi::set_fcc(c, zero);
            c.f[0] = cos.0;
            if zero {
                c.cycles += 2;
                c.f[4] = l.off(light::SCALE).read_u32(c);
            } else {
                powi_setup(c, 5, 1, l)?;
                if c.g[V1][0] != 0 {
                    light_powi(c, 1, light::POWI_INNER)?;
                }
            }
        }
        inner_cone_finish(c);
    } else {
        // penumbra: cone = inner^exp * (cos - outer) / (inner - outer)
        c.cycles += 4;
        c.g[V1][0] = exponent;
        c.g[V0][0] = 0x48_0000;
        let inner = fr(c, 3);
        if exponent < 5 {
            c.cycles += 6;
            let e = exponent as usize;
            abi::set_word(
                c,
                V1,
                ((e as u32) << 2).wrapping_add(light::PENUMBRA_TABLE_AT),
            );
            abi::set_word(c, V0, light::PENUMBRA_TABLE_AT);
            abi::set_word(c, A0, light::PENUMBRA_TABLE[e]);
            match e {
                0 => {
                    c.cycles += 4;
                    c.g[AT][0] = FpuF32::ONE.0 as u64;
                    c.f[3] = FpuF32::ONE.0;
                }
                1 => {
                    c.cycles += 2;
                }
                2 => {
                    c.cycles += 1 + 2;
                    c.f[3] = (inner * inner).0;
                }
                3 => {
                    c.cycles += 4;
                    let p = inner * inner;
                    c.f[0] = p.0;
                    c.f[3] = (p * inner).0;
                }
                _ => {
                    c.cycles += 5;
                    let p = inner * inner * inner;
                    c.f[0] = p.0;
                    c.f[3] = (p * inner).0;
                }
            }
            c.f[4] = l.off(light::SCALE).read_u32(c);
        } else {
            c.cycles += 5;
            c.f[3] = 0;
            let zero = inner.eq_s(FpuF32::ZERO);
            abi::set_fcc(c, zero);
            c.f[0] = inner.0;
            if zero {
                c.cycles += 3;
                c.f[0] = 0;
                c.f[4] = l.off(light::SCALE).read_u32(c);
            } else {
                powi_setup(c, 1, 3, l)?;
                if c.g[V1][0] != 0 {
                    light_powi(c, 3, light::POWI_PENUMBRA)?;
                }
                c.cycles += 1;
                c.f[0] = c.f[3];
            }
            c.cycles += 1;
            c.f[3] = c.f[0];
        }
        penumbra_finish(c);
    }
    Ok(true)
}

/// Before a `^exp` loop (exponent in v1, base register `base`, not 0): a
/// negative exponent inverts the base (f0 = 1 / base, v1 = -v1); the
/// accumulator `acc` = 1, f4 = the light's scale.
pub(crate) fn powi_setup(c: &mut Cpu, base: usize, acc: usize, l: Addr) -> Flow {
    c.cycles += 2;
    if (c.g[V1][0] as i64) < 0 {
        c.cycles += 4;
        c.g[AT][0] = FpuF32::ONE.0 as u64;
        let n = c.g[V1][0] as u32;
        abi::set_word(c, V1, 0u32.wrapping_sub(n));
        c.f[0] = FpuF32::ONE.div(fr(c, base)).0;
    }
    c.cycles += 4;
    c.g[AT][0] = FpuF32::ONE.0 as u64;
    c.f[acc] = FpuF32::ONE.0;
    c.f[4] = l.off(light::SCALE).read_u32(c);
    Ok(())
}

/// The `^exp` loop by squaring (head 0x2f5f48 / 0x2f6018): `acc *= f0`
/// for each set bit of v1, `f0 *= f0`, `v1 >>= 1` until v1 is 0. Err when
/// an event unwound the thread at the back edge.
pub(crate) fn light_powi(c: &mut Cpu, acc: usize, head: u32) -> Flow {
    loop {
        c.cycles += 3;
        let n = c.g[V1][0];
        c.g[V0][0] = n & 1;
        if n & 1 != 0 {
            c.f[acc] = (fr(c, acc) * fr(c, 0)).0;
        }
        c.cycles += 3;
        let n = sx(((n as u32) as i32 >> 1) as u32);
        c.g[V1][0] = n;
        let b = fr(c, 0);
        c.f[0] = (b * b).0;
        if n == 0 {
            return Ok(());
        }
        if back_edge_unwinds(c, head) {
            return Err(Unwound);
        }
    }
}

/// 0x2f5f60: inside the inner cone, f2 *= cone (f1); then the result.
pub(crate) fn inner_cone_finish(c: &mut Cpu) {
    c.cycles += 2;
    c.f[2] = (fr(c, 2) * fr(c, 1)).0;
    spot_finish(c);
}

/// 0x2f6038: in the penumbra, f2 *= inner^exp (f3) * (cos - outer) /
/// (inner - outer); then the result.
pub(crate) fn penumbra_finish(c: &mut Cpu) {
    c.cycles += 5;
    let width = fr(c, 1) - fr(c, 6);
    c.f[1] = width.0;
    let into = fr(c, 5) - fr(c, 6);
    let f2 = fr(c, 2) * fr(c, 3);
    let ramp = into.div(width);
    c.f[0] = ramp.0;
    c.f[2] = (f2 * ramp).0;
    spot_finish(c);
}

/// 0x2f604c: f0 = scale (f4) * f2.
pub(crate) fn spot_finish(c: &mut Cpu) {
    c.cycles += 2;
    c.f[0] = (fr(c, 4) * fr(c, 2)).0;
}

/// Type 2: no cone. False when out of reach.
pub(crate) fn plain_light(c: &mut Cpu, l: Addr) -> Result<bool, Unwound> {
    light_distance(c, 0, 0x002f_6068)?;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    let near = light_attenuation(c, l, [0x38a4, 0x38a0, 0x389c]);
    // falloff in f2: (100 / d)^power when near, else 1
    let mode = c.g[V1][0];
    let x = fr(c, 1);
    let mut intensity_loaded = false;
    if near {
        c.cycles += 3;
        c.g[V0][0] = ((mode as i64) < 3) as u64;
        if mode == 2 {
            c.cycles += 2;
            c.f[2] = (x * x).0;
        } else {
            c.cycles += 2;
            let low = (mode as i64) < 3;
            c.g[V0][0] = 1;
            if low {
                c.cycles += 2;
                if mode == 1 {
                    c.f[2] = x.0;
                } else {
                    c.cycles += 2;
                    c.f[1] = l.off(light::INTENSITY).read_u32(c);
                    intensity_loaded = true;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 3;
                if mode == 3 {
                    let x2 = x * x;
                    c.f[0] = x2.0;
                    c.cycles += 2;
                    c.f[2] = (x2 * x).0;
                } else {
                    c.cycles += 2 + 2;
                    c.g[AT][0] = FpuF32::ONE.0 as u64;
                    c.f[2] = FpuF32::ONE.0;
                }
            }
        }
    }
    if !intensity_loaded {
        c.cycles += 1;
        c.f[1] = l.off(light::INTENSITY).read_u32(c);
    }
    c.cycles += 5;
    let scale = l.off(light::SCALE).read_fpu(c);
    let lit = fr(c, 2) * fr(c, 1);
    c.f[1] = lit.0;
    let lit = lit * fr(c, 4);
    c.f[2] = lit.0;
    c.f[0] = (scale * lit).0;
    Ok(true)
}

/// `bool WorldLightMan_LightDistance(this, const Vec3* pos, const Light*
/// light, float* dist, float* inv_dist /*t0*/, float* facing /*t1*/)`
/// (0x2f6168): false when `pos` is outside the light's radius. Otherwise
/// stores the distance (1 at the light's centre) and its inverse, and, when
/// `facing` is given, `-dot(light.dir, light.pos - pos) / dist`.
pub fn world_light_man_light_distance(c: &mut Cpu) {
    c.cycles += 20;
    let pos = abi::arg(c, 1);
    let lt_at = abi::arg(c, 2);
    let lx = ldf(c, 2, p(lt_at, light::POS));
    let px = ldf(c, 1, pos);
    let ly = ldf(c, 3, p(lt_at, light::POS + 4));
    let py = ldf(c, 0, p(pos, 4));
    let dx = lx - px;
    setf(c, 7, dx);
    let lz = ldf(c, 2, p(lt_at, light::POS + 8));
    let dy = ly - py;
    setf(c, 6, dy);
    let pz = ldf(c, 4, p(pos, 8));
    let dx2 = dx * dx;
    let radius = ldf(c, 0, p(lt_at, light::RADIUS));
    let dz = lz - pz;
    setf(c, 5, dz);
    let dy2 = dy * dy;
    setf(c, 3, dy2);
    let r2 = radius * radius;
    setf(c, 0, r2);
    let dz2 = dz * dz;
    setf(c, 2, dz2);
    let d2 = dx2 + dy2 + dz2;
    setf(c, 1, d2);
    let outside = lt(c, r2, d2);
    c.g[V0][0] = 0;
    if outside {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    setf(c, 0, FpuF32::ZERO);
    let (dist, inv) = if eq(c, d2, FpuF32::ZERO) {
        c.cycles += 3;
        let one = fconst(c, 4, 0x3f80_0000);
        setf(c, 3, one);
        (one, one)
    } else {
        c.cycles += 6;
        let one = fconst(c, 0, 0x3f80_0000);
        let dist = d2.sqrt();
        setf(c, 3, dist);
        let inv = one.div(dist);
        setf(c, 4, inv);
        (dist, inv)
    };
    c.cycles += 2;
    if c.g[T1][0] != 0 {
        c.cycles += 12;
        let ux = ldf(c, 0, p(lt_at, light::DIR));
        let uy = ldf(c, 2, p(lt_at, light::DIR + 4));
        let fx = dx * ux;
        let uz = ldf(c, 1, p(lt_at, light::DIR + 8));
        let fy = dy * uy;
        setf(c, 2, fy);
        let fz = dz * uz;
        setf(c, 1, fz);
        let facing = -(fx + fy + fz) * inv;
        setf(c, 0, facing);
        let out = abi::arg(c, 5);
        c.swc1(0, out);
    }
    c.swc1(3, abi::arg(c, 3));
    let _ = dist;
    c.cycles += 3;
    c.g[V0][0] = 1;
    c.swc1(4, abi::arg(c, 4));
    abi::ret(c);
}

/// `int WorldLightMan_FindStrongestLights(a, b, sLightMgr *mgr,
/// sLight **out, int max)` (0x002f5b68): clear the output, then insert each
/// type-6 light's contribution into the descending top-`max` list.
pub fn world_light_man_find_strongest_lights(c: &mut Cpu) {
    use lights::*;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x110);
        frame.save(c, S2, 0xf0);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0xe0);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S6, 0xb0);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S7, 0xa0);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, FP, 0x90);
        c.g[S7][0] = c.g[A1][0];
        frame.save(c, S1, 0x100);
        c.g[FP][0] = c.g[A0][0];
        frame.save(c, S4, 0xd0);
        frame.save(c, S5, 0xc0);
        frame.save_ra(c, 0x80);
        let no_out = (c.g[S2][0] as i64) <= 0;
        c.swc1(20, off(c.g[SP][0] as u32, 0x120));
        if !no_out {
            c.cycles += 3;
            c.g[A0][0] = c.g[SP][0];
            c.g[V1][0] = c.g[S3][0];
            c.g[V0][0] = c.g[S2][0];
            loop {
                c.cycles += 7;
                c.stw(c.g[V1][0] as u32, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.stw(c.g[A0][0] as u32, 0);
                c.g[V1][0] = sx(off(c.g[V1][0] as u32, 4));
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(off(c.g[A0][0] as u32, 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_5bc0);
                }
            }
        }
        c.cycles += 4;
        c.g[S5][0] = c.ldw(off(c.g[S6][0] as u32, COUNT));
        c.mtc1(0, 20);
        let no_lights = c.g[S5][0] == 0;
        c.g[V1][0] = 0;
        if !no_lights {
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            loop {
                c.cycles += 7;
                c.g[S4][0] = sx(off(c.g[V1][0] as u32, 1));
                c.g[V0][0] = addu(c.g[S6][0], c.g[V0][0]);
                c.g[S1][0] = c.ldw(off(c.g[V0][0] as u32, ARRAY));
                c.g[V0][0] = 6;
                c.g[V1][0] = c.ldw(off(c.g[S1][0] as u32, TYPE));
                let not_light = c.g[V1][0] != c.g[V0][0];
                c.g[V1][0] = c.g[S4][0];
                if !not_light {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[S7][0];
                    c.g[A2][0] = c.g[S1][0];
                    callee::LIGHT_CONTRIBUTION.call(c, 0x002f_5c1c)?;
                    c.mov_s(1, 0);
                    c.c_lt_s(20, 1);
                    let not_brighter = !c.fcc();
                    c.g[V1][0] = c.g[S4][0];
                    if !not_brighter {
                        // Find the insertion point: while k < count && s < score[k].
                        c.cycles += 3;
                        c.g[A3][0] = 0;
                        let empty = (c.g[S0][0] as i64) <= 0;
                        c.g[T1][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                        // The `s >= score[0]` exit of the search jumps out of
                        // the block, skipping the search tail's extra cycle.
                        let mut skipped_tail = false;
                        if !empty {
                            c.cycles += 5;
                            c.lwc1(0, c.g[SP][0] as u32);
                            c.c_lt_s(1, 0);
                            let not_first = !c.fcc();
                            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S2][0] as i64)) as u64;
                            if !not_first {
                                c.cycles += 2;
                                c.g[V1][0] = c.g[SP][0];
                                c.g[A3][0] = sx(off(c.g[A3][0] as u32, 1));
                                loop {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S0][0] as i64)) as u64;
                                    let done = c.g[V0][0] == 0;
                                    c.g[V1][0] = sx(off(c.g[V1][0] as u32, 4));
                                    if done {
                                        break;
                                    }
                                    c.cycles += 5;
                                    c.lwc1(0, c.g[V1][0] as u32);
                                    c.c_lt_s(1, 0);
                                    if !c.fcc() {
                                        break;
                                    }
                                    c.g[A3][0] = sx(off(c.g[A3][0] as u32, 1));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002f_5c58);
                                    }
                                }
                            } else {
                                skipped_tail = true;
                            }
                        }
                        if !skipped_tail {
                            c.cycles += 1;
                            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S2][0] as i64)) as u64;
                        }
                        let insert = c.g[V0][0] != 0;
                        c.cycles += 2;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                        if insert {
                            c.cycles += 5;
                            c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
                            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
                            let no_shift = c.g[V0][0] == 0;
                            c.g[T0][0] = sx((c.g[A3][0] as u32) << 2);
                            if !no_shift {
                                c.cycles += 4;
                                c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                                c.g[A1][0] = addu(c.g[V0][0], c.g[S3][0]);
                                c.g[A0][0] = addu(c.g[V0][0], c.g[SP][0]);
                                loop {
                                    c.cycles += 9;
                                    c.g[V0][0] = c.ldw((c.g[A1][0] as u32).wrapping_sub(4));
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                                    c.lwc1(0, (c.g[A0][0] as u32).wrapping_sub(4));
                                    c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
                                    c.stw(c.g[A1][0] as u32, c.g[V0][0]);
                                    c.swc1(0, c.g[A0][0] as u32);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(4));
                                    let more = c.g[V1][0] != 0;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(4));
                                    if !more {
                                        break;
                                    }
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002f_5ca8);
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = addu(c.g[T0][0], c.g[S3][0]);
                            c.g[V1][0] = addu(c.g[SP][0], c.g[T0][0]);
                            c.stw(c.g[V0][0] as u32, c.g[S1][0]);
                            c.swc1(1, c.g[V1][0] as u32);
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
                        c.g[V1][0] = addu(c.g[SP][0], c.g[V0][0]);
                        c.lwc1(20, c.g[V1][0] as u32);
                        c.g[V1][0] = c.g[S4][0];
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < c.g[S5][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_5bf0);
                }
            }
        }
        c.cycles += 13;
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
        frame.restore_fpr(c, 20, 0x120);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Handle tests.

/// `bool PS2VisualEffectsMan_IsHandleNull(Handle* h)` (0x2f7be0): true when the
/// handle's first word is 0.
pub fn ps2_visual_effects_man_is_handle_null(c: &mut Cpu) {
    c.cycles += 3;
    let h = abi::arg(c, 0);
    c.g[V0][0] = c.lw(h);
    c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
    abi::ret(c);
}

/// `operator delete` (`__builtin_delete`, 0x317e50).
pub(crate) const DELETE: u32 = 0x0031_7e50;

/// `TransitionEffect_FreeData` (0x2e4d30).
pub(crate) const FREE_DATA: u32 = 0x002e_4d30;

// ---------------------------------------------------------------------------
// Deleting destructors.

/// The shared body of the tiny deleting destructors: republish the class's
/// vtable pointer at `off` in `this`, then `operator delete(this)` when bit 0
/// of the destructor flags (`a1`) is set. `ret` is the address the delete
/// returns to.
pub(crate) fn deleting_dtor(c: &mut Cpu, off: u32, vtable: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = sx(vtable);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] &= 1;
        let deleting = c.g[A1][0] != 0;
        let this = c.g[A0][0] as u32;
        let v = c.g[V0][0];
        Addr(this).off(off).sw(c, v);
        if deleting {
            c.cycles += 2;
            call_ptr(c, DELETE, ret)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The same, for the `TransitionEffect` classes: bit 0 calls
/// `TransitionEffect_FreeData(this)` (which unregisters the effect's data
/// block) instead of freeing the object.
pub(crate) fn free_effect_data_dtor(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = sx(0x0048_80e0);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] &= 1;
        let free_data = c.g[A1][0] != 0;
        let this = c.g[A0][0] as u32;
        let v = c.g[V0][0];
        Addr(this).sw(c, v);
        if free_data {
            c.cycles += 2;
            call_ptr(c, FREE_DATA, ret)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `VfxComponent_dtor` (0x2f7bf0): vtable at `+0`.
pub fn vfx_component_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_7c14);
}

/// `VfxPowerUpFXToggles_dtor` (0x2f7f00): vtable at `+0`.
pub fn vfx_power_up_fx_toggles_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_7f24);
}

/// `cPS2FogParticleMan_BaseDtor` (0x2f7f30): vtable at `+0x4`.
pub fn c_ps2_fog_particle_man_base_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x4, 0x0048_8308, 0x002f_7f54);
}

/// `cPS2FogParticleMan_dtor` (0x2f7f60): vtable at `+0x4`.
pub fn c_ps2_fog_particle_man_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x4, 0x0048_8308, 0x002f_7f84);
}

/// `VfxSplashToggles_dtor` (0x2f7f90): vtable at `+0`.
pub fn vfx_splash_toggles_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_7fb4);
}

/// `TransitionEffect_dtor` (0x2f8000): vtable at `+0`, frees the effect data.
pub fn transition_effect_dtor(c: &mut Cpu) {
    free_effect_data_dtor(c, 0x002f_8024);
}

/// `TransitionEffect_FadeDtor` (0x2f8030): vtable at `+0`, frees the effect
/// data.
pub fn transition_effect_fade_dtor(c: &mut Cpu) {
    free_effect_data_dtor(c, 0x002f_8054);
}

/// `TransitionWipe_dtor` (0x2f80d8): vtable at `+0`, frees the effect data.
pub fn transition_wipe_dtor(c: &mut Cpu) {
    free_effect_data_dtor(c, 0x002f_80fc);
}

/// `TransitionEffect_RandomDtor` (0x2f8108): vtable at `+0`, frees the effect
/// data.
pub fn transition_effect_random_dtor(c: &mut Cpu) {
    free_effect_data_dtor(c, 0x002f_812c);
}

/// `VfxSnowfallToggles_dtor` (0x2f8138): vtable at `+0`.
pub fn vfx_snow_fall_toggles_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_815c);
}

/// `VfxBoardTrail_dtor` (0x2f8188): vtable at `+0`.
pub fn vfx_board_trail_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_81ac);
}

/// `VfxLightGlow_dtor` (0x2f81f0): vtable at `+0`.
pub fn vfx_light_glow_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_8214);
}

/// `VfxDepthFog_dtor` (0x2f8220): vtable at `+0`.
pub fn vfx_depth_fog_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_8244);
}

/// `VfxHeightFog_dtor` (0x2f8250): vtable at `+0`.
pub fn vfx_height_fog_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_8274);
}

/// `VfxShadow_dtor` (0x2f8280): vtable at `+0`.
pub fn vfx_shadow_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_82a4);
}

/// `VfxSnowSurface_dtor` (0x2f82b0): vtable at `+0`.
pub fn vfx_snow_surface_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_82d4);
}

/// `VfxFlags_dtor` (0x2f82e0): vtable at `+0`.
pub fn vfx_flags_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_8304);
}

/// `VisualEffectsMan_dtor` (0x2f8310): vtable at `+0x44`.
pub fn visual_effects_man_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x44, 0x0048_7da0, 0x002f_8334);
}

/// `void WorldLightMan_ResetRegions(WorldLightMan* man)` (0x2f5998): set the
/// eight region records to -1.
pub fn world_light_man_reset_regions(c: &mut Cpu) {
    const HEAD: u32 = 0x002f_59a8;
    c.cycles += 4;
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x10));
    c.g[A1][0] = 0;
    c.g[V1][0] |= 0xffff;
    loop {
        c.cycles += 7;
        Addr(r32(c, A0)).sw(c, c.g[V1][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        c.g[V0][0] = (c.g[A1][0] < 8) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x40));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void RiderBase_LoadCheatCode(RiderBase* self, const sCheatCode* code)`
/// (0x2f6a90): read the code's 4-byte entries into `self + 4`.
pub fn rider_base_load_cheat_code(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_arg_vtable(c, 0x002f_6ab8, 0x10, 4, Some(4))
    })
}

/// `void WorldLightMan_FreeOverlapLists(WorldLightMan* self)` (0x2f53b0):
/// release the 8 overlap lists at `+0x40`, 0x40 bytes apart.
pub fn world_light_man_free_overlap_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(0x40));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        loop {
            c.cycles += 6;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x40));
            call_ptr(c, 0x0031_7e50, 0x002f_53dc)?;
            c.g[V0][0] = (c.g[S1][0] < 8) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x002f_53d0);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Leaves: empty hooks.

/// `void RiderBase_RenderCheatCode(RiderBase* r)` (0x2f6908): render hook with
/// no cheat code to draw.
pub fn rider_base_render_cheat_code(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void* BoardSparkEmitter_ctor(BoardSparkEmitter* e)` (0x2f7a68): the
/// constructor returns its argument (the base member is initialised inline).
pub fn board_spark_emitter_ctor(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World light manager.

/// `WorldLightInstance* WorldLightMan_EvictOldestInstance(WorldLightMan* man,
/// int threshold)` (0x2f59d0): scan the 32 light instances at `+0x210`
/// (0x180 bytes each), keep the one with the smallest stamp at `+4` that is
/// below `threshold`, unlink it and move it to the free slot named by its low
/// five bits (`+0x3210`), then return it (or the last scanned instance).
pub fn world_light_man_evict_oldest_instance(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    c.g[T0][0] = 0;
    c.g[A3][0] = 0;
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x210));
    loop {
        c.cycles += 4;
        c.g[V1][0] = lw(c, p(c.g[A2][0] as u32, 4));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let skip = c.g[V0][0] == 0;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        if !skip {
            c.cycles += 2;
            c.g[A1][0] = c.g[V1][0];
            c.g[T0][0] = c.g[A2][0];
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[A3][0] < 0x20) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x180));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002f_59e0);
            }
            continue;
        }
        c.cycles += 6;
        c.g[V0][0] = c.ld(c.g[T0][0] as u32);
        c.g[V1][0] = lw(c, p(c.g[T0][0] as u32, 0x174));
        c.g[V0][0] = c.g[V0][0] << 24;
        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        let has_next = c.g[V1][0] != 0;
        c.g[V0][0] &= 0x1f;
        if has_next {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(c.g[T0][0] as u32, 0x170));
            if c.g[V0][0] != 0 {
                sw(c, p(c.g[V1][0] as u32, 0x170), c.g[V0][0]);
            } else {
                c.cycles += 2;
                sw(c, p(c.g[V1][0] as u32, 0x170), 0);
                c.cycles += 3;
                c.g[V0][0] = c.g[T0][0];
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V1][0] = lw(c, p(c.g[T0][0] as u32, 0x170));
            c.g[V0][0] = lw(c, p(c.g[T0][0] as u32, 0x174));
            sw(c, p(c.g[V1][0] as u32, 0x174), c.g[V0][0]);
        } else {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = lw(c, p(c.g[T0][0] as u32, 0x170));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            sw(c, p(c.g[V0][0] as u32, 0x3210), c.g[V1][0]);
            c.g[V0][0] = lw(c, p(c.g[T0][0] as u32, 0x170));
            if c.g[V0][0] != 0 {
                sw(c, p(c.g[V0][0] as u32, 0x174), 0);
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[T0][0];
                abi::ret(c);
                return;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[T0][0];
        abi::ret(c);
        return;
    }
}

// ---------------------------------------------------------------------------
// `Avalanche_DestroyAll` (0x002d9b40).
//
// `void Avalanche::DestroyAll()`: free every avalanche's node list and clear
// the manager's tables.

pub fn avalanche_destroy_all(c: &mut Cpu) {
    guest(c, |c| {
        'b_2d9ba8: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x002d9b5c);
            call_ptr(c, 0x002d9a80, 0x002d9b5c)?;
            c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
            let t = c.g[S1][0] == 0u64;
            c.g[A0][0] = 0x540000;
            if t {
                break 'b_2d9ba8;
            } else {
                c.cycles += 2;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                'l_2d9b70: loop {
                    'b_2d9b90: {
                        c.cycles += 2;
                        let t = c.g[S0][0] == 0u64;
                        c.g[S2][0] = c.g[S1][0];
                        if t {
                            break 'b_2d9b90;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S0][0];
                            'l_2d9b80: loop {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x002d9b88);
                                c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf4));
                                call_ptr(c, 0x00317e50, 0x002d9b88)?;
                                let t = c.g[S0][0] != 0u64;
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    if crate::events::event_due(c) {
                                        crate::events::deliver_event_here(c, 0x002d9b80);
                                    }
                                    continue 'l_2d9b80;
                                } else {
                                    break 'b_2d9b90;
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[RA][0] = sx(0x002d9b9c);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00317e50, 0x002d9b9c)?;
                    if c.g[S1][0] != 0u64 {
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x002d9b70);
                        }
                        continue 'l_2d9b70;
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = 0x540000;
                        break 'b_2d9ba8;
                    }
                }
            }
        }
        'b_2d9be8: {
            'b_2d9be4: {
                c.cycles += 9;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x9c0), 0u64);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7bb0));
                c.sw((c.g[GP][0] as u32).wrapping_add(0x9d0), 0u64);
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x002d9bc0);
                c.g[A2][0] = 0x400;
                call_ptr(c, 0x00416210, 0x002d9bc0)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c4));
                let t = c.g[A0][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c8));
                if t {
                    break 'b_2d9be4;
                } else {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_2d9be8;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x002d9bdc);
                        call_ptr(c, 0x00317e98, 0x002d9bdc)?;
                        c.sw((c.g[GP][0] as u32).wrapping_add(0x9c4), 0u64);
                        c.sw((c.g[GP][0] as u32).wrapping_add(0x9c8), 0u64);
                        break 'b_2d9be4;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_2d9be8;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[GP][0] as u32).wrapping_add(0x9cc), 0u64);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `WorldLightMan_AllocOverlapList(self)` (0x002f5298): build the eight
/// 0x40-byte overlap-list headers of the world light manager, each with an
/// empty 0xa14-byte list, then initialise the visibility flags and the 0x20
/// scratch slots.
pub fn world_light_man_alloc_overlap_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save(c, S0, 0x50);
        frame.save(c, S4, 0x10);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0xffff_ffff_ffff_0000 | 0xffff;
        c.g[S3][0] = c.g[A0][0];
        c.g[S2][0] = 0;
        c.g[S4][0] = 0x0048_0000;
        let this = c.g[S3][0];
        c.g[S0][0] = this;
        loop {
            c.cycles += 18;
            sw_3(c, c.g[S0][0], 0x10, c.g[S1][0]);
            c.g[A0][0] = 0xa14;
            sw_3(c, c.g[S0][0], 0x14, 0);
            c.g[A1][0] = sx(0x0048_7cd8);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x002f_52ec)?;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V1][0] = lw_3(c, this, 0);
            c.g[A0][0] = (c.g[S2][0] < 8) as u64;
            sw_3(c, c.g[S0][0], 0x40, c.g[V0][0]);
            sw_3(c, c.g[V0][0], 0, c.g[V1][0]);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
            sw_3(c, c.g[V0][0], 4, 0);
            sw_3(c, c.g[V0][0], 8, c.g[S1][0]);
            sw_3(c, c.g[V0][0], 0x10c, c.g[S1][0]);
            let more = c.g[A0][0] != 0;
            sw_3(c, c.g[V0][0], 0x210, c.g[S1][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002f_52d0);
            }
        }

        // Initialise the flags over the eight list headers.
        c.cycles += 8;
        c.g[A0][0] = sx(0xffff_0000 | 0xffff);
        c.g[V0][0] = sx(0x004f_f120);
        c.g[A3][0] = c.g[V0][0];
        c.g[A2][0] = sx((this as u32).wrapping_add(0x3210));
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.g[V1][0] = this;
        c.g[A1][0] = sx((this as u32).wrapping_add(0x3000));
        sw_3(c, c.g[V1][0], 0x214, 0);
        loop {
            c.cycles += 10;
            sw_3(c, c.g[V1][0], 0x210, c.g[A0][0]);
            c.lq(2, c.g[A3][0] as u32);
            sw_3(c, c.g[V1][0], 0x2d0, 0);
            c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x2c0));
            sw_3(c, c.g[V1][0], 0x380, 0);
            sw_3(c, c.g[V1][0], 0x384, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x180));
            c.g[V0][0] = (c.g[V1][0] < c.g[A1][0]) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            sw_3(c, c.g[V1][0], 0x214, 0);
            if event_due(c) {
                deliver_event_here(c, 0x002f_5338);
            }
        }

        c.cycles += 4;
        sw_3(c, this, 0x3290, 0);
        c.g[V1][0] = c.g[A2][0];
        c.g[S2][0] = 0;
        loop {
            c.cycles += 7;
            sw_3(c, c.g[V1][0], 0, 0);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V0][0] = (c.g[S2][0] < 0x20) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002f_5370);
            }
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

/// `void cBoardSparkFX_Reset(SparkFx* self)` (0x2daa78): reset a board spark

/// effect: the default position (`D_004FF130`), three zeroed words and two
/// copies of the default GS descriptor (`D_00501420`) with the blend factor,
/// texture page and alpha patched (the second with the CLUT bit). The
/// texture handle comes from `D_004A289C + 0xfa8`.
pub fn c_board_spark_fx_reset(c: &mut Cpu) {
    c.cycles += 83;
    c.g[V0][0] = 0x500000;
    c.g[T3][0] = 0x500000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
    c.g[T1][0] = 0xffffffffffbf0000;
    c.lq(3, c.pa(V0, 0));
    c.g[T1][0] = c.g[T1][0] | 0xffff;
    c.stw_2(A0, 0x70, 0u64);
    c.g[T6][0] = 0x400000;
    c.sq(3, c.pa(A0, 0x10));
    c.g[T0][0] = 0xfffffffffe7f0000;
    c.stw_2(A0, 0x74, 0u64);
    c.g[T0][0] = c.g[T0][0] | 0xffff;
    c.stw_2(A0, 0x78, 0u64);
    c.g[A3][0] = 0xfffffffffff00000;
    c.g[A2][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1420));
    c.ldl(2, c.pa(A2, 0x7));
    c.ldr(2, c.pa(A2, 0));
    c.ldl(3, c.pa(A2, 0xf));
    c.ldr(3, c.pa(A2, 0x8));
    c.g[A1][0] = c.ldw_2(A2, 0x10);
    c.sdl(2, c.pa(A0, 0x83));
    c.sdr(2, c.pa(A0, 0x7c));
    c.sdl(3, c.pa(A0, 0x8b));
    c.sdr(3, c.pa(A0, 0x84));
    c.stw_2(A0, 0x8c, c.g[A1][0]);
    c.g[A3][0] = c.g[A3][0] | 0xfff;
    c.g[A2][0] = 0xffffffffffcf0000;
    c.g[T5][0] = 0xfffffffffffffffc;
    c.g[V0][0] = c.ldw_2(A0, 0x80);
    c.g[A2][0] = c.g[A2][0] | 0xffff;
    c.g[V1][0] = c.ldw_2(A0, 0x84);
    c.g[T2][0] = 0xfffffffffffffc1f;
    c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
    c.g[T4][0] = 0xffffffffffffff83;
    c.g[V0][0] = c.g[V0][0] | c.g[T6][0];
    c.g[V1][0] = c.g[V1][0] & c.g[T2][0];
    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
    c.g[A1][0] = c.ldw_2(GP, -0x854);
    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
    c.g[V1][0] = c.g[V1][0] | 0xe0;
    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
    c.stw_2(A0, 0x84, c.g[V1][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
    c.g[V0][0] = c.g[V0][0] | 0x2;
    c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
    c.g[V0][0] = c.g[V0][0] | 0x1c;
    c.stw_2(A0, 0x80, c.g[V0][0]);
    c.g[V1][0] = c.ldw_2(A1, 0xfa8);
    c.sth_2(A0, 0x8c, c.g[V1][0]);
    c.g[A1][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1420));
    c.ldl(15, c.pa(A1, 0x7));
    c.ldr(15, c.pa(A1, 0));
    c.ldl(2, c.pa(A1, 0xf));
    c.ldr(2, c.pa(A1, 0x8));
    c.g[V1][0] = c.ldw_2(A1, 0x10);
    c.sdl(15, c.pa(A0, 0x97));
    c.sdr(15, c.pa(A0, 0x90));
    c.sdl(2, c.pa(A0, 0x9f));
    c.sdr(2, c.pa(A0, 0x98));
    c.stw_2(A0, 0xa0, c.g[V1][0]);
    c.g[V0][0] = c.ldw_2(A0, 0x94);
    c.g[V1][0] = c.ldw_2(A0, 0x98);
    c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
    c.g[T3][0] = c.ldw_2(GP, -0x854);
    c.g[V0][0] = c.g[V0][0] | c.g[T6][0];
    c.g[V1][0] = c.g[V1][0] & c.g[T2][0];
    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
    c.g[V1][0] = c.g[V1][0] | 0xe0;
    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
    c.stw_2(A0, 0x98, c.g[V1][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
    c.g[A1][0] = c.ldw_2(A0, 0x90);
    c.g[V0][0] = c.g[V0][0] | 0x9000;
    c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
    c.g[A1][0] = c.g[A1][0] | 0xc;
    c.g[V0][0] = c.g[V0][0] | 0x2;
    c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
    c.g[V0][0] = c.g[V0][0] | 0x1c;
    c.stw_2(A0, 0x94, c.g[V0][0]);
    c.g[V1][0] = c.ldw_2(T3, 0xfa8);
    c.stw_2(A0, 0x90, c.g[A1][0]);
    let ra = c.g[RA][0] as u32;
    c.sth_2(A0, 0xa0, c.g[V1][0]);
    c.pc = ra;
    return;
}

/// `cBoardTrailFX_Reset` (0x002e8560): reinitialise a board-trail effect:
/// clear the fade/alpha words, install the two default colours, and fill the
/// 54 trail vertices with the template colour at `gp+0x14c0..0x14c8`.
pub fn c_board_trail_fx_reset(c: &mut Cpu) {
    c.cycles += 28;
    let this = c.g[A0][0];
    let gp = c.g[GP][0] as u32;
    c.g[T1][0] = this;
    c.g[V0][0] = 0x2;
    c.sw((this as u32).wrapping_add(0x94), c.g[V0][0]);
    c.g[A1][0] = 0x50_0000;
    c.sw((this as u32).wrapping_add(0x90), 0u64);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xed0));
    c.sw((this as u32).wrapping_add(0x8c), 0u64);
    c.g[A0][0] = 0x50_0000;
    c.sw((this as u32).wrapping_add(0x98), 0u64);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xee0));
    c.sw((this as u32).wrapping_add(0x9c), 0u64);
    c.g[T3][0] = 0u64;
    c.sw((this as u32).wrapping_add(0xa0), 0u64);
    c.g[T2][0] = 0u64;
    c.sw((this as u32).wrapping_add(0xa4), 0u64);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0); // f0 = 1.0
    c.lq(3, c.g[A1][0] as u32); // the default colour quad ...
    c.mov_s(3, 0); // ... with alpha forced to 1.0
    c.sq(3, (this as u32).wrapping_add(0xb0));
    c.lq(2, c.g[A0][0] as u32);
    c.sq(2, (this as u32).wrapping_add(0xc0));
    c.lq(3, c.g[A0][0] as u32);
    c.swc1(0, (this as u32).wrapping_add(0x4));
    c.sq(3, (this as u32).wrapping_add(0xd0));
    c.swc1(0, (this as u32).wrapping_add(0x8));
    c.sw((this as u32).wrapping_add(0xe0), 0u64);
    let this = c.g[T1][0] as u32;
    loop {
        c.cycles += 70;
        c.g[V0][0] = c.lw(this.wrapping_add(0x18));
        c.g[V1][0] = c.g[T3][0] & 0x1;
        c.lwc1(0, gp.wrapping_add(0x14c8));
        c.mtc1(3, 2); // f2 = (int) the colour template's first word
        c.cvt_s_w(2, 2);
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
        c.swc1(0, c.g[V0][0] as u32);
        c.g[T0][0] = ((c.g[T3][0] as i64) < 54i64) as u64;
        c.g[V0][0] = c.lw(this.wrapping_add(0x10));
        c.lwc1(0, gp.wrapping_add(0x14c4));
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.swc1(0, c.g[V0][0] as u32);
        c.g[V1][0] = c.lw(this.wrapping_add(0x1c));
        c.lwc1(1, gp.wrapping_add(0x14c0));
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.swc1(1, c.g[V1][0] as u32);
        c.lwc1(0, gp.wrapping_add(0x14c0));
        c.g[V0][0] = c.lw(this.wrapping_add(0x20));
        c.sub_s(0, 3, 0);
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.swc1(0, c.g[V0][0] as u32);
        c.lwc1(1, gp.wrapping_add(0x14c4));
        c.g[V0][0] = c.lw(this.wrapping_add(0x14));
        c.sub_s(1, 3, 1);
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.swc1(1, c.g[V0][0] as u32);
        c.lwc1(0, gp.wrapping_add(0x14c8));
        c.g[V0][0] = c.lw(this.wrapping_add(0x24));
        c.sub_s(0, 3, 0);
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.swc1(0, c.g[V0][0] as u32);
        // Second channel: the same six words plus 4, at 1.0 / 1.0 / 1.0.
        c.g[A0][0] = c.lw(this.wrapping_add(0x24));
        c.g[A1][0] = c.lw(this.wrapping_add(0x14));
        c.g[A3][0] = c.lw(this.wrapping_add(0x18));
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A2][0] = c.lw(this.wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V1][0] = c.lw(this.wrapping_add(0x1c));
        c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V0][0] = c.lw(this.wrapping_add(0x20));
        c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.swc1(2, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
        c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.swc1(2, (c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw(this.wrapping_add(0x24));
        c.g[A1][0] = c.lw(this.wrapping_add(0x14));
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A3][0] = c.lw(this.wrapping_add(0x18));
        c.g[A2][0] = c.lw(this.wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A0][0] = c.lw(this.wrapping_add(0x1c));
        c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V0][0] = c.lw(this.wrapping_add(0x20));
        c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.swc1(3, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x30));
        c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x8));
        c.swc1(3, (c.g[A2][0] as u32).wrapping_add(0x8));
        let more = c.g[T0][0] != 0;
        c.swc1(3, (c.g[A3][0] as u32).wrapping_add(0x8));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002e_85d0);
            }
            continue;
        }
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
        return;
    }
}

pub(crate) mod cheat {

    /// The rider (`cRider*`).
    pub const RIDER: u32 = 0x00;

    /// Signed byte: 1 = the cheat is on, 0 = off, -1 = wrong input.
    pub const STATE: u32 = 0x04;

    /// Event and progress bits (bit 1: the cheat's variant flag).
    pub const FLAGS: u32 = 0x07;

    pub const STEP: u32 = 0x5;

    pub const COUNT: u32 = 0x6;

    /// The code: presses per step (9 signed bytes at 0x445e30).
    pub const CODE: u32 = 0x0044_5e30;

    pub const STEPS: u64 = 9;

    /// `gp+0xad0`: set to 8 when the full code is entered with both buttons.
    pub const GP_UNLOCK: u32 = 0xad0;
}

pub(crate) mod cheat_rider {

    pub const VELOCITY: u32 = 0x1e0;

    /// Three rates set to 60 while the cheat is on (0 when off).
    pub const RATE_A: u32 = 0x2e8;

    pub const RATE_B: u32 = 0x2ec;

    pub const RATE_C: u32 = 0x2f0;

    /// The movement mode (11 = cheat mode).
    pub const MODE: u32 = 0x2f4;

    /// 1.0 while the cheat is on.
    pub const CHEAT_SCALE: u32 = 0x2f8;

    /// Must be positive for the speed/steer assist to run.
    pub const ASSIST: u32 = 0x2fc;

    /// 0 while the cheat is on, 1 when off.
    pub const NORMAL_PHYSICS: u32 = 0x304;

    /// The cheat's variant flag (1 when off).
    pub const VARIANT: u32 = 0x308;

    pub const MAX_SPEED: u32 = 0x30c;

    /// How much of the speed follows the steer direction (0 = none).
    pub const STEER: u32 = 0x310;

    /// How much of the speed is pulled up to the maximum.
    pub const BLEND: u32 = 0x314;

    /// The steer direction.
    pub const STEER_DIR: u32 = 0x3a0;

    /// The physics sub-object (with its own vtable).
    pub const PHYSICS: u32 = 0x6c0;
}

pub(crate) mod cheat_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub const CLEAR: Callee = Callee {
        addr: 0x002f_64f0,
        f: ee::s09::RiderBase_ClearCheatCode,
    };

    pub const MATCH: Callee = Callee {
        addr: 0x002f_6910,
        f: ee::s09::RiderBase_MatchCheatCode,
    };

    /// `BELibrary_GetInterface` (through the function table).
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

    /// gp+: how many clears are still due.
    pub const CLEARS_DUE_GP: u32 = 0xad0;

    /// gp-: the world.
    pub const WORLD_GP: u32 = 0x848;

    /// gp-: .lit4 0.001 (minimum steer-direction length) and 0.0001
    /// (minimum velocity length).
    pub const MIN_DIR_LEN_GP: u32 = 0x3898;

    pub const MIN_VEL_LEN_GP: u32 = 0x3894;

    /// Non-zero: cheats switched off globally.
    pub const CHEATS_OFF: u32 = 0x0053_4b30;

    /// The zero vector the velocity is reset to.
    pub const ZERO_VEC: u32 = 0x004f_f130;

    pub const GET_EVENTS: Callee = Callee {
        addr: 0x002f_6b38,
        f: ee::s09::RiderBase_GetCheatCodeEvents,
    };

    pub const CLEAR_EVENTS: Callee = Callee {
        addr: 0x002f_6b28,
        f: ee::s09::RiderBase_ClearCheatCodeEvents,
    };
}

pub(crate) mod attack_fx {

    /// `RiderBase* owner`.
    #[cfg(test)]
    pub const OWNER: u32 = 0x00;

    /// `vec4 pos[4]`: the four spark positions.
    pub const POS: u32 = 0x10;

    /// `vec4 col[4]`: their colours `{1, c, c, c}`.
    pub const COL: u32 = 0x50;

    /// `float size[4]`.
    pub const SIZE: u32 = 0x90;

    /// `int active`.
    pub const ACTIVE: u32 = 0xa0;

    /// Rider: the attack strength (sparks while `> 0`).
    pub const R_STRENGTH: u32 = 0x350;

    /// Rider: facing flag (picks which hand's bone).
    pub const R_FLIP: u32 = 0x320;

    /// Rider: the model (`+0x30`: the bone matrix array, 64 bytes each).
    pub const R_MODEL: u32 = 0x780;

    pub const MODEL_BONES: u32 = 0x30;

    /// Rider: the anim controller.
    pub const R_ANIM: u32 = 0x784;

    /// Rider: the left / right hand bones.
    pub const R_BONE_A: u32 = 0x8b0;

    pub const R_BONE_B: u32 = 0x8b8;

    /// Anim states of the punches (track 0) and their track-1 variants.
    pub const PUNCH_A: u64 = 0x146;

    pub const PUNCH_B: u64 = 0x147;

    pub const PUNCH_A1: u64 = 0x143;

    pub const PUNCH_B1: u64 = 0x144;

    /// Anim table type of a track-1 sequence that carries the punch flags.
    pub const SEQ_TYPE_PUNCH: u64 = 0xd;

    /// `D_004FF120`: the matrix's replacement translation row.
    pub const ORIGIN_ROW: u32 = 0x004f_f120;

    /// `gp+0xa0c`: the effects random seed; `gp-0x3930`: colour scale 0.9.
    pub const RAND_SEED: u32 = 0xa0c;

    pub const COLOUR_SCALE: u32 = 0x3930;
}

/// `base + off` for a guest pointer held in a register.
#[inline(always)]
pub(crate) fn cheat_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// VU0 `|vf3.xyz|` into Q.
#[inline(always)]
pub(crate) fn cheat_vu0_length(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
}

/// VU0 `vf5 = vf3 / |vf3.xyz|` (all four lanes).
#[inline(always)]
pub(crate) fn cheat_vu0_normalise(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
}

/// `void RiderBase_UpdateCheatCode(CheatCode* self)` (0x2f6518): the
/// per-tick update of a rider's cheat code. Does nothing while the physics
/// sub-object's virtual query (slot 0x48) says so; otherwise runs a pending
/// clear, matches the input against the code, and switches the rider into
/// cheat mode 11 (rates 60, scale 1, variant flag from the state bits) or
/// back to mode 0. In cheat mode with a positive assist, pulls the speed up
/// toward the maximum (`max * blend + speed * (1 - blend)`), then turns the
/// velocity toward the steer direction (`dir * speed * steer + vel *
/// speed * (1 - steer)`, both normalised).
///
/// Tick rate: the 60.0 written to the three timers (+0x2e8/+0x2ec/+0x2f0)
/// is seconds (`Rider_UpdateBoostTimers` decays them by `time scale * 1/60`
/// from `.lit4` per tick, which follows the rate), and it is rewritten every
/// tick while the cheat is on: not a step. The speed blend (+0x314) and the
/// steer share (+0x310) are per-tick smoothing shares and follow the rate
/// (`cheat_per_tick_blend`).
pub fn rider_base_update_cheat_code(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: s0 = self; s0 and ra saved.
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save_ra(c, 0x60);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];

        // The physics sub-object's virtual query (vtable slot 0x48).
        c.g[A0][0] = c.lw(cheat_at(this, cheat::RIDER));
        let rider = c.g[A0][0];
        c.g[V1][0] = c.lw(cheat_at(rider, cheat_rider::PHYSICS));
        c.g[A0][0] = sx(cheat_at(rider, cheat_rider::PHYSICS));
        c.g[A1][0] = c.lh(cheat_at(c.g[V1][0], 0x48));
        c.g[V0][0] = c.lw(cheat_at(c.g[V1][0], 0x4c));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, method, 0x002f_6544)?;
        if c.g[V0][0] == 0 {
            update_cheat_code_body(c, frame, this)?;
        }

        // Epilogue.
        frame.restore(c, S0, 0x70);
        c.cycles += 3;
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// A per-tick blend share `a` (`x += (t - x) * a` once per tick at 60 Hz)
/// in FPR `f` as it becomes at the current rate: `1 - (1 - a)^(60/rate)`, the
/// same approach per unit of wall-clock time. At 60 the loaded value stays.
#[inline]
fn cheat_per_tick_blend(c: &mut Cpu, f: usize) {
    let hz = crate::tick::rate(c);
    if hz != 60 {
        let a = f32::from_bits(c.f[f]);
        c.f[f] = (1.0 - (1.0 - a).powf(60.0 / hz as f32)).to_bits();
    }
}

/// The body of `RiderBase_UpdateCheatCode` after the virtual query said 0;
/// `Ok` on every normal exit (all go to the epilogue).
pub(crate) fn update_cheat_code_body(c: &mut Cpu, frame: Frame, this: u64) -> crate::call::Flow {
    use cheat_callee as callee;
    use cheat_rider as rider_f;
    use ssx3_ee_rt::vu0::{Src, X};

    // A pending clear (while at least two are due).
    c.cycles += 4;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = c.lw(gp.wrapping_add(callee::CLEARS_DUE_GP));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        callee::CLEAR.call(c, 0x002f_6564)?;
        let at = (c.g[GP][0] as u32).wrapping_add(callee::CLEARS_DUE_GP);
        c.g[V0][0] = c.lw(at);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.sw(at, c.g[V0][0]);
    }

    // Match the input, query the back end (result unused), and honour the
    // global cheats-off switch.
    c.cycles += 10;
    c.g[A0][0] = c.g[S0][0];
    callee::MATCH.call(c, 0x002f_6578)?;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = c.lw(gp.wrapping_sub(callee::WORLD_GP));
    c.g[A1][0] = 0x7;
    c.g[A0][0] = c.lw(cheat_at(c.g[V0][0], 0x78));
    call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x002f_6588)?;
    c.g[V1][0] = 0x53_0000;
    c.g[V0][0] = c.lw(callee::CHEATS_OFF);
    if c.g[V0][0] != 0 {
        c.sb(cheat_at(this, cheat::STATE), 0);
    }

    // Enter or leave cheat mode.
    c.cycles += 4;
    c.g[V0][0] = c.lb(cheat_at(this, cheat::STATE));
    c.g[A1][0] = 0x1;
    if c.g[V0][0] != c.g[A1][0] {
        // Off: variant flag 1; leave mode 11 (if in it) for mode 0.
        c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.cycles += 6;
        c.g[V1][0] = 0xb;
        c.sw(cheat_at(c.g[V0][0], rider_f::VARIANT), c.g[A1][0]);
        c.g[A0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.g[V0][0] = c.lw(cheat_at(c.g[A0][0], rider_f::MODE));
        if c.g[V0][0] != c.g[V1][0] {
            return Ok(());
        }
        c.cycles += 11;
        c.sw(cheat_at(c.g[A0][0], rider_f::MODE), 0);
        c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.sw(cheat_at(c.g[V0][0], rider_f::CHEAT_SCALE), 0);
        c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.sw(cheat_at(c.g[V1][0], rider_f::RATE_C), 0);
        c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.sw(cheat_at(c.g[V0][0], rider_f::RATE_B), 0);
        c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.sw(cheat_at(c.g[V1][0], rider_f::RATE_A), 0);
        c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.sw(cheat_at(c.g[V0][0], rider_f::NORMAL_PHYSICS), c.g[A1][0]);
    } else {
        // On: mode 11, scale 1.0, rates 60.0, variant from flag bit 1.
        c.cycles += 23;
        c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.g[V0][0] = 0xb;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.sw(cheat_at(c.g[V1][0], rider_f::MODE), c.g[V0][0]);
        c.g[AT][0] = 0x4270_0000;
        c.mtc1(AT, 0);
        c.g[V0][0] = c.lbu(cheat_at(this, cheat::FLAGS));
        c.g[A0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[V0][0] &= 0x1;
        c.sw(cheat_at(c.g[A0][0], rider_f::VARIANT), c.g[V0][0]);
        c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.swc1(1, cheat_at(c.g[V1][0], rider_f::CHEAT_SCALE));
        c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.swc1(0, cheat_at(c.g[V0][0], rider_f::RATE_C));
        c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.swc1(0, cheat_at(c.g[V1][0], rider_f::RATE_B));
        c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.swc1(0, cheat_at(c.g[V0][0], rider_f::RATE_A));
        c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
        c.sw(cheat_at(c.g[V1][0], rider_f::NORMAL_PHYSICS), 0);
    }

    // The assist runs only in cheat mode with a positive assist value.
    c.cycles += 5;
    c.g[A0][0] = c.lw(cheat_at(this, cheat::RIDER));
    c.g[V1][0] = 0xb;
    c.g[V0][0] = c.lw(cheat_at(c.g[A0][0], rider_f::MODE));
    if c.g[V0][0] != c.g[V1][0] {
        return Ok(());
    }
    c.cycles += 6;
    let rider = c.g[A0][0];
    c.lwc1(0, cheat_at(rider, rider_f::ASSIST));
    c.mtc1(0, 1);
    c.c_lt_s(1, 0);
    if !c.fcc() {
        return Ok(());
    }

    // speed = |vel|; below the maximum (and moving): vel = norm(vel) *
    // (max * blend + speed * (1 - blend)), speed = |vel|.
    c.cycles += 14;
    c.lwc1(0, cheat_at(rider, rider_f::MAX_SPEED));
    c.lqc2(3, cheat_at(rider, rider_f::VELOCITY));
    cheat_vu0_length(c);
    c.cfc2(V0, 22);
    c.mtc1(V0, 3);
    c.c_lt_s(3, 0);
    if c.fcc() {
        c.cycles += 3;
        c.c_eq_s(3, 1);
        if !c.fcc() {
            c.cycles += 39;
            c.lqc2(3, cheat_at(rider, rider_f::VELOCITY));
            cheat_vu0_normalise(c);
            c.sqc2(5, frame.at(c, 0x10));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 1);
            c.lq(V0, frame.at(c, 0x10));
            c.sq(V0, frame.at(c, 0x0));
            c.sq(V0, cheat_at(rider, rider_f::VELOCITY));
            c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
            let r = c.g[V1][0];
            c.lwc1(2, cheat_at(r, rider_f::BLEND));
            cheat_per_tick_blend(c, 2);
            c.lwc1(0, cheat_at(r, rider_f::MAX_SPEED));
            c.sub_s(1, 1, 2);
            c.mul_s(0, 0, 2);
            c.mul_s(1, 3, 1);
            c.add_s(0, 0, 1);
            c.mfc1(A0, 0);
            c.lqc2(4, cheat_at(r, rider_f::VELOCITY));
            c.qmtc2(A0, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, cheat_at(r, rider_f::VELOCITY));
            c.g[V0][0] = c.lw(cheat_at(this, cheat::RIDER));
            c.lqc2(3, cheat_at(c.g[V0][0], rider_f::VELOCITY));
            cheat_vu0_length(c);
            c.cfc2(V1, 22);
            c.mtc1(V1, 3);
            c.g[A0][0] = c.lw(cheat_at(this, cheat::RIDER));
        }
    }

    // Steering: only with a non-zero steer, a steer direction longer than
    // 0.001 and a velocity longer than 0.0001.
    c.cycles += 6;
    c.mtc1(0, 2);
    let rider = c.g[A0][0];
    c.lwc1(0, cheat_at(rider, rider_f::STEER));
    c.c_eq_s(0, 2);
    if c.fcc() {
        return Ok(());
    }
    c.cycles += 15;
    let gp = c.g[GP][0] as u32;
    c.lwc1(0, gp.wrapping_sub(callee::MIN_DIR_LEN_GP));
    c.lqc2(3, cheat_at(rider, rider_f::STEER_DIR));
    cheat_vu0_length(c);
    c.cfc2(V0, 22);
    c.mtc1(V0, 1);
    c.c_lt_s(0, 1);
    if !c.fcc() {
        return Ok(());
    }
    c.cycles += 15;
    c.lwc1(0, gp.wrapping_sub(callee::MIN_VEL_LEN_GP));
    c.lqc2(3, cheat_at(rider, rider_f::VELOCITY));
    cheat_vu0_length(c);
    c.cfc2(V0, 22);
    c.mtc1(V0, 1);
    c.c_lt_s(0, 1);
    if !c.fcc() {
        return Ok(());
    }

    // vel = norm(dir) * (speed * steer) + norm(vel) * (speed * (1 - steer)).
    c.cycles += 61;
    c.lqc2(3, cheat_at(rider, rider_f::STEER_DIR));
    cheat_vu0_normalise(c);
    c.sqc2(5, frame.at(c, 0x30));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    c.lq(V0, frame.at(c, 0x30));
    c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
    c.sq(V0, frame.at(c, 0x20));
    c.lwc1(0, cheat_at(c.g[V1][0], rider_f::STEER));
    cheat_per_tick_blend(c, 0);
    c.mul_s(0, 3, 0);
    c.mfc1(V1, 0);
    c.lqc2(4, frame.at(c, 0x20));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0x30));
    c.lq(V0, frame.at(c, 0x30));
    c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
    c.sq(V0, frame.at(c, 0x10));
    c.lqc2(3, cheat_at(c.g[V1][0], rider_f::VELOCITY));
    cheat_vu0_normalise(c);
    c.sqc2(5, frame.at(c, 0x50));
    c.lq(V0, frame.at(c, 0x50));
    c.g[V1][0] = c.lw(cheat_at(this, cheat::RIDER));
    c.sq(V0, frame.at(c, 0x40));
    c.lwc1(0, cheat_at(c.g[V1][0], rider_f::STEER));
    cheat_per_tick_blend(c, 0);
    c.sub_s(1, 1, 0);
    c.mul_s(1, 3, 1);
    c.mfc1(V1, 1);
    c.lqc2(4, frame.at(c, 0x40));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0x50));
    c.lq(V0, frame.at(c, 0x50));
    c.sq(V0, frame.at(c, 0x30));
    c.lqc2(3, frame.at(c, 0x10));
    c.lqc2(4, frame.at(c, 0x30));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x50));
    c.lq(V0, frame.at(c, 0x50));
    c.sq(V0, frame.at(c, 0x0));
    c.sq(V0, cheat_at(rider, rider_f::VELOCITY));

    // The original's `x < 0 && x >= 0` guard (never both for a compare
    // without NaN): reset the velocity to the zero vector.
    c.g[A0][0] = c.lw(cheat_at(this, cheat::RIDER));
    let rider = c.g[A0][0];
    c.lwc1(0, cheat_at(rider, rider_f::VELOCITY));
    c.c_lt_s(0, 2);
    if !c.fcc() {
        return Ok(());
    }
    c.cycles += 4;
    c.c_le_s(2, 0);
    c.g[V0][0] = 0x50_0000;
    if !c.fcc() {
        return Ok(());
    }
    c.cycles += 4;
    c.g[V0][0] = sx(callee::ZERO_VEC);
    c.lq(A3, callee::ZERO_VEC);
    c.sq(A3, cheat_at(rider, rider_f::VELOCITY));
    Ok(())
}

/// One step of the effects LCG: `prod = seed (reg) * 0x18fcd` (mult into
/// `prod`), as the original interleaves it.
#[inline(always)]
pub(crate) fn afx_mult(c: &mut Cpu, prod: usize, seed: usize) {
    c.mult(0, prod, seed, A1);
}

/// `out = ((prod + 0xe9507c) & 0x7fffff) | 0x3f800000`: the new seed, also a
/// float in [1, 2).
#[inline(always)]
pub(crate) fn afx_bits(c: &mut Cpu, out: usize, prod: usize) {
    c.g[out][0] = (sx(r32(c, prod).wrapping_add(r32(c, T4))) & c.g[A0][0]) | c.g[T1][0];
}

/// Store the seed in `reg` and read it back as a float minus 1 (`[0, 1)`).
#[inline(always)]
pub(crate) fn afx_store_unit(c: &mut Cpu, reg: usize) -> crate::ps2f::FpuF32 {
    use crate::ps2f::FpuF32;
    let seed_at = crate::guest::Addr(r32(c, GP).wrapping_add(attack_fx::RAND_SEED));
    seed_at.sw(c, c.g[reg][0]);
    seed_at.read_fpu(c) - FpuF32(c.f[6])
}

/// `void RiderBase_UpdateAttackFX(AttackFX* self)` (0x2f1150): the sparks
/// of a rider's punch. Active while the owner's attack strength
/// (`+0x350`) is positive. The owner's anim controller decides the hand:
/// punch A (track 0 state 0x146, or track 1 state 0x143 with a type-0xd
/// sequence whose flag bit 0 is set and bit 1 clear) uses bone `+0x8b0`
/// (`+0x8b8` when `+0x320` is clear), punch B (0x147 / 0x144) the other;
/// anything else switches the effect off. When active, the hand's bone
/// translation plus `(8, 0, 0)` through the bone matrix (translation
/// replaced by `D_004FF120`) is the centre, and four sparks get a random
/// offset of `±3s` per axis, a size `15 .. 15 + 15s` and a colour `{1, c,
/// c, c}` with `c = 0.9 s rand` (`s` the strength). Frame 0xf0 (s0-s4).
pub fn rider_base_update_attack_fx(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use attack_fx::*;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xf0);
        c.f[1] = 0;
        for (reg, off) in [(S0, 0xe0), (S1, 0xd0)] {
            frame.save(c, reg, off);
        }
        c.g[S0][0] = c.g[A0][0];
        for (reg, off) in [(S2, 0xc0), (S3, 0xb0), (S4, 0xa0)] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x90);
        let this = Addr(r32(c, S0));
        let owner = this.lw(c);
        c.g[V0][0] = owner;
        let strength = Addr((owner as u32).wrapping_add(R_STRENGTH)).read_fpu(c);
        c.f[0] = strength.0;
        let on = FpuF32(0).lt(strength);
        crate::abi::set_fcc(c, on);
        c.g[V1][0] = 1;
        if !on {
            c.cycles += 1;
            c.g[V1][0] = 0;
        }
        c.cycles += 2;
        this.off(ACTIVE).sw(c, c.g[V1][0]);
        if !on {
            c.cycles += 1;
            frame.restore(c, S0, 0xe0);
        } else {
            // The anim states of tracks 0 and 1, and the punch flags of a
            // type-0xd track-1 sequence (bit 1 clear, bit 0 set).
            c.cycles += 18;
            c.g[A0][0] = Addr((owner as u32).wrapping_add(R_ANIM)).lw(c);
            c.g[A1][0] = 0;
            c.g[S1][0] = Addr((owner as u32).wrapping_add(R_BONE_A)).lw(c);
            c.g[S2][0] = 0;
            call_ptr(c, 0x0031_2aa0, 0x002f_11ac)?;
            c.g[S4][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            let owner = this.lw(c);
            c.g[V0][0] = owner;
            c.g[A0][0] = Addr((owner as u32).wrapping_add(R_ANIM)).lw(c);
            call_ptr(c, 0x0031_2aa0, 0x002f_11c0)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            let owner = this.lw(c);
            c.g[V0][0] = owner;
            c.g[A0][0] = Addr((owner as u32).wrapping_add(R_ANIM)).lw(c);
            call_ptr(c, 0x0031_1ae8, 0x002f_11d4)?;
            c.g[V1][0] = SEQ_TYPE_PUNCH;
            let seq_type = c.g[V0][0];
            c.g[V0][0] = PUNCH_A;
            if seq_type == SEQ_TYPE_PUNCH {
                // Flag bit `bit` of the track-1 sequence (`+0xb0`, 64 bits).
                let test_flag =
                    |c: &mut Cpu, bit: u64, ret1: u32, ret2: u32| -> crate::call::Flow {
                        c.cycles += 9;
                        let owner = this.lw(c);
                        c.g[V0][0] = owner;
                        c.g[A1][0] = 1;
                        c.g[A0][0] = Addr((owner as u32).wrapping_add(R_ANIM)).lw(c);
                        call_ptr(c, 0x0031_1b20, ret1)?;
                        c.g[A0][0] = sx(r32(c, V0).wrapping_add(0xb0));
                        c.g[A1][0] = bit;
                        call_ptr(c, 0x0014_46a0, ret2)
                    };
                test_flag(c, 1, 0x002f_11f0, 0x002f_11fc)?;
                let bit1 = c.g[V0][0] != 0;
                c.g[V0][0] = PUNCH_A;
                if !bit1 {
                    test_flag(c, 0, 0x002f_1214, 0x002f_1220)?;
                    c.g[S2][0] = (c.g[V0][0] != 0) as u64;
                    c.g[V0][0] = PUNCH_A;
                }
            }
            let (track0, track1, flagged) = (c.g[S4][0], c.g[S3][0], c.g[S2][0] != 0);

            // Punch A?
            c.cycles += 2;
            let mut punch_a = track0 == PUNCH_A;
            c.g[V0][0] = PUNCH_A1;
            if !punch_a {
                c.cycles += 2;
                let t1_match = track1 == PUNCH_A1;
                c.g[V0][0] = PUNCH_B;
                if t1_match {
                    c.cycles += 2;
                    punch_a = flagged;
                }
            }
            if punch_a {
                c.cycles += 4;
                let owner = this.lw(c);
                c.g[A0][0] = owner;
                c.g[V0][0] = Addr((owner as u32).wrapping_add(R_FLIP)).lw(c);
                let bone = if c.g[V0][0] == 0 {
                    R_BONE_B
                } else {
                    c.cycles += 2;
                    R_BONE_A
                };
                c.g[S1][0] = Addr((owner as u32).wrapping_add(bone)).lw(c);
            } else {
                // Punch B?
                c.cycles += 2;
                let mut punch_b = track0 == PUNCH_B;
                c.g[V0][0] = PUNCH_B1;
                if !punch_b {
                    c.cycles += 2;
                    if track1 == PUNCH_B1 {
                        c.cycles += 2;
                        punch_b = flagged;
                    }
                }
                if punch_b {
                    c.cycles += 4;
                    let owner = this.lw(c);
                    c.g[A0][0] = owner;
                    c.g[V0][0] = Addr((owner as u32).wrapping_add(R_FLIP)).lw(c);
                    let bone = if c.g[V0][0] == 0 {
                        R_BONE_A
                    } else {
                        c.cycles += 1;
                        R_BONE_B
                    };
                    c.g[S1][0] = Addr((owner as u32).wrapping_add(bone)).lw(c);
                } else {
                    this.off(ACTIVE).sw(c, 0);
                }
            }

            c.cycles += 3;
            c.g[V0][0] = this.off(ACTIVE).lw(c);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0xe0);
            } else {
                afx_emit_sparks(c);
                c.cycles += 1;
                frame.restore(c, S0, 0xe0);
            }
        }

        c.cycles += 7;
        for (reg, off) in [(S1, 0xd0), (S2, 0xc0), (S3, 0xb0), (S4, 0xa0)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x90);
        frame.close(c);
        Ok(())
    });
}

/// The spark positions, sizes and colours (s0 = self, s1 = bone): the
/// 66-cycle setup and the four 85-cycle iterations. Frame: `base` at
/// 0x00, the bone matrix copy at 0x10, the offset at 0x60, scratch
/// vectors at 0x50/0x70/0x80.
pub(crate) fn afx_emit_sparks(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use attack_fx::*;
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    c.cycles += 66;
    let s = sp(c);
    let owner = Addr(r32(c, S0)).lw(c);
    c.g[A1][0] = owner;
    c.g[A0][0] = sx(r32(c, S1) << 6);
    c.g[A2][0] = sx(s.wrapping_add(0x10));
    // base = bones[bone].row3 (the bone's translation).
    let model = Addr((owner as u32).wrapping_add(R_MODEL)).lw(c);
    c.g[V1][0] = model;
    let bones = Addr((model as u32).wrapping_add(MODEL_BONES)).lw(c);
    c.g[V0][0] = sx((bones as u32).wrapping_add(r32(c, A0)));
    c.lq(V1, r32(c, V0).wrapping_add(0x30));
    c.sq(V1, s);
    // m = bones[bone], with row 3 = D_004FF120.
    let model = Addr((owner as u32).wrapping_add(R_MODEL)).lw(c);
    c.g[V0][0] = model;
    let bones = Addr((model as u32).wrapping_add(MODEL_BONES)).lw(c);
    c.g[V1][0] = bones;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(bones as u32));
    let mat = r32(c, A0);
    let copy = r32(c, A2);
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, mat.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, copy.wrapping_add(0x10 * i));
    }
    c.g[V0][0] = sx(ORIGIN_ROW);
    c.g[AT][0] = 0x4100_0000;
    c.f[0] = 0x4100_0000; // 8.0
                          // The LCG constants: a1 = 0x18fcd, t4 = 0xe9507c, a0 = 0x7fffff,
                          // t1 = 1.0f.
    c.g[A1][0] = 0x1_8fcd;
    c.lq(V1, ORIGIN_ROW);
    c.g[T4][0] = 0xe9_507c;
    Addr(s.wrapping_add(0x70)).write_fpu(c, FpuF32(c.f[0]));
    c.g[A0][0] = 0x7f_ffff;
    c.sq(V1, s.wrapping_add(0x40));
    c.g[T1][0] = 0x3f80_0000;
    for off in [0x74, 0x78, 0x7c] {
        Addr(s.wrapping_add(off)).sw(c, 0);
    }
    // rnd = m * (8, 0, 0, 0); base += rnd.
    c.lqc2(8, s.wrapping_add(0x70));
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, copy.wrapping_add(0x10 * i));
    }
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, s.wrapping_add(0x80));
    c.g[AT][0] = 0x4040_0000;
    c.f[9] = 0x4040_0000; // 3.0
    c.lq(V0, s.wrapping_add(0x80));
    c.g[T0][0] = sx(r32(c, S0).wrapping_add(SIZE));
    c.g[AT][0] = 0x4170_0000;
    c.f[7] = 0x4170_0000; // 15.0
    c.g[A3][0] = c.g[S0][0];
    c.sq(V0, s.wrapping_add(0x60));
    c.g[A2][0] = 3;
    c.lqc2(3, s);
    c.lqc2(4, s.wrapping_add(0x60));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x80));
    c.g[AT][0] = 0x3f80_0000;
    c.f[6] = 0x3f80_0000; // 1.0
    c.lq(V0, s.wrapping_add(0x80));
    c.f[8] = Addr(r32(c, GP).wrapping_sub(COLOUR_SCALE)).read_fpu(c).0;
    c.sq(V0, s);
    c.sq(V0, s.wrapping_add(0x50));

    // Four sparks: a2 counts 3..0, a3 walks self, t0 the sizes.
    loop {
        c.cycles += 85;
        let s = sp(c);
        let (one, fifteen, three) = (FpuF32(c.f[6]), FpuF32(c.f[7]), FpuF32(c.f[9]));
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(RAND_SEED)).lw(c);
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        let owner = Addr(r32(c, S0)).lw(c);
        c.g[V1][0] = owner;
        afx_mult(c, T2, V0);
        let strength = Addr((owner as u32).wrapping_add(R_STRENGTH)).read_fpu(c);
        Addr(s.wrapping_add(0x6c)).sw(c, 0);
        // Offset range ±3s, size 15 .. 15 + 15s.
        let spread = strength * three;
        let lo = -spread;
        let hi_size = strength * fifteen + fifteen;
        let width = spread - lo;
        afx_bits(c, V0, T2);
        afx_mult(c, T2, V0);
        let rx = afx_store_unit(c, V0);
        afx_bits(c, V1, T2);
        afx_mult(c, T2, V1);
        let ry = afx_store_unit(c, V1);
        afx_bits(c, V0, T2);
        let ox = lo + width * rx;
        let rz = afx_store_unit(c, V0);
        let oy = lo + width * ry;
        Addr(s.wrapping_add(0x60)).write_fpu(c, ox);
        Addr(s.wrapping_add(0x64)).write_fpu(c, oy);
        let oz_part = width * rz;
        let oz = lo + oz_part;
        Addr(s.wrapping_add(0x68)).write_fpu(c, oz);
        // pos[i] = base + offset.
        c.lqc2(3, s);
        c.lqc2(4, s.wrapping_add(0x60));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, s.wrapping_add(0x70));
        c.g[V1][0] = Addr(r32(c, GP).wrapping_add(RAND_SEED)).lw(c);
        c.lq(V0, s.wrapping_add(0x70));
        afx_mult(c, T2, V1);
        c.sq(V0, r32(c, A3).wrapping_add(POS));
        c.sq(V0, s.wrapping_add(0x50));
        // size[i] = 15 + (hi - 15) * rand.
        afx_bits(c, V1, T2);
        let size_range = hi_size - fifteen;
        let rs = afx_store_unit(c, V1);
        afx_mult(c, V0, V1);
        afx_bits(c, V1, V0);
        let size = size_range * rs + fifteen;
        Addr(r32(c, T0)).write_fpu(c, size);
        c.g[T0][0] = sx(r32(c, T0).wrapping_add(4));
        // col[i] = {1, c, c, c}, c = 0.9 s rand.
        let owner = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = owner;
        let strength = Addr((owner as u32).wrapping_add(R_STRENGTH)).read_fpu(c);
        let scaled = strength * FpuF32(c.f[8]);
        Addr(s.wrapping_add(0x50)).write_fpu(c, one);
        let rc = afx_store_unit(c, V1);
        let tint = scaled * rc;
        for off in [0x5c, 0x54, 0x58] {
            Addr(s.wrapping_add(off)).write_fpu(c, tint);
        }
        c.ldl(V1, s.wrapping_add(0x57));
        c.ldr(V1, s.wrapping_add(0x50));
        c.ldl(T2, s.wrapping_add(0x5f));
        c.ldr(T2, s.wrapping_add(0x58));
        let a3 = r32(c, A3);
        c.sdl(V1, a3.wrapping_add(COL + 7));
        c.sdr(V1, a3.wrapping_add(COL));
        c.sdl(T2, a3.wrapping_add(COL + 0xf));
        c.sdr(T2, a3.wrapping_add(COL + 8));
        // The FPU registers as the original leaves them.
        c.f[0] = rc.0;
        c.f[1] = tint.0;
        c.f[2] = ox.0;
        c.f[3] = oz_part.0;
        c.f[4] = size.0;
        c.f[5] = oz.0;
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[A3][0] = sx(a3.wrapping_add(0x10));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002f_1398);
            }
            continue;
        }
        break;
    }
}

// ---------------------------------------------------------------------------
// WorldLightMan_GetInstanceLighting (0x002f5400)
// ---------------------------------------------------------------------------

/// `WorldLightMan` fields: the world, the instance nodes (32 of 0x180
/// bytes), the 32 hash heads and the node count.
pub(crate) const WLM_WORLD: u32 = 0x4;

pub(crate) const WLM_NODES: u32 = 0x210;

pub(crate) const WLM_NODE_SIZE: u32 = 0x180;

pub(crate) const WLM_MAX_NODES: u64 = 0x20;

pub(crate) const WLM_HEADS: u32 = 0x3210;

pub(crate) const WLM_COUNT: u32 = 0x3290;

/// Instance node fields: id, priority, current lighting (0xa0 bytes),
/// position, overlap region, target lighting (0xa0 bytes), hash links.
pub(crate) const WLN_ID: u32 = 0x0;

pub(crate) const WLN_PRIORITY: u32 = 0x4;

pub(crate) const WLN_CUR: u32 = 0x10;

pub(crate) const WLN_POS: u32 = 0xb0;

pub(crate) const WLN_BOX: u32 = 0xc0;

pub(crate) const WLN_TARGET: u32 = 0xd0;

pub(crate) const WLN_NEXT: u32 = 0x170;

pub(crate) const WLN_PREV: u32 = 0x174;

/// Overlap region fields: id, priority, bounds (min, max), cell.
pub(crate) const WLB_ID: u32 = 0x0;

pub(crate) const WLB_PRIORITY: u32 = 0x4;

pub(crate) const WLB_MIN: u32 = 0x10;

pub(crate) const WLB_MAX: u32 = 0x20;

pub(crate) const WLB_CELL: u32 = 0x30;

/// Instance fields: the model (0 for a plain instance), the position, the
/// lighting key.
pub(crate) const INST_MODEL: u32 = 0xc;

pub(crate) const INST_POS: u32 = 0x40;

pub(crate) const INST_KEY: u32 = 0x78;

/// gp-relative globals: the frame state (`+0x18` the priority stamp), the
/// irradiance override flag, the default irradiance index; `.lit4` -99999.
pub(crate) const GP_FRAME_STATE: u32 = 0x2a74;

pub(crate) const GP_IRRADIANCE_OVERRIDE: u32 = 0xa74;

pub(crate) const GP_DEFAULT_IRRADIANCE: u32 = 0x12d4;

pub(crate) const GP_NEG_99999: u32 = 0x38b4;

/// The irradiance table and the "resolved" tag of an irradiance reference.
pub(crate) const IRRADIANCE_TABLE: u32 = 0x0050_47f8;

#[cfg(test)]
pub(crate) const IRRADIANCE_RESOLVED: u64 = 0x12_3400;

pub(crate) const WLM_EVICT_OLDEST: Callee = Callee {
    addr: 0x002f_59d0,
    f: ee::s09::f_002f59d0,
};

pub(crate) const WLM_GET_OVERLAP_REGION: Callee = Callee {
    addr: 0x002f_5a70,
    f: ee::s09::f_002f5a70,
};

pub(crate) const IRRADIANCE_GET_OVERRIDE: Callee = Callee {
    addr: 0x002e_e2f8,
    f: ee::s09::f_002ee2f8,
};

pub(crate) const WLM_ADD_POINT_LIGHTS: Callee = Callee {
    addr: 0x002f_5af0,
    f: ee::s09::f_002f5af0,
};

pub(crate) const INSTANCE_GET_NODE_MATRIX: u32 = 0x0034_fed8;

pub(crate) const OVERLAP_LIST_GATHER_TYPE2: u32 = 0x0033_1450;

pub(crate) const IRRADIANCE_FIND_INDEX: u32 = 0x0038_ac50;

pub(crate) const IRRADIANCE_GET_ENTRY: u32 = 0x0038_abf8;

/// Copy a lighting set (0xa0 bytes, 0x20 per step) from `v0` to `v1` up
/// to `end` (in `a0` or `s7`), with `lq`/`sq` through `t0`/`t1` (GPR
/// numbers `lo`, `lo + 1`); back-edge event checks at `head`.
pub(crate) fn copy_lighting(c: &mut Cpu, end_reg: usize, lo: usize, head: u32) {
    loop {
        c.cycles += 7;
        let src = r32(c, V0);
        let dst = r32(c, V1);
        c.lq(lo, src);
        c.lq(lo + 1, src + 0x10);
        c.sq(lo, dst);
        c.sq(lo + 1, dst + 0x10);
        c.g[V0][0] = sx(src.wrapping_add(0x20));
        let more = c.g[V0][0] != c.g[end_reg][0];
        c.g[V1][0] = sx(dst.wrapping_add(0x20));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The instance's position into `sp+0` (and `sp+0x10` / `sp+0x50`): the
/// model node matrix's translation (`v0`, the model, non-zero; `a0` set by
/// the caller), or the plain instance position.
pub(crate) fn instance_position(
    c: &mut Cpu,
    s: u32,
    matrix_cycles: u64,
    plain_cycles: u64,
    ret: u32,
) -> crate::call::Flow {
    let obj = r32(c, S0);
    if c.g[V0][0] != 0 {
        c.cycles += matrix_cycles;
        c.g[A1][0] = 0;
        c.g[A2][0] = sx(s + 0x10);
        call_ptr(c, INSTANCE_GET_NODE_MATRIX, ret)?;
        c.lq(V0, s + 0x40);
        c.sq(V0, s);
        c.sq(V0, s + 0x50);
    } else {
        c.cycles += plain_cycles;
        c.lq(V0, obj + INST_POS);
        c.sq(V0, s);
        c.sq(V0, s + 0x10);
    }
    Ok(())
}

/// Resolve the irradiance reference in `s0` (tagging it resolved), fetch
/// its entry and copy it into the node's target (`s4`).
pub(crate) fn resolve_irradiance(
    c: &mut Cpu,
    ret_find: u32,
    ret_get: u32,
    head: u32,
) -> crate::call::Flow {
    c.cycles += 4;
    let r = r32(c, S0);
    c.g[V0][0] = Addr(r + 4).lw(c);
    c.g[S1][0] |= 0x3400;
    c.g[S5][0] = 0x50_0000;
    if c.g[V0][0] != c.g[S1][0] {
        c.cycles += 5;
        c.g[A0][0] = sx(IRRADIANCE_TABLE);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, IRRADIANCE_FIND_INDEX, ret_find)?;
        Addr(r).sw(c, c.g[V0][0]);
        Addr(r + 4).sw(c, c.g[S1][0]);
    }
    c.cycles += 5;
    c.g[A1][0] = Addr(r).lw(c);
    c.g[A0][0] = sx(IRRADIANCE_TABLE);
    call_ptr(c, IRRADIANCE_GET_ENTRY, ret_get)?;
    c.g[V1][0] = c.g[S4][0];
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(0xa0));
    copy_lighting(c, A0, A1, head);
    c.cycles += 2;
    c.g[V0][0] = c.g[S4][0];
    Ok(())
}

/// `Lighting* WorldLightMan_GetInstanceLighting(WorldLightMan* self,
/// Instance* inst, int force)` (0x002f5400): the lighting set for a world
/// instance, cached per instance key in 32 hashed nodes. A new instance
/// takes a free node (or evicts the oldest) and is lit from scratch; a
/// cached one is re-lit only when `force` is set and it moved more than 10
/// units (its irradiance reloaded past 500). Re-lighting refreshes the
/// node's overlap region (a fresh one, or rebuilt as a 1000-unit box when
/// the position left it), reloads the irradiance (override, the world's
/// two irradiance references, or the default entry), and adds the point
/// lights to the copy. Returns `&node->cur`.
pub fn world_light_man_get_instance_lighting(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x120);
        frame.save(c, S0, 0x110);
        frame.save(c, S1, 0x100);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0xe0);
        c.g[S1][0] = 0;
        frame.save(c, S5, 0xc0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S6, 0xb0);
        frame.save(c, S2, 0xf0);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S4, 0xd0);
        frame.save(c, S7, 0xa0);
        frame.save(c, FP, 0x90);
        frame.save_ra(c, 0x80);
        let s = sp(c);
        let obj = r32(c, S0);
        let this = r32(c, S3);
        let gp = r32(c, GP);

        // The instance's hash chain.
        let key = Addr(obj + INST_KEY).lw(c);
        c.g[A0][0] = key;
        c.g[V1][0] = sx((key as u32) >> 8);
        c.g[S5][0] = c.g[V1][0] & 0x1f;
        let slot = r32(c, S5);
        c.g[V0][0] = sx(this.wrapping_add(slot << 2));
        c.g[S2][0] = Addr(r32(c, V0) + WLM_HEADS).lw(c);
        c.g[A1][0] = Addr(gp + GP_FRAME_STATE).lw(c);
        if c.g[S2][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S2) + WLN_ID).lw(c);
            if c.g[V0][0] == c.g[A0][0] {
                c.g[S1][0] = 1;
            } else {
                c.cycles += 2;
                c.g[S2][0] = Addr(r32(c, S2) + WLN_NEXT).lw(c);
                // 0x002f5470
                loop {
                    c.cycles += 2;
                    if c.g[S2][0] == 0 {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = Addr(r32(c, S2) + WLN_ID).lw(c);
                    c.g[V0][0] = Addr(obj + INST_KEY).lw(c);
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[S2][0] = Addr(r32(c, S2) + WLN_NEXT).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x002f_5470);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[S1][0] = 1;
                    break;
                }
            }
        }

        // 0x002f548c: a new node for a new instance.
        c.cycles += 2;
        c.g[S4][0] = Addr(r32(c, A1) + 0x18).lw(c);
        if c.g[S1][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = Addr(this + WLM_COUNT).lw(c);
            let fits = c.g[A0][0] < WLM_MAX_NODES;
            c.g[V0][0] = WLM_NODE_SIZE as u64;
            if fits {
                c.cycles += 6;
                let count = r32(c, A0);
                c.g[V1][0] = sx(count.wrapping_add(1));
                let off = abi::mult(c, count, WLM_NODE_SIZE);
                c.g[V0][0] = sx(off);
                Addr(this + WLM_COUNT).sw(c, c.g[V1][0]);
                let off = off.wrapping_add(WLM_NODES);
                c.g[V0][0] = sx(off);
                c.g[S2][0] = sx(this.wrapping_add(off));
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S4][0];
                WLM_EVICT_OLDEST.call(c, 0x002f_54c8)?;
                c.g[S2][0] = c.g[V0][0];
            }
            // 0x002f54cc: link it at the head of its chain.
            c.cycles += 9;
            let node = r32(c, S2);
            c.g[A0][0] = Addr(obj + INST_KEY).lw(c);
            c.g[V1][0] = sx(slot << 2);
            Addr(node + WLN_ID).sw(c, c.g[A0][0]);
            let head = this.wrapping_add(WLM_HEADS).wrapping_add(slot << 2);
            c.g[V0][0] = sx(head);
            c.g[A0][0] = Addr(head).lw(c);
            Addr(node + WLN_PREV).sw(c, 0);
            Addr(node + WLN_NEXT).sw(c, c.g[A0][0]);
            if c.g[A0][0] != 0 {
                c.cycles += 1;
                Addr(r32(c, A0) + WLN_PREV).sw(c, c.g[S2][0]);
            }
            c.cycles += 1;
            Addr(head).sw(c, c.g[S2][0]);
        }

        // 0x002f54f8: stamp it; moved (s4) / reload (s5).
        c.cycles += 4;
        let node = r32(c, S2);
        Addr(node + WLN_PRIORITY).sw(c, c.g[S4][0]);
        c.g[S5][0] = 0;
        c.g[S4][0] = 0;
        if c.g[S1][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(obj + INST_MODEL).lw(c);
            c.g[S5][0] = 1;
            c.g[S4][0] = 1;
            if c.g[V0][0] != 0 {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
            }
            instance_position(c, s, 0, 4, 0x002f_5528)?;
        } else {
            c.cycles += 2;
            if c.g[S6][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(obj + INST_MODEL).lw(c);
                c.g[A0][0] = c.g[S0][0];
                instance_position(c, s, 7, 3, 0x002f_5568)?;
                // 0x002f5584: how far it moved since it was lit.
                c.cycles += 22;
                c.lqc2(3, s);
                c.lqc2(4, node + WLN_POS);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s + 0x20);
                c.g[AT][0] = 0x4120_0000;
                set_fr(c, 0, FpuF32(0x4120_0000));
                c.lq(V0, s + 0x20);
                c.sq(V0, s + 0x10);
                c.lqc2(3, s + 0x10);
                {
                    let v = c.vu0();
                    v.add(0b1000, 6, 0, Src::Bc(0, W));
                    v.mul(0b1111, 4, 3, Src::Vec(3));
                    v.adda(0b1000, 4, Src::Bc(4, Y));
                    v.madda(0b1000, 6, Src::Bc(4, Z));
                    v.madd(0b1000, 4, 6, Src::Bc(4, W));
                    v.sqrt(4, 0);
                    v.waitq();
                }
                c.cfc2(V1, 22);
                c.mtc1(V1, 1);
                let dist = fr_2(c, 1);
                if fcc_lt(c, FpuF32(0x4120_0000), dist) {
                    c.g[S4][0] = 1;
                }
                c.cycles += 6;
                c.g[AT][0] = 0x43fa_0000;
                set_fr(c, 0, FpuF32(0x43fa_0000));
                if fcc_lt(c, FpuF32(0x43fa_0000), dist) {
                    c.g[S5][0] = 1;
                }
            }
        }

        // 0x002f55f4
        c.cycles += 2;
        c.g[S6][0] = sx(node + WLN_CUR);
        if c.g[S4][0] != 0 {
            // The node's overlap region: its own, or a fresh one.
            c.cycles += 5;
            c.lq(V0, s);
            c.g[A0][0] = 0;
            c.g[V1][0] = Addr(node + WLN_BOX).lw(c);
            c.sq(V0, node + WLN_POS);
            let mut own = false;
            if c.g[V1][0] != 0 {
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, V1) + WLB_ID).lw(c);
                c.g[V0][0] = Addr(node + WLN_ID).lw(c);
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V1][0] = Addr(node + WLN_BOX).lw(c);
                    own = true;
                }
            }
            if !own {
                c.cycles += 8;
                c.g[A1][0] = Addr(node + WLN_PRIORITY).lw(c);
                c.g[A0][0] = c.g[S3][0];
                WLM_GET_OVERLAP_REGION.call(c, 0x002f_562c)?;
                c.g[V1][0] = Addr(node + WLN_ID).lw(c);
                c.g[A0][0] = 1;
                Addr(node + WLN_BOX).sw(c, c.g[V0][0]);
                Addr(r32(c, V0) + WLB_ID).sw(c, c.g[V1][0]);
                c.g[V1][0] = Addr(node + WLN_BOX).lw(c);
            }
            // 0x002f5640
            c.cycles += 3;
            c.g[V0][0] = Addr(node + WLN_PRIORITY).lw(c);
            Addr(r32(c, V1) + WLB_PRIORITY).sw(c, c.g[V0][0]);
            let mut rebuild = c.g[A0][0] != 0;
            if !rebuild {
                // 0x002f5658: is the position strictly inside the region's
                // bounds (v1)? Failing x-min or z-max, or passing, goes
                // through 0x002f56d4; v0 ends 1 either way.
                let bx = Addr(node + WLN_BOX).lw(c);
                c.g[V0][0] = bx;
                let bx = bx as u32;
                c.g[V1][0] = 0;
                let via_d4 = 'test: {
                    for i in 0..3u32 {
                        c.cycles += if i == 0 { 7 } else { 6 };
                        let lo = Addr(bx + WLB_MIN + 4 * i).read_fpu(c);
                        let pos = Addr(node + WLN_POS + 4 * i).read_fpu(c);
                        set_fr(c, 0, lo);
                        set_fr(c, 1, pos);
                        if !fcc_lt(c, lo, pos) {
                            if i == 0 {
                                break 'test true;
                            }
                            c.g[V0][0] = 1;
                            break 'test false;
                        }
                        c.cycles += 5;
                        let hi = Addr(bx + WLB_MAX + 4 * i).read_fpu(c);
                        set_fr(c, 0, hi);
                        if !fcc_lt(c, pos, hi) {
                            if i == 2 {
                                break 'test true;
                            }
                            c.g[V0][0] = 1;
                            break 'test false;
                        }
                    }
                    c.g[V1][0] = 1;
                    true
                };
                if via_d4 {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                }
                // 0x002f56d8
                c.cycles += 3;
                if c.g[V1][0] == 0 {
                    c.g[A0][0] = c.g[V0][0];
                }
                c.g[S4][0] = sx(node + WLN_TARGET);
                rebuild = c.g[A0][0] != 0;
            }
            if rebuild {
                // 0x002f56e4: a 1000-unit box around the position, its
                // overlap list gathered again.
                c.cycles += 34;
                c.g[AT][0] = 0x447a_0000;
                let k = FpuF32(0x447a_0000);
                set_fr(c, 0, k);
                c.g[A0][0] = Addr(node + WLN_BOX).lw(c);
                Addr(s + 0x40).write_fpu(c, k);
                Addr(s + 0x44).write_fpu(c, k);
                Addr(s + 0x48).write_fpu(c, k);
                Addr(s + 0x4c).sw(c, 0);
                c.lqc2(3, node + WLN_POS);
                c.lqc2(4, s + 0x40);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s + 0x50);
                c.lq(V0, s + 0x50);
                Addr(s + 0x68).write_fpu(c, k);
                c.sq(V0, s + 0x30);
                Addr(s + 0x60).write_fpu(c, k);
                Addr(s + 0x64).write_fpu(c, k);
                Addr(s + 0x6c).sw(c, 0);
                c.lqc2(3, node + WLN_POS);
                c.lqc2(4, s + 0x60);
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s + 0x70);
                c.lq(V0, s + 0x70);
                c.lq(V1, s + 0x30);
                c.sq(V0, s + 0x20);
                c.sq(V0, s + 0x50);
                c.sq(V1, s + 0x10);
                let bx = r32(c, A0);
                c.sq(V1, bx + WLB_MIN);
                c.lq(V0, s + 0x20);
                c.sq(V0, bx + WLB_MAX);
                let bx = Addr(node + WLN_BOX).lw(c) as u32;
                c.g[A1][0] = sx(bx);
                c.g[A0][0] = Addr(bx + WLB_CELL).lw(c);
                c.g[A1][0] = sx(bx + WLB_MIN);
                call_ptr(c, OVERLAP_LIST_GATHER_TYPE2, 0x002f_5768)?;
                c.g[S4][0] = sx(node + WLN_TARGET);
            }

            // 0x002f576c: reload the target irradiance.
            c.cycles += 4;
            c.g[S6][0] = sx(node + WLN_CUR);
            c.g[S7][0] = sx(node + WLN_NEXT);
            c.g[FP][0] = sx(node + WLN_POS);
            if c.g[S5][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = c.g[S4][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(gp + GP_IRRADIANCE_OVERRIDE).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    IRRADIANCE_GET_OVERRIDE.call(c, 0x002f_5790)?;
                    c.g[V1][0] = c.g[S4][0];
                    c.g[A0][0] = sx(r32(c, V0).wrapping_add(0xa0));
                    copy_lighting(c, A0, A1, 0x002f_5798);
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S4][0];
                } else {
                    c.g[A1][0] = Addr(this + WLM_WORLD).lw(c);
                    // 0x002f57bc: the world samples the ground light under
                    // the position, then its irradiance references.
                    c.cycles += 19;
                    let world = r32(c, A1);
                    c.f[12] = Addr(node + WLN_POS).read_u32(c);
                    c.g[V1][0] = Addr(world + 4).lw(c);
                    let vt = r32(c, V1);
                    c.f[13] = Addr(node + WLN_POS + 4).read_u32(c);
                    c.g[A0][0] = Addr(vt + 0x10).lh(c);
                    c.g[V0][0] = Addr(vt + 0x14).lw(c);
                    c.f[14] = gp_lit(c, GP_NEG_99999).0;
                    c.g[A0][0] = sx(world.wrapping_add(r32(c, A0)));
                    call_ptr(c, r32(c, V0), 0x002f_57dc)?;
                    c.g[V1][0] = Addr(this + WLM_WORLD).lw(c);
                    c.g[A2][0] = Addr(r32(c, V1)).lw(c);
                    c.g[A1][0] = Addr(r32(c, A2) + 4).lw(c);
                    c.g[A0][0] = Addr(r32(c, A1) + 0x140).lh(c);
                    c.g[V0][0] = Addr(r32(c, A1) + 0x144).lw(c);
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    call_ptr(c, r32(c, V0), 0x002f_57f8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                    c.g[S1][0] = 0x12_0000;
                    if c.g[V0][0] != 0 {
                        resolve_irradiance(c, 0x002f_5824, 0x002f_5838, 0x002f_5840)?;
                    } else {
                        // 0x002f5864: the second reference.
                        c.cycles += 11;
                        c.g[V0][0] = Addr(this + WLM_WORLD).lw(c);
                        c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                        c.g[V1][0] = Addr(r32(c, A1) + 4).lw(c);
                        c.g[A0][0] = Addr(r32(c, V1) + 0x128).lh(c);
                        c.g[V0][0] = Addr(r32(c, V1) + 0x12c).lw(c);
                        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                        call_ptr(c, r32(c, V0), 0x002f_5880)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                        c.g[S1][0] = 0x12_0000;
                        if c.g[V0][0] != 0 {
                            resolve_irradiance(c, 0x002f_58ac, 0x002f_58c0, 0x002f_58c8)?;
                        } else {
                            // 0x002f58ec: the default entry.
                            c.cycles += 6;
                            c.g[A0][0] = 0x50_0000;
                            c.g[A1][0] = Addr(gp + GP_DEFAULT_IRRADIANCE).lw(c);
                            c.g[A0][0] = sx(IRRADIANCE_TABLE);
                            call_ptr(c, IRRADIANCE_GET_ENTRY, 0x002f_58fc)?;
                            c.g[V1][0] = c.g[S4][0];
                            c.g[A0][0] = sx(r32(c, V0).wrapping_add(0xa0));
                            copy_lighting(c, A0, A1, 0x002f_5904);
                            c.cycles += 1;
                            c.g[V0][0] = c.g[S4][0];
                        }
                    }
                }
            }

            // 0x002f5924: cur = target, plus the point lights.
            c.cycles += 1;
            c.g[V1][0] = c.g[S6][0];
            copy_lighting(c, S7, A0, 0x002f_5928);
            c.cycles += 7;
            c.g[V0][0] = Addr(node + WLN_BOX).lw(c);
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[FP][0];
            c.g[A3][0] = c.g[S6][0];
            c.g[A2][0] = Addr(r32(c, V0) + WLB_CELL).lw(c);
            c.g[T0][0] = 4;
            WLM_ADD_POINT_LIGHTS.call(c, 0x002f_5960)?;
        }

        // 0x002f5960
        c.cycles += 13;
        c.g[V0][0] = c.g[S6][0];
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
    });
}

/// How [`rider_base_match_cheat_code`] leaves: straight to the epilogue
/// (`lq s0` already done), through 0x2f6a44 (+1, `lq s0`), or through
/// 0x2f6a40 (+1, store the flags in v0, then 0x2f6a44).
pub(crate) enum CheatExit {
    Direct,
    Done,
    StoreFlags,
}

/// `void RiderBase_MatchCheatCode(Matcher* self)` (0x2f6910): consume the
/// pending button events; on a new press (flag 1 clear) step the code
/// state machine: from failed (-1) the 0x40 button restarts; while entering
/// (0) a 0x10 press counts and a 0x20 press checks the count against the
/// code's step (9 steps unlock, a mismatch fails); once unlocked (1) both
/// buttons (0x60) set `gp+0xad0 = 8` and 0x40 toggles flag 2. With no event
/// flag 1 is cleared.
pub fn rider_base_match_cheat_code(c: &mut Cpu) {
    use cheat::*;
    use cheat_callee::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        let at = |off: u32| Addr(this.wrapping_add(off));
        GET_EVENTS.call(c, 0x002f_6928)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        CLEAR_EVENTS.call(c, 0x002f_6934)?;
        let events = c.g[S1][0];
        let exit = if events == 0 {
            c.cycles += 3;
            c.g[V0][0] = at(FLAGS).lbu(c) & 0xfe;
            CheatExit::StoreFlags
        } else {
            c.g[V1][0] = at(FLAGS).lbu(c);
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & 1;
            if c.g[V0][0] != 0 {
                CheatExit::Direct
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V1][0] | 1;
                c.g[V1][0] = at(STATE).lb(c);
                let flags = c.g[A0][0];
                at(FLAGS).sb(c, flags);
                let state = c.g[V1][0] as i64;
                if state == 0 {
                    cheat_entering(c, this, events)
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    if state > 0 {
                        c.cycles += 2;
                        c.g[V0][0] = 0x60;
                        if state == 1 {
                            // Unlocked.
                            c.cycles += 2;
                            if events == 0x60 {
                                c.cycles += 3;
                                c.g[V0][0] = 8;
                                let gp = c.g[GP][0] as u32;
                                Addr(gp.wrapping_add(GP_UNLOCK)).sw(c, 8);
                                CheatExit::Done
                            } else {
                                c.cycles += 2;
                                let toggle = events & 0x40 != 0;
                                c.g[V0][0] = c.g[A0][0] ^ 2;
                                if toggle {
                                    CheatExit::StoreFlags
                                } else {
                                    CheatExit::Done
                                }
                            }
                        } else {
                            c.cycles += 2;
                            CheatExit::Direct
                        }
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = events & 0x40;
                        if state == -1 {
                            // Failed: the 0x40 button restarts.
                            c.cycles += 2;
                            if c.g[V0][0] == 0 {
                                CheatExit::Direct
                            } else {
                                c.cycles += 5;
                                at(FLAGS).sb(c, 0);
                                at(STATE).sb(c, 0);
                                at(COUNT).sb(c, 0);
                                at(STEP).sb(c, 0);
                                CheatExit::Done
                            }
                        } else {
                            c.cycles += 2;
                            CheatExit::Direct
                        }
                    }
                }
            }
        };
        match exit {
            CheatExit::StoreFlags => {
                c.cycles += 2;
                let flags = c.g[V0][0];
                at(FLAGS).sb(c, flags);
                frame.restore(c, S0, 0x20);
            }
            CheatExit::Done => {
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            CheatExit::Direct => frame.restore(c, S0, 0x20),
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// State 0 of [`rider_base_match_cheat_code`]: count 0x10 presses; on 0x20
/// compare the count with the code's step.
pub(crate) fn cheat_entering(c: &mut Cpu, this: u32, events: u64) -> CheatExit {
    use cheat::*;
    let at = |off: u32| Addr(this.wrapping_add(off));
    c.cycles += 3;
    c.g[V0][0] = events & 0x20;
    if events & 0x10 != 0 {
        c.cycles += 4;
        c.g[V0][0] = sx((at(COUNT).lbu(c) as u32).wrapping_add(1));
        let n = c.g[V0][0];
        at(COUNT).sb(c, n);
        return CheatExit::Done;
    }
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = 0x44_0000;
        return CheatExit::Done;
    }
    c.cycles += 7;
    let step = at(STEP).lb(c);
    c.g[A0][0] = at(COUNT).lb(c);
    c.g[V1][0] = sx((step as u32).wrapping_add(CODE));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
    c.g[A1][0] = at(STEP).lbu(c);
    if c.g[A0][0] != c.g[V0][0] {
        // Wrong count: failed.
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        at(STATE).sb(c, u64::MAX);
        return CheatExit::Done;
    }
    c.cycles += 6;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    at(COUNT).sb(c, 0);
    c.g[A0][0] = c.g[V0][0] & 0xff;
    c.g[V1][0] = STEPS;
    let next = c.g[V0][0];
    at(STEP).sb(c, next);
    if c.g[A0][0] == STEPS {
        // The whole code: unlocked.
        c.cycles += 5;
        c.g[V0][0] = 1;
        at(STEP).sb(c, 0);
        at(STATE).sb(c, 1);
        at(COUNT).sb(c, 0);
    }
    CheatExit::Done
}

/// `void RiderBase::SaveCheatCode(cStream* s)` (0x2f6a58): writes the cheat
/// code word (+4).
pub fn rider_base_save_cheat_code(c: &mut Cpu) {
    stream_write_member(c, 0x4, 0x4, 0x002f_6a80);
}

// ---------------------------------------------------------------------------
// WorldLightMan_GetOverlapRegion (0x2f5a70)

/// The overlap region table in the world light manager (`self+0x10`).
pub const WORLD_LIGHT_OVERLAP_REGIONS: u32 = 0x10;

/// Per-region stride of the overlap region table.
pub const WORLD_LIGHT_REGION_STRIDE: u32 = 0x40;

/// `sBox_5400* WorldLightMan_GetOverlapRegion(sMan_5400* self, int priority)`
/// (0x2f5a70): the 8-entry table at `self+0x10`; each entry has a state word
/// (`-1` = free) and a priority. Returns the entry with the least priority
/// strictly above `priority`, or the first free entry (state `-1`), or null
/// when all 8 are used.
pub fn world_light_man_get_overlap_region(c: &mut Cpu) {
    c.cycles += 11;
    let selfp = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(selfp.wrapping_add(WORLD_LIGHT_OVERLAP_REGIONS)).lw(c);
    c.g[A3][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    c.g[T3][0] = 0;
    c.g[T2][0] = 0;
    c.g[V0][0] = !(0u64 | c.g[V0][0]);
    c.g[A3][0] |= 0xffff;
    c.g[T0][0] = WORLD_LIGHT_OVERLAP_REGIONS as u64;
    c.g[T1][0] = WORLD_LIGHT_OVERLAP_REGIONS as u64;
    let t = c.g[V0][0] != 0;
    c.g[A2][0] = sx(selfp.wrapping_add(WORLD_LIGHT_OVERLAP_REGIONS));
    if !t {
        // A free first entry.
        c.cycles += 2;
        c.g[T3][0] = c.g[A2][0];
    } else {
        c.cycles += 1;
        c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
        'scan: loop {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let skip = c.g[V0][0] == 0;
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
            if !skip {
                c.cycles += 2;
                c.g[A1][0] = c.g[V1][0];
                c.g[T3][0] = sx(selfp.wrapping_add(c.g[T0][0] as u32));
            }
            c.cycles += 5;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(WORLD_LIGHT_REGION_STRIDE));
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(WORLD_LIGHT_REGION_STRIDE));
            c.g[V0][0] = (c.g[T2][0] < 8) as u64;
            let done = c.g[V0][0] == 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(WORLD_LIGHT_REGION_STRIDE));
            if done {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002f_5aac);
                }
                continue 'scan;
            }
            c.cycles += 1;
            c.g[T3][0] = sx(selfp.wrapping_add(c.g[T1][0] as u32));
            break;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T3][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// WorldLightMan_AddPointLights (0x2f5af0)

/// `WorldLightMan_FindStrongestLights` (0x2f5b68), same shard.
pub(crate) const FIND_STRONGEST_LIGHTS: Callee = Callee {
    addr: 0x002f_5b68,
    f: ee::s09::WorldLightMan_FindStrongestLights,
};

/// The per-light apply step, cross-shard (`call_indirect` in the generated
/// body).
pub(crate) const LIGHT_APPLY: u32 = 0x0038_a6a8;

/// `void WorldLightMan_AddPointLights(WorldLightMan* self, void* a1, void* a2,
/// void* a3, int count)` (0x2f5af0): collect the `count` strongest point
/// lights into a stack buffer, then add each non-null one through
/// `func_0038A6A8(a3, a1, light)`.
pub fn world_light_man_add_point_lights(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S2, 0xa0);
        c.g[S0][0] = c.g[T0][0];
        frame.save(c, S3, 0x90);
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S1, 0xb0);
        frame.save_ra(c, 0x80);
        // The light buffer is the 32-word scratch area at the frame base.
        c.g[A3][0] = c.g[SP][0];
        FIND_STRONGEST_LIGHTS.call(c, 0x002f_5b1c)?;
        let mut remaining = c.g[S0][0] as i64;
        c.g[S1][0] = c.g[S0][0];
        if remaining > 0 {
            c.cycles += 1;
            c.g[S0][0] = c.g[SP][0];
            loop {
                c.cycles += 3;
                c.g[A2][0] = Addr(r32(c, S0)).lw(c);
                let empty = c.g[A2][0] == 0;
                c.g[A0][0] = c.g[S3][0];
                if !empty {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, LIGHT_APPLY, 0x002f_5b3c)?;
                }
                c.cycles += 3;
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                remaining = c.g[S1][0] as i64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                if remaining == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_5b28);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cPS2FogParticleMan_v02 (0x2dc190)
// ---------------------------------------------------------------------------

/// `gp+0x2a90`: the graphics manager.
pub(crate) const FOG_V02_GP_GFX: u32 = 0x2a90;

/// Graphics manager fields: sprite scale x/y (`+0x6af0`, `+0x6b04`, the
/// projection's diagonal), the view-projection matrix, the clip box
/// (min x/y/z, max x/y/z) and the "matrices current" flag.
pub(crate) const FOG_V02_GFX_SCALE_X: u32 = 0x6af0;

pub(crate) const FOG_V02_GFX_SCALE_Y: u32 = 0x6b04;

pub(crate) const FOG_V02_GFX_VIEW: u32 = 0x6b30;

pub(crate) const FOG_V02_GFX_MIN_X: u32 = 0x6b70;

pub(crate) const FOG_V02_GFX_MIN_Y: u32 = 0x6b74;

pub(crate) const FOG_V02_GFX_MAX_Z: u32 = 0x6b78;

pub(crate) const FOG_V02_GFX_MAX_X: u32 = 0x6b80;

pub(crate) const FOG_V02_GFX_MAX_Y: u32 = 0x6b84;

pub(crate) const FOG_V02_GFX_MIN_Z: u32 = 0x6b88;

pub(crate) const FOG_V02_GFX_VIEW_VALID: u32 = 0x6b90;

/// `.lit4` constants (`gp-`): far cut-off 25000, fade start 18000, fade
/// scale 1/7000, near ramp end 2499.67 and near ramp scale 1/1999.67.
pub(crate) const FOG_V02_LIT_FAR: u32 = 0x3c58;

pub(crate) const FOG_V02_LIT_FADE_START: u32 = 0x3c54;

pub(crate) const FOG_V02_LIT_FADE_SCALE: u32 = 0x3c50;

pub(crate) const FOG_V02_LIT_NEAR_END: u32 = 0x3c4c;

pub(crate) const FOG_V02_LIT_NEAR_SCALE: u32 = 0x3c48;

/// Fog object: world matrix (`+0x10`), mesh (`+0x64`), bounding box min
/// (`+0x68`) and max (`+0x74`).
pub(crate) const FOG_V02_OBJ_MAT: u32 = 0x10;

pub(crate) const FOG_V02_OBJ_MESH: u32 = 0x64;

pub(crate) const FOG_V02_OBJ_MIN: u32 = 0x68;

pub(crate) const FOG_V02_OBJ_MAX: u32 = 0x74;

/// Mesh: part-list count (`+4`), list (`+8`; the list's part at `+4`).
pub(crate) const FOG_V02_MESH_COUNT: u32 = 0x4;

pub(crate) const FOG_V02_MESH_LIST: u32 = 0x8;

pub(crate) const FOG_V02_LIST_PART: u32 = 0x4;

/// Part: vertex count (`+0x1c`) and vertices (`+0x20`, 0x1c bytes each:
/// position, colour rgb at `+0xc`, size at `+0x18`).
pub(crate) const FOG_V02_PART_COUNT: u32 = 0x1c;

pub(crate) const FOG_V02_PART_VERTS: u32 = 0x20;

pub(crate) const FOG_V02_VERT_SIZE: u32 = 0x1c;

/// The sort buffer (`{index, depth}` pairs) the render pass qsorts.
pub(crate) const FOG_V02_SORT_BUF: u32 = 0x0053_7ac0;

/// Load the 4x4 matrix at `m` into vf4..vf7.
pub(crate) fn fog_v02_load_mat(c: &mut Cpu, m: u32) {
    c.lqc2(4, m);
    c.lqc2(5, m.wrapping_add(0x10));
    c.lqc2(6, m.wrapping_add(0x20));
    c.lqc2(7, m.wrapping_add(0x30));
}

/// `vf[dst] = (vf4 vf5 vf6 vf7) * vf[v]` (the VU0 matrix-vector product).
pub(crate) fn fog_v02_xform(c: &mut Cpu, v: u8, dst: u8) {
    c.vu0().mula(0b1111, 4, Src::Bc(v, X));
    c.vu0().madda(0b1111, 5, Src::Bc(v, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(v, Z));
    c.vu0().madd(0b1111, dst, 7, Src::Bc(v, W));
}

/// `void cPS2FogParticleMan_v02(FogList* out, FogObj** objs, int n, int clip)`
/// (0x2dc190): queue the fog particles of `n` objects. Each object whose
/// bounding-box centre is nearer than 25000 gets a distance fade (1 below
/// 18000, then linear to 0); each vertex of its mesh parts is transformed by
/// view * object matrix, faded in between depth 500 and 2500, projected, and
/// (unless both corners are outside the same clip plane when `clip` is set)
/// appended to `out` as a screen quad (corners `p -/+ size * scale`) with
/// colour `(alpha, r, g, b) * 128`, its index and depth going to the sort
/// buffer.
pub fn ps2_fog_particle_man_v02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x170);
        frame.save(c, S1, 0x150);
        frame.save(c, S0, 0x160);
        frame.save(c, S5, 0x110);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S6, 0x100);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, FP, 0xe0);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S2, 0x140);
        frame.save(c, S3, 0x130);
        frame.save(c, S4, 0x120);
        frame.save(c, S7, 0xf0);
        frame.save_ra(c, 0xd0);
        let gp = r32(c, GP);
        c.g[S1][0] = Addr(gp.wrapping_add(FOG_V02_GP_GFX)).lw(c);
        let gfx = r32(c, S1);
        c.g[V0][0] = Addr(gfx).off(FOG_V02_GFX_VIEW_VALID).lw(c);
        c.g[FP][0] = c.g[A1][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            crate::game::render::particle::UPDATE_VIEW_MATRICES.call(c, 0x002d_c1e0)?;
        }

        c.cycles += 2;
        c.g[A0][0] = 0;
        if (c.g[S6][0] as i64) > 0 {
            c.cycles += 10;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 11); // 0.5
            c.g[S4][0] = sx(r32(c, S1).wrapping_add(FOG_V02_GFX_VIEW));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 13); // 1.0
            c.g[S2][0] = sx(sp(c).wrapping_add(0x90));
            let gp = r32(c, GP);
            c.lwc1(12, gp.wrapping_sub(FOG_V02_LIT_FAR));
            c.g[S7][0] = sx(sp(c).wrapping_add(0x50));
            c.g[V0][0] = sx(r32(c, A0) << 2);
            loop {
                fog_v02_object(c);
                c.cycles += 3;
                let more = (c.g[A0][0] as i64) < (c.g[S6][0] as i64);
                c.g[V0][0] = sx(r32(c, A0) << 2);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_c210);
                }
            }
        }

        c.cycles += 12;
        frame.restore(c, S0, 0x160);
        frame.restore(c, S1, 0x150);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore(c, S5, 0x110);
        frame.restore(c, S6, 0x100);
        frame.restore(c, S7, 0xf0);
        frame.restore(c, FP, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// One object (`objs[a0]`, `v0 = a0 * 4`): distance fade from the box
/// centre, then every part of its mesh. Leaves `a0` = the next index.
pub(crate) fn fog_v02_object(c: &mut Cpu) {
    c.cycles += 46;
    c.g[T7][0] = sx(r32(c, A0).wrapping_add(1));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, FP)));
    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
    let obj = r32(c, V1);
    let s = sp(c);
    // Centre = (min + max) * 0.5, w = 1, through the view matrix.
    c.lwc1(3, obj.wrapping_add(FOG_V02_OBJ_MAX));
    c.lwc1(0, obj.wrapping_add(FOG_V02_OBJ_MIN));
    c.lwc1(1, obj.wrapping_add(FOG_V02_OBJ_MIN + 4));
    c.add_s(0, 0, 3);
    c.lwc1(4, obj.wrapping_add(FOG_V02_OBJ_MAX + 4));
    c.lwc1(3, obj.wrapping_add(FOG_V02_OBJ_MAX + 8));
    c.lwc1(2, obj.wrapping_add(FOG_V02_OBJ_MIN + 8));
    c.add_s(1, 1, 4);
    c.mul_s(5, 0, 11);
    c.swc1(0, s.wrapping_add(0x30));
    c.add_s(2, 2, 3);
    c.swc1(13, s.wrapping_add(0x1c));
    c.mul_s(3, 1, 11);
    c.swc1(1, s.wrapping_add(0x34));
    c.swc1(5, s.wrapping_add(0x10));
    c.mul_s(0, 2, 11);
    c.swc1(2, s.wrapping_add(0x38));
    c.swc1(3, s.wrapping_add(0x14));
    c.swc1(5, s.wrapping_add(0x20));
    c.swc1(0, s.wrapping_add(0x18));
    c.swc1(0, s.wrapping_add(0x28));
    c.swc1(3, s.wrapping_add(0x24));
    c.lqc2(8, s.wrapping_add(0x10));
    let view = r32(c, S4);
    fog_v02_load_mat(c, view);
    fog_v02_xform(c, 8, 12);
    c.sqc2(12, s.wrapping_add(0x40));
    c.lq(V0, s.wrapping_add(0x40));
    c.sq(V0, s);
    // Depth = (float)(int)w.
    c.lwc1(0, s.wrapping_add(0xc));
    c.cvt_w_s(1, 0);
    c.mfc1(V0, 1);
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    c.c_le_s(12, 1);
    if c.fcc() {
        // Beyond the far cut-off.
        c.g[A0][0] = c.g[T7][0];
        return;
    }

    // Fade (f10): 1 below the fade start, else (far - d) / 7000.
    c.cycles += 5;
    let gp = r32(c, GP);
    c.lwc1(0, gp.wrapping_sub(FOG_V02_LIT_FADE_START));
    c.c_lt_s(1, 0);
    let near = c.fcc();
    c.g[V0][0] = sx(obj.wrapping_add(FOG_V02_OBJ_MAT));
    if near {
        c.cycles += 4;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 10);
    } else {
        c.cycles += 3;
        c.sub_s(1, 12, 1);
        c.lwc1(0, gp.wrapping_sub(FOG_V02_LIT_FADE_SCALE));
        c.mul_s(10, 1, 0);
    }

    // view * object matrix -> sp+0x90 (s2), copied to sp+0x50 (s7).
    c.cycles += 43;
    let view = r32(c, S4);
    fog_v02_load_mat(c, view);
    let m = r32(c, V0);
    c.lqc2(8, m);
    c.lqc2(9, m.wrapping_add(0x10));
    c.lqc2(10, m.wrapping_add(0x20));
    c.lqc2(11, m.wrapping_add(0x30));
    for k in 0..4u8 {
        fog_v02_xform(c, 8 + k, 12 + k);
    }
    let s2 = r32(c, S2);
    for k in 0..4u32 {
        c.sqc2(12 + k as usize, s2.wrapping_add(0x10 * k));
    }
    for k in 0..4u32 {
        c.lqc2(1 + k as usize, s2.wrapping_add(0x10 * k));
    }
    let s7 = r32(c, S7);
    for k in 0..4u32 {
        c.sqc2(1 + k as usize, s7.wrapping_add(0x10 * k));
    }
    let obj = r32(c, V1);
    c.g[T3][0] = Addr(obj).off(FOG_V02_OBJ_MESH).lw(c);
    c.g[T9][0] = sx(sp(c).wrapping_add(0x50));
    c.g[T7][0] = sx(r32(c, A0).wrapping_add(1));
    c.g[V1][0] = 0;
    let mesh = r32(c, T3);
    c.g[V0][0] = Addr(mesh).off(FOG_V02_MESH_COUNT).lw(c);
    let any = (c.g[V0][0] as i64) > 0;
    c.g[T8][0] = Addr(mesh).off(FOG_V02_MESH_LIST).lw(c);
    if any {
        c.cycles += 1;
        let list = r32(c, T8);
        c.g[T0][0] = Addr(list).off(FOG_V02_LIST_PART).lw(c);
        loop {
            fog_v02_part(c);
            c.cycles += 4;
            c.g[V1][0] = c.g[T6][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            let list = r32(c, T8);
            c.g[T0][0] = Addr(list).off(FOG_V02_LIST_PART).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x002d_c3a8);
            }
        }
    }
    c.cycles += 1;
    c.g[A0][0] = c.g[T7][0];
}

/// One pass over the part `t0` (mesh index `v1`); leaves `t6 = v1 + 1` and
/// `v0` = the mesh's part count.
pub(crate) fn fog_v02_part(c: &mut Cpu) {
    c.cycles += 2;
    c.g[T6][0] = sx(r32(c, V1).wrapping_add(1));
    if c.g[T0][0] != 0 {
        c.cycles += 3;
        let part = r32(c, T0);
        c.g[V0][0] = Addr(part).off(FOG_V02_PART_COUNT).lw(c);
        if c.g[V0][0] == 0 {
            let mesh = r32(c, T3);
            c.g[V0][0] = Addr(mesh).off(FOG_V02_MESH_COUNT).lw(c);
            return;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(part).off(FOG_V02_PART_VERTS).lw(c);
        let any = (c.g[V0][0] as i64) > 0;
        c.g[T2][0] = 0;
        if any {
            c.cycles += 10;
            c.g[V0][0] = 0x53_0000;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 9); // 1.0
            c.g[T4][0] = sx(FOG_V02_SORT_BUF);
            c.g[AT][0] = 0x4300_0000;
            c.mtc1(AT, 8); // 128.0
            c.g[S3][0] = sx(FOG_V02_SORT_BUF + 4);
            c.g[T5][0] = c.g[T9][0];
            c.g[T1][0] = 0x30;
            loop {
                fog_v02_vertex(c);
                c.cycles += 4;
                c.g[T2][0] = sx(r32(c, T2).wrapping_add(1));
                c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(FOG_V02_VERT_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_c3f0);
                }
            }
        }
    }
    c.cycles += 1;
    let mesh = r32(c, T3);
    c.g[V0][0] = Addr(mesh).off(FOG_V02_MESH_COUNT).lw(c);
}

/// One vertex (`a2`): transform by view * object matrix (`t5`), and when
/// in front of the camera emit it. Leaves `v0` = the part's vertex count.
pub(crate) fn fog_v02_vertex(c: &mut Cpu) {
    c.cycles += 25;
    let v = r32(c, A2);
    let s = sp(c);
    c.lwc1(0, v);
    c.lwc1(1, v.wrapping_add(4));
    c.lwc1(2, v.wrapping_add(8));
    c.swc1(0, s.wrapping_add(0x20));
    c.swc1(1, s.wrapping_add(0x24));
    c.swc1(2, s.wrapping_add(0x28));
    c.swc1(9, s.wrapping_add(0x2c));
    c.lqc2(8, s.wrapping_add(0x20));
    let m = r32(c, T5);
    fog_v02_load_mat(c, m);
    fog_v02_xform(c, 8, 12);
    c.sqc2(12, s.wrapping_add(0x30));
    c.lq(V0, s.wrapping_add(0x30));
    c.sq(V0, s.wrapping_add(0x10));
    c.lwc1(0, s.wrapping_add(0x1c));
    c.cvt_w_s(1, 0);
    c.mfc1(A1, 1); // depth = (int)w
    if (c.g[A1][0] as i64) > 0 {
        fog_v02_emit(c);
    }
    let part = r32(c, T0);
    c.g[V0][0] = Addr(part).off(FOG_V02_PART_COUNT).lw(c);
}

/// Alpha from the near ramp; when non-zero, project and append the quad.
pub(crate) fn fog_v02_emit(c: &mut Cpu) {
    c.cycles += 8;
    c.mtc1(A1, 1);
    c.cvt_s_w(1, 1);
    c.g[AT][0] = 0x43fa_0000;
    c.mtc1(AT, 2); // 500.0
    c.c_le_s(1, 2);
    c.mtc1(ZERO, 0);
    if c.fcc() {
        c.mul_s(7, 10, 0);
    } else {
        c.cycles += 4;
        let gp = r32(c, GP);
        c.lwc1(0, gp.wrapping_sub(FOG_V02_LIT_NEAR_END));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.cycles += 4;
            c.sub_s(1, 1, 2);
            c.lwc1(0, gp.wrapping_sub(FOG_V02_LIT_NEAR_SCALE));
            c.mul_s(0, 1, 0);
        } else {
            c.cycles += 2;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
        }
        c.cycles += 1;
        c.mul_s(7, 10, 0);
    }
    c.cycles += 5;
    c.mtc1(ZERO, 6);
    c.c_eq_s(7, 6);
    if c.fcc() {
        return;
    }

    // Sort entry, perspective divide, and the two corners.
    c.cycles += 47;
    let s = sp(c);
    let out = r32(c, S0);
    let gfx = r32(c, S1);
    c.lwc1(3, s.wrapping_add(0x1c));
    c.g[A3][0] = 0x30;
    c.g[A0][0] = Addr(out).lhu(c);
    c.div_s(3, 9, 3);
    c.lwc1(0, s.wrapping_add(0x10));
    c.g[V0][0] = sx(r32(c, A0) << 3);
    c.lwc1(2, s.wrapping_add(0x14));
    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, T4)));
    let (at, val) = (r32(c, V1), c.g[A0][0]);
    c.sw(at, val);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
    let (at, val) = (r32(c, V0), c.g[A1][0]);
    c.sw(at, val);
    c.swc1(3, s.wrapping_add(0x1c));
    c.mul_s(0, 0, 3);
    c.mul_s(2, 2, 3);
    c.g[A0][0] = Addr(out).lw(c);
    c.swc1(0, s.wrapping_add(0x10));
    c.swc1(2, s.wrapping_add(0x14));
    c.mult(0, V0, A0, A3);
    let v = r32(c, A2);
    c.lwc1(1, v.wrapping_add(0x18));
    c.lwc1(2, gfx.wrapping_add(FOG_V02_GFX_SCALE_X));
    c.mul_s(1, 1, 3);
    c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_SCALE_Y));
    c.swc1(6, s.wrapping_add(0x28));
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(out));
    c.swc1(6, s.wrapping_add(0x2c));
    c.mul_s(0, 0, 1);
    c.mul_s(2, 2, 1);
    c.swc1(0, s.wrapping_add(0x24));
    c.swc1(2, s.wrapping_add(0x20));
    // a = p - offset
    c.lqc2(3, s.wrapping_add(0x10));
    c.lqc2(4, s.wrapping_add(0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x90));
    c.lq(V0, s.wrapping_add(0x90));
    c.sq(V0, s.wrapping_add(0x30));
    let e = r32(c, A0);
    c.sq(V0, e.wrapping_add(0x10));
    // b = p + offset
    c.g[V1][0] = Addr(out).lw(c);
    c.lqc2(3, s.wrapping_add(0x10));
    c.lqc2(4, s.wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x90));
    c.mult(0, V0, V1, A3);
    c.g[V1][0] = sx(r32(c, V0).wrapping_add(out));
    c.lq(V0, s.wrapping_add(0x90));
    c.sq(V0, s.wrapping_add(0x30));
    let clip = c.g[S5][0] != 0;
    let e = r32(c, V1);
    c.sq(V0, e.wrapping_add(0x20));

    if clip && fog_v02_clipped(c, out, gfx) {
        return;
    }

    // Colour: alpha, r, g, b (each * 128); then count++.
    c.cycles += 27;
    for k in 0..4u32 {
        let (cnt, idx) = if k % 2 == 0 { (V0, V1) } else { (V1, V0) };
        c.g[cnt][0] = Addr(out).lw(c);
        if k == 0 {
            c.mul_s(0, 7, 8);
            c.mult(0, idx, cnt, T1);
        } else {
            let v = r32(c, A2);
            c.lwc1(0, v.wrapping_add(0x8 + 4 * k));
            c.mult(0, idx, cnt, T1);
            c.mul_s(0, 0, 8);
        }
        c.g[cnt][0] = sx(r32(c, idx).wrapping_add(out));
        let e = r32(c, cnt);
        c.swc1(0, e.wrapping_add(0x30 + 4 * k));
    }
    c.g[V0][0] = Addr(out).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    let n = c.g[V0][0];
    Addr(out).sw(c, n);
}

/// Outcodes of the two corners (bit 2 x < min, 1 x > max, 8 y < min, 4 y >
/// max, 0x20 z > max, 0x10 z < min; all but 0x10 flipped for w < 0): true
/// when both are outside one plane.
pub(crate) fn fog_v02_clipped(c: &mut Cpu, out: u32, gfx: u32) -> bool {
    // Corner a (entry + 0x10); the clip planes stay in f5/f4/f3/f2.
    c.cycles += 12;
    c.g[V0][0] = Addr(out).lw(c);
    c.g[A0][0] = 0;
    c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MIN_X));
    c.mult(0, A1, V0, A3);
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = sx(r32(c, A1).wrapping_add(out));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x10));
    let p = r32(c, V0);
    c.lwc1(1, p);
    c.c_lt_s(1, 0);
    let t = c.fcc();
    c.mov_s(5, 0);
    if t {
        c.cycles += 1;
        c.g[A0][0] = 2;
    }
    c.cycles += 5;
    c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MAX_X));
    c.c_lt_s(0, 1);
    let t = c.fcc();
    c.mov_s(4, 0);
    if t {
        c.cycles += 1;
        c.g[A0][0] |= 0x1;
    }
    c.cycles += 6;
    c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MIN_Y));
    c.lwc1(1, p.wrapping_add(4));
    c.c_lt_s(1, 0);
    let t = c.fcc();
    c.mov_s(3, 0);
    if t {
        c.cycles += 1;
        c.g[A0][0] |= 0x8;
    }
    c.cycles += 5;
    c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MAX_Y));
    c.c_lt_s(0, 1);
    let t = c.fcc();
    c.mov_s(2, 0);
    if t {
        c.cycles += 1;
        c.g[A0][0] |= 0x4;
    }
    c.cycles += 5;
    c.lwc1(0, p.wrapping_add(0xc));
    c.c_lt_s(0, 6);
    if !c.fcc() {
        c.lwc1(1, p.wrapping_add(8));
        c.cycles += 5;
        c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MAX_Z));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.g[A0][0] |= 0x20;
        }
        c.cycles += 5;
        c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MIN_Z));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.g[A0][0] |= 0x10;
        }
    } else {
        c.cycles += 2;
        c.g[A0][0] ^= 0x2f;
    }

    // Corner b (entry + 0x20).
    c.cycles += 8;
    c.mult(0, A1, V1, T1);
    c.g[V0][0] = sx(r32(c, A1).wrapping_add(out));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x20));
    let q = r32(c, V0);
    c.lwc1(0, q);
    c.c_lt_s(0, 5);
    let t = c.fcc();
    c.g[V1][0] = 0;
    if t {
        c.cycles += 1;
        c.g[V1][0] = 2;
    }
    c.cycles += 4;
    c.c_lt_s(4, 0);
    if c.fcc() {
        c.g[V1][0] |= 0x1;
    }
    c.cycles += 5;
    c.lwc1(0, q.wrapping_add(4));
    c.c_lt_s(0, 3);
    if c.fcc() {
        c.g[V1][0] |= 0x8;
    }
    c.cycles += 4;
    c.c_lt_s(2, 0);
    if c.fcc() {
        c.g[V1][0] |= 0x4;
    }
    c.cycles += 6;
    c.lwc1(1, q.wrapping_add(0xc));
    c.mtc1(ZERO, 0);
    c.c_lt_s(1, 0);
    if !c.fcc() {
        c.lwc1(1, q.wrapping_add(8));
        c.cycles += 5;
        c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MAX_Z));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.g[V1][0] |= 0x20;
        }
        c.cycles += 5;
        c.lwc1(0, gfx.wrapping_add(FOG_V02_GFX_MIN_Z));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.g[V1][0] |= 0x10;
        }
    } else {
        c.cycles += 2;
        c.g[V1][0] ^= 0x2f;
    }

    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0] & c.g[V1][0];
    c.g[V0][0] != 0
}

/// `void RiderBase_ResetAttackFXThunk(void)` (0x2f7ba8): forwards to
/// `RiderBase_ResetAttackFX`.
pub fn rider_base_reset_attack_fx_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002f_1148, 0x002f_7bb8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void RiderBase_ResetPowerUpTrailThunk(void)` (0x2f7ed0): forwards to
/// `RiderBase_ResetPowerUpTrail`.
pub fn rider_base_reset_power_up_trail_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002e_adc0, 0x002f_7ee0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

pub(crate) const CHEAT_CTOR: RiderWrapperCtor = RiderWrapperCtor {
    rider_off: 0x10,
    vt_a: 0x7920,
    vt_b: 0x78d8,
    ret_rider: 0x002f_6264,
    loop1: 0x002f_62b0,
    loop2: 0x002f_6314,
    tail: 0x002f_64f0,
    ret_tail: 0x002f_6370,
};

/// `RiderBase* RiderBase_CtorCheatCode(RiderBase* self, int inChrg)`
/// (0x2f6238): rider wrapper at `self + 0x10`, then `RiderBase_ClearCheatCode`.
pub fn rider_base_ctor_cheat_code(c: &mut Cpu) {
    rider_wrapper_ctor(c, &CHEAT_CTOR);
}

/// `void RiderBase_ResetCheatCode(RiderBase* self)` (0x2f64e8): an empty
/// function (the cheat-code state is cleared by `ClearCheatCode`).
pub fn rider_base_reset_cheat_code(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void RiderBase_ClearCheatCode(RiderBase* self)` (0x2f64f0): zeroes the
/// 4 bytes at `self + 4` (`memset(self + 4, 0, 4)`).
pub fn rider_base_clear_cheat_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 4;
        call_ptr(c, 0x0041_6210, 0x002f_6508)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AttachedEmitterTypeData_InitPart1` (0x002f83b0): static initialiser of
/// the attached-emitter type table (first half). Loads the table base from
/// `gp+0xac0` and fills the per-type default fields (floats, zeros and small
/// ints) in one straight line; no calls.
pub fn attached_emitter_type_data_init_part1(c: &mut Cpu) {
    c.cycles += 786;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xac0));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 1);
    c.g[V1][0] = 0x1;
    c.g[AT][0] = 0xffffffffbf800000;
    c.mtc1(1, 11);
    c.g[A1][0] = 0x5;
    c.g[AT][0] = 0x42200000;
    c.mtc1(1, 7);
    c.g[A0][0] = 0x18;
    c.g[AT][0] = 0x43160000;
    c.mtc1(1, 0);
    c.g[A2][0] = 0x2;
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.g[A3][0] = 0x19;
    c.swc1(20, c.g[SP][0] as u32);
    c.g[T0][0] = 0x3;
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x74));
    c.g[T1][0] = 0x8;
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x10));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x54));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x64));
    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x20), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x24), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x28), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x30), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x34), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x38), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x40), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x44), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x48), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x50), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x58), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x60), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x68), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x70), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x78), 0u64);
    c.g[AT][0] = 0x41a00000;
    c.mtc1(1, 9);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x80), 0u64);
    c.g[AT][0] = 0x430c0000;
    c.mtc1(1, 13);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x84));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x88));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x90));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x94));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x98));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x9c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xa0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc8), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xcc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd0), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xd4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xd8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfc), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x100));
    c.g[AT][0] = 0x42c80000;
    c.mtc1(1, 3);
    c.g[AT][0] = 0x41200000;
    c.mtc1(1, 6);
    c.g[AT][0] = 0x42480000;
    c.mtc1(1, 5);
    c.g[AT][0] = 0x3fc00000;
    c.mtc1(1, 16);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x104));
    c.g[AT][0] = 0x428c0000;
    c.mtc1(1, 0);
    c.g[AT][0] = 0x3e800000;
    c.mtc1(1, 2);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x108));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x120));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x124));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x11c));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x128));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x140));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x150));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x164));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x174));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x184));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x10c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x110), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x114), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x118));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x130), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x134), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x138), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x144), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x148), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x154), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x158), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x15c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x160), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x168), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x16c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x170), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x178), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x17c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x180), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x188), 0u64);
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3770));
    c.g[AT][0] = 0x40400000;
    c.mtc1(1, 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x18c), 0u64);
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x190));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1fc));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1f4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x194));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x198));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x19c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1a0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1a4), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1a8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1e0), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x1e4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1e8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1f0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x200), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x204), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x208), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x20c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x210));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x214));
    c.g[AT][0] = 0x41f00000;
    c.mtc1(1, 0);
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x376c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x218));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x230));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x234));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x274));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x284));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x294));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x21c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x220), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x224), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x228));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x22c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x238), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x23c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x240), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x244), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x248), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x24c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x250), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x254), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x258), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x25c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x260), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x264), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x268), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x26c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x270), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x278), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x27c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x280), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x288), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x28c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x290), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x298), 0u64);
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3768));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x29c), 0u64);
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x2a0));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x304));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2a4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2a8));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2ac));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2b0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2b4), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2b8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2e4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2f0), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x2f4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2f8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x300), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x308), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x30c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x310), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x314), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x318), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x31c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x320));
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3764));
    c.g[AT][0] = 0x43fa0000;
    c.mtc1(1, 12);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x324));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3760));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x328));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x340));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x384));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x394));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x3a4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x32c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x330), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x334), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x338));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x33c));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x344));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x348), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x34c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x350), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x354), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x358), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x35c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x360), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x364), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x368), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x36c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x370), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x374), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x378), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x37c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x380), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x388), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x38c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x390), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x398), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x39c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3a8), 0u64);
    c.g[AT][0] = 0xffffffffc3480000;
    c.mtc1(1, 2);
    c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x375c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3ac), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x3b0));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x414));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x3b4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x3b8));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x3bc));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x3c0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3c4), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x3c8));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x3cc));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x3d0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3f0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3f4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x400), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x404));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x408));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x40c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x410), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x418), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x41c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x420), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x424), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x428), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x42c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x430));
    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3758));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x434));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x438));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4b4));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x450));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x454));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x494));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4a4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x43c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x440), c.g[A2][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x444), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x448));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x44c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x458), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x45c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x460), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x464), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x468), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x46c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x470), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x474), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x478), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x47c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x480), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x484), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x488), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x48c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x490), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x498), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x49c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4bc), 0u64);
    c.g[AT][0] = 0x3fa00000;
    c.mtc1(1, 0);
    c.g[AT][0] = 0x41000000;
    c.mtc1(1, 2);
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x4c0));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x518));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x528));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x50c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x504), c.g[A0][0]);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4c4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4c8));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4cc));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4d0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4d4), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4d8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4f0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4f4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x500), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x508), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x510), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x514));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x51c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x520), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x524), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x52c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x530), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x534), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x538), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x53c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x540));
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 14);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x544));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x548));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x55c));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x560));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x5c4));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x564));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x5a4));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x5b4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x54c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x550), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x554), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x558));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x568), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x56c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x570), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x574), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x578), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x57c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x580), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x584), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x588), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x58c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x590), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x594), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x598), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x59c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5c8), 0u64);
    c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x3754));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5cc), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3750));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x374c));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3748));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3744));
    c.g[AT][0] = 0x42f00000;
    c.mtc1(1, 5);
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3740));
    c.g[AT][0] = 0x40e00000;
    c.mtc1(1, 7);
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x5d0));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5d4));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x5d8));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x5e0));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x5e8));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x5f0));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x61c));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x628));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x638));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x5dc));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0x5ec));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x614), c.g[A3][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5f4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x600), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x604), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x608), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x60c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x610), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x618), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x620), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x624));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x62c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x630), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x634), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x63c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x640), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x644), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x648), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x64c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x650));
    c.g[AT][0] = 0x40c00000;
    c.mtc1(1, 2);
    c.g[AT][0] = 0x41c00000;
    c.mtc1(1, 5);
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x373c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x654));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x658));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x670));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x680));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x65c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x660), c.g[A2][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x664), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x668));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x66c));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x674));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x678));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x67c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x684), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x688), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x68c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x690), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x694), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x698), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x69c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6a4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d8), 0u64);
    c.g[AT][0] = 0xffffffffc1ac0000;
    c.mtc1(1, 3);
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3738));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3734));
    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3730));
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x372c));
    c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x3728));
    c.g[AT][0] = 0xffffffffc0400000;
    c.mtc1(1, 17);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6dc), 0u64);
    c.g[AT][0] = 0xffffffffc15c0000;
    c.mtc1(1, 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e0), 0u64);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x748));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x6e8));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x6ec));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x6fc));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x704));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x750));
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x758));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x6e4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x6f0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6f4), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x6f8));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x700));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x708), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x70c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x710), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x714), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x718), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x71c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x720), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x728), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x72c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x724), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x730), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x734));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x738));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x73c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x740), 0u64);
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x744));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x74c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x754), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x75c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x760));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x764));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x768));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x780));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x790));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x770), c.g[A2][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x76c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x774), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x778));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x77c));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x784));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x788));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x78c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x794), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x798), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x79c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7a4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7ec), 0u64);
    c.g[AT][0] = 0x41820000;
    c.mtc1(1, 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7f0), 0u64);
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x7f8));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x7fc));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x80c));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x814));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x858));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x860));
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x868));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x7f4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x800));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x804), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x808));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x810));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x818), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x81c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x820), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x824), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x828), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x82c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x830), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x838), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x83c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x834), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x840), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x844));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x848));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x84c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x850), 0u64);
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x854));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x85c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x864), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x86c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x870));
    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3724));
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x3720));
    c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x371c));
    c.g[AT][0] = 0x42af0000;
    c.mtc1(1, 17);
    c.g[AT][0] = 0x43870000;
    c.mtc1(1, 18);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x874));
    c.g[AT][0] = 0x425a0000;
    c.mtc1(1, 0);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x878));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x87c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x880), c.g[T0][0]);
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x89c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x884), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x888));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x88c));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x890));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x894));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x898));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x8a0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8a4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8ac), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x8b0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8bc), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x8c0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8dc), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x8e0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8ec), 0u64);
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8f0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8f4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8f8), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3718));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3714));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3710));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x370c));
    c.g[AT][0] = 0xffffffffc21e0000;
    c.mtc1(1, 16);
    c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x3708));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x900), 0u64);
    c.g[AT][0] = 0x42dc0000;
    c.mtc1(1, 20);
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0x928));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x904));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x908));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x90c));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x910));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x914), 0u64);
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x918));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x91c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x920), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x924), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x92c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x930), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x934), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x938), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x93c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x940), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x948), 0u64);
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x94c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x944), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x950), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x954));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x958));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x95c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x960), 0u64);
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x964));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x968), 0u64);
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x96c));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0x970));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x974), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x978), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x97c), 0u64);
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x980));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x984));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x988));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x990), c.g[T1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x998));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x9a0));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x9a8));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x9ac));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x9b0));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x9d0));
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x9f0));
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0xa00));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x98c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x994), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x99c));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x9a4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9bc), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x9c0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9f4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa04), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa08), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa0c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa10), 0u64);
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0xa38));
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xa8c));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xa14));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xa20));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0xa28));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xa5c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa54), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa60), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xa64));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xa68));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0xa74));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xa7c));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0xa80));
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xa90));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xa18));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xa1c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa24), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa2c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa30), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa34), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa3c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa40), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa44), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa48), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa4c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa50), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa58), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa6c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa70), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa78), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa84), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa88), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xa94));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xa98));
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.lwc1(20, c.g[SP][0] as u32);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xa9c));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t;
}

/// `AttachedEmitterTypeData_InitPart2` (0x002f9838): second half of the
/// attached-emitter type table initialiser (same base, `gp+0xac0`); a straight
/// line of stores, no calls.
pub fn attached_emitter_type_data_init_part2(c: &mut Cpu) {
    c.cycles += 879;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xac0));
    c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x28));
    c.g[A0][0] = 0x5;
    c.g[AT][0] = 0xffffffffbf800000;
    c.mtc1(1, 13);
    c.g[V1][0] = 0x1;
    c.lwc1(25, (c.g[GP][0] as u32).wrapping_sub(0x35f0));
    c.g[A1][0] = 0x29;
    c.g[AT][0] = 0x43c80000;
    c.mtc1(1, 3);
    c.g[T0][0] = 0xa;
    c.g[AT][0] = 0x40a00000;
    c.mtc1(1, 5);
    c.g[T2][0] = 0x8;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x35ec));
    c.g[A2][0] = 0x4;
    c.g[AT][0] = 0x3e800000;
    c.mtc1(1, 1);
    c.g[T1][0] = 0xf;
    c.swc1(31, (c.g[SP][0] as u32).wrapping_add(0x58));
    c.g[A3][0] = 0x3;
    c.swc1(30, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[T3][0] = 0x26;
    c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x18));
    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.swc1(20, c.g[SP][0] as u32);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xaac));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xab4));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xb14));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xab0));
    c.swc1(25, (c.g[V0][0] as u32).wrapping_add(0xabc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xaa0), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xaa4), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xaa8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xab8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xac0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xac4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xac8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xacc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xad0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xad4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xad8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xadc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xae0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xae4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xae8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xaec), 0u64);
    c.g[AT][0] = 0xffffffffc3390000;
    c.mtc1(1, 4);
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xaf4));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xb04));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xaf0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xaf8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xafc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb00), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb08), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb0c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb10), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb18), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb1c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x35e8));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xb20));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xb30));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 2);
    c.g[AT][0] = 0x41a00000;
    c.mtc1(1, 11);
    c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x35e4));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x35e0));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x35dc));
    c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x35d8));
    c.g[AT][0] = 0x430c0000;
    c.mtc1(1, 17);
    c.g[AT][0] = 0x40400000;
    c.mtc1(1, 26);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xb58));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xb2c));
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0xb3c));
    c.g[AT][0] = 0x41820000;
    c.mtc1(1, 20);
    c.g[AT][0] = 0x41900000;
    c.mtc1(1, 21);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xb24));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xb28));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb34), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xb38));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb40), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb44), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb48), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb4c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb50), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb54), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb5c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb60), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb68), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb6c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb64), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb70), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0xb74));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0xb78));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x35d4));
    c.g[AT][0] = 0x418e0000;
    c.mtc1(1, 10);
    c.g[AT][0] = 0x40000000;
    c.mtc1(1, 15);
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xb84));
    c.swc1(26, (c.g[V0][0] as u32).wrapping_add(0xb88));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0xb8c));
    c.g[AT][0] = 0x40800000;
    c.mtc1(1, 31);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb7c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb80), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb90), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb94), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb98), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb9c), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0xba0));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xba4));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x35d0));
    c.g[AT][0] = 0x42f00000;
    c.mtc1(1, 1);
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xba8));
    c.g[AT][0] = 0x42960000;
    c.mtc1(1, 16);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xbac));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xbc8));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc24));
    c.swc1(31, (c.g[V0][0] as u32).wrapping_add(0xbc0));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0xbcc));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0xbe0));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0xbf0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbb0), c.g[T0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbb4), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xbb8));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xbbc));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0xbc4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbd0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbd4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbd8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbdc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbe4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbe8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbf4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbf8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbfc), 0u64);
    c.g[AT][0] = 0x41200000;
    c.mtc1(1, 22);
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x35cc));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0xc04));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0xc14));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc00), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc08), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc0c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc10), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc18), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc1c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc20), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc28), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc2c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x35c8));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x35c4));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x35c0));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x35bc));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x35b8));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x35b4));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x35b0));
    c.g[AT][0] = 0x428c0000;
    c.mtc1(1, 8);
    c.swc1(22, (c.g[V0][0] as u32).wrapping_add(0xc30));
    c.lwc1(30, (c.g[GP][0] as u32).wrapping_sub(0x35ac));
    c.g[AT][0] = 0xffffffffc2700000;
    c.mtc1(1, 19);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc34));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc38));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xc3c));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xc40));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xc68));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0xc6c));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc70));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0xc7c));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0xc58));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0xc5c));
    c.lwc1(27, (c.g[GP][0] as u32).wrapping_sub(0x35a8));
    c.g[AT][0] = 0xffffffffc35f0000;
    c.mtc1(1, 28);
    c.g[AT][0] = 0x425c0000;
    c.mtc1(1, 29);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc44), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc48));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc4c));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc50));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc54), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc60), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc64), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc78), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc74), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc80), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0xc84));
    c.swc1(30, (c.g[V0][0] as u32).wrapping_add(0xc88));
    c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x35a4));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0xc94));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0xc9c));
    c.swc1(27, (c.g[V0][0] as u32).wrapping_add(0xca0));
    c.swc1(28, (c.g[V0][0] as u32).wrapping_add(0xca8));
    c.swc1(29, (c.g[V0][0] as u32).wrapping_add(0xcb0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc8c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc90), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc98), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xca4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcac), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xcb4));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x35a0));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x359c));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3598));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3594));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3590));
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xcb8));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xcd0));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xcd4));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xcd8));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xce0));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xcbc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcc4), c.g[A2][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcc0), c.g[T2][0]);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xcc8));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xccc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcdc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xce4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xce8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcf0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcf4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcf8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcfc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd00), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd04), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd08), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd0c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd10), 0u64);
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x358c));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3588));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd14), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd18), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd1c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd20), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd24), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd28), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd2c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd30), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd34), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd38), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd3c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3584));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3580));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x357c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd40), 0u64);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xd4c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xd50));
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0xd5c));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xd6c));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xd70));
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xda4));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0xdac));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xd68));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xd78));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xd44));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xd48));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd54), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xd58));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd60), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd64), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd74), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd7c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd80), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd88), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd8c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd84), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd90), c.g[V1][0]);
    c.swc1(26, (c.g[V0][0] as u32).wrapping_add(0xda8));
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0xd94));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0xd98));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd9c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xda0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdb0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdb4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdb8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdbc), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0xdc0));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xdc4));
    c.g[AT][0] = 0x40200000;
    c.mtc1(1, 0);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xdc8));
    c.g[AT][0] = 0x42c80000;
    c.mtc1(1, 18);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xde8));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0xdec));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0xde0));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0xe00));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0xe10));
    c.g[AT][0] = 0x42af0000;
    c.mtc1(1, 20);
    c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x3578));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdd0), c.g[T1][0]);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xdcc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdd4), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xdd8));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xddc));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0xde4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdf0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdf4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdf8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdfc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe04), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe08), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe0c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe14), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe18), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe1c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe20), 0u64);
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xe24));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0xe34));
    c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x3574));
    c.g[AT][0] = 0x43480000;
    c.mtc1(1, 23);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe28), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe2c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe30), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe38), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe3c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe40), 0u64);
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0xe44));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe48), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe4c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3570));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x356c));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3568));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3564));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3560));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x355c));
    c.swc1(22, (c.g[V0][0] as u32).wrapping_add(0xe50));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xe54));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xe58));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xe5c));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xe60));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xe6c));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xe70));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0xe7c));
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xe8c));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0xe90));
    c.swc1(27, (c.g[V0][0] as u32).wrapping_add(0xec0));
    c.swc1(28, (c.g[V0][0] as u32).wrapping_add(0xec8));
    c.swc1(29, (c.g[V0][0] as u32).wrapping_add(0xecc));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xe68));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0xe78));
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xe88));
    c.swc1(23, (c.g[V0][0] as u32).wrapping_add(0xe9c));
    c.g[AT][0] = 0x42d20000;
    c.mtc1(1, 24);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe64), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe74), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe80), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe84), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe98), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe94), c.g[A0][0]);
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 8);
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0xeb4));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0xebc));
    c.swc1(24, (c.g[V0][0] as u32).wrapping_add(0xed0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xea0), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0xea4));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0xea8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xeac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xeb0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xeb8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xec4), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xed4));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3558));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3554));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3550));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x354c));
    c.g[AT][0] = 0x43870000;
    c.mtc1(1, 5);
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0xed8));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xedc));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xef0));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xef8));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xefc));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0xf20));
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xf34));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0xf44));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xf54));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0xf10));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xee0), c.g[T0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xee4), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xee8));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xeec));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0xef4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf00), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf04), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf08), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf0c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf14), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf18), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf1c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf24), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf28), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf2c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf30), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf38), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf3c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf40), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf48), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf4c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf50), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf58), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf5c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3548));
    c.swc1(22, (c.g[V0][0] as u32).wrapping_add(0xf60));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xf64));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3544));
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3540));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x353c));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3538));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3534));
    c.g[AT][0] = 0x43960000;
    c.mtc1(1, 5);
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3530));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x352c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xf68));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xf6c));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xf70));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xf8c));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0xf9c));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xfa0));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xfac));
    c.swc1(30, (c.g[V0][0] as u32).wrapping_add(0xfb8));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0xfc4));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0xfcc));
    c.swc1(23, (c.g[V0][0] as u32).wrapping_add(0xfd0));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0xfd8));
    c.swc1(24, (c.g[V0][0] as u32).wrapping_add(0xfdc));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xf88));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf74), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf78), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf7c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf80), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf84), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf90), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf94), 0u64);
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0xf98));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfa8), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfa4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfb0), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0xfb4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfbc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfc0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfc8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfd4), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0xfe0));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xfe4));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3528));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xfe8));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xfec));
    c.swc1(31, (c.g[V0][0] as u32).wrapping_add(0x1000));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xff0), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xff4), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xff8));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xffc));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1004));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1008), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x100c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1010), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1014), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1018), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x101c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1020), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1024), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1028), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x102c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1030), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1034), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1038), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x103c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1040), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1044), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1048), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x104c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1050), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1054), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1058), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x105c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1060), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1064), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1068), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x106c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1070), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1074));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1078));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x107c));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1080));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1084));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1088));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x108c));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1090));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1094), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1098), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x109c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10a4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10b8), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10b4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10c0), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x10c4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10c8), 0u64);
    c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x3524));
    c.lwc1(23, (c.g[GP][0] as u32).wrapping_sub(0x3520));
    c.lwc1(24, (c.g[GP][0] as u32).wrapping_sub(0x351c));
    c.lwc1(27, (c.g[GP][0] as u32).wrapping_sub(0x3518));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10ec), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x10f0));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x10f4));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x10f8));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0x1110));
    c.swc1(23, (c.g[V0][0] as u32).wrapping_add(0x1114));
    c.swc1(24, (c.g[V0][0] as u32).wrapping_add(0x1118));
    c.swc1(27, (c.g[V0][0] as u32).wrapping_add(0x111c));
    c.swc1(25, (c.g[V0][0] as u32).wrapping_add(0x1130));
    c.swc1(25, (c.g[V0][0] as u32).wrapping_add(0x1140));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x10fc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1100), c.g[A3][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1104), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x1108));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x110c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1120), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1124), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1128), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x112c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1134), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1138), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x113c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1144), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1148), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x114c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1150), 0u64);
    c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x3514));
    c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x3510));
    c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x350c));
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x1154));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1158), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x115c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1160), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x1164));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1168), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x116c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1170), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x1174));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1178), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x117c), 0u64);
    c.g[AT][0] = 0x40c80000;
    c.mtc1(1, 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1180), 0u64);
    c.g[AT][0] = 0x41100000;
    c.mtc1(1, 15);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x11e8));
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x11a0));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x1184));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1188), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x118c), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1190));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1194), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1198));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0x119c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11a4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11c4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11d0), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x11d4));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3508));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x11d8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11e0), 0u64);
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x11e4));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x11ec));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11f0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11f4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11fc), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x1200));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1204));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3504));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3500));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x34fc));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x34f8));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x1208));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x120c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1220));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x1228));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x122c));
    c.swc1(26, (c.g[V0][0] as u32).wrapping_add(0x1250));
    c.swc1(25, (c.g[V0][0] as u32).wrapping_add(0x1224));
    c.swc1(26, (c.g[V0][0] as u32).wrapping_add(0x1240));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1210), c.g[T2][0]);
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x1284));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1214), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x1218));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x121c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1230), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1234), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1238), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x123c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1244), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1248), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x124c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1254), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1258), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x125c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1260), 0u64);
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x1264));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1268), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x126c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1270), 0u64);
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x1274));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1278), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x127c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1280), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1288), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x128c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x34f4));
    c.g[AT][0] = 0x3fe00000;
    c.mtc1(1, 1);
    c.g[AT][0] = 0x42100000;
    c.mtc1(1, 3);
    c.g[AT][0] = 0xffffffffc1740000;
    c.mtc1(1, 4);
    c.g[AT][0] = 0xffffffffc1400000;
    c.mtc1(1, 5);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1290), 0u64);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1294));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x12dc));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x12f4));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x12f8));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x12fc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12d4), c.g[T3][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1298), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x129c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12a4), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x12a8));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x12ac));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x12b0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12b4), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x12b8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12c8), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x12cc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12d8), 0u64);
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x34f0));
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x34ec));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12e0), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x12e4));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x12e8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12f0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1300), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1304), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1308), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x130c), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x1310));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1314));
    c.g[AT][0] = 0x40880000;
    c.mtc1(1, 0);
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x34e8));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x34e4));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x34e0));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x34dc));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x34d8));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x1318));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x131c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1330));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x1334));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x1338));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x133c));
    c.swc1(22, (c.g[V0][0] as u32).wrapping_add(0x1360));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x1374));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x1394));
    c.swc1(22, (c.g[V0][0] as u32).wrapping_add(0x1358));
    c.swc1(22, (c.g[V0][0] as u32).wrapping_add(0x135c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1320), c.g[A3][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1324), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x1328));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x132c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1340), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1344), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1348), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x134c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1350), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1354), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1364), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1368), 0u64);
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x1384));
    c.g[AT][0] = 0xffffffffc3700000;
    c.mtc1(1, 12);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x136c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1370), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1378), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x137c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1380), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1388), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x138c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1390), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1398), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x139c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x34d4));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x34d0));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x34cc));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x34c8));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x34c4));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x34c0));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x34bc));
    c.g[AT][0] = 0x3fc00000;
    c.mtc1(1, 8);
    c.g[AT][0] = 0x41740000;
    c.mtc1(1, 9);
    c.g[AT][0] = 0xffffffffc1140000;
    c.mtc1(1, 10);
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x13a0));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x13a8));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x13ac));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x13b8));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x13bc));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x13d0));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x13d8));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x13ec));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13e4), c.g[A1][0]);
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x13f8));
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x1404));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x1408));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x13a4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13f0), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x13f4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1400), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x140c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1410), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1414), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1418), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x141c), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x1420));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1424));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1428));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0x1440));
    c.swc1(23, (c.g[V0][0] as u32).wrapping_add(0x1444));
    c.swc1(24, (c.g[V0][0] as u32).wrapping_add(0x1448));
    c.swc1(27, (c.g[V0][0] as u32).wrapping_add(0x144c));
    c.swc1(25, (c.g[V0][0] as u32).wrapping_add(0x1470));
    c.swc1(25, (c.g[V0][0] as u32).wrapping_add(0x1460));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1430), c.g[A3][0]);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x1438));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x142c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1434), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x143c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1450), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1454), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1458), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x145c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1464), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1468), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x146c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1474), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1478), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x147c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1480), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x1484));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1488), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x148c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1490), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x1494));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1498), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x149c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14a0), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x14a4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14ac), 0u64);
    c.g[AT][0] = 0xffffffffc0c80000;
    c.mtc1(1, 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14b0), 0u64);
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x14d0));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1518));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x14b4));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0x14cc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14f4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1500), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x1504));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x1508));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x151c));
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x1530));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1534));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14bc), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x14c0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14c4), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x14c8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14e8), 0u64);
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x34b8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14f0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x150c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1510), 0u64);
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x1514));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1520), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1524), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1528), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x152c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x34b4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1538));
    c.lwc1(31, (c.g[SP][0] as u32).wrapping_add(0x58));
    c.lwc1(30, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x28));
    c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x18));
    c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.lwc1(20, c.g[SP][0] as u32);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x153c));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t;
}

/// `AttachedEmitterTypeData_FreePart2`: Empty free hook: returns at once.
pub fn oc815_attached_emitter_type_data_free_part2(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `AttachedEmitterTypeData_FreePart1`: an empty function (`jr ra`).
pub fn oc817_attached_emitter_type_data_free_part1(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `VisualEffectsMan_StaticInitDestroy(int init, int mode)` (0x002f6b88), the visual-effects
/// manager's static initialiser / destructor. Only for `mode == 0xffff`: `init != 0` fills the
/// static emitter and light parameter tables (about 600 cycles of float and vector stores from
/// the gp constants) and runs `mEmitter_ctor` over two arrays; `init == 0` runs
/// `VisualEffectsMan_EmitterHolderDtor` over the holder array backwards. Any other `mode` does
/// nothing. The store sequence is kept as the generated body schedules it (scratch registers
/// and cycles included).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn visual_effects_man_static_init_destroy(c: &mut Cpu) {
    if c.pc != 0x002f6b88 {
        return ee::s09::f_002f6b88__generated(c);
    }
    guest(c, |c| {
        'b_2f7a20: {
            'b_2f7a1c: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                c.g[V0][0] = 0xffff;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[A1][0] != c.g[V0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                if t {
                    break 'b_2f7a1c;
                } else {
                    'b_2f79e8: {
                        c.cycles += 2;
                        let t = c.g[A0][0] == 0u64;
                        c.g[A3][0] = 0x4d0000;
                        if t {
                            break 'b_2f79e8;
                        } else {
                            c.cycles += 605;
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3890));
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x57e0));
                            c.g[T3][0] = 0x4d0000;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x388c));
                            c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x5810));
                            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[S1][0] = 0x4d0000;
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 11);
                            c.g[T5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x57b0));
                            c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[A2][0] = 0x4d0000;
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3888));
                            c.g[A1][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3884));
                            c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x57d0));
                            c.swc1(11, (c.g[T5][0] as u32).wrapping_add(0xc));
                            c.g[T1][0] = 0x4d0000;
                            c.swc1(3, (c.g[A1][0] as u32).wrapping_add(0x57c0));
                            c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x57f0));
                            c.swc1(4, (c.g[A2][0] as u32).wrapping_add(0x57d0));
                            c.g[T2][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3880));
                            c.g[T7][0] = sx((c.g[T2][0] as u32).wrapping_add(0x5800));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T4][0] = 0x4d0000;
                            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x387c));
                            c.g[T6][0] = sx((c.g[T4][0] as u32).wrapping_add(0x5820));
                            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3878));
                            c.g[T0][0] = 0x4d0000;
                            c.swc1(8, (c.g[A3][0] as u32).wrapping_add(0x57e0));
                            c.g[S2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5830));
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3874));
                            c.g[S3][0] = 0x4d0000;
                            c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5840));
                            c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = 0x4d0000;
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3870));
                            c.g[A2][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5850));
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x386c));
                            c.g[S7][0] = 0x4d0000;
                            c.swc1(5, (c.g[T1][0] as u32).wrapping_add(0x57f0));
                            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x5860));
                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x3868));
                            c.g[T1][0] = 0x4d0000;
                            c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x5870));
                            c.swc1(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = 0x4d0000;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5880));
                            c.swc1(12, (c.g[T2][0] as u32).wrapping_add(0x5800));
                            c.g[S6][0] = 0x4d0000;
                            c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x3864));
                            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x58a0));
                            c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[S5][0] = 0x4d0000;
                            c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x3860));
                            c.g[T2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x58b0));
                            c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x385c));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(15, (c.g[T3][0] as u32).wrapping_add(0x5810));
                            c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x3858));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.swc1(17, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x3854));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(13, (c.g[T4][0] as u32).wrapping_add(0x5820));
                            c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58c0));
                            c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x3850));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(14, (c.g[T6][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x57c0));
                            c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0xc));
                            c.swc1(18, (c.g[T0][0] as u32).wrapping_add(0x5830));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x57b0));
                            c.g[T0][0] = 0x4d0000;
                            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                            c.g[T4][0] = sx((c.g[T0][0] as u32).wrapping_add(0x58d0));
                            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                            c.g[T0][0] = 0x4d0000;
                            c.g[T5][0] = 0x4d0000;
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = sx((c.g[T5][0] as u32).wrapping_add(0x58e0));
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[T5][0] = 0x4d0000;
                            c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x384c));
                            c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x57c0));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3848));
                            c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5900));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[T0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[T5][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5910));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.g[T0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5920));
                            c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x8));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(19, (c.g[S2][0] as u32).wrapping_add(0x4));
                            c.g[T6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5950));
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3844));
                            c.g[T9][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5940));
                            c.swc1(1, (c.g[A1][0] as u32).wrapping_add(0xc));
                            c.g[S0][0] = 0x4d0000;
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3840));
                            c.g[T7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5960));
                            c.swc1(7, (c.g[A3][0] as u32).wrapping_add(0xc));
                            c.g[S0][0] = 0x4d0000;
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x383c));
                            c.swc1(2, (c.g[S2][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3838));
                            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5980));
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3834));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(3, (c.g[S3][0] as u32).wrapping_add(0x5840));
                            c.swc1(4, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5990));
                            c.swc1(5, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x5850));
                            c.g[T8][0] = sx((c.g[S0][0] as u32).wrapping_add(0x59a0));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[A1][0] = 0x4d0000;
                            c.swc1(0, (c.g[S7][0] as u32).wrapping_add(0x5860));
                            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x59b0));
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3830));
                            c.g[A1][0] = 0x4d0000;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5890));
                            c.swc1(6, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = 0x7ff;
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x58a0));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x58b0));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[A2][0] = 0x4d0000;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5a10));
                            c.g[V0][0] = 0x4d0000;
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x382c));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5870));
                            c.swc1(7, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = 0x4d0000;
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5880));
                            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5a30));
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = 0x4d0000;
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5a40));
                            c.g[T1][0] = 0x4d0000;
                            c.g[A3][0] = c.g[T1][0];
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x5890));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5890));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3828));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[A3][0] = 0x4d0000;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                            c.sw((c.g[T2][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[V1][0] = 0x4d0000;
                            c.sw((c.g[T2][0] as u32).wrapping_add(0x8), 0u64);
                            c.sw((c.g[T2][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[T2][0] = 0x4d0000;
                            c.swc1(1, (c.g[S4][0] as u32).wrapping_add(0x5920));
                            c.g[T1][0] = sx((c.g[T2][0] as u32).wrapping_add(0x5ad0));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3824));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x58c0));
                            c.g[T2][0] = 0xffffffffffff0000;
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x58d0));
                            c.g[T2][0] = c.g[T2][0] | 0xff00;
                            c.swc1(11, (c.g[T4][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = c.g[S4][0];
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x58e0));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x58f0));
                            c.swc1(11, (c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x58f0));
                            c.swc1(11, (c.g[V1][0] as u32).wrapping_add(0xc));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3820));
                            c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5ae0));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x5900));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5910));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x381c));
                            c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5af0));
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3818));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3814));
                            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5b00));
                            c.swc1(5, (c.g[T5][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = 0x4d0000;
                            c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(6, (c.g[T0][0] as u32).wrapping_add(0x4));
                            c.swc1(7, (c.g[T0][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5930));
                            c.g[T0][0] = 0x4d0000;
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5930));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3810));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0xc));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x380c));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5940));
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5930));
                            c.swc1(11, (c.g[T4][0] as u32).wrapping_add(0x4));
                            c.g[T3][0] = 0x4d0000;
                            c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x8));
                            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x58f0));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[T4][0] = 0x4d0000;
                            c.swc1(11, (c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = 0x4d0000;
                            c.swc1(11, (c.g[T3][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x58f0));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3808));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[T3][0] = sx((c.g[T4][0] as u32).wrapping_add(0x5b60));
                            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = 0xc;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[T5][0] = 0xb;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[T9][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(2, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0xc));
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3804));
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3800));
                            c.swc1(3, (c.g[T9][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5950));
                            c.sw((c.g[T9][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(4, (c.g[T6][0] as u32).wrapping_add(0x4));
                            c.swc1(5, (c.g[T6][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x37fc));
                            c.g[T6][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x37f8));
                            c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x5960));
                            c.swc1(6, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.g[T6][0] = 0x4d0000;
                            c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.g[T7][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x37f4));
                            c.g[S4][0] = c.g[T7][0];
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x5970));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5970));
                            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x37f0));
                            c.swc1(8, (c.g[S4][0] as u32).wrapping_add(0x8));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x5980));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(9, (c.g[S2][0] as u32).wrapping_add(0x4));
                            c.g[T6][0] = 0xffffffffffffffff;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x5990));
                            c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0xc));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x59a0));
                            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x37ec));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x37e8));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5970));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x59b0));
                            c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0xc));
                            c.swc1(10, (c.g[S0][0] as u32).wrapping_add(0x8));
                            c.swc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x37e4));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x59c0));
                            c.g[T7][0] = c.g[S0][0];
                            c.swc1(13, (c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x59c0));
                            c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5a20));
                            c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x37e0));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x37dc));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x5a00));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x37d8));
                            c.g[S0][0] = c.g[T7][0];
                            c.swc1(3, (c.g[A1][0] as u32).wrapping_add(0xc));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x59d0));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x37d4));
                            c.swc1(2, (c.g[S4][0] as u32).wrapping_add(0xc));
                            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x37d0));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x59d0));
                            c.swc1(5, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[T7][0] = c.g[S4][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x59e0));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x59e0));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.g[S4][0] = c.g[S0][0];
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x59f0));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x59f0));
                            c.g[T7][0] = 0x4d0000;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5a00));
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x37cc));
                            c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[S0][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x37c8));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5a10));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x37c4));
                            c.g[A1][0] = c.g[S4][0];
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5a20));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5a20));
                            c.swc1(8, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x37c0));
                            c.g[A1][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x5a30));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x59d0));
                            c.swc1(9, (c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[T7][0] = 0x4d0000;
                            c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x59e0));
                            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x37bc));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5a40));
                            c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5a50));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x59f0));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5a00));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x4d0000;
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5a20));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[T7][0] = 0x4d0000;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5a60));
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5aa0));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.g[A2][0] = 0x4d0000;
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5a50));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x37b8));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5ab0));
                            c.swc1(13, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.g[A1][0] = 0x4d0000;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x37b4));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(1, (c.g[S4][0] as u32).wrapping_add(0xc));
                            c.g[S0][0] = c.g[T7][0];
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[S4][0] = 0x4d0000;
                            c.g[A2][0] = c.g[A1][0];
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5a60));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x5a70));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5a70));
                            c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5a80));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x5a80));
                            c.g[V0][0] = c.g[S4][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A1][0] = 0x4d0000;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5a90));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5a90));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A2][0] = c.g[A1][0];
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x37b0));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x5aa0));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5aa0));
                            c.swc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[S0][0] = c.g[T7][0];
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x37ac));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x5ab0));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5ab0));
                            c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.g[S4][0];
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x37a8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5ac0));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5ac0));
                            c.g[A1][0] = 0x4d0000;
                            c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = 0x4d0000;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5a50));
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x37a4));
                            c.g[T7][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x37a0));
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5a60));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x5ad0));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(6, (c.g[T1][0] as u32).wrapping_add(0x4));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5a70));
                            c.swc1(7, (c.g[T1][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5a80));
                            c.swc1(13, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5a90));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = 0x4d0000;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5aa0));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[A2][0] = 0x4d0000;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5ab0));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[T7][0] = 0x4d0000;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5ac0));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[S0][0] = 0xffffffffffffffff;
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x57c0));
                            c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A1][0] = 0x4d0000;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2440), c.g[S0][0]);
                            c.g[A2][0] = 0x4d0000;
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x379c));
                            c.g[T1][0] = 0x4d0000;
                            c.lwc1(9, (c.g[S4][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = c.g[S0][0] & 0xff;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3798));
                            c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                            c.swc1(2, (c.g[A3][0] as u32).wrapping_add(0xc));
                            c.g[T2][0] = c.g[T1][0];
                            c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x5b10));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3794));
                            c.g[T7][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3790));
                            c.g[S0][0] = c.g[T7][0];
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x5ae0));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b20));
                            c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = c.g[T0][0];
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5b50));
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x378c));
                            c.g[V1][0] = 0x4d0000;
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3788));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x5af0));
                            c.swc1(5, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A2][0] = 0x1;
                            c.swc1(6, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3784));
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3780));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x5b00));
                            c.swc1(7, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = 0x2;
                            c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x5b10));
                            c.g[A0][0] = c.g[V1][0];
                            c.sw((c.g[T2][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b40));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x5b20));
                            c.g[T1][0] = 0x4;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T7][0] = 0x6;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2440), c.g[V0][0]);
                            c.g[V0][0] = c.g[S4][0];
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5b30));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b30));
                            c.g[S4][0] = 0x8;
                            c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x5b40));
                            c.swc1(11, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.swc1(9, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x5b50));
                            c.sw((c.g[T2][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[T0][0] = 0x3;
                            c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[T2][0] = 0x5;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[S0][0] = 0x7;
                            c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2458), c.g[A2][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2480), c.g[T7][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2478), c.g[T2][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2460), c.g[A3][0]);
                            c.g[V1][0] = c.g[T7][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2468), c.g[T0][0]);
                            c.g[A3][0] = c.g[A2][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2470), c.g[T1][0]);
                            c.g[A0][0] = c.g[T2][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2450), 0u64);
                            c.g[T7][0] = 0x4d0000;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2488), c.g[S0][0]);
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5b50));
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2490), c.g[S4][0]);
                            c.g[A0][0] = c.g[A0][0] | 0x500;
                            c.g[T0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2460));
                            c.g[V0][0] = c.g[S0][0] & 0xff;
                            c.g[T1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2468));
                            c.g[V1][0] = c.g[V1][0] | 0x600;
                            c.g[A2][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2470));
                            c.g[V0][0] = c.g[V0][0] | 0x700;
                            c.g[T2][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2450));
                            c.g[A3][0] = c.g[A3][0] | 0x100;
                            c.swc1(9, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.g[A2][0] = c.g[A2][0] | 0x400;
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                            c.g[T0][0] = c.g[T0][0] | 0x200;
                            c.g[T1][0] = c.g[T1][0] | 0x300;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2470), c.g[A2][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2478), c.g[A0][0]);
                            c.g[A1][0] = c.g[S4][0] & 0xff;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2480), c.g[V1][0]);
                            c.g[A1][0] = c.g[A1][0] | 0x800;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2488), c.g[V0][0]);
                            c.g[S0][0] = 0x9;
                            c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x5b60));
                            c.g[S4][0] = 0xa;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2450), c.g[T2][0]);
                            c.g[V0][0] = 0xd;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2458), c.g[A3][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2460), c.g[T0][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2468), c.g[T1][0]);
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x244c), 0u64);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2448), 0u64);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2490), c.g[A1][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2498), c.g[S0][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x24a0), c.g[S4][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x24a8), c.g[T5][0]);
                            c.g[V1][0] = c.g[S0][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x24b0), c.g[S1][0]);
                            c.g[A1][0] = c.g[S4][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x24b8), c.g[V0][0]);
                            c.g[A2][0] = c.g[T5][0] & 0xff;
                            c.g[A0][0] = c.g[S1][0] & 0xff;
                            c.g[V1][0] = c.g[V1][0] | 0x900;
                            c.g[V0][0] = c.g[V0][0] & 0xff;
                            c.g[A1][0] = c.g[A1][0] | 0xa00;
                            c.g[A2][0] = c.g[A2][0] | 0xb00;
                            c.g[A0][0] = c.g[A0][0] | 0xc00;
                            c.g[V0][0] = c.g[V0][0] | 0xd00;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2498), c.g[V1][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x24a0), c.g[A1][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x24a8), c.g[A2][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x24b0), c.g[A0][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x24b8), c.g[V0][0]);
                            'l_2f7538: loop {
                                c.cycles += 7;
                                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x1));
                                let t = c.g[FP][0] != c.g[T6][0];
                                c.g[V0][0] = 0x3f;
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002f7538);
                                    }
                                    continue 'l_2f7538;
                                } else {
                                    c.cycles += 1;
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    'l_2f7558: loop {
                                        c.cycles += 7;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        let t = c.g[V0][0] != c.g[V1][0];
                                        c.g[S0][0] = 0x3f;
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002f7558);
                                            }
                                            continue 'l_2f7558;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x4f0000;
                                            c.g[S1][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0x1890));
                                            c.g[S2][0] = 0xffffffffffffffff;
                                            'l_2f7580: loop {
                                                c.cycles += 5;
                                                c.g[A0][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0xd0));
                                                c.g[RA][0] = sx(0x002f758c);
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                call_ptr(c, 0x003714b8, 0x002f758c)?;
                                                let t = c.g[S0][0] != c.g[S2][0];
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x2f0));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002f7580);
                                                    }
                                                    continue 'l_2f7580;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x377c),
                                                    );
                                                    c.g[V0][0] = 0x7;
                                                    c.g[V1][0] = 0xffffffffffffffff;
                                                    c.swc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_add(0x24bc),
                                                    );
                                                    'l_2f75a8: loop {
                                                        c.cycles += 7;
                                                        c.g[V0][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        let t = c.g[V0][0] != c.g[V1][0];
                                                        c.g[A0][0] = 0x500000;
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x002f75a8);
                                                            }
                                                            continue 'l_2f75a8;
                                                        } else {
                                                            c.cycles += 265;
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x5510));
                                                            c.lwc1(
                                                                4,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x3778),
                                                            );
                                                            c.g[V1][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x54f0));
                                                            c.g[A1][0] = 0x500000;
                                                            c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0x54d0));
                                                            c.g[AT][0] = 0x3f000000;
                                                            c.mtc1(1, 5);
                                                            c.g[A2][0] = 0x500000;
                                                            c.g[AT][0] = 0x3f800000;
                                                            c.mtc1(1, 0);
                                                            c.swc1(
                                                                4,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.g[A1][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_sub(0x54b0));
                                                            c.swc1(
                                                                5,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.g[T3][0] = 0x500000;
                                                            c.lwc1(
                                                                1,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x3774),
                                                            );
                                                            c.g[T6][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_sub(0x5510),
                                                            );
                                                            c.g[T5][0] = 0x500000;
                                                            c.swc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[T3][0] = sx((c.g[T5][0] as u32)
                                                                .wrapping_sub(0x53f0));
                                                            c.swc1(
                                                                0,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_sub(0x54f0),
                                                            );
                                                            c.g[T7][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.g[S0][0] = 0x500000;
                                                            c.g[S1][0] = 0x500000;
                                                            c.g[T5][0] = sx((c.g[T7][0] as u32)
                                                                .wrapping_sub(0x53d0));
                                                            c.swc1(
                                                                0,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_sub(0x54d0),
                                                            );
                                                            c.g[T6][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_sub(0x53b0));
                                                            c.g[AT][0] = 0x43a00000;
                                                            c.mtc1(1, 3);
                                                            c.g[S0][0] = 0x500000;
                                                            c.g[AT][0] = 0x43700000;
                                                            c.mtc1(1, 2);
                                                            c.g[T7][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.g[A3][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_sub(0x5490),
                                                            );
                                                            c.g[A2][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_sub(0x5490));
                                                            c.swc1(
                                                                0,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_sub(0x54b0),
                                                            );
                                                            c.g[T0][0] = 0x500000;
                                                            c.swc1(
                                                                3,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x24c8),
                                                            );
                                                            c.g[A3][0] = sx((c.g[T0][0] as u32)
                                                                .wrapping_sub(0x5470));
                                                            c.swc1(
                                                                2,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x24cc),
                                                            );
                                                            c.g[T1][0] = 0x500000;
                                                            c.sw(
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x24c4),
                                                                0u64,
                                                            );
                                                            c.g[T0][0] = sx((c.g[T1][0] as u32)
                                                                .wrapping_sub(0x5450));
                                                            c.sw(
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x24c0),
                                                                0u64,
                                                            );
                                                            c.g[T2][0] = 0x500000;
                                                            c.swc1(
                                                                4,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[T1][0] = sx((c.g[T2][0] as u32)
                                                                .wrapping_sub(0x5430));
                                                            c.swc1(
                                                                0,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[T4][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[T2][0] = sx((c.g[T4][0] as u32)
                                                                .wrapping_sub(0x5410));
                                                            c.swc1(
                                                                0,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[S4][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[T4][0] = sx((c.g[S4][0] as u32)
                                                                .wrapping_sub(0x5390));
                                                            c.swc1(0, c.g[SP][0] as u32);
                                                            c.g[V0][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[S1][0] = 0x4d0000;
                                                            c.swc1(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[T7][0] = 0x4d0000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[A1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x5870));
                                                            c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x5500));
                                                            c.ldl(
                                                                20,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(20, c.g[A1][0] as u32);
                                                            c.ldl(
                                                                3,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                3,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                20,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(20, c.g[A0][0] as u32);
                                                            c.sdl(
                                                                3,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                3,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[V0][0] = sx((c.g[T7][0] as u32)
                                                                .wrapping_add(0x5880));
                                                            c.g[S4][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x54e0));
                                                            c.ldl(
                                                                16,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(16, c.g[V0][0] as u32);
                                                            c.ldl(
                                                                17,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                17,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                16,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(16, c.g[S4][0] as u32);
                                                            c.sdl(
                                                                17,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                17,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V0][0] = 0x500000;
                                                            c.g[V1][0] = 0x4d0000;
                                                            c.g[S0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x5890));
                                                            c.g[T7][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x54c0));
                                                            c.ldl(
                                                                4,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(4, c.g[S0][0] as u32);
                                                            c.ldl(
                                                                5,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                5,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                4,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(4, c.g[T7][0] as u32);
                                                            c.sdl(
                                                                5,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                5,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[S1][0] = 0x4d0000;
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[T7][0] = 0x500000;
                                                            c.g[A1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x57b0));
                                                            c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x54a0));
                                                            c.ldl(
                                                                20,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(20, c.g[A1][0] as u32);
                                                            c.ldl(
                                                                2,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                2,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                20,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(20, c.g[A0][0] as u32);
                                                            c.sdl(
                                                                2,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                2,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[S0][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[S1][0] = 0x500000;
                                                            c.sw(
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.g[S4][0] = 0x500000;
                                                            c.g[A2][0] = 0x500000;
                                                            c.g[V0][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_sub(0x5470),
                                                            );
                                                            c.g[V1][0] = 0x4d0000;
                                                            c.sw(
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_sub(0x5450),
                                                            );
                                                            c.sw(
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_sub(0x5430),
                                                            );
                                                            c.sw(
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.g[S0][0] = 0x4d0000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_sub(0x5410),
                                                            );
                                                            c.sw(
                                                                (c.g[T2][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_sub(0x53f0),
                                                            );
                                                            c.sw(
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[T0][0] = 0x4d0000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T2][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A3][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x57c0));
                                                            c.g[A2][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x5480));
                                                            c.ldl(
                                                                4,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(4, c.g[A3][0] as u32);
                                                            c.ldl(
                                                                5,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                5,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                4,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(4, c.g[A2][0] as u32);
                                                            c.sdl(
                                                                5,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                5,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[V0][0] = 0x500000;
                                                            c.g[T7][0] = sx((c.g[T0][0] as u32)
                                                                .wrapping_add(0x57d0));
                                                            c.g[T3][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x5460));
                                                            c.ldl(
                                                                9,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(9, c.g[T7][0] as u32);
                                                            c.ldl(
                                                                10,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                10,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                9,
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(9, c.g[T3][0] as u32);
                                                            c.sdl(
                                                                10,
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                10,
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A1][0] = 0x4d0000;
                                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x57e0));
                                                            c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x5440));
                                                            c.ldl(
                                                                17,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(17, c.g[A0][0] as u32);
                                                            c.ldl(
                                                                20,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                20,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                17,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(17, c.g[V1][0] as u32);
                                                            c.sdl(
                                                                20,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                20,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[T2][0] = 0x4d0000;
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[V0][0] = 0x500000;
                                                            c.g[T1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x57f0));
                                                            c.g[T0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x5420));
                                                            c.ldl(
                                                                6,
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(6, c.g[T1][0] as u32);
                                                            c.ldl(
                                                                7,
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                7,
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                6,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(6, c.g[T0][0] as u32);
                                                            c.sdl(
                                                                7,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                7,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[S4][0] = 0x4d0000;
                                                            c.g[S1][0] = sx((c.g[T2][0] as u32)
                                                                .wrapping_add(0x5800));
                                                            c.g[S0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x5400));
                                                            c.ldl(
                                                                11,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(11, c.g[S1][0] as u32);
                                                            c.ldl(
                                                                15,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                15,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                11,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(11, c.g[S0][0] as u32);
                                                            c.sdl(
                                                                15,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                15,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[A2][0] = sx((c.g[S4][0] as u32)
                                                                .wrapping_add(0x5810));
                                                            c.g[A1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x53e0));
                                                            c.ldl(
                                                                2,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(2, c.g[A2][0] as u32);
                                                            c.ldl(
                                                                4,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                4,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                2,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(2, c.g[A1][0] as u32);
                                                            c.sdl(
                                                                4,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                4,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A3][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_sub(0x53d0),
                                                            );
                                                            c.g[T0][0] = 0x500000;
                                                            c.sw(
                                                                (c.g[T5][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.g[T1][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_sub(0x53b0),
                                                            );
                                                            c.g[V0][0] = 0x500000;
                                                            c.sw(
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.g[T2][0] = 0x4d0000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_sub(0x5390),
                                                            );
                                                            c.g[V1][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[T4][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[T7][0] = 0x4d0000;
                                                            c.sw(
                                                                (c.g[T4][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T5][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T5][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T4][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[T6][0] = sx((c.g[T2][0] as u32)
                                                                .wrapping_add(0x5820));
                                                            c.g[T5][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x53c0));
                                                            c.ldl(
                                                                11,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(11, c.g[T6][0] as u32);
                                                            c.ldl(
                                                                12,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                12,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                11,
                                                                (c.g[T5][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(11, c.g[T5][0] as u32);
                                                            c.sdl(
                                                                12,
                                                                (c.g[T5][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                12,
                                                                (c.g[T5][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V0][0] = sx((c.g[T7][0] as u32)
                                                                .wrapping_add(0x5830));
                                                            c.g[S4][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x53a0));
                                                            c.ldl(
                                                                16,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(16, c.g[V0][0] as u32);
                                                            c.ldl(
                                                                17,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                17,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                16,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(16, c.g[S4][0] as u32);
                                                            c.sdl(
                                                                17,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                17,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V0][0] = 0x500000;
                                                            c.g[V1][0] = 0x4d0000;
                                                            c.g[A3][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x5840));
                                                            c.g[A2][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x5380));
                                                            c.ldl(
                                                                4,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(4, c.g[A3][0] as u32);
                                                            c.ldl(
                                                                5,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                5,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                4,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(4, c.g[A2][0] as u32);
                                                            c.sdl(
                                                                5,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                5,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xa0),
                                                            );
                                                            if t {
                                                                break 'b_2f7a20;
                                                            } else {
                                                                break 'b_2f79e8;
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
                    c.cycles += 4;
                    c.g[V0][0] = 0x4f0000;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1890));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x500000;
                    if t {
                        break 'b_2f7a1c;
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5c90));
                        let t = c.g[V1][0] == c.g[S0][0];
                        c.g[S1][0] = c.g[V1][0];
                        if t {
                            break 'b_2f7a1c;
                        } else {
                            c.cycles += 1;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2f0));
                            'l_2f7a08: loop {
                                c.cycles += 5;
                                c.g[A1][0] = 0u64;
                                c.g[RA][0] = sx(0x002f7a14);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x002d56b0,
                                    f: ee::s09::VisualEffectsMan_EmitterHolderDtor,
                                }
                                .call(c, 0x002f7a14)?;
                                let t = c.g[S0][0] != c.g[S1][0];
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2f0));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002f7a08);
                                    }
                                    continue 'l_2f7a08;
                                } else {
                                    break 'b_2f7a1c;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            break 'b_2f7a20;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// Replacements in this module: (address, function, name).
/// `AttachedEmitterTypeData_StaticInit1` (priority 0xffff, initialize 1): fills the
/// attached-emitter colour table at 0x4faca0.. (sColor constructors). Leaf; the
/// body is transcribed from the generated text with its register and cycle effects.
pub fn attached_emitter_type_data_static_init1(c: &mut Cpu) {
    'b_2f97ec: {
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
        let t = c.g[A1][0] != c.g[V0][0];
        c.sq(30, c.g[SP][0] as u32);
        if t {
            break 'b_2f97ec;
        } else {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[T5][0] = 0x500000;
            if t {
                break 'b_2f97ec;
            } else {
                c.cycles += 477;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x3704));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3700));
                c.g[T6][0] = 0x500000;
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x36fc));
                c.g[T7][0] = 0x500000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x36f8));
                c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x5340));
                c.swc1(9, (c.g[T5][0] as u32).wrapping_sub(0x5360));
                c.g[S1][0] = 0x500000;
                c.swc1(10, (c.g[T6][0] as u32).wrapping_sub(0x5350));
                c.g[S2][0] = 0x500000;
                c.swc1(11, (c.g[T7][0] as u32).wrapping_sub(0x5340));
                c.g[S3][0] = 0x500000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x36f4));
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5310));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x36f0));
                c.g[S5][0] = 0x500000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x36ec));
                c.g[S4][0] = 0x500000;
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[T3][0] = 0x500000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x36e8));
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x5370));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x5360));
                c.swc1(8, (c.g[S1][0] as u32).wrapping_sub(0x5330));
                c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x5350));
                c.swc1(4, (c.g[S2][0] as u32).wrapping_sub(0x5320));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x5330));
                c.swc1(5, (c.g[S3][0] as u32).wrapping_sub(0x5310));
                c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5320));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x36e4));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x5300));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x36e0));
                c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x52f0));
                c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[T9][0] = 0x500000;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 6);
                c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_sub(0x52e0));
                c.swc1(3, (c.g[S4][0] as u32).wrapping_sub(0x5300));
                c.g[S6][0] = 0x500000;
                c.swc1(1, (c.g[T3][0] as u32).wrapping_sub(0x52f0));
                c.g[T7][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x52b0));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_sub(0x5370));
                c.g[S2][0] = 0x500000;
                c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5290));
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S3][0] = 0x500000;
                c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x36dc));
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5280));
                c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x36d8));
                c.g[T8][0] = 0x500000;
                c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = 0x500000;
                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x36d4));
                c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x5270));
                c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x36d0));
                c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_sub(0x52d0));
                c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[S7][0] = 0x500000;
                c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x500000;
                c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x500000;
                c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5260));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T6][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x52c0));
                c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x36cc));
                c.g[V1][0] = 0x500000;
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x36c8));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5220));
                c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x5200));
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x36c4));
                c.g[S0][0] = 0x500000;
                c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5210));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x51f0));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x51e0));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x500000;
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x51d0));
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0x500000;
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x36c0));
                c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x51a0));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = 0x500000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x36bc));
                c.g[A2][0] = 0x500000;
                c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x51c0));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x36b8));
                c.g[A2][0] = 0x500000;
                c.swc1(2, (c.g[T9][0] as u32).wrapping_sub(0x52e0));
                c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5190));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x36b4));
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x5170));
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x36b0));
                c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0xc));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x36ac));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x36a8));
                c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T8][0] as u32).wrapping_sub(0x52d0));
                c.g[T2][0] = 0x500000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
                c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x5110));
                c.swc1(0, (c.g[S7][0] as u32).wrapping_sub(0x52c0));
                c.g[T2][0] = 0x500000;
                c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x36a4));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_sub(0x52b0));
                c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x52a0));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x52a0));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x36a0));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x5290));
                c.g[T4][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x50f0));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x369c));
                c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x50e0));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3698));
                c.g[S0][0] = 0x500000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3694));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x5280));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x5270));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
                c.g[T5][0] = 0x500000;
                c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x52a0));
                c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.g[T6][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5070));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[T7][0] = 0x500000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                c.g[S2][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x5090));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                c.g[T7][0] = 0x500000;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
                c.g[T5][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x5060));
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T7][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5040));
                c.g[S3][0] = 0x500000;
                c.g[S0][0] = 0x500000;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5230));
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5030));
                c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5020));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3690));
                c.g[S0][0] = 0x500000;
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x368c));
                c.swc1(1, (c.g[S0][0] as u32).wrapping_sub(0x5200));
                c.g[S0][0] = 0x500000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3688));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x5260));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x5250));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5250));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3684));
                c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3680));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x5240));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5240));
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x367c));
                c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3678));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x5230));
                c.g[S0][0] = 0x500000;
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x3674));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x5220));
                c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x500000;
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x500000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3670));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x5210));
                c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x500000;
                c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5250));
                c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = 0x500000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x366c));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x51f0));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x51e0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5240));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5230));
                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.swc1(2, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T0][0] as u32).wrapping_add(0xc));
                c.swc1(3, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3668));
                c.g[FP][0] = 0x500000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3664));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x51d0));
                c.swc1(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x500000;
                c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3660));
                c.g[A0][0] = c.g[V1][0];
                c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x51c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x51b0));
                c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[FP][0];
                c.swc1(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5180));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x365c));
                c.g[A1][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x51b0));
                c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x51a0));
                c.g[A0][0] = 0x500000;
                c.swc1(9, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x5190));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x51b0));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x5180));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x5170));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x5100));
                c.swc1(10, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.swc1(11, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x3658));
                c.g[A2][0] = c.g[A0][0];
                c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x5160));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5160));
                c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[A0][0];
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5140));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x500000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.g[V0][0];
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3654));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5150));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3650));
                c.g[A2][0] = 0x500000;
                c.swc1(4, (c.g[S0][0] as u32).wrapping_sub(0x5120));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x364c));
                c.g[S0][0] = c.g[A2][0];
                c.swc1(3, (c.g[A3][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5130));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3648));
                c.swc1(2, (c.g[FP][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x500000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3644));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x5120));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x5150));
                c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x500000;
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x5140));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[V1][0];
                c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x5130));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5100));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A1][0] = 0x500000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A2][0] = 0x500000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3640));
                c.g[S0][0] = 0x500000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x363c));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5150));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x5110));
                c.g[FP][0] = 0x500000;
                c.swc1(6, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x5140));
                c.swc1(7, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x500000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3638));
                c.g[A3][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x5100));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5130));
                c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x500000;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x3634));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5120));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x50f0));
                c.swc1(9, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0x500000;
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x50c0));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3630));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x50e0));
                c.swc1(10, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = 0x500000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x50d0));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5080));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x500000;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                c.g[V0][0] = c.g[FP][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x50a0));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[A3][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x50d0));
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[T2][0] = 0x500000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x362c));
                c.g[A3][0] = 0x500000;
                c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = c.g[T2][0];
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3628));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x50b0));
                c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = 0x500000;
                c.swc1(1, (c.g[T6][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x50c0));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x50b0));
                c.g[A3][0] = 0x500000;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T2][0] = c.g[A3][0];
                c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x50a0));
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x5050));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x5090));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x50d0));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = 0x500000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3624));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x50c0));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x5080));
                c.g[V0][0] = 0x500000;
                c.swc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x50b0));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3620));
                c.g[A2][0] = 0x500000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x5070));
                c.g[V1][0] = 0x500000;
                c.swc1(4, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x50a0));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x361c));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5080));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x5060));
                c.g[A1][0] = 0x500000;
                c.swc1(5, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = 0x500000;
                c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3618));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3614));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x5050));
                c.swc1(6, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0x500000;
                c.swc1(7, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.swc1(12, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3610));
                c.sw((c.g[T2][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x360c));
                c.g[T2][0] = 0x500000;
                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[T5][0] = c.g[T2][0];
                c.swc1(1, (c.g[S3][0] as u32).wrapping_add(0xc));
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x5010));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3608));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3604));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x5040));
                c.swc1(3, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.swc1(4, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3600));
                c.g[T7][0] = 0x500000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x35fc));
                c.g[S0][0] = c.g[T7][0];
                c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x5030));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5000));
                c.swc1(5, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.swc1(6, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x35f8));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x35f4));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x5020));
                c.swc1(7, (c.g[S3][0] as u32).wrapping_add(0x4));
                c.swc1(8, (c.g[S3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x5010));
                c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[T7][0] as u32).wrapping_sub(0x5000));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                c.sw((c.g[T5][0] as u32).wrapping_add(0x4), 0u64);
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                break 'b_2f97ec;
            }
        }
    }
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
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = t;
    return;
}

/// `void AttachedEmitterTypeData_StaticInit2(int init, uint mask)` (0x2fa640):
/// when `mask == 0xffff` and `init != 0`, resets the attached-emitter type data
/// statics (the 0x4fb010..0x4fb380 table of float vectors) to their initial
/// values; otherwise does nothing. A leaf: the body is the flat store table of
/// the generated code (scripted transcription, cycle counts included).
pub fn attached_emitter_type_data_static_init2(c: &mut Cpu) {
    'b_2fadec: {
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
        let t = c.g[A1][0] != c.g[V0][0];
        c.sq(30, c.g[SP][0] as u32);
        if t {
            break 'b_2fadec;
        } else {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[T5][0] = 0x500000;
            if t {
                break 'b_2fadec;
            } else {
                c.cycles += 477;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x34b0));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x34ac));
                c.g[T6][0] = 0x500000;
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x34a8));
                c.g[T7][0] = 0x500000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x34a4));
                c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x4fc0));
                c.swc1(9, (c.g[T5][0] as u32).wrapping_sub(0x4fe0));
                c.g[S1][0] = 0x500000;
                c.swc1(10, (c.g[T6][0] as u32).wrapping_sub(0x4fd0));
                c.g[S2][0] = 0x500000;
                c.swc1(11, (c.g[T7][0] as u32).wrapping_sub(0x4fc0));
                c.g[S3][0] = 0x500000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x34a0));
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4f90));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x349c));
                c.g[S5][0] = 0x500000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3498));
                c.g[S4][0] = 0x500000;
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[T3][0] = 0x500000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3494));
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x4ff0));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x4fe0));
                c.swc1(8, (c.g[S1][0] as u32).wrapping_sub(0x4fb0));
                c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x4fd0));
                c.swc1(4, (c.g[S2][0] as u32).wrapping_sub(0x4fa0));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4fb0));
                c.swc1(5, (c.g[S3][0] as u32).wrapping_sub(0x4f90));
                c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x4fa0));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3490));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x4f80));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x348c));
                c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x4f70));
                c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[T9][0] = 0x500000;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 6);
                c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_sub(0x4f60));
                c.swc1(3, (c.g[S4][0] as u32).wrapping_sub(0x4f80));
                c.g[S6][0] = 0x500000;
                c.swc1(1, (c.g[T3][0] as u32).wrapping_sub(0x4f70));
                c.g[T7][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x4f30));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_sub(0x4ff0));
                c.g[S2][0] = 0x500000;
                c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x4f10));
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S3][0] = 0x500000;
                c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x3488));
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4f00));
                c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x3484));
                c.g[T8][0] = 0x500000;
                c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = 0x500000;
                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x3480));
                c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x4ef0));
                c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x347c));
                c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_sub(0x4f50));
                c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[S7][0] = 0x500000;
                c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x500000;
                c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x500000;
                c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4ee0));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T6][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x4f40));
                c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x3478));
                c.g[V1][0] = 0x500000;
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x3474));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4ea0));
                c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x4e80));
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x3470));
                c.g[S0][0] = 0x500000;
                c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e90));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e70));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e60));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x500000;
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4e50));
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0x500000;
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x346c));
                c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4e20));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = 0x500000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3468));
                c.g[A2][0] = 0x500000;
                c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4e40));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3464));
                c.g[A2][0] = 0x500000;
                c.swc1(2, (c.g[T9][0] as u32).wrapping_sub(0x4f60));
                c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4e10));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3460));
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4df0));
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x345c));
                c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0xc));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3458));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3454));
                c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T8][0] as u32).wrapping_sub(0x4f50));
                c.g[T2][0] = 0x500000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
                c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x4d90));
                c.swc1(0, (c.g[S7][0] as u32).wrapping_sub(0x4f40));
                c.g[T2][0] = 0x500000;
                c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3450));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_sub(0x4f30));
                c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x4f20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4f20));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x344c));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x4f10));
                c.g[T4][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4d70));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3448));
                c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4d60));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3444));
                c.g[S0][0] = 0x500000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3440));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x4f00));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x4ef0));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
                c.g[T5][0] = 0x500000;
                c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x4f20));
                c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.g[T6][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4cf0));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[T7][0] = 0x500000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                c.g[S2][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x4d10));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                c.g[T7][0] = 0x500000;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
                c.g[T5][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x4ce0));
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T7][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4cc0));
                c.g[S3][0] = 0x500000;
                c.g[S0][0] = 0x500000;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4eb0));
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4cb0));
                c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4ca0));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x343c));
                c.g[S0][0] = 0x500000;
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3438));
                c.swc1(1, (c.g[S0][0] as u32).wrapping_sub(0x4e80));
                c.g[S0][0] = 0x500000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3434));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x4ee0));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x4ed0));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4ed0));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3430));
                c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x342c));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x4ec0));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4ec0));
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x3428));
                c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3424));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x4eb0));
                c.g[S0][0] = 0x500000;
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x3420));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x4ea0));
                c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x500000;
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x500000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x341c));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x4e90));
                c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x500000;
                c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4ed0));
                c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = 0x500000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3418));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x4e70));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x4e60));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4ec0));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4eb0));
                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.swc1(2, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T0][0] as u32).wrapping_add(0xc));
                c.swc1(3, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3414));
                c.g[FP][0] = 0x500000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3410));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x4e50));
                c.swc1(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x500000;
                c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x340c));
                c.g[A0][0] = c.g[V1][0];
                c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x4e40));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e30));
                c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[FP][0];
                c.swc1(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4e00));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3408));
                c.g[A1][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x4e30));
                c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x4e20));
                c.g[A0][0] = 0x500000;
                c.swc1(9, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x4e10));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4e30));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x4e00));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x4df0));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x4d80));
                c.swc1(10, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.swc1(11, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x3404));
                c.g[A2][0] = c.g[A0][0];
                c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x4de0));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4de0));
                c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[A0][0];
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4dc0));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x500000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.g[V0][0];
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3400));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4dd0));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x33fc));
                c.g[A2][0] = 0x500000;
                c.swc1(4, (c.g[S0][0] as u32).wrapping_sub(0x4da0));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x33f8));
                c.g[S0][0] = c.g[A2][0];
                c.swc1(3, (c.g[A3][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4db0));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x33f4));
                c.swc1(2, (c.g[FP][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x500000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x33f0));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x4da0));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x4dd0));
                c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x500000;
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x4dc0));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[V1][0];
                c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x4db0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4d80));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A1][0] = 0x500000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A2][0] = 0x500000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x33ec));
                c.g[S0][0] = 0x500000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x33e8));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4dd0));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x4d90));
                c.g[FP][0] = 0x500000;
                c.swc1(6, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x4dc0));
                c.swc1(7, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x500000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x33e4));
                c.g[A3][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x4d80));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4db0));
                c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x500000;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x33e0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4da0));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x4d70));
                c.swc1(9, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0x500000;
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4d40));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x33dc));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x4d60));
                c.swc1(10, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = 0x500000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x4d50));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4d00));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x500000;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                c.g[V0][0] = c.g[FP][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4d20));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[A3][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4d50));
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[T2][0] = 0x500000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x33d8));
                c.g[A3][0] = 0x500000;
                c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = c.g[T2][0];
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x33d4));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4d30));
                c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = 0x500000;
                c.swc1(1, (c.g[T6][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x4d40));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x4d30));
                c.g[A3][0] = 0x500000;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T2][0] = c.g[A3][0];
                c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x4d20));
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x4cd0));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x4d10));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4d50));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = 0x500000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x33d0));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x4d40));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x4d00));
                c.g[V0][0] = 0x500000;
                c.swc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4d30));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x33cc));
                c.g[A2][0] = 0x500000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x4cf0));
                c.g[V1][0] = 0x500000;
                c.swc1(4, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4d20));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x33c8));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4d00));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x4ce0));
                c.g[A1][0] = 0x500000;
                c.swc1(5, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = 0x500000;
                c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x33c4));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x33c0));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x4cd0));
                c.swc1(6, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0x500000;
                c.swc1(7, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.swc1(12, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x33bc));
                c.sw((c.g[T2][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x33b8));
                c.g[T2][0] = 0x500000;
                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[T5][0] = c.g[T2][0];
                c.swc1(1, (c.g[S3][0] as u32).wrapping_add(0xc));
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x4c90));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x33b4));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x33b0));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x4cc0));
                c.swc1(3, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.swc1(4, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x33ac));
                c.g[T7][0] = 0x500000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x33a8));
                c.g[S0][0] = c.g[T7][0];
                c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x4cb0));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4c80));
                c.swc1(5, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.swc1(6, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x33a4));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x33a0));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x4ca0));
                c.swc1(7, (c.g[S3][0] as u32).wrapping_add(0x4));
                c.swc1(8, (c.g[S3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x4c90));
                c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[T7][0] as u32).wrapping_sub(0x4c80));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                c.sw((c.g[T5][0] as u32).wrapping_add(0x4), 0u64);
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                break 'b_2fadec;
            }
        }
    }
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
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// RiderBase_DtorCheatCode (0x2f6388)

/// Offsets in the rider's data block (`*self`).
pub(crate) const CHEAT_TABLE_PTR: u32 = 0x6c0;

pub(crate) const CHEAT_HANDLER: u32 = 0x6d0;

pub(crate) const CHEAT_LIST_PTR: u32 = 0x6e8;

/// Table starts in the game image: the cheat list (copied in 0x20-byte
/// chunks up to its end at +0x40), the cheat table (copied up to +0xa0), the
/// handler word and the two relative-offset halfwords.
/// Upper half of the image addresses (`lui 0x49`, then `addiu`).
pub(crate) const CHEAT_IMAGE_HI: u32 = 0x0049_0000;

/// Base destructor (the rider data block, flags 0) and the delete operator.
pub(crate) const RIDER_DTOR: u32 = 0x0011_b978;

pub(crate) const BUILTIN_DELETE: u32 = 0x0031_7e50;

/// `void RiderBase_DtorCheatCode(RiderBase* self, u64 flags)` (0x2f6388): the
/// cheat-code part of the rider destructor. The rider's data block (`*self`)
/// gets its cheat handler, table and list pointed at the defaults in the game
/// image. Unless `flags` is non-zero, the list and the table are first copied
/// to the stack frame and the rider points at the copies, whose
/// offset halfwords are rebased to `self`. Bit 1 of `flags` runs the base
/// destructor `Rider_dtor(data, 0)`; bit 0 deletes `self`.
pub fn rider_base_dtor_cheat_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x130);
        c.g[V0][0] = CHEAT_IMAGE_HI as u64;
        frame.save(c, 16, 0x120);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7920));
        frame.save(c, 17, 0x110);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x100);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = 0x0046_0000;
        c.g[A3][0] = c.g[V0][0];
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
        let list = c.g[A2][0];
        Addr((c.g[V1][0] as u32).wrapping_add(CHEAT_LIST_PTR)).sw(c, list);
        c.g[V1][0] = CHEAT_IMAGE_HI as u64;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[T2][0] = c.g[V1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x78d8));
        let handler = c.g[A0][0];
        Addr((c.g[A1][0] as u32).wrapping_add(CHEAT_HANDLER)).sw(c, handler);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let skip_copy = c.g[S1][0] != 0u64;
        let table = c.g[V1][0];
        Addr((c.g[V0][0] as u32).wrapping_add(CHEAT_TABLE_PTR)).sw(c, table);

        if !skip_copy {
            copy_cheat_tables(c);
        }

        // Bit 1: the base destructor on the data block.
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x2;
        let no_base = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if !no_base {
            c.cycles += 4;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = 0u64;
            call_ptr(c, RIDER_DTOR, 0x002f_64bc)?;
            c.g[V0][0] = c.g[S1][0] & 0x1;
        }

        // Bit 0: delete `self`.
        c.cycles += 2;
        if c.g[V0][0] != 0u64 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, BUILTIN_DELETE, 0x002f_64d0)?;
        }
        frame.restore(c, 16, 0x120);

        c.cycles += 4;
        frame.restore(c, 17, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// The stack copy of the cheat list and table (the `flags == 0` path of
/// [`rider_base_dtor_cheat_code`]). Loops check events at their back edges.
/// The register values after each loop are the generated body's.
pub(crate) fn copy_cheat_tables(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = c.g[A2][0];
    c.g[V1][0] = c.g[SP][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    // List: 0x20-byte chunks from the image to the last entry.
    loop {
        c.cycles += 11;
        c.g[A0][0] = Addr(c.g[A1][0] as u32).ld(c);
        c.g[A2][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).ld(c);
        c.g[T1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10)).ld(c);
        c.g[T3][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x18)).ld(c);
        let (w0, w1, w2, w3) = (c.g[A0][0], c.g[A2][0], c.g[T1][0], c.g[T3][0]);
        Addr(c.g[V1][0] as u32).sd(c, w0);
        Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sd(c, w1);
        Addr((c.g[V1][0] as u32).wrapping_add(0x10)).sd(c, w2);
        Addr((c.g[V1][0] as u32).wrapping_add(0x18)).sd(c, w3);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
        let more = c.g[A1][0] != c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002f_63ec);
            }
            continue;
        }
        break;
    }
    // Last list entry, the rebased offset halfword, the list head at the stack.
    c.cycles += 14;
    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7920));
    c.g[A2][0] = Addr(c.g[A1][0] as u32).ld(c);
    let last = c.g[A2][0];
    Addr(c.g[V1][0] as u32).sd(c, last);
    let stack_head = c.g[SP][0];
    Addr((c.g[V0][0] as u32).wrapping_add(CHEAT_LIST_PTR)).sw(c, stack_head);
    c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x78d8));
    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lhu(c);
    c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[A1][0] = c.g[T0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    let rebased = c.g[V1][0];
    Addr(sp(c).wrapping_add(0x8)).sh(c, rebased);
    // Table: 0x20-byte chunks to the end of the table.
    loop {
        c.cycles += 11;
        c.g[V0][0] = Addr(c.g[A2][0] as u32).ld(c);
        c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8)).ld(c);
        c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x10)).ld(c);
        c.g[T3][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x18)).ld(c);
        let (v0, v1, a0, t3) = (c.g[V0][0], c.g[V1][0], c.g[A0][0], c.g[T3][0]);
        Addr(c.g[A1][0] as u32).sd(c, v0);
        Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sd(c, v1);
        Addr((c.g[A1][0] as u32).wrapping_add(0x10)).sd(c, a0);
        Addr((c.g[A1][0] as u32).wrapping_add(0x18)).sd(c, t3);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
        let more = c.g[A2][0] != c.g[T1][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002f_6450);
            }
            continue;
        }
        break;
    }
    c.cycles += 10;
    c.g[V0][0] = Addr(c.g[A2][0] as u32).ld(c);
    c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8)).ld(c);
    let (v0, v1) = (c.g[V0][0], c.g[V1][0]);
    Addr(c.g[A1][0] as u32).sd(c, v0);
    Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sd(c, v1);
    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x78d8));
    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lhu(c);
    let t0 = c.g[T0][0];
    Addr((c.g[A0][0] as u32).wrapping_add(CHEAT_TABLE_PTR)).sw(c, t0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    let rebased_table = c.g[V1][0];
    Addr(sp(c).wrapping_add(0x58)).sh(c, rebased_table);
}

// ---------------------------------------------------------------------------
// The effect-object reset thunks (0x2f7ab8..0x2f7b90): tail calls to the
// `Reset` of each board effect, 7-cycle forwarding wrappers.

/// `cBoardWakeFX_Reset` (0x2dcf28).
pub(crate) const BOARD_WAKE_RESET: Callee = Callee {
    addr: 0x002d_cf28,
    f: ee::s09::f_002dcf28,
};

/// `cBoardSparkFX_Reset` (0x2daa78).
pub(crate) const BOARD_SPARK_RESET: Callee = Callee {
    addr: 0x002d_aa78,
    f: ee::s09::f_002daa78,
};

/// `cBoardTrailFX_Reset` (0x2e8560).
pub(crate) const BOARD_TRAIL_RESET: Callee = Callee {
    addr: 0x002e_8560,
    f: ee::s09::f_002e8560,
};

/// `cAngerFX_Reset` (0x2d4be0).
pub(crate) const ANGER_RESET: Callee = Callee {
    addr: 0x002d_4be0,
    f: ee::s09::f_002d4be0,
};

/// `cPlayerPulseFX_Reset` (0x2e3930).
pub(crate) const PLAYER_PULSE_RESET: Callee = Callee {
    addr: 0x002e_3930,
    f: ee::s09::f_002e3930,
};

/// `cAirStreamerFX_Reset` (0x2ef6a0).
pub(crate) const AIR_STREAMER_RESET: Callee = Callee {
    addr: 0x002e_f6a0,
    f: ee::s09::f_002ef6a0,
};

/// `void cBoardWakeFX_ResetThunk(void)` (0x2f7ab8): `cBoardWakeFX_Reset()`.
pub fn cboard_wake_fx_reset_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(BOARD_WAKE_RESET), 0x002f_7ac8)
}

/// `void cBoardSparkFX_ResetThunk(void)` (0x2f7ae0): `cBoardSparkFX_Reset()`.
pub fn cboard_spark_fx_reset_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(BOARD_SPARK_RESET), 0x002f_7af0)
}

/// `void cBoardTrailFX_ResetThunk(void)` (0x2f7b08): `cBoardTrailFX_Reset()`.
pub fn cboard_trail_fx_reset_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(BOARD_TRAIL_RESET), 0x002f_7b18)
}

/// `void cAngerFX_ResetThunk(void)` (0x2f7b30): `cAngerFX_Reset()`.
pub fn cang_er_fx_reset_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(ANGER_RESET), 0x002f_7b40)
}

/// `void cPlayerPulseFX_ResetThunk(void)` (0x2f7b58): `cPlayerPulseFX_Reset()`.
pub fn cplayer_pulse_fx_reset_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(PLAYER_PULSE_RESET), 0x002f_7b68)
}

/// `void cAirStreamerFX_ResetThunk(void)` (0x2f7b80): `cAirStreamerFX_Reset()`.
pub fn cair_streamer_fx_reset_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(AIR_STREAMER_RESET), 0x002f_7b90)
}

// ---------------------------------------------------------------------------
// VfxAvalanche_ctor (0x2f7c38)

pub(crate) const AVALANCHE_INIT: Callee = Callee {
    addr: 0x002d_6410,
    f: ee::s09::f_002d6410,
};

/// `void VfxAvalanche_ctor(VfxAvalanche* self)` (0x2f7c38): stores the
/// avalanche vtable in `self` and calls `VfxAvalanche_Init(self)`; returns
/// `self`.
pub fn vfx_avalanche_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = CHEAT_IMAGE_HI as u64;
        frame.save(c, 16, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x79b8));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let vtable = c.g[V0][0];
        Addr(c.g[S0][0] as u32).sw(c, vtable);
        AVALANCHE_INIT.call(c, 0x002f_7c58)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cSpeedBoostStreamerFX::ResetThunk (0x2f7ea8)

/// `void cSpeedBoostStreamerFX_ResetThunk(void)` (0x2f7ea8): forwards to
/// `cSpeedBoostStreamerFX_Reset()` (0x2e6640) in gcc's 7-cycle wrapper shape.
pub fn c_speed_boost_streamer_fx_reset_thunk(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x002e_6640,
            f: ee::s09::f_002e6640,
        }),
        0x002f_7eb8,
    );
}

// ---------------------------------------------------------------------------
// TransitionEffect_FadeCtor (0x2f8060): cFade_TObj::cFade_TObj(this, colour)

/// `cFade_TObj::cFade_TObj(this, colour)` (0x2f8060): runs
/// `TransitionEffect_ctor(this)` (0x2e4d70) first, with the two halves of the
/// 16-byte colour block at `colour` left in v0/v1 (the registers the original
/// holds at that call), then writes the fade vtable word (0x4880c0) at `this`
/// and copies the colour block to `this + 4 .. this + 0x14`. The colour
/// block is staged on the stack through unaligned doubleword halves
/// (`ldl`/`ldr`, `sdl`/`sdr`).
pub fn transition_effect_fade_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);

        // Copy colour[0..16] to the stack buffer at sp+0.
        let colour = c.g[A1][0] as u32;
        c.ldl(2, colour.wrapping_add(0x7));
        c.ldr(2, colour);
        c.ldl(3, colour.wrapping_add(0xf));
        c.ldr(3, colour.wrapping_add(0x8));
        let buf = frame.at(c, 0);
        c.sdl(2, buf.wrapping_add(0x7));
        c.sdr(2, buf);
        c.sdl(3, buf.wrapping_add(0xf));
        c.sdr(3, buf.wrapping_add(0x8));

        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x002e_4d70,
            f: ee::s09::f_002e4d70,
        }
        .call(c, 0x002f_8098)?;

        c.g[V1][0] = sx(0x0049_0000u32.wrapping_sub(0x7f40));
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        let this = c.g[S0][0] as u32;
        c.sw(this, c.g[V1][0]);

        // Copy the stack buffer to this + 4 .. this + 0x14.
        c.ldl(3, buf.wrapping_add(0x7));
        c.ldr(3, buf);
        c.ldl(4, buf.wrapping_add(0xf));
        c.ldr(4, buf.wrapping_add(0x8));
        c.sdl(3, this.wrapping_add(0xb));
        c.sdr(3, this.wrapping_add(0x4));
        c.sdl(4, this.wrapping_add(0x13));
        c.sdr(4, this.wrapping_add(0xc));

        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cCutSceneTransitionMan dtor (0x2f81b8)

/// `cCutSceneTransitionMan::~cCutSceneTransitionMan()` (0x2f81b8): forwards
/// to `GameProcess_dtor` (0x3546c8) through the function table (gcc's 7-cycle
/// wrapper shape with `jalr`).
pub fn c_cut_scene_transition_man_dtor(c: &mut Cpu) {
    crate::call::tail_wrapper(c, None, Target::Table(0x0035_46c8), 0x002f_81c8);
}

// ---------------------------------------------------------------------------
// Static constructor and destructor wrappers (0x2f8370 .. 0x2fae18)

/// The 8-cycle static-init wrappers: `StaticInit(a0, 0xffff)` in a 0x10
/// frame, returning what the callee leaves.
pub(crate) fn static_init_forward(c: &mut Cpu, a0: u64, callee: Callee, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = a0;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        callee.call(c, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `VisualEffectsMan_GlobalCtors` (0x2f8370): `VisualEffectsMan_StaticInitDestroy(1, 0xffff)`.
pub fn visual_effects_man_global_ctors(c: &mut Cpu) {
    static_init_forward(
        c,
        1,
        Callee {
            addr: 0x002f_6b88,
            f: ee::s09::f_002f6b88,
        },
        0x002f_8384,
    );
}

/// `VisualEffectsMan_GlobalDtors` (0x2f8390): `VisualEffectsMan_StaticInitDestroy(0, 0xffff)`.
pub fn visual_effects_man_global_dtors(c: &mut Cpu) {
    static_init_forward(
        c,
        0,
        Callee {
            addr: 0x002f_6b88,
            f: ee::s09::f_002f6b88,
        },
        0x002f_83a4,
    );
}

/// `AttachedEmitterTypeData_GlobalCtors1` (0x2f9818): `AttachedEmitterTypeData_StaticInit1(1, 0xffff)`.
pub fn attached_emitter_type_data_global_ctors1(c: &mut Cpu) {
    static_init_forward(
        c,
        1,
        Callee {
            addr: 0x002f_9040,
            f: ee::s09::f_002f9040,
        },
        0x002f_982c,
    );
}

/// `AttachedEmitterTypeData_GlobalCtors2` (0x2fae18): `AttachedEmitterTypeData_StaticInit2(1, 0xffff)`.
pub fn attached_emitter_type_data_global_ctors2(c: &mut Cpu) {
    static_init_forward(
        c,
        1,
        Callee {
            addr: 0x002f_a640,
            f: ee::s10::f_002fa640,
        },
        0x002f_ae2c,
    );
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc789_install_fakes;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;

    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::leaf;

    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;

    pub(crate) use crate::game::bx::bxstring::tests::alloc_ready;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::pool8;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::fepopup::tests::f_2;
    pub(crate) use crate::game::input::inputmap::tests::put_f32;
    pub(crate) use crate::game::intersect::worldsphtree::tests::forward_setup;

    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::render::particle::tests::base;
    pub(crate) use crate::game::render::particle::tests::put_vec3;
    pub(crate) use crate::game::render::particle::tests::uf;
    pub(crate) use crate::game::render::particle::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::ui::uiengine::tests::POOL0;
    pub(crate) use crate::game::world::worldcache::tests::check_all;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;

    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::w;

    use ssx3_ee as ee;

    #[test]
    fn rider_base_get_main_state_name_matches() {
        check(
            ssx3_ee::s00::f_00111830,
            rider_base_get_main_state_name,
            200,
            &|c, seed| {
                let mut r = Rng(0x1118_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let state = match seed % 5 {
                    0 => 0,
                    1 => 6,
                    2 => 7,
                    3 => 0xffff_ffff,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + 0xde0, state);
                for i in 0..7 {
                    c.bus.write32(0x0045_9d70 + i * 4, r.next_u32());
                }
                c.bus.write32(0x0045_6c48, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_base_get_sub_state_name_matches() {
        check(
            ssx3_ee::s00::f_00111860,
            rider_base_get_sub_state_name,
            200,
            &|c, seed| {
                let mut r = Rng(0x1118_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let state = match seed % 5 {
                    0 => 0,
                    1 => 14,
                    2 => 15,
                    3 => 0xffff_ffff,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + 0xde4, state);
                for i in 0..15 {
                    c.bus.write32(0x0045_9dc8 + i * 4, r.next_u32());
                }
                c.bus.write32(0x0045_6c48, r.next_u32());
            },
        );
    }

    #[test]
    fn c_ps2_fog_particle_man_base_ctor_matches() {
        check(
            ssx3_ee::s09::f_002dbf80,
            c_ps2_fog_particle_man_base_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x1005 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_base_clear_cheat_code_events_matches() {
        check(
            ssx3_ee::s09::f_002f6b28,
            rider_base_clear_cheat_code_events,
            200,
            &|c, seed| {
                let mut r = Rng(0x1013 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write8(DATA + 7, (r.next_u32() & 0xff) as u8);
            },
        );
    }

    #[test]
    fn rider_base_get_cheat_code_events_matches() {
        check(
            ssx3_ee::s09::f_002f6b38,
            rider_base_get_cheat_code_events,
            200,
            &|c, seed| {
                let mut r = Rng(0x1014 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write8(DATA + 7, (r.next_u32() & 0xff) as u8);
            },
        );
    }

    #[test]
    fn aivisual_effects_man_dtor_matches() {
        check(
            ee::s09::f_002f8340,
            aivisual_effects_man_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8340, 0);
                alloc_setup(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    pub(crate) fn f(x: f32) -> u32 {
        x.to_bits()
    }

    pub(crate) fn pick<T: Copy>(r: &mut Rng, xs: &[T]) -> T {
        xs[r.next_u32() as usize % xs.len()]
    }

    // ------------------------------------------------------------ lights

    #[test]
    fn world_light_man_light_contribution_matches() {
        check(
            ee::s09::f_002f5d30__generated,
            world_light_man_light_contribution,
            500,
            &|c, seed| {
                let mut r = Rng(0x0119_4700 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                put_words(c, light::INNER_TABLE_AT, &light::INNER_TABLE);
                put_words(c, light::PENUMBRA_TABLE_AT, &light::PENUMBRA_TABLE);
                // range threshold, attenuation start and slope (both light types)
                for (at, v) in [
                    (0x38b0, 1.0f32),
                    (0x38ac, 2.0),
                    (0x38a8, 0.1),
                    (0x38a4, 1.0),
                    (0x38a0, 2.0),
                    (0x389c, 0.1),
                ] {
                    c.bus.write32(GAME_GP - at, f(v));
                }
                let (point, l) = (DATA, DATA + 0x100);
                fill(c, &mut r, l, 0x20);
                c.bus.write32(
                    l + 0x10,
                    pick(&mut r, &[0u32, 1, 1, 1, 1, 2, 2, 3, 0xffff_ffff]),
                );
                c.bus.write32(l + 0x14, r.f32(2.0));
                c.bus.write32(l + 0x18, r.f32(2.0));
                // ranges up to 1000: distances above 100 make the falloff term
                // (100 / d) drop below 1
                c.bus.write32(
                    l + 0x1c,
                    f(pick(&mut r, &[0.5f32, 5.0, 100.0, 40.0, 1000.0, 1000.0])),
                );
                // the light at the origin pointing along -z or +z; the point
                // on the z axis or off it
                let dir = pick(&mut r, &[-1.0f32, 1.0]);
                for (i, v) in [0.0f32, 0.0, dir].iter().enumerate() {
                    c.bus.write32(l + 0x2c + 4 * i as u32, f(*v));
                    c.bus.write32(l + 0x38 + 4 * i as u32, 0);
                }
                let dist = pick(
                    &mut r,
                    &[0.0f32, 0.5, 1.5, 3.0, 10.0, 60.0, 150.0, 400.0, 250.0],
                );
                let side = pick(&mut r, &[0.0f32, 0.0, 0.3, 1.0, 5.0]);
                c.bus.write32(point, f(side));
                c.bus.write32(point + 4, 0);
                c.bus
                    .write32(point + 8, f(dist * pick(&mut r, &[1.0f32, -1.0])));
                c.bus
                    .write32(l + 0x5c, f(pick(&mut r, &[0.95f32, 0.5, 0.0, -0.2])));
                c.bus
                    .write32(l + 0x60, f(pick(&mut r, &[0.7f32, 0.2, -1.0, 0.99])));
                c.bus
                    .write8(l + 0x64, pick(&mut r, &[0xffu8, 0, 1, 2, 3, 4]));
                c.bus.write8(
                    l + 0x65,
                    pick(&mut r, &[0xfdu8, 0xff, 0, 1, 2, 3, 4, 5, 7, 12, 0x80]),
                );
                c.g[A0][0] = sx(DATA + 0x200);
                c.g[A1][0] = sx(point);
                c.g[A2][0] = sx(l);
                c.pc = light::ENTRY;
                if seed % 6 == 5 {
                    // re-entry at a `^exp` loop head after an unwind
                    c.pc = if seed % 4 == 1 {
                        light::POWI_INNER
                    } else {
                        light::POWI_PENUMBRA
                    };
                    c.g[SP][0] = sx(crate::difftest::SP - 0x30);
                    c.g[S0][0] = sx(l);
                    c.g[V1][0] = (1 + r.next_u32() % 40) as u64;
                    for i in 0..7 {
                        c.f[i] = r.f32(1.5);
                    }
                }
            },
        );
    }

    #[test]
    fn world_light_man_light_distance_matches() {
        check(
            ee::s09::f_002f6168__generated,
            world_light_man_light_distance,
            SEEDS_2,
            &|c, seed| {
                let mut r = base(c, seed, 0x2f61);
                let (pos, light, out) = (DATA, DATA + 0x100, DATA + 0x200);
                let spread = [0.0f32, 40.0, 3.0, 8.0][(seed % 4) as usize];
                for k in 0..3 {
                    let l = uf(&mut r, -50.0, 50.0);
                    c.bus.write32(light + 0x38 + 4 * k, f(l));
                    c.bus
                        .write32(pos + 4 * k, f(l + uf(&mut r, -spread, spread)));
                }
                c.bus.write32(light + 0x1c, f(uf(&mut r, 2.0, 20.0)));
                put_vec3(c, &mut r, light + 0x2c, -1.0, 1.0);
                c.g[A1][0] = sx(pos);
                c.g[A2][0] = sx(light);
                c.g[A3][0] = sx(out);
                c.g[T0][0] = sx(out + 4);
                c.g[T1][0] = if seed % 3 == 0 { 0 } else { sx(out + 8) };
            },
        );
    }

    #[test]
    fn world_light_man_find_strongest_lights_matches() {
        check(
            ssx3_ee::s09::f_002f5b68__generated,
            world_light_man_find_strongest_lights,
            300,
            &|c, seed| {
                let mut r = Rng(0x2f5b_68 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let mgr = DATA;
                let out = DATA + 0x800;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = mgr as u64;
                c.g[A3][0] = out as u64;
                let max = 1 + seed % 5;
                c.g[T0][0] = max as u64;
                let count = seed % 5;
                c.bus.write32(mgr + 0x210, count);
                for i in 0..4 {
                    let light = DATA + 0x1000 + 0x100 * i as u32;
                    c.bus.write32(mgr + 0x214 + 4 * i, light);
                    c.bus.write32(
                        light + 8,
                        if i < count / 2 + (seed % 2) {
                            6
                        } else {
                            r.next_u32() % 8
                        },
                    );
                    c.bus.write32(light + 0x10, 0);
                    put_f32(c, light + 0x18, r.f32(2.0));
                }
            },
        );
    }

    #[test]
    fn ps2_visual_effects_man_is_handle_null_matches() {
        check(
            ssx3_ee::s09::f_002f7be0__generated,
            ps2_visual_effects_man_is_handle_null,
            200,
            &|c, seed| {
                let mut r = Rng(0x2f7b_e001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let v = if seed % 3 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(DATA, v);
                c.pc = 0x002f_7be0;
            },
        );
    }

    /// A deleting-destructor case: a random object pointer, both values of the
    /// delete flag, and a real `operator delete` (with the allocator mutex
    /// taken) that may unwind at the fake's call index.
    pub(crate) fn dtor_case(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            alloc_setup(c, &mut r);
            c.g[A0][0] = ptr(&mut r) as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn vfx_component_dtor_matches() {
        dtor_case(ee::s09::f_002f7bf0__generated, vfx_component_dtor, 0x2f7bf0);
    }

    #[test]
    fn vfx_power_up_fx_toggles_dtor_matches() {
        dtor_case(
            ee::s09::f_002f7f00__generated,
            vfx_power_up_fx_toggles_dtor,
            0x2f7f00,
        );
    }

    #[test]
    fn c_ps2_fog_particle_man_base_dtor_matches() {
        dtor_case(
            ee::s09::f_002f7f30__generated,
            c_ps2_fog_particle_man_base_dtor,
            0x2f7f30,
        );
    }

    #[test]
    fn c_ps2_fog_particle_man_dtor_matches() {
        dtor_case(
            ee::s09::f_002f7f60__generated,
            c_ps2_fog_particle_man_dtor,
            0x2f7f60,
        );
    }

    #[test]
    fn vfx_splash_toggles_dtor_matches() {
        dtor_case(
            ee::s09::f_002f7f90__generated,
            vfx_splash_toggles_dtor,
            0x2f7f90,
        );
    }

    #[test]
    fn transition_effect_dtor_matches() {
        dtor_case(
            ee::s09::f_002f8000__generated,
            transition_effect_dtor,
            0x2f8000,
        );
    }

    #[test]
    fn transition_effect_fade_dtor_matches() {
        dtor_case(
            ee::s09::f_002f8030__generated,
            transition_effect_fade_dtor,
            0x2f8030,
        );
    }

    #[test]
    fn transition_wipe_dtor_matches() {
        dtor_case(
            ee::s09::f_002f80d8__generated,
            transition_wipe_dtor,
            0x2f80d8,
        );
    }

    #[test]
    fn transition_effect_random_dtor_matches() {
        dtor_case(
            ee::s09::f_002f8108__generated,
            transition_effect_random_dtor,
            0x2f8108,
        );
    }

    #[test]
    fn vfx_snow_fall_toggles_dtor_matches() {
        dtor_case(
            ee::s09::f_002f8138__generated,
            vfx_snow_fall_toggles_dtor,
            0x2f8138,
        );
    }

    #[test]
    fn vfx_board_trail_dtor_matches() {
        dtor_case(
            ee::s09::f_002f8188__generated,
            vfx_board_trail_dtor,
            0x2f8188,
        );
    }

    #[test]
    fn vfx_light_glow_dtor_matches() {
        dtor_case(
            ee::s09::f_002f81f0__generated,
            vfx_light_glow_dtor,
            0x2f81f0,
        );
    }

    #[test]
    fn vfx_depth_fog_dtor_matches() {
        dtor_case(ee::s09::f_002f8220__generated, vfx_depth_fog_dtor, 0x2f8220);
    }

    #[test]
    fn vfx_height_fog_dtor_matches() {
        dtor_case(
            ee::s09::f_002f8250__generated,
            vfx_height_fog_dtor,
            0x2f8250,
        );
    }

    #[test]
    fn vfx_shadow_dtor_matches() {
        dtor_case(ee::s09::f_002f8280__generated, vfx_shadow_dtor, 0x2f8280);
    }

    #[test]
    fn vfx_snow_surface_dtor_matches() {
        dtor_case(
            ee::s09::f_002f82b0__generated,
            vfx_snow_surface_dtor,
            0x2f82b0,
        );
    }

    #[test]
    fn vfx_flags_dtor_matches() {
        dtor_case(ee::s09::f_002f82e0__generated, vfx_flags_dtor, 0x2f82e0);
    }

    #[test]
    fn visual_effects_man_dtor_matches() {
        dtor_case(
            ee::s09::f_002f8310__generated,
            visual_effects_man_dtor,
            0x2f8310,
        );
    }

    #[test]
    fn world_light_man_reset_regions_matches() {
        check(
            ee::s09::f_002f5998__generated,
            world_light_man_reset_regions,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f59_98, 0);
                let man = DATA + 0x100;
                c.g[A0][0] = sx(man);
                fill(c, &mut r, man, 0x240);
            },
        );
    }

    #[test]
    fn rider_base_load_cheat_code_matches() {
        check(
            ee::s09::f_002f6a90__generated,
            rider_base_load_cheat_code,
            120,
            &|c, seed| {
                forward_setup(c, seed, 0x2f6a_90, 0x10);
            },
        );
    }

    #[test]
    fn world_light_man_free_overlap_lists_matches() {
        check(
            ee::s09::f_002f53b0__generated,
            world_light_man_free_overlap_lists,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x2f53_b0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x300);
                for i in 0..8u32 {
                    let slot = this + 0x40 + 0x40 * i;
                    c.bus.write32(
                        slot,
                        if seed % 3 == 0 {
                            0
                        } else {
                            DATA + 0x1_4000 + 0x10 * i
                        },
                    );
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------------------------- leaves

    #[test]
    fn rider_base_render_cheat_code_matches() {
        leaf(
            ssx3_ee::s09::f_002f6908__generated,
            rider_base_render_cheat_code,
            1,
        );
    }

    #[test]
    fn board_spark_emitter_ctor_matches() {
        leaf(
            ssx3_ee::s09::f_002f7a68__generated,
            board_spark_emitter_ctor,
            21,
        );
    }

    // ----------------------------------------------------------- world light

    #[test]
    fn world_light_man_evict_oldest_instance_matches() {
        check(
            ssx3_ee::s09::f_002f59d0__generated,
            world_light_man_evict_oldest_instance,
            24,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x002f_59d0);
                let man = DATA + 0x100;
                let scratch = DATA + 0x9000;
                clear_2(c, man + 0x210, 0x20 * 0x180);
                clear_2(c, scratch, 0x400);
                c.g[A0][0] = sx(man);
                c.g[A1][0] = (seed % 64) as u64;
                for i in 0..32u32 {
                    let node = man + 0x210 + 0x180 * i;
                    w(c, node, r.next_u32());
                    w(
                        c,
                        node + 4,
                        if seed % 4 == 0 {
                            0x1000
                        } else {
                            r.next_u32() % 64
                        },
                    );
                    let prev = match seed % 3 {
                        0 => 0,
                        1 => scratch,
                        _ => node.wrapping_sub(0x180),
                    };
                    let next = if seed % 2 == 0 { 0 } else { scratch + 4 };
                    w(c, node + 0x170, prev);
                    w(c, node + 0x174, next);
                }
            },
        );
    }

    #[test]
    fn avalanche_destroy_all_matches() {
        strict(
            ee::s09::f_002d9b40__generated,
            avalanche_destroy_all,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d9b, 10);
                pool8(c, POOL0);
                for k in 0..16 {
                    c.bus.write32(DATA + 0x1c * k, (r.next_u32() % 2) as u32);
                }
                c.bus.write32(
                    GAME_GP + 0x9c0,
                    if seed % 3 == 0 { 0 } else { POOL0 + 0x600 },
                );
                c.bus.write32(POOL0 + 0x600 + 4, POOL0 + 0x100);
                c.bus.write32(POOL0 + 0x100 + 0xf4, 0);
                c.bus.write32(
                    GAME_GP + 0x9c4,
                    if seed % 2 == 0 { 0 } else { POOL0 + 0x200 },
                );
                c.bus
                    .write32(GAME_GP + 0x9c8, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(GAME_GP + 0x9cc, r.next_u32());
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn c_board_spark_fx_reset_matches() {
        same(
            ssx3_ee::s09::f_002daa78__generated,
            c_board_spark_fx_reset,
            200,
            &|c, seed| {
                let mut r = Rng(0x2daa78 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let d = DATA + 0x100;
                c.g[A0][0] = d as u64;
                for i in 0..0x28u32 / 4 {
                    c.bus.write32(d + 0x80 + 4 * i, r.next_u32());
                }
                put_words(
                    c,
                    0x004f_f130,
                    &[r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                put_words(
                    c,
                    0x0050_1420,
                    &[r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                let texman = DATA + 0x600;
                c.bus.write32(GAME_GP - 0x854, texman);
                c.bus.write32(texman + 0xfa8, r.next_u32());
            },
        );
    }

    #[test]
    fn c_board_trail_fx_reset_matches() {
        check_all(
            ee::s09::f_002e8560__generated,
            c_board_trail_fx_reset,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e8560, 0);
                let self_ = DATA + 0x1000;
                c.g[A0][0] = sx(self_);
                for off in [0x10u32, 0x14, 0x18, 0x1c, 0x20, 0x24] {
                    w(c, self_ + off, 0x800 + (r.next_u32() & 0x3f0));
                }
                f_2(c, 0x004f_f120, r.next_u32());
                f_2(c, 0x004f_f124, r.next_u32());
                f_2(c, 0x004f_f128, r.next_u32());
                f_2(c, 0x004f_f12c, r.next_u32());
                f_2(c, 0x004f_f130, r.next_u32());
                f_2(c, 0x004f_f134, r.next_u32());
                for off in [0x14c0u32, 0x14c4, 0x14c8] {
                    f_2(c, crate::difftest::GAME_GP + off, r.edge_f32());
                }
            },
        );
    }

    // ---------------------------------------- RiderBase_UpdateCheatCode

    /// `RiderBase_UpdateCheatCode` against the generated body. The physics
    /// sub-object's query is either a real `return 0` (the update runs) or
    /// a fake (non-zero: nothing to do, or an unwind); the clear, the match
    /// and `BELibrary_GetInterface` run for real (the interface's method is
    /// a fake). The state byte covers on/off/wrong; the rider covers cheat
    /// mode or not, the assist sign, speeds below/above the maximum (and
    /// zero), zero steer, short steer directions and short velocities. The
    /// final `x < 0 && x >= 0` reset is unreachable without NaNs.
    #[test]
    fn rider_base_update_cheat_code_matches() {
        use crate::difftest::fakes::{self, FAKE};
        use crate::difftest::{put_vec, DATA, GAME_GP};
        use crate::game::visualfx::boardwakefx::tests::{f, gp_at, RETURN_ZERO};

        const SELF: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const PHYS_VT: u32 = DATA + 0x2000;
        const LIB: u32 = DATA + 0x2100;
        const LIB_VT: u32 = DATA + 0x2200;
        const WORLD: u32 = DATA + 0x2300;
        /// `g_beLibrary[]` and its index (gp-0x1efc).
        const BE_LIBRARY: u32 = 0x0053_08a8;

        fakes::covered(
            ee::s09::f_002f6518__generated,
            rider_base_update_cheat_code,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2f_6518, 2);

                // .lit4 minimum lengths, the clear counter, the world, the
                // global cheats-off switch.
                c.bus.write32(gp_at(-0x3898), f(0.001));
                c.bus.write32(gp_at(-0x3894), f(0.0001));
                let due = [0u32, 1, 2, 3, 8][(seed % 5) as usize];
                c.bus.write32(gp_at(0xad0), due);
                c.bus.write32(gp_at(-0x848), WORLD);
                c.bus.write32(WORLD + 0x78, r.next_u32());
                c.bus
                    .write32(cheat_callee::CHEATS_OFF, (seed % 7 == 0) as u32);

                // BELibrary_GetInterface: library 0, its method a fake.
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(BE_LIBRARY, LIB);
                c.bus.write32(LIB + 4, LIB_VT);
                c.bus.write16(LIB_VT + 8, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(LIB_VT + 0xc, FAKE + 0x10);

                // The cheat state: on, off or wrong; flags without events
                // mostly (the match then leaves the state alone).
                c.bus.write32(SELF + cheat::RIDER, RIDER);
                let state = [1u8, 1, 1, 0, 0xff, 2][(r.next_u32() % 6) as usize];
                c.bus.write8(SELF + cheat::STATE, state);
                c.bus.write8(SELF + 5, (r.next_u32() % 9) as u8);
                c.bus.write8(SELF + 6, (r.next_u32() % 4) as u8);
                let mut flags = r.next_u32() as u8;
                if seed % 3 != 0 {
                    flags &= 0x8f;
                }
                c.bus.write8(SELF + cheat::FLAGS, flags);

                // The rider: the physics query (0 from a real function,
                // or a fake), then the cheat-mode fields.
                for k in 0..0x80 {
                    c.bus.write32(RIDER + 0x1e0 + 4 * k, r.edge_f32());
                }
                c.bus.write32(RIDER + cheat_rider::PHYSICS, PHYS_VT);
                c.bus.write16(PHYS_VT + 0x48, (r.next_u32() & 0x7c) as u16);
                let query = if seed % 6 == 5 { FAKE } else { RETURN_ZERO };
                c.bus.write32(PHYS_VT + 0x4c, query);
                let mode = if seed % 8 == 3 { r.next_u32() % 12 } else { 11 };
                c.bus.write32(RIDER + cheat_rider::MODE, mode);
                let assist = match seed % 9 {
                    0 => f(-1.0),
                    1 => 0,
                    2 => r.edge_f32(),
                    _ => f(1.0),
                };
                c.bus.write32(RIDER + cheat_rider::ASSIST, assist);
                let abs = |x: u32| f(f32::from_bits(x).abs());
                let vel = match (seed / 2) % 6 {
                    0 => [0, 0, 0, r.next_u32()],
                    1 => [f(0.00002), f(-0.00003), f(0.00001), r.next_u32()],
                    2 => [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    _ => [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)],
                };
                put_vec(c, RIDER + cheat_rider::VELOCITY, vel);
                let max = match (seed / 3) % 4 {
                    0 => r.edge_f32(),
                    1 => 0,
                    _ => abs(r.f32(250.0)),
                };
                c.bus.write32(RIDER + cheat_rider::MAX_SPEED, max);
                c.bus.write32(RIDER + cheat_rider::BLEND, r.f32(1.0));
                let steer = match seed % 7 {
                    1 => 0,
                    2 => r.edge_f32(),
                    _ => r.f32(1.0),
                };
                c.bus.write32(RIDER + cheat_rider::STEER, steer);
                let dir = match (seed / 4) % 5 {
                    0 => [f(0.0002), 0, f(0.0001), r.next_u32()],
                    1 => [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    _ => [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(1.0)],
                };
                put_vec(c, RIDER + cheat_rider::STEER_DIR, dir);

                c.g[A0][0] = sx(SELF);
            },
        );
    }

    /// The cheat assist's speed blend (+0x314) and steer share (+0x310) are
    /// per-tick smoothing: one update at 60 and `rate/60` updates at 120 /
    /// 240 Hz bring the velocity to (nearly) the same speed and heading. The
    /// cheat stays on (state 1, no events), the velocity starts at 100 along
    /// x, the steer direction is (0.6, 0, 0.8), the maximum speed 200.
    /// Unconverted, 120 Hz would turn about twice as far (0.29 vs 0.154 rad).
    #[test]
    fn rider_base_update_cheat_code_follows_the_rate() {
        use crate::difftest::{fakes, put_vec, DATA, GAME_GP};
        use crate::game::visualfx::boardwakefx::tests::{f, gp_at, RETURN_ZERO};
        const SELF: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const PHYS_VT: u32 = DATA + 0x2000;
        const LIB: u32 = DATA + 0x2100;
        const LIB_VT: u32 = DATA + 0x2200;
        const WORLD: u32 = DATA + 0x2300;
        const BE_LIBRARY: u32 = 0x0053_08a8;

        fn after(hz: u32) -> (f32, f32) {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            fakes::machine(c, 1, 0x2f_6519, 0);
            crate::tick::set_for_test(c, hz);
            c.bus.write32(gp_at(-0x3898), f(0.001));
            c.bus.write32(gp_at(-0x3894), f(0.0001));
            c.bus.write32(gp_at(0xad0), 0);
            c.bus.write32(gp_at(-0x848), WORLD);
            c.bus.write32(WORLD + 0x78, 0);
            c.bus.write32(cheat_callee::CHEATS_OFF, 0);
            c.bus.write32(GAME_GP - 0x1efc, 0);
            c.bus.write32(BE_LIBRARY, LIB);
            c.bus.write32(LIB + 4, LIB_VT);
            c.bus.write16(LIB_VT + 8, 0);
            c.bus.write32(LIB_VT + 0xc, fakes::FAKE + 0x10);
            c.bus.write32(SELF + cheat::RIDER, RIDER);
            c.bus.write8(SELF + cheat::STATE, 1);
            c.bus.write8(SELF + 5, 0);
            c.bus.write8(SELF + 6, 0);
            c.bus.write8(SELF + cheat::FLAGS, 0);
            for k in 0..0x80 {
                c.bus.write32(RIDER + 0x1e0 + 4 * k, 0);
            }
            c.bus.write32(RIDER + cheat_rider::PHYSICS, PHYS_VT);
            c.bus.write16(PHYS_VT + 0x48, 0);
            c.bus.write32(PHYS_VT + 0x4c, RETURN_ZERO);
            c.bus.write32(RIDER + cheat_rider::ASSIST, f(1.0));
            put_vec(c, RIDER + cheat_rider::VELOCITY, [f(100.0), 0, 0, 0]);
            c.bus.write32(RIDER + cheat_rider::MAX_SPEED, f(200.0));
            c.bus.write32(RIDER + cheat_rider::BLEND, f(0.015));
            c.bus.write32(RIDER + cheat_rider::STEER, f(0.18));
            put_vec(c, RIDER + cheat_rider::STEER_DIR, [f(0.6), 0, f(0.8), 0]);
            for _ in 0..hz / 60 {
                c.g[A0][0] = SELF as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.g[SP][0] = crate::difftest::SP as u64;
                rider_base_update_cheat_code(c);
                assert_eq!(c.pc, crate::difftest::RA, "{hz} Hz: unwound");
                assert_eq!(c.bus.read32(RIDER + cheat_rider::MODE), 11, "{hz} Hz");
            }
            let x = f32::from_bits(c.bus.read32(RIDER + cheat_rider::VELOCITY));
            let z = f32::from_bits(c.bus.read32(RIDER + cheat_rider::VELOCITY + 8));
            ((x * x + z * z).sqrt(), z.atan2(x))
        }
        let (speed, heading) = after(60);
        assert!((speed - 95.319).abs() < 0.01, "{speed}");
        assert!((heading - 0.15394).abs() < 1e-3, "{heading}");
        for hz in [120, 240] {
            let (s, h) = after(hz);
            assert!((s - speed).abs() < 0.5, "{hz} Hz: speed {s} vs {speed}");
            assert!(
                (h - heading).abs() < 0.005,
                "{hz} Hz: heading {h} vs {heading}"
            );
        }
    }

    /// `RiderBase_UpdateAttackFX`: the effect at DATA, its rider with an
    /// attack strength that is 0, -0, negative or denormal on one seed in
    /// six (the effect switches off at once) and else 0.05..2; the rider's
    /// anim controller (real callees) with track 0 in {punch A, punch B,
    /// punch A1, other} and track 1 in {A1, B1, 0x1b6 (no table entry),
    /// other}, the anim table type of those states 0xd on two seeds in
    /// three, the track-1 sequence's 64-bit flags random (bits 0 and 1 pick
    /// the flagged track-1 punches); the facing flag set or clear, bones
    /// 0..3 into four random matrices, `D_004FF120`, the 0.9 colour scale
    /// and a random seed. Covers both hands of both punches, the switched
    /// off fall-through and the four-spark loop.
    #[test]
    fn rider_base_update_attack_fx_matches() {
        use crate::difftest::{check_lohi, fill, scramble, Rng, DATA, GAME_GP, SP};
        use attack_fx::*;
        const FX: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const CTL: u32 = DATA + 0x2000;
        const SEQS: u32 = DATA + 0x2100;
        const SEQ: u32 = DATA + 0x2200;
        const MODEL: u32 = DATA + 0x3000;
        const BONES: u32 = DATA + 0x3100;
        /// `AnimTable_GetEntry`'s table (0x1c bytes per state).
        const ANIM_TABLE: u32 = 0x0044_6990;
        check_lohi(
            ee::s09::f_002f1150__generated,
            rider_base_update_attack_fx,
            300,
            &|c, seed| {
                let mut r = Rng(0x2f11_5000_1234 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                fill(c, &mut r, SP - 0x100, 0x40);
                fill(c, &mut r, FX, 0x30);
                c.bus.write32(FX + OWNER, RIDER);
                c.g[A0][0] = FX as u64;
                // The rider.
                let strength = if seed % 6 == 0 {
                    [0, 0x8000_0000, 0xbf80_0000, 0x0000_0123][(r.next_u32() % 4) as usize]
                } else {
                    (0.05 + (r.next_u32() % 196) as f32 * 0.01).to_bits()
                };
                c.bus.write32(RIDER + R_STRENGTH, strength);
                c.bus.write32(
                    RIDER + R_FLIP,
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    },
                );
                c.bus.write32(RIDER + R_BONE_A, r.next_u32() % 4);
                c.bus.write32(RIDER + R_BONE_B, r.next_u32() % 4);
                c.bus.write32(RIDER + R_MODEL, MODEL);
                c.bus.write32(MODEL + MODEL_BONES, BONES);
                for i in 0..64 {
                    c.bus.write32(BONES + 4 * i, r.f32(10.0));
                }
                for i in 0..4 {
                    c.bus.write32(ORIGIN_ROW + 4 * i, r.f32(2.0));
                }
                c.bus.write32(GAME_GP - COLOUR_SCALE, 0.9f32.to_bits());
                c.bus.write32(GAME_GP + RAND_SEED, r.next_u32());
                // The anim controller: track states, the anim table, the
                // track-1 sequence's flags.
                c.bus.write32(RIDER + R_ANIM, CTL);
                let track0 = [PUNCH_A, PUNCH_B, PUNCH_A1, 0x10][(r.next_u32() % 4) as usize];
                let track1 = [PUNCH_A1, PUNCH_B1, 0x1b6, 0x20][(r.next_u32() % 4) as usize];
                c.bus.write32(CTL, track0 as u32);
                c.bus.write32(CTL + 4, track1 as u32);
                for state in [PUNCH_A1, PUNCH_B1, 0x20] {
                    let ty = if r.next_u32() % 3 != 0 {
                        SEQ_TYPE_PUNCH as u32
                    } else {
                        r.next_u32() % 0x20
                    };
                    c.bus.write32(ANIM_TABLE + 0x1c * state as u32, ty);
                }
                c.bus.write32(CTL + 0x50, SEQS);
                c.bus.write32(SEQS + 8 + 4, SEQ);
                let flags = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.bus.write64(SEQ + 0xb0, flags);
            },
        );
    }

    /// `WorldLightMan_GetInstanceLighting`: a light manager with 0..=32
    /// cached nodes in consistent hash chains, overlap regions sharing one
    /// empty cell (so the gather and the point lights take their short
    /// paths), and the world's lookups as fakes (their irradiance
    /// references forced to fixture records). Seeds cover a cached / new
    /// instance (free node or eviction), a plain or modelled instance
    /// (node matrix through fakes), `force` with small / medium / large
    /// moves, the node's own region inside / outside its bounds or a fresh
    /// region, and every irradiance source (override, first or second
    /// reference resolved or by name, default); some fake calls unwind.
    #[test]
    fn world_light_man_get_instance_lighting_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
        const THIS: u32 = 0x0085_0000;
        const CELL: u32 = 0x0087_0000;
        const TREE: u32 = 0x0087_1000;
        const REF1: u32 = 0x0088_0000;
        const REF2: u32 = 0x0088_0100;
        const ENTRIES: u32 = 0x0089_0000;
        const NAMES: u32 = 0x0089_1000;
        const WORLD: u32 = 0x008a_0000;
        const WORLD_VT: u32 = 0x008a_1000;
        const INNER: u32 = 0x008a_2000;
        const INNER_VT: u32 = 0x008a_3000;
        const FRAME_STATE: u32 = 0x008a_4000;
        const OBJ: u32 = 0x008b_0000;
        const MODEL: u32 = 0x008b_1000;
        const MODEL_VT: u32 = 0x008b_2000;
        check(
            ee::s09::f_002f5400__generated,
            world_light_man_get_instance_lighting,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f54, 5);
                elf_image(c);
                c.pc = 0x002f_5400;
                let gp = GAME_GP;
                let fl = |x: f32| x.to_bits();
                let prio = 100 + seed % 50;
                put_words(c, FRAME_STATE + 0x18, &[prio]);
                put_words(c, gp + GP_FRAME_STATE, &[FRAME_STATE]);

                // World lookups: all fakes (delta 0).
                put_words(c, THIS + WLM_WORLD, &[WORLD]);
                put_words(c, WORLD, &[INNER, WORLD_VT]);
                put_words(c, WORLD_VT + 0x10, &[0, FAKE]);
                put_words(c, INNER + 4, &[INNER_VT]);
                put_words(c, INNER_VT + 0x140, &[0, FAKE + 0x10]);
                put_words(c, INNER_VT + 0x128, &[0, FAKE + 0x20]);

                // Irradiance: table of 2 named entries, references.
                put_words(c, IRRADIANCE_TABLE, &[0, 2, ENTRIES, NAMES]);
                fill(c, &mut r, ENTRIES - 0xa0, 3 * 0x28);
                put_words(c, NAMES, &[0x0063_6261, 0, 0x007a_7978, 0]);
                let reference = |c: &mut Cpu, r: &mut Rng, at: u32, k: u32| match k {
                    0 => put_words(c, at, &[0, 0]),
                    1 => put_words(c, at, &[r.next_u32() % 3, IRRADIANCE_RESOLVED as u32]),
                    2 => put_words(c, at, &[0x007a_7978, 0]),
                    _ => put_words(c, at, &[0x0071_7171, 0]),
                };
                let k1 = r.next_u32() % 4;
                let k2 = r.next_u32() % 4;
                reference(c, &mut r, REF1, k1);
                reference(c, &mut r, REF2, k2);
                put_words(
                    c,
                    gp + 0x24c0,
                    &[r.next_u32() % 3, IRRADIANCE_RESOLVED as u32],
                );
                put_words(c, gp + GP_IRRADIANCE_OVERRIDE, &[(seed % 5 == 0) as u32]);
                put_words(c, gp + GP_DEFAULT_IRRADIANCE, &[r.next_u32() % 4]);
                put_words(c, gp - GP_NEG_99999, &[fl(-99999.0)]);

                // The instance.
                let pos = [r.f32(800.0), r.f32(800.0), r.f32(800.0), fl(1.0)];
                let modelled = seed % 3 == 0;
                put_words(c, OBJ + INST_MODEL, &[if modelled { MODEL } else { 0 }]);
                put_words(c, MODEL + 0xc, &[MODEL_VT]);
                put_words(c, MODEL_VT + 0xd0, &[0, FAKE + 0x30]);
                put_words(c, MODEL_VT + 0xe8, &[0, FAKE + 0x40]);
                put_vec(c, OBJ + INST_POS, pos);

                // Overlap regions (some free) sharing the empty cell.
                put_words(c, CELL, &[TREE]);
                for i in 0..8 {
                    let b = THIS + 0x10 + i * 0x40;
                    let id = if r.next_u32() % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32()
                    };
                    put_words(c, b, &[id, r.next_u32() % prio]);
                    put_words(c, b + WLB_CELL, &[CELL]);
                }

                // Cached nodes in consistent chains.
                let count = match seed % 4 {
                    0 => 32,
                    _ => r.next_u32() % 33,
                };
                put_words(c, THIS + WLM_COUNT, &[count]);
                let mut ids = Vec::new();
                for i in 0..count {
                    let n = THIS + WLM_NODES + i * WLM_NODE_SIZE;
                    let id = (r.next_u32() & !0x1f00) | (r.next_u32() % 4) << 8;
                    ids.push(id);
                    put_words(c, n, &[id, r.next_u32() % (prio + 1)]);
                    fill(c, &mut r, n + WLN_CUR, 0x28);
                    fill(c, &mut r, n + WLN_TARGET, 0x28);
                    // Its position: on the instance, moved a little, a lot.
                    let d = [0.0f32, 30.0, 700.0][(r.next_u32() % 3) as usize];
                    let p = [fl(f32::from_bits(pos[0]) + d), pos[1], pos[2], fl(1.0)];
                    put_vec(c, n + WLN_POS, p);
                    // Its region: none, its own (bounds around the instance,
                    // sometimes not containing it), or someone else's.
                    let b = THIS + 0x10 + (r.next_u32() % 8) * 0x40;
                    match r.next_u32() % 4 {
                        0 => put_words(c, n + WLN_BOX, &[0]),
                        1 => {
                            put_words(c, b, &[!id]);
                            put_words(c, n + WLN_BOX, &[b]);
                        }
                        _ => {
                            put_words(c, b, &[id]);
                            put_words(c, n + WLN_BOX, &[b]);
                            let mut lo = [0u32; 4];
                            let mut hi = [0u32; 4];
                            for a in 0..3 {
                                let v = f32::from_bits(pos[a]);
                                let out = r.next_u32() % 8;
                                lo[a] = fl(if out == 0 { v + 5.0 } else { v - 900.0 });
                                hi[a] = fl(if out == 1 { v - 5.0 } else { v + 900.0 });
                            }
                            put_vec(c, b + WLB_MIN, lo);
                            put_vec(c, b + WLB_MAX, hi);
                        }
                    }
                    // Link at the head of its chain.
                    let head = THIS + WLM_HEADS + ((id >> 8) & 0x1f) * 4;
                    let old = c.bus.read32(head);
                    put_words(c, n + WLN_NEXT, &[old, 0]);
                    if old != 0 {
                        put_words(c, old + WLN_PREV, &[n]);
                    }
                    put_words(c, head, &[n]);
                }
                let key = if !ids.is_empty() && seed % 2 == 0 {
                    ids[(r.next_u32() as usize) % ids.len()]
                } else {
                    (r.next_u32() & !0x1f00) | (r.next_u32() % 4) << 8
                };
                put_words(c, OBJ + INST_KEY, &[key]);

                // Fake call results: the references (index after the world
                // query, and the node matrix's two calls when modelled).
                let base = if modelled { 3 } else { 1 };
                put_words(c, LOG_RESULTS + 4 * base, &[REF1, REF2]);

                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(OBJ);
                c.g[A2][0] = (seed % 7 != 0) as u64;
            },
        );
    }

    /// RiderBase_MatchCheatCode: every state (-2, -1, 0, 1, 2), flag bytes
    /// with and without flag 1 and the event bits (0x60 exactly too), counts
    /// that match the code's step or not, the last step.
    #[test]
    fn rider_base_match_cheat_code_matches() {
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        check(
            ee::s09::f_002f6910__generated,
            rider_base_match_cheat_code,
            400,
            &|c, seed| {
                let mut r = Rng(0x2f69_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x002f_6910;
                c.g[GP][0] = GAME_GP as u64;
                let code = [3u8, 1, 4, 1, 5, 9, 2, 6, 5];
                for (i, &n) in code.iter().enumerate() {
                    c.bus.write8(0x0044_5e30 + i as u32, n);
                }
                let this = DATA;
                let state = [0xffu8, 0, 1, 2, 0xfe][(seed % 5) as usize];
                let step = if seed % 3 == 0 {
                    8
                } else {
                    (r.next_u32() % 9) as u8
                };
                let count = if seed % 2 == 0 {
                    code[step as usize]
                } else {
                    (r.next_u32() % 10) as u8
                };
                let flags = match (seed / 5) % 6 {
                    0 => 0x60,
                    1 => 0x60 | (r.next_u32() as u8 & 0x8e),
                    2 => r.next_u32() as u8 & 0x8e,
                    3 => r.next_u32() as u8 & !1,
                    _ => r.next_u32() as u8,
                };
                c.bus.write32(this, r.next_u32());
                c.bus.write8(this + 4, state);
                c.bus.write8(this + 5, step);
                c.bus.write8(this + 6, count);
                c.bus.write8(this + 7, flags);
                c.g[A0][0] = sx(this);
            },
        );
    }

    pub(crate) use crate::game::ai::ai::tests::stream_save_setup;

    /// `RiderBase_SaveCheatCode`.
    #[test]
    fn rider_base_save_cheat_code_matches() {
        covered(
            ee::s09::f_002f6a58__generated,
            rider_base_save_cheat_code,
            200,
            &|c, seed| stream_save_setup(c, seed, 0x002f_6a58),
        );
    }

    use crate::difftest::SP as STACK;

    pub(crate) use crate::game::animation::rideranim::tests::CTRL;
    pub(crate) use crate::game::animation::rideranim::tests::MAGIC;

    pub(crate) use crate::game::animation::rideranim::tests::active;
    pub(crate) use crate::game::animation::rideranim::tests::chain;
    pub(crate) use crate::game::animation::rideranim::tests::fake_ret;
    pub(crate) use crate::game::animation::rideranim::tests::install;

    /// `WorldLightMan_GetOverlapRegion`: all 8 table entries random, with
    /// the state words set so the scan, the free-entry hit, the
    /// priority hit and the "all used" exit are all reached; events off,
    /// delivered or unwinding at the loop head.
    #[test]
    fn world_light_man_get_overlap_region_matches() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s09::f_002f5a70__generated,
            world_light_man_get_overlap_region,
            240,
            &|c, seed| {
                let mut r = Rng(0x2f5a_7000 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                let prio = (r.next_u32() % 0x10) as u64;
                let free_at = (seed / 3) % 10; // 0..7: that entry; 8-9: none
                for i in 0..8u32 {
                    let e = this + WORLD_LIGHT_OVERLAP_REGIONS + WORLD_LIGHT_REGION_STRIDE * i;
                    let state = if i == free_at {
                        u32::MAX
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(e, state);
                    c.bus.write32(e + 4, r.next_u32() % 0x18);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = prio;
                schedule(c, &mut r, seed, 0x002f_5a70)
            },
        );
    }

    // -- slot 1/2: WorldLightMan_FindStrongestLights (0x2f5b68) --------------

    pub(crate) fn fake_find_strongest_lights(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 1, fake_find_strongest_lights);
        }
        c.g[V0][0] = 1;
        c.cycles += 6;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    pub(crate) fn fake_light_apply(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 2, fake_light_apply);
        }
        fake_ret(c, 3);
    }

    #[test]
    fn world_light_man_add_point_lights_matches() {
        install(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002F5B68),
            fake_find_strongest_lights,
            1,
        );
        install(
            std::ptr::addr_of_mut!(ee::s12::OVERRIDE_0038A6A8),
            fake_light_apply,
            2,
        );
        check(
            ee::s09::f_002f5af0__generated,
            world_light_man_add_point_lights,
            200,
            &|c, seed| {
                machine(c, seed, 0x002f_5af0, 2);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                c.g[A3][0] = (DATA + 0x4000) as u64;
                let count: i32 = match seed % 4 {
                    0 => 0,
                    1 => -1,
                    2 => 2,
                    _ => 4,
                };
                c.g[T0][0] = sx(count as u32);
                // The light buffer is the 32-word area at the frame base.
                let buf = STACK - 0xd0;
                for i in 0..32u32 {
                    let v = if (seed + i) % 3 == 0 {
                        0
                    } else {
                        (DATA + 0x5000 + (i << 4)) | 0x1000
                    };
                    c.bus.write32(buf + 4 * i, v);
                }
                c.bus.write32(CTRL, MAGIC);
                c.pc = 0x002f_5af0;
            },
        );
    }

    use crate::game::render::particle::man;

    pub(crate) fn fog_v02_f(v: f32) -> u32 {
        v.to_bits()
    }

    /// `cPS2FogParticleMan_v02` over 0..3 objects with random meshes: box
    /// centres before/inside/after the 18000..25000 fade, vertex depths
    /// around 0, 500 and 2500, empty and missing parts, clip on and off
    /// with corners inside and outside the clip box, and the stale-matrix
    /// path through `PS2GraphicsMan_UpdateViewMatrices`.
    #[test]
    fn ps2_fog_particle_man_v02_matches() {
        check(
            ee::s09::f_002dc190__generated,
            ps2_fog_particle_man_v02,
            300,
            &|c, seed| {
                let mut r = Rng(0x2dc1_9000 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x002d_c190;
                c.g[GP][0] = sx(GAME_GP);
                let gp = GAME_GP;
                for (off, v) in [
                    (FOG_V02_LIT_FAR, 0x46c3_5000u32),
                    (FOG_V02_LIT_FADE_START, 0x468c_a000),
                    (FOG_V02_LIT_FADE_SCALE, 0x3915_cbec),
                    (FOG_V02_LIT_NEAR_END, 0x451c_3aae),
                    (FOG_V02_LIT_NEAR_SCALE, 0x3a03_1803),
                ] {
                    c.bus.write32(gp - off, v);
                }

                // Graphics manager at DATA; its view matrix maps w = z.
                let gfx = DATA;
                c.bus.write32(gp + FOG_V02_GP_GFX, gfx);
                let sx_ = 0.5 + (r.next_u32() % 8) as f32;
                let sy_ = 0.5 + (r.next_u32() % 8) as f32;
                for base in [man::WORLD, FOG_V02_GFX_VIEW] {
                    for i in 0..16u32 {
                        c.bus.write32(gfx + base + 4 * i, 0);
                    }
                    c.bus.write32(gfx + base, fog_v02_f(sx_));
                    c.bus.write32(gfx + base + 0x14, fog_v02_f(sy_));
                    c.bus.write32(gfx + base + 0x28, fog_v02_f(1.0));
                    c.bus.write32(gfx + base + 0x2c, fog_v02_f(1.0));
                }
                c.bus.write32(gfx + FOG_V02_GFX_MIN_X, fog_v02_f(-0.6));
                c.bus.write32(gfx + FOG_V02_GFX_MAX_X, fog_v02_f(0.6));
                c.bus.write32(gfx + FOG_V02_GFX_MIN_Y, fog_v02_f(-0.6));
                c.bus.write32(gfx + FOG_V02_GFX_MAX_Y, fog_v02_f(0.6));
                c.bus.write32(
                    gfx + FOG_V02_GFX_MIN_Z,
                    fog_v02_f(r.next_u32() as f32 % 1500.0),
                );
                c.bus.write32(
                    gfx + FOG_V02_GFX_MAX_Z,
                    fog_v02_f(1000.0 + r.next_u32() as f32 % 2000.0),
                );
                let valid = if seed % 8 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 5
                };
                c.bus.write32(gfx + FOG_V02_GFX_VIEW_VALID, valid);
                // For the stale path: identity camera, an empty defer manager.
                let cam = DATA + 0x7000;
                for i in 0..4u32 {
                    for k in 0..4u32 {
                        c.bus.write32(
                            cam + 0x10 * i + 4 * k,
                            if i == k { fog_v02_f(1.0) } else { 0 },
                        );
                    }
                }
                let defer = DATA + 0x8000;
                let rec = defer + 0x68000;
                c.bus.write32(rec + 0x1cc4, 0);
                c.bus.write32(rec + 0x1cc8, 0);
                c.bus.write32(rec + 0x2ed0, 0);
                c.bus.write32(gfx + man::CAMERA, cam);
                c.bus.write32(gfx + man::DEFER, defer);

                // Objects.
                let objs = DATA + 0x80000;
                let out = DATA + 0x90000;
                let n: i32 = match seed % 10 {
                    0 => 0,
                    1 => -1,
                    _ => 1 + (r.next_u32() % 3) as i32,
                };
                for i in 0..3u32 {
                    let obj = DATA + 0x81000 + i * 0x100;
                    let mesh = DATA + 0x82000 + i * 0x40;
                    let list = mesh + 0x20;
                    let part = DATA + 0x83000 + i * 0x40;
                    let verts = DATA + 0x84000 + i * 0x400;
                    c.bus.write32(objs + 4 * i, obj);
                    // Box centre depth: near, fading or culled.
                    let cz = match r.next_u32() % 5 {
                        0 => 30000.0,
                        1 => 25000.0,
                        2 => 18000.0 + (r.next_u32() % 7000) as f32,
                        _ => (r.next_u32() % 18000) as f32,
                    };
                    let hz = (r.next_u32() % 100) as f32;
                    for (k, (lo, hi)) in [(-3.0f32, 5.0f32), (-2.0, 1.0), (cz - hz, cz + hz)]
                        .into_iter()
                        .enumerate()
                    {
                        c.bus
                            .write32(obj + FOG_V02_OBJ_MIN + 4 * k as u32, fog_v02_f(lo));
                        c.bus
                            .write32(obj + FOG_V02_OBJ_MAX + 4 * k as u32, fog_v02_f(hi));
                    }
                    // Object matrix: identity plus a translation.
                    let tz = (r.next_u32() % 1500) as f32 - 200.0;
                    for row in 0..4u32 {
                        for k in 0..4u32 {
                            let v = if row == k { 1.0 } else { 0.0 };
                            c.bus
                                .write32(obj + FOG_V02_OBJ_MAT + 0x10 * row + 4 * k, fog_v02_f(v));
                        }
                    }
                    c.bus.write32(obj + FOG_V02_OBJ_MAT + 0x38, fog_v02_f(tz));
                    c.bus.write32(obj + FOG_V02_OBJ_MESH, mesh);
                    c.bus.write32(mesh + FOG_V02_MESH_COUNT, r.next_u32() % 4);
                    c.bus.write32(mesh + FOG_V02_MESH_LIST, list);
                    let part_ptr = if r.next_u32() % 6 == 0 { 0 } else { part };
                    c.bus.write32(list + FOG_V02_LIST_PART, part_ptr);
                    let nv = match r.next_u32() % 6 {
                        0 => 0,
                        _ => 1 + r.next_u32() % 6,
                    };
                    c.bus.write32(part + FOG_V02_PART_COUNT, nv);
                    c.bus.write32(part + FOG_V02_PART_VERTS, verts);
                    for k in 0..nv {
                        let v = verts + k * FOG_V02_VERT_SIZE;
                        let z = match r.next_u32() % 6 {
                            0 => -300.0 - tz,
                            1 => 500.0 - tz,
                            2 => 0.5 - tz,
                            _ => (r.next_u32() % 3200) as f32 - tz,
                        };
                        let w = (z + tz).abs().max(1.0);
                        let x = ((r.next_u32() % 2000) as f32 / 1000.0 - 1.0) * w;
                        let y = ((r.next_u32() % 2000) as f32 / 1000.0 - 1.0) * w;
                        c.bus.write32(v, fog_v02_f(x));
                        c.bus.write32(v + 4, fog_v02_f(y));
                        c.bus.write32(v + 8, fog_v02_f(z));
                        for ch in 0..3u32 {
                            c.bus.write32(v + 0xc + 4 * ch, r.f32(1.0));
                        }
                        let size = if r.next_u32() % 4 == 0 {
                            0.0
                        } else {
                            (r.next_u32() % 400) as f32
                        };
                        c.bus.write32(v + 0x18, fog_v02_f(size));
                    }
                }
                c.bus.write32(out, r.next_u32() % 4);

                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(objs);
                c.g[A2][0] = n as i64 as u64;
                c.g[A3][0] = if seed % 3 == 0 {
                    0
                } else {
                    1 + (r.next_u32() % 9) as u64
                };
            },
        );
    }

    #[test]
    fn rider_base_reset_attack_fx_thunk_matches() {
        check(
            ee::s09::f_002f7ba8__generated,
            rider_base_reset_attack_fx_thunk,
            60,
            &|c, seed| {
                let mut r = Rng(0x002f_7ba8 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x002f_7ba8;
                c.g[GP][0] = GAME_GP as u64;
                let _ = seed;
            },
        );
    }

    #[test]
    fn rider_base_reset_power_up_trail_thunk_matches() {
        check(
            ee::s09::f_002f7ed0__generated,
            rider_base_reset_power_up_trail_thunk,
            60,
            &|c, seed| {
                let mut r = Rng(0x002f_7ed0 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x002f_7ed0;
                c.g[GP][0] = GAME_GP as u64;
                let _ = seed;
            },
        );
    }

    use crate::difftest::fakes;

    pub(crate) use crate::game::sound::bankmonitor::tests::oc789_arm;

    pub(crate) use crate::game::visualfx::boardwakefx::tests::rider_wrapper_fixture;

    /// `RiderBase_CtorCheatCode` against the generated body: with and
    /// without `inChrg` (rider ctor call; stack copies of both vtable
    /// templates), then `RiderBase_ClearCheatCode` (a real callee over a
    /// faked memset); unwinds in each call.
    #[test]
    fn rider_base_ctor_cheat_code_matches() {
        check(
            ee::s09::f_002f6238__generated,
            rider_base_ctor_cheat_code,
            80,
            &|c, seed| rider_wrapper_fixture(c, seed, 0x2f6238, 0x002f_6238, [0x4886e0, 0x488728]),
        );
    }

    /// `RiderBase_ResetCheatCode`: an empty function.
    #[test]
    fn rider_base_reset_cheat_code_matches() {
        check(
            ee::s09::f_002f64e8__generated,
            rider_base_reset_cheat_code,
            4,
            &|c, _| {
                c.pc = 0x002f_64e8;
            },
        );
    }

    /// `RiderBase_ClearCheatCode`: one `memset(self + 4, 0, 4)` call, unwinding or not.
    #[test]
    fn rider_base_clear_cheat_code_matches() {
        oc789_install_fakes();
        fakes::covered(
            ee::s09::f_002f64f0__generated,
            rider_base_clear_cheat_code,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x789c, 1);
                c.pc = 0x002f_64f0;
                oc789_arm(c);
                c.g[A0][0] = sx(DATA + 0x1_0000 + (r.next_u32() & 0xfff0));
            },
        );
    }

    use crate::difftest;
    use crate::difftest::fresh;
    use std::sync::atomic::AtomicUsize;

    /// Base of the type table the initialisers fill (`*(gp+0xac0)`).
    pub(crate) const TYPE_TABLE: u32 = DATA + 0x4000;

    /// Both initialisers write constants into the type table through the
    /// pointer at `gp+0xac0`; the seeds scramble every register and put junk
    /// in the table and the stack so every store is visible.
    pub(crate) fn init_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0xa77a_c4ed ^ (seed as u64) << 12);
        difftest::scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP + 0xac0, TYPE_TABLE);
        difftest::fill(c, &mut r, TYPE_TABLE, 0x580);
        difftest::fill(c, &mut r, difftest::SP - 0x100, 0x40);
    }

    #[test]
    fn attached_emitter_type_data_init_part1_matches() {
        difftest::check(
            ee::s09::f_002f83b0__generated,
            attached_emitter_type_data_init_part1,
            8,
            &init_setup,
        );
    }

    #[test]
    fn attached_emitter_type_data_init_part2_matches() {
        difftest::check(
            ee::s09::f_002f9838__generated,
            attached_emitter_type_data_init_part2,
            8,
            &init_setup,
        );
    }

    // ---- WScriptMan_LoadState -------------------------------------------

    pub(crate) const LS_ENTRY: u32 = 0x0030_bd20;

    pub(crate) const LS_SELF: u32 = DATA + 0x2000;

    pub(crate) const LS_STREAM: u32 = DATA + 0x6000;

    pub(crate) const LS_VTABLE: u32 = DATA + 0x7000;

    pub(crate) const LS_WORLD: u32 = DATA + 0x8000;

    pub(crate) const LS_TABLES: u32 = DATA + 0x4_0000;

    pub(crate) const LS_NODES: u32 = DATA + 0x5_0000;

    /// Gate for the fakes (in the machine's RAM): only the tests that write
    /// the magic word see them.
    pub(crate) const LS_CTRL: u32 = DATA + 0x2f_fff0;

    pub(crate) const LS_MAGIC: u32 = 0x5c21_9a10;

    /// What the stream's `read` hands out: the `count` word and the table id.
    pub(crate) const LS_COUNT_WORD: u32 = DATA + 0x2f_ffe0;

    pub(crate) const LS_ID_WORD: u32 = DATA + 0x2f_ffe4;

    /// Remaining nodes of each list, keyed by the list address.
    pub(crate) const LS_POPS: u32 = DATA + 0x30_0000;

    pub(crate) fn ls_pops(list: u32) -> u32 {
        LS_POPS + (list & 0xf_fffc)
    }

    /// The gated callee fake: logs like `fakes::fake_call`; `PopFront` hands out
    /// the list's remaining nodes (a counter in RAM), then 0.
    pub(crate) fn oc810_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0] as u32;
        let ra = c.g[RA][0] as u32;
        let pops = addr == 0x0030_b208;
        let n = if pops { c.bus.read32(ls_pops(a0)) } else { 0 };
        fakes::fake_call(c, addr);
        if pops && c.pc == ra {
            if n > 0 {
                c.bus.write32(ls_pops(a0), n - 1);
                c.g[V0][0] = (LS_NODES + 0x10 * n) as u64;
            } else {
                c.g[V0][0] = 0;
            }
        }
    }

    /// The stream's `read(buf, len)`: a 4-byte read to `sp` delivers the
    /// count, to `sp+4` the table id; anything else only logs.
    pub(crate) fn oc810_read_miss(c: &mut Cpu, target: u32) {
        if target == fakes::FAKE && c.g[A2][0] as u32 == 4 {
            let dest = c.g[A1][0] as u32;
            let v = c.bus.read32(if dest & 4 == 0 {
                LS_COUNT_WORD
            } else {
                LS_ID_WORD
            });
            c.bus.write32(dest, v);
        }
        fakes::fake_call(c, target)
    }

    macro_rules! oc810_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(LS_CTRL) != LS_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc810_fake_call(c, $addr);
                }
            )*
            fn oc810_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc810_fakes!(
        (
            oc810_fake_pop_front,
            OC810_PREV_POP_FRONT,
            ee::s10::OVERRIDE_0030B208,
            ee::s10::f_0030b208__generated,
            0x0030_b208
        ),
        (
            oc810_fake_push_front,
            OC810_PREV_PUSH_FRONT,
            ee::s10::OVERRIDE_0030B260,
            ee::s10::f_0030b260__generated,
            0x0030_b260
        ),
        (
            oc810_fake_load_process_list,
            OC810_PREV_LOAD_PROCESS_LIST,
            ee::s10::OVERRIDE_0030BEE8,
            ee::s10::f_0030bee8__generated,
            0x0030_bee8
        ),
        (
            oc810_fake_collect_set_load,
            OC810_PREV_COLLECT_SET_LOAD,
            ee::s10::OVERRIDE_0030C700,
            ee::s10::f_0030c700__generated,
            0x0030_c700
        ),
        (
            oc810_fake_luno_clear,
            OC810_PREV_LUNO_CLEAR,
            ee::s05::OVERRIDE_00224F68,
            ee::s05::f_00224f68__generated,
            0x0022_4f68
        ),
        (
            oc810_fake_luno_read,
            OC810_PREV_LUNO_READ,
            ee::s05::OVERRIDE_002259A8,
            ee::s05::f_002259a8__generated,
            0x0022_59a8
        ),
        (
            oc810_fake_cache_get_data,
            OC810_PREV_CACHE_GET_DATA,
            ee::s13::OVERRIDE_003ACA38,
            ee::s13::f_003aca38__generated,
            0x003a_ca38
        ),
    );

    /// A script manager with a stream: `self->World`, the 50 global table
    /// pointers (some null), the lists' node counts, and the stream reads.
    pub(crate) fn ls_setup(c: &mut Cpu, seed: u32) {
        oc810_install_fakes();
        let mut r = fakes::machine(c, seed, 0x0810_5c21, 14);
        c.hooks.miss = oc810_read_miss;
        c.bus.write32(LS_CTRL, LS_MAGIC);
        difftest::fill(c, &mut r, LS_SELF, 0x100);
        c.bus.write32(LS_SELF + 0x28c, LS_WORLD);
        c.bus.write32(LS_WORLD + 4, DATA + 0x9000);
        for i in 0..50u32 {
            let t = if r.next_u32() % 3 == 0 {
                0
            } else {
                LS_TABLES + 0x10 * i
            };
            c.bus.write32(LS_SELF + 0x3cc + 4 * i, t);
        }
        c.bus.write32(ls_pops(LS_SELF + 0x2b8), seed % 3);
        c.bus.write32(ls_pops(LS_SELF + 0x2bc), (seed / 3) % 3);
        c.bus.write32(
            LS_COUNT_WORD,
            [0, 1, 2, 3, 0x8000_0000][(seed / 9 % 5) as usize],
        );
        c.bus.write32(LS_ID_WORD, r.next_u32() % 50);
        let vt = LS_VTABLE;
        c.bus
            .write16(vt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(vt + 0x14, fakes::FAKE);
        c.bus.write32(LS_STREAM, vt);
        c.g[A0][0] = LS_SELF as u64;
        c.g[A1][0] = LS_STREAM as u64;
        c.pc = LS_ENTRY;
    }

    /// The seeds reach the list loops and the counted loop: several distinct
    /// cycle totals, and some runs that return normally.
    #[test]
    fn wscript_man_load_state_paths() {
        let mut seen = std::collections::BTreeSet::new();
        let mut returned = 0;
        for seed in 0..180 {
            let mut c = fresh();
            ls_setup(&mut c, seed);
            let before = c.cycles;
            difftest::run_generated(&mut c, ee::s10::f_0030bd20__generated);
            seen.insert(c.cycles - before);
            returned += (c.pc == difftest::RA) as u32;
        }
        assert!(seen.len() >= 8, "paths {seen:?}");
        assert!(returned >= 20, "returned {returned}");
    }

    pub(crate) use crate::game::sound::bankmanager::tests::oc817_leaf_setup;

    /// `AttachedEmitterTypeData_FreePart2` against the generated body.
    #[test]
    fn oc815_attached_emitter_type_data_free_part2_matches() {
        crate::difftest::check(
            ee::s10::f_002fa5f8__generated,
            oc815_attached_emitter_type_data_free_part2,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x815, 0);
                c.pc = 0x002fa5f8;
            },
        );
    }

    /// `AttachedEmitterTypeData_FreePart1`: returns at once.
    #[test]
    fn attached_emitter_type_data_free_part1_matches() {
        check(
            ee::s09::f_002f8ff8__generated,
            oc817_attached_emitter_type_data_free_part1,
            20,
            &oc817_leaf_setup,
        );
    }

    /// Gate word of this test's callee fakes (in the machine's own RAM).
    const WL_CTRL: u32 = DATA + 0x38_0000;

    const WL_MAGIC: u32 = 0x0815_a110;

    /// Every callee of the function under test; all are faked (the real
    /// chains reach the disc loader and the state constructors, which run
    /// away on test data).
    const WL_ADDRS: [u32; 1] = [0x0031_7d70];

    fn wl_idx(addr: u32) -> u32 {
        WL_ADDRS
            .iter()
            .position(|&a| a == addr)
            .expect("fake address") as u32
    }

    /// The fake callee: logs the call like `crate::difftest::fakes::fake_call`, then returns a
    /// small number (the allocator, constructors and library lookups keep
    /// the fake's pointer), or 0 for the calls the fixture's per-callee mask
    /// picks (bit n = nth call of that callee: a failed allocation, a
    /// missing object).
    fn wl_fake_call(c: &mut Cpu, addr: u32) {
        let i = wl_idx(addr);
        let (mask, count) = (WL_CTRL + 0x100 + 4 * i, WL_CTRL + 0x300 + 4 * i);
        let n = c.bus.read32(count);
        c.bus.write32(count, n.wrapping_add(1));
        let zm = c.bus.read32(mask);
        let ra = c.g[RA][0] as u32;
        crate::difftest::fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let zero = n < 32 && zm >> n & 1 != 0;
        if zero {
            c.g[V0][0] = 0;
        } else if !matches!(addr, 0x0031_7d70) {
            c.g[V0][0] >>= 4;
            c.g[V0][0] &= 3;
        }
    }

    macro_rules! wl_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(WL_CTRL) != WL_MAGIC {
                        return crate::difftest::fakes::fall_back(c, &$prev, $gen);
                    }
                    wl_fake_call(c, $addr);
                }
            )*
            fn wl_install_fakes() {
                use std::ptr::addr_of_mut;
                $( crate::difftest::fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    wl_fakes!((
        wl_fk_00317d70,
        WL_PREV_00317D70,
        ee::s10::OVERRIDE_00317D70,
        ee::s10::f_00317d70__generated,
        0x0031_7d70
    ),);

    /// Arm the fakes and give each callee a random call mask (none, all, or
    /// random bits, by seed and callee).
    fn wl_arm(c: &mut Cpu, r: &mut Rng, seed: u32) {
        wl_install_fakes();
        c.bus.write32(WL_CTRL, WL_MAGIC);
        for i in 0..WL_ADDRS.len() as u32 {
            let z = match (seed / 5 + i) % 5 {
                0 | 1 => 0,
                2 => u32::MAX,
                _ => r.next_u32(),
            };
            c.bus.write32(WL_CTRL + 0x100 + 4 * i, z);
        }
    }

    /// `WorldLightMan_AllocOverlapList`: the eight list allocations (their
    /// results are the fake's pointers), and unwinds of those calls.
    #[test]
    fn world_light_man_alloc_overlap_list_matches() {
        crate::difftest::fakes::covered(
            ee::s09::f_002f5298__generated,
            world_light_man_alloc_overlap_list,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f5298, 8);
                wl_arm(c, &mut r, seed);
                // Real allocations only: no failed (null) ones.
                c.bus.write32(WL_CTRL + 0x100, 0);
                let this = DATA + 0x30_0000;
                fill(c, &mut r, this, 0xcc4);
                c.g[A0][0] = sx(this);
                c.pc = 0x002f_5298;
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use std::ptr::addr_of_mut;

    /// Gate of the oc1054 fakes (other tests' machines fall back to the real callees).
    pub(crate) const OC1054_GATE: u32 = crate::difftest::DATA + 0xf_0000;

    pub(crate) const OC1054_MAGIC: u32 = 0x1054_0c0e;

    macro_rules! oc1054_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1054_GATE) != OC1054_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc1054_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc1054_fakes! {
        oc1054_fake_emitter_ctor, 0x0037_14b8, ee::s12::OVERRIDE_003714B8, ee::s12::f_003714b8__generated;
        oc1054_fake_holder_dtor, 0x002d_56b0, ee::s09::OVERRIDE_002D56B0, ee::s09::f_002d56b0__generated;
    }

    /// `a1 == 0xffff` (init when `a0 != 0`, destroy when `a0 == 0`) or another
    /// mode (early exit, every sixth seed); `calls`: the callee call index at
    /// which a fake may unwind.
    pub(crate) fn oc1054_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        oc1054_hook();
        let mut r = machine(c, seed, 0x1054, calls);
        elf_image(c);
        c.bus.write32(OC1054_GATE, OC1054_MAGIC);
        c.g[A0][0] = if seed % 2 == 0 {
            0
        } else {
            r.next_u32() as u64 | 1
        };
        c.g[A1][0] = if seed % 6 == 5 {
            r.next_u32() as u64
        } else {
            0xffff
        };
        r
    }

    /// Early exit, destroy loop and init (with its two constructor loops),
    /// with fakes that unwind at random callee calls.
    #[test]
    fn visual_effects_man_static_init_destroy_matches() {
        check(
            ee::s09::f_002f6b88__generated,
            visual_effects_man_static_init_destroy,
            300,
            &|c, seed| {
                oc1054_setup(c, seed, 130);
            },
        );
    }

    /// The seeds reach the early exit, the destroy loop and the init path.
    #[test]
    fn visual_effects_man_static_init_destroy_path_count() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..12 {
            let mut c = crate::difftest::fresh();
            oc1054_setup(&mut c, seed, 0);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s09::f_002f6b88__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 3, "only {} distinct paths", seen.len());
    }

    /// Events delivered or unwinding at the loop back edges.
    #[test]
    fn visual_effects_man_static_init_destroy_events() {
        check_events(
            ee::s09::f_002f6b88__generated,
            visual_effects_man_static_init_destroy,
            120,
            &|c, seed| {
                let mut r = oc1054_setup(c, seed, 0);
                schedule(c, &mut r, seed, 0x002f_6b88)
            },
        );
    }

    /// Static init of the attached-emitter colour table: runs only when
    /// a1 == 0xffff and a0 != 0 (otherwise a bare prologue/epilogue). The
    /// float constants come from the `.lit4` pool at gp-0x3704.., so the
    /// fixture fills the pool with random floats.
    #[test]
    fn attached_emitter_type_data_static_init1_matches() {
        crate::difftest::check(
            ee::s09::f_002f9040__generated,
            attached_emitter_type_data_static_init1,
            16,
            &|c, seed| {
                let mut r = crate::difftest::Rng(0x2f9040 + seed as u64);
                c.g[SP][0] = sx(0x0070_0000);
                c.g[GP][0] = sx(0x0050_8000);
                c.g[RA][0] = sx(crate::difftest::RA);
                c.pc = 0x002f9040;
                c.g[A0][0] = if seed % 4 == 1 { 0 } else { 1 };
                c.g[A1][0] = if seed % 4 == 2 { 1 } else { 0xffff };
                for i in 0..0x100u32 {
                    let a = 0x0050_8000 - 0x3800 + i * 4;
                    let v = r.f32(2.0);
                    c.bus.write32(a, v);
                }
            },
        );
    }

    /// Reset path (init != 0, mask == 0xffff), the early exits (init == 0,
    /// mask != 0xffff), and varied literal-pool floats around gp.
    #[test]
    fn attached_emitter_type_data_static_init2_matches() {
        check(
            ee::s10::f_002fa640__generated,
            attached_emitter_type_data_static_init2,
            30,
            &|c, seed| {
                let _ = machine(c, seed, 0x2fa640, 0);
                match seed % 4 {
                    0 => {
                        c.g[A0][0] = 0;
                        c.g[A1][0] = 0xffff;
                    }
                    1 => {
                        c.g[A0][0] = 1;
                        c.g[A1][0] = 0x1234 + seed as u64;
                    }
                    _ => {
                        c.g[A0][0] = 1 + (seed % 3) as u64;
                        c.g[A1][0] = 0xffff;
                    }
                }
                c.pc = 0x002f_a640;
            },
        );
    }

    /// Gate word of this module's fakes (own address and magic in the machine's
    /// RAM: other tests' machines run the real bodies).
    pub(crate) const OC1069_CTRL: u32 = DATA + 0x21_0000;

    pub(crate) const OC1069_MAGIC: u32 = 0x1069_f00d;

    /// Fixture objects: the rider's `*self` holder and the rider data block,
    /// and the avalanche object (clear of the fakes' pointer results).
    pub(crate) const OC1069_THIS: u32 = DATA + 0x3000;

    pub(crate) const OC1069_RIDER: u32 = DATA + 0x4000;

    pub(crate) const OC1069_AVALANCHE: u32 = DATA + 0x5000;

    /// Every callee of the three bodies: a gated fake in its override slot that
    /// logs the call and returns (or unwinds, as the machine chooses).
    macro_rules! oc1069_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1069_CTRL) != OC1069_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            /// Puts every fake in its override slot (once per process).
            fn oc1069_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc1069_fakes! {
        oc1069_fake_002dcf28, 0x002d_cf28, ee::s09::OVERRIDE_002DCF28, ee::s09::f_002dcf28__generated;
        oc1069_fake_002daa78, 0x002d_aa78, ee::s09::OVERRIDE_002DAA78, ee::s09::f_002daa78__generated;
        oc1069_fake_002e8560, 0x002e_8560, ee::s09::OVERRIDE_002E8560, ee::s09::f_002e8560__generated;
        oc1069_fake_002d4be0, 0x002d_4be0, ee::s09::OVERRIDE_002D4BE0, ee::s09::f_002d4be0__generated;
        oc1069_fake_002e3930, 0x002e_3930, ee::s09::OVERRIDE_002E3930, ee::s09::f_002e3930__generated;
        oc1069_fake_002ef6a0, 0x002e_f6a0, ee::s09::OVERRIDE_002EF6A0, ee::s09::f_002ef6a0__generated;
        oc1069_fake_002d6410, 0x002d_6410, ee::s09::OVERRIDE_002D6410, ee::s09::f_002d6410__generated;
        oc1069_fake_0011b978, 0x0011_b978, ee::s00::OVERRIDE_0011B978, ee::s00::f_0011b978__generated;
        oc1069_fake_00317e50, 0x0031_7e50, ee::s10::OVERRIDE_00317E50, ee::s10::f_00317e50__generated;
    }

    /// Hooked fakes, scrambled registers, the gate on.
    pub(crate) fn oc1069_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1069_hook();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1069_CTRL, OC1069_MAGIC);
        r
    }

    /// The rider case: `flags` covers the copy path (zero, half of the seeds),
    /// each destructor bit, and high bits that only the 64-bit tests see.
    pub(crate) fn oc1069_dtor_setup(c: &mut Cpu, seed: u32) -> Events {
        oc1069_hook();
        let mut r = machine(c, seed, 0x2f_6388, 0);
        elf_image(c);
        c.bus.write32(OC1069_CTRL, OC1069_MAGIC);
        let mut flags = if seed % 2 == 0 {
            0u64
        } else {
            [1u64, 2, 3][((seed / 2) % 3) as usize]
        };
        if seed % 16 == 9 {
            flags |= (r.next_u32() as u64) << 32;
        }
        c.bus.write32(OC1069_THIS, OC1069_RIDER);
        c.g[A0][0] = sx(OC1069_THIS);
        c.g[A1][0] = flags;
        schedule(c, &mut r, seed, 0x002f_6388)
    }

    /// The copy path, both destructor bits, and the event deliveries in the
    /// two copy loops, against the generated body.
    #[test]
    fn rider_base_dtor_cheat_code_matches() {
        check_events(
            ee::s09::f_002f6388__generated,
            rider_base_dtor_cheat_code,
            300,
            &oc1069_dtor_setup,
        );
    }

    /// The thunks forward to their `Reset`, with the callee's return or unwind.
    pub(crate) fn oc1069_thunk_setup(c: &mut Cpu, seed: u32, salt: u64) {
        oc1069_machine(c, seed, salt, 1);
    }

    #[test]
    fn cboard_wake_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7ab8__generated,
            cboard_wake_fx_reset_thunk,
            40,
            &|c, seed| oc1069_thunk_setup(c, seed, 0x2f_7ab8),
        );
    }

    #[test]
    fn cboard_spark_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7ae0__generated,
            cboard_spark_fx_reset_thunk,
            40,
            &|c, seed| oc1069_thunk_setup(c, seed, 0x2f_7ae0),
        );
    }

    #[test]
    fn cboard_trail_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7b08__generated,
            cboard_trail_fx_reset_thunk,
            40,
            &|c, seed| oc1069_thunk_setup(c, seed, 0x2f_7b08),
        );
    }

    #[test]
    fn cang_er_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7b30__generated,
            cang_er_fx_reset_thunk,
            40,
            &|c, seed| oc1069_thunk_setup(c, seed, 0x2f_7b30),
        );
    }

    #[test]
    fn cplayer_pulse_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7b58__generated,
            cplayer_pulse_fx_reset_thunk,
            40,
            &|c, seed| oc1069_thunk_setup(c, seed, 0x2f_7b58),
        );
    }

    #[test]
    fn cair_streamer_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7b80__generated,
            cair_streamer_fx_reset_thunk,
            40,
            &|c, seed| oc1069_thunk_setup(c, seed, 0x2f_7b80),
        );
    }

    /// The avalanche object gets its vtable, `Init` runs, `self` comes back.
    #[test]
    fn vfx_avalanche_ctor_matches() {
        check(
            ee::s09::f_002f7c38__generated,
            vfx_avalanche_ctor,
            40,
            &|c, seed| {
                oc1069_machine(c, seed, 0x2f_7c38, 1);
                c.bus.write32(OC1069_AVALANCHE, seed);
                c.g[A0][0] = sx(OC1069_AVALANCHE);
            },
        );
    }

    #[test]
    fn c_speed_boost_streamer_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7ea8__generated,
            c_speed_boost_streamer_fx_reset_thunk,
            150,
            &|c, seed| {
                let mut r = Rng(0x2f7e ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.next_u32());
                c.bus.write32(
                    0x870,
                    if seed % 2 == 0 {
                        r.next_u32()
                    } else {
                        r.next_u32() | 0x8000_0000
                    },
                );
            },
        );
    }

    #[test]
    fn transition_effect_fade_ctor_matches() {
        check(
            ee::s09::f_002f8060__generated,
            transition_effect_fade_ctor,
            150,
            &|c, seed| {
                let mut r = Rng(0x2f80 ^ seed as u64);
                scramble(c, &mut r);
                let src = DATA + 0x200;
                for k in 0..8 {
                    c.bus.write32(src + 4 * k, r.next_u32());
                }
                // The colour block may start unaligned: cover 0..6 byte offsets.
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (src + (seed % 3) * 3) as u64;
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn c_cut_scene_transition_man_dtor_matches() {
        check(
            ee::s09::f_002f81b8__generated,
            c_cut_scene_transition_man_dtor,
            60,
            &|c, seed| {
                fakes::machine_rt(c, seed, 0x2f81, 0);
                // `this` at DATA+0x100 with its process-list links; an odd
                // flag takes the delete path.
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let links = seed % 3 != 0;
                c.bus
                    .write32(DATA + 0x104, if links { DATA + 0x200 } else { 0 });
                c.bus
                    .write32(DATA + 0x108, if links { DATA + 0x280 } else { 0 });
            },
        );
    }

    #[test]
    fn visual_effects_man_global_ctors_matches() {
        check(
            ee::s09::f_002f8370__generated,
            visual_effects_man_global_ctors,
            8,
            &|c, seed| {
                fakes::machine_rt(c, seed, 0x2f83, 0);
            },
        );
    }

    #[test]
    fn visual_effects_man_global_dtors_matches() {
        check(
            ee::s09::f_002f8390__generated,
            visual_effects_man_global_dtors,
            8,
            &|c, seed| {
                fakes::machine_rt(c, seed, 0x2f84, 0);
            },
        );
    }

    #[test]
    fn attached_emitter_type_data_global_ctors1_matches() {
        check(
            ee::s09::f_002f9818__generated,
            attached_emitter_type_data_global_ctors1,
            8,
            &|c, seed| {
                fakes::machine_rt(c, seed, 0x2f98, 0);
            },
        );
    }

    #[test]
    fn attached_emitter_type_data_global_ctors2_matches() {
        check(
            ee::s10::f_002fae18__generated,
            attached_emitter_type_data_global_ctors2,
            8,
            &|c, seed| {
                fakes::machine_rt(c, seed, 0x2fae, 0);
            },
        );
    }
}
