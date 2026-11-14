//! `src/misc.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_reg;
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
pub(crate) use crate::game::fe::ovstatehud::oc224_gp;
pub(crate) use crate::game::fe::ovstatehud::oc224_ld_u;
pub(crate) use crate::game::fe::ovstatehud::oc224_sd_u;
pub(crate) use crate::game::fe::splashmanager::gpo;
pub(crate) use crate::game::fe::splashmanager::oc230_gp;
pub(crate) use crate::game::fe::splashmanager::stk;
pub(crate) use crate::game::intersect::aifwddiff::oc230_add;
pub(crate) use crate::game::intersect::aifwddiff::oc230_at;
pub(crate) use crate::game::intersect::aifwddiff::oc230_sll;
pub(crate) use crate::game::main::gameload::vcall;
pub(crate) use crate::game::main::gameload::R2D_CTX_GP;
pub(crate) use crate::game::main::gameload::R2D_STATE_TOP;
pub(crate) use crate::game::ui::uistate::oc224_at;
pub(crate) use crate::game::visualfx::lensfx::at;
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
        0x002f_4db8,
        lens_fx_update_sun_position,
        "LensFX_UpdateSunPosition",
    ),
    (0x0021_d1a0, hud_draw_meter_bar, "HUD_DrawMeterBar"),
    (0x0010_8a48, rider_find_rail, "Rider_FindRail"),
    (0x0021_e750, hud_align_x, "HUD_AlignX"),
    (
        0x0010_bd10,
        computer_actor_rider_select_target_rider,
        "ComputerActorRider_SelectTargetRider",
    ),
    (0x002b_af90, wp_data_find_cell_index, "WPData_FindCellIndex"),
    (
        0x002f_1510,
        rider_base_render_attack_fx,
        "RiderBase_RenderAttackFX",
    ),
    (0x0011_3e80, rider_set_slew0_target, "Rider_SetSlew0Target"),
    (0x0021_e7a8, hud_align_y, "HUD_AlignY"),
    (0x0022_00c0, hud_draw_speedometer, "HUD_DrawSpeedometer"),
    (0x002f_4a08, lens_fx_draw_sun, "LensFX_DrawSun"),
    (0x003d_4f10, path_update_tracks, "PATH_UpdateTracks"),
    (0x003e_12e0, stream_file_get_chunk, "StreamFile_GetChunk"),
    (0x0042_c078, d_intr, "DIntr"),
    (
        0x0010_aa70,
        computer_actor_rider_think_prewind,
        "ComputerActorRider_ThinkPrewind",
    ),
    (
        0x0010_b0e8,
        computer_actor_rider_update_spin_plan,
        "ComputerActorRider_UpdateSpinPlan",
    ),
    (
        0x0010_bb18,
        computer_actor_rider_find_speed_marker,
        "ComputerActorRider_FindSpeedMarker",
    ),
    (
        0x0011_4cc0,
        rider_turn_around_if_backward,
        "Rider_TurnAroundIfBackward",
    ),
    (
        0x0011_61d0,
        rider_try_enter_board_press,
        "Rider_TryEnterBoardPress",
    ),
    (
        0x002d_bf98,
        ps2_fog_particle_man_render,
        "cPS2FogParticleMan_Render",
    ),
    (
        0x002f_0a98,
        ps2_visual_effects_man_update,
        "PS2VisualEffectsMan_Update",
    ),
    (
        0x002f_0cd0,
        ps2_visual_effects_man_render,
        "PS2VisualEffectsMan_Render",
    ),
    (0x003b_96b8, sndmidi_service, "SNDMIDI_Service"),
    (0x003d_5128, path_update, "PATH_Update"),
    (0x003f_f9a8, libpad_get_dma_buffer, "libpad_GetDmaBuffer"),
    (0x0040_d588, tanf, "tanf"),
    (
        0x0010_0f88,
        computer_actor_rider_cruise_chase_rider,
        "ComputerActorRider_CruiseChaseRider",
    ),
    (
        0x0010_bbf8,
        computer_actor_rider_find_board_press_marker,
        "ComputerActorRider_FindBoardPressMarker",
    ),
    (
        0x0010_c0a8,
        computer_actor_rider_set_cruise_crouch,
        "ComputerActorRider_SetCruiseCrouch",
    ),
    (
        0x0030_19c8,
        wscript_fn_get_object_slot32,
        "WScriptFn_GetObjectSlot32",
    ),
    (0x003e_4db8, ea_timer_rcnt1_handler, "EATimer_Rcnt1Handler"),
    (0x0040_d758, expf, "expf"),
    (0x0040_da10, logf, "logf"),
    (0x0042_c0c0, e_intr, "EIntr"),
    (0x0041_5fc8, memcmp, "memcmp"),
    (0x0010_86b8, rider_probe_rail, "Rider_ProbeRail"),
    (0x0011_3db0, race_state_race_update, "RaceStateRace_Update"),
    (
        0x0021_e7e0,
        hud_draw_stretched_panel,
        "HUD_DrawStretchedPanel",
    ),
    (
        0x002f_5148,
        world_light_man_light_instance_cb,
        "WorldLightMan_LightInstanceCB",
    ),
    (0x003d_62c8, path_track_set_volume, "PATH_TrackSetVolume"),
    (
        0x003d_fc48,
        stream_file_consume_bytes,
        "StreamFile_ConsumeBytes",
    ),
    (
        0x003e_13e8,
        stream_file_release_chunk,
        "StreamFile_ReleaseChunk",
    ),
    (
        0x0010_09e0,
        computer_actor_rider_cruise_approach_jump,
        "ComputerActorRider_CruiseApproachJump",
    ),
    (
        0x0010_b590,
        computer_actor_rider_think_natural_air,
        "ComputerActorRider_ThinkNaturalAir",
    ),
    (
        0x0030_3ba0,
        wscript_fn_crowd2d_control,
        "WScriptFn_Crowd2dControl",
    ),
    (
        0x003d_fe88,
        stream_file_classify_chunk,
        "StreamFile_ClassifyChunk",
    ),
    (0x003e_6328, ea_shell_sort_pairs, "EA_ShellSortPairs"),
    (
        0x0011_5640,
        rider_reset_grind_orientation,
        "Rider_ResetGrindOrientation",
    ),
    (
        0x002f_c0d0,
        world_obj_factory_object,
        "WorldObjFactory_Object",
    ),
    (
        0x002f_e2c0,
        wscript_fn_add_uv_scroll_modifier,
        "WScriptFn_AddUVScrollModifier",
    ),
    (0x003b_49d0, refpack_decode, "RefPack_Decode"),
    (
        0x003d_fed0,
        stream_file_parse_chunks,
        "StreamFile_ParseChunks",
    ),
    (0x003e_0170, stream_file_on_read, "StreamFile_OnRead"),
    (
        0x003e_0270,
        stream_file_start_next_request,
        "StreamFile_StartNextRequest",
    ),
    (0x003f_f390, libpad_send_act_dma, "libpad_SendActDma"),
    (0x0040_0190, sce_pad_set_act_direct, "scePadSetActDirect"),
    (0x0010_57b8, rider_bounce_off_wall, "Rider_BounceOffWall"),
    (0x0010_5d98, rider_handle_impact, "Rider_HandleImpact"),
    (
        0x0011_58b8,
        input_quantize_stick_direction,
        "Input_QuantizeStickDirection",
    ),
    (
        0x0010_65b0,
        rider_deflect_along_wall,
        "Rider_DeflectAlongWall",
    ),
    (
        0x0010_bfa8,
        computer_actor_rider_plan_air_tricks,
        "ComputerActorRider_PlanAirTricks",
    ),
    (
        0x002f_bcb8,
        world_obj_factory_anim_object,
        "WorldObjFactory_AnimObject",
    ),
    (
        0x002f_ee98,
        world_obj_factory_add_dynamic_particle_data,
        "WorldObjFactory_AddDynamicParticleData",
    ),
    (
        0x002f_ff00,
        wscript_fn_play_object_sound,
        "WScriptFn_PlayObjectSound",
    ),
    (
        0x0030_2968,
        wscript_fn_set_self_object,
        "WScriptFn_SetSelfObject",
    ),
    (0x003b_4740, refpack_unpack, "RefPack_Unpack"),
    (0x003d_3c90, path_remap_node, "PATH_RemapNode"),
    (
        0x003e_0c88,
        stream_file_queue_range,
        "StreamFile_QueueRange",
    ),
    (0x0040_d610, libm_log, "log"),
    (
        0x0010_c1d0,
        computer_actor_rider_roll_skip_tricks,
        "ComputerActorRider_RollSkipTricks",
    ),
    (
        0x0010_c258,
        computer_actor_rider_roll_uber_trick,
        "ComputerActorRider_RollUberTrick",
    ),
    (
        0x0010_c320,
        computer_actor_rider_roll_fitted_trick,
        "ComputerActorRider_RollFittedTrick",
    ),
    (
        0x002f_c420,
        wscript_fn_make_dead_node,
        "WScriptFn_MakeDeadNode",
    ),
    (
        0x0030_00a8,
        wscript_fn_start_object_loop,
        "WScriptFn_StartObjectLoop",
    ),
    (
        0x0030_2490,
        wscript_fn_particle_control,
        "WScriptFn_ParticleControl",
    ),
    (
        0x003d_fcd8,
        stream_file_alloc_request,
        "StreamFile_AllocRequest",
    ),
    (
        0x003d_fd58,
        stream_file_enqueue_request,
        "StreamFile_EnqueueRequest",
    ),
    (
        0x003d_fe18,
        stream_file_free_request,
        "StreamFile_FreeRequest",
    ),
    (
        0x003e_14c8,
        stream_file_get_channel_bytes,
        "StreamFile_GetChannelBytes",
    ),
    (0x003e_1530, stream_file_is_idle, "StreamFile_IsIdle"),
    (
        0x0010_c3b8,
        computer_actor_rider_roll_random_trick,
        "ComputerActorRider_RollRandomTrick",
    ),
    (0x0040_f840, isnan, "isnan"),
];

// ---------------------------------------------------------------------------
// Misc newlib: allocator glue and the cache-range loops.

pub(crate) const MALLOC_R: Callee = Callee {
    addr: 0x0041_dbf8,
    f: ssx3_ee::s15::f_0041dbf8,
};

pub(crate) const MEMSET: Callee = Callee {
    addr: 0x0041_6210,
    f: ssx3_ee::s15::f_00416210,
};

/// `struct glue *_sfmoreglue(struct _reent *r, int n)` (0x41ff98): allocate
/// `n * 0x58 + 0xc` for a `FILE` glue record and zero its data area.
pub fn sfmoreglue(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x58;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.mult(0, S2, S1, V0); // s2 = n * 0x58, LO/HI
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
        MALLOC_R.call(c, 0x0041_ffc0)?;
        c.g[S0][0] = c.g[V0][0];
        let null = c.g[S0][0] == 0;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        if null {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 9;
            Addr(c.g[S0][0] as u32).off(0x4).sw(c, c.g[S1][0]);
            Addr(c.g[S0][0] as u32).sw(c, 0);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S2][0];
            Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0;
            MEMSET.call(c, 0x0041_ffe8)?;
            c.g[V0][0] = c.g[S0][0];
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

/// `void *_calloc_r(struct _reent *r, size_t n, size_t size)` (0x4224a0):
/// `malloc(n * size)` then zero it (inline for small blocks, `memset`
/// otherwise).
pub fn calloc_r(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        c.mult(0, 0, A1, A2); // LO/HI = n * size
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lo[0];
        MALLOC_R.call(c, 0x0042_24bc)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_sub(4)).lw(c);
            c.cycles += 7;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[V0][0] |= 0xfffc;
            c.g[V1][0] &= c.g[V0][0];
            c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(4));
            c.g[V0][0] = (c.g[A2][0] < 0x25) as u64;
            let large = c.g[V0][0] == 0;
            c.g[V0][0] = (c.g[A2][0] < 0x14) as u64;
            if large {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                MEMSET.call(c, 0x0042_2548)?;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.g[S0][0];
                } else {
                    c.cycles += 5;
                    Addr(c.g[S0][0] as u32).sw(c, 0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = (c.g[A2][0] < 0x1c) as u64;
                    let t = c.g[V0][0] != 0;
                    Addr(c.g[S0][0] as u32).off(0x4).sw(c, 0);
                    if !t {
                        c.cycles += 5;
                        c.g[V0][0] = (c.g[A2][0] < 0x24) as u64;
                        Addr(c.g[S0][0] as u32).off(0x8).sw(c, 0);
                        Addr(c.g[S0][0] as u32).off(0xc).sw(c, 0);
                        let t = c.g[V0][0] != 0;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                        if !t {
                            c.cycles += 3;
                            Addr(c.g[S0][0] as u32).off(0x10).sw(c, 0);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                            Addr(c.g[S0][0] as u32).off(0x14).sw(c, 0);
                        }
                    }
                }
                c.cycles += 5;
                Addr(c.g[A0][0] as u32).sw(c, 0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                Addr(c.g[A0][0] as u32).off(0x4).sw(c, 0);
                Addr(c.g[A0][0] as u32).sw(c, 0);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod meter_bar {

    /// `float pos[2]` (HUD_GetElemPos).
    pub const POS: u32 = 0x00;

    /// `float x, y`: the aligned screen position. `y` walks up the bar as
    /// the full bar is drawn.
    pub const X: u32 = 0x10;

    pub const Y: u32 = 0x14;

    /// `float w, h` (HUD_GetElemSize, then scaled).
    pub const W: u32 = 0x20;

    pub const H: u32 = 0x24;

    /// `float sx, sy` (HUD_GetElemScale, then scaled; `sy` also by the
    /// segment scale: one segment is `23 * sy` high, the top cap `31 * sy`).
    pub const SCALE: u32 = 0x30;

    pub const SY: u32 = 0x34;

    /// `float {1, 1}`: the scale passed to HUD_AlignElemPos.
    pub const UNIT_SCALE: u32 = 0x40;

    /// The quad: four 0x30-byte vertices.
    pub const QUAD: u32 = 0x50;

    pub const VERTEX: u32 = 0x30;

    /// Vertex fields: texture `u, v`, `z = 1`, the colour as four ints,
    /// screen `x, y`, `0`, `1`.
    pub const V_U: u32 = 0x00;

    pub const V_V: u32 = 0x04;

    pub const V_Z: u32 = 0x08;

    pub const V_COLOR: u32 = 0x10;

    pub const V_X: u32 = 0x20;

    pub const V_Y: u32 = 0x24;

    pub const V_ZERO: u32 = 0x28;

    pub const V_ONE: u32 = 0x2c;

    /// HUD sprite fields: texture id (`lw`, stored as a short into the
    /// render state), v of the top edge, u left, u right, v of the bottom.
    pub const TEX_ID: u32 = 0x00;

    pub const TEX_V0: u32 = 0x0c;

    pub const TEX_U0: u32 = 0x10;

    pub const TEX_U1: u32 = 0x14;

    pub const TEX_V1: u32 = 0x18;

    /// The sprite table of the HUD object: 12 bytes per meter style
    /// (`top cap, segment, bottom cap` pointers).
    pub const SPRITES: u32 = 0x4fc;

    /// Renderer (`gp-0x854`) fields: the current render-state record and the
    /// vptr; render-state fields: blend bits (5..9 of word 2), texture id.
    pub const RENDERER: u32 = 0x854;

    pub const STATE_TOP: u32 = 0xe84;

    pub const VPTR: u32 = 0x10d8;

    pub const STATE_FLAGS: u32 = 0x08;

    pub const STATE_TEX: u32 = 0x10;

    /// Renderer method `DrawPrims(count, verts, 0)`.
    pub const DRAW_PRIMS: u32 = 0x238;

    /// Layout element fields: horizontal / vertical alignment and the blend
    /// mode byte.
    pub const ELEM_HALIGN: u32 = 0x20;

    pub const ELEM_VALIGN: u32 = 0x21;

    pub const ELEM_BLEND: u32 = 0x22;

    /// Segment and top cap heights in layout pixels.
    pub const SEGMENT_H: u32 = 0x41b8_0000;
    // 23.0
    pub const TOP_CAP_H: u32 = 0x41f8_0000;
}

/// `f[n]` as an FPU float.
#[inline(always)]
pub(crate) fn mb_f(c: &Cpu, n: usize) -> crate::ps2f::FpuF32 {
    crate::ps2f::FpuF32(c.f[n])
}

/// `f[n] = v`.
#[inline(always)]
pub(crate) fn mb_set(c: &mut Cpu, n: usize, v: crate::ps2f::FpuF32) {
    c.f[n] = v.0;
}

/// `lwc1` from the frame.
#[inline(always)]
pub(crate) fn mb_ld(c: &mut Cpu, off: u32) -> crate::ps2f::FpuF32 {
    crate::guest::Addr(sp(c).wrapping_add(off)).read_fpu(c)
}

/// `swc1` to the frame.
#[inline(always)]
pub(crate) fn mb_st(c: &mut Cpu, off: u32, v: crate::ps2f::FpuF32) {
    crate::guest::Addr(sp(c).wrapping_add(off)).write_fpu(c, v)
}

/// `lwc1` of a sprite field.
#[inline(always)]
pub(crate) fn mb_tex(c: &mut Cpu, tex: u64, off: u32) -> crate::ps2f::FpuF32 {
    crate::guest::Addr((tex as u32).wrapping_add(off)).read_fpu(c)
}

/// The renderer (`lw gp-0x854`).
#[inline(always)]
pub(crate) fn mb_renderer(c: &mut Cpu) -> u64 {
    crate::guest::Addr(r32(c, GP).wrapping_sub(meter_bar::RENDERER)).lw(c)
}

/// The quad's texture coordinates from sprite `tex`, the top edge at
/// `v_top` (the sprite's v0, or clipped). Leaves `f0 = v1`, `f1 = u1`, as
/// the original's alternating loads do.
pub(crate) fn mb_put_uvs(c: &mut Cpu, tex: u64, v_top: crate::ps2f::FpuF32) {
    use meter_bar::*;
    let u0 = mb_tex(c, tex, TEX_U0);
    let u1 = mb_tex(c, tex, TEX_U1);
    let v1 = mb_tex(c, tex, TEX_V1);
    for (i, u, v) in [(0, u0, v_top), (1, u1, v_top), (2, u0, v1), (3, u1, v1)] {
        mb_st(c, QUAD + VERTEX * i + V_U, u);
        mb_st(c, QUAD + VERTEX * i + V_V, v);
    }
    mb_set(c, 0, v1);
    mb_set(c, 1, u1);
}

/// The quad's screen corners: `(x0, top)`, `(x1, top)`, `(x0, bottom)`,
/// `(x1, bottom)`.
pub(crate) fn mb_put_xy(
    c: &mut Cpu,
    x0: crate::ps2f::FpuF32,
    x1: crate::ps2f::FpuF32,
    top: crate::ps2f::FpuF32,
    bottom: crate::ps2f::FpuF32,
) {
    use meter_bar::*;
    for (i, x, y) in [(0, x0, top), (1, x1, top), (2, x0, bottom), (3, x1, bottom)] {
        mb_st(c, QUAD + VERTEX * i + V_X, x);
        mb_st(c, QUAD + VERTEX * i + V_Y, y);
    }
}

/// Bind sprite `tex`'s texture: `sh id, state+0x10`. The registers the
/// original uses differ per block: the caller names them (`state_reg` holds
/// the render-state record, `id_reg` the id).
pub(crate) fn mb_bind(c: &mut Cpu, renderer: u64, tex: u64, state_reg: usize, id_reg: usize) {
    use meter_bar::*;
    let id = crate::guest::Addr((tex as u32).wrapping_add(TEX_ID)).lw(c);
    c.g[id_reg][0] = id;
    let state = crate::guest::Addr((renderer as u32).wrapping_add(STATE_TOP)).lw(c);
    c.g[state_reg][0] = state;
    crate::guest::Addr((state as u32).wrapping_add(STATE_TEX)).sh(c, id);
}

/// `renderer->DrawPrims(4, quad, 0)` returning to `ret` (t0 = renderer,
/// v0 = vptr, v1 = method, a0 = adjusted this).
pub(crate) fn mb_draw_quad(c: &mut Cpu, ret_to: u32) -> crate::call::Flow {
    use meter_bar::*;
    c.g[A1][0] = 4;
    c.g[A2][0] = c.g[S3][0];
    c.g[A3][0] = 0;
    let renderer = mb_renderer(c);
    c.g[T0][0] = renderer;
    let vt = crate::guest::Addr((renderer as u32).wrapping_add(VPTR)).lw(c);
    c.g[V0][0] = vt;
    let m = crate::call::VSlot::read(c, vt, DRAW_PRIMS);
    c.g[V1][0] = m.func;
    c.g[A0][0] = m.this(renderer);
    call_ptr(c, m.func as u32, ret_to)
}

/// The partial bar's clipped cap: when the cap's top `top` (f`top_reg`)
/// reaches above the fill line (f21), its top edge moves down to the fill
/// line and its top `v` (f2) moves proportionally: `v0 + (v1 - v0) *
/// (fill - top) / height` (f3 = cap height).
pub(crate) fn mb_clip_cap(c: &mut Cpu, tex: u64, top_reg: usize) {
    use meter_bar::*;
    c.cycles += 7;
    let v1 = mb_tex(c, tex, TEX_V1);
    let fill = mb_f(c, 21);
    let cut = fill - mb_f(c, top_reg);
    mb_set(c, 1, cut);
    mb_set(c, top_reg, fill);
    let dv = ((v1 - mb_f(c, 2)) * cut).div(mb_f(c, 3));
    mb_set(c, 0, dv);
    mb_set(c, 2, mb_f(c, 2) + dv);
}

/// A partial bar's cap from sprite `tex` (in a0's block): v top in f2,
/// screen top in f`top_reg`, bottom in f4. 39 or 37 cycles are charged by
/// the caller.
pub(crate) fn mb_draw_partial_cap(
    c: &mut Cpu,
    tex: u64,
    top_reg: usize,
    ret_to: u32,
) -> crate::call::Flow {
    use meter_bar::*;
    let renderer = c.g[V1][0];
    mb_bind(c, renderer, tex, V0, A0);
    let x = mb_ld(c, X);
    let v_top = mb_f(c, 2);
    mb_put_uvs(c, tex, v_top);
    mb_set(c, 3, x);
    let x1 = x + mb_ld(c, W);
    mb_set(c, 2, x1);
    let (top, bottom) = (mb_f(c, top_reg), mb_f(c, 4));
    mb_put_xy(c, x, x1, top, bottom);
    mb_draw_quad(c, ret_to)
}

/// `void HUD_DrawMeterBar(HUDState* hud (a0), HUDElem* elems (a1), s8 elem
/// (a2), int style (a3), const MeterSeg* seg (t0: {float scale; int
/// count}), const float colour[4] (t1), int flags (t2), float rect_out[4]
/// or null (t3), float fill (f12), float sx (f13), float sy (f14))`
/// (0x21d1a0): draws a vertical meter (the boost bar) at a HUD layout
/// element. The element's position, size and scale (sy also times
/// `seg->scale`) are aligned on screen; the colour (times 128, as ints)
/// goes into the four vertices and the element's blend byte into the
/// render state. A full bar (`fill == 1`) is drawn bottom to top: the
/// bottom cap (sprite 2, 23 units high, its bottom at the element's
/// bottom), `count` segments (sprite 1, 23 units each) and the top cap
/// (sprite 0, 31 units). A partial bar fills from the bottom up to
/// `y + h * (1 - fill)`: the same pieces, the one that crosses the fill line
/// clipped (its top edge and top `v` moved down) and the drawing stopped
/// there. The drawn rectangle goes to `rect_out` when given. Frame 0x1c0
/// (s0-s7, f20-f22).
pub fn hud_draw_meter_bar(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    use meter_bar::*;
    guest(c, |c| {
        c.cycles += 82;
        let frame = Frame::open(c, 0x1c0);
        for (reg, off) in [
            (S0, 0x190),
            (S1, 0x180),
            (S2, 0x170),
            (S3, 0x160),
            (S4, 0x150),
            (S5, 0x140),
            (S6, 0x130),
            (S7, 0x120),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x110);
        for (f, off) in [(22, 0x1b0), (21, 0x1a8), (20, 0x1a0)] {
            frame.save_fpr(c, f, off);
        }
        // The element (elems + elem * 0x24) and the style's sprite triple
        // (hud + 0x4fc + style * 12).
        c.g[V0][0] = 0xc;
        c.mult(0, A3, A3, V0);
        c.g[S0][0] = c.g[A2][0] as u8 as i8 as i64 as u64;
        c.g[V0][0] = 0x24;
        c.mult(0, V1, S0, V0);
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(SPRITES));
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[T2][0];
        c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[S6][0] = c.g[T0][0];
        c.g[S7][0] = sx(r32(c, S2).wrapping_add(r32(c, A3)));
        c.g[S5][0] = c.g[T1][0];
        c.g[S4][0] = c.g[T3][0];
        c.f[22] = c.f[12];
        c.f[21] = c.f[14];
        c.f[20] = c.f[13];
        let elem = r32(c, S0);
        let sprites = r32(c, S7);
        let seg = r32(c, S6);
        let colour = r32(c, S5);
        let rect_out = c.g[S4][0];

        // HUD_GetElemPos(elem, pos); HUD_GetElemSize(elem, size, null);
        // HUD_GetElemScale(elem, scale).
        c.g[A1][0] = sx(sp(c).wrapping_add(POS));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001e_91f8, 0x0021_d224)?;
        c.g[S1][0] = sx(sp(c).wrapping_add(W));
        c.g[A2][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001e_9220, 0x0021_d238)?;
        c.g[A1][0] = sx(sp(c).wrapping_add(SCALE));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001e_9290, 0x0021_d244)?;

        // Scale by (sx, sy) and the segment scale; align with scale {1, 1}.
        let scale_x = mb_ld(c, SCALE) * mb_f(c, 20);
        let scale_y = mb_ld(c, SY) * mb_f(c, 21);
        let seg_scale = crate::guest::Addr(seg).read_fpu(c);
        let w = mb_ld(c, W) * scale_x;
        let h = mb_ld(c, H) * scale_y;
        let sy = scale_y * seg_scale;
        mb_set(c, 2, scale_x);
        mb_set(c, 0, scale_y);
        mb_set(c, 1, sy);
        mb_set(c, 3, w);
        mb_set(c, 4, h);
        c.g[AT][0] = 0x3f80_0000;
        mb_set(c, 5, FpuF32::ONE);
        mb_st(c, SCALE, scale_x);
        mb_st(c, SY, sy);
        mb_st(c, W, w);
        mb_st(c, H, h);
        mb_st(c, UNIT_SCALE, FpuF32::ONE);
        mb_st(c, UNIT_SCALE + 4, FpuF32::ONE);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(sp(c).wrapping_add(X));
        c.g[A2][0] = sx(sp(c).wrapping_add(POS));
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = sx(sp(c).wrapping_add(UNIT_SCALE));
        c.g[T1][0] = crate::guest::Addr(elem.wrapping_add(ELEM_HALIGN)).lb(c);
        c.g[T2][0] = crate::guest::Addr(elem.wrapping_add(ELEM_VALIGN)).lb(c);
        c.g[T3][0] = c.g[S3][0];
        call_ptr(c, 0x001f_10f8, 0x0021_d2b0)?;

        // The element's blend byte into bits 5..9 of the render state.
        let renderer = mb_renderer(c);
        c.g[V0][0] = renderer;
        c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
        let blend = crate::guest::Addr(elem.wrapping_add(ELEM_BLEND)).lb(c);
        c.g[S3][0] = sx(sp(c).wrapping_add(QUAD));
        let state = crate::guest::Addr((renderer as u32).wrapping_add(STATE_TOP)).lw(c);
        c.g[A0][0] = state;
        c.g[A3][0] = u64::MAX;
        c.g[V1][0] = sx((blend as u32) << 5) & 0x3e0;
        c.g[A2][0] = 3;
        let flags_at = crate::guest::Addr((state as u32).wrapping_add(STATE_FLAGS));
        let flags = (flags_at.lw(c) & c.g[A1][0]) | c.g[V1][0];
        c.g[V0][0] = flags;
        flags_at.sw(c, flags);

        // A countdown with no effect (a2 = 3 .. -1).
        loop {
            c.cycles += 7;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.g[A0][0] = sx(sp(c).wrapping_add(QUAD + V_X));
            if c.g[A2][0] != c.g[A3][0] {
                if event_due(c) {
                    deliver_event_here(c, 0x0021_d2e8);
                }
                continue;
            }
            break;
        }

        // The colour as ints (times 128): t1, a3, a2, a1 = c[2], c[0],
        // c[1], c[3].
        c.cycles += 23;
        c.g[AT][0] = 0x4300_0000;
        let k = FpuF32(0x4300_0000);
        let col = crate::guest::Addr(colour);
        let c2 = col.off(8).read_fpu(c) * k;
        let c3 = col.off(12).read_fpu(c) * k;
        let c0 = col.read_fpu(c) * k;
        let c1 = col.off(4).read_fpu(c) * k;
        mb_set(c, 2, c1);
        mb_set(c, 3, c0);
        mb_set(c, 4, c3);
        c.g[V1][0] = c.g[S3][0];
        c.g[T0][0] = sx(r32(c, S3).wrapping_add(4 * VERTEX));
        c.g[AT][0] = 0x3f80_0000;
        mb_set(c, 5, FpuF32::ONE);
        c.g[T1][0] = sx(c2.to_i32() as u32);
        c.g[A3][0] = sx(c0.to_i32() as u32);
        c.g[A2][0] = sx(c1.to_i32() as u32);
        c.f[1] = c1.to_i32() as u32;
        c.g[A1][0] = sx(c3.to_i32() as u32);
        c.f[0] = c3.to_i32() as u32;

        // The four vertices' constant fields.
        loop {
            c.cycles += 11;
            let v = crate::guest::Addr(r32(c, V1));
            v.off(V_COLOR + 0xc).sw(c, c.g[A3][0]);
            v.off(V_COLOR).sw(c, c.g[A2][0]);
            v.off(V_COLOR + 4).sw(c, c.g[T1][0]);
            v.off(V_COLOR + 8).sw(c, c.g[A1][0]);
            v.off(V_Z).write_fpu(c, mb_f(c, 5));
            let tail = crate::guest::Addr(r32(c, A0));
            tail.off(V_ZERO - V_X).sw(c, 0);
            tail.off(V_ONE - V_X).write_fpu(c, mb_f(c, 5));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(VERTEX));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T0][0] as i64)) as u64;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(VERTEX));
            if c.g[V0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0021_d360);
                }
                continue;
            }
            break;
        }

        c.cycles += 6;
        c.g[AT][0] = 0x3f80_0000;
        mb_set(c, 0, FpuF32::ONE);
        let full = mb_f(c, 22).eq_s(FpuF32::ONE);
        crate::abi::set_fcc(c, full);
        if full {
            // ---- Full bar: bottom cap, segments, top cap, bottom to top.
            c.cycles += 2;
            let t = mb_ld(c, X);
            mb_set(c, 1, t);
            if rect_out != 0 {
                c.cycles += 7;
                let (x, y, w, h) = (mb_f(c, 1), mb_ld(c, Y), mb_ld(c, W), mb_ld(c, H));
                mb_set(c, 2, y);
                mb_set(c, 3, w);
                mb_set(c, 0, h);
                let out = crate::guest::Addr(rect_out as u32);
                out.write_fpu(c, x);
                out.off(12).write_fpu(c, h);
                out.off(4).write_fpu(c, y);
                out.off(8).write_fpu(c, w);
            }

            // Bottom cap (sprite 2): from the element's bottom up 23 * sy.
            c.cycles += 80;
            let tex = crate::guest::Addr(sprites.wrapping_add(8)).lw(c);
            let bottom = mb_ld(c, Y) + mb_ld(c, H);
            mb_st(c, Y, bottom);
            c.g[AT][0] = SEGMENT_H as u64;
            mb_set(c, 21, FpuF32(SEGMENT_H));
            c.g[S0][0] = 0;
            let renderer = mb_renderer(c);
            c.g[T0][0] = renderer;
            mb_bind(c, renderer, tex, V1, V0);
            let piece = mb_ld(c, SY) * mb_f(c, 21);
            mb_set(c, 20, piece);
            let x = mb_ld(c, X);
            let x1 = x + mb_ld(c, W);
            let top = bottom - piece;
            mb_set(c, 3, x);
            mb_set(c, 4, bottom);
            mb_set(c, 2, x1);
            mb_set(c, 5, top);
            let v_top = mb_tex(c, tex, TEX_V0);
            mb_put_uvs(c, tex, v_top);
            mb_put_xy(c, x, x1, top, bottom);
            mb_draw_quad(c, 0x0021_d498)?;

            // Segments (sprite 1), `count` of them.
            let y = mb_ld(c, Y) - mb_f(c, 20);
            mb_set(c, 0, y);
            mb_st(c, Y, y);
            let tex = crate::guest::Addr(sprites.wrapping_add(4)).lw(c);
            c.g[V1][0] = tex;
            let renderer = mb_renderer(c);
            c.g[A0][0] = renderer;
            mb_bind(c, renderer, tex, A1, V0);
            let t = mb_ld(c, SY);
            mb_set(c, 1, t);
            mb_set(c, 20, mb_f(c, 1) * mb_f(c, 21));
            c.g[V0][0] = crate::guest::Addr(seg.wrapping_add(4)).lw(c);
            let v_top = mb_tex(c, tex, TEX_V0);
            mb_put_uvs(c, tex, v_top);
            if (c.g[V0][0] as i64) > 0 {
                loop {
                    c.cycles += 29;
                    let bottom = mb_ld(c, Y);
                    let x = mb_ld(c, X);
                    let x1 = x + mb_ld(c, W);
                    let top = bottom - mb_f(c, 20);
                    mb_set(c, 2, bottom);
                    mb_set(c, 1, x);
                    mb_set(c, 0, x1);
                    mb_set(c, 3, top);
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    mb_put_xy(c, x, x1, top, bottom);
                    mb_draw_quad(c, 0x0021_d564)?;
                    let y = mb_ld(c, Y) - mb_f(c, 20);
                    mb_set(c, 0, y);
                    let count = crate::guest::Addr(seg.wrapping_add(4)).lw(c);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (count as i64)) as u64;
                    mb_st(c, Y, y);
                    if c.g[V0][0] != 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0021_d508);
                        }
                        continue;
                    }
                    break;
                }
            }

            // Top cap (sprite 0): 31 * sy high.
            c.cycles += 49;
            let tex = crate::guest::Addr(sprites).lw(c);
            c.g[V1][0] = tex;
            let renderer = mb_renderer(c);
            c.g[A0][0] = renderer;
            mb_bind(c, renderer, tex, V0, T0);
            c.g[AT][0] = TOP_CAP_H as u64;
            let piece = mb_ld(c, SY) * FpuF32(TOP_CAP_H);
            let x = mb_ld(c, X);
            let bottom = mb_ld(c, Y);
            let x1 = x + mb_ld(c, W);
            let top = bottom - piece;
            mb_set(c, 4, x);
            mb_set(c, 5, bottom);
            mb_set(c, 2, top);
            mb_set(c, 3, x1);
            let v_top = mb_tex(c, tex, TEX_V0);
            mb_put_uvs(c, tex, v_top);
            mb_put_xy(c, x, x1, top, bottom);
            mb_draw_quad(c, 0x0021_d638)?;
            frame.restore(c, S0, 0x190);
        } else {
            // ---- Partial bar: up to the fill line y + h * (1 - fill) (f21).
            mb_set(c, 0, FpuF32::ONE - mb_f(c, 22));
            c.cycles += 6;
            let h = mb_ld(c, H);
            let y = mb_ld(c, Y);
            mb_set(c, 1, h);
            mb_set(c, 2, y);
            let empty = h * mb_f(c, 0);
            mb_set(c, 0, empty);
            let bottom = y + h;
            mb_set(c, 4, bottom);
            let fill = y + empty;
            mb_set(c, 21, fill);
            if rect_out != 0 {
                c.cycles += 7;
                let filled = bottom - fill;
                let (x, w) = (mb_ld(c, X), mb_ld(c, W));
                mb_set(c, 0, filled);
                mb_set(c, 1, x);
                mb_set(c, 2, w);
                let out = crate::guest::Addr(rect_out as u32);
                out.write_fpu(c, x);
                out.off(12).write_fpu(c, filled);
                out.off(8).write_fpu(c, w);
                out.off(4).write_fpu(c, fill);
            }

            // Bottom cap (sprite 2), clipped at the fill line.
            c.cycles += 11;
            let sy = mb_ld(c, SY);
            mb_set(c, 0, sy);
            c.g[S0][0] = 1;
            c.g[AT][0] = SEGMENT_H as u64;
            mb_set(c, 22, FpuF32(SEGMENT_H));
            let tex = crate::guest::Addr(sprites.wrapping_add(8)).lw(c);
            c.g[T1][0] = tex;
            let piece = sy * mb_f(c, 22);
            mb_set(c, 3, piece);
            let top = mb_f(c, 4) - piece;
            mb_set(c, 20, top);
            let clipped = top.le(mb_f(c, 21));
            crate::abi::set_fcc(c, clipped);
            let t = mb_tex(c, tex, TEX_V0);
            mb_set(c, 2, t);
            if !clipped {
                c.cycles += 1;
                c.g[S0][0] = 0;
            }
            c.cycles += 2;
            c.g[V1][0] = mb_renderer(c);
            if clipped {
                mb_clip_cap(c, tex, 20);
            }
            c.cycles += 39;
            mb_draw_partial_cap(c, tex, 20, 0x0021_d75c)?;
            c.f[4] = c.f[20];
            if !clipped {
                // Segments (sprite 1) until one crosses the fill line.
                c.cycles += 26;
                let tex = crate::guest::Addr(sprites.wrapping_add(4)).lw(c);
                c.g[S0][0] = tex;
                c.g[S2][0] = 0;
                let renderer = mb_renderer(c);
                c.g[V0][0] = renderer;
                mb_bind(c, renderer, tex, V1, A0);
                let sy = mb_ld(c, SY);
                mb_set(c, 0, sy);
                mb_set(c, 22, sy * mb_f(c, 22));
                let v_top = mb_tex(c, tex, TEX_V0);
                mb_put_uvs(c, tex, v_top);
                c.g[V0][0] = crate::guest::Addr(seg.wrapping_add(4)).lw(c);
                let mut crossed = false;
                if (c.g[V0][0] as i64) > 0 {
                    c.cycles += 1;
                    mb_set(c, 20, mb_f(c, 4) - mb_f(c, 22));
                    loop {
                        c.cycles += 4;
                        let clipped = mb_f(c, 20).le(mb_f(c, 21));
                        crate::abi::set_fcc(c, clipped);
                        c.g[S1][0] = 1;
                        if !clipped {
                            c.cycles += 1;
                            c.g[S1][0] = 0;
                        }
                        c.cycles += 2;
                        let t = mb_ld(c, X);
                        mb_set(c, 1, t);
                        if clipped {
                            c.cycles += 11;
                            let tex = c.g[S0][0];
                            let v0 = mb_tex(c, tex, TEX_V0);
                            let fill = mb_f(c, 21);
                            let cut = fill - mb_f(c, 20);
                            mb_set(c, 2, cut);
                            mb_set(c, 20, fill);
                            let dv = ((mb_tex(c, tex, TEX_V1) - v0) * cut).div(mb_f(c, 22));
                            let v_top = v0 + dv;
                            mb_set(c, 0, v_top);
                            mb_st(c, QUAD + VERTEX + V_V, v_top);
                            mb_st(c, QUAD + V_V, v_top);
                            let t = mb_ld(c, X);
                            mb_set(c, 1, t);
                        }
                        c.cycles += 21;
                        let x = mb_f(c, 1);
                        let x1 = x + mb_ld(c, W);
                        mb_set(c, 0, x1);
                        let (top, bottom) = (mb_f(c, 20), mb_f(c, 4));
                        mb_put_xy(c, x, x1, top, bottom);
                        mb_draw_quad(c, 0x0021_d864)?;
                        c.f[4] = c.f[20];
                        if c.g[S1][0] != 0 {
                            crossed = true;
                            break;
                        }
                        c.cycles += 5;
                        let count = crate::guest::Addr(seg.wrapping_add(4)).lw(c);
                        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (count as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            mb_set(c, 20, mb_f(c, 4) - mb_f(c, 22));
                            if event_due(c) {
                                deliver_event_here(c, 0x0021_d7d0);
                            }
                            continue;
                        }
                        break;
                    }
                }
                if !crossed {
                    // Top cap (sprite 0), clipped at the fill line.
                    c.cycles += 11;
                    let sy = mb_ld(c, SY);
                    mb_set(c, 1, sy);
                    c.g[V0][0] = 1;
                    c.g[AT][0] = TOP_CAP_H as u64;
                    mb_set(c, 0, FpuF32(TOP_CAP_H));
                    let tex = crate::guest::Addr(sprites).lw(c);
                    c.g[S2][0] = tex;
                    let piece = sy * FpuF32(TOP_CAP_H);
                    mb_set(c, 3, piece);
                    let top = mb_f(c, 4) - piece;
                    mb_set(c, 5, top);
                    let clipped = top.le(mb_f(c, 21));
                    crate::abi::set_fcc(c, clipped);
                    let t = mb_tex(c, tex, TEX_V0);
                    mb_set(c, 2, t);
                    if !clipped {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                    c.cycles += 2;
                    c.g[V1][0] = mb_renderer(c);
                    if clipped {
                        mb_clip_cap(c, tex, 5);
                    }
                    c.cycles += 37;
                    mb_draw_partial_cap(c, tex, 5, 0x0021_d968)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x190);
        }

        c.cycles += 13;
        for (reg, off) in [
            (S1, 0x180),
            (S2, 0x170),
            (S3, 0x160),
            (S4, 0x150),
            (S5, 0x140),
            (S6, 0x130),
            (S7, 0x120),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x110);
        for (f, off) in [(22, 0x1b0), (21, 0x1a8), (20, 0x1a0)] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod rail_rider {

    /// Velocity (`vec4`).
    pub const VELOCITY: u32 = 0x1e0;

    /// The rider's path (`+0x2c`: nodes, 32 bytes each).
    pub const PATH: u32 = 0x780;

    /// The animation controller.
    pub const ANIM: u32 = 0x784;

    /// The overlap list of objects near the rider.
    pub const OVERLAP: u32 = 0x860;

    /// Current path node index.
    pub const NODE: u32 = 0x8a0;

    /// Offset from the path node to the rider's probe point (`vec4`).
    pub const PROBE_OFFSET: u32 = 0x9d0;
}

// ---------------------------------------------------------------------------
// Rider_FindRail (0x108a48, seg_1218.cpp).

/// `Rider_GetMainState(rider)`.
pub(crate) const RIDER_GET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe98,
    f: ee::s00::Rider_GetMainState,
};

/// `Rider_ProbeRail(rider, pos, pos + 0x10)`.
pub(crate) const RIDER_PROBE_RAIL: Callee = Callee {
    addr: 0x0010_86b8,
    f: ee::s00::Rider_ProbeRail,
};

/// `OverlapList_FindNearestRail(list, pos, out, mask, radius)` (cross-shard).
pub(crate) const OVERLAP_LIST_FIND_NEAREST_RAIL: u32 = 0x0033_4680;

/// `AnimController_GetLayerAnimEntry(ctrl, layer)` (cross-shard).
pub(crate) const ANIM_GET_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;

/// `AnimController_GetLayerSequence(ctrl, layer)` (cross-shard).
pub(crate) const ANIM_GET_LAYER_SEQUENCE: u32 = 0x0031_1b20;

/// `Flags64_Test(flags, bit)` (cross-shard).
pub(crate) const FLAGS64_TEST: u32 = 0x0014_46a0;

/// `int Rider_FindRail(Rider* self, sVec4 out[2])` (0x108a48): in the main
/// states 0/1, look for a rail near the rider's probe point (path node +
/// offset, radius 300). Rejects a rail behind the rider (closer than -0.2 x
/// speed along the velocity) and, when the animation on layer 2 is a grab
/// (kind 0x12) or a trick (0x13/0x14), a rail the sequence's flags rule out;
/// otherwise returns `Rider_ProbeRail(self, out, out + 1)`. Returns 0 when
/// there is none.
pub fn rider_find_rail(c: &mut Cpu) {
    use rail_rider::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        let me = c.g[S0][0] as u32;
        let lw = |c: &mut Cpu, a: u32| c.lw(a);
        RIDER_GET_MAIN_STATE.call(c, 0x0010_8a64)?;
        'out: {
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                RIDER_GET_MAIN_STATE.call(c, 0x0010_8a74)?;
                c.g[V1][0] = 1;
                let other = c.g[V0][0] != 1;
                c.g[V0][0] = 0;
                if other {
                    break 'out;
                }
                c.cycles += 1;
            }
            c.g[A0][0] = lw(c, me.wrapping_add(PATH));

            // 0x108a84: probe point = path node + offset; nearest rail.
            c.cycles += 19;
            c.g[A1][0] = c.g[SP][0];
            c.g[V0][0] = lw(c, me.wrapping_add(NODE));
            c.g[A2][0] = c.g[S1][0];
            c.g[V1][0] = lw(c, (c.g[A0][0] as u32).wrapping_add(0x2c));
            c.g[A3][0] = 1;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
            c.g[AT][0] = 0x4396_0000; // 300.0
            c.mtc1(1, 12);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lqc2(3, c.g[V1][0] as u32);
            c.lqc2(4, me.wrapping_add(PROBE_OFFSET));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp(c).wrapping_add(0x10));
            c.lq(2, sp(c).wrapping_add(0x10));
            c.g[A0][0] = lw(c, me.wrapping_add(OVERLAP));
            c.sq(2, sp(c));
            call_ptr(c, OVERLAP_LIST_FIND_NEAREST_RAIL, 0x0010_8ac8)?;
            let none = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if none {
                break 'out;
            }

            // 0x108ad0: |velocity| (VU0 length).
            c.cycles += 15;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7e38));
            c.lqc2(3, me.wrapping_add(VELOCITY));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 2);
            c.c_lt_s(0, 2);
            if c.fcc() {
                // 0x108b0c: moving: reject a rail behind the rider.
                c.cycles += 21;
                let out = c.g[S1][0] as u32;
                c.lqc2(3, out);
                c.lqc2(4, sp(c));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x20));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7e34));
                c.lq(2, sp(c).wrapping_add(0x20));
                c.mul_s(1, 2, 1);
                c.sq(2, sp(c).wrapping_add(0x10));
                c.lqc2(3, sp(c).wrapping_add(0x10));
                c.lqc2(5, me.wrapping_add(VELOCITY));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(3, 4);
                c.mtc1(3, 0);
                c.c_lt_s(0, 1);
                let behind = c.fcc();
                c.g[V0][0] = 0;
                if behind {
                    break 'out;
                }
                c.cycles += 1;
            }
            c.g[A0][0] = lw(c, me.wrapping_add(ANIM));

            // 0x108b64: the animation on layer 2 may forbid the rail.
            c.cycles += 5;
            c.g[A1][0] = 2;
            call_ptr(c, ANIM_GET_LAYER_ANIM_ENTRY, 0x0010_8b6c)?;
            c.g[V1][0] = 0x12;
            let grab = c.g[V0][0] != 0x12;
            c.g[A0][0] = lw(c, me.wrapping_add(ANIM));
            let probe;
            if !grab {
                // Kind 0x12: allowed when flag 2 is set or flag 0 is clear.
                c.cycles += 7;
                c.g[A1][0] = 2;
                call_ptr(c, ANIM_GET_LAYER_SEQUENCE, 0x0010_8b80)?;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                c.g[A1][0] = 2;
                call_ptr(c, FLAGS64_TEST, 0x0010_8b8c)?;
                let flag2 = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if flag2 {
                    probe = true;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = lw(c, me.wrapping_add(ANIM));
                    c.g[A1][0] = 2;
                    call_ptr(c, ANIM_GET_LAYER_SEQUENCE, 0x0010_8ba0)?;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = 0;
                    call_ptr(c, FLAGS64_TEST, 0x0010_8bac)?;
                    probe = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                }
            } else {
                // 0x108bbc: kinds 0x13/0x14 need flag 2.
                c.cycles += 5;
                c.g[A1][0] = 2;
                call_ptr(c, ANIM_GET_LAYER_ANIM_ENTRY, 0x0010_8bc4)?;
                c.g[V1][0] = 0x13;
                let is_13 = c.g[V0][0] == 0x13;
                c.g[A0][0] = lw(c, me.wrapping_add(ANIM));
                let mut check = true;
                if !is_13 {
                    c.cycles += 5;
                    c.g[A1][0] = 2;
                    call_ptr(c, ANIM_GET_LAYER_ANIM_ENTRY, 0x0010_8bd8)?;
                    c.g[V1][0] = 0x14;
                    check = c.g[V0][0] == 0x14;
                    c.g[A1][0] = c.g[S1][0];
                    if check {
                        c.cycles += 1;
                        c.g[A0][0] = lw(c, me.wrapping_add(ANIM));
                    }
                }
                if check {
                    // 0x108be8
                    c.cycles += 7;
                    c.g[A1][0] = 2;
                    call_ptr(c, ANIM_GET_LAYER_SEQUENCE, 0x0010_8bf0)?;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = 2;
                    call_ptr(c, FLAGS64_TEST, 0x0010_8bfc)?;
                    probe = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[S1][0];
                    // The branch back to 0x108bb4 is a back edge: an event
                    // may be due there (and may unwind; re-entry at 0x108bb4
                    // runs the generated code).
                    if !probe && crate::events::back_edge_unwinds(c, 0x0010_8bb4) {
                        return Err(crate::call::Unwound);
                    }
                } else {
                    probe = true;
                }
            }
            if probe {
                // 0x108c04
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                RIDER_PROBE_RAIL.call(c, 0x0010_8c10)?;
            } else {
                // 0x108bb4
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        // 0x108c10
        c.cycles += 5;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// HUD_AlignX (0x21e750, ovstatehud.cpp).

/// The HUD's virtual screen width (640.0).
pub(crate) const HUD_SCREEN_WIDTH: u32 = 0x4420_0000;

/// 0.5.
pub(crate) const HALF: u32 = 0x3f00_0000;

/// `float HUD_AlignX(float x, int align, int flags, float width, float
/// scale)` (0x21e750): the left edge of an element of `width * scale` at
/// `x`. Flag 1 mirrors the layout (`x = 640 - x`, and the alignment flips
/// unless flag 2 is set); alignment 0 centres the element on `x`, positive
/// alignment right-aligns it, negative leaves `x` as the left edge.
pub fn hud_align_x(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    c.cycles += 3;
    let align = c.g[A1][0];
    let flags = c.g[A2][0];
    let mut x = FpuF32(c.f[12]);
    c.g[V0][0] = flags & 1;
    c.g[V1][0] = align;
    if flags & 1 != 0 {
        c.cycles += 6;
        c.g[AT][0] = HUD_SCREEN_WIDTH as u64;
        c.f[0] = HUD_SCREEN_WIDTH;
        c.g[V0][0] = flags & 2;
        c.g[V1][0] = sx(0u32.wrapping_sub(align as u32));
        x = FpuF32(HUD_SCREEN_WIDTH) - x;
        if flags & 2 != 0 {
            c.g[V1][0] = align;
        }
    }
    c.cycles += 2;
    let v = c.g[V1][0];
    let size = FpuF32(c.f[14]) * FpuF32(c.f[13]);
    let shift = if v == 0 {
        // Centred: half the size.
        c.cycles += 5;
        c.g[AT][0] = HALF as u64;
        c.f[1] = HALF;
        Some(size * FpuF32(HALF))
    } else {
        c.cycles += 2;
        if (v as i64) > 0 {
            c.cycles += 1;
            Some(size)
        } else {
            None
        }
    };
    if let Some(shift) = shift {
        c.cycles += 1;
        c.f[0] = shift.0;
        x = x - shift;
    }
    c.cycles += 2;
    c.f[12] = x.0;
    c.f[0] = x.0;
    ret(c);
}

pub(crate) mod rubber_band {}

pub(crate) mod target {

    /// The chosen target rider (-1: none).
    pub const TARGET: u32 = 0xe70;

    /// The actor's AI view, starting with the table of the six riders
    /// (0x24 each: `{ Rider* rider; ?; float distance; float heading; ...
    /// }`).
    pub const TABLE: u32 = 0x18;

    pub const ENTRY_SIZE: u32 = 0x24;

    pub const ENTRY_DIST: u32 = 0x8;

    pub const ENTRY_HEADING: u32 = 0xc;

    /// Also in the AI view: the actor's velocity and its distance to the
    /// finish.
    pub const RIDER_VELOCITY: u32 = 0x1b0;

    pub const RIDER_FINISH_DIST: u32 = 0x4c8;
}

// ---------------------------------------------------------------------------
// WPData_FindCellIndex (0x2baf90)

pub(crate) const QUAD_TREE_FIND_LEAF: Callee = Callee {
    addr: 0x002c_1cd8,
    f: ee::s08::QuadTree_FindLeaf,
};

/// `int WPData_FindCellIndex(WPData* wp, float x, float z)` (0x2baf90): the
/// world-painter cell at (x, z): the leaf of the data's quadtree
/// (`QuadTree_FindLeaf(wp->tree, x, z)`) and its cell index (`leaf + 4`).
pub fn wp_data_find_cell_index(c: &mut Cpu) {
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, A0)).lw(c);
        QUAD_TREE_FIND_LEAF.call(c, 0x002b_afa0)?;
        c.g[V0][0] = Addr(r32(c, V0)).off(4).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// -------------------------------------------------- RiderBase_RenderAttackFX

/// `void RiderBase_RenderAttackFX(AttackFX* this)` (0x2f1510): while the
/// attack effect is active (`this+0xa0`), draw its four sprites. Pushes a
/// GS state record (copies the current 20-byte record in the graphics
/// manager's packet, `gfx+0xe84`, `gfx = gp-0x854`), loads the template at
/// 0x501420 into it, sets the frame buffer (`gfx+0xfb0`) and patches its
/// blend/test fields; then per sprite `i` the display's `DrawSprite`
/// (virtual +0x270/+0x274) with position `this+0x10+16i`, size `(s, s)`
/// (`s = this+0x90+4i`), uv `(0, 0)..(1, 1)` and colour `this+0x50+16i`;
/// finally pops the record.
///
/// The register use (the mask constants in a2..t7, the GPR temporaries of
/// the record copies) and the blocks follow the original's.
pub fn rider_base_render_attack_fx(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0xb0);
        'b_2f173c: {
            c.cycles += 13;
            c.g[T8][0] = c.g[A0][0];
            frame.save(c, S0, 0x90);
            frame.save(c, S1, 0x80);
            frame.save(c, S2, 0x70);
            frame.save(c, S3, 0x60);
            frame.save(c, S4, 0x50);
            frame.save(c, S5, 0x40);
            frame.save_ra(c, 0x30);
            frame.save_fpr(c, 20, 0xa0);
            c.g[V0][0] = c.lw(at(c, T8, 0xa0));
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = c.lw(gpo(c, -0x854));
            if t {
                break 'b_2f173c;
            } else {
                c.cycles += 99;
                c.g[A1][0] = 0x500000;
                c.g[A2][0] = 0xfffffffffe7f0000;
                c.g[T0][0] = 0xffffffffffcf0000;
                c.g[V0][0] = c.lw(at(c, V1, 0xe84));
                c.g[A2][0] = c.g[A2][0] | 0xffff;
                c.g[T0][0] = c.g[T0][0] | 0xffff;
                c.g[T1][0] = 0xfffffffffff00000;
                c.ldl(4, at(c, V0, 0x7));
                c.ldr(4, c.g[V0][0] as u32);
                c.ldl(7, at(c, V0, 0xf));
                c.ldr(7, at(c, V0, 0x8));
                c.g[T2][0] = c.lw(at(c, V0, 0x10));
                c.sdl(4, at(c, V0, 0x1b));
                c.sdr(4, at(c, V0, 0x14));
                c.sdl(7, at(c, V0, 0x23));
                c.sdr(7, at(c, V0, 0x1c));
                c.sw(at(c, V0, 0x24), c.g[T2][0]);
                c.g[T1][0] = c.g[T1][0] | 0xfff;
                c.g[A3][0] = 0x10000;
                c.g[T4][0] = 0xfffffffffffffffc;
                c.g[V0][0] = c.lw(at(c, V1, 0xe84));
                c.g[A3][0] = c.g[A3][0] | 0x4000;
                c.g[A0][0] = c.lw(gpo(c, -0x854));
                c.g[T2][0] = 0xffffffffe0000000;
                c.g[V0][0] = sx(at(c, V0, 0x14));
                c.g[T2][0] = c.g[T2][0] | 0x3ff;
                c.sw(at(c, V1, 0xe84), c.g[V0][0]);
                c.g[T5][0] = 0xfffffffffffffc1f;
                c.g[T6][0] = 0xffffffffffffff83;
                c.g[T3][0] = 0xffffffffffbf0000;
                c.g[V0][0] = c.lw(at(c, A0, 0xe84));
                c.g[T3][0] = c.g[T3][0] | 0xffff;
                c.g[T7][0] = 0x400000;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.g[A0][0] = sx(at(c, A1, 0x1420));
                c.ldl(16, at(c, A0, 0x7));
                c.ldr(16, c.g[A0][0] as u32);
                c.ldl(17, at(c, A0, 0xf));
                c.ldr(17, at(c, A0, 0x8));
                c.g[V1][0] = c.lw(at(c, A0, 0x10));
                c.sdl(16, at(c, V0, 0x7));
                c.sdr(16, c.g[V0][0] as u32);
                c.sdl(17, at(c, V0, 0xf));
                c.sdr(17, at(c, V0, 0x8));
                c.sw(at(c, V0, 0x10), c.g[V1][0]);
                c.g[S2][0] = sx(at(c, T8, 0x10));
                c.g[S5][0] = sx(stk(c, 0x10));
                c.g[S4][0] = sx(stk(c, 0x20));
                c.g[A0][0] = c.lw(gpo(c, -0x854));
                c.g[S1][0] = sx(at(c, T8, 0x50));
                c.g[S0][0] = sx(at(c, T8, 0x90));
                c.g[S3][0] = 0x3;
                c.g[V1][0] = c.lw(at(c, A0, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0xfb0));
                c.sh(at(c, V1, 0x10), c.g[V0][0]);
                c.g[A1][0] = c.lw(gpo(c, -0x854));
                c.g[V1][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = c.g[V0][0] | 0xc;
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                c.g[A0][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.sw(at(c, A0, 0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, V1, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.sw(at(c, V1, 0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                c.sw(at(c, A0, 0x4), c.g[V0][0]);
                c.g[A2][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A2, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
                c.g[V0][0] = c.g[V0][0] | 0x2;
                c.sw(at(c, A2, 0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, V1, 0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.sw(at(c, V1, 0x8), c.g[V0][0]);
                c.g[A0][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
                c.g[V0][0] = c.g[V0][0] | 0xe0;
                c.sw(at(c, A0, 0x8), c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, V1, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T6][0];
                c.g[V0][0] = c.g[V0][0] | 0x1c;
                c.sw(at(c, V1, 0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T7][0];
                c.sw(at(c, A0, 0x4), c.g[V0][0]);
                c.g[V0][0] = c.lw(gpo(c, -0x854));
                'l_2f16d0: loop {
                    c.cycles += 23;
                    c.g[A1][0] = c.g[S2][0];
                    c.lwc1(0, c.g[S0][0] as u32);
                    c.g[T1][0] = c.g[S1][0];
                    c.g[V1][0] = c.lw(at(c, V0, 0x10d8));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[T0][0] = c.g[S4][0];
                    c.g[A0][0] = c.lh(at(c, V1, 0x270));
                    c.g[S1][0] = sx(at(c, S1, 0x10));
                    c.swc1(0, stk(c, 0x4));
                    c.g[S0][0] = sx(at(c, S0, 0x4));
                    c.swc1(0, stk(c, 0));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw(stk(c, 0x10), 0u64);
                    c.g[S2][0] = sx(at(c, S2, 0x10));
                    c.sw(stk(c, 0x14), 0u64);
                    c.swc1(20, stk(c, 0x20)); // uv max (1, 1)
                    c.swc1(20, stk(c, 0x24));
                    c.g[V0][0] = c.lw(at(c, V1, 0x274));
                    let t = c.g[V0][0] as u32;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    call_ptr(c, t, 0x002f1724)?; // display->DrawSprite
                    let t = (c.g[S3][0] as i64) >= 0;
                    c.g[V0][0] = c.lw(gpo(c, -0x854));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002f16d0);
                        }
                        continue 'l_2f16d0;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(gpo(c, -0x854));
                        c.g[V0][0] = c.lw(at(c, V1, 0xe84));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                        c.sw(at(c, V1, 0xe84), c.g[V0][0]);
                        break 'b_2f173c;
                    }
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider_SelectTargetRider

/// `int AI_GetFrameCounter()` (0x1298c8).
pub(crate) const AI_GET_FRAME_COUNTER: Callee = Callee {
    addr: 0x0012_98c8,
    f: ee::s00::AI_GetFrameCounter,
};

/// `bool ComputerActorRider_IsRivalTarget(actor, index)` (0x10d9e8).
pub(crate) const IS_RIVAL_TARGET: Callee = Callee {
    addr: 0x0010_d9e8,
    f: ee::s00::ComputerActorRider_IsRivalTarget,
};

/// `float atanf(float)` (0x31c228) and `float cosf(float)` (0x31c040).
pub(crate) const ATANF: u32 = 0x0031_c228;

pub(crate) const COSF: u32 = 0x0031_c040;

/// The target-rider score of table entry `s1`/index `s2` (f22 = its
/// distance): distance weighted by how far its heading is from the actor's
/// travel direction (`|cos|` + 1 for rivals, 1 - cos for others); the best
/// (smallest) score becomes the target and the rival flag goes to `*s4`.
pub(crate) fn score_target(c: &mut Cpu) -> Flow {
    use target::*;
    let actor = r32(c, S0);
    let gp = r32(c, GP);
    let s = sp(c);
    c.cycles += 15;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S2][0];
    IS_RIVAL_TARGET.call(c, 0x0010_be44)?;
    c.mtc1(0, 23);
    c.g[S3][0] = c.g[V0][0];
    c.g[V0][0] = c.lw(actor.wrapping_add(TABLE));
    let rider = r32(c, V0);
    c.lq(V1, rider.wrapping_add(RIDER_VELOCITY));
    c.sq(V1, s);
    c.lq(V1, rider.wrapping_add(RIDER_VELOCITY));
    c.lwc1(21, s.wrapping_add(0x4));
    c.sq(V1, s.wrapping_add(0x10));
    c.lwc1(20, s.wrapping_add(0x10));
    // f12 = the travel angle atan2-style from (x f20, y f21).
    c.c_eq_s(20, 23);
    let mut reload = true;
    if !c.fcc() {
        c.cycles += 7;
        c.div_s(12, 21, 20);
        call_ptr(c, ATANF, 0x0010_beac)?;
        c.c_lt_s(20, 23);
        let negative_x = c.fcc();
        c.mov_s(12, 0);
        if negative_x {
            c.cycles += 4;
            c.c_lt_s(23, 21);
            if !c.fcc() {
                c.sub_s(0, 0, 25);
            } else {
                c.cycles += 1;
                c.add_s(0, 0, 25);
            }
            c.cycles += 1;
            c.mov_s(12, 0);
        }
    } else {
        c.cycles += 4;
        c.c_eq_s(21, 23);
        let zero = c.fcc();
        c.mov_s(12, 21);
        if !zero {
            // +-pi/2.
            c.cycles += 4;
            c.c_le_s(23, 21);
            c.lwc1(12, gp.wrapping_sub(0x7df4));
            let up = c.fcc();
            c.g[V0][0] = c.lw(actor.wrapping_add(TABLE));
            if up {
                c.cycles += 3;
                c.lwc1(12, gp.wrapping_sub(0x7df0));
            }
            reload = false;
        }
    }
    if reload {
        c.cycles += 1;
        c.g[V0][0] = c.lw(actor.wrapping_add(TABLE));
    }
    // The heading difference wrapped to a turn: d - round(d / 2pi) * 2pi.
    c.cycles += 14;
    c.lwc1(1, gp.wrapping_sub(0x7dec));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 2);
    c.lwc1(0, r32(c, V0).wrapping_add(ENTRY_HEADING));
    c.sub_s(0, 0, 12);
    c.mul_s(1, 0, 1);
    c.add_s(1, 1, 2);
    c.cvt_w_s(2, 1);
    c.cvt_s_w(2, 2);
    c.c_lt_s(1, 2);
    if c.fcc() {
        c.sub_s(2, 2, 24);
    }
    c.cycles += 6;
    c.lwc1(12, gp.wrapping_sub(0x7de8));
    c.mul_s(12, 2, 12);
    c.sub_s(12, 0, 12);
    call_ptr(c, COSF, 0x0010_bf20)?;
    if c.g[S3][0] == 0 {
        c.sub_s(0, 24, 0);
    } else {
        c.cycles += 2;
        c.abs_s(0, 0);
        c.add_s(0, 0, 24);
    }
    c.cycles += 5;
    c.mul_s(22, 22, 0);
    c.c_le_s(22, 26);
    if c.fcc() {
        c.cycles += 4;
        c.sw(actor.wrapping_add(TARGET), c.g[S2][0]);
        c.mov_s(26, 22);
        c.sw(r32(c, S4), c.g[S3][0]);
    }
    Ok(())
}

/// `bool ComputerActorRider_SelectTargetRider(ComputerActor* actor, int*
/// rival)` (0x10bd10): keep the current target rider while it is within
/// range and the actor within 600 units of the finish; otherwise, every 12th
/// AI frame and while within 540 units of the finish, pick the best-scoring
/// rider within 700 units. Returns whether there is a target.
pub fn computer_actor_rider_select_target_rider(c: &mut Cpu) {
    use target::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0x70);
        frame.save(c, S4, 0x30);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save_ra(c, 0x20);
        for (f, off) in [
            (26, 0xb0),
            (25, 0xa8),
            (24, 0xa0),
            (23, 0x98),
            (22, 0x90),
            (21, 0x88),
            (20, 0x80),
        ] {
            frame.save_fpr(c, f, off);
        }
        c.g[S0][0] = c.g[A0][0];
        let actor = r32(c, S0);
        let gp = r32(c, GP);
        c.g[V1][0] = c.lw(actor.wrapping_add(TARGET));
        c.g[S4][0] = c.g[A1][0];
        'done: {
            if (c.g[V1][0] as i64) >= 0 {
                // Drop the target when it is beyond the range at gp-0x7e00
                // or the actor is more than 600 units from the finish.
                c.cycles += 10;
                c.g[V0][0] = ENTRY_SIZE as u64;
                c.g[A0][0] = c.lw(actor.wrapping_add(TABLE));
                c.mult(0, V0, V1, V0);
                c.lwc1(1, gp.wrapping_sub(0x7e00));
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.lwc1(0, r32(c, V0).wrapping_add(ENTRY_DIST));
                c.c_lt_s(1, 0);
                let mut drop = c.fcc();
                c.g[V0][0] = u64::MAX;
                if !drop {
                    c.cycles += 7;
                    c.lwc1(1, r32(c, A0).wrapping_add(RIDER_FINISH_DIST));
                    c.g[AT][0] = 0x4416_0000; // 600.0
                    c.mtc1(AT, 0);
                    c.c_lt_s(0, 1);
                    drop = c.fcc();
                }
                if drop {
                    c.cycles += 2;
                    c.sw(actor.wrapping_add(TARGET), c.g[V0][0]);
                }
                c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
                c.cycles += 2;
                if (c.g[V0][0] as i64) >= 0 {
                    c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
                    break 'done;
                }
            }
            // Choose only below 540 units to the finish, every 12th frame.
            c.cycles += 8;
            c.g[V0][0] = c.lw(actor.wrapping_add(TABLE));
            c.g[AT][0] = 0x4407_0000; // 540.0
            c.mtc1(AT, 1);
            c.lwc1(0, r32(c, V0).wrapping_add(RIDER_FINISH_DIST));
            c.c_lt_s(0, 1);
            if !c.fcc() {
                c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
                break 'done;
            }
            c.cycles += 6;
            AI_GET_FRAME_COUNTER.call(c, 0x0010_bdd4)?;
            c.g[V1][0] = 0xc;
            c.div(0, V0, V1);
            c.cycles += 3;
            c.g[A0][0] = c.hi[0];
            if c.g[A0][0] != 0 {
                c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
                break 'done;
            }
            c.cycles += 6;
            c.lwc1(26, gp.wrapping_sub(0x7dfc));
            c.g[S2][0] = 0;
            c.lwc1(25, gp.wrapping_sub(0x7df8));
            c.g[S1][0] = 0;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 24);
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.lw(actor.wrapping_add(TABLE));
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                let entry = r32(c, V0);
                c.g[V1][0] = c.lw(entry);
                if c.g[V1][0] != 0 {
                    c.cycles += 7;
                    c.g[AT][0] = 0x442f_0000; // 700.0
                    c.mtc1(AT, 0);
                    c.lwc1(22, entry.wrapping_add(ENTRY_DIST));
                    c.c_lt_s(0, 22);
                    if !c.fcc() {
                        score_target(c)?;
                    }
                }
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 6) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(ENTRY_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_be08);
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(actor.wrapping_add(TARGET));
        }
        // v0 = target >= 0.
        c.cycles += 17;
        c.g[V0][0] = !c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        for (f, off) in [
            (26, 0xb0),
            (25, 0xa8),
            (24, 0xa0),
            (23, 0x98),
            (22, 0x90),
            (21, 0x88),
            (20, 0x80),
        ] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod rider_lights {}

pub(crate) mod renderer {

    pub(crate) use crate::game::fe::ovstatehud::renderer::STATE;
    pub(crate) use crate::game::fe::ovstatehud::renderer::VTABLE;
}

pub(crate) mod lens_fx {

    /// Non-zero while the sun is on screen (halfword).
    pub const VISIBLE: u32 = 0x2c;

    /// The sun's screen position (x, y).
    pub const SUN_X: u32 = 0x1f0;

    pub const SUN_Y: u32 = 0x1f4;

    /// The flare intensity.
    pub const INTENSITY: u32 = 0x204;
}

pub(crate) mod sun_callee {
    use super::Callee;
    use ssx3_ee as ee;

    pub const RED: Callee = Callee {
        addr: 0x002e_e100,
        f: ee::s09::WorldPainter_GetSunRed,
    };

    pub const GREEN: Callee = Callee {
        addr: 0x002e_e148,
        f: ee::s09::WorldPainter_GetSunGreen,
    };

    pub const BLUE: Callee = Callee {
        addr: 0x002e_e190,
        f: ee::s09::WorldPainter_GetSunBlue,
    };

    pub const INTENSITY: Callee = Callee {
        addr: 0x002e_e1d8,
        f: ee::s09::WorldPainter_GetSunIntensity,
    };

    pub const SIZE: Callee = Callee {
        addr: 0x002e_e2b0,
        f: ee::s09::WorldPainter_GetSunSize,
    };

    pub const TEXTURE: Callee = Callee {
        addr: 0x002e_e220,
        f: ee::s09::WorldPainter_GetSunTexture,
    };

    pub const DRAW_FLARE_SET: Callee = Callee {
        addr: 0x002f_4690,
        f: ee::s09::LensFX_DrawFlareSet,
    };
}

pub(crate) mod path {

    /// `PathState*`: tick lengths at +0xc (normal) and +0x10 (streamed),
    /// the 24 track slots at +0x58.
    pub const STATE: u32 = 0x0044_f420;

    /// Mode byte: `'B'` = streamed (block) mode.
    pub const MODE: u32 = 0x0044_f434;

    /// Current PATH time.
    pub const TIME: u32 = 0x0044_f438;

    pub const TRACKS: u32 = 24;

    pub const SLOT0: u32 = 0x58;
}

pub(crate) mod track {

    pub const BUSY: u32 = 0x3;

    pub const VOLUME_FADE: u32 = 0xd;

    pub const PLAY_STATE: u32 = 0xe;

    pub const AUX_FADE: u32 = 0x15;

    pub const FADE3: u32 = 0x1d;

    pub const LEAD: u32 = 0x24;

    pub const STREAMED: u32 = 0x2f;

    pub const ACTIVE: u32 = 0x30;

    pub const NODE: u32 = 0x36;

    pub const DEADLINE: u32 = 0x48;

    pub const STOP_AT: u32 = 0x4c;

    pub const HANDLE: u32 = 0x54;
}

pub(crate) mod speedo {

    /// The HUD settings block copied to the stack (0x288 bytes, 8-aligned).
    pub const SETTINGS: u32 = 0x0053_5610;

    pub const SETTINGS_SIZE: u32 = 0x280;

    /// Settings word 0: bit 19 clear = the converted units (factor and
    /// second label).
    pub const UNITS_FLAG: u64 = 0x8_0000;

    /// The rider's velocity provider (an object with a vtable).
    pub const RIDER_VELOCITY: u32 = 0x6c0;

    pub const SLOT_GET_VELOCITY: u32 = 0x10;

    /// The speedometer's HUD element (element 25 of 0x24 bytes).
    pub const ELEM: u32 = 0x384;

    pub const ELEM_ALIGN_X: u32 = 0x3a4;

    pub const ELEM_ALIGN_Y: u32 = 0x3a5;

    /// The HUD's text object.
    pub const HUD_TEXT: u32 = 0x42c;

    /// Text object fields: colour (8 bytes at 0x40), base scale, scale.
    pub const TEXT_COLOR: u32 = 0x40;

    pub const TEXT_BASE_SCALE: u32 = 0x30;

    pub const TEXT_SCALE: u32 = 0x38;

    pub const TEXT_LINE: u32 = 0x14;

    /// The two unit labels (flag set / clear) and the number format.
    pub const LABEL_A: u32 = 0x004a_2308;

    pub const LABEL_B: u32 = 0x004a_2310;

    pub const NUMBER_FORMAT: u32 = 0x004a_2138;

    /// `gp -` constants: speed scale, units conversion factor, the two digit-scale
    /// factors (fewer than three digits or not), the label offset.
    pub const GP_NEG_SPEED_SCALE: u32 = 0x5320;

    pub const GP_NEG_UNITS_FACTOR: u32 = 0x531c;

    pub const GP_NEG_SCALE_SHORT: u32 = 0x5318;

    pub const GP_NEG_SCALE_LONG: u32 = 0x5314;

    pub const GP_NEG_LABEL_OFFSET: u32 = 0x5310;

    // Callees (all called through the function table).
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const HUD_GET_ELEM_POS: u32 = 0x001e_91f8;

    pub const HUD_GET_ELEM_SCALE: u32 = 0x001e_9290;

    pub const HUD_GET_ELEM_COLOR: u32 = 0x001e_91a8;

    pub const FONT_MEASURE_TEXT: u32 = 0x0039_1fb0;

    pub const HUD_ALIGN_ELEM_POS: u32 = 0x001f_10f8;

    pub const FONT_DRAW_TEXT: u32 = 0x0039_1cb0;

    pub const SPRINTF: u32 = 0x0041_7828;

    pub const STRLEN: u32 = 0x0041_6810;

    pub const FONT_DRAW_TEXT_CENTERED: u32 = 0x0039_2908;

    pub const COPY_HEAD: u32 = 0x0022_01e0;
}

pub(crate) mod chunk {
    use super::ee;
    use super::Callee;

    /// Stream manager (`*handle`): the ring start (`+0x30`) and its mutex
    /// (`+4`).
    pub const MAN_MUTEX: u32 = 0x4;

    pub const MAN_RING: u32 = 0x30;

    /// Stream handle: its tag (`+4`), the bytes still queued (`+8`), the
    /// next chunk (`+0xc`).
    pub const H_TAG: u32 = 0x4;

    pub const H_QUEUED: u32 = 0x8;

    pub const H_NEXT: u32 = 0xc;

    /// Chunk header: `+0` -1 = wrap to the ring start; `+4` tag (top byte)
    /// and size (low 24 bits).
    pub const CH_WRAP: u32 = 0x0;

    pub const CH_TAG_SIZE: u32 = 0x4;

    pub const SIZE_MASK: u64 = 0xff_ffff;

    pub const TAG_MASK: u64 = 0xffff_ffff_ff00_0000;

    pub const HEAD: u32 = 0x003e_13a0;

    pub const STREAM_FILE_VALIDATE: Callee = Callee {
        addr: 0x003d_fbd0,
        f: ee::s14::StreamFile_Validate,
    };

    pub const EA_MUTEX_LOCK: Callee = Callee {
        addr: 0x003e_5700,
        f: ee::s14::EAMutex_Lock,
    };

    pub const EA_MUTEX_UNLOCK: Callee = Callee {
        addr: 0x003e_5760,
        f: ee::s14::EAMutex_Unlock,
    };
}

// ---------------------------------------------------------------------------
// HUD_AlignY (0x0021e7a8)

/// `float HUD_AlignY(void* self, int align, float pos, float size, float
/// scale)` (0x0021e7a8): the top edge of a HUD element aligned at `pos`.
/// `align == 0` centres it (`pos - scale*size*0.5`), `align > 0` bottom
/// aligns it (`pos - scale*size`), `align < 0` keeps `pos` (top aligned).
/// Result in f0 (and f12 holds it too).
pub fn hud_align_y(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    let align = c.g[A1][0];
    let pos = FpuF32(c.f[12]);
    let size = FpuF32(c.f[13]);
    let scale = FpuF32(c.f[14]);
    c.cycles += 2;
    let mut top = pos;
    if align == 0 {
        c.cycles += 5;
        const HALF: u32 = 0x3f00_0000;
        c.g[AT][0] = HALF as u64;
        c.f[1] = HALF;
        let offset = (scale * size) * FpuF32(HALF);
        c.f[0] = offset.0;
        c.cycles += 1;
        top = pos - offset;
    } else {
        c.cycles += 2;
        if (align as i64) > 0 {
            c.cycles += 1;
            let offset = scale * size;
            c.f[0] = offset.0;
            c.cycles += 1;
            top = pos - offset;
        }
    }
    c.f[12] = top.0;
    c.cycles += 2;
    c.f[0] = top.0;
    ret(c);
}

// ---------------------------------------------------------------------------
// DIntr (0x42c078).

/// COP0 Status bit EIE (interrupts enabled).
pub(crate) const STATUS_EIE: u64 = 0x1_0000;

/// `int DIntr(void)` (0x42c078): disable interrupts; returns 1 if they were
/// enabled. The `di` is repeated until Status.EIE reads back clear.
pub fn d_intr(c: &mut Cpu) {
    c.cycles += 5;
    c.mfc0(V1, 12);
    c.g[V0][0] = STATUS_EIE;
    c.g[V1][0] &= c.g[V0][0];
    let was_on = c.g[V1][0] != 0;
    c.g[A0][0] = was_on as u64;
    if !was_on {
        c.cycles += 3;
        c.g[A0][0] = 0;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    c.cycles += 1;
    loop {
        c.cycles += 7;
        c.di();
        c.mfc0(V0, 12);
        c.g[V1][0] = STATUS_EIE;
        c.g[V0][0] &= c.g[V1][0];
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0042_c090);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `void LensFX_DrawSun(LensFX* fx, Viewport* vp, int second_view)`
/// (0x2f4a08): while the sun is visible, reads the view's sun colour,
/// intensity, size (1 means 320) and texture from the world painter
/// (view 6 or 7); if the alpha `intensity * fx->intensity` is positive,
/// pushes an additive 2D draw state on the renderer (`gp+0x2a90`), sets the
/// viewport, binds the texture, draws the sun sprite at the sun's screen
/// position, draws the flare set (`LensFX_DrawFlareSet`), and pops the
/// state.
pub fn lens_fx_draw_sun(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S4, 0x90);
        frame.save(c, S5, 0x80);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S6, 0x70);
        frame.save(c, S7, 0x60);
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 22, 0xf0);
        frame.save_fpr(c, 21, 0xe8);
        frame.save_fpr(c, 20, 0xe0);
        let fx = r32(c, S4);
        c.g[V0][0] = c.lhu(fx.wrapping_add(lens_fx::VISIBLE));
        c.g[S5][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            let drawn = draw_sun_body(c, fx)?;
            if drawn {
                c.cycles += 1;
                frame.restore(c, S0, 0xd0);
            }
        } else {
            c.cycles += 1;
            frame.restore(c, S0, 0xd0);
        }
        c.cycles += 13;
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore(c, S7, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 22, 0xf0);
        frame.restore_fpr(c, 21, 0xe8);
        frame.restore_fpr(c, 20, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// The visible body: false when the alpha was not positive (s0 already
/// restored), true after drawing.
pub(crate) fn draw_sun_body(c: &mut Cpu, fx: u32) -> Result<bool, crate::call::Unwound> {
    // Colour (1, r, g, b) at sp+0x10, copied to sp+0.
    c.cycles += 42;
    c.g[V0][0] = 6;
    c.g[S0][0] = 7;
    if c.g[A2][0] == 0 {
        c.g[S0][0] = c.g[V0][0];
    }
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 22);
    c.g[A0][0] = c.g[S0][0];
    sun_callee::RED.call(c, 0x002f_4a68)?;
    c.mov_s(21, 0);
    c.g[A0][0] = c.g[S0][0];
    sun_callee::GREEN.call(c, 0x002f_4a74)?;
    c.mov_s(20, 0);
    c.g[A0][0] = c.g[S0][0];
    sun_callee::BLUE.call(c, 0x002f_4a80)?;
    let sp = sp(c);
    c.swc1(20, sp.wrapping_add(0x18));
    c.g[S7][0] = sx(sp.wrapping_add(0x10));
    c.swc1(21, sp.wrapping_add(0x14));
    c.swc1(0, sp.wrapping_add(0x1c));
    c.swc1(22, sp.wrapping_add(0x10));
    oc224_ld_u(c, V0, sp.wrapping_add(0x10));
    oc224_ld_u(c, V1, sp.wrapping_add(0x18));
    oc224_sd_u(c, V0, sp);
    oc224_sd_u(c, V1, sp.wrapping_add(8));
    // alpha *= fx->intensity * sun intensity
    c.g[A0][0] = c.g[S0][0];
    sun_callee::INTENSITY.call(c, 0x002f_4abc)?;
    c.lwc1(2, fx.wrapping_add(lens_fx::INTENSITY));
    c.g[A0][0] = c.g[S0][0];
    c.lwc1(1, sp);
    c.mul_s(2, 2, 0);
    c.mul_s(1, 1, 2);
    c.swc1(1, sp);
    sun_callee::SIZE.call(c, 0x002f_4ad8)?;
    c.mov_s(20, 0);
    c.g[A0][0] = c.g[S0][0];
    sun_callee::TEXTURE.call(c, 0x002f_4ae4)?;
    c.c_eq_s(20, 22);
    let unit = c.fcc();
    c.g[S6][0] = c.g[V0][0];
    if unit {
        c.cycles += 2;
        c.g[AT][0] = 0x43a0_0000;
        c.mtc1(AT, 20);
    }
    c.cycles += 6;
    c.lwc1(0, sp);
    c.mtc1(0, 21);
    c.c_le_s(0, 21);
    let invisible = c.fcc();
    c.lq(S0, sp.wrapping_add(0xd0));
    if invisible {
        return Ok(false);
    }

    // Push the draw state (a copy one up), then the default 2D record.
    c.cycles += 155;
    c.g[S0][0] = c.lw(oc224_gp(c, 0x2a90));
    c.g[A0][0] = 0x0050_0000;
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    let st = r32(c, V0);
    oc224_ld_u(c, A3, st);
    oc224_ld_u(c, V1, st.wrapping_add(8));
    c.g[A1][0] = c.lw(st.wrapping_add(0x10));
    oc224_sd_u(c, A3, st.wrapping_add(0x14));
    oc224_sd_u(c, V1, st.wrapping_add(0x1c));
    let v = c.g[A1][0];
    c.sw(st.wrapping_add(0x24), v);
    c.g[V1][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
    let v = c.g[V0][0];
    c.sw(oc224_at(c, S0, renderer::STATE), v);
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1420));
    let def = r32(c, A2);
    oc224_ld_u(c, A3, def);
    oc224_ld_u(c, V0, def.wrapping_add(8));
    c.g[A1][0] = c.lw(def.wrapping_add(0x10));
    let st = r32(c, V1);
    oc224_sd_u(c, A3, st.wrapping_add(0x14));
    oc224_sd_u(c, V0, st.wrapping_add(0x1c));
    let v = c.g[A1][0];
    c.sw(st.wrapping_add(0x24), v);
    // BeginScene (vt[0xa8]).
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, V0, 0xa8));
    c.g[V1][0] = c.lw(oc224_at(c, V0, 0xac));
    let t = r32(c, V1);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x002f_4b94)?;
    // SetViewport(vp, 1, 0, 0, 1) (vt[0xd0]).
    let vp = r32(c, S5);
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.mov_s(16, 21);
    c.lwc1(12, vp);
    c.mov_s(17, 22);
    c.g[A0][0] = c.lh(oc224_at(c, V0, 0xd0));
    c.g[A2][0] = 0;
    c.lwc1(13, vp.wrapping_add(4));
    c.g[A1][0] = 1;
    c.lwc1(14, vp.wrapping_add(8));
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(oc224_at(c, V0, 0xd4));
    let t = r32(c, V1);
    c.lwc1(15, vp.wrapping_add(0xc));
    call_ptr(c, t, 0x002f_4bc8)?;
    // Begin2D (vt[0xf8]), SetTexture(0x4ff1a0) (vt[0x110]).
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, V0, 0xf8));
    c.g[V1][0] = c.lw(oc224_at(c, V0, 0xfc));
    let t = r32(c, V1);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x002f_4bdc)?;
    c.g[A2][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.g[A1][0] = 0x0050_0000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe60));
    c.g[A0][0] = c.lh(oc224_at(c, A2, 0x110));
    c.g[V0][0] = c.lw(oc224_at(c, A2, 0x114));
    let t = r32(c, V0);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x002f_4bf8)?;

    // The state's blend/test bits.
    c.g[A2][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V1][0] = 0xffff_ffff_ffbf_0000;
    c.g[V1][0] |= 0xffff;
    c.g[A1][0] = 0x0040_0000;
    c.g[V0][0] = c.lw(oc224_at(c, A2, 4));
    c.g[A0][0] = 0xffff_ffff_fe7f_0000;
    c.g[A0][0] |= 0xffff;
    c.g[T7][0] = 0x0100_0000;
    c.g[V0][0] &= c.g[V1][0];
    c.g[T3][0] = 0xffff_ffff_ffcf_0000;
    c.g[V0][0] |= c.g[A1][0];
    c.g[T3][0] |= 0xffff;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, A2, 4), v);
    c.g[T4][0] = 0xffff_ffff_fff0_0000;
    c.g[T4][0] |= 0xfff;
    c.g[T5][0] = 0x0001_0000;
    c.g[V1][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[T5][0] |= 0x4000;
    c.g[S1][0] = 0xffff_ffff_ffff_fffc;
    c.g[T6][0] = 0xffff_ffff_e000_0000;
    c.g[V0][0] = c.lw(oc224_at(c, V1, 4));
    c.g[T6][0] |= 0x3ff;
    c.g[S2][0] = 0xffff_ffff_ffff_ff83;
    c.g[S3][0] = 0xffff_ffff_ffff_fc1f;
    c.g[V0][0] &= c.g[A0][0];
    c.g[T2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x33));
    c.g[V0][0] |= c.g[T7][0];
    c.g[T2][0] = sx((c.g[T2][0] as u32) << 2);
    let v = c.g[V0][0];
    c.sw(oc224_at(c, V1, 4), v);
    c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.g[A3][0] = sx(sp.wrapping_add(0x30));
    c.g[T0][0] = sx(sp.wrapping_add(0x40));
    c.g[V1][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[T1][0] = c.g[SP][0];
    c.g[A2][0] = sx(sp.wrapping_add(0x20));
    c.g[A1][0] = c.g[S7][0];
    c.g[V0][0] = c.lw(oc224_at(c, V1, 4));
    c.g[V0][0] &= c.g[T3][0];
    let v = c.g[V0][0];
    c.sw(oc224_at(c, V1, 4), v);
    c.g[A0][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, A0, 4));
    c.g[V0][0] &= c.g[T4][0];
    c.g[V0][0] |= c.g[T5][0];
    let v = c.g[V0][0];
    c.sw(oc224_at(c, A0, 4), v);
    c.g[V1][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, V1, 4));
    c.g[V0][0] &= c.g[S1][0];
    let v = c.g[V0][0];
    c.sw(oc224_at(c, V1, 4), v);
    c.g[A0][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, A0, 8));
    c.g[V0][0] &= c.g[T6][0];
    let v = c.g[V0][0];
    c.sw(oc224_at(c, A0, 8), v);
    c.g[V1][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, V1, 4));
    c.g[V0][0] &= c.g[S2][0];
    c.g[V0][0] |= 0x1c;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, V1, 4), v);
    c.g[A0][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[V0][0] = c.lw(oc224_at(c, A0, 8));
    c.g[V0][0] &= c.g[S3][0];
    c.g[V0][0] |= 0x100;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, A0, 8), v);
    // The texture id (renderer table at +0x101c) into the state.
    c.g[V1][0] = c.lw(oc224_at(c, T2, 0xf50));
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    let v = c.g[V1][0];
    c.sh(oc224_at(c, V0, 0x10), v);
    // DrawSprite((x, y, 0, 1), (size, size), (0, 0), (1, 1), colour).
    c.swc1(21, sp.wrapping_add(0x18));
    c.lwc1(0, fx.wrapping_add(lens_fx::SUN_X));
    c.lwc1(1, fx.wrapping_add(lens_fx::SUN_Y));
    c.swc1(0, sp.wrapping_add(0x10));
    c.swc1(1, sp.wrapping_add(0x14));
    c.swc1(22, sp.wrapping_add(0x1c));
    c.g[V1][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, V1, 0x270));
    c.swc1(20, sp.wrapping_add(0x20));
    c.swc1(20, sp.wrapping_add(0x24));
    c.swc1(21, sp.wrapping_add(0x34));
    c.swc1(22, sp.wrapping_add(0x44));
    c.swc1(21, sp.wrapping_add(0x30));
    c.swc1(22, sp.wrapping_add(0x40));
    c.g[V0][0] = c.lw(oc224_at(c, V1, 0x274));
    let t = r32(c, V0);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x002f_4d40)?;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S5][0];
    sun_callee::DRAW_FLARE_SET.call(c, 0x002f_4d4c)?;
    // Pop the state; End2D (vt[0x100]), EndScene (vt[0xb0]).
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::STATE));
    c.g[A1][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
    let v = c.g[V0][0];
    c.sw(oc224_at(c, S0, renderer::STATE), v);
    c.g[A0][0] = c.lh(oc224_at(c, A1, 0x100));
    c.g[V1][0] = c.lw(oc224_at(c, A1, 0x104));
    let t = r32(c, V1);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x002f_4d6c)?;
    c.g[V0][0] = c.lw(oc224_at(c, S0, renderer::VTABLE));
    c.g[A0][0] = c.lh(oc224_at(c, V0, 0xb0));
    c.g[V1][0] = c.lw(oc224_at(c, V0, 0xb4));
    let t = r32(c, V1);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x002f_4d80)?;
    Ok(true)
}

// ---------------------------------------------------------------------------
// Rider_SetSlew0Target (0x113e80)

/// Rider: +0x1e0 velocity, slew 0 (lean) value +0x1f0, rate +0x1f4,
/// target +0x1f8, +0x438 the main state.
pub(crate) const RIDER_VELOCITY: u32 = 0x1e0;

pub(crate) const RIDER_SLEW0: u32 = 0x1f0;

pub(crate) const RIDER_SLEW0_RATE: u32 = 0x1f4;

pub(crate) const RIDER_SLEW0_TARGET: u32 = 0x1f8;

pub(crate) const RIDER_MAIN_STATE: u32 = 0x438;

/// gp-relative constants: full-lean speed (694.54), rate per unit of
/// distance (12.0), minimum and maximum rate (0.1, 14.0), the rate factor
/// in states 2, 3 and 13 (0.756), seconds per tick (1/60).
pub(crate) const GP_SLEW0_FULL_SPEED: u32 = 0x7c40;

pub(crate) const GP_SLEW0_RATE_SCALE: u32 = 0x7c3c;

pub(crate) const GP_SLEW0_RATE_MIN: u32 = 0x7c38;

pub(crate) const GP_SLEW0_RATE_MAX: u32 = 0x7c34;

pub(crate) const GP_SLEW0_STATE_FACTOR: u32 = 0x7c30;

pub(crate) const GP_SLEW0_DT: u32 = 0x7c2c;

/// `void Rider_SetSlew0Target(Rider* rider, float target)` (0x113e80): set
/// the lean slew target, scaled down below full-lean speed, and its rate:
/// the distance to the current value times 12, clamped to [0.1, 14] (the
/// lower bound wins when above it; reduced in states 2, 3 and 13), per tick.
pub fn rider_set_slew0_target(c: &mut Cpu) {
    let rider = Addr(abi::arg(c, 0));
    let gp = c.g[GP][0] as u32;
    let lit = |o: u32| Addr(gp.wrapping_sub(o));
    c.cycles += 18;
    let full = lit(GP_SLEW0_FULL_SPEED).read_fpu(c);
    abi::set_fpr(c, 1, full);
    // |velocity| (VU0 macro mode, as the original).
    c.lqc2(3, rider.off(RIDER_VELOCITY).0);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V0, 22);
    let speed = FpuF32(c.g[V0][0] as u32);
    c.g[AT][0] = 0x3f80_0000; // 1.0
    let one = FpuF32(0x3f80_0000);
    abi::set_fpr(c, 2, one);
    let ratio = speed.div(full);
    abi::set_fpr(c, 0, ratio);
    let slow = ratio.lt(one);
    abi::set_fcc(c, slow);
    let mut target = abi::farg(c, 0);
    if slow {
        target = target * ratio;
        abi::set_fpr(c, 12, target);
    }
    c.cycles += 11;
    let cur = rider.off(RIDER_SLEW0).read_fpu(c);
    let scale = lit(GP_SLEW0_RATE_SCALE).read_fpu(c);
    let lo = lit(GP_SLEW0_RATE_MIN).read_fpu(c);
    let hi = lit(GP_SLEW0_RATE_MAX).read_fpu(c);
    abi::set_fpr(c, 3, hi);
    abi::set_fpr(c, 1, lo);
    let d = (target - cur).abs() * scale;
    abi::set_fpr(c, 0, d);
    let above = lo.le(d);
    abi::set_fcc(c, above);
    let mut rate = lo;
    if above {
        c.cycles += 1;
        rate = d.min(hi);
    }
    abi::set_fpr(c, 2, rate);
    c.cycles += 5;
    c.g[V1][0] = rider.off(RIDER_MAIN_STATE).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(2));
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    let mut slowed = c.g[V0][0] != 0;
    if !slowed {
        c.cycles += 3;
        c.g[V0][0] = 0xd;
        slowed = c.g[V1][0] == c.g[V0][0];
    }
    if slowed {
        c.cycles += 2;
        let k = lit(GP_SLEW0_STATE_FACTOR).read_fpu(c);
        abi::set_fpr(c, 0, k);
        rate = rate * k;
    }
    abi::set_fpr(c, 1, rate);
    c.cycles += 5;
    let dt = lit(GP_SLEW0_DT).read_fpu(c);
    rider.off(RIDER_SLEW0_TARGET).write_fpu(c, target);
    let per_tick = rate * dt;
    abi::set_fpr(c, 0, per_tick);
    rider.off(RIDER_SLEW0_RATE).write_fpu(c, per_tick);
    abi::ret(c);
}

pub(crate) const PATH_SERVICE_QUEUES: Callee = Callee {
    addr: 0x003d_1e80,
    f: ee::s14::PATH_ServiceQueues,
};

pub(crate) const PATH_VOLUME_FADE: Callee = Callee {
    addr: 0x003d_5a98,
    f: ee::s14::PATH_TrackUpdateVolumeFade,
};

pub(crate) const PATH_AUX_FADE: Callee = Callee {
    addr: 0x003d_5cc0,
    f: ee::s14::PATH_TrackUpdateAuxSendFade,
};

pub(crate) const PATH_FADE3: Callee = Callee {
    addr: 0x003d_5ee8,
    f: ee::s14::PATH_TrackUpdateFade3,
};

pub(crate) const PATH_QUEUED_COUNT: Callee = Callee {
    addr: 0x003d_4950,
    f: ee::s14::PATH_TrackGetQueuedCount,
};

pub(crate) const PATH_TRACK_ADVANCE: Callee = Callee {
    addr: 0x003d_3d20,
    f: ee::s14::PATH_TrackAdvance,
};

/// `void PATH_UpdateTracks(void)` (0x3d4f10): the per-tick service of the
/// PATH interactive-music player. Unless in streamed (`'B'`) mode, service
/// the event queues first. For each of the 24 track slots that is in use
/// and idle: run its volume / aux-send / third fades while it is active;
/// then, for the active tracks of the current mode, stop a track whose stop
/// time has come, or (if its platform stream is playing) advance it to the
/// next node once its queue runs low. Finally (normal mode) service the
/// queues until they are empty.
pub fn path_update_tracks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[V1][0] = 0x45_0000;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = Addr(path::MODE).lbu(c);
        frame.save(c, S4, 0x50);
        c.g[V0][0] ^= 0x42;
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S4][0] = (c.g[V0][0] == 0) as u64;
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        // s6 = tick length, s7 = half of it, fp = 0x450000 (globals base).
        let streamed = c.g[S4][0] != 0;
        c.cycles += if streamed { 5 } else { 4 };
        c.g[V0][0] = 0x45_0000;
        c.g[V1][0] = Addr(path::STATE).lw(c);
        c.g[FP][0] = c.g[V0][0];
        let tick_off = if streamed { 0x10 } else { 0xc };
        c.g[S6][0] = Addr(r32(c, V1).wrapping_add(tick_off)).lw(c);

        c.cycles += 2;
        c.g[S3][0] = 0;
        if !streamed {
            c.cycles += 2;
            PATH_SERVICE_QUEUES.call(c, 0x003d_4f84)?;
        }
        c.cycles += 3;
        c.g[S7][0] = sx(r32(c, S6) >> 1);
        c.g[V0][0] = Addr(path::STATE).lw(c);
        loop {
            c.cycles += 5;
            c.g[V1][0] = sx(r32(c, S3) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[S0][0] = Addr(r32(c, V0).wrapping_add(path::SLOT0)).lw(c);
            if c.g[S0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S0).wrapping_add(track::BUSY)).lbu(c);
                if c.g[V0][0] == 0 {
                    path_update_track(c)?;
                }
            }
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
            c.cycles += 3;
            let more = c.g[S3][0] < path::TRACKS as u64;
            c.g[V0][0] = Addr(path::STATE).lw(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_4f90);
            }
        }

        // Drain the queues (s0 is already restored for the callee's frame).
        c.cycles += 1;
        loop {
            c.cycles += 2;
            frame.restore(c, S0, 0x90);
            if c.g[S4][0] != 0 {
                break;
            }
            c.cycles += 4;
            PATH_SERVICE_QUEUES.call(c, 0x003d_50f0)?;
            frame.restore(c, S0, 0x90);
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_50e0);
            }
        }
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

/// One idle track (s0, slot s3) of `PATH_UpdateTracks`.
pub(crate) fn path_update_track(c: &mut Cpu) -> Flow {
    use track::*;
    let t = r32(c, S0);
    c.cycles += 8;
    c.g[V0][0] = 0x45_0000;
    c.g[A1][0] = Addr(t + LEAD).lw(c);
    c.g[V1][0] = Addr(path::TIME).lw(c);
    c.g[S5][0] = c.g[V0][0];
    c.g[A0][0] = Addr(t + NODE).lh(c);
    c.g[S2][0] = 0;
    // s1 = time + lead: the end of what is queued.
    c.g[S1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    if (c.g[A0][0] as i64) >= 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(t + ACTIVE).lw(c);
        c.g[S2][0] = (c.g[V0][0] != 0) as u64;
    }
    c.cycles += 2;
    if c.g[S2][0] != 0 {
        c.cycles += 3;
        let fades = [
            (VOLUME_FADE, PATH_VOLUME_FADE, 0x003d_4ff4),
            (AUX_FADE, PATH_AUX_FADE, 0x003d_5008),
            (FADE3, PATH_FADE3, 0x003d_501c),
        ];
        for (i, (field, fade, ret)) in fades.into_iter().enumerate() {
            if i > 0 {
                c.cycles += 2;
            }
            c.g[V0][0] = Addr(t + field).lb(c);
            if (c.g[V0][0] as i64) >= 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                fade.call(c, ret)?;
            }
        }
    }
    c.g[V0][0] = Addr(t + STREAMED).lbu(c);
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] != 0) as u64;
    if c.g[V0][0] != c.g[S4][0] {
        return Ok(());
    }
    c.cycles += 2;
    if c.g[S2][0] == 0 {
        return Ok(());
    }

    c.cycles += 3;
    c.g[V1][0] = Addr(t + STOP_AT).lw(c);
    c.g[V0][0] = Addr(path::TIME).lw(c);
    if c.g[V1][0] != 0 {
        // Stop pending: stop the stream once time + tick/2 reaches it.
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S7)));
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        if c.g[V0][0] != 0 {
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = Addr(t + HANDLE).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, 0x002a_e9f8, 0x003d_505c)?;
        Addr(t + STOP_AT).sw(c, 0);
        c.cycles += 1;
        return Ok(());
    }
    // Stream playing?
    c.cycles += 4;
    c.g[A0][0] = Addr(t + HANDLE).lw(c);
    call_ptr(c, 0x002a_ebd0, 0x003d_506c)?;
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S0][0];
    PATH_QUEUED_COUNT.call(c, 0x003d_507c)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = Addr(t + DEADLINE).lw(c);
    c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, S6)));
    c.cycles += 2;
    // Not yet: the queue reaches past the next tick (deadline), or holds
    // more than the lead.
    if c.g[V0][0] != 0 {
        c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
    } else {
        c.g[V0][0] = Addr(t + LEAD).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(t + PLAY_STATE).lb(c);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[A0][0] = Addr(t + HANDLE).lw(c);
        call_ptr(c, 0x002a_f428, 0x003d_50b8)?;
        let started = c.g[V0][0] != 0;
        c.g[V0][0] = u64::MAX;
        if !started {
            c.cycles += 1;
            Addr(t + PLAY_STATE).sb(c, u64::MAX);
        }
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S3][0];
    PATH_TRACK_ADVANCE.call(c, 0x003d_50cc)?;
    c.cycles += 1;
    Ok(())
}

/// `void HUD_DrawSpeedometer(Hud* hud, Elem* elems, Rider* rider, int
/// arg)` (0x2200c0): the speed readout. Copies the HUD settings to the
/// stack, takes the length of the rider's velocity (VU0 `sqrt`) times the
/// speed scale (and the units factor when settings bit 19 is clear),
/// draws the unit label at the speedometer element (measured, aligned with
/// `HUD_AlignElemPos`), then the number (`sprintf("%d", speed + 0.5)`)
/// centred above it, scaled down for three or more digits.
///
/// The settings block is at a constant 8-aligned address, so only the
/// compiled code's aligned copy loop is reachable (its `ldl/ldr` twin is
/// dead code).
pub fn hud_draw_speedometer(c: &mut Cpu) {
    use speedo::*;
    guest(c, |c| {
        let p = |a: u32, o: u32| a.wrapping_add(o);
        c.cycles += 25;
        let frame = Frame::open(c, 0x3d0);
        frame.save(c, S1, 0x3a0);
        frame.save(c, S2, 0x390);
        let rider = r32(c, A2);
        set_reg(c, S1, reg(c, A2));
        frame.save(c, S4, 0x370);
        let hud = r32(c, A0);
        set_reg(c, S2, reg(c, A0));
        let sp0 = sp(c);
        Addr(sp0 + 0x310).sw(c, reg(c, A3));
        let elems = r32(c, A1);
        set_reg(c, S4, reg(c, A1));
        frame.save(c, S0, 0x3b0);
        frame.save(c, S3, 0x380);
        frame.save(c, S5, 0x360);
        frame.save(c, S6, 0x350);
        frame.save(c, S7, 0x340);
        frame.save(c, FP, 0x330);
        frame.save_ra(c, 0x320);
        frame.save_fpr(c, 20, 0x3c0);
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x0022_0104)?;
        set_reg(c, A0, reg(c, V0));
        set_reg(c, A1, 4);
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x0022_0110)?;

        // Copy the settings to sp+0 (32 bytes per step).
        set_reg(c, V1, 0x53_0000);
        set_reg(c, A3, sx(SETTINGS));
        set_reg(c, V0, 0); // SETTINGS & 7
        set_reg(c, A0, sx(sp0));
        c.cycles += 13;
        let (label_pos, label_size) = (sp0 + 0x2d0, sp0 + 0x2e0);
        set_reg(c, T2, sx(label_pos));
        set_reg(c, T3, sx(label_size));
        Addr(sp0 + 0x314).sw(c, sx(label_pos));
        let end = SETTINGS + SETTINGS_SIZE;
        set_reg(c, V0, sx(end));
        Addr(sp0 + 0x318).sw(c, sx(label_size));
        set_reg(c, A1, 0x4a_0000);
        set_reg(c, T0, 0x4a_0000);
        let velocity = p(rider, RIDER_VELOCITY);
        set_reg(c, A2, sx(velocity));
        let elem = p(elems, ELEM);
        set_reg(c, S0, sx(elem));
        let (pos, scale, color, number) = (sp0 + 0x290, sp0 + 0x2a0, sp0 + 0x2b0, sp0 + 0x2f0);
        set_reg(c, S5, sx(pos));
        set_reg(c, S6, sx(scale));
        set_reg(c, S7, sx(color));
        set_reg(c, S3, sx(number));
        let mut from = SETTINGS;
        let mut to = sp0;
        loop {
            c.cycles += 11;
            for (k, r) in [V1, T1, T2, T3].into_iter().enumerate() {
                let v = c.ld(from + 8 * k as u32);
                set_reg(c, r, v);
            }
            for (k, r) in [V1, T1, T2, T3].into_iter().enumerate() {
                c.sd(to + 8 * k as u32, reg(c, r));
            }
            from += 0x20;
            to += 0x20;
            set_reg(c, A3, sx(from));
            set_reg(c, A0, sx(to));
            if from == end {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, COPY_HEAD);
            }
        }
        c.cycles += 1;
        let vt = Addr(velocity).lw(c);
        set_reg(c, V1, vt);

        // The tail doubleword; then the velocity vector.
        c.cycles += 26;
        set_reg(c, FP, sx(LABEL_B));
        c.ldl(V0, from + 7);
        c.ldr(V0, from);
        c.sdl(V0, to + 7);
        set_reg(c, S1, sx(LABEL_A));
        c.sdr(V0, to);
        let slot = VSlot::read(c, vt, SLOT_GET_VELOCITY);
        set_reg(c, A0, slot.delta);
        set_reg(c, V0, slot.func);
        set_reg(c, A0, slot.this(sx(velocity)));
        call_ptr(c, slot.func as u32, 0x0022_0238)?;
        // |v| with VU0 (vf6.w = 1), into f0, times the speed scale.
        c.lqc2(3, r32(c, V0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V1, 22);
        c.mtc1(V1, 0);
        let gp = r32(c, GP);
        c.lwc1(1, gp.wrapping_sub(GP_NEG_SPEED_SCALE));
        let units = Addr(sp0).lw(c) & UNITS_FLAG;
        set_reg(c, V1, UNITS_FLAG);
        set_reg(c, V0, units);
        c.mul_s(20, 0, 1);
        let mut label = LABEL_A;
        if units == 0 {
            c.cycles += 3;
            c.lwc1(0, gp.wrapping_sub(GP_NEG_UNITS_FACTOR));
            set_reg(c, S1, sx(LABEL_B));
            label = LABEL_B;
            c.mul_s(20, 20, 0);
        }

        // The label: element position, scale and colour; measure; align; draw.
        c.cycles += 71;
        set_reg(c, A0, sx(elem));
        set_reg(c, A1, sx(pos));
        call_ptr(c, HUD_GET_ELEM_POS, 0x0022_0290)?;
        set_reg(c, A1, sx(scale));
        set_reg(c, A0, sx(elem));
        call_ptr(c, HUD_GET_ELEM_SCALE, 0x0022_029c)?;
        set_reg(c, A0, sx(elem));
        set_reg(c, A1, sx(color));
        call_ptr(c, HUD_GET_ELEM_COLOR, 0x0022_02a8)?;
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V1, sx(text));
        set_reg(c, A2, sx(sp0 + 0x2c0));
        set_reg(c, A1, sx(label));
        set_reg(c, A3, 0);
        c.ldl(T3, color + 7);
        c.ldr(T3, color);
        c.ldl(V0, color + 0xf);
        c.ldr(V0, color + 8);
        c.sdl(T3, p(text, TEXT_COLOR + 7));
        c.sdr(T3, p(text, TEXT_COLOR));
        c.sdl(V0, p(text, TEXT_COLOR + 0xf));
        c.sdr(V0, p(text, TEXT_COLOR + 8));
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V0, sx(text));
        c.lwc1(1, scale);
        c.lwc1(0, p(text, TEXT_BASE_SCALE));
        c.lwc1(2, p(text, TEXT_BASE_SCALE + 4));
        c.mul_s(0, 0, 1);
        c.swc1(0, p(text, TEXT_SCALE));
        c.lwc1(1, scale + 4);
        c.mul_s(2, 2, 1);
        c.swc1(2, p(text, TEXT_SCALE + 4));
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, A0, sx(text));
        c.lwc1(12, p(text, TEXT_SCALE));
        c.lwc1(13, p(text, TEXT_SCALE + 4));
        call_ptr(c, FONT_MEASURE_TEXT, 0x0022_030c)?;
        c.lwc1(1, sp0 + 0x2c8);
        set_reg(c, T0, sx(number));
        c.lwc1(2, sp0 + 0x2cc);
        set_reg(c, A2, sx(pos));
        set_reg(c, AT, 0x3f80_0000);
        c.mtc1(AT, 0);
        set_reg(c, A0, sx(hud));
        let ay = Addr(p(elems, ELEM_ALIGN_Y)).lb(c);
        set_reg(c, T2, ay);
        let ax = Addr(p(elems, ELEM_ALIGN_X)).lb(c);
        set_reg(c, T1, ax);
        let v = Addr(sp0 + 0x314).lw(c);
        set_reg(c, A1, v);
        let v = Addr(sp0 + 0x310).lw(c);
        set_reg(c, T3, v);
        let v = Addr(sp0 + 0x318).lw(c);
        set_reg(c, A3, v);
        c.swc1(1, label_size);
        c.swc1(0, number + 4);
        c.swc1(0, number);
        c.swc1(2, label_size + 4);
        call_ptr(c, HUD_ALIGN_ELEM_POS, 0x0022_0350)?;
        c.lwc1(12, label_pos);
        set_reg(c, A1, sx(label));
        c.lwc1(13, label_pos + 4);
        let v = Addr(p(hud, HUD_TEXT)).lw(c);
        set_reg(c, A0, v);
        call_ptr(c, FONT_DRAW_TEXT, 0x0022_0364)?;

        // The number: sprintf("%d", (int)(speed + 0.5)).
        set_reg(c, AT, 0x3f00_0000);
        c.mtc1(AT, 0);
        set_reg(c, T1, 0x4a_0000);
        set_reg(c, A1, sx(NUMBER_FORMAT));
        c.add_s(0, 20, 0);
        c.cvt_w_s(1, 0);
        c.mfc1(A2, 1);
        set_reg(c, A0, sx(number));
        call_ptr(c, SPRINTF, 0x0022_0388)?;
        set_reg(c, A0, sx(number));
        call_ptr(c, STRLEN, 0x0022_0390)?;
        let short = reg(c, V0) < 3;
        set_reg(c, V0, short as u64);
        c.lwc1(0, gp.wrapping_sub(GP_NEG_SCALE_SHORT));
        c.lwc1(1, scale);
        if !short {
            c.cycles += 1;
            c.lwc1(0, gp.wrapping_sub(GP_NEG_SCALE_LONG));
        }
        c.cycles += 62;
        c.lwc1(2, scale + 4);
        c.mul_s(1, 1, 0);
        c.mul_s(2, 2, 0);
        c.swc1(1, scale);
        c.swc1(2, scale + 4);
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V0, sx(text));
        set_reg(c, A1, sx(number));
        c.lwc1(0, scale);
        c.lwc1(1, p(text, TEXT_BASE_SCALE));
        c.lwc1(2, p(text, TEXT_BASE_SCALE + 4));
        c.mul_s(1, 1, 0);
        set_reg(c, AT, 0x3f00_0000);
        c.mtc1(AT, 4);
        c.lwc1(5, gp.wrapping_sub(GP_NEG_LABEL_OFFSET));
        c.swc1(1, p(text, TEXT_SCALE));
        c.lwc1(0, scale + 4);
        c.mul_s(2, 2, 0);
        c.swc1(2, p(text, TEXT_SCALE + 4));
        // x = label x + width / 2; y = label y - (line * scale.y * sy / 2 + off).
        c.lwc1(0, label_size);
        c.lwc1(12, label_pos);
        c.mul_s(0, 0, 4);
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V0, sx(text));
        c.lwc1(3, scale + 4);
        c.lwc1(2, label_pos + 4);
        set_reg(c, A0, sx(text));
        c.add_s(12, 12, 0);
        c.swc1(12, label_pos);
        c.lwc1(0, p(text, TEXT_LINE));
        c.cvt_s_w(0, 0);
        c.lwc1(1, p(text, TEXT_BASE_SCALE + 4));
        c.mul_s(0, 0, 1);
        c.mul_s(0, 0, 3);
        c.mul_s(0, 0, 4);
        c.add_s(0, 0, 5);
        c.sub_s(2, 2, 0);
        c.mov_s(13, 2);
        c.swc1(2, label_pos + 4);
        call_ptr(c, FONT_DRAW_TEXT_CENTERED, 0x0022_043c)?;
        // Restore the text scale (base * 1).
        set_reg(c, AT, 0x3f80_0000);
        c.mtc1(AT, 0);
        let text = Addr(p(hud, HUD_TEXT)).lw(c) as u32;
        set_reg(c, V0, sx(text));
        let unit = sp0 + 0x300;
        c.swc1(0, unit + 4);
        c.swc1(0, unit);
        c.lwc1(2, p(text, TEXT_BASE_SCALE));
        c.lwc1(0, p(text, TEXT_BASE_SCALE + 4));
        c.swc1(2, p(text, TEXT_SCALE));
        c.lwc1(1, unit + 4);
        c.mul_s(0, 0, 1);
        frame.restore(c, S0, 0x3b0);
        frame.restore(c, S1, 0x3a0);
        frame.restore(c, S2, 0x390);
        frame.restore(c, S3, 0x380);
        frame.restore(c, S4, 0x370);
        frame.restore(c, S5, 0x360);
        frame.restore(c, S6, 0x350);
        frame.restore(c, S7, 0x340);
        frame.restore(c, FP, 0x330);
        frame.restore_ra(c, 0x320);
        frame.restore_fpr(c, 20, 0x3c0);
        c.swc1(0, p(text, TEXT_SCALE + 4));
        frame.close(c);
        Ok(())
    })
}

/// `u8* StreamFile_GetChunk(StreamFile* handle)` (0x3e12e0): takes the
/// handle's next queued chunk. Null for an invalid handle or an empty
/// queue. Otherwise clears the chunk's tag byte (keeping its size),
/// subtracts the size from the queued count under the manager's mutex and,
/// when data remains, walks the ring (chunks by size, a -1 word wraps to the
/// ring start) to the next chunk carrying this handle's tag; returns the
/// chunk taken.
pub fn stream_file_get_chunk(c: &mut Cpu) {
    use chunk::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        let sp0 = sp(c);
        set_reg(c, A1, sx(sp0));
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        set_reg(c, A2, sx(sp0) | 4);
        STREAM_FILE_VALIDATE.call(c, 0x003e_1300)?;
        let invalid = reg(c, V0) != 0;
        set_reg(c, V0, 0);
        'out: {
            if invalid {
                break 'out;
            }
            c.cycles += 4;
            let h = Addr(sp0 + 4).lw(c) as u32;
            set_reg(c, V1, sx(h));
            let queued = Addr(h + H_QUEUED).lw(c);
            set_reg(c, V0, queued);
            if queued == 0 {
                c.cycles += 2;
                set_reg(c, V0, 0);
                break 'out;
            }
            let chunk = Addr(h + H_NEXT).lw(c) as u32;
            set_reg(c, S2, sx(chunk));
            c.cycles += 17;
            set_reg(c, V0, SIZE_MASK);
            let ts = Addr(chunk + CH_TAG_SIZE).lw(c);
            set_reg(c, V1, ts);
            let size = ts & SIZE_MASK;
            set_reg(c, S1, size);
            Addr(chunk + CH_TAG_SIZE).sw(c, size);
            let man = Addr(sp0).lw(c);
            set_reg(c, A0, addu(man, MAN_MUTEX as u64));
            EA_MUTEX_LOCK.call(c, 0x003e_1340)?;
            let h = Addr(sp0 + 4).lw(c);
            set_reg(c, V0, h);
            let h = h as u32;
            let left = sx((Addr(h + H_QUEUED).lw(c) as u32).wrapping_sub(size as u32));
            set_reg(c, S0, left);
            Addr(h + H_QUEUED).sw(c, left);
            let man = Addr(sp0).lw(c);
            set_reg(c, A0, addu(man, MAN_MUTEX as u64));
            EA_MUTEX_UNLOCK.call(c, 0x003e_135c)?;
            let mut at = sx(chunk.wrapping_add(size as u32));
            set_reg(c, A2, at);
            if (left as i64) > 0 {
                c.cycles += 8;
                let h = Addr(sp0 + 4).lw(c);
                set_reg(c, A1, h);
                let tag_word = Addr((at as u32).wrapping_add(CH_TAG_SIZE)).lw(c) & TAG_MASK;
                set_reg(c, V0, tag_word);
                set_reg(c, A0, TAG_MASK);
                let tag = sx((Addr((h as u32).wrapping_add(H_TAG)).lw(c) as u32) << 24);
                set_reg(c, V1, tag);
                set_reg(c, T1, h);
                if tag_word != tag {
                    c.cycles += 7;
                    set_reg(c, A0, 0xff_0000);
                    let man = Addr(sp0).lw(c);
                    set_reg(c, A1, man);
                    set_reg(c, T0, u64::MAX);
                    set_reg(c, A0, SIZE_MASK);
                    set_reg(c, A3, TAG_MASK);
                    let mut first = Addr(at as u32 + CH_WRAP).lw(c);
                    set_reg(c, V0, first);
                    loop {
                        c.cycles += 2;
                        if first != u64::MAX {
                            let ts = Addr((at as u32).wrapping_add(CH_TAG_SIZE)).lw(c);
                            set_reg(c, V0, ts);
                            c.cycles += 2;
                            let size = ts & SIZE_MASK;
                            set_reg(c, S1, size);
                            at = addu(at, size);
                        } else {
                            c.cycles += 2;
                            at = Addr((man as u32).wrapping_add(MAN_RING)).lw(c);
                        }
                        set_reg(c, A2, at);
                        c.cycles += 4;
                        let tag_word = Addr((at as u32).wrapping_add(CH_TAG_SIZE)).lw(c) & TAG_MASK;
                        set_reg(c, V0, tag_word);
                        if tag_word == tag {
                            break;
                        }
                        first = Addr(at as u32 + CH_WRAP).lw(c);
                        set_reg(c, V0, first);
                        if event_due(c) {
                            deliver_event_here(c, HEAD);
                        }
                    }
                }
                c.cycles += 1;
                Addr((h as u32).wrapping_add(H_NEXT)).sw(c, at);
            }
            c.cycles += 1;
            set_reg(c, V0, sx(chunk));
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod oc230_tanf {
    use crate::call::Callee;

    pub const REM_PIO2F: Callee = Callee {
        addr: 0x0040_e878,
        f: ssx3_ee::s15::__ieee754_rem_pio2f,
    };

    pub const KERNEL_TANF: Callee = Callee {
        addr: 0x0040_f5a8,
        f: ssx3_ee::s15::__kernel_tanf,
    };
}

pub(crate) mod oc230_cpu_rider {

    /// The actor's rider.
    pub const RIDER: u32 = 0x18;

    /// The rider's track query object (vtable at +0x34, query slot 8).
    pub const TRACK_QUERY: u32 = 0xab8;

    pub const QUERY_VTABLE: u32 = 0x34;

    pub const QUERY_SLOT: u32 = 0x8;

    /// The rider's track position and distance.
    pub const TRACK_POS: u32 = 0x4c0;

    pub const TRACK_DIST: u32 = 0x4c4;

    /// Marker records (0x10 bytes: type, value, distance).
    pub const SPEED_MARKER: u64 = 0x11;
}

pub(crate) mod turn_around {

    /// `+0x120` orientation quaternion.
    pub const ROT: u32 = 0x120;

    /// `+0x1c0` world matrix row z (the up axis).
    pub const WORLD_UP: u32 = 0x1c0;

    /// `+0x1e0` velocity.
    pub const VEL: u32 = 0x1e0;

    /// `+0x214` current brake slew value.
    pub const BRAKE: u32 = 0x214;

    /// `+0x3a0` ground forward direction.
    pub const GROUND_FORWARD: u32 = 0x3a0;

    /// `gp-0x7b98`: how far backward (velocity along the ground forward
    /// axis) the rider must move before turning.
    pub const GP_BACKWARD_LIMIT: u32 = 0x7b98;
}

pub(crate) mod prewind {
    use super::ee;
    use super::Callee;

    /// `ComputerActorRider`: its rider; the control block (+0xdf0: speed
    /// threshold, +0x48/+0x4c analog values); the scripted-start flag and
    /// its two points.
    pub const RIDER: u32 = 0x18;

    pub const CONTROL: u32 = 0xdf0;

    pub const SCRIPTED: u32 = 0xe1c;

    pub const SCRIPT_FROM: u32 = 0xe50;

    pub const SCRIPT_TO: u32 = 0xe60;

    /// `Rider`: position, look-ahead point, velocity, track distance and
    /// lateral offset, the track.
    pub const POS: u32 = 0x110;

    pub const VELOCITY: u32 = 0x1e0;

    pub const AHEAD: u32 = 0x490;

    pub const TRACK_DIST: u32 = 0x4c0;

    pub const TRACK_SIDE: u32 = 0x4c4;

    pub const TRACK: u32 = 0xab8;

    /// Track: vtable at +0x34, `Query(out, max, from, to)` at slot 8.
    pub const TRACK_VT: u32 = 0x34;

    pub const SLOT_QUERY: u32 = 0x8;

    /// A track event of this type ends the prewind.
    pub const EVENT_PREWIND_END: u64 = 0x10;

    /// gp-0x7e30: subtracted from the speed threshold.
    pub const GP_SPEED_MARGIN: u32 = 0x7e30;

    /// `Path_GetPointAtDistance(out, track, in_range, t)` (cross-shard).
    pub const PATH_GET_POINT: u32 = 0x0026_ab20;

    pub const COMPUTE_STEER: Callee = Callee {
        addr: 0x0010_0348,
        f: ee::s00::ComputerActorRider_ComputeSteer,
    };
}

/// `float tanf(float x)` (0x40d588, fdlibm): |x| <= pi/4 goes straight to
/// `__kernel_tanf(x, 0, 1)`; inf/NaN give `x - x`; otherwise x is reduced
/// by `__ieee754_rem_pio2f` (y[0], y[1] at sp+0) and the kernel gets
/// `(y0, y1, 1 - 2 * (n & 1))`.
pub fn tanf(c: &mut Cpu) {
    use oc230_tanf as callee;
    guest(c, |c| {
        c.cycles += 11;
        c.mfc1(V0, 12);
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = 0x7fff_ffff;
        c.g[A0][0] &= c.g[V1][0]; // |x| bits
        c.g[V0][0] = (0x3f49_0fda_i64 < c.g[A0][0] as i64) as u64; // > pi/4
        frame.save_ra(c, 0x10);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.mtc1(0, 13);
            c.g[A0][0] = 1;
        } else {
            c.cycles += 5;
            c.g[V0][0] = (0x7f7f_ffff_i64 < c.g[A0][0] as i64) as u64; // inf/NaN
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.sub_s(0, 12, 12);
                return oc230_tanf_exit(c, frame);
            }
            c.cycles += 8;
            c.g[A0][0] = c.g[SP][0];
            callee::REM_PIO2F.call(c, 0x0040_d5e4)?;
            let s = sp(c);
            c.g[V0][0] &= 1;
            c.g[A0][0] = 1;
            c.g[V0][0] = oc230_sll(c, V0, 1);
            c.lwc1(12, s);
            c.lwc1(13, s.wrapping_add(4));
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V0)));
        }
        c.cycles += 2;
        callee::KERNEL_TANF.call(c, 0x0040_d604)?;
        oc230_tanf_exit(c, frame)
    })
}

pub(crate) fn oc230_tanf_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// `bool ComputerActorRider_FindSpeedMarker(ComputerActorRider* a, float*
/// speed, int* reached)` (0x10bb18): asks the rider's track query for up
/// to 12 markers between its position and 600 units ahead (into a frame
/// buffer); at the first speed marker (type 0x11) writes its speed (value
/// * gp-0x7e04, km/h to units) and whether its distance has been reached,
/// and returns true; false when there is none.
pub fn computer_actor_rider_find_speed_marker(c: &mut Cpu) {
    use oc230_cpu_rider as cr;
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x100);
        c.g[AT][0] = 0x4416_0000; // 600
        c.mtc1(AT, 0);
        frame.save(c, S0, 0xf0);
        frame.save(c, S1, 0xe0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0xd0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0xc0);
        c.g[S2][0] = c.g[A2][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0xc;
        c.g[V0][0] = c.lw(oc230_at(c, S0, cr::RIDER));
        let rider = r32(c, V0);
        c.g[A3][0] = c.lw(rider.wrapping_add(cr::TRACK_QUERY));
        c.lwc1(13, rider.wrapping_add(cr::TRACK_DIST));
        c.g[V1][0] = c.lw(oc230_at(c, A3, cr::QUERY_VTABLE));
        c.lwc1(12, rider.wrapping_add(cr::TRACK_POS));
        c.add_s(13, 13, 0);
        c.g[A0][0] = c.lh(oc230_at(c, V1, cr::QUERY_SLOT));
        c.g[V0][0] = c.lw(oc230_at(c, V1, cr::QUERY_SLOT + 4));
        let t = r32(c, V0);
        c.g[A0][0] = oc230_add(c, A3, A0);
        call_ptr(c, t, 0x0010_bb70)?;
        c.g[A1][0] = c.g[V0][0]; // count
        let none = (c.g[A1][0] as i64) <= 0;
        c.g[A0][0] = 0;
        let mut found = false;
        if !none {
            c.cycles += 3;
            c.lwc1(2, oc230_gp(c, 0u32.wrapping_sub(0x7e04)));
            c.g[A2][0] = cr::SPEED_MARKER;
            c.g[V1][0] = c.g[SP][0];
            loop {
                c.cycles += 3;
                let rec = r32(c, V1);
                c.g[V0][0] = c.lw(rec);
                if c.g[V0][0] == c.g[A2][0] {
                    // speed = value * scale; reached = distance <= rider's.
                    c.cycles += 11;
                    c.lwc1(0, rec.wrapping_add(4));
                    c.cvt_s_w(0, 0);
                    c.lwc1(1, rec.wrapping_add(8));
                    c.mul_s(0, 0, 2);
                    c.swc1(0, r32(c, S1));
                    c.g[V0][0] = c.lw(oc230_at(c, S0, cr::RIDER));
                    c.lwc1(0, r32(c, V0).wrapping_add(cr::TRACK_DIST));
                    c.c_le_s(1, 0);
                    let reached = c.fcc();
                    c.g[V1][0] = 1;
                    if !reached {
                        c.cycles += 1;
                        c.g[V1][0] = 0;
                    }
                    c.cycles += 3;
                    let v = c.g[V1][0];
                    c.sw(r32(c, S2), v);
                    c.g[V0][0] = 1;
                    found = true;
                    break;
                }
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx(rec.wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_bb88);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore_ra(c, 0xc0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Render-state stack helpers (2D render context, `*(gp - 0x854)`)
// ---------------------------------------------------------------------------

/// The render context (`lw gp-0x854`).
pub(crate) fn r2d_ctx(c: &mut Cpu) -> u64 {
    let gp = c.g[GP][0] as u32;
    Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c)
}

/// `top[1] = top[0]` (0x14 bytes: two unaligned doublewords through `d0`,
/// `d1`, then a word through `w`); `top` is the current state.
pub(crate) fn r2d_copy_up(c: &mut Cpu, top: u32, d0: usize, d1: usize, w: usize) {
    c.ldl(d0, top + 0x7);
    c.ldr(d0, top);
    c.ldl(d1, top + 0xf);
    c.ldr(d1, top + 0x8);
    c.g[w][0] = Addr(top).off(0x10).lw(c);
    c.sdl(d0, top + 0x1b);
    c.sdr(d0, top + 0x14);
    c.sdl(d1, top + 0x23);
    c.sdr(d1, top + 0x1c);
    let v = c.g[w][0];
    Addr(top).off(0x24).sw(c, v);
}

/// `reg = ctx->top; v0 = reg->word[off]; v0 = (v0 & and) | or; store`.
pub(crate) fn r2d_bits(c: &mut Cpu, ctx: u32, reg: usize, off: u32, and: u64, or: Option<u64>) {
    c.g[reg][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
    let top = r32(c, reg);
    c.g[V0][0] = Addr(top).off(off).lw(c);
    c.g[V0][0] &= and;
    if let Some(or) = or {
        c.g[V0][0] |= or;
    }
    let v = c.g[V0][0];
    Addr(top).off(off).sw(c, v);
}

// ---------------------------------------------------------------------------
// cPS2FogParticleMan_Render (0x2dbf98)
// ---------------------------------------------------------------------------

/// The fog particle manager: queued particle count (+0), vtable (+4; slot
/// 0x18 draws them).
pub(crate) const FOG_COUNT: u32 = 0x0;

pub(crate) const FOG_VT: u32 = 0x4;

pub(crate) const VT_FOG_DRAW: u32 = 0x18;

/// The sort buffer (8-byte {particle, depth} pairs) and its comparator
/// (`cPS2FogParticleMan_CompareDesc`).
pub(crate) const FOG_SORT_BUF: u32 = 0x0053_7ac0;

pub(crate) const FOG_COMPARE: u32 = 0x002d_c168;

/// `void cPS2FogParticleMan_Render(cPS2FogParticleMan* m)` (0x2dbf98): if
/// particles were queued, sort them back to front (`qsort`), push the 2D
/// render state and set it up for fog sprites (alpha blending, depth test
/// and write settings, the context's fog colour index, full mask), let the
/// manager draw them, pop the state and empty the queue.
pub fn ps2_fog_particle_man_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        let man = r32(c, S3);
        c.g[A1][0] = Addr(man).off(FOG_COUNT).lw(c);
        c.g[A3][0] = 0x2e_0000;
        if c.g[A1][0] != 0 {
            c.cycles += 98;
            // qsort(buf, count, 8, CompareDesc)
            c.g[A0][0] = sx(FOG_SORT_BUF);
            c.g[A3][0] = sx(FOG_COMPARE);
            c.g[A2][0] = 8;
            call_ptr(c, 0x0041_8ef8, 0x002d_bfd4)?;

            // Push the render state.
            c.g[V1][0] = r2d_ctx(c);
            let ctx = r32(c, V1);
            c.g[T5][0] = 0xffff_ffff_ffff_fc1f;
            c.g[T6][0] = 0xffff_ffff_ffff_fffc;
            c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            c.g[A3][0] = 0xffff_ffff_e000_03ff;
            c.g[T7][0] = u64::MAX;
            let top = r32(c, V0);
            r2d_copy_up(c, top, A0, A1, T0);
            c.g[A2][0] = 0xf_fc00;
            c.g[S0][0] = 0xffff_ffff_ffff_ff83;
            c.g[T1][0] = 0xffff_ffff_ffbf_ffff;
            c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            c.g[A1][0] = r2d_ctx(c);
            c.g[S1][0] = 0x40_0000;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x14));
            c.g[T0][0] = 0xffff_ffff_fe7f_ffff;
            let v = c.g[V0][0];
            Addr(ctx).off(R2D_STATE_TOP).sw(c, v);
            c.g[T2][0] = 0xffff_ffff_ffcf_ffff;
            c.g[S2][0] = 0x30_0000;

            // Fog sprite state.
            let ctx = r32(c, A1);
            c.g[A0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            c.g[T4][0] = Addr(ctx).off(0xf60).lw(c);
            c.g[T3][0] = 0xffff_ffff_fff0_0fff;
            let top = r32(c, A0);
            c.g[V0][0] = Addr(top).off(0x8).lw(c);
            c.g[V0][0] &= c.g[T5][0];
            c.g[V0][0] |= 0xe0;
            let v = c.g[V0][0];
            Addr(top).off(0x8).sw(c, v);
            let (t6, a3, a2) = (c.g[T6][0], c.g[A3][0], c.g[A2][0]);
            r2d_bits(c, ctx, V1, 0x4, t6, Some(0x2));
            r2d_bits(c, ctx, A0, 0x8, a3, Some(a2));
            c.g[V1][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            let fog = c.g[T4][0];
            Addr(r32(c, V1)).off(0x10).sh(c, fog);
            c.g[A1][0] = r2d_ctx(c);
            let ctx = r32(c, A1);
            c.g[A0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            let top = r32(c, A0);
            c.g[V0][0] = Addr(top).lw(c);
            c.g[V0][0] |= 0xc;
            let v = c.g[V0][0];
            Addr(top).sw(c, v);
            c.g[V1][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            let mask = c.g[T7][0];
            Addr(r32(c, V1)).off(0x12).sh(c, mask);
            c.g[A2][0] = r2d_ctx(c);
            let ctx = r32(c, A2);
            let (s0, t1, s1, t0, t2, s2, t3) = (
                c.g[S0][0], c.g[T1][0], c.g[S1][0], c.g[T0][0], c.g[T2][0], c.g[S2][0], c.g[T3][0],
            );
            r2d_bits(c, ctx, V1, 0x4, s0, Some(0x14));
            r2d_bits(c, ctx, A0, 0x4, t1, Some(s1));
            r2d_bits(c, ctx, V1, 0x4, t0, None);
            r2d_bits(c, ctx, A1, 0x4, t2, Some(s2));
            r2d_bits(c, ctx, A0, 0x4, t3, None);

            // m->Draw()
            c.g[V1][0] = Addr(man).off(FOG_VT).lw(c);
            let (obj, vt) = (c.g[S3][0], c.g[V1][0]);
            vcall(c, obj, vt, VT_FOG_DRAW, V0, 0x002d_c134)?;

            // Pop the state; empty the queue.
            c.g[V1][0] = r2d_ctx(c);
            let ctx = r32(c, V1);
            c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
            let v = c.g[V0][0];
            Addr(ctx).off(R2D_STATE_TOP).sw(c, v);
            Addr(man).off(FOG_COUNT).sw(c, 0);
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

// ---------------------------------------------------------------------------
// PS2VisualEffectsMan_Render (0x2f0cd0)
// ---------------------------------------------------------------------------

/// The visual effects manager's effect systems (17 pointers from +0).
pub(crate) const VFX_SYSTEMS: u32 = 17;

/// Effect system vtable slot: render.
pub(crate) const VT_VFX_RENDER: u32 = 0x18;

/// `void PS2VisualEffectsMan_Render(PS2VisualEffectsMan* m)` (0x2f0cd0):
/// render each of the 17 effect systems in order (`sys->Render()`).
pub fn ps2_visual_effects_man_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 110;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let man = r32(c, S0);
        for k in 0..VFX_SYSTEMS {
            c.g[A1][0] = Addr(man).off(4 * k).lw(c);
            let sys = c.g[A1][0];
            c.g[V0][0] = Addr(sys as u32).lw(c);
            let vt = c.g[V0][0];
            vcall(c, sys, vt, VT_VFX_RENDER, V1, 0x002f_0cf8 + 0x18 * k)?;
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const RIDER_BUILD_WORLD_MATRIX: Callee = Callee {
    addr: 0x0011_e098,
    f: ee::s00::Rider_BuildWorldMatrix,
};

pub(crate) const RIDER_TOGGLE_STANCE: Callee = Callee {
    addr: 0x0011_5168,
    f: ee::s00::Rider_ToggleStance,
};

/// `bool Rider_TurnAroundIfBackward(Rider* rider)` (0x114cc0): when the
/// rider is not braking and moves backward along the ground
/// (`vel · groundForward < -limit`), turn half way round the up axis (the
/// orientation becomes `(up, 0) ⊗ rot`, a 180° rotation), rebuild the world
/// matrix and switch stance. Returns whether it turned.
pub fn rider_turn_around_if_backward(c: &mut Cpu) {
    use turn_around::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        c.mtc1(0, 2);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x50);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        let r = rider as u32;
        c.lwc1(0, r.wrapping_add(BRAKE));
        c.c_eq_s(0, 2);
        let not_braking = c.fcc();
        c.g[V0][0] = 0;
        if not_braking {
            c.cycles += 15;
            // f1 = vel · groundForward (VU0 dot product, w lane).
            let gp = c.g[GP][0] as u32;
            c.lwc1(0, gp.wrapping_sub(GP_BACKWARD_LIMIT));
            c.lqc2(3, r.wrapping_add(VEL));
            c.lqc2(5, r.wrapping_add(GROUND_FORWARD));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(V0, 4);
            c.mtc1(V0, 1);
            c.neg_s(0, 0);
            c.c_lt_s(0, 1);
            if c.fcc() {
                // Not backward enough.
                c.g[V0][0] = 0;
            } else {
                c.cycles += 32;
                // The up axis as a pure quaternion (w = 0 from f2), built
                // through the stack as the original does.
                c.lq(V0, r.wrapping_add(WORLD_UP));
                c.g[A0][0] = rider;
                let s = sp(c);
                c.swc1(2, s.wrapping_add(0x1c));
                c.sq(V0, s.wrapping_add(0x30));
                c.lwc1(0, s.wrapping_add(0x30));
                c.lwc1(1, s.wrapping_add(0x34));
                c.lwc1(2, s.wrapping_add(0x38));
                for base in [0x10u32, 0x20] {
                    c.swc1(0, s.wrapping_add(base));
                    c.swc1(1, s.wrapping_add(base + 4));
                    c.swc1(2, s.wrapping_add(base + 8));
                }
                // rot = up ⊗ rot (VU0 quaternion product).
                c.lqc2(4, s.wrapping_add(0x10));
                c.lqc2(5, r.wrapping_add(ROT));
                c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                c.sqc2(8, s.wrapping_add(0x40));
                c.lq(V0, s.wrapping_add(0x40));
                c.sq(V0, r.wrapping_add(ROT));
                c.sq(V0, s);
                RIDER_BUILD_WORLD_MATRIX.call(c, 0x0011_4d98)?;
                c.g[A0][0] = rider;
                RIDER_TOGGLE_STANCE.call(c, 0x0011_4da0)?;
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------- PS2VisualEffectsMan_Update

/// `PS2VisualEffectsMan_ApplyWorldPainter(self, index)`.
pub(crate) const VFX_APPLY_WORLD_PAINTER: Callee = Callee {
    addr: 0x002f_00a0,
    f: ee::s09::PS2VisualEffectsMan_ApplyWorldPainter,
};

/// `void PS2VisualEffectsMan_Update(PS2VisualEffectsMan* self)` (0x2f0a98):
/// reset the 17 effect objects (`self->objs[0..17]`, virtual slot `+0x10`
/// each), then for every player view `i` (the count at
/// `world->app(+0x84)->game(+0x84)->players(+0x10)`) apply the world
/// painter's parameters for the view's painter index (`gp+0xab8[i]`) and
/// reset the drawer's view slot `i` (virtual `+0x398` of `ctx+0x10d8`,
/// `ctx` at `gp-0x854`).
pub fn ps2_visual_effects_man_update(c: &mut Cpu) {
    /// `gp-0x848`: the world; `gp-0x854`: the render context.
    const WORLD_GP: u32 = 0x848;
    const CTX_GP: u32 = 0x854;
    /// `gp+0xab8`: the painter index of each view.
    const VIEW_PAINTER_GP: u32 = 0xab8;
    /// Loop head (0x2f0c70), for event delivery.
    const HEAD: u32 = 0x002f_0c70;
    /// The number of player views.
    fn view_count(c: &mut Cpu) -> u64 {
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(WORLD_GP));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x84));
        c.lw(r32(c, A0).wrapping_add(0x10))
    }
    guest(c, |c| {
        c.cycles += 115;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        for k in 0..17u32 {
            // objs[k]->Reset() (virtual +0x10).
            let obj = c.lw(this.wrapping_add(4 * k));
            c.g[A1][0] = obj;
            c.g[V0][0] = c.lw(obj as u32);
            c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x10));
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x14));
            let target = r32(c, V1);
            c.g[A0][0] = sx((obj as u32).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x002f_0acc + 0x18 * k)?;
        }
        let count = view_count(c);
        c.g[V0][0] = count;
        if (count as i64) > 0 {
            c.cycles += 3;
            c.g[S1][0] = sx(r32(c, GP).wrapping_add(VIEW_PAINTER_GP));
            c.g[A1][0] = c.lw(r32(c, S1));
            loop {
                c.cycles += 18;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                VFX_APPLY_WORLD_PAINTER.call(c, 0x002f_0c7c)?;
                // ctx->drawer->ResetView(i) (virtual +0x398).
                let ctx = c.lw(r32(c, GP).wrapping_sub(CTX_GP));
                c.g[A2][0] = ctx;
                c.g[A1][0] = c.g[S2][0];
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V0][0] = c.lw((ctx as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x398));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x39c));
                let target = r32(c, V1);
                c.g[A0][0] = sx((ctx as u32).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x002f_0c9c)?;
                let count = view_count(c);
                let more = (c.g[S2][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    c.cycles += 1;
                    break;
                }
                c.g[A1][0] = c.lw(r32(c, S1));
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// --------------------------------------- ComputerActorRider_UpdateSpinPlan

/// `RiderSpinSub_GetFlipProgress(spin, &a, &b)` and
/// `RiderSpinSub_GetSpinProgress(spin, &c, &d)`: the heights still needed
/// to finish the current flip / spin (and with a margin).
pub(crate) const RIDER_SPIN_GET_FLIP_PROGRESS: u32 = 0x0013_5cb0;

pub(crate) const RIDER_SPIN_GET_SPIN_PROGRESS: u32 = 0x0013_5db0;

/// `void ComputerActorRider_UpdateSpinPlan(ComputerActorRider* ai, float
/// height)` (0x10b0e8): drop the AI's planned flip (`+0xe38`) and spin
/// (`+0xe3c`) when there is no longer height for them. Both go when the AI
/// must land (`+0xe20`) and is lower than 1. Otherwise, while planning
/// (`+0xe10`): the flip goes when `height + k0` is below the flip's need,
/// or below its margin while the need is under `k1`; the spin likewise
/// (with `k2`, `k3`), and dropping the spin drops the flip too.
pub fn computer_actor_rider_update_spin_plan(c: &mut Cpu) {
    /// AI fields.
    const RIDER: u32 = 0x18;
    const PLANNING: u32 = 0xe10;
    const MUST_LAND: u32 = 0xe20;
    const FLIP: u32 = 0xe38;
    const SPIN: u32 = 0xe3c;
    /// `rider+0x77c`: the rider state; its spin sub-state at +0x230.
    const RIDER_STATE: u32 = 0x77c;
    const SPIN_SUB: u32 = 0x230;
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        c.g[A1][0] = c.g[SP][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        c.f[20] = c.f[12];
        let ai = r32(c, S0);
        c.g[V0][0] = c.lw(ai.wrapping_add(RIDER));
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(RIDER_STATE));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(SPIN_SUB));
        call_ptr(c, RIDER_SPIN_GET_FLIP_PROGRESS, 0x0010_b118)?;
        c.g[V0][0] = c.lw(ai.wrapping_add(RIDER));
        c.g[A1][0] = c.g[SP][0] | 0x8;
        c.g[A2][0] = c.g[SP][0] | 0xc;
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(RIDER_STATE));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(SPIN_SUB));
        call_ptr(c, RIDER_SPIN_GET_SPIN_PROGRESS, 0x0010_b130)?;
        let sp0 = sp(c);
        c.g[V0][0] = c.lw(ai.wrapping_add(MUST_LAND));
        c.g[V1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.c_lt_s(20, 0);
            if c.fcc() {
                c.g[V1][0] = 1;
            }
        }
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            // Too low to finish anything: drop both.
            c.cycles += 3;
            c.sw(ai.wrapping_add(FLIP), 0);
            c.sw(ai.wrapping_add(SPIN), 0);
            c.cycles += 1;
        } else {
            c.g[V0][0] = c.lw(ai.wrapping_add(PLANNING));
            c.cycles += 2;
            if c.g[V0][0] != 0 && plan_checks(c, ai, sp0) {
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });

    /// The flip and spin checks (0x10b170..0x10b23c); true when they end
    /// through the shared exit (0x10b23c, one more cycle).
    fn plan_checks(c: &mut Cpu, ai: u32, sp0: u32) -> bool {
        let gp = r32(c, GP);
        // The flip: needs a (sp+0), b (sp+4).
        c.cycles += 6;
        c.lwc1(0, ai.wrapping_add(FLIP));
        c.mtc1(0, 3);
        c.c_eq_s(0, 3);
        if !c.fcc() {
            c.cycles += 7;
            c.lwc1(0, gp.wrapping_sub(0x7e20));
            c.lwc1(2, sp0);
            c.add_s(1, 20, 0);
            c.c_lt_s(1, 2);
            if c.fcc() {
                c.swc1(3, ai.wrapping_add(FLIP));
                c.cycles += 1;
            } else {
                c.cycles += 5;
                c.lwc1(0, sp0 + 0x4);
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.cycles += 6;
                    c.lwc1(0, gp.wrapping_sub(0x7e1c));
                    c.sub_s(0, 2, 0);
                    c.c_lt_s(0, 3);
                    if c.fcc() {
                        c.cycles += 1;
                        c.swc1(3, ai.wrapping_add(FLIP));
                        c.cycles += 1;
                    }
                }
            }
        }
        c.lwc1(0, ai.wrapping_add(SPIN));

        // The spin: needs c (sp+8), d (sp+0xc).
        c.cycles += 5;
        c.mtc1(0, 1);
        c.c_eq_s(0, 1);
        if c.fcc() {
            return false;
        }
        c.cycles += 7;
        c.lwc1(0, gp.wrapping_sub(0x7e18));
        c.lwc1(2, sp0 + 0x8);
        c.add_s(12, 20, 0);
        c.c_lt_s(12, 2);
        if !c.fcc() {
            c.cycles += 5;
            c.lwc1(0, sp0 + 0xc);
            c.c_lt_s(12, 0);
            if !c.fcc() {
                return false;
            }
            c.cycles += 6;
            c.lwc1(0, gp.wrapping_sub(0x7e14));
            c.sub_s(0, 2, 0);
            c.c_lt_s(0, 1);
            if !c.fcc() {
                return false;
            }
            c.cycles += 1;
        }
        c.swc1(1, ai.wrapping_add(FLIP));
        c.cycles += 1;
        c.swc1(1, ai.wrapping_add(SPIN));
        true
    }
}

/// `gp - off` (the small-data constants).
#[inline(always)]
pub(crate) fn gp_lit(c: &Cpu, off: u32) -> u32 {
    r32(c, GP).wrapping_sub(off)
}

// ---------------------------------------------------------------------------
// __ieee754_logf (0x40e550, newlib e_logf.c).

/// `at = bits; mtc1 at, f` (a float literal through `at`).
#[inline(always)]
pub(crate) fn lit(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.mtc1(AT, f);
}

// ---------------------------------------------------------------------------
// ComputerActorRider_ThinkPrewind (0x10aa70, src/seg/seg_1218.cpp).

/// `VF4.x = |VF3|` (the compiled VU0 length: dot product, sqrt into Q).
pub(crate) fn vu0_length_vf3(c: &mut Cpu) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
}

/// `void ComputerActorRider_ThinkPrewind(ComputerActorRider* self, Pad*
/// pad)` (0x10aa70): the CPU rider's input before the start. Queries the
/// track events in [dist, dist + 50]; presses bit 14 while slower than the
/// threshold; on a scripted start presses bit 13 once the rider is within
/// 50 of the goal distance and stops there. Otherwise a prewind-end event
/// (type 0x10) presses bit 13 and steers (pad word 1, 6 bits, from
/// `ComputeSteer` towards the event's point relative to half the
/// look-ahead), and the two analog values go to pad bits 15..20 and 21..26
/// (times 31). Frame 0x160 (the event buffer at +0, s0-s3, f20).
pub fn computer_actor_rider_think_prewind(c: &mut Cpu) {
    use prewind::*;
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x160);
        c.g[A2][0] = 0xc;
        frame.save(c, S1, 0x130);
        frame.save(c, S2, 0x120);
        frame.save(c, S0, 0x140);
        frame.save(c, S3, 0x110);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x150);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x100);
        let me = r32(c, S2);
        let pad = r32(c, S1);
        lit(c, 20, 0x4248_0000); // 50.0
        c.g[V0][0] = c.lw(me.wrapping_add(RIDER));
        c.g[S0][0] = c.lw(at(c, V0, TRACK));
        c.lwc1(13, at(c, V0, TRACK_SIDE));
        c.g[V1][0] = c.lw(at(c, S0, TRACK_VT));
        c.lwc1(12, at(c, V0, TRACK_DIST));
        c.add_s(13, 13, 20);
        vcall_a0_s0(c, SLOT_QUERY, 0x0010_aacc)?;
        // Slower than the threshold: bit 14.
        c.lwc1(1, me.wrapping_add(CONTROL));
        c.g[A3][0] = c.g[V0][0];
        c.lwc1(2, gp_lit(c, GP_SPEED_MARGIN));
        c.g[T0][0] = u64::MAX;
        c.g[V0][0] = c.lw(me.wrapping_add(RIDER));
        c.sub_s(1, 1, 2);
        c.lqc2(3, at(c, V0, VELOCITY));
        vu0_length_vf3(c);
        c.cfc2(V1, 22);
        c.mtc1(V1, 0);
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.cycles += 4;
            c.g[V0][0] = c.lw(pad) | 0x4000;
            c.sw(pad, c.g[V0][0]);
        }
        c.g[V0][0] = c.lw(me.wrapping_add(SCRIPTED));
        c.cycles += 2;
        c.g[A2][0] = 0;
        if c.g[V0][0] != 0 {
            // 0x10ab34: |pos - from| < |to - from| - 50: bit 13.
            c.cycles += 38;
            let s = sp(c);
            c.lqc2(3, me.wrapping_add(SCRIPT_TO));
            c.lqc2(4, me.wrapping_add(SCRIPT_FROM));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s.wrapping_add(0xd0));
            c.lq(2, s.wrapping_add(0xd0));
            c.g[V1][0] = c.lw(me.wrapping_add(RIDER));
            c.sq(2, s.wrapping_add(0xc0));
            c.lqc2(3, at(c, V1, POS));
            c.lqc2(4, me.wrapping_add(SCRIPT_FROM));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s.wrapping_add(0xe0));
            c.lq(2, s.wrapping_add(0xe0));
            c.sq(2, s.wrapping_add(0xd0));
            c.lqc2(3, s.wrapping_add(0xc0));
            vu0_length_vf3(c);
            c.cfc2(V1, 22);
            c.mtc1(V1, 0);
            c.lqc2(3, s.wrapping_add(0xd0));
            vu0_length_vf3(c);
            c.cfc2(V0, 22);
            c.mtc1(V0, 1);
            c.sub_s(0, 0, 20);
            c.c_lt_s(1, 0);
            let close = c.fcc();
            frame.restore(c, S0, 0x140);
            if close {
                c.cycles += 4;
                c.g[V0][0] = c.lw(pad) | 0x2000;
                c.sw(pad, c.g[V0][0]);
            }
        } else {
            // 0x10abdc: look for a prewind-end event (t0 = its index).
            c.cycles += 2;
            c.g[S3][0] = sx(me.wrapping_add(CONTROL));
            if (c.g[A3][0] as i64) > 0 {
                c.cycles += 4;
                c.g[V1][0] = c.lw(sp(c));
                c.g[V0][0] = EVENT_PREWIND_END;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                let mut found = c.g[V1][0] == EVENT_PREWIND_END;
                if found {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(pad);
                    c.g[T0][0] = 0;
                } else {
                    // 0x10ac00
                    loop {
                        c.cycles += 3;
                        let more = (c.g[A2][0] as i64) < (c.g[A3][0] as i64);
                        c.g[V0][0] = more as u64;
                        c.g[A0][0] = sx(r32(c, A2) << 4);
                        if !more {
                            break;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = EVENT_PREWIND_END;
                        c.g[V0][0] = sx(sp(c).wrapping_add(r32(c, A0)));
                        c.g[V1][0] = c.lw(r32(c, V0));
                        if c.g[V1][0] == c.g[A1][0] {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw(pad);
                            c.g[T0][0] = c.g[A2][0];
                            found = true;
                            break;
                        }
                        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                        if event_due(c) {
                            deliver_event_here(c, 0x0010_ac00);
                        }
                    }
                }
                if found {
                    c.cycles += 2;
                    c.g[V0][0] |= 0x2000;
                    c.sw(pad, c.g[V0][0]);
                }
            }
            // 0x10ac30
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            c.g[A0][0] = sx(r32(c, T0) << 4);
            if c.g[T0][0] != u64::MAX {
                // Steer towards the event point (relative to half the
                // look-ahead).
                c.cycles += 45;
                let s = sp(c);
                c.g[V0][0] = c.lw(me.wrapping_add(RIDER));
                c.lqc2(3, at(c, V0, AHEAD));
                c.lqc2(4, at(c, V0, POS));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s.wrapping_add(0xd0));
                lit(c, 0, 0x3f00_0000);
                c.lq(2, s.wrapping_add(0xd0));
                c.g[V1][0] = sx(s.wrapping_add(0xc).wrapping_add(r32(c, A0)));
                c.g[A1][0] = c.g[S0][0];
                c.sq(2, s.wrapping_add(0xc0));
                c.g[S0][0] = sx(s.wrapping_add(0xe0));
                c.mfc1(V0, 0);
                c.lqc2(4, s.wrapping_add(0xc0));
                c.qmtc2(V0, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, s.wrapping_add(0xe0));
                c.g[A0][0] = sx(s.wrapping_add(0xd0));
                c.lq(2, s.wrapping_add(0xe0));
                c.g[A2][0] = 0;
                c.sq(2, s.wrapping_add(0xc0));
                c.sq(2, s.wrapping_add(0xd0));
                c.lwc1(12, r32(c, V1));
                call_ptr(c, PATH_GET_POINT, 0x0010_aca0)?;
                let s = sp(c);
                c.lqc2(3, s.wrapping_add(0xd0));
                c.lqc2(4, s.wrapping_add(0xc0));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s.wrapping_add(0xf0));
                c.g[A0][0] = c.g[S2][0];
                c.lq(2, s.wrapping_add(0xf0));
                c.g[A1][0] = c.g[S0][0];
                c.sq(2, s.wrapping_add(0xe0));
                COMPUTE_STEER.call(c, 0x0010_acc4)?;
                lit(c, 1, 0x41f8_0000); // 31.0
                c.g[A0][0] = 0xffff_ffff_ffff_ffc0;
                c.g[V1][0] = c.lw(pad.wrapping_add(4));
                c.mul_s(0, 0, 1);
                c.g[V1][0] &= c.g[A0][0];
                c.cvt_w_s(1, 0);
                c.mfc1(V0, 1);
                c.g[V0][0] &= 0x3f;
                c.g[V1][0] |= c.g[V0][0];
                c.sw(pad.wrapping_add(4), c.g[V1][0]);
            }
            // 0x10acf0: the analog values (6 bits each, times 31).
            c.cycles += 26;
            let ctl = r32(c, S3);
            c.lwc1(0, ctl.wrapping_add(0x48));
            c.g[A0][0] = 0xffff_ffff_ffe0_7fff;
            lit(c, 1, 0x41f8_0000);
            c.g[V1][0] = c.lw(pad);
            c.g[A1][0] = 0xffff_ffff_f81f_ffff;
            c.mul_s(0, 0, 1);
            c.g[V1][0] &= c.g[A0][0];
            c.cvt_w_s(2, 0);
            c.mfc1(V0, 2);
            c.g[V0][0] &= 0x3f;
            c.g[V0][0] = sx(r32(c, V0) << 15);
            c.g[V1][0] |= c.g[V0][0];
            c.sw(pad, c.g[V1][0]);
            c.g[V1][0] &= c.g[A1][0];
            c.lwc1(0, ctl.wrapping_add(0x4c));
            c.mul_s(0, 0, 1);
            c.cvt_w_s(1, 0);
            c.mfc1(V0, 1);
            c.g[V0][0] &= 0x3f;
            c.g[V0][0] = sx(r32(c, V0) << 21);
            c.g[V1][0] |= c.g[V0][0];
            c.sw(pad, c.g[V1][0]);
            frame.restore(c, S0, 0x140);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore_ra(c, 0x100);
        frame.restore_fpr(c, 20, 0x150);
        frame.close(c);
        Ok(())
    })
}

/// `s0->vtable(v1)[slot]` with `a0 = s0 + delta`, the method in v0.
pub(crate) fn vcall_a0_s0(c: &mut Cpu, slot: u32, ret_to: u32) -> crate::call::Flow {
    c.g[A0][0] = c.lh(at(c, V1, slot));
    c.g[V0][0] = c.lw(at(c, V1, slot + 4));
    let method = r32(c, V0);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret_to)
}

// ------------------------------------------------------------ PATH_Update

/// `void PATH_Update(char mode)` (0x3d5128): the PATH music player's tick.
/// Runs only when PATH is not locked (byte 0x44f42d), is initialised (byte
/// 0x44f42c) and no player is current (0x44f430 == 0). It then stores the
/// mode byte, marks "busy" (current = -1), samples the time in ms, refreshes
/// the platform streams, and for each of the four players (0x517610, 0x928
/// bytes each) that is active (+0x34) makes it current (state pointer
/// 0x44f420, current = its handle at +0x30) and, with a handle, selects its
/// master track and updates its tracks. Finally it refreshes the platform
/// streams with reset and clears current and state.
pub fn path_update(c: &mut Cpu) {
    use crate::game::misc::path::MODE;
    use crate::game::misc::path::STATE;
    use crate::game::misc::path::TIME;
    /// Non-zero: PATH is locked, skip the tick.
    const LOCKED: u32 = 0x0044_f42d;
    /// Zero: PATH is not initialised.
    const READY: u32 = 0x0044_f42c;
    /// The current player's handle (-1 while updating, 0 when idle).
    const CURRENT: u32 = 0x0044_f430;
    /// The four players.
    const PLAYERS: u32 = 0x0051_7610;
    const PLAYER_SIZE: u32 = 0x928;
    const PLAYER_COUNT: u64 = 4;
    /// Player fields: its handle and its "active" word.
    const HANDLE: u32 = 0x30;
    const ACTIVE: u32 = 0x34;
    const PATH_SELECT_MASTER_TRACK: Callee = Callee {
        addr: 0x003d_4b00,
        f: ee::s14::PATH_SelectMasterTrack,
    };
    const PATH_UPDATE_TRACKS: Callee = Callee {
        addr: 0x003d_4f10,
        f: ee::s14::PATH_UpdateTracks,
    };
    const PATH_PLAT_GET_TIME_MS: u32 = 0x002a_f370;
    const PATH_PLAT_REFRESH_STATUS: u32 = 0x002a_efa8;
    /// The player loop head.
    const LOOP: u32 = 0x003d_51d8;

    guest(c, |c| {
        c.cycles += 15;
        c.g[V1][0] = 0x45_0000;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = Addr(LOCKED).lbu(c);
        c.g[A0][0] &= 0xff;
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);

        let mut run = c.g[V0][0] == 0;
        if run {
            c.cycles += 4;
            c.g[V0][0] = 0x45_0000;
            c.g[V1][0] = Addr(READY).lbu(c);
            run = c.g[V1][0] != 0;
            c.g[V1][0] = 0x45_0000;
        }
        if run {
            c.cycles += 3;
            c.g[V0][0] = Addr(CURRENT).lw(c);
            c.g[S1][0] = c.g[V1][0];
            run = c.g[V0][0] == 0;
        }
        if run {
            // Busy: current = -1; sample the time.
            c.cycles += 22;
            c.g[V0][0] = u64::MAX;
            c.g[V1][0] = 0x45_0000;
            c.sb(MODE, c.g[A0][0]);
            c.g[S2][0] = 0xffff_ffff_ffff_0000;
            c.sw(CURRENT, c.g[V0][0]);
            call_ptr(c, PATH_PLAT_GET_TIME_MS, 0x003d_519c)?;
            c.g[S0][0] = 0;
            c.g[V1][0] = 0x45_0000;
            c.g[A0][0] = 0;
            c.sw(TIME, c.g[V0][0]);
            c.g[S4][0] = 0x45_0000;
            call_ptr(c, PATH_PLAT_REFRESH_STATUS, 0x003d_51b4)?;

            // s0 = player index, a0 = its offset (s0 * 0x928, via mult).
            c.g[FP][0] = PLAYER_SIZE as u64;
            c.g[V0][0] = 0x51_0000;
            c.g[S2][0] |= 0xffff;
            c.g[S3][0] = sx(PLAYERS);
            c.g[S7][0] = u64::MAX;
            c.g[S6][0] = sx(PLAYERS + ACTIVE);
            c.g[S5][0] = sx(PLAYERS + HANDLE);
            c.mult(0, A0, S0, FP);
            loop {
                c.cycles += 4;
                let off = c.g[A0][0] as u32;
                c.g[V0][0] = sx(off.wrapping_add(r32(c, S6)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                let active = c.g[V1][0] != 0;
                c.g[V0][0] = Addr(CURRENT).lw(c);
                if active {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.sw(CURRENT, c.g[S2][0]);
                    }
                    c.cycles += 3;
                    let in_range = c.g[S0][0] < PLAYER_COUNT;
                    c.g[V0][0] = sx(off.wrapping_add(r32(c, S5)));
                    if in_range {
                        // Make it current: state = the player, current = handle.
                        c.cycles += 6;
                        c.g[V1][0] = sx(off.wrapping_add(r32(c, S3)));
                        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                        c.sw(STATE, c.g[V1][0]);
                        c.g[V0][0] = c.g[A0][0];
                        c.sw(CURRENT, c.g[A0][0]);
                    } else {
                        c.cycles += 2;
                        if c.g[S0][0] != c.g[S7][0] {
                            c.sw(CURRENT, 0);
                        }
                        c.cycles += 1;
                        c.g[V0][0] = Addr(CURRENT).lw(c);
                    }
                    c.cycles += 2;
                    let handle = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                    if handle {
                        c.cycles += 4;
                        PATH_SELECT_MASTER_TRACK.call(c, 0x003d_5230)?;
                        PATH_UPDATE_TRACKS.call(c, 0x003d_5238)?;
                        c.cycles += 1;
                        c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                    }
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                }
                c.cycles += 4;
                c.g[S0][0] = c.g[V0][0] & 0xff;
                c.g[V1][0] = (c.g[S0][0] < PLAYER_COUNT) as u64;
                c.mult(0, A0, S0, FP);
                if c.g[V1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOOP);
                }
            }
            // Refresh with reset; idle again.
            c.cycles += 4;
            c.g[A0][0] = 1;
            call_ptr(c, PATH_PLAT_REFRESH_STATUS, 0x003d_5254)?;
            c.sw(CURRENT, 0);
            c.sw(STATE, 0);
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

// ------------------------------------------------ Rider_TryEnterBoardPress

/// `bool Rider_TryEnterBoardPress(Rider* r, float dir)` (0x1161d0): start a
/// board press (nose press for `dir > 0`, tail press for `dir < 0`). Fails
/// (returns 0) for `dir == 0`, while the press sub-state (+0x328) is 3 or 4,
/// or, outside the main state 0, when the board's forward axis (+0x1e0) and
/// the velocity (+0x3a0) point apart (VU0 dot product < 0). Otherwise plays
/// animation 0x18 / 0x20 on the rider's anim controller (+0x784) with blend
/// -1, stores the press side (1 nose, 2 tail) at +0x330, enters sub-state
/// BOARD_PRESS and returns 1.
pub fn rider_try_enter_board_press(c: &mut Cpu) {
    /// Press sub-state (3 and 4 block a new press).
    const PRESS_STATE: u32 = 0x328;
    /// Press side: 1 nose, 2 tail.
    const PRESS_SIDE: u32 = 0x330;
    const FORWARD: u32 = 0x1e0;
    const VELOCITY: u32 = 0x3a0;
    const ANIM_CTL: u32 = 0x784;
    const ANIM_NOSE_PRESS: u64 = 0x18;
    const ANIM_TAIL_PRESS: u64 = 0x20;
    const BOARD_PRESS: u64 = 1;
    const ANIM_CONTROLLER_PLAY_ANIM: u32 = 0x0031_28e8;
    const RIDER_GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };
    const RIDER_SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fec8,
        f: ee::s00::Rider_SetSubState,
    };
    /// -1.0f: the anim blend argument.
    const MINUS_ONE: u32 = 0xbf80_0000;

    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save_fpr(c, 21, 0x28);
        frame.save_fpr(c, 20, 0x20);
        c.f[21] = 0;
        let dir = FpuF32(c.f[12]);
        c.f[20] = dir.0;
        frame.save(c, S0, 0x10);
        let no_dir = dir.eq_s(FpuF32::ZERO);
        abi::set_fcc(c, no_dir);
        frame.save_ra(c, 0);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        c.g[V0][0] = 0;
        let r = Addr(rider as u32);
        'out: {
            if no_dir {
                break 'out;
            }
            c.cycles += 5;
            let state = r.off(PRESS_STATE).lw(c);
            let blocked = (state as u32).wrapping_sub(3) < 2;
            c.g[V0][0] = 0;
            if blocked {
                break 'out;
            }
            c.cycles += 4;
            c.g[A0][0] = rider;
            RIDER_GET_MAIN_STATE.call(c, 0x0011_6218)?;
            if c.g[V0][0] == 0 {
                // forward . velocity (xyz) via VU0, in f0.
                c.cycles += 13;
                c.lqc2(3, r.off(FORWARD).0);
                c.lqc2(5, r.off(VELOCITY).0);
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(V0, 4);
                c.f[0] = c.g[V0][0] as u32;
                let apart = abi::c_lt_s(c, FpuF32(c.f[0]), FpuF32(c.f[21]));
                c.g[V0][0] = 0;
                if apart {
                    break 'out;
                }
            }
            c.cycles += 5;
            c.f[0] = 0;
            let nose = abi::c_lt_s(c, FpuF32::ZERO, dir);
            c.g[A0][0] = r.off(ANIM_CTL).lw(c);
            let (anim, side, ret) = if nose {
                c.cycles += 7;
                (ANIM_NOSE_PRESS, 1, 0x0011_627c)
            } else {
                c.cycles += 6;
                (ANIM_TAIL_PRESS, 2, 0x0011_6298)
            };
            c.g[A1][0] = anim;
            c.g[AT][0] = sx(MINUS_ONE);
            c.f[12] = MINUS_ONE;
            c.g[A2][0] = 0;
            call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, ret)?;
            c.g[V1][0] = side;
            c.cycles += 5;
            r.off(PRESS_SIDE).sw(c, side);
            c.g[A0][0] = rider;
            c.g[A1][0] = BOARD_PRESS;
            RIDER_SET_SUB_STATE.call(c, 0x0011_62ac)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------------------------- SNDMIDI_Service

/// `void SNDMIDI_Service(void)` (0x3b96b8): the MIDI sequencer tick. For each
/// of the 16 MIDI slots (0x50a120, 0x70 bytes each) that is playing (state
/// byte +0x28 == 3) and whose volume fade (+0x18, `SNDMIDI_UpdateFade`) did
/// not stop it: advance the tick counter (+0x0) and, while the scaled time
/// `ticks * tempo% (+0x24) / 100` has reached the next event's time (+0x4)
/// and the slot is still playing, process one event and count it.
pub fn sndmidi_service(c: &mut Cpu) {
    const SLOTS: u32 = 0x0050_a120;
    const SLOT_SIZE: u64 = 0x70;
    const SLOT_COUNT: i64 = 16;
    /// Slot fields.
    const TICKS: u32 = 0x0;
    const NEXT_EVENT: u32 = 0x4;
    const FADE: u32 = 0x18;
    const TEMPO: u32 = 0x24;
    const STATE: u32 = 0x28;
    const PLAYING: u64 = 3;
    const UPDATE_FADE: Callee = Callee {
        addr: 0x003b_8de8,
        f: ee::s13::SNDMIDI_UpdateFade,
    };
    const PROCESS_EVENT: Callee = Callee {
        addr: 0x003b_94b0,
        f: ee::s13::SNDMIDI_ProcessEvent,
    };

    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x51_0000;
        frame.save(c, S2, 0x70);
        frame.save(c, S5, 0x40);
        c.g[S2][0] = 0; // slot index
        frame.save(c, S6, 0x30);
        c.g[S5][0] = 100; // percent divisor
        frame.save(c, S7, 0x20);
        c.g[S6][0] = PLAYING;
        frame.save(c, FP, 0x10);
        c.g[S7][0] = SLOT_SIZE;
        frame.save(c, S0, 0x90);
        c.g[FP][0] = sx(SLOTS);
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save_ra(c, 0);
        c.mult(0, V0, S2, S7);
        loop {
            c.cycles += 4;
            let slot = (c.g[V0][0] as u32).wrapping_add(r32(c, FP));
            c.g[S0][0] = sx(slot);
            let e = Addr(slot);
            c.g[S3][0] = e.off(STATE).lb(c);
            c.g[S4][0] = sx(r32(c, S2).wrapping_add(1));
            // false: the path that reaches the slot loop's end directly.
            let mut via_tail = true;
            'slot: {
                if c.g[S3][0] != c.g[S6][0] {
                    break 'slot;
                }
                c.cycles += 3;
                c.g[V0][0] = e.off(FADE).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    UPDATE_FADE.call(c, 0x003b_9724)?;
                    if c.g[V0][0] != 0 {
                        // Stopped by its fade.
                        via_tail = false;
                        break 'slot;
                    }
                    c.cycles += 1;
                }
                c.g[V0][0] = e.off(TICKS).lw(c);
                // (divide-by-zero check of the divisor)
                c.cycles += 2;
                c.cycles += 10;
                c.g[A1][0] = e.off(TEMPO).lhu(c);
                c.g[S4][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(1));
                c.g[V1][0] = e.off(NEXT_EVENT).lw(c);
                c.mult(0, V0, V0, A1);
                c.divu(0, V0, S5);
                c.g[S1][0] = c.lo[0]; // scaled time
                c.g[V1][0] = (c.g[S1][0] < c.g[V1][0]) as u64;
                let early = c.g[V1][0] != 0;
                e.off(TICKS).sw(c, c.g[A0][0]);
                if early {
                    break 'slot;
                }
                c.cycles += 3;
                c.g[V0][0] = e.off(STATE).lb(c);
                if c.g[V0][0] != c.g[S3][0] {
                    via_tail = false;
                    break 'slot;
                }
                c.cycles += 1;
                c.g[S3][0] = PLAYING;
                loop {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    PROCESS_EVENT.call(c, 0x003b_9778)?;
                    c.g[V0][0] = e.off(NEXT_EVENT).lw(c);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.g[V1][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    let done = c.g[V1][0] != 0;
                    e.off(NEXT_EVENT).sw(c, c.g[V0][0]);
                    if done {
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = e.off(STATE).lb(c);
                    if c.g[V0][0] != c.g[S3][0] {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_9770);
                    }
                }
            }
            if via_tail {
                c.cycles += 1;
            }
            c.g[S2][0] = c.g[S4][0];
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < SLOT_COUNT) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.mult(0, V0, S2, S7);
            if event_due(c) {
                deliver_event_here(c, 0x003b_9700);
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

/// `sx(a + b)` on the low words (`addu`).
#[inline(always)]
pub(crate) fn add32(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

// ---------------------------------------------------------------------------
// libpad.

/// libpad's port table: 0x70 bytes per port, 0x1c per slot; `+0` the DMA
/// buffer (two 0x80-byte halves), `+0x10` non-zero while the port is open.
pub(crate) const PAD_PORTS: u32 = 0x0052_6250;

pub(crate) const PAD_PORT_SIZE: u64 = 0x70;

pub(crate) const PAD_SLOT_SIZE: u64 = 0x1c;

pub(crate) const MEMCPY: Callee = Callee {
    addr: 0x0041_605c,
    f: ee::s15::f_0041605c,
};

pub(crate) const SYNC_DCACHE: Callee = Callee {
    addr: 0x0042_4698,
    f: ee::s15::f_00424698,
};

/// `u8* libpad_GetDmaBuffer(int port, int slot)` (0x3ff9a8): the port's
/// 0x100-byte DMA buffer (written back from the data cache first), and of
/// its two 0x80-byte halves the one with the larger frame counter (`+0x58`
/// against `+0xd8`).
pub fn libpad_get_dma_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[V0][0] = PAD_SLOT_SIZE;
        c.g[V1][0] = PAD_PORT_SIZE;
        c.mult(0, A1, A1, V0);
        c.mult(1, A0, A0, V1);
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x52_0000;
        frame.save_ra(c, 0x10);
        c.g[V0][0] = sx(PAD_PORTS);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[A1][0] = add32(c.g[A1][0], c.g[A0][0]);
        c.g[A1][0] = add32(c.g[A1][0], c.g[V0][0]);
        c.g[S0][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = add32(c.g[S0][0], 0xff);
        SYNC_DCACHE.call(c, 0x003f_f9e4)?;
        let buf = r32(c, S0);
        c.g[V0][0] = Addr(buf + 0x58).lw(c);
        c.g[V1][0] = Addr(buf + 0xd8).lw(c);
        frame.restore_ra(c, 0x10);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.g[V0][0] = sx(r32(c, V0) << 7);
        c.g[V0][0] = add32(c.g[S0][0], c.g[V0][0]);
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod get_object_slot32 {

    /// `gp+0x25fc`: the default arguments are initialised.
    pub const GP_DEFAULTS_READY: u32 = 0x25fc;

    /// The default arguments (12 bytes at 0x4fbc00 = 0x500000 - 0x4400):
    /// object id -1, time -1.0, a byte at +8.
    pub const DEFAULTS_HI: u64 = 0x0050_0000;

    pub const DEFAULTS: u32 = 0x004f_bc00;

    /// `gp+0xba8`: the declared argument types (2 = float).
    pub const GP_ARG_TYPES: u32 = 0xba8;

    pub const TYPE_FLOAT: u64 = 2;

    /// `gp+0x16c8`: the script object manager (`**mgr + 8`: the per-class
    /// tables, `+0x1c` of a class: the handle table); `gp+0xce8`: the
    /// world (`+0x290`: the player object, for id -1).
    pub const GP_OBJECT_MAN: u32 = 0x16c8;

    pub const GP_WORLD: u32 = 0xce8;

    pub const WORLD_PLAYER: u32 = 0x290;

    /// `gp-0x3368`: 1/30 (frames to seconds).
    pub const GP_FRAME_TIME: u32 = 0x3368;

    /// `EA_MemCopy(dst, src, n)` and `LunoVMRegister_AsInt(reg)` (through
    /// the table).
    pub const MEM_COPY: u32 = 0x003e_6574;

    pub const REGISTER_AS_INT: u32 = 0x0022_6610;
}

pub(crate) mod libm_wrap {
    use super::*;

    /// `_LIB_VERSION` (-1 `_IEEE_`, 0 `_SVID_`, 2 `_POSIX_`), at 0x499688.
    pub const LIB_VERSION: u32 = 0x0049_9688;

    /// The name string "expf" the exception record points at.
    pub const EXPF_NAME: u32 = 0x0049_9088;

    /// `HUGE` (SVID overflow result, a double).
    pub const SVID_HUGE: u32 = 0x0049_9090;

    /// `HUGE_VAL` (a double).
    pub const HUGE_VAL: u32 = 0x0049_9680;

    /// `struct exception` on the frame: type, name, arg1, arg2, retval, err.
    pub const EXC_TYPE: u32 = 0x0;

    pub const EXC_NAME: u32 = 0x4;

    pub const EXC_ARG1: u32 = 0x8;

    pub const EXC_ARG2: u32 = 0x10;

    pub const EXC_RETVAL: u32 = 0x18;

    pub const EXC_ERR: u32 = 0x20;

    pub const IEEE754_EXPF: Callee = Callee {
        addr: 0x0040_e048,
        f: ee::s15::__ieee754_expf,
    };

    pub const FINITEF: Callee = Callee {
        addr: 0x0040_f8c0,
        f: ee::s15::finitef,
    };

    pub const EXTENDSFDF2: Callee = Callee {
        addr: 0x0041_3af8,
        f: ee::s15::__extendsfdf2,
    };

    pub const MATHERR: Callee = Callee {
        addr: 0x0040_f878,
        f: ee::s15::matherr,
    };

    pub const GETREENT: Callee = Callee {
        addr: 0x0041_5fb8,
        f: ee::s15::__getreent,
    };

    pub const TRUNCDFSF2: Callee = Callee {
        addr: 0x0041_27a8,
        f: ee::s15::__truncdfsf2,
    };
}

pub(crate) mod libpad {}

pub(crate) mod cruise {

    /// Computer actor: the rider (`Rider*` at +0x18; its velocity vector at
    /// +0x1e0) and the rider's speed at +0xdf8.
    pub const RIDER: u32 = 0x18;

    pub const RIDER_VELOCITY: u32 = 0x1e0;

    pub const SPEED: u32 = 0xdf8;

    /// `.lit4`: the fast-speed threshold (833.33) and the cruise crouch
    /// amount (0.97).
    pub const LIT_FAST: u32 = 0x7de4;

    pub const LIT_CROUCH: u32 = 0x7de0;

    /// The virtual pad word: crouch amount (0..31) in its top 6 bits.
    pub const PAD_CROUCH_KEEP: u64 = 0x03ff_ffff;

    pub const PAD_CROUCH_SHIFT: u32 = 26;
}

/// `LunoVMRegister* WScriptFn_GetObjectSlot32(LunoVMRegister* ret, int n,
/// Arg* args)` (0x3019c8): script function. The arguments (object id,
/// time) start from the defaults (-1, -1.0) and take each passed argument
/// `{idx, val, _, type}` (an int passed for a float slot is converted). The
/// object is the player for id -1, else looked up by class (low byte) and
/// handle (the rest). Answers, as an int register, the object's
/// controller's virtual `+0x100` (slot 32) called with `time / 30`, or 0
/// without a controller or object. Frame 0x70 (s0-s3; the arguments at
/// +0x10).
pub fn wscript_fn_get_object_slot32(c: &mut Cpu) {
    use get_object_slot32::*;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let gp = c.g[GP][0] as u32;
        let ready = c.lw(gp.wrapping_add(GP_DEFAULTS_READY));
        c.g[V0][0] = ready;
        frame.save(c, S3, 0x30);
        frame.save(c, S1, 0x50);
        let args = c.g[A2][0];
        c.g[S3][0] = args;
        frame.save(c, S2, 0x40);
        let ret = c.g[A0][0];
        c.g[S1][0] = ret;
        frame.save(c, S0, 0x60);
        let n = c.g[A1][0];
        c.g[S2][0] = n;
        frame.save_ra(c, 0x20);
        c.g[A2][0] = DEFAULTS_HI;
        if ready == 0 {
            // First use: defaults = { -1, -1.0f, (byte kept) }.
            c.cycles += 12;
            c.g[V1][0] = sx(DEFAULTS);
            c.g[V0][0] = u64::MAX;
            let b = c.lbu(DEFAULTS.wrapping_add(0x8));
            c.g[A1][0] = b;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 0);
            c.g[A0][0] = 1;
            c.sw(DEFAULTS, u64::MAX);
            c.sb(DEFAULTS.wrapping_add(0x8), b);
            c.swc1(0, DEFAULTS.wrapping_add(0x4));
            c.sw(gp.wrapping_add(GP_DEFAULTS_READY), 1);
            c.sb(sp(c), b);
        }

        // buf (sp+0x10) = defaults.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let buf = sp(c).wrapping_add(0x10);
        c.g[S0][0] = sx(buf);
        c.g[A1][0] = sx(DEFAULTS);
        c.g[A2][0] = 0xc;
        c.sw(buf, u64::MAX);
        c.g[A0][0] = sx(buf);
        call_ptr(c, MEM_COPY, 0x0030_1a48)?;
        c.g[A2][0] = args;
        if (n as i64) > 0 {
            // Each passed argument into its slot.
            c.cycles += 4;
            c.g[A1][0] = n;
            c.g[T0][0] = sx(gp.wrapping_add(GP_ARG_TYPES));
            c.g[A3][0] = TYPE_FLOAT;
            loop {
                c.cycles += 8;
                let arg = c.g[A2][0] as u32;
                let idx = c.lw(arg);
                let ty = c.lw(arg.wrapping_add(0xc));
                c.g[V1][0] = ty;
                let off = (idx as u32) << 2;
                let slot = buf.wrapping_add(off);
                c.g[A0][0] = sx(slot);
                let declared = c.lw((c.g[T0][0] as u32).wrapping_add(off));
                c.g[V0][0] = declared;
                if ty != declared && declared == TYPE_FLOAT {
                    c.cycles += 6;
                    c.lwc1(0, arg.wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, slot);
                } else {
                    if ty != declared {
                        c.cycles += 2;
                    }
                    c.cycles += 1;
                    let val = c.lw(arg.wrapping_add(0x4));
                    c.g[V0][0] = val;
                    c.sw(slot, val);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx(arg.wrapping_add(0x10));
                if c.g[A1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_1a60);
                }
            }
        }

        // The object: the player for id -1, else class table[id & 0xff],
        // handle table[id >> 8] >> 8, times 4.
        c.cycles += 4;
        let buf = c.g[S0][0] as u32;
        let id = c.lw(buf);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        let man = c.lw(gp.wrapping_add(GP_OBJECT_MAN));
        c.g[A1][0] = man;
        let obj = if id == u64::MAX {
            c.cycles += 3;
            let world = c.lw(gp.wrapping_add(GP_WORLD));
            c.g[V0][0] = world;
            c.lw((world as u32).wrapping_add(WORLD_PLAYER))
        } else {
            c.cycles += 8;
            let class_off = ((id & 0xff) as u32) << 2;
            c.g[V0][0] = sx(class_off);
            let m = c.lw(man as u32);
            c.g[V1][0] = m;
            let tables = c.lw((m as u32).wrapping_add(0x8));
            c.g[A0][0] = tables;
            let entry = class_off.wrapping_add(tables as u32);
            c.g[V0][0] = sx(entry);
            let class = c.lw(entry);
            c.g[V1][0] = class;
            c.g[V0][0] = sx((id as u32) >> 8);
            let mut obj = 0;
            if class != 0 {
                c.cycles += 7;
                let handles = c.lw((class as u32).wrapping_add(0x1c));
                c.g[A0][0] = handles;
                let h = ((id as u32) >> 8) << 2;
                c.g[V0][0] = sx(h.wrapping_add(handles as u32));
                let word = c.lw(h.wrapping_add(handles as u32));
                c.g[V1][0] = word;
                let index = (word as u32) >> 8;
                obj = sx(index << 2);
                if index == 0 {
                    c.g[V0][0] = obj;
                }
            }
            if obj == 0 {
                c.cycles += 1;
            }
            obj
        };
        c.g[V0][0] = obj;

        c.cycles += 2;
        if obj == 0 {
            // No object: 0.
            c.g[V0][0] = 1;
            c.cycles += 4;
            c.g[A0][0] = ret;
            c.sw((ret as u32).wrapping_add(0x8), 1);
            call_ptr(c, REGISTER_AS_INT, 0x0030_1b64)?;
            c.sw(c.g[V0][0] as u32, 0);
        } else {
            c.cycles += 3;
            let ctrl = c.lw((obj as u32).wrapping_add(0xc));
            c.g[A1][0] = ctrl;
            c.lwc1(0, buf.wrapping_add(0x4));
            let value = if ctrl != 0 {
                // ctrl->vslot32(time / 30).
                c.cycles += 9;
                let vt = c.lw((ctrl as u32).wrapping_add(0xc));
                c.g[V1][0] = vt;
                c.lwc1(12, gp.wrapping_sub(GP_FRAME_TIME));
                let slot = crate::call::VSlot::read(c, vt, 0x100);
                c.g[A0][0] = slot.delta;
                c.mul_s(12, 0, 12);
                c.g[V0][0] = slot.func;
                c.g[A0][0] = slot.this(ctrl);
                call_ptr(c, slot.func as u32, 0x0030_1b34)?;
                c.g[V0][0]
            } else {
                c.cycles += 1;
                0
            };
            c.g[S0][0] = value;
            c.cycles += 6;
            c.g[V0][0] = 1;
            c.g[A0][0] = ret;
            c.sw((ret as u32).wrapping_add(0x8), 1);
            call_ptr(c, REGISTER_AS_INT, 0x0030_1b50)?;
            c.sw(c.g[V0][0] as u32, value);
        }

        c.cycles += 8;
        c.g[V0][0] = ret;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `float expf(float x)` (0x40d758), newlib's `w_expf.c` wrapper: compute
/// `__ieee754_expf(x)`; unless `_LIB_VERSION` is `_IEEE_` or `x` is not
/// finite, report overflow (`x > 88.72284`, type 3) or underflow (`x <
/// -103.97208`, type 4) through `matherr` / `errno = ERANGE` (POSIX skips
/// `matherr`) and return the exception's (double) result as a float.
pub fn expf(c: &mut Cpu) {
    use libm_wrap::*;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save_fpr(c, 20, 0x60);
        c.mov_s(20, 12);
        let a = frame.at(c, 0x40);
        c.sd(a, c.g[S1][0]);
        let a = frame.at(c, 0x30);
        c.sd(a, c.g[S0][0]);
        frame.save_fpr(c, 21, 0x68);
        frame.save_ra(c, 0x50);
        c.g[S1][0] = 0x004a_0000;
        IEEE754_EXPF.call(c, 0x0040_d77c)?;
        c.g[S0][0] = c.lw(LIB_VERSION);
        c.g[V0][0] = u64::MAX;
        let ieee = c.g[S0][0] == c.g[V0][0];
        c.mov_s(21, 0); // the result
        'checked: {
            if ieee {
                break 'checked;
            }
            c.cycles += 4;
            c.mov_s(12, 20);
            FINITEF.call(c, 0x0040_d794)?;
            let finite = c.g[V0][0] != 0;
            c.mov_s(0, 21);
            if !finite {
                break 'checked;
            }
            let version = c.g[S0][0];
            // The error path: the exception record is filled, then
            // matherr (unless POSIX) and errno.
            let posix;
            c.cycles += 7;
            c.g[AT][0] = 0x42b1_7180; // 88.72284
            c.mtc1(AT, 0);
            c.c_lt_s(0, 20);
            let overflow = c.fcc();
            c.g[V0][0] = 0x004a_0000;
            if overflow {
                c.cycles += 10;
                c.g[V1][0] = 3;
                c.g[V0][0] = sx(EXPF_NAME);
                let sp0 = sp(c);
                c.sw(sp0 + EXC_TYPE, c.g[V1][0]);
                c.sw(sp0 + EXC_NAME, c.g[V0][0]);
                c.mov_s(12, 20);
                c.sw(sp0 + EXC_ERR, 0);
                EXTENDSFDF2.call(c, 0x0040_d7d4)?;
                let sp0 = sp(c);
                c.sd(sp0 + EXC_ARG1, c.g[V0][0]);
                c.sd(sp0 + EXC_ARG2, c.g[V0][0]);
                if version != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = 0x004a_0000;
                    c.g[V1][0] = c.ld(HUGE_VAL);
                    c.sd(sp0 + EXC_RETVAL, c.g[V1][0]);
                } else {
                    c.cycles += 4;
                    c.g[AT][0] = 0x004a_0000;
                    c.g[V0][0] = c.ld(SVID_HUGE);
                    c.sd(sp0 + EXC_RETVAL, c.g[V0][0]);
                }
                c.cycles += 4;
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_sub(0x6978));
                c.g[V0][0] = 2;
                posix = c.g[V1][0] == c.g[V0][0];
                if !posix {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 7;
                c.g[AT][0] = 0xffff_ffff_c2cf_f1b5; // -103.97208
                c.mtc1(AT, 0);
                c.c_lt_s(20, 0);
                let underflow = c.fcc();
                c.g[V1][0] = 4;
                if !underflow {
                    c.cycles += 1;
                    c.mov_s(0, 21);
                    break 'checked;
                }
                c.cycles += 12;
                c.g[V0][0] = sx(EXPF_NAME);
                let sp0 = sp(c);
                c.sw(sp0 + EXC_TYPE, c.g[V1][0]);
                c.sw(sp0 + EXC_NAME, c.g[V0][0]);
                c.mov_s(12, 20);
                c.sw(sp0 + EXC_ERR, 0);
                EXTENDSFDF2.call(c, 0x0040_d848)?;
                let sp0 = sp(c);
                c.g[V1][0] = 0;
                c.sd(sp0 + EXC_ARG1, c.g[V0][0]);
                c.sd(sp0 + EXC_RETVAL, c.g[V1][0]);
                c.g[V1][0] = 2;
                posix = c.g[S0][0] == c.g[V1][0];
                c.sd(sp0 + EXC_ARG2, c.g[V0][0]);
            }
            let mut set_errno = posix;
            if !posix {
                c.cycles += 4;
                c.g[A0][0] = c.g[SP][0];
                MATHERR.call(c, 0x0040_d868)?;
                let handled = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(sp(c) + EXC_ERR);
                set_errno = !handled;
            }
            if set_errno {
                // errno = ERANGE.
                c.cycles += 5;
                GETREENT.call(c, 0x0040_d878)?;
                c.g[V1][0] = 0x22;
                c.sw(r32(c, V0), c.g[V1][0]);
                c.g[V0][0] = c.lw(sp(c) + EXC_ERR);
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                // errno = exc.err.
                c.cycles += 4;
                GETREENT.call(c, 0x0040_d894)?;
                c.g[V1][0] = c.lw(sp(c) + EXC_ERR);
                c.sw(r32(c, V0), c.g[V1][0]);
            }
            c.cycles += 4;
            c.g[A0][0] = c.ld(sp(c) + EXC_RETVAL);
            TRUNCDFSF2.call(c, 0x0040_d8a4)?;
            frame.restore_ra(c, 0x50);
            return finish(c, frame);
        }
        c.cycles += 1;
        frame.restore_ra(c, 0x50);
        finish(c, frame)
    });

    /// The shared epilogue (ra already reloaded).
    fn finish(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
        c.cycles += 6;
        let a = frame.at(c, 0x40);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0x30);
        c.g[S0][0] = c.ld(a);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// ComputerActorRider_FindBoardPressMarker (0x10bbf8)

/// Computer rider: its rider (+0x18); rider: the course query object
/// (+0xab8) and the track position (+0x4c0) and distance (+0x4c4).
pub(crate) const CAR_RIDER: u32 = 0x18;

pub(crate) const RIDER_QUERY: u32 = 0xab8;

pub(crate) const RIDER_TRACK_POS: u32 = 0x4c0;

pub(crate) const RIDER_TRACK_DIST: u32 = 0x4c4;

/// The query object's vtable pointer (+0x34) and its "markers in range"
/// slot `{delta, fn}` at +0x8.
pub(crate) const QUERY_VTABLE: u32 = 0x34;

pub(crate) const QUERY_VT_MARKERS: u32 = 0x8;

/// Marker hits on the stack: 12 of 0x10 bytes `{type, value, ..}`.
pub(crate) const MAX_HITS: u64 = 12;

pub(crate) const HIT_SIZE: u32 = 0x10;

pub(crate) const MARKER_BOARD_PRESS: u64 = 0x13;

/// `bool ComputerActorRider_FindBoardPressMarker(ComputerActorRider* self,
/// int* press)` (0x10bbf8): ask the course for up to 12 markers between the
/// rider's track position and 50 units ahead; on the first board-press
/// marker (type 0x13) store whether its value is positive and return 1,
/// else `*press = 0` and return 0. The scan keeps its state in the
/// original's registers (v1 cursor, a0 index, a1 count) across the
/// back-edge event check.
pub fn computer_actor_rider_find_board_press_marker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xe0);
        c.g[AT][0] = 0x4248_0000;
        let ahead = FpuF32(0x4248_0000);
        c.f[0] = ahead.0;
        frame.save(c, S0, 0xd0);
        c.g[A2][0] = MAX_HITS;
        frame.save_ra(c, 0xc0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(CAR_RIDER).lw(c);
        let rider = Addr(c.g[V0][0] as u32);
        c.g[A3][0] = rider.off(RIDER_QUERY).lw(c);
        let query = c.g[A3][0] as u32;
        let dist = rider.off(RIDER_TRACK_DIST).read_fpu(c);
        c.g[V1][0] = Addr(query).off(QUERY_VTABLE).lw(c);
        let vt = Addr(c.g[V1][0] as u32);
        c.f[12] = rider.off(RIDER_TRACK_POS).read_fpu(c).0;
        c.f[13] = (dist + ahead).0;
        let delta = vt.off(QUERY_VT_MARKERS).lh(c) as u32;
        c.g[V0][0] = vt.off(QUERY_VT_MARKERS + 4).lw(c);
        c.g[A0][0] = sx(query.wrapping_add(delta));
        let markers = c.g[V0][0] as u32;
        call_ptr(c, markers, 0x0010_bc40)?;
        c.g[A1][0] = c.g[V0][0];
        let press = Addr(c.g[S0][0] as u32);
        press.sw(c, 0);
        c.g[A0][0] = 0;
        let mut found = false;
        if (c.g[A1][0] as i64) > 0 {
            c.cycles += 2;
            c.g[A2][0] = MARKER_BOARD_PRESS;
            c.g[V1][0] = c.g[SP][0];
            loop {
                c.cycles += 3;
                let hit = Addr(c.g[V1][0] as u32);
                c.g[V0][0] = hit.lw(c);
                if c.g[V0][0] == c.g[A2][0] {
                    c.cycles += 5;
                    let value = hit.off(4).lw(c);
                    c.g[V0][0] = 1;
                    c.g[V1][0] = (0 < value as i64) as u64;
                    let v = c.g[V1][0];
                    press.sw(c, v);
                    found = true;
                    break;
                }
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[V1][0] = sx(hit.0.wrapping_add(HIT_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_bc58);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0xd0);
        frame.restore_ra(c, 0xc0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EATimer_Rcnt1Handler (0x3e4db8)

/// EE timer 1 registers.
pub(crate) const T1_COUNT: u32 = 0x1000_0800;

pub(crate) const T1_MODE: u32 = 0x1000_0810;

pub(crate) const T1_COMP: u32 = 0x1000_0820;

/// T1_MODE: clear the compare interrupt flag (write 1).
pub(crate) const T1_MODE_EQUF: u64 = 0x400;

/// EATimer state: the 8 tick callbacks, tick counters, period (halfword)
/// and the "in callback" flag.
pub(crate) const EATIMER_CALLBACKS: u32 = 0x0045_0da8;

pub(crate) const EATIMER_TICKS: u32 = 0x0045_0dcc;

pub(crate) const EATIMER_TICKS2: u32 = 0x0045_0dd0;

pub(crate) const EATIMER_PERIOD: u32 = 0x0045_0de0;

pub(crate) const EATIMER_IN_CALLBACK: u32 = 0x0045_0de4;

/// `int EATimer_Rcnt1Handler(void)` (0x3e4db8): the timer 1 compare
/// interrupt. Advance the compare value by the period (relative to the
/// current count), reset the count, acknowledge the interrupt, bump both
/// tick counters, run each of the 8 registered callbacks with the
/// in-callback flag set, re-enable interrupts and return 0. The loop
/// keeps the original's registers (s1 slot pointer, s0 remaining) across
/// its back-edge event check.
pub fn ea_timer_rcnt1_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x0045_0000;
        frame.save(c, S0, 0x40);
        c.g[A1][0] = T1_COMP as u64;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[A0][0] = T1_COUNT as u64;
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = T1_MODE as u64;
        // Next compare = period + compare - count; restart the count.
        let period = Addr(EATIMER_PERIOD).lhu(c);
        let compare = Addr(T1_COMP).lw(c);
        c.g[T0][0] = 0x0045_0000;
        c.g[A3][0] = Addr(T1_COUNT).lw(c);
        c.g[T1][0] = 0x0045_0000;
        c.g[S2][0] = 0x0045_0000;
        let next = (period as u32)
            .wrapping_add(compare as u32)
            .wrapping_sub(c.g[A3][0] as u32);
        c.g[V0][0] = sx(next);
        Addr(T1_COMP).sw(c, sx(next));
        c.g[S1][0] = sx(EATIMER_CALLBACKS);
        Addr(T1_COUNT).sw(c, 0);
        c.g[S3][0] = 1;
        c.g[S0][0] = 7;
        // Acknowledge the interrupt.
        let mode = Addr(T1_MODE).lw(c) | T1_MODE_EQUF;
        Addr(T1_MODE).sw(c, mode);
        // Tick counters.
        c.g[V0][0] = sx((Addr(EATIMER_TICKS).lw(c) as u32).wrapping_add(1));
        let v = c.g[V0][0];
        Addr(EATIMER_TICKS).sw(c, v);
        c.g[V1][0] = sx((Addr(EATIMER_TICKS2).lw(c) as u32).wrapping_add(1));
        let v = c.g[V1][0];
        Addr(EATIMER_TICKS2).sw(c, v);
        loop {
            c.cycles += 3;
            let slot = Addr(c.g[S1][0] as u32);
            c.g[V0][0] = slot.lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                let callback = c.g[V0][0] as u32;
                let flag = c.g[S3][0];
                Addr(EATIMER_IN_CALLBACK).sw(c, flag);
                call_ptr(c, callback, 0x003e_4e5c)?;
                Addr(EATIMER_IN_CALLBACK).sw(c, 0);
            }
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
            c.cycles += 2;
            let more = (c.g[S0][0] as i64) >= 0;
            c.g[S1][0] = sx(slot.0.wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_4e48);
            }
        }
        c.cycles += 10;
        c.ei();
        frame.restore(c, S0, 0x40);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------- logf

/// `float logf(float x)` (0x40da10): newlib's `w_logf.c` wrapper. Returns
/// `__ieee754_logf(x)`; unless `_LIB_VERSION` is `_IEEE_` or `x` is NaN or
/// positive, it builds a `struct exception` on the stack ("log", `x` as
/// both doubles, retval `-HUGE` for `_SVID_` else `-HUGE_VAL`), sets type
/// `SING` for 0 / `DOMAIN` for negatives, sets `errno` (ERANGE / EDOM) for
/// `_POSIX_` or when `matherr` declines, copies the struct's `err` to
/// `errno` when nonzero, and returns the retval as a float.
pub fn logf(c: &mut Cpu) {
    const IEEE754_LOGF: Callee = Callee {
        addr: 0x0040_e550,
        f: ee::s15::__ieee754_logf,
    };
    const ISNANF: Callee = Callee {
        addr: 0x0040_f9d0,
        f: ee::s15::isnanf,
    };
    const EXTENDSFDF2: Callee = Callee {
        addr: 0x0041_3af8,
        f: ee::s15::__extendsfdf2,
    };
    const SUBDF3: Callee = Callee {
        addr: 0x0041_1f70,
        f: ee::s15::__subdf3,
    };
    const GETREENT: Callee = Callee {
        addr: 0x0041_5fb8,
        f: ee::s15::__getreent,
    };
    const MATHERR: Callee = Callee {
        addr: 0x0040_f878,
        f: ee::s15::matherr,
    };
    const TRUNCDFSF2: Callee = Callee {
        addr: 0x0041_27a8,
        f: ee::s15::__truncdfsf2,
    };
    /// `_LIB_VERSION` (`0x4a0000 - 0x6978`): -1 `_IEEE_`, 0 `_SVID_`, 2 `_POSIX_`.
    const HI: u32 = 0x004a_0000;
    const LIB_VERSION: u32 = HI - 0x6978;
    const IEEE: u64 = u64::MAX;
    const SVID: u32 = 0;
    const POSIX: u64 = 2;
    /// The name "log", the `-HUGE` (SVID) double, `HUGE_VAL` (+inf).
    const NAME_LOG: u32 = HI - 0x6f58;
    const MINUS_HUGE: u32 = HI - 0x6f50;
    const HUGE_VAL: u32 = HI - 0x6980;
    /// `struct exception` on the stack.
    const EXC_TYPE: u32 = 0x0;
    const EXC_NAME: u32 = 0x4;
    const EXC_ARG1: u32 = 0x8;
    const EXC_ARG2: u32 = 0x10;
    const EXC_RETVAL: u32 = 0x18;
    const EXC_ERR: u32 = 0x20;
    /// Exception types and errno values.
    const DOMAIN: u64 = 1;
    const SING: u64 = 2;
    const EDOM: u64 = 0x21;
    const ERANGE: u64 = 0x22;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save_fpr(c, 20, 0x60);
        let x = FpuF32(c.f[12]);
        c.f[20] = x.0;
        let a = frame.at(c, 0x40);
        c.sd(a, c.g[S1][0]);
        let a = frame.at(c, 0x30);
        c.sd(a, c.g[S0][0]);
        frame.save_fpr(c, 21, 0x68);
        frame.save_ra(c, 0x50);
        c.g[S1][0] = HI as u64;
        IEEE754_LOGF.call(c, 0x0040_da34)?;
        let version = Addr(LIB_VERSION).lw(c);
        c.g[S0][0] = version;
        c.g[V0][0] = IEEE;
        c.f[21] = c.f[0];
        let mut error = false;
        if version != IEEE {
            c.cycles += 4;
            c.f[12] = x.0;
            ISNANF.call(c, 0x0040_da4c)?;
            let nan = c.g[V0][0] != 0;
            c.f[0] = c.f[21];
            if nan {
                return finish(c, frame);
            }
            c.cycles += 5;
            c.f[0] = 0;
            let positive = FpuF32(0).lt(x);
            abi::set_fcc(c, positive);
            c.g[V0][0] = HI as u64;
            error = !positive;
        }
        if !error {
            c.cycles += 2;
            c.f[0] = c.f[21];
            return finish(c, frame);
        }

        // Fill the exception record.
        c.cycles += 8;
        let exc = |c: &Cpu, off: u32| sp(c).wrapping_add(off);
        Addr(exc(c, EXC_ERR)).sw(c, 0);
        c.g[V0][0] = sx(NAME_LOG);
        c.f[12] = x.0;
        Addr(exc(c, EXC_NAME)).sw(c, NAME_LOG as u64);
        EXTENDSFDF2.call(c, 0x0040_da84)?;
        let a = exc(c, EXC_ARG1);
        c.sd(a, c.g[V0][0]);
        let a = exc(c, EXC_ARG2);
        c.sd(a, c.g[V0][0]);
        if c.g[S0][0] == SVID as u64 {
            c.cycles += 4;
            c.g[AT][0] = HI as u64;
            c.g[V0][0] = c.ld(MINUS_HUGE);
        } else {
            c.cycles += 5;
            c.g[V0][0] = HI as u64;
            c.g[A0][0] = 0;
            c.g[A1][0] = c.ld(HUGE_VAL);
            SUBDF3.call(c, 0x0040_dab0)?;
        }
        let a = exc(c, EXC_RETVAL);
        c.sd(a, c.g[V0][0]);

        // Classify, then errno.
        c.cycles += 5;
        c.f[0] = 0;
        let zero = x.eq_s(FpuF32(0));
        abi::set_fcc(c, zero);
        c.g[V0][0] = SING;
        let errno_value;
        let mut ask_matherr = false;
        if zero {
            c.cycles += 3;
            c.g[V1][0] = Addr(LIB_VERSION).lw(c);
            Addr(exc(c, EXC_TYPE)).sw(c, SING);
            if c.g[V1][0] != POSIX {
                ask_matherr = true;
                errno_value = 0;
            } else {
                c.cycles += 4;
                GETREENT.call(c, 0x0040_dadc)?;
                errno_value = ERANGE;
            }
        } else {
            c.cycles += 5;
            c.g[V1][0] = DOMAIN;
            c.g[V0][0] = Addr(LIB_VERSION).lw(c);
            c.g[A0][0] = POSIX;
            Addr(exc(c, EXC_TYPE)).sw(c, DOMAIN);
            ask_matherr = c.g[V0][0] != POSIX;
            errno_value = EDOM;
        }
        let mut set_errno = !ask_matherr || errno_value == ERANGE;
        if ask_matherr {
            c.cycles += 4;
            c.g[A0][0] = c.g[SP][0];
            MATHERR.call(c, 0x0040_db00)?;
            let handled = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(exc(c, EXC_ERR)).lw(c);
            set_errno = !handled;
        }
        if set_errno {
            let code = if errno_value == ERANGE {
                ERANGE
            } else {
                // `DOMAIN`, or a declined `SING` (EDOM here as well).
                c.cycles += 3;
                GETREENT.call(c, 0x0040_db10)?;
                EDOM
            };
            c.g[V1][0] = code;
            c.cycles += 2;
            Addr(r32(c, V0)).sw(c, code);
            c.g[V0][0] = Addr(exc(c, EXC_ERR)).lw(c);
        }
        // A nonzero `err` from matherr goes to errno.
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            GETREENT.call(c, 0x0040_db2c)?;
            let err = Addr(exc(c, EXC_ERR)).lw(c);
            c.g[V1][0] = err;
            Addr(r32(c, V0)).sw(c, err);
        }
        c.cycles += 2;
        let a = exc(c, EXC_RETVAL);
        c.g[A0][0] = c.ld(a);
        TRUNCDFSF2.call(c, 0x0040_db3c)?;
        finish(c, frame)
    });

    /// The epilogue.
    fn finish(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
        c.cycles += 7;
        frame.restore_ra(c, 0x50);
        let a = frame.at(c, 0x40);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0x30);
        c.g[S0][0] = c.ld(a);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    }
}

// -------------------------------------- ComputerActorRider_CruiseChaseRider

/// `void ComputerActorRider_CruiseChaseRider(ComputerActorRider* self,
/// ControlInput* pad)` (0x100f88): the "chase a rider" cruise step. With
/// the target rider (AI rider slot `self+0xe70`) and the record of that
/// slot in the model (`model + slot*0x24`: distance +8, heading +0xc):
///
/// - heading difference to the target wrapped to [-pi, pi]; own and
///   target speeds (the target's velocity from its path interface, vtable
///   +0x10 at `+0x6c0`);
/// - behind (|angle| > pi/2): steer at the path, boost when slower than
///   the target, else cruise-crouch;
/// - otherwise the "close enough" angle `atan(150 / d)` (pi/2 at d == 0,
///   +pi when d < 0): outside it, or not faster than the target, steer and
///   crouch; else when the target is ready (vtable +0x48), re-select the AI
///   path if both follow the same one, then steer and crouch;
/// - pack the steering (`* 31`, 6 bits at bit 20) into the pad flags, then
///   resolve the jump state as the other cruise steps do.
///
/// The original's blocks are laid out in address order and entered by
/// falling out of the previous one; each `'b_…` label is one of them.
pub fn computer_actor_rider_cruise_chase_rider(c: &mut Cpu) {
    use crate::game::ai::computer::{cruise, jump_desc_copy, vu0_length_sq};
    const SELECT_AI_PATH: Callee = Callee {
        addr: 0x0011_2a50,
        f: ee::s00::Rider_SelectAIPath,
    };
    /// `float Math_Atan(float)`.
    const MATH_ATAN: u32 = 0x0031_c228;
    /// Rider fields.
    const R_MODEL: u32 = 0x18;
    const R_MARKER_DIST: u32 = 0xdf0;
    const R_TARGET: u32 = 0xe70;
    const R_JUMP_STATE: u32 = 0xf38;
    /// Rider-slot record in the model: `0x24` bytes, distance and heading.
    const REC_SIZE: u64 = 0x24;
    const REC_DIST: u32 = 0x8;
    const REC_HEADING: u32 = 0xc;
    /// Model fields: heading, velocity, followed AI path.
    const M_HEADING: u32 = 0x4cc;
    const M_VELOCITY: u32 = 0x1e0;
    const M_AI_PATH: u32 = 0xab8;
    /// The target rider entry's path interface (`{delta, fn}` slots).
    const E_IFACE: u32 = 0x6c0;
    const IFACE_GET_VELOCITY: u32 = 0x10;
    const IFACE_READY: u32 = 0x48;
    /// The jump descriptors (`0x450000 - 0x30e0`).
    const JUMP_DESCS: u32 = 0x0044_0000 - 0x30e0;
    let gp_lit = |c: &Cpu, off: u32| r32(c, GP).wrapping_sub(off);
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0xc0);
        let a = gp_lit(c, 0x7fa8);
        c.lwc1(0, a); // 1 / 2pi
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x70);
        c.g[S4][0] = REC_SIZE;
        frame.save(c, S3, 0x50);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 24, 0xb0);
        frame.save_fpr(c, 23, 0xa8);
        frame.save_fpr(c, 22, 0xa0);
        frame.save_fpr(c, 21, 0x98);
        frame.save_fpr(c, 20, 0x90);
        let this = r32(c, S0);
        c.g[V0][0] = Addr(gp_lit(c, 0x848)).lw(c);
        c.g[A0][0] = Addr(this).off(R_TARGET).lw(c);
        c.g[A2][0] = Addr(this).off(R_MODEL).lw(c);
        c.mult(0, A0, A0, S4);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 3); // 0.5
        c.lwc1(2, r32(c, A2).wrapping_add(M_HEADING));
        c.g[A1][0] = Addr(r32(c, V0)).off(0x84).lw(c);
        c.g[V1][0] = Addr(this).off(R_TARGET).lw(c);
        c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
        c.g[V0][0] = Addr(r32(c, A1)).off(0xc).lw(c);
        c.lwc1(1, r32(c, A0).wrapping_add(REC_HEADING));
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[V0][0] = abi::addu(c.g[V0][0], c.g[V1][0]);
        c.sub_s(1, 1, 2);
        c.g[S1][0] = Addr(r32(c, V0)).off(0x28).lw(c);
        // floor(diff / 2pi + 0.5)
        c.mul_s(0, 1, 0);
        c.add_s(0, 0, 3);
        c.cvt_w_s(2, 0);
        c.cvt_s_w(2, 2);
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.cycles += 3;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.sub_s(2, 2, 0);
        }

        'b_101200: {
            'b_1011a4: {
                'b_101190: {
                    'b_1010fc: {
                        c.cycles += 37;
                        let a = gp_lit(c, 0x7fa4);
                        c.lwc1(0, a); // 2pi
                        c.lqc2(3, r32(c, A2).wrapping_add(M_VELOCITY));
                        vu0_length_sq(c);
                        c.cfc2(V0, 22);
                        c.mtc1(V0, 24); // own speed
                        let entry = r32(c, S1);
                        c.g[V1][0] = Addr(entry).off(E_IFACE).lw(c);
                        c.g[A1][0] = sx(entry.wrapping_add(E_IFACE));
                        c.mul_s(0, 2, 0);
                        c.g[S3][0] = c.g[A1][0];
                        let vt = r32(c, V1);
                        c.g[A0][0] = Addr(vt).off(IFACE_GET_VELOCITY).lh(c);
                        c.g[V0][0] = Addr(vt).off(IFACE_GET_VELOCITY + 4).lw(c);
                        c.sub_s(22, 1, 0); // the wrapped angle
                        let target = r32(c, V0);
                        c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
                        call_ptr(c, target, 0x0010_1080)?;
                        c.mtc1(0, 21); // steer = 0
                        let a = gp_lit(c, 0x7fa0);
                        c.lwc1(1, a); // pi/2
                        c.abs_s(0, 22);
                        c.lqc2(3, r32(c, V0));
                        vu0_length_sq(c);
                        c.cfc2(V1, 22);
                        c.mtc1(V1, 23); // target speed
                        c.c_lt_s(1, 0);
                        let behind = c.fcc();
                        c.g[A0][0] = c.g[SP][0];
                        if !behind {
                            break 'b_1010fc;
                        }
                        c.cycles += 9;
                        c.g[A1][0] = c.g[S0][0];
                        cruise::GET_STEER_TARGET.call(c, 0x0010_10cc)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[SP][0];
                        cruise::COMPUTE_STEER.call(c, 0x0010_10d8)?;
                        c.c_lt_s(24, 23);
                        let slower = c.fcc();
                        c.mov_s(21, 0);
                        if !slower {
                            break 'b_101190;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S2][0];
                        cruise::CROUCH_AND_BOOST.call(c, 0x0010_10f4)?;
                        break 'b_101200;
                    }
                    // Ahead: the "close enough" angle from the distance.
                    'b_101178: {
                        'b_101154: {
                            'b_101128: {
                                c.cycles += 8;
                                c.g[V0][0] = Addr(this).off(R_TARGET).lw(c);
                                c.g[V1][0] = Addr(this).off(R_MODEL).lw(c);
                                c.mult(0, V0, V0, S4);
                                c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
                                c.lwc1(20, r32(c, V1).wrapping_add(REC_DIST));
                                c.c_eq_s(20, 21);
                                if !c.fcc() {
                                    break 'b_101128;
                                }
                                c.cycles += 3;
                                let a = gp_lit(c, 0x7f9c);
                                c.lwc1(1, a); // pi/2
                                break 'b_101154;
                            }
                            c.cycles += 9;
                            c.g[AT][0] = 0x4316_0000; // 150.0
                            c.mtc1(1, 12);
                            c.div_s(12, 12, 20);
                            call_ptr(c, MATH_ATAN, 0x0010_113c)?;
                            c.c_lt_s(20, 21);
                            let negative = c.fcc();
                            c.mov_s(1, 0);
                            if negative {
                                c.cycles += 2;
                                let a = gp_lit(c, 0x7f98);
                                c.lwc1(0, a); // pi
                                c.add_s(1, 1, 0);
                            }
                        }
                        c.cycles += 5;
                        c.abs_s(0, 22);
                        c.c_lt_s(1, 0);
                        let outside = c.fcc();
                        c.g[A0][0] = c.g[SP][0];
                        if outside {
                            break 'b_101178;
                        }
                        c.cycles += 4;
                        c.c_le_s(24, 23);
                        if !c.fcc() {
                            // Faster than the target: ask whether it is ready.
                            c.g[V1][0] = Addr(r32(c, S1)).off(E_IFACE).lw(c);
                            break 'b_1011a4;
                        }
                    }
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S0][0];
                    cruise::GET_STEER_TARGET.call(c, 0x0010_1180)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[SP][0];
                    cruise::COMPUTE_STEER.call(c, 0x0010_118c)?;
                    c.mov_s(21, 0);
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                cruise::SET_CRUISE_CROUCH.call(c, 0x0010_119c)?;
                break 'b_101200;
            }
            c.cycles += 6;
            let vt = r32(c, V1);
            c.g[A0][0] = Addr(vt).off(IFACE_READY).lh(c);
            c.g[V0][0] = Addr(vt).off(IFACE_READY + 4).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = abi::addu(c.g[S3][0], c.g[A0][0]);
            call_ptr(c, target, 0x0010_11b4)?;
            if c.g[V0][0] == 0 {
                break 'b_101200;
            }
            c.cycles += 5;
            c.g[A0][0] = Addr(this).off(R_MODEL).lw(c);
            c.g[V1][0] = Addr(r32(c, S1)).off(M_AI_PATH).lw(c);
            c.g[V0][0] = Addr(r32(c, A0)).off(M_AI_PATH).lw(c);
            if c.g[V1][0] == c.g[V0][0] {
                // Both follow the same AI path: pick another.
                c.cycles += 3;
                c.g[A1][0] = 0;
                SELECT_AI_PATH.call(c, 0x0010_11d8)?;
            }
            c.g[A0][0] = c.g[SP][0];
            c.cycles += 9;
            c.g[A1][0] = c.g[S0][0];
            cruise::GET_STEER_TARGET.call(c, 0x0010_11e4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            cruise::COMPUTE_STEER.call(c, 0x0010_11f0)?;
            c.mov_s(21, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            cruise::SET_CRUISE_CROUCH.call(c, 0x0010_1200)?;
        }

        // Pack the steering, then the jump state.
        'b_1012dc: {
            c.cycles += 24;
            c.g[AT][0] = 0x41f8_0000; // 31.0
            c.mtc1(1, 0);
            c.g[V0][0] = 0xffff_ffff_fc0f_0000;
            let pad = r32(c, S2);
            c.g[V1][0] = Addr(pad).lw(c);
            c.g[V0][0] |= 0xffff;
            c.mul_s(0, 21, 0);
            let sp0 = sp(c);
            Addr(sp0).off(0x24).sw(c, 0);
            c.g[V1][0] &= c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(this.wrapping_add(R_MARKER_DIST));
            c.g[A2][0] = sx(sp0.wrapping_add(0x10));
            c.cvt_w_s(1, 0);
            c.mfc1(V0, 1);
            c.g[T0][0] = sx(sp0.wrapping_add(0x18));
            c.g[A3][0] = sx(sp0.wrapping_add(0x14));
            c.g[T1][0] = sx(sp0.wrapping_add(0x1c));
            c.g[T2][0] = sx(sp0.wrapping_add(0x20));
            c.g[V0][0] &= 0x3f;
            c.g[V0][0] = sx(r32(c, V0) << 20);
            c.g[V1][0] |= c.g[V0][0];
            let flags = c.g[V1][0];
            Addr(pad).sw(c, flags);
            cruise::FIND_JUMP_MARKER.call(c, 0x0010_1258)?;
            let none = c.g[V0][0] == 0;
            c.g[V0][0] = u64::MAX;
            if !none {
                c.cycles += 9;
                c.g[V1][0] = 0x44_0000;
                Addr(this).off(R_TARGET).sw(c, u64::MAX);
                c.g[A2][0] = sx(JUMP_DESCS);
                jump_desc_copy(c, V0, JUMP_DESCS, 7, 0);
                Addr(this).off(R_JUMP_STATE).sh(c, 0);
                break 'b_1012dc;
            }
            'b_1012d8: {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx(sp(c).wrapping_add(0x24));
                cruise::SELECT_TARGET_RIDER.call(c, 0x0010_1290)?;
                let selected = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(sp(c)).off(0x24).lw(c);
                if !selected {
                    c.cycles += 8;
                    c.g[V0][0] = sx(JUMP_DESCS);
                    jump_desc_copy(c, A3, JUMP_DESCS, 0xf, 0x8);
                    c.g[V1][0] = 1;
                    break 'b_1012d8;
                }
                c.cycles += 2;
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = 0x44_0000;
                if none {
                    break 'b_1012dc;
                }
                c.cycles += 6;
                c.g[V1][0] = 3;
                c.g[V0][0] = sx(JUMP_DESCS);
                jump_desc_copy(c, A3, JUMP_DESCS, 0x1f, 0x18);
            }
            c.cycles += 1;
            let state = c.g[V1][0];
            Addr(this).off(R_JUMP_STATE).sh(c, state);
        }
        c.cycles += 13;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 24, 0xb0);
        frame.restore_fpr(c, 23, 0xa8);
        frame.restore_fpr(c, 22, 0xa0);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    });
}

/// `int EIntr(void)` (0x42c0c0): enable interrupts (`ei`); returns 1 if
/// they were already enabled.
pub fn e_intr(c: &mut Cpu) {
    c.cycles += 6;
    c.mfc0(V0, 12);
    c.g[V1][0] = STATUS_EIE;
    c.g[V0][0] &= c.g[V1][0];
    c.ei();
    c.g[V0][0] = (c.g[V0][0] != 0) as u64;
    ret(c);
}

/// `void ComputerActorRider_SetCruiseCrouch(ComputerActor* self, Pad* pad)`
/// (0x10c0a8): the AI's crouch while cruising: full (1.0) below the fast
/// speed (|velocity| < 833.33, VU0 length), 0.97 when faster, none below
/// speed 20; written as `crouch * 31` into the pad word's top 6 bits.
pub fn computer_actor_rider_set_cruise_crouch(c: &mut Cpu) {
    use cruise::*;
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.cycles += 17;
    let gp = r32(c, GP);
    let me = abi::arg(c, 0);
    c.lwc1(1, gp.wrapping_sub(LIT_FAST));
    c.g[V0][0] = Addr(me.wrapping_add(RIDER)).lw(c);
    c.lqc2(3, r32(c, V0).wrapping_add(RIDER_VELOCITY));
    // |velocity| (x, y, z) into Q, then f0.
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V1, 22);
    c.mtc1(V1, 0);
    c.lwc1(2, gp.wrapping_sub(LIT_CROUCH));
    c.c_lt_s(0, 1);
    let slow = c.fcc();
    c.lwc1(1, me.wrapping_add(SPEED));
    if slow {
        c.cycles += 2;
        c.g[AT][0] = 0x3f80_0000; // 1.0
        c.mtc1(AT, 2);
    }
    c.cycles += 5;
    c.g[AT][0] = 0x41a0_0000; // 20.0
    c.mtc1(AT, 0);
    c.c_lt_s(1, 0);
    if c.fcc() {
        c.cycles += 1;
        c.mtc1(0, 2);
    }
    c.cycles += 13;
    c.g[AT][0] = 0x41f8_0000; // 31.0
    c.mtc1(AT, 0);
    c.g[V1][0] = PAD_CROUCH_KEEP;
    let pad = Addr(abi::arg(c, 1));
    c.g[A0][0] = pad.lw(c);
    c.mul_s(0, 2, 0);
    c.g[A0][0] &= c.g[V1][0];
    c.cvt_w_s(1, 0);
    c.mfc1(V0, 1);
    c.g[V0][0] = sx(r32(c, V0) << PAD_CROUCH_SHIFT);
    c.g[A0][0] |= c.g[V0][0];
    let word = c.g[A0][0];
    pad.sw(c, word);
    ret(c);
}

// ---------------------------------------------------------------------------
// memcmp (0x415fc8)

/// `int memcmp(const void* a, const void* b, size_t n)` (0x415fc8): when
/// `n >= 16` and both pointers are 16-byte aligned, compare quadwords
/// (`lq`, `pxor`, fold the halves) while they match; the rest (or the
/// first differing quadword) byte by byte. Returns the difference of the
/// first differing bytes (unsigned), or 0.
pub fn memcmp(c: &mut Cpu) {
    const QUAD_LOOP: u32 = 0x0041_5fe0;
    const BYTE_LOOP: u32 = 0x0041_6030;
    c.cycles += 3;
    let short = c.g[A2][0] < 0x10;
    c.g[V0][0] = c.g[A0][0] | c.g[A1][0];
    'bytes: {
        if short {
            break 'bytes;
        }
        c.cycles += 3;
        c.g[V0][0] &= 0xf;
        if c.g[V0][0] != 0 {
            break 'bytes;
        }
        loop {
            c.cycles += 11;
            let a = r32(c, A0);
            let b = r32(c, A1);
            c.lq(3, a);
            c.g[A3][0] = (c.g[A2][0] < 0x20) as u64;
            c.lq(2, b);
            c.g[A0][0] = sx(a.wrapping_add(0x10));
            c.pxor(T0, V0, V1);
            c.g[V0][0] = sx(b.wrapping_add(0x10));
            c.pcpyud(T2, T0, A3);
            c.g[T1][0] = c.g[T2][0] | c.g[T0][0];
            if c.g[T1][0] != 0 {
                // A difference in this quadword: find it byte by byte.
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x10));
                break 'bytes;
            }
            c.g[A1][0] = c.g[V0][0];
            c.cycles += 2;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0x10));
            if c.g[A3][0] != 0 {
                break 'bytes;
            }
            if event_due(c) {
                deliver_event_here(c, QUAD_LOOP);
            }
        }
    }
    c.cycles += 5;
    c.g[V0][0] = u64::MAX;
    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
    if c.g[A2][0] != u64::MAX {
        c.cycles += 2;
        c.g[A3][0] = u64::MAX;
        loop {
            c.cycles += 4;
            let a = r32(c, A0);
            let b = r32(c, A1);
            c.g[V1][0] = Addr(a).lbu(c);
            c.g[V0][0] = Addr(b).lbu(c);
            c.g[A0][0] = sx(a.wrapping_add(1));
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                ret(c);
                return;
            }
            c.cycles += 3;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.g[A1][0] = sx(b.wrapping_add(1));
            if c.g[A2][0] == c.g[A3][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, BYTE_LOOP);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

pub(crate) mod sfile {

    /// The header's mutex (an `EAMutex`).
    pub const MUTEX: u32 = 0x4;

    /// Start and end of the chunk buffer.
    pub const BUFFER_START: u32 = 0x30;

    pub const BUFFER_END: u32 = 0x34;

    /// Read state: 2 = stalled waiting for space (set to 1 = reading again).
    pub const READ_STATE: u32 = 0x38;

    pub const STALLED: u64 = 2;

    /// Read sizes for the next request: normal, and after a low-water hit.
    pub const READ_SIZE: u32 = 0x3c;

    pub const READ_SIZE_LOW: u32 = 0x40;

    /// Set when the buffered bytes fell under the low-water mark.
    pub const LOW_WATER_HIT: u32 = 0x48;

    /// Chunk fields: status (-2 = released) and size in bytes.
    pub const CHUNK_STATUS: u32 = 0x0;

    pub const CHUNK_BYTES: u32 = 0x4;

    pub const RELEASED: u64 = 0xffff_ffff_ffff_fffe;
}

pub(crate) mod sfile_callee {
    use super::ee;
    use super::Callee;

    pub const VALIDATE: Callee = Callee {
        addr: 0x003d_fbd0,
        f: ee::s14::f_003dfbd0,
    };

    pub const CONSUME_BYTES: Callee = Callee {
        addr: 0x003d_fc48,
        f: ee::s14::f_003dfc48,
    };

    pub const MUTEX_LOCK: Callee = Callee {
        addr: 0x003e_5700,
        f: ee::s14::f_003e5700,
    };

    pub const MUTEX_UNLOCK: Callee = Callee {
        addr: 0x003e_5760,
        f: ee::s14::f_003e5760,
    };

    pub const ISSUE_READ: Callee = Callee {
        addr: 0x003e_03d0,
        f: ee::s14::f_003e03d0,
    };
}

pub(crate) mod hud_panel {

    /// Texture id stored into the draw state before drawing.
    pub const TEXTURE_ID: u32 = 0x474;

    /// Panel texture coordinates: v0, u0, u1, v1.
    pub const TEX_V0: u32 = 0x480;

    pub const TEX_U0: u32 = 0x484;

    pub const TEX_U1: u32 = 0x488;

    pub const TEX_V1: u32 = 0x48c;

    /// World (gp-0x854) -> draw state (+0xe84), whose texture slot is at +0x10.
    pub const WORLD: i32 = -0x854;

    pub const WORLD_DRAW_STATE: u32 = 0xe84;

    pub const DRAW_STATE_TEXTURE: u32 = 0x10;

    /// Width of each fixed end cap of a wide panel.
    pub const CAP: u32 = 0x4210_0000;
    // 36.0
    pub const WIDE: u32 = 0x4290_0000;
}

pub(crate) mod path_track {

    /// Track volume (signed byte, percent).
    pub const VOLUME: u32 = 0x4;

    /// Status byte: negative while the track is playing.
    pub const STATUS: u32 = 0xe;

    /// The track's own volume scale (signed byte, percent).
    pub const SCALE: u32 = 0x34;

    /// The platform voice index.
    pub const PLAT: u32 = 0x54;

    /// `PATH_PlatSetVolume(plat, volume)` (cross-shard).
    pub const PLAT_SET_VOLUME: u32 = 0x002a_f6c0;
}

/// `void StreamFile_ReleaseChunk(StreamFile *file, Chunk *chunk)` (0x3e13e8):
/// gives a consumed chunk's bytes back to the stream's ring buffer. Valid
/// file, chunk inside the buffer and not released yet: mark it released,
/// consume its bytes, and if the reader was stalled for space set it reading
/// again (under the file's mutex) and issue the next read.
pub fn stream_file_release_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        // Out parameters: header at 0(sp), handle at 4(sp).
        c.g[A2][0] = c.g[SP][0] | 0x4;
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        sfile_callee::VALIDATE.call(c, 0x003e_1408)?;
        release_chunk_validated(c, frame)?;
        frame.restore(c, S0, 0x30);
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The body of `StreamFile_ReleaseChunk` after `StreamFile_Validate` (s0 =
/// chunk), up to the reload of s0.
pub(crate) fn release_chunk_validated(c: &mut Cpu, frame: Frame) -> Flow {
    use sfile::*;
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    // The chunk must lie inside [start, end - 8].
    c.cycles += 5;
    let header = Addr(frame.at(c, 0)).lw(c);
    c.g[A0][0] = header;
    let start = Addr((header as u32).wrapping_add(BUFFER_START)).lw(c);
    c.g[V0][0] = (c.g[S0][0] < start) as u64;
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 5;
    let end = Addr((header as u32).wrapping_add(BUFFER_END)).lw(c);
    c.g[V0][0] = (sx((end as u32).wrapping_sub(8)) < c.g[S0][0]) as u64;
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(CHUNK_STATUS)).lw(c);
    c.g[V1][0] = RELEASED;
    c.g[S1][0] = STALLED;
    if c.g[V0][0] == RELEASED {
        c.cycles += 1;
        return Ok(());
    }
    c.cycles += 10;
    c.g[A1][0] = Addr(r32(c, S0).wrapping_add(CHUNK_BYTES)).lw(c);
    Addr(r32(c, S0).wrapping_add(CHUNK_STATUS)).sw(c, RELEASED);
    sfile_callee::CONSUME_BYTES.call(c, 0x003e_1454)?;
    let header = Addr(frame.at(c, 0)).lw(c);
    c.g[A0][0] = sx((header as u32).wrapping_add(MUTEX));
    sfile_callee::MUTEX_LOCK.call(c, 0x003e_1460)?;
    // s0 = the read state before the change (stalled -> reading).
    let header = Addr(frame.at(c, 0)).lw(c);
    c.g[V1][0] = header;
    c.g[S0][0] = Addr((header as u32).wrapping_add(READ_STATE)).lw(c);
    if c.g[S0][0] == c.g[S1][0] {
        c.cycles += 3;
        c.g[V0][0] = 1;
        Addr((header as u32).wrapping_add(READ_STATE)).sw(c, 1);
    }
    let header = Addr(frame.at(c, 0)).lw(c);
    c.cycles += 4;
    c.g[A0][0] = sx((header as u32).wrapping_add(MUTEX));
    sfile_callee::MUTEX_UNLOCK.call(c, 0x003e_1484)?;
    let was_stalled = c.g[S0][0] == c.g[S1][0];
    // The caller's s0 comes back before the read is issued (IssueRead's
    // prologue saves that value); the caller's own reload repeats it.
    frame.restore(c, S0, 0x30);
    if !was_stalled {
        return Ok(());
    }
    // Was stalled: issue the next read.
    c.cycles += 4;
    let header = Addr(frame.at(c, 0)).lw(c);
    c.g[A0][0] = header;
    c.g[V0][0] = Addr((header as u32).wrapping_add(LOW_WATER_HIT)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[A1][0] = Addr((header as u32).wrapping_add(READ_SIZE)).lw(c);
        sfile_callee::ISSUE_READ.call(c, 0x003e_14b4)?;
        c.cycles += 1;
    } else {
        c.cycles += 4;
        c.g[A1][0] = Addr((header as u32).wrapping_add(READ_SIZE_LOW)).lw(c);
        sfile_callee::ISSUE_READ.call(c, 0x003e_14a4)?;
    }
    Ok(())
}

pub(crate) const HUD_DRAW_PANEL_SEGMENT: Callee = Callee {
    addr: 0x0021_ea00,
    f: ee::s05::f_0021ea00,
};

/// FPU register `n` as a float.
#[inline]
pub(crate) fn hud_panel_fpr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `lwc1` from `base + off`.
#[inline]
pub(crate) fn hud_panel_lwc1(c: &mut Cpu, base: u64, off: u32) -> FpuF32 {
    Addr((base as u32).wrapping_add(off)).read_fpu(c)
}

/// `void HUD_DrawStretchedPanel(HUD *self, const Color *colour, float pos[2],
/// float size[2], float t, float scale_x, float scale_y, float alpha_min,
/// float alpha_max)` (0x21e7e0): draws a HUD panel scaled about its centre
/// (pos and size are updated in place), with its colour's first component
/// scaled by a triangle wave of `t` between `alpha_min` and `alpha_max`.
/// Panels wider than 72 are drawn as two 36-wide end caps and a stretched
/// middle (`HUD_DrawPanelSegment` three times), others as one segment.
pub fn hud_draw_stretched_panel(c: &mut Cpu) {
    use hud_panel::*;
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x80);
        let one = FpuF32(0x3f80_0000);
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = one.0;
        frame.save(c, S0, 0x50);
        let range = hud_panel_fpr(c, 16) - hud_panel_fpr(c, 15);
        c.f[16] = range.0;
        frame.save(c, S1, 0x40);
        let t = hud_panel_fpr(c, 12);
        let past_peak = abi::c_lt_s(c, one, t);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A3][0];
        frame.save_fpr(c, 22, 0x70);
        c.g[S3][0] = c.g[SP][0];
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        // A local copy of the colour (16 bytes, possibly unaligned source).
        let colour = r32(c, A1);
        c.g[V0][0] = Addr(colour).read_u64_unaligned(c);
        c.g[V1][0] = Addr(colour.wrapping_add(8)).read_u64_unaligned(c);
        let (lo, hi) = (c.g[V0][0], c.g[V1][0]);
        Addr(frame.at(c, 0)).write_u64_unaligned(c, lo);
        Addr(frame.at(c, 8)).write_u64_unaligned(c, hi);
        let red = Addr(frame.at(c, 0)).read_fpu(c);
        c.f[1] = red.0;
        // Triangle wave: t for t <= 1, else 2 - t.
        let wave = if past_peak {
            c.cycles += 5;
            c.g[AT][0] = 0x4000_0000;
            let fall = FpuF32(0x4000_0000) - t;
            range * fall
        } else {
            c.cycles += 1;
            range * t
        };
        c.cycles += 38;
        let alpha = hud_panel_fpr(c, 15) + wave;
        let red = red * alpha;
        c.f[12] = red.0;
        c.f[0] = one.0;
        let s0 = c.g[S0][0];
        let s2 = c.g[S2][0];
        let width = hud_panel_lwc1(c, s0, 0);
        let height = hud_panel_lwc1(c, s0, 4);
        let half = FpuF32(0x3f00_0000);
        c.f[6] = half.0;
        let x = hud_panel_lwc1(c, s2, 0);
        let y = hud_panel_lwc1(c, s2, 4);
        let grow_h = height * (hud_panel_fpr(c, 14) - one);
        let grow_w = width * (hud_panel_fpr(c, 13) - one);
        c.f[4] = grow_h.0;
        c.f[5] = grow_w.0;
        Addr(frame.at(c, 0)).write_fpu(c, red);
        c.g[AT][0] = sx(WIDE);
        c.f[22] = WIDE;
        let half_h = grow_h * half;
        let half_w = grow_w * half;
        c.f[2] = half_w.0;
        let y = y - half_h;
        let x = x - half_w;
        c.f[3] = x.0;
        Addr((s2 as u32).wrapping_add(4)).write_fpu(c, y);
        Addr(s2 as u32).write_fpu(c, x);
        let width = hud_panel_lwc1(c, s0, 0) + grow_w;
        let height = hud_panel_lwc1(c, s0, 4) + grow_h;
        c.f[0] = width.0;
        c.f[1] = height.0;
        Addr(s0 as u32).write_fpu(c, width);
        Addr((s0 as u32).wrapping_add(4)).write_fpu(c, height);
        // The draw state's texture.
        let world = Addr((c.g[GP][0] as u32).wrapping_add(WORLD as u32)).lw(c);
        c.g[V1][0] = world;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(TEXTURE_ID)).lw(c);
        c.g[V0][0] = Addr((world as u32).wrapping_add(WORLD_DRAW_STATE)).lw(c);
        let texture = c.g[A0][0];
        Addr(r32(c, V0).wrapping_add(DRAW_STATE_TEXTURE)).sh(c, texture);
        let width = hud_panel_lwc1(c, s0, 0);
        c.f[12] = width.0;
        let wide = abi::c_lt_s(c, FpuF32(WIDE), width);
        c.g[A0][0] = c.g[S1][0];
        if wide {
            draw_wide_panel(c)?;
        } else {
            c.cycles += 9;
            let this = c.g[A0][0];
            c.f[13] = hud_panel_lwc1(c, s0, 4).0;
            c.f[14] = hud_panel_lwc1(c, this, TEX_U0).0;
            c.g[A1][0] = c.g[S3][0];
            c.f[15] = hud_panel_lwc1(c, this, TEX_V0).0;
            c.g[A2][0] = c.g[S2][0];
            c.f[16] = hud_panel_lwc1(c, this, TEX_U1).0;
            c.f[17] = hud_panel_lwc1(c, this, TEX_V1).0;
            HUD_DRAW_PANEL_SEGMENT.call(c, 0x0021_e9d4)?;
        }
        frame.restore(c, S0, 0x50);
        c.cycles += 9;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 22, 0x70);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// The wide case of `HUD_DrawStretchedPanel`: left cap (u0..mid), middle
/// (mid..mid), right cap (mid..u1), with `pos[0]` advanced after each.
pub(crate) fn draw_wide_panel(c: &mut Cpu) -> Flow {
    use hud_panel::*;
    c.cycles += 46;
    let this = c.g[S1][0];
    let u0 = hud_panel_lwc1(c, this, TEX_U0);
    let u1 = hud_panel_lwc1(c, this, TEX_U1);
    c.f[0] = u1.0;
    c.g[A1][0] = c.g[S3][0];
    c.f[14] = u0.0;
    c.g[AT][0] = sx(CAP);
    c.f[21] = CAP;
    let size = c.g[S0][0];
    c.f[13] = hud_panel_lwc1(c, size, 4).0;
    c.f[15] = hud_panel_lwc1(c, this, TEX_V0).0;
    c.f[12] = CAP;
    c.f[17] = hud_panel_lwc1(c, this, TEX_V1).0;
    c.g[A2][0] = c.g[S2][0];
    let mid = (u0 + u1) * hud_panel_fpr(c, 6);
    c.f[20] = mid.0;
    c.f[16] = mid.0;
    HUD_DRAW_PANEL_SEGMENT.call(c, 0x0021_e938)?;

    // Middle: pos[0] += 36, width - 72.
    let (s0, s1, s2) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
    c.g[A0][0] = s1;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = s2;
    let x = hud_panel_lwc1(c, s2, 0) + hud_panel_fpr(c, 21);
    c.f[0] = x.0;
    c.f[14] = c.f[20];
    c.f[16] = c.f[20];
    Addr(s2 as u32).write_fpu(c, x);
    let width = hud_panel_lwc1(c, s0, 0);
    c.f[13] = hud_panel_lwc1(c, s0, 4).0;
    c.f[12] = (width - hud_panel_fpr(c, 22)).0;
    c.f[15] = hud_panel_lwc1(c, s1, TEX_V0).0;
    c.f[17] = hud_panel_lwc1(c, s1, TEX_V1).0;
    HUD_DRAW_PANEL_SEGMENT.call(c, 0x0021_e970)?;

    // Right cap: pos[0] += width - 72.
    let (s0, s1, s2) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
    let middle = hud_panel_lwc1(c, s0, 0) - hud_panel_fpr(c, 22);
    c.f[1] = middle.0;
    c.g[A0][0] = s1;
    let x = hud_panel_lwc1(c, s2, 0) + middle;
    c.f[0] = x.0;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = s2;
    c.f[12] = c.f[21];
    c.f[14] = c.f[20];
    Addr(s2 as u32).write_fpu(c, x);
    c.f[13] = hud_panel_lwc1(c, s0, 4).0;
    c.f[15] = hud_panel_lwc1(c, s1, TEX_V0).0;
    c.f[16] = hud_panel_lwc1(c, s1, TEX_U1).0;
    c.f[17] = hud_panel_lwc1(c, s1, TEX_V1).0;
    HUD_DRAW_PANEL_SEGMENT.call(c, 0x0021_e9ac)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Rider_ProbeRail (0x1086b8)

/// Rider: the path (`+0x2c`: nodes of 0x20 bytes, position then rotation).
pub(crate) const PROBE_RAIL_PATH: u32 = 0x780;

/// Rider: the current path node index.
pub(crate) const PROBE_RAIL_NODE: u32 = 0x8a0;

/// Rider: the probe offset added to the node position (`vec4`).
pub(crate) const PROBE_RAIL_OFFSET: u32 = 0x9d0;

/// Rider: the blend between the near and far probe radii (0..1).
pub(crate) const PROBE_RAIL_BLEND: u32 = 0x25c;

/// Rider: nonzero picks the short far radius (50 instead of 100).
pub(crate) const PROBE_RAIL_SHORT: u32 = 0x330;

/// Rider: the overlap list of objects near the rider.
pub(crate) const PROBE_RAIL_OVERLAP: u32 = 0x860;

/// `gp-0x7e3c`: 0.001f, the "lines are parallel" threshold.
pub(crate) const PROBE_RAIL_PARALLEL_GP: u32 = 0x7e3c;

/// `OverlapList_FindNearestRail(list, pos, out, mask, radius)`.
pub(crate) const PROBE_RAIL_FIND_NEAREST: u32 = 0x0033_4680;

/// The VU0 dot-product idiom of this function: `vf4 = vf[a] * vf[b]`, then
/// the w lane accumulates y, z and w through `vf6.w = 2` (vf0.w + vf0.w).
pub(crate) fn probe_rail_dot(c: &mut Cpu, a: u8, b: u8) {
    let v = c.vu0();
    v.add(0b1000, 6, 0, Src::Bc(0, W));
    v.mul(0b1111, 4, a, Src::Vec(b));
    v.adda(0b1000, 4, Src::Bc(4, Y));
    v.madda(0b1000, 6, Src::Bc(4, Z));
    v.madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `vf5 = vf3 (+|-) vf4` with vf3/vf4 loaded from `a`/`b`, through the
/// stack temporary `tmp` (`sqc2`, then `lq`/`sq` via r2) into `dst`.
pub(crate) fn probe_rail_vop(c: &mut Cpu, a: u32, b: u32, sub: bool, tmp: u32, dst: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    if sub {
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    } else {
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    }
    c.sqc2(5, tmp);
    c.lq(2, tmp);
    c.sq(2, dst);
}

/// `vf5 = vf4 * vf3.x` (vf4 loaded from `v`, vf3.x = the float in f[fs]
/// moved through GPR `rt`), stored to `tmp` and copied to `dst`.
pub(crate) fn probe_rail_scale(c: &mut Cpu, fs: usize, rt: usize, v: u32, tmp: u32, dst: u32) {
    c.mfc1(rt, fs);
    c.lqc2(4, v);
    c.qmtc2(rt, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, tmp);
    c.lq(2, tmp);
    c.sq(2, dst);
}

/// `|vec at addr| <= near radius (f20)`: the dot idiom on itself, `sqrt`
/// into Q, Q read back through v1 into f0, then `c.le.s f0, f20`.
pub(crate) fn probe_rail_within_near(c: &mut Cpu, addr: u32) -> bool {
    c.lqc2(3, addr);
    probe_rail_dot(c, 3, 3);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
    c.mtc1(3, 0);
    c.c_le_s(0, 20);
    c.fcc()
}

/// `int Rider_ProbeRail(Rider* self, const sVec4* target, const sVec4* dir)`
/// (0x1086b8): whether a rail runs close to the rider's probe point. The
/// probe is the current path node's position plus the probe offset, looking
/// along the node rotation's forward axis; the near radius is
/// `lerp(30, 50, blend)` and the far one `lerp(100 or 50, 170, blend)`.
/// The point on the probe segment (clamped to `±(far - near)`) closest to
/// `target` within the near radius is a hit; otherwise, unless `dir` is
/// (nearly) parallel to the probe, the point closest to the line through
/// `target` along `dir` is asked for the nearest rail in the overlap list
/// (`OverlapList_FindNearestRail`, radius 300), and that rail must lie within
/// the near radius. Returns 1 or 0.
pub fn rider_probe_rail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0x110);
        c.g[T0][0] = c.g[A0][0];
        let me = r32(c, T0);
        frame.save_fpr(c, 20, 0x100);
        frame.save_ra(c, 0xf0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 9); // 1.0
                       // node = &path->nodes[index]
        c.g[A0][0] = Addr(me).off(PROBE_RAIL_PATH).lw(c);
        c.g[V0][0] = Addr(me).off(PROBE_RAIL_NODE).lw(c);
        c.g[V1][0] = Addr(r32(c, A0)).off(0x2c).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 5);
        c.g[AT][0] = 0x4248_0000;
        c.mtc1(AT, 10); // 50.0
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        let node = r32(c, V1);
        c.g[AT][0] = 0x41f0_0000;
        c.mtc1(AT, 8); // 30.0
                       // The forward axis of the node rotation (quaternion x,y,z,w at
                       // +0x10): (1 - 2(y^2 + z^2), 2(xy + wz), 2(xz - wy)), w = 0.
        c.lwc1(4, node + 0x14);
        c.g[A0][0] = c.g[SP][0];
        let fwd = sp(c);
        c.lwc1(5, node + 0x18);
        c.lwc1(0, node + 0x10);
        c.mul_s(1, 4, 4);
        c.lwc1(3, node + 0x1c);
        c.mul_s(7, 5, 5);
        c.mul_s(2, 0, 5);
        Addr(fwd).off(0xc).sw(c, 0);
        c.mul_s(6, 3, 4);
        c.add_s(1, 1, 7);
        c.mul_s(0, 0, 4);
        c.mul_s(3, 3, 5);
        c.sub_s(2, 2, 6);
        c.add_s(1, 1, 1);
        c.add_s(0, 0, 3);
        c.add_s(2, 2, 2);
        c.sub_s(1, 9, 1);
        c.add_s(0, 0, 0);
        c.swc1(2, fwd + 8);
        c.swc1(1, fwd);
        c.swc1(0, fwd + 4);
        // pos (sp+0x10) = node position + probe offset (through sp+0x20).
        probe_rail_vop(
            c,
            node,
            me + PROBE_RAIL_OFFSET,
            false,
            fwd + 0x20,
            fwd + 0x10,
        );
        let pos = fwd + 0x10;
        // Radii: f20 = near = 50 t + 30 (1 - t); f0 = far.
        c.lwc1(2, me + PROBE_RAIL_BLEND);
        c.sub_s(9, 9, 2);
        c.g[V1][0] = Addr(me).off(PROBE_RAIL_SHORT).lw(c);
        c.mul_s(0, 2, 10);
        c.mul_s(8, 9, 8);
        c.add_s(20, 0, 8);
        if c.g[V1][0] == 0 {
            c.cycles += 8;
            c.g[AT][0] = 0x432a_0000;
            c.mtc1(AT, 1); // 170.0
            c.g[AT][0] = 0x42c8_0000;
            c.mtc1(AT, 0); // 100.0
            c.mul_s(1, 2, 1);
            c.mul_s(0, 9, 0);
            c.add_s(0, 1, 0);
        } else {
            c.cycles += 5;
            c.g[AT][0] = 0x432a_0000;
            c.mtc1(AT, 0); // 170.0
            c.mul_s(1, 9, 10); // (1 - t) * 50
            c.mul_s(0, 2, 0);
            c.add_s(0, 0, 1);
        }

        // Clamp the target's projection on the probe to [near - far, far - near].
        c.cycles += 22;
        let target = r32(c, A1);
        probe_rail_vop(c, target, pos, true, fwd + 0x30, fwd + 0x20);
        c.sub_s(4, 20, 0); // lo
        c.sub_s(8, 0, 20); // hi
        c.mov_s(1, 4);
        c.lqc2(3, fwd);
        c.lqc2(5, fwd + 0x20);
        probe_rail_dot(c, 3, 5);
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.c_le_s(4, 0);
        if c.fcc() {
            c.min_s(1, 0, 8);
        }

        // The closest point on the probe: within the near radius is a hit.
        c.cycles += 33;
        probe_rail_scale(c, 1, 3, fwd, fwd + 0x50, fwd + 0x40);
        probe_rail_vop(c, pos, fwd + 0x40, false, fwd + 0x50, fwd + 0x30);
        probe_rail_vop(c, fwd + 0x30, target, true, fwd + 0x50, fwd + 0x20);
        let near = probe_rail_within_near(c, fwd + 0x20);
        c.g[V0][0] = 1;
        if !near {
            // den = 1 - (dir . fwd)^2; parallel lines miss.
            c.cycles += 18;
            let dir = r32(c, A2);
            c.lqc2(3, dir);
            c.lqc2(5, fwd);
            probe_rail_dot(c, 3, 5);
            c.qmfc2(2, 4);
            c.mtc1(2, 3); // dd
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2);
            c.mul_s(0, 3, 3);
            c.lwc1(1, r32(c, GP).wrapping_sub(PROBE_RAIL_PARALLEL_GP));
            c.sub_s(2, 2, 0);
            c.c_lt_s(2, 1);
            let parallel = c.fcc();
            c.g[V0][0] = 0;
            if !parallel {
                // u = ((pos - target) / den) . (dir * dd - fwd), clamped.
                c.cycles += 42;
                probe_rail_vop(c, pos, target, true, fwd + 0x50, fwd + 0x40);
                c.mov_s(1, 4);
                c.mfc1(3, 2);
                c.qmtc2(3, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, fwd + 0x40);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 4, 4, Src::Q);
                c.sqc2(4, fwd + 0x50);
                c.lq(2, fwd + 0x50);
                c.sq(2, fwd + 0x30);
                probe_rail_scale(c, 3, 3, dir, fwd + 0x60, fwd + 0x50);
                probe_rail_vop(c, fwd + 0x50, fwd, true, fwd + 0x60, fwd + 0x40);
                c.lqc2(3, fwd + 0x30);
                c.lqc2(5, fwd + 0x40);
                probe_rail_dot(c, 3, 5);
                c.qmfc2(3, 4);
                c.mtc1(3, 0);
                c.c_le_s(4, 0);
                if c.fcc() {
                    c.min_s(1, 0, 8);
                }

                // p (sp+0x40) = pos + fwd * u; ask the overlap list.
                c.cycles += 22;
                c.mfc1(2, 1);
                c.lqc2(4, fwd);
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, fwd + 0x60);
                c.g[AT][0] = 0x4396_0000;
                c.mtc1(AT, 12); // radius 300.0
                c.lq(2, fwd + 0x60);
                c.g[A1][0] = sx(fwd + 0x40);
                c.g[A2][0] = sx(fwd + 0x70);
                c.g[A3][0] = 1;
                c.sq(2, fwd + 0x50);
                probe_rail_vop(c, pos, fwd + 0x50, false, fwd + 0x60, fwd + 0x40);
                c.g[A0][0] = Addr(me).off(PROBE_RAIL_OVERLAP).lw(c);
                call_ptr(c, PROBE_RAIL_FIND_NEAREST, 0x0010_89d0)?;
                let found = c.g[V0][0] != 0;
                c.g[V0][0] = 0;
                if found {
                    // The rail point must lie within the near radius of p.
                    c.cycles += 22;
                    let sp_now = sp(c);
                    c.lqc2(3, sp_now + 0x40);
                    c.lqc2(4, sp_now + 0x70);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp_now + 0x60);
                    c.g[A0][0] = 1;
                    c.lq(2, sp_now + 0x60);
                    c.sq(2, sp_now + 0x20);
                    c.sq(2, sp_now + 0x50);
                    if !probe_rail_within_near(c, sp_now + 0x20) {
                        c.g[A0][0] = 0;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[A0][0];
                }
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0xf0);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RaceStateRace_Update (0x113db0)

/// AI race-state object: the frame counter of the current state (+0xc).
pub(crate) const RACE_STATE_FRAMES: u32 = 0xc;

/// The world (gp-0x848) and its game module (+0x84).
pub(crate) const RACE_WORLD_GP: u32 = 0x848;

pub(crate) const RACE_WORLD_GAME_MODULE: u32 = 0x84;

/// The AI state "race finished" and the game module state it requests.
pub(crate) const RACE_AI_STATE_FINISHED: u64 = 0x6;

pub(crate) const RACE_MODULE_STATE_END_RACE: u64 = 0x5;

/// `AI_AllPlayersFinished` (0x12a250).
pub(crate) const AI_ALL_PLAYERS_FINISHED: Callee = Callee {
    addr: 0x0012_a250,
    f: ee::s00::AI_AllPlayersFinished,
};

/// `AI_SetRaceState` (0x113b10).
pub(crate) const AI_SET_RACE_STATE: Callee = Callee {
    addr: 0x0011_3b10,
    f: ee::s00::f_00113b10,
};

/// `GameModule_RequestState` (0x231250, through the function table).
pub(crate) const GAME_MODULE_REQUEST_STATE: u32 = 0x0023_1250;

/// `void RaceStateRace_Update(void *state, AI *ai /*a1*/)` (0x113db0): the
/// per-frame update of the AI's "race" state. Counts the frame
/// (`ai->frames++`); once every player has finished, switches the AI to the
/// finished state (6) and asks the game module for the end-of-race state
/// (`GameModule_RequestState(world->module, 5, 0, 0)`). Frame 0x20 (ra at
/// 0, s0 at 0x10).
pub fn race_state_race_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S0][0];
        let ai = Addr(r32(c, S0));
        let frames = sx((ai.off(RACE_STATE_FRAMES).lw(c) as u32).wrapping_add(1));
        c.g[V0][0] = frames;
        ai.off(RACE_STATE_FRAMES).sw(c, frames);
        AI_ALL_PLAYERS_FINISHED.call(c, 0x0011_3dd4)?;
        c.g[V1][0] = 1;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 10;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = RACE_AI_STATE_FINISHED;
            AI_SET_RACE_STATE.call(c, 0x0011_3dec)?;
            let world = Addr(r32(c, GP).wrapping_sub(RACE_WORLD_GP)).lw(c);
            c.g[V0][0] = world;
            c.g[A1][0] = RACE_MODULE_STATE_END_RACE;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[A0][0] = Addr(world as u32).off(RACE_WORLD_GAME_MODULE).lw(c);
            call_ptr(c, GAME_MODULE_REQUEST_STATE, 0x0011_3e04)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_ConsumeBytes (0x3dfc48)

/// Stream file fields: its mutex (+4), the read mode (+0x38), the refill
/// priority (+0x40), the low-water mark (+0x44), the "needs refill" flag
/// (+0x48), the buffered byte count (+0x4c) and the file handle (+0x174).
pub(crate) const STREAM_FILE_MUTEX: u32 = 0x4;

pub(crate) const STREAM_FILE_MODE: u32 = 0x38;

pub(crate) const STREAM_FILE_PRIORITY: u32 = 0x40;

pub(crate) const STREAM_FILE_LOW_WATER: u32 = 0x44;

pub(crate) const STREAM_FILE_NEEDS_REFILL: u32 = 0x48;

pub(crate) const STREAM_FILE_BUFFERED: u32 = 0x4c;

pub(crate) const STREAM_FILE_HANDLE: u32 = 0x174;

/// The read mode whose refills are reprioritised.
pub(crate) const STREAM_FILE_MODE_STREAMING: u64 = 1;

pub(crate) const STREAM_FILE_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};

pub(crate) const STREAM_FILE_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

pub(crate) const STREAM_FILE_SET_REQUEST_PRIORITY: Callee = Callee {
    addr: 0x003d_cfc0,
    f: ee::s14::EAFile_SetRequestPriority,
};

/// `void StreamFile_ConsumeBytes(StreamFile *st, int n)` (0x3dfc48): takes
/// `n` bytes off the buffered count (under the stream's mutex). When that
/// crosses the low-water mark, flags the stream for a refill and, for a
/// streaming read, raises its file request to the refill priority. Frame
/// 0x50 (ra at 0, s3 / s2 / s1 / s0 at 0x10 .. 0x40).
pub fn stream_file_consume_bytes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = sx(r32(c, S3).wrapping_add(STREAM_FILE_MUTEX));
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        STREAM_FILE_MUTEX_LOCK.call(c, 0x003d_fc74)?;
        let st = Addr(r32(c, S3));
        let old = st.off(STREAM_FILE_BUFFERED).lw(c);
        c.g[S0][0] = old;
        c.g[A0][0] = c.g[S1][0];
        let new = sx((old as u32).wrapping_sub(r32(c, S2)));
        c.g[S1][0] = new;
        st.off(STREAM_FILE_BUFFERED).sw(c, new);
        STREAM_FILE_MUTEX_UNLOCK.call(c, 0x003d_fc88)?;
        let st = Addr(r32(c, S3));
        let low = st.off(STREAM_FILE_LOW_WATER).lw(c);
        c.g[V0][0] = low;
        c.g[S0][0] = ((old as i64) < (low as i64)) as u64;
        if c.g[S0][0] == 0 {
            c.cycles += 3;
            let crossed = (new as i64) < (low as i64);
            c.g[V0][0] = 1;
            if crossed {
                c.cycles += 3;
                let mode = st.off(STREAM_FILE_MODE).lw(c);
                c.g[V1][0] = mode;
                st.off(STREAM_FILE_NEEDS_REFILL).sw(c, 1);
                if mode == STREAM_FILE_MODE_STREAMING {
                    c.cycles += 3;
                    c.g[A1][0] = st.off(STREAM_FILE_PRIORITY).lw(c);
                    c.g[A0][0] = st.off(STREAM_FILE_HANDLE).lw(c);
                    STREAM_FILE_SET_REQUEST_PRIORITY.call(c, 0x003d_fcbc)?;
                }
            }
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x2f5148 WorldLightMan_LightInstanceCB

/// `WorldLightMan_GetInstanceLighting(self, inst, force)` (0x2f5400).
pub(crate) const LIGHT_INSTANCE_GET_LIGHTING: Callee = Callee {
    addr: 0x002f_5400,
    f: ee::s09::WorldLightMan_GetInstanceLighting,
};

/// `void WorldLightMan_LightInstanceCB(Instance* inst)` (0x2f5148): the
/// world's per-instance lighting callback: looks up the instance's lighting
/// in the game module's light manager (`world->game_module->light_man`,
/// +0x84 then +0x90), forcing a re-light check when bit 12 of the
/// instance's flags (+8) is set.
pub fn world_light_man_light_instance_cb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        let world_slot = r32(c, GP).wrapping_sub(0x848);
        c.g[V1][0] = c.lw(world_slot);
        frame.save_ra(c, 0);
        let inst = r32(c, A0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = c.lw(inst.wrapping_add(0x8));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x84));
        c.g[A2][0] = sx((r32(c, A2) as i32 >> 12) as u32);
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x90));
        c.g[A2][0] &= 1;
        LIGHT_INSTANCE_GET_LIGHTING.call(c, 0x002f_5170)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PATH_TrackSetVolume(PathTrack* track, int volume)` (0x3d62c8):
/// store the volume (as a signed byte) and, while the track is playing
/// (status byte negative), apply `scale * volume / 100` to its platform
/// voice.
pub fn path_track_set_volume(c: &mut Cpu) {
    use path_track::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(r32(c, A1) << 24);
        frame.save_ra(c, 0);
        c.g[V1][0] = sx((r32(c, A1) as i32 >> 24) as u32);
        let track = r32(c, A0);
        c.sb(track.wrapping_add(VOLUME), c.g[V1][0]);
        c.g[V0][0] = c.lb(track.wrapping_add(STATUS));
        frame.restore_ra(c, 0);
        if (c.g[V0][0] as i64) < 0 {
            c.cycles += 4;
            c.g[A1][0] = c.lb(track.wrapping_add(SCALE));
            c.g[V0][0] = 100;
            c.cycles += 6;
            c.mult(0, A1, A1, V1);
            c.div(0, A1, V0);
            c.g[A1][0] = c.lo[0];
            c.g[A0][0] = c.lw(track.wrapping_add(PLAT));
            call_ptr(c, PLAT_SET_VOLUME, 0x003d_630c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_ClassifyChunk (0x3dfe88, seg_2DCAE8.c)

/// StreamFile fields: the chunk classifier table and its entry count.
pub(crate) const SFC_ENTRIES: u32 = 0x1c;

pub(crate) const SFC_COUNT: u32 = 0x20;

/// Classifier entry (12 bytes): `{mask, value, result}`.
pub(crate) const SFC_ENTRY_SIZE: u32 = 0xc;

pub(crate) const SFC_ENTRY_VALUE: u32 = 0x4;

pub(crate) const SFC_ENTRY_RESULT: u32 = 0x8;

/// `int StreamFile_ClassifyChunk(StreamFile* self, const int* chunk_id)`
/// (0x3dfe88): the result of the first classifier entry whose
/// `(id & mask) == value`, or -2 when none matches.
pub fn stream_file_classify_chunk(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.g[V1][0] = Addr(this.wrapping_add(SFC_COUNT)).lw(c);
    c.g[A2][0] = 0;
    c.g[A1][0] = Addr(r32(c, A1)).lw(c);
    if (c.g[V1][0] as i64) > 0 {
        c.cycles += 2;
        c.g[A0][0] = Addr(this.wrapping_add(SFC_ENTRIES)).lw(c);
        c.g[A3][0] = c.g[V1][0];
        loop {
            // 0x3dfea0
            c.cycles += 5;
            let entry = r32(c, A0);
            c.g[V0][0] = Addr(entry).lw(c);
            c.g[V1][0] = Addr(entry.wrapping_add(SFC_ENTRY_VALUE)).lw(c);
            c.g[V0][0] &= c.g[A1][0];
            let matched = c.g[V0][0] == c.g[V1][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            if matched {
                c.cycles += 2;
                c.g[V0][0] = Addr(entry.wrapping_add(SFC_ENTRY_RESULT)).lw(c);
                ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
            c.g[A0][0] = sx(entry.wrapping_add(SFC_ENTRY_SIZE));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_fea0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = sx(-2i32 as u32);
    ret(c);
}

// ---------------------------------------------------------------------------
// EA_ShellSortPairs (0x3e6328, seg_2DCAE8.c)

/// Shell sort gap sequence (1, 4, 13, 40, ...: 3h+1), `D_00451218`.
pub(crate) const OC264_SHELL_GAPS: u32 = 0x0045_1218;

/// `void EA_ShellSortPairs(int n, int* keys, int* vals)` (0x3e6328): sorts
/// `n` key/value pairs by key, descending, with a 3h+1 Shell sort (gap
/// table at 0x451218). A leaf; its loops keep their state in the original's
/// registers (t5 n, t2 keys, a2 vals, t6 gap pointer, t0 gap, a1/t3 i,
/// t1/t4 the pair being inserted, a3 j) so events see the same machine.
pub fn ea_shell_sort_pairs(c: &mut Cpu) {
    #[inline(always)]
    fn reg(c: &Cpu, r: usize) -> u32 {
        c.g[r][0] as u32
    }
    #[inline(always)]
    fn set(c: &mut Cpu, r: usize, v: u32) {
        c.g[r][0] = sx(v);
    }
    #[inline(always)]
    fn less(c: &Cpu, a: usize, b: usize) -> u64 {
        ((c.g[a][0] as i64) < (c.g[b][0] as i64)) as u64
    }

    // Find the first gap not below n: k = 1; while gaps[k] < n: k++.
    c.cycles += 8;
    c.g[T5][0] = c.g[A0][0];
    set(c, A0, OC264_SHELL_GAPS);
    c.g[T2][0] = c.g[A1][0];
    c.g[V0][0] = Addr(OC264_SHELL_GAPS + 4).lw(c);
    c.g[V0][0] = less(c, V0, T5);
    c.g[V1][0] = 1;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        set(c, A0, OC264_SHELL_GAPS + 4);
        loop {
            c.cycles += 7;
            let v = reg(c, A0).wrapping_add(4);
            set(c, A0, v);
            c.g[V0][0] = Addr(reg(c, A0)).lw(c);
            c.g[V0][0] = less(c, V0, T5);
            let v = reg(c, V1).wrapping_add(1);
            set(c, V1, v);
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_6350);
            }
        }
    }

    // t6 = &gaps[k - 1]; one insertion-sort pass per gap, down to gap 1.
    c.cycles += 5;
    let v = reg(c, V1) << 2;
    set(c, V1, v);
    set(c, V0, OC264_SHELL_GAPS);
    let v = reg(c, V1).wrapping_add(OC264_SHELL_GAPS).wrapping_sub(4);
    set(c, T6, v);
    loop {
        c.cycles += 4;
        c.g[T0][0] = Addr(reg(c, T6)).lw(c);
        c.g[V0][0] = less(c, T0, T5);
        c.g[A1][0] = c.g[T0][0];
        if c.g[V0][0] != 0 {
            // for i = gap; i < n; i++: insert (keys[i], vals[i]).
            loop {
                c.cycles += 8;
                let v = reg(c, A1) << 2;
                set(c, V0, v);
                let v = reg(c, A1).wrapping_add(1);
                set(c, T3, v);
                let v = reg(c, V0).wrapping_add(reg(c, A2));
                set(c, V1, v);
                let v = reg(c, A1).wrapping_sub(reg(c, T0));
                set(c, A3, v);
                let v = reg(c, V0).wrapping_add(reg(c, T2));
                set(c, V0, v);
                c.g[T4][0] = Addr(reg(c, V1)).lw(c);
                c.g[T1][0] = Addr(reg(c, V0)).lw(c);
                // j = i - gap; shift smaller keys up while j >= 0.
                loop {
                    c.cycles += 6;
                    let v = reg(c, A3) << 2;
                    set(c, A1, v);
                    c.g[V0][0] = Addr(reg(c, A1).wrapping_add(reg(c, T2))).lw(c);
                    let smaller = less(c, V0, T1) != 0;
                    let v = reg(c, A3).wrapping_add(reg(c, T0));
                    set(c, V1, v);
                    if !smaller {
                        break;
                    }
                    c.cycles += 9;
                    let v = reg(c, A1).wrapping_add(reg(c, A2));
                    set(c, A1, v);
                    let v = reg(c, V1) << 2;
                    set(c, V1, v);
                    let v = reg(c, A3).wrapping_sub(reg(c, T0));
                    set(c, A3, v);
                    let v = reg(c, V1).wrapping_add(reg(c, T2));
                    set(c, A0, v);
                    let key = c.g[V0][0];
                    Addr(reg(c, A0)).sw(c, key);
                    let v = reg(c, V1).wrapping_add(reg(c, A2));
                    set(c, V1, v);
                    c.g[V0][0] = Addr(reg(c, A1)).lw(c);
                    let more = (c.g[A3][0] as i64) >= 0;
                    let val = c.g[V0][0];
                    Addr(reg(c, V1)).sw(c, val);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_63b0);
                    }
                }
                // keys[j + gap] = key; vals[j + gap] = val.
                c.cycles += 9;
                let v = reg(c, A3).wrapping_add(reg(c, T0));
                set(c, V0, v);
                c.g[A1][0] = c.g[T3][0];
                let v = reg(c, V0) << 2;
                set(c, V0, v);
                c.g[A0][0] = less(c, A1, T5);
                let v = reg(c, V0).wrapping_add(reg(c, T2));
                set(c, V1, v);
                let v = reg(c, V0).wrapping_add(reg(c, A2));
                set(c, V0, v);
                let (key, val) = (c.g[T1][0], c.g[T4][0]);
                Addr(reg(c, V1)).sw(c, key);
                Addr(reg(c, V0)).sw(c, val);
                if c.g[A0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003e_6390);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T0][0] as i64) < 2) as u64;
        let v = reg(c, T6).wrapping_sub(4);
        set(c, T6, v);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003e_6380);
        }
    }
}

// ---------------------------------------------------------------------------
// ComputerActorRider_CruiseApproachJump (0x1009e0)

/// `ComputerActorRider_FindJumpMarker(self, &speed, &below, &o3, &o4, &o5, &o6)`.
pub(crate) const CAJ_FIND_JUMP_MARKER: Callee = Callee {
    addr: 0x0010_b980,
    f: ee::s00::ComputerActorRider_FindJumpMarker,
};

/// `ComputerActorRider_CruiseFollowPath(self, pad)`.
pub(crate) const CAJ_CRUISE_FOLLOW_PATH: Callee = Callee {
    addr: 0x0010_0680,
    f: ee::s00::ComputerActorRider_CruiseFollowPath,
};

/// `ComputerActorRider_GetSteerTarget(&v, self)`.
pub(crate) const CAJ_GET_STEER_TARGET: Callee = Callee {
    addr: 0x0010_0610,
    f: ee::s00::ComputerActorRider_GetSteerTarget,
};

/// `float ComputerActorRider_ComputeSteer(self, &v)`.
pub(crate) const CAJ_COMPUTE_STEER: Callee = Callee {
    addr: 0x0010_0348,
    f: ee::s00::ComputerActorRider_ComputeSteer,
};

/// `ComputerActorRider_PlanAirTricks(self, o5, o6, o4, o3)`.
pub(crate) const CAJ_PLAN_AIR_TRICKS: Callee = Callee {
    addr: 0x0010_bfa8,
    f: ee::s00::ComputerActorRider_PlanAirTricks,
};

/// `ComputerActorRider_CrouchAndBoost(self, pad)`.
pub(crate) const CAJ_CROUCH_AND_BOOST: Callee = Callee {
    addr: 0x0010_c140,
    f: ee::s00::ComputerActorRider_CrouchAndBoost,
};

/// ComputerActorRider: rider (+0x18), cruise speed (+0xdf0), think state
/// (+0xf38, `u16`) and think handler (+0xf44, an 8-byte member pointer).
pub(crate) const CAJ_RIDER: u32 = 0x18;

pub(crate) const CAJ_SPEED: u32 = 0xdf0;

pub(crate) const CAJ_THINK_STATE: u32 = 0xf38;

pub(crate) const CAJ_THINK_FN: u32 = 0xf44;

/// The think-handler table (0x43cf20); entry 1 (cruise) at +8.
pub(crate) const CAJ_THINK_TABLE: u32 = 0x0043_cf20;

/// Rider: velocity (+0x1e0) and height above ground (+0x4c8).
pub(crate) const CAJ_RIDER_VELOCITY: u32 = 0x1e0;

pub(crate) const CAJ_RIDER_HEIGHT: u32 = 0x4c8;

/// `gp-0x7fc4`: 10000.0 (height limit); `gp-0x7fc0`: 138.8889 (speed band).
pub(crate) const CAJ_HEIGHT_LIMIT_GP: u32 = 0x7fc4;

pub(crate) const CAJ_SPEED_BAND_GP: u32 = 0x7fc0;

/// `void ComputerActorRider_CruiseApproachJump(ComputerActorRider* self,
/// Pad* pad)` (0x1009e0): approaching a jump marker while cruising. No
/// marker ahead: go back to the cruise think state and follow the path.
/// Otherwise steer to the target (stick x, 6 bits at bit 20 of pad word 0),
/// and if the marker is below, the rider is not too high and the steer is
/// small, plan the air tricks and press both trick buttons (bits 14-15);
/// else crouch and boost when slower than the marker speed by more than
/// 138.9, or set the pad's slow-down bits (0x1f) when faster by more.
pub fn computer_actor_rider_cruise_approach_jump(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        let base = sp(c);
        c.g[A2][0] = sx(base.wrapping_add(0x10));
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x30);
        let me = r32(c, S0);
        c.g[A1][0] = sx(me.wrapping_add(CAJ_SPEED));
        c.g[A3][0] = sx(base.wrapping_add(0x14));
        c.g[T0][0] = sx(base.wrapping_add(0x18));
        c.g[T1][0] = sx(base.wrapping_add(0x1c));
        c.g[T2][0] = sx(base.wrapping_add(0x20));
        CAJ_FIND_JUMP_MARKER.call(c, 0x0010_0a14)?;
        let found = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[SP][0];
        if !found {
            // Back to cruising: think handler = table[1], state 1.
            c.cycles += 13;
            c.g[V1][0] = 1;
            c.g[V0][0] = sx(CAJ_THINK_TABLE);
            c.g[A0][0] = c.g[S0][0];
            c.ldl(A1, CAJ_THINK_TABLE + 0xf);
            c.ldr(A1, CAJ_THINK_TABLE + 0x8);
            let me = r32(c, S0);
            c.sdl(A1, me.wrapping_add(CAJ_THINK_FN + 7));
            c.sdr(A1, me.wrapping_add(CAJ_THINK_FN));
            c.g[A1][0] = c.g[S1][0];
            Addr(me.wrapping_add(CAJ_THINK_STATE)).sh(c, 1);
            CAJ_CRUISE_FOLLOW_PATH.call(c, 0x0010_0a48)?;
            frame.restore(c, S0, 0x50);
        } else {
            c.cycles += 33;
            c.g[A1][0] = c.g[S0][0];
            CAJ_GET_STEER_TARGET.call(c, 0x0010_0a58)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            CAJ_COMPUTE_STEER.call(c, 0x0010_0a64)?;
            // Stick x = (int)(steer * 31), 6 bits at bit 20 of pad word 0.
            let steer = FpuF32(c.f[0]);
            c.f[2] = steer.0;
            let pad = r32(c, S1);
            let word = Addr(pad).lw(c);
            c.g[AT][0] = 0x41f8_0000;
            c.g[V0][0] = 0xffff_ffff_fc0f_ffff;
            c.g[A0][0] = Addr(sp(c).wrapping_add(0x10)).lw(c);
            let scaled = steer * FpuF32(0x41f8_0000);
            c.f[0] = scaled.0;
            c.g[V1][0] = word & c.g[V0][0];
            c.f[1] = scaled.to_i32() as u32;
            c.g[V0][0] = sx(c.f[1]) & 0x3f;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
            c.g[V1][0] |= c.g[V0][0];
            let packed = c.g[V1][0];
            Addr(pad).sw(c, packed);
            // |rider velocity| (VU0 macro mode, as the original).
            let me = r32(c, S0);
            c.g[V1][0] = Addr(me.wrapping_add(CAJ_RIDER)).lw(c);
            let rider = r32(c, V1);
            c.lqc2(3, rider.wrapping_add(CAJ_RIDER_VELOCITY));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(V0, 22);
            let len = FpuF32(c.g[V0][0] as u32);
            c.f[3] = len.0;
            // Marker below, rider low enough and a small steer: air tricks.
            let mut tricks = false;
            if c.g[A0][0] != 0 {
                c.cycles += 6;
                let height = Addr(rider.wrapping_add(CAJ_RIDER_HEIGHT)).read_fpu(c);
                c.f[1] = height.0;
                let limit = Addr(r32(c, GP).wrapping_sub(CAJ_HEIGHT_LIMIT_GP)).read_fpu(c);
                c.f[0] = limit.0;
                if abi::c_lt_s(c, height, limit) {
                    c.cycles += 7;
                    c.g[AT][0] = 0x3f80_0000;
                    c.f[1] = 0x3f80_0000;
                    let mag = steer.abs();
                    c.f[0] = mag.0;
                    let small = abi::c_lt_s(c, mag, FpuF32(0x3f80_0000));
                    c.g[A1][0] = Addr(sp(c).wrapping_add(0x1c)).lw(c);
                    if small {
                        tricks = true;
                    } else {
                        c.cycles += 1;
                    }
                }
            }
            if tricks {
                c.cycles += 9;
                c.g[A0][0] = c.g[S0][0];
                let base = sp(c);
                c.g[A2][0] = Addr(base.wrapping_add(0x20)).lw(c);
                c.g[A3][0] = Addr(base.wrapping_add(0x18)).lw(c);
                c.g[T0][0] = Addr(base.wrapping_add(0x14)).lw(c);
                CAJ_PLAN_AIR_TRICKS.call(c, 0x0010_0b1c)?;
                let pad = r32(c, S1);
                c.g[V0][0] = Addr(pad).lw(c) | 0xc000;
                let v = c.g[V0][0];
                Addr(pad).sw(c, v);
                c.cycles += 1;
                frame.restore(c, S0, 0x50);
            } else {
                let me = r32(c, S0);
                let speed = Addr(me.wrapping_add(CAJ_SPEED)).read_fpu(c);
                c.f[1] = speed.0;
                c.cycles += 6;
                let band = Addr(r32(c, GP).wrapping_sub(CAJ_SPEED_BAND_GP)).read_fpu(c);
                c.f[2] = band.0;
                let low = speed - band;
                c.f[0] = low.0;
                let slow = abi::c_lt_s(c, len, low);
                c.g[A0][0] = c.g[S0][0];
                if slow {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    CAJ_CROUCH_AND_BOOST.call(c, 0x0010_0b50)?;
                    frame.restore(c, S0, 0x50);
                } else {
                    c.cycles += 5;
                    let high = speed + band;
                    c.f[0] = high.0;
                    let fast = abi::c_lt_s(c, high, len);
                    c.g[V1][0] = 0xffff_ffff_ffff_ffc0;
                    if fast {
                        c.cycles += 4;
                        let pad = r32(c, S1);
                        c.g[V0][0] = (Addr(pad.wrapping_add(4)).lw(c) & c.g[V1][0]) | 0x1f;
                        let v = c.g[V0][0];
                        Addr(pad.wrapping_add(4)).sw(c, v);
                    }
                    c.cycles += 1;
                    frame.restore(c, S0, 0x50);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// ComputerActorRider_ThinkNaturalAir (0x10b590)

/// `Rand_NextGame` (the AI's random generator, called through the table).
pub(crate) const NATURAL_AIR_RAND: u32 = 0x0031_7810;

pub(crate) const NATURAL_AIR_GET_STEER_TARGET: Callee = Callee {
    addr: 0x0010_0610,
    f: ee::s00::ComputerActorRider_GetSteerTarget,
};

pub(crate) const NATURAL_AIR_COMPUTE_STEER: Callee = Callee {
    addr: 0x0010_0348,
    f: ee::s00::ComputerActorRider_ComputeSteer,
};

/// AI think state (the `ComputerActorRider`).
pub(crate) const AI_RIDER: u32 = 0x18;

pub(crate) const AI_TRICK: u32 = 0xe0c;

pub(crate) const AI_IN_TRICK: u32 = 0xe10;

pub(crate) const AI_E14: u32 = 0xe14;

pub(crate) const AI_E18: u32 = 0xe18;

pub(crate) const AI_E20: u32 = 0xe20;

pub(crate) const AI_SPIN_DIR: u32 = 0xe38;

pub(crate) const AI_FLIP_DIR: u32 = 0xe3c;

/// Rider -> air predictor; its state and the height above the landing.
pub(crate) const RIDER_AIR: u32 = 0x788;

pub(crate) const AIR_HEIGHT: u32 = 0x98;

pub(crate) const AIR_GROUND: u32 = 0xa0;

pub(crate) const AIR_STATE: u32 = 0xac;

/// `void ComputerActorRider_ThinkNaturalAir(ComputerActorRider *self, Pad *pad)`
/// (0x10b590): the AI's "natural" air (no trick planned). With the air
/// predictor in state 1 or 3 the height above the landing decides: above 1
/// unit it spins (random direction), above 3 it also flips; below 0.5 with
/// neither, it steers toward the path target (`ComputeSteer * -31` into the
/// pad's stick bits 24..29). The trick slot (`self+0xe0c`, -1) goes to pad
/// byte 2.
pub fn computer_actor_rider_think_natural_air(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        c.g[V1][0] = 1;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x40);
        let ai = r32(c, S0);
        let rider = Addr(ai).off(AI_RIDER).lw(c);
        c.g[V0][0] = rider;
        let air = Addr(rider as u32).off(RIDER_AIR).lw(c);
        c.g[A0][0] = air;
        let state = Addr(air as u32).off(AIR_STATE).lw(c);
        c.g[A1][0] = state;
        // State 1 or 3: the predictor has a landing.
        let landing = if state == 1 {
            c.cycles += 1;
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 3;
            if state == 3 {
                c.cycles += 1;
            }
            state == 3
        };
        c.g[A2][0] = landing as u64;

        c.cycles += 2;
        let height = if landing {
            c.cycles += 4;
            let ground = Addr(air as u32).off(AIR_GROUND).read_fpu(c);
            let h = Addr(air as u32).off(AIR_HEIGHT).read_fpu(c);
            c.f[1] = ground.0;
            c.f[0] = h.0;
            h - ground
        } else {
            c.cycles += 1;
            FpuF32::ZERO
        };
        c.f[20] = height.0;

        // Defaults: no trick, no spin/flip.
        c.cycles += 11;
        let ai = Addr(ai);
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = FpuF32::ONE.0;
        c.g[V1][0] = 1;
        c.g[V0][0] = u64::MAX;
        ai.off(AI_IN_TRICK).sw(c, 0);
        let spin = abi::c_lt_s(c, FpuF32::ONE, height);
        ai.off(AI_TRICK).sw(c, u64::MAX);
        ai.off(AI_E14).sw(c, 0);
        ai.off(AI_E18).sw(c, 1);
        ai.off(AI_E20).sw(c, 0);
        if spin {
            c.cycles += 6;
            ai.off(AI_IN_TRICK).sw(c, 1);
            ai.off(AI_SPIN_DIR).write_fpu(c, FpuF32::ONE);
            call_ptr(c, NATURAL_AIR_RAND, 0x0010_b628)?;
            natural_air_random_sign(c, AI_SPIN_DIR);
        } else {
            c.cycles += 1;
            Addr(r32(c, S0)).off(AI_SPIN_DIR).sw(c, 0);
        }

        c.cycles += 6;
        let three = FpuF32::from_f32(3.0);
        c.g[AT][0] = 0x4040_0000;
        c.f[0] = three.0;
        let flip = abi::c_lt_s(c, three, height);
        c.g[V0][0] = 1;
        if flip {
            c.cycles += 8;
            c.g[AT][0] = 0x3f80_0000;
            c.f[0] = FpuF32::ONE.0;
            let ai = Addr(r32(c, S0));
            ai.off(AI_IN_TRICK).sw(c, 1);
            ai.off(AI_FLIP_DIR).write_fpu(c, FpuF32::ONE);
            call_ptr(c, NATURAL_AIR_RAND, 0x0010_b674)?;
            natural_air_random_sign(c, AI_FLIP_DIR);
        } else {
            c.cycles += 1;
            Addr(r32(c, S0)).off(AI_FLIP_DIR).sw(c, 0);
        }

        // Low and not tricking: steer toward the path.
        c.cycles += 6;
        let half = FpuF32::from_f32(0.5);
        c.g[AT][0] = 0x3f00_0000;
        c.f[0] = half.0;
        if abi::c_lt_s(c, height, half) {
            c.cycles += 6;
            let flip_dir = Addr(r32(c, S0)).off(AI_FLIP_DIR).read_fpu(c);
            c.f[0] = flip_dir.0;
            c.f[1] = 0;
            let still = flip_dir.eq_s(FpuF32::ZERO);
            abi::set_fcc(c, still);
            if still {
                c.cycles += 5;
                let spin_dir = Addr(r32(c, S0)).off(AI_SPIN_DIR).read_fpu(c);
                c.f[0] = spin_dir.0;
                let still = spin_dir.eq_s(FpuF32::ZERO);
                abi::set_fcc(c, still);
                c.g[A0][0] = c.g[SP][0];
                if still {
                    c.cycles += 21;
                    c.g[A1][0] = c.g[S0][0];
                    NATURAL_AIR_GET_STEER_TARGET.call(c, 0x0010_b6e0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[SP][0];
                    NATURAL_AIR_COMPUTE_STEER.call(c, 0x0010_b6ec)?;
                    let steer = -FpuF32(c.f[0]);
                    let k = FpuF32::from_f32(31.0);
                    c.g[AT][0] = 0x41f8_0000;
                    c.f[1] = k.0;
                    let pad = Addr(r32(c, S1));
                    let word = pad.lw(c);
                    c.g[V1][0] = 0xffff_ffff_c0ff_ffff;
                    c.g[A1][0] = u64::MAX;
                    let stick = steer * k;
                    c.f[0] = stick.0;
                    let bits = stick.to_i32() as u32;
                    c.f[1] = bits;
                    let field = sx((bits & 0x3f) << 24);
                    c.g[V0][0] = field;
                    let word = (word & 0xffff_ffff_c0ff_ffff) | field;
                    c.g[A0][0] = word;
                    pad.sw(c, word);
                    Addr(r32(c, S0)).off(AI_TRICK).sw(c, u64::MAX);
                }
                c.cycles += 1;
            }
        }
        c.g[V0][0] = Addr(r32(c, S0)).off(AI_TRICK).lbu(c);

        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        let trick = c.g[V0][0];
        Addr(r32(c, S1)).off(2).sb(c, trick);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// After `Rand_NextGame`: an odd result negates the direction at `field`.
pub(crate) fn natural_air_random_sign(c: &mut Cpu, field: u32) {
    c.g[V0][0] &= 1;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        let dir = Addr(r32(c, S0)).off(field);
        let v = -dir.read_fpu(c);
        c.f[0] = v.0;
        dir.write_fpu(c, v);
    }
}

/// `WScriptResult WScriptFn_Crowd2dControl(int argc, WScriptArg* args)`
/// (0x303ba0, hidden result pointer in a0): the world-script command that
/// adds, removes or binds a 2D crowd instance. The arguments (16 bytes
/// each: slot index, value, -, type) fill a 5-word buffer initialised from
/// a static default (`{-1, -1, 0, 0, ..}`, set up on first use); an int
/// given for a float slot (type table 0x4465d8) is converted. `buf[0]` is
/// the object handle (-1: the default object), `buf[1]` the mode (0 bind
/// frame, 1 nothing, 2 add, 3 remove) on the crowd manager (gp-0x6f0) with
/// the object's instance handle (`+0x78`) and `buf[2]`/`buf[3]`. Returns the
/// static "no value" result.
pub fn wscript_fn_crowd2d_control(c: &mut Cpu) {
    const INIT_GUARD: u32 = 0x26f0; // gp+
    const DEFAULTS: u32 = 0x004f_bc80;
    const ARG_TYPES: u32 = 0x0044_65d8;
    const HANDLE_TABLES: u32 = 0x16c8; // gp+
    const WORLD: u32 = 0xce8; // gp+
    const DEFAULT_OBJECT: u32 = 0x290;
    const INSTANCE: u32 = 0x78;
    const CROWD_MAN: u32 = 0x6f0; // gp-
                                  // The static "none" result is at 0x4c9098 (0x4d0000 - 0x6f68).
    const FLOAT_ARG: u64 = 2;
    const LOOP_HEAD: u32 = 0x0030_3c40;
    const MEMCPY: Callee = Callee {
        addr: 0x003e_6574,
        f: ee::s14::f_003e6574,
    };
    const BIND_INSTANCE_FRAME: Callee = Callee {
        addr: 0x0022_9820,
        f: ee::s05::f_00229820,
    };
    const CROWD_ADD: Callee = Callee {
        addr: 0x0022_9788,
        f: ee::s05::f_00229788,
    };
    const CROWD_REMOVE: Callee = Callee {
        addr: 0x0022_97d8,
        f: ee::s05::f_002297d8,
    };
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(gp.wrapping_add(INIT_GUARD)).lw(c);
        frame.save(c, S2, 0x50);
        frame.save(c, S1, 0x60);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[A2][0] = 0x0050_0000;
        if c.g[V0][0] == 0 {
            // Guarded static init of the default argument buffer.
            c.cycles += 13;
            c.g[V1][0] = sx(DEFAULTS);
            c.g[V0][0] = u64::MAX;
            let tag = Addr(DEFAULTS + 0x10).lbu(c);
            c.g[A1][0] = tag;
            Addr(DEFAULTS).sw(c, u64::MAX);
            c.g[A0][0] = u64::MAX;
            Addr(DEFAULTS + 0x10).sb(c, tag);
            c.g[V0][0] = 1;
            Addr(DEFAULTS + 4).sw(c, u64::MAX);
            Addr(gp.wrapping_add(INIT_GUARD)).sw(c, 1);
            Addr(DEFAULTS + 8).sw(c, 0);
            Addr(DEFAULTS + 0xc).sw(c, 0);
            Addr(sp(c)).sb(c, tag);
        }
        // buf = defaults (buf[0] = -1 first).
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let buf = sp(c).wrapping_add(0x10);
        c.g[S0][0] = sx(buf);
        c.g[A1][0] = sx(DEFAULTS);
        c.g[A2][0] = 0x14;
        Addr(buf).sw(c, u64::MAX);
        c.g[A0][0] = c.g[S0][0];
        MEMCPY.call(c, 0x0030_3c24)?;
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(ARG_TYPES);
            c.g[A3][0] = FLOAT_ARG;
            loop {
                c.cycles += 8;
                let arg = Addr(c.g[A2][0] as u32);
                let slot = arg.lw(c);
                c.g[V1][0] = arg.off(0xc).lw(c);
                let off = sx((slot as u32) << 2);
                c.g[A0][0] = sx((off as u32).wrapping_add(c.g[S0][0] as u32));
                let ty_addr = (off as u32).wrapping_add(c.g[T0][0] as u32);
                c.g[V0][0] = Addr(ty_addr).lw(c);
                let dst = c.g[A0][0] as u32;
                let convert = if c.g[V1][0] == c.g[V0][0] {
                    false
                } else {
                    c.cycles += 2;
                    c.g[V0][0] == c.g[A3][0]
                };
                if convert {
                    c.cycles += 4;
                    c.lwc1(0, arg.off(4).0);
                    c.cvt_s_w(0, 0);
                    c.swc1(0, dst);
                } else {
                    let v = arg.off(4).lw(c);
                    c.g[V0][0] = v;
                    c.cycles += 1;
                    Addr(dst).sw(c, v);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if c.g[A1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOOP_HEAD);
                }
            }
        }
        // Resolve the object handle.
        c.cycles += 4;
        let buf = Addr(c.g[S0][0] as u32);
        let id = buf.lw(c);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        c.g[A1][0] = Addr(gp.wrapping_add(HANDLE_TABLES)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            let world = Addr(gp.wrapping_add(WORLD)).lw(c);
            c.g[V0][0] = Addr(world as u32).off(DEFAULT_OBJECT).lw(c);
        } else {
            c.cycles += 8;
            let tables = Addr(c.g[A1][0] as u32).lw(c);
            c.g[V1][0] = tables;
            let sets = Addr(tables as u32).off(8).lw(c);
            c.g[A0][0] = sets;
            let set_addr = (((id & 0xff) as u32) << 2).wrapping_add(sets as u32);
            c.g[V0][0] = sx(set_addr);
            let set = Addr(set_addr).lw(c);
            c.g[V1][0] = set;
            c.g[V0][0] = sx((id as u32) >> 8);
            let mut found = false;
            if set != 0 {
                c.cycles += 7;
                let entries = Addr(set as u32).off(0x1c).lw(c);
                c.g[A0][0] = entries;
                let entry_addr = ((c.g[V0][0] as u32) << 2).wrapping_add(entries as u32);
                c.g[V0][0] = sx(entry_addr);
                let entry = Addr(entry_addr).lw(c);
                c.g[V1][0] = entry;
                let index = (entry as u32) >> 8;
                c.g[V0][0] = sx(index << 2);
                found = index != 0;
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 2;
        let obj = c.g[V0][0];
        c.g[V1][0] = 0x004d_0000;
        if obj != 0 {
            c.cycles += 5;
            let mode = buf.off(4).lw(c);
            c.g[A0][0] = mode;
            c.g[A3][0] = buf.off(0xc).lw(c);
            c.g[A1][0] = Addr(obj as u32).off(INSTANCE).lw(c);
            c.g[A2][0] = buf.off(8).lw(c);
            let crowd = gp.wrapping_sub(CROWD_MAN);
            if mode == 0 {
                c.cycles += 4;
                c.g[A0][0] = Addr(crowd).lw(c);
                BIND_INSTANCE_FRAME.call(c, 0x0030_3d08)?;
                c.g[V1][0] = 0x004d_0000;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                if mode == 1 {
                    c.cycles += 1;
                    c.g[V1][0] = 0x004d_0000;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 3;
                    if mode == 2 {
                        c.cycles += 4;
                        c.g[A0][0] = Addr(crowd).lw(c);
                        CROWD_ADD.call(c, 0x0030_3d2c)?;
                        c.g[V1][0] = 0x004d_0000;
                    } else {
                        c.cycles += 2;
                        c.g[V1][0] = 0x004d_0000;
                        if mode == 3 {
                            c.cycles += 2;
                            c.g[A0][0] = Addr(crowd).lw(c);
                            CROWD_REMOVE.call(c, 0x0030_3d44)?;
                            c.cycles += 1;
                            c.g[V1][0] = 0x004d_0000;
                        }
                    }
                }
            }
        }
        // *result = the static "none" result (12 bytes, unaligned copy).
        c.cycles += 15;
        let result = c.g[S3][0] as u32;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let src = c.g[A3][0] as u32;
        c.ldl(A0, src.wrapping_add(7));
        c.ldr(A0, src);
        let tail = Addr(src).off(8).lw(c);
        c.g[A1][0] = tail;
        c.sdl(A0, result.wrapping_add(7));
        c.sdr(A0, result);
        Addr(result).off(8).sw(c, tail);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RefPack_Decode (0x003b49d0)

/// `sx(a + b)` on the low words (`addu`/`addiu`).
#[inline]
pub(crate) fn refpack_addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// One RefPack byte copy loop: `do { *dst++ (t1) = *src++; } while (--v1
/// != end)` with `end` holding -1, an event check at `head` per byte.
#[inline]
pub(crate) fn refpack_copy(c: &mut Cpu, src: usize, end: usize, head: u32) {
    loop {
        c.cycles += 7;
        let b = Addr(r32(c, src)).lbu(c);
        c.g[V0][0] = b;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[src][0] = refpack_addu(c.g[src][0], 1);
        Addr(r32(c, T1)).sb(c, b);
        let more = c.g[V1][0] != c.g[end][0];
        c.g[T1][0] = refpack_addu(c.g[T1][0], 1);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `int RefPack_Decode(u8* dst, const u8* src, int* consumed)`
/// (0x003b49d0): EA RefPack (QFS) decompression. The header gives the
/// unpacked size (3 or 4 big-endian bytes, after an optional compressed
/// size when flag 0x100 is set); then control bytes: `0xxxxxxx` (2-byte
/// form: 0-3 literals, a 3-10 byte match up to 1 KiB back), `10xxxxxx`
/// (3-byte form: 4-67 bytes up to 16 KiB back), `110xxxxx` (4-byte form:
/// 5-1028 bytes up to 128 KiB back), `111xxxxx` below 0xfc (4-112
/// literals), and 0xfc-0xff (0-3 literals, end). Returns the unpacked size
/// (0 for a null `src`) and stores the bytes read to `*consumed` if given.
/// Registers as compiled: t1 = dst, a3 = src, t0 = control byte, t7 =
/// size, t6 = -1.
pub fn refpack_decode(c: &mut Cpu) {
    c.cycles += 4;
    c.g[T1][0] = c.g[A0][0];
    c.g[A3][0] = c.g[A1][0];
    c.g[T7][0] = 0;
    if c.g[A1][0] != 0 {
        refpack_decode_stream(c);
    }
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[A1][0] as u32));
    if c.g[A2][0] != 0 {
        c.cycles += 1;
        let consumed = c.g[V0][0];
        Addr(r32(c, A2)).sw(c, consumed);
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T7][0];
    abi::ret(c);
}

/// The header and the control-byte loop of [`refpack_decode`].
pub(crate) fn refpack_decode_stream(c: &mut Cpu) {
    // Header: 16-bit flags, an optional packed size, the unpacked size.
    c.cycles += 7;
    let src = r32(c, A1);
    let h0 = Addr(src).lbu(c);
    c.g[V0][0] = Addr(src).off(1).lbu(c);
    let flags = refpack_addu(h0 << 8, c.g[V0][0]);
    c.g[V1][0] = flags;
    c.g[A0][0] = flags & 0x8000;
    c.g[A3][0] = sx(src.wrapping_add(2));
    let four = c.g[A0][0] != 0;
    c.cycles += if four { 18 } else { 13 };
    c.g[V1][0] = flags & 0x100;
    c.g[V0][0] = sx(src.wrapping_add(if four { 6 } else { 5 }));
    if c.g[V1][0] != 0 {
        c.g[A3][0] = c.g[V0][0];
    }
    // t7 = big-endian size (v1/a0 alternate as the byte registers).
    let mut p = r32(c, A3);
    let mut size = Addr(p).lbu(c);
    p = p.wrapping_add(1);
    for (i, reg) in [V1, A0, V1].into_iter().enumerate() {
        if i == 2 && !four {
            break;
        }
        let b = Addr(p).lbu(c);
        c.g[reg][0] = b;
        c.g[V0][0] = sx((size as u32) << 8);
        p = p.wrapping_add(1);
        size = refpack_addu(c.g[V0][0], b);
    }
    c.g[T7][0] = size;
    c.g[A3][0] = sx(p);

    c.cycles += 4;
    c.g[T6][0] = u64::MAX;
    c.g[T2][0] = refpack_addu(c.g[A3][0], 1);
    'stream: loop {
        c.cycles += 1;
        c.g[T0][0] = Addr(r32(c, A3)).lbu(c);
        loop {
            // t0 = control byte at a3; t2 = a3 + 1.
            c.cycles += 3;
            let ctl = c.g[T0][0];
            c.g[V0][0] = ctl & 0x80;
            c.g[A3][0] = c.g[T2][0];
            if ctl & 0x80 == 0 {
                // 0ooccrrr oooooooo: r literals, then c+3 bytes from o+1 back.
                c.cycles += 7;
                c.g[A0][0] = Addr(r32(c, A3)).lbu(c);
                c.g[V1][0] = ctl & 0x3;
                c.g[T3][0] = ctl & 0x1c;
                c.g[A3][0] = refpack_addu(c.g[A3][0], 1);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                c.g[T2][0] = ctl & 0x60;
                if c.g[V1][0] != c.g[T6][0] {
                    c.cycles += 3;
                    c.g[T0][0] = u64::MAX;
                    refpack_copy(c, A3, T0, 0x003b_4ac0);
                }
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[T2][0] as u32) << 3);
                c.g[V1][0] = sx((c.g[T3][0] as u32) >> 2);
                c.g[V0][0] = !(c.g[A0][0] | c.g[V0][0]);
                c.g[A0][0] = refpack_addu(c.g[V0][0], c.g[T1][0]);
                c.g[V1][0] = refpack_addu(c.g[V1][0], 2);
                c.g[T2][0] = refpack_addu(c.g[A3][0], 1);
                c.g[T0][0] = u64::MAX;
                refpack_copy(c, A0, T0, 0x003b_4b00);
            } else {
                c.cycles += 3;
                c.g[V0][0] = ctl & 0x20;
                if ctl & 0x40 == 0 {
                    // 10cccccc rroooooo oooooooo: r literals, c+4 bytes.
                    c.cycles += 8;
                    c.g[A0][0] = Addr(r32(c, A3)).lbu(c);
                    c.g[T2][0] = ctl & 0x3f;
                    c.g[A3][0] = refpack_addu(c.g[A3][0], 1);
                    c.g[T3][0] = Addr(r32(c, A3)).lbu(c);
                    c.g[V1][0] = sx((c.g[A0][0] as u32) >> 6);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                    let none = c.g[V1][0] == c.g[T6][0];
                    c.g[A3][0] = refpack_addu(c.g[A3][0], 1);
                    if !none {
                        c.cycles += 2;
                        c.g[T0][0] = u64::MAX;
                        refpack_copy(c, A3, T0, 0x003b_4b58);
                    }
                    c.cycles += 9;
                    c.g[V0][0] = c.g[A0][0] & 0x3f;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                    c.g[V1][0] = refpack_addu(c.g[T2][0], 3);
                    c.g[V0][0] = !(c.g[T3][0] | c.g[V0][0]);
                    c.g[T2][0] = refpack_addu(c.g[A3][0], 1);
                    c.g[A0][0] = refpack_addu(c.g[V0][0], c.g[T1][0]);
                    c.g[T0][0] = u64::MAX;
                    refpack_copy(c, A0, T0, 0x003b_4b98);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = ctl & 0x1f;
                    if ctl & 0x20 == 0 {
                        // 110occrr oooooooo oooooooo cccccccc.
                        c.cycles += 11;
                        c.g[A0][0] = Addr(r32(c, A3)).lbu(c);
                        c.g[V1][0] = ctl & 0x3;
                        c.g[A3][0] = refpack_addu(c.g[A3][0], 1);
                        c.g[T5][0] = ctl & 0xc;
                        c.g[T3][0] = Addr(r32(c, A3)).lbu(c);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                        c.g[A3][0] = refpack_addu(c.g[A3][0], 1);
                        c.g[T2][0] = ctl & 0x10;
                        c.g[T4][0] = Addr(r32(c, A3)).lbu(c);
                        let none = c.g[V1][0] == c.g[T6][0];
                        c.g[A3][0] = refpack_addu(c.g[A3][0], 1);
                        if !none {
                            c.cycles += 2;
                            c.g[T0][0] = u64::MAX;
                            refpack_copy(c, A3, T0, 0x003b_4bf8);
                        }
                        c.cycles += 13;
                        c.g[V0][0] = (c.g[T2][0] != 0) as u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 8);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                        c.g[V1][0] = sx((c.g[T5][0] as u32) << 6);
                        c.g[V0][0] = refpack_addu(c.g[V0][0], c.g[A0][0]);
                        c.g[V1][0] = refpack_addu(c.g[V1][0], c.g[T4][0]);
                        c.g[V0][0] = !(c.g[T3][0] | c.g[V0][0]);
                        c.g[A0][0] = refpack_addu(c.g[V0][0], c.g[T1][0]);
                        c.g[V1][0] = refpack_addu(c.g[V1][0], 4);
                        c.g[T2][0] = refpack_addu(c.g[A3][0], 1);
                        c.g[T0][0] = u64::MAX;
                        refpack_copy(c, A0, T0, 0x003b_4c48);
                    } else {
                        // 111rrrrr: (r+1)*4 literals, or the end code.
                        c.cycles += 5;
                        c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V1][0] = refpack_addu(c.g[A0][0], 4);
                        let literals = c.g[V1][0] < 0x71;
                        c.g[V0][0] = literals as u64;
                        c.g[V1][0] = refpack_addu(c.g[A0][0], 3);
                        if !literals {
                            break 'stream;
                        }
                        c.cycles += 2;
                        let none = c.g[V1][0] == c.g[T6][0];
                        c.g[T2][0] = refpack_addu(c.g[A3][0], 1);
                        if !none {
                            c.cycles += 2;
                            c.g[A0][0] = u64::MAX;
                            refpack_copy(c, A3, A0, 0x003b_4c90);
                            c.cycles += 2;
                            c.g[T2][0] = refpack_addu(c.g[A3][0], 1);
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_4a88);
                        }
                        continue 'stream;
                    }
                }
            }
            // After a match: the next control byte.
            c.cycles += 2;
            c.g[T0][0] = Addr(r32(c, A3)).lbu(c);
            if event_due(c) {
                deliver_event_here(c, 0x003b_4a8c);
            }
        }
    }
    // End code: 0-3 trailing literals.
    c.cycles += 4;
    c.g[V1][0] = c.g[T0][0] & 0x3;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
    if c.g[V1][0] != c.g[T6][0] {
        c.cycles += 3;
        c.g[A0][0] = u64::MAX;
        refpack_copy(c, A3, A0, 0x003b_4cd0);
    }
}

// ---------------------------------------------------------------------------
// WorldObjFactory_Object (0x2fc0d0)
// ---------------------------------------------------------------------------

/// Guard word of the factory's static default parameter block.
pub(crate) const WOF_DEFAULTS_READY_GP: u32 = 0x24f0;

/// The static default parameter block `{id = -1, 0, 0, tag}` (16 bytes).
pub(crate) const WOF_DEFAULTS: u32 = 0x004f_b4f0;

/// Parameter kinds by slot (`D_00445F90`): kind 2 is a float slot.
pub(crate) const WOF_PARAM_KINDS: u32 = 0x0044_5f90;

pub(crate) const WOF_KIND_FLOAT: u64 = 2;

/// gp-relative: the world-object directory (`id & 0xff` selects a section,
/// `id >> 8` an object in it) and the world (its default object at +0x290).
pub(crate) const WOF_DIRECTORY_GP: u32 = 0x16c8;

pub(crate) const WOF_WORLD_GP: u32 = 0xce8;

pub(crate) const WOF_WORLD_DEFAULT_OBJECT: u32 = 0x290;

/// The factory's result value (12 bytes, returned by copy).
pub(crate) const WOF_RESULT: u32 = 0x004c_9098;

/// Object fields: the object's active node.
pub(crate) const WOF_OBJECT_NODE: u32 = 0xc;

/// `Result WorldObjFactory_Object(Param* params, int count)` (0x2fc0d0;
/// a0 = the hidden result slot, a1 = count, a2 = params): fills a 4-word
/// parameter block from the defaults and the `{slot, value, -, kind}`
/// parameters (an int given for a float slot is converted), looks the object
/// id up (-1: the world's default object), and unless the object already
/// has an active node of kind 0x11 for these parameters, creates an
/// `Object` (0x2c bytes) for it. Returns the factory's result value.
pub fn world_obj_factory_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        let ready = Addr(c.g[GP][0] as u32).off(WOF_DEFAULTS_READY_GP).lw(c);
        c.g[V0][0] = ready;
        frame.save(c, S3, 0x40);
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[A2][0] = 0x50_0000;
        frame.save_ra(c, 0x20);
        let defaults = Addr(WOF_DEFAULTS);
        if ready == 0 {
            // First use: initialise the static defaults (guarded local).
            c.cycles += 11;
            c.g[V1][0] = sx(WOF_DEFAULTS);
            c.g[V0][0] = u64::MAX;
            let tag = defaults.off(0xc).lbu(c);
            c.g[A1][0] = tag;
            defaults.sw(c, u64::MAX);
            c.g[A0][0] = 1;
            defaults.off(0xc).sb(c, tag);
            Addr(c.g[GP][0] as u32).off(WOF_DEFAULTS_READY_GP).sw(c, 1);
            defaults.off(4).sw(c, 0);
            defaults.off(8).sw(c, 0);
            Addr(sp(c)).sb(c, tag);
        }
        // The parameter block at 0x10(sp), copied from the defaults.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        let block = frame.at(c, 0x10);
        c.g[S1][0] = sx(block);
        c.g[A1][0] = sx(WOF_DEFAULTS);
        c.g[A2][0] = 0x10;
        Addr(block).sw(c, u64::MAX);
        c.g[A0][0] = sx(block);
        c.g[S4][0] = 0x4d_0000;
        call_ptr(c, 0x003e_6574, 0x002f_c154)?; // memcpy
        c.g[A2][0] = c.g[S3][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(WOF_PARAM_KINDS);
            c.g[A3][0] = WOF_KIND_FLOAT;
            loop {
                c.cycles += 8;
                let param = Addr(r32(c, A2));
                let slot4 = (param.lw(c) as u32) << 2;
                let given_kind = param.off(0xc).lw(c);
                c.g[V1][0] = given_kind;
                let dest = Addr(slot4.wrapping_add(r32(c, S1)));
                c.g[A0][0] = sx(dest.0);
                let kind = Addr(slot4.wrapping_add(r32(c, T0))).lw(c);
                c.g[V0][0] = kind;
                if given_kind != kind && kind == c.g[A3][0] {
                    // An int for a float slot.
                    c.cycles += 2 + 4;
                    let value = param.off(4).lw(c) as u32;
                    let converted = FpuF32::from_i32(value as i32);
                    c.f[0] = converted.0;
                    dest.write_fpu(c, converted);
                } else {
                    c.cycles += if given_kind == kind { 1 } else { 2 + 1 };
                    let value = param.off(4).lw(c);
                    c.g[V0][0] = value;
                    dest.sw(c, value);
                }
                c.cycles += 3;
                let left = r32(c, A1).wrapping_sub(1);
                c.g[A1][0] = sx(left);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if left == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_c170);
                }
            }
        }
        // Look the object up.
        c.cycles += 4;
        let id = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        let directory = Addr(c.g[GP][0] as u32).off(WOF_DIRECTORY_GP).lw(c);
        c.g[A1][0] = directory;
        if !id == 0 {
            c.cycles += 3;
            let world = Addr(c.g[GP][0] as u32).off(WOF_WORLD_GP).lw(c);
            c.g[V0][0] = world;
            c.g[S0][0] = Addr(world as u32).off(WOF_WORLD_DEFAULT_OBJECT).lw(c);
        } else {
            c.cycles += 8;
            let id = id as u32;
            let sections = Addr(directory as u32).lw(c);
            let table = Addr(sections as u32).off(8).lw(c);
            c.g[A0][0] = table;
            let at = ((id & 0xff) << 2).wrapping_add(table as u32);
            c.g[V0][0] = sx(at);
            let section = Addr(at).lw(c);
            c.g[V1][0] = section;
            let object = if section != 0 {
                c.cycles += 7;
                let entries = Addr(section as u32).off(0x1c).lw(c);
                c.g[A0][0] = entries;
                let entry = Addr(((id >> 8) << 2).wrapping_add(entries as u32)).lw(c);
                c.g[V1][0] = entry;
                let index = (entry as u32) >> 8;
                let object = if index != 0 {
                    sx(index << 2)
                } else {
                    c.cycles += 1;
                    0
                };
                c.g[V0][0] = object;
                c.cycles += 1;
                object
            } else {
                c.cycles += 2;
                0
            };
            c.g[A2][0] = object;
            c.cycles += 1;
            c.g[S0][0] = object;
        }
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            let node = Addr(r32(c, S0)).off(WOF_OBJECT_NODE).lw(c);
            c.g[A0][0] = node;
            c.g[A1][0] = 0x11;
            let mut create = true;
            if node != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                Callee {
                    addr: 0x002f_ae38,
                    f: ee::s10::f_002fae38,
                }
                .call(c, 0x002f_c240)?;
                create = c.g[V0][0] == 0;
            }
            if create {
                c.cycles += 13;
                c.g[A1][0] = 0x4a_3bf8;
                c.g[A0][0] = 0x2c;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                Callee {
                    addr: 0x0031_7d70,
                    f: ee::s10::f_00317d70,
                }
                .call(c, 0x002f_c260)?;
                let block = Addr(r32(c, S1));
                let flag = block.off(8).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A3][0] = block.off(4).lw(c);
                c.g[A1][0] = 1;
                c.g[A2][0] = c.g[S0][0];
                c.g[T0][0] = (flag != 0) as u64;
                call_ptr(c, 0x0035_6db0, 0x002f_c27c)?; // Object_ctor
            }
        }
        // Return the factory's result value by copy.
        c.cycles += 16;
        let result = Addr(r32(c, S4).wrapping_sub(0x4d_0000 - WOF_RESULT));
        c.g[A1][0] = sx(result.0);
        let head = result.read_u64_unaligned(c);
        let tail = result.off(8).lw(c);
        c.g[V1][0] = tail;
        let out = Addr(r32(c, S2));
        out.write_u64_unaligned(c, head);
        out.off(8).sw(c, tail);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_ParseChunks (0x3dfed0, seg_2DCAE8.c)

pub(crate) const SFPC_ENTRY: u32 = 0x003d_fed0;

/// The loop head the trampoline re-enters at after an unwind (0x3dff08).
pub(crate) const SFPC_HEAD: u32 = 0x003d_ff08;

pub(crate) const SFPC_END: u32 = 0x003e_00d8;

/// StreamFile fields: its mutex (+4), the per-class chunk tallies (+0x24,
/// 16-byte entries `{.., .., bytes +8, first chunk +0xc}` for class 1..),
/// the low-water mark (+0x44), the "needs refill" flag (+0x48), the parsed
/// byte count (+0x4c), the parse cursor (+0x54), the fill cursor (+0x58)
/// and the stream header (+0x60).
pub(crate) const SFPC_MUTEX: u32 = 0x4;

pub(crate) const SFPC_TALLIES: u32 = 0x24;

pub(crate) const SFPC_LOW_WATER: u32 = 0x44;

pub(crate) const SFPC_NEEDS_REFILL: u32 = 0x48;

pub(crate) const SFPC_PARSED: u32 = 0x4c;

pub(crate) const SFPC_CURSOR: u32 = 0x54;

pub(crate) const SFPC_FILL: u32 = 0x58;

pub(crate) const SFPC_HEADER: u32 = 0x60;

/// Header fields: the stream state (+4; 4 = shutting down) and the id of
/// the end-of-stream chunk (+0x11c).
pub(crate) const SFPC_HDR_STATE: u32 = 0x4;

pub(crate) const SFPC_HDR_END_ID: u32 = 0x11c;

pub(crate) const SFPC_STATE_CLOSING: u64 = 4;

/// Chunk header: `{id, bytes | class << 24}`.
pub(crate) const SFPC_CHUNK_BYTES: u32 = 0x4;

/// Tally entry fields.
pub(crate) const SFPC_TALLY_BYTES: u32 = 0x8;

pub(crate) const SFPC_TALLY_FIRST: u32 = 0xc;

pub(crate) const SFPC_CLASSIFY: Callee = Callee {
    addr: 0x003d_fe88,
    f: ee::s14::StreamFile_ClassifyChunk,
};

pub(crate) const SFPC_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};

pub(crate) const SFPC_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

/// Where the parse loop goes after one chunk.
pub(crate) enum SfpcNext {
    /// Back to the "enough bytes for a header?" test (v0 = fill cursor).
    Top,
    /// Return v0.
    Exit,
}

/// `int StreamFile_ParseChunks(StreamFile* self)` (0x3dfed0): walks the
/// chunks between the parse cursor and the fill cursor. Each complete chunk
/// is classified; under the stream's mutex (and unless the stream is shutting
/// down) an unknown chunk is marked dead (id -2), a known one is tagged with
/// its class (`bytes | class << 24`), added to its class tally and to the
/// parsed byte count (clearing the "needs refill" flag when that crosses the
/// low-water mark), and the cursor moves past it. The end-of-stream chunk (or
/// one whose length already carries a class tag) is padded to 64-byte
/// alignment and ends the walk with 1; a shutdown or an incomplete chunk ends
/// it with 0. Frame 0x90 (ra at 0, s7..s0 at 0x10..0x80). Dispatcher-shaped:
/// re-entered at [`SFPC_HEAD`] after an unwind at the loop's event check.
pub fn stream_file_parse_chunks(c: &mut Cpu) {
    guest(c, |c| {
        let mut at_head = match c.pc {
            SFPC_ENTRY => {
                c.cycles += 14;
                let frame = Frame::open(c, 0x90);
                frame.save(c, S0, 0x80);
                frame.save(c, S7, 0x10);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S1, 0x70);
                c.g[S7][0] = sx(0xffff_ffc0);
                frame.save(c, S2, 0x60);
                frame.save(c, S3, 0x50);
                frame.save(c, S4, 0x40);
                frame.save(c, S5, 0x30);
                frame.save_ra(c, 0);
                frame.save(c, S6, 0x20);
                c.g[S6][0] = c.lw(r32(c, S0).wrapping_add(SFPC_HEADER));
                // 0x3e0054
                c.cycles += 1;
                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(SFPC_FILL));
                false
            }
            SFPC_HEAD => true,
            pc => {
                c.jump_out(pc, SFPC_ENTRY, SFPC_END);
                return Ok(());
            }
        };
        loop {
            if !at_head {
                // 0x3e0058: at least a chunk header (8 bytes) buffered?
                c.cycles += 5;
                let chunk = c.lw(r32(c, S0).wrapping_add(SFPC_CURSOR));
                c.g[S2][0] = chunk;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(chunk as u32));
                let short = (c.g[V0][0] as i64) < 8;
                c.g[V0][0] = 0;
                if short {
                    break;
                }
                // 0x3e006c: a length that already carries a class tag turns
                // the chunk into an 8-byte end-of-stream chunk.
                c.cycles += 5;
                c.g[S1][0] = c.lw(r32(c, S2).wrapping_add(SFPC_CHUNK_BYTES));
                c.g[V0][0] = c.g[S1][0] & 0xffff_ffff_ff00_0000;
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(r32(c, S6).wrapping_add(SFPC_HDR_END_ID));
                    c.g[S1][0] = 8;
                    c.sw(r32(c, S2).wrapping_add(SFPC_CHUNK_BYTES), 8);
                    c.sw(r32(c, S2), c.g[V0][0]);
                }
                c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(SFPC_CURSOR));
                // 0x3e0094: the whole chunk buffered?
                c.cycles += 5;
                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(SFPC_FILL));
                c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break;
                }
                c.g[V1][0] = c.lw(r32(c, S2));
                if back_edge_unwinds(c, SFPC_HEAD) {
                    return Err(Unwound);
                }
            }
            at_head = false;
            match sfpc_parse_one(c)? {
                SfpcNext::Top => continue,
                SfpcNext::Exit => break,
            }
        }
        // 0x3e00ac
        c.cycles += 11;
        let frame = Frame { size: 0x90 };
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `len` padded so that `cursor + len` ends 64-byte aligned (`s7` = -64).
pub(crate) fn sfpc_align64(c: &Cpu, len: u32, cursor_low: u64) -> (u64, u64) {
    let low = cursor_low & 0x3f;
    let padded = sx(len.wrapping_add(low as u32));
    let padded = sx((padded as u32).wrapping_add(0x3f)) & c.g[S7][0];
    (low, padded)
}

/// One complete chunk at s2 (length s1, id in v1): 0x3dff08..0x3e0050.
pub(crate) fn sfpc_parse_one(c: &mut Cpu) -> Result<SfpcNext, Unwound> {
    // 0x3dff08: the end chunk is padded to 64-byte alignment first.
    c.cycles += 3;
    c.g[V0][0] = c.lw(r32(c, S6).wrapping_add(SFPC_HDR_END_ID));
    if c.g[V1][0] == c.g[V0][0] {
        c.cycles += 6;
        let (len, cursor) = (r32(c, S1), c.g[A0][0]);
        let (low, padded) = sfpc_align64(c, len, cursor);
        c.g[V1][0] = low;
        c.g[V0][0] = padded;
        c.g[S1][0] = sx((padded as u32).wrapping_sub(low as u32));
    }
    c.g[A0][0] = c.g[S0][0];
    // 0x3dff2c
    c.cycles += 5;
    c.g[A1][0] = c.g[S2][0];
    SFPC_CLASSIFY.call(c, 0x003d_ff34)?;
    c.g[S5][0] = c.g[V0][0];
    c.g[V0][0] = sx(r32(c, S5) << 24);
    if (c.g[S5][0] as i64) < 0 {
        // 0x3dff40: unknown chunk: mark it dead and skip it.
        c.cycles += 8;
        c.g[S3][0] = sx(r32(c, S0).wrapping_add(SFPC_MUTEX));
        c.g[A0][0] = c.g[S3][0];
        SFPC_LOCK.call(c, 0x003d_ff4c)?;
        c.g[V0][0] = c.lw(r32(c, S6).wrapping_add(SFPC_HDR_STATE)) ^ SFPC_STATE_CLOSING;
        c.g[S4][0] = (c.g[V0][0] < 1) as u64;
        c.g[V1][0] = sx(0xffff_fffe);
        if c.g[S4][0] == 0 {
            c.cycles += 4;
            c.sw(r32(c, S2), c.g[V1][0]);
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(SFPC_CURSOR));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
            c.sw(r32(c, S0).wrapping_add(SFPC_CURSOR), c.g[V0][0]);
        }
        // 0x3dff70
        c.cycles += 4;
        c.g[A0][0] = c.g[S3][0];
        SFPC_UNLOCK.call(c, 0x003d_ff78)?;
    } else {
        // 0x3dff80: tag the chunk with its class and account for it.
        c.cycles += 9;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(SFPC_MUTEX));
        c.g[V0][0] |= c.g[S1][0];
        c.sw(r32(c, S2).wrapping_add(SFPC_CHUNK_BYTES), c.g[V0][0]);
        SFPC_LOCK.call(c, 0x003d_ff90)?;
        c.g[V0][0] = c.lw(r32(c, S6).wrapping_add(SFPC_HDR_STATE)) ^ SFPC_STATE_CLOSING;
        c.g[S4][0] = (c.g[V0][0] < 1) as u64;
        c.g[A2][0] = sx(r32(c, S0).wrapping_add(SFPC_MUTEX));
        if c.g[S4][0] == 0 {
            // 0x3dffa4: tallies[class - 1].bytes += len (first chunk noted).
            c.cycles += 8;
            let tallies = c.lw(r32(c, S0).wrapping_add(SFPC_TALLIES));
            c.g[V1][0] = sx(r32(c, S5) << 4);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x10));
            c.g[V0][0] = sx((tallies as u32).wrapping_add(r32(c, V1)));
            let tally = r32(c, V0);
            c.g[A0][0] = c.lw(tally.wrapping_add(SFPC_TALLY_BYTES));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
            c.sw(tally.wrapping_add(SFPC_TALLY_BYTES), c.g[A0][0]);
            if c.g[A0][0] == c.g[S1][0] {
                c.cycles += 1;
                c.sw(tally.wrapping_add(SFPC_TALLY_FIRST), c.g[S2][0]);
            }
            // 0x3dffc8: parsed count and cursor; low-water crossing.
            c.cycles += 9;
            let this = r32(c, S0);
            c.g[V1][0] = c.lw(this.wrapping_add(SFPC_PARSED));
            c.g[V0][0] = c.lw(this.wrapping_add(SFPC_CURSOR));
            c.g[A0][0] = c.lw(this.wrapping_add(SFPC_LOW_WATER));
            c.g[A1][0] = sx(r32(c, V1).wrapping_add(r32(c, S1)));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
            c.sw(this.wrapping_add(SFPC_PARSED), c.g[A1][0]);
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.sw(this.wrapping_add(SFPC_CURSOR), c.g[V0][0]);
            if c.g[V1][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.sw(this.wrapping_add(SFPC_NEEDS_REFILL), 0);
                }
            }
        }
        // 0x3dfff8
        c.cycles += 2;
        c.g[A0][0] = c.g[A2][0];
        SFPC_UNLOCK.call(c, 0x003e_0000)?;
    }
    // 0x3e0000
    c.cycles += 2;
    if c.g[S4][0] == 0 {
        c.g[V1][0] = c.lw(r32(c, S2));
        // 0x3e0040: stop after the end chunk.
        c.cycles += 3;
        c.g[V0][0] = c.lw(r32(c, S6).wrapping_add(SFPC_HDR_END_ID));
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(SFPC_FILL));
            return Ok(SfpcNext::Top);
        }
        c.cycles += 2;
        c.g[V0][0] = 1;
        return Ok(SfpcNext::Exit);
    }
    // 0x3e0008: shutting down: pad a non-end chunk and stop with 0.
    c.cycles += 4;
    c.g[V1][0] = c.lw(r32(c, S6).wrapping_add(SFPC_HDR_END_ID));
    c.g[V0][0] = c.lw(r32(c, S2));
    let is_end = c.g[V0][0] == c.g[V1][0];
    c.g[V0][0] = 0;
    if is_end {
        return Ok(SfpcNext::Exit);
    }
    c.cycles += 10;
    let cursor = c.lw(r32(c, S0).wrapping_add(SFPC_CURSOR));
    c.g[A0][0] = sx(r32(c, S5) << 24);
    let len = r32(c, S1);
    let (low, padded) = sfpc_align64(c, len, cursor);
    c.g[V0][0] = low;
    c.g[V1][0] = padded;
    c.g[S1][0] = sx((padded as u32).wrapping_sub(low as u32));
    c.g[A0][0] |= c.g[S1][0];
    c.sw(r32(c, S2).wrapping_add(SFPC_CHUNK_BYTES), c.g[A0][0]);
    // 0x3e00a8
    c.cycles += 1;
    c.g[V0][0] = 0;
    Ok(SfpcNext::Exit)
}

// ---------------------------------------------------------------------------
// WScriptFn_AddUVScrollModifier (0x2fe2c0, seg_1FBE38.cpp)

/// The default UV-scroll parameter block (52 bytes, a guarded static local:
/// `{object id = -1, 5, 0, 0, 1/60, 1/60, 1, 0, 0, 0, 0, 1, tag}`) and its guard.
pub(crate) const WSUV_DEFAULTS: u32 = 0x004f_b7e0;

pub(crate) const WSUV_DEFAULTS_HI: u64 = 0x0050_0000;

pub(crate) const WSUV_GUARD: u32 = 0x252c;

/// `1/60` in `.lit4` (gp-0x3384).
pub(crate) const WSUV_SIXTIETH: u32 = 0x3384;

/// Script argument entries: `{param index, value, .., type}` (16 bytes);
/// the parameters' declared types (`D_00446238`, 2 = float).
pub(crate) const WSUV_PARAM_TYPES: u64 = 0x0044_6238;

pub(crate) const WSUV_TYPE_FLOAT: u64 = 2;

/// The world (`gp+0xce8`, its own object at +0x290) and the object table
/// (`gp+0x16c8`) the object id resolves through.
pub(crate) const WSUV_WORLD: u32 = 0xce8;

pub(crate) const WSUV_WORLD_SELF: u32 = 0x290;

pub(crate) const WSUV_OBJ_TABLE: u32 = 0x16c8;

/// The script's void result (`D_004C9098`, 12 bytes).
pub(crate) const WSUV_RESULT_HI: u64 = 0x004d_0000;

pub(crate) const WSUV_RESULT_LO: u32 = 0x6f68;

pub(crate) const WSUV_MEMCPY: u32 = 0x003e_6574;

pub(crate) const WSUV_ADD_MODIFIER: u32 = 0x0035_5b90;

/// `ScriptValue WScriptFn_AddUVScrollModifier(int argc, Arg* args)`
/// (0x2fe2c0, result returned through a0): fills a 13-word UV-scroll
/// parameter block from the defaults and the script arguments (an integer
/// passed for a float parameter is converted), resolves the object id in
/// word 0 (-1: the world itself) and, if the object has a model instance
/// whose virtual `+0x80` check passes, adds the modifier to it
/// (`ModObj_AddUVScrollModifier`). Returns the void script value. Frame 0xa0
/// (block at 0x10, ra at 0x50, s3..s0 at 0x60..0x90).
pub fn wscript_fn_add_uv_scroll_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(WSUV_GUARD));
        frame.save(c, S0, 0x90);
        c.g[A3][0] = WSUV_DEFAULTS_HI;
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x80);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x50);
        if c.g[V0][0] == 0 {
            // First use: build the defaults.
            c.cycles += 25;
            let d = WSUV_DEFAULTS;
            c.g[V0][0] = sx(d);
            c.g[V1][0] = u64::MAX;
            c.f[0] = 0;
            c.g[A2][0] = c.lbu(d.wrapping_add(0x30));
            c.g[A0][0] = 5;
            c.lwc1(2, r32(c, GP).wrapping_sub(WSUV_SIXTIETH));
            c.g[A1][0] = 1;
            c.g[AT][0] = 0x3f80_0000;
            c.f[1] = 0x3f80_0000;
            c.sw(d, c.g[V1][0]);
            c.sb(d.wrapping_add(0x30), c.g[A2][0]);
            c.sw(d.wrapping_add(0x4), 5);
            c.swc1(2, d.wrapping_add(0x14));
            c.swc1(0, d.wrapping_add(0x28));
            c.swc1(1, d.wrapping_add(0x2c));
            c.sw(r32(c, GP).wrapping_add(WSUV_GUARD), 1);
            c.swc1(0, d.wrapping_add(0x8));
            c.swc1(0, d.wrapping_add(0xc));
            c.swc1(2, d.wrapping_add(0x10));
            c.swc1(1, d.wrapping_add(0x18));
            c.swc1(0, d.wrapping_add(0x1c));
            c.swc1(0, d.wrapping_add(0x20));
            c.swc1(0, d.wrapping_add(0x24));
            c.sb(sp(c), c.g[A2][0]);
        }
        // 0x2fe354: copy the defaults into the frame's block.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let block = sp(c).wrapping_add(0x10);
        c.g[S1][0] = sx(block);
        c.g[A2][0] = 0x34;
        c.g[A1][0] = sx(r32(c, A3).wrapping_sub(0x4820));
        c.sw(block, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, WSUV_MEMCPY, 0x002f_e374)?;
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S0][0] as i64) > 0 {
            // Apply the script arguments.
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = WSUV_PARAM_TYPES;
            c.g[A3][0] = WSUV_TYPE_FLOAT;
            loop {
                // 0x2fe390
                c.cycles += 8;
                let arg = r32(c, A2);
                c.g[V0][0] = c.lw(arg);
                c.g[V1][0] = c.lw(arg.wrapping_add(0xc));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                c.g[V0][0] = c.lw(r32(c, V0));
                let convert = if c.g[V1][0] == c.g[V0][0] {
                    false
                } else {
                    c.cycles += 2;
                    c.g[V0][0] == c.g[A3][0]
                };
                if convert {
                    c.cycles += 4;
                    c.lwc1(0, arg.wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, r32(c, A0));
                } else {
                    c.g[V0][0] = c.lw(arg.wrapping_add(0x4));
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_e390);
                }
            }
        }
        // 0x2fe3d8: resolve the object id.
        c.cycles += 4;
        c.g[A2][0] = c.lw(r32(c, S1));
        c.g[V0][0] = !c.g[A2][0];
        c.g[A1][0] = c.lw(r32(c, GP).wrapping_add(WSUV_OBJ_TABLE));
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(WSUV_WORLD));
            c.g[V0][0] = c.lw(r32(c, V0).wrapping_add(WSUV_WORLD_SELF));
        } else {
            c.cycles += 8;
            let id = r32(c, A2);
            c.g[V0][0] = sx((id & 0xff) << 2);
            c.g[V1][0] = c.lw(r32(c, A1));
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = c.lw(r32(c, V0));
            c.g[V0][0] = sx(id >> 8);
            let mut found = false;
            if c.g[V1][0] != 0 {
                c.cycles += 7;
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                c.g[V1][0] = c.lw(r32(c, V0));
                c.g[V0][0] = sx(r32(c, V1) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, V0) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // 0x2fe434: the object's model instance.
        c.cycles += 3;
        c.g[S0][0] = c.lw(r32(c, V0).wrapping_add(0xc));
        c.g[V1][0] = WSUV_RESULT_HI;
        if c.g[S0][0] != 0 {
            c.cycles += 7;
            let inst = r32(c, S0);
            c.g[V1][0] = c.lw(inst.wrapping_add(0xc));
            let vt = r32(c, V1);
            c.g[A0][0] = c.lh(vt.wrapping_add(0x80));
            c.g[V0][0] = c.lw(vt.wrapping_add(0x84));
            let method = r32(c, V0);
            c.g[A0][0] = sx(inst.wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x002f_e454)?;
            let ok = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if ok {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, WSUV_ADD_MODIFIER, 0x002f_e464)?;
            }
            c.cycles += 1;
            c.g[V1][0] = WSUV_RESULT_HI;
        }
        // 0x2fe468: return the void script value through a0.
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(r32(c, V1).wrapping_sub(WSUV_RESULT_LO));
        let src = r32(c, A3);
        let dst = r32(c, S3);
        c.ldl(4, src.wrapping_add(7));
        c.ldr(4, src);
        c.g[A1][0] = c.lw(src.wrapping_add(8));
        c.sdl(4, dst.wrapping_add(7));
        c.sdr(4, dst);
        c.sw(dst.wrapping_add(8), c.g[A1][0]);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_StartNextRequest (0x3e0270, seg_2DCAE8.c)

/// StreamFile fields: its mutex (+4), the read state (+0x38), the parse /
/// fill cursors (+0x54 / +0x58), the current request (+0x60), the open
/// file's name (+0x6c), its pending close/open file request (+0x16c / +0x174)
/// and the bytes left to read (+0x170).
pub(crate) const SNR_MUTEX: u32 = 0x4;

pub(crate) const SNR_READ_STATE: u32 = 0x38;

pub(crate) const SNR_CURSOR: u32 = 0x54;

pub(crate) const SNR_FILL: u32 = 0x58;

pub(crate) const SNR_REQUEST: u32 = 0x60;

pub(crate) const SNR_NAME: u32 = 0x6c;

pub(crate) const SNR_FILE: u32 = 0x16c;

pub(crate) const SNR_LEFT: u32 = 0x170;

pub(crate) const SNR_FILE_REQUEST: u32 = 0x174;

/// Request fields: state (+4; 1 = ready, 2 = started), next (+0xc), kind
/// (+0x10; 1 = in-memory), file name (+0x14), size (+0x118), the cursor at
/// its start (+0x120).
pub(crate) const SNR_REQ_STATE: u32 = 0x4;

pub(crate) const SNR_REQ_NEXT: u32 = 0xc;

pub(crate) const SNR_REQ_KIND: u32 = 0x10;

pub(crate) const SNR_REQ_NAME: u32 = 0x14;

pub(crate) const SNR_REQ_SIZE: u32 = 0x118;

pub(crate) const SNR_REQ_START: u32 = 0x120;

pub(crate) const SNR_STRCMP: u32 = 0x0041_65a8;

pub(crate) const SNR_STRCPY: u32 = 0x0041_66f4;

/// `StreamFile_OnOpen` / `StreamFile_OnClose` (request callbacks).
pub(crate) const SNR_ON_OPEN: u64 = 0x003e_00d8;

pub(crate) const SNR_ON_CLOSE: u64 = 0x003e_0118;

pub(crate) const SNR_QUEUE_OPEN: Callee = Callee {
    addr: 0x003d_d4e8,
    f: ee::s14::EAFile_QueueOpen,
};

pub(crate) const SNR_QUEUE_CLOSE: Callee = Callee {
    addr: 0x003d_d5a0,
    f: ee::s14::EAFile_QueueClose,
};

pub(crate) const SNR_SET_CALLBACK: Callee = Callee {
    addr: 0x003d_cf70,
    f: ee::s14::EAFile_SetRequestCallback,
};

pub(crate) const SNR_ISSUE_READ: Callee = Callee {
    addr: 0x003e_03d0,
    f: ee::s14::StreamFile_IssueRead,
};

/// `void StreamFile_StartNextRequest(StreamFile* self, int priority)`
/// (0x3e0270): under the stream's mutex, picks the request to stream (the
/// current one if ready, else the next queued one); with none, the stream
/// goes idle (read state 0). Otherwise the request is marked started at the
/// parse cursor, the fill cursor is reset to it and, for an in-memory
/// request or one on the already open file, reading starts at once
/// (`StreamFile_IssueRead`); a different file is remembered by name and
/// opened (or the old one closed first) through the file queue, with
/// `StreamFile_OnOpen` / `OnClose` as callbacks. Frame 0x50 (ra at 0, s3..s0
/// at 0x10..0x40).
pub fn stream_file_start_next_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(SNR_MUTEX));
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        SFPC_LOCK.call(c, 0x003e_029c)?;
        let this = r32(c, S0);
        c.g[A0][0] = c.lw(this.wrapping_add(SNR_REQUEST));
        c.g[V0][0] = 1;
        // s1: no request to start.
        let mut idle = true;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            let req = r32(c, A0);
            c.g[V1][0] = c.lw(req.wrapping_add(SNR_REQ_STATE));
            c.g[S1][0] = 0;
            if c.g[V1][0] == c.g[V0][0] {
                idle = false;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(req.wrapping_add(SNR_REQ_NEXT));
                if c.g[V0][0] != 0 {
                    c.sw(this.wrapping_add(SNR_REQUEST), c.g[V0][0]);
                    idle = false;
                }
            }
        }
        if idle {
            c.cycles += 1;
            c.g[S1][0] = 1;
        }
        // 0x3e02c4
        c.cycles += 2;
        if c.g[S1][0] == 0 {
            c.g[S2][0] = c.lw(this.wrapping_add(SNR_REQUEST));
            c.cycles += 4;
            c.g[V0][0] = 2;
            c.g[V1][0] = c.lw(this.wrapping_add(SNR_CURSOR));
            let req = r32(c, S2);
            c.sw(req.wrapping_add(SNR_REQ_STATE), 2);
            c.sw(req.wrapping_add(SNR_REQ_START), c.g[V1][0]);
        } else {
            c.cycles += 3;
            c.sw(this.wrapping_add(SNR_READ_STATE), 0);
        }
        // 0x3e02e8
        c.cycles += 4;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(SNR_MUTEX));
        SFPC_UNLOCK.call(c, 0x003e_02f0)?;
        if c.g[S1][0] == 0 {
            c.cycles += 6;
            let this = r32(c, S0);
            c.g[V0][0] = c.lw(this.wrapping_add(SNR_CURSOR));
            c.g[A0][0] = 1;
            c.sw(this.wrapping_add(SNR_FILL), c.g[V0][0]);
            let req = r32(c, S2);
            c.g[V1][0] = c.lw(req.wrapping_add(SNR_REQ_KIND));
            let mut read_now = true;
            if c.g[V1][0] != c.g[A0][0] {
                // A file request: open its file unless it is already open.
                c.g[V0][0] = c.lw(req.wrapping_add(SNR_REQ_SIZE));
                c.cycles += 8;
                c.g[S1][0] = sx(this.wrapping_add(SNR_NAME));
                c.g[S2][0] = sx(req.wrapping_add(SNR_REQ_NAME));
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S2][0];
                c.sw(this.wrapping_add(SNR_LEFT), c.g[V0][0]);
                call_ptr(c, SNR_STRCMP, 0x003e_0330)?;
                let same = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if !same {
                    read_now = false;
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, SNR_STRCPY, 0x003e_0340)?;
                    let this = r32(c, S0);
                    c.g[A0][0] = c.lw(this.wrapping_add(SNR_FILE));
                    c.g[A1][0] = c.g[S3][0];
                    let (queued, callback, ret) = if c.g[A0][0] == 0 {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[A1][0] = 1;
                        c.g[A3][0] = c.g[S0][0];
                        SNR_QUEUE_OPEN.call(c, 0x003e_0360)?;
                        (c.g[V0][0], SNR_ON_OPEN, 0x003e_0378)
                    } else {
                        c.cycles += 4;
                        c.g[A2][0] = c.g[S0][0];
                        SNR_QUEUE_CLOSE.call(c, 0x003e_0388)?;
                        (c.g[V0][0], SNR_ON_CLOSE, 0x003e_03a0)
                    };
                    c.sw(r32(c, S0).wrapping_add(SNR_FILE_REQUEST), queued);
                    if queued != 0 {
                        c.cycles += 6;
                        c.g[A1][0] = 0x003e_0000;
                        c.g[A0][0] = queued;
                        c.g[A1][0] = callback;
                        SNR_SET_CALLBACK.call(c, ret)?;
                    } else {
                        c.cycles += 1;
                    }
                }
            } else {
                c.cycles += 2;
                c.sw(this.wrapping_add(SNR_LEFT), 0);
            }
            if read_now {
                // 0x3e03a8
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
                SNR_ISSUE_READ.call(c, 0x003e_03b4)?;
                c.cycles += 1;
            }
        }
        // 0x3e03b8
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// libpad_SendActDma (0x3ff390)

/// libpad's per-port/slot state table (0x526250): 0x70 bytes per port,
/// 0x1c per slot.
pub(crate) const PAD_SLOTS: u32 = 0x0052_6250;

pub(crate) const PAD_PORT_STRIDE: u32 = 0x70;

pub(crate) const PAD_SLOT_STRIDE: u32 = 0x1c;

/// Slot fields: actuator packet buffer (+4), IOP destination (+8), last
/// DMA id (+0xc).
pub(crate) const PAD_ACT_BUF: u32 = 0x4;

pub(crate) const PAD_IOP_DEST: u32 = 0x8;

pub(crate) const PAD_DMA_ID: u32 = 0xc;

/// libpad's debug-print flag (0x4515cc) and the "busy" message (0x497888).
pub(crate) const PAD_DEBUG_BASE: u32 = 0x0045_0000;

pub(crate) const PAD_DEBUG_FLAG: u32 = 0x15cc;

pub(crate) const PAD_BUSY_MSG: u32 = 0x0049_7888;

/// `int libpad_SendActDma(int port, int slot)` (0x3ff390): send the slot's
/// actuator (vibration) packet to the IOP. If the previous DMA is still
/// running (`sceSifDmaStat >= 0`) it gives up (printing a message when
/// libpad debugging is on) and returns 0; otherwise it bumps the packet
/// counter, writes back the buffer, DMAs 0x20 bytes into the IOP's
/// double-buffered destination and records the DMA id (returns 1; 0 and
/// the message when `sceSifSetDma` fails).
pub fn libpad_send_act_dma(c: &mut Cpu) {
    use crate::guest::Addr;
    const DMA_STAT: Callee = Callee {
        addr: 0x0042_4140,
        f: ee::s15::sceSifDmaStat,
    };
    const SYNC_DCACHE: Callee = Callee {
        addr: 0x0042_4698,
        f: ee::s15::SyncDCache,
    };
    const SET_DMA: Callee = Callee {
        addr: 0x0042_4160,
        f: ee::s15::sceSifSetDma,
    };
    const PRINTF: Callee = Callee {
        addr: 0x0042_5c60,
        f: ee::s15::scePrintf,
    };
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x150);
        c.g[V1][0] = PAD_SLOT_STRIDE as u64;
        c.sd(frame.at(c, 0x120), c.g[S2][0]);
        c.sd(frame.at(c, 0x100), c.g[S0][0]);
        c.g[S2][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[A0][0] = PAD_PORT_STRIDE as u64;
        c.mult(1, A0, S2, A0);
        c.mult(0, V1, S0, V1);
        c.sd(frame.at(c, 0x130), c.g[S3][0]);
        c.g[S3][0] = 0x0052_0000;
        c.sd(frame.at(c, 0x140), c.g[RA][0]);
        c.sd(frame.at(c, 0x110), c.g[S1][0]);
        c.g[V0][0] = sx(PAD_SLOTS);
        c.g[V1][0] = crate::abi::addu(c.g[V1][0], c.g[A0][0]);
        let slot = PAD_SLOTS.wrapping_add(r32(c, V1));
        c.g[A1][0] = sx(slot);
        c.g[V0][0] = sx(slot);
        c.g[A0][0] = Addr(slot).off(PAD_DMA_ID).lw(c);
        c.g[S1][0] = Addr(slot).off(PAD_ACT_BUF).lw(c);

        let mut busy = false;
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            DMA_STAT.call(c, 0x003f_f3e8)?;
            busy = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = sx(PAD_DEBUG_BASE);
        }
        if !busy {
            c.cycles += 28;
            c.g[A3][0] = PAD_SLOT_STRIDE as u64;
            c.g[V1][0] = PAD_PORT_STRIDE as u64;
            c.mult(0, A3, S0, A3);
            c.mult(1, V1, S2, V1);
            let buf = r32(c, S1);
            let counter = sx((Addr(buf).lw(c) as u32).wrapping_add(1));
            c.g[A2][0] = counter;
            c.g[S3][0] = sx(PAD_SLOTS);
            c.g[A0][0] = sx(buf);
            c.g[A1][0] = sx(buf.wrapping_add(0x1f));
            c.g[S2][0] = crate::abi::addu(c.g[A3][0], c.g[V1][0]);
            let slot = PAD_SLOTS.wrapping_add(r32(c, S2));
            c.g[V1][0] = sx(slot);
            let half = (counter as u32 & 1) << 5;
            c.g[V0][0] = half as u64;
            let dest = (Addr(slot).off(PAD_IOP_DEST).lw(c) as u32).wrapping_add(half);
            Addr(buf).sw(c, counter);
            c.g[S0][0] = sx(dest);
            SYNC_DCACHE.call(c, 0x003f_f434)?;

            // One sceSifDmaData {src, dest, size 0x20, attr 0} on the stack.
            c.g[V0][0] = 0x20;
            let sp0 = frame.at(c, 0);
            let (dest, src) = (c.g[S0][0], c.g[S1][0]);
            Addr(sp0).off(4).sw(c, dest);
            Addr(sp0).sw(c, src);
            c.g[A0][0] = c.g[SP][0];
            Addr(sp0).off(8).sw(c, 0x20);
            c.g[A1][0] = 1;
            Addr(sp0).off(0xc).sw(c, 0);
            SET_DMA.call(c, 0x003f_f454)?;
            c.g[A0][0] = c.g[V0][0];
            let slot = r32(c, S3).wrapping_add(r32(c, S2));
            c.g[V1][0] = sx(slot);
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = 1;
                let id = c.g[A0][0];
                Addr(slot).off(PAD_DMA_ID).sw(c, id);
                return pad_send_act_dma_epilogue(c, frame);
            }
            c.cycles += 1;
            c.g[V0][0] = sx(PAD_DEBUG_BASE);
        }

        // Busy (or failed): print when debugging, return 0.
        c.cycles += 3;
        c.g[V1][0] = Addr(PAD_DEBUG_BASE).off(PAD_DEBUG_FLAG).lw(c);
        c.g[A0][0] = 0x0049_0000;
        if c.g[V1][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = sx(PAD_BUSY_MSG);
            PRINTF.call(c, 0x003f_f478)?;
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        pad_send_act_dma_epilogue(c, frame)
    })
}

/// libpad_SendActDma's epilogue: reload ra, s3..s0 (`ld`), pop the frame.
pub(crate) fn pad_send_act_dma_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 7;
    frame.restore_ra(c, 0x140);
    c.g[S3][0] = c.ld(frame.at(c, 0x130));
    c.g[S2][0] = c.ld(frame.at(c, 0x120));
    c.g[S1][0] = c.ld(frame.at(c, 0x110));
    c.g[S0][0] = c.ld(frame.at(c, 0x100));
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// scePadSetActDirect (0x400190)

/// Slot field: the pad's DMA receive buffer (two 0x80-byte halves; the
/// current one has the connection state byte at +0x72, 1 = stable).
pub(crate) const PAD_STATE_STABLE: u64 = 1;

pub(crate) const PAD_DMA_STATE: u32 = 0x72;

/// Actuator packet: command (+4) = 1, length (+8) = 6, the six motor bytes
/// at +0xc.
pub(crate) const PAD_ACT_CMD: u32 = 0x4;

pub(crate) const PAD_ACT_LEN: u32 = 0x8;

pub(crate) const PAD_ACT_DATA: u32 = 0xc;

pub(crate) const PAD_ACT_BYTES: u32 = 6;

/// `int scePadSetActDirect(int port, int slot, const u8* data)` (0x400190):
/// set the vibration motors of a connected pad. Returns 0 unless the pad's
/// state is "stable"; then copies the six actuator bytes into the slot's
/// actuator packet (command 1, length 6) and sends it
/// (`libpad_SendActDma`), returning 1 when that succeeded.
pub fn sce_pad_set_act_direct(c: &mut Cpu) {
    use crate::guest::Addr;
    const GET_DMA_BUFFER: Callee = Callee {
        addr: 0x003f_f9a8,
        f: ee::s15::libpad_GetDmaBuffer,
    };
    const SEND_ACT_DMA: Callee = Callee {
        addr: 0x003f_f390,
        f: ee::s15::libpad_SendActDma,
    };
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S2][0] = c.g[A1][0];
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S0][0] = c.g[A2][0];
        GET_DMA_BUFFER.call(c, 0x0040_01b4)?;
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(PAD_DMA_STATE)).lbu(c);
        let stable = c.g[V1][0] == PAD_STATE_STABLE;
        c.g[V0][0] = 0;
        if stable {
            c.cycles += 11;
            c.g[V1][0] = PAD_SLOT_STRIDE as u64;
            c.g[A0][0] = PAD_PORT_STRIDE as u64;
            c.mult(0, V1, S2, V1);
            c.mult(1, A0, S1, A0);
            c.g[A1][0] = 0;
            c.g[V1][0] = crate::abi::addu(c.g[V1][0], c.g[A0][0]);
            let slot = PAD_SLOTS.wrapping_add(r32(c, V1));
            c.g[V0][0] = sx(slot);
            let packet = Addr(slot).off(PAD_ACT_BUF).lw(c);
            c.g[A2][0] = packet;
            c.g[A3][0] = sx((packet as u32).wrapping_add(PAD_ACT_DATA));
            // Copy the six motor bytes.
            loop {
                c.cycles += 8;
                let i = r32(c, A1);
                let src = r32(c, S0).wrapping_add(i);
                let dst = r32(c, A3).wrapping_add(i);
                c.g[V0][0] = sx(src);
                c.g[A0][0] = sx(dst);
                let byte = Addr(src).lbu(c);
                c.g[V1][0] = byte;
                c.g[A1][0] = sx(i.wrapping_add(1));
                let more = ((c.g[A1][0] as i64) < PAD_ACT_BYTES as i64) as u64;
                c.g[V0][0] = more;
                Addr(dst).sb(c, byte);
                if more == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0040_01f0);
                }
            }
            c.cycles += 10;
            c.g[S0][0] = 1;
            c.g[V0][0] = PAD_ACT_BYTES as u64;
            let packet = Addr(r32(c, A2));
            packet.off(PAD_ACT_CMD).sw(c, 1);
            c.g[A0][0] = c.g[S1][0];
            packet.off(PAD_ACT_LEN).sw(c, PAD_ACT_BYTES as u64);
            c.g[A1][0] = c.g[S2][0];
            SEND_ACT_DMA.call(c, 0x0040_022c)?;
            c.g[V0][0] ^= 1;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = 0;
            }
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x30);
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_ResetGrindOrientation (0x115640, seg/seg_14B10.cpp)

/// Rider: the grind orientation (0 none, 3 / 4 sideways left / right).
pub(crate) const RGO_GRIND_SIDE: u32 = 0x328;

/// Rider: the rotation axis quadword handed to `Rider_RotateAroundAxis`.
pub(crate) const RGO_AXIS: u32 = 0x1c0;

/// Rider: its animation controller.
pub(crate) const RGO_ANIM: u32 = 0x784;

/// AnimController: the root rotation quadword and the heading vector.
pub(crate) const RGO_ANIM_ROOT_ROT: u32 = 0x30;

pub(crate) const RGO_ANIM_HEADING: u32 = 0x40;

/// The identity rotation quadword (0x4ff130) and the up axis (0x4ff160).
pub(crate) const RGO_IDENTITY: u32 = 0x004f_f130;

pub(crate) const RGO_UP: u32 = 0x004f_f160;

/// `gp - 0x7b70` / `gp - 0x7b6c`: +pi/2 and -pi/2.
pub(crate) const RGO_QUARTER_TURN_GP: u32 = 0x7b70;

pub(crate) const RGO_NEG_QUARTER_TURN_GP: u32 = 0x7b6c;

pub(crate) const RGO_GET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe98,
    f: ee::s00::Rider_GetMainState,
};

pub(crate) const RGO_ROTATE_AROUND_AXIS: Callee = Callee {
    addr: 0x0011_dfe0,
    f: ee::s00::Rider_RotateAroundAxis,
};

pub(crate) const RGO_EMPTY_STANCE_HOOK: Callee = Callee {
    addr: 0x0011_6930,
    f: ee::s00::Rider_EmptyStanceHook,
};

/// `Math_SinCos(float, float* sin, float* cos)`, `AnimController_RotateRoots`
/// and `cRiderAnimBase_play(anim, id, flags, -1.0)` (other shards).
pub(crate) const RGO_MATH_SIN_COS: u32 = 0x0031_be50;

pub(crate) const RGO_ROTATE_ROOTS: u32 = 0x0031_1b48;

pub(crate) const RGO_PLAY: u32 = 0x0031_28e8;

/// The heading vector `up * sin` (from the sincos results at `sin_at`,
/// `sin_at + 4`; the up axis in `up_reg`, its x at `s0 - 0xea0`) built in the frame's first quadword and stored into the
/// animation controller in `anim_reg`: shared by the two sideways arms.
pub(crate) fn rgo_store_heading(c: &mut Cpu, sin_at: u32, up_reg: usize, anim_reg: usize) {
    let sp_ = sp(c);
    let up = r32(c, up_reg);
    let s = Addr(sp_.wrapping_add(sin_at)).read_fpu(c);
    let y = Addr(up.wrapping_add(8)).read_fpu(c);
    let x = Addr(r32(c, S0).wrapping_sub(0xea0)).read_fpu(c);
    let z = Addr(up.wrapping_add(4)).read_fpu(c);
    let f1 = s * y;
    let f2 = s * x;
    let f3 = Addr(sp_.wrapping_add(sin_at + 4)).read_fpu(c);
    let f0 = s * z;
    abi::set_fpr(c, 1, f1);
    abi::set_fpr(c, 2, f2);
    abi::set_fpr(c, 3, f3);
    abi::set_fpr(c, 4, z);
    abi::set_fpr(c, 0, f0);
    Addr(sp_ + 0xc).write_fpu(c, f3);
    Addr(sp_ + 0x8).write_fpu(c, f1);
    Addr(sp_).write_fpu(c, f2);
    Addr(sp_ + 0x4).write_fpu(c, f0);
    c.lq(2, sp_);
    let anim = r32(c, anim_reg);
    c.sq(2, anim.wrapping_add(RGO_ANIM_HEADING));
}

/// Reset the animation controller's root rotation to identity and compute
/// sin/cos of -0.0 into `sin_at` (`Math_SinCos`), the controller in `anim_reg`.
pub(crate) fn rgo_reset_root(
    c: &mut Cpu,
    anim_reg: usize,
    sin_at: u32,
    ret_: u32,
) -> crate::call::Flow {
    c.f[12] = 0;
    c.g[V0][0] = sx(RGO_IDENTITY);
    c.g[anim_reg][0] = Addr(r32(c, S3).wrapping_add(RGO_ANIM)).lw(c);
    c.lq(3, RGO_IDENTITY);
    c.f[12] ^= 0x8000_0000;
    c.g[A0][0] = sx(sp(c).wrapping_add(sin_at));
    c.g[A1][0] = sx(sp(c).wrapping_add(sin_at + 4));
    let anim = r32(c, anim_reg);
    c.sq(3, anim.wrapping_add(RGO_ANIM_ROOT_ROT));
    call_ptr(c, RGO_MATH_SIN_COS, ret_)
}

/// A sideways grind arm: turn the rider by the quarter turn at `gp - lit`
/// around its axis, rotate the animation roots by the same angle, then
/// reset the root rotation and the heading vector.
pub(crate) fn rgo_turn_sideways(
    c: &mut Cpu,
    lit: u32,
    sin_at: u32,
    rets: [u32; 3],
) -> crate::call::Flow {
    c.lq(2, r32(c, S3).wrapping_add(RGO_AXIS));
    c.g[A1][0] = c.g[SP][0];
    let gp = r32(c, GP);
    let angle = Addr(gp.wrapping_sub(lit)).read_fpu(c);
    abi::set_fpr(c, 20, angle);
    c.g[A0][0] = c.g[S3][0];
    c.sq(2, sp(c));
    c.g[S0][0] = 0x50_0000;
    abi::set_fpr(c, 12, angle);
    c.g[S1][0] = sx(RGO_UP);
    RGO_ROTATE_AROUND_AXIS.call(c, rets[0])?;
    c.g[A0][0] = Addr(r32(c, S3).wrapping_add(RGO_ANIM)).lw(c);
    c.f[12] = c.f[20];
    call_ptr(c, RGO_ROTATE_ROOTS, rets[1])?;
    rgo_reset_root(c, S2, sin_at, rets[2])
}

/// `void Rider_ResetGrindOrientation(Rider* rider)` (0x115640): when the
/// rider leaves a grind (orientation at +0x328 non-zero), restore its
/// animation. In main state 1, a sideways grind (3 / 4) resets the root
/// rotation and heading, and the matching end-of-grind animation plays
/// (0x11a / 0x115, else 0x10c). In main state 0, a sideways grind turns the
/// rider back by a quarter turn (+pi/2 for 3, -pi/2 for 4) first, and
/// animation 5 plays. Other main states only clear the orientation. Then
/// `Rider_EmptyStanceHook`.
pub fn rider_reset_grind_orientation(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S3, 0x40);
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 20, 0x80);
        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(RGO_GRIND_SIDE)).lw(c);
        let grinding = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x70);
        if grinding {
            c.cycles += 5;
            RGO_GET_MAIN_STATE.call(c, 0x0011_5674)?;
            c.g[V1][0] = 1;
            if c.g[V0][0] == 1 {
                c.cycles += 5;
                let side = Addr(r32(c, S3).wrapping_add(RGO_GRIND_SIDE)).lw(c);
                let sideways = (sx((side as u32).wrapping_sub(3)) < 2) as u64;
                c.g[V0][0] = 0x50_0000;
                if sideways != 0 {
                    c.cycles += 25;
                    c.g[S0][0] = 0x50_0000;
                    rgo_reset_root(c, S1, 0x10, 0x0011_56bc)?;
                    c.g[V0][0] = sx(r32(c, S0).wrapping_sub(0xea0));
                    rgo_store_heading(c, 0x10, V0, S1);
                }
                c.cycles += 4;
                let side = Addr(r32(c, S3).wrapping_add(RGO_GRIND_SIDE)).lw(c);
                c.g[V1][0] = side;
                c.g[V0][0] = 4;
                c.g[A0][0] = Addr(r32(c, S3).wrapping_add(RGO_ANIM)).lw(c);
                if side == 3 {
                    c.cycles += 3;
                    c.g[A1][0] = 0x11a;
                } else {
                    c.cycles += 2;
                    if side == 4 {
                        c.cycles += 2;
                        c.g[A1][0] = 0x115;
                    } else {
                        c.cycles += 1;
                        c.g[A1][0] = 0x10c;
                    }
                }
                c.cycles += 6;
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.f[12] = 0xbf80_0000;
                c.g[A2][0] = 0;
                call_ptr(c, RGO_PLAY, 0x0011_5738)?;
                Addr(r32(c, S3).wrapping_add(RGO_GRIND_SIDE)).sw(c, 0);
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                RGO_GET_MAIN_STATE.call(c, 0x0011_5748)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 4;
                    let side = Addr(r32(c, S3).wrapping_add(RGO_GRIND_SIDE)).lw(c);
                    c.g[V1][0] = side;
                    c.g[V0][0] = 4;
                    let turned = if side == 3 {
                        c.cycles += 30;
                        rgo_turn_sideways(
                            c,
                            RGO_QUARTER_TURN_GP,
                            0x18,
                            [0x0011_5784, 0x0011_5790, 0x0011_57b8],
                        )?;
                        true
                    } else {
                        c.cycles += 2;
                        if c.g[V1][0] == c.g[V0][0] {
                            c.cycles += 29;
                            rgo_turn_sideways(
                                c,
                                RGO_NEG_QUARTER_TURN_GP,
                                0x20,
                                [0x0011_5804, 0x0011_5810, 0x0011_5838],
                            )?;
                            true
                        } else {
                            false
                        }
                    };
                    if turned {
                        c.cycles += 8;
                        let sin_at = if side == 3 { 0x18 } else { 0x20 };
                        rgo_store_heading(c, sin_at, S1, S2);
                    }
                    c.g[A0][0] = Addr(r32(c, S3).wrapping_add(RGO_ANIM)).lw(c);
                    c.cycles += 6;
                    c.g[A1][0] = 5;
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.f[12] = 0xbf80_0000;
                    c.g[A2][0] = 0;
                    call_ptr(c, RGO_PLAY, 0x0011_5888)?;
                }
                Addr(r32(c, S3).wrapping_add(RGO_GRIND_SIDE)).sw(c, 0);
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S3][0];
            RGO_EMPTY_STANCE_HOOK.call(c, 0x0011_5894)?;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_OnRead (0x3e0170, ealib/seg_2DCAE8.c)

/// StreamFile fields: its mutex (+4), the read-ahead length (+0x40), the
/// fill cursor (+0x58), the stream header (+0x60), the bytes read so far
/// (+0x170), the EAFile request (+0x174) and the requested length (+0x178).
pub(crate) const SFR_MUTEX: u32 = 0x4;

pub(crate) const SFR_READ_AHEAD: u32 = 0x40;

pub(crate) const SFR_FILL: u32 = 0x58;

pub(crate) const SFR_HEADER: u32 = 0x60;

pub(crate) const SFR_BYTES_READ: u32 = 0x170;

pub(crate) const SFR_REQUEST: u32 = 0x174;

pub(crate) const SFR_REQUEST_LEN: u32 = 0x178;

/// Header fields: the stream state (+4; 3 = done, 4 = closing), the source
/// kind (+0x10; 1 = memory, no EAFile request) and the total size (+0x118).
pub(crate) const SFR_HDR_STATE: u32 = 0x4;

pub(crate) const SFR_HDR_SOURCE: u32 = 0x10;

pub(crate) const SFR_HDR_SIZE: u32 = 0x118;

pub(crate) const SFR_STATE_DONE: u64 = 3;

pub(crate) const SFR_STATE_CLOSING: u64 = 4;

pub(crate) const SFR_RELEASE_REQUEST: Callee = Callee {
    addr: 0x003d_d310,
    f: ee::s14::EAFile_ReleaseRequest,
};

pub(crate) const SFR_PARSE_CHUNKS: Callee = Callee {
    addr: 0x003d_fed0,
    f: ee::s14::StreamFile_ParseChunks,
};

pub(crate) const SFR_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};

pub(crate) const SFR_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

pub(crate) const SFR_START_NEXT_REQUEST: Callee = Callee {
    addr: 0x003e_0270,
    f: ee::s14::StreamFile_StartNextRequest,
};

pub(crate) const SFR_ISSUE_READ: Callee = Callee {
    addr: 0x003e_03d0,
    f: ee::s14::StreamFile_IssueRead,
};

/// `void StreamFile_OnRead(int, int, StreamFile* file)` (0x3e0170): the
/// read-completion callback. Counts the bytes that arrived (the requested
/// length for a memory source, else what `EAFile_ReleaseRequest` reports),
/// advances the fill cursor and parses the new chunks. When the stream is
/// closing, finished (end of the source, a short read) or an end chunk was
/// parsed, marks it done (state 3, under the mutex, unless closing) and
/// starts the next request; otherwise issues the next read.
pub fn stream_file_on_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S2, 0x20);
        let file = r32(c, S0);
        c.g[S2][0] = Addr(file.wrapping_add(SFR_HEADER)).lw(c);
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SFR_HDR_SOURCE)).lw(c);
        if c.g[V0][0] == 1 {
            c.cycles += 7;
            c.g[A1][0] = Addr(file.wrapping_add(SFR_REQUEST_LEN)).lw(c);
            c.g[V1][0] = Addr(file.wrapping_add(SFR_BYTES_READ)).lw(c);
            c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SFR_HDR_SIZE)).lw(c);
            let total = sx(r32(c, V1).wrapping_add(r32(c, A1)));
            c.g[V0][0] = ((total as i64) < (c.g[V0][0] as i64)) as u64;
            // At (or past) the end of the source.
            c.g[S1][0] = c.g[V0][0] ^ 1;
        } else {
            c.cycles += 6;
            c.g[A0][0] = Addr(file.wrapping_add(SFR_REQUEST)).lw(c);
            SFR_RELEASE_REQUEST.call(c, 0x003e_01c4)?;
            let file = r32(c, S0);
            c.g[V1][0] = Addr(file.wrapping_add(SFR_REQUEST_LEN)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            // A short read.
            c.g[S1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[V1][0] = Addr(file.wrapping_add(SFR_BYTES_READ)).lw(c);
        }
        c.cycles += 12;
        let file = r32(c, S0);
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = Addr(file.wrapping_add(SFR_FILL)).lw(c);
        c.g[S3][0] = SFR_STATE_CLOSING;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
        let (bytes_read, fill) = (c.g[V1][0], c.g[V0][0]);
        Addr(file.wrapping_add(SFR_BYTES_READ)).sw(c, bytes_read);
        Addr(file.wrapping_add(SFR_FILL)).sw(c, fill);
        SFR_PARSE_CHUNKS.call(c, 0x003e_01f4)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SFR_HDR_STATE)).lw(c);
        let closing = c.g[V0][0] == c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        if !closing {
            c.cycles += 2;
            let finished = c.g[S1][0] != 0;
            c.g[S1][0] = sx(r32(c, S0).wrapping_add(SFR_MUTEX));
            let mut done = finished;
            if !finished {
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    // More to read: issue the next read.
                    c.g[A1][0] = Addr(r32(c, S0).wrapping_add(SFR_READ_AHEAD)).lw(c);
                    c.cycles += 3;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                    SFR_ISSUE_READ.call(c, 0x003e_0250)?;
                    frame.restore(c, S0, 0x40);
                } else {
                    done = true;
                }
            }
            if done {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                SFR_LOCK.call(c, 0x003e_021c)?;
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SFR_HDR_STATE)).lw(c);
                let closing = c.g[V0][0] == c.g[S3][0];
                c.g[V0][0] = SFR_STATE_DONE;
                if !closing {
                    c.cycles += 1;
                    Addr(r32(c, S2).wrapping_add(SFR_HDR_STATE)).sw(c, SFR_STATE_DONE);
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                SFR_UNLOCK.call(c, 0x003e_0234)?;
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.cycles += 6;
                frame.restore(c, S1, 0x30);
                frame.restore(c, S2, 0x20);
                frame.restore(c, S3, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        // Start the next request.
        c.cycles += 4;
        c.g[A1][0] = Addr(r32(c, A0).wrapping_add(SFR_READ_AHEAD)).lw(c);
        SFR_START_NEXT_REQUEST.call(c, 0x003e_0240)?;
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod impact {

    /// Rider: the board axes (rows of the world matrix) and origin.
    pub const AXIS_SIDE: u32 = 0x160;

    pub const AXIS_FORWARD: u32 = 0x170;

    pub const AXIS_UP: u32 = 0x180;

    pub const ORIGIN: u32 = 0x190;

    /// `mMatrix.row[2]` (its z is the up-tilt used by the ground clamp).
    pub const MATRIX_ROW2: u32 = 0x1c0;

    pub const VELOCITY: u32 = 0x1e0;

    /// Launch argument for the air predictor (`unk_2E4`).
    pub const LAUNCH_ARG: u32 = 0x2e4;

    /// Last impact normal and the accumulated impact strength.
    pub const IMPULSE: u32 = 0x3e0;

    pub const IMPACT_ACCUM: u32 = 0x3f0;

    /// The rider's interface vtable pointer (methods called with `+0x6c0`).
    pub const VTBL: u32 = 0x6c0;

    pub const HANDLER: u32 = 0x77c;

    pub const ANIM: u32 = 0x784;

    pub const AIR_PREDICTOR: u32 = 0x788;

    /// Vtable slots (`{short delta; short; fn}`).
    pub const VT_VIBRATION_BUMP: u32 = 0x88;

    pub const VT_GET_POSITION: u32 = 0x28;

    pub const VT_GET_VELOCITY: u32 = 0x10;

    /// Handler (owner) fields.
    pub const HANDLER_AIR_STATE: u32 = 0x30;

    pub const HANDLER_HIT_IN_AIR: u32 = 0x314;

    pub const HANDLER_HIT_VELOCITY: u32 = 0x320;

    /// Impact info: contact point, normal, impact speed.
    pub const INFO_POS: u32 = 0x0;

    pub const INFO_NORMAL: u32 = 0x20;

    pub const INFO_NORMAL_Z: u32 = 0x28;

    pub const INFO_SPEED: u32 = 0x30;

    /// Animation id meaning "no wipeout: just bump".
    pub const NO_ANIM: u64 = 0x1b6;

    /// gp-relative constants.
    pub const STEEP_NORMAL_Z: i32 = -0x7f08;
    // 0.8
    pub const AIR_SIDE_SPEED: i32 = -0x7f04;
    // 416.532
    pub const AIR_FRONT_SPEED: i32 = -0x7f00;
    // 833.338
    pub const CLAMP_MIN: i32 = -0x7efc;
    // 0.2
    pub const CLAMP_MAX: i32 = -0x7ef8;
    // 1.4
    pub const GROUND_SIDE_SPEED: i32 = -0x7ef4;
    // 1388.88
    pub const GROUND_FRONT_SPEED: i32 = -0x7ef0;
    // 2083.23
    pub const FAST_SPEED: i32 = 0x1b94;
}

pub(crate) mod bow_rider {

    /// `+0x180` the board's facing direction (`unk_180`).
    pub const FACING: u32 = 0x180;

    /// `+0x1e0 mVelocity`.
    pub const VEL: u32 = 0x1e0;

    /// `+0x2e4` the value handed to `AirPredictor_Prime` (`unk_2E4`).
    pub const AIR_PRIME_ARG: u32 = 0x2e4;

    /// `+0x320` current stance, `+0x324 mNaturalStance`.
    pub const STANCE: u32 = 0x320;

    pub const NATURAL_STANCE: u32 = 0x324;

    /// `+0x3f4 mImpactAccum2`: slow wall hits accumulate here.
    pub const IMPACT_ACCUM: u32 = 0x3f4;

    /// `+0x438 mSurfaceType`.
    pub const SURFACE_TYPE: u32 = 0x438;

    /// `+0x6c0` the `RiderIface` sub-object (its vtable pointer first).
    pub const IFACE: u32 = 0x6c0;

    /// `+0x77c mRiderHandler` (`+0x30` its control mode).
    pub const HANDLER: u32 = 0x77c;

    /// `+0x784 mRiderAnim`, `+0x788 mAirPredictor`, `+0x790 mMetrix`.
    pub const ANIM: u32 = 0x784;

    pub const AIR_PREDICTOR: u32 = 0x788;

    pub const METRIX: u32 = 0x790;
}

pub(crate) mod bow_iface {

    pub const VIBRATION_BUMP: u32 = 0x88;

    pub const GET_POSITION: u32 = 0x28;

    pub const GET_VELOCITY: u32 = 0x10;
}

pub(crate) mod bow_hit {

    /// `+0x10` the wall normal, `+0x20` the wall's own velocity.
    pub const NORMAL: u32 = 0x10;

    pub const NORMAL_Z: u32 = 0x18;

    pub const VEL: u32 = 0x20;

    /// `+0x4c` the material handed to `Rider_HandleImpact`.
    pub const MATERIAL: u32 = 0x4c;

    /// `+0x5c` index into the collision object's surface table.
    pub const SURFACE: u32 = 0x5c;
}

pub(crate) mod bow_obj {

    pub const HANDLE: u32 = 0xc;

    pub const SURFACES: u32 = 0x88;

    pub const HANDLE_PUSH_SLOT: u32 = 0x70;
}

pub(crate) mod bow_lit {

    /// 1.1: how far past the wall the rider is pushed out (times depth).
    pub const PUSH_SCALE: u32 = 0x7f30;

    /// 55.6 / 1388.9: the clamp of the grip rebound.
    pub const REBOUND_MIN: u32 = 0x7f2c;

    pub const REBOUND_MAX: u32 = 0x7f28;

    /// 833.3: below this speed a hit adds to the impact accumulator.
    pub const SLOW_SPEED: u32 = 0x7f24;

    /// 0.2: the floor of the normal's z in the accumulator.
    pub const MIN_NORMAL_Z: u32 = 0x7f20;

    /// 0.3: the facing threshold of the launch.
    pub const LAUNCH_FACING: u32 = 0x7f1c;

    /// 277.8: the least outward speed of a launch.
    pub const LAUNCH_MIN_OUT: u32 = 0x7f18;

    /// 555.6 / 27.8: the launch's along-wall speed rule.
    pub const LAUNCH_SPEED_ADD: u32 = 0x7f14;

    pub const LAUNCH_SPEED_MIN: u32 = 0x7f10;

    /// 0.2: blend past which the bail animation restarts.
    pub const BAIL_BLEND: u32 = 0x7f0c;
}

pub(crate) mod bow_callee {
    use super::ee;
    use super::Callee;

    pub const GET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fee8,
        f: ee::s00::Rider_GetSubState,
    };

    pub const GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };

    pub const SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fec8,
        f: ee::s00::Rider_SetSubState,
    };

    pub const PLAY_BUMP_ANIM: Callee = Callee {
        addr: 0x0010_8388,
        f: ee::s00::Rider_PlayBumpAnim,
    };

    pub const TRANSLATE: Callee = Callee {
        addr: 0x0010_6538,
        f: ee::s00::Rider_Translate,
    };

    pub const DEFLECT_ALONG_WALL: Callee = Callee {
        addr: 0x0010_65b0,
        f: ee::s00::Rider_DeflectAlongWall,
    };

    pub const BUILD_WORLD_MATRIX: Callee = Callee {
        addr: 0x0011_e098,
        f: ee::s00::Rider_BuildWorldMatrix,
    };

    pub const BANK_TRICK_BOOST: Callee = Callee {
        addr: 0x0010_e910,
        f: ee::s00::Rider_BankTrickBoost,
    };

    pub const INTERRUPT_TRICK: Callee = Callee {
        addr: 0x0011_9e38,
        f: ee::s00::RiderMetrix_InterruptTrick,
    };

    pub const AIR_PREDICTOR_PRIME: Callee = Callee {
        addr: 0x0011_35b8,
        f: ee::s00::AirPredictor_Prime,
    };

    pub const HANDLE_IMPACT: Callee = Callee {
        addr: 0x0010_5d98,
        f: ee::s00::Rider_HandleImpact,
    };
}

pub(crate) mod bow_table {

    /// `RiderAnimBase_getCurrentAnim(anim, layer)`.
    pub const ANIM_CURRENT: u32 = 0x0031_1ae8;

    /// `RiderAnimBase_getAnimState(anim, layer)` / `..._getBlend` (f0).
    pub const ANIM_STATE: u32 = 0x0031_2aa0;

    pub const ANIM_BLEND: u32 = 0x0031_2ab0;

    /// `cRiderAnimBase_play(anim, id, layer, speed)`.
    pub const ANIM_PLAY: u32 = 0x0031_28e8;
}

pub(crate) const IMPACT_GET_SUB_STATE: Callee = Callee {
    addr: 0x0011_fee8,
    f: ee::s00::Rider_GetSubState,
};

pub(crate) const IMPACT_GET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe98,
    f: ee::s00::Rider_GetMainState,
};

pub(crate) const IMPACT_CHECK_RESET_PATH: Callee = Callee {
    addr: 0x0011_6120,
    f: ee::s00::Rider_CheckResetPath,
};

pub(crate) const IMPACT_AIR_PREDICTOR_PRIME: Callee = Callee {
    addr: 0x0011_35b8,
    f: ee::s00::AirPredictor_Prime,
};

pub(crate) const IMPACT_START_WIPEOUT: Callee = Callee {
    addr: 0x0010_eb30,
    f: ee::s00::Rider_StartWipeout,
};

pub(crate) const IMPACT_PLAY_BUMP_ANIM: Callee = Callee {
    addr: 0x0010_8388,
    f: ee::s00::Rider_PlayBumpAnim,
};

/// `AnimController_GetLayerSequence` (through the function table).
pub(crate) const IMPACT_ANIM_LAYER_SEQUENCE: u32 = 0x0031_1ae8;

/// `Rand_NextGame` (AIrand, through the function table).
pub(crate) const IMPACT_RAND: u32 = 0x0031_7810;

/// `reg + off` (32-bit address).
#[inline(always)]
pub(crate) fn impact_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `gp + off`.
#[inline(always)]
pub(crate) fn impact_gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// The compiler's inline VU0 dot product: vf4.x = vf3 . vf(other) (x, y, z
/// and w; vf6.x = 1).
pub(crate) fn impact_dot(c: &mut Cpu, other: u8) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(other));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// f0 = |d1|, f1 = |d0|, f2 = |d2| (d0..d2 in f22, f20, f21) and whether
/// |d0| < |d1| (6 cycles).
pub(crate) fn impact_abs_dots(c: &mut Cpu) -> bool {
    c.cycles += 6;
    c.abs_s(0, 20);
    c.abs_s(1, 22);
    c.c_lt_s(1, 0);
    c.abs_s(2, 21);
    c.fcc()
}

/// How a branch of the animation choice ends.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ImpactTail {
    /// The end of the choice block (`+1` cycle, v0 = 0x1b6), then `Choose`.
    Picked,
    /// Compare s0 with v0 (0x1b6): wipe out or bump.
    Choose,
    /// Straight to the bump animation.
    Bump,
}

/// Animation ids and return addresses of a side (forward-axis) hit.
pub(crate) struct ImpactSideHit {
    /// Close head-on hit at speed.
    close: u64,
    /// Random head-on wipeouts for rolls 0, 1, 2, 3 and 4..5.
    roll: [u64; 5],
    /// Glancing hit: odd / even random.
    glance: u64,
    glance_even: u64,
    ret_glance: u32,
    ret_roll6: u32,
    ret_roll3: u32,
}

/// Main state 1 (riding): the side-hit set.
pub(crate) const IMPACT_RIDING_SIDE: ImpactSideHit = ImpactSideHit {
    close: 0x156,
    roll: [0x15b, 0x157, 0x158, 0x15a, 0x159],
    glance: 0x155,
    glance_even: 0x154,
    ret_glance: 0x0010_6264,
    ret_roll6: 0x0010_61fc,
    ret_roll3: 0x0010_620c,
};

/// Other main states: the side-hit set.
pub(crate) const IMPACT_GROUND_SIDE: ImpactSideHit = ImpactSideHit {
    close: 0x14c,
    roll: [0x14e, 0x14f, 0x151, 0x14d, 0x150],
    glance: 0x14b,
    glance_even: 0x14a,
    ret_glance: 0x0010_646c,
    ret_roll6: 0x0010_6404,
    ret_roll3: 0x0010_6414,
};

/// `void Rider_HandleImpact(Rider* rider, ImpactInfo* info, int surface)`
/// (0x105d98): the rider hit something. Unless frozen (sub state 9): add
/// `1 - max(info.normal . impulse, 0)` to the impact accumulator and keep
/// the normal; a fatal surface resets the rider to the path. Otherwise rumble
/// (`vibrationBump(speed)`). In the air (main state 2) the hit is only
/// recorded on the handler (and the air predictor re-primed when the
/// handler flies). On the ground it picks a wipeout animation from which
/// board axis the normal mostly faces (side, forward, up), the impact speed
/// (scaled by tilt off the riding state), the distance above the board and
/// the rider's speed (random variants), and starts the wipeout; with no
/// wipeout the bump animation plays.
pub fn rider_handle_impact(c: &mut Cpu) {
    use impact::*;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x90);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S4, 0x50);
        frame.save_ra(c, 0x30);
        for (f, off) in [(23, 0xb8), (22, 0xb0), (21, 0xa8), (20, 0xa0)] {
            frame.save_fpr(c, f, off);
        }
        IMPACT_GET_SUB_STATE.call(c, 0x0010_5dd8)?;
        c.g[V1][0] = 9;
        let frozen = c.g[V0][0] == c.g[V1][0];
        frame.restore(c, S0, 0x90);
        if frozen {
            impact_epilogue(c, frame);
            return Ok(());
        }

        impact_accumulate(c);
        c.lq(V0, impact_at(c, S3, INFO_NORMAL));
        let no_surface = c.g[S5][0] == c.g[V1][0];
        c.sq(V0, impact_at(c, S1, IMPULSE));
        if !no_surface {
            // world->game->surfaces[surface].fatal (0xb0 bytes each).
            c.cycles += 9;
            c.g[A1][0] = c.lw(impact_gp(c, -0x848));
            c.g[V1][0] = 0xb0;
            c.mult(0, V1, S5, V1);
            c.g[A0][0] = c.lw(impact_at(c, A1, 0x84));
            c.g[V0][0] = c.lw(impact_at(c, A0, 0x44));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = c.lw(impact_at(c, V0, 0x44));
            let fatal = c.g[A0][0] != 0;
            c.g[A1][0] = 0;
            if fatal {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = 1;
                IMPACT_CHECK_RESET_PATH.call(c, 0x0010_5e68)?;
                frame.restore(c, S0, 0x90);
                impact_epilogue(c, frame);
                return Ok(());
            }
        }

        // vibrationBump(info.speed).
        c.cycles += 12;
        c.g[V1][0] = c.lw(impact_at(c, S1, VTBL));
        c.g[S2][0] = sx(impact_at(c, S1, VTBL));
        c.lwc1(12, impact_at(c, S3, INFO_SPEED));
        c.g[A0][0] = c.lh(impact_at(c, V1, VT_VIBRATION_BUMP));
        c.g[V0][0] = c.lw(impact_at(c, V1, VT_VIBRATION_BUMP + 4));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x0010_5e8c)?;
        c.g[A0][0] = c.g[S1][0];
        IMPACT_GET_MAIN_STATE.call(c, 0x0010_5e94)?;
        c.g[V1][0] = 2;
        if c.g[V0][0] == c.g[V1][0] {
            impact_in_air(c)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
            impact_epilogue(c, frame);
            return Ok(());
        }
        c.g[A0][0] = c.g[S1][0];

        // Riding (main state 1): prime the air predictor.
        c.cycles += 4;
        c.g[S4][0] = 1;
        IMPACT_GET_MAIN_STATE.call(c, 0x0010_5f10)?;
        if c.g[V0][0] == c.g[S4][0] {
            c.cycles += 17;
            impact_prime_air_predictor(c, [0x0010_5f2c, 0x0010_5f44, 0x0010_5f58])?;
        }
        c.lq(V0, impact_at(c, S1, AXIS_SIDE));

        impact_measure(c)?;
        c.g[A0][0] = c.lw(impact_at(c, S1, ANIM));
        call_ptr(c, IMPACT_ANIM_LAYER_SEQUENCE, 0x0010_6038)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        IMPACT_GET_SUB_STATE.call(c, 0x0010_6044)?;
        c.g[V1][0] = 0xb;
        let wiping_out = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = NO_ANIM;
        let mut tail = if wiping_out {
            ImpactTail::Choose
        } else {
            impact_pick_anim(c)?
        };

        if tail == ImpactTail::Picked {
            c.cycles += 1;
            c.g[V0][0] = NO_ANIM;
            tail = ImpactTail::Choose;
        }
        if tail == ImpactTail::Choose {
            c.cycles += 2;
            let none = c.g[S0][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            if !none {
                // Rider_StartWipeout(rider, anim, 0, surface, info).
                c.cycles += 7;
                c.g[A1][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S5][0];
                c.g[T0][0] = c.g[S3][0];
                c.g[A2][0] = 0;
                IMPACT_START_WIPEOUT.call(c, 0x0010_64ec)?;
                frame.restore(c, S0, 0x90);
                impact_epilogue(c, frame);
                return Ok(());
            }
        }
        // Rider_PlayBumpAnim(rider, info, 1).
        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = 1;
        IMPACT_PLAY_BUMP_ANIM.call(c, 0x0010_6504)?;
        c.cycles += 1;
        frame.restore(c, S0, 0x90);
        impact_epilogue(c, frame);
        Ok(())
    })
}

/// `impactAccum (+0x3f0) += 1 - max(info.normal . impulse, 0)` (21 cycles;
/// s1 = rider, s3 = info; leaves v1 = -1).
///
/// Tick rate: the wall-contact path calls `Rider_HandleImpact` on every tick
/// the rider pushes into a wall, and `Rider_UpdateImpacts` decays this
/// accumulator per tick (`tick::PER_TICK`, 0.956 converted as a decay) and
/// resets the rider to the path above 4.5: the reset is the "stuck against a
/// wall" detector. The gain is therefore a per-tick amount and is scaled by
/// `60 / rate` off 60 Hz (same time to the reset at every rate); at 60 the
/// original's `add.s` is kept bit for bit.
pub(crate) fn impact_accumulate(c: &mut Cpu) {
    use impact::*;
    c.cycles += 21;
    c.mtc1(0, 3);
    c.lqc2(3, impact_at(c, S3, INFO_NORMAL));
    c.lqc2(5, impact_at(c, S1, IMPULSE));
    impact_dot(c, 5);
    c.qmfc2(V0, 4);
    c.mtc1(V0, 1);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 2);
    c.g[V1][0] = u64::MAX;
    c.max_s(1, 1, 3);
    c.lwc1(0, impact_at(c, S1, IMPACT_ACCUM));
    c.sub_s(2, 2, 1);
    if crate::tick::rate(c) != 60 {
        crate::game::ai::rider::per_tick_60(c, 2);
    }
    c.add_s(0, 0, 2);
    c.swc1(0, impact_at(c, S1, IMPACT_ACCUM));
}

/// Restore s1-s5, ra, f20-f23 and return (12 cycles).
pub(crate) fn impact_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 12;
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore_ra(c, 0x30);
    for (f, off) in [(23, 0xb8), (22, 0xb0), (21, 0xa8), (20, 0xa0)] {
        frame.restore_fpr(c, f, off);
    }
    frame.close(c);
}

/// In the air (main state 2): when the handler flies (air state 1), prime the
/// air predictor; flag the hit on the handler and keep the velocity.
pub(crate) fn impact_in_air(c: &mut Cpu) -> crate::call::Flow {
    use impact::*;
    c.cycles += 5;
    c.g[V1][0] = c.lw(impact_at(c, S1, HANDLER));
    c.g[S3][0] = 1;
    c.g[V0][0] = c.lw(impact_at(c, V1, HANDLER_AIR_STATE));
    if c.g[V0][0] == c.g[S3][0] {
        c.cycles += 18;
        impact_prime_air_predictor(c, [0x0010_5ec8, 0x0010_5ee0, 0x0010_5ef4])?;
        c.g[V1][0] = c.lw(impact_at(c, S1, HANDLER));
    }
    c.sw(impact_at(c, V1, HANDLER_HIT_IN_AIR), c.g[S3][0]);
    c.cycles += 3;
    c.lq(V0, impact_at(c, S1, VELOCITY));
    c.sq(V0, impact_at(c, V1, HANDLER_HIT_VELOCITY));
    Ok(())
}

/// `AirPredictor_Prime(rider->airPredictor, getPosition(), getVelocity(),
/// rider->launchArg)` (s2 = rider + 0x6c0, the method `this` base).
pub(crate) fn impact_prime_air_predictor(c: &mut Cpu, rets: [u32; 3]) -> crate::call::Flow {
    use impact::*;
    c.g[V1][0] = c.lw(impact_at(c, S1, VTBL));
    c.g[A0][0] = c.lh(impact_at(c, V1, VT_GET_POSITION));
    c.g[V0][0] = c.lw(impact_at(c, V1, VT_GET_POSITION + 4));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, rets[0])?;
    c.g[V1][0] = c.lw(impact_at(c, S1, VTBL));
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.lh(impact_at(c, V1, VT_GET_VELOCITY));
    c.g[V0][0] = c.lw(impact_at(c, V1, VT_GET_VELOCITY + 4));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, rets[1])?;
    c.g[A0][0] = c.lw(impact_at(c, S1, AIR_PREDICTOR));
    c.g[A1][0] = c.g[S0][0];
    c.lwc1(12, impact_at(c, S1, LAUNCH_ARG));
    c.g[A2][0] = c.g[V0][0];
    IMPACT_AIR_PREDICTOR_PRIME.call(c, rets[2])
}

/// The 61-cycle block before the layer query: d0 = normal . side (f22),
/// d1 = normal . forward (f20), d2 = normal . up (f21) and the contact
/// height above the board, (pos - origin) . up (f23). v0 holds the side axis.
pub(crate) fn impact_measure(c: &mut Cpu) -> crate::call::Flow {
    use impact::*;
    c.cycles += 61;
    c.g[A1][0] = 2;
    c.lq(A0, impact_at(c, S1, AXIS_FORWARD));
    c.g[S0][0] = NO_ANIM;
    let s = sp(c);
    c.sq(V0, s);
    c.lqc2(3, s);
    c.lqc2(5, impact_at(c, S3, INFO_NORMAL));
    impact_dot(c, 5);
    c.qmfc2(A2, 4);
    c.mtc1(A2, 22);
    c.lq(V1, impact_at(c, S1, AXIS_UP));
    c.sq(A0, s);
    c.lqc2(3, s);
    c.lqc2(5, impact_at(c, S3, INFO_NORMAL));
    impact_dot(c, 5);
    c.qmfc2(A2, 4);
    c.mtc1(A2, 20);
    c.lq(V0, impact_at(c, S1, ORIGIN));
    c.sq(V1, s);
    c.lqc2(3, s);
    c.lqc2(5, impact_at(c, S3, INFO_NORMAL));
    impact_dot(c, 5);
    c.qmfc2(A0, 4);
    c.mtc1(A0, 21);
    c.sq(V0, s);
    c.lqc2(3, impact_at(c, S3, INFO_POS));
    c.lqc2(4, s);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x20));
    c.lq(V0, s.wrapping_add(0x20));
    c.lq(V1, impact_at(c, S1, AXIS_UP));
    c.sq(V0, s.wrapping_add(0x10));
    c.sq(V1, s.wrapping_add(0x20));
    c.lqc2(3, s.wrapping_add(0x10));
    c.lqc2(5, s.wrapping_add(0x20));
    impact_dot(c, 5);
    c.qmfc2(V0, 4);
    c.mtc1(V0, 23);
    Ok(())
}

/// The animation choice when the rider is not already wiping out (sub
/// state != 0xb). s2 = the current layer-2 sequence, s0 = 0x1b6.
pub(crate) fn impact_pick_anim(c: &mut Cpu) -> Result<ImpactTail, crate::call::Unwound> {
    use impact::*;
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    IMPACT_GET_MAIN_STATE.call(c, 0x0010_6058)?;
    if c.g[V0][0] == c.g[S4][0] {
        // Riding into a steep wall (normal.z > 0.8)?
        c.cycles += 6;
        c.lwc1(1, impact_at(c, S3, INFO_NORMAL_Z));
        c.lwc1(0, impact_gp(c, STEEP_NORMAL_Z));
        c.c_lt_s(0, 1);
        let steep = c.fcc();
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x14));
        if steep {
            impact_pick_steep(c);
            return Ok(ImpactTail::Picked);
        }
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S1][0];
    IMPACT_GET_MAIN_STATE.call(c, 0x0010_6138)?;
    c.g[V1][0] = 1;
    if c.g[V0][0] != c.g[V1][0] {
        c.lq(V0, impact_at(c, S1, MATRIX_ROW2));
        return impact_pick_ground(c);
    }
    impact_pick_riding(c)
}

/// Riding into a steep wall: sequences 0x14/0x15 -> 0x169; else by the
/// dominant axis: forward 0x162/0x163, up 0x15e (or 0x15f from sequences
/// 0x12/0x13), side 0x165/0x164. v0 = sequence - 0x14.
pub(crate) fn impact_pick_steep(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[S0][0] = 0x169;
        return;
    }
    if impact_abs_dots(c) {
        c.cycles += 3;
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_lt_s(20, 0);
            let back = c.fcc();
            c.g[S0][0] = 0x163;
            if back {
                c.cycles += 2;
                c.g[S0][0] = 0x162;
            }
            return;
        }
    }
    c.cycles += 3;
    c.c_lt_s(1, 2);
    if c.fcc() {
        c.cycles += 3;
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_lt_s(21, 0);
            let below = c.fcc();
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x12));
            if below {
                c.cycles += 2;
                c.g[S0][0] = 0x15e;
                return;
            }
            c.cycles += 5;
            c.g[V1][0] = 1;
            c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
            c.g[A0][0] = 0x15f;
            if c.g[V1][0] == 0 {
                c.g[S0][0] = c.g[A0][0];
            }
            return;
        }
    }
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(22, 0);
    let left = c.fcc();
    c.g[S0][0] = 0x164;
    if left {
        c.cycles += 2;
        c.g[S0][0] = 0x165;
    }
}

/// Riding (main state 1), not steep: forward hits above 416.5 (side
/// set), up hits 0x15e / 0x15f (sequences 0x12..0x15), side hits above 833.3
/// 0x153/0x152; any wipeout from sequences 0x14/0x15 becomes 0x169.
pub(crate) fn impact_pick_riding(c: &mut Cpu) -> Result<ImpactTail, crate::call::Unwound> {
    use impact::*;
    let via_end = impact_pick_riding_anim(c)?;
    if via_end {
        c.cycles += 1;
        c.g[V0][0] = NO_ANIM;
    }
    c.cycles += 2;
    let none = c.g[S0][0] == c.g[V0][0];
    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x14));
    if none {
        return Ok(ImpactTail::Bump);
    }
    c.cycles += 5;
    c.g[V1][0] = 1;
    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
    c.g[A0][0] = 0x169;
    if c.g[V1][0] == 0 {
        c.g[S0][0] = c.g[A0][0];
    }
    Ok(ImpactTail::Picked)
}

/// The riding choice proper; true when it leaves through the block end
/// (`+1`, v0 = 0x1b6), false when it jumps to the compare with v0 = 0x1b6.
pub(crate) fn impact_pick_riding_anim(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    use impact::*;
    if impact_abs_dots(c) {
        c.cycles += 3;
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.cycles += 6;
            c.lwc1(1, impact_gp(c, AIR_SIDE_SPEED));
            c.lwc1(0, impact_at(c, S3, INFO_SPEED));
            c.c_lt_s(1, 0);
            let hard = c.fcc();
            c.g[V0][0] = NO_ANIM;
            if !hard {
                return Ok(false);
            }
            return impact_side_hit(c, &IMPACT_RIDING_SIDE);
        }
    }
    c.cycles += 3;
    c.c_lt_s(1, 2);
    if c.fcc() {
        c.cycles += 3;
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_lt_s(21, 0);
            let below = c.fcc();
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x12));
            if below {
                c.cycles += 2;
                c.g[S0][0] = 0x15e;
                return Ok(true);
            }
            c.cycles += 5;
            c.g[V1][0] = 3;
            c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
            c.g[A0][0] = 0x15f;
            if c.g[V1][0] == 0 {
                c.g[S0][0] = c.g[A0][0];
            }
            return Ok(true);
        }
    }
    c.cycles += 6;
    c.lwc1(1, impact_gp(c, AIR_FRONT_SPEED));
    c.lwc1(0, impact_at(c, S3, INFO_SPEED));
    c.c_lt_s(1, 0);
    let hard = c.fcc();
    c.g[V0][0] = NO_ANIM;
    if !hard {
        return Ok(false);
    }
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(22, 0);
    let left = c.fcc();
    c.g[S0][0] = 0x152;
    if !left {
        return Ok(false);
    }
    c.cycles += 1;
    c.g[S0][0] = 0x153;
    Ok(true)
}

/// Other main states: the speed limits scale with
/// k = clamp(row2.z + 1 - |normal.z|, 0.2, 1.4) (f4); forward hits above
/// k * 1388.9 (side set), up hits nothing, side hits above k * 2083.2
/// 0x149/0x148. v0 holds `mMatrix.row[2]`.
pub(crate) fn impact_pick_ground(c: &mut Cpu) -> Result<ImpactTail, crate::call::Unwound> {
    use impact::*;
    c.cycles += 15;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3);
    let s = sp(c);
    c.sq(V0, s.wrapping_add(0x20));
    c.lwc1(2, impact_gp(c, CLAMP_MIN));
    c.lwc1(0, s.wrapping_add(0x28));
    c.lwc1(1, impact_at(c, S3, INFO_NORMAL_Z));
    c.mov_s(4, 2);
    c.add_s(0, 0, 3);
    c.abs_s(1, 1);
    c.lwc1(3, impact_gp(c, CLAMP_MAX));
    c.sub_s(0, 0, 1);
    c.c_le_s(2, 0);
    if c.fcc() {
        c.min_s(4, 0, 3);
    }
    if impact_abs_dots(c) {
        c.cycles += 3;
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.cycles += 7;
            c.lwc1(0, impact_gp(c, GROUND_SIDE_SPEED));
            c.lwc1(1, impact_at(c, S3, INFO_SPEED));
            c.mul_s(0, 4, 0);
            c.c_lt_s(0, 1);
            let hard = c.fcc();
            c.g[V0][0] = NO_ANIM;
            if !hard {
                return Ok(ImpactTail::Choose);
            }
            return Ok(if impact_side_hit(c, &IMPACT_GROUND_SIDE)? {
                ImpactTail::Picked
            } else {
                ImpactTail::Choose
            });
        }
    }
    c.cycles += 3;
    c.c_lt_s(1, 2);
    if c.fcc() {
        c.cycles += 4;
        c.c_lt_s(0, 2);
        let up = c.fcc();
        c.g[V0][0] = NO_ANIM;
        if up {
            return Ok(ImpactTail::Choose);
        }
    }
    c.cycles += 7;
    c.lwc1(0, impact_gp(c, GROUND_FRONT_SPEED));
    c.lwc1(1, impact_at(c, S3, INFO_SPEED));
    c.mul_s(0, 4, 0);
    c.c_lt_s(0, 1);
    let hard = c.fcc();
    c.g[V0][0] = NO_ANIM;
    if !hard {
        return Ok(ImpactTail::Choose);
    }
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(22, 0);
    let left = c.fcc();
    c.g[S0][0] = 0x148;
    if !left {
        return Ok(ImpactTail::Choose);
    }
    c.cycles += 1;
    c.g[S0][0] = 0x149;
    Ok(ImpactTail::Picked)
}

/// A hard hit on the forward axis. Facing it (d1 >= 0): a random glance
/// (odd/even). Head-on: close (height < 60) and fast (|velocity| > the fast
/// speed) -> `close` (returns false: straight to the compare); else a
/// random head-on wipeout out of 6 (fast) or 3. True: through the block end.
pub(crate) fn impact_side_hit(
    c: &mut Cpu,
    ids: &ImpactSideHit,
) -> Result<bool, crate::call::Unwound> {
    use impact::*;
    c.cycles += 4;
    c.mtc1(0, 0);
    c.c_lt_s(20, 0);
    if !c.fcc() {
        c.cycles += 6;
        c.g[S0][0] = ids.glance;
        call_ptr(c, IMPACT_RAND, ids.ret_glance)?;
        c.g[V0][0] &= 1;
        c.g[V1][0] = ids.glance_even;
        if c.g[V0][0] == 0 {
            c.g[S0][0] = c.g[V1][0];
        }
        return Ok(true);
    }
    // f2 = |velocity| (VU0 sqrt), f0 = 60.
    c.cycles += 16;
    c.g[AT][0] = 0x4270_0000;
    c.mtc1(AT, 0);
    c.lqc2(3, impact_at(c, S1, VELOCITY));
    impact_dot(c, 3);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V0, 22);
    c.mtc1(V0, 2);
    c.c_lt_s(23, 0);
    let close = c.fcc();
    c.lwc1(1, impact_gp(c, FAST_SPEED));
    if close {
        c.cycles += 4;
        c.c_lt_s(1, 2);
        c.g[S0][0] = ids.close;
        if c.fcc() {
            c.cycles += 2;
            c.g[V0][0] = NO_ANIM;
            return Ok(false);
        }
    } else {
        c.cycles += 1;
        c.c_lt_s(1, 2);
    }
    c.cycles += 2;
    if c.fcc() {
        c.cycles += 4;
        call_ptr(c, IMPACT_RAND, ids.ret_roll6)?;
        c.g[V1][0] = 6;
    } else {
        c.cycles += 3;
        call_ptr(c, IMPACT_RAND, ids.ret_roll3)?;
        c.g[V1][0] = 3;
    }
    c.cycles += 3;
    c.divu(0, V0, V1);
    c.cycles += 3;
    c.g[A0][0] = c.hi[0];
    c.g[V0][0] = 1;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[S0][0] = ids.roll[0];
        return Ok(true);
    }
    c.cycles += 2;
    let not_one = c.g[A0][0] != c.g[V0][0];
    c.g[V0][0] = 2;
    if !not_one {
        c.cycles += 2;
        c.g[S0][0] = ids.roll[1];
        return Ok(true);
    }
    c.cycles += 2;
    let two = c.g[A0][0] == c.g[V0][0];
    c.g[S0][0] = ids.roll[2];
    if two {
        return Ok(true);
    }
    c.cycles += 5;
    c.g[V1][0] = c.g[A0][0] ^ 3;
    c.g[S0][0] = ids.roll[4];
    c.g[V0][0] = ids.roll[3];
    if c.g[V1][0] == 0 {
        c.g[S0][0] = c.g[V0][0];
    }
    Ok(true)
}

/// The bail animation the launch plays (and checks for).
pub(crate) const BOW_BAIL_ANIM: u64 = 0x10c;

#[inline(always)]
pub(crate) fn bow_f(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

#[inline(always)]
pub(crate) fn bow_set_f(c: &mut Cpu, n: usize, v: FpuF32) {
    abi::set_fpr(c, n, v)
}

/// A gp-relative `.lit4` constant.
#[inline(always)]
pub(crate) fn bow_lit(c: &mut Cpu, neg_off: u32) -> FpuF32 {
    let gp = r32(c, GP);
    Addr(gp.wrapping_sub(neg_off)).read_fpu(c)
}

/// The original's dot macro: `vf4.x = vf[a] . vf[b]` over all four lanes
/// (`vf6.x = 1`, `ACC = x + y`, `+ z`, `+ w`).
pub(crate) fn bow_dot4(c: &mut Cpu, a: u8, b: u8) {
    let v = c.vu0();
    v.add(0b1000, 6, 0, Src::Bc(0, W));
    v.mul(0b1111, 4, a, Src::Vec(b));
    v.adda(0b1000, 4, Src::Bc(4, Y));
    v.madda(0b1000, 6, Src::Bc(4, Z));
    v.madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// The length of `vf3` into `v{reg}` (`sqrt` of the dot, read back as Q).
pub(crate) fn bow_length3(c: &mut Cpu, reg: usize) {
    bow_dot4(c, 3, 3);
    let v = c.vu0();
    v.sqrt(4, 0);
    v.waitq();
    c.cfc2(reg, 22);
}

/// `vf5 = vf4 * s` with `s` moved through `v{reg}` into `vf3`
/// (`mfc1; qmtc2; vmulx`).
pub(crate) fn bow_scale_vf4(c: &mut Cpu, reg: usize, s_freg: usize) {
    c.mfc1(reg, s_freg);
    c.qmtc2(reg, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
}

/// The closing speed `n . (hit.vel - rider.vel)` into `v1` (the copy of
/// the difference goes through `scratch(sp)` into `slot(sp)`).
pub(crate) fn bow_closing_speed(c: &mut Cpu, scratch: u32, slot: u32) {
    let (rider, hit) = (r32(c, S2), r32(c, S1));
    c.lqc2(3, hit.wrapping_add(bow_hit::VEL));
    c.lqc2(4, rider.wrapping_add(bow_rider::VEL));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(scratch));
    c.lq(V0, sp(c).wrapping_add(scratch));
    c.sq(V0, sp(c).wrapping_add(slot));
    c.lqc2(3, hit.wrapping_add(bow_hit::NORMAL));
    c.lqc2(5, sp(c).wrapping_add(slot));
    bow_dot4(c, 3, 5);
    c.qmfc2(V1, 4);
}

/// Restore the frame (s0 first, as each path does after its last call) and
/// return.
pub(crate) fn bow_epilogue(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, 0x80);
    c.cycles += 10;
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore_ra(c, 0x40);
    frame.restore_fpr(c, 23, 0xa8);
    frame.restore_fpr(c, 22, 0xa0);
    frame.restore_fpr(c, 21, 0x98);
    frame.restore_fpr(c, 20, 0x90);
    frame.close(c);
}

/// A call through the `{i16 delta; fn}` slot `off` of the vtable at `vt`
/// on the object `this` (`a0 = this + delta`).
pub(crate) fn bow_virtual(
    c: &mut Cpu,
    this: u32,
    vt: u32,
    off: u32,
    ret: u32,
) -> crate::call::Flow {
    let delta = Addr(vt.wrapping_add(off)).lh(c);
    c.g[A0][0] = delta;
    let f = Addr(vt.wrapping_add(off + 4)).lw(c);
    c.g[V0][0] = f;
    c.g[A0][0] = sx(this.wrapping_add(delta as u32));
    call_ptr(c, f as u32, ret)
}

/// The grip of the hit surface, clamped to [0, 1] (f22); leaves the
/// surface entry's address in a0 and f0/f1/f12 as the original does.
pub(crate) fn bow_surface_grip(c: &mut Cpu) -> u32 {
    c.cycles += 13;
    let (obj, hit) = (r32(c, S0), r32(c, S1));
    c.g[V0][0] = Addr(hit.wrapping_add(bow_hit::SURFACE)).lw(c);
    c.g[A0][0] = Addr(obj.wrapping_add(bow_obj::SURFACES)).lw(c);
    // v1 = 12 (the entry size).
    c.mult(0, V0, V0, V1);
    c.f[12] = 0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[1] = 0x3f80_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
    let entry = (c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32);
    c.g[A0][0] = sx(entry);
    let raw = Addr(entry.wrapping_add(4)).read_fpu(c);
    bow_set_f(c, 0, raw);
    let positive = FpuF32::ZERO.le(raw);
    abi::set_fcc(c, positive);
    let mut grip = FpuF32::ZERO;
    if positive {
        c.cycles += 1;
        grip = raw.min(FpuF32::ONE);
    }
    bow_set_f(c, 22, grip);
    entry
}

/// Pushed out of a solid wall: translate by `n * depth * 1.1`, the rebound
/// `vn + clamp(grip * vn)` into f20, deflect along the wall on the ground,
/// rebuild the world matrix.
pub(crate) fn bow_push_out(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 17;
    let scale = bow_lit(c, bow_lit::PUSH_SCALE);
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[SP][0];
    let push = bow_f(c, 23) * scale;
    bow_set_f(c, 0, push);
    let hit = r32(c, S1);
    c.lqc2(4, hit.wrapping_add(bow_hit::NORMAL));
    bow_scale_vf4(c, V1, 0);
    c.sqc2(5, sp(c).wrapping_add(0x10));
    c.lq(V0, sp(c).wrapping_add(0x10));
    c.sq(V0, sp(c));
    bow_callee::TRANSLATE.call(c, 0x0010_594c)?;
    c.f[0] = 0;
    let vn = bow_f(c, 21);
    if abi::c_lt_s(c, vn, FpuF32::ZERO) {
        bow_set_f(c, 21, FpuF32::ZERO);
    }

    c.cycles += 7;
    let vn = bow_f(c, 21);
    let rebound = bow_f(c, 22) * vn;
    bow_set_f(c, 1, rebound);
    let lo = bow_lit(c, bow_lit::REBOUND_MIN);
    bow_set_f(c, 0, lo);
    let hi = bow_lit(c, bow_lit::REBOUND_MAX);
    bow_set_f(c, 2, hi);
    let above = lo.le(rebound);
    abi::set_fcc(c, above);
    let mut clamped = lo;
    if above {
        c.cycles += 1;
        clamped = rebound.min(hi);
    }
    bow_set_f(c, 3, clamped);

    c.cycles += 5;
    bow_set_f(c, 20, vn + clamped);
    c.g[A0][0] = c.g[S2][0];
    bow_callee::GET_MAIN_STATE.call(c, 0x0010_598c)?;
    let on_ground = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S2][0];
    if on_ground {
        c.cycles += 2;
        c.g[A1][0] = c.g[S1][0];
        bow_callee::DEFLECT_ALONG_WALL.call(c, 0x0010_599c)?;
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    bow_callee::BUILD_WORLD_MATRIX.call(c, 0x0010_59a4)?;
    Ok(())
}

/// A soft wall: the rebound `sqrt(max(b^2 - c, 0)) - b` with
/// `k = 1 / (100 / bounce + 1)`, `b = vn * k`, `c = (1 - grip) * k` (f20).
pub(crate) fn bow_soft_rebound(c: &mut Cpu) {
    c.cycles += 17;
    c.g[AT][0] = 0x42c8_0000;
    let hundred = FpuF32(0x42c8_0000);
    c.g[AT][0] = 0x3f80_0000;
    let one = FpuF32::ONE;
    bow_set_f(c, 1, one);
    let bounce = bow_f(c, 20);
    let grip = bow_f(c, 22);
    let vn = bow_f(c, 21);
    let q = hundred.div(bounce);
    let slip = one - grip;
    c.f[4] = 0;
    let q = q + one;
    let k = one.div(q);
    bow_set_f(c, 1, k);
    let b = vn * k;
    bow_set_f(c, 2, b);
    let slip = slip * k;
    bow_set_f(c, 3, slip);
    let d = (b * b - slip).max(FpuF32::ZERO).sqrt();
    bow_set_f(c, 0, d);
    bow_set_f(c, 20, d - b);
}

/// Slow hits (main state 1, or 2 with the handler in mode 1) add
/// `(2.5 - facing) * max(n.z, 0.2)` to the impact accumulator when the
/// rider is slower than 833. Leaves a0 = the animation.
///
/// Tick rate: like [`impact_accumulate`], the gain is per wall-contact tick
/// (`Rider_UpdateImpacts` decays +0x3f4 per tick, 0.978 in
/// `tick::PER_TICK`, and resets the rider above 5), so off 60 Hz it is scaled
/// by `60 / rate`; at 60 it is the original sum.
pub(crate) fn bow_accumulate_impact(c: &mut Cpu) {
    c.cycles += 15;
    let limit = bow_lit(c, bow_lit::SLOW_SPEED);
    bow_set_f(c, 0, limit);
    let rider = r32(c, S2);
    c.lqc2(3, rider.wrapping_add(bow_rider::VEL));
    bow_length3(c, V0);
    c.mtc1(V0, 1);
    let speed = bow_f(c, 1);
    let slow = abi::c_lt_s(c, speed, limit);
    let accum = rider.wrapping_add(bow_rider::IMPACT_ACCUM);
    c.g[V0][0] = sx(accum);
    if slow {
        c.cycles += 8;
        let hit = r32(c, S1);
        let nz = Addr(hit.wrapping_add(bow_hit::NORMAL_Z)).read_fpu(c);
        bow_set_f(c, 2, nz);
        let floor = bow_lit(c, bow_lit::MIN_NORMAL_Z);
        bow_set_f(c, 3, floor);
        c.g[AT][0] = 0x4020_0000;
        let steep = floor.le(nz);
        abi::set_fcc(c, steep);
        let old = Addr(accum).read_fpu(c);
        bow_set_f(c, 1, old);
        let weight = FpuF32(0x4020_0000) - bow_f(c, 22);
        let add = if steep {
            c.cycles += 2;
            weight * nz
        } else {
            c.cycles += 1;
            weight * floor
        };
        c.cycles += 2;
        let add = if crate::tick::rate(c) != 60 {
            crate::game::ai::rider::rate_step_60(c, add)
        } else {
            add
        };
        let sum = old + add;
        bow_set_f(c, 0, sum);
        Addr(accum).write_fpu(c, sum);
    }
    c.cycles += 1;
    c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::ANIM)).lw(c);
}

/// The launch off the wall: keep the along-wall velocity (rescaled to
/// `(speed + 555.6) / 2 / max(speed, 27.8)`), push out at
/// `max(rebound - vn, 277.8)`, bump the pad, restart the bail animation
/// (unless it is already playing early), and prime the air predictor.
pub(crate) fn bow_launch(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 35;
    let (rider, hit) = (r32(c, S2), r32(c, S1));
    c.lqc2(3, rider.wrapping_add(bow_rider::VEL));
    c.lqc2(4, hit.wrapping_add(bow_hit::VEL));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x20));
    let out = bow_f(c, 20) - bow_f(c, 21);
    bow_set_f(c, 1, out);
    c.lq(V0, sp(c).wrapping_add(0x20));
    let min_out = bow_lit(c, bow_lit::LAUNCH_MIN_OUT);
    bow_set_f(c, 0, min_out);
    c.sq(V0, sp(c).wrapping_add(0x10));
    c.lqc2(4, hit.wrapping_add(bow_hit::NORMAL));
    bow_scale_vf4(c, V1, 21);
    c.sqc2(5, sp(c).wrapping_add(0x30));
    let faster = abi::c_lt_s(c, min_out, out);
    c.lq(V0, sp(c).wrapping_add(0x30));
    bow_set_f(c, 3, min_out);
    c.sq(V0, sp(c).wrapping_add(0x20));
    c.lqc2(3, sp(c).wrapping_add(0x10));
    c.lqc2(4, sp(c).wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x30));
    c.lq(V0, sp(c).wrapping_add(0x30));
    c.sq(V0, sp(c));
    c.lqc2(3, sp(c));
    bow_length3(c, V1);
    c.mtc1(V1, 20);
    if faster {
        bow_set_f(c, 3, out);
    }

    c.cycles += 9;
    let add = bow_lit(c, bow_lit::LAUNCH_SPEED_ADD);
    let min_speed = bow_lit(c, bow_lit::LAUNCH_SPEED_MIN);
    bow_set_f(c, 2, min_speed);
    let speed = bow_f(c, 20);
    c.g[AT][0] = 0x3f00_0000;
    bow_set_f(c, 1, FpuF32(0x3f00_0000));
    let slow = speed.le(min_speed);
    abi::set_fcc(c, slow);
    let half = (speed + add) * FpuF32(0x3f00_0000);
    let scale = if slow {
        c.cycles += 3;
        half.div(min_speed)
    } else {
        c.cycles += 1;
        half.div(speed)
    };
    bow_set_f(c, 0, scale);

    c.cycles += 31;
    c.lqc2(4, sp(c));
    bow_scale_vf4(c, V0, 0);
    c.sqc2(5, sp(c));
    c.f[12] = c.f[21];
    c.lqc2(4, hit.wrapping_add(bow_hit::NORMAL));
    bow_scale_vf4(c, V1, 3);
    c.sqc2(5, sp(c).wrapping_add(0x30));
    c.lq(V0, sp(c).wrapping_add(0x30));
    c.sq(V0, sp(c).wrapping_add(0x20));
    c.lqc2(3, sp(c));
    c.lqc2(4, sp(c).wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x30));
    c.lq(V0, sp(c).wrapping_add(0x30));
    let vt = Addr(rider.wrapping_add(bow_rider::IFACE)).lw(c);
    c.g[V1][0] = vt;
    c.sq(V0, rider.wrapping_add(bow_rider::VEL));
    c.sq(V0, sp(c).wrapping_add(0x10));
    bow_virtual(c, rider, vt as u32, bow_iface::VIBRATION_BUMP, 0x0010_5c40)?;

    let rider = r32(c, S2);
    c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::ANIM)).lw(c);
    c.g[A1][0] = 2;
    call_ptr(c, bow_table::ANIM_STATE, 0x0010_5c4c)?;
    c.g[V1][0] = BOW_BAIL_ANIM;
    let restart = if c.g[V0][0] != BOW_BAIL_ANIM {
        true
    } else {
        c.cycles += 8;
        let rider = r32(c, S2);
        c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::ANIM)).lw(c);
        c.g[A1][0] = 2;
        call_ptr(c, bow_table::ANIM_BLEND, 0x0010_5c64)?;
        let limit = bow_lit(c, bow_lit::BAIL_BLEND);
        bow_set_f(c, 1, limit);
        let blend = bow_f(c, 0);
        let late = abi::c_lt_s(c, limit, blend);
        if late {
            c.cycles += 1;
        }
        late
    };
    let rider = r32(c, S2);
    if restart {
        Addr(rider.wrapping_add(bow_rider::SURFACE_TYPE)).sw(c, 0);
        bow_restart_bail(c)?;
    }
    let rider = r32(c, S2);
    c.g[V1][0] = Addr(rider.wrapping_add(bow_rider::IFACE)).lw(c);

    c.cycles += 18;
    let iface = rider.wrapping_add(bow_rider::IFACE);
    c.g[S0][0] = sx(iface);
    let vt = c.g[V1][0] as u32;
    bow_virtual(c, iface, vt, bow_iface::GET_POSITION, 0x0010_5cfc)?;
    let rider = r32(c, S2);
    let vt = Addr(rider.wrapping_add(bow_rider::IFACE)).lw(c);
    c.g[V1][0] = vt;
    c.g[S1][0] = c.g[V0][0];
    let iface = r32(c, S0);
    bow_virtual(c, iface, vt as u32, bow_iface::GET_VELOCITY, 0x0010_5d14)?;
    let rider = r32(c, S2);
    c.f[12] = Addr(rider.wrapping_add(bow_rider::AIR_PRIME_ARG))
        .read_fpu(c)
        .0;
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::AIR_PREDICTOR)).lw(c);
    c.g[A2][0] = c.g[V0][0];
    bow_callee::AIR_PREDICTOR_PRIME.call(c, 0x0010_5d28)?;
    Ok(())
}

/// The launch's trick interruption: bank the trick boost, interrupt the
/// trick, sub states 0xd then 5, play the bail animation at speed -1.
pub(crate) fn bow_restart_bail(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 27;
    c.f[12] = c.f[20];
    c.g[T0][0] = c.g[S1][0];
    c.g[A3][0] = 0;
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = 0;
    bow_callee::BANK_TRICK_BOOST.call(c, 0x0010_5c98)?;
    let rider = r32(c, S2);
    c.g[V0][0] = Addr(rider.wrapping_add(bow_rider::NATURAL_STANCE)).lw(c);
    c.g[A2][0] = 0;
    c.g[A1][0] = Addr(rider.wrapping_add(bow_rider::STANCE)).lw(c);
    c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::METRIX)).lw(c);
    let switched = c.g[A1][0] ^ c.g[V0][0];
    c.g[A1][0] = (switched != 0) as u64;
    bow_callee::INTERRUPT_TRICK.call(c, 0x0010_5cb4)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 0xd;
    bow_callee::SET_SUB_STATE.call(c, 0x0010_5cc0)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 5;
    bow_callee::SET_SUB_STATE.call(c, 0x0010_5ccc)?;
    let rider = r32(c, S2);
    c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::ANIM)).lw(c);
    c.g[A1][0] = BOW_BAIL_ANIM;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.f[12] = 0xbf80_0000;
    c.g[A2][0] = 1;
    call_ptr(c, bow_table::ANIM_PLAY, 0x0010_5ce4)
}

/// `void Rider_BounceOffWall(Rider* rider, CollObj* obj, void* info,
/// WallHit* hit, float depth)` (0x1057b8): the rider's response to a wall
/// hit. Nothing in sub state 9; a surface without bounce only plays the
/// bump animation. Otherwise the closing speed `vn` gives either a push out
/// of a solid wall (rebound `vn + clamp(grip * vn)`) or a soft rebound;
/// slow hits feed the impact accumulator; in main state 1 with a cruising
/// animation and the board facing the wall, the rider launches off it into
/// the air; else the rebound is added along the normal and
/// `Rider_HandleImpact` runs.
pub fn rider_bounce_off_wall(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save_fpr(c, 23, 0xa8);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 22, 0xa0);
        frame.save_fpr(c, 21, 0x98);
        frame.save_fpr(c, 20, 0x90);
        c.g[S2][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A2][0];
        c.f[23] = c.f[12];
        bow_callee::GET_SUB_STATE.call(c, 0x0010_57f8)?;
        let sub_state = c.g[V0][0];
        c.g[V1][0] = 0xc;
        if sub_state == 9 {
            c.cycles += 1;
            bow_epilogue(c, frame);
            return Ok(());
        }

        let entry = bow_surface_grip(c);
        c.cycles += 5;
        let bounce = Addr(entry).read_fpu(c);
        bow_set_f(c, 20, bounce);
        let no_bounce = bounce.eq_s(FpuF32::ZERO);
        abi::set_fcc(c, no_bounce);
        c.g[A0][0] = c.g[S2][0];
        if no_bounce {
            c.cycles += 5;
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = 1;
            bow_callee::PLAY_BUMP_ANIM.call(c, 0x0010_585c)?;
            bow_epilogue(c, frame);
            return Ok(());
        }

        // The closing speed vn (f21), 0 when moving away from the wall.
        c.cycles += 19;
        bow_closing_speed(c, 0x10, 0x00);
        c.mtc1(V1, 0);
        let toward = FpuF32::ZERO.le(bow_f(c, 0));
        abi::set_fcc(c, toward);
        c.f[21] = 0;
        if toward {
            c.cycles += 17;
            bow_closing_speed(c, 0x20, 0x10);
            c.mtc1(V1, 21);
        } else {
            c.cycles += 1;
        }
        let obj = r32(c, S0);
        c.g[A1][0] = Addr(obj.wrapping_add(bow_obj::HANDLE)).lw(c);

        // Solid (no handle, or its slot +0x70 says so) or soft.
        c.cycles += 2;
        let handle = c.g[A1][0] as u32;
        let solid = if c.g[A1][0] == 0 {
            true
        } else {
            c.cycles += 7;
            let vt = Addr(handle.wrapping_add(0xc)).lw(c);
            c.g[V1][0] = vt;
            bow_virtual(c, handle, vt as u32, bow_obj::HANDLE_PUSH_SLOT, 0x0010_5914)?;
            c.g[V0][0] != 0
        };
        if solid {
            bow_push_out(c)?;
        } else {
            bow_soft_rebound(c);
        }
        let rider = r32(c, S2);
        c.lq(V0, rider.wrapping_add(bow_rider::FACING));

        // facing = board direction . n (f22).
        c.cycles += 16;
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = 1;
        c.sq(V0, sp(c));
        c.lqc2(3, sp(c));
        let hit = r32(c, S1);
        c.lqc2(5, hit.wrapping_add(bow_hit::NORMAL));
        bow_dot4(c, 3, 5);
        c.qmfc2(V1, 4);
        c.mtc1(V1, 22);
        bow_callee::GET_MAIN_STATE.call(c, 0x0010_5a28)?;
        let slow_hit_state = if c.g[V0][0] == c.g[S0][0] {
            true
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            bow_callee::GET_MAIN_STATE.call(c, 0x0010_5a38)?;
            c.g[V1][0] = 2;
            if c.g[V0][0] != 2 {
                false
            } else {
                c.cycles += 4;
                let rider = r32(c, S2);
                let handler = Addr(rider.wrapping_add(bow_rider::HANDLER)).lw(c);
                c.g[V0][0] = handler;
                c.g[V1][0] = Addr((handler as u32).wrapping_add(0x30)).lw(c);
                c.g[V1][0] == c.g[S0][0]
            }
        };
        if slow_hit_state {
            bow_accumulate_impact(c);
        } else {
            let rider = r32(c, S2);
            c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::ANIM)).lw(c);
        }

        // The launch test: main state 1, animation 9/2/0xb/1, facing > 0.3.
        c.cycles += 9;
        c.g[A1][0] = 2;
        call_ptr(c, bow_table::ANIM_CURRENT, 0x0010_5ad0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        bow_callee::GET_MAIN_STATE.call(c, 0x0010_5adc)?;
        let state = c.g[V0][0];
        c.g[V1][0] = state;
        c.g[V0][0] = 9;
        let mut launch = false;
        if state == 1 {
            let anim = c.g[S0][0];
            c.cycles += 2;
            c.g[V0][0] = 2;
            let cruising = anim == 9 || {
                c.cycles += 2;
                c.g[V0][0] = 0xb;
                anim == 2 || {
                    c.cycles += 2;
                    anim == 0xb || {
                        c.cycles += 2;
                        anim == state
                    }
                }
            };
            if cruising {
                c.cycles += 4;
                let threshold = bow_lit(c, bow_lit::LAUNCH_FACING);
                bow_set_f(c, 0, threshold);
                let facing = bow_f(c, 22);
                launch = abi::c_lt_s(c, threshold, facing);
            }
        }
        if launch {
            bow_launch(c)?;
            bow_epilogue(c, frame);
            return Ok(());
        }

        // vel += n * rebound; then the impact.
        c.cycles += 15;
        let hit = r32(c, S1);
        c.lqc2(4, hit.wrapping_add(bow_hit::NORMAL));
        bow_scale_vf4(c, V0, 20);
        c.sqc2(5, sp(c).wrapping_add(0x10));
        c.g[A0][0] = c.g[S2][0];
        c.lq(V0, sp(c).wrapping_add(0x10));
        c.g[A1][0] = c.g[S3][0];
        c.sq(V0, sp(c));
        let rider = r32(c, S2);
        c.lqc2(3, rider.wrapping_add(bow_rider::VEL));
        c.lqc2(4, sp(c));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, rider.wrapping_add(bow_rider::VEL));
        c.g[A2][0] = Addr(hit.wrapping_add(bow_hit::MATERIAL)).lw(c);
        bow_callee::HANDLE_IMPACT.call(c, 0x0010_5d6c)?;
        c.cycles += 1;
        bow_epilogue(c, frame);
        Ok(())
    })
}

/// `.lit4` constants of `Input_QuantizeStickDirection` (gp-relative):
/// the angles for a vertical stick (y < 0, y > 0), the atan quadrant
/// corrections (added for y > 0, subtracted otherwise) and the final
/// snap-to-zero threshold.
pub(crate) const QUANTIZE_STICK_DOWN_ANGLE: u32 = 0x7b68;

pub(crate) const QUANTIZE_STICK_UP_ANGLE: u32 = 0x7b64;

pub(crate) const QUANTIZE_STICK_ADD_PI: u32 = 0x7b60;

pub(crate) const QUANTIZE_STICK_SUB_PI: u32 = 0x7b5c;

pub(crate) const QUANTIZE_STICK_SNAP: u32 = 0x7b58;

/// `Math_Atan(f12)` and `Math_SinCos(sin*, cos*, f12)`.
pub(crate) const QUANTIZE_STICK_ATAN: u32 = 0x0031_c228;

pub(crate) const QUANTIZE_STICK_SINCOS: u32 = 0x0031_be50;

/// A `.lit4` float at `gp - off`.
pub(crate) fn quantize_stick_lit(c: &mut Cpu, off: u32) -> FpuF32 {
    Addr((c.g[GP][0] as u32).wrapping_sub(off)).read_fpu(c)
}

/// Zero `*p` when `|*p| <= eps` (`le`) or `|*p| < eps` (`!le`), leaving
/// `|*p|` in f0 and the compare in the FPU condition bit.
pub(crate) fn quantize_stick_snap(c: &mut Cpu, p: u32, eps: FpuF32, le: bool) {
    let v = Addr(p).read_fpu(c).abs();
    c.f[0] = v.0;
    let small = if le { v.le(eps) } else { v.lt(eps) };
    crate::abi::set_fcc(c, small);
    if small {
        Addr(p).sw(c, 0);
    }
}

/// `void Input_QuantizeStickDirection(Rider*, float* y, float* x, float
/// step, float eps)` (0x1158b8): snap the stick vector (x, y) to the
/// nearest multiple of the angle `step`, keeping its length (clamped to
/// [-1, 1] per axis); components within `eps` count as zero, and results
/// below the snap threshold become zero.
pub fn input_quantize_stick_direction(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save_fpr(c, 24, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.f[24] = c.f[12];
        frame.save_fpr(c, 23, 0x58);
        frame.save_fpr(c, 22, 0x50);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        let x_ptr = c.g[S0][0] as u32;
        let y_ptr = c.g[S1][0] as u32;
        let eps = FpuF32(c.f[13]);
        quantize_stick_snap(c, x_ptr, eps, true);
        c.cycles += 6;
        quantize_stick_snap(c, y_ptr, eps, true);

        c.cycles += 11;
        let x = Addr(x_ptr).read_fpu(c);
        let y = Addr(y_ptr).read_fpu(c);
        c.f[21] = x.0;
        c.f[20] = y.0;
        let xx = x * x;
        let yy = y * y;
        c.f[1] = yy.0;
        let zero = FpuF32::ZERO;
        c.f[22] = zero.0;
        c.f[0] = (xx + yy).0;
        let len = (xx + yy).sqrt();
        c.f[23] = len.0;
        let still = len.eq_s(zero);
        crate::abi::set_fcc(c, still);
        if !still {
            // The stick angle (atan2(y, x)) in f1.
            c.cycles += 3;
            let vertical = x.eq_s(zero);
            crate::abi::set_fcc(c, vertical);
            let angle;
            if vertical {
                c.cycles += 4;
                let y_zero = y.eq_s(zero);
                crate::abi::set_fcc(c, y_zero);
                if y_zero {
                    angle = y;
                } else {
                    c.cycles += 4;
                    let up = zero.le(y);
                    crate::abi::set_fcc(c, up);
                    let down = quantize_stick_lit(c, QUANTIZE_STICK_DOWN_ANGLE);
                    if up {
                        c.cycles += 3;
                        angle = quantize_stick_lit(c, QUANTIZE_STICK_UP_ANGLE);
                    } else {
                        angle = down;
                    }
                }
            } else {
                c.cycles += 7;
                c.f[12] = y.div(x).0;
                call_ptr(c, QUANTIZE_STICK_ATAN, 0x0011_5988)?;
                let x = FpuF32(c.f[21]);
                let y = FpuF32(c.f[20]);
                let zero = FpuF32(c.f[22]);
                let left = x.lt(zero);
                crate::abi::set_fcc(c, left);
                let a = FpuF32(c.f[0]);
                if !left {
                    angle = a;
                } else {
                    c.cycles += 3;
                    let up = zero.lt(y);
                    crate::abi::set_fcc(c, up);
                    if up {
                        c.cycles += 3;
                        let k = quantize_stick_lit(c, QUANTIZE_STICK_ADD_PI);
                        c.f[0] = k.0;
                        angle = a + k;
                    } else {
                        c.cycles += 2;
                        let k = quantize_stick_lit(c, QUANTIZE_STICK_SUB_PI);
                        c.f[0] = k.0;
                        angle = a - k;
                    }
                }
            }
            c.f[1] = angle.0;

            // Round angle / step to the nearest integer (half away from 0).
            c.cycles += 7;
            let step = FpuF32(c.f[24]);
            let steps = angle.div(step);
            c.f[2] = steps.0;
            let negative = angle.lt(FpuF32::ZERO);
            crate::abi::set_fcc(c, negative);
            let half: u32 = if negative {
                c.cycles += 3;
                0xbf00_0000
            } else {
                0x3f00_0000
            };
            c.g[AT][0] = sx(half);
            let rounded = steps + FpuF32(half);
            c.f[0] = rounded.0;

            c.cycles += 18;
            let n = rounded.to_i32();
            c.f[1] = n as u32;
            c.g[V0][0] = sx(n as u32);
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[SP][0] | 0x4;
            c.f[12] = (FpuF32::from_i32(n) * step).0;
            call_ptr(c, QUANTIZE_STICK_SINCOS, 0x0011_5a00)?;

            // x = clamp(cos * len), y = clamp(sin * len).
            let len = FpuF32(c.f[23]);
            let cos = Addr(sp(c).wrapping_add(4)).read_fpu(c);
            let lo = FpuF32(0xbf80_0000);
            let hi = FpuF32(0x3f80_0000);
            c.f[1] = lo.0;
            let new_x = cos * len;
            c.f[2] = hi.0;
            c.g[AT][0] = sx(hi.0);
            let above = lo.le(new_x);
            crate::abi::set_fcc(c, above);
            let mut clamped_x = lo;
            if above {
                c.cycles += 1;
                clamped_x = new_x.min(hi);
            }
            c.f[4] = clamped_x.0;

            c.cycles += 7;
            let sin = Addr(sp(c)).read_fpu(c);
            let new_y = sin * len;
            c.f[0] = new_y.0;
            let above = lo.le(new_y);
            crate::abi::set_fcc(c, above);
            Addr(c.g[S0][0] as u32).write_fpu(c, clamped_x);
            let mut clamped_y = lo;
            if above {
                c.cycles += 1;
                clamped_y = new_y.min(hi);
            }
            c.f[3] = clamped_y.0;

            c.cycles += 8;
            Addr(c.g[S1][0] as u32).write_fpu(c, clamped_y);
            let snap = quantize_stick_lit(c, QUANTIZE_STICK_SNAP);
            c.f[1] = snap.0;
            let x_ptr = c.g[S0][0] as u32;
            let y_ptr = c.g[S1][0] as u32;
            quantize_stick_snap(c, x_ptr, snap, false);
            c.cycles += 6;
            quantize_stick_snap(c, y_ptr, snap, false);
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 9;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 24, 0x60);
        frame.restore_fpr(c, 23, 0x58);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod sample_part_xf {}

pub(crate) mod set_self_object {

    /// gp offsets: the static defaults `{object id, flag byte}`, their
    /// "initialised" flag, the declared argument types, the object manager
    /// and the world.
    pub const GP_DEFAULTS: u32 = 0x2678;

    pub const GP_DEFAULTS_FLAG: u32 = 0x267c;

    pub const GP_DEFAULTS_READY: u32 = 0x2680;

    pub const GP_ARG_TYPES: u32 = 0xbf8;

    pub const GP_OBJECT_MANAGER: u32 = 0x16c8;

    pub const GP_WORLD: u32 = 0xce8;

    /// The world's "self" object field.
    pub const WORLD_SELF: u32 = 0x290;

    /// The declared type that converts an int argument to float.
    pub const TYPE_FLOAT: u64 = 2;

    /// `EA_MemCopy`.
    pub const MEM_COPY: u32 = 0x003e_6574;

    /// The 12-byte "ok" result copied into the return register.
    pub const RESULT_OK: u32 = 0x004c_9098;
}

pub(crate) mod deflect_along_wall {

    /// Rider fields: the orientation quaternion, forward and up vectors.
    pub const ORIENTATION: u32 = 0x120;

    pub const FORWARD: u32 = 0x1b0;

    pub const UP: u32 = 0x1c0;

    /// The wall contact's normal (in the second argument).
    pub const CONTACT_NORMAL: u32 = 0x10;

    /// gp-relative constants: -pi/2 and pi/2 (atan2 with x = 0), +pi and
    /// -pi (quadrant fix), the deflection limit (110 degrees) and the
    /// largest step per call (20 degrees).
    pub const GP_MINUS_HALF_PI: u32 = 0x7eec;

    pub const GP_HALF_PI: u32 = 0x7ee8;

    pub const GP_PI_ADD: u32 = 0x7ee4;

    pub const GP_PI_SUB: u32 = 0x7ee0;

    pub const GP_LIMIT: u32 = 0x7edc;

    pub const GP_STEP: u32 = 0x7ed8;

    /// `float atanf(float)` and `void Math_SinCos(float, float* s, float* c)`.
    pub const ATANF: u32 = 0x0031_c228;

    pub const SIN_COS: u32 = 0x0031_be50;
}

pub(crate) mod sfqr {

    /// Stream file: its mutex, read state (0 idle), the "urgent" flag and
    /// the two priorities `StartNextRequest` takes.
    pub const MUTEX: u32 = 0x4;

    pub const READ_STATE: u32 = 0x38;

    pub const PRIORITY: u32 = 0x3c;

    pub const URGENT_PRIORITY: u32 = 0x40;

    pub const URGENT: u32 = 0x48;

    /// Request: kind (0: a file range), name (255 chars), offset, size.
    pub const REQ_KIND: u32 = 0x10;

    pub const REQ_NAME: u32 = 0x14;

    pub const REQ_OFFSET: u32 = 0x118;

    pub const REQ_SIZE: u32 = 0x11c;
}

pub(crate) mod capat {

    pub const TRICK: u32 = 0xe0c;

    pub const PLANNED: u32 = 0xe10;

    pub const SKIP: u32 = 0xe14;

    pub const DONE: u32 = 0xe18;

    pub const ARG_T0: u32 = 0xe1c;

    pub const ARG_A3: u32 = 0xe20;

    /// Spin / flip directions (+-1.0 or 0).
    pub const SPIN: u32 = 0xe38;

    pub const FLIP: u32 = 0xe3c;
}

/// `int PATH_RemapNode(int id, int node)` (0x3d3c90): remaps a path node
/// number through the remap ranges of path `id` in the path table
/// (`*0x44f420`): the byte at +4 of the path's word (`words[index[id]]`)
/// picks range k (`ranges[k-1]..ranges[k]` of `words`); each word of the
/// range is `{from << 16 | to}`, and every word whose `from` equals the
/// current node replaces it by `to`. Negative ids and k == 0 return `node`.
pub fn path_remap_node(c: &mut Cpu) {
    const PATH_TABLE: u32 = 0x0045_0000 - 0xbe0;
    const WORDS: u32 = 0x34;
    const INDEX: u32 = 0x38;
    const RANGES: u32 = 0x48;
    c.cycles += 2;
    c.g[A2][0] = c.g[A1][0];
    if c.g[A0][0] as i64 >= 0 {
        c.cycles += 12;
        let table = c.lw(PATH_TABLE);
        c.g[A1][0] = table;
        let index = c.lw((table as u32).wrapping_add(INDEX));
        let words = c.lw((table as u32).wrapping_add(WORDS));
        c.g[A3][0] = words;
        let entry = sx(((c.g[A0][0] as u32) << 1).wrapping_add(index as u32));
        c.g[A0][0] = entry;
        let word = c.lh(entry as u32);
        let word_addr = ((word as u32) << 2).wrapping_add(words as u32);
        let k = c.lbu(word_addr.wrapping_add(4));
        let k4 = sx((k as u32) << 2);
        c.g[V0][0] = k4;
        if k == 0 {
            c.g[V1][0] = index;
        } else {
            c.cycles += 8;
            let ranges = c.lw((table as u32).wrapping_add(RANGES));
            let range = (k4 as u32).wrapping_add(ranges as u32);
            c.g[V0][0] = sx(range);
            let start = c.lw(range.wrapping_sub(4));
            let end = c.lw(range);
            c.g[A0][0] = start;
            c.g[A1][0] = sx((start as u32) << 2);
            let n = sx((end as u32).wrapping_sub(start as u32));
            c.g[V0][0] = n;
            let first = sx((words as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = first;
            if n as i64 > 0 {
                c.cycles += 2;
                c.g[A1][0] = first;
                c.g[A0][0] = n;
                loop {
                    c.cycles += 8;
                    let w = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                    let from = sx(((w as u32) as i32 >> 16) as u32);
                    let to = w & 0xffff;
                    c.g[V1][0] = to;
                    c.g[V0][0] = c.g[A2][0] ^ from;
                    if c.g[V0][0] == 0 {
                        c.g[A2][0] = to;
                    }
                    if c.g[A0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_3cf8);
                    }
                }
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    ret(c);
}

/// `LunoVMRegister* WScriptFn_SetSelfObject(LunoVMRegister* ret, int n,
/// Arg* args)` (0x302968): script function. The object id starts from the
/// default (-1) and takes each passed argument `{idx, val, _, type}` (an int
/// passed for a float slot is converted). The id (class in the low byte,
/// handle in the rest) is resolved through the object manager's class
/// table; the world's self object (+0x290) becomes the handle word >> 8,
/// times 4, or 0 when the class or handle is missing. Returns the 12-byte
/// "ok" result in `*ret`. Frame 0x70 (s0-s3; the arguments at +0x10).
pub fn wscript_fn_set_self_object(c: &mut Cpu) {
    use set_self_object::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        let gp = c.g[GP][0] as u32;
        let ready = c.lw(gp.wrapping_add(GP_DEFAULTS_READY));
        c.g[V0][0] = ready;
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        if ready == 0 {
            // First use: default id -1 (the flag byte kept, its copy at sp).
            c.cycles += 7;
            let flag = c.lbu(gp.wrapping_add(GP_DEFAULTS_FLAG));
            c.g[V0][0] = flag;
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            c.sw(gp.wrapping_add(GP_DEFAULTS), u64::MAX);
            c.sw(gp.wrapping_add(GP_DEFAULTS_READY), 1);
            c.sb(sp(c), flag);
        }

        // buf (sp+0x10) = defaults.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let buf = sp(c).wrapping_add(0x10);
        c.g[S0][0] = sx(buf);
        c.g[A2][0] = 0x8;
        c.g[A0][0] = sx(buf);
        c.g[A1][0] = sx(gp.wrapping_add(GP_DEFAULTS));
        c.sw(buf, u64::MAX);
        call_ptr(c, MEM_COPY, 0x0030_29d0)?;
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S1][0] as i64) > 0 {
            // Each passed argument into its slot.
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(GP_ARG_TYPES));
            c.g[A3][0] = TYPE_FLOAT;
            loop {
                c.cycles += 8;
                let arg = c.g[A2][0] as u32;
                let idx = c.lw(arg);
                let ty = c.lw(arg.wrapping_add(0xc));
                c.g[V1][0] = ty;
                let off = (idx as u32) << 2;
                let slot = (c.g[S0][0] as u32).wrapping_add(off);
                c.g[A0][0] = sx(slot);
                let declared = c.lw((c.g[T0][0] as u32).wrapping_add(off));
                c.g[V0][0] = declared;
                if ty != declared && declared == c.g[A3][0] {
                    c.cycles += 6;
                    c.lwc1(0, arg.wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, slot);
                } else {
                    if ty != declared {
                        c.cycles += 2;
                    }
                    c.cycles += 1;
                    let val = c.lw(arg.wrapping_add(0x4));
                    c.g[V0][0] = val;
                    c.sw(slot, val);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx(arg.wrapping_add(0x10));
                if c.g[A1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_29e8);
                }
            }
        }

        // The object: class table[id & 0xff], handle table[id >> 8] >> 8,
        // times 4 (0 when either is missing).
        c.cycles += 10;
        let gp = c.g[GP][0] as u32;
        let manager = c.lw(gp.wrapping_add(GP_OBJECT_MANAGER));
        let id = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = id;
        let m = c.lw(manager as u32);
        c.g[A0][0] = m;
        let classes = c.lw((m as u32).wrapping_add(0x8));
        let entry = ((id as u32 & 0xff) << 2).wrapping_add(classes as u32);
        c.g[V0][0] = sx(entry);
        let class = c.lw(entry);
        c.g[V1][0] = class;
        c.g[A2][0] = c.lw(gp.wrapping_add(GP_WORLD));
        let mut object = 0u64;
        if class != 0 {
            c.cycles += 8;
            let handles = c.lw((class as u32).wrapping_add(0x1c));
            c.g[A0][0] = handles;
            let word = c.lw(((id as u32) >> 8 << 2).wrapping_add(handles as u32));
            c.g[V1][0] = word;
            let handle = (word as u32) >> 8;
            object = sx(handle << 2);
            if handle == 0 {
                c.cycles += 1;
                object = 0;
            }
        } else {
            c.cycles += 1;
        }
        c.g[V0][0] = object;

        // world->self = object; *ret = the ok result.
        c.cycles += 17;
        c.sw((c.g[A2][0] as u32).wrapping_add(WORLD_SELF), object);
        c.g[V1][0] = 0x004d_0000;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(RESULT_OK);
        c.ldl(A0, RESULT_OK.wrapping_add(0x7));
        c.ldr(A0, RESULT_OK);
        c.g[A1][0] = c.lw(RESULT_OK.wrapping_add(0x8));
        let ret_reg = c.g[S3][0] as u32;
        c.sdl(A0, ret_reg.wrapping_add(0x7));
        c.sdr(A0, ret_reg);
        c.sw(ret_reg.wrapping_add(0x8), c.g[A1][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `vf6 = vf4 x vf5` (the outer-product pair; vf6.x is then zeroed by
/// `vsub.x vf6, vf6, vf6`), as the original's macro-mode sequence.
pub(crate) fn deflect_cross(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::Src;
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
}

/// `vf4.w = dot3(vf3, vf<b>)` through `vmul` / `vadda` / `vmadda` / `vmadd`
/// (vf6.w = vf0.w first), as the original does.
pub(crate) fn deflect_dot(c: &mut Cpu, b: u8) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `void Rider::DeflectAlongWall(WallContact* contact)` (0x1065b0): with the
/// wall normal `n = norm(up x contact.normal)` and `t = up x n`, the heading
/// angle relative to the wall is `atan2(fwd . n, fwd . t)`. A positive angle
/// is pushed towards +110 degrees, a negative one towards -110 degrees, by
/// at most 20 degrees per call; when that turn `d` is non-zero the rider's
/// orientation (+0x120) is rotated by `-d` about up (a quaternion from
/// `Math_SinCos(-d / 2)`, multiplied on the left). Frame 0xc0 (s0, f20-f22).
pub fn rider_deflect_along_wall(c: &mut Cpu) {
    use deflect_along_wall::*;
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    guest(c, |c| {
        c.cycles += 63;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0x90);
        frame.save_fpr(c, 22, 0xb0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x80);
        frame.save_fpr(c, 21, 0xa8);
        frame.save_fpr(c, 20, 0xa0);
        c.mtc1(0, 22); // 0.0
        let rider = c.g[S0][0] as u32;
        let s = sp(c);
        // n = norm(up x contact.normal) at sp+0x0 (via sp+0x30/+0x10).
        c.lq(V0, rider.wrapping_add(UP));
        c.sq(V0, s.wrapping_add(0x20));
        c.lqc2(4, s.wrapping_add(0x20));
        c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(CONTACT_NORMAL));
        deflect_cross(c);
        c.sqc2(6, s.wrapping_add(0x30));
        c.lq(V0, s.wrapping_add(0x30));
        c.sq(V0, s.wrapping_add(0x10));
        c.lqc2(3, s.wrapping_add(0x10));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, s.wrapping_add(0x30));
        // t = up x n (sp+0x10).
        c.lq(V1, rider.wrapping_add(UP));
        c.lq(V0, s.wrapping_add(0x30));
        c.sq(V1, s.wrapping_add(0x20));
        c.sq(V0, s);
        c.lqc2(4, s.wrapping_add(0x20));
        c.lqc2(5, s);
        deflect_cross(c);
        c.sqc2(6, s.wrapping_add(0x30));
        // x = fwd . t (f20), y = fwd . n (f21).
        c.lq(V1, rider.wrapping_add(FORWARD));
        c.lq(V0, s.wrapping_add(0x30));
        c.sq(V1, s.wrapping_add(0x20));
        c.sq(V0, s.wrapping_add(0x10));
        c.lqc2(3, s.wrapping_add(0x20));
        c.lqc2(5, s.wrapping_add(0x10));
        deflect_dot(c, 5);
        c.qmfc2(A0, 4);
        c.mtc1(A0, 20);
        c.lqc2(3, s.wrapping_add(0x20));
        c.lqc2(5, s);
        deflect_dot(c, 5);
        c.qmfc2(V0, 4);
        c.mtc1(V0, 21);

        // angle (f12) = atan2(y, x).
        let gp = c.g[GP][0] as u32;
        c.c_eq_s(20, 22);
        if c.fcc() {
            // x == 0: 0, -pi/2 or pi/2 by y.
            c.cycles += 4;
            c.c_eq_s(21, 22);
            let y_zero = c.fcc();
            c.mov_s(12, 21);
            if !y_zero {
                c.cycles += 4;
                c.c_le_s(22, 21);
                c.lwc1(12, gp.wrapping_sub(GP_MINUS_HALF_PI));
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(12, gp.wrapping_sub(GP_HALF_PI));
                }
            }
        } else {
            c.cycles += 7;
            c.div_s(12, 21, 20);
            call_ptr(c, ATANF, 0x0010_66e4)?;
            c.c_lt_s(20, 22);
            let x_negative = c.fcc();
            c.mov_s(12, 0);
            if x_negative {
                c.cycles += 3;
                let gp = c.g[GP][0] as u32;
                c.c_lt_s(22, 21);
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(0, gp.wrapping_sub(GP_PI_ADD));
                    c.add_s(12, 12, 0);
                } else {
                    c.cycles += 2;
                    c.lwc1(0, gp.wrapping_sub(GP_PI_SUB));
                    c.sub_s(12, 12, 0);
                }
            }
        }

        // d (f3) = clamp(limit - angle, 0, step) for a positive angle, else
        // clamp(-limit - angle, -step, 0).
        c.cycles += 6;
        let gp = c.g[GP][0] as u32;
        c.mtc1(0, 2);
        c.lwc1(0, gp.wrapping_sub(GP_LIMIT));
        c.c_lt_s(2, 12);
        c.lwc1(1, gp.wrapping_sub(GP_STEP));
        if c.fcc() {
            c.cycles += 5;
            c.sub_s(0, 0, 12);
            c.c_le_s(2, 0);
            c.mov_s(3, 2);
            if c.fcc() {
                c.cycles += 2;
                c.min_s(3, 0, 1);
            }
        } else {
            c.neg_s(0, 0);
            c.cycles += 6;
            c.neg_s(1, 1);
            c.sub_s(0, 0, 12);
            c.c_le_s(1, 0);
            c.mov_s(3, 1);
            if c.fcc() {
                c.cycles += 1;
                c.min_s(3, 0, 2);
            }
        }

        c.cycles += 6;
        c.mov_s(1, 3);
        c.mtc1(0, 0);
        c.c_eq_s(1, 0);
        if !c.fcc() {
            // Rotate the orientation by -d about up.
            c.cycles += 37;
            c.neg_s(0, 1);
            c.g[AT][0] = 0x3f00_0000; // 0.5f
            c.mtc1(AT, 12);
            let rider = c.g[S0][0] as u32;
            let s = sp(c);
            c.lq(V0, rider.wrapping_add(UP));
            c.g[A0][0] = sx(s.wrapping_add(0x70));
            c.g[A1][0] = sx(s.wrapping_add(0x74));
            c.mul_s(12, 0, 12);
            c.sq(V0, s.wrapping_add(0x20));
            call_ptr(c, SIN_COS, 0x0010_67a0)?;
            let s = sp(c);
            // q = (up * sin, cos) at sp+0x50.
            c.lwc1(0, s.wrapping_add(0x70));
            c.lwc1(1, s.wrapping_add(0x20));
            c.lwc1(2, s.wrapping_add(0x24));
            c.lwc1(4, s.wrapping_add(0x28));
            c.mul_s(1, 0, 1);
            c.mul_s(2, 0, 2);
            c.lwc1(3, s.wrapping_add(0x74));
            c.mul_s(0, 0, 4);
            c.swc1(3, s.wrapping_add(0x5c));
            c.swc1(1, s.wrapping_add(0x50));
            c.swc1(2, s.wrapping_add(0x54));
            c.swc1(0, s.wrapping_add(0x58));
            // orientation = orientation * q (VU0 quaternion product).
            let rider = c.g[S0][0] as u32;
            c.lqc2(4, rider.wrapping_add(ORIENTATION));
            c.lqc2(5, s.wrapping_add(0x50));
            c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().mula(0b1110, 4, Src::Bc(5, W));
            c.vu0().madda(0b1110, 5, Src::Bc(4, W));
            c.vu0().suba(0b0001, 7, Src::Bc(7, X));
            c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
            c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
            c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
            c.sqc2(8, s.wrapping_add(0x60));
            c.lq(V0, s.wrapping_add(0x60));
            c.sq(V0, rider.wrapping_add(ORIENTATION));
            c.sq(V0, s.wrapping_add(0x40));
        }
        frame.restore(c, S0, 0x90);
        c.cycles += 6;
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 22, 0xb0);
        frame.restore_fpr(c, 21, 0xa8);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const SFQR_VALIDATE: Callee = Callee {
    addr: 0x003d_fbd0,
    f: ee::s14::StreamFile_Validate,
};

pub(crate) const SFQR_ALLOC_REQUEST: Callee = Callee {
    addr: 0x003d_fcd8,
    f: ee::s14::StreamFile_AllocRequest,
};

pub(crate) const SFQR_ENQUEUE_REQUEST: Callee = Callee {
    addr: 0x003d_fd58,
    f: ee::s14::StreamFile_EnqueueRequest,
};

pub(crate) const SFQR_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};

pub(crate) const SFQR_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

pub(crate) const SFQR_START_NEXT_REQUEST: Callee = Callee {
    addr: 0x003e_0270,
    f: ee::s14::StreamFile_StartNextRequest,
};

/// `int StreamFile_QueueRange(StreamFile* file, const char* name, int
/// offset, int size)` (0x3e0c88): queue a read of `size` bytes at `offset`
/// of file `name` on a valid ("STRM") stream file. Takes a free request
/// (0 when none or the file is invalid), fills it, appends it to the queue
/// and, if the stream was idle, marks it busy and starts the next request
/// (at the urgent priority when the urgent flag is set). Returns the
/// request's id.
pub fn stream_file_queue_range(c: &mut Cpu) {
    let frame = Frame { size: 0x60 };
    guest(c, |c| {
        c.cycles += 14;
        Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0x10);
        // Validate(file, &stream (sp+0), &file (sp+4)).
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[SP][0] | 4;
        SFQR_VALIDATE.call(c, 0x003e_0cb8)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            sfqr_queue(c)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The stream (the validated header) saved at `sp+0`.
pub(crate) fn sfqr_stream(c: &mut Cpu) -> u64 {
    Addr(sp(c)).lw(c)
}

/// [`stream_file_queue_range`] after validation: the request, the queue
/// and the idle-stream start; leaves the return value in v0.
pub(crate) fn sfqr_queue(c: &mut Cpu) -> crate::call::Flow {
    use sfqr::*;
    c.cycles += 5;
    c.g[A0][0] = sfqr_stream(c);
    SFQR_ALLOC_REQUEST.call(c, 0x003e_0cc8)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        return Ok(());
    }
    let req = r32(c, S0);
    Addr(req).off(REQ_KIND).sw(c, 0);
    c.cycles += 16;
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = 0xff;
    c.g[A0][0] = sx(req.wrapping_add(REQ_NAME));
    call_ptr(c, 0x0041_6ce0, 0x003e_0cec)?; // strncpy(req->name, name, 255)
    let req = r32(c, S0);
    let offset = c.g[S2][0];
    Addr(req).off(REQ_OFFSET).sw(c, offset);
    c.g[A1][0] = c.g[S0][0];
    let size = c.g[S3][0];
    Addr(req).off(REQ_SIZE).sw(c, size);
    c.g[A0][0] = sfqr_stream(c);
    SFQR_ENQUEUE_REQUEST.call(c, 0x003e_0d00)?;
    c.g[A0][0] = sx((sfqr_stream(c) as u32).wrapping_add(MUTEX));
    SFQR_MUTEX_LOCK.call(c, 0x003e_0d0c)?;
    // Was the stream idle? Then mark it busy.
    c.g[V1][0] = sfqr_stream(c);
    let stream = r32(c, V1);
    c.g[S1][0] = Addr(stream).off(READ_STATE).lw(c);
    if c.g[S1][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = 1;
        Addr(stream).off(READ_STATE).sw(c, 1);
    }
    c.g[A0][0] = sfqr_stream(c);
    c.cycles += 4;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(MUTEX));
    SFQR_MUTEX_UNLOCK.call(c, 0x003e_0d30)?;
    if c.g[S1][0] == 0 {
        c.cycles += 4;
        c.g[A0][0] = sfqr_stream(c);
        let stream = r32(c, A0);
        c.g[V0][0] = Addr(stream).off(URGENT).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A1][0] = Addr(stream).off(URGENT_PRIORITY).lw(c);
            SFQR_START_NEXT_REQUEST.call(c, 0x003e_0d50)?;
        } else {
            c.cycles += 3;
            c.g[A1][0] = Addr(stream).off(PRIORITY).lw(c);
            SFQR_START_NEXT_REQUEST.call(c, 0x003e_0d60)?;
        }
    }
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    Ok(())
}

pub(crate) const CAPAT_ROLL_SKIP_TRICKS: Callee = Callee {
    addr: 0x0010_c1d0,
    f: ee::s00::ComputerActorRider_RollSkipTricks,
};

/// Store `1.0` (spin / flip wanted) or `0` at `off` of the rider in s0.
pub(crate) fn capat_axis(c: &mut Cpu, want: bool, off: u32) {
    c.cycles += 2;
    let at = r32(c, S0).wrapping_add(off);
    if !want {
        c.sw(at, 0);
    } else {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.swc1(0, at);
    }
}

/// `void ComputerActorRider_PlanAirTricks(ComputerActorRider* self, int
/// spin, int flip, int a3, int t0)` (0x10bfa8): reset the planned trick,
/// plan one if a spin or a flip is wanted, roll whether to skip tricks,
/// store the two parameters, and give the wanted axes a random direction
/// (+-1.0).
pub fn computer_actor_rider_plan_air_tricks(c: &mut Cpu) {
    use capat::*;
    let frame = Frame { size: 0x60 };
    guest(c, |c| {
        c.cycles += 17;
        Frame::open(c, 0x60);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        let this = r32(c, S0);
        c.g[V1][0] = sx(this.wrapping_add(PLANNED));
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A2][0];
        c.sw(this.wrapping_add(TRICK), u64::MAX);
        c.g[S3][0] = c.g[T0][0];
        c.g[V0][0] = 0;
        // planned = spin || flip
        let mut planned = true;
        if c.g[S1][0] == 0 {
            c.cycles += 2;
            planned = c.g[S4][0] != 0;
        }
        if planned {
            c.cycles += 2;
            c.g[V0][0] = 1;
        }
        let v = c.g[V0][0];
        c.sw(r32(c, V1), v);
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        CAPAT_ROLL_SKIP_TRICKS.call(c, 0x0010_c004)?;
        let this = r32(c, S0);
        c.g[V1][0] = c.lw(this.wrapping_add(PLANNED));
        let skip = c.g[V0][0];
        c.sw(this.wrapping_add(SKIP), skip);
        let a3 = c.g[S2][0];
        c.sw(this.wrapping_add(ARG_A3), a3);
        let t0 = c.g[S3][0];
        c.sw(this.wrapping_add(ARG_T0), t0);
        c.sw(this.wrapping_add(DONE), 0);
        let mut late = true;
        if c.g[V1][0] != 0 {
            let spin = c.g[S1][0] != 0;
            capat_axis(c, spin, SPIN);
            let flip = c.g[S4][0] != 0;
            capat_axis(c, flip, FLIP);
            // Random signs (AIrand() & 1 negates).
            for (i, (off, ret_to)) in [(SPIN, 0x0010_c04cu32), (FLIP, 0x0010_c06c)]
                .into_iter()
                .enumerate()
            {
                c.cycles += 5;
                call_ptr(c, 0x0031_7810, ret_to)?; // Rand_NextGame
                c.g[V0][0] &= 1;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    let at = r32(c, S0).wrapping_add(off);
                    c.lwc1(0, at);
                    c.neg_s(0, 0);
                    c.swc1(0, at);
                } else if i == 1 {
                    late = false;
                }
            }
        }
        if late {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x50);
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObjFactory_AddDynamicParticleData (0x2fee98).

/// The default particle record (0xf0 bytes, a function-local static) and
/// its init guard (`gp+0x253c`).
pub(crate) const DPD_TEMPLATE: u32 = 0x004f_b938;

pub(crate) const DPD_GUARD: u32 = 0x253c;

/// Record fields: `+0x14`/`+0x1c` a range whose top is clamped to its
/// bottom, `+0xd8` the target object handle (-1: the world script
/// manager's default object), `+0xdc` the modifier slot, `+0xe0..` a vector.
pub(crate) const DPD_RANGE_LO: u32 = 0x14;

pub(crate) const DPD_RANGE_HI: u32 = 0x1c;

pub(crate) const DPD_HANDLE: u32 = 0xd8;

pub(crate) const DPD_SLOT: u32 = 0xdc;

pub(crate) const DPD_VEC: u32 = 0xe0;

/// Per-field value types (2: float) of the record (`0x446380`).
pub(crate) const DPD_FIELD_TYPES: u32 = 0x0044_6380;

/// The allocation name. (The returned 12-byte value is at `0x4c9098`,
/// addressed from `s5 = 0x4d0000`.)
pub(crate) const DPD_ALLOC_NAME: u32 = 0x0048_9750;

/// The object handle table (`gp+0x16c8`) and the world script manager
/// (`gp+0xce8`, default object at `+0x290`).
pub(crate) const DPD_HANDLES: u32 = 0x16c8;

pub(crate) const DPD_SCRIPT_MAN: u32 = 0xce8;

pub(crate) const DPD_ALLOC: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};

pub(crate) const DPD_DELETE: Callee = Callee {
    addr: 0x0031_7e50,
    f: ee::s10::__builtin_delete,
};

/// `EA_MemCopy(dst, src, n)` and `ModObj_AddDynamicParticleModifier(obj,
/// record, slot, &vec)` (through the table).
pub(crate) const DPD_MEM_COPY: u32 = 0x003e_6574;

pub(crate) const DPD_ADD_MODIFIER: u32 = 0x0035_5d10;

/// The default record's initial words (offset, value); every other word up
/// to `+0xe8` is 0.0.
pub(crate) const DPD_DEFAULTS: &[(u32, u32)] = &[
    (0x00, 1),
    (0x08, 0xbf80_0000), // -1.0
    (0x0c, 0x3f80_0000), // 1.0
    (0x10, 0x4080_0000), // 4.0
    (0x84, 0x3f80_0000),
    (0x94, 0x3f80_0000),
    (0x98, 0x3f80_0000),
    (0x9c, 0x3f80_0000),
    (0xa0, 0x3f80_0000),
    (0xa4, 0x3f80_0000),
    (0xb4, 0x3f80_0000),
    (0xc4, 0x10),
    (0xd0, 1),
    (0xd4, 0x41a0_0000), // 20.0
    (0xd8, 0xffff_ffff),
];

/// `Result WorldObjFactory_AddDynamicParticleData(Result* out, int n,
/// Entry* entries)` (0x2fee98): build a dynamic particle record (0xf0
/// bytes, allocated) from the default record (initialised on first use)
/// and `n` entries `{field, value, _, type}` (an int stored into a float
/// field is converted); clamp its range; look up the target object (by
/// handle, or the default one) and, when the object's node accepts
/// particles (virtual `+0x80`), hand the record to
/// `ModObj_AddDynamicParticleModifier`, else free it. Returns the constant
/// at `0x4c9098` (12 bytes into `*out`).
pub fn world_obj_factory_add_dynamic_particle_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(DPD_GUARD)).lw(c);
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = 0x50_0000;
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x70);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x20);
        if c.g[V0][0] == 0 {
            // First use: fill the default record.
            c.cycles += 76;
            let t = Addr(DPD_TEMPLATE);
            for off in (0..=0xe8).step_by(4) {
                let v = DPD_DEFAULTS
                    .iter()
                    .find(|&&(o, _)| o == off)
                    .map_or(0, |&(_, v)| v);
                t.off(off).sw(c, v as u64);
            }
            c.g[A1][0] = t.off(0xec).lbu(c);
            Addr(r32(c, GP).wrapping_add(DPD_GUARD)).sw(c, 1);
            // (Its trailing byte copied through a temporary.)
            Addr(sp(c)).sb(c, c.g[A1][0]);
            c.g[V0][0] = sx(DPD_TEMPLATE);
            c.g[A0][0] = 1;
            c.g[V1][0] = u64::MAX;
            c.g[A2][0] = 0x10;
            c.g[AT][0] = 0x41a0_0000;
            c.f[0] = 0;
            c.f[1] = 0x41a0_0000;
            c.f[2] = 0x4080_0000;
            c.f[3] = 0x3f80_0000;
        }

        // The record: allocate, copy the defaults.
        c.cycles += 17;
        c.g[A0][0] = 0xf0;
        c.g[A1][0] = sx(DPD_ALLOC_NAME);
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[S5][0] = 0x4d_0000;
        DPD_ALLOC.call(c, 0x002f_f01c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A2][0] = 0xf0;
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = sx(DPD_TEMPLATE);
        c.g[A0][0] = c.g[S1][0];
        Addr(r32(c, S1)).off(DPD_HANDLE).sw(c, u64::MAX);
        call_ptr(c, DPD_MEM_COPY, 0x002f_f03c)?;
        c.g[A2][0] = c.g[S4][0];
        if (c.g[S3][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S3][0];
            c.g[T0][0] = sx(DPD_FIELD_TYPES);
            c.g[A3][0] = 2;
            loop {
                let entry = Addr(r32(c, A2));
                c.cycles += 8;
                let field = (entry.lw(c) as u32) << 2;
                c.g[V1][0] = entry.off(0xc).lw(c);
                c.g[A0][0] = sx(field.wrapping_add(r32(c, S1)));
                c.g[V0][0] = Addr(field.wrapping_add(r32(c, T0))).lw(c);
                let dst = r32(c, A0);
                let int_into_float = c.g[V1][0] != c.g[V0][0] && {
                    c.cycles += 2;
                    c.g[V0][0] == c.g[A3][0]
                };
                if int_into_float {
                    c.cycles += 4;
                    c.lwc1(0, entry.0.wrapping_add(4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, dst);
                } else {
                    c.g[V0][0] = entry.off(4).lw(c);
                    c.cycles += 1;
                    Addr(dst).sw(c, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                c.g[A2][0] = sx(entry.0.wrapping_add(0x10));
                if c.g[A1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_f058);
                }
            }
        }

        // Clamp the range top to its bottom.
        c.cycles += 6;
        let rec = r32(c, S1);
        c.lwc1(0, rec.wrapping_add(DPD_RANGE_HI));
        c.lwc1(1, rec.wrapping_add(DPD_RANGE_LO));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, rec.wrapping_add(DPD_RANGE_HI));
        }

        // The target object.
        c.cycles += 4;
        c.g[A2][0] = Addr(rec).off(DPD_HANDLE).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        c.g[A1][0] = Addr(r32(c, GP).wrapping_add(DPD_HANDLES)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, GP).wrapping_add(DPD_SCRIPT_MAN)).lw(c);
            c.g[V0][0] = Addr(r32(c, V0)).off(0x290).lw(c);
        } else {
            c.cycles += 8;
            let handle = r32(c, A2);
            c.g[V0][0] = sx((handle & 0xff) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(8).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = sx(handle >> 8);
            let mut found = false;
            if c.g[V1][0] != 0 {
                c.cycles += 7;
                c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
                let at = (handle >> 8 << 2).wrapping_add(r32(c, A0));
                c.g[V0][0] = sx(at);
                c.g[V1][0] = Addr(at).lw(c);
                let index = r32(c, V1) >> 8;
                c.g[V0][0] = sx(index << 2);
                found = index != 0;
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }

        c.cycles += 3;
        c.g[S0][0] = Addr(r32(c, V0)).off(0xc).lw(c);
        let mut handed_over = false;
        if c.g[S0][0] != 0 {
            c.cycles += 7;
            let node = r32(c, S0);
            c.g[V1][0] = Addr(node).off(0xc).lw(c);
            let vt = r32(c, V1);
            c.g[A0][0] = Addr(vt).off(0x80).lh(c);
            c.g[V0][0] = Addr(vt).off(0x84).lw(c);
            c.g[A0][0] = sx(node.wrapping_add(r32(c, A0)));
            let accepts = r32(c, V0);
            call_ptr(c, accepts, 0x002f_f134)?;
            let ok = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            if ok {
                c.cycles += 15;
                let rec = r32(c, S1);
                c.lwc1(3, rec.wrapping_add(DPD_VEC));
                c.lwc1(2, rec.wrapping_add(DPD_VEC + 4));
                c.g[A0][0] = c.g[S0][0];
                c.lwc1(1, rec.wrapping_add(DPD_VEC + 8));
                let v = frame.at(c, 0x10);
                c.g[A3][0] = sx(v);
                c.g[AT][0] = 0x3f80_0000;
                c.f[0] = 0x3f80_0000;
                c.swc1(3, v);
                c.swc1(2, v + 4);
                c.swc1(1, v + 8);
                c.swc1(0, v + 0xc);
                c.g[A2][0] = Addr(rec).off(DPD_SLOT).lw(c);
                call_ptr(c, DPD_ADD_MODIFIER, 0x002f_f170)?;
                handed_over = true;
            }
        }
        if !handed_over {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            DPD_DELETE.call(c, 0x002f_f180)?;
        }

        // Return the 12-byte constant.
        c.cycles += 17;
        c.g[A1][0] = sx(r32(c, S5).wrapping_sub(0x6f68));
        let src = r32(c, A1);
        c.ldl(2, src.wrapping_add(7));
        c.ldr(2, src);
        c.g[V1][0] = Addr(src).off(8).lw(c);
        let out = r32(c, S2);
        c.sdl(2, out.wrapping_add(7));
        c.sdr(2, out);
        Addr(out).off(8).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
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

// ---------------------------------------------------------------------------
// RefPack_Unpack (0x3b4740).

pub(crate) const REFPACK_DECODE: Callee = Callee {
    addr: 0x003b_49d0,
    f: ee::s13::RefPack_Decode,
};

/// Where RefPack_Unpack's compare chain leaves: reload `ra` first (`E8`),
/// or with `ra` already reloaded (`EC`), both returning `a1`; or straight
/// out with `v0` already set.
pub(crate) enum RefPackExit {
    ReloadRa,
    ReturnA1,
    Out,
}

/// `int RefPack_Unpack(const u8* src, u8* dst)` (0x3b4740): decode a
/// RefPack stream (`xx fb` header) whose type byte (ignoring bit 0) is 0x10
/// or 0x90 with `RefPack_Decode`; any other stream yields 0. Returns the
/// decoded size.
pub fn refpack_unpack(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = 0xfb;
        let src = Addr(r32(c, A3));
        c.g[V1][0] = src.off(1).lbu(c);
        c.g[A1][0] = 0;
        let exit = 'chain: {
            if c.g[V1][0] != 0xfb {
                break 'chain RefPackExit::ReloadRa;
            }
            c.cycles += 5;
            c.g[V0][0] = src.lbu(c);
            c.g[V1][0] = 0x46;
            let kind = c.g[V0][0] & 0xfe;
            c.g[A2][0] = kind;
            c.g[V0][0] = (kind < 71) as u64;
            if kind == 0x46 {
                break 'chain RefPackExit::ReloadRa;
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1c;
            if kind < 71 {
                c.cycles += 2;
                c.g[V0][0] = (kind < 29) as u64;
                if kind == 0x1c {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                c.g[V0][0] = 0x18;
                if kind >= 29 {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                c.g[V0][0] = (kind < 25) as u64;
                if kind == 0x18 {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                c.g[V0][0] = 0x10;
                if kind >= 25 {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                frame.restore_ra(c, 0);
                if kind != 0x10 {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A1][0];
                    break 'chain RefPackExit::Out;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = (kind < 159) as u64;
                if kind == 0x9e {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                c.g[V0][0] = 0x98;
                if kind >= 159 {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                c.g[V0][0] = (kind < 153) as u64;
                if kind == 0x98 {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                c.g[V0][0] = 0x90;
                if kind >= 153 {
                    break 'chain RefPackExit::ReloadRa;
                }
                c.cycles += 2;
                frame.restore_ra(c, 0);
                if kind != 0x90 {
                    break 'chain RefPackExit::ReturnA1;
                }
            }
            // RefPack_Decode(dst, src, NULL).
            c.cycles += 4;
            c.g[A1][0] = c.g[A3][0];
            c.g[A2][0] = 0;
            REFPACK_DECODE.call(c, 0x003b_47e4)?;
            c.g[A1][0] = c.g[V0][0];
            RefPackExit::ReloadRa
        };
        if let RefPackExit::ReloadRa = exit {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        if !matches!(exit, RefPackExit::Out) {
            c.cycles += 1;
            c.g[V0][0] = c.g[A1][0];
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// log (0x40d610), newlib's `w_log.c` wrapper (soft-float doubles in a0/v0).

/// `_LIB_VERSION` (-1 IEEE, 0 SVID, 1 XOPEN, 2 POSIX).
pub(crate) const LOG_LIB_VERSION: u32 = 0x0049_9688;

/// The exception's name ("log"), `-HUGE` (SVID) and `HUGE_VAL`.
pub(crate) const LOG_NAME: u32 = 0x0049_9070;

pub(crate) const LOG_MINUS_HUGE: u32 = 0x0049_9078;

pub(crate) const LOG_HUGE_VAL: u32 = 0x0049_9680;

/// `struct exception` on the stack: `+0` type, `+4` name, `+8`/`+0x10`
/// the arguments, `+0x18` the return value, `+0x20` err.
pub(crate) const LOG_EXC_TYPE: u32 = 0;

pub(crate) const LOG_EXC_NAME: u32 = 4;

pub(crate) const LOG_EXC_ARG1: u32 = 8;

pub(crate) const LOG_EXC_ARG2: u32 = 0x10;

pub(crate) const LOG_EXC_RETVAL: u32 = 0x18;

pub(crate) const LOG_EXC_ERR: u32 = 0x20;

pub(crate) const LOG_IEEE754_LOG: Callee = Callee {
    addr: 0x0040_db58,
    f: ee::s15::__ieee754_log,
};

pub(crate) const LOG_ISNAN: Callee = Callee {
    addr: 0x0040_f840,
    f: ee::s15::isnan,
};

pub(crate) const LOG_CMPDF2: Callee = Callee {
    addr: 0x0041_2500,
    f: ee::s15::__cmpdf2,
};

pub(crate) const LOG_SUBDF3: Callee = Callee {
    addr: 0x0041_1f70,
    f: ee::s15::__subdf3,
};

pub(crate) const LOG_GETREENT: Callee = Callee {
    addr: 0x0041_5fb8,
    f: ee::s15::__getreent,
};

pub(crate) const LOG_MATHERR: Callee = Callee {
    addr: 0x0040_f878,
    f: ee::s15::matherr,
};

/// The next step of `log`'s error report.
pub(crate) enum LogStep {
    /// Not POSIX: let `matherr` handle it.
    AskMatherr,
    /// errno = EDOM.
    Edom,
    /// Store `v1` into errno (`v0` the reentrancy struct).
    StoreErrno,
    /// `matherr` handled it.
    Handled,
}

/// `double log(double x)` (0x40d610): `__ieee754_log(x)`; outside IEEE
/// mode, for x <= 0 (not NaN) fill a `struct exception` (SING for 0, DOMAIN
/// for negatives; return -HUGE in SVID mode, else -HUGE_VAL), set errno
/// (ERANGE/EDOM) directly in POSIX mode, otherwise ask `matherr` and set
/// EDOM if it declines; a non-zero `exc.err` goes to errno too. Returns the
/// exception's value.
pub fn libm_log(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x90);
        let a = frame.at(c, 0x30);
        c.sd(a, c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        let a = frame.at(c, 0x60);
        c.sd(a, c.g[S3][0]);
        let a = frame.at(c, 0x50);
        c.sd(a, c.g[S2][0]);
        let a = frame.at(c, 0x40);
        c.sd(a, c.g[S1][0]);
        c.g[S3][0] = 0x4a_0000;
        frame.save_ra(c, 0x80);
        let a = frame.at(c, 0x70);
        c.sd(a, c.g[S4][0]);
        LOG_IEEE754_LOG.call(c, 0x0040_d638)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[S1][0] = Addr(LOG_LIB_VERSION).lw(c);
        c.g[V0][0] = u64::MAX;
        'done: {
            let ieee = c.g[S1][0] == u64::MAX;
            c.g[V0][0] = c.g[S2][0];
            if ieee {
                break 'done;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            LOG_ISNAN.call(c, 0x0040_d654)?;
            let nan = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S2][0];
            if nan {
                break 'done;
            }
            c.cycles += 6;
            c.g[S4][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S4][0];
            LOG_CMPDF2.call(c, 0x0040_d66c)?;
            let positive = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = c.g[S2][0];
            if positive {
                break 'done;
            }

            // x <= 0: the exception record.
            c.cycles += 7;
            Addr(frame.at(c, LOG_EXC_ERR)).sw(c, 0);
            c.g[V0][0] = sx(LOG_NAME);
            Addr(frame.at(c, LOG_EXC_ARG1)).sd(c, c.g[S0][0]);
            Addr(frame.at(c, LOG_EXC_NAME)).sw(c, c.g[V0][0]);
            Addr(frame.at(c, LOG_EXC_ARG2)).sd(c, c.g[S0][0]);
            if c.g[S1][0] == 0 {
                // SVID: -HUGE.
                c.cycles += 4;
                c.g[AT][0] = 0x4a_0000;
                c.g[V0][0] = Addr(LOG_MINUS_HUGE).ld(c);
            } else {
                // -HUGE_VAL.
                c.cycles += 5;
                c.g[V0][0] = 0x4a_0000;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = Addr(LOG_HUGE_VAL).ld(c);
                LOG_SUBDF3.call(c, 0x0040_d6b0)?;
            }
            Addr(frame.at(c, LOG_EXC_RETVAL)).sd(c, c.g[V0][0]);

            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            LOG_CMPDF2.call(c, 0x0040_d6c0)?;
            let zero = c.g[V0][0] == 0;
            c.g[V1][0] = 1;
            // POSIX sets errno directly; otherwise matherr may handle it.
            let posix_erange = if zero {
                // SING.
                c.cycles += 4;
                c.g[V0][0] = 2;
                c.g[V1][0] = Addr(LOG_LIB_VERSION).lw(c);
                Addr(frame.at(c, LOG_EXC_TYPE)).sw(c, c.g[V0][0]);
                c.g[V1][0] == 2
            } else {
                false
            };
            let step = if zero {
                if posix_erange {
                    c.cycles += 4;
                    LOG_GETREENT.call(c, 0x0040_d6e0)?;
                    c.g[V1][0] = 0x22; // ERANGE
                    LogStep::StoreErrno
                } else {
                    LogStep::AskMatherr
                }
            } else {
                // DOMAIN.
                c.cycles += 4;
                c.g[V0][0] = Addr(LOG_LIB_VERSION).lw(c);
                c.g[A0][0] = 2;
                Addr(frame.at(c, LOG_EXC_TYPE)).sw(c, c.g[V1][0]);
                if c.g[V0][0] == 2 {
                    LogStep::Edom
                } else {
                    LogStep::AskMatherr
                }
            };
            let step = if let LogStep::AskMatherr = step {
                c.cycles += 4;
                c.g[A0][0] = sx(sp(c));
                LOG_MATHERR.call(c, 0x0040_d700)?;
                let handled = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(frame.at(c, LOG_EXC_ERR)).lw(c);
                if handled {
                    LogStep::Handled
                } else {
                    LogStep::Edom
                }
            } else {
                step
            };
            let step = if let LogStep::Edom = step {
                c.cycles += 3;
                LOG_GETREENT.call(c, 0x0040_d710)?;
                c.g[V1][0] = 0x21; // EDOM
                LogStep::StoreErrno
            } else {
                step
            };
            if let LogStep::StoreErrno = step {
                c.cycles += 2;
                Addr(r32(c, V0)).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(frame.at(c, LOG_EXC_ERR)).lw(c);
            }
            // A non-zero exc.err becomes errno.
            c.cycles += 2;
            let exc_err = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(frame.at(c, LOG_EXC_RETVAL)).ld(c);
            if exc_err {
                c.cycles += 5;
                LOG_GETREENT.call(c, 0x0040_d72c)?;
                c.g[V1][0] = Addr(frame.at(c, LOG_EXC_ERR)).lw(c);
                Addr(r32(c, V0)).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(frame.at(c, LOG_EXC_RETVAL)).ld(c);
            }
        }
        c.cycles += 8;
        frame.restore_ra(c, 0x80);
        let a = frame.at(c, 0x70);
        c.g[S4][0] = c.ld(a);
        let a = frame.at(c, 0x60);
        c.g[S3][0] = c.ld(a);
        let a = frame.at(c, 0x50);
        c.g[S2][0] = c.ld(a);
        let a = frame.at(c, 0x40);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0x30);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObjFactory_AnimObject (0x2fbcb8)

/// gp-relative guard word of the static default anim parameters.
pub(crate) const WOFA_DEFAULTS_READY_GP: u32 = 0x24e8;

/// The static default parameter block (11 words: id -1, 1, 0, -1.0, -1.0,
/// 30.0, 0, -1.0, 0, 0, tag byte at +0x28).
pub(crate) const WOFA_DEFAULTS: u32 = 0x004f_b498;

/// Bytes copied from the defaults into the local block.
pub(crate) const WOFA_BLOCK_BYTES: u64 = 0x2c;

/// Parameter kinds by slot (`D_00445F48`): kind 2 is a float slot.
pub(crate) const WOFA_PARAM_KINDS: u32 = 0x0044_5f48;

/// gp-relative: the world-object directory and the world.
pub(crate) const WOFA_DIRECTORY_GP: u32 = 0x16c8;

pub(crate) const WOFA_WORLD_GP: u32 = 0xce8;

/// World -> its default object (used for id -1).
pub(crate) const WOFA_WORLD_DEFAULT_OBJECT: u32 = 0x290;

/// Object -> its active node.
pub(crate) const WOFA_OBJECT_NODE: u32 = 0xc;

/// The factory's result value (12 bytes, returned by copy).
pub(crate) const WOFA_RESULT: u32 = 0x004c_9098;

pub(crate) const WOFA_ATTACH_MODIFIER: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

pub(crate) const WOFA_MEM_ALLOC_NAMED: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};

/// `Result WorldObjFactory_AnimObject(Param* params, int count)` (0x2fbcb8;
/// a0 = hidden result slot, a1 = count, a2 = params): fills an 11-word anim
/// parameter block from the (lazily initialised) defaults and the
/// `{slot, value, -, kind}` parameters (an int for a float slot is
/// converted), looks the object id up (-1: the world's default object), and
/// unless the object already has an active node accepting the modifier
/// (`AttachModifier(node, 1, block)`), allocates and constructs an
/// `AnimObject` (0x6c bytes) for it. Returns the factory's result value.
/// Frame 0xa0 (s0-s4).
pub fn world_obj_factory_anim_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        let gp = Addr(c.g[GP][0] as u32);
        c.g[V0][0] = gp.off(WOFA_DEFAULTS_READY_GP).lw(c);
        frame.save(c, S3, 0x60);
        frame.save(c, S0, 0x90);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x80);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x50);
        c.g[A2][0] = 0x50_0000;
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0x40);
        let defaults = Addr(WOFA_DEFAULTS);
        if !ready {
            // First use: initialise the static defaults (guarded local).
            c.cycles += 22;
            c.g[V0][0] = sx(WOFA_DEFAULTS);
            let tag = defaults.off(0x28).lbu(c);
            c.g[A1][0] = tag;
            c.g[V1][0] = u64::MAX;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.f[0] = 0xbf80_0000; // -1.0
            c.g[A0][0] = 1;
            c.g[AT][0] = 0x41f0_0000;
            c.f[1] = 0x41f0_0000; // 30.0
            defaults.sw(c, u64::MAX);
            defaults.off(0x28).sb(c, tag);
            defaults.off(0x14).write_u32(c, 0x41f0_0000);
            defaults.off(0x1c).write_u32(c, 0xbf80_0000);
            gp.off(WOFA_DEFAULTS_READY_GP).sw(c, 1);
            defaults.off(0x4).sw(c, 1);
            defaults.off(0x8).sw(c, 0);
            defaults.off(0xc).write_u32(c, 0xbf80_0000);
            defaults.off(0x10).write_u32(c, 0xbf80_0000);
            defaults.off(0x18).sw(c, 0);
            defaults.off(0x20).sw(c, 0);
            defaults.off(0x24).sw(c, 0);
            Addr(sp(c)).sb(c, tag);
        }
        // The parameter block at 0x10(sp), copied from the defaults.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        let block = frame.at(c, 0x10);
        c.g[S1][0] = sx(block);
        c.g[A1][0] = sx(WOFA_DEFAULTS);
        c.g[A2][0] = WOFA_BLOCK_BYTES;
        Addr(block).sw(c, u64::MAX);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x4d_0000;
        call_ptr(c, 0x003e_6574, 0x002f_bd68)?; // EA_MemCopy
        c.g[A2][0] = c.g[S3][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(WOFA_PARAM_KINDS);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                let param = Addr(c.g[A2][0] as u32);
                let slot4 = (param.lw(c) as u32) << 2;
                c.g[V1][0] = param.off(0xc).lw(c);
                let dest = Addr(slot4.wrapping_add(c.g[S1][0] as u32));
                c.g[A0][0] = sx(dest.0);
                c.g[V0][0] = sx(slot4.wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                if c.g[V1][0] != c.g[V0][0] && c.g[V0][0] == c.g[A3][0] {
                    // An int given for a float slot.
                    c.cycles += 2 + 4;
                    let value = param.off(4).read_u32(c);
                    let converted = crate::ps2f::FpuF32::from_i32(value as i32);
                    c.f[0] = converted.0;
                    dest.write_fpu(c, converted);
                } else {
                    c.cycles += if c.g[V1][0] == c.g[V0][0] { 1 } else { 2 + 1 };
                    c.g[V0][0] = param.off(4).lw(c);
                    let value = c.g[V0][0];
                    dest.sw(c, value);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if c.g[A1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_bd80);
                }
            }
        }
        // Look the object up.
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        let gp = Addr(c.g[GP][0] as u32);
        c.g[A1][0] = gp.off(WOFA_DIRECTORY_GP).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = gp.off(WOFA_WORLD_GP).lw(c);
            let world = c.g[V0][0] as u32;
            c.g[S0][0] = Addr(world.wrapping_add(WOFA_WORLD_DEFAULT_OBJECT)).lw(c);
        } else {
            c.cycles += 8;
            let id = c.g[A2][0] as u32;
            c.g[V0][0] = sx((id & 0xff) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx(id >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !found {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A3][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A3][0] = 0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A3][0];
        }
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(WOFA_OBJECT_NODE)).lw(c);
            c.g[A1][0] = 1;
            let mut create = true;
            if c.g[A0][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                WOFA_ATTACH_MODIFIER.call(c, 0x002f_be50)?;
                create = c.g[V0][0] == 0;
            }
            if create {
                c.cycles += 12;
                c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x6958));
                c.g[A0][0] = 0x6c;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                WOFA_MEM_ALLOC_NAMED.call(c, 0x002f_be70)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                c.g[A3][0] = c.g[S0][0];
                c.g[T0][0] = c.g[S1][0];
                c.g[A2][0] = 1;
                call_ptr(c, 0x0034_1aa0, 0x002f_be88)?; // AnimObject_ctor
            }
        }
        // Return the factory's result value by copy.
        c.cycles += 16;
        let result = Addr((c.g[S4][0] as u32).wrapping_sub(0x4d_0000 - WOFA_RESULT));
        c.g[A1][0] = sx(result.0);
        let head = result.read_u64_unaligned(c);
        c.g[V0][0] = head;
        c.g[V1][0] = result.off(8).lw(c);
        let out = Addr(c.g[S2][0] as u32);
        out.write_u64_unaligned(c, head);
        let tail = c.g[V1][0];
        out.off(8).sw(c, tail);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_PlayObjectSound (0x2fff00)

/// `gp` word: the static parameter defaults are initialised.
pub(crate) const OC300_POS_DEFAULTS_READY_GP: u32 = 0x2574;

/// The static parameter defaults (3 words: object id, sound, -; 0x4fbb08).
pub(crate) const OC300_POS_DEFAULTS: u32 = 0x004f_bb08;

/// `gp` table: the kind of each parameter slot (2 = float).
pub(crate) const OC300_POS_PARAM_KINDS_GP: u32 = 0xb38;

pub(crate) const OC300_POS_KIND_FLOAT: u64 = 2;

/// `gp` words: the object directory, the world (its default object at
/// `+0x290`).
pub(crate) const OC300_POS_DIRECTORY_GP: u32 = 0x16c8;

pub(crate) const OC300_POS_WORLD_GP: u32 = 0xce8;

pub(crate) const OC300_POS_WORLD_DEFAULT_OBJECT: u32 = 0x290;

/// The script functions' shared result value (12 bytes at 0x4c9098).
pub(crate) const OC300_POS_RESULT: u32 = 0x004c_9098;

/// `Result WScriptFn_PlayObjectSound(Param* params, int count)` (0x2fff00;
/// a0 = the hidden result slot, a1 = count, a2 = params): fills a 3-word
/// parameter block (object id, sound) from the defaults and the
/// `{slot, value, -, kind}` parameters (an int given for a float slot is
/// converted), looks the object up (id -1: the world's default object) and,
/// if there is one, plays the sound on it
/// (`SSXAudioSystem_PlayObjectSound(GetInstance(), obj, sound)`). Returns
/// the shared script result.
pub fn wscript_fn_play_object_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let gp = r32(c, GP);
        let ready = Addr(gp).off(OC300_POS_DEFAULTS_READY_GP).lw(c);
        c.g[V0][0] = ready;
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x50_0000;
        let defaults = Addr(OC300_POS_DEFAULTS);
        if ready == 0 {
            // First use: initialise the static defaults (guarded local).
            c.cycles += 10;
            c.g[V1][0] = sx(OC300_POS_DEFAULTS);
            c.g[V0][0] = u64::MAX;
            let tag = defaults.off(8).lbu(c);
            c.g[A1][0] = tag;
            defaults.sw(c, u64::MAX);
            c.g[A0][0] = 1;
            defaults.off(8).sb(c, tag);
            defaults.off(4).sw(c, u64::MAX);
            Addr(gp).off(OC300_POS_DEFAULTS_READY_GP).sw(c, 1);
            Addr(sp(c)).sb(c, tag);
        }
        // The parameter block at 0x10(sp), copied from the defaults.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let block = frame.at(c, 0x10);
        c.g[S0][0] = sx(block);
        c.g[A1][0] = sx(OC300_POS_DEFAULTS);
        c.g[A2][0] = 0xc;
        Addr(block).sw(c, u64::MAX);
        c.g[A0][0] = sx(block);
        call_ptr(c, 0x003e_6574, 0x002f_ff78)?; // memcpy
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(r32(c, GP).wrapping_add(OC300_POS_PARAM_KINDS_GP));
            c.g[A3][0] = OC300_POS_KIND_FLOAT;
            loop {
                c.cycles += 8;
                let param = Addr(r32(c, A2));
                let slot4 = (param.lw(c) as u32) << 2;
                let given_kind = param.off(0xc).lw(c);
                c.g[V1][0] = given_kind;
                let dest = Addr(slot4.wrapping_add(r32(c, S0)));
                c.g[A0][0] = sx(dest.0);
                let kind = Addr(slot4.wrapping_add(r32(c, T0))).lw(c);
                c.g[V0][0] = kind;
                if given_kind != kind && kind == c.g[A3][0] {
                    // An int for a float slot.
                    c.cycles += 2 + 4;
                    let value = param.off(4).lw(c) as u32;
                    let converted = FpuF32::from_i32(value as i32);
                    c.f[0] = converted.0;
                    dest.write_fpu(c, converted);
                } else {
                    c.cycles += if given_kind == kind { 1 } else { 2 + 1 };
                    let value = param.off(4).lw(c);
                    c.g[V0][0] = value;
                    dest.sw(c, value);
                }
                c.cycles += 3;
                let left = r32(c, A1).wrapping_sub(1);
                c.g[A1][0] = sx(left);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if left == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_ff90);
                }
            }
        }
        // Look the object up.
        c.cycles += 4;
        let id = Addr(r32(c, S0)).lw(c);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        let directory = Addr(r32(c, GP)).off(OC300_POS_DIRECTORY_GP).lw(c);
        c.g[A1][0] = directory;
        if !id == 0 {
            c.cycles += 3;
            let world = Addr(r32(c, GP)).off(OC300_POS_WORLD_GP).lw(c);
            c.g[V0][0] = world;
            c.g[S1][0] = Addr(world as u32).off(OC300_POS_WORLD_DEFAULT_OBJECT).lw(c);
        } else {
            c.cycles += 8;
            let id = id as u32;
            c.g[V0][0] = sx((id & 0xff) << 2);
            let sections = Addr(directory as u32).lw(c);
            c.g[V1][0] = sections;
            let table = Addr(sections as u32).off(8).lw(c);
            c.g[A0][0] = table;
            let at = ((id & 0xff) << 2).wrapping_add(table as u32);
            c.g[V0][0] = sx(at);
            let section = Addr(at).lw(c);
            c.g[V1][0] = section;
            let object = if section != 0 {
                c.cycles += 7;
                let entries = Addr(section as u32).off(0x1c).lw(c);
                c.g[A0][0] = entries;
                let entry = Addr(((id >> 8) << 2).wrapping_add(entries as u32)).lw(c);
                c.g[V1][0] = entry;
                let index = (entry as u32) >> 8;
                let object = if index != 0 {
                    sx(index << 2)
                } else {
                    c.cycles += 1;
                    0
                };
                c.g[V0][0] = object;
                c.cycles += 1;
                object
            } else {
                c.cycles += 2;
                0
            };
            c.g[A1][0] = object;
            c.cycles += 1;
            c.g[S1][0] = object;
        }
        c.cycles += 2;
        c.g[V1][0] = 0x4d_0000;
        if c.g[S1][0] != 0 {
            // SSXAudioSystem_PlayObjectSound(GetInstance(), obj, sound).
            c.cycles += 7;
            c.g[S0][0] = Addr(r32(c, S0)).off(4).lw(c);
            call_ptr(c, 0x0028_b180, 0x0030_0054)?; // SSXAudioSystem_GetInstance
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0029_74a0, 0x0030_0064)?; // SSXAudioSystem_PlayObjectSound
            c.g[V1][0] = 0x4d_0000;
        }
        // *result = the shared script result (12 bytes).
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(OC300_POS_RESULT);
        let out = r32(c, S3);
        c.ldl(4, OC300_POS_RESULT.wrapping_add(0x7));
        c.ldr(4, OC300_POS_RESULT);
        c.g[A1][0] = Addr(OC300_POS_RESULT).off(8).lw(c);
        c.sdl(4, out.wrapping_add(0x7));
        c.sdr(4, out);
        Addr(out).off(8).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_MakeDeadNode (0x2fc420)

/// gp-relative guard of the static default parameter block.
pub(crate) const OC305_DEAD_NODE_DEFAULTS_READY_GP: u32 = 0x24f4;

/// The static default parameter block `{id = -1, 0, tag}` (12 bytes;
/// `0x50_0000 - 0x4b00`).
pub(crate) const OC305_DEAD_NODE_DEFAULTS: u32 = 0x004f_b500;

/// gp-relative: parameter kinds by slot (`D_004A3C00`); kind 2 is a float.
pub(crate) const OC305_DEAD_NODE_KINDS_GP: u32 = 0xb10;

pub(crate) const OC305_DEAD_NODE_KIND_FLOAT: u64 = 2;

/// gp-relative: the world-object directory and the world (its default
/// object at `+0x290`).
pub(crate) const OC305_DEAD_NODE_DIRECTORY_GP: u32 = 0x16c8;

pub(crate) const OC305_DEAD_NODE_WORLD_GP: u32 = 0xce8;

pub(crate) const OC305_DEAD_NODE_WORLD_DEFAULT_OBJECT: u32 = 0x290;

/// Object: its active node (`+0xc`).
pub(crate) const OC305_DEAD_NODE_OBJECT_NODE: u32 = 0xc;

/// The factory's result value (12 bytes, returned by copy; `0x4d_0000 -
/// 0x6f68`).
pub(crate) const OC305_DEAD_NODE_RESULT: u32 = 0x004c_9098;

/// `EA_MemCopy` (0x3e6574).
pub(crate) const OC305_EA_MEM_COPY: u32 = 0x003e_6574;

/// `WorldObjFactory_AttachModifier` (0x2fae38).
pub(crate) const OC305_WOF_ATTACH_MODIFIER: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// `WorldObjFactory_DeadNode` (0x2fc2c0).
pub(crate) const OC305_WOF_DEAD_NODE: Callee = Callee {
    addr: 0x002f_c2c0,
    f: ee::s10::WorldObjFactory_DeadNode,
};

/// `Result WScriptFn_MakeDeadNode(Param* params, int count)` (0x2fc420; a0
/// = the hidden result slot, a1 = count, a2 = params): fill a 3-word
/// parameter block from the static defaults and the `{slot, value, -, kind}`
/// parameters (an int given for a float slot is converted), look the object
/// id up (-1: the world's default object), and unless the object's active
/// node already takes a kind-6 modifier for these parameters, make it a
/// dead node (`WorldObjFactory_DeadNode(object, block)`). Returns the
/// factory's result value. Frame 0x80 (ra at 0x20, s4..s0 at
/// 0x30..0x70, the block at 0x10).
pub fn wscript_fn_make_dead_node(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        let gp = c.g[GP][0] as u32;
        let ready = Addr(gp).off(OC305_DEAD_NODE_DEFAULTS_READY_GP).lw(c);
        c.g[V0][0] = ready;
        frame.save(c, S3, 0x40);
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[A2][0] = 0x50_0000;
        frame.save_ra(c, 0x20);
        let defaults = Addr(OC305_DEAD_NODE_DEFAULTS);
        if ready == 0 {
            // First use: initialise the static defaults (guarded local).
            c.cycles += 10;
            c.g[V1][0] = sx(defaults.0);
            c.g[V0][0] = u64::MAX;
            let tag = defaults.off(8).lbu(c);
            c.g[A1][0] = tag;
            defaults.sw(c, u64::MAX);
            c.g[A0][0] = 1;
            defaults.off(8).sb(c, tag);
            Addr(gp).off(OC305_DEAD_NODE_DEFAULTS_READY_GP).sw(c, 1);
            defaults.off(4).sw(c, 0);
            Addr(sp(c)).sb(c, tag);
        }
        // The parameter block at 0x10(sp), copied from the defaults.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        let block = frame.at(c, 0x10);
        c.g[S1][0] = sx(block);
        c.g[A1][0] = sx(defaults.0);
        c.g[A2][0] = 0xc;
        Addr(block).sw(c, u64::MAX);
        c.g[A0][0] = sx(block);
        c.g[S4][0] = 0x4d_0000;
        call_ptr(c, OC305_EA_MEM_COPY, 0x002f_c4a0)?;
        c.g[A2][0] = c.g[S3][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(OC305_DEAD_NODE_KINDS_GP));
            c.g[A3][0] = OC305_DEAD_NODE_KIND_FLOAT;
            loop {
                // Param cursor a2, count a1 (an event at the head sees them).
                c.cycles += 8;
                let param = Addr(r32(c, A2));
                let slot4 = (param.lw(c) as u32) << 2;
                let given_kind = param.off(0xc).lw(c);
                c.g[V1][0] = given_kind;
                let dest = Addr(slot4.wrapping_add(r32(c, S1)));
                c.g[A0][0] = sx(dest.0);
                let kind = Addr(slot4.wrapping_add(r32(c, T0))).lw(c);
                c.g[V0][0] = kind;
                if given_kind != kind && kind == c.g[A3][0] {
                    // An int for a float slot.
                    c.cycles += 2 + 4;
                    let value = param.off(4).lw(c) as u32;
                    let converted = FpuF32::from_i32(value as i32);
                    c.f[0] = converted.0;
                    dest.write_fpu(c, converted);
                } else {
                    c.cycles += if given_kind == kind { 1 } else { 2 + 1 };
                    let value = param.off(4).lw(c);
                    c.g[V0][0] = value;
                    dest.sw(c, value);
                }
                c.cycles += 3;
                let left = r32(c, A1).wrapping_sub(1);
                c.g[A1][0] = sx(left);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if left == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_c4b8);
                }
            }
        }
        // Look the object up.
        c.cycles += 4;
        let gp = c.g[GP][0] as u32;
        let id = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        let directory = Addr(gp).off(OC305_DEAD_NODE_DIRECTORY_GP).lw(c);
        c.g[A1][0] = directory;
        if !id == 0 {
            c.cycles += 3;
            let world = Addr(gp).off(OC305_DEAD_NODE_WORLD_GP).lw(c);
            c.g[V0][0] = world;
            c.g[S0][0] = Addr(world as u32)
                .off(OC305_DEAD_NODE_WORLD_DEFAULT_OBJECT)
                .lw(c);
        } else {
            c.cycles += 8;
            let id = id as u32;
            let sections = Addr(directory as u32).lw(c);
            let table = Addr(sections as u32).off(8).lw(c);
            c.g[A0][0] = table;
            let at = ((id & 0xff) << 2).wrapping_add(table as u32);
            c.g[V0][0] = sx(at);
            let section = Addr(at).lw(c);
            c.g[V1][0] = section;
            let object = if section != 0 {
                c.cycles += 8;
                let entries = Addr(section as u32).off(0x1c).lw(c);
                c.g[A0][0] = entries;
                let entry = Addr(((id >> 8) << 2).wrapping_add(entries as u32)).lw(c);
                c.g[V1][0] = entry;
                let index = (entry as u32) >> 8;
                let object = if index != 0 {
                    sx(index << 2)
                } else {
                    c.cycles += 1;
                    0
                };
                c.g[V0][0] = object;
                c.cycles += 1;
                object
            } else {
                0
            };
            c.g[A0][0] = object;
            c.cycles += 1;
            c.g[S0][0] = object;
        }
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            let node = Addr(r32(c, S0)).off(OC305_DEAD_NODE_OBJECT_NODE).lw(c);
            c.g[A0][0] = node;
            c.g[A1][0] = 6;
            let mut kill = true;
            if node != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                OC305_WOF_ATTACH_MODIFIER.call(c, 0x002f_c584)?;
                kill = c.g[V0][0] == 0;
            }
            if kill {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                OC305_WOF_DEAD_NODE.call(c, 0x002f_c598)?;
            }
        }
        // Return the factory's result value by copy.
        c.cycles += 16;
        let result = Addr(r32(c, S4).wrapping_sub(0x4d_0000 - OC305_DEAD_NODE_RESULT));
        c.g[A1][0] = sx(result.0);
        let head = result.read_u64_unaligned(c);
        let tail = result.off(8).lw(c);
        c.g[V1][0] = tail;
        let out = Addr(r32(c, S2));
        out.write_u64_unaligned(c, head);
        out.off(8).sw(c, tail);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `StreamFile_Validate(file, &header, &handle)` (0x3dfbd0): 0 when the
/// stream file is valid (and the out parameters are written), else 1.
pub(crate) const SFILE_VALIDATE: Callee = Callee {
    addr: 0x003d_fbd0,
    f: ee::s14::f_003dfbd0,
};

/// The stream handle's buffered byte count for its channel.
pub(crate) const SFILE_HANDLE_CHANNEL_BYTES: u32 = 0x8;

/// `int StreamFile_GetChannelBytes(StreamFile *file)` (0x3e14c8): the bytes
/// buffered for the stream file's channel (handle +0x8), or 0 when the file
/// does not validate.
pub fn stream_file_get_channel_bytes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        // Out parameters: header at 0(sp), handle at 4(sp).
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[SP][0] | 0x4;
        SFILE_VALIDATE.call(c, 0x003e_14dc)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 2;
            let handle = Addr(frame.at(c, 4)).lw(c);
            c.g[V1][0] = handle;
            c.g[V0][0] = Addr((handle as u32).wrapping_add(SFILE_HANDLE_CHANNEL_BYTES)).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The stream header's read state (0 = idle).
pub(crate) const SFILE_HEADER_READ_STATE: u32 = 0x38;

/// `bool StreamFile_IsIdle(StreamFile *file)` (0x3e1530): whether a valid
/// stream file has no read in flight (header read state 0) and no bytes
/// buffered for its channel; false for an invalid file.
pub fn stream_file_is_idle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        // Out parameters: header at 0(sp), handle at 4(sp).
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[SP][0] | 0x4;
        SFILE_VALIDATE.call(c, 0x003e_1544)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 5;
            let header = Addr(frame.at(c, 0)).lw(c);
            c.g[V1][0] = header;
            c.g[A0][0] = 0;
            let state = Addr((header as u32).wrapping_add(SFILE_HEADER_READ_STATE)).lw(c);
            c.g[V0][0] = 0;
            if state == 0 {
                c.cycles += 4;
                let handle = Addr(frame.at(c, 4)).lw(c);
                let bytes = Addr((handle as u32).wrapping_add(SFILE_HANDLE_CHANNEL_BYTES)).lw(c);
                c.g[V1][0] = bytes;
                let idle = (bytes == 0) as u64;
                c.g[A0][0] = idle;
                c.g[V0][0] = idle;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_AllocRequest (0x3dfcd8)

/// The stream-file request serial counter (`0x520000 - 0x63e8`); bumped by
/// 0x100 per request, never 0, kept in the request's tag word above the
/// low byte.
pub(crate) const STREAM_FILE_REQUEST_SERIAL: u32 = 0x0051_9c18;

pub(crate) const STREAM_FILE_FREE_REQUESTS: u32 = 0x68;

/// Request field: the next request in the free list.
pub(crate) const STREAM_FILE_REQUEST_NEXT: u32 = 0xc;

/// `StreamFileRequest* StreamFile_AllocRequest(StreamFile* sf)` (0x3dfcd8):
/// under the stream file's mutex, pops a request off the free list, gives it
/// the next serial (`serial += 0x100`, skipping 0) in its tag word (keeping
/// the low byte); returns it, or null when the list is empty.
pub fn stream_file_alloc_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(STREAM_FILE_MUTEX));
        Callee {
            addr: 0x003e_5700,
            f: ee::s14::EAMutex_Lock,
        }
        .call(c, 0x003d_fcf4)?;
        let sf = r32(c, S0);
        c.g[V0][0] = Addr(sf + STREAM_FILE_FREE_REQUESTS).lw(c);
        c.g[S1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            let req = c.g[V0][0];
            c.g[S1][0] = req;
            c.g[A0][0] = 0x52_0000;
            let serial = Addr(STREAM_FILE_REQUEST_SERIAL).lw(c);
            let next = Addr((req as u32).wrapping_add(STREAM_FILE_REQUEST_NEXT)).lw(c);
            c.g[V1][0] = next;
            let serial = sx((serial as u32).wrapping_add(0x100));
            c.g[V0][0] = serial;
            Addr(sf + STREAM_FILE_FREE_REQUESTS).sw(c, next);
            Addr(STREAM_FILE_REQUEST_SERIAL).sw(c, serial);
            if serial == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
                Addr(STREAM_FILE_REQUEST_SERIAL).sw(c, 0x100);
            }
            c.cycles += 4;
            let serial = Addr(STREAM_FILE_REQUEST_SERIAL).lw(c);
            c.g[V1][0] = serial;
            let tag = Addr(req as u32).lbu(c) | serial;
            c.g[V0][0] = tag;
            Addr(req as u32).sw(c, tag);
        }
        c.cycles += 8;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(STREAM_FILE_MUTEX));
        Callee {
            addr: 0x003e_5760,
            f: ee::s14::EAMutex_Unlock,
        }
        .call(c, 0x003d_fd40)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_FreeRequest (0x3dfe18)

/// Stream file: the request list's head (+0x5c), current (+0x60) and tail
/// (+0x64), and the free-request list (+0x68).
pub(crate) const STREAM_FILE_FREE_REQUEST_HEAD: u32 = 0x5c;

pub(crate) const STREAM_FILE_FREE_REQUEST_CURRENT: u32 = 0x60;

pub(crate) const STREAM_FILE_FREE_REQUEST_TAIL: u32 = 0x64;

pub(crate) const STREAM_FILE_FREE_REQUEST_FREE: u32 = 0x68;

/// Request: in-use word (+4), previous (+8) and next (+0xc) links.
pub(crate) const STREAM_FILE_FREE_REQUEST_BUSY: u32 = 0x4;

pub(crate) const STREAM_FILE_FREE_REQUEST_PREV: u32 = 0x8;

pub(crate) const STREAM_FILE_FREE_REQUEST_NEXT: u32 = 0xc;

/// `void StreamFile_FreeRequest(StreamFile* file, Request* req)`
/// (0x3dfe18): unlinks `req` from the file's doubly linked request list
/// (fixing head / tail, and moving the current request to its neighbour,
/// next first), marks it unused and pushes it on the free list.
pub fn stream_file_free_request(c: &mut Cpu) {
    const PREV: u32 = STREAM_FILE_FREE_REQUEST_PREV;
    const NEXT: u32 = STREAM_FILE_FREE_REQUEST_NEXT;
    let file = Addr(c.g[A0][0] as u32);
    let req_word = c.g[A1][0];
    let req = Addr(req_word as u32);

    // prev->next = next (or head = next).
    c.cycles += 3;
    c.g[V0][0] = file.off(STREAM_FILE_FREE_REQUEST_HEAD).lw(c);
    if req_word != c.g[V0][0] {
        c.g[V1][0] = req.off(PREV).lw(c);
        c.cycles += 2;
        c.g[V0][0] = req.off(NEXT).lw(c);
        let next = c.g[V0][0];
        Addr(c.g[V1][0] as u32).off(NEXT).sw(c, next);
    } else {
        c.cycles += 3;
        c.g[V0][0] = req.off(NEXT).lw(c);
        let next = c.g[V0][0];
        file.off(STREAM_FILE_FREE_REQUEST_HEAD).sw(c, next);
    }

    // next->prev = prev (or tail = prev).
    c.cycles += 3;
    c.g[V0][0] = file.off(STREAM_FILE_FREE_REQUEST_TAIL).lw(c);
    if req_word != c.g[V0][0] {
        c.g[V1][0] = req.off(NEXT).lw(c);
        c.cycles += 2;
        c.g[V0][0] = req.off(PREV).lw(c);
        let prev = c.g[V0][0];
        Addr(c.g[V1][0] as u32).off(PREV).sw(c, prev);
    } else {
        c.cycles += 3;
        c.g[V0][0] = req.off(PREV).lw(c);
        let prev = c.g[V0][0];
        file.off(STREAM_FILE_FREE_REQUEST_TAIL).sw(c, prev);
    }

    // The current request moves to the next one, else the previous one.
    c.cycles += 3;
    c.g[V0][0] = file.off(STREAM_FILE_FREE_REQUEST_CURRENT).lw(c);
    if req_word == c.g[V0][0] {
        c.cycles += 3;
        c.g[V0][0] = req.off(NEXT).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = req.off(PREV).lw(c);
        }
        c.cycles += 2;
        let current = c.g[V0][0];
        file.off(STREAM_FILE_FREE_REQUEST_CURRENT).sw(c, current);
    }
    req.off(STREAM_FILE_FREE_REQUEST_BUSY).sw(c, 0);

    // Push on the free list.
    c.cycles += 4;
    c.g[V0][0] = file.off(STREAM_FILE_FREE_REQUEST_FREE).lw(c);
    let free = c.g[V0][0];
    req.off(NEXT).sw(c, free);
    file.off(STREAM_FILE_FREE_REQUEST_FREE).sw(c, req_word);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// ComputerActorRider_RollSkipTricks (0x10c1d0)

/// Computer actor: the chance (0..1) of doing tricks (+0xdfc).
pub(crate) const ROLL_SKIP_TRICKS_CHANCE: u32 = 0xdfc;

/// `bool ComputerActorRider_RollSkipTricks(ComputerActorRider* self)`
/// (0x10c1d0): rolls a game random number in 0..99 and returns whether
/// `chance * 100 <= roll`, i.e. whether the AI skips its tricks this time.
pub fn computer_actor_rider_roll_skip_tricks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0031_7810, 0x0010_c1e4)?; // Rand_NextGame
        c.g[V1][0] = 100;
        let chance = Addr(r32(c, S0)).off(ROLL_SKIP_TRICKS_CHANCE).read_fpu(c);
        let rand = r32(c, V0);
        let (_, roll) = abi::divu(c, rand, 100);
        c.g[AT][0] = 0x42c8_0000;
        let hundred = FpuF32(0x42c8_0000);

        c.cycles += 3;
        c.g[V0][0] = sx(roll);
        let threshold = chance * hundred;
        let rolled = if (c.g[V0][0] as i64) < 0 {
            // Unsigned to float (never taken: the remainder is below 100).
            c.cycles += 6;
            let v = c.g[V0][0];
            c.g[V1][0] = sx((v as u32) >> 1);
            c.g[V0][0] = (v & 1) | c.g[V1][0];
            let half = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
            half + half
        } else {
            c.cycles += 4;
            FpuF32::from_i32(roll as i32)
        };
        abi::set_fpr(c, 1, threshold);
        abi::set_fpr(c, 0, rolled);

        c.cycles += 4;
        let skip = threshold.le(rolled);
        abi::set_fcc(c, skip);
        c.g[V0][0] = 1;
        if !skip {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_ParticleControl (0x302490)

/// gp+: the static-defaults guard; the argument type table; the handle
/// tables; the world.
pub(crate) const PARTICLE_CONTROL_INIT_GUARD: u32 = 0x2644;

pub(crate) const PARTICLE_CONTROL_ARG_TYPES: u32 = 0xbd8;

pub(crate) const PARTICLE_CONTROL_HANDLE_TABLES: u32 = 0x16c8;

pub(crate) const PARTICLE_CONTROL_WORLD: u32 = 0xce8;

/// The static default argument buffer (3 words: handle, mode, tag byte).
pub(crate) const PARTICLE_CONTROL_DEFAULTS: u32 = 0x004f_bc30;

/// The static "no value" result (0x4d0000 - 0x6f68).
#[cfg(test)]
pub(crate) const PARTICLE_CONTROL_RESULT_NONE: u32 = 0x004c_9098;

/// `WScriptResult WScriptFn_ParticleControl(int argc, WScriptArg* args)`
/// (0x302490, hidden result pointer in a0): the world-script command that
/// resets (mode 0) or prunes (mode 1) the particles of an object's
/// modifier block. The arguments (16 bytes each: slot index, value, -,
/// type) fill a 3-word buffer initialised from a static default (`{-1, -1,
/// tag}`, set up on first use); an int given for a float slot is
/// converted. `buf[0]` is the object handle (-1: the world's default
/// object); the object's model (`+0xc`) must answer its "has particles"
/// virtual (slot 0x80) before its modifier block (`+0x1c`) is touched.
/// Returns the static "no value" result.
pub fn wscript_fn_particle_control(c: &mut Cpu) {
    const DEFAULT_OBJECT: u32 = 0x290;
    const MODEL: u32 = 0xc;
    const MODIFIERS: u32 = 0x1c;
    const FLOAT_ARG: u64 = 2;
    const LOOP_HEAD: u32 = 0x0030_2520;
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(gp.wrapping_add(PARTICLE_CONTROL_INIT_GUARD)).lw(c);
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A2][0]; // args
        frame.save(c, S3, 0x30);
        c.g[S0][0] = c.g[A1][0]; // argc
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A0][0]; // result
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x0050_0000;
        if c.g[V0][0] == 0 {
            // Guarded static init of the default argument buffer.
            c.cycles += 11;
            c.g[V1][0] = sx(PARTICLE_CONTROL_DEFAULTS);
            c.g[V0][0] = u64::MAX;
            let tag = Addr(PARTICLE_CONTROL_DEFAULTS + 8).lbu(c);
            c.g[A1][0] = tag;
            Addr(PARTICLE_CONTROL_DEFAULTS).sw(c, u64::MAX);
            c.g[A0][0] = u64::MAX;
            Addr(PARTICLE_CONTROL_DEFAULTS + 8).sb(c, tag);
            c.g[V0][0] = 1;
            Addr(PARTICLE_CONTROL_DEFAULTS + 4).sw(c, u64::MAX);
            Addr(gp.wrapping_add(PARTICLE_CONTROL_INIT_GUARD)).sw(c, 1);
            Addr(sp(c)).sb(c, tag);
        }

        // buf = defaults (buf[0] = -1 first).
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let buf = sp(c).wrapping_add(0x10);
        c.g[S1][0] = sx(buf);
        c.g[A1][0] = sx(PARTICLE_CONTROL_DEFAULTS);
        c.g[A2][0] = 0xc;
        Addr(buf).sw(c, u64::MAX);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003e_6574, 0x0030_250c)?; // memcpy
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(gp.wrapping_add(PARTICLE_CONTROL_ARG_TYPES));
            c.g[A3][0] = FLOAT_ARG;
            loop {
                c.cycles += 8;
                let arg = Addr(c.g[A2][0] as u32);
                let slot = arg.lw(c);
                c.g[V1][0] = arg.off(0xc).lw(c);
                let off = (slot as u32) << 2;
                c.g[A0][0] = sx(off.wrapping_add(c.g[S1][0] as u32));
                let ty_addr = off.wrapping_add(c.g[T0][0] as u32);
                c.g[V0][0] = sx(ty_addr);
                c.g[V0][0] = Addr(ty_addr).lw(c);
                let dst = c.g[A0][0] as u32;
                let convert = if c.g[V1][0] == c.g[V0][0] {
                    false
                } else {
                    c.cycles += 2;
                    c.g[V0][0] == c.g[A3][0]
                };
                if convert {
                    c.cycles += 4;
                    let v = FpuF32::from_i32(arg.off(4).lw(c) as i32);
                    abi::set_fpr(c, 0, v);
                    Addr(dst).write_fpu(c, v);
                } else {
                    let v = arg.off(4).lw(c);
                    c.g[V0][0] = v;
                    c.cycles += 1;
                    Addr(dst).sw(c, v);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if c.g[A1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOOP_HEAD);
                }
            }
        }

        // Resolve the object handle.
        c.cycles += 4;
        let buf = Addr(c.g[S1][0] as u32);
        let id = buf.lw(c);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        c.g[A1][0] = Addr(gp.wrapping_add(PARTICLE_CONTROL_HANDLE_TABLES)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            let world = Addr(gp.wrapping_add(PARTICLE_CONTROL_WORLD)).lw(c);
            c.g[V0][0] = world;
            c.g[S0][0] = Addr(world as u32).off(DEFAULT_OBJECT).lw(c);
        } else {
            c.cycles += 8;
            let tables = Addr(c.g[A1][0] as u32).lw(c);
            c.g[V1][0] = tables;
            let sets = Addr(tables as u32).off(8).lw(c);
            c.g[A0][0] = sets;
            let set_addr = ((id as u32 & 0xff) << 2).wrapping_add(sets as u32);
            c.g[V0][0] = sx(set_addr);
            let set = Addr(set_addr).lw(c);
            c.g[V1][0] = set;
            c.g[V0][0] = sx((id as u32) >> 8);
            let mut found = false;
            if set != 0 {
                c.cycles += 7;
                let entries = Addr(set as u32).off(0x1c).lw(c);
                c.g[A0][0] = entries;
                let entry_addr = ((c.g[V0][0] as u32) << 2).wrapping_add(entries as u32);
                c.g[V0][0] = sx(entry_addr);
                let entry = Addr(entry_addr).lw(c);
                c.g[V1][0] = entry;
                let index = (entry as u32) >> 8;
                c.g[V0][0] = sx(index << 2);
                found = index != 0;
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[V0][0];
        }

        // The object's model, its "has particles" virtual, the mode.
        c.cycles += 2;
        c.g[V1][0] = 0x004d_0000;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = Addr(r32(c, S0)).off(MODEL).lw(c);
            c.g[S1][0] = Addr(r32(c, S1)).off(4).lw(c); // mode
            if c.g[A1][0] != 0 {
                c.cycles += 7;
                let model = r32(c, A1);
                let vt = Addr(model).off(0xc).lw(c);
                c.g[V1][0] = vt;
                let delta = Addr(vt as u32).off(0x80).lh(c);
                let target = Addr(vt as u32).off(0x84).lw(c);
                c.g[V0][0] = target;
                c.g[A0][0] = sx(model.wrapping_add(delta as u32));
                call_ptr(c, target as u32, 0x0030_25f0)?;
                let has_particles = c.g[V0][0] != 0;
                c.g[V1][0] = 0x004d_0000;
                if has_particles {
                    c.cycles += 2;
                    let mode = c.g[S1][0];
                    c.g[V1][0] = Addr(r32(c, S0)).off(MODEL).lw(c);
                    let call = if mode == 0 {
                        c.cycles += 3;
                        c.g[A0][0] = Addr(r32(c, V1)).off(MODIFIERS).lw(c);
                        Some((0x0035_3228, 0x0030_2628, 4)) // ModifierBlock_ResetParticles
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        if mode == 1 {
                            c.g[A0][0] = Addr(r32(c, V1)).off(MODIFIERS).lw(c);
                            c.cycles += 2;
                            Some((0x0035_3278, 0x0030_2640, 3)) // ModifierBlock_RemoveDeadParticles
                        } else {
                            c.cycles += 2;
                            None
                        }
                    };
                    c.g[V1][0] = 0x004d_0000;
                    if let Some((f, ret_to, cost)) = call {
                        if c.g[A0][0] != 0 {
                            c.cycles += cost;
                            call_ptr(c, f, ret_to)?;
                            c.g[V1][0] = 0x004d_0000;
                        }
                    }
                }
            }
        }

        // *result = the static "no value" result (12 bytes, unaligned copy).
        c.cycles += 15;
        let result = c.g[S3][0] as u32;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let src = c.g[A3][0] as u32;
        let head = Addr(src).read_u64_unaligned(c);
        c.g[A0][0] = head;
        let tail = Addr(src).off(8).lw(c);
        c.g[A1][0] = tail;
        Addr(result).write_u64_unaligned(c, head);
        Addr(result).off(8).sw(c, tail);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// Rider ability flags; bit 0: uber tricks unlocked.
pub(crate) const CAR_RIDER_ABILITY_FLAGS: u32 = 0xb2c;

/// The AI rider's trick skill (0..1 float) in the computer actor.
pub(crate) const CAR_TRICK_SKILL: u32 = 0xdfc;

/// `BENewRaceInt_GetTrack` result that forbids uber tricks.
pub(crate) const CAR_NO_UBER_TRACK: u64 = 0xe;

/// 100.0f.
pub(crate) const CAR_HUNDRED: u32 = 0x42c8_0000;

/// `bool ComputerActorRider_RollUberTrick(ComputerActorRider* self)`
/// (0x10c258): whether the AI rider tries an uber trick now. Only riders
/// with uber tricks unlocked (rider flags bit 0), never on track 0xe
/// (`BENewRaceInt_GetTrack` of the race interface); then a dice roll:
/// `Rand_NextGame() % 100 < chance * 100`.
pub fn computer_actor_rider_roll_uber_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let me = c.g[S0][0] as u32;
        c.g[V1][0] = Addr(me.wrapping_add(CAR_RIDER)).lw(c);
        let flags = Addr((c.g[V1][0] as u32).wrapping_add(CAR_RIDER_ABILITY_FLAGS)).lw(c);
        c.g[V0][0] = 0;
        if flags & 1 != 0 {
            // BELibrary_GetInterface(BELibrary_GetInstance(), 0) -> GetTrack.
            c.cycles += 10;
            call_ptr(c, 0x0014_dc80, 0x0010_c284)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x0010_c290)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_4be0, 0x0010_c298)?;
            c.g[V1][0] = CAR_NO_UBER_TRACK;
            let track = c.g[V0][0];
            c.g[V0][0] = 0;
            if track != CAR_NO_UBER_TRACK {
                car_roll_against_chance(c)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The dice roll of `ComputerActorRider_RollUberTrick` (0x10c2a4..):
/// `v0 = (float)(Rand_NextGame() % 100u) < self->chance * 100.0f`.
pub(crate) fn car_roll_against_chance(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 9;
    call_ptr(c, 0x0031_7810, 0x0010_c2ac)?;
    c.g[V1][0] = 100;
    let me = c.g[S0][0] as u32;
    let chance = Addr(me.wrapping_add(CAR_TRICK_SKILL)).read_fpu(c);
    c.f[1] = chance.0;
    let roll_raw = c.g[V0][0] as u32;
    abi::divu(c, roll_raw, 100);
    c.g[AT][0] = CAR_HUNDRED as u64;
    let hundred = FpuF32(CAR_HUNDRED);
    c.f[0] = hundred.0;

    c.cycles += 3;
    c.g[V0][0] = c.hi[0];
    let threshold = chance * hundred;
    c.f[1] = threshold.0;
    let roll = car_u32_to_float(c);
    c.f[0] = roll.0;

    c.cycles += 4;
    let hit = abi::c_lt_s(c, roll, threshold);
    c.g[V0][0] = 1;
    if !hit {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    Ok(())
}

/// gcc's `(float)(unsigned)v0` (the trick rolls): `cvt.s.w` directly, or for
/// values >= 2^31 halve (keeping the low bit), convert and double. Leaves
/// v0/v1 as the original does.
pub(crate) fn car_u32_to_float(c: &mut Cpu) -> FpuF32 {
    if (c.g[V0][0] as i64) < 0 {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[V0][0] = (c.g[V0][0] & 1) | c.g[V1][0];
        let half = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
        half + half
    } else {
        c.cycles += 4;
        FpuF32::from_i32(c.g[V0][0] as u32 as i32)
    }
}

// ---------------------------------------------------------------------------
// ComputerActorRider_RollFittedTrick (0x10c320)

/// 70.0f and 30.0f.
pub(crate) const CAR_SEVENTY: u32 = 0x428c_0000;

pub(crate) const CAR_THIRTY: u32 = 0x41f0_0000;

/// `bool ComputerActorRider_RollFittedTrick(ComputerActorRider* self)`
/// (0x10c320): whether the AI rider picks a trick fitted to the jump: a
/// dice roll `Rand_NextGame() % 100 < chance * 70 + 30` (so at least 30%).
pub fn computer_actor_rider_roll_fitted_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0031_7810, 0x0010_c334)?;
        c.g[V1][0] = 100;
        let me = c.g[S0][0] as u32;
        let chance = Addr(me.wrapping_add(CAR_TRICK_SKILL)).read_fpu(c);
        c.f[0] = chance.0;
        let roll_raw = c.g[V0][0] as u32;
        abi::divu(c, roll_raw, 100);
        let (seventy, thirty) = (FpuF32(CAR_SEVENTY), FpuF32(CAR_THIRTY));
        c.f[1] = seventy.0;
        c.g[AT][0] = CAR_THIRTY as u64;
        c.f[2] = thirty.0;

        c.cycles += 4;
        let scaled = chance * seventy;
        c.g[V0][0] = c.hi[0];
        let threshold = scaled + thirty;
        c.f[0] = threshold.0;
        let roll = car_u32_to_float(c);
        c.f[1] = roll.0;

        c.cycles += 4;
        let hit = abi::c_lt_s(c, roll, threshold);
        c.g[V0][0] = 1;
        if !hit {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ComputerActorRider_RollRandomTrick (0x10c3b8)

/// 40.0f and 60.0f (probability percentages).
const CAR_FORTY: u32 = 0x4220_0000;
const CAR_SIXTY: u32 = 0x4270_0000;

/// `bool ComputerActorRider_RollRandomTrick(ComputerActorRider* self)`
/// (0x10c3b8): whether the AI rider picks a random trick: a
/// dice roll `Rand_NextGame() % 100 < chance * 40 + 60` (so at least 60%).
/// These constants are percentages, not time steps.
pub fn computer_actor_rider_roll_random_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0031_7810, 0x0010_c3cc)?;
        c.g[V1][0] = 100;
        let me = c.g[S0][0] as u32;
        let chance = Addr(me.wrapping_add(CAR_TRICK_SKILL)).read_fpu(c);
        c.f[0] = chance.0;
        let roll_raw = c.g[V0][0] as u32;
        abi::divu(c, roll_raw, 100);
        let (forty, sixty) = (FpuF32(CAR_FORTY), FpuF32(CAR_SIXTY));
        c.f[1] = forty.0;
        c.g[AT][0] = CAR_SIXTY as u64;
        c.f[2] = sixty.0;

        c.cycles += 4;
        let scaled = chance * forty;
        c.g[V0][0] = c.hi[0];
        let threshold = scaled + sixty;
        c.f[0] = threshold.0;
        let roll = car_u32_to_float(c);
        c.f[1] = roll.0;

        c.cycles += 4;
        let hit = abi::c_lt_s(c, roll, threshold);
        c.g[V0][0] = 1;
        if !hit {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_EnqueueRequest (0x3dfd58)

/// StreamFile: its mutex (+4) and the request list: head (+0x5c), current
/// (+0x60) and tail (+0x64).
pub(crate) const SFQ_MUTEX: u32 = 0x4;

pub(crate) const SFQ_HEAD: u32 = 0x5c;

pub(crate) const SFQ_CURRENT: u32 = 0x60;

pub(crate) const SFQ_TAIL: u32 = 0x64;

/// Request: state (+4), previous (+8), next (+0xc).
pub(crate) const SFQ_REQ_STATE: u32 = 0x4;

pub(crate) const SFQ_REQ_PREV: u32 = 0x8;

pub(crate) const SFQ_REQ_NEXT: u32 = 0xc;

pub(crate) const SFQ_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};

pub(crate) const SFQ_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

/// `void StreamFile_EnqueueRequest(StreamFile* sf, Request* req)`
/// (0x3dfd58): mark the request queued (state 1, no next) and append it to
/// the stream file's doubly linked request list under the file's mutex. An
/// empty list makes it head, current and tail at once.
pub fn stream_file_enqueue_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let (sf, req) = (Addr(c.g[S1][0] as u32), Addr(c.g[S0][0] as u32));
        req.off(SFQ_REQ_NEXT).sw(c, 0);
        c.g[A0][0] = sx(sf.off(SFQ_MUTEX).0);
        req.off(SFQ_REQ_STATE).sw(c, 1);
        SFQ_MUTEX_LOCK.call(c, 0x003d_fd84)?;

        let (sf, req) = (Addr(c.g[S1][0] as u32), Addr(c.g[S0][0] as u32));
        c.g[V0][0] = sf.off(SFQ_TAIL).lw(c);
        if c.g[V0][0] != 0 {
            // Append after the tail.
            let tail = c.g[V0][0];
            req.off(SFQ_REQ_PREV).sw(c, tail);
            c.cycles += 3;
            c.g[V0][0] = sf.off(SFQ_TAIL).lw(c);
            Addr((c.g[V0][0] as u32).wrapping_add(SFQ_REQ_NEXT)).sw(c, req.0 as u64);
            sf.off(SFQ_TAIL).sw(c, req.0 as u64);
        } else {
            c.cycles += 5;
            req.off(SFQ_REQ_PREV).sw(c, 0);
            sf.off(SFQ_TAIL).sw(c, req.0 as u64);
            sf.off(SFQ_HEAD).sw(c, req.0 as u64);
            sf.off(SFQ_CURRENT).sw(c, req.0 as u64);
        }

        c.cycles += 7;
        c.g[A0][0] = sx(sf.off(SFQ_MUTEX).0);
        SFQ_MUTEX_UNLOCK.call(c, 0x003d_fdb8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_StartObjectLoop (0x3000a8)

/// `gp` word: the static parameter defaults are initialised.
pub(crate) const SOL_DEFAULTS_READY_GP: u32 = 0x2578;

/// The static parameter defaults (object id, sound, volume?, tag byte at
/// +0xc; 0x4fbb18), copied as a 16-byte block.
pub(crate) const SOL_DEFAULTS: u32 = 0x004f_bb18;

/// The kind of each parameter slot (2 = float).
pub(crate) const SOL_PARAM_KINDS: u32 = 0x0044_6550;

pub(crate) const SOL_KIND_FLOAT: u64 = 2;

/// `gp` words: the object directory, the world (its default object at
/// `+0x290`).
pub(crate) const SOL_DIRECTORY_GP: u32 = 0x16c8;

pub(crate) const SOL_WORLD_GP: u32 = 0xce8;

pub(crate) const SOL_WORLD_DEFAULT_OBJECT: u32 = 0x290;

/// The script functions' shared result value (12 bytes at 0x4c9098).
pub(crate) const SOL_RESULT: u32 = 0x004c_9098;

/// `Result WScriptFn_StartObjectLoop(Param* params, int count)` (0x3000a8;
/// a0 = the hidden result slot, a1 = count, a2 = params): fills a 4-word
/// parameter block (object id, sound, arg) from the defaults and the
/// `{slot, value, -, kind}` parameters (an int given for a float slot is
/// converted), looks the object up (id -1: the world's default object) and,
/// if there is one, starts the looping sound on it
/// (`SSXAudioSystem_StartObjectLoop(GetInstance(), obj, sound, arg)`).
/// Returns the shared script result.
pub fn wscript_fn_start_object_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let gp = r32(c, GP);
        let ready = Addr(gp).off(SOL_DEFAULTS_READY_GP).lw(c);
        c.g[V0][0] = ready;
        frame.save(c, S1, 0x50);
        c.g[A3][0] = 0x50_0000;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let defaults = Addr(SOL_DEFAULTS);
        if ready == 0 {
            // First use: initialise the static defaults (guarded local).
            c.cycles += 12;
            c.g[V1][0] = sx(SOL_DEFAULTS);
            c.g[V0][0] = u64::MAX;
            let tag = defaults.off(0xc).lbu(c);
            c.g[A2][0] = tag;
            defaults.sw(c, u64::MAX);
            c.g[A0][0] = u64::MAX;
            defaults.off(0xc).sb(c, tag);
            c.g[A1][0] = 1;
            defaults.off(4).sw(c, u64::MAX);
            defaults.off(8).sw(c, u64::MAX);
            Addr(gp).off(SOL_DEFAULTS_READY_GP).sw(c, 1);
            Addr(sp(c)).sb(c, tag);
        }
        // The parameter block at 0x10(sp), copied from the defaults.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let block = frame.at(c, 0x10);
        c.g[S0][0] = sx(block);
        c.g[A2][0] = 0x10;
        c.g[A1][0] = sx(SOL_DEFAULTS);
        Addr(block).sw(c, u64::MAX);
        c.g[A0][0] = sx(block);
        call_ptr(c, 0x003e_6574, 0x0030_0128)?; // memcpy
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(SOL_PARAM_KINDS);
            c.g[A3][0] = SOL_KIND_FLOAT;
            loop {
                c.cycles += 8;
                let param = Addr(r32(c, A2));
                let slot4 = (param.lw(c) as u32) << 2;
                let given_kind = param.off(0xc).lw(c);
                c.g[V1][0] = given_kind;
                let dest = Addr(slot4.wrapping_add(r32(c, S0)));
                c.g[A0][0] = sx(dest.0);
                let kind = Addr(slot4.wrapping_add(r32(c, T0))).lw(c);
                c.g[V0][0] = kind;
                if given_kind != kind && kind == c.g[A3][0] {
                    // An int for a float slot.
                    c.cycles += 2 + 4;
                    let value = param.off(4).lw(c) as u32;
                    let converted = FpuF32::from_i32(value as i32);
                    c.f[0] = converted.0;
                    dest.write_fpu(c, converted);
                } else {
                    c.cycles += if given_kind == kind { 1 } else { 2 + 1 };
                    let value = param.off(4).lw(c);
                    c.g[V0][0] = value;
                    dest.sw(c, value);
                }
                c.cycles += 3;
                let left = r32(c, A1).wrapping_sub(1);
                c.g[A1][0] = sx(left);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if left == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_0140);
                }
            }
        }
        // Look the object up.
        c.cycles += 4;
        let id = Addr(r32(c, S0)).lw(c);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        let directory = Addr(r32(c, GP)).off(SOL_DIRECTORY_GP).lw(c);
        c.g[A1][0] = directory;
        if !id == 0 {
            c.cycles += 3;
            let world = Addr(r32(c, GP)).off(SOL_WORLD_GP).lw(c);
            c.g[V0][0] = world;
            c.g[S2][0] = Addr(world as u32).off(SOL_WORLD_DEFAULT_OBJECT).lw(c);
        } else {
            c.cycles += 8;
            let id = id as u32;
            c.g[V0][0] = sx((id & 0xff) << 2);
            let sections = Addr(directory as u32).lw(c);
            c.g[V1][0] = sections;
            let table = Addr(sections as u32).off(8).lw(c);
            c.g[A0][0] = table;
            let at = ((id & 0xff) << 2).wrapping_add(table as u32);
            c.g[V0][0] = sx(at);
            let section = Addr(at).lw(c);
            c.g[V1][0] = section;
            let object = if section != 0 {
                c.cycles += 7;
                let entries = Addr(section as u32).off(0x1c).lw(c);
                c.g[A0][0] = entries;
                let entry = Addr(((id >> 8) << 2).wrapping_add(entries as u32)).lw(c);
                c.g[V1][0] = entry;
                let index = (entry as u32) >> 8;
                let object = if index != 0 {
                    sx(index << 2)
                } else {
                    c.cycles += 1;
                    0
                };
                c.g[V0][0] = object;
                c.cycles += 1;
                object
            } else {
                c.cycles += 2;
                0
            };
            c.g[A1][0] = object;
            c.cycles += 1;
            c.g[S2][0] = object;
        }
        c.cycles += 2;
        c.g[V1][0] = 0x4d_0000;
        if c.g[S2][0] != 0 {
            // SSXAudioSystem_StartObjectLoop(GetInstance(), obj, sound, arg).
            c.cycles += 9;
            c.g[S1][0] = Addr(r32(c, S0)).off(8).lw(c);
            c.g[S0][0] = Addr(r32(c, S0)).off(4).lw(c);
            call_ptr(c, 0x0028_b180, 0x0030_0208)?; // SSXAudioSystem_GetInstance
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S1][0];
            call_ptr(c, 0x0029_7950, 0x0030_021c)?; // SSXAudioSystem_StartObjectLoop
            c.g[V1][0] = 0x4d_0000;
        }
        // *result = the shared script result (12 bytes).
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(SOL_RESULT);
        let out = r32(c, S3);
        c.ldl(4, SOL_RESULT.wrapping_add(0x7));
        c.ldr(4, SOL_RESULT);
        c.g[A1][0] = Addr(SOL_RESULT).off(8).lw(c);
        c.sdl(4, out.wrapping_add(0x7));
        c.sdr(4, out);
        let tail = c.g[A1][0];
        Addr(out).off(8).sw(c, tail);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// isnan (0x40f840)

/// `int isnan(double x)` (0x40f840): the libc test, `(hx & 0x7fffffff) |
/// (lx | -lx) >> 31 > 0x7ff00000`, with `hx`/`lx` the two halves of `a0`
/// (the double is passed in one 64-bit GPR).
pub fn isnan(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V0][0] = c.g[A0][0] << 32;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64; // lx
    c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64; // hx
    c.g[V1][0] = 0x7fff_0000;
    c.g[A1][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
    c.g[V1][0] |= 0xffff;
    c.g[V0][0] |= c.g[A1][0];
    c.g[A0][0] &= c.g[V1][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
    c.g[V1][0] = 0x7ff0_0000;
    c.g[A0][0] |= c.g[V0][0];
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V0][0] = sx((c.g[A0][0] as u32) >> 31);
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::difftest::check;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_COUNT;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::main::ssxapp::tests::f;
    pub(crate) use crate::game::visualfx::lensfx::tests::painter_sun;
    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::cpu::sx;
    use ssx3_ee_rt::Cpu;

    // ----------------------------------------------------------------- sun

    /// Bit n set: fake call n returns 0 (for the off-screen test).
    pub(crate) const ZERO_RESULTS: u32 = fakes::LOG + 0x34;

    pub(crate) fn fake_maybe_zero(c: &mut Cpu, target: u32) {
        fake_call(c, target);
        let n = c.bus.read32(LOG_COUNT).wrapping_sub(1);
        if n < 32 && c.bus.read32(ZERO_RESULTS) >> n & 1 != 0 {
            c.g[V0][0] = 0;
        }
    }

    /// `LensFX_UpdateSunPosition` input. Fake calls: 0 the sun texture
    /// (-1 for a quarter of the seeds), 1 the azimuth, 2 the elevation
    /// (each 0.0 for some seeds: the `.lit4` default angles), 3 the
    /// off-screen test (0 = on screen for half the seeds), 4 the
    /// projection. `Math_SinCos` runs for real.
    pub(crate) fn sun_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x4c46_5853, 5);
        c.hooks.miss = fake_maybe_zero;
        let fx = DATA + 0x1000;
        let cam = DATA + 0x2000;
        let gfx = DATA + 0x3000;
        let results = DATA + 0x100;
        // Painter views 6 and 7 share one sun object.
        for view in [6, 7] {
            painter_sun(
                c,
                &mut r,
                view,
                DATA + 0x200,
                DATA + 0x6000,
                &[0xd8, 0xe0, 0x108],
            );
        }
        // The projection interface.
        c.bus.write32(GAME_GP + 0x2a90, gfx);
        c.bus.write32(gfx + 0x10d8, DATA + 0x7000);
        for (i, off) in [0x2d8u32, 0x2e0].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 % 64) & !3;
            c.bus.write16(DATA + 0x7000 + off, delta as u16);
            c.bus
                .write32(DATA + 0x7000 + off + 4, FAKE + 0x40 + 0x10 * i as u32);
        }
        // Forced fake results: pointers to the texture / azimuth /
        // elevation words.
        let texture = if seed % 4 == 0 {
            u32::MAX
        } else {
            r.next_u32() % 8
        };
        c.bus.write32(results, texture);
        let az = if seed % 3 == 0 { 0 } else { f(&mut r, 360.0) };
        let el = if seed % 5 < 2 { 0 } else { f(&mut r, 90.0) };
        c.bus.write32(results + 4, az);
        c.bus.write32(results + 8, el);
        for (n, p) in [results, results + 4, results + 8].into_iter().enumerate() {
            c.bus.write32(fakes::LOG_RESULTS + 4 * n as u32, p);
        }
        c.bus
            .write32(ZERO_RESULTS, if seed % 2 == 0 { 1 << 3 } else { 0 });
        // `.lit4`: degrees to radians, the default angles.
        c.bus
            .write32(GAME_GP - 0x38c0, (std::f32::consts::PI / 180.0).to_bits());
        c.bus.write32(GAME_GP - 0x38bc, f(&mut r, 3.0));
        c.bus.write32(GAME_GP - 0x38b8, f(&mut r, 1.5));
        // The camera: viewport, view rows, translation, distance.
        for i in 0..4 {
            c.bus.write32(cam + 4 * i, f(&mut r, 640.0));
        }
        for i in 4..0x16 {
            c.bus.write32(cam + 4 * i, f(&mut r, 2.0));
        }
        c.bus.write32(cam + 0x40, f(&mut r, 1000.0));
        c.bus.write32(cam + 0x44, f(&mut r, 1000.0));
        c.bus.write32(cam + 0x48, f(&mut r, 1000.0));
        c.bus.write32(cam + 0x58, f(&mut r, 3000.0));
        fill(c, &mut r, fx, 16);
        fill(c, &mut r, fx + 0x1f0, 8);
        c.g[A0][0] = sx(fx);
        c.g[A1][0] = sx(cam);
        c.g[A2][0] = if seed % 7 < 3 {
            0
        } else {
            r.next_u32() as u64 | 1
        };
    }

    #[test]
    fn lens_fx_update_sun_position_matches() {
        covered(
            ssx3_ee::s09::f_002f4db8__generated,
            super::lens_fx_update_sun_position,
            300,
            &|c, seed| sun_setup(c, seed),
        );
    }
    use super::*;

    /// `HUD_DrawMeterBar`: a HUD object whose sprite table (+0x4fc, style
    /// 0..3) points at three of four sprites with random UVs; layout
    /// elements -1..4 (the element byte comes with junk upper bits) with
    /// small positions, heights 8..300 (sometimes negative: no texture, so
    /// 0), scales 0.25..2 and random align/blend bytes; segment scale
    /// 0.5..1.5 and count -1..5; colour, align flags, an out rect on 3 of 4
    /// seeds; fill (f12) exactly 1 (full bar) on every third seed, else
    /// 0..1.1 (the partial bar's caps and segments clipped or not, the
    /// segment loop ending at the fill line or at the count); sx/sy (f13,
    /// f14) 0.5..2. The renderer at gp-0x854 has a render-state record and
    /// a vtable whose `DrawPrims` (+0x238) is a fake that logs every
    /// register and sometimes unwinds. The HUD layout callees run for real.
    #[test]
    fn hud_draw_meter_bar_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::{DATA, GAME_GP};
        const HUD: u32 = DATA;
        const ELEMS: u32 = DATA + 0x1000;
        const SEG: u32 = DATA + 0x1400;
        const COLOUR: u32 = DATA + 0x1410;
        const RECT: u32 = DATA + 0x1420;
        const SPRITE: u32 = DATA + 0x1500;
        const RENDERER: u32 = DATA + 0x2000;
        const STATE: u32 = DATA + 0x4000;
        const VT: u32 = DATA + 0x4100;
        covered(
            ee::s05::f_0021d1a0__generated,
            hud_draw_meter_bar,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x21d1a0, 8);
                let unit = |r: &mut crate::difftest::Rng, lo: f32, steps: u32, step: f32| {
                    (lo + (r.next_u32() % steps) as f32 * step).to_bits()
                };
                // Sprites: id, v0, u0, u1, v1.
                for k in 0..4 {
                    let s = SPRITE + 0x20 * k;
                    c.bus.write32(s, r.next_u32());
                    for off in [0xc, 0x10, 0x14, 0x18] {
                        c.bus.write32(s + off, r.f32(1.0));
                    }
                }
                for style in 0..4 {
                    for i in 0..3 {
                        let p = SPRITE + 0x20 * (r.next_u32() % 4);
                        c.bus
                            .write32(HUD + meter_bar::SPRITES + 0xc * style + 4 * i, p);
                    }
                }
                // Layout elements -1..4.
                for i in 0..6u32 {
                    let e = ELEMS - 0x24 + 0x24 * i;
                    c.bus.write16(e, (r.next_u32() % 640) as u16);
                    c.bus.write16(e + 2, (r.next_u32() % 448) as u16);
                    c.bus.write16(e + 4, (1 + r.next_u32() % 64) as u16);
                    let h = if r.next_u32() % 8 == 0 {
                        -((r.next_u32() % 16) as i16)
                    } else {
                        (8 + r.next_u32() % 293) as i16
                    };
                    c.bus.write16(e + 6, h as u16);
                    for off in [8, 0xc] {
                        let s = unit(&mut r, 0.25, 8, 0.25);
                        c.bus.write32(e + off, s);
                    }
                    for off in [0x10, 0x14, 0x18, 0x1c] {
                        c.bus.write32(e + off, r.f32(1.0));
                    }
                    c.bus.write8(e + 0x20, (r.next_u32() % 4) as u8);
                    c.bus.write8(e + 0x21, (r.next_u32() % 4) as u8);
                    c.bus.write8(e + 0x22, r.next_u32() as u8);
                }
                let seg_scale = unit(&mut r, 0.5, 5, 0.25);
                c.bus.write32(SEG, seg_scale);
                c.bus
                    .write32(SEG + 4, ((r.next_u32() % 7) as i32 - 1) as u32);
                for i in 0..4 {
                    c.bus.write32(COLOUR + 4 * i, r.f32(1.0));
                }
                // The renderer: render state and DrawPrims.
                c.bus.write32(GAME_GP - meter_bar::RENDERER, RENDERER);
                c.bus.write32(RENDERER + meter_bar::STATE_TOP, STATE);
                c.bus.write32(STATE + meter_bar::STATE_FLAGS, r.next_u32());
                c.bus.write32(RENDERER + meter_bar::VPTR, VT);
                c.bus
                    .write16(VT + meter_bar::DRAW_PRIMS, ((r.next_u32() % 16) * 4) as u16);
                c.bus.write32(VT + meter_bar::DRAW_PRIMS + 4, FAKE);
                // Arguments.
                let idx = (r.next_u32() % 6) as i32 - 1;
                c.g[A0][0] = HUD as u64;
                c.g[A1][0] = ELEMS as u64;
                c.g[A2][0] = (r.next_u32() & !0xff) as u64 | (idx as u8) as u64;
                c.g[A3][0] = (r.next_u32() % 4) as u64;
                c.g[T0][0] = SEG as u64;
                c.g[T1][0] = COLOUR as u64;
                c.g[T2][0] = (r.next_u32() % 4) as u64;
                c.g[T3][0] = if seed % 4 == 0 { 0 } else { RECT as u64 };
                c.f[12] = if seed % 3 == 0 {
                    0x3f80_0000
                } else {
                    ((r.next_u32() % 1101) as f32 / 1000.0).to_bits()
                };
                c.f[13] = unit(&mut r, 0.5, 7, 0.25);
                c.f[14] = unit(&mut r, 0.5, 7, 0.25);
            },
        );
    }

    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::{elf_image, put_vec};

    // ------------------------------------------------------------------
    // ChaseCameraAlgorithm_ApplyAirSwing (0x164878).

    /// The chase-camera state, its swing info, the camera target ("rig",
    /// first word its vtable), the vtable and the rider the target points at.
    /// All above the fake results (`DATA..DATA+0x40000`), the heap arena
    /// (`DATA+0x40000..+0x80000`) and the fake log / world (`DATA+0x10_0000`).
    pub(crate) const AS_ME: u32 = DATA + 0x20_0000;

    pub(crate) const AS_INFO: u32 = DATA + 0x20_1000;

    pub(crate) const AS_RIG: u32 = DATA + 0x20_2000;

    pub(crate) const AS_VT: u32 = DATA + 0x20_2800;

    pub(crate) const AS_RIDER: u32 = DATA + 0x20_3000;

    /// Fields of the state (`ChaseCameraAlgorithm`).
    pub(crate) const AS_EYE_TARGET: u32 = 0x20;

    pub(crate) const AS_EYE: u32 = 0x40;

    pub(crate) const AS_TARGET: u32 = 0x30;

    pub(crate) const AS_SWING_CLOCK: u32 = 0x22c;

    pub(crate) const AS_DURATION: u32 = 0x230;

    pub(crate) const AS_TIME: u32 = 0x234;

    pub(crate) const AS_BLEND_OUT: u32 = 0x23c;

    pub(crate) const AS_BLEND_A: u32 = 0x240;

    pub(crate) const AS_BLEND_B: u32 = 0x244;

    pub(crate) const AS_AIRBORNE: u32 = 0x2d0;

    pub(crate) const AS_SWINGING: u32 = 0x2d8;

    pub(crate) const AS_SPLINE: u32 = 0x334;

    /// A real leaf that returns 0 (`v0 = 0; jr ra`), for the "no swing" answer
    /// of vtable slot 0x98 (the fakes never return 0).
    pub(crate) const RETURN_ZERO: u32 = 0x0014_06c0;

    /// `cCSICubicSpline_InitWithStartSlope` / `cCSICubicSpline_AddPoint`.
    pub(crate) const SPLINE_INIT: u32 = 0x0031_d738;

    pub(crate) const SPLINE_ADD: u32 = 0x0031_d7e0;

    /// Call a real guest function from the setup with `f12`/`f13` set.
    pub(crate) fn setup_call(
        c: &mut Cpu,
        addr: u32,
        f: GuestFn,
        a0: u32,
        a1: u64,
        f12: f32,
        f13: f32,
    ) {
        c.f[12] = f12.to_bits();
        c.f[13] = f13.to_bits();
        crate::game::camera::camera::tests::run(c, addr, f, &[sx(a0), a1]);
    }

    /// One air-swing frame: airborne or not, swing running or about to start
    /// (rig says yes / no), the spline initialised in the call itself or
    /// built beforehand (a live 9-point spline on the test heap), blend-out
    /// and the final minimum-distance clamp; every rig query is a logging fake
    /// (some unwind), `CheckDistanceReset` and the spline/maths are real.
    #[test]
    fn chase_camera_algorithm_apply_air_swing_matches() {
        covered(
            ee::s01::f_00164878__generated,
            crate::game::camera::camera::chase_camera_apply_air_swing,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = crate::game::camera::camera::tests::alloc_base(c, seed, 0x0016_4878);
                // Fake calls: 0x98 (n=0), 0x60 (1, 2), 0x80 (3), reset slot 8.
                let unwind = if seed % 3 == 0 {
                    r.next_u32() % 5
                } else {
                    u32::MAX
                };
                c.bus.write32(LOG_UNWIND_AT, unwind);
                for i in 0..0x360 / 4 {
                    c.bus.write32(AS_ME + 4 * i, 0);
                }
                // The rig: vtable, rider pointer (CheckDistanceReset's reset).
                c.bus.write32(AS_RIG, AS_VT);
                c.bus.write32(AS_RIG + 4, AS_RIDER);
                for (i, slot) in [0x08u32, 0x60, 0x80, 0x98].into_iter().enumerate() {
                    let delta = (r.next_u32() % 0x40) & !3;
                    c.bus.write16(AS_VT + slot, delta as u16);
                    c.bus.write32(AS_VT + slot + 4, FAKE + 0x10 * i as u32);
                }
                if seed % 5 == 0 {
                    c.bus.write32(AS_VT + 0x98 + 4, RETURN_ZERO);
                }
                put_vec(c, AS_RIDER + 0x1b0, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                c.bus.write32(AS_ME + AS_TARGET, AS_RIG);
                // World up (D_004FF160) and random vectors where the fake
                // results point (below the pool allocator's tables at
                // DATA+0x20000; above them the fakes read allocator words).
                put_vec(c, 0x004f_f160, [0, 0, 1.0f32.to_bits(), 0]);
                for i in 0..0x2_0000 / 4 {
                    let v = if i % 4 == 3 { 0 } else { r.f32(1.0) };
                    c.bus.write32(DATA + 4 * i, v);
                }
                let big = if seed % 7 == 0 { 30.0 } else { 600.0 };
                put_vec(
                    c,
                    AS_ME + AS_EYE_TARGET,
                    [r.f32(big), r.f32(big), r.f32(big), 0],
                );
                put_vec(c, AS_ME + AS_EYE, [r.f32(big), r.f32(big), r.f32(big), 0]);
                for off in [0x110u32, 0x120, 0x130, 0x140, 0x150, 0x160, 0x170, 0x180] {
                    put_vec(
                        c,
                        AS_ME + off,
                        [r.f32(300.0), r.f32(300.0), r.f32(300.0), 0],
                    );
                }
                put_vec(
                    c,
                    AS_INFO + 0x10,
                    [r.f32(600.0), r.f32(600.0), r.f32(600.0), 0],
                );
                put_vec(c, AS_INFO + 0x20, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                let airborne = seed % 4 != 0;
                let swinging = seed % 3 == 1;
                c.bus.write32(AS_ME + AS_AIRBORNE, airborne as u32);
                c.bus.write32(AS_ME + AS_SWINGING, swinging as u32);
                c.bus.write32(AS_ME + AS_SWING_CLOCK, r.f32(1.0));
                c.bus.write32(
                    AS_ME + AS_DURATION,
                    (2.3 + (r.next_u32() % 100) as f32 / 50.0).to_bits(),
                );
                let time = (r.next_u32() % 400) as f32 / 100.0;
                c.bus.write32(AS_ME + AS_TIME, time.to_bits());
                c.bus.write32(AS_ME + 0x238, r.f32(1.5));
                let blend = |r: &mut Rng, zero: bool| {
                    if zero {
                        0
                    } else {
                        ((r.next_u32() % 100) as f32 / 100.0).to_bits()
                    }
                };
                let z = seed % 6 == 2;
                let v = blend(&mut r, z);
                c.bus.write32(AS_ME + AS_BLEND_OUT, v);
                let v = blend(&mut r, seed % 5 == 3);
                c.bus.write32(AS_ME + AS_BLEND_A, v);
                let v = blend(&mut r, seed % 7 == 4);
                c.bus.write32(AS_ME + AS_BLEND_B, v);
                if swinging {
                    // A live spline, as the start of the swing leaves it.
                    let spl = AS_ME + AS_SPLINE;
                    setup_call(c, SPLINE_INIT, ee::s10::f_0031d738, spl, 9, 0.0, 0.0);
                    let mut y = 0.0f32;
                    for i in 0..9 {
                        y += (r.next_u32() % 100) as f32 / 800.0;
                        setup_call(
                            c,
                            SPLINE_ADD,
                            ee::s10::f_0031d7e0,
                            spl,
                            0,
                            i as f32 / 8.0,
                            y,
                        );
                    }
                }
                // Arguments: (this, info, blend-out time).
                c.g[A0][0] = sx(AS_ME);
                c.g[A1][0] = sx(AS_INFO);
                c.f[12] = ((r.next_u32() % 100) as f32 / 100.0 + 0.1).to_bits();
                c.g[RA][0] = crate::difftest::RA as u64;
                c.pc = 0x0016_4878;
            },
        );
    }

    // ------------------------------------------------------------------
    // Rider_FindRail (0x108a48).

    /// The rider, its state object, path, nodes, overlap list with one plain
    /// rail object (bounds, matrix, flags), animation controller and
    /// sequence list; the output pair of vectors.
    pub(crate) const FR_ME: u32 = DATA + 0x1000;

    pub(crate) const FR_STATE: u32 = DATA + 0x2000;

    pub(crate) const FR_PATH: u32 = DATA + 0x3000;

    pub(crate) const FR_NODES: u32 = DATA + 0x3100;

    pub(crate) const FR_LIST: u32 = DATA + 0x4000;

    pub(crate) const FR_RAIL: u32 = DATA + 0x5000;

    pub(crate) const FR_RAIL_FLAGS: u32 = DATA + 0x5200;

    pub(crate) const FR_ANIM: u32 = DATA + 0x6000;

    pub(crate) const FR_SEQS: u32 = DATA + 0x6100;

    pub(crate) const FR_SEQ: u32 = DATA + 0x6200;

    pub(crate) const FR_OUT: u32 = DATA + 0x7000;

    /// `AnimTable` (0x446990, 0x1c bytes per animation): the test writes the
    /// kind of animation 7.
    pub(crate) const ANIM_TABLE: u32 = 0x0044_6990;

    /// Main state 0/1/other; one rail in reach or none (or masked out); the
    /// rider still or moving (rail ahead / behind); layer-2 animation none,
    /// grab (0x12), trick (0x13/0x14) or other, with every combination of
    /// sequence flags 0 and 2. All callees are real (bounded on this data).
    #[test]
    fn rider_find_rail_matches() {
        use crate::difftest::fakes::machine;
        crate::difftest::check(
            ee::s00::f_00108a48__generated,
            rider_find_rail,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0010_8a48, 0);
                let me = FR_ME;
                c.bus.write32(me + 0x77c, FR_STATE);
                c.bus
                    .write32(FR_STATE + 0xde0, [0, 1, 0, 1, 2][(seed % 5) as usize]);
                // Path node + probe offset.
                c.bus.write32(me + rail_rider::PATH, FR_PATH);
                c.bus.write32(FR_PATH + 0x2c, FR_NODES);
                let node = r.next_u32() % 4;
                c.bus.write32(me + rail_rider::NODE, node);
                for i in 0..4 {
                    put_vec(
                        c,
                        FR_NODES + 0x20 * i,
                        [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0],
                    );
                }
                put_vec(
                    c,
                    me + rail_rider::PROBE_OFFSET,
                    [r.f32(5.0), r.f32(5.0), r.f32(5.0), 0],
                );
                let vel = if seed % 5 == 3 {
                    [0, 0, 0, 0]
                } else {
                    [r.f32(50.0), r.f32(50.0), r.f32(50.0), 0]
                };
                put_vec(c, me + rail_rider::VELOCITY, vel);
                c.bus.write32(me + 0x25c, r.f32(10.0));
                c.bus.write32(me + 0x330, r.next_u32() & 1);
                // The overlap list: none, or one plain rail (type 1) whose flags
                // pass the mask (1) or not.
                c.bus.write32(me + rail_rider::OVERLAP, FR_LIST);
                c.bus.write32(FR_LIST + 0x210, (seed % 7 != 0) as u32);
                c.bus.write32(FR_LIST + 0x214, FR_RAIL);
                c.bus.write32(FR_RAIL + 8, 1);
                c.bus.write32(FR_RAIL + 0x68, FR_RAIL_FLAGS);
                c.bus.write32(FR_RAIL_FLAGS + 0x1c, (seed % 11 != 5) as u32);
                c.bus.write32(FR_RAIL_FLAGS + 0x28, r.next_u32());
                for off in [0x6c, 0x70, 0x74] {
                    c.bus.write32(FR_RAIL + off, (-1.0e4f32).to_bits());
                }
                for off in [0x78, 0x7c, 0x80] {
                    c.bus.write32(FR_RAIL + off, 1.0e4f32.to_bits());
                }
                for i in 0..4 {
                    let w = if i == 3 { 1.0f32.to_bits() } else { 0 };
                    put_vec(
                        c,
                        FR_RAIL + 0x10 + 0x10 * i,
                        [r.f32(100.0), r.f32(100.0), r.f32(100.0), w],
                    );
                }
                // Layer-2 animation (0x1b6 = none) and its sequence flags.
                c.bus.write32(me + rail_rider::ANIM, FR_ANIM);
                c.bus
                    .write32(FR_ANIM + 8, if seed % 6 == 0 { 0x1b6 } else { 7 });
                let kind = [0x12u32, 0x13, 0x14, 0x12, 5][(r.next_u32() % 5) as usize];
                c.bus.write32(ANIM_TABLE + 7 * 0x1c, kind);
                c.bus.write32(FR_ANIM + 0x50, FR_SEQS);
                c.bus.write32(FR_SEQS + 0x10 + 4, FR_SEQ);
                c.bus.write64(FR_SEQ + 0xb0, (r.next_u32() & 0b101) as u64);
                put_vec(c, FR_OUT, [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0]);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(FR_OUT);
                c.pc = 0x0010_8a48;
            },
        );
    }

    // ------------------------------------------------------------------
    // HUD_AlignX (0x21e750).

    /// Alignments -2..2, all four flag combinations, random and edge floats.
    #[test]
    fn hud_align_x_matches() {
        use crate::difftest::{scramble, Rng};
        crate::difftest::check(
            ee::s05::f_0021e750__generated,
            hud_align_x,
            300,
            &|c, seed| {
                let mut r = Rng(0x21e7_50 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                c.g[A1][0] = sx((seed % 5) as i32 as u32).wrapping_sub(2);
                c.g[A2][0] = ((seed / 5) % 4) as u64 | (r.next_u32() as u64 & !3) << 8;
                for f in [12, 13, 14] {
                    c.f[f] = if seed % 6 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(400.0)
                    };
                }
                c.pc = 0x0021_e750;
            },
        );
    }

    use crate::difftest::fakes::{vtable, LOG_RESULTS};
    use crate::difftest::{fresh, put_words};

    /// WPData_FindCellIndex on a real quadtree (QuadTree_FindLeaf): a root
    /// leaf, or a tree of up to three internal levels with random leaf
    /// cells; points inside the tree and outside it (negative or too large
    /// grid coordinates: no leaf).
    #[test]
    fn wp_data_find_cell_index_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::DATA;
        const WP: u32 = DATA;
        const TREE: u32 = DATA + 0x100;
        const NODES: u32 = DATA + 0x1000;
        crate::difftest::check(
            ee::s08::f_002baf90__generated,
            wp_data_find_cell_index,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2baf_90, 0);
                c.pc = 0x002b_af90;
                let f = |x: f32| x.to_bits();
                c.bus.write32(WP, TREE);
                // Grid: 0x8000 cells over 0..1024 world units.
                c.bus.write32(TREE, f(32.0));
                c.bus.write32(TREE + 4, f(0.0));
                c.bus.write32(TREE + 8, f(0.0));
                c.bus.write16(TREE + 0x14, 0);
                c.bus.write32(TREE + 0x20, NODES);
                // Nodes (8 bytes): internal = four child halfwords (index << 1,
                // bit 0 of the first set), leaf = even first halfword and its
                // cell index at +4.
                let depth = seed % 4;
                let mut next = 1u32;
                let mut level = vec![0u32];
                for d in 0..=depth {
                    let mut below = Vec::new();
                    for &n in &level {
                        let node = NODES + 8 * n;
                        if d < depth && r.next_u32() % 4 != 0 {
                            for k in 0..4u32 {
                                let child = next;
                                next += 1;
                                below.push(child);
                                c.bus
                                    .write16(node + 2 * k, (child << 1 | (k == 0) as u32) as u16);
                            }
                        } else {
                            c.bus.write16(node, (r.next_u32() as u16) & !1);
                            c.bus.write32(node + 4, r.next_u32() % 5000);
                        }
                    }
                    level = below;
                }
                let coord = |r: &mut crate::difftest::Rng| match r.next_u32() % 8 {
                    0 => -((r.next_u32() % 100) as f32) - 1.0,
                    1 => 1024.0 + (r.next_u32() % 100) as f32,
                    _ => (r.next_u32() % 1024) as f32 + 0.5,
                };
                c.f[12] = f(coord(&mut r));
                c.f[13] = f(coord(&mut r));
                c.g[A0][0] = sx(WP);
            },
        );
    }

    // -- 0x002f1510 RiderBase_RenderAttackFX ----------------------------------

    /// `RiderBase_RenderAttackFX`: inactive and active effects with random
    /// sprites, a random packet record and template, the display's
    /// `DrawSprite` a fake (some calls unwind).
    #[test]
    fn rider_base_render_attack_fx_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::{fill, DATA, GAME_GP};
        const THIS: u32 = DATA;
        const GFX: u32 = DATA + 0x1_0000;
        const DISP: u32 = DATA + 0x2_0000;
        const PKT: u32 = DATA + 0x3_0000;
        const TEMPLATE: u32 = 0x0050_1420;
        covered(
            ee::s09::f_002f1510__generated,
            rider_base_render_attack_fx,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_1510, 4);
                let r = &mut r;
                c.pc = 0x002f_1510;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                w(c, GAME_GP.wrapping_sub(0x854), GFX);
                w(c, GFX + 0x10d8, DISP);
                c.bus
                    .write16(DISP + 0x270, (r.next_u32() as i16 & !3) as u16);
                w(c, DISP + 0x274, FAKE);
                w(c, GFX + 0xe84, PKT + 4 * (r.next_u32() % 8));
                w(c, GFX + 0xfb0, r.next_u32());
                fill(c, r, PKT, 0x40);
                fill(c, r, TEMPLATE, 5);
                fill(c, r, THIS, 0x30);
                for i in 0..4 {
                    w(c, THIS + 0x90 + 4 * i, r.f32(64.0));
                }
                w(
                    c,
                    THIS + 0xa0,
                    if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.g[A0][0] = sx(THIS);
            },
        );
    }

    /// ComputerActorRider_SelectTargetRider: a current target (none, near,
    /// far) and the actor's distance to the finish (below/above 540 and
    /// 600), the AI frame counter (a multiple of 12 or not), six table
    /// entries (empty, near, beyond 700), the rival flags and the actor's
    /// travel direction (zero x and/or y, either sign) so every heading
    /// branch and the real `atanf`/`cosf` run.
    #[test]
    fn computer_actor_rider_select_target_rider_matches() {
        check(
            ee::s00::f_0010bd10__generated,
            computer_actor_rider_select_target_rider,
            300,
            &|c, seed| {
                let mut r = Rng(0x10bd_10 ^ (seed as u64) << 24);
                scramble(c, &mut r);
                c.pc = 0x0010_bd10;
                c.g[GP][0] = GAME_GP as u64;
                let actor = DATA;
                let table = DATA + 0x400;
                let flag = DATA + 0x700;
                // AI_GetFrameCounter: [[[gp-0x848]+0x84]+0xc]+8.
                let (a, b, d) = (DATA + 0x5000, DATA + 0x5100, DATA + 0x5200);
                c.bus.write32(GAME_GP - 0x848, a);
                c.bus.write32(a + 0x84, b);
                c.bus.write32(b + 0xc, d);
                c.bus.write32(
                    d + 8,
                    if seed % 3 == 0 {
                        7
                    } else {
                        12 * (r.next_u32() % 50)
                    },
                );
                // Rivals: IsRivalTarget reads the actor's table +0xf0 / +0xf8.
                c.bus.write32(table + 0xf0, r.next_u32() % 2);
                c.bus.write32(table + 0xf8, r.next_u32() % 6);
                for (off, v) in [
                    (0x7e00u32, 500.0f32),
                    (0x7dfc, 1.0e9),
                    (0x7df8, std::f32::consts::PI),
                    (0x7df4, -std::f32::consts::FRAC_PI_2),
                    (0x7df0, std::f32::consts::FRAC_PI_2),
                    (0x7dec, 1.0 / (2.0 * std::f32::consts::PI)),
                    (0x7de8, 2.0 * std::f32::consts::PI),
                ] {
                    c.bus.write32(GAME_GP - off, v.to_bits());
                }
                c.bus.write32(actor + 0x18, table);
                c.bus
                    .write32(actor + 0xe70, [u32::MAX, 1, 3][(r.next_u32() % 3) as usize]);
                for k in 0..6u32 {
                    let e = table + 0x24 * k;
                    c.bus.write32(
                        e,
                        if r.next_u32() % 4 == 0 {
                            0
                        } else {
                            DATA + 0x900
                        },
                    );
                    let dist = [100.0f32, 650.0, 800.0][(r.next_u32() % 3) as usize];
                    c.bus.write32(e + 8, dist.to_bits());
                    c.bus.write32(e + 0xc, r.f32(4.0));
                }
                let finish = [300.0f32, 560.0, 700.0][(r.next_u32() % 3) as usize];
                c.bus.write32(table + 0x4c8, finish.to_bits());
                let comp = |r: &mut Rng| match r.next_u32() % 4 {
                    0 => 0,
                    _ => r.f32(10.0),
                };
                put_vec(
                    c,
                    table + 0x1b0,
                    [comp(&mut r), comp(&mut r), r.next_u32(), r.next_u32()],
                );
                c.g[A0][0] = sx(actor);
                c.g[A1][0] = sx(flag);
            },
        );
    }

    pub(crate) use crate::game::fe::ovstatehud::tests::gpa;

    /// Centred, bottom and top alignment (align 0, positive, negative with
    /// the upper half set) over edge-case floats.
    #[test]
    fn hud_align_y_matches() {
        use crate::difftest::{scramble, Rng};
        check(
            ee::s05::f_0021e7a8__generated,
            hud_align_y,
            300,
            &|c, seed| {
                let mut r = Rng(0x21e7a8 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => 1 + (r.next_u32() & 0xff) as u64,
                    2 => sx(0xffff_fffe),
                    _ => 0x1_0000_0000,
                };
                c.f[12] = if seed % 3 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(500.0)
                };
                c.f[13] = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(100.0)
                };
                c.f[14] = if seed % 7 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(4.0)
                };
                c.pc = 0x0021_e7a8;
            },
        );
    }

    /// `DIntr` with interrupts on and off (random other Status bits).
    #[test]
    fn d_intr_matches() {
        check(ee::s15::f_0042c078__generated, d_intr, 64, &|c, seed| {
            let mut r = Rng(0x42c0_7800 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let status = r.next_u32() & !(STATUS_EIE as u32);
            c.cop0[12] = if seed % 2 == 0 {
                status
            } else {
                status | STATUS_EIE as u32
            };
            c.pc = 0x0042_c078;
        });
    }

    // ------------------------------------------------------- LensFX_DrawSun

    pub(crate) const SUN_FX: u32 = DATA;

    pub(crate) const SUN_VP: u32 = DATA + 0x300;

    pub(crate) const SUN_HOLDER: u32 = DATA + 0x400;

    pub(crate) const SUN_VT: u32 = DATA + 0x600;

    pub(crate) const SUN_REND: u32 = DATA + 0x1000;

    pub(crate) const SUN_REND_VT: u32 = DATA + 0x3000;

    pub(crate) const SUN_REND_STATE: u32 = DATA + 0x3400;

    pub(crate) const SUN_RESULTS: u32 = DATA + 0x3800;

    /// The lens FX at `SUN_FX` (visible most seeds, no flares), a viewport,
    /// the world painter's sun for views 6 and 7 (methods are fakes whose
    /// forced results point at the colour, intensity, size and texture id
    /// at `SUN_RESULTS`: the alpha is positive or not, the size sometimes
    /// 1), the renderer (`gp+0x2a90` and `gp-0x854`) with a vtable of fakes.
    pub(crate) fn draw_sun_setup(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::LOG_RESULTS;
        let mut r = machine(c, seed, 0x002f_4a08, 12);
        for view in [6u32, 7] {
            let entry = 0x004f_a370 + view * 0xf0;
            c.bus.write32(entry + 0x14, SUN_HOLDER);
        }
        c.bus.write32(SUN_HOLDER, SUN_HOLDER + 0x40);
        c.bus.write32(SUN_HOLDER + 0x40 + 4, SUN_VT);
        for (i, off) in [0xe8u32, 0xf0, 0xf8, 0x100, 0x118, 0x108]
            .into_iter()
            .enumerate()
        {
            c.bus
                .write16(SUN_VT + off, ((r.next_u32() % 16) * 4) as u16);
            c.bus.write32(SUN_VT + off + 4, FAKE + 0x10 * i as u32);
        }
        // Results of the six getters (red, green, blue, intensity, size,
        // texture).
        for i in 0..6u32 {
            c.bus.write32(LOG_RESULTS + 4 * i, SUN_RESULTS + 0x10 * i);
        }
        for i in 0..3u32 {
            c.bus.write32(SUN_RESULTS + 0x10 * i, r.f32(1.0));
        }
        let intensity = if seed % 4 == 3 { 0 } else { r.f32(2.0) };
        c.bus.write32(SUN_RESULTS + 0x30, intensity);
        let size = if seed % 3 == 0 {
            0x3f80_0000
        } else {
            r.f32(400.0)
        };
        c.bus.write32(SUN_RESULTS + 0x40, size);
        c.bus.write32(SUN_RESULTS + 0x50, r.next_u32() % 8);
        // The renderer: every method a fake, a random state stack.
        for rend in [gpa(0x2a90), gpa(-0x854)] {
            c.bus.write32(rend, SUN_REND);
        }
        c.bus.write32(SUN_REND + 0x10d8, SUN_REND_VT);
        for off in (0..0x300).step_by(8) {
            c.bus
                .write16(SUN_REND_VT + off, ((r.next_u32() % 16) * 4) as u16);
            c.bus.write32(SUN_REND_VT + off + 4, FAKE + 0x100 + 2 * off);
        }
        c.bus
            .write32(SUN_REND + 0xe84, SUN_REND_STATE + 4 * (seed % 4));
        fill(c, &mut r, SUN_REND_STATE, 0x20);
        fill(c, &mut r, SUN_REND + 0xf50 + 0xcc, 8);
        // The lens FX and the viewport.
        c.bus
            .write16(SUN_FX + 0x2c, if seed % 9 == 0 { 0 } else { 1 });
        c.bus.write32(SUN_FX + 0x1f0, r.f32(600.0));
        c.bus.write32(SUN_FX + 0x1f4, r.f32(400.0));
        c.bus.write32(SUN_FX + 0x200, 0);
        c.bus.write32(SUN_FX + 0x204, r.f32(1.0));
        for i in 0..4 {
            c.bus.write32(SUN_VP + 4 * i, r.f32(640.0));
        }
        c.g[A0][0] = SUN_FX as u64;
        c.g[A1][0] = SUN_VP as u64;
        c.g[A2][0] = if seed % 2 == 0 {
            0
        } else {
            r.next_u32() as u64
        };
        c.pc = 0x002f_4a08;
    }

    #[test]
    fn lens_fx_draw_sun_matches() {
        covered(
            ee::s09::f_002f4a08__generated,
            lens_fx_draw_sun,
            200,
            &draw_sun_setup,
        );
    }

    /// Rider_SetSlew0Target: below and above full-lean speed, rate below,
    /// inside and above its bounds, states 2/3/13 and others.
    #[test]
    fn rider_set_slew0_target_matches() {
        check(
            ee::s00::f_00113e80__generated,
            rider_set_slew0_target,
            300,
            &|c, seed| {
                let mut r = Rng(0x113e_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let rider = DATA;
                c.g[A0][0] = sx(rider);
                let v = if seed % 3 == 0 { 900.0 } else { 300.0 };
                for k in 0..4 {
                    c.bus.write32(rider + RIDER_VELOCITY + 4 * k, r.f32(v));
                }
                c.f[12] = if seed % 7 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(2.0)
                };
                c.bus.write32(rider + RIDER_SLEW0, r.f32(2.0));
                put_words(
                    c,
                    GAME_GP - GP_SLEW0_FULL_SPEED,
                    &[
                        694.538_8f32.to_bits(),
                        12.002_403f32.to_bits(),
                        0.1f32.to_bits(),
                        14.001_209f32.to_bits(),
                        0.756_134f32.to_bits(),
                        (1.0f32 / 60.0).to_bits(),
                    ],
                );
                let state = [2, 3, 13, 0, 1, 4, 12, 14][(r.next_u32() % 8) as usize];
                c.bus.write32(rider + RIDER_MAIN_STATE, state);
            },
        );
    }

    // --- PATH_UpdateTracks -------------------------------------------------

    /// The PATH player in normal or streamed mode with a mix of empty, busy,
    /// inactive, fading, stopping and playing tracks. Platform handles: 0 has
    /// no stream (the fades' platform calls take their short path), 1..3 are
    /// streams that are stopped / playing. Every playing track still has
    /// enough queued (deadline or lead), so PATH_TrackAdvance (a large tree
    /// of its own) is not entered.
    pub(crate) fn path_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x003d_4f10 ^ (seed as u64) << 12);
        crate::difftest::scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        let state = DATA + 0x100;
        let streamed = seed % 2 == 0;
        c.bus
            .write8(0x0044_f434, if streamed { b'B' } else { b'A' });
        w(c, 0x0044_f420, state);
        let time = 0x1000 + r.next_u32() % 0x1_0000;
        w(c, 0x0044_f438, time);
        let ticks = [r.next_u32() % 0x1000, r.next_u32() % 0x1000];
        w(c, state + 0xc, ticks[0]);
        w(c, state + 0x10, ticks[1]);
        let step = ticks[streamed as usize];
        w(c, state + 0x14, r.next_u32());
        w(c, state + 0x18, r.next_u32());
        w(c, state + 0x91c, 0); // no queues to service
        w(c, GAME_GP + 0x410, DATA + 0xf000); // audio system: no PATH streams
                                              // Platform stream table: 0 none, 1 stopped, 2/3 playing.
        let table = 0x004d_3e98;
        w(c, table, 0);
        for h in 1..4u32 {
            let p = DATA + 0xe000 + 0x100 * h;
            w(c, table + 4 * h, p);
            c.bus.write8(p + 0x64, 1);
            c.bus.write8(p + 0x65, 0);
            w(c, p + 0x58, [0, 0, 1, 7][h as usize]);
            w(c, p + 0xc, 1);
        }
        for i in 0..24u32 {
            let t = DATA + 0x2000 + 0x100 * i;
            let used = r.next_u32() % 5 != 0;
            w(c, state + 0x58 + 4 * i, if used { t } else { 0 });
            crate::difftest::fill(c, &mut r, t, 0x40);
            c.bus.write8(t + 0x3, (r.next_u32() % 6 == 0) as u8);
            c.bus
                .write16(t + 0x36, if r.next_u32() % 4 == 0 { 0xffff } else { 3 });
            w(c, t + 0x30, (r.next_u32() % 5 != 0) as u32);
            let mut fading = false;
            for f in [0xd, 0x15, 0x1d] {
                let on = r.next_u32() % 4 == 0;
                fading |= on;
                c.bus.write8(t + f, if on { 0 } else { 0xff });
            }
            let mine = (r.next_u32() % 4 != 0) == streamed;
            c.bus.write8(t + 0x2f, mine as u8);
            let handle = if fading { 0 } else { 1 + r.next_u32() % 3 };
            w(c, t + 0x54, handle);
            let stop = if handle != 0 && r.next_u32() % 3 == 0 {
                time + r.next_u32() % (step + 2)
            } else {
                0
            };
            w(c, t + 0x4c, stop);
            w(c, t + 0x20, 0); // nothing queued
            let lead = (r.next_u32() % 0x200) as i32 - 0x100;
            let deadline = if lead < 0 && r.next_u32() % 2 == 0 {
                0
            } else {
                (time as i32 + lead) as u32 + step + 1 + r.next_u32() % 0x100
            };
            w(c, t + 0x24, lead as u32);
            w(c, t + 0x48, deadline);
        }
        c.pc = 0x003d_4f10;
    }

    #[test]
    fn path_update_tracks_matches() {
        crate::difftest::check(
            ee::s14::f_003d4f10__generated,
            path_update_tracks,
            200,
            &|c, seed| path_fixture(c, seed),
        );
    }

    /// A float in [lo, hi) as its bit pattern.
    pub(crate) fn uf(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        (lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)).to_bits()
    }

    pub(crate) fn put_cstr(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// HUD_DrawSpeedometer: the BELibrary dummy and interface (a fake
    /// method), the rider's velocity method a fake returning a random vector,
    /// random settings (units bit both ways), HUD elements and text object as
    /// in the position-panel test, speeds giving one to three digits; real
    /// HUD/font/sprintf/strlen callees.
    #[test]
    fn hud_draw_speedometer_matches() {
        use super::speedo::*;
        covered(
            ee::s05::f_002200c0__generated,
            hud_draw_speedometer,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22_00c0, 2);
                c.pc = 0x0022_00c0;
                let gp = GAME_GP;
                let (hud, elems, rider, text) = (DATA, DATA + 0x1000, DATA + 0x1c00, DATA + 0x800);
                // BELibrary singleton and interface slot 0 with a fake method.
                c.bus.write32(gp - 0x1f04, DATA + 0x30_0000);
                c.bus.write32(gp - 0x1efc, 0);
                let lib_if = DATA + 0x3000;
                c.bus.write32(0x0053_08a8, lib_if);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(lib_if + 4, vt);
                let vt2 = vtable(c, &mut r, &[SLOT_GET_VELOCITY]);
                c.bus.write32(rider + RIDER_VELOCITY, vt2);
                let vel = DATA + 0x2000;
                c.bus.write32(LOG_RESULTS + 4, vel);
                for k in 0..4 {
                    let v = uf(&mut r, -70.0, 70.0);
                    c.bus.write32(vel + 4 * k, v);
                }
                // The settings block (word 0: the units flag).
                fill(c, &mut r, SETTINGS, 0x288 / 4);
                let w0 = r.next_u32() & !(UNITS_FLAG as u32);
                c.bus.write32(
                    SETTINGS,
                    w0 | if seed % 2 == 0 { UNITS_FLAG as u32 } else { 0 },
                );
                put_cstr(c, NUMBER_FORMAT, b"%d");
                put_cstr(c, LABEL_A, b"MPH");
                put_cstr(c, LABEL_B, b"KPH");
                for off in [
                    GP_NEG_SPEED_SCALE,
                    GP_NEG_UNITS_FACTOR,
                    GP_NEG_SCALE_SHORT,
                    GP_NEG_SCALE_LONG,
                    GP_NEG_LABEL_OFFSET,
                ] {
                    let v = uf(&mut r, 0.2, 3.0);
                    c.bus.write32(gp - off, v);
                }
                // The HUD, its text object and elements.
                c.bus.write32(hud + HUD_TEXT, text);
                c.bus.write32(text + TEXT_BASE_SCALE, uf(&mut r, 1.0, 20.0));
                c.bus
                    .write32(text + TEXT_BASE_SCALE + 4, uf(&mut r, 1.0, 20.0));
                c.bus.write32(text + TEXT_LINE, r.next_u32() % 32);
                for i in 0..48u32 {
                    let e = elems + 0x24 * i;
                    c.bus.write16(e, (r.next_u32() % 640) as u16);
                    c.bus.write16(e + 2, (r.next_u32() % 448) as u16);
                    c.bus.write16(e + 4, (r.next_u32() % 64) as u16);
                    c.bus.write16(e + 6, (r.next_u32() % 64) as u16);
                    for k in 0..4 {
                        let v = uf(&mut r, 0.0, 1.0);
                        c.bus.write32(e + 0x10 + 4 * k, v);
                    }
                    c.bus.write8(e + 0x20, (r.next_u32() % 3) as u8);
                    c.bus.write8(e + 0x21, (r.next_u32() % 3) as u8);
                    c.bus.write8(e + 0x22, (r.next_u32() % 8) as u8);
                }
                // Font_DrawText's globals (as in the position-panel test).
                c.bus.write32(gp - 0xf28, DATA + 0x1800);
                fill(c, &mut r, DATA + 0x1800, 4);
                c.bus.write32(gp - 0x854, DATA + 0x1900);
                c.bus.write32(DATA + 0x1900 + 0xe84, DATA + 0x1a00);
                c.bus.write32(DATA + 0x1a00 + 8, r.next_u32());
                c.g[A0][0] = sx(hud);
                c.g[A1][0] = sx(elems);
                c.g[A2][0] = sx(rider);
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// StreamFile_GetChunk: null / bad-magic / valid handles, empty or
    /// non-empty queues, a ring of 2-6 chunks plus a wrap marker with this
    /// handle's tag somewhere other than the chunk taken; the mutex's kernel
    /// calls are fakes (some unwind).
    #[test]
    fn stream_file_get_chunk_matches() {
        use super::chunk::*;
        covered(
            ee::s14::f_003e12e0__generated,
            stream_file_get_chunk,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_12e0, 4);
                c.pc = 0x003e_12e0;
                let (h, man, ring) = (DATA, DATA + 0x100, DATA + 0x1000);
                c.g[A0][0] = if seed % 13 == 0 { 0 } else { sx(h) };
                c.bus.write32(h, man);
                c.bus.write32(
                    man,
                    if seed % 11 == 0 {
                        r.next_u32()
                    } else {
                        0x4d52_5453
                    },
                );
                // The mutex at man+4: owner, depth, semaphore.
                c.bus.write32(man + 8, r.next_u32() % 4);
                c.bus.write32(man + 0xc, r.next_u32() % 3);
                c.bus.write32(man + 0x10, r.next_u32() % 64);
                c.bus.write32(man + MAN_RING, ring);
                let tag = 1 + r.next_u32() % 255;
                c.bus.write32(h + H_TAG, tag);
                let k = 2 + r.next_u32() % 5;
                let take = r.next_u32() % k;
                let mut hit = (take + 1 + r.next_u32() % (k - 1)) % k;
                if hit == take {
                    hit = (take + 1) % k;
                }
                let mut at = ring;
                let mut take_at = ring;
                let mut take_size = 0;
                for i in 0..k {
                    let size = 0x10 * (1 + r.next_u32() % 8);
                    let t = if i == hit || (i != take && r.next_u32() % 3 == 0) {
                        tag
                    } else {
                        (tag + 1 + r.next_u32() % 254) % 256
                    };
                    let w0 = r.next_u32() & 0x7fff_ffff;
                    c.bus.write32(at, w0);
                    c.bus.write32(at + CH_TAG_SIZE, t << 24 | size);
                    if i == take {
                        take_at = at;
                        take_size = size;
                    }
                    at += size;
                }
                c.bus.write32(at, u32::MAX);
                c.bus.write32(
                    at + CH_TAG_SIZE,
                    r.next_u32() & 0x00ff_ffff | 0x0100_0000 * ((tag + 7) % 256),
                );
                c.bus.write32(h + H_NEXT, take_at);
                let queued = match seed % 4 {
                    0 => 0,
                    1 => take_size,
                    2 => take_size + 0x10 * (1 + r.next_u32() % 16),
                    _ => r.next_u32() % 0x100,
                };
                c.bus.write32(h + H_QUEUED, queued);
            },
        );
    }

    use crate::difftest::require_paths;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::fe::ovstatehud::tests::obj;

    /// tanf: small (|x| <= pi/4), medium and huge arguments (reduced by
    /// the real `__ieee754_rem_pio2f`, which reads its tables from the ELF
    /// image), and inf/NaN bit patterns; the real `__kernel_tanf` runs.
    #[test]
    fn tanf_matches() {
        check(ee::s15::f_0040d588__generated, tanf, 200, &|c, seed| {
            let mut r = Rng(0x0040_d588 ^ (seed as u64) << 12);
            crate::difftest::elf_image(c);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0040_d588;
            let sign = (r.next_u32() & 1) << 31;
            c.f[12] = sign
                | match seed % 6 {
                    0 => r.f32(0.78) & 0x7fff_ffff,
                    1 => 0x3f49_0fda + r.next_u32() % 3,
                    2 => (1.0f32 + (r.next_u32() % 1000) as f32 * 0.1).to_bits(),
                    3 => (1.0e4f32 + (r.next_u32() % 100000) as f32 * 1.0e3).to_bits(),
                    4 => 0x7f80_0000 | r.next_u32() % 4 * (r.next_u32() & 0x7f_ffff),
                    _ => r.edge_f32() & 0x7fff_ffff,
                };
        });
    }

    /// ComputerActorRider_FindSpeedMarker: the track query is a fake that
    /// may unwind and returns a forced count (1..12); the marker
    /// records it "wrote" (the frame buffer at sp) are set up in the stack
    /// area beforehand, with a speed marker at a random index or none.
    #[test]
    fn computer_actor_rider_find_speed_marker_matches() {
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::difftest::SP as STACK;
        covered(
            ee::s00::f_0010bb18__generated,
            computer_actor_rider_find_speed_marker,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_bb18, 1);
                c.pc = 0x0010_bb18;
                let (actor, rider, query, vt) = (DATA, DATA + 0x100, DATA + 0x1000, DATA + 0x1100);
                c.bus.write32(actor + oc230_cpu_rider::RIDER, rider);
                c.bus.write32(rider + oc230_cpu_rider::TRACK_QUERY, query);
                let (pos, dist) = (r.f32(1000.0), r.f32(1000.0));
                c.bus.write32(rider + oc230_cpu_rider::TRACK_POS, pos);
                c.bus.write32(rider + oc230_cpu_rider::TRACK_DIST, dist);
                c.bus.write32(query + oc230_cpu_rider::QUERY_VTABLE, vt);
                c.bus.write16(vt + 8, (r.next_u32() % 8 * 4) as u16);
                c.bus.write32(vt + 0xc, FAKE);
                c.bus.write32(GAME_GP - 0x7e04, 27.777_78f32.to_bits());
                // (A non-positive count cannot be forced: the fake returns the
                // forced word zero-extended, and 0 means "not forced".)
                let n = if seed % 6 == 0 {
                    1
                } else {
                    1 + r.next_u32() % 12
                };
                c.bus.write32(LOG_RESULTS, n);
                let buf = STACK - 0x100;
                let marker = if seed % 4 == 0 { 99 } else { r.next_u32() % 12 };
                for i in 0..12u32 {
                    let rec = buf + 0x10 * i;
                    let ty = if i == marker {
                        0x11
                    } else {
                        r.next_u32() % 0x20 & !0x11 | 1
                    };
                    c.bus.write32(rec, ty);
                    c.bus.write32(rec + 4, r.next_u32() % 200);
                    let lim = if r.next_u32() % 2 == 0 {
                        dist
                    } else {
                        r.f32(1000.0)
                    };
                    c.bus.write32(rec + 8, lim);
                }
                c.g[A0][0] = sx(actor);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x2010);
            },
        );
    }

    /// Empty queue (nothing happens) and 1-6 queued particles sorted by the
    /// real qsort and comparator; random render-state stack contents; the
    /// manager's draw call is a fake (some unwind).
    #[test]
    fn ps2_fog_particle_man_render_matches() {
        covered(
            ee::s09::f_002dbf98__generated,
            ps2_fog_particle_man_render,
            300,
            &|c, seed| {
                c.pc = 0x002d_bf98;
                let mut r = machine(c, seed, 0x2dbf98, 1);
                let gp = c.g[GP][0] as u32;
                let man = DATA;
                let ctx = DATA + 0x1000;
                let stack = DATA + 0x2004;
                let n = seed % 7;
                c.bus.write32(man + FOG_COUNT, n);
                c.bus.write32(man + FOG_VT, DATA + 0x5000);
                crate::game::main::gamemode::tests::slot(
                    c,
                    DATA + 0x5000,
                    VT_FOG_DRAW,
                    (r.next_u32() as i16) & !3,
                    FAKE,
                );
                crate::difftest::fill(c, &mut r, FOG_SORT_BUF, 2 * n);
                c.bus.write32(gp.wrapping_sub(R2D_CTX_GP), ctx);
                c.bus
                    .write32(ctx + R2D_STATE_TOP, stack + 0x14 * (seed % 3));
                c.bus.write32(ctx + 0xf60, r.next_u32());
                crate::difftest::fill(c, &mut r, stack, 0x14);
                c.g[A0][0] = sx(man);
            },
        );
    }

    /// 17 effect systems with fake render methods (some unwind).
    #[test]
    fn ps2_visual_effects_man_render_matches() {
        covered(
            ee::s09::f_002f0cd0__generated,
            ps2_visual_effects_man_render,
            300,
            &|c, seed| {
                c.pc = 0x002f_0cd0;
                let mut r = machine(c, seed, 0x2f0cd0, VFX_SYSTEMS);
                let man = DATA;
                for k in 0..VFX_SYSTEMS {
                    let sys = DATA + 0x100 + 0x40 * k;
                    let vt = DATA + 0x1000 + 0x40 * k;
                    c.bus.write32(man + 4 * k, sys);
                    c.bus.write32(sys, vt);
                    crate::game::main::gamemode::tests::slot(
                        c,
                        vt,
                        VT_VFX_RENDER,
                        (r.next_u32() as i16) & !3,
                        FAKE + 0x10 * k,
                    );
                }
                c.g[A0][0] = sx(man);
            },
        );
    }

    /// Rider_TurnAroundIfBackward: braking (no turn), moving forward (no
    /// turn) and moving backward (turn: rebuild the world matrix and toggle
    /// the stance through the real callees).
    #[test]
    fn rider_turn_around_if_backward_matches() {
        use super::turn_around::*;
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x4cc0, 0);
            elf_image(c);
            c.pc = 0x0011_4cc0;
            let rider = obj(&mut r);
            c.g[A0][0] = rider as u64;
            // Whole rider area the callees touch: random floats.
            for off in (0..0x800).step_by(4) {
                c.bus.write32(rider + off, r.f32(1.5));
            }
            let brake = match seed % 4 {
                0 => r.f32(1.0),
                1 => 0x8000_0000,
                _ => 0,
            };
            c.bus.write32(rider + BRAKE, brake);
            let vel_scale = if seed % 3 == 0 { 1.0 } else { -1.0 };
            for i in 0..4 {
                let v = r.f32(1.0);
                let fwd = r.f32(1.0);
                c.bus.write32(rider + GROUND_FORWARD + 4 * i, fwd);
                // Velocity roughly along (or against) the ground forward.
                let vf = f32::from_bits(fwd) * 3000.0 * vel_scale + f32::from_bits(v);
                c.bus.write32(rider + VEL + 4 * i, vf.to_bits());
                c.bus.write32(rider + 0x110 + 4 * i, r.f32(1e5));
            }
            let owner = obj(&mut r);
            c.bus.write32(rider + 0x77c, owner);
            c.bus.write32(rider + 0x784, owner);
        };
        check(
            ee::s00::f_00114cc0__generated,
            rider_turn_around_if_backward,
            300,
            &setup,
        );
        // Braking (10+4), not backward (10+15+4), turned (and more).
        require_paths(ee::s00::f_00114cc0__generated, 300, &[14, 29], &setup);
        let mut turned = 0;
        for seed in 0..300 {
            let mut c = fresh();
            setup(&mut c, seed);
            ee::s00::f_00114cc0__generated(&mut c);
            turned += (c.g[V0][0] == 1) as u32;
        }
        assert!(turned > 20, "turned {turned}");
    }

    // ---------------------------------------------- PS2VisualEffectsMan_Update

    /// `PS2VisualEffectsMan_Update`: the 17 resets and the drawer's view
    /// resets are fakes (one may unwind); 0..3 views, each through the real
    /// `PS2VisualEffectsMan_ApplyWorldPainter` (its fixture from
    /// `renderstateman`'s test: painter descriptors whose methods are
    /// `MenuSys_IsActive`).
    #[test]
    fn ps2_visual_effects_man_update_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA, GAME_GP};
        check(
            ee::s09::f_002f0a98__generated,
            ps2_visual_effects_man_update,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f0a, 20);
                c.pc = 0x002f_0a98;
                let g = GAME_GP;
                let (vt, ctx, players) = (DATA + 0x6000, DATA + 0x7000, DATA + 0x2300);
                // The 17 effect objects share one vtable whose Reset is a fake.
                for k in 0..17u32 {
                    let obj = DATA + 0x5000 + 0x40 * k;
                    c.bus.write32(DATA + 4 * k, obj);
                    c.bus.write32(obj, vt);
                }
                c.bus.write16(vt + 0x10, (r.next_u32() & 0x3c) as u16);
                c.bus.write32(vt + 0x14, FAKE);
                c.bus.write32(g - 0x854, ctx);
                c.bus.write32(ctx + 0x10d8, vt);
                c.bus.write16(vt + 0x398, (r.next_u32() & 0x3c) as u16);
                c.bus.write32(vt + 0x39c, FAKE + 0x10);
                // world -> app (+0x84) -> players (+0x84; the view count at
                // +0x10) and game module (+0x28) -> handle.
                c.bus.write32(g - 0x848, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 0x84, players);
                c.bus.write32(
                    players + 0x10,
                    match seed % 5 {
                        4 => (-(r.next_u32() as i32 % 3).abs()) as u32,
                        k => k,
                    },
                );
                for i in 0..4u32 {
                    c.bus.write32(g + 0xab8 + 4 * i, r.next_u32() % 4);
                }
                c.bus.write32(DATA + 0x2100 + 0x28, DATA + 0x2200);
                c.bus.write32(DATA + 0x2200, r.next_u32() & 1);
                c.bus.write32(g + 0x2a90, DATA + 0x4000);
                c.bus.write32(g + 0xa70, 0);
                for off in (0..0x40u32).step_by(4) {
                    c.bus.write32(0x004f_a370 + off, DATA + 0x1000);
                }
                c.bus.write32(0x004f_a370 + 0x24, r.edge_f32());
                c.bus.write32(DATA + 0x1000, DATA + 0x1100);
                c.bus.write32(DATA + 0x1104, DATA + 0x1200);
                c.bus.write32(DATA + 0x1100, 0);
                for slot in (0x30..0x1b8u32).step_by(8) {
                    c.bus.write16(DATA + 0x1200 + slot, 0);
                    c.bus.write32(DATA + 0x1200 + slot + 4, 0x002c_c098);
                }
                c.bus.write32(g + 0x1238, (seed % 3 == 0) as u32);
                c.bus.write32(g + 0x14ec, (seed % 7 == 0) as u32);
                c.bus.write32(g + 0x12dc, (seed % 4 == 1) as u32);
                c.bus.write32(g + 0xa6c, r.next_u32());
                for off in [
                    0x1244u32, 0x1248, 0x123c, 0x1240, 0x151c, 0x1524, 0x12e4, 0x12e8, 0x12ec,
                    0x12f0, 0x12f4, 0x1300, 0x1304, 0x14f4, 0x14f8, 0x14fc, 0x1500, 0x1504, 0x1508,
                ] {
                    c.bus.write32(g + off, r.next_u32());
                }
                for off in (0..0x260u32).step_by(4) {
                    c.bus.write32(DATA + 0x4000 + off, r.next_u32());
                }
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    // --------------------------------------- ComputerActorRider_UpdateSpinPlan

    /// `ComputerActorRider_UpdateSpinPlan`: the real flip/spin progress
    /// queries (their attribute factor through a fake BE interface, which
    /// may unwind), must-land on/off, planning on/off, planned flip/spin
    /// zero or not, heights and `.lit4` margins chosen to reach each drop.
    #[test]
    fn computer_actor_rider_update_spin_plan_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA, GAME_GP};
        const AI: u32 = DATA + 0x9_0000;
        const RIDER: u32 = DATA + 0x9_1000;
        const STATE: u32 = DATA + 0x9_3000;
        const MODEL: u32 = DATA + 0x9_4000;
        const BE_LIBRARY: u32 = DATA + 0x8_6000;
        const BE_IFACE: u32 = DATA + 0x8_7000;
        const BE_VTABLE: u32 = DATA + 0x8_7100;
        check(
            ee::s00::f_0010b0e8__generated,
            computer_actor_rider_update_spin_plan,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_b0e8, 2);
                c.pc = 0x0010_b0e8;
                let pick =
                    |r: &mut crate::difftest::Rng, v: &[u32]| v[(r.next_u32() as usize) % v.len()];
                let f = |x: f32| x.to_bits();
                c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, BE_IFACE);
                c.bus.write32(BE_IFACE + 4, BE_VTABLE);
                c.bus.write16(BE_VTABLE + 8, 0);
                c.bus.write32(BE_VTABLE + 0xc, FAKE + 0x80);
                // The progress queries' own `.lit4` constants.
                for off in [0x7330u32, 0x732c, 0x7328, 0x7324, 0x7320, 0x731c] {
                    c.bus.write32(GAME_GP - off, f(0.5));
                }
                // The plan's margins.
                for off in [0x7e14u32, 0x7e18, 0x7e1c, 0x7e20] {
                    c.bus.write32(
                        GAME_GP - off,
                        pick(&mut r, &[f(0.0), f(0.5), f(2.0), f(-1.0)]),
                    );
                }
                c.bus.write32(AI + 0x18, RIDER);
                c.bus.write32(RIDER + 0x77c, STATE);
                let x = STATE + 0x230;
                for off in [0x10u32, 0x14, 0x18, 0x1c, 0x38, 0x3c] {
                    c.bus.write32(x + off, r.f32(4.0));
                }
                c.bus.write32(x + 0x58, MODEL);
                c.bus.write32(MODEL + 0x2ec, r.f32(2.0));
                c.bus.write32(MODEL + 0x86c, r.next_u32() % 4);
                c.bus.write32(MODEL + 0xb34, 1 + r.next_u32() % 3);
                c.bus.write32(AI + 0xe20, pick(&mut r, &[0, 0, 1]));
                c.bus.write32(AI + 0xe10, pick(&mut r, &[0, 1, 1, 1]));
                c.bus
                    .write32(AI + 0xe38, pick(&mut r, &[0, f(1.0), f(1.0)]));
                c.bus
                    .write32(AI + 0xe3c, pick(&mut r, &[0, f(1.0), f(1.0)]));
                c.f[12] = pick(&mut r, &[f(0.5), f(2.0), f(-3.0), f(10.0)]);
                c.g[A0][0] = sx(AI);
            },
        );
    }

    // ------------------------------------------------------------ HUD_DrawTime

    /// A positive float in [lo, hi) as bits.
    pub(crate) fn uf_2(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        (lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)).to_bits()
    }

    // ------------------------------------------- ComputerActorRider_ThinkPrewind

    /// ComputerActorRider_ThinkPrewind: the track query a fake returning
    /// 1..12 events (written on the stack where the buffer lies), with a
    /// prewind-end event first, later or not at all; slower or faster than
    /// the threshold; the scripted start (inside / outside the 50 margin);
    /// a real Path_GetPointAtDistance on a small track and a real
    /// ComputeSteer whose two virtual methods are fakes (may unwind).
    #[test]
    fn computer_actor_rider_think_prewind_matches() {
        use crate::difftest::fakes::{covered, vtable};
        use crate::difftest::{elf_image, SP};
        use prewind::*;
        const CAR: u32 = 0x00a0_0000;
        const PAD: u32 = 0x00a0_8000;
        const RIDER_AT: u32 = 0x00a1_0000;
        const TRK: u32 = 0x00a2_0000;
        const SEGS: u32 = 0x00a2_1000;
        const VECS: u32 = 0x00a3_0000;
        covered(
            ee::s00::f_0010aa70__generated,
            computer_actor_rider_think_prewind,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x10_aa70, 3);
                c.pc = 0x0010_aa70;
                let f = |x: f32| x.to_bits();
                fill(c, &mut r, CAR + CONTROL, 0x20);
                c.bus.write32(CAR + CONTROL, f((r.next_u32() % 60) as f32));
                c.bus.write32(CAR + CONTROL + 0x48, uf_2(&mut r, 0.0, 1.0));
                c.bus.write32(CAR + CONTROL + 0x4c, uf_2(&mut r, 0.0, 1.0));
                c.bus.write32(CAR + RIDER, RIDER_AT);
                c.bus.write32(CAR + SCRIPTED, (seed % 4 == 0) as u32);
                for off in [SCRIPT_FROM, SCRIPT_TO] {
                    put_vec(c, CAR + off, [r.f32(200.0), r.f32(200.0), r.f32(200.0), 0]);
                }
                fill(c, &mut r, PAD, 2);
                // The rider.
                for off in [POS, VELOCITY, AHEAD] {
                    put_vec(
                        c,
                        RIDER_AT + off,
                        [r.f32(60.0), r.f32(60.0), r.f32(60.0), 0],
                    );
                }
                c.bus
                    .write32(RIDER_AT + TRACK_DIST, uf_2(&mut r, 0.0, 200.0));
                c.bus.write32(RIDER_AT + TRACK_SIDE, r.f32(20.0));
                c.bus.write32(RIDER_AT + TRACK, TRK);
                let mvt = vtable(c, &mut r, &[0x10, 0x28]);
                c.bus.write32(RIDER_AT + 0x6c0, mvt);
                // The track: Query (fake) and the path segments.
                let tvt = vtable(c, &mut r, &[SLOT_QUERY]);
                c.bus.write32(TRK + TRACK_VT, tvt);
                c.bus.write32(TRK + 8, r.next_u32() % 4);
                for k in 0..3 {
                    c.bus.write32(TRK + 0xc + 4 * k, r.f32(100.0));
                }
                c.bus.write32(TRK + 0x18, SEGS);
                for i in 0..4 {
                    put_vec(
                        c,
                        SEGS + 16 * i,
                        [r.f32(2.0), r.f32(2.0), r.f32(2.0), uf_2(&mut r, 10.0, 60.0)],
                    );
                }
                // Query's result and the events it "wrote".
                let n = 1 + r.next_u32() % 12;
                c.bus.write32(LOG_RESULTS, n);
                let sel = match seed % 3 {
                    0 => 0,
                    1 => r.next_u32() % n,
                    _ => u32::MAX,
                };
                let buf = SP - 0x160;
                for i in 0..12 {
                    let e = buf + 16 * i;
                    fill(c, &mut r, e, 4);
                    let ty = if i == sel { 0x10 } else { r.next_u32() % 0x10 };
                    c.bus.write32(e, ty);
                    c.bus.write32(e + 0xc, uf_2(&mut r, 0.0, 250.0));
                }
                // ComputeSteer's axis and motion vectors.
                put_vec(c, VECS, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                put_vec(c, VECS + 0x10, [0, 0, 0, 0]);
                c.bus.write32(LOG_RESULTS + 4, VECS);
                c.bus.write32(LOG_RESULTS + 8, VECS + 0x10);
                c.g[A0][0] = CAR as u64;
                c.g[A1][0] = PAD as u64;
            },
        );
    }

    /// `PATH_Update` with its real callees: locked / uninitialised / busy
    /// early outs, and the full tick over four players that are inactive,
    /// active without a handle, or active with a handle (an empty track
    /// list, so `PATH_SelectMasterTrack` and `PATH_UpdateTracks` take their
    /// short paths; no platform streams).
    #[test]
    fn path_update_matches() {
        check(
            ee::s14::f_003d5128__generated,
            path_update,
            120,
            &|c, seed| {
                let mut r = Rng(0x003d_5128 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                c.pc = 0x003d_5128;
                let gate = seed % 6;
                c.bus.write8(
                    0x0044_f42d,
                    (gate == 0) as u8 * (1 + r.next_u32() as u8 % 3),
                );
                c.bus.write8(0x0044_f42c, (gate != 1) as u8);
                c.bus
                    .write32(0x0044_f430, if gate == 2 { r.next_u32() | 1 } else { 0 });
                c.bus.write32(0x0045_0dcc, r.next_u32()); // timer count
                c.bus.write32(0x0045_0dc8, 1 + r.next_u32() % 0x1000); // timer period
                for i in 0..4u32 {
                    let p = 0x0051_7610 + 0x928 * i;
                    let kind = r.next_u32() % 3;
                    c.bus
                        .write32(p + 0x34, (kind != 0) as u32 * (1 + r.next_u32() % 5));
                    c.bus
                        .write32(p + 0x30, if kind == 2 { DATA + 0x100 * i } else { 0 });
                }
                c.g[A0][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
                if seed % 2 == 0 {
                    c.g[A0][0] = (c.g[A0][0] & !0xff) | b'B' as u64;
                }
            },
        );
    }

    /// `Rider_TryEnterBoardPress` with its real callees: dir 0 / positive /
    /// negative / edge floats, press states 3, 4 and others, main state 0
    /// (the VU0 alignment test, both signs) or not. `AnimController_PlayAnim`
    /// reaches its virtual call through a test vtable at a fake forced to
    /// return 0x207 (its early exit; sometimes it unwinds); `SetSubState`'s
    /// owner has a state id past the state tables.
    #[test]
    fn rider_try_enter_board_press_matches() {
        check(
            ee::s00::f_001161d0__generated,
            rider_try_enter_board_press,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1161d0, 1);
                c.pc = 0x0011_61d0;
                let rider = DATA;
                let owner = DATA + 0x1000;
                let ctl = DATA + 0x2000;
                let vt = DATA + 0x3000;
                c.g[A0][0] = rider as u64;
                c.f[12] = match seed % 5 {
                    0 => 0,
                    1 => 0x8000_0000,
                    2 => r.f32(4.0) & !0x8000_0000,
                    3 => r.f32(4.0) | 0x8000_0000,
                    _ => r.edge_f32(),
                };
                let press = match (seed / 5) % 4 {
                    0 => 3,
                    1 => 4,
                    2 => r.next_u32(),
                    _ => 0,
                };
                c.bus.write32(rider + 0x328, press);
                c.bus.write32(rider + 0x77c, owner);
                let main = if (seed / 20) % 2 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 9
                };
                c.bus.write32(owner + 0xde0, main);
                c.bus.write32(owner + 0xde4, 0xe + r.next_u32() % 8);
                for k in 0..4 {
                    c.bus.write32(rider + 0x1e0 + 4 * k, r.f32(3.0));
                    c.bus.write32(rider + 0x3a0 + 4 * k, r.f32(30.0));
                }
                c.bus.write32(rider + 0x784, ctl);
                c.bus.write32(ctl + 0x58, vt);
                c.bus.write16(vt + 0x20, 0x10);
                c.bus.write32(vt + 0x24, fakes::FAKE);
                c.bus.write32(fakes::LOG_RESULTS, 0x207);
            },
        );
    }

    /// `SNDMIDI_Service` with its real callees over 16 slots: playing or
    /// not; no fade, a fade that keeps playing (no voices of the slot, so
    /// no volume calls) or one that ends (stop with an invalid handle,
    /// returns at once); scaled time before, at or 1..3 events past the next
    /// event. Events are program changes behind a non-zero delta counter
    /// (+0x27), so `SNDMIDI_ProcessEvent` only counts down.
    #[test]
    fn sndmidi_service_matches() {
        check(
            ee::s13::f_003b96b8__generated,
            sndmidi_service,
            100,
            &|c, seed| {
                let mut r = Rng(0x003b_96b8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                c.pc = 0x003b_96b8;
                // Voice table: no voice belongs to any slot.
                for v in 0..24 {
                    c.bus.write8(0x0050_a820 + 8 * v + 4, 0xff);
                }
                let events = DATA;
                put_words(c, events, &[0x0005_c000]);
                for i in 0..16u32 {
                    let e = 0x0050_a120 + 0x70 * i;
                    let k = r.next_u32();
                    let playing = k % 5 != 0;
                    c.bus
                        .write8(e + 0x28, if playing { 3 } else { (k % 3) as u8 });
                    match (k >> 4) % 3 {
                        0 => c.bus.write32(e + 0x18, 0),
                        1 => {
                            // Fading up, still below the target.
                            c.bus.write32(e + 0x18, 5);
                            c.bus.write32(e + 0x14, 10);
                            c.bus.write32(e + 0x1c, 100);
                        }
                        _ => {
                            // Fading out to -1: stop.
                            c.bus.write32(e + 0x18, (-5i32) as u32);
                            c.bus.write32(e + 0x14, 2);
                            c.bus.write32(e + 0x1c, u32::MAX);
                            c.bus.write32(e + 0x10, u32::MAX);
                        }
                    }
                    let ticks = r.next_u32() % 1000;
                    let tempo = r.next_u32() % 200;
                    c.bus.write32(e, ticks);
                    c.bus.write16(e + 0x24, tempo as u16);
                    let scaled = ticks * tempo / 100;
                    let next = match (k >> 8) % 5 {
                        0 => scaled + 1 + r.next_u32() % 10,
                        n => scaled.wrapping_sub(n - 1),
                    };
                    c.bus.write32(e + 0x4, next);
                    c.bus.write32(e + 0x8, events);
                    c.bus.write8(e + 0x27, 0xff);
                }
            },
        );
    }

    /// `libpad_GetDmaBuffer` for every port/slot, either half newer,
    /// interrupts on or off (the cache sync's two paths).
    #[test]
    fn libpad_get_dma_buffer_matches() {
        check(
            ee::s15::f_003ff9a8__generated,
            libpad_get_dma_buffer,
            120,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x3f_f9a8, 0);
                c.cop0[12] = if seed % 2 == 0 { 0 } else { 0x1_0000 };
                let port = (seed / 2) % 2;
                let slot = (seed / 4) % 4;
                let buf = DATA + 0x1000 + 0x100 * (r.next_u32() % 8);
                c.bus.write32(PAD_PORTS + 0x70 * port + 0x1c * slot, buf);
                c.bus.write32(buf + 0x58, r.next_u32() % 4);
                c.bus.write32(buf + 0xd8, r.next_u32() % 4);
                c.g[A0][0] = port as u64;
                c.g[A1][0] = slot as u64;
                c.pc = 0x003f_f9a8;
            },
        );
    }

    pub(crate) use crate::game::visualfx::boardwakefx::tests::oc238_uf;

    /// WScriptFn_GetObjectSlot32: defaults initialised or not; 0..3 passed
    /// arguments (none, or n <= 0) with matching / int-for-float / other
    /// types; ids -1 (the player, present or not) and class/handle ids with
    /// a missing class, a zero handle or an object; objects with or without
    /// a controller, whose slot-32 method is a fake (may unwind).
    #[test]
    fn wscript_fn_get_object_slot32_matches() {
        covered(
            ee::s10::f_003019c8__generated,
            wscript_fn_get_object_slot32,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_19c8, 1);
                c.pc = 0x0030_19c8;
                let gp = |off: i32| GAME_GP.wrapping_add(off as u32);
                let ready = seed % 4 != 0;
                c.bus.write32(gp(0x25fc), ready as u32);
                // An object id: -1 (the player) or handle << 8 | class.
                let some_id = |r: &mut Rng| -> u32 {
                    if r.next_u32() % 3 == 0 {
                        0xffff_ffff
                    } else {
                        (r.next_u32() % 16) << 8 | r.next_u32() % 8
                    }
                };
                fill(c, &mut r, 0x004f_bc00, 3);
                let default_id = some_id(&mut r);
                c.bus.write32(0x004f_bc00, default_id);
                // Declared types: id int, time float, a third slot.
                c.bus.write32(gp(0xba8), r.next_u32() % 2);
                c.bus
                    .write32(gp(0xba8 + 4), if seed % 5 == 0 { 1 } else { 2 });
                c.bus.write32(gp(0xba8 + 8), r.next_u32() % 3);
                c.bus.write32(gp(-0x3368), (1.0f32 / 30.0).to_bits());
                // Objects.
                let mgr = DATA + 0x100;
                let m = DATA + 0x200;
                let tables = DATA + 0x300;
                let class = DATA + 0x400;
                let handles = DATA + 0x800;
                let world = DATA + 0xc00;
                let obj = DATA + 0x1000;
                let ctrl = DATA + 0x1100;
                let vt = DATA + 0x1200;
                c.bus.write32(gp(0x16c8), mgr);
                c.bus.write32(mgr, m);
                c.bus.write32(m + 8, tables);
                for k in 0..8 {
                    let present = r.next_u32() % 4 != 0;
                    c.bus
                        .write32(tables + 4 * k, if present { class } else { 0 });
                }
                c.bus.write32(class + 0x1c, handles);
                for k in 0..16 {
                    let word = if r.next_u32() % 4 == 0 {
                        r.next_u32() & 0xff
                    } else {
                        ((obj >> 2) << 8) | (r.next_u32() & 0xff)
                    };
                    c.bus.write32(handles + 4 * k, word);
                }
                c.bus.write32(gp(0xce8), world);
                c.bus
                    .write32(world + 0x290, if seed % 7 == 0 { 0 } else { obj });
                c.bus
                    .write32(obj + 0xc, if seed % 5 == 1 { 0 } else { ctrl });
                c.bus.write32(ctrl + 0xc, vt);
                c.bus
                    .write16(vt + 0x100, ((r.next_u32() & 0x3f) * 4) as u16);
                c.bus.write32(vt + 0x104, FAKE);
                // Arguments {idx, val, _, type}.
                let args = DATA + 0x2000;
                let n = match seed % 6 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    _ => 1 + r.next_u32() % 3,
                };
                for k in 0..3 {
                    let a = args + 0x10 * k;
                    let idx = if k == 0 {
                        r.next_u32() % 2
                    } else {
                        r.next_u32() % 3
                    };
                    c.bus.write32(a, idx);
                    let val = if idx == 0 {
                        some_id(&mut r)
                    } else if r.next_u32() % 2 == 0 {
                        r.next_u32() % 1000
                    } else {
                        oc238_uf(&mut r, -100.0, 100.0).to_bits()
                    };
                    c.bus.write32(a + 4, val);
                    c.bus.write32(a + 8, r.next_u32());
                    c.bus.write32(a + 0xc, r.next_u32() % 3);
                }
                c.g[A0][0] = (DATA + 0x3000) as u64;
                c.g[A1][0] = sx(n);
                c.g[A2][0] = args as u64;
            },
        );
    }

    // ------------------------------------------------------------------ expf

    /// `expf`: arguments in range, overflowing, underflowing, at the
    /// thresholds and not finite, under each `_LIB_VERSION` (IEEE, SVID,
    /// XOPEN, POSIX); the real libm helpers (`matherr` returns 0).
    #[test]
    fn expf_matches() {
        use super::libm_wrap::*;
        use crate::difftest::{check, elf_image, scramble, Rng, GAME_GP};
        let f = |x: f32| x.to_bits();
        check(ee::s15::f_0040d758__generated, expf, 300, &|c, seed| {
            let mut r = Rng(0x40d7_5800 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0040_d758;
            let version = [0xffff_ffffu32, 0, 1, 2][(seed % 4) as usize];
            c.bus.write32(LIB_VERSION, version);
            let x = match (seed / 4) % 9 {
                0 => f(1.5),
                1 => f(-2.25),
                2 => f(88.8),
                3 => f(500.0),
                4 => f(-104.5),
                5 => f(-2000.0),
                6 => f(88.72284),
                7 => r.edge_f32(),
                _ => r.f32(120.0),
            };
            c.f[12] = x;
        });
    }

    /// ComputerActorRider_FindBoardPressMarker: no markers (the vtable
    /// method is the real leaf at 0x1406c0 returning 0), and a fake returning
    /// 1..12 hits with or without a board-press marker (positive or not)
    /// among them.
    #[test]
    fn computer_actor_rider_find_board_press_marker_matches() {
        covered(
            ee::s00::f_0010bbf8__generated,
            computer_actor_rider_find_board_press_marker,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_bbf8, 1);
                c.pc = 0x0010_bbf8;
                let this = DATA;
                let rider = DATA + 0x1000;
                let query = DATA + 0x2000;
                let press = DATA + 0x3000;
                c.bus.write32(this + CAR_RIDER, rider);
                c.bus.write32(rider + RIDER_QUERY, query);
                c.bus.write32(rider + RIDER_TRACK_POS, r.edge_f32());
                c.bus.write32(rider + RIDER_TRACK_DIST, r.f32(5000.0));
                let vt = vtable(c, &mut r, &[QUERY_VT_MARKERS]);
                c.bus.write32(query + QUERY_VTABLE, vt);
                if seed % 8 == 0 {
                    c.bus.write32(vt + QUERY_VT_MARKERS + 4, 0x0014_06c0);
                }
                let n = r.next_u32() % 12 + 1;
                c.bus.write32(LOG_RESULTS, n);
                // The hit buffer the (fake) query leaves on the stack.
                let buf = crate::difftest::SP - 0xe0;
                for i in 0..12 {
                    let ty = if r.next_u32() % 5 == 0 {
                        0x13
                    } else {
                        r.next_u32() % 0x20
                    };
                    let ty = if seed % 3 == 0 && ty == 0x13 {
                        0x12
                    } else {
                        ty
                    };
                    c.bus.write32(buf + HIT_SIZE * i, ty);
                    c.bus
                        .write32(buf + HIT_SIZE * i + 4, (r.next_u32() % 4) << 30);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = press as u64;
            },
        );
    }

    /// EATimer_Rcnt1Handler: the timer 1 registers of the test machine, a
    /// random period and tick counters, and 8 callback slots that are empty
    /// or fakes.
    #[test]
    fn ea_timer_rcnt1_handler_matches() {
        covered(
            ee::s14::f_003e4db8__generated,
            ea_timer_rcnt1_handler,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_4db8, 8);
                c.pc = 0x003e_4db8;
                c.bus.write16(EATIMER_PERIOD, r.next_u32() as u16);
                c.bus.write32(EATIMER_TICKS, r.next_u32());
                c.bus.write32(EATIMER_TICKS2, r.next_u32());
                c.bus.write32(EATIMER_IN_CALLBACK, r.next_u32());
                c.bus.write32(T1_COMP, r.next_u32() & 0xffff);
                c.bus.write32(T1_COUNT, r.next_u32() & 0xffff);
                for i in 0..8 {
                    let cb = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        FAKE + 0x10 * i
                    };
                    c.bus.write32(EATIMER_CALLBACKS + 4 * i, cb);
                }
            },
        );
    }

    // ---------------------------------------------------------------------- logf

    /// `logf` under every `_LIB_VERSION` (-1 `_IEEE_`, 0 `_SVID_`, 1
    /// `_XOPEN_`, 2 `_POSIX_`) with positive, NaN, +-0, negative and
    /// infinite inputs; the real `__ieee754_logf`, soft-float helpers,
    /// `matherr` and `__getreent` run on the ELF image.
    #[test]
    fn logf_matches() {
        check(ee::s15::f_0040da10__generated, logf, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0040_da10, 0);
            elf_image(c);
            c.pc = 0x0040_da10;
            let version = match seed % 5 {
                0 => u32::MAX,
                1 => 0,
                2 => 1,
                _ => 2,
            };
            c.bus.write32(0x004a_0000 - 0x6978, version);
            c.f[12] = match seed / 5 % 7 {
                0 => r.f32(1000.0) & 0x7fff_ffff,
                1 => 0x7fc0_0000 | (r.next_u32() & 0x803f_ffff),
                2 => (r.next_u32() & 1) << 31,
                3 => r.f32(1000.0) | 0x8000_0000,
                4 => 0xff80_0000,
                5 => 0x7f80_0000,
                _ => r.edge_f32(),
            };
        });
    }

    // -------------------------------------- ComputerActorRider_CruiseChaseRider

    /// `ComputerActorRider_CruiseChaseRider` on the cruise fixture of
    /// `ComputerActorRider_CruiseFollowPath` (same steering, crouch, boost,
    /// jump-marker and target-rider callees, all real): the chased rider is
    /// AI slot 1..3, whose path interface returns a random velocity (fake)
    /// and is ready or not (fake / the real zero leaf); headings behind and
    /// ahead, record distances zero / positive / negative / edge, the same
    /// or another AI path than the model (real path re-selection).
    #[test]
    fn computer_actor_rider_cruise_chase_rider_matches() {
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::difftest::put_vec;
        use crate::game::ai::computer::tests::{
            cruise_setup, MODEL, PMF, RIDER_2, RIDER_ENTRY, WORLD,
        };
        check(
            ee::s00::f_00100f88__generated,
            computer_actor_rider_cruise_chase_rider,
            240,
            &|c, seed| {
                cruise_setup(c, seed);
                let mut r = Rng(0x0010_0f88 ^ (seed as u64) << 20);
                c.pc = 0x0010_0f88;
                // The chased rider: AI slot 1..3 (`ai + 0x28 + 4*slot`).
                let slot = 1 + seed % 3;
                let ai = WORLD + 0x200;
                c.bus.write32(RIDER_2 + 0xe70, slot);
                for i in 1..4u32 {
                    c.bus
                        .write32(ai + 0x28 + 4 * i, RIDER_ENTRY + 0x100 * (i - 1));
                }
                let entry = RIDER_ENTRY + 0x100 * (slot - 1);
                c.bus.write32(entry + 0x6c0, PMF);
                c.bus.write32(
                    entry + 0xab8,
                    if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
                );
                // Its path interface: velocity (first fake call; the forced
                // result points at a random vector) and "ready".
                let vel = DATA + 0x9_0000;
                c.bus.write16(PMF + 0x10, 0);
                c.bus.write32(PMF + 0x14, FAKE + 0x30);
                c.bus.write16(PMF + 0x48, 0);
                c.bus.write32(
                    PMF + 0x4c,
                    if seed % 4 == 0 {
                        0x0014_06c0
                    } else {
                        FAKE + 0x40
                    },
                );
                c.bus.write32(LOG_RESULTS, vel);
                put_vec(c, vel, [r.f32(150.0), r.f32(150.0), r.f32(150.0), 0]);
                // Headings and the slot record (distance, heading).
                c.bus.write32(MODEL + 0x4cc, r.f32(3.2));
                let rec = MODEL + 0x24 * slot;
                c.bus.write32(
                    rec + 0x8,
                    match seed % 5 {
                        0 => 0,
                        1 => r.f32(400.0) & 0x7fff_ffff,
                        2 => r.f32(400.0) | 0x8000_0000,
                        3 => r.f32(40.0),
                        _ => r.edge_f32(),
                    },
                );
                c.bus.write32(rec + 0xc, r.f32(6.4));
            },
        );
    }

    /// `EIntr` with interrupts on and off (random other Status bits).
    #[test]
    fn e_intr_matches() {
        crate::difftest::check(ee::s15::f_0042c0c0__generated, e_intr, 64, &|c, seed| {
            let mut r = Rng(0x42c0_c000 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let status = r.next_u32() & !(STATUS_EIE as u32);
            c.cop0[12] = if seed % 2 == 0 {
                status
            } else {
                status | STATUS_EIE as u32
            };
            c.pc = 0x0042_c0c0;
        });
    }

    /// `ComputerActorRider_SetCruiseCrouch`: rider velocities below and
    /// above the fast threshold, speeds below and above 20, the real
    /// `.lit4` constants; a random pad word (the low 26 bits kept).
    #[test]
    fn computer_actor_rider_set_cruise_crouch_matches() {
        use super::cruise::*;
        use crate::difftest::GAME_GP;
        crate::difftest::check(
            ee::s00::f_0010c0a8__generated,
            computer_actor_rider_set_cruise_crouch,
            300,
            &|c, seed| {
                let mut r = Rng(0x0010_c0a8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0010_c0a8;
                c.g[GP][0] = GAME_GP as u64;
                let (me, rider, pad) = (DATA, DATA + 0x1000, DATA + 0x2000);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(pad);
                c.bus.write32(GAME_GP - LIT_FAST, 833.333_4f32.to_bits());
                c.bus.write32(GAME_GP - LIT_CROUCH, 0.97f32.to_bits());
                c.bus.write32(me + RIDER, rider);
                let v = if seed % 2 == 0 { 400.0 } else { 1200.0 };
                for i in 0..4 {
                    c.bus.write32(rider + RIDER_VELOCITY + 4 * i, r.f32(v));
                }
                let speed = if seed % 3 == 0 {
                    r.f32(20.0)
                } else {
                    r.f32(100.0)
                };
                c.bus
                    .write32(me + SPEED, if seed % 5 == 0 { r.edge_f32() } else { speed });
                c.bus.write32(pad, r.next_u32());
            },
        );
    }

    /// `memcmp`: lengths 0..80, aligned or misaligned pointers, equal
    /// buffers or one differing byte anywhere (inside or past `n`).
    #[test]
    fn memcmp_matches() {
        const A: u32 = DATA + 0x100;
        const B: u32 = DATA + 0x400;
        check(ee::s15::f_00415fc8__generated, memcmp, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x415fc8, 0);
            let (oa, ob) = match seed % 4 {
                0 | 1 => (0, 0),
                2 => (r.next_u32() % 16, 0),
                _ => (0x10 * (r.next_u32() % 2), r.next_u32() % 16),
            };
            let n = match seed % 5 {
                0 => r.next_u32() % 16,
                _ => r.next_u32() % 80,
            };
            for i in 0..0x60 {
                let v = r.next_u32();
                c.bus.write32(A + 4 * i, v);
                c.bus.write32(B + 4 * i, v);
            }
            // Shift B so that A+oa and B+ob hold the same bytes.
            for i in 0..0x100u32 {
                let v = c.bus.read8(A + oa + i);
                c.bus.write8(B + ob + i, v);
            }
            if seed % 3 != 0 {
                let at = r.next_u32() % (n + 4);
                let v = c.bus.read8(B + ob + at);
                c.bus
                    .write8(B + ob + at, v ^ (1 + r.next_u32() % 255) as u8);
            }
            c.g[A0][0] = sx(A + oa);
            c.g[A1][0] = sx(B + ob);
            c.g[A2][0] = n as u64;
        });
    }

    /// Stream file fixture: handle at DATA -> header at DATA+0x1000 (starting
    /// with the magic 'MRTS'), buffer DATA+0x2000..DATA+0x3000, chunk inside or outside it.
    /// The mutex syscalls go to the fakes (some unwind). The real
    /// `StreamFile_IssueRead` takes its short path: no pending request, the
    /// ring full enough that it only marks the reader stalled again.
    #[test]
    fn stream_file_release_chunk_matches() {
        use crate::difftest::fakes::{covered, machine};
        covered(
            ee::s14::f_003e13e8__generated,
            stream_file_release_chunk,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e13e8, 10);
                c.pc = 0x003e_13e8;
                let handle = DATA;
                let header = DATA + 0x1000;
                let start = DATA + 0x2000;
                let end = DATA + 0x3000;
                let case = seed % 9;
                c.g[A0][0] = if case == 0 { 0 } else { handle as u64 };
                c.bus.write32(handle, header);
                let magic = 0x4d52_5453;
                c.bus.write32(
                    header,
                    if case == 1 {
                        magic ^ (1 + r.next_u32() % 0xff)
                    } else {
                        magic
                    },
                );
                c.bus.write32(header + sfile::BUFFER_START, start);
                c.bus.write32(header + sfile::BUFFER_END, end);
                let chunk = match case {
                    2 => start - 4 * (1 + r.next_u32() % 4),
                    3 => end - 8 + 4 * (1 + r.next_u32() % 4),
                    4 => end - 8,
                    _ => start + 0x100,
                };
                c.g[A1][0] = chunk as u64;
                let status = if case == 5 {
                    0xffff_fffe
                } else {
                    r.next_u32() & 0xff
                };
                c.bus.write32(chunk + sfile::CHUNK_STATUS, status);
                c.bus
                    .write32(chunk + sfile::CHUNK_BYTES, r.next_u32() & 0xfff);
                // Read state: mostly stalled (2), sometimes reading (1) or idle.
                let state = match r.next_u32() % 4 {
                    0 => r.next_u32() % 2,
                    _ => 2,
                };
                c.bus.write32(header + sfile::READ_STATE, state);
                c.bus
                    .write32(header + sfile::READ_SIZE, r.next_u32() & 0xffff);
                c.bus
                    .write32(header + sfile::READ_SIZE_LOW, r.next_u32() & 0xffff);
                c.bus
                    .write32(header + sfile::LOW_WATER_HIT, r.next_u32() & 1);
                // StreamFile_ConsumeBytes: buffered count and low-water mark (kept
                // above the count in state 1, so it never reprioritises a request).
                c.bus.write32(header + 0x4c, r.next_u32() & 0xfffff);
                let low = if state == 1 {
                    0x7fff_ffff
                } else {
                    r.next_u32() & 0xfffff
                };
                c.bus.write32(header + 0x44, low);
                // Mutex: owner, depth, semaphore (owner never the fake thread id).
                c.bus
                    .write32(header + 0x8, 0x7700_0000 | r.next_u32() & 0xffff);
                c.bus.write32(header + 0xc, r.next_u32() & 3);
                c.bus.write32(header + 0x10, r.next_u32() & 0xff);
                // StreamFile_IssueRead short path.
                c.bus.write32(header + 0x50, DATA + 0x2200);
                c.bus.write32(header + 0x54, DATA + 0x2200);
                c.bus.write32(header + 0x58, DATA + 0x2100);
                c.bus.write32(header + 0x5c, DATA + 0x4000);
                c.bus.write32(DATA + 0x4000 + 0xc, 0);
                c.bus.write32(header + 0x17c, 0x1000);
            },
        );
    }

    /// A HUD at DATA with random texture coordinates, a (sometimes
    /// unaligned) colour, pos/size in DATA; widths below and above 72 (the
    /// three-segment path), t on both sides of the triangle-wave peak. The
    /// world's draw callback (`HUD_DrawPanelSegment`'s vtable call) is a fake
    /// that sometimes unwinds.
    #[test]
    fn hud_draw_stretched_panel_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        covered(
            ee::s05::f_0021e7e0__generated,
            hud_draw_stretched_panel,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x21e7e0, 3);
                c.pc = 0x0021_e7e0;
                let hud = DATA;
                let colour = DATA + 0x100 + (seed % 4) * 4 + (seed % 3);
                let pos = DATA + 0x200;
                let size = DATA + 0x210;
                let world = DATA + 0x1000;
                let draw_state = DATA + 0x3000;
                let vtable = DATA + 0x5000;
                c.g[A0][0] = hud as u64;
                c.g[A1][0] = colour as u64;
                c.g[A2][0] = pos as u64;
                c.g[A3][0] = size as u64;
                c.bus.write32(hud + hud_panel::TEXTURE_ID, r.next_u32());
                for off in [
                    hud_panel::TEX_V0,
                    hud_panel::TEX_U0,
                    hud_panel::TEX_U1,
                    hud_panel::TEX_V1,
                ] {
                    c.bus
                        .write32(hud + off, ((r.next_u32() % 1000) as f32 / 1000.0).to_bits());
                }
                // Byte by byte: the colour may be unaligned.
                for i in 0..4 {
                    let bits = ((r.next_u32() % 256) as f32).to_bits();
                    for (k, b) in bits.to_le_bytes().into_iter().enumerate() {
                        c.bus.write8(colour + 4 * i + k as u32, b);
                    }
                }
                c.bus.write32(pos, r.f32(400.0));
                c.bus.write32(pos + 4, r.f32(300.0));
                let width = if seed % 2 == 0 {
                    20 + r.next_u32() % 50
                } else {
                    75 + r.next_u32() % 300
                };
                c.bus.write32(size, (width as f32).to_bits());
                c.bus
                    .write32(size + 4, ((10 + r.next_u32() % 120) as f32).to_bits());
                // t in [0, 2), scales around 1 (rarely exactly 1), alpha range.
                c.f[12] = ((r.next_u32() % 2000) as f32 / 1000.0).to_bits();
                c.f[13] = (0.8 + (r.next_u32() % 400) as f32 / 1000.0).to_bits();
                c.f[14] = (0.8 + (r.next_u32() % 400) as f32 / 1000.0).to_bits();
                if seed % 7 == 0 {
                    c.f[13] = 1.0f32.to_bits();
                }
                c.f[15] = ((r.next_u32() % 500) as f32 / 1000.0).to_bits();
                c.f[16] = (0.5 + (r.next_u32() % 500) as f32 / 1000.0).to_bits();
                c.bus.write32(GAME_GP - 0x854, world);
                c.bus
                    .write32(world + hud_panel::WORLD_DRAW_STATE, draw_state);
                c.bus.write32(world + 0x10d8, vtable);
                c.bus.write16(vtable + 0x238, (r.next_u32() & 0x3c) as u16);
                c.bus.write32(vtable + 0x23c, FAKE);
            },
        );
    }

    /// The rider on a path node (identity or random rotation), the probe
    /// offset, blend and short-radius flag; a target near the probe (a hit),
    /// at mid range or far; a direction parallel to the probe (identity
    /// rotation, dir = +x) or random; the overlap list with one plain rail
    /// whose mask passes or not, or empty. The overlap query is real (the
    /// same bounded data as `rider_find_rail_matches`).
    #[test]
    fn rider_probe_rail_matches() {
        use crate::difftest::{check, elf_image, put_vec};
        check(
            ee::s00::f_001086b8__generated,
            rider_probe_rail,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = fakes::machine(c, seed, 0x1086_b8, 0);
                let me = DATA + 0x1000;
                let path = DATA + 0x3000;
                let nodes = DATA + 0x3100;
                let list = DATA + 0x4000;
                let rail = DATA + 0x5000;
                let rail_flags = DATA + 0x5200;
                let target = DATA + 0x7000;
                let dir = DATA + 0x7100;
                c.bus.write32(me + PROBE_RAIL_PATH, path);
                c.bus.write32(path + 0x2c, nodes);
                let node = r.next_u32() % 4;
                c.bus.write32(me + PROBE_RAIL_NODE, node);
                let identity = seed % 5 == 0;
                let mut node_pos = [0f32; 3];
                for i in 0..4 {
                    let p = [r.f32(100.0), r.f32(100.0), r.f32(100.0)];
                    if i == node {
                        node_pos = p.map(f32::from_bits);
                    }
                    put_vec(c, nodes + 0x20 * i, [p[0], p[1], p[2], 0]);
                    let q = if identity {
                        [0, 0, 0, 1.0f32.to_bits()]
                    } else {
                        [r.f32(0.7), r.f32(0.7), r.f32(0.7), r.f32(1.0)]
                    };
                    put_vec(c, nodes + 0x20 * i + 0x10, q);
                }
                let off = [r.f32(5.0), r.f32(5.0), r.f32(5.0)];
                put_vec(c, me + PROBE_RAIL_OFFSET, [off[0], off[1], off[2], 0]);
                c.bus.write32(
                    me + PROBE_RAIL_BLEND,
                    (f32::from_bits(r.f32(0.5)) + 0.5).to_bits(),
                );
                c.bus.write32(me + PROBE_RAIL_SHORT, r.next_u32() & 1);
                // The target: near the probe point, mid range, or far.
                let spread = [8.0f32, 60.0, 400.0][(seed / 5 % 3) as usize];
                let t: Vec<u32> = (0..3)
                    .map(|i| {
                        let d = f32::from_bits(r.f32(spread));
                        (node_pos[i] + f32::from_bits(off[i]) + d).to_bits()
                    })
                    .collect();
                put_vec(c, target, [t[0], t[1], t[2], 0]);
                let d = if identity && seed % 2 == 0 {
                    [1.0f32.to_bits(), 0, 0, 0]
                } else {
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]
                };
                put_vec(c, dir, d);
                // The overlap list: one plain rail (type 1) or none.
                c.bus.write32(me + PROBE_RAIL_OVERLAP, list);
                c.bus.write32(list + 0x210, (seed % 7 != 0) as u32);
                c.bus.write32(list + 0x214, rail);
                c.bus.write32(rail + 8, 1);
                c.bus.write32(rail + 0x68, rail_flags);
                c.bus.write32(rail_flags + 0x1c, (seed % 11 != 5) as u32);
                c.bus.write32(rail_flags + 0x28, r.next_u32());
                for off in [0x6c, 0x70, 0x74] {
                    c.bus.write32(rail + off, (-1.0e4f32).to_bits());
                }
                for off in [0x78, 0x7c, 0x80] {
                    c.bus.write32(rail + off, 1.0e4f32.to_bits());
                }
                for i in 0..4 {
                    let w = if i == 3 { 1.0f32.to_bits() } else { 0 };
                    let base = if i == 3 { node_pos } else { [0.0; 3] };
                    let v: Vec<u32> = (0..3)
                        .map(|k| (base[k] + f32::from_bits(r.f32(50.0))).to_bits())
                        .collect();
                    put_vec(c, rail + 0x10 + 0x10 * i, [v[0], v[1], v[2], w]);
                }
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(target);
                c.g[A2][0] = sx(dir);
                c.pc = 0x0010_86b8;
            },
        );
    }

    /// RaceStateRace_Update: no riders (all finished), riders all finished
    /// or some still racing (`rider->physics->+0x470` progress below 0);
    /// the AI already in the finished state or not (the real
    /// AI_SelectRaceState runs through its jump table); the real
    /// GameModule_RequestState stores the request.
    #[test]
    fn race_state_race_update_matches() {
        check(
            ee::s00::f_00113db0__generated,
            race_state_race_update,
            300,
            &|c, seed| {
                let mut r = Rng(0x0011_3db0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0011_3db0;
                // AI_SelectRaceState's jump table (states 1..7).
                put_words(
                    c,
                    0x0045_6ca0,
                    &[
                        0x0011_3b74,
                        0x0011_3b94,
                        0x0011_3ba8,
                        0x0011_3bbc,
                        0x0011_3bd0,
                        0x0011_3be4,
                        0x0011_3bf8,
                    ],
                );
                let ai = DATA;
                let state = if seed % 3 == 0 { 6 } else { r.next_u32() % 9 };
                c.bus.write32(ai, state);
                c.bus.write32(ai + RACE_STATE_FRAMES, r.next_u32());
                c.bus.write32(ai + 0x98, r.next_u32());
                let riders = if seed % 4 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 4
                };
                c.bus.write32(ai + 0x7c, riders);
                let all_done = seed % 2 == 0;
                for i in 0..riders {
                    let rider = DATA + 0x1000 + 0x100 * i;
                    let phys = DATA + 0x2000 + 0x1000 * i;
                    c.bus.write32(ai + 0x40 + 4 * i, rider);
                    c.bus.write32(rider + 0x18, phys);
                    let racing = !all_done && r.next_u32() % 2 == 0;
                    let v = (r.next_u32() % 100) as f32 + 0.5;
                    let v = if racing {
                        -v
                    } else if r.next_u32() % 4 == 0 {
                        0.0
                    } else {
                        v
                    };
                    c.bus.write32(phys + 0x470, v.to_bits());
                }
                let world = DATA + 0x8000;
                c.bus.write32(GAME_GP - RACE_WORLD_GP, world);
                c.bus.write32(world + RACE_WORLD_GAME_MODULE, DATA + 0x9000);
                c.g[A1][0] = sx(ai);
            },
        );
    }

    /// StreamFile_ConsumeBytes: the count already below the low-water mark,
    /// staying above it, or crossing it (streaming mode or not: the real
    /// EAFile_SetRequestPriority with a null or unknown handle); the real
    /// EAMutex lock / unlock (syscalls faked), owned by the caller or not.
    #[test]
    fn stream_file_consume_bytes_matches() {
        check(
            ee::s14::f_003dfc48__generated,
            stream_file_consume_bytes,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003d_fc48, 0);
                c.pc = 0x003d_fc48;
                let st = DATA;
                let low = r.next_u32() % 0x10000;
                let old = match seed % 3 {
                    0 => low.wrapping_sub(1 + r.next_u32() % 0x100),
                    _ => low + r.next_u32() % 0x1000,
                };
                let n = match seed % 4 {
                    0 => r.next_u32() % 0x10,
                    1 => r.next_u32() % 0x2000,
                    2 => old.wrapping_sub(low) + 1 + r.next_u32() % 4,
                    _ => r.next_u32(),
                };
                c.bus.write32(st + STREAM_FILE_BUFFERED, old);
                c.bus.write32(st + STREAM_FILE_LOW_WATER, low);
                c.bus.write32(st + STREAM_FILE_NEEDS_REFILL, r.next_u32());
                c.bus.write32(
                    st + STREAM_FILE_MODE,
                    if seed % 2 == 0 { 1 } else { r.next_u32() % 3 },
                );
                c.bus.write32(st + STREAM_FILE_PRIORITY, r.next_u32() % 8);
                c.bus.write32(
                    st + STREAM_FILE_HANDLE,
                    if seed % 5 == 0 { 0 } else { r.next_u32() },
                );
                // The mutex: owner thread id (+4), lock count (+8), semaphore (+0xc).
                let m = st + STREAM_FILE_MUTEX;
                put_words(
                    c,
                    m + 4,
                    &[r.next_u32() % 4, r.next_u32() % 3, r.next_u32() % 16],
                );
                c.g[A0][0] = sx(st);
                c.g[A1][0] = sx(n);
            },
        );
    }

    /// WorldLightMan_LightInstanceCB: the world, its game module and light
    /// manager; an instance whose lighting node is cached at the head of
    /// its hash chain, with no model and the node's own position (so the
    /// real GetInstanceLighting takes its short path whether or not the
    /// force bit (flags bit 12) is set). Flags are random otherwise.
    #[test]
    fn world_light_man_light_instance_cb_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{put_vec, GAME_GP};
        const WORLD: u32 = 0x00a0_0000;
        const GAME_MODULE: u32 = 0x00a0_1000;
        const LIGHT_MAN: u32 = 0x00a1_0000;
        const NODE: u32 = 0x00a2_0000;
        const FRAME_STATE: u32 = 0x00a3_0000;
        const INST: u32 = 0x00a4_0000;
        crate::difftest::check(
            ee::s09::f_002f5148__generated,
            world_light_man_light_instance_cb,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f51_4800, 0);
                c.pc = 0x002f_5148;
                c.bus.write32(GAME_GP - 0x848, WORLD);
                c.bus.write32(WORLD + 0x84, GAME_MODULE);
                c.bus.write32(GAME_MODULE + 0x90, LIGHT_MAN);
                c.bus.write32(GAME_GP + 0x2a74, FRAME_STATE);
                c.bus.write32(FRAME_STATE + 0x18, 100 + seed % 50);
                let key = r.next_u32();
                let slot = (key >> 8) & 0x1f;
                c.bus.write32(LIGHT_MAN + 0x3210 + 4 * slot, NODE);
                c.bus.write32(NODE, key);
                let pos = [r.f32(500.0), r.f32(500.0), r.f32(500.0), r.f32(1.0)];
                put_vec(c, NODE + 0xb0, pos);
                let flags = r.next_u32();
                let flags = if seed % 2 == 0 {
                    flags | 0x1000
                } else {
                    flags & !0x1000
                };
                c.bus.write32(INST + 0x8, flags);
                c.bus.write32(INST + 0xc, 0);
                put_vec(c, INST + 0x40, pos);
                c.bus.write32(INST + 0x78, key);
                c.g[A0][0] = INST as u64;
            },
        );
    }

    /// PATH_TrackSetVolume: random volume and scale bytes (both signs),
    /// the track playing (status byte negative) on half the seeds; the
    /// real PATH_PlatSetVolume takes its early out (no audio streams:
    /// the audio system's +0x6280 is 0). LO/HI compared too.
    #[test]
    fn path_track_set_volume_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{DATA, GAME_GP};
        use path_track::*;
        const TRACK: u32 = DATA + 0x4_0000;
        const PLAT0: u32 = DATA + 0x4_1000;
        const AUDIO: u32 = DATA + 0x5_0000;
        crate::difftest::check_lohi(
            ee::s14::f_003d62c8__generated,
            path_track_set_volume,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003d_62c8, 0);
                c.pc = 0x003d_62c8;
                let status = r.next_u32() as u8;
                let status = if seed % 2 == 0 {
                    status | 0x80
                } else {
                    status & 0x7f
                };
                c.bus.write8(TRACK + STATUS, status);
                c.bus.write8(TRACK + VOLUME, r.next_u32() as u8);
                c.bus.write8(TRACK + SCALE, r.next_u32() as u8);
                let plat = r.next_u32() % 8;
                c.bus.write32(TRACK + PLAT, plat);
                c.bus.write32(0x004d_3e98 + 4 * plat, PLAT0 + 0x100 * plat);
                c.bus.write32(GAME_GP + 0x410, AUDIO);
                c.bus.write32(AUDIO + 0x6280, 0);
                c.g[A0][0] = TRACK as u64;
                c.g[A1][0] = if seed % 5 == 0 {
                    r.next_u32() as u64
                } else {
                    (r.next_u32() % 101) as u64
                };
            },
        );
    }

    // (fakes imported above)

    /// StreamFile_ClassifyChunk: entry counts -1..5, chunk ids that match
    /// the first, a later or no entry (masks of all, some or no bits).
    #[test]
    fn stream_file_classify_chunk_matches() {
        const SELF: u32 = DATA + 0x1000;
        const KEY: u32 = DATA + 0x1100;
        const ENTRIES: u32 = DATA + 0x1200;
        check(
            ee::s14::f_003dfe88__generated,
            stream_file_classify_chunk,
            300,
            &|c, seed| {
                let mut r = Rng(0x003d_fe88 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003d_fe88;
                let n = (seed % 7) as i32 - 1;
                c.bus.write32(SELF + SFC_COUNT, n as u32);
                c.bus.write32(SELF + SFC_ENTRIES, ENTRIES);
                let key = r.next_u32();
                c.bus.write32(KEY, key);
                let hit = if n > 0 && seed % 3 != 0 {
                    (r.next_u32() % n as u32) as i32
                } else {
                    -1
                };
                for i in 0..6u32 {
                    let e = ENTRIES + SFC_ENTRY_SIZE * i;
                    let mask = [u32::MAX, 0xffff_0000, 0xff, 0][(r.next_u32() % 4) as usize];
                    let value = if i as i32 == hit {
                        key & mask
                    } else {
                        (key & mask) ^ (1 + r.next_u32() % 0xff)
                    };
                    c.bus.write32(e, mask);
                    c.bus.write32(e + SFC_ENTRY_VALUE, value);
                    c.bus.write32(e + SFC_ENTRY_RESULT, r.next_u32());
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(KEY);
            },
        );
    }

    /// EA_ShellSortPairs: 0..60 pairs (gaps 1, 4, 13, 40) with keys from a
    /// small range (ties) or the full range (negatives).
    #[test]
    fn ea_shell_sort_pairs_matches() {
        const GAPS: [u32; 14] = [
            1,
            4,
            13,
            40,
            121,
            364,
            1093,
            3280,
            9841,
            29524,
            88573,
            265720,
            797161,
            0x7fff_ffff,
        ];
        check(
            ee::s14::f_003e6328__generated,
            ea_shell_sort_pairs,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e63_28, 0);
                c.pc = 0x003e_6328;
                put_words(c, OC264_SHELL_GAPS, &GAPS);
                let n = seed % 61;
                let keys = DATA + 0x1000;
                let vals = DATA + 0x2000;
                c.g[A0][0] = sx(n);
                c.g[A1][0] = sx(keys);
                c.g[A2][0] = sx(vals);
                for i in 0..n {
                    let k = if seed % 2 == 0 {
                        r.next_u32() % 8
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(keys + 4 * i, k);
                    c.bus.write32(vals + 4 * i, r.next_u32());
                }
            },
        );
    }

    // ----------------------------------------- CruiseApproachJump fixture

    /// The test's ComputerActorRider, its rider and pad, the record of the
    /// fake callees' arguments and the fakes' answers.
    pub(crate) const CAJ_SELF: u32 = crate::difftest::DATA;

    pub(crate) const CAJ_RIDER_AT: u32 = crate::difftest::DATA + 0x2000;

    pub(crate) const CAJ_PAD: u32 = crate::difftest::DATA + 0x3000;

    pub(crate) const CAJ_REC: u32 = crate::difftest::DATA + 0x4000;

    pub(crate) const CAJ_CTRL: u32 = crate::difftest::DATA + 0x5000;

    /// Log a fake call's argument registers, `ra`, `sp` and cycles at
    /// `CAJ_REC + slot * 0x80` (the RAM comparison checks them).
    pub(crate) fn caj_record(c: &mut Cpu, slot: u32) {
        let at = CAJ_REC + slot * 0x80;
        for (i, r) in [
            A0,
            A1,
            A2,
            A3,
            T0,
            T1,
            T2,
            RA,
            ssx3_ee_rt::cpu::reg::SP,
            S0,
            S1,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus.write64(at + 8 * i as u32, c.g[r][0]);
        }
        c.bus.write64(at + 0x60, c.cycles);
        c.bus.write32(at + 0x68, c.f[12]);
    }

    /// `FindJumpMarker`: found = ctrl+0; writes below (ctrl+4) and four
    /// marker words through a2, a3, t0, t1, t2.
    pub(crate) fn caj_fake_find_jump_marker(c: &mut Cpu) {
        caj_record(c, 0);
        let found = c.bus.read32(CAJ_CTRL);
        let below = c.bus.read32(CAJ_CTRL + 4);
        let a2 = c.g[A2][0] as u32;
        c.bus.write32(a2, below);
        for (i, r) in [A3, T0, T1, T2].into_iter().enumerate() {
            let p = c.g[r][0] as u32;
            c.bus.write32(p, 0x100 + i as u32 * 3 + found);
        }
        c.g[V0][0] = found as u64;
        c.cycles += 11;
        abi::ret(c);
    }

    /// `CruiseFollowPath`: only logged.
    pub(crate) fn caj_fake_cruise_follow_path(c: &mut Cpu) {
        caj_record(c, 1);
        c.cycles += 7;
        abi::ret(c);
    }

    /// `GetSteerTarget(&v, self)`: writes a vector from ctrl+0x10.
    pub(crate) fn caj_fake_get_steer_target(c: &mut Cpu) {
        caj_record(c, 2);
        let v = c.g[A0][0] as u32;
        for i in 0..4 {
            let w = c.bus.read32(CAJ_CTRL + 0x10 + 4 * i);
            c.bus.write32(v + 4 * i, w);
        }
        c.cycles += 5;
        abi::ret(c);
    }

    /// `ComputeSteer`: returns the steer at ctrl+8 in f0.
    pub(crate) fn caj_fake_compute_steer(c: &mut Cpu) {
        caj_record(c, 3);
        c.f[0] = c.bus.read32(CAJ_CTRL + 8);
        c.cycles += 9;
        abi::ret(c);
    }

    /// Installs the fakes in the callees' override slots; restores them on
    /// drop.
    pub(crate) struct CajSlots([fn(&mut Cpu); 4]);

    impl CajSlots {
        fn install() -> CajSlots {
            unsafe {
                let saved = CajSlots([
                    ssx3_ee::s00::OVERRIDE_0010B980,
                    ssx3_ee::s00::OVERRIDE_00100680,
                    ssx3_ee::s00::OVERRIDE_00100610,
                    ssx3_ee::s00::OVERRIDE_00100348,
                ]);
                ssx3_ee::s00::OVERRIDE_0010B980 = caj_fake_find_jump_marker;
                ssx3_ee::s00::OVERRIDE_00100680 = caj_fake_cruise_follow_path;
                ssx3_ee::s00::OVERRIDE_00100610 = caj_fake_get_steer_target;
                ssx3_ee::s00::OVERRIDE_00100348 = caj_fake_compute_steer;
                saved
            }
        }
    }

    impl Drop for CajSlots {
        fn drop(&mut self) {
            unsafe {
                ssx3_ee::s00::OVERRIDE_0010B980 = self.0[0];
                ssx3_ee::s00::OVERRIDE_00100680 = self.0[1];
                ssx3_ee::s00::OVERRIDE_00100610 = self.0[2];
                ssx3_ee::s00::OVERRIDE_00100348 = self.0[3];
            }
        }
    }

    /// ComputerActorRider_CruiseApproachJump: no marker (back to cruising),
    /// or a marker with the rider below/above it, high or low, a small or
    /// large steer (the real PlanAirTricks), and slower (the real
    /// CrouchAndBoost), within or faster than the speed band. The four
    /// callees with override slots are fakes.
    #[test]
    fn computer_actor_rider_cruise_approach_jump_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, elf_image};
        let _slots = CajSlots::install();
        check(
            ee::s00::f_001009e0__generated,
            computer_actor_rider_cruise_approach_jump,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_09e0, 0);
                elf_image(c);
                c.pc = 0x0010_09e0;
                c.g[A0][0] = sx(CAJ_SELF);
                c.g[A1][0] = sx(CAJ_PAD);
                c.bus.write32(CAJ_SELF + CAJ_RIDER, CAJ_RIDER_AT);
                let speed = 50.0 + (r.next_u32() % 250) as f32;
                c.bus.write32(CAJ_SELF + CAJ_SPEED, speed.to_bits());
                c.bus.write32(CAJ_SELF + 0xdfc, r.f32(1.0));
                c.bus.write32(CAJ_PAD, r.next_u32());
                c.bus.write32(CAJ_PAD + 4, r.next_u32());
                // Rider velocity: |v| around the cruise speed, +-300.
                let delta = f32::from_bits(r.f32(300.0));
                let target = (speed + delta).max(0.0);
                let (x, y) = (r.f32(1.0), r.f32(1.0));
                let (x, y) = (f32::from_bits(x), f32::from_bits(y));
                let n = (x * x + y * y + 1.0).sqrt();
                for (i, v) in [x / n, y / n, 1.0 / n, 0.0].into_iter().enumerate() {
                    c.bus.write32(
                        CAJ_RIDER_AT + CAJ_RIDER_VELOCITY + 4 * i as u32,
                        (v * target).to_bits(),
                    );
                }
                let height = if seed % 3 == 0 { 20000.0f32 } else { 5.0 };
                c.bus
                    .write32(CAJ_RIDER_AT + CAJ_RIDER_HEIGHT, height.to_bits());
                // The fakes' answers.
                c.bus.write32(CAJ_CTRL, (seed % 5 != 0) as u32);
                c.bus.write32(CAJ_CTRL + 4, (seed % 2) as u32);
                let steer = match seed % 7 {
                    0 => r.edge_f32(),
                    1 => 2.5f32.to_bits(),
                    _ => r.f32(1.0),
                };
                c.bus.write32(CAJ_CTRL + 8, steer);
                for i in 0..4 {
                    let w = r.f32(100.0);
                    c.bus.write32(CAJ_CTRL + 0x10 + 4 * i, w);
                }
            },
        );
    }

    // ------------------------------------------- ComputerActorRider_ThinkNaturalAir

    pub(crate) fn natural_air_f32s(c: &mut Cpu, a: u32, v: [f32; 4]) {
        put_words(c, a, &v.map(f32::to_bits));
    }

    /// The AI in the air with the predictor in each state and heights around
    /// the 0.5 / 1 / 3 thresholds (spin, flip, steer); the real
    /// `Rand_NextGame` and `GetSteerTarget` run, `ComputeSteer`'s two virtual
    /// calls are fakes (one may unwind).
    #[test]
    fn computer_actor_rider_think_natural_air_matches() {
        covered(
            ee::s00::f_0010b590__generated,
            computer_actor_rider_think_natural_air,
            240,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0010_b590, 2);
                c.pc = 0x0010_b590;
                let ai = DATA + 0x8_0000;
                let rider = DATA + 0x9_0000;
                let air = DATA + 0xa_0000;
                let vt = DATA + 0xb_0000;
                let axis = DATA + 0xc_0000;
                let motion = DATA + 0xc_1000;
                let pad = DATA + 0xd_0000;
                c.g[A0][0] = sx(ai);
                c.g[A1][0] = sx(pad);
                fill(c, &mut r, pad, 4);
                // The game's random generator state (both parities).
                fill(c, &mut r, 0x004f_f030, 6);
                fill(c, &mut r, ai + 0xe00, 0x10);
                c.bus.write32(ai + AI_RIDER, rider);
                c.bus.write32(rider + RIDER_AIR, air);
                c.bus
                    .write32(air + AIR_STATE, [1u32, 3, 0, 2][(seed % 4) as usize]);
                let heights = [-1.0f32, 0.0, 0.2, 0.5, 0.7, 1.0, 2.0, 3.0, 5.0];
                let h = heights[((seed / 4) as usize) % heights.len()];
                let ground = f32::from_bits(r.f32(10.0));
                c.bus.write32(air + AIR_GROUND, ground.to_bits());
                c.bus.write32(air + AIR_HEIGHT, (ground + h).to_bits());
                // GetSteerTarget's inputs.
                for off in [0x110u32, 0x490, 0x4b0] {
                    natural_air_f32s(
                        c,
                        rider + off,
                        [
                            f32::from_bits(r.f32(2.0)),
                            f32::from_bits(r.f32(2.0)),
                            0.0,
                            0.0,
                        ],
                    );
                }
                // ComputeSteer: model axis, two virtual calls (fakes).
                c.bus.write32(rider + 0x6c0, vt);
                c.bus
                    .write16(vt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x14, FAKE);
                c.bus
                    .write16(vt + 0x28, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x2c, FAKE + 0x10);
                c.bus.write32(fakes::LOG_RESULTS, axis);
                c.bus.write32(fakes::LOG_RESULTS + 4, motion);
                natural_air_f32s(c, motion, [0.0; 4]);
                natural_air_f32s(
                    c,
                    rider + 0x1c0,
                    [
                        f32::from_bits(r.f32(2.0)),
                        f32::from_bits(r.f32(2.0)),
                        if seed % 3 == 0 {
                            1.0
                        } else {
                            -f32::from_bits(r.f32(1.0))
                        },
                        0.0,
                    ],
                );
                natural_air_f32s(
                    c,
                    axis,
                    [
                        f32::from_bits(r.f32(2.0)),
                        f32::from_bits(r.f32(2.0)),
                        f32::from_bits(r.f32(2.0)),
                        0.0,
                    ],
                );
            },
        );
    }

    /// WScriptFn_Crowd2dControl: the static defaults initialised or not;
    /// 0..4 extra arguments over the handle and mode arguments (or none at
    /// all), int arguments for float slots converted; the handle -1 (the
    /// world's default object, present or not), an empty handle set, an
    /// empty entry, or a found object; the modes bind (the real bind finds
    /// no instance), none, add and remove (the real crowd manager has no
    /// free or matching slot) and an unknown one.
    #[test]
    fn wscript_fn_crowd2d_control_matches() {
        use crate::difftest::GAME_GP;
        const DEFAULTS: u32 = 0x004f_bc80;
        const ARG_TYPES: u32 = 0x0044_65d8;
        const RESULT_NONE: u32 = 0x004c_9098;
        check(
            ee::s10::f_00303ba0__generated,
            wscript_fn_crowd2d_control,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x30_3ba0, 0);
                let gp = GAME_GP;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // Static defaults: fresh, or already set up (handle -1).
                let inited = seed % 2;
                w(c, gp + 0x26f0, inited);
                fill(c, &mut r, DEFAULTS, 5);
                w(c, DEFAULTS, u32::MAX);
                w(c, DEFAULTS + 4, r.next_u32() % 5);
                w(c, ARG_TYPES, 0);
                w(c, ARG_TYPES + 4, 0);
                for i in 2..5 {
                    w(c, ARG_TYPES + 4 * i, r.next_u32() % 3);
                }
                fill(c, &mut r, RESULT_NONE, 3);
                // Handle tables: gp+0x16c8 -> P -> Q; Q+8 -> sets.
                let (p, q, sets) = (DATA + 0x2000, DATA + 0x2100, DATA + 0x2200);
                let (set, entries, obj) = (DATA + 0x2800, DATA + 0x2900, DATA + 0x3000);
                let world = DATA + 0x3800;
                let crowd = DATA + 0x4000;
                w(c, gp + 0x16c8, p);
                w(c, p, q);
                w(c, q + 8, sets);
                w(c, sets + 4 * 3, set);
                w(c, sets + 4 * 7, 0);
                w(c, set + 0x1c, entries);
                let found = (obj >> 2) << 8 | (r.next_u32() & 0xff);
                w(
                    c,
                    entries + 4 * 5,
                    if seed / 2 % 4 == 2 {
                        r.next_u32() & 0xff
                    } else {
                        found
                    },
                );
                w(c, gp + 0xce8, world);
                w(c, world + 0x290, if seed % 9 == 4 { 0 } else { obj });
                // The instance handle: its set is empty (bind finds nothing).
                w(c, obj + 0x78, r.next_u32() & 0xffff_ff00 | 7);
                // Crowd manager: 64 slots, none free (-1) and none matching.
                w(c, gp.wrapping_sub(0x6f0), crowd);
                for i in 0..64 {
                    w(c, crowd + 8 + 4 * i, r.next_u32() & 0x7fff_fff0 | 0x10);
                }
                let id = match seed / 2 % 4 {
                    0 => u32::MAX,
                    1 => 5 << 8 | 7,
                    _ => 5 << 8 | 3,
                };
                let mode = match seed / 8 % 5 {
                    4 => r.next_u32() % 16,
                    m => m,
                };
                // Arguments (16 bytes: slot, value, -, type).
                let args = DATA + 0x100;
                let extra = seed / 3 % 5;
                let mut n = 0;
                let mut put = |c: &mut Cpu, slot: u32, val: u32, ty: u32| {
                    let a = args + 0x10 * n;
                    c.bus.write32(a, slot);
                    c.bus.write32(a + 4, val);
                    c.bus.write32(a + 8, 0x5555);
                    c.bus.write32(a + 0xc, ty);
                    n += 1;
                };
                put(c, 0, id, 0);
                put(c, 1, mode, 0);
                for _ in 0..extra {
                    let slot = 2 + r.next_u32() % 3;
                    let val = r.next_u32();
                    let ty = r.next_u32() % 3;
                    put(c, slot, val, ty);
                }
                let argc = match seed % 11 {
                    0 => 0,
                    5 => sx(0xffff_fffe),
                    _ => n as u64,
                };
                c.g[A0][0] = sx(DATA + 0x10);
                c.g[A1][0] = argc;
                c.g[A2][0] = sx(args);
                c.pc = 0x0030_3ba0;
            },
        );
    }

    use crate::difftest::RA as RET_ADDR;

    /// A random RefPack stream: header (3- or 4-byte sizes, with or without
    /// the packed-size field), then every command form with random
    /// literal counts, lengths and offsets, and an end code with 0-3
    /// trailing literals.
    pub(crate) fn refpack_stream(r: &mut Rng, seed: u32) -> Vec<u8> {
        let mut s = Vec::new();
        let four = seed % 2 == 1;
        let skip = seed % 4 >= 2;
        let flags: u16 = 0x10fb | (if four { 0x8000 } else { 0 }) | (if skip { 0x100 } else { 0 });
        s.extend_from_slice(&flags.to_be_bytes());
        let width = if four { 4 } else { 3 };
        for _ in 0..if skip { 2 } else { 1 } {
            let v = r.next_u32();
            s.extend_from_slice(&v.to_be_bytes()[4 - width..]);
        }
        let lit = |r: &mut Rng, s: &mut Vec<u8>, n: u32| {
            for _ in 0..n {
                s.push(r.next_u32() as u8);
            }
        };
        let n = r.next_u32() % 9;
        for _ in 0..n {
            let rl = r.next_u32() % 4;
            match r.next_u32() % 4 {
                0 => {
                    let len = 3 + r.next_u32() % 8;
                    let off = r.next_u32() % 1024;
                    s.push((((off >> 8) << 5) | ((len - 3) << 2) | rl) as u8);
                    s.push(off as u8);
                }
                1 => {
                    let len = 4 + r.next_u32() % 64;
                    let off = r.next_u32() % 0x4000;
                    s.push((0x80 | (len - 4)) as u8);
                    s.push(((rl << 6) | (off >> 8)) as u8);
                    s.push(off as u8);
                }
                2 => {
                    let len = 5 + r.next_u32() % 300;
                    let off = r.next_u32() % 0x2_0000;
                    s.push((0xc0 | ((off >> 16) << 4) | (((len - 5) >> 8) << 2) | rl) as u8);
                    s.push((off >> 8) as u8);
                    s.push(off as u8);
                    s.push((len - 5) as u8);
                }
                _ => {
                    let k = r.next_u32() % 28;
                    s.push((0xe0 | k) as u8);
                    lit(r, &mut s, (k + 1) * 4);
                    continue;
                }
            }
            lit(r, &mut s, rl);
        }
        let rl = r.next_u32() % 4;
        s.push((0xfc | rl) as u8);
        lit(r, &mut s, rl);
        s
    }

    /// RefPack_Decode: random streams through every command form, the
    /// header variants, a null source, and with or without `consumed`.
    #[test]
    fn refpack_decode_matches() {
        check(
            ee::s13::f_003b49d0__generated,
            refpack_decode,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b49d0, 0);
                let src = DATA + 0x1000;
                for (i, b) in refpack_stream(&mut r, seed).into_iter().enumerate() {
                    c.bus.write8(src + i as u32, b);
                }
                c.g[A0][0] = sx(DATA + 0x4_0000);
                c.g[A1][0] = if seed % 13 == 0 { 0 } else { sx(src) };
                c.g[A2][0] = if seed % 5 == 0 { 0 } else { sx(DATA + 0x800) };
                c.pc = 0x003b_49d0;
            },
        );
    }

    /// `WorldObjFactory_Object`: the static defaults initialised or not;
    /// -1..4 parameters with random slots/kinds (ints into float slots get
    /// converted; slot 0 is the object id); the id is -1 (the world's
    /// default object, sometimes none) or a section/object pair whose
    /// section or entry may be empty. Found objects have no node (an
    /// `Object` is allocated from the pool allocator and constructed for
    /// real: process list, counters, flags) or a node whose modifier check
    /// is a fake returning non-zero (nothing created). Node flag 0x100 is
    /// always set so WorldObj_ctor does not reach World_AddFocusObject.
    /// Not reached: a node whose modifier check fails (needs a zero result
    /// through the real AttachModifier chain).
    #[test]
    fn world_obj_factory_object_matches() {
        use crate::difftest::{check, GAME_GP};
        use crate::game::fe::feasyncfile::tests::alloc_base_3;
        const RESULT_SLOT: u32 = DATA + 0x3000;
        const PARAMS: u32 = DATA + 0x1000;
        const DIR: u32 = DATA + 0x20_0000;
        const SECTIONS: u32 = DIR + 0x100;
        const TABLE: u32 = DIR + 0x200;
        const SECTION: u32 = DIR + 0x300;
        const ENTRIES: u32 = DIR + 0x400;
        const WORLD: u32 = DIR + 0xa00;
        const NODE: u32 = DIR + 0xc00;
        const VT: u32 = DIR + 0xd00;
        const PROCS: u32 = DIR + 0x1000;
        const TAILS: u32 = DIR + 0x1800;
        const OBJS: u32 = DIR + 0x2000;
        check(
            ee::s10::f_002fc0d0__generated,
            world_obj_factory_object,
            300,
            &|c, seed| {
                let mut r = alloc_base_3(c, seed, 0x2f_c0d0, 1);
                c.pc = 0x002f_c0d0;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // Static defaults.
                let ready = seed % 4 != 0;
                w(c, GAME_GP + WOF_DEFAULTS_READY_GP, ready as u32);
                fill(c, &mut r, WOF_DEFAULTS, 4);
                if ready && seed % 3 == 0 {
                    w(c, WOF_DEFAULTS, u32::MAX);
                }
                for i in 0..4 {
                    w(c, WOF_PARAM_KINDS + 4 * i, r.next_u32() % 4);
                }
                // Directory: sections 0..2 present, 3 empty; entries 0..2 objects, 3 empty.
                w(c, GAME_GP + WOF_DIRECTORY_GP, DIR);
                w(c, DIR, SECTIONS);
                w(c, SECTIONS + 8, TABLE);
                for i in 0..4 {
                    w(c, TABLE + 4 * i, if i == 3 { 0 } else { SECTION });
                    let obj = OBJS + 0x100 * i;
                    w(
                        c,
                        ENTRIES + 4 * i,
                        if i == 3 {
                            r.next_u32() & 0xff
                        } else {
                            (obj >> 2) << 8 | (r.next_u32() & 0xff)
                        },
                    );
                    fill(c, &mut r, obj, 0x10);
                    w(c, obj + 8, r.next_u32() | 0x100);
                    w(
                        c,
                        obj + WOF_OBJECT_NODE,
                        if (seed / 4) % 3 == 0 { NODE } else { 0 },
                    );
                }
                w(c, SECTION + 0x1c, ENTRIES);
                w(c, GAME_GP + WOF_WORLD_GP, WORLD);
                w(
                    c,
                    WORLD + WOF_WORLD_DEFAULT_OBJECT,
                    if seed % 7 == 0 { 0 } else { OBJS + 0x100 },
                );
                // The node's modifier check: a fake (non-zero result).
                w(c, NODE + 0xc, VT);
                c.bus.write16(VT + 0x50, 0);
                w(c, VT + 0x54, FAKE);
                // Process list for the new object (type 1).
                w(c, GAME_GP + 0x2898 + 4, PROCS);
                w(c, PROCS + 0x44, r.next_u32());
                w(c, PROCS + 0x44 + 8, TAILS);
                w(c, PROCS + 0x44 + 0x28, TAILS + 0x40);
                // The result value.
                fill(c, &mut r, WOF_RESULT, 3);
                // Parameters.
                let count: u32 = if seed % 11 == 0 { u32::MAX } else { seed % 5 };
                for i in 0..4 {
                    let p = PARAMS + 0x10 * i;
                    let slot = r.next_u32() % 4;
                    let value = if slot == 0 {
                        match r.next_u32() % 4 {
                            0 => u32::MAX,
                            _ => (r.next_u32() % 4) | (r.next_u32() % 4) << 8,
                        }
                    } else {
                        r.next_u32()
                    };
                    w(c, p, slot);
                    w(c, p + 4, value);
                    w(c, p + 8, r.next_u32());
                    w(c, p + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(RESULT_SLOT + 4 * (seed % 2));
                c.g[A1][0] = sx(count);
                c.g[A2][0] = sx(PARAMS);
            },
        );
    }

    /// StreamFile_ParseChunks: a stream at DATA (header DATA+0x1000, three
    /// classifier entries at DATA+0x2000, class tallies at DATA+0x3000) with
    /// 0-4 chunks of known/unknown classes in a buffer at an unaligned
    /// DATA+0x4000+k, optionally ending in an end-of-stream chunk or a chunk
    /// whose length already carries a class tag, and a fill cursor that cuts
    /// the last chunk short or not. The header's state is sometimes 4
    /// (shutting down). The mutex syscalls go to the fakes (some unwind); one
    /// seed in seven re-enters at the loop head with the frame already open.
    #[test]
    fn stream_file_parse_chunks_matches() {
        use crate::difftest::fakes::{covered, machine};
        covered(
            ee::s14::f_003dfed0__generated,
            stream_file_parse_chunks,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3dfed0, 12);
                let this = DATA;
                let header = DATA + 0x1000;
                let classes = DATA + 0x2000;
                let tallies = DATA + 0x3000;
                let end_id = 0x454e_4421u32;
                let ids = [0x4141_0001u32, 0x4242_0002, end_id];
                c.g[A0][0] = this as u64;
                c.bus.write32(this + SFPC_HEADER, header);
                c.bus.write32(
                    header + SFPC_HDR_STATE,
                    if seed % 5 == 0 { 4 } else { r.next_u32() % 4 },
                );
                c.bus.write32(header + SFPC_HDR_END_ID, end_id);
                // Classifier: {mask, value, class}; the end id is class 3.
                c.bus.write32(this + 0x1c, classes);
                c.bus.write32(this + 0x20, 3);
                for (i, id) in ids.iter().enumerate() {
                    let e = classes + 12 * i as u32;
                    c.bus.write32(e, 0xffff_ffff);
                    c.bus.write32(e + 4, *id);
                    c.bus.write32(e + 8, i as u32 + 1);
                }
                c.bus.write32(this + SFPC_TALLIES, tallies);
                for k in 0..3 {
                    let n = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32() & 0xfff
                    };
                    c.bus.write32(tallies + 16 * k + SFPC_TALLY_BYTES, n);
                }
                c.bus.write32(this + SFPC_PARSED, r.next_u32() & 0xfff);
                c.bus.write32(this + SFPC_LOW_WATER, r.next_u32() & 0xfff);
                c.bus.write32(this + SFPC_NEEDS_REFILL, 1);
                // Mutex: owner, depth, semaphore (owner never the fake thread id).
                c.bus
                    .write32(this + 0x8, 0x7700_0000 | r.next_u32() & 0xffff);
                c.bus.write32(this + 0xc, r.next_u32() & 3);
                c.bus.write32(this + 0x10, r.next_u32() & 0xff);
                // Chunks.
                let start = DATA + 0x4000 + 4 * (r.next_u32() % 16);
                let mut p = start;
                for _ in 0..r.next_u32() % 5 {
                    let id = match r.next_u32() % 3 {
                        0 => ids[0],
                        1 => ids[1],
                        _ => r.next_u32(),
                    };
                    let len = 8 + 4 * (r.next_u32() % 16);
                    c.bus.write32(p, id);
                    c.bus.write32(p + 4, len);
                    p += len;
                }
                match r.next_u32() % 3 {
                    0 => {
                        c.bus.write32(p, end_id);
                        c.bus.write32(p + 4, 8 + 4 * (r.next_u32() % 8));
                        p += 0x80;
                    }
                    1 => {
                        c.bus.write32(p, r.next_u32());
                        c.bus.write32(p + 4, (1 + r.next_u32() % 3) << 24 | 0x10);
                        p += 0x80;
                    }
                    _ => {}
                }
                c.bus.write32(this + SFPC_CURSOR, start);
                let fill = match r.next_u32() % 3 {
                    0 => p.wrapping_sub(r.next_u32() % 0x40),
                    _ => p,
                };
                c.bus.write32(this + SFPC_FILL, fill.max(start));
                c.pc = SFPC_ENTRY;
                if seed % 7 == 3 && fill >= start + 8 {
                    // Re-entry at the loop head: frame open, first chunk loaded.
                    let frame = STACK_TOP - 0x90;
                    c.g[ssx3_ee_rt::cpu::reg::SP][0] = frame as u64;
                    c.bus.write64(frame, RET_ADDR as u64);
                    c.g[S0][0] = this as u64;
                    c.g[S6][0] = header as u64;
                    c.g[S7][0] = sx(0xffff_ffc0);
                    c.g[S2][0] = start as u64;
                    c.g[A0][0] = start as u64;
                    c.g[S1][0] = c.bus.read32(start + 4) as u64 & 0xff_ffff;
                    c.g[V1][0] = sx(c.bus.read32(start));
                    c.pc = SFPC_HEAD;
                }
            },
        );
    }

    /// WScriptFn_AddUVScrollModifier: defaults built or not (guard), 0-4
    /// script arguments (raw or converted to float; parameter types at
    /// 0x446238), object id -1 (the world), a null table slot, a zero handle
    /// or a real object; no model instance, or one whose virtual check is a
    /// fake (non-zero, some unwind) or a real leaf returning 0. The passing
    /// check runs the real `ModObj_AddUVScrollModifier` on the heap arena.
    #[test]
    fn wscript_fn_add_uv_scroll_modifier_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::{fill, GAME_GP};
        covered(
            ee::s10::f_002fe2c0__generated,
            wscript_fn_add_uv_scroll_modifier,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2fe2c0, 1);
                crate::game::be::bedbreward::tests::alloc_ready(c);
                c.pc = 0x002f_e2c0;
                let result = DATA + 0x10;
                let args = DATA + 0x100;
                let world = DATA + 0x1000;
                let obj = DATA + 0x2000;
                let table = DATA + 0x3000;
                let inst = DATA + 0x4000;
                let vt = DATA + 0x5000;
                let block = DATA + 0x6000;
                c.bus.write32(GAME_GP + WSUV_GUARD, (seed % 3 == 0) as u32);
                fill(c, &mut r, WSUV_DEFAULTS, 13);
                // (A built block's object id is always -1.)
                c.bus.write32(WSUV_DEFAULTS, 0xffff_ffff);
                fill(c, &mut r, 0x004c_9098, 3);
                for i in 0..13u32 {
                    let t = if i == 0 { 1 } else { r.next_u32() % 4 };
                    c.bus.write32(WSUV_PARAM_TYPES as u32 + 4 * i, t);
                }
                // Arguments.
                let n = match seed % 6 {
                    0 => 0,
                    1 => -(r.next_u32() as i32 % 3) - 1,
                    _ => (1 + r.next_u32() % 4) as i32,
                };
                let id = match (seed / 2) % 5 {
                    0 => None,
                    1 => Some(r.next_u32() % 4 << 8),
                    2 => Some((r.next_u32() % 4) << 8 | 1),
                    _ => Some((1 + r.next_u32() % 3) << 8 | (1 + r.next_u32() % 3)),
                };
                for k in 0..4u32 {
                    let a = args + 16 * k;
                    let idx = if k == 0 && id.is_some() {
                        0
                    } else {
                        1 + r.next_u32() % 12
                    };
                    c.bus.write32(a, idx);
                    c.bus
                        .write32(a + 4, if idx == 0 { id.unwrap() } else { r.next_u32() });
                    c.bus.write32(a + 8, r.next_u32());
                    c.bus.write32(a + 12, r.next_u32() % 4);
                }
                c.g[A0][0] = result as u64;
                c.g[A1][0] = sx(n as u32);
                c.g[A2][0] = args as u64;
                // Object resolution.
                c.bus.write32(GAME_GP + WSUV_WORLD, world);
                c.bus.write32(world + WSUV_WORLD_SELF, obj);
                c.bus.write32(GAME_GP + WSUV_OBJ_TABLE, table);
                c.bus.write32(table, table + 0x100);
                c.bus.write32(table + 0x108, table + 0x200);
                for j in 0..4u32 {
                    let slot = if j == 0 { 0 } else { table + 0x300 + 0x10 * j };
                    c.bus.write32(table + 0x200 + 4 * j, slot);
                    if slot != 0 {
                        c.bus.write32(slot + 0x1c, table + 0x400);
                    }
                }
                for k in 0..4u32 {
                    let handle = if k == 0 {
                        0x12
                    } else {
                        (obj >> 2) << 8 | (r.next_u32() & 0xff)
                    };
                    c.bus.write32(table + 0x400 + 4 * k, handle);
                }
                c.bus
                    .write32(obj + 0xc, if seed % 7 == 0 { 0 } else { inst });
                c.bus.write32(inst + 0xc, vt);
                c.bus.write32(inst + 0x1c, block);
                c.bus.write32(block + 4, 0);
                c.bus.write16(vt + 0x80, (r.next_u32() % 4 * 4) as u16);
                c.bus
                    .write32(vt + 0x84, if seed % 4 == 1 { 0x0014_06c0 } else { FAKE });
            },
        );
    }

    /// StreamFile_StartNextRequest: no current request, a ready one, a
    /// queued next one or none; in-memory or file requests whose file name
    /// matches the open file. The mutex syscalls go to the fakes (some
    /// unwind); the real `StreamFile_IssueRead` stalls at once (the ring
    /// cursors all at the buffer start). The name-change branch (strcpy and
    /// the file queue's open/close) is not reached: `EAFile_QueueOpen` /
    /// `QueueClose` need the whole file-request pool.
    #[test]
    fn stream_file_start_next_request_matches() {
        use crate::difftest::fakes::{covered, machine};
        covered(
            ee::s14::f_003e0270__generated,
            stream_file_start_next_request,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e0270, 6);
                c.pc = 0x003e_0270;
                let this = DATA;
                let req = DATA + 0x1000;
                let next = DATA + 0x1400;
                let ring = DATA + 0x2200;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (r.next_u32() % 8) as u64;
                // Mutex: owner, depth, semaphore (owner never the fake thread id).
                c.bus
                    .write32(this + 0x8, 0x7700_0000 | r.next_u32() & 0xffff);
                c.bus.write32(this + 0xc, r.next_u32() & 3);
                c.bus.write32(this + 0x10, r.next_u32() & 0xff);
                c.bus.write32(this + SNR_READ_STATE, 1 + r.next_u32() % 2);
                c.bus
                    .write32(this + SNR_REQUEST, if seed % 5 == 0 { 0 } else { req });
                c.bus.write32(
                    req + SNR_REQ_STATE,
                    if seed % 3 == 0 {
                        1
                    } else {
                        2 + r.next_u32() % 3
                    },
                );
                c.bus
                    .write32(req + SNR_REQ_NEXT, if seed % 4 == 1 { 0 } else { next });
                for q in [req, next] {
                    c.bus
                        .write32(q + SNR_REQ_KIND, if r.next_u32() % 2 == 0 { 1 } else { 2 });
                    c.bus.write32(q + SNR_REQ_SIZE, r.next_u32() & 0xffff);
                    c.bus.write32(q + SNR_REQ_START, r.next_u32());
                    for (i, b) in b"track01.big\0".iter().enumerate() {
                        c.bus.write8(q + SNR_REQ_NAME + i as u32, *b);
                    }
                }
                for (i, b) in b"track01.big\0".iter().enumerate() {
                    c.bus.write8(this + SNR_NAME + i as u32, *b);
                }
                c.bus.write32(this + SNR_LEFT, r.next_u32());
                c.bus.write32(this + SNR_FILE, 0);
                // StreamFile_IssueRead stall path.
                c.bus.write32(this + 0x30, ring);
                c.bus.write32(this + 0x34, DATA + 0x3000);
                c.bus.write32(this + 0x50, ring);
                c.bus.write32(this + SNR_CURSOR, ring);
                c.bus.write32(this + SNR_FILL, r.next_u32());
                c.bus.write32(this + 0x5c, DATA + 0x4000);
                c.bus.write32(DATA + 0x4000 + 0xc, 0);
                c.bus.write32(this + 0x17c, 0x1000);
            },
        );
    }

    /// Pad slot table at 0x526250 (port 0..1, slot 0..3) pointing at an
    /// actuator buffer in DATA. Odd seeds use the fake syscalls (DmaStat
    /// "busy", SetDma succeeds), even seeds the machine's SIF (DmaStat -1,
    /// the first SetDma returns id 0 = failure); the debug flag is set on
    /// some seeds so the busy message is printed.
    #[test]
    fn libpad_send_act_dma_matches() {
        use crate::difftest::{check_lohi, fakes, scramble, Rng, DATA};
        check_lohi(
            ee::s15::f_003ff390__generated,
            libpad_send_act_dma,
            240,
            &|c, seed| {
                let mut r = if seed % 2 == 1 {
                    fakes::machine(c, seed, 0x3ff390, 0)
                } else {
                    let mut r = Rng(0x3ff390 ^ (seed as u64) << 8);
                    scramble(c, &mut r);
                    r
                };
                c.pc = 0x003f_f390;
                let port = seed / 2 % 2;
                let slot = seed / 4 % 4;
                let entry = PAD_SLOTS + port * PAD_PORT_STRIDE + slot * PAD_SLOT_STRIDE;
                let buf = DATA + 0x400;
                c.bus.write32(entry + PAD_ACT_BUF, buf);
                c.bus
                    .write32(entry + PAD_IOP_DEST, 0x0001_0000 + (r.next_u32() & 0xff0));
                let id = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() & 0xff
                };
                c.bus.write32(entry + PAD_DMA_ID, id);
                c.bus.write32(buf, r.next_u32());
                for i in 1..8 {
                    c.bus.write32(buf + 4 * i, r.next_u32());
                }
                let debug = if seed % 5 == 0 { 1 } else { 0 };
                c.bus.write32(PAD_DEBUG_BASE + PAD_DEBUG_FLAG, debug);
                c.g[A0][0] = port as u64;
                c.g[A1][0] = slot as u64;
            },
        );
    }

    /// scePadSetActDirect: pad slot (port 0..1, slot 0..3) with a DMA
    /// buffer whose current half reports state 1 (stable) or not, and an
    /// actuator packet; the real libpad_GetDmaBuffer and libpad_SendActDma
    /// run, the SIF syscalls faked on odd seeds (send succeeds) and real on
    /// even ones (first SetDma returns id 0: failure).
    #[test]
    fn sce_pad_set_act_direct_matches() {
        use crate::difftest::{check_lohi, fakes, scramble, Rng, DATA};
        check_lohi(
            ee::s15::f_00400190__generated,
            sce_pad_set_act_direct,
            240,
            &|c, seed| {
                let mut r = if seed % 2 == 1 {
                    fakes::machine(c, seed, 0x400190, 0)
                } else {
                    let mut r = Rng(0x400190 ^ (seed as u64) << 8);
                    scramble(c, &mut r);
                    r
                };
                c.pc = 0x0040_0190;
                let port = seed / 2 % 2;
                let slot = seed / 4 % 4;
                let entry = PAD_SLOTS + port * PAD_PORT_STRIDE + slot * PAD_SLOT_STRIDE;
                let dma = DATA + 0x1000;
                let act = DATA + 0x400;
                let data = DATA + 0x300;
                c.bus.write32(entry, dma);
                c.bus.write32(entry + PAD_ACT_BUF, act);
                c.bus
                    .write32(entry + PAD_IOP_DEST, 0x0001_0000 + (r.next_u32() & 0xff0));
                c.bus.write32(
                    entry + PAD_DMA_ID,
                    if seed % 3 == 0 {
                        0
                    } else {
                        r.next_u32() & 0xff
                    },
                );
                // Which half is current: +0x58 < +0xd8 selects the second.
                c.bus.write32(dma + 0x58, r.next_u32() % 4);
                c.bus.write32(dma + 0xd8, r.next_u32() % 4);
                let state = |r: &mut Rng| if seed % 5 == 0 { r.next_u32() % 8 } else { 1 };
                let s0 = state(&mut r);
                let s1 = state(&mut r);
                c.bus.write8(dma + 0x72, s0 as u8);
                c.bus.write8(dma + 0xf2, s1 as u8);
                for i in 0..8 {
                    c.bus.write32(act + 4 * i, r.next_u32());
                    c.bus.write32(data + 4 * i, r.next_u32());
                }
                c.bus.write32(PAD_DEBUG_BASE + PAD_DEBUG_FLAG, 0);
                c.g[A0][0] = port as u64;
                c.g[A1][0] = slot as u64;
                c.g[A2][0] = sx(data);
            },
        );
    }

    /// Rider_ResetGrindOrientation: not grinding; main state 1 with each
    /// orientation (3, 4, other); main state 0 with each orientation (the
    /// quarter turns run the real RotateAroundAxis / RotateRoots / SinCos);
    /// another main state. cRiderAnimBase_play runs for real on an animation
    /// controller whose virtual call is a fake (as in
    /// `rider_update_timed_event_anim_matches`).
    #[test]
    fn rider_reset_grind_orientation_matches() {
        check(
            ee::s00::f_00115640__generated,
            rider_reset_grind_orientation,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x11_5640, 1);
                c.pc = 0x0011_5640;
                let rider = DATA;
                let ctrl = DATA + 0x4000;
                let state = DATA + 0x6000;
                let seqs = DATA + 0x7000;
                c.g[A0][0] = sx(rider);
                let vt = fakes::vtable(c, &mut r, &[0x20]);
                c.bus.write32(rider + RGO_ANIM, ctrl);
                c.bus.write32(ctrl + 0x58, vt);
                c.bus.write32(ctrl + 4, 0x1b6);
                c.bus.write32(ctrl + 8, 0x1b6);
                c.bus.write32(ctrl + 0x50, seqs);
                for i in 0..8 {
                    c.bus.write32(LOG_RESULTS + 4 * i, 0x207);
                }
                c.bus.write32(rider + 0x77c, state);
                let main_state = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 2 + r.next_u32() % 4,
                };
                c.bus.write32(state + 0xde0, main_state);
                let side = match (seed / 3) % 5 {
                    0 => 0,
                    1 | 2 => 3,
                    3 => 4,
                    _ => 1 + r.next_u32() % 8,
                };
                c.bus.write32(rider + RGO_GRIND_SIDE, side);
                for i in 0..4 {
                    c.bus.write32(rider + RGO_AXIS + 4 * i, r.f32(1.0));
                    c.bus.write32(rider + 0x120 + 4 * i, r.f32(1.0));
                }
            },
        );
    }

    /// StreamFile_OnRead: memory source (end reached or not) and EAFile
    /// source (the release finds an empty request slot); then a closing
    /// stream, a finished one, a parsed end chunk, or a further read. The
    /// real callees take short paths: ParseChunks sees no complete chunk
    /// (fill before the cursor) or one end-of-stream chunk (fixture as in
    /// `stream_file_parse_chunks_matches`); StartNextRequest finds no next
    /// header; IssueRead finds the ring idle and the buffer full (state 2).
    /// The mutex syscalls go to the fakes.
    #[test]
    fn stream_file_on_read_matches() {
        check(
            ee::s14::f_003e0170__generated,
            stream_file_on_read,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_0170, 0);
                c.pc = 0x003e_0170;
                let file = DATA;
                let header = DATA + 0x1000;
                let classes = DATA + 0x2000;
                let tallies = DATA + 0x3000;
                let buf = DATA + 0x4000;
                let issue = DATA + 0x5000;
                let end_id = 0x454e_4421u32;
                c.g[A2][0] = sx(file);
                c.bus.write32(file + SFR_HEADER, header);
                // 0: closing, 1: maybe finished, 2: end chunk, 3: further read.
                let case = seed % 4;
                let state = if case == 0 {
                    4
                } else {
                    [0u32, 2, 3][(r.next_u32() % 3) as usize]
                };
                c.bus.write32(header + SFR_HDR_STATE, state);
                c.bus.write32(header + 0xc, 0);
                c.bus.write32(header + 0x11c, end_id);
                let memory = seed % 3 == 0;
                c.bus.write32(
                    header + SFR_HDR_SOURCE,
                    if memory { 1 } else { r.next_u32() % 4 + 2 },
                );
                // Case 3 is never finished: a memory source short of its end,
                // an EAFile read of the full (zero) length.
                let len = if case == 3 && !memory {
                    0
                } else {
                    r.next_u32() & 0xfff
                };
                c.bus.write32(file + SFR_REQUEST_LEN, len);
                c.bus.write32(file + SFR_BYTES_READ, r.next_u32() & 0xffff);
                let size = if case == 3 {
                    0x10_0000
                } else {
                    r.next_u32() & 0x1_ffff
                };
                c.bus.write32(header + SFR_HDR_SIZE, size);
                // EAFile request slot table (zeroed entries: nothing to release).
                c.bus.write32(0x0051_9ad8, DATA + 0xf_0000);
                c.bus.write32(file + SFR_REQUEST, r.next_u32() & 0xff);
                // Classifier and tallies for the end chunk.
                c.bus.write32(file + 0x1c, classes);
                c.bus.write32(file + 0x20, 1);
                c.bus.write32(classes, 0xffff_ffff);
                c.bus.write32(classes + 4, end_id);
                c.bus.write32(classes + 8, 3);
                c.bus.write32(file + 0x24, tallies);
                c.bus.write32(file + 0x44, r.next_u32() & 0xfff);
                c.bus.write32(file + 0x48, 1);
                c.bus.write32(file + 0x4c, r.next_u32() & 0xfff);
                // Mutex: owner (never the fake thread id), depth, semaphore.
                c.bus
                    .write32(file + 0x8, 0x7700_0000 | r.next_u32() & 0xffff);
                c.bus.write32(file + 0xc, r.next_u32() & 3);
                c.bus.write32(file + 0x10, r.next_u32() & 0xff);
                // Cursor (+0x54) and fill (+0x58, OnRead adds the byte count):
                // the end chunk, or the fill short of the cursor.
                c.bus.write32(file + 0x54, buf);
                c.bus.write32(file + 0x50, buf);
                let n = if memory { len } else { 0 };
                let fill = if case == 2 {
                    c.bus.write32(buf, end_id);
                    c.bus.write32(buf + 4, 8);
                    buf + 8
                } else {
                    buf - 0x10
                };
                c.bus.write32(file + SFR_FILL, fill.wrapping_sub(n));
                // IssueRead: ring idle, buffer full.
                c.bus.write32(file + 0x5c, issue);
                c.bus.write32(issue + 0xc, 0);
                c.bus.write32(file + 0x17c, 0x1000);
                c.bus.write32(file + SFR_READ_AHEAD, r.next_u32() & 0xff);
            },
        );
    }

    use crate::difftest::fakes::{
        self as bow_fakes, covered as bow_covered, machine as bow_machine, FAKE as BOW_FAKE,
    };
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::{
        put_vec as bow_put_vec, put_words as bow_put_words, DATA as BOW_DATA, GAME_GP as BOW_GP,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Rider_HandleImpact fixture (all structures at DATA + 0x200000, away
    /// from the fakes' random vtable). Sub state 9 (frozen), 0xb (already
    /// wiping out: bump) or other; main state 1 (riding), 2 (in the air,
    /// handler air state 1 or not) or 3 (other); surface -1 or a non-fatal
    /// one; random board axes, normals (steep ones too), impact speeds
    /// around every limit, contact heights around 60, velocities around the
    /// fast speed, layer-2 sequences 0x12..0x15 or others, and the game's
    /// Rand state. The rider's methods (vibrationBump, getPosition,
    /// getVelocity) are fakes; AirPredictor_Prime, the layer query and
    /// Rand_NextGame are real. Rider_PlayBumpAnim returns at once (main state
    /// not 0/4). Rider_StartWipeout is bounded by making its first call (the
    /// vibrationBump fake again) unwind: every seed unwinds at or before that
    /// fake call index.
    ///
    /// Not reached: a fatal surface (Rider_CheckResetPath reaches audio and
    /// the disc-loading request loop).
    #[test]
    fn rider_handle_impact_matches() {
        use crate::difftest::fakes::{self, covered, machine};
        use crate::difftest::{put_vec, DATA, GAME_GP};
        covered(
            ee::s00::f_00105d98__generated,
            rider_handle_impact,
            400,
            &|c, seed| {
                let base = DATA + 0x20_0000;
                let rider = base;
                let info = base + 0x1000;
                let handler = base + 0x2000;
                let anim = base + 0x3000;
                let pred = base + 0x4000;
                let world = base + 0x5000;
                let game = base + 0x5100;
                let surfaces = base + 0x6000;
                let pos_v = base + 0x8000;
                let vel_v = base + 0x8100;
                let mut r = machine(c, seed, 0x105d98, 4);
                c.pc = 0x0010_5d98;
                let f = |x: f32| x.to_bits();

                // gp constants (.lit4) and the fast-speed global.
                for (off, v) in [
                    (-0x7f08i32, 0.8f32),
                    (-0x7f04, 416.532),
                    (-0x7f00, 833.337_6),
                    (-0x7efc, 0.2),
                    (-0x7ef8, 1.4),
                    (-0x7ef4, 1388.880_9),
                    (-0x7ef0, 2083.227_3),
                    (0x1b94, 500.0),
                ] {
                    c.bus.write32(GAME_GP.wrapping_add(off as u32), f(v));
                }
                // Rand_NextGame state.
                for i in 0..6u32 {
                    c.bus.write32(0x004f_f030 + 4 * i, r.next_u32());
                }

                // States.
                let sub = match seed % 8 {
                    0 => 9,
                    1 => 0xb,
                    _ => r.next_u32() % 9,
                };
                let main = match seed / 8 % 5 {
                    0 => 2,
                    1 | 2 => 1,
                    _ => 3,
                };
                c.bus.write32(rider + 0x77c, handler);
                c.bus.write32(handler + 0xde4, sub);
                c.bus.write32(handler + 0xde0, main);
                c.bus.write32(handler + 0x30, r.next_u32() % 2);

                // Methods: vibrationBump 0x88, getPosition 0x28, getVelocity 0x10.
                let vt = fakes::vtable(c, &mut r, &[0x88, 0x28, 0x10]);
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write32(fakes::LOG_RESULTS + 4, pos_v);
                c.bus.write32(fakes::LOG_RESULTS + 8, vel_v);
                for i in 0..8 {
                    c.bus.write32(pos_v + 4 * i, r.f32(500.0));
                    c.bus.write32(vel_v + 4 * i, r.f32(500.0));
                }
                // Unwind at or before Rider_StartWipeout's first call.
                let last = if main == 1 { 3 } else { 1 };
                let unwind = if r.next_u32() % 4 == 0 {
                    r.next_u32() % (last + 1)
                } else {
                    last
                };
                c.bus.write32(fakes::LOG_UNWIND_AT, unwind);

                // Air predictor and its launch argument.
                c.bus.write32(rider + 0x788, pred);
                for i in 0..64 {
                    c.bus.write32(pred + 4 * i, r.next_u32());
                }
                c.bus.write32(rider + 0x2e4, r.edge_f32());
                for i in 0..4 {
                    c.bus.write32(0x004f_f160 + i * 4, r.next_u32());
                    c.bus.write32(0x004f_f120 + i * 4, r.next_u32());
                }
                for i in 0..2 {
                    c.bus.write32(0x004a_5960 + i * 4, r.next_u32());
                }

                // Surface: -1 or a non-fatal entry of the surface table.
                let surface: u64 = if seed % 3 == 0 {
                    u64::MAX
                } else {
                    (r.next_u32() % 0x12) as u64
                };
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0x44, surfaces);
                for k in 0..0x12 {
                    c.bus.write32(surfaces + k * 0xb0 + 0x44, 0);
                }

                // Animation controller: layer 2 sequence (via AnimTable_GetEntry).
                c.bus.write32(rider + 0x784, anim);
                let idle = r.next_u32() % 6 == 0;
                c.bus.write32(anim + 8, if idle { 0x1b6 } else { 3 });
                let cur =
                    [0x12u32, 0x13, 0x14, 0x15, 0x11, 0x16, 0x40][(r.next_u32() % 7) as usize];
                c.bus.write32(0x0044_6990 + 3 * 0x1c, cur);

                // Board axes, origin, matrix row 2 and velocity.
                for off in [0x160u32, 0x170, 0x180] {
                    put_vec(c, rider + off, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                }
                put_vec(
                    c,
                    rider + 0x190,
                    [r.f32(100.0), r.f32(100.0), r.f32(100.0), f(1.0)],
                );
                put_vec(c, rider + 0x1c0, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                let speed = [0.0f32, 100.0, 400.0, 600.0, 3000.0][(r.next_u32() % 5) as usize];
                put_vec(
                    c,
                    rider + 0x1e0,
                    [r.f32(speed), r.f32(speed), r.f32(speed), 0],
                );
                put_vec(c, rider + 0x3e0, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                c.bus.write32(rider + 0x3f0, r.f32(3.0));

                // Impact info: contact point near the origin (heights around
                // 60), normal (steep on some seeds), impact speed.
                put_vec(c, info, [r.f32(150.0), r.f32(150.0), r.f32(150.0), f(1.0)]);
                let nz = if seed % 4 == 1 {
                    f(0.81 + (r.next_u32() % 19) as f32 * 0.01)
                } else {
                    r.f32(1.0)
                };
                put_vec(c, info + 0x20, [r.f32(1.0), r.f32(1.0), nz, 0]);
                let vn =
                    [100.0f32, 420.0, 900.0, 1500.0, 2500.0, 4000.0][(r.next_u32() % 6) as usize];
                c.bus.write32(info + 0x30, f(vn));

                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(info);
                c.g[A2][0] = surface;
            },
        );
    }

    /// Tick rate: the impact accumulator's gain is per contact tick. From the
    /// same rider and hit, two ticks at 120 Hz add what one tick adds at 60
    /// (and at 60 the gain is the original `1 - max(n . impulse, 0)`).
    #[test]
    fn rider_handle_impact_accumulator_follows_the_rate() {
        use crate::difftest::{put_vec, DATA};
        use impact::*;
        let run = |hz: u32, ticks: u32| -> f32 {
            let mut c = crate::difftest::fresh();
            crate::tick::set_for_test(&mut c, hz);
            let rider = DATA + 0x20_0000;
            let info = rider + 0x1000;
            let f = |x: f32| x.to_bits();
            put_vec(&mut c, info + INFO_NORMAL, [f(0.6), f(0.0), f(0.8), 0]);
            put_vec(&mut c, rider + IMPULSE, [f(0.0), f(0.6), f(0.8), 0]);
            c.bus.write32(rider + IMPACT_ACCUM, f(1.25));
            c.g[S1][0] = sx(rider);
            c.g[S3][0] = sx(info);
            for _ in 0..ticks {
                impact_accumulate(&mut c);
            }
            f32::from_bits(c.bus.read32(rider + IMPACT_ACCUM))
        };
        let at_60 = run(60, 1);
        assert!((at_60 - (1.25 + 1.0 - 0.64)).abs() < 1e-5, "{at_60}");
        let at_120 = run(120, 2);
        assert!(
            (at_120 - at_60).abs() < 1e-5,
            "120 Hz {at_120} vs 60 Hz {at_60}"
        );
        let at_240 = run(240, 4);
        assert!(
            (at_240 - at_60).abs() < 1e-5,
            "240 Hz {at_240} vs 60 Hz {at_60}"
        );
    }

    /// Tick rate: Rider_BounceOffWall's slow-hit gain to the bounce
    /// accumulator (+0x3f4) is per contact tick: 2 calls at 120 Hz and 4 at
    /// 240 add what one adds at 60 (`(2.5 - facing) * max(n.z, 0.2)`).
    #[test]
    fn rider_bounce_off_wall_accumulator_follows_the_rate() {
        use crate::difftest::{put_vec, DATA, GAME_GP};
        let run = |hz: u32, ticks: u32| -> f32 {
            let mut c = crate::difftest::fresh();
            crate::tick::set_for_test(&mut c, hz);
            let f = |x: f32| x.to_bits();
            c.g[GP][0] = sx(GAME_GP);
            c.bus
                .write32(GAME_GP.wrapping_sub(bow_lit::SLOW_SPEED), f(833.337_6));
            c.bus
                .write32(GAME_GP.wrapping_sub(bow_lit::MIN_NORMAL_Z), f(0.2));
            let rider = DATA + 0x20_0000;
            let hit = rider + 0x1000;
            put_vec(
                &mut c,
                rider + bow_rider::VEL,
                [f(100.0), f(0.0), f(0.0), 0],
            );
            c.bus.write32(rider + bow_rider::IMPACT_ACCUM, f(1.5));
            c.bus.write32(hit + bow_hit::NORMAL_Z, f(0.75));
            c.g[S2][0] = sx(rider);
            c.g[S1][0] = sx(hit);
            bow_set_f(&mut c, 22, FpuF32::from_f32(0.5));
            for _ in 0..ticks {
                bow_accumulate_impact(&mut c);
            }
            f32::from_bits(c.bus.read32(rider + bow_rider::IMPACT_ACCUM))
        };
        let at_60 = run(60, 1);
        assert!((at_60 - (1.5 + 2.0 * 0.75)).abs() < 1e-5, "{at_60}");
        let at_120 = run(120, 2);
        assert!(
            (at_120 - at_60).abs() < 1e-5,
            "120 Hz {at_120} vs 60 Hz {at_60}"
        );
        let at_240 = run(240, 4);
        assert!(
            (at_240 - at_60).abs() < 1e-5,
            "240 Hz {at_240} vs 60 Hz {at_60}"
        );
    }

    /// Control block of the gated fakes: the magic word (without it they
    /// run the slot's previous function, so other tests are unaffected),
    /// the queue cursors and the scripted answers.
    pub(crate) const BOW_CTRL: u32 = BOW_DATA + 0x20_0000;

    pub(crate) const BOW_MAGIC: u32 = 0x1057_b8a1;

    pub(crate) const BOW_SUB_NEXT: u32 = BOW_CTRL + 0x04;

    pub(crate) const BOW_MAIN_NEXT: u32 = BOW_CTRL + 0x08;

    /// `RiderAnimBase` answers: current animation, state, blend (f0).
    pub(crate) const BOW_ANIM_CURRENT: u32 = BOW_CTRL + 0x0c;

    pub(crate) const BOW_ANIM_STATE: u32 = BOW_CTRL + 0x10;

    pub(crate) const BOW_ANIM_BLEND: u32 = BOW_CTRL + 0x14;

    /// Eight answers each of `Rider_GetSubState` / `Rider_GetMainState`.
    pub(crate) const BOW_SUB_QUEUE: u32 = BOW_CTRL + 0x40;

    pub(crate) const BOW_MAIN_QUEUE: u32 = BOW_CTRL + 0x80;

    /// Fixture objects.
    pub(crate) const BOW_RIDER: u32 = BOW_DATA + 0x1000;

    pub(crate) const BOW_IFACE_VT: u32 = BOW_DATA + 0x0800;

    pub(crate) const BOW_HANDLER: u32 = BOW_DATA + 0x0c00;

    pub(crate) const BOW_OBJ: u32 = BOW_DATA + 0x3000;

    pub(crate) const BOW_HIT: u32 = BOW_DATA + 0x3400;

    pub(crate) const BOW_SURFACES: u32 = BOW_DATA + 0x3800;

    pub(crate) const BOW_HANDLE: u32 = BOW_DATA + 0x3c00;

    pub(crate) const BOW_HANDLE_VT: u32 = BOW_DATA + 0x3d00;

    pub(crate) const BOW_METRIX: u32 = BOW_DATA + 0x4000;

    /// A real leaf that returns 0 (the soft-wall answer of the handle).
    pub(crate) const BOW_RETURNS_ZERO: u32 = 0x0014_06c0;

    /// `RiderMetrix_FinishTrick`: always unwinds (it is reached only
    /// inside `Rider_BankTrickBoost`, whose remaining callees have no
    /// short path on test data).
    pub(crate) const BOW_FINISH_TRICK: u32 = 0x0011_a228;

    pub(crate) fn bow_next(c: &mut Cpu, cursor: u32, queue: u32) -> u64 {
        let n = c.bus.read32(cursor);
        c.bus.write32(cursor, n + 1);
        c.bus.read32(queue + 4 * n.min(7)) as u64
    }

    /// A logged fake (`fakes::fake`, which may unwind) with the scripted
    /// answers.
    pub(crate) fn bow_fake(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        bow_fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        match target {
            0x0011_fee8 => {
                let v = bow_next(c, BOW_SUB_NEXT, BOW_SUB_QUEUE);
                c.g[V0][0] = v;
            }
            0x0011_fe98 => {
                let v = bow_next(c, BOW_MAIN_NEXT, BOW_MAIN_QUEUE);
                c.g[V0][0] = v;
            }
            0x0031_1ae8 => c.g[V0][0] = c.bus.read32(BOW_ANIM_CURRENT) as u64,
            0x0031_2aa0 => c.g[V0][0] = c.bus.read32(BOW_ANIM_STATE) as u64,
            0x0031_2ab0 => c.f[0] = c.bus.read32(BOW_ANIM_BLEND),
            BOW_FINISH_TRICK => c.pc = 0xdead_f000,
            _ => {}
        }
    }

    macro_rules! bow_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(BOW_CTRL) == BOW_MAGIC {
                    return bow_fake(c, $addr);
                }
                let p = $prev.load(Ordering::Relaxed);
                if p == 0 || p == $name as *const () as usize {
                    return $gen(c);
                }
                // SAFETY: `p` was read from the same slot (a `fn(&mut Cpu)`).
                let f = unsafe { std::mem::transmute::<usize, fn(&mut Cpu)>(p) };
                f(c)
            }
        };
    }

    bow_gated!(
        bow_fake_sub_state,
        BOW_PREV_SUB_STATE,
        0x0011_fee8,
        ee::s00::f_0011fee8__generated
    );

    bow_gated!(
        bow_fake_main_state,
        BOW_PREV_MAIN_STATE,
        0x0011_fe98,
        ee::s00::f_0011fe98__generated
    );

    bow_gated!(
        bow_fake_set_sub_state,
        BOW_PREV_SET_SUB_STATE,
        0x0011_fec8,
        ee::s00::f_0011fec8__generated
    );

    bow_gated!(
        bow_fake_translate,
        BOW_PREV_TRANSLATE,
        0x0010_6538,
        ee::s00::f_00106538__generated
    );

    bow_gated!(
        bow_fake_world_matrix,
        BOW_PREV_WORLD_MATRIX,
        0x0011_e098,
        ee::s00::f_0011e098__generated
    );

    bow_gated!(
        bow_fake_air_prime,
        BOW_PREV_AIR_PRIME,
        0x0011_35b8,
        ee::s00::f_001135b8__generated
    );

    bow_gated!(
        bow_fake_finish_trick,
        BOW_PREV_FINISH_TRICK,
        BOW_FINISH_TRICK,
        ee::s00::f_0011a228__generated
    );

    bow_gated!(
        bow_fake_anim_current,
        BOW_PREV_ANIM_CURRENT,
        0x0031_1ae8,
        ee::s10::f_00311ae8__generated
    );

    bow_gated!(
        bow_fake_anim_state,
        BOW_PREV_ANIM_STATE,
        0x0031_2aa0,
        ee::s10::f_00312aa0__generated
    );

    bow_gated!(
        bow_fake_anim_blend,
        BOW_PREV_ANIM_BLEND,
        0x0031_2ab0,
        ee::s10::f_00312ab0__generated
    );

    bow_gated!(
        bow_fake_anim_play,
        BOW_PREV_ANIM_PLAY,
        0x0031_28e8,
        ee::s10::f_003128e8__generated
    );

    /// Put `fake` in `slot` once, chaining to what was there.
    pub(crate) fn bow_hook(slot: *mut fn(&mut Cpu), fake: fn(&mut Cpu), prev: &AtomicUsize) {
        crate::difftest::fakes::hook(slot, fake, prev);
    }

    #[allow(unused_unsafe)]
    pub(crate) fn bow_install_fakes() {
        unsafe {
            use std::ptr::addr_of_mut;
            bow_hook(
                addr_of_mut!(ee::s00::OVERRIDE_0011FEE8),
                bow_fake_sub_state,
                &BOW_PREV_SUB_STATE,
            );
            bow_hook(
                addr_of_mut!(ee::s00::OVERRIDE_0011FE98),
                bow_fake_main_state,
                &BOW_PREV_MAIN_STATE,
            );
            bow_hook(
                addr_of_mut!(ee::s00::OVERRIDE_0011FEC8),
                bow_fake_set_sub_state,
                &BOW_PREV_SET_SUB_STATE,
            );
            bow_hook(
                addr_of_mut!(ee::s00::OVERRIDE_00106538),
                bow_fake_translate,
                &BOW_PREV_TRANSLATE,
            );
            bow_hook(
                addr_of_mut!(ee::s00::OVERRIDE_0011E098),
                bow_fake_world_matrix,
                &BOW_PREV_WORLD_MATRIX,
            );
            bow_hook(
                addr_of_mut!(ee::s00::OVERRIDE_001135B8),
                bow_fake_air_prime,
                &BOW_PREV_AIR_PRIME,
            );
            bow_hook(
                addr_of_mut!(ee::s00::OVERRIDE_0011A228),
                bow_fake_finish_trick,
                &BOW_PREV_FINISH_TRICK,
            );
            bow_hook(
                addr_of_mut!(ee::s10::OVERRIDE_00311AE8),
                bow_fake_anim_current,
                &BOW_PREV_ANIM_CURRENT,
            );
            bow_hook(
                addr_of_mut!(ee::s10::OVERRIDE_00312AA0),
                bow_fake_anim_state,
                &BOW_PREV_ANIM_STATE,
            );
            bow_hook(
                addr_of_mut!(ee::s10::OVERRIDE_00312AB0),
                bow_fake_anim_blend,
                &BOW_PREV_ANIM_BLEND,
            );
            bow_hook(
                addr_of_mut!(ee::s10::OVERRIDE_003128E8),
                bow_fake_anim_play,
                &BOW_PREV_ANIM_PLAY,
            );
        }
    }

    /// A random quadword of floats in [-range, range).
    pub(crate) fn bow_vec(c: &mut Cpu, r: &mut crate::difftest::Rng, at: u32, range: f32) {
        let v = [r.f32(range), r.f32(range), r.f32(range), r.f32(range)];
        bow_put_vec(c, at, v);
    }

    /// Sub state 9 (nothing); a bounceless surface (bump animation, its
    /// real `Rider_PlayBumpAnim` takes the short path); moving toward or away
    /// from the wall; solid walls (no handle, or the handle's slot answers
    /// non-zero: push out, with and without the ground deflection, the
    /// real `Rider_DeflectAlongWall` runs) and soft ones (the slot is a real
    /// leaf returning 0); slow-hit states 1 / 2 with the handler in mode 1
    /// or not, slow or fast riders, steep or flat normals; the launch test
    /// (state, animation 9/2/0xb/1/other, facing) and the launch with and
    /// without the bail restart (`Rider_BankTrickBoost` runs for real until
    /// its `RiderMetrix_FinishTrick`, which unwinds); the plain rebound
    /// with the real `Rider_HandleImpact` (sub state 9: its short path).
    /// The other callees are logged fakes, one of which may unwind.
    #[test]
    fn rider_bounce_off_wall_matches() {
        bow_install_fakes();
        bow_covered(
            ee::s00::f_001057b8__generated,
            rider_bounce_off_wall,
            300,
            &|c, seed| {
                bow_install_fakes();
                let mut r = bow_machine(c, seed, 0x0010_57b8, 8);
                c.pc = 0x0010_57b8;

                // .lit4 constants at gp-0x7f30 .. gp-0x7f0c.
                bow_put_words(
                    c,
                    BOW_GP - 0x7f30,
                    &[
                        0x3f8c_cccd,
                        0x425e_38e4,
                        0x44ad_9c72,
                        0x4450_5556,
                        0x3e4c_cccd,
                        0x3e99_999a,
                        0x438a_e38e,
                        0x440a_e38e,
                        0x41de_38e4,
                        0x3e4c_cccd,
                    ],
                );

                // The scripted answers.
                c.bus.write32(BOW_CTRL, BOW_MAGIC);
                let sub0 = if seed % 10 == 0 {
                    9
                } else {
                    [0, 1, 2, 5][(r.next_u32() % 4) as usize]
                };
                c.bus.write32(BOW_SUB_QUEUE, sub0);
                for i in 1..8 {
                    c.bus.write32(BOW_SUB_QUEUE + 4 * i, 9);
                }
                let no_bounce = seed % 7 == 1;
                for i in 0..8 {
                    let v = if no_bounce {
                        [1, 2, 3][(r.next_u32() % 3) as usize]
                    } else {
                        [0, 1, 1, 2, 3][(r.next_u32() % 5) as usize]
                    };
                    c.bus.write32(BOW_MAIN_QUEUE + 4 * i, v);
                }
                c.bus.write32(
                    BOW_ANIM_CURRENT,
                    [9, 2, 0xb, 1, 5][(r.next_u32() % 5) as usize],
                );
                c.bus.write32(
                    BOW_ANIM_STATE,
                    if r.next_u32() % 2 == 0 { 0x10c } else { 0x10b },
                );
                let blend = r.f32(0.5);
                c.bus.write32(BOW_ANIM_BLEND, blend);

                // Arguments: rider, collision object, info, hit, depth.
                c.g[A0][0] = BOW_RIDER as u64;
                c.g[A1][0] = BOW_OBJ as u64;
                c.g[A2][0] = (BOW_DATA + 0x5000) as u64;
                c.g[A3][0] = BOW_HIT as u64;
                c.f[12] = r.f32(20.0);

                // The hit: normal, wall velocity, material, surface index.
                bow_vec(c, &mut r, BOW_HIT + 0x10, 1.0);
                bow_vec(c, &mut r, BOW_HIT + 0x20, 600.0);
                let mat = r.next_u32();
                c.bus.write32(BOW_HIT + 0x4c, mat);
                let surf = r.next_u32() % 4;
                c.bus.write32(BOW_HIT + 0x5c, surf);
                for i in 0..4 {
                    let entry = BOW_SURFACES + 0x10 + 12 * i;
                    let bounce = if no_bounce { 0 } else { r.f32(80.0) };
                    c.bus.write32(entry, bounce);
                    let grip = r.f32(1.5);
                    c.bus.write32(entry + 4, grip);
                }

                // The collision object: surface table, optional handle whose
                // slot +0x70 answers solid (fake: non-zero) or soft (0).
                c.bus.write32(BOW_OBJ + 0x88, BOW_SURFACES);
                let handle = if seed % 3 == 0 { 0 } else { BOW_HANDLE };
                c.bus.write32(BOW_OBJ + 0xc, handle);
                c.bus.write32(BOW_HANDLE + 0xc, BOW_HANDLE_VT);
                c.bus
                    .write16(BOW_HANDLE_VT + 0x70, ((r.next_u32() % 8) * 4) as u16);
                let push = if r.next_u32() % 2 == 0 {
                    BOW_FAKE + 0x80
                } else {
                    BOW_RETURNS_ZERO
                };
                c.bus.write32(BOW_HANDLE_VT + 0x74, push);

                // The rider.
                if r.next_u32() % 2 == 0 {
                    // Facing the wall's normal.
                    let n = c.bus.read128(BOW_HIT + 0x10);
                    c.bus.write128(BOW_RIDER + 0x180, n);
                } else {
                    bow_vec(c, &mut r, BOW_RIDER + 0x180, 1.0);
                }
                let speed = if r.next_u32() % 2 == 0 { 300.0 } else { 1200.0 };
                bow_vec(c, &mut r, BOW_RIDER + 0x1e0, speed);
                let w = r.f32(5.0);
                c.bus.write32(BOW_RIDER + 0x2e4, w);
                let (stance, natural) = (r.next_u32() % 2, r.next_u32() % 2);
                c.bus.write32(BOW_RIDER + 0x320, stance);
                c.bus.write32(BOW_RIDER + 0x324, natural);
                let accum = r.f32(10.0);
                c.bus.write32(BOW_RIDER + 0x3f4, accum);
                let surface_type = r.next_u32();
                c.bus.write32(BOW_RIDER + 0x438, surface_type);
                c.bus.write32(BOW_RIDER + 0x6c0, BOW_IFACE_VT);
                c.bus.write32(BOW_RIDER + 0x77c, BOW_HANDLER);
                c.bus.write32(BOW_HANDLER + 0x30, r.next_u32() % 2);
                let anim = bow_fakes::ptr(&mut r);
                c.bus.write32(BOW_RIDER + 0x784, anim);
                let predictor = bow_fakes::ptr(&mut r);
                c.bus.write32(BOW_RIDER + 0x788, predictor);
                c.bus.write32(BOW_RIDER + 0x790, BOW_METRIX);
                for off in [0x14u32, 0x70, 0x114, 0x134, 0x198] {
                    let v = r.next_u32() % 16;
                    c.bus.write32(BOW_METRIX + off, v);
                }
                // RiderIface vtable: vibrationBump, getPosition, getVelocity.
                for (i, off) in [0x88u32, 0x28, 0x10].into_iter().enumerate() {
                    c.bus
                        .write16(BOW_IFACE_VT + off, ((r.next_u32() % 8) * 4) as u16);
                    c.bus
                        .write32(BOW_IFACE_VT + off + 4, BOW_FAKE + 0x10 * i as u32);
                }
            },
        );
    }

    /// A stick vector (x at DATA, y at DATA+4): zero, on an axis, tiny
    /// (within eps), or anywhere in [-1.5, 1.5]^2; steps of pi/4, pi/8 or
    /// random; the real `Math_Atan` / `Math_SinCos` and the game's `.lit4`.
    pub(crate) fn quantize_stick_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::{elf_image, GAME_GP};
        let mut r = Rng(0x1158_b800 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        elf_image(c);
        c.g[GP][0] = GAME_GP as u64;
        let x_ptr = DATA;
        let y_ptr = DATA + 4;
        let mut comp = |k: u32| -> u32 {
            match k % 6 {
                0 => 0,
                1 => 0x8000_0000,
                2 => r.f32(0.05),
                3 => r.edge_f32(),
                _ => r.f32(1.5),
            }
        };
        let (x, y) = match seed % 5 {
            0 => (0, comp(seed / 5)),
            1 => (comp(seed / 5), 0),
            _ => (comp(seed / 5), comp(seed / 7 + 3)),
        };
        c.bus.write32(x_ptr, x);
        c.bus.write32(y_ptr, y);
        c.f[12] = match seed % 4 {
            0 => std::f32::consts::FRAC_PI_4.to_bits(),
            1 => std::f32::consts::FRAC_PI_8.to_bits(),
            2 => (0.3f32).to_bits(),
            _ => r.f32(2.0) & 0x7fff_ffff | 0x3c00_0000,
        };
        c.f[13] = match seed % 3 {
            0 => (0.1f32).to_bits(),
            1 => 0,
            _ => r.f32(0.2) & 0x7fff_ffff,
        };
        c.g[A0][0] = sx(DATA + 0x100);
        c.g[A1][0] = sx(y_ptr);
        c.g[A2][0] = sx(x_ptr);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0011_58b8;
    }

    /// `Input_QuantizeStickDirection`: still stick, vertical (up/down),
    /// horizontal, all quadrants, clamping and the final snap to zero.
    #[test]
    fn input_quantize_stick_direction_matches() {
        check(
            ee::s00::f_001158b8__generated,
            input_quantize_stick_direction,
            200,
            &quantize_stick_fixture,
        );
    }

    /// `PATH_RemapNode`: the path table at `*0x44f420` with index (+0x38),
    /// words (+0x34) and ranges (+0x48). Ids are negative on some seeds; the
    /// range byte k is 0..3 (0: no remap); ranges are sometimes empty or
    /// reversed (n <= 0); range words remap small node numbers so that
    /// matches (and chains of them) happen.
    #[test]
    fn path_remap_node_matches() {
        const TABLE: u32 = DATA;
        const WORDS: u32 = DATA + 0x1000;
        const INDEX: u32 = DATA + 0x3000;
        const RANGES: u32 = DATA + 0x3800;
        check(
            ee::s14::f_003d3c90__generated,
            path_remap_node,
            300,
            &|c, seed| {
                let mut r = Rng(0x3d3c90 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.pc = 0x003d_3c90;
                c.bus.write32(0x0045_0000 - 0xbe0, TABLE);
                c.bus.write32(TABLE + 0x34, WORDS);
                c.bus.write32(TABLE + 0x38, INDEX);
                c.bus.write32(TABLE + 0x48, RANGES);
                let id = r.next_u32() % 64;
                let word = r.next_u32() % 256;
                c.bus.write16(INDEX + 2 * id, word as u16);
                let k = seed % 4;
                c.bus.write8(WORDS + 4 * word + 4, k as u8);
                // ranges[0..4]: increasing, sometimes empty or reversed
                let mut at = r.next_u32() % 64;
                for i in 0..4 {
                    c.bus.write32(RANGES + 4 * i, 0x200 + at);
                    at = match r.next_u32() % 6 {
                        0 => at,
                        1 => at.saturating_sub(3),
                        _ => at + 1 + r.next_u32() % 12,
                    };
                }
                for i in 0..0x300 {
                    let from = r.next_u32() % 6;
                    let to = if r.next_u32() % 4 == 0 {
                        r.next_u32() & 0xffff
                    } else {
                        r.next_u32() % 6
                    };
                    c.bus.write32(WORDS + 4 * (0x200 + i), from << 16 | to);
                }
                c.g[A0][0] = if seed % 5 == 0 {
                    sx(0x8000_0000 | r.next_u32())
                } else {
                    id as u64
                };
                c.g[A1][0] = (r.next_u32() % 6) as u64;
            },
        );
    }

    /// `WScriptFn_SetSelfObject`: defaults initialised or not; 0, negative or
    /// 1..3 arguments (ids, ints and floats into int and float slots); class
    /// tables with missing classes and handle words with a zero handle; the
    /// real EA_MemCopy.
    #[test]
    fn wscript_fn_set_self_object_matches() {
        use set_self_object::*;
        check(
            ee::s10::f_00302968__generated,
            wscript_fn_set_self_object,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_2968, 0);
                c.pc = 0x0030_2968;
                let gp = |off: u32| GAME_GP.wrapping_add(off);
                c.bus.write32(gp(GP_DEFAULTS_READY), (seed % 4 != 0) as u32);
                fill(c, &mut r, RESULT_OK, 3);
                // An object id: class (low byte) 0..7 and handle 0..15, or one
                // with class 0xff (never present).
                let some_id = |r: &mut Rng| -> u32 {
                    if r.next_u32() % 4 == 0 {
                        r.next_u32() | 0xff00_00ff
                    } else {
                        (r.next_u32() % 16) << 8 | r.next_u32() % 8
                    }
                };
                let default_id = some_id(&mut r);
                c.bus.write32(gp(GP_DEFAULTS), default_id);
                c.bus.write32(gp(GP_DEFAULTS_FLAG), r.next_u32());
                // Declared types: the id slot int (never converted: a float's
                // bits would index far outside the handle table), the others
                // int or float.
                c.bus.write32(gp(GP_ARG_TYPES), r.next_u32() % 2);
                for k in 1..4 {
                    c.bus.write32(gp(GP_ARG_TYPES + 4 * k), r.next_u32() % 3);
                }
                let mgr = DATA + 0x100;
                let m = DATA + 0x200;
                let tables = DATA + 0x300;
                let class = DATA + 0x400;
                let handles = DATA + 0x800;
                let world = DATA + 0xc00;
                c.bus.write32(gp(GP_OBJECT_MANAGER), mgr);
                c.bus.write32(mgr, m);
                c.bus.write32(m + 8, tables);
                for k in 0..256 {
                    let present = k < 8 && r.next_u32() % 4 != 0;
                    c.bus
                        .write32(tables + 4 * k, if present { class } else { 0 });
                }
                c.bus.write32(class + 0x1c, handles);
                for k in 0..16 {
                    let word = if r.next_u32() % 4 == 0 {
                        r.next_u32() & 0xff
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(handles + 4 * k, word);
                }
                c.bus.write32(gp(GP_WORLD), world);
                let args = DATA + 0x2000;
                let n = match seed % 6 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    _ => 1 + r.next_u32() % 3,
                };
                for k in 0..3 {
                    let a = args + 0x10 * k;
                    let idx = if k == 0 { 0 } else { r.next_u32() % 2 };
                    c.bus.write32(a, idx);
                    let val = if idx == 0 && r.next_u32() % 4 != 0 {
                        some_id(&mut r)
                    } else {
                        r.next_u32() % 1000
                    };
                    c.bus.write32(a + 4, val);
                    c.bus.write32(a + 8, r.next_u32());
                    c.bus.write32(a + 0xc, r.next_u32() % 3);
                }
                c.g[A0][0] = sx(DATA + 0x3000 + (seed % 2) * 4);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(args);
            },
        );
    }

    /// A random 3-vector (w random too) of components in [-1, 1].
    pub(crate) fn deflect_vec(c: &mut Cpu, r: &mut Rng, addr: u32) {
        for i in 0..4 {
            c.bus.write32(addr + 4 * i, r.f32(1.0));
        }
    }

    /// `Rider_DeflectAlongWall` with the real atanf and Math_SinCos on the
    /// game image: random up/forward/normal vectors (forward sometimes
    /// along the wall normal or the tangent, so x = 0 or y = 0, and
    /// sometimes zero: no turn), unit orientation quaternions.
    #[test]
    fn rider_deflect_along_wall_matches() {
        use deflect_along_wall::*;
        check(
            ee::s00::f_001065b0__generated,
            rider_deflect_along_wall,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_65b0, 0);
                elf_image(c);
                c.pc = 0x0010_65b0;
                let rider = DATA;
                let contact = DATA + 0x800;
                // Up mostly vertical, the wall normal mostly horizontal.
                let up = [r.f32(0.3), 1.0f32.to_bits(), r.f32(0.3), r.next_u32()];
                for (i, w) in up.iter().enumerate() {
                    c.bus.write32(rider + UP + 4 * i as u32, *w);
                }
                deflect_vec(c, &mut r, contact + CONTACT_NORMAL);
                c.bus.write32(contact + CONTACT_NORMAL + 4, r.f32(0.2));
                deflect_vec(c, &mut r, rider + ORIENTATION);
                match seed % 6 {
                    0 => {
                        // Zero forward: x = y = 0.
                        for i in 0..4 {
                            c.bus.write32(rider + FORWARD + 4 * i, 0);
                        }
                    }
                    1 => {
                        // Forward along up: both dots ~0 (x exactly 0 when up
                        // is exactly vertical).
                        c.bus.write32(rider + UP, 0);
                        c.bus.write32(rider + UP + 8, 0);
                        c.bus.write32(rider + FORWARD, 0);
                        c.bus.write32(rider + FORWARD + 4, r.f32(1.0));
                        c.bus.write32(rider + FORWARD + 8, 0);
                    }
                    2 => {
                        // Up vertical, normal along x: n along z, t along x; a
                        // forward along z gives x = 0.
                        c.bus.write32(rider + UP, 0);
                        c.bus.write32(rider + UP + 8, 0);
                        c.bus.write32(contact + CONTACT_NORMAL, 1.0f32.to_bits());
                        c.bus.write32(contact + CONTACT_NORMAL + 4, 0);
                        c.bus.write32(contact + CONTACT_NORMAL + 8, 0);
                        c.bus.write32(rider + FORWARD, 0);
                        c.bus.write32(rider + FORWARD + 4, r.f32(1.0));
                        c.bus.write32(rider + FORWARD + 8, r.f32(1.0));
                    }
                    _ => deflect_vec(c, &mut r, rider + FORWARD),
                }
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(contact);
            },
        );
    }

    /// `StreamFile_QueueRange`: an invalid file (null, or no "STRM"
    /// header), no free request, then a queued request on an empty or a
    /// non-empty queue, with the stream busy or idle (then
    /// `StartNextRequest` at either priority: the request's name matches
    /// the open file and `IssueRead` stalls at once). The mutex syscalls go
    /// to the fakes (some unwind).
    #[test]
    fn stream_file_queue_range_matches() {
        const FILE: u32 = DATA;
        const HDR: u32 = DATA + 0x1000;
        const REQ: u32 = DATA + 0x2000;
        const NEXT_FREE: u32 = DATA + 0x2400;
        const TAIL: u32 = DATA + 0x2800;
        const NAME: u32 = DATA + 0x3000;
        const RING: u32 = DATA + 0x4000;
        check(
            ee::s14::f_003e0c88__generated,
            stream_file_queue_range,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_0c88, 8);
                c.pc = 0x003e_0c88;
                let w = |c: &mut ssx3_ee_rt::Cpu, a: u32, v: u32| c.bus.write32(a, v);
                c.g[A0][0] = if seed % 11 == 0 { 0 } else { FILE as u64 };
                w(c, FILE, HDR);
                w(
                    c,
                    HDR,
                    if seed % 13 == 1 {
                        r.next_u32()
                    } else {
                        0x4d52_5453
                    },
                );
                // Mutex: owner (never the fake thread id), depth, semaphore.
                w(c, HDR + 0x8, 0x7700_0000 | r.next_u32() & 0xffff);
                w(c, HDR + 0xc, r.next_u32() & 3);
                w(c, HDR + 0x10, r.next_u32() & 0xff);
                // Free list: one request (or none), id counter.
                w(c, HDR + 0x68, if seed % 9 == 4 { 0 } else { REQ });
                w(c, REQ + 0xc, NEXT_FREE);
                w(c, REQ, r.next_u32());
                w(
                    c,
                    0x0051_9c18,
                    if seed % 7 == 2 {
                        0xffff_ff00
                    } else {
                        r.next_u32()
                    },
                );
                // Queue: empty or one request.
                let queued = seed % 3 == 0;
                w(c, HDR + 0x64, if queued { TAIL } else { 0 });
                w(c, HDR + 0x5c, if queued { TAIL } else { 0 });
                w(c, HDR + 0x60, if queued { TAIL } else { 0 });
                w(c, TAIL + 4, 2);
                w(c, TAIL + 0xc, 0);
                // Busy or idle; the two priorities.
                w(
                    c,
                    HDR + 0x38,
                    if seed % 2 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 2
                    },
                );
                w(c, HDR + 0x48, (seed / 2) % 2);
                w(c, HDR + 0x3c, r.next_u32() % 8);
                w(c, HDR + 0x40, r.next_u32() % 8);
                // The file name, also the open file's name.
                for (i, b) in b"track01.big\0".iter().enumerate() {
                    c.bus.write8(NAME + i as u32, *b);
                    c.bus.write8(HDR + 0x6c + i as u32, *b);
                }
                c.g[A1][0] = NAME as u64;
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32() & 0xffff);
                // StreamFile_IssueRead stall path.
                w(c, HDR + 0x170, r.next_u32());
                w(c, HDR + 0x16c, 0);
                w(c, HDR + 0x30, RING);
                w(c, HDR + 0x34, DATA + 0x5000);
                w(c, HDR + 0x50, RING);
                w(c, HDR + 0x54, RING);
                w(c, HDR + 0x58, r.next_u32());
                w(c, HDR + 0x17c, 0x1000);
            },
        );
    }

    /// `ComputerActorRider_PlanAirTricks`: spin / flip wanted (1..3) or not
    /// (0), the real `RollSkipTricks` and `Rand_NextGame` on a random
    /// generator state (both signs of each axis).
    #[test]
    fn computer_actor_rider_plan_air_tricks_matches() {
        const RIDER: u32 = DATA + 0x20_0000;
        check(
            ee::s00::f_0010bfa8__generated,
            computer_actor_rider_plan_air_tricks,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_bfa8, 0);
                c.pc = 0x0010_bfa8;
                c.g[A0][0] = sx(RIDER);
                let pick = |r: &mut Rng, on: bool| -> u64 {
                    if on {
                        1 + (r.next_u32() % 3) as u64
                    } else {
                        0
                    }
                };
                c.g[A1][0] = pick(&mut r, seed % 2 == 1);
                c.g[A2][0] = pick(&mut r, seed / 2 % 2 == 1);
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                for i in 0..0x100 {
                    c.bus.write32(RIDER + 0xd00 + 4 * i, r.next_u32());
                }
                c.bus.write32(RIDER + 0xdfc, r.f32(100.0));
                // Rand_NextGame's generator state.
                for i in 0..8 {
                    c.bus.write32(0x004f_f030 + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// WorldObjFactory_AddDynamicParticleData: the default record fresh
    /// (guard 0) or already built (random words, the range in either
    /// order); 0..4 entries (and n <= 0) with field types random against
    /// the type table (int-into-float conversions), optionally setting the
    /// handle (-1, a resolving one, one whose table slot or index is empty)
    /// and the modifier slot; the target node's `+0x80` virtual is a fake
    /// (accepts: the real ModObj_AddDynamicParticleModifier, which frees the
    /// record on its short paths) or a leaf returning 0 (the record is
    /// deleted). The record comes from a dlmalloc arena over test memory.
    #[test]
    fn world_obj_factory_add_dynamic_particle_data_matches() {
        use crate::difftest::fakes::{FAKE, LOG_COUNT, LOG_UNWIND_AT};
        use crate::difftest::GAME_GP;
        use crate::game::bx::cubicspline::tests::oc238_run;
        const OUT: u32 = DATA + 0x500;
        const ENTRIES: u32 = DATA + 0x600;
        const SCRIPT_MAN: u32 = DATA + 0x2000;
        const HANDLES: u32 = DATA + 0x2100;
        const HANDLE_TABLE: u32 = DATA + 0x2200;
        const SETS: u32 = DATA + 0x2300;
        const SET: u32 = DATA + 0x2400;
        const SET_INDEX: u32 = DATA + 0x2500;
        const OBJ: u32 = DATA + 0x3000;
        const NODE: u32 = DATA + 0x3100;
        const NODE_VT: u32 = DATA + 0x3200;
        const MODS: u32 = DATA + 0x3300;
        const MOD_LIST: u32 = DATA + 0x3400;
        check(
            ee::s10::f_002fee98__generated,
            world_obj_factory_add_dynamic_particle_data,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2fee98, 0);
                // The game heap over test memory, pools off.
                c.bus.write32(GAME_GP + 0xdd8, 0);
                oc238_run(
                    c,
                    0x0031_ff60,
                    ee::s10::f_0031ff60,
                    &[0x004f_f250, (DATA + 0x40000) as u64, 0x40000],
                );
                // The default record: fresh, or built (random words).
                if seed % 2 == 0 {
                    c.bus.write32(GAME_GP + DPD_GUARD, 0);
                } else {
                    c.bus.write32(GAME_GP + DPD_GUARD, 1);
                    fill(c, &mut r, DPD_TEMPLATE, 0x3c);
                    c.bus.write32(DPD_TEMPLATE + 0x14, r.f32(10.0));
                    c.bus.write32(DPD_TEMPLATE + 0x1c, r.f32(10.0));
                    c.bus.write32(DPD_TEMPLATE + 0xd8, u32::MAX);
                    c.bus.write32(DPD_TEMPLATE + 0xdc, u32::MAX);
                }
                fill(c, &mut r, DPD_TEMPLATE + 0xec, 1);
                // Field types: 0..2 (2: float); handle and slot plain ints.
                for i in 0..0x3c {
                    c.bus.write32(DPD_FIELD_TYPES + 4 * i, r.next_u32() % 3);
                }
                c.bus.write32(DPD_FIELD_TYPES + 4 * 54, 0);
                c.bus.write32(DPD_FIELD_TYPES + 4 * 55, 0);
                // Entries.
                let mut entries = Vec::new();
                for _ in 0..r.next_u32() % 4 {
                    let field = r.next_u32() % 54;
                    entries.push((field, r.next_u32() % 0x400, r.next_u32() % 3));
                }
                let handle = match seed % 5 {
                    0 => None,
                    1 => Some(u32::MAX),
                    2 => Some(0x0300 | 5), // empty set slot
                    3 => Some(0x0400 | 6), // empty index
                    _ => Some(0x0200 | 7), // resolves to OBJ
                };
                if let Some(h) = handle {
                    entries.push((54, h, 0));
                }
                if seed % 3 != 0 {
                    entries.push((55, if seed % 6 == 1 { u32::MAX } else { 3 }, 0));
                }
                for (i, &(field, value, ty)) in entries.iter().enumerate() {
                    let e = ENTRIES + 0x10 * i as u32;
                    c.bus.write32(e, field);
                    c.bus.write32(e + 4, value);
                    c.bus.write32(e + 0xc, ty);
                }
                let n = if seed % 13 == 0 {
                    u32::MAX
                } else {
                    entries.len() as u32
                };
                // Handle lookup tables.
                c.bus.write32(GAME_GP + DPD_SCRIPT_MAN, SCRIPT_MAN);
                c.bus.write32(SCRIPT_MAN + 0x290, OBJ);
                c.bus.write32(GAME_GP + DPD_HANDLES, HANDLES);
                c.bus.write32(HANDLES, HANDLE_TABLE);
                c.bus.write32(HANDLE_TABLE + 8, SETS);
                for j in 0..8 {
                    c.bus.write32(SETS + 4 * j, if j == 5 { 0 } else { SET });
                }
                for k in 0..8 {
                    let v = if k == 4 {
                        r.next_u32() & 0xff
                    } else {
                        (OBJ >> 2) << 8
                    };
                    c.bus.write32(SET + 0x1c, SET_INDEX);
                    c.bus.write32(SET_INDEX + 4 * k, v);
                }
                c.bus.write32(0xc, 0); // the null object's node
                c.bus
                    .write32(OBJ + 0xc, if seed % 7 == 3 { 0 } else { NODE });
                c.bus.write32(NODE + 0xc, NODE_VT);
                c.bus
                    .write16(NODE_VT + 0x80, ((r.next_u32() % 0x10) & !3) as u16);
                c.bus.write32(
                    NODE_VT + 0x84,
                    if seed % 4 == 1 { 0x0014_06c0 } else { FAKE },
                );
                // No modifier slots: AddDynamicParticleModifier frees the record.
                c.bus.write32(NODE + 0x18, MODS);
                c.bus.write32(MODS + 0x80, MOD_LIST);
                c.bus.write32(MOD_LIST + 4, 0);
                // The returned constant.
                fill(c, &mut r, 0x004c_9098, 3);
                if seed % 3 == 2 {
                    let k = c.bus.read32(LOG_COUNT);
                    c.bus.write32(LOG_UNWIND_AT, k + r.next_u32() % 6);
                }
                c.g[A0][0] = sx(OUT);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(ENTRIES);
                c.pc = 0x002f_ee98;
            },
        );
    }

    /// RefPack_Unpack: a real RefPack stream (`game::misc` tests' generator)
    /// whose type byte is every value of interest (0x10/0x11/0x90/0x91
    /// decode; 0x46, 0x1c, 0x18, 0x9e, 0x98 and the values around and
    /// between them do not), or a header without the `fb` marker.
    #[test]
    fn refpack_unpack_matches() {
        use crate::game::misc::tests::refpack_stream;
        const KINDS: &[u8] = &[
            0x10, 0x11, 0x90, 0x91, 0x46, 0x47, 0x1c, 0x1d, 0x18, 0x19, 0x9e, 0x98, 0x99, 0x00,
            0x02, 0x12, 0x1a, 0x1e, 0x30, 0x48, 0x60, 0x80, 0x92, 0x9a, 0xa0, 0xfe, 0x10, 0x90,
        ];
        check(
            ee::s13::f_003b4740__generated,
            refpack_unpack,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3b4740, 0);
                let src = DATA + 0x1000;
                // The generator's header variant follows the type byte's bits
                // (0x80: 4-byte sizes, 0x01: packed size present).
                let kind = KINDS[seed as usize % KINDS.len()];
                let variant = (kind >> 7) as u32 | ((kind & 1) as u32) << 1;
                let mut stream = refpack_stream(&mut r, variant);
                stream[0] = kind;
                if seed % 11 == 5 {
                    stream[1] = r.next_u32() as u8 & 0xfa;
                }
                for (i, b) in stream.into_iter().enumerate() {
                    c.bus.write8(src + i as u32, b);
                }
                c.g[A0][0] = sx(src);
                c.g[A1][0] = sx(DATA + 0x4_0000);
                c.pc = 0x003b_4740;
            },
        );
    }

    /// log: every `_LIB_VERSION` mode (IEEE, SVID, XOPEN, POSIX) against
    /// positive, zero (both signs), negative, NaN and infinite arguments
    /// (the real soft-float callees, matherr and errno from the ELF image).
    #[test]
    fn libm_log_matches() {
        use crate::difftest::elf_image;
        check(ee::s15::f_0040d610__generated, libm_log, 300, &|c, seed| {
            elf_image(c);
            let mut r = fakes::machine(c, seed, 0x40d610, 0);
            let version = [u32::MAX, 0, 1, 2][(seed % 4) as usize];
            c.bus.write32(LOG_LIB_VERSION, version);
            let x: f64 = match (seed / 4) % 8 {
                0 => 0.0,
                1 => -0.0,
                2 => -(1.0 + (r.next_u32() % 1000) as f64),
                3 => f64::NAN,
                4 => f64::INFINITY,
                5 => f64::NEG_INFINITY,
                6 => (r.next_u32() % 100_000) as f64 / 7.0 + 1e-9,
                _ => f64::from_bits(r.next_u32() as u64), // denormal
            };
            c.g[A0][0] = x.to_bits();
            c.pc = 0x0040_d610;
        });
    }

    /// WorldObjFactory_AnimObject: the static defaults initialised or not;
    /// -1..3 parameters with random slots/kinds (ints into float slots get
    /// converted; slot 0 is the object id); the id is -1 (the world's
    /// default object, sometimes none) or a section/object pair whose
    /// section or entry may be empty. Found objects have no node (an
    /// `AnimObject` is allocated from the pool allocator and constructed for
    /// real; node flag 0x100 keeps WorldObj_ctor away from the focus
    /// manager) or a node whose modifier check is a fake returning non-zero.
    /// Not reached: a node whose modifier check returns 0.
    #[test]
    fn world_obj_factory_anim_object_matches() {
        use crate::difftest::fakes::FAKE;
        use crate::difftest::GAME_GP;
        use crate::game::fe::feasyncfile::tests::alloc_base_3;
        const RESULT_SLOT: u32 = DATA + 0x3000;
        const PARAMS: u32 = DATA + 0x1000;
        const DIR: u32 = DATA + 0x20_0000;
        const SECTIONS: u32 = DIR + 0x100;
        const TABLE: u32 = DIR + 0x200;
        const SECTION: u32 = DIR + 0x300;
        const ENTRIES: u32 = DIR + 0x400;
        const WORLD: u32 = DIR + 0xa00;
        const NODE: u32 = DIR + 0xc00;
        const VT: u32 = DIR + 0xd00;
        const PROCS: u32 = DIR + 0x1000;
        const TAILS: u32 = DIR + 0x1800;
        const OBJS: u32 = DIR + 0x2000;
        check(
            ee::s10::f_002fbcb8__generated,
            world_obj_factory_anim_object,
            300,
            &|c, seed| {
                let mut r = alloc_base_3(c, seed, 0x2f_bcb8, 1);
                c.pc = 0x002f_bcb8;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // Static defaults.
                let ready = seed % 4 != 0;
                w(c, GAME_GP + WOFA_DEFAULTS_READY_GP, ready as u32);
                fill(c, &mut r, WOFA_DEFAULTS, 11);
                if ready && seed % 3 == 0 {
                    w(c, WOFA_DEFAULTS, u32::MAX);
                }
                for i in 0..11 {
                    w(c, WOFA_PARAM_KINDS + 4 * i, r.next_u32() % 4);
                }
                // Directory: sections 0..2 present, 3 empty; entries 0..2 objects, 3 empty.
                w(c, GAME_GP + WOFA_DIRECTORY_GP, DIR);
                w(c, DIR, SECTIONS);
                w(c, SECTIONS + 8, TABLE);
                for i in 0..4 {
                    w(c, TABLE + 4 * i, if i == 3 { 0 } else { SECTION });
                    let obj = OBJS + 0x100 * i;
                    w(
                        c,
                        ENTRIES + 4 * i,
                        if i == 3 {
                            r.next_u32() & 0xff
                        } else {
                            (obj >> 2) << 8 | (r.next_u32() & 0xff)
                        },
                    );
                    w(c, obj + 8, 0x100);
                    w(
                        c,
                        obj + WOFA_OBJECT_NODE,
                        if (seed / 4) % 3 == 0 { NODE } else { 0 },
                    );
                }
                w(c, SECTION + 0x1c, ENTRIES);
                w(c, GAME_GP + WOFA_WORLD_GP, WORLD);
                w(
                    c,
                    WORLD + WOFA_WORLD_DEFAULT_OBJECT,
                    if seed % 7 == 0 { 0 } else { OBJS + 0x100 },
                );
                // The node's modifier check: a fake (non-zero result).
                w(c, NODE + 0xc, VT);
                c.bus.write16(VT + 0x50, 0);
                w(c, VT + 0x54, FAKE);
                // Process list for the new object (list 1).
                w(c, GAME_GP + 0x2898 + 4, PROCS);
                w(c, PROCS + 0x44, r.next_u32());
                w(c, PROCS + 0x44 + 8, TAILS);
                w(c, PROCS + 0x44 + 0x28, TAILS + 0x40);
                // The result value.
                fill(c, &mut r, WOFA_RESULT, 3);
                // Parameters.
                let count: u32 = if seed % 11 == 0 { u32::MAX } else { seed % 4 };
                for i in 0..3 {
                    let p = PARAMS + 0x10 * i;
                    let slot = r.next_u32() % 11;
                    let value = if slot == 0 {
                        match r.next_u32() % 4 {
                            0 => u32::MAX,
                            _ => (r.next_u32() % 4) | (r.next_u32() % 4) << 8,
                        }
                    } else {
                        r.next_u32()
                    };
                    w(c, p, slot);
                    w(c, p + 4, value);
                    w(c, p + 8, r.next_u32());
                    w(c, p + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(RESULT_SLOT + 4 * (seed % 2));
                c.g[A1][0] = sx(count);
                c.g[A2][0] = sx(PARAMS);
            },
        );
    }

    /// `WScriptFn_PlayObjectSound`: defaults initialised or not (default id
    /// -1 or random), 0..3 parameters (or a negative count) of matching,
    /// mismatching and int-for-float kinds, object ids -1 (the world's
    /// default object, present or not), sections present or empty, entries
    /// with and without an object; when an object is found, the real
    /// `GetInstance` and `PlayObjectSound` (sound 0, or object sounds
    /// suppressed: its early exits).
    #[test]
    fn wscript_fn_play_object_sound_matches() {
        const RESULT_SLOT: u32 = DATA + 0x3000;
        const PARAMS: u32 = DATA + 0x1000;
        const AUDIO: u32 = DATA + 0x8000;
        const DIR: u32 = DATA + 0x20_0000;
        const SECTIONS: u32 = DIR + 0x100;
        const TABLE: u32 = DIR + 0x200;
        const SECTION: u32 = DIR + 0x300;
        const ENTRIES: u32 = DIR + 0x400;
        const WORLD: u32 = DIR + 0xa00;
        const OBJS: u32 = DIR + 0x2000;
        check(
            ee::s10::f_002fff00__generated,
            wscript_fn_play_object_sound,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_ff00, 0);
                c.pc = 0x002f_ff00;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // Static defaults.
                let ready = seed % 4 != 0;
                w(c, GAME_GP + OC300_POS_DEFAULTS_READY_GP, ready as u32);
                crate::difftest::fill(c, &mut r, OC300_POS_DEFAULTS, 3);
                let id = match seed % 5 {
                    0 => u32::MAX,
                    k => (r.next_u32() % 4) << 8 | (k - 1),
                };
                w(c, OC300_POS_DEFAULTS, id);
                // Sound 0 or a non-0x66 one (suppressed below).
                let sound = if seed % 3 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 0x60
                };
                w(c, OC300_POS_DEFAULTS + 4, sound);
                for i in 0..3 {
                    w(
                        c,
                        GAME_GP + OC300_POS_PARAM_KINDS_GP + 4 * i,
                        r.next_u32() % 4,
                    );
                }
                // Parameters: never slot 0 (the id) or slot 1 with a 0x66.
                let count = match seed % 7 {
                    6 => (r.next_u32() % 3).wrapping_neg(),
                    k => k % 4,
                };
                for i in 0..4u32 {
                    let p = PARAMS + 0x10 * i;
                    w(c, p, 1 + r.next_u32() % 2);
                    w(c, p + 4, 1 + r.next_u32() % 0x60);
                    w(c, p + 8, r.next_u32());
                    w(c, p + 0xc, r.next_u32() % 4);
                }
                // Directory: sections 0..2 present, 3 empty; entries 0..2
                // objects, 3 empty.
                w(c, GAME_GP + OC300_POS_DIRECTORY_GP, DIR);
                w(c, DIR, SECTIONS);
                w(c, SECTIONS + 8, TABLE);
                for i in 0..4 {
                    w(c, TABLE + 4 * i, if i == 3 { 0 } else { SECTION });
                    let obj = OBJS + 0x100 * i;
                    w(
                        c,
                        ENTRIES + 4 * i,
                        if i == 3 {
                            r.next_u32() & 0xff
                        } else {
                            (obj >> 2) << 8 | (r.next_u32() & 0xff)
                        },
                    );
                    crate::difftest::fill(c, &mut r, obj, 0x20);
                }
                w(c, SECTION + 0x1c, ENTRIES);
                w(c, GAME_GP + OC300_POS_WORLD_GP, WORLD);
                w(
                    c,
                    WORLD + OC300_POS_WORLD_DEFAULT_OBJECT,
                    if seed % 10 == 5 { 0 } else { OBJS + 0x100 },
                );
                // The audio system: object sounds suppressed (non-negative).
                w(c, GAME_GP + 0x410, AUDIO);
                w(c, AUDIO + 0x62a8, r.next_u32() & 0x7fff_ffff);
                crate::difftest::fill(c, &mut r, OC300_POS_RESULT, 3);
                c.g[A0][0] = sx(RESULT_SLOT);
                c.g[A1][0] = count as i32 as i64 as u64;
                c.g[A2][0] = sx(PARAMS);
            },
        );
    }

    /// `WScriptFn_MakeDeadNode`: the static defaults initialised or not;
    /// -1..3 parameters (the first always sets slot 1, the dead-node mode;
    /// others random slots/kinds, ints into float slots converted; slot 0 is
    /// the object id); the id is -1 (the world's default object, sometimes
    /// none) or a section/object pair whose section or entry may be empty.
    /// Found objects have no node (the real `WorldObjFactory_DeadNode` runs
    /// with a mode that sets the object's dead flags or returns at once) or
    /// a node whose modifier check is a fake returning non-zero (nothing
    /// done; sometimes unwinding). Not reached: a node whose modifier check
    /// fails (needs a zero result through the real AttachModifier chain),
    /// and the dead-node modes 0/3 that allocate.
    #[test]
    fn wscript_fn_make_dead_node_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{fill, DATA, GAME_GP};
        const RESULT_SLOT: u32 = DATA + 0x3000;
        const PARAMS: u32 = DATA + 0x1000;
        const DIR: u32 = DATA + 0x20_0000;
        const SECTIONS: u32 = DIR + 0x100;
        const TABLE: u32 = DIR + 0x200;
        const SECTION: u32 = DIR + 0x300;
        const ENTRIES: u32 = DIR + 0x400;
        const WORLD: u32 = DIR + 0xa00;
        const NODE: u32 = DIR + 0xc00;
        const VT: u32 = DIR + 0xd00;
        const OBJS: u32 = DIR + 0x2000;
        crate::difftest::check(
            ee::s10::f_002fc420__generated,
            wscript_fn_make_dead_node,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_c420, 1);
                c.pc = 0x002f_c420;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // Dead-node modes that return or only set flags.
                const MODES: [u32; 5] = [1, 2, 5, u32::MAX, 0x7fff];
                let count: u32 = if seed % 11 == 0 { u32::MAX } else { seed % 4 };
                let ready = seed % 4 != 0 || (count as i32) <= 0;
                w(c, GAME_GP + OC305_DEAD_NODE_DEFAULTS_READY_GP, ready as u32);
                fill(c, &mut r, OC305_DEAD_NODE_DEFAULTS, 3);
                if ready {
                    let id = if seed % 3 == 0 {
                        u32::MAX
                    } else {
                        (r.next_u32() % 4) | (r.next_u32() % 4) << 8
                    };
                    w(c, OC305_DEAD_NODE_DEFAULTS, id);
                    w(
                        c,
                        OC305_DEAD_NODE_DEFAULTS + 4,
                        MODES[(r.next_u32() % 5) as usize],
                    );
                }
                let kinds = GAME_GP + OC305_DEAD_NODE_KINDS_GP;
                w(c, kinds, 0);
                w(c, kinds + 4, r.next_u32() % 4);
                w(c, kinds + 8, r.next_u32() % 4);
                // Directory: sections 0..2 present, 3 empty; entries 0..2 objects, 3 empty.
                w(c, GAME_GP + OC305_DEAD_NODE_DIRECTORY_GP, DIR);
                w(c, DIR, SECTIONS);
                w(c, SECTIONS + 8, TABLE);
                for i in 0..4 {
                    w(c, TABLE + 4 * i, if i == 3 { 0 } else { SECTION });
                    let obj = OBJS + 0x100 * i;
                    let entry = if i == 3 {
                        r.next_u32() & 0xff
                    } else {
                        (obj >> 2) << 8 | (r.next_u32() & 0xff)
                    };
                    w(c, ENTRIES + 4 * i, entry);
                    fill(c, &mut r, obj, 0x10);
                    let node = if (seed / 4) % 3 == 0 { NODE } else { 0 };
                    w(c, obj + OC305_DEAD_NODE_OBJECT_NODE, node);
                }
                w(c, SECTION + 0x1c, ENTRIES);
                w(c, GAME_GP + OC305_DEAD_NODE_WORLD_GP, WORLD);
                w(
                    c,
                    WORLD + OC305_DEAD_NODE_WORLD_DEFAULT_OBJECT,
                    if seed % 7 == 0 { 0 } else { OBJS + 0x100 },
                );
                // The node's modifier check: a fake (non-zero result).
                w(c, NODE + 0xc, VT);
                c.bus.write16(VT + 0x50, 0);
                w(c, VT + 0x54, FAKE);
                // The result value.
                fill(c, &mut r, OC305_DEAD_NODE_RESULT, 3);
                // Parameters: {slot, value, -, kind}.
                for i in 0..3 {
                    let p = PARAMS + 0x10 * i;
                    let slot = if i == 0 { 1 } else { r.next_u32() % 3 };
                    let value = match slot {
                        0 => match r.next_u32() % 4 {
                            0 => u32::MAX,
                            _ => (r.next_u32() % 4) | (r.next_u32() % 4) << 8,
                        },
                        1 => MODES[(r.next_u32() % 5) as usize],
                        _ => r.next_u32(),
                    };
                    w(c, p, slot);
                    w(c, p + 4, value);
                    w(c, p + 8, r.next_u32());
                    w(c, p + 0xc, if slot == 0 { 0 } else { r.next_u32() % 4 });
                }
                c.g[A0][0] = sx(RESULT_SLOT + 4 * (seed % 2));
                c.g[A1][0] = sx(count);
                c.g[A2][0] = sx(PARAMS);
            },
        );
    }

    /// The stream header magic `StreamFile_Validate` checks.
    pub(crate) const SFILE_MAGIC: u32 = 0x4d52_5453;

    #[test]
    fn stream_file_get_channel_bytes_matches() {
        check(
            ee::s14::f_003e14c8__generated,
            stream_file_get_channel_bytes,
            300,
            &|c, seed| {
                let mut r = Rng(0x3e14_c800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003e_14c8;
                let (file, header) = (DATA + 0x100, DATA + 0x200);
                fill(c, &mut r, DATA, 0x400 / 4);
                fill(c, &mut r, crate::difftest::SP - 0x40, 0x10);
                c.bus.write32(file, header);
                // file = 0, bad magic, or valid.
                c.bus.write32(
                    header,
                    if seed % 3 == 0 {
                        r.next_u32()
                    } else {
                        SFILE_MAGIC
                    },
                );
                c.g[A0][0] = if seed % 5 == 0 { 0 } else { file as u64 };
            },
        );
    }

    #[test]
    fn stream_file_is_idle_matches() {
        check(
            ee::s14::f_003e1530__generated,
            stream_file_is_idle,
            300,
            &|c, seed| {
                let mut r = Rng(0x3e15_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003e_1530;
                let (file, header) = (DATA + 0x100, DATA + 0x200);
                fill(c, &mut r, DATA, 0x400 / 4);
                fill(c, &mut r, crate::difftest::SP - 0x40, 0x10);
                c.bus.write32(file, header);
                c.bus.write32(
                    header,
                    if seed % 7 == 0 {
                        r.next_u32()
                    } else {
                        SFILE_MAGIC
                    },
                );
                if seed % 3 != 0 {
                    c.bus.write32(header + SFILE_HEADER_READ_STATE, 0);
                }
                if seed % 2 == 0 {
                    c.bus.write32(file + SFILE_HANDLE_CHANNEL_BYTES, 0);
                }
                c.g[A0][0] = if seed % 11 == 0 { 0 } else { file as u64 };
            },
        );
    }

    /// `StreamFile_AllocRequest`: the real mutex lock/unlock (the fake
    /// GetThreadId returns 0x41: the owner, so the fast path, or another
    /// thread, so the semaphore path through fakes); an empty or non-empty
    /// free list; a serial that wraps to 0.
    #[test]
    fn stream_file_alloc_request_matches() {
        covered(
            ee::s14::f_003dfcd8__generated,
            stream_file_alloc_request,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003d_fcd8, 4);
                c.pc = 0x003d_fcd8;
                let sf = DATA + 0x400 + (r.next_u32() % 0x40) * 4;
                fill(c, &mut r, sf, 0x20);
                let mutex = sf + STREAM_FILE_MUTEX;
                c.bus.write32(mutex + 4, 0x40 + seed % 3);
                c.bus.write32(mutex + 8, 1 + seed % 2);
                c.bus.write32(mutex, r.next_u32() & 0xff);
                for i in 0..4 {
                    c.bus.write32(LOG_RESULTS + 4 * i, 0x41);
                }
                let req = DATA + 0x2000 + (r.next_u32() % 0x40) * 4;
                fill(c, &mut r, req, 8);
                let head = if seed % 4 == 0 { 0 } else { req };
                c.bus.write32(sf + STREAM_FILE_FREE_REQUESTS, head);
                let serial = if seed % 5 == 1 {
                    0xffff_ff00
                } else {
                    r.next_u32()
                };
                c.bus.write32(STREAM_FILE_REQUEST_SERIAL, serial);
                c.g[A0][0] = sx(sf);
            },
        );
    }

    /// StreamFile_FreeRequest: the request at the head, the tail, the middle
    /// or alone in the list, current or not, with and without a next
    /// neighbour (current falls back to the previous one).
    #[test]
    fn stream_file_free_request_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        const FILE: u32 = DATA;
        const REQS: u32 = DATA + 0x100;
        check(
            ee::s14::f_003dfe18__generated,
            stream_file_free_request,
            200,
            &|c, seed| {
                let mut r = Rng(0x003d_fe18 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003d_fe18;
                // A list of 1..4 requests (0x20 apart), doubly linked.
                let n = 1 + seed % 4;
                let at = |i: u32| REQS + 0x20 * i;
                for i in 0..n {
                    c.bus.write32(at(i) + 4, r.next_u32());
                    c.bus.write32(at(i) + 8, if i == 0 { 0 } else { at(i - 1) });
                    c.bus
                        .write32(at(i) + 0xc, if i + 1 == n { 0 } else { at(i + 1) });
                }
                c.bus.write32(FILE + 0x5c, at(0));
                c.bus.write32(FILE + 0x64, at(n - 1));
                let pick = (seed / 4) % n;
                let current = match r.next_u32() % 3 {
                    0 => at(pick),
                    1 => at(r.next_u32() % n),
                    _ => 0,
                };
                c.bus.write32(FILE + 0x60, current);
                c.bus
                    .write32(FILE + 0x68, if seed % 2 == 0 { 0 } else { DATA + 0x400 });
                c.g[A0][0] = sx(FILE);
                c.g[A1][0] = sx(at(pick));
            },
        );
    }

    /// ComputerActorRider_RollSkipTricks: the real game random number
    /// generator from a random state, trick chances from edge-case floats,
    /// fractions of 1 and exact multiples of 1/100 (ties with the roll).
    #[test]
    fn computer_actor_rider_roll_skip_tricks_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        const RAND_STATE: u32 = 0x004f_f030;
        check(
            ee::s00::f_0010c1d0__generated,
            computer_actor_rider_roll_skip_tricks,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_c1d0, 0);
                c.pc = 0x0010_c1d0;
                fill(c, &mut r, RAND_STATE, 0x10);
                let chance = match seed % 4 {
                    0 => r.edge_f32(),
                    1 => ((r.next_u32() % 101) as f32 / 100.0).to_bits(),
                    _ => (r.next_u32() as f32 / u32::MAX as f32).to_bits(),
                };
                c.bus.write32(DATA + ROLL_SKIP_TRICKS_CHANCE, chance);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    /// WScriptFn_ParticleControl: the static defaults initialised or not;
    /// 0..4 extra arguments over the handle and mode arguments (or none at
    /// all), int arguments for a float slot converted; the handle -1 (the
    /// world's default object, present or not), an empty handle set, an
    /// empty entry, or a found object; no model, a model whose "has
    /// particles" virtual says no (a real leaf returning 0) or yes (a fake);
    /// modes 0 / 1 / other with and without a modifier block (the real
    /// reset / prune see an empty particle list).
    #[test]
    fn wscript_fn_particle_control_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        check(
            ee::s10::f_00302490__generated,
            wscript_fn_particle_control,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_2490, 0);
                let gp = GAME_GP;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // Static defaults: fresh, or already set up (handle -1).
                w(c, gp + PARTICLE_CONTROL_INIT_GUARD, seed % 2);
                fill(c, &mut r, PARTICLE_CONTROL_DEFAULTS, 3);
                w(c, PARTICLE_CONTROL_DEFAULTS, u32::MAX);
                w(c, PARTICLE_CONTROL_DEFAULTS + 4, r.next_u32() % 3);
                let types = gp + PARTICLE_CONTROL_ARG_TYPES;
                w(c, types, 0);
                w(c, types + 4, 0);
                w(c, types + 8, r.next_u32() % 3);
                fill(c, &mut r, PARTICLE_CONTROL_RESULT_NONE, 3);
                // Handle tables: gp+0x16c8 -> P -> Q; Q+8 -> sets.
                let (p, q, sets) = (DATA + 0x2000, DATA + 0x2100, DATA + 0x2200);
                let (set, entries, obj) = (DATA + 0x2800, DATA + 0x2900, DATA + 0x3000);
                let (world, model, vt, mods) =
                    (DATA + 0x3800, DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
                w(c, gp + PARTICLE_CONTROL_HANDLE_TABLES, p);
                w(c, p, q);
                w(c, q + 8, sets);
                w(c, sets + 4 * 3, set);
                w(c, sets + 4 * 7, 0);
                w(c, set + 0x1c, entries);
                let found = (obj >> 2) << 8 | (r.next_u32() & 0xff);
                w(
                    c,
                    entries + 4 * 5,
                    if seed / 2 % 4 == 2 {
                        r.next_u32() & 0xff
                    } else {
                        found
                    },
                );
                w(c, gp + PARTICLE_CONTROL_WORLD, world);
                w(c, world + 0x290, if seed % 9 == 4 { 0 } else { obj });
                // The model and its "has particles" virtual (slot 0x80).
                w(c, obj + 0xc, if seed % 7 == 3 { 0 } else { model });
                w(c, model + 0xc, vt);
                c.bus
                    .write16(vt + 0x80, (r.next_u32() as i16 as i32 & !3) as u16);
                w(c, vt + 0x84, if seed % 5 == 1 { 0x0014_06c0 } else { FAKE });
                w(c, model + 0x1c, if seed % 6 == 5 { 0 } else { mods });
                w(c, mods + 0x10, 0); // no particles
                let id = match seed / 2 % 4 {
                    0 => u32::MAX,
                    1 => 5 << 8 | 7,
                    _ => 5 << 8 | 3,
                };
                let mode = match seed / 8 % 4 {
                    3 => r.next_u32() % 16,
                    m => m % 2,
                };
                // Arguments (16 bytes: slot, value, -, type).
                let args = DATA + 0x100;
                let extra = seed / 3 % 5;
                let mut n = 0;
                let mut put = |c: &mut Cpu, slot: u32, val: u32, ty: u32| {
                    let a = args + 0x10 * n;
                    c.bus.write32(a, slot);
                    c.bus.write32(a + 4, val);
                    c.bus.write32(a + 8, 0x5555);
                    c.bus.write32(a + 0xc, ty);
                    n += 1;
                };
                put(c, 0, id, 0);
                put(c, 1, mode, 0);
                for _ in 0..extra {
                    let val = r.next_u32();
                    let ty = r.next_u32() % 3;
                    put(c, 2, val, ty);
                }
                let argc = match seed % 11 {
                    0 => 0,
                    5 => sx(0xffff_fffe),
                    _ => n as u64,
                };
                c.g[A0][0] = sx(DATA + 0x10);
                c.g[A1][0] = argc;
                c.g[A2][0] = sx(args);
                c.pc = 0x0030_2490;
            },
        );
    }

    /// ComputerActorRider_RollUberTrick: uber tricks locked or unlocked,
    /// track 0xe or another, chances from 0 to 1 (and edge floats); real
    /// BELibrary_GetInstance/GetInterface/GetTrack/Rand_NextGame, the race
    /// interface's method a fake (sometimes unwinding).
    #[test]
    fn computer_actor_rider_roll_uber_trick_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::GAME_GP;
        covered(
            ee::s00::f_0010c258__generated,
            computer_actor_rider_roll_uber_trick,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10c258, 1);
                c.pc = 0x0010_c258;
                let (me, rider, lib, iface, vt) = (
                    DATA,
                    DATA + 0x2000,
                    DATA + 0x4000,
                    DATA + 0x5000,
                    DATA + 0x5100,
                );
                c.g[A0][0] = me as u64;
                c.bus.write32(me + CAR_RIDER, rider);
                let flags = r.next_u32() & !1 | (seed % 4 != 0) as u32;
                c.bus.write32(rider + CAR_RIDER_ABILITY_FLAGS, flags);
                let chance = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    (r.next_u32() as f32 / u32::MAX as f32).to_bits()
                };
                c.bus.write32(me + CAR_TRICK_SKILL, chance);
                // BELibrary built; interface 0's GetTrack-provider method is a fake.
                c.bus.write32(GAME_GP - 0x1f04, lib);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, iface);
                c.bus.write32(iface + 4, vt);
                c.bus.write16(vt + 8, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(vt + 0xc, FAKE);
                // BENewRaceInt's track.
                let track = if seed % 3 == 0 {
                    0xe
                } else {
                    r.next_u32() % 0x20
                };
                c.bus.write32(0x0053_5c08, track);
                // Rand_NextGame's generator state.
                fill(c, &mut r, 0x0050_0000 - 0xfd0, 8);
            },
        );
    }

    /// StreamFile_EnqueueRequest into an empty list or after a tail; the
    /// real EAMutex lock / unlock (syscalls faked, some unwinding), the
    /// mutex owned by the caller or not.
    #[test]
    fn stream_file_enqueue_request_matches() {
        use crate::difftest::fakes::{covered, machine};
        use crate::difftest::put_words;
        covered(
            ee::s14::f_003dfd58__generated,
            stream_file_enqueue_request,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003d_fd58, 3);
                c.pc = 0x003d_fd58;
                let (sf, req, tail) = (DATA, DATA + 0x1000, DATA + 0x2000);
                fill(c, &mut r, sf, 0x20);
                fill(c, &mut r, req, 4);
                fill(c, &mut r, tail, 4);
                c.bus
                    .write32(sf + SFQ_TAIL, if seed % 3 == 0 { 0 } else { tail });
                // The mutex: owner thread id (+4), lock count (+8), semaphore (+0xc).
                put_words(
                    c,
                    sf + SFQ_MUTEX + 4,
                    &[r.next_u32() % 4, r.next_u32() % 3, r.next_u32() % 16],
                );
                c.g[A0][0] = sx(sf);
                c.g[A1][0] = sx(req);
            },
        );
    }

    /// ComputerActorRider_RollFittedTrick: chances from 0 to 1 (and edge
    /// floats) against the real Rand_NextGame on random generator states.
    #[test]
    fn computer_actor_rider_roll_fitted_trick_matches() {
        crate::difftest::check_lohi(
            ee::s00::f_0010c320__generated,
            computer_actor_rider_roll_fitted_trick,
            300,
            &|c, seed| {
                let mut r = Rng(0x0010_c320 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0010_c320;
                let chance = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    (r.next_u32() as f32 / u32::MAX as f32).to_bits()
                };
                c.bus.write32(DATA + CAR_TRICK_SKILL, chance);
                fill(c, &mut r, 0x0050_0000 - 0xfd0, 8);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    /// Random rolls cover ordinary skills, out-of-range skills, and edge floats.
    #[test]
    fn computer_actor_rider_roll_random_trick_matches() {
        crate::difftest::check_lohi(
            ee::s00::f_0010c3b8__generated,
            computer_actor_rider_roll_random_trick,
            350,
            &|c, seed| {
                let mut r = Rng(0x0010_c3b8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0010_c3b8;
                let skill = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    [-1.0f32, 0.0, 0.25, 0.5, 0.75, 1.0, 2.0][(seed % 7) as usize].to_bits()
                };
                c.bus.write32(DATA + CAR_TRICK_SKILL, skill);
                fill(c, &mut r, 0x004f_f030, 8); // real Rand_NextGame state
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    /// `WScriptFn_StartObjectLoop`: defaults initialised or not (default id
    /// -1 or random), 0..3 parameters (or a negative count) of matching,
    /// mismatching and int-for-float kinds, object ids -1 (the world's
    /// default object, present or not), sections present or empty, entries
    /// with and without an object; when an object is found, the real
    /// `GetInstance` and `StartObjectLoop` (sound 0, or object sounds
    /// suppressed: its early exits).
    #[test]
    fn wscript_fn_start_object_loop_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::GAME_GP;
        const RESULT_SLOT: u32 = DATA + 0x3000;
        const PARAMS: u32 = DATA + 0x1000;
        const AUDIO: u32 = DATA + 0x8000;
        const DIR: u32 = DATA + 0x20_0000;
        const SECTIONS: u32 = DIR + 0x100;
        const TABLE: u32 = DIR + 0x200;
        const SECTION: u32 = DIR + 0x300;
        const ENTRIES: u32 = DIR + 0x400;
        const WORLD: u32 = DIR + 0xa00;
        const OBJS: u32 = DIR + 0x2000;
        check(
            ee::s10::f_003000a8__generated,
            wscript_fn_start_object_loop,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_00a8, 0);
                c.pc = 0x0030_00a8;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // Static defaults.
                let ready = seed % 4 != 0;
                w(c, GAME_GP + SOL_DEFAULTS_READY_GP, ready as u32);
                fill(c, &mut r, SOL_DEFAULTS, 4);
                let id = match seed % 5 {
                    0 => u32::MAX,
                    k => (r.next_u32() % 4) << 8 | (k - 1),
                };
                w(c, SOL_DEFAULTS, id);
                // Sound 0 or a non-0x66 one (suppressed below).
                let sound = if seed % 3 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 0x60
                };
                w(c, SOL_DEFAULTS + 4, sound);
                for i in 0..4 {
                    w(c, SOL_PARAM_KINDS + 4 * i, r.next_u32() % 4);
                }
                // Parameters: never slot 0 (the id) or a 0x66 sound.
                let count = match seed % 7 {
                    6 => (r.next_u32() % 3).wrapping_neg(),
                    k => k % 4,
                };
                for i in 0..4u32 {
                    let p = PARAMS + 0x10 * i;
                    w(c, p, 1 + r.next_u32() % 3);
                    w(c, p + 4, 1 + r.next_u32() % 0x60);
                    w(c, p + 8, r.next_u32());
                    w(c, p + 0xc, r.next_u32() % 4);
                }
                // Directory: sections 0..2 present, 3 empty; entries 0..2
                // objects, 3 empty.
                w(c, GAME_GP + SOL_DIRECTORY_GP, DIR);
                w(c, DIR, SECTIONS);
                w(c, SECTIONS + 8, TABLE);
                for i in 0..4 {
                    w(c, TABLE + 4 * i, if i == 3 { 0 } else { SECTION });
                    let obj = OBJS + 0x100 * i;
                    w(
                        c,
                        ENTRIES + 4 * i,
                        if i == 3 {
                            r.next_u32() & 0xff
                        } else {
                            (obj >> 2) << 8 | (r.next_u32() & 0xff)
                        },
                    );
                    fill(c, &mut r, obj, 0x20);
                }
                w(c, SECTION + 0x1c, ENTRIES);
                w(c, GAME_GP + SOL_WORLD_GP, WORLD);
                w(
                    c,
                    WORLD + SOL_WORLD_DEFAULT_OBJECT,
                    if seed % 10 == 5 { 0 } else { OBJS + 0x100 },
                );
                // The audio system: object sounds suppressed (non-negative).
                w(c, GAME_GP + 0x410, AUDIO);
                w(c, AUDIO + 0x62a8, r.next_u32() & 0x7fff_ffff);
                fill(c, &mut r, SOL_RESULT, 3);
                c.g[A0][0] = sx(RESULT_SLOT);
                c.g[A1][0] = count as i32 as i64 as u64;
                c.g[A2][0] = sx(PARAMS);
            },
        );
    }

    /// `isnan` on zeroes, normal doubles, floats bit patterns and the
    /// NaN/signalling-NaN encodings.
    #[test]
    fn isnan_matches() {
        use crate::difftest::{check, scramble, Rng};
        check(ee::s15::f_0040f840__generated, isnan, 200, &|c, seed| {
            let mut r = Rng(0x5eed_0bad ^ seed as u64);
            scramble(c, &mut r);
            let a0 = match seed % 6 {
                0 => 0,
                1 => 0x7ff0_0000_0000_0000, // +inf
                2 => 0xfff0_0000_0000_0001, // -inf, low word set
                3 => 0x7ff8_0000_0000_0000, // NaN
                4 => 0x7ff0_0000_7ff0_0000, // inf high, junk low
                _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
            };
            c.g[A0][0] = a0;
            c.pc = 0x0040_f840;
        });
    }
}

pub(crate) mod callee {

    pub(crate) use crate::game::main::ssxapp::callee::WORLD_PAINTER_GET_SUN_AZIMUTH;
    pub(crate) use crate::game::main::ssxapp::callee::WORLD_PAINTER_GET_SUN_ELEVATION;
    pub(crate) use crate::game::main::ssxapp::callee::WORLD_PAINTER_GET_SUN_TEXTURE;
}

pub(crate) mod table_callee {
    pub(crate) use crate::game::main::ssxapp::table_callee::MATH_SIN_COS;
}

pub(crate) mod lens {

    /// Screen rectangle of the sun sprite: x, y, w, h (floats).
    pub const RECT: u32 = 0x00;

    /// The clamped copy of the rectangle drawn (x, y, w, h).
    pub const CLAMPED: u32 = 0x10;

    /// Sprite depth / size copied from +0x1f8.
    pub const DEPTH: u32 = 0x24;

    /// Visible flags (two halfwords).
    pub const VISIBLE_A: u32 = 0x2c;

    pub const VISIBLE_B: u32 = 0x2e;

    /// The projected sun position (quad).
    pub const SCREEN: u32 = 0x1f0;

    /// Occlusion state cleared when the sun is off screen.
    pub const OCCLUSION: u32 = 0x204;
}

/// gp offset of the graphics object used for projection (`+0x10d8` its
/// camera interface vtable).
pub(crate) const LENS_GRAPHICS_GP: u32 = 0x2a90;

/// `.lit4` entries (gp-relative, negative): the painter-angle scale
/// (degrees to radians) and the default azimuth / elevation (radians).
pub(crate) const LIT_ANGLE_SCALE: u32 = 0x38c0;

pub(crate) const LIT_DEFAULT_AZIMUTH: u32 = 0x38bc;

pub(crate) const LIT_DEFAULT_ELEVATION: u32 = 0x38b8;

/// `void LensFX_UpdateSunPosition(LensFX* fx, Camera* cam, bool alt)`
/// (0x2f4db8): position the sun sprite. Painter view `alt ? 7 : 6`; a sun
/// texture of -1 hides the sun. Otherwise the sun direction comes from the
/// painter's azimuth / elevation (scaled to radians; the `.lit4` defaults
/// when both are 0) through `Math_SinCos`, is put `cam+0x58 - 500` units
/// from the camera position (camera row 3 transformed by the view rows),
/// and projected (camera interface slot 91 = off screen test; slot 92 =
/// project). On screen: both visible flags 1, the projected point at
/// +0x1f0, the 16x16 rect centred on it at +0, and its copy at +0x10 with
/// x / y clamped to `[0, viewport edge - 16]`; off screen: flags and the
/// occlusion state cleared.
pub fn lens_fx_update_sun_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S2, 0x80);
        frame.save(c, S1, 0x90);
        c.g[S2][0] = c.g[A0][0]; // fx
        frame.save(c, S3, 0x70);
        c.g[S1][0] = c.g[A1][0]; // camera
        frame.save(c, S4, 0x60);
        c.g[S3][0] = c.g[A2][0]; // alt view
        frame.save(c, S0, 0xa0);
        c.g[A0][0] = 6;
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 22, 0xc0);
        frame.save_fpr(c, 21, 0xb8);
        frame.save_fpr(c, 20, 0xb0);
        c.g[S4][0] = c.lw(r32(c, GP).wrapping_add(LENS_GRAPHICS_GP));
        callee::WORLD_PAINTER_GET_SUN_TEXTURE.call(c, 0x002f_4df8)?;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] == c.g[V1][0] {
            // No sun.
            c.cycles += 3;
            c.sh(at(c, S2, lens::VISIBLE_B), 0);
            c.sh(at(c, S2, lens::VISIBLE_A), 0);
            sun_epilogue(c, frame);
            return Ok(());
        }
        c.lwc1(1, at(c, S1, 0x58));
        // The painter angles.
        c.cycles += 18;
        c.g[V0][0] = 6;
        c.g[AT][0] = 0x43fa_0000; // 500.0
        c.mtc1(AT, 0);
        c.g[S0][0] = if c.g[S3][0] == 0 { c.g[V0][0] } else { 7 };
        c.lwc1(20, r32(c, GP).wrapping_sub(LIT_ANGLE_SCALE));
        c.sub_s(22, 1, 0); // distance = cam+0x58 - 500
        c.g[A0][0] = c.g[S0][0];
        callee::WORLD_PAINTER_GET_SUN_AZIMUTH.call(c, 0x002f_4e34)?;
        c.mul_s(21, 0, 20); // azimuth
        c.g[A0][0] = c.g[S0][0];
        callee::WORLD_PAINTER_GET_SUN_ELEVATION.call(c, 0x002f_4e40)?;
        c.mul_s(20, 0, 20); // elevation
        c.mtc1(0, 0);
        c.c_eq_s(21, 0);
        let azimuth_set = !c.fcc();
        c.mov_s(12, 21);
        let painter_angles = azimuth_set || {
            c.cycles += 4;
            c.c_eq_s(20, 0);
            !c.fcc()
        };
        if azimuth_set {
            c.cycles += 1;
        }
        c.g[A0][0] = sx(sp(c).wrapping_add(0x40)); // &sincos(azimuth)
        if painter_angles {
            c.cycles += 7;
            c.g[A1][0] = sx(sp(c).wrapping_add(0x44));
            call_ptr(c, table_callee::MATH_SIN_COS, 0x002f_4e98)?;
            c.mov_s(12, 20);
            c.g[A0][0] = sx(sp(c).wrapping_add(0x48));
            c.g[A1][0] = sx(sp(c).wrapping_add(0x4c));
            call_ptr(c, table_callee::MATH_SIN_COS, 0x002f_4ea8)?;
        } else {
            c.cycles += 9;
            c.lwc1(12, r32(c, GP).wrapping_sub(LIT_DEFAULT_AZIMUTH));
            c.g[A1][0] = sx(sp(c).wrapping_add(0x44));
            call_ptr(c, table_callee::MATH_SIN_COS, 0x002f_4e74)?;
            c.lwc1(12, r32(c, GP).wrapping_sub(LIT_DEFAULT_ELEVATION));
            c.g[A0][0] = sx(sp(c).wrapping_add(0x48));
            c.g[A1][0] = sx(sp(c).wrapping_add(0x4c));
            call_ptr(c, table_callee::MATH_SIN_COS, 0x002f_4e84)?;
        }
        c.lq(V0, at(c, S1, 0x40));
        sun_world_position(c);
        // Off screen?
        let vt = c.lw(at(c, S4, 0x10d8));
        c.g[V1][0] = vt;
        c.g[A0][0] = c.lh((vt as u32).wrapping_add(0x2d8));
        c.g[V0][0] = c.lw((vt as u32).wrapping_add(0x2dc));
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S4).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x002f_5024)?;
        let off_screen = c.g[V0][0] != 0;
        c.g[V1][0] = 1;
        if off_screen {
            c.cycles += 4;
            c.sw(at(c, S2, lens::OCCLUSION), 0);
            c.sh(at(c, S2, lens::VISIBLE_A), 0);
            c.sh(at(c, S2, lens::VISIBLE_B), 0);
            sun_epilogue(c, frame);
            return Ok(());
        }
        // Project the sun to the screen.
        c.cycles += 41;
        c.g[A0][0] = sx(sp(c).wrapping_add(0x20));
        c.sh(at(c, S2, lens::VISIBLE_B), c.g[V1][0]);
        c.g[A2][0] = c.g[S0][0];
        c.sh(at(c, S2, lens::VISIBLE_A), c.g[V1][0]);
        let vt = c.lw(at(c, S4, 0x10d8));
        c.g[V0][0] = vt;
        c.g[A1][0] = c.lh((vt as u32).wrapping_add(0x2e0));
        c.g[V1][0] = c.lw((vt as u32).wrapping_add(0x2e4));
        let method = r32(c, V1);
        c.g[A1][0] = sx(r32(c, S4).wrapping_add(r32(c, A1)));
        call_ptr(c, method, 0x002f_5060)?;
        sun_screen_rect(c);
        sun_epilogue(c, frame);
        Ok(())
    })
}

/// The sun's world position (v0 = camera row 3 at entry): the camera
/// position `-(row 3)` through the camera rows (FPU) at sp+0, the
/// direction `(r44 r4c, r40 r4c, r48, 0)` from the two `Math_SinCos`
/// result pairs at sp+0x40..0x4c, and `sp+0x10 = position + direction *
/// distance (f22)` (VU0 macro ops). Leaves s0 = a1 = sp+0x10.
pub(crate) fn sun_world_position(c: &mut Cpu) {
    c.cycles += 96;
    let sp = sp(c);
    let cam = r32(c, S1);
    let at_sp = |o: u32| sp.wrapping_add(o);
    let at_cam = |o: u32| cam.wrapping_add(o);
    c.g[S0][0] = sx(at_sp(0x10));
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
    c.mtc1(AT, 0);
    c.g[A1][0] = c.g[S0][0];
    // Negate the camera translation row: (-1) * row 3.
    c.sq(V0, at_sp(0x20));
    c.mfc1(V1, 0);
    c.lqc2(4, at_sp(0x20));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at_sp(0x30));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 14); // f14 = 1.0
    c.lq(V0, at_sp(0x30));
    // Transform it by the camera rotation rows (FPU).
    c.lwc1(6, at_cam(0x10));
    c.sq(V0, sp);
    c.lwc1(12, at_cam(0x14));
    c.lwc1(3, at_cam(0x20));
    c.lwc1(9, at_cam(0x24));
    c.lwc1(2, at_cam(0x30));
    c.lwc1(1, at_cam(0x34));
    c.lwc1(0, sp);
    c.lwc1(5, at_sp(0x4));
    c.lwc1(7, at_cam(0x40));
    c.mul_s(6, 0, 6);
    c.lwc1(8, at_cam(0x44));
    c.mul_s(12, 5, 12);
    c.mul_s(9, 5, 9);
    c.lwc1(10, at_cam(0x18));
    c.mul_s(1, 5, 1);
    c.lwc1(11, at_cam(0x28));
    c.mul_s(3, 0, 3);
    c.lwc1(4, at_sp(0x8));
    c.mul_s(2, 0, 2);
    c.lwc1(13, at_cam(0x38));
    c.mul_s(5, 5, 8);
    c.mul_s(0, 0, 7);
    c.lwc1(8, at_cam(0x48));
    c.add_s(3, 3, 9);
    c.lwc1(7, at_cam(0x2c));
    c.add_s(2, 2, 1);
    c.lwc1(9, at_cam(0x1c));
    c.add_s(0, 0, 5);
    c.lwc1(1, at_sp(0xc));
    c.mul_s(8, 4, 8);
    c.lwc1(5, at_cam(0x4c));
    c.mul_s(10, 4, 10);
    c.mul_s(11, 4, 11);
    c.add_s(6, 6, 12);
    c.mul_s(4, 4, 13);
    c.lwc1(12, at_cam(0x3c));
    c.mul_s(5, 1, 5);
    c.sq(V0, at_sp(0x10));
    c.mul_s(9, 1, 9);
    c.mul_s(7, 1, 7);
    c.mul_s(1, 1, 12);
    c.add_s(6, 6, 10);
    c.add_s(3, 3, 11);
    c.add_s(2, 2, 4);
    c.add_s(0, 0, 8);
    c.add_s(3, 3, 7);
    c.add_s(2, 2, 1);
    c.add_s(0, 0, 5);
    c.add_s(6, 6, 9);
    c.swc1(3, at_sp(0x14));
    c.swc1(2, at_sp(0x18));
    c.swc1(0, at_sp(0x1c));
    c.swc1(6, at_sp(0x10));
    c.lq(V0, at_sp(0x10));
    c.sq(V0, sp); // the camera position
                  // The direction from the Math_SinCos results.
    c.lwc1(3, at_sp(0x4c));
    c.lwc1(0, at_sp(0x44));
    c.lwc1(1, at_sp(0x40));
    c.mul_s(0, 0, 3);
    c.lwc1(2, at_sp(0x48));
    c.mul_s(1, 1, 3);
    c.swc1(14, at_sp(0xc));
    c.swc1(2, at_sp(0x18));
    c.swc1(0, at_sp(0x10));
    c.swc1(1, at_sp(0x14));
    c.sw(at_sp(0x1c), 0);
    // position = camera + direction * distance (VU0).
    c.mfc1(V0, 22);
    c.lqc2(4, at_sp(0x10));
    c.qmtc2(V0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at_sp(0x10));
    c.lqc2(3, at_sp(0x10));
    c.lqc2(4, sp);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at_sp(0x10));
}

/// After the projection (the screen point returned at sp+0x20): store it at
/// +0x1f0, centre a 16x16 rect on it at +0, copy the rect to +0x10 and
/// clamp its x to `[0, cam[0] + cam[2] - 16]` and its y to `[0, cam[0] +
/// cam[3] - 16]` (negative becomes 0; the viewport words at cam+0..0xc),
/// copy the depth from +0x1f8 to +0x24.
pub(crate) fn sun_screen_rect(c: &mut Cpu) {
    let fx = r32(c, S2);
    let cam = r32(c, S1);
    c.lq(V0, sp(c).wrapping_add(0x20));
    c.g[AT][0] = 0x4100_0000; // 8.0
    c.mtc1(AT, 2);
    c.sq(V0, fx.wrapping_add(lens::SCREEN));
    c.g[AT][0] = 0x4180_0000; // 16.0
    c.mtc1(AT, 4);
    c.lwc1(0, fx.wrapping_add(lens::SCREEN));
    c.lwc1(1, fx.wrapping_add(lens::SCREEN + 4));
    c.sub_s(0, 0, 2);
    c.swc1(4, fx.wrapping_add(lens::RECT + 8));
    c.sub_s(1, 1, 2);
    c.swc1(4, fx.wrapping_add(lens::RECT + 0xc));
    c.mtc1(0, 3); // f3 = 0
    c.swc1(0, fx.wrapping_add(lens::RECT));
    c.swc1(1, fx.wrapping_add(lens::RECT + 4));
    c.mov_s(5, 3);
    // Copy the rect (unaligned doubleword pairs).
    c.ldl(V0, fx.wrapping_add(lens::RECT + 7));
    c.ldr(V0, fx.wrapping_add(lens::RECT));
    c.ldl(V1, fx.wrapping_add(lens::RECT + 0xf));
    c.ldr(V1, fx.wrapping_add(lens::RECT + 8));
    c.sdl(V0, fx.wrapping_add(lens::CLAMPED + 7));
    c.sdr(V0, fx.wrapping_add(lens::CLAMPED));
    c.sdl(V1, fx.wrapping_add(lens::CLAMPED + 0xf));
    c.sdr(V1, fx.wrapping_add(lens::CLAMPED + 8));
    // Clamp x.
    c.lwc1(0, cam);
    c.lwc1(1, cam.wrapping_add(8));
    c.lwc1(2, fx.wrapping_add(lens::CLAMPED));
    c.add_s(0, 0, 1);
    c.c_le_s(3, 2);
    let inside = c.fcc();
    c.sub_s(0, 0, 4);
    if inside {
        c.cycles += 1;
        c.min_s(5, 2, 0);
    }
    // Clamp y.
    c.cycles += 9;
    c.swc1(5, fx.wrapping_add(lens::CLAMPED));
    c.lwc1(2, fx.wrapping_add(lens::CLAMPED + 4));
    c.mov_s(5, 3);
    c.lwc1(1, cam.wrapping_add(0xc));
    c.lwc1(0, cam);
    c.c_le_s(3, 2);
    c.add_s(0, 0, 1);
    let inside = c.fcc();
    c.sub_s(0, 0, 4);
    if inside {
        c.cycles += 1;
        c.min_s(5, 2, 0);
    }
    c.cycles += 3;
    c.lwc1(0, fx.wrapping_add(0x1f8));
    c.swc1(5, fx.wrapping_add(lens::CLAMPED + 4));
    c.swc1(0, fx.wrapping_add(lens::DEPTH));
}

/// `LensFX_UpdateSunPosition`'s shared epilogue.
pub(crate) fn sun_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore_ra(c, 0x50);
    frame.restore_fpr(c, 22, 0xc0);
    frame.restore_fpr(c, 21, 0xb8);
    frame.restore_fpr(c, 20, 0xb0);
    frame.close(c);
}
