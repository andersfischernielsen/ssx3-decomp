//! `src/visualfx/renderstateman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::sp;
use crate::ps2f::FpuF32;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::call;
use crate::tick;
pub(crate) use crate::game::visualfx::boardwakefx::GFX_MAN;
pub(crate) use crate::game::visualfx::boardwakefx::GS_TOP;
pub(crate) use crate::game::visualfx::boardwakefx::RECORD;
pub(crate) use crate::game::visualfx::boardwakefx::GS_DEFAULT;
pub(crate) use crate::game::visualfx::boardwakefx::GFX_VTABLE;
pub(crate) use crate::game::visualfx::boardwakefx::DRAW;
pub(crate) use crate::game::visualfx::boardwakefx::at;
pub(crate) use crate::game::visualfx::boardwakefx::gfx;
pub(crate) use crate::game::visualfx::boardwakefx::addiu;
pub(crate) use crate::game::visualfx::boardwakefx::copy_record;
pub(crate) use crate::game::visualfx::boardwakefx::gs_field;
pub(crate) use crate::game::visualfx::boardwakefx::gfx_method;
pub(crate) use crate::game::visualfx::boardwakefx::draw;
pub(crate) use crate::game::visualfx::boardwakefx::pop_record;
pub(crate) use crate::game::visualfx::boardwakefx::back_edge;
pub(crate) use crate::game::visualfx::boardwakefx::vmul_bc;
pub(crate) use crate::game::visualfx::boardwakefx::vadd;
pub(crate) use crate::game::visualfx::boardwakefx::copy_quad;
pub(crate) use crate::game::visualfx::boardwakefx::Streamer;
pub(crate) use crate::game::visualfx::boardwakefx::streamer_vertex;
pub(crate) use crate::game::visualfx::boardwakefx::streamer_segment;
pub(crate) use crate::game::visualfx::boardtrailfx::p;
pub(crate) use crate::game::visualfx::boardtrailfx::gp;
pub(crate) use crate::game::visualfx::boardtrailfx::shl32_2 as shl32;
pub(crate) use crate::game::visualfx::boardtrailfx::add32;
pub(crate) use crate::game::render::particle::at as at_2;
pub(crate) use crate::game::render::lightman::gp_2;
pub(crate) use crate::game::bx::memblockman::gp_2 as gp_3;
pub(crate) use crate::game::worldpainter::quadtree::create_with;
pub(crate) use crate::game::camera::camera::gp as gp_4;
pub(crate) use crate::game::render::particle::this_with_delta;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002e_ea50, world_painter_get_fog18, "WorldPainter_GetFog18"),
    (0x002e_ea98, world_painter_get_fog20, "WorldPainter_GetFog20"),
    (0x002e_eae0, world_painter_get_fog28, "WorldPainter_GetFog28"),
    (0x002e_eb28, world_painter_get_fog30, "WorldPainter_GetFog30"),
    (0x002e_eb70, world_painter_get_screen_tint_red, "WorldPainter_GetScreenTintRed"),
    (0x002e_ebd0, world_painter_get_screen_tint_green, "WorldPainter_GetScreenTintGreen"),
    (0x002e_ec30, world_painter_get_screen_tint_blue, "WorldPainter_GetScreenTintBlue"),
    (0x002e_ec90, world_painter_get_screen_fill_red, "WorldPainter_GetScreenFillRed"),
    (0x002e_ecf0, world_painter_get_screen_fill_green, "WorldPainter_GetScreenFillGreen"),
    (0x002e_ed50, world_painter_get_screen_fill_blue, "WorldPainter_GetScreenFillBlue"),
    (0x002e_efa8, world_painter_get_lighting30, "WorldPainter_GetLighting30"),
    (0x002e_eff0, world_painter_get_lighting28, "WorldPainter_GetLighting28"),
    (0x002e_f0e8, world_painter_get_irradiance_name0, "func_002EF0E8"),
    (0x002e_f140, world_painter_get_irradiance_name1, "func_002EF140"),
    (0x002e_f198, world_painter_get_irradiance_name2, "WorldPainter_GetIrradianceName2"),
    (0x002e_f368, world_painter_set_script_surface_mode, "WorldPainter_SetScriptSurfaceMode"),
    (0x002e_f378, world_painter_set_irradiance_override, "WorldPainter_SetIrradianceOverride"),
    (0x002e_f6a0, c_air_streamer_fx_reset, "cAirStreamerFX_Reset"),
    (0x002e_f6d0, c_air_streamer_fx_update, "cAirStreamerFX_Update"),
    (0x002e_f950, c_air_streamer_fx_render, "cAirStreamerFX_Render"),
    (0x002f_00a0, ps2_visual_effects_man_apply_world_painter, "PS2VisualEffectsMan_ApplyWorldPainter"),
    (0x002f_0368, ai_visual_effects_man_ctor, "AIVisualEffectsMan_ctor"),
    (0x002f_0390, ps2_ai_visual_effects_man_create, "PS2AIVisualEffectsMan_Create"),
    (0x002f_03c8, ps2_ai_visual_effects_man_ctor, "PS2AIVisualEffectsMan_ctor"),
    (0x002f_0438, ps2ai_visual_effects_man_dtor, "PS2AIVisualEffectsMan_dtor"),
    (0x002f_0510, ps2_visual_effects_man_create, "PS2VisualEffectsMan_Create"),
];

/// `cAirStreamerFX` fields.
pub(crate) mod air {

    /// The rider (`+0x780` model, `+0x89c`/`+0x8a4` bone indices, `+0x2ec`
    /// speed).
    pub const RIDER: u32 = 0;

    /// Point pairs (32 bytes per index).
    pub const POINTS: u32 = 0x4;

    /// Index of the newest point.
    pub const FIRST: u32 = 0x8;

    /// Number of segments.
    pub const COUNT: u32 = 0xc;

    /// Alpha scale (textured mode).
    pub const ALPHA: u32 = 0x10;

    /// Length used for the v step.
    pub const LENGTH: u32 = 0x14;
}

// ---------------------------------------------------------------------------
// Air streamer wake ribbon.

/// `cAirStreamerFX` (a wake ribbon attached to a rider) fields.
pub(crate) mod air_streamer {

    /// The owner entity.
    pub const OWNER: u32 = 0;

    /// The ring of ribbon points (25 slots of two vec4s).
    pub const RING: u32 = 0x4;

    /// Ring slot index, 0..25.
    pub const HEAD: u32 = 0x8;

    /// Live slot count, approached towards 0 or 1.
    pub const COUNT: u32 = 0xc;

    /// How many slots the ribbon may use.
    pub const LEN: u32 = 0x14;

    /// The ribbon's texture start, a float wrapped to [0, 1].
    pub const START: u32 = 0x10;
}

/// `sRSMObj`: the object an `sRSMEntry` points at, and its vtable.
pub(crate) mod rsm_obj {

    /// `vt`.
    pub const VTABLE: u32 = 0x4;

    /// Entry 41's `{ short delta; short; void (*fn)(); }` pair.
    pub const SLOT_41: u32 = 0x148;
}

// ---------------------------------------------------------------------------
// PS2VisualEffectsMan_ApplyWorldPainter (0x2f00a0).

/// `gp` globals the world-painter copy reads (`D_0050xxxx` and friends).
pub(crate) mod painter {

    /// Painter descriptor array base; accessors index it with a0.
    pub const DESCS: u32 = 0x004f_a370;

    /// Graphics manager (`*(gp + 0x2a90)`).
    pub const GFX: i32 = 0x2a90;

    /// "Applied" flag pair.
    pub const READY: i32 = 0x1238;

    pub const HAVE_TINT: i32 = 0x14ec;
}

/// Callees and fixed addresses.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::PS2_SOUND_MANAGER_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_SET_FILE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_STOP;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SPEECH_INSTANCE_STOP_STREAM;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_GET_STREAM_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_POST_EVENT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_FADE_OUT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SNDSTRM_GET_REQUEST_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_SET_STATE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::CONSOLE_CONFIG_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STRCPY;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MENU_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STATS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::AI_VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_GET_SECTION_SCRIPT_FUNC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MOD_OBJ_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::INTC_REMOVE_HANDLER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EMITTER_RENDER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_ON_VIF1_DMA_END;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_MAN_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_STOP;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_STOPPED;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_BINDING_CLEAR_LISTENER;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PUSH_PAGE;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_FORMAT_FLOAT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_DRAW_ROW;
    pub(crate) use crate::game::fe::festaterewards::callee::IN_SEQ_INSTANCE_SETUP;
    pub(crate) use crate::game::fe::festaterewards::callee::LIGHT_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_LIST_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::CUI_TEXT_SCROLL_SET_NAV_BUTTONS;
    pub(crate) use crate::game::fe::festaterewards::callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::AI_VISUAL_EFFECTS_MAN_CTOR;
}


/// `void cAirStreamerFX_Reset(cAirStreamerFX *this)` (0x2ef6a0): reset the
/// streamer and set its id from the owning rider's id.
pub fn c_air_streamer_fx_reset(c: &mut Cpu) {
    c.cycles += 11;
    let this = abi::arg(c, 0);
    c.g[V1][0] = Addr(this).lw(c);
    c.g[A1][0] = 0x12;
    c.g[A2][0] = 0x19;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x870)).lw(c);
    Addr(this).off(0x10).sw(c, 0);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 0i64) as u64;
    Addr(this).off(0x8).sw(c, 0);
    if c.g[V0][0] == 0 {
        c.g[A1][0] = c.g[A2][0];
    }
    Addr(this).off(0xc).sw(c, 0);
    Addr(this).off(0x14).sw(c, c.g[A1][0]);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Visual effects.

/// `void AIVisualEffectsMan_ctor(AIVisualEffectsMan *this)` (0x2f0368):
/// install the vtable and publish the singleton at `gp+0xa90`.
pub fn ai_visual_effects_man_ctor(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x48_0000;
    let this = abi::arg(c, 0);
    Addr(this).sw(c, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7d78));
    Addr(this).off(0x4).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    Addr((c.g[GP][0] as u32).wrapping_add(0xa90)).sw(c, c.g[A0][0]);
    abi::ret(c);
}


pub(crate) const AIR: Streamer = Streamer { this: T6, points: air::POINTS, count: air::COUNT, tmp: 0x1380, half: 0x9c0 };


