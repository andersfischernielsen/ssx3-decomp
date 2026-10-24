//! `src/visualfx/worldlightman.cpp`: hand-written replacements (moved from the
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
use crate::call::Target;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_2;
pub(crate) use crate::game::be::bedbreward::BUILTIN_DELETE_ADDR;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::r;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_9 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::p_7 as p_2;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_3;
pub(crate) use crate::game::intersect::worldsphtree::forward_to_arg_vtable;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::render::lightman::addu;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::render::particle::eq;
pub(crate) use crate::game::render::particle::fconst;
pub(crate) use crate::game::render::particle::ldf;
pub(crate) use crate::game::render::particle::lt;
pub(crate) use crate::game::render::particle::p;
pub(crate) use crate::game::render::particle::setf;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
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
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_FROM_SCRATCHPAD;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_TO_SCRATCHPAD;
    pub(crate) use crate::game::render::lightman::callee::EA_MEMSET;
    pub(crate) use crate::game::render::lightman::callee::FLAG_MANAGER_GROUP_COMPUTE_VERTS;
    pub(crate) use crate::game::render::lightman::callee::INTERSECT_INFO_EVALUATE;
    pub(crate) use crate::game::render::lightman::callee::LIGHT_CONTRIBUTION;
    pub(crate) use crate::game::render::lightman::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::render::lightman::callee::MATH_SIN_COS;
    pub(crate) use crate::game::render::lightman::callee::MULTI_SPLINE_MODIFIER_DEBUG_DRAW_PATH;
    pub(crate) use crate::game::render::lightman::callee::SPATIAL_HASH_UPDATE;
    pub(crate) use crate::game::render::lightman::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::TYPED_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_AI_FRAME_COUNT;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_FLAG_MANAGER;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_NASH;
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
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
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
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.sw((c.g[GP][0] as u32).wrapping_add(0x9cc), 0u64);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());

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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
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
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP as TEST_SP;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;
    pub(crate) use crate::game::ai::aianimeventmap::tests::MEM_MUTEX;
    pub(crate) use crate::game::bx::bxstring::tests::alloc_ready;
    pub(crate) use crate::game::bx::bxstring::tests::step;
    pub(crate) use crate::game::dirtysock::tags2::tests::pool8;
    pub(crate) use crate::game::dirtysock::tags2::tests::slot_8 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::fepopup::tests::f_2;
    pub(crate) use crate::game::input::inputmap::tests::put_f32;
    pub(crate) use crate::game::intersect::worldsphtree::tests::forward_setup;
    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_4;
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
    pub(crate) use crate::game::worldpainter::quadtree::tests::put_vtable;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals;
    pub(crate) use crate::game::wscript::wscriptman::tests::w;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD;
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
}