/// `void cAirStreamerFX_Render(cAirStreamerFX *fx)` (0x2ef950): with at
/// least two segments, push a GS state (textured with `gfx+0x1044` when
/// the global switch `gp+0x1630` is set or the rider is moving, else flat
/// with `gfx+0x104c`), build the streamer's two triangle strips (each
/// point pair offset along the rider's bone, faded by `1 - (i/len)²`) and
/// draw them; pop the state.
///
/// Frame 0x1430: s0 +0x1420, s1 +0x1410, ra +0x1400.
pub fn c_air_streamer_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x1430);
        c.g[T6][0] = c.g[A0][0];
        frame.save(c, S0, 0x1420);
        frame.save(c, S1, 0x1410);
        frame.save_ra(c, 0x1400);
        let n = c.lw(at(c.g[T6][0], air::COUNT));
        c.g[V0][0] = ((n as i64) < 2) as u64;
        let skip = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x1420);
        if !skip {
            air_draw(c, frame)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x1410);
        frame.restore_ra(c, 0x1400);
        frame.close(c);
        Ok(())
    })
}


pub(crate) fn air_draw(c: &mut Cpu, frame: Frame) -> Flow {
    let sp = r32(c, SP);
    let gp = r32(c, GP);
    c.cycles += 34;
    // Push the GS state.
    let g = gfx(c);
    c.g[A0][0] = g;
    c.g[A2][0] = 0x50_0000;
    c.lwc1(10, gp.wrapping_sub(0x3938));
    let top = c.lw(at(g, GS_TOP));
    c.g[V1][0] = top;
    c.mtc1(ZERO, 1);
    copy_record(c, top as u32, at(top, RECORD), V0, A1, A3);
    let top = c.lw(at(g, GS_TOP));
    c.g[A1][0] = gfx(c);
    c.g[V0][0] = sx(at(top, RECORD));
    let new_top = c.g[V0][0];
    c.sw(at(g, GS_TOP), new_top);
    let top = c.lw(at(c.g[A1][0], GS_TOP));
    c.g[V1][0] = top;
    c.g[A3][0] = sx(GS_DEFAULT);
    copy_record(c, GS_DEFAULT, top as u32, V0, A0, A1);
    c.g[V0][0] = c.lw(gp.wrapping_add(0x1630));
    c.g[T9][0] = 0;
    let mut textured = c.g[V0][0] != 0;
    if !textured {
        c.cycles += 6;
        let rider = c.lw(at(c.g[T6][0], air::RIDER));
        c.g[V0][0] = rider;
        c.lwc1(0, at(rider, 0x2ec));
        c.c_lt_s(1, 0);
        textured = c.fcc();
        c.g[V1][0] = gfx(c);
    }
    if textured {
        c.cycles += 15;
        let g = gfx(c);
        c.g[V1][0] = g;
        c.g[A1][0] = 0xffff_ffff_ffff_fff3;
        c.g[T9][0] = 1;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 10);
        let top = c.lw(at(g, GS_TOP));
        c.g[A0][0] = top;
        let tex = c.lw(at(g, 0x1044));
        c.g[V0][0] = tex;
        c.sh(at(top, 0x10), tex);
        let g = gfx(c);
        c.g[V1][0] = g;
        gs_field(c, g, A0, 0, 0xffff_ffff_ffff_fff3, 0);
        c.lwc1(1, at(c.g[T6][0], air::ALPHA));
    } else {
        c.cycles += 7;
        let g = c.g[V1][0];
        gs_field(c, g, A0, 0, u64::MAX, 0xc);
        let top = c.lw(at(g, GS_TOP));
        c.g[A1][0] = top;
        let tex = c.lw(at(g, 0x104c));
        c.g[V0][0] = tex;
        c.sh(at(top, 0x10), tex);
    }
    c.cycles += 57;
    let g = gfx(c);
    c.g[A1][0] = g;
    c.g[V1][0] = 0xffff_ffff_ffbf_ffff;
    c.g[T2][0] = 0x40_0000;
    gs_field(c, g, A0, 4, 0xffff_ffff_ffbf_ffff, 0x40_0000);
    c.g[A2][0] = 0xffff_ffff_fe7f_ffff;
    c.g[A3][0] = 0xffff_ffff_ffcf_ffff;
    c.g[T0][0] = 0xffff_ffff_fff0_0fff;
    c.g[T1][0] = 0x1_4000;
    c.g[T3][0] = 0xffff_ffff_ffff_fffc;
    c.g[T2][0] = 0xffff_ffff_e000_03ff;
    c.g[T4][0] = 0xffff_ffff_ffff_fc1f;
    c.g[T5][0] = 0xffff_ffff_ffff_ff83;
    c.g[T7][0] = 0x33;
    c.g[S1][0] = sx(sp.wrapping_add(AIR.half));
    c.g[T8][0] = u64::MAX;
    gs_field(c, g, A0, 4, 0xffff_ffff_fe7f_ffff, 0);
    gs_field(c, g, V1, 4, 0xffff_ffff_ffcf_ffff, 0);
    gs_field(c, g, A0, 4, 0xffff_ffff_fff0_0fff, 0x1_4000);
    gs_field(c, g, V1, 4, 0xffff_ffff_ffff_fffc, 0);
    gs_field(c, g, A0, 8, 0xffff_ffff_e000_03ff, 0);
    gs_field(c, g, V1, 8, 0xffff_ffff_ffff_fc1f, 0x100);
    gs_field(c, g, A0, 4, 0xffff_ffff_ffff_ff83, 0x1c);
    // gcc's empty constructor loops over the 2 x 52 vertices.
    loop {
        c.cycles += 7;
        addiu(c, T7, T7, u32::MAX);
        let more = c.g[T7][0] != c.g[T8][0];
        c.g[V0][0] = 0x33;
        if !more {
            break;
        }
        back_edge(c, 0x002e_fb50);
    }
    c.cycles += 3;
    c.g[A2][0] = c.lw(at(c.g[T6][0], air::RIDER));
    c.g[A0][0] = c.lw(at(c.g[T6][0], air::LENGTH));
    c.g[V1][0] = u64::MAX;
    loop {
        c.cycles += 7;
        addiu(c, V0, V0, u32::MAX);
        let more = c.g[V0][0] != c.g[V1][0];
        c.mov_s(4, 1);
        if !more {
            break;
        }
        back_edge(c, 0x002e_fb78);
    }
    // Colour 1.0, the offset along the rider's bone, 1/(len-1)² and
    // 1/count.
    c.cycles += 42;
    let rider = c.g[A2][0];
    c.g[A1][0] = c.lw(at(rider, 0x780));
    addiu(c, A0, A0, u32::MAX);
    c.g[V0][0] = c.lw(at(rider, 0x89c));
    c.mult(0, A0, A0, A0);
    c.g[V1][0] = c.lw(at(c.g[A1][0], 0x30));
    c.g[V0][0] = sx(r32(c, V0) << 6);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.lwc1(1, gp.wrapping_sub(0x3934));
    let bone = r32(c, V1);
    c.lq(V0, bone.wrapping_add(0x30));
    c.mtc1(A0, 0);
    c.cvt_s_w(0, 0);
    c.mtc1(ZERO, 7);
    c.g[S0][0] = 0;
    let t = sp.wrapping_add(AIR.tmp);
    c.sq(V0, t.wrapping_add(0x10));
    vmul_bc(c, V1, 1, t.wrapping_add(0x10), t.wrapping_add(0x20));
    c.div_s(9, 3, 0);
    c.lwc1(0, at(c.g[T6][0], air::COUNT));
    c.cvt_s_w(0, 0);
    c.lq(V0, t.wrapping_add(0x20));
    for k in 0..3 {
        c.swc1(3, t.wrapping_add(0x10 + 4 * k));
    }
    c.div_s(8, 3, 0);
    c.swc1(3, t.wrapping_add(0x1c));
    c.sq(V0, t);
    let rider = c.lw(at(c.g[T6][0], air::RIDER));
    c.g[V0][0] = rider;
    c.g[AT][0] = 0x4170_0000;
    c.mtc1(AT, 1);
    c.g[A1][0] = c.lw(at(rider, 0x780));
    c.g[V1][0] = c.lw(at(rider, 0x8a4));
    c.g[A0][0] = c.lw(at(c.g[A1][0], 0x34));
    c.g[V1][0] = sx(r32(c, V1) << 6);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    let axis = r32(c, A0).wrapping_add(0x20);
    c.lq(V0, axis);
    let flat = c.g[T9][0] == 0;
    c.sq(V0, t.wrapping_add(0x40));
    if !flat {
        // Textured: colour (1, k, k, k) with k from gp+0xa84, width 37.5.
        c.cycles += 15;
        c.lwc1(0, gp.wrapping_add(0xa84));
        c.swc1(3, t.wrapping_add(0x50));
        c.swc1(0, t.wrapping_add(0x5c));
        c.swc1(0, t.wrapping_add(0x54));
        c.swc1(0, t.wrapping_add(0x58));
        c.g[AT][0] = 0x4216_0000;
        c.mtc1(AT, 1);
        c.ldl(T0, t.wrapping_add(0x57));
        c.ldr(T0, t.wrapping_add(0x50));
        c.ldl(V0, t.wrapping_add(0x5f));
        c.ldr(V0, t.wrapping_add(0x58));
        c.sdl(T0, t.wrapping_add(0x17));
        c.sdr(T0, t.wrapping_add(0x10));
        c.sdl(V0, t.wrapping_add(0x1f));
        c.sdr(V0, t.wrapping_add(0x18));
    }
    // Half width: the bone axis * width/2.
    c.cycles += 14;
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 0);
    c.g[A3][0] = 0;
    c.mul_s(0, 1, 0);
    vmul_bc(c, V1, 0, t.wrapping_add(0x40), t.wrapping_add(0x60));
    c.lq(V0, t.wrapping_add(0x60));
    c.g[V1][0] = c.lw(at(c.g[T6][0], air::COUNT));
    c.sq(V0, t.wrapping_add(0x50));
    let none = (c.g[V1][0] as i64) <= 0;
    c.g[A2][0] = c.lw(at(c.g[T6][0], air::FIRST));
    if !none {
        c.cycles += 6;
        c.g[AT][0] = 0x437f_0000;
        c.mtc1(AT, 2);
        c.g[T0][0] = 0x18;
        c.g[AT][0] = 0x4300_0000;
        c.mtc1(AT, 6);
        c.g[A1][0] = sx(sp.wrapping_add(AIR.half));
        loop {
            // alpha = scale * (1 - f7² / (len-1)²), clamped at 0.
            c.cycles += 12;
            c.mul_s(0, 7, 7);
            c.mtc1(ZERO, 5);
            c.g[V0][0] = (-1i64 < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.g[T0][0];
            }
            c.mul_s(0, 0, 9);
            c.sub_s(0, 3, 0);
            c.mul_s(0, 10, 0);
            c.c_lt_s(0, 5);
            let clamp = c.fcc();
            c.swc1(0, t.wrapping_add(0x10));
            if clamp {
                c.cycles += 1;
                c.swc1(5, t.wrapping_add(0x10));
            }
            if !streamer_segment(c, &AIR) {
                break;
            }
            back_edge(c, 0x002e_fcc8);
        }
    }
    c.cycles += 23;
    c.g[T0][0] = gfx(c);
    c.g[A2][0] = c.g[SP][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A3][0] = 0;
    draw(c, T0, V0, V1, 0x002e_ff44)?;
    c.g[T0][0] = gfx(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = 0;
    draw(c, T0, V0, V1, 0x002e_ff68)?;
    pop_record(c, A0);
    frame.restore(c, S0, 0x1420);
    Ok(())
}


// ---------------------------------------------------------------------------
// World-painter irradiance names.

/// `const char* func_002EF0E8(void)` (0x2ef0e8): the name of irradiance slot
/// 0 — the built-in global name once the world painter exists, else the
/// first object's render-state name.
pub fn world_painter_get_irradiance_name0(c: &mut Cpu) {
    world_painter_irradiance_name(c, 0x002e_f128, 0x128, 0x12c);
}


/// `const char* func_002EF140(void)` (0x2ef140): the name of irradiance slot
/// 1 (see [`world_painter_get_irradiance_name0`]).
pub fn world_painter_get_irradiance_name1(c: &mut Cpu) {
    world_painter_irradiance_name(c, 0x002e_f180, 0x130, 0x134);
}


/// The shared body of the two name getters: `gp+0xa74` set means "return the
/// global name", else `vt[slot]` of the object in `D_004FA370`.
pub(crate) fn world_painter_irradiance_name(c: &mut Cpu, ret: u32, delta_off: u32, fn_off: u32) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp(c, 0xa74)).lw(c);
        frame.save_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = sx(gp(c, 0x24c0));
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 14;
            c.g[V1][0] = 0xf0;
            c.g[V0][0] = 0x50_0000;
            c.mult(0, 3, 4, 3);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5c90));
            c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
            c.g[A1][0] = Addr(r32(c, A0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A1), 4)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), delta_off)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), fn_off)).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = add32(c.g[A1][0], c.g[A0][0]);
            call_ptr(c, target, ret)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `void cAirStreamerFX_Update(cAirStreamerFX* self)` (0x2ef6d0): append the
/// rider's wake points to the ribbon (once per other AI frame), keep the live
/// slot count near two, and advance the texture start by the wake length,
/// wrapped to [0, 1]. VU0 does the vector scaling and subtraction, as the
/// original does.
pub fn c_air_streamer_fx_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x90 };
        air_streamer_at(c, frame, c.pc)
    })
}


pub(crate) fn air_streamer_at(c: &mut Cpu, frame: Frame, pc: u32) -> Flow {
    match pc {
        0x002e_f6d0 => {
            c.cycles += 9;
            Frame::open(c, 0x90);
            frame.save(c, S0, 0x80);
            frame.save_ra(c, 0x70);
            let fx = abi::arg(c, 0);
            c.g[S0][0] = sx(fx);
            c.g[A0][0] = Addr(fx).lw(c);
            call_ptr(c, 0x0011_fe98, 0x002e_f6e8)?;
            c.g[V1][0] = 1;
            if c.g[V0][0] != c.g[V1][0] {
                c.g[A0][0] = Addr(p(fx, air_streamer::COUNT)).lw(c);
                air_approach_count(c, frame)
            } else {
                air_build(c, frame)
            }
        }
        0x002e_f6f4 => air_build(c, frame),
        0x002e_f7bc => air_build_second(c, frame),
        0x002e_f82c => air_step(c, frame),
        0x002e_f848 => air_limit(c, frame),
        0x002e_f860 => air_tail_bump(c, frame),
        0x002e_f868 => air_copy(c, frame),
        0x002e_f894 => air_approach_count(c, frame),
        0x002e_f8a4 => air_approach_count_neg(c, frame),
        0x002e_f8b4 => air_store_count(c, frame),
        0x002e_f8b8 => air_load_owner(c, frame),
        0x002e_f8bc => air_length(c, frame),
        0x002e_f908 => air_start_increase(c, frame),
        0x002e_f918 => air_start_clamp(c, frame),
        0x002e_f934 => air_start_subtract(c, frame),
        0x002e_f940 => {
            air_exit(c, frame);
            Ok(())
        }
        other => {
            c.jump_out(other, 0x002e_f6d0, 0x002e_f94c);
            Ok(())
        }
    }
}


/// 0x2ef6f4: fetch the three bone points of the ribbon (the rider's two wake
/// bones and their up axis) to the stack, scaled by 90 degrees.
pub(crate) fn air_build(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 50;
    c.g[A2][0] = Addr(r32(c, S0)).lw(c);
    c.lwc1(1, gp(c, -0x3940));
    c.g[V0][0] = Addr(p(r32(c, A2), 0x780)).lw(c);
    c.g[V1][0] = Addr(p(r32(c, A2), 0x89c)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, V0), 0x30)).lw(c);
    c.g[V1][0] = shl32(c.g[V1][0], 6);
    c.g[AT][0] = 0x42b4_0000;
    c.mtc1(1, 0);
    c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
    c.lq(2, p(r32(c, A0), 0x30));
    c.sq(2, sp(c));
    c.g[A1][0] = Addr(p(r32(c, A2), 0x780)).lw(c);
    c.g[V1][0] = Addr(p(r32(c, A2), 0x8a4)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, A1), 0x30)).lw(c);
    c.g[V1][0] = shl32(c.g[V1][0], 6);
    c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
    c.lq(2, p(r32(c, A0), 0x30));
    c.sq(2, p(sp(c), 0x10));
    c.g[A1][0] = Addr(p(r32(c, A2), 0x780)).lw(c);
    c.g[V1][0] = Addr(p(r32(c, A2), 0x8a4)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, A1), 0x34)).lw(c);
    c.g[V1][0] = shl32(c.g[V1][0], 6);
    c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
    c.lq(2, r32(c, A0));
    c.sq(2, p(sp(c), 0x20));
    c.mfc1(3, 1);
    c.lqc2(4, sp(c));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x50));
    c.lq(2, p(sp(c), 0x50));
    c.sq(2, p(sp(c), 0x40));
    c.lqc2(3, p(sp(c), 0x10));
    c.lqc2(4, p(sp(c), 0x40));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp(c), 0x50));
    c.lq(2, p(sp(c), 0x50));
    c.g[A0][0] = Addr(p(r32(c, S0), air_streamer::HEAD)).lw(c);
    c.sq(2, p(sp(c), 0x30));
    c.g[A0][0] = shl32(c.g[A0][0], 5);
    c.g[V1][0] = Addr(p(r32(c, S0), air_streamer::RING)).lw(c);
    c.mfc1(5, 0);
    c.lqc2(4, p(sp(c), 0x20));
    c.qmtc2(5, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x60));
    c.lq(2, p(sp(c), 0x60));
    c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
    c.sq(2, p(sp(c), 0x50));
    c.lqc2(3, p(sp(c), 0x30));
    air_build_second(c, frame)
}


/// 0x2ef7bc: write the new point and the difference vector into the ribbon,
/// then let the AI frame counter decide whether to append.
pub(crate) fn air_build_second(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 28;
    c.lqc2(4, p(sp(c), 0x50));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp(c), 0x60));
    c.lq(2, p(sp(c), 0x60));
    c.sq(2, p(sp(c), 0x40));
    c.sq(2, r32(c, A0));
    c.g[V1][0] = Addr(p(r32(c, S0), air_streamer::HEAD)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, S0), air_streamer::RING)).lw(c);
    c.mfc1(2, 0);
    c.lqc2(4, p(sp(c), 0x20));
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x60));
    c.g[V1][0] = shl32(c.g[V1][0], 5);
    c.lq(2, p(sp(c), 0x60));
    c.g[V1][0] = add32(c.g[V1][0], c.g[A0][0]);
    c.sq(2, p(sp(c), 0x50));
    c.lqc2(3, p(sp(c), 0x30));
    c.lqc2(4, p(sp(c), 0x50));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp(c), 0x60));
    c.lq(2, p(sp(c), 0x60));
    c.sq(2, p(sp(c), 0x40));
    c.sq(2, p(r32(c, V1), 0x10));
    call_ptr(c, 0x0012_98c8, 0x002e_f820)?;
    let ai_frame = r32(c, V0);
    c.g[V0][0] &= 1;
    let skip = if tick::rate(c) == 60 { c.g[V0][0] != 0 } else { !air_ring_due(c, ai_frame) };
    if skip {
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        air_length(c, frame)
    } else {
        air_step(c, frame)
    }
}


/// Above 60 Hz: whether the air streamer records a point this tick. The
/// original records on even AI frames (ticks), every other 60 Hz frame: 30
/// per second. At rate `r` the AI frame `n` is 60 Hz frame `n * 60 / r`;
/// record on the tick that frame starts, when it is even, so the 25 points
/// cover the same 0.83 s at any rate.
pub(crate) fn air_ring_due(c: &mut Cpu, ai_frame: u32) -> bool {
    let r = tick::rate(c) as u64;
    let frame60 = |n: u32| n as u64 * 60 / r;
    let now = frame60(ai_frame);
    now != frame60(ai_frame.wrapping_sub(1)) && now % 2 == 0
}


/// 0x2ef82c: advance the tail index.
pub(crate) fn air_step(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    c.g[A1][0] = Addr(p(r32(c, S0), air_streamer::HEAD)).lw(c);
    c.g[V1][0] = 0x19;
    c.g[V0][0] = sx(r32(c, A1).wrapping_add(1));
    c.div(0, 2, 3);
    c.sw(p(r32(c, S0), air_streamer::HEAD), c.g[V0][0]);
    air_limit(c, frame)
}


/// 0x2ef848: wrap the head index at the limit.
pub(crate) fn air_limit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[V1][0] = Addr(p(r32(c, S0), air_streamer::COUNT)).lw(c);
    c.g[V0][0] = Addr(p(r32(c, S0), air_streamer::LEN)).lw(c);
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[A0][0] = c.hi[0];
    let room = c.g[V0][0] != 0;
    c.sw(p(r32(c, S0), air_streamer::HEAD), c.g[A0][0]);
    if room {
        air_tail_bump(c, frame)
    } else {
        air_copy(c, frame)
    }
}


/// 0x2ef860.
pub(crate) fn air_tail_bump(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
    c.sw(p(r32(c, S0), air_streamer::COUNT), c.g[V0][0]);
    air_copy(c, frame)
}


/// 0x2ef868: copy the head point onto the tail point.
pub(crate) fn air_copy(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 11;
    c.g[V0][0] = Addr(p(r32(c, S0), air_streamer::RING)).lw(c);
    c.g[A1][0] = shl32(c.g[A1][0], 5);
    c.g[V1][0] = Addr(p(r32(c, S0), air_streamer::HEAD)).lw(c);
    c.g[A1][0] = add32(c.g[A1][0], c.g[V0][0]);
    c.lq(4, r32(c, A1));
    c.g[V1][0] = shl32(c.g[V1][0], 5);
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.sq(4, r32(c, V1));
    c.lq(2, p(r32(c, A1), 0x10));
    c.sq(2, p(r32(c, V1), 0x10));
    air_load_owner(c, frame)
}


/// 0x2ef894: `Approach(count, 2)` — the slot count jumps by two when it is
/// long enough, else falls back to the negative side (0x2ef8a4).
pub(crate) fn air_approach_count(c: &mut Cpu, frame: Frame) -> Flow {
    // The fade after landing: two points per 60 Hz frame of this streamer's
    // clock (every tick at 60); between those frames the count holds.
    let fx = r32(c, S0);
    if !tick::frame60_due(c, fx, 0) {
        c.g[V1][0] = c.g[A0][0];
        return air_store_count(c, frame);
    }
    c.cycles += 4;
    c.g[V0][0] = 2;
    let over = 2i64 < c.g[A0][0] as i64;
    if over {
        c.g[V1][0] = sx(r32(c, A0).wrapping_sub(0x2));
    } else {
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(0x2));
    }
    c.g[V0][0] = over as u64;
    if !over {
        // 0x2ef8a4: the negative half, reached by the branch.
        c.cycles += 4;
        let neg = -3i64 < c.g[A0][0] as i64;
        c.g[V0][0] = neg as u64;
        if neg {
            c.g[V1][0] = 0;
        }
    }
    air_store_count(c, frame)
}


/// 0x2ef8a4: the negative half of the index adjustment (reached by hand).
pub(crate) fn air_approach_count_neg(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[V0][0] = 0xffff_ffff_ffff_fffd;
    c.g[V1][0] = sx(r32(c, A0).wrapping_add(0x2));
    let neg = -3i64 < c.g[A0][0] as i64;
    c.g[V0][0] = neg as u64;
    if neg {
        c.g[V1][0] = 0;
    }
    air_store_count(c, frame)
}


/// 0x2ef8b4.
pub(crate) fn air_store_count(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.sw(p(r32(c, S0), air_streamer::COUNT), c.g[V1][0]);
    air_load_owner(c, frame)
}


/// 0x2ef8b8.
pub(crate) fn air_load_owner(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    air_length(c, frame)
}


/// 0x2ef8bc: advance the texture start by the wake length (a VU0 dot product
/// and vsqrt).
pub(crate) fn air_length(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 19;
    c.lwc1(2, gp(c, -0x393c));
    // The scroll per tick is speed * (0.01 / 60) (a folded constant the
    // `.lit4` patch does not see): * 60/rate above 60.
    crate::game::ai::rider::per_tick_60(c, 2);
    c.lqc2(3, p(r32(c, V0), 0x1e0));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
    c.mtc1(3, 0);
    c.lwc1(3, p(r32(c, S0), air_streamer::START));
    c.mul_s(0, 0, 2);
    c.mtc1(0, 1);
    c.sub_s(3, 3, 0);
    c.c_lt_s(3, 1);
    let shrinking = !c.fcc();
    c.swc1(3, p(r32(c, S0), air_streamer::START));
    if shrinking {
        air_start_clamp(c, frame)
    } else {
        air_start_increase(c, frame)
    }
}


/// 0x2ef908.
pub(crate) fn air_start_increase(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.add_s(0, 3, 0);
    c.swc1(0, p(r32(c, S0), air_streamer::START));
    air_start_clamp(c, frame)
}


/// 0x2ef918: wrap the texture start to [0, 1].
pub(crate) fn air_start_clamp(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    c.lwc1(0, p(r32(c, S0), air_streamer::START));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.c_lt_s(1, 0);
    if !c.fcc() {
        frame.restore(c, S0, 0x80);
        air_exit(c, frame);
        return Ok(());
    }
    air_start_subtract(c, frame)
}


/// 0x2ef934.
pub(crate) fn air_start_subtract(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.sub_s(0, 0, 1);
    c.swc1(0, p(r32(c, S0), air_streamer::START));
    frame.restore(c, S0, 0x80);
    air_exit(c, frame)
}


/// 0x2ef940: the epilogue.
pub(crate) fn air_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0x70);
    frame.close(c);
    Ok(())
}
 // the world pointer (D_004A28A8)
pub(crate) const WORLD_PAINTER_TABLE: u32 = 0x004f_a370;


// ---------------------------------------------------------------------------
// World painter.

/// `float WorldPainter_GetLighting28(int entry)` (0x2eeff0): call slot 41 of
/// the `sRSMEntry` at index `entry` (0xf0 bytes each) and return the float it
/// points at.
pub fn world_painter_get_lighting28(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0xf0;
        let index = abi::arg(c, 0);
        // `mult a0, a0, v1`: the entry index scaled by sizeof(sRSMEntry)
        // (leaves LO/HI as the original does).
        let offset = abi::mult(c, index, 0xf0);
        c.g[V0][0] = 0x0050_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(WORLD_PAINTER_TABLE);
        let entry = offset.wrapping_add(WORLD_PAINTER_TABLE);
        c.g[A0][0] = sx(entry);
        let p1c = Addr(entry).off(0x1c).lw(c);
        c.g[V1][0] = p1c;
        let obj = Addr(p1c as u32).lw(c);
        c.g[A2][0] = obj;
        let vt = Addr(obj as u32).off(rsm_obj::VTABLE).lw(c);
        c.g[A1][0] = vt;
        let delta = Addr(vt as u32).off(rsm_obj::SLOT_41).lh(c);
        c.g[A0][0] = delta;
        let method = Addr(vt as u32).off(rsm_obj::SLOT_41 + 4).lw(c);
        c.g[V0][0] = method;
        c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
        call_ptr(c, method as u32, 0x002e_f028)?;
        // The method returns a pointer to the value.
        let value = c.g[V0][0] as u32;
        c.lwc1(0, value);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `float WorldPainter_GetLighting30(void* p)` (0x2eefa8): the same through
/// render-state entry `i`'s object at_2 `+0x1c`, virtual slot 42.
pub fn world_painter_get_lighting30(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 18;
        c.g[V1][0] = 0xf0;
        c.mult(0, A0, A0, V1);
        c.g[V0][0] = 0x50_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5c90));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(at_2(c, A0, 0x1c));
        c.g[A2][0] = c.lw(r32(c, V1));
        c.g[A1][0] = c.lw(at_2(c, A2, 0x4));
        c.g[A0][0] = c.lh(at_2(c, A1, 0x150));
        c.g[V0][0] = c.lw(at_2(c, A1, 0x154));
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002e_efe0)?;
        c.lwc1(0, r32(c, V0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The table of world-painter view records (`D_004FA370`); each entry is
/// 0xf0 bytes.
pub(crate) const WP_VIEWS: u32 = 0x004f_a370;

pub(crate) const WP_ENTRY_SIZE: u32 = 0xf0;

/// The screen tint/fill overrides switch (`D_004A3B60`, `gp+0xa70`).
pub(crate) const SCREEN_OVERRIDES: u32 = 0xa70;


// ---------------------------------------------------------------------------
// WorldPainter virtual getters.

/// The 0x2ee*** family all do the same thing: index the global view table
/// (`&D_004FA370 + index * 0xf0`), look up `holder = table[index].field`,
/// `obj = *holder`, `vtable = obj->vtable`, then call
/// `vtable[vt_off/4](obj + delta)` through the function table and return the
/// float it points at.
pub(crate) fn world_painter_virtual_getter(c: &mut Cpu, table_off: u32, vt_off: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0xf0;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = index * 0xf0
        c.g[V0][0] = 0x50_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x50_0000u32.wrapping_sub(0x5c90));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        let entry = r32(c, A0);
        let holder = Addr(entry).off(table_off).lw(c);
        c.g[V1][0] = holder;
        let holder = r32(c, V1);
        let obj = Addr(holder).lw(c);
        c.g[A2][0] = obj;
        let obj = r32(c, A2);
        let vt = Addr(obj).off(4).lw(c);
        c.g[A1][0] = vt;
        let vt = r32(c, A1);
        c.g[A0][0] = Addr(vt).off(vt_off).lh(c);
        c.g[V0][0] = Addr(vt).off(vt_off + 4).lw(c);
        let target = r32(c, V0);
        let delta = r32(c, A0);
        c.g[RA][0] = sx(ret);
        c.g[A0][0] = sx(obj.wrapping_add(delta));
        call_ptr(c, target, ret)?;
        let r = r32(c, V0);
        c.lwc1(0, r);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The tint/fill family: like [`world_painter_virtual_getter`], but guarded
/// by the screen-override global (`gp+0xa70`): when it is set the function
/// returns a constant instead of calling.
pub(crate) fn world_painter_guarded_getter(
    c: &mut Cpu,
    vt_off: u32,
    ret: u32,
    default: GetterDefault,
) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp(c, SCREEN_OVERRIDES as i32)).lw(c);
        let overridden = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if overridden {
            match default {
                GetterDefault::One => {
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                }
                GetterDefault::Zero => {
                    c.cycles += 3;
                    c.mtc1(0, 0);
                }
            }
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 14;
        c.g[V1][0] = 0xf0;
        c.g[V0][0] = 0x50_0000;
        c.mult(0, V1, A0, V1); // v1 = index * 0xf0
        c.g[V0][0] = sx(0x50_0000u32.wrapping_sub(0x5c90));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        let entry = r32(c, V1);
        c.g[A0][0] = Addr(entry).off(0xc).lw(c);
        let holder = r32(c, A0);
        c.g[A1][0] = Addr(holder).lw(c);
        let obj = r32(c, A1);
        c.g[V1][0] = Addr(obj).off(4).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(vt_off).lh(c);
        c.g[V0][0] = Addr(vt).off(vt_off + 4).lw(c);
        let target = r32(c, V0);
        let delta = r32(c, A0);
        c.g[RA][0] = sx(ret);
        c.g[A0][0] = sx(obj.wrapping_add(delta));
        call_ptr(c, target, ret)?;
        let r = r32(c, V0);
        c.lwc1(0, r);
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// The constant a guarded getter returns when the screen overrides are on.
#[derive(Clone, Copy)]
pub(crate) enum GetterDefault {
    /// 1.0f (`mtc1 at = 0x3f800000`).
    One,
    /// 0.0f (`mtc1 zero`).
    Zero,
}


/// `float WorldPainter_GetFog18(int index)` (0x2eea50): vt slot `+0x48`.
pub fn world_painter_get_fog18(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x4, 0x48, 0x002e_ea88);
}


/// `float WorldPainter_GetFog20(int index)` (0x2eea98): vt slot `+0x50`.
pub fn world_painter_get_fog20(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x4, 0x50, 0x002e_ead0);
}


/// `float WorldPainter_GetFog28(int index)` (0x2eeae0): vt slot `+0x58`.
pub fn world_painter_get_fog28(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x4, 0x58, 0x002e_eb18);
}


/// `float WorldPainter_GetFog30(int index)` (0x2eeb28): vt slot `+0x60`.
pub fn world_painter_get_fog30(c: &mut Cpu) {
    world_painter_virtual_getter(c, 0x4, 0x60, 0x002e_eb60);
}


/// `float WorldPainter_GetScreenTintRed(int index)` (0x2eeb70): vt slot
/// `+0xa0`; 1.0f when the screen overrides are on.
pub fn world_painter_get_screen_tint_red(c: &mut Cpu) {
    world_painter_guarded_getter(c, 0xa0, 0x002e_ebc0, GetterDefault::One);
}


/// `float WorldPainter_GetScreenTintGreen(int index)` (0x2eebd0): vt slot
/// `+0xa8`; 1.0f when the screen overrides are on.
pub fn world_painter_get_screen_tint_green(c: &mut Cpu) {
    world_painter_guarded_getter(c, 0xa8, 0x002e_ec20, GetterDefault::One);
}


/// `float WorldPainter_GetScreenTintBlue(int index)` (0x2eec30): vt slot
/// `+0xb0`; 1.0f when the screen overrides are on.
pub fn world_painter_get_screen_tint_blue(c: &mut Cpu) {
    world_painter_guarded_getter(c, 0xb0, 0x002e_ec80, GetterDefault::One);
}


/// `float WorldPainter_GetScreenFillRed(int index)` (0x2eec90): vt slot
/// `+0xb8`; 0.0f when the screen overrides are on.
pub fn world_painter_get_screen_fill_red(c: &mut Cpu) {
    world_painter_guarded_getter(c, 0xb8, 0x002e_ecdc, GetterDefault::Zero);
}


/// `float WorldPainter_GetScreenFillGreen(int index)` (0x2eecf0): vt slot
/// `+0xc0`; 0.0f when the screen overrides are on.
pub fn world_painter_get_screen_fill_green(c: &mut Cpu) {
    world_painter_guarded_getter(c, 0xc0, 0x002e_ed3c, GetterDefault::Zero);
}


/// `float WorldPainter_GetScreenFillBlue(int index)` (0x2eed50): vt slot
/// `+0xc8`; 0.0f when the screen overrides are on.
pub fn world_painter_get_screen_fill_blue(c: &mut Cpu) {
    world_painter_guarded_getter(c, 0xc8, 0x002e_ed9c, GetterDefault::Zero);
}


/// One `WorldPainter_Get*` accessor: the painter index goes in a0, the value
/// comes back in f0.
pub(crate) fn painter_get(c: &mut Cpu, addr: u32, ret: u32, f: GuestFn, index: u64) -> Flow {
    c.g[A0][0] = index;
    Callee { addr, f }.call(c, ret)
}


/// Store the previous accessor's result at `cache + slot + off` and call the
/// next accessor (the interleaved store/call pattern of the cache loop).
pub(crate) fn painter_step(
    c: &mut Cpu,
    index: u64,
    cache: u64,
    slot: u64,
    off: u32,
    addr: u32,
    ret: u32,
    f: GuestFn,
) -> Flow {
    c.g[V0][0] = sx((cache as u32).wrapping_add(slot as u32));
    c.g[A0][0] = index;
    let dst = (c.g[V0][0] as u32).wrapping_add(off);
    c.swc1(0, dst);
    Callee { addr, f }.call(c, ret)
}


/// `void PS2VisualEffectsMan_ApplyWorldPainter(cache, int index)` (0x2f00a0):
/// copy the world painter's parameters for `index` into the effects
/// manager's per-index record.
///
/// The first time (`D_004A5D38 == 0`) the fog, screen-tint and weather values
/// are latched into the gp_2 globals and a 4-float record (`+0x4720`), and the
/// tint-enable flag is set. Then the light-glow values are latched (unless
/// `D_004A63CC` says otherwise) and the record at `cache + index*4` gets the
/// surface, camera, fog, tint and fill values at `+0x8c..+0x24c`.
///
/// Frame 0x60: s1 at +0x30, s2 at +0x20, s0 at +0x40, ra at +0x10, f21 at
/// +0x58, f20 at +0x50.
pub fn ps2_visual_effects_man_apply_world_painter(c: &mut Cpu) {
    use ssx3_ee::s09 as wp;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = c.lw(gp_2(c, painter::READY));
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0x10);
        c.swc1(21, frame.at(c, 0x58));
        c.swc1(20, frame.at(c, 0x50));
        let applied = c.g[V0][0] != 0;
        c.g[S2][0] = c.lw(gp_2(c, painter::GFX));
        let index = c.g[S1][0];
        if !applied {
            // First use: latch the fog / tint / weather parameters.
            c.cycles += 35;
            c.g[S0][0] = 0x0050_0000;
            painter_get(c, 0x002e_e9c0, 0x002f_00d8, wp::WorldPainter_GetFog08, index)?;
            c.g[V0][0] = 1;
            c.sw(gp_2(c, 0x1244), c.g[V0][0]);
            c.swc1(0, gp_2(c, 0x1248));
            painter_get(c, 0x002e_ea08, 0x002f_00ec, wp::WorldPainter_GetFog10, index)?;
            c.swc1(0, gp_2(c, 0x123c));
            painter_get(c, 0x002e_ea50, 0x002f_00f8, wp::WorldPainter_GetFog18, index)?;
            c.swc1(0, gp_2(c, 0x1240));
            painter_get(c, 0x002e_ea98, 0x002f_0104, wp::WorldPainter_GetFog20, index)?;
            c.mov_s(21, 0);
            painter_get(c, 0x002e_eae0, 0x002f_0110, wp::WorldPainter_GetFog28, index)?;
            c.mov_s(20, 0);
            painter_get(c, 0x002e_eb28, 0x002f_011c, wp::WorldPainter_GetFog30, index)?;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.swc1(21, frame.at(c, 4));
            c.swc1(1, frame.at(c, 0));
            c.swc1(20, frame.at(c, 8));
            c.swc1(0, frame.at(c, 0xc));
            c.g[A1][0] = sx(0x0050_0000u32.wrapping_add(0x4720));
            c.ldl(2, frame.at(c, 7));
            c.ldr(2, frame.at(c, 0));
            c.ldl(3, frame.at(c, 0xf));
            c.ldr(3, frame.at(c, 8));
            let dst = c.g[A1][0] as u32;
            c.sdl(2, dst.wrapping_add(7));
            c.sdr(2, dst);
            c.sdl(3, dst.wrapping_add(0xf));
            c.sdr(3, dst.wrapping_add(8));
        }

        // Screen tint / fill / weather (skipped once latched).
        c.cycles += 3;
        c.g[V0][0] = c.lw(gp_2(c, painter::HAVE_TINT));
        let have_tint = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gp_2(c, 0x12dc));
        if !have_tint {
            c.cycles += 29;
            c.g[S0][0] = 0;
            painter_get(c, 0x002e_eb70, 0x002f_0170, wp::WorldPainter_GetScreenTintRed, index)?;
            c.swc1(0, gp_2(c, 0x14f4));
            painter_get(c, 0x002e_ebd0, 0x002f_017c, wp::WorldPainter_GetScreenTintGreen, index)?;
            c.swc1(0, gp_2(c, 0x14f8));
            painter_get(c, 0x002e_ec30, 0x002f_0188, wp::WorldPainter_GetScreenTintBlue, index)?;
            c.swc1(0, gp_2(c, 0x14fc));
            painter_get(c, 0x002e_ec90, 0x002f_0194, wp::WorldPainter_GetScreenFillRed, index)?;
            c.swc1(0, gp_2(c, 0x1500));
            painter_get(c, 0x002e_ecf0, 0x002f_01a0, wp::WorldPainter_GetScreenFillGreen, index)?;
            c.swc1(0, gp_2(c, 0x1504));
            painter_get(c, 0x002e_ed50, 0x002f_01ac, wp::WorldPainter_GetScreenFillBlue, index)?;
            c.swc1(0, gp_2(c, 0x1508));
            painter_get(c, 0x002e_e738, 0x002f_01b8, wp::WorldPainter_GetWeather58, index)?;
            c.swc1(0, gp_2(c, 0x1524));
            c.g[A0][0] = 1;
            c.g[V0][0] = c.lw(gp_2(c, -0x848));
            c.g[V1][0] = c.lw(at(c.g[V0][0], 0x84));
            c.g[V1][0] = c.lw(at(c.g[V1][0], 0x28));
            c.g[V0][0] = c.lw(at(c.g[V1][0], 0));
            let mut store_again = true;
            if c.g[V0][0] == c.g[A0][0] {
                c.g[S0][0] = 1;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V1][0];
                Callee { addr: 0x002f_7be0, f: wp::PS2VisualEffectsMan_IsHandleNull }
                    .call(c, 0x002f_01e0)?;
                if c.g[V0][0] == 0 {
                    c.sw(gp_2(c, 0x151c), c.g[S0][0]);
                    store_again = false;
                } else {
                    c.cycles += 1;
                    c.g[S0][0] = 1;
                }
            }
            if store_again {
                c.cycles += 1;
                c.sw(gp_2(c, 0x151c), c.g[S0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(gp_2(c, 0x12dc));
        }

        // Light glows (skipped when the weather flag is set).
        c.cycles += 2;
        let have_glow = c.g[V0][0] != 0;
        c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
        if !have_glow {
            c.cycles += 22;
            painter_get(c, 0x002e_edb0, 0x002f_0204, wp::WorldPainter_GetLightGlow08, index)?;
            c.swc1(0, gp_2(c, 0x12e4));
            painter_get(c, 0x002e_edf8, 0x002f_0210, wp::WorldPainter_GetLightGlow10, index)?;
            c.swc1(0, gp_2(c, 0x12e8));
            painter_get(c, 0x002e_ee40, 0x002f_021c, wp::WorldPainter_GetLightGlow18, index)?;
            c.swc1(0, gp_2(c, 0x12ec));
            painter_get(c, 0x002e_ee88, 0x002f_0228, wp::WorldPainter_GetLightGlow20, index)?;
            c.swc1(0, gp_2(c, 0x12f0));
            painter_get(c, 0x002e_eed0, 0x002f_0234, wp::WorldPainter_GetLightGlow28, index)?;
            c.swc1(0, gp_2(c, 0x12f4));
            painter_get(c, 0x002e_ef18, 0x002f_0240, wp::WorldPainter_GetLightGlow30, index)?;
            c.swc1(0, gp_2(c, 0x1300));
            painter_get(c, 0x002e_ef60, 0x002f_024c, wp::WorldPainter_GetLightGlow38, index)?;
            c.swc1(0, gp_2(c, 0x1304));
            c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
        }

        // The per-index record: each step stores the previous accessor's
        // value and starts the next one.
        let cache = c.g[S2][0];
        let slot = c.g[S0][0];
        c.cycles += 67;
        c.g[V1][0] = c.lw(gp_2(c, 0xa6c));
        c.g[V0][0] = sx((cache as u32).wrapping_add(slot as u32));
        c.g[A0][0] = index;
        let dst = at(c.g[V0][0], 0x8c);
        c.sw(dst, c.g[V1][0]);
        Callee { addr: 0x002e_f038, f: wp::WorldPainter_GetSurface08 }.call(c, 0x002f_0268)?;
        painter_step(c, index, cache, slot, 0xac, 0x002e_e3b8, 0x002f_0278, wp::WorldPainter_GetCamera08)?;
        painter_step(c, index, cache, slot, 0xcc, 0x002e_e9c0, 0x002f_0288, wp::WorldPainter_GetFog08)?;
        painter_step(c, index, cache, slot, 0xec, 0x002e_ea98, 0x002f_0298, wp::WorldPainter_GetFog20)?;
        painter_step(c, index, cache, slot, 0x10c, 0x002e_eae0, 0x002f_02a8, wp::WorldPainter_GetFog28)?;
        painter_step(c, index, cache, slot, 0x12c, 0x002e_eb28, 0x002f_02b8, wp::WorldPainter_GetFog30)?;
        painter_step(c, index, cache, slot, 0x14c, 0x002e_ea08, 0x002f_02c8, wp::WorldPainter_GetFog10)?;
        painter_step(c, index, cache, slot, 0x16c, 0x002e_ea50, 0x002f_02d8, wp::WorldPainter_GetFog18)?;
        painter_step(c, index, cache, slot, 0x18c, 0x002e_eb70, 0x002f_02e8, wp::WorldPainter_GetScreenTintRed)?;
        painter_step(c, index, cache, slot, 0x1ac, 0x002e_ebd0, 0x002f_02f8, wp::WorldPainter_GetScreenTintGreen)?;
        painter_step(c, index, cache, slot, 0x1cc, 0x002e_ec30, 0x002f_0308, wp::WorldPainter_GetScreenTintBlue)?;
        painter_step(c, index, cache, slot, 0x1ec, 0x002e_ec90, 0x002f_0318, wp::WorldPainter_GetScreenFillRed)?;
        painter_step(c, index, cache, slot, 0x20c, 0x002e_ecf0, 0x002f_0328, wp::WorldPainter_GetScreenFillGreen)?;
        painter_step(c, index, cache, slot, 0x22c, 0x002e_ed50, 0x002f_0338, wp::WorldPainter_GetScreenFillBlue)?;

        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        frame.restore(c, S1, 0x30);
        let dst = at(c.g[S0][0], 0x24c);
        c.swc1(0, dst);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 21, 0x58);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}


/// `void WorldPainter_SetScriptSurfaceMode(WorldPainter* p)` (0x2ef368):
/// remembers the painter (`D_004A3B5C`) and whether it is non-null
/// (`D_004A4444`, as the 0/1 `v0`).
pub fn world_painter_set_script_surface_mode(c: &mut Cpu) {
    c.cycles += 4;
    let painter = c.g[A0][0];
    c.g[V0][0] = (0 < painter) as u64;
    c.sw(gp_3(c, 0xa6c), painter);
    c.sw(gp_3(c, 0x1354), c.g[V0][0]);
    abi::ret(c);
}


/// `void* PS2AIVisualEffectsMan_Create()` (0x2f0390): allocate the 8-byte AI
/// effects manager and construct it.
pub fn ps2_ai_visual_effects_man_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 8, 0x0048_7b88, 0, 0x002f_03b0, |c, ret| {
            callee::AI_VFX_MAN_CTOR.call(c, ret)
        })
    })
}


/// `void* PS2VisualEffectsMan_Create()` (0x2f0510): allocate the 0x48-byte
/// visual effects manager and construct it.
pub fn ps2_visual_effects_man_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 0x48, 0x0048_7ba0, 0, 0x002f_0530, |c, ret| {
            callee::VFX_MAN_CTOR.call(c, ret)
        })
    })
}


/// `strcpy` (0x4166f4) — reached through the function table.
pub(crate) const STRLEN_AND_STRCPY: u32 = 0x0041_66f4;


// ---------------------------------------------------------------------------
// WorldPainter_SetIrradianceOverride.

/// `void WorldPainter_SetIrradianceOverride(const char* name)` (0x2ef378):
/// copy `name` into the world painter's override string at `gp_4 + 0x24c0` and
/// mark the override active; a null name just clears the flag.
pub fn world_painter_set_irradiance_override(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let named = c.g[A0][0] != 0;
        frame.save_ra(c, 0);
        let flag = gp_4(c, 0xa74);
        if !named {
            c.cycles += 2;
            c.sw(flag, 0);
        } else {
            c.cycles += 6;
            let buffer = gp_4(c, 0x24c0);
            c.g[A1][0] = c.g[A0][0];
            c.g[A0][0] = sx(buffer);
            call_ptr(c, STRLEN_AND_STRCPY, 0x002e_f39c)?;
            c.g[V1][0] = 1;
            let one = c.g[V1][0];
            c.sw(flag, one);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `const char* WorldPainter_GetIrradianceName2(void)` (0x2ef198): the static
/// fallback name when no world painter is live, else the painter's name.
pub fn world_painter_get_irradiance_name2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp(c, 0xa74)).lw(c);
        let live = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !live {
            c.cycles += 14;
            c.g[V1][0] = 0xf0;
            c.g[V0][0] = 0x0050_0000;
            c.mult(0, V1, A0, V1);
            c.g[V0][0] = sx(0x004f_a370);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
            c.g[A1][0] = Addr(r32(c, A0)).lw(c);
            c.g[V1][0] = Addr(r32(c, A1)).off(0x4).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x138).lh(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x13c).lw(c);
            let target = r32(c, V0);
            c.g[RA][0] = sx(0x002e_f1d8);
            c.g[A0][0] = this_with_delta(c.g[A1][0], c.g[A0][0]);
            call_ptr(c, target, 0x002e_f1d8)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        c.g[V0][0] = sx(gp(c, 0x24c0));
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `PS2AIVisualEffectsMan* PS2AIVisualEffectsMan_ctor(PS2AIVisualEffectsMan* m)`
/// (0x2f03c8): construct the AI visual-effects manager, allocating and
/// initialising its avalanche effect.
pub fn ps2_ai_visual_effects_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 17, 0x10);
        sqsp(c, 16, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::AI_VISUAL_EFFECTS_MAN_CTOR.call(c, 0x002f_03e0)?;
        c.g[V0][0] = 0x0048_7d50;
        c.g[A1][0] = 0x0048_7568;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        Addr(p_2(r32(c, S1), 4)).sw(c, c.g[V0][0]);
        c.g[A0][0] = 4;
        call::call_ptr(c, 0x0031_7d70, 0x002f_0404)?;
        c.g[V1][0] = 0x0048_8648;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        Addr(c.g[S0][0] as u32).sw(c, c.g[V1][0]);
        call::call_ptr(c, 0x002d_6410, 0x002f_041c)?;
        Addr(c.g[S1][0] as u32).sw(c, c.g[S0][0]);
        c.g[V0][0] = c.g[S1][0];
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// Destructors / save methods.

/// `func_002F0438` (0x2f0438): the PS2 AI visual-effects manager destructor.
/// Installs the deleting-destructor vtable `0x487d78`, destroys the owned
/// object at `+0` through its vtable (`vtable[8]`, flag 3) and, when the
/// delete flag (`a1 & 1`) is set, frees `this` (`0x317e50`).
pub fn ps2ai_visual_effects_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(low(c, V0).wrapping_add(0x7d50));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sw_r(c, S0, 0x4, c.g[V0][0]);
        c.g[A2][0] = lw_r(c, S0, 0);
        let none = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 6;
            c.g[V0][0] = Addr(low(c, A2)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = lh_r(c, V0, 0x8);
            c.g[V1][0] = lw_r(c, V0, 0xc);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x002f_047c);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x002f_047c)?;
        }
        c.cycles += 5;
        c.g[V0][0] = 0x48_0000;
        c.g[V1][0] = c.g[S1][0] & 0x1;
        c.g[V0][0] = sx(low(c, V0).wrapping_add(0x7d78));
        let keep = c.g[V1][0] == 0;
        sw_r(c, S0, 0x4, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[RA][0] = sx(0x002f_0498);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x002f_0498)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::fakes;
    use crate::difftest::put_words;
    use ssx3_ee as ee;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::SP;
    use crate::difftest::fakes::vtable;
    use crate::difftest::put_vec;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::fakes::ptr;
    use crate::difftest::RA as TEST_RA;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::slot;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::delta;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::gfx_fixture;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::streamer_rider;
    pub(crate) use crate::game::ai::rider::tests::put_world_2 as put_world;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::worldpainter::quadtree::tests::pool_allocator;
    pub(crate) use crate::game::worldpainter::quadtree::tests::MEM_HEAP;
    pub(crate) use crate::game::worldpainter::quadtree::tests::run;
    pub(crate) use crate::game::worldpainter::quadtree::tests::heap_init;
    pub(crate) use crate::game::worldpainter::quadtree::tests::alloc_machine;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::vtable_2;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::camera::cameracontroller::tests::pool;
    pub(crate) use crate::game::ai::ai::tests::slot as slot_2;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3;
    pub(crate) use crate::game::bx::memblockman::tests::fake_slot;


    #[test]
    fn c_air_streamer_fx_reset_matches() {
        check(ssx3_ee::s09::f_002ef6a0, c_air_streamer_fx_reset, 200, &|c, seed| {
            let mut r = Rng(0x100e ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.bus.write32(
                DATA + 0x100 + 0x870,
                if seed % 2 == 0 { r.next_u32() } else { r.next_u32() | 0x8000_0000 },
            );
        });
    }


    #[test]
    fn ai_visual_effects_man_ctor_matches() {
        check(ssx3_ee::s09::f_002f0368, ai_visual_effects_man_ctor, 100, &|c, seed| {
            let mut r = Rng(0x100f ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }


    #[test]
    fn c_air_streamer_fx_render_matches() {
        covered(ssx3_ee::s09::f_002ef950__generated, c_air_streamer_fx_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2ef9_50, 2);
            gfx_fixture(c, &mut r);
            let (this, rider, points) = (DATA, DATA + 0x100, DATA + 0x2000);
            c.g[A0][0] = sx(this);
            c.bus.write32(this + air::RIDER, rider);
            c.bus.write32(this + air::POINTS, points);
            // Up to 26 segments (52 vertices per strip).
            let n = if seed % 5 == 0 { r.next_u32() % 2 } else { 2 + r.next_u32() % 25 };
            c.bus.write32(this + air::COUNT, n);
            c.bus.write32(this + air::FIRST, r.next_u32() % 0x19);
            c.bus.write32(this + air::ALPHA, r.f32(1.0));
            c.bus.write32(this + air::LENGTH, 1 + r.next_u32() % 30);
            streamer_rider(c, &mut r, rider, 0x89c, 0x8a4);
            c.bus.write32(rider + 0x2ec, r.f32(10.0));
            fill(c, &mut r, points, 0x19 * 8);
            c.bus.write32(GAME_GP + 0x1630, (seed % 3 == 0) as u32);
            fill(c, &mut r, GAME_GP - 0x3940, 8);
            c.bus.write32(GAME_GP + 0xa84, r.f32(1.0));
        });
    }


    #[test]
    fn world_painter_get_irradiance_name0_matches() {
        world_painter_name_matches(
            ee::s09::f_002ef0e8__generated,
            world_painter_get_irradiance_name0,
            0x2ef0e8,
            0x128,
        );
    }


    #[test]
    fn world_painter_get_irradiance_name1_matches() {
        world_painter_name_matches(
            ee::s09::f_002ef140__generated,
            world_painter_get_irradiance_name1,
            0x2ef140,
            0x130,
        );
    }


    /// The two irradiance-name getters differ only in the vtable slot.
    pub(crate) fn world_painter_name_matches(
        generated: GuestFn,
        hand: GuestFn,
        salt: u64,
        slot: u32,
    ) {
        check(generated, hand, 200, &|c, seed| {
            let mut r = machine(c, seed, salt, 2);
            let entry = 0x004f_a460u32;
            let obj = DATA + 0x100;
            let cell = DATA + 0x200;
            // The painter exists: return the built-in name.
            c.bus.write32(GAME_GP + 0xa74, if seed % 3 == 0 { 0 } else { 1 });
            if seed % 3 == 0 {
                return;
            }
            if seed % 3 == 1 {
                // Empty object table: the call target is 0 (fake).
                c.bus.write32(entry + 0x1c, 0);
                return;
            }
            let vt = fakes::vtable(c, &mut r, &[slot]);
            c.bus.write32(cell, obj);
            c.bus.write32(obj + 4, vt);
            c.bus.write32(entry + 0x1c, cell);
        });
    }


    #[test]
    fn c_air_streamer_fx_update_matches() {
        check(ee::s09::f_002ef6d0__generated, c_air_streamer_fx_update, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2ef6d0, 2);
            let world = DATA + 0x100;
            let man = DATA + 0x200;
            let ctx = DATA + 0x300;
            put_world(c, world, man, ctx);
            c.bus.write32(ctx + 8, if seed % 2 == 0 { 0 } else { 1 });
            let fx = DATA;
            let owner = DATA + 0x1000;
            let state = DATA + 0x1400;
            c.bus.write32(fx, owner);
            c.bus.write32(owner + 0x77c, state);
            c.bus.write32(state + 0xde0, [1u32, 0, 2, 3, 1][(seed % 5) as usize]);
            c.bus.write32(fx + 0x4, DATA + 0x2000); // ring
            c.bus.write32(fx + 0x8, seed % 0x19); // head
            c.bus.write32(fx + 0xc, [0u32, 1, 2, 4, 0x20][(seed % 5) as usize]); // count
            c.bus.write32(fx + 0x14, [1u32, 2, 3][(seed % 3) as usize]); // len
            c.bus.write32(fx + 0x10, if seed % 2 == 0 { 0xbf80_0000 } else { r.f32(2.0) });
            c.bus.write32(owner + 0x780, DATA + 0x1800);
            c.bus.write32(owner + 0x89c, 1);
            c.bus.write32(owner + 0x8a4, 2);
            c.bus.write32(owner + 0x1e0, r.next_u32());
            put_words(c, GAME_GP - 0x393c, &[0x3c88_8889]);
            c.pc = 0x002e_f6d0;
            c.g[A0][0] = sx(fx);
        });
    }


    /// The air streamer after `ticks` updates at `hz` (the AI frame counter
    /// stepping once per tick): (head, count, texture start). `fx` is a
    /// fresh address per run (the 60 Hz clock is keyed by it).
    fn air_after(hz: u32, ticks: u32, fx: u32, airborne: bool) -> (u32, u32, f32) {
        let mut c = crate::difftest::fresh();
        let _r = machine(&mut c, 1, 0x2ef6d1, 0);
        crate::tick::set_for_test(&mut c, hz);
        let (world, man, ctx) = (DATA + 0x100, DATA + 0x200, DATA + 0x300);
        put_world(&mut c, world, man, ctx);
        let owner = DATA + 0x1000;
        let state = DATA + 0x1400;
        c.bus.write32(owner + 0x77c, state);
        c.bus.write32(state + 0xde0, if airborne { 1 } else { 0 });
        c.bus.write32(owner + 0x780, DATA + 0x1800);
        put_vec(&mut c, owner + 0x1e0, [100.0f32.to_bits(), 0, 0, 0]);
        // 0.01/60, the executable's value.
        put_words(&mut c, GAME_GP - 0x393c, &[0x392e_c33e]);
        c.bus.write32(fx, owner);
        c.bus.write32(fx + air_streamer::RING, DATA + 0x2000);
        c.bus.write32(fx + air_streamer::HEAD, 0);
        c.bus.write32(fx + air_streamer::COUNT, if airborne { 0 } else { 20 });
        c.bus.write32(fx + air_streamer::LEN, 25);
        c.bus.write32(fx + air_streamer::START, 0.5f32.to_bits());
        for t in 0..ticks {
            c.bus.write32(ctx + 8, t);
            c.g[A0][0] = sx(fx);
            c.pc = 0x002e_f6d0;
            c_air_streamer_fx_update(&mut c);
        }
        (
            c.bus.read32(fx + air_streamer::HEAD),
            c.bus.read32(fx + air_streamer::COUNT),
            f32::from_bits(c.bus.read32(fx + air_streamer::START)),
        )
    }


    /// The air streamer records 30 points per second, fades two points per
    /// 60 Hz frame after landing and scrolls at the same speed at 120 and
    /// 240 Hz as at 60.
    #[test]
    fn c_air_streamer_fx_update_follows_the_rate() {
        let mut fx = DATA + 0x8000;
        for airborne in [true, false] {
            for frames in [3u32, 8] {
                let at60 = air_after(60, frames, fx, airborne);
                fx += 0x100;
                if airborne {
                    assert_eq!((at60.0, at60.1), ((frames + 1) / 2, (frames + 1) / 2), "{at60:?}");
                } else {
                    assert_eq!(at60.1, 20 - 2 * frames, "{at60:?}");
                }
                for hz in [120u32, 240] {
                    let at = air_after(hz, frames * hz / 60, fx, airborne);
                    fx += 0x100;
                    assert_eq!((at.0, at.1), (at60.0, at60.1), "{hz} Hz {airborne} {frames}: {at:?} vs {at60:?}");
                    let d = (at.2 - at60.2).abs();
                    assert!(d.min((1.0 - d).abs()) < 1e-4, "{hz} Hz {airborne} {frames}: start {} vs {}", at.2, at60.2);
                }
            }
        }
    }


    // ---------------------------------------------------------- world painter

    /// `WorldPainter_GetLighting28` dispatches through slot 41 of a vtable:
    /// point it at [`FAKE`] so the call lands in the fake-call hook.
    #[test]
    fn world_painter_get_lighting28_matches() {
        check(ee::s09::f_002eeff0__generated, world_painter_get_lighting28, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0xeff0, 0);
            let index = if seed % 2 == 0 { 1 } else { 1 + r.next_u32() % 0x20 };
            c.g[A0][0] = index as u64; // entry index (0xf0 bytes each)
            let entry = 0x004f_a370u32.wrapping_add(index * 0xf0);
            c.bus.write32(entry + 0x1c, DATA + 0x100); // entry->p1C
            c.bus.write32(DATA + 0x100, DATA + 0x200); // the object
            c.bus.write32(DATA + 0x204, DATA + 0x400); // o->vt
            c.bus.write16(DATA + 0x400 + 0x148, 0xfff8); // slot 41: delta -8
            c.bus.write32(DATA + 0x400 + 0x14c, FAKE); // slot 41: method
        });
    }


    #[test]
    fn world_painter_get_lighting30_matches() {
        covered(ssx3_ee::s09::f_002eefa8__generated, world_painter_get_lighting30, 120, &|c, seed| {
            let mut r = machine(c, seed, 59, 1);
            let i = r.next_u32() % 3;
            let slot = 0x004f_a370 + i * 0xf0;
            let cell = DATA + 0x400 + i * 0x100;
            let obj = DATA + 0x800 + i * 0x100;
            let vt = vtable(c, &mut r, &[0x150]);
            c.bus.write32(obj, vt);
            c.bus.write32(cell, obj);
            c.bus.write32(slot + 0x1c, cell);
            c.g[A0][0] = sx(i);
        });
    }


    // ------------------------------------------------------------- fog getters

    /// `entry = &D_004FA370[index]`, `holder = entry[table_off/4]`,
    /// `obj = *holder`, `obj->vt = *obj+4`, slot `vt_off` a fake.
    pub(crate) fn table_getter_setup(c: &mut Cpu, seed: u32, salt: u64, table_off: u32, vt_off: u32) {
        let mut r = fakes::machine(c, seed, salt, 1);
        let index = seed % 4;
        c.g[A0][0] = index as u64;
        let entry = WP_VIEWS.wrapping_add(index.wrapping_mul(WP_ENTRY_SIZE));
        let holder = fakes::ptr(&mut r);
        let obj = fakes::ptr(&mut r);
        let vt = fakes::ptr(&mut r);
        c.bus.write32(entry.wrapping_add(table_off), holder);
        c.bus.write32(holder, obj);
        c.bus.write32(obj.wrapping_add(4), vt);
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt.wrapping_add(vt_off), delta as u16);
        c.bus.write32(vt.wrapping_add(vt_off).wrapping_add(4), FAKE);
    }


    pub(crate) fn fog_setup(c: &mut Cpu, seed: u32, vt_off: u32) {
        table_getter_setup(c, seed, 0x0e50_00a1 ^ vt_off as u64, 0x4, vt_off);
    }


    #[test]
    fn world_painter_get_fog18_matches() {
        fakes::covered(ee::s09::f_002eea50__generated, world_painter_get_fog18, 200, &|c, seed| {
            fog_setup(c, seed, 0x48);
        });
    }


    #[test]
    fn world_painter_get_fog20_matches() {
        fakes::covered(ee::s09::f_002eea98__generated, world_painter_get_fog20, 200, &|c, seed| {
            fog_setup(c, seed, 0x50);
        });
    }


    #[test]
    fn world_painter_get_fog28_matches() {
        fakes::covered(ee::s09::f_002eeae0__generated, world_painter_get_fog28, 200, &|c, seed| {
            fog_setup(c, seed, 0x58);
        });
    }


    #[test]
    fn world_painter_get_fog30_matches() {
        fakes::covered(ee::s09::f_002eeb28__generated, world_painter_get_fog30, 200, &|c, seed| {
            fog_setup(c, seed, 0x60);
        });
    }


    // ---------------------------------------------- tint / screen-fill getters

    pub(crate) fn tint_setup(c: &mut Cpu, seed: u32, vt_off: u32) {
        table_getter_setup(c, seed, 0x0e50_00b2 ^ vt_off as u64, 0xc, vt_off);
        // D_004A3B60: set on two seeds out of three (the constant path).
        let on = if seed % 3 == 0 { 0 } else { 1 };
        c.bus.write32(GAME_GP.wrapping_add(SCREEN_OVERRIDES), on);
    }


    #[test]
    fn world_painter_get_screen_tint_red_matches() {
        fakes::covered(
            ee::s09::f_002eeb70__generated,
            world_painter_get_screen_tint_red,
            200,
            &|c, seed| tint_setup(c, seed, 0xa0),
        );
    }


    #[test]
    fn world_painter_get_screen_tint_green_matches() {
        fakes::covered(
            ee::s09::f_002eebd0__generated,
            world_painter_get_screen_tint_green,
            200,
            &|c, seed| tint_setup(c, seed, 0xa8),
        );
    }


    #[test]
    fn world_painter_get_screen_tint_blue_matches() {
        fakes::covered(
            ee::s09::f_002eec30__generated,
            world_painter_get_screen_tint_blue,
            200,
            &|c, seed| tint_setup(c, seed, 0xb0),
        );
    }


    #[test]
    fn world_painter_get_screen_fill_red_matches() {
        fakes::covered(
            ee::s09::f_002eec90__generated,
            world_painter_get_screen_fill_red,
            200,
            &|c, seed| tint_setup(c, seed, 0xb8),
        );
    }


    #[test]
    fn world_painter_get_screen_fill_green_matches() {
        fakes::covered(
            ee::s09::f_002eecf0__generated,
            world_painter_get_screen_fill_green,
            200,
            &|c, seed| tint_setup(c, seed, 0xc0),
        );
    }


    #[test]
    fn world_painter_get_screen_fill_blue_matches() {
        fakes::covered(
            ee::s09::f_002eed50__generated,
            world_painter_get_screen_fill_blue,
            200,
            &|c, seed| tint_setup(c, seed, 0xc8),
        );
    }


    #[test]
    fn apply_world_painter_matches() {
        check(
            ssx3_ee::s09::f_002f00a0__generated,
            ps2_visual_effects_man_apply_world_painter,
            300,
            &|c, seed| {
                let mut r = Rng(0x2f00_a0c5 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let g = GAME_GP;
                let index = (seed % 4) as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = index;
                // world -> app (+0x84) -> game module (+0x28) -> handle.
                c.bus.write32(g - 0x848, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 0x28, DATA + 0x2200);
                c.bus.write32(DATA + 0x2200, match seed % 4 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32() & 1,
                });
                c.bus.write32(g + 0x2a90, DATA + 0x4000);
                // `gp_2 + 0xa70 == 0`: the tint accessors take the 1.0f path.
                c.bus.write32(g + 0xa70, 0);
                // The painter descriptors all lead to one object whose vtable
                // methods are MenuSys_IsActive (it returns 0 for a zero word).
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
                c.bus.write32(g + 0x14ec, (seed % 5 == 0) as u32);
                c.bus.write32(g + 0x12dc, (seed % 4 == 1) as u32);
                c.bus.write32(g + 0xa6c, r.next_u32());
                for off in [0x1244u32, 0x1248, 0x123c, 0x1240, 0x151c, 0x1524,
                            0x12e4, 0x12e8, 0x12ec, 0x12f0, 0x12f4, 0x1300,
                            0x1304, 0x14f4, 0x14f8, 0x14fc, 0x1500, 0x1504, 0x1508] {
                    c.bus.write32(g + off, r.next_u32());
                }
                for off in (0..0x260u32).step_by(4) {
                    c.bus.write32(DATA + 0x4000 + off, r.next_u32());
                }
            },
        );
    }


    #[test]
    fn world_painter_set_script_surface_mode_matches() {
        check(ee::s09::f_002ef368__generated, world_painter_set_script_surface_mode, SEEDS_3, &|c, seed| {
            let mut r = rng(0x002e_f368, seed);
            scramble(c, &mut r);
            c.g[GP][0] = sx(GAME_GP);
            c.bus.write32(0x004a_3b5c, r.next_u32());
            c.bus.write32(0x004a_4444, r.next_u32());
            c.g[A0][0] = match seed % 3 {
                0 => 0,
                1 => sx(DATA),
                _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
            };
        });
    }


    #[test]
    fn ps2_ai_visual_effects_man_create_matches() {
        check(ee::s09::f_002f0390__generated, ps2_ai_visual_effects_man_create, 40, &|c, seed| {
            alloc_machine(c, seed, 0x2f03_90);
        });
    }


    #[test]
    fn ps2_visual_effects_man_create_matches() {
        check(ee::s09::f_002f0510__generated, ps2_visual_effects_man_create, 40, &|c, seed| {
            alloc_machine(c, seed, 0x2f05_10);
        });
    }


    /// A NUL-terminated `s` at `addr`.
    pub(crate) fn put_str(c: &mut Cpu, addr: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }


    // -----------------------------------------------------------------------
    // WorldPainter_SetIrradianceOverride.

    #[test]
    fn world_painter_set_irradiance_override_matches() {
        check(ssx3_ee::s09::f_002ef378__generated, world_painter_set_irradiance_override, 48, &|c, seed| {
            let mut r = machine(c, seed, 0x2ef3_78, 0);
            c.g[GP][0] = GAME_GP as u64;
            if seed % 2 == 0 {
                c.g[A0][0] = 0;
            } else {
                let name = DATA + 0x800;
                match seed % 3 {
                    0 => put_str(c, name, ""),
                    1 => put_str(c, name, "sunset"),
                    _ => {
                        c.bus.write32(name, r.next_u32());
                        c.bus.write8(name + 4, 0);
                    }
                }
                c.g[A0][0] = name as u64;
            }
            c.bus.write32(GAME_GP + 0xa74, r.next_u32());
        });
    }


    #[test]
    fn world_painter_get_irradiance_name2_matches() {
        check(ee::s09::f_002ef198__generated, world_painter_get_irradiance_name2, SEEDS_4, &|c, seed| {
            let mut r = common(c, seed, 0x2ef198, 1);
            let painter = DATA + 0x100;
            let obj = DATA + 0x200;
            let vt = DATA + 0x300;
            clear(c, painter, 0x200);
            clear(c, obj, 0x200);
            clear(c, vt, 0x200);
            c.bus.write32(0x004f_a47c, painter);
            c.bus.write32(painter + 0x1c, obj);
            c.bus.write32(obj + 4, vt);
            vtable_2(c, vt, &[(0x138, 0, FAKE)]);
            c.bus.write32(gp_at(0xa74), ((seed % 2) as u32) * 1);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn ps2_ai_visual_effects_man_ctor_matches() {
        check(
            ssx3_ee::s09::f_002f03c8__generated,
            ps2_ai_visual_effects_man_ctor,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x2f03c8, 0);
                pool(c);
                let m = DATA + 0x1000;
                c.g[A0][0] = m as u64;
                let _ = seed;
            },
        );
    }


    #[test]
    fn ps2ai_visual_effects_man_dtor_matches() {
        check(
            ee::s09::f_002f0438__generated,
            ps2ai_visual_effects_man_dtor,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x2f0438, 3);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0; // no operator delete on a test pointer
                if seed % 3 == 0 {
                    w(c, DATA, 0);
                } else {
                    let obj = DATA + 0x400;
                    w(c, DATA, obj);
                    fake_slot(c, obj, DATA + 0x800, 8);
                }
            },
        );
    }
}
