//! `src/render/lightman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
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
pub(crate) use crate::game::animation::animsequencer::dtor;
pub(crate) use crate::game::bx::bxstring::gp as gp_3;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::splashmanager::oc230_gp;
pub(crate) use crate::game::intersect::aifwddiff::oc230_at;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::render::particle::p;
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
    (0x0038_dee8, light_man_v09, "LightMan_v09"),
    (0x0038_df38, light_man_v10, "LightMan_v10"),
    (0x0038_df98, light_man_v11, "LightMan_v11"),
    (0x0038_e008, light_man_v12, "LightMan_v12"),
    (
        0x0038_f2a8,
        vfx_plants_get_sway_matrix,
        "VfxPlants_GetSwayMatrix",
    ),
    (0x0038_f300, spr_stream_set_mode, "SprStream_SetMode"),
    (0x0038_f738, spr_stream_download, "SprStream_Download"),
    (
        0x0038_f768,
        dma_wait_from_scratchpad,
        "Dma_WaitFromScratchpad",
    ),
    (0x0038_f7b0, dma_wait_to_scratchpad, "Dma_WaitToScratchpad"),
    (
        0x0038_f930,
        tex_man_swizzle_t8_block,
        "TexMan_SwizzleT8Block",
    ),
    (
        0x0039_0458,
        ps2_graphics_man_reset_color_filter,
        "PS2GraphicsMan_ResetColorFilter",
    ),
    (0x0039_0c60, vfx_screen_tint_update, "VfxScreenTint_Update"),
    (
        0x0039_0ec8,
        vfx_screen_tint_trigger_flash,
        "VfxScreenTint_TriggerFlash",
    ),
    (
        0x0039_0ef8,
        vfx_screen_tint_poll_flash_sound,
        "VfxScreenTint_PollFlashSound",
    ),
    (0x0039_12a8, vu0_upload_microcode_a, "Vu0_UploadMicrocodeA"),
    (0x0039_1360, vu0_upload_microcode_b, "Vu0_UploadMicrocodeB"),
    (0x0039_14f8, vu0_read_vi1, "Vu0_ReadVI1"),
    (0x0039_16c0, font_dtor, "Font_dtor"),
    (0x0039_1708, font_load, "Font_Load"),
    (0x0038_ee78, vfx_plants_update, "VfxPlants_Update"),
    (0x0039_0f20, vfx_screen_tint_render, "VfxScreenTint_Render"),
    (
        0x0039_0c20,
        vfx_render_toggles_update,
        "VfxRenderToggles_Update",
    ),
    (
        0x0038_f800,
        vfx_visual_effects_test_render,
        "VfxVisualEffectsTest_Render",
    ),
    (0x0039_1480, vu0_copy_output_mem, "Vu0_CopyOutputMem"),
    (0x0039_1418, vu0_load_regs_and_call, "Vu0_LoadRegsAndCall"),
    (
        0x0038_e0a8,
        render2d_transform_point,
        "Render2D_TransformPoint",
    ),
    (0x0038_dbd0, light_man_create, "LightMan_Create"),
    (0x0038_dc50, light_man_empty_hook_a, "LightMan_EmptyHookA"),
    (0x0038_dc58, light_man_empty_hook_b, "LightMan_EmptyHookB"),
    (0x0038_dc60, light_man_empty_hook_c, "LightMan_EmptyHookC"),
    (0x0038_dc68, light_man_empty_hook_d, "LightMan_EmptyHookD"),
    (0x0038_dc70, light_man_v05, "LightMan_v05"),
    (0x0038_e118, render2d_draw_quad, "Render2D_DrawQuad"),
    (
        0x0038_e200,
        render2d_draw_quad_rot90,
        "Render2D_DrawQuadRot90",
    ),
    (
        0x0038_e2e8,
        render2d_draw_quad_xform,
        "Render2D_DrawQuadXform",
    ),
    (
        0x0038_e468,
        render2d_draw_quad_rot90_xform,
        "Render2D_DrawQuadRot90Xform",
    ),
    (0x0038_e5e8, render2d_draw_quad_uv, "Render2D_DrawQuadUV"),
    (
        0x0038_e738,
        render2d_draw_quad_uv_rot90,
        "Render2D_DrawQuadUVRot90",
    ),
    (
        0x0038_e880,
        render2d_draw_quad_uv_xform,
        "Render2D_DrawQuadUVXform",
    ),
    (
        0x0038_ea60,
        render2d_draw_quad_uv_rot90_xform,
        "Render2D_DrawQuadUVRot90Xform",
    ),
    (
        0x0038_ec40,
        render2d_draw_filled_rect,
        "Render2D_DrawFilledRect",
    ),
    (0x0039_15e8, font_ctor, "Font_ctor"),
    (0x0037_1f98, dma_send_gif_chain, "Dma_SendGifChain"),
];

// ---------------------------------------------------------------------------
// Callees the generator emitted as direct calls (the readable alias, which
// dispatches through the override slot) and as cross-shard `call_indirect`s.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::fe::feasyncfile::callee::LIGHT_MAN_EMPTY_HOOK_D;

    pub(crate) use crate::game::fe::festaterewards::callee::LIGHT_MAN_EMPTY_HOOK;

    pub(crate) use crate::game::intersect::riderspheretree::callee::DMA_FROM_SCRATCHPAD;

    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;

    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;

    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;

    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;

    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2 as SPRINTF;

    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;

    pub const LUNO_TABLE_DTOR: Callee = Callee {
        addr: 0x0022_4df0,
        f: ee::s05::f_00224df0,
    };

    pub const WORLD_GET_FLAG_MANAGER: Callee = Callee {
        addr: 0x002d_1cb0,
        f: ee::s09::World_GetFlagManager,
    };

    pub const WORLD_GET_AI_FRAME_COUNT: Callee = Callee {
        addr: 0x002d_1c98,
        f: ee::s09::World_GetAIFrameCount,
    };

    pub const WORLD_GET_NASH: Callee = Callee {
        addr: 0x002d_1be0,
        f: ee::s09::World_GetNash,
    };

    pub const LIGHT_CONTRIBUTION: Callee = Callee {
        addr: 0x002f_5d30,
        f: ee::s09::WorldLightMan_LightContribution,
    };

    pub const MATH_SIN_COS: Callee = Callee {
        addr: 0x0031_be50,
        f: ee::s10::Math_SinCos,
    };

    pub const SPATIAL_HASH_UPDATE: Callee = Callee {
        addr: 0x0032_91e0,
        f: ee::s10::SpatialHash_Update,
    };

    pub const FLAG_MANAGER_GROUP_COMPUTE_VERTS: Callee = Callee {
        addr: 0x0034_bca0,
        f: ee::s11::cFlagManager_GroupComputeVerts,
    };

    pub const INTERSECT_INFO_EVALUATE: Callee = Callee {
        addr: 0x0035_1538,
        f: ee::s11::IntersectInfo_Evaluate,
    };

    pub const TYPED_PROCESS_SAVE: Callee = Callee {
        addr: 0x0035_4948,
        f: ee::s11::TypedProcess_Save,
    };

    pub const SYNC_PROCESS_SAVE: Callee = Callee {
        addr: 0x0035_4980,
        f: ee::s11::SyncProcess_Save,
    };

    pub const MULTI_SPLINE_MODIFIER_DEBUG_DRAW_PATH: Callee = Callee {
        addr: 0x0034_5430,
        f: ee::s11::MultiSplineModifier_DebugDrawPath,
    };

    pub const EA_MEMSET: Callee = Callee {
        addr: 0x003e_6448,
        f: ee::s14::EA_MemSet,
    };

    pub const DMA_WAIT_FROM_SCRATCHPAD: Callee = Callee {
        addr: 0x0038_f768,
        f: ee::s12::Dma_WaitFromScratchpad,
    };

    pub const DMA_WAIT_TO_SCRATCHPAD: Callee = Callee {
        addr: 0x0038_f7b0,
        f: ee::s12::Dma_WaitToScratchpad,
    };
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;

    /// `void LightMan_EmptyHookB(void*, ...)`: the directional-light hook
    /// (empty).
    pub const LIGHT_MAN_EMPTY_HOOK_B: Callee = Callee {
        addr: 0x0038_dc58,
        f: ee::s12::LightMan_EmptyHookB,
    };

    /// `LightMan_ctor(LightMan*)`: install the base vtable and register the
    /// manager.
    pub const LIGHT_MAN_CTOR: Callee = Callee {
        addr: 0x0038_ae10,
        f: ee::s12::LightMan_ctor,
    };

    /// `LightMan::~LightMan(LightMan*, int flags)`.
    pub const LIGHT_MAN_DTOR: Callee = Callee {
        addr: 0x0038_ae28,
        f: ee::s12::LightMan_dtor,
    };

    /// `float Math_Asin(float)`.
    pub const MATH_ASIN: Callee = Callee {
        addr: 0x0031_c128,
        f: ee::s10::Math_Asin,
    };

    /// `void Dma_SendGifChain(void* tadr, int flags)`: start a chain-mode
    /// GIF DMA at `tadr`.
    pub const DMA_SEND_GIF_CHAIN: Callee = Callee {
        addr: 0x0037_1f98,
        f: ee::s12::Dma_SendGifChain,
    };

    /// `Render2D_TransformPoint(Vec4 *out, const Vec2 *p, const Mat3 *m)`.
    pub(crate) const RENDER2D_TRANSFORM_POINT: Callee = Callee {
        addr: 0x0038_e0a8,
        f: ee::s12::Render2D_TransformPoint,
    };
}

// ---------------------------------------------------------------------------
// VfxScreenTint_Update (0x390c60).

/// The screen-tint animation state (`gp` globals, `D_004A46xx`).
pub(crate) mod tint {

    /// Master enable (`D_004A460C`).
    pub const ENABLED: i32 = 0x151c;

    /// Countdown of the frames left before another flash may start
    /// (`D_004A46B4`).
    pub const COOLDOWN: i32 = 0x15c4;

    /// Flash timer (`D_004A4698`): -1 while idle, then counts up.
    pub const TIMER: i32 = 0x15a8;

    /// Trigger threshold (`D_004A4610`).
    pub const TRIGGER: i32 = 0x1520;

    /// Flash probability factor (`D_004A4614`).
    pub const CHANCE: i32 = 0x1524;

    /// Idle delay (`D_004A4618`).
    pub const D0: i32 = 0x1528;

    /// Second segment (`D_004A461C`).
    pub const D1: i32 = 0x152c;

    /// Third segment (`D_004A4620`).
    pub const D2: i32 = 0x1530;

    /// Fourth segment (`D_004A4624`).
    pub const D3: i32 = 0x1534;
}

// ---------------------------------------------------------------------------
// PS2 graphics manager (colour filter)

/// `void PS2GraphicsMan_ResetColorFilter(PS2GraphicsMan *man, int index)`
/// (0x390458): write the six default colour-filter floats from `gp + 0x14f4`
/// into the record at `man + index*0x18 + 0x6ca4`.
pub fn ps2_graphics_man_reset_color_filter(c: &mut Cpu) {
    c.cycles += 17;
    c.g[V0][0] = 0x18;
    let gp = c.g[GP][0] as u32;
    c.f[1] = Addr(gp.wrapping_add(0x14f4)).read_fpu(c).0;
    c.mult(0, 5, 5, 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6ca4));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let dst = c.g[A0][0] as u32;
    Addr(dst).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = Addr(gp.wrapping_add(0x14f8)).read_fpu(c).0;
    Addr(dst).off(0x4).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = Addr(gp.wrapping_add(0x14fc)).read_fpu(c).0;
    Addr(dst).off(0x8).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = Addr(gp.wrapping_add(0x1500)).read_fpu(c).0;
    Addr(dst).off(0xc).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = Addr(gp.wrapping_add(0x1504)).read_fpu(c).0;
    Addr(dst).off(0x10).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = Addr(gp.wrapping_add(0x1508)).read_fpu(c).0;
    Addr(dst).off(0x14).write_fpu(c, FpuF32(c.f[0]));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Screen tint

/// `void VfxScreenTint_TriggerFlash(float intensity)` (0x390ec8): when the
/// screen tint is active, start a flash: store the scaled intensity and its
/// integer value in the global tint block.
///
/// Tick rate: the thunder delay (`+0x15c4`, ticks counted down by
/// `VfxScreenTint_Update`) is converted from 60 Hz frames with
/// `tick::ticks_from_frames60` (exact at 60).
pub fn vfx_screen_tint_trigger_flash(c: &mut Cpu) {
    c.cycles += 3;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_add(0x151c)).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        let rate = Addr(gp.wrapping_sub(0x2698)).read_fpu(c);
        c.f[0] = rate.0;
        Addr(gp.wrapping_add(0x15a8)).sw(c, 0);
        let scaled = abi::farg(c, 0) * rate;
        c.f[0] = scaled.0;
        Addr(gp.wrapping_add(0x15ac)).sw(c, 0);
        Addr(gp.wrapping_add(0x15c0)).write_fpu(c, abi::farg(c, 0));
        // The thunder delay is a count of 60 Hz frames (distance times the
        // speed of sound per frame): the same wall-clock delay at any rate.
        let mut i = scaled.to_i32();
        if i > 0 {
            i = crate::tick::ticks_from_frames60(c, i as u32) as i32;
        }
        c.f[1] = i as u32;
        Addr(gp.wrapping_add(0x15c4)).write_fpu(c, FpuF32(c.f[1]));
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool VfxScreenTint_PollFlashSound(void)` (0x390ef8): true once while the
/// flash is active and its countdown has reached zero, clearing the countdown.
pub fn vfx_screen_tint_poll_flash_sound(c: &mut Cpu) {
    c.cycles += 3;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_add(0x151c)).lw(c);
    let inactive = c.g[V0][0] == 0;
    c.g[V0][0] = Addr(gp.wrapping_add(0x15c4)).lw(c);
    if inactive {
        // fall through to the false return
    } else {
        c.cycles += 2;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        if c.g[V0][0] != 0 {
            // counting down: still false
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            Addr(gp.wrapping_add(0x15c4)).sw(c, c.g[V1][0]);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// SprStream_Download.

/// `void SprStream_Download(u32 madr, int size, int flags)` (0x38f738): send
/// `size >> 4` qwords from the scratchpad to `madr` through the DMA manager's
/// state object (`gp + 0x2a94`).
pub fn spr_stream_download(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[V0][0] = c.g[A0][0];
        c.g[A3][0] = c.g[A2][0];
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 4) as u32);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a94));
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[V0][0];
        callee::DMA_FROM_SCRATCHPAD.call(c, 0x0038_f758)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DMA.

/// `void Dma_WaitToScratchpad(void)` (0x38f7b0): spin on DMA channel 4's
/// `CHCR` until the transfer to the scratchpad (`0x100`) has completed.
pub fn dma_wait_to_scratchpad(c: &mut Cpu) {
    /// DMA channel 4 control register (the original polls `0x1000d400`).
    const D_CHCR4: u32 = 0x1000_d400;
    /// `CHCR.STR`-side "transfer running" bit the original waits on.
    const BUSY: u64 = 0x100;

    c.cycles += 6;
    c.g[V0][0] = 0x1000_0000;
    c.g[V0][0] |= 0xd400;
    c.g[V1][0] = Addr(D_CHCR4).lw(c);
    c.g[V1][0] &= BUSY;
    let done = c.g[V1][0] == 0;
    c.g[V1][0] = 0x1000_0000;
    if !done {
        c.cycles += 2;
        c.g[V1][0] |= 0xd400;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(D_CHCR4).lw(c);
            c.g[V0][0] &= BUSY;
            if c.g[V0][0] == 0 {
                break;
            }
            if c.cycles >= c.next_event {
                crate::events::deliver_event_here(c, 0x0038_f7d0);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`, 32-bit wrap.
#[inline(always)]
pub(crate) fn off(a: u32, o: u32) -> u32 {
    a.wrapping_add(o)
}

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, o: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(o as u32)
}

/// Loads and stores as methods: a method's receiver borrow is two-phased, so
/// an argument may read `c.g` as the original's address arithmetic does.
pub(crate) trait Oc49Mem {
    fn ldw(&mut self, a: u32) -> u64;
    fn ldh(&mut self, a: u32) -> u64;
    fn ldhu(&mut self, a: u32) -> u64;
    fn stw(&mut self, a: u32, v: u64);
    fn sth(&mut self, a: u32, v: u64);
}

impl Oc49Mem for Cpu {
    #[inline(always)]
    fn ldw(&mut self, a: u32) -> u64 {
        Addr(a).lw(self)
    }
    #[inline(always)]
    fn ldh(&mut self, a: u32) -> u64 {
        Addr(a).lh(self)
    }
    #[inline(always)]
    fn ldhu(&mut self, a: u32) -> u64 {
        Addr(a).lhu(self)
    }
    #[inline(always)]
    fn stw(&mut self, a: u32, v: u64) {
        Addr(a).sw(self, v);
    }
    #[inline(always)]
    fn sth(&mut self, a: u32, v: u64) {
        Addr(a).sh(self, v);
    }
}

/// `addu`: 32-bit add, sign-extended into the 64-bit register.
#[inline(always)]
pub(crate) fn addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// `int SprStream_SetMode(SprStream *self, int mode)` (0x0038f300): set the
/// ring-buffer mode, its element count/stride and the buffer addresses, then
/// flush the scratchpad DMA. The buffer loop is re-enterable at
/// `0x0038f3e8` (the trampoline resumes a loop whose event asked to unwind).
pub fn spr_stream_set_mode(c: &mut Cpu) {
    guest(c, |c| {
        let resumed = c.pc == 0x0038_f3e8;
        if !resumed && c.pc != 0x0038_f300 {
            c.jump_out(c.pc, 0x0038_f300, 0x0038_f460);
            return Ok(());
        }
        let frame = if resumed {
            Frame { size: 0x10 }
        } else {
            Frame::open(c, 0x10)
        };
        let mut run_loop = resumed;
        if !resumed {
            c.cycles += 6;
            c.g[A2][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            c.g[V0][0] = c.ldw(c.g[A2][0] as u32);
            let same = c.g[V0][0] == c.g[A1][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            if same {
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 5;
            c.stw(c.g[A2][0] as u32, c.g[A1][0]);
            c.g[A1][0] = c.g[V0][0];
            let in_range = c.g[A1][0] < 7;
            c.g[V0][0] = in_range as u64;
            c.g[V0][0] = 0x49_0000;
            if in_range {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx(0x49_0000u32.wrapping_add(0x2ef0));
                c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
                c.g[A0][0] = c.ldw(c.g[V1][0] as u32);
                let target = c.g[A0][0] as u32;
                match target {
                    0x0038_f344 => {
                        c.cycles += 2;
                        c.g[V0][0] = 1;
                    }
                    0x0038_f34c | 0x0038_f354 | 0x0038_f35c | 0x0038_f364 => {
                        c.cycles += 2;
                        c.g[V0][0] = 2;
                    }
                    0x0038_f36c => {
                        c.cycles += 1;
                        c.g[V0][0] = 3;
                    }
                    _ => {
                        c.jump_out(target, 0x0038_f300, 0x0038_f460);
                        return Ok(());
                    }
                }
                c.cycles += 1;
                c.stw(off(c.g[A2][0] as u32, 4), c.g[V0][0]);
            }
            c.cycles += 4;
            c.stw(off(c.g[A2][0] as u32, 8), 0);
            let stride_range = c.g[A1][0] < 7;
            c.g[V0][0] = stride_range as u64;
            c.stw(off(c.g[A2][0] as u32, 0xc), 0);
            if stride_range {
                c.cycles += 7;
                c.g[V0][0] = 0x49_0000;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx(0x49_0000u32.wrapping_add(0x2f10));
                c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
                c.g[A0][0] = c.ldw(c.g[V1][0] as u32);
                let target = c.g[A0][0] as u32;
                // Each handler block has its own cost (0x38f3c8 is the
                // fall-through tail and costs one cycle, the rest two).
                let (handler, stride) = match target {
                    0x0038_f3a0 => (2u64, 0x400u64),
                    0x0038_f3a8 => (2, 0x200),
                    0x0038_f3b0 => (2, 0x1c0),
                    0x0038_f3b8 => (2, 0x180),
                    0x0038_f3c0 => (2, 0x140),
                    0x0038_f3c8 => (1, 0x100),
                    _ => {
                        c.jump_out(target, 0x0038_f300, 0x0038_f460);
                        return Ok(());
                    }
                };
                c.cycles += handler;
                c.cycles += 1;
                c.g[V0][0] = stride;
                c.stw(off(c.g[A2][0] as u32, 0x10), c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 4));
            let no_buffers = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = 0;
            if !no_buffers {
                c.cycles += 3;
                c.g[A1][0] = 0x7000_0000;
                c.g[V1][0] = sx(off(c.g[A2][0] as u32, 0x2c));
                run_loop = true;
            }
        }
        if run_loop {
            if spr_stream_buffer_loop(c).is_err() {
                return Ok(());
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.ldw(off(c.g[A2][0] as u32, 4));
        c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 0x10));
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = ((c.g[V0][0] as i64) < 1024) as u64;
        let fits = c.g[V1][0] == 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        if fits {
            c.cycles += 1;
            c.stw(gp(c, 0x2a94), 0);
        } else {
            c.cycles += 4;
            c.g[V1][0] = 0x7000_0000;
            c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
            c.stw(gp(c, 0x2a94), c.g[V0][0]);
        }
        c.cycles += 4;
        callee::DMA_WAIT_FROM_SCRATCHPAD.call(c, 0x0038_f44c)?;
        callee::DMA_WAIT_TO_SCRATCHPAD.call(c, 0x0038_f454)?;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `0x38f3e8` buffer-fill loop of `SprStream_SetMode`: writes
/// `0x70000000 + stride*i*16` and clears the per-entry words until
/// `i == count`. Returns `Ok` when the loop completed (the caller continues
/// at `0x38f418`) and `Err` when an event asked the thread to unwind.
pub(crate) fn spr_stream_buffer_loop(c: &mut Cpu) -> Flow {
    loop {
        c.cycles += 12;
        c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 0x10));
        c.stw((c.g[V1][0] as u32).wrapping_sub(0xc), 0);
        c.mult(0, V0, V0, A0);
        c.stw(c.g[V1][0] as u32, 0);
        c.g[A0][0] = sx(off(c.g[A0][0] as u32, 1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = addu(c.g[V0][0], c.g[A1][0]);
        c.stw((c.g[V1][0] as u32).wrapping_sub(0x18), c.g[V0][0]);
        c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 4));
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx(off(c.g[V1][0] as u32, 4));
        if !more {
            return Ok(());
        }
        if back_edge_unwinds(c, 0x0038_f3e8) {
            return Err(crate::call::Unwound);
        }
    }
}

// ---------------------------------------------------------------------------
// DMA.

/// `void Dma_WaitFromScratchpad(void)` (0x38f768): spin until the DMA
/// controller's "from scratchpad" channel (D_STAT bit 8) is idle.
pub fn dma_wait_from_scratchpad(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x1000_0000;
    c.g[V0][0] = c.g[V0][0] | 0xd000;
    c.g[V1][0] = c.lw(r32(c, V0));
    c.g[V1][0] = c.g[V1][0] & 0x100;
    let idle = c.g[V1][0] == 0;
    c.g[V1][0] = 0x1000_0000;
    if !idle {
        c.cycles += 2;
        c.g[V1][0] = c.g[V1][0] | 0xd000;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[V0][0] = c.g[V0][0] & 0x100;
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0038_f788);
            }
        }
    }
    c.cycles += 2;
    c.pc = c.g[RA][0] as u32;
}

/// `gp_2 + off` (a gp_2-relative global) as a guest address.
#[inline(always)]
pub(crate) fn gp_2(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `rd = rd + k` on a 32-bit register value (`addiu`).
#[inline(always)]
pub(crate) fn addiu(c: &mut Cpu, rd: usize, k: u32) {
    let v = (c.g[rd][0] as u32).wrapping_add(k);
    c.g[rd][0] = sx(v);
}

/// The frame after the prologue (the part skipped when `ENABLED` is 0).
/// Returns true when it already reloaded `s0` (the original's early return
/// from the flash path, which skips the tail's `lq s0`).
pub(crate) fn tint_body(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    use tint::*;
    // Per-frame cooldown.
    c.cycles += 3;
    c.g[V0][0] = c.lw(gp_2(c, COOLDOWN));
    let cooled = (c.g[V0][0] as i64) <= 0;
    addiu(c, V0, 0xffff_ffff);
    if !cooled {
        c.cycles += 1;
        c.sw(gp_2(c, COOLDOWN), c.g[V0][0]);
    }

    // A running flash: pick the next one at random, else leave the timer
    // counting up to the trigger.
    c.cycles += 3;
    c.g[A3][0] = c.lw(gp_2(c, TIMER));
    let idle = (c.g[A3][0] as i64) >= 0;
    c.g[A2][0] = c.lw(gp_2(c, TRIGGER));
    if !idle {
        // `RandF() < chance^2` decides whether a flash fires now.
        c.cycles += 16;
        c.g[S0][0] = 0x007f_0000;
        call_ptr(c, 0x0031_77f0, 0x0039_0ca0)?;
        c.g[S0][0] |= 0xffff;
        c.g[S1][0] = 0x3f80_0000;
        let mask = c.g[S0][0];
        c.g[V0][0] &= mask;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20); // f20 = 1.0
        let exp = c.g[S1][0];
        c.g[V0][0] |= exp;
        c.lwc1(0, gp_2(c, CHANCE));
        c.mtc1(2, 2);
        c.mul_s(0, 0, 0);
        // The chance is per 60 Hz frame: at rate r roll with
        // 1 - (1 - p)^(60/r) per tick (the same strikes per second).
        let hz = crate::tick::rate(c);
        if hz != 60 {
            let p = (f32::from_bits(c.f[0]) as f64).clamp(0.0, 1.0);
            c.f[0] = ((1.0 - (1.0 - p).powf(60.0 / hz as f64)) as f32).to_bits();
        }
        c.sub_s(1, 2, 20);
        c.c_lt_s(1, 0);
        if !c.fcc() {
            frame.restore(c, S0, 0x20);
            return Ok(true);
        }
        // Flash: duration `(rand - 1) * K0 + K1` scaled by K2.
        c.cycles += 15;
        call_ptr(c, 0x0031_77f0, 0x0039_0ce0)?;
        let mask = c.g[S0][0];
        c.g[V0][0] &= mask;
        c.lwc1(0, gp_2(c, -0x26a4));
        let exp = c.g[S1][0];
        c.g[V0][0] |= exp;
        c.lwc1(2, gp_2(c, -0x26a0));
        c.mtc1(2, 3);
        c.lwc1(1, gp_2(c, -0x269c));
        c.sub_s(12, 3, 20);
        c.mul_s(12, 12, 0);
        c.add_s(12, 12, 2);
        c.mul_s(12, 12, 1);
        Callee {
            addr: 0x0039_0ec8,
            f: ssx3_ee::s12::VfxScreenTint_TriggerFlash,
        }
        .call(c, 0x0039_0d0c)?;
        frame.restore(c, S0, 0x20);
        return Ok(true);
    }

    // The flash timer: count up to the trigger or clamp to it.
    c.cycles += 2;
    let trig_neg = (c.g[A2][0] as i64) < 0;
    c.g[V0][0] = c.lw(gp_2(c, D0));
    let advance;
    if trig_neg {
        advance = true;
    } else {
        c.cycles += 9;
        c.g[V1][0] = c.lw(gp_2(c, D1));
        c.g[A1][0] = c.lw(gp_2(c, D2));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.lw(gp_2(c, D3));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.sw(gp_2(c, TIMER), c.g[A2][0]);
            advance = false;
        } else {
            advance = true;
        }
    }
    if advance {
        c.cycles += 2;
        // The four segment lengths are 60 Hz frames: the flash timer steps
        // on its own 60 Hz clock (every tick at 60).
        let t = c.g[A3][0] as u32;
        let key = gp_2(c, TIMER);
        if crate::tick::frame60_due(c, key, t) {
            c.g[V0][0] = sx(t.wrapping_add(1));
            c.sw(gp_2(c, TIMER), c.g[V0][0]);
            crate::tick::frame60_ran(c, key, t.wrapping_add(1));
        }
    }

    // The tint level and colours for the current segment of the flash.
    c.cycles += 5;
    c.g[V1][0] = c.lw(gp_2(c, TIMER));
    c.g[A1][0] = c.lw(gp_2(c, D0));
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let past_first = c.g[V0][0] != 0;
    c.g[T0][0] = c.lw(gp_2(c, D1));
    if !past_first {
        // Ramp up over the first segment: level = (timer / d0) * K.
        c.cycles += 17;
        c.mtc1(3, 0);
        c.cvt_s_w(0, 0);
        c.lwc1(5, gp_2(c, 0x1538));
        c.mtc1(5, 2);
        c.cvt_s_w(2, 2);
        c.lwc1(3, gp_2(c, 0x1544));
        c.lwc1(4, gp_2(c, 0x1548));
        c.g[V0][0] = 5;
        c.lwc1(1, gp_2(c, 0x154c));
        c.div_s(0, 0, 2);
        c.swc1(3, gp_2(c, 0x15b0));
        c.swc1(4, gp_2(c, 0x15b4));
        c.swc1(1, gp_2(c, 0x15b8));
        c.sw(gp_2(c, 0x15bc), c.g[V0][0]);
        c.mul_s(0, 0, 5);
        c.swc1(0, gp_2(c, 0x15ac));
        return Ok(false);
    } else {
        // Second segment: its peak colours.
        c.cycles += 4;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let third = c.g[V0][0] != 0;
        c.g[A2][0] = c.lw(gp_2(c, D2));
        if !third {
            c.cycles += 6;
            c.lwc1(3, gp_2(c, 0x1538));
            c.g[V0][0] = 5;
            c.lwc1(0, gp_2(c, 0x1544));
            c.lwc1(1, gp_2(c, 0x1548));
            c.lwc1(2, gp_2(c, 0x154c));
        } else {
            // Third segment.
            c.cycles += 4;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let tail = c.g[V0][0] != 0;
            c.g[A3][0] = c.lw(gp_2(c, D3));
            if !tail {
                // Third segment: its peak colours.
                c.cycles += 5;
                c.lwc1(3, gp_2(c, 0x153c));
                c.g[V0][0] = 5;
                c.lwc1(0, gp_2(c, 0x1550));
                c.lwc1(1, gp_2(c, 0x1554));
                c.lwc1(2, gp_2(c, 0x1558));
            } else {
                // Past the third segment: the fourth, or back to idle.
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let past_end = c.g[V0][0] != 0;
                c.lwc1(2, gp_2(c, 0x14f4));
                if !past_end {
                    // Last segment: level ramps down from 1.
                    c.cycles += 29;
                    c.mtc1(5, 2);
                    c.cvt_s_w(2, 2);
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 1);
                    c.mtc1(3, 0);
                    c.cvt_s_w(0, 0);
                    c.lwc1(8, gp_2(c, 0x1540));
                    c.mtc1(8, 6);
                    c.cvt_s_w(6, 6);
                    c.lwc1(4, gp_2(c, 0x155c));
                    c.mtc1(6, 7);
                    c.cvt_s_w(7, 7);
                    c.lwc1(5, gp_2(c, 0x1560));
                    c.sub_s(0, 0, 2);
                    c.lwc1(3, gp_2(c, 0x1564));
                    c.mtc1(7, 2);
                    c.cvt_s_w(2, 2);
                    c.g[V0][0] = 5;
                    c.swc1(4, gp_2(c, 0x15b0));
                    c.sub_s(0, 0, 6);
                    c.swc1(5, gp_2(c, 0x15b4));
                    c.swc1(3, gp_2(c, 0x15b8));
                    c.sw(gp_2(c, 0x15bc), c.g[V0][0]);
                    c.sub_s(0, 0, 7);
                    c.div_s(0, 0, 2);
                    c.sub_s(1, 1, 0);
                    c.mul_s(1, 1, 8);
                    c.swc1(1, gp_2(c, 0x15ac));
                } else {
                    c.cycles += 10;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    c.lwc1(1, gp_2(c, 0x14f8));
                    c.g[V1][0] = 5;
                    c.lwc1(0, gp_2(c, 0x14fc));
                    c.sw(gp_2(c, TIMER), c.g[V0][0]);
                    c.swc1(2, gp_2(c, 0x15b0));
                    c.swc1(1, gp_2(c, 0x15b4));
                    c.swc1(0, gp_2(c, 0x15b8));
                    c.sw(gp_2(c, 0x15bc), c.g[V1][0]);
                    c.sw(gp_2(c, 0x15ac), 0);
                }
                return Ok(false);
            }
        }
    }

    // Segment colours written for the second, third and fourth segments.
    c.cycles += 6;
    c.swc1(3, gp_2(c, 0x15ac));
    c.swc1(0, gp_2(c, 0x15b0));
    c.swc1(1, gp_2(c, 0x15b4));
    c.swc1(2, gp_2(c, 0x15b8));
    c.sw(gp_2(c, 0x15bc), c.g[V0][0]);
    Ok(false)
}

/// `void func_00390C60(void)` (0x390c60): animate the light manager's screen
/// tint. With `D_004A460C` set, tick the cooldown, and every frame either
/// roll for a new flash (`RandF() < D_004A4614^2`, with a random duration and
/// colour) or step the running flash through its four segments, writing the
/// level (+0x15ac) and colours (+0x15b0..+0x15bc).
///
/// Tick rate: the flash timer advances on a 60 Hz clock
/// (`tick::frame60_due`, keyed by the timer's address), the per-tick strike
/// chance is `1 - (1 - p)^(60/rate)`, and the thunder countdown (+0x15c4)
/// counts the ticks `VfxScreenTint_TriggerFlash` converted. Exact at 60.
///
/// Frame 0x40: s0 at +0x20, s1 at +0x10, ra at 0, f20 at +0x30.
pub fn vfx_screen_tint_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw(gp_2(c, tint::ENABLED));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.swc1(20, frame.at(c, 0x30));
        let reloaded = if c.g[V0][0] != 0 {
            tint_body(c, frame)?
        } else {
            false
        };
        if !reloaded {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `base + off` on a guest pointer (the `lw`/`sw` offsets).
#[inline(always)]
pub(crate) fn at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `int Vu0_ReadVI1(void)` (0x3914f8): VU0's VI1 register (`cfc2 $vi1`).
pub fn vu0_read_vi1(c: &mut Cpu) {
    c.cycles += 3;
    c.cfc2(V0, 1);
    abi::ret(c);
}

/// `void Font_dtor(Font* f, int flags)` (0x3916c0): free the font and, when
/// `flags & 1`, the object.
pub fn font_dtor(c: &mut Cpu) {
    guest(c, |c| dtor(c, callee::FONT_FREE, 0x0039_16dc, 0x0039_16f0))
}

/// `void LightMan_v09(void* obj, int a, int arg, float f)` (0x38dee8): resolve
/// the object's light hook and forward the call.
pub fn light_man_v09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x20);
        c.mov_s(20, 12);
        c.g[A2][0] = Addr(this).lw(c);
        c.g[V1][0] = Addr(r32(c, A2)).off(0x10).lh(c);
        c.g[V0][0] = Addr(r32(c, A2)).off(0x14).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, V1)));
        call_ptr(c, target, 0x0038_df14)?;
        c.g[A1][0] = c.g[S0][0];
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[V0][0];
        callee::LIGHT_MAN_EMPTY_HOOK_A.call(c, 0x0038_df24)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `Matrix* VfxPlants_GetSwayMatrix(const Vec2* pos)` (0x38f2a8): one of eight
/// sway matrices chosen from the plant's distance to the origin.
pub fn vfx_plants_get_sway_matrix(c: &mut Cpu) {
    c.cycles += 22;
    let a0 = r32(c, A0);
    c.lwc1(1, p(a0, 0x4));
    c.g[A1][0] = u64::MAX;
    c.lwc1(0, p(a0, 0x0));
    c.mul_s(1, 1, 1);
    c.lwc1(2, gp_3(c, 0x1368));
    c.mul_s(0, 0, 0);
    c.g[A0][0] = 0x0050_0000;
    c.g[A0][0] = sx(0x0050_4d80);
    c.add_s(0, 0, 1);
    c.sqrt_s(0, 0);
    c.mul_s(2, 2, 0);
    c.cvt_w_s(0, 2);
    c.mfc1(2, 0);
    c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[V1][0] = sx(r32(c, V0).wrapping_add(7));
    if c.g[A1][0] != 0 {
        c.g[V1][0] = c.g[V0][0];
    }
    c.g[V1][0] = sx(((r32(c, V1) as i32) >> 3) as u32);
    c.g[V1][0] = sx(r32(c, V1) << 3);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
    c.g[V0][0] = sx(r32(c, V0) << 6);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    abi::ret(c);
}

/// `void LightMan_v10(LightMan* man, void* obj, int a2, int a3)` (0x38df38):
/// resolve the object's light hook and run it.
pub fn light_man_v10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let a3 = c.g[A3][0];
        let a2 = c.g[A2][0];
        c.g[S0][0] = a3;
        frame.save_ra(c, 0);
        c.g[S1][0] = a2;
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(20, 12);
        c.g[A2][0] = Addr(r32_2(c, A0)).lw(c);
        c.g[V1][0] = Addr(r32_2(c, A2)).off(0x10).lh(c);
        c.g[V0][0] = Addr(r32_2(c, A2)).off(0x14).lw(c);
        let target = r32_2(c, V0);
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[V1][0]);
        call_ptr(c, target, 0x0038_df6c)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0038_dc58, 0x0038_df80)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void LightMan_v11(void* obj, int b, int c, int d, float f)` (0x38df98):
/// look up the light manager's virtual method and hand the arguments on.
pub fn light_man_v11(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A3][0];
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x40);
        c.mov_s(20, 12);
        c.g[A2][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[V1][0] = Addr(p_2(c.g[A2][0] as u32, 0x10)).lh(c);
        c.g[V0][0] = Addr(p_2(c.g[A2][0] as u32, 0x14)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[V1][0]);
        call::call_ptr(c, target, 0x0038_dfd4)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S0][0];
        c.mov_s(12, 20);
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        callee::LIGHT_MAN_EMPTY_HOOK.call(c, 0x0038_dfec)?;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    });
}

/// `void Font_Load(font, name, flags)` (0x00391708): load the font file and
/// parse its glyphs, freeing the file image afterwards.
pub fn font_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[A1][0] = sx(0x490000);
        c.g[A2][0] = c.g[S0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x490000u32.wrapping_add(0x2f30));
        c.g[A0][0] = sx(word(c, S1).wrapping_add(0x64));
        callee::SPRINTF.call(c, 0x0039_173c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0] ^ 0x100;
        callee::FILE_LOAD.call(c, 0x0039_1748)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        callee::FONT_PARSE_GLYPHS.call(c, 0x0039_175c)?;
        if c.g[S0][0] == 0 {
            c.lq(16, frame.at(c, 0x30));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::MEM_FREE_2.call(c, 0x0039_176c)?;
            c.lq(16, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Texture swizzling.

/// `int TexMan_SwizzleT8Block(void* unused, u8* src, u8* dst)` (0x38f930):
/// reorder a 16x16 T8 block through the swizzle table at `D_0044BDE0`.
pub fn tex_man_swizzle_t8_block(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x0045_0000;
    c.g[T3][0] = 0;
    c.g[T6][0] = sx(r32(c, V0).wrapping_sub(0x4220));
    c.g[V1][0] = 0;
    'outer: loop {
        c.cycles += 6;
        c.g[V0][0] = c.g[V1][0] & 1;
        c.g[T5][0] = sx(r32(c, V1).wrapping_add(1));
        c.g[T1][0] = sx(r32(c, V0) << 6);
        c.g[V1][0] = 0;
        c.g[T4][0] = sx(r32(c, A1).wrapping_add(0x40));
        c.g[V0][0] = sx(r32(c, T1) << 2);
        'middle: loop {
            c.cycles += 4;
            c.g[T2][0] = sx(r32(c, V1).wrapping_add(1));
            c.g[A3][0] = sx(r32(c, V0).wrapping_add(r32(c, T6)));
            c.g[T0][0] = 0;
            'inner: loop {
                c.cycles += 11;
                c.g[V0][0] = Addr(r32(c, A3)).lw(c);
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, T3)));
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
                c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                c.g[T3][0] = sx(r32(c, T3).wrapping_add(1));
                c.g[V0][0] = (c.g[T0][0] < 4u64) as u64;
                let more = c.g[V0][0] != 0;
                Addr(r32(c, A0)).sb(c, c.g[V1][0]);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0038_f968);
                    }
                    continue 'inner;
                }
                c.cycles += 4;
                c.g[V1][0] = c.g[T2][0];
                c.g[V0][0] = (c.g[V1][0] < 0x10u64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, T1) << 2);
                if !more {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[T5][0];
                    c.g[V0][0] = (c.g[V1][0] < 4u64) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[T4][0];
                    if !more {
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        abi::ret(c);
                        return;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0038_f940);
                    }
                    continue 'outer;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0038_f958);
                }
                continue 'middle;
            }
        }
    }
}

/// `void LightMan_v12(obj, a, b, c, d, e, f12, f13, f14)` (0x38e008): call the
/// object's hook with the eight parameters, then the default empty hook.
pub fn light_man_v12(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[T1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        frame.save_fpr(c, 22, 0x60);
        frame.save_fpr(c, 21, 0x58);
        c.mov_s(22, 12);
        frame.save_fpr(c, 20, 0x50);
        c.mov_s(21, 13);
        c.mov_s(20, 14);
        c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
        c.g[V0][0] = lw_2(c, V1, 0x14);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, target, 0x0038_e05c)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.mov_s(12, 22);
        c.mov_s(13, 21);
        c.g[T0][0] = c.g[S0][0];
        c.mov_s(14, 20);
        c.g[A0][0] = c.g[V0][0];
        callee::LIGHT_MAN_EMPTY_HOOK_D.call(c, 0x0038_e080)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x60);
        frame.restore_fpr(c, 21, 0x58);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// VU0 microcode upload.

/// `void Vu0_UploadMicrocodeA(void)` (0x3912a8): wait for the VU0 micro
/// program to stop, install the microcode entry point at `0x43bd00`, start it
/// and flush the instruction cache.
pub fn vu0_upload_microcode_a(c: &mut Cpu) {
    guest(c, |c| {
        'first: {
            c.cycles += 8;
            c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
            c.g[V0][0] = sx(0x1000_8000);
            c.sd(r(c, SP), c.g[RA][0]);
            c.g[V1][0] = Addr(r(c, V0)).lw(c);
            c.g[V1][0] = c.g[V1][0] & 0x100;
            let idle = c.g[V1][0] == 0;
            c.g[V1][0] = sx(0x1000_0000);
            if idle {
                break 'first;
            }
            c.cycles += 2;
            c.g[V1][0] = sx(0x1000_8000);
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                let busy = c.g[V0][0] != 0;
                if !busy {
                    break 'first;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_12d0);
                }
            }
        }
        'second: {
            c.cycles += 16;
            c.g[A0][0] = sx(0x1000_8030);
            c.g[V0][0] = sx(0x0043_bd00);
            c.g[V1][0] = sx(0x1000_0000);
            Addr(r(c, A0)).sw(c, c.g[V0][0]);
            c.g[V1][0] = sx(0x1000_8020);
            c.g[A0][0] = sx(0x1000_0000);
            Addr(r(c, V1)).sw(c, 0);
            c.g[A0][0] = sx(0x1000_8000);
            c.g[V1][0] = 0x104;
            Addr(r(c, A0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(r(c, A0)).lw(c);
            c.g[V0][0] = c.g[V0][0] & 0x100;
            let idle = c.g[V0][0] == 0;
            c.g[V1][0] = sx(0x1000_0000);
            if idle {
                break 'second;
            }
            c.cycles += 1;
            c.g[V1][0] = sx(0x1000_8000);
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                let busy = c.g[V0][0] != 0;
                if !busy {
                    break 'second;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_1330);
                }
            }
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0039_1354);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0042_4020, 0x0039_1354)?;
        c.g[RA][0] = c.ld(r(c, SP));
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `void Vu0_UploadMicrocodeB(void)` (0x391360): the second half of the VU0
/// upload — install the entry point at `0x43cca0` and start the program.
pub fn vu0_upload_microcode_b(c: &mut Cpu) {
    guest(c, |c| {
        'first: {
            c.cycles += 8;
            c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
            c.g[V0][0] = sx(0x1000_8000);
            c.sd(r(c, SP), c.g[RA][0]);
            c.g[V1][0] = Addr(r(c, V0)).lw(c);
            c.g[V1][0] = c.g[V1][0] & 0x100;
            let idle = c.g[V1][0] == 0;
            c.g[V1][0] = sx(0x1000_0000);
            if idle {
                break 'first;
            }
            c.cycles += 2;
            c.g[V1][0] = sx(0x1000_8000);
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                let busy = c.g[V0][0] != 0;
                if !busy {
                    break 'first;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_1388);
                }
            }
        }
        'second: {
            c.cycles += 16;
            c.g[A0][0] = sx(0x1000_8030);
            c.g[V0][0] = sx(0x0043_cca0);
            c.g[V1][0] = sx(0x1000_0000);
            Addr(r(c, A0)).sw(c, c.g[V0][0]);
            c.g[V1][0] = sx(0x1000_8020);
            c.g[A0][0] = sx(0x1000_0000);
            Addr(r(c, V1)).sw(c, 0);
            c.g[A0][0] = sx(0x1000_8000);
            c.g[V1][0] = 0x104;
            Addr(r(c, A0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(r(c, A0)).lw(c);
            c.g[V0][0] = c.g[V0][0] & 0x100;
            let idle = c.g[V0][0] == 0;
            c.g[V1][0] = sx(0x1000_0000);
            if idle {
                break 'second;
            }
            c.cycles += 1;
            c.g[V1][0] = sx(0x1000_8000);
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                let busy = c.g[V0][0] != 0;
                if !busy {
                    break 'second;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_13e8);
                }
            }
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0039_140c);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0042_4020, 0x0039_140c)?;
        c.g[RA][0] = c.ld(r(c, SP));
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Font.

/// `Font* Font_ctor(Font* self, char* name, int flags)` (0x3915e8): set the
/// default UV transforms and colours, then load the font.
pub fn font_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x40));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.sq(S0, p(r32(c, SP), 0x30));
        c.sw(r32(c, SP), 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p(r32(c, SP), 4)).sw(c, 0);
        c.sd(p(r32(c, SP), 0x20), c.g[RA][0]);
        Addr(p(r32(c, S0), 8)).sw(c, 0);
        c.ldl(2, p(r32(c, SP), 7));
        c.ldr(2, r32(c, SP));
        c.sdl(2, p(r32(c, S0), 0x2f));
        c.sdr(2, p(r32(c, S0), 0x28));
        c.swc1(0, r32(c, SP));
        c.swc1(0, p(r32(c, SP), 4));
        c.ldl(2, p(r32(c, SP), 7));
        c.ldr(2, r32(c, SP));
        c.sdl(2, p(r32(c, S0), 0x37));
        c.sdr(2, p(r32(c, S0), 0x30));
        c.swc1(0, p(r32(c, SP), 0x10));
        c.swc1(0, p(r32(c, SP), 0x14));
        c.swc1(0, p(r32(c, SP), 0x18));
        c.swc1(0, p(r32(c, SP), 0x1c));
        c.ldl(2, p(r32(c, S0), 0x37));
        c.ldr(2, p(r32(c, S0), 0x30));
        c.sdl(2, p(r32(c, S0), 0x3f));
        c.sdr(2, p(r32(c, S0), 0x38));
        c.ldl(2, p(r32(c, SP), 0x17));
        c.ldr(2, p(r32(c, SP), 0x10));
        c.ldl(3, p(r32(c, SP), 0x1f));
        c.ldr(3, p(r32(c, SP), 0x18));
        c.sdl(2, p(r32(c, S0), 0x47));
        c.sdr(2, p(r32(c, S0), 0x40));
        c.sdl(3, p(r32(c, S0), 0x4f));
        c.sdr(3, p(r32(c, S0), 0x48));
        c.swc1(0, r32(c, SP));
        Addr(p(r32(c, SP), 4)).sw(c, 0);
        Addr(p(r32(c, SP), 8)).sw(c, 0);
        Addr(p(r32(c, SP), 0xc)).sw(c, 0);
        c.ldl(2, p(r32(c, SP), 7));
        c.ldr(2, r32(c, SP));
        c.ldl(3, p(r32(c, SP), 0xf));
        c.ldr(3, p(r32(c, SP), 8));
        c.sdl(2, p(r32(c, S0), 0x57));
        c.sdr(2, p(r32(c, S0), 0x50));
        c.sdl(3, p(r32(c, S0), 0x5f));
        c.sdr(3, p(r32(c, S0), 0x58));
        Addr(p(r32(c, S0), 0x64)).sb(c, 0);
        call_ptr(c, 0x0039_1708, 0x0039_16ac)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(p(r32(c, SP), 0x20));
        c.lq(S0, p(r32(c, SP), 0x30));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

pub(crate) mod plants {

    /// The light/effects manager whose vtable slot gives the clock.
    pub const GP_MANAGER: u32 = 0x2a90;

    /// Its vtable pointer and the clock method's slot.
    pub const MANAGER_VTABLE: u32 = 0x10d8;

    pub const SLOT_CLOCK: u32 = 0x390;

    /// `1/600`: clock ticks to cycles (`gp - 0x26ac`).
    pub const GP_NEG_TIME_SCALE: u32 = 0x26ac;

    /// Degrees to radians (`gp - 0x26a8`).
    pub const GP_NEG_DEG2RAD: u32 = 0x26a8;

    /// Tweakables (`D_004A4448..`): sway amplitude scale/offset, wobble
    /// amplitude, twist amplitude, the four speeds (ints), the sway and
    /// wobble directions (degrees).
    pub const GP_SWAY_AMP: u32 = 0x1358;

    pub const GP_SWAY_BASE: u32 = 0x135c;

    pub const GP_WOBBLE_AMP: u32 = 0x1360;

    pub const GP_TWIST_AMP: u32 = 0x1364;

    pub const GP_SWAY_SPEED: u32 = 0x136c;

    pub const GP_TWIST_SPEED: u32 = 0x1370;

    pub const GP_WOBBLE_SPEED: u32 = 0x1374;

    pub const GP_WOBBLE_PHASE: u32 = 0x1378;

    pub const GP_SWAY_DIR: u32 = 0x137c;

    pub const GP_WOBBLE_DIR: u32 = 0x1380;

    /// The twist axis (`D_004FF160`) and the base matrix (`D_004FF1A0`).
    pub const AXIS: u32 = 0x004f_f160;

    pub const BASE: u32 = 0x004f_f1a0;

    /// The eight output matrices (`D_00504D80`, 64 bytes each).
    pub const CELLS: u32 = 0x0050_4d80;

    pub const SIN: u32 = 0x0031_bf60;

    pub const SIN_COS: u32 = 0x0031_be50;

    pub const COS: u32 = 0x0031_c040;

    pub const HEAD: u32 = 0x0038_ef20;
}

// ---------------------------------------------------------------------------
// VfxPlants_Update (0x38ee78)

/// `lqc2 vf1..vf4, src; sqc2 vf1..vf4, dst` (a 64-byte matrix copy).
#[inline(always)]
pub(crate) fn copy_mat(c: &mut Cpu, src: u32, dst: u32) {
    for k in 0..4u32 {
        c.lqc2(1 + k as usize, src.wrapping_add(0x10 * k));
    }
    for k in 0..4u32 {
        c.sqc2(1 + k as usize, dst.wrapping_add(0x10 * k));
    }
}

/// `dst = a * b` with the VU0 macro ops (vf4-7 = a, vf8-11 = b, vf12-15 =
/// the result columns).
#[inline(always)]
pub(crate) fn mul_mat(c: &mut Cpu, a: u32, b: u32, dst: u32) {
    for k in 0..4u32 {
        c.lqc2(4 + k as usize, a.wrapping_add(0x10 * k));
    }
    for k in 0..4u32 {
        c.lqc2(8 + k as usize, b.wrapping_add(0x10 * k));
    }
    for k in 0..4u8 {
        c.vu0().mula(0b1111, 4, Src::Bc(8 + k, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8 + k, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8 + k, Z));
        c.vu0().madd(0b1111, 12 + k, 7, Src::Bc(8 + k, W));
    }
    for k in 0..4u32 {
        c.sqc2(12 + k as usize, dst.wrapping_add(0x10 * k));
    }
}

/// `void VfxPlants_Update(void)` (0x38ee78): builds the eight plant sway
/// matrices for this frame from the manager's clock. Per phase `i/8`: a sway
/// amplitude, a wobble amplitude and a twist angle from `sinf` of the
/// scaled clock; `m0 = base * rot(axis, twist)`; the cell matrix is the base
/// with its third row pushed along the sway direction, `m0`'s third column
/// along the wobble direction, and finally `cell = cell * m0`.
pub fn vfx_plants_update(c: &mut Cpu) {
    use plants::*;
    guest(c, |c| {
        c.cycles += 42;
        let frame = Frame::open(c, 0x1b0);
        let gp = r32(c, GP);
        let manager = Addr(gp.wrapping_add(GP_MANAGER)).lw(c);
        set_reg(c, A1, manager);
        frame.save(c, S1, 0x150);
        frame.save(c, S2, 0x140);
        set_reg(c, S1, 0);
        frame.save(c, S3, 0x130);
        frame.save(c, S4, 0x120);
        frame.save(c, S5, 0x110);
        frame.save(c, S6, 0x100);
        frame.save(c, S7, 0xf0);
        set_reg(c, S6, 0x50_0000);
        frame.save(c, FP, 0xe0);
        set_reg(c, S5, sx(AXIS));
        frame.save_fpr(c, 26, 0x1a0);
        frame.save_fpr(c, 25, 0x198);
        frame.save_fpr(c, 24, 0x190);
        frame.save_fpr(c, 23, 0x188);
        frame.save(c, S0, 0x160);
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 22, 0x180);
        frame.save_fpr(c, 21, 0x178);
        frame.save_fpr(c, 20, 0x170);
        c.lwc1(25, gp.wrapping_sub(GP_NEG_TIME_SCALE));
        let vt = Addr((manager as u32).wrapping_add(MANAGER_VTABLE)).lw(c);
        set_reg(c, V1, vt);
        set_reg(c, AT, 0x3f80_0000);
        c.mtc1(AT, 26); // 1.0
        let slot = VSlot::read(c, vt, SLOT_CLOCK);
        set_reg(c, A0, slot.delta);
        set_reg(c, V0, slot.func);
        c.lwc1(24, gp.wrapping_sub(GP_NEG_DEG2RAD));
        set_reg(c, A0, slot.this(manager));
        call_ptr(c, slot.func as u32, 0x0038_eef4)?;
        // f23 = (float)clock.
        c.mtc1(V0, 23);
        c.cvt_s_w(23, 23);
        let sp0 = sp(c);
        let (sin_at, cos_at) = (sp0 + 0xc0, sp0 + 0xc4);
        set_reg(c, FP, sx(cos_at));
        set_reg(c, V0, sx(sin_at));
        set_reg(c, S7, sx(sp0));
        Addr(sp0 + 0xc8).sw(c, sx(sin_at));
        let rot = sp0 + 0x40;
        set_reg(c, S2, sx(rot));
        set_reg(c, V0, 0x50_0000);
        let tmp = sp0 + 0x80;
        set_reg(c, S3, sx(tmp));
        set_reg(c, S4, sx(BASE));
        let m0 = sp0;
        let mut i: u32 = 0;
        loop {
            c.cycles += 207;
            // sway = SWAY_AMP * sin(i/8 + clock * SWAY_SPEED * scale) + SWAY_BASE
            c.lwc1(12, gp.wrapping_add(GP_SWAY_SPEED));
            c.cvt_s_w(12, 12);
            set_reg(c, AT, 0x3e00_0000);
            c.mtc1(AT, 0); // 0.125
            c.mtc1(S1, 20);
            c.cvt_s_w(20, 20);
            c.mul_s(12, 23, 12);
            c.mul_s(20, 20, 0); // f20 = i / 8
            c.mul_s(12, 12, 25);
            c.add_s(12, 20, 12);
            call_ptr(c, SIN, 0x0038_ef4c)?;
            c.lwc1(12, gp.wrapping_add(GP_WOBBLE_SPEED));
            c.lwc1(21, gp.wrapping_add(GP_SWAY_AMP));
            c.mul_s(12, 23, 12);
            c.lwc1(1, gp.wrapping_add(GP_SWAY_BASE));
            c.mul_s(21, 21, 0);
            c.mul_s(12, 12, 25);
            c.add_s(21, 21, 1); // f21 = sway
            c.add_s(12, 20, 12);
            call_ptr(c, SIN, 0x0038_ef70)?;
            // wobble = WOBBLE_AMP * sin(i/8 * WOBBLE_PHASE * scale + b)
            c.lwc1(12, gp.wrapping_add(GP_WOBBLE_PHASE));
            c.mul_s(12, 20, 12);
            c.mul_s(12, 12, 25);
            c.add_s(12, 12, 0);
            call_ptr(c, SIN, 0x0038_ef84)?;
            c.lwc1(12, gp.wrapping_add(GP_TWIST_SPEED));
            c.lwc1(22, gp.wrapping_add(GP_WOBBLE_AMP));
            c.mul_s(12, 23, 12);
            c.mul_s(22, 22, 0); // f22 = wobble
            c.mul_s(12, 12, 25);
            c.add_s(12, 20, 12);
            call_ptr(c, SIN, 0x0038_efa0)?;
            // angle = TWIST_AMP * sin(...); m0 = base; sincos(&s, &c, angle).
            c.lwc1(12, gp.wrapping_add(GP_TWIST_AMP));
            c.mul_s(12, 12, 0);
            copy_mat(c, BASE, m0);
            let v = Addr(sp0 + 0xc8).lw(c);
            set_reg(c, A0, v);
            set_reg(c, A1, sx(cos_at));
            call_ptr(c, SIN_COS, 0x0038_efd4)?;
            // rot = rotation about the axis by the angle (Rodrigues).
            c.lwc1(11, cos_at);
            c.lwc1(4, AXIS + 8);
            c.sub_s(1, 26, 11); // 1 - cos
            c.lwc1(0, AXIS);
            c.lwc1(6, AXIS + 4);
            c.lwc1(8, sin_at);
            c.mul_s(2, 1, 4);
            Addr(rot + 0x30).sw(c, 0);
            c.mul_s(5, 1, 0);
            Addr(rot + 0x34).sw(c, 0);
            c.mul_s(1, 1, 6);
            Addr(rot + 0x38).sw(c, 0);
            c.mul_s(13, 8, 4);
            Addr(rot + 0x0c).sw(c, 0);
            c.mul_s(12, 8, 0);
            Addr(rot + 0x1c).sw(c, 0);
            c.mul_s(9, 2, 4);
            Addr(rot + 0x2c).sw(c, 0);
            c.mul_s(10, 5, 4);
            c.swc1(26, rot + 0x3c);
            c.mul_s(3, 2, 0);
            c.mul_s(7, 5, 0);
            c.mul_s(4, 1, 4);
            c.mul_s(0, 1, 0);
            c.mul_s(8, 8, 6);
            c.mul_s(2, 2, 6);
            c.mul_s(5, 5, 6);
            c.mul_s(1, 1, 6);
            c.add_s(3, 3, 8);
            c.sub_s(0, 0, 13);
            c.sub_s(2, 2, 12);
            c.add_s(9, 9, 11);
            c.swc1(3, rot + 0x08);
            c.add_s(7, 7, 11);
            c.swc1(0, rot + 0x04);
            c.add_s(5, 5, 13);
            c.swc1(2, rot + 0x18);
            c.sub_s(10, 10, 8);
            c.swc1(9, rot + 0x28);
            c.add_s(1, 1, 11);
            c.swc1(7, rot);
            c.add_s(4, 4, 12);
            c.swc1(5, rot + 0x10);
            c.swc1(10, rot + 0x20);
            c.swc1(1, rot + 0x14);
            c.swc1(4, rot + 0x24);
            // m0 = m0 * rot; cell = base.
            mul_mat(c, m0, rot, m0);
            set_reg(c, V1, 0x50_0000);
            let cell = CELLS.wrapping_add(i << 6);
            set_reg(c, V0, sx(CELLS));
            set_reg(c, S0, sx(cell));
            copy_mat(c, BASE, cell);
            // cell row 2 += (sin, cos)(SWAY_DIR) * sway
            c.lwc1(12, gp.wrapping_add(GP_SWAY_DIR));
            c.mul_s(12, 12, 24);
            call_ptr(c, SIN, 0x0038_f138)?;
            c.mul_s(0, 0, 21);
            c.lwc1(1, cell + 0x20);
            c.lwc1(12, gp.wrapping_add(GP_SWAY_DIR));
            c.add_s(1, 1, 0);
            c.mul_s(12, 12, 24);
            c.swc1(1, cell + 0x20);
            call_ptr(c, COS, 0x0038_f154)?;
            c.mul_s(0, 0, 21);
            c.lwc1(1, cell + 0x24);
            c.lwc1(12, gp.wrapping_add(GP_WOBBLE_DIR));
            c.add_s(1, 1, 0);
            c.mul_s(12, 12, 24);
            c.swc1(1, cell + 0x24);
            // m0 column 2 += (sin, cos)(WOBBLE_DIR) * wobble
            call_ptr(c, SIN, 0x0038_f170)?;
            c.mul_s(0, 22, 0);
            c.lwc1(1, m0 + 0x08);
            c.lwc1(12, gp.wrapping_add(GP_WOBBLE_DIR));
            c.add_s(1, 1, 0);
            c.mul_s(12, 12, 24);
            c.swc1(1, m0 + 0x08);
            call_ptr(c, COS, 0x0038_f18c)?;
            c.mul_s(22, 22, 0);
            c.lwc1(0, m0 + 0x18);
            c.add_s(0, 0, 22);
            c.swc1(0, m0 + 0x18);
            // cell = cell * m0 (through two stack copies).
            mul_mat(c, cell, m0, tmp);
            copy_mat(c, tmp, rot);
            copy_mat(c, rot, cell);
            i += 1;
            set_reg(c, S1, sx(i));
            let more = (i as i32) < 8;
            set_reg(c, V0, more as u64);
            frame.restore(c, S0, 0x160);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
        c.cycles += 18;
        frame.restore(c, S1, 0x150);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore(c, S5, 0x110);
        frame.restore(c, S6, 0x100);
        frame.restore(c, S7, 0xf0);
        frame.restore(c, FP, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.restore_fpr(c, 26, 0x1a0);
        frame.restore_fpr(c, 25, 0x198);
        frame.restore_fpr(c, 24, 0x190);
        frame.restore_fpr(c, 23, 0x188);
        frame.restore_fpr(c, 22, 0x180);
        frame.restore_fpr(c, 21, 0x178);
        frame.restore_fpr(c, 20, 0x170);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod oc230_renderer {

    /// Top of the draw-state stack (20-byte records).
    pub const STATE: u32 = 0xe84;

    /// Output mode (1: 640 wide; else 320, doubled when `INTERLACE`).
    pub const MODE: u32 = 0x270;

    pub const INTERLACE: u32 = 0x274;

    /// The vtable (gcc 2.9x `{delta, fn}` slots).
    pub const VTABLE: u32 = 0x10d8;
}

/// Renderer virtual call through `s0`'s vtable slot `slot` (`this` = s0 +
/// delta, the rest of the arguments already set), returning to `ret`.
pub(crate) fn oc230_render_vcall(c: &mut Cpu, slot: u32, ret: u32) -> crate::call::Flow {
    c.g[V0][0] = c.lw(oc230_at(c, S0, oc230_renderer::VTABLE));
    c.g[A0][0] = c.lh(oc230_at(c, V0, slot));
    c.g[V1][0] = c.lw(oc230_at(c, V0, slot + 4));
    let t = r32(c, V1);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, t, ret)
}

/// Copy the 20-byte record at `src` to `dst` with the original's
/// unaligned `ldl/ldr`/`sdl/sdr` pairs through `lo`/`hi` and `t` (`lw`).
pub(crate) fn oc230_copy_state(c: &mut Cpu, src: u32, dst: u32, lo: usize, hi: usize, t: usize) {
    c.ldl(lo, src.wrapping_add(7));
    c.ldr(lo, src);
    c.ldl(hi, src.wrapping_add(0xf));
    c.ldr(hi, src.wrapping_add(8));
    c.g[t][0] = c.lw(src.wrapping_add(0x10));
    c.sdl(lo, dst.wrapping_add(7));
    c.sdr(lo, dst);
    c.sdl(hi, dst.wrapping_add(0xf));
    c.sdr(hi, dst.wrapping_add(8));
    let v = c.g[t][0];
    c.sw(dst.wrapping_add(0x10), v);
}

/// `void VfxScreenTint_Render(void)` (0x390f20): when the screen tint is
/// enabled (`gp+0x151c`) and its alpha (`gp+0x15ac`) is not zero, pushes a
/// copy of the renderer's draw state, begins the scene, sets the full-screen
/// viewport (640x480, or 320x480 offset by 320 when interlaced), resets the
/// state to the default record (0x501420) with alpha blending, draws a
/// screen-sized quad (four vertices, colour = tint rgb * alpha * 255, alpha
/// * 255) and pops the state.
pub fn vfx_screen_tint_render(c: &mut Cpu) {
    use oc230_renderer as rd;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x110);
        c.g[V0][0] = c.lw(oc230_gp(c, 0x151c));
        frame.save(c, S1, 0xe0);
        frame.save(c, S0, 0xf0);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 21, 0x108);
        frame.save_fpr(c, 20, 0x100);
        if c.g[V0][0] != 0 {
            // visible = alpha != 0
            c.cycles += 6;
            c.lwc1(1, oc230_gp(c, 0x15ac));
            c.mtc1(0, 0);
            c.c_eq_s(1, 0);
            if !c.fcc() {
                c.g[S1][0] = 1;
            }
        }
        c.cycles += 2;
        c.g[S0][0] = c.lw(oc230_gp(c, 0x2a90));
        if c.g[S1][0] != 0 {
            // Push the draw state (copy the top record one up).
            c.cycles += 33;
            c.g[V0][0] = c.lw(oc230_at(c, S0, rd::STATE));
            let top = r32(c, V0);
            oc230_copy_state(c, top, top.wrapping_add(0x14), V1, A0, A1);
            c.g[V1][0] = c.lw(oc230_at(c, S0, rd::STATE));
            c.g[V0][0] = c.lw(oc230_at(c, S0, rd::VTABLE));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x14));
            let v = c.g[V1][0];
            c.sw(oc230_at(c, S0, rd::STATE), v);
            // BeginScene (0xa8), Begin2D (0xf8), slot 0x128.
            c.g[A0][0] = c.lh(oc230_at(c, V0, 0xa8));
            c.g[V1][0] = c.lw(oc230_at(c, V0, 0xac));
            let t = r32(c, V1);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, t, 0x0039_0fb0)?;
            oc230_render_vcall(c, 0xf8, 0x0039_0fc4)?;
            oc230_render_vcall(c, 0x128, 0x0039_0fd8)?;
            // Viewport width f20 and x offset f21.
            c.g[A0][0] = c.lw(oc230_at(c, S0, rd::MODE));
            c.g[V0][0] = 1;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[V0][0] = c.lw(oc230_at(c, S0, rd::INTERLACE));
                c.cycles += 5;
                c.g[AT][0] = 0x43a0_0000; // 320
                c.mtc1(AT, 20);
                c.mtc1(0, 21);
                if c.g[V0][0] != 0 {
                    c.mov_s(21, 20);
                }
                c.cycles += 1;
            } else {
                c.cycles += 5;
                c.g[AT][0] = 0x4420_0000; // 640
                c.mtc1(AT, 20);
                c.mtc1(0, 21);
            }
            c.g[V0][0] = c.lw(oc230_at(c, S0, rd::VTABLE));

            // SetViewport(this, 0, 0, x, 0, w, 480, 0, 1) (0xd0).
            c.cycles += 104;
            c.g[A1][0] = 0;
            c.mtc1(0, 13);
            c.mov_s(14, 20);
            c.g[A0][0] = c.lh(oc230_at(c, V0, 0xd0));
            c.g[A2][0] = 0;
            c.g[V1][0] = c.lw(oc230_at(c, V0, 0xd4));
            c.mov_s(12, 21);
            c.g[AT][0] = 0x43f0_0000; // 480
            c.mtc1(AT, 15);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 17);
            let t = r32(c, V1);
            c.mov_s(16, 13);
            call_ptr(c, t, 0x0039_104c)?;

            // Reset the top state to the default record, then its bits.
            c.g[V1][0] = c.lw(oc230_at(c, S0, rd::STATE));
            c.g[V0][0] = 0x50_0000;
            c.g[A2][0] = 0xffff_ffff_ffff_fc1f;
            c.g[A0][0] = 0xffff_ffff_ffbf_0000;
            c.g[A1][0] = sx(0x0050_1420);
            let top = r32(c, V1);
            oc230_copy_state(c, 0x0050_1420, top, A3, T0, T1);
            c.g[A0][0] |= 0xffff;
            c.g[T2][0] = 0x40_0000;
            c.g[A1][0] = 0xffff_ffff_fe7f_0000;
            c.g[V1][0] = c.lw(oc230_at(c, S0, rd::STATE));
            c.g[A1][0] |= 0xffff;
            c.g[T3][0] = 0x100_0000;
            c.g[A3][0] = 0xffff_ffff_ffcf_0000;
            c.g[V0][0] = c.lw(oc230_at(c, V1, 8));
            c.g[A3][0] |= 0xffff;
            c.g[T0][0] = 0xffff_ffff_fff0_0000;
            c.g[T1][0] = 0x1_0000;
            c.g[V0][0] &= c.g[A2][0];
            c.g[T0][0] |= 0xfff;
            c.g[V0][0] |= 0xa0;
            c.g[A2][0] = c.lw(oc230_gp(c, 0x15bc));
            let v = c.g[V0][0];
            c.sw(oc230_at(c, V1, 8), v);
            c.g[T1][0] |= 0x4000;
            c.g[T4][0] = 0xffff_ffff_ffff_ff83;
            c.g[A2][0] = sx(r32(c, A2) << 2);
            c.g[V1][0] = c.lw(oc230_at(c, S0, rd::STATE));
            c.g[A2][0] &= 0x7c;
            c.g[T5][0] = u64::MAX;
            c.add_s(14, 21, 20);
            c.g[V0][0] = c.lw(oc230_at(c, V1, 4));
            c.g[T7][0] = u64::MAX;
            c.g[T6][0] = 3;
            // word 4 of the top state: (w & mask) | bits, five times.
            c.g[V0][0] &= c.g[A0][0];
            c.g[V0][0] |= c.g[T2][0];
            let v = c.g[V0][0];
            c.sw(oc230_at(c, V1, 4), v);
            for (base, mask, bits) in [
                (A0, A1, Some(T3)),
                (V1, A3, None),
                (A0, T0, Some(T1)),
                (A1, T4, Some(A2)),
            ] {
                c.g[base][0] = c.lw(oc230_at(c, S0, rd::STATE));
                c.g[V0][0] = c.lw(oc230_at(c, base, 4));
                c.g[V0][0] &= c.g[mask][0];
                if let Some(b) = bits {
                    c.g[V0][0] |= c.g[b][0];
                }
                let v = c.g[V0][0];
                c.sw(oc230_at(c, base, 4), v);
            }
            c.g[V1][0] = c.lw(oc230_at(c, S0, rd::STATE));
            let v = c.g[T5][0];
            c.sh(oc230_at(c, V1, 0x10), v);

            // Quad corner positions (x, y, 0, 1) built in the scratch
            // quadword at sp+0xc0 and copied to the vertices' +0x20.
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 1);
            let sp = sp(c);
            c.swc1(21, sp.wrapping_add(0xc0));
            c.sw(sp.wrapping_add(0xc8), 0);
            c.sw(sp.wrapping_add(0xc4), 0);
            c.swc1(1, sp.wrapping_add(0xcc));
            c.g[AT][0] = 0x43f0_0000;
            c.mtc1(AT, 0);
            c.lq(V0, sp.wrapping_add(0xc0));
            c.swc1(14, sp.wrapping_add(0xc0));
            c.sw(sp.wrapping_add(0xc8), 0);
            c.sw(sp.wrapping_add(0xc4), 0);
            c.sq(V0, sp.wrapping_add(0x20));
            c.lq(V1, sp.wrapping_add(0xc0));
            c.swc1(21, sp.wrapping_add(0xc0));
            c.sw(sp.wrapping_add(0xc8), 0);
            c.swc1(0, sp.wrapping_add(0xc4));
            c.sq(V1, sp.wrapping_add(0x50));
            c.lq(A0, sp.wrapping_add(0xc0));
            c.swc1(14, sp.wrapping_add(0xc0));
            c.sw(sp.wrapping_add(0xc8), 0);
            c.sq(A0, sp.wrapping_add(0x80));
            c.lq(V0, sp.wrapping_add(0xc0));
            c.sq(V0, sp.wrapping_add(0xb0));
            if c.g[S1][0] != 0 {
                // Colour = tint (r, g, b) * alpha * 255, alpha * 255.
                c.cycles += 25;
                c.lwc1(4, oc230_gp(c, 0x15ac));
                c.mov_s(5, 1);
                c.lwc1(3, oc230_gp(c, 0x15b0));
                c.g[V1][0] = c.g[SP][0];
                c.lwc1(2, oc230_gp(c, 0x15b4));
                c.g[T0][0] = sx(sp.wrapping_add(0xc0));
                c.lwc1(1, oc230_gp(c, 0x15b8));
                c.mul_s(3, 4, 3);
                c.g[AT][0] = 0x437f_0000; // 255
                c.mtc1(AT, 0);
                c.mul_s(2, 4, 2);
                c.mul_s(1, 4, 1);
                c.mul_s(3, 3, 0);
                c.mul_s(4, 4, 0);
                c.mul_s(2, 2, 0);
                c.mul_s(1, 1, 0);
                c.cvt_w_s(0, 3);
                c.mfc1(A3, 0);
                c.cvt_w_s(0, 2);
                c.mfc1(A2, 0);
                c.cvt_w_s(0, 1);
                c.mfc1(A1, 0);
                c.cvt_w_s(0, 4);
                c.mfc1(A0, 0);
                let v = c.g[A3][0];
                c.sw(oc230_at(c, V1, 0x10), v);
                // Per vertex (0x30 apart): q = 1 at +8, colour at +0x10.
                loop {
                    c.cycles += 8;
                    let vert = r32(c, V1);
                    let (a2, a1, a0) = (c.g[A2][0], c.g[A1][0], c.g[A0][0]);
                    c.sw(vert.wrapping_add(0x14), a2);
                    c.sw(vert.wrapping_add(0x18), a1);
                    c.sw(vert.wrapping_add(0x1c), a0);
                    c.swc1(5, vert.wrapping_add(8));
                    c.g[V1][0] = sx(vert.wrapping_add(0x30));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    let v = c.g[A3][0];
                    c.sw(oc230_at(c, V1, 0x10), v);
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_1218);
                    }
                }
                // DrawPrimitive(this, 4, vertices, 0) (0x238).
                c.cycles += 8;
                c.g[V0][0] = c.lw(oc230_at(c, S0, rd::VTABLE));
                c.g[A1][0] = 4;
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = 0;
                c.g[A0][0] = c.lh(oc230_at(c, V0, 0x238));
                c.g[V1][0] = c.lw(oc230_at(c, V0, 0x23c));
                let t = r32(c, V1);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0039_1258)?;
            }

            // End2D (0x100), EndScene (0xb0), pop the draw state.
            c.cycles += 13;
            oc230_render_vcall(c, 0x100, 0x0039_126c)?;
            oc230_render_vcall(c, 0xb0, 0x0039_1280)?;
            c.g[V0][0] = c.lw(oc230_at(c, S0, rd::STATE));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
            let v = c.g[V0][0];
            c.sw(oc230_at(c, S0, rd::STATE), v);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.restore_fpr(c, 21, 0x108);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// VfxRenderToggles_Update (0x390c20)

/// `gp+0x2a90`: the graphics manager (`D_004A5B80`).
pub(crate) const TOGGLES_GFX_GP: u32 = 0x2a90;

/// Graphics manager: the vtable pointer of the render-state interface.
pub(crate) const TOGGLES_GFX_VTABLE: u32 = 0x10d8;

/// Vtable slot 16 (`{delta: i16, fn}` at +0x80): set the render toggles.
pub(crate) const TOGGLES_SLOT: u32 = 0x80;

/// `gp+0x13d0`: the toggle flags (`D_004A44C0`).
pub(crate) const TOGGLES_FLAGS_GP: u32 = 0x13d0;

/// `gp+0x13d8`: a one-shot "changed" latch (`D_004A44C8`), cleared here.
pub(crate) const TOGGLES_LATCH_GP: u32 = 0x13d8;

/// `void VfxRenderToggles_Update(void)` (0x390c20): pass the render toggle
/// flags to the graphics manager (vtable slot 16), then clear the one-shot
/// latch if it is set.
pub fn vfx_render_toggles_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        let gp = r32(c, GP);
        c.g[A2][0] = Addr(gp.wrapping_add(TOGGLES_GFX_GP)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(gp.wrapping_add(TOGGLES_FLAGS_GP)).lw(c);
        let gfx = r32(c, A2);
        c.g[V0][0] = Addr(gfx).off(TOGGLES_GFX_VTABLE).lw(c);
        let vt = r32(c, V0);
        c.g[A0][0] = Addr(vt).off(TOGGLES_SLOT).lh(c);
        c.g[V1][0] = Addr(vt).off(TOGGLES_SLOT + 4).lw(c);
        let method = r32(c, V1);
        c.g[A0][0] = sx(gfx.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0039_0c44)?;
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(TOGGLES_LATCH_GP)).lw(c);
        if c.g[V0][0] != 0 {
            Addr(gp.wrapping_add(TOGGLES_LATCH_GP)).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// VfxVisualEffectsTest_Render (0x38f800)

/// `cWorldPainterMan_DebugDraw()` (0x2bb6b8, through the table).
pub(crate) const VFX_TEST_RENDER_DEBUG_DRAW: u32 = 0x002b_b6b8;

/// `void VfxVisualEffectsTest_Render(void)` (0x38f800): the visual-effects
/// test's render hook: draw the world painter's debug view.
pub fn vfx_visual_effects_test_render(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Table(VFX_TEST_RENDER_DEBUG_DRAW),
        0x0038_f810,
    );
}

// ---------------------------------------------------------------------------
// 0x00391480 Vu0_CopyOutputMem

/// VU0 data memory (EE address) where the microprograms leave their output.
pub(crate) const VU0_OUTPUT_MEM: u32 = 0x1100_4200;

/// `void Vu0_CopyOutputMem(int unused, u128* dst)` (0x391480): copy the 100
/// quadwords of VU0 output memory (`0x11004200`) to `dst`, ten per loop
/// pass (a0 = source, a1 = destination, a2 = passes left - 1; an event check
/// at each back edge).
pub fn vu0_copy_output_mem(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = VU0_OUTPUT_MEM as u64;
    c.g[A2][0] = 0x9;
    loop {
        c.cycles += 24;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        for k in 0..10u32 {
            // Alternating v1 / v0, as the original's unrolled body.
            let reg = if k % 2 == 0 { V1 } else { V0 };
            c.lq(reg, (c.g[A0][0] as u32).wrapping_add(0x10 * k));
            if k == 9 {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa0));
                c.sq(reg, (c.g[A1][0] as u32).wrapping_add(0x90));
            } else {
                c.sq(reg, (c.g[A1][0] as u32).wrapping_add(0x10 * k));
            }
        }
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa0));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_1490);
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// Vu0_LoadRegsAndCall (0x391418)

/// `void Vu0_LoadRegsAndCall(void* unused, const Vec4 regs[16])` (0x391418):
/// loads VF1..VF16 from the 16 quadwords at `regs`, sets VI27 to the
/// microprogram address 0 and runs the VU0 microprogram there
/// (`vcallmsr vi27`). Leaves v0 = 0.
pub fn vu0_load_regs_and_call(c: &mut Cpu) {
    c.cycles += 25;
    let regs = r32(c, A1);
    for vf in 1..=16u32 {
        c.lqc2(vf as usize, regs.wrapping_add((vf - 1) * 0x10));
    }
    // The microprogram address, as VI27 expects it (byte address >> 3).
    c.g[V0][0] = 0;
    c.ctc2(V0, 27);
    c.vu0().vnop();
    c.vu0().vnop();
    c.vcallmsr();
    ret(c);
}

// ---------------------------------------------------------------------------
// 2D rendering

/// `void Render2D_TransformPoint(Vec4 *out, const Vec2 *p, const Mat4 *m)`
/// (0x38e0a8): transform a 2D point by `m` into `out` (w = 1).
pub fn render2d_transform_point(c: &mut Cpu) {
    c.cycles += 28;
    let a0 = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    c.f[6] = Addr(a1).off(0x4).read_fpu(c).0;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.f[2] = Addr(a2).off(0xc).read_fpu(c).0;
    c.g[V0][0] = c.g[A0][0];
    c.f[0] = Addr(a2).off(0x4).read_fpu(c).0;
    c.f[3] = Addr(a2).off(0x10).read_fpu(c).0;
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[6])).0;
    c.f[4] = Addr(a1).read_fpu(c).0;
    c.f[1] = Addr(a2).read_fpu(c).0;
    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[6])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[4])).0;
    c.f[7] = Addr(a2).off(0x1c).read_fpu(c).0;
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[4])).0;
    c.f[5] = Addr(a2).off(0x18).read_fpu(c).0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[4] = c.g[1][0] as u32;
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[3])).0;
    let sp = c.g[SP][0] as u32;
    Addr(sp).off(0x8).sw(c, 0);
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[2])).0;
    Addr(sp).off(0xc).write_fpu(c, FpuF32(c.f[4]));
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[7])).0;
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[5])).0;
    Addr(sp).off(0x4).write_fpu(c, FpuF32(c.f[0]));
    Addr(sp).write_fpu(c, FpuF32(c.f[1]));
    c.lq(3, sp);
    c.sq(3, a0);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

pub(crate) mod light_record {

    pub const SIZE: u32 = 0x70;

    /// 0 directional, 1 spot, 2 point, 3 ambient; others are skipped.
    pub const TYPE: u32 = 0x10;

    pub const INTENSITY: u32 = 0x14;

    /// Red, green, blue (floats).
    pub const COLOUR: u32 = 0x20;

    /// Direction x, y, z (floats).
    pub const DIR: u32 = 0x2c;

    pub const POS: u32 = 0x38;

    /// The spot cone: its angle is `pi/2 - asin(cone)`.
    pub const CONE: u32 = 0x5c;

    /// Unknown signed bytes passed to the hooks (+0x65 as a float).
    pub const BYTE_64: u32 = 0x64;

    pub const BYTE_65: u32 = 0x65;

    pub const DIRECTIONAL: u64 = 0;

    pub const SPOT: u64 = 1;

    pub const POINT: u64 = 2;

    pub const AMBIENT: u64 = 3;
}

// ---------------------------------------------------------------------------
// The light manager (render/lightman.cpp, render/font.cpp).
//
// A LightMan is 0x14 bytes: the vtable at +0, the instance table (one word
// per light id) at +0x4, then the base id (+0x8), the next index (+0xc) and
// the table's capacity (+0x10).

pub(crate) const LIGHT_MAN_INSTANCES: u32 = 0x4;

/// `LightMan_Create`'s allocation: 0x14 bytes named "LightMan" (0x492ee0).
pub(crate) const LIGHT_MAN_SIZE: u32 = 0x14;

pub(crate) const LIGHT_MAN_ALLOC_NAME: u32 = 0x0049_2ee0;

/// The PS2 light manager's vtable.
pub(crate) const LIGHT_MAN_PS2_VTABLE: u32 = 0x0049_3650;

/// `LightMan* cLightMan::construct()` (0x38dbd0): allocate the light
/// manager (`EA_MemAllocNamed(0x14, "LightMan", 0, 0)`), construct it with
/// `LightMan_ctor` and install the PS2 vtable (0x493650). Returns it.
pub fn light_man_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(LIGHT_MAN_ALLOC_NAME);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = sx(LIGHT_MAN_SIZE);
        callee::MEM_ALLOC_NAMED.call(c, 0x0038_dbf4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::LIGHT_MAN_CTOR.call(c, 0x0038_dc00)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(LIGHT_MAN_PS2_VTABLE);
        frame.restore_ra(c, 0);
        c.sw(r32(c, S0), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void LightMan_EmptyHookA(void* light, float* colour, float intensity)`
/// (0x38dc50): the ambient-light hook; does nothing on the PS2.
pub fn light_man_empty_hook_a(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void LightMan_EmptyHookB(void* light, float* colour, float* dir, float
/// intensity)` (0x38dc58): the directional-light hook; does nothing.
pub fn light_man_empty_hook_b(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void LightMan_EmptyHookC(void* light, float* colour, float* pos, int
/// unknown, float intensity)` (0x38dc60): the point-light hook; does
/// nothing.
pub fn light_man_empty_hook_c(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void LightMan_EmptyHookD(void* light, float* colour, float* pos, float*
/// dir, float angle, float intensity, float unknown, int unknown)`
/// (0x38dc68): the spot-light hook; does nothing.
pub fn light_man_empty_hook_d(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `LightMan_v05`'s stack frame: the colour `{1.0, r, g, b}` the hooks get,
/// the unit direction, and the saved first id.
pub(crate) const LIGHT_V05_COLOUR: u32 = 0x0;

pub(crate) const LIGHT_V05_DIR: u32 = 0x10;

pub(crate) const LIGHT_V05_FIRST_ID: u32 = 0x20;

/// `pi / 2` (gp - 0x26b0).
pub(crate) const LIGHT_V05_HALF_PI_GP: u32 = 0x26b0;

/// The direction (`f4` = x already loaded) normalised into the frame's
/// direction slot; a zero-length one is stored as it is.
pub(crate) fn light_man_v05_direction(c: &mut Cpu) {
    let light = r32(c, S0);
    c.lwc1(5, light.wrapping_add(light_record::DIR + 4));
    c.mul_s(0, 4, 4);
    c.lwc1(3, light.wrapping_add(light_record::DIR + 8));
    c.mul_s(1, 5, 5);
    c.mul_s(2, 3, 3);
    c.add_s(0, 0, 1);
    c.add_s(0, 0, 2);
    c.sqrt_s(0, 0);
    c.c_eq_s(0, 21);
    let dir = sp(c).wrapping_add(LIGHT_V05_DIR);
    if c.fcc() {
        c.swc1(4, dir);
        c.cycles += 2;
        c.swc1(5, dir + 4);
        c.swc1(3, dir + 8);
    } else {
        c.cycles += 8;
        c.div_s(0, 20, 0);
        c.mul_s(1, 3, 0);
        c.mul_s(2, 4, 0);
        c.mul_s(0, 5, 0);
        c.swc1(1, dir + 8);
        c.swc1(2, dir);
        c.swc1(0, dir + 4);
    }
}

/// One light of `LightMan_v05` (s0 the record, s3 its index, s1 the slot
/// once fetched), up to its hook call.
pub(crate) fn light_man_v05_light(c: &mut Cpu) -> crate::call::Flow {
    use light_record::*;
    c.cycles += 19;
    let this = r32(c, S6);
    c.g[V1][0] = Addr(this).lw(c);
    c.g[V0][0] = Addr(sp(c).wrapping_add(LIGHT_V05_FIRST_ID)).lw(c);
    // Virtual `GetInstance(first + i)` (slot +0x10: this adjustment, method).
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lh(c);
    c.g[A1][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
    let get_instance = r32(c, V0);
    c.g[A0][0] = sx(r32(c, S6).wrapping_add(r32(c, A0)));
    call_ptr(c, get_instance, 0x0038_dcf4)?;
    c.g[S1][0] = c.g[V0][0];
    c.sw(r32(c, S1), c.g[S0][0]);
    c.g[V0][0] = 1;
    let light = r32(c, S0);
    let colour = sp(c).wrapping_add(LIGHT_V05_COLOUR);
    c.swc1(20, colour);
    c.lwc1(0, light.wrapping_add(COLOUR));
    c.lwc1(1, light.wrapping_add(COLOUR + 4));
    c.lwc1(2, light.wrapping_add(COLOUR + 8));
    c.g[V1][0] = Addr(light.wrapping_add(TYPE)).lw(c);
    c.swc1(0, colour + 4);
    c.swc1(1, colour + 8);
    let kind = c.g[V1][0];
    c.swc1(2, colour + 0xc);

    if kind == SPOT {
        c.cycles += 14;
        c.lwc1(4, light.wrapping_add(DIR));
        c.g[S2][0] = c.g[S5][0];
        light_man_v05_direction(c);
        c.cycles += 16;
        c.lwc1(12, r32(c, S0).wrapping_add(CONE));
        callee::MATH_ASIN.call(c, 0x0038_de54)?;
        let light = r32(c, S0);
        let half_pi = r32(c, GP).wrapping_sub(LIGHT_V05_HALF_PI_GP);
        c.lwc1(12, half_pi);
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = Addr(light.wrapping_add(BYTE_65)).lb(c);
        c.g[A2][0] = c.g[S2][0];
        c.sub_s(12, 12, 0);
        c.lwc1(13, light.wrapping_add(INTENSITY));
        c.mtc1(V0, 14);
        c.cvt_s_w(14, 14);
        c.g[T0][0] = Addr(light.wrapping_add(BYTE_64)).lb(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A3][0] = c.g[FP][0];
        return callee::LIGHT_MAN_EMPTY_HOOK_D.call(c, 0x0038_de84);
    }

    c.cycles += 3;
    c.g[V0][0] = 2;
    if (kind as i64) < 2 {
        c.cycles += 2;
        if kind != DIRECTIONAL {
            c.cycles += 2;
            return Ok(());
        }
        c.lwc1(4, light.wrapping_add(DIR));
        c.cycles += 12;
        light_man_v05_direction(c);
        c.cycles += 7;
        c.lwc1(12, r32(c, S0).wrapping_add(INTENSITY));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = sx(sp(c).wrapping_add(LIGHT_V05_DIR));
        return callee::LIGHT_MAN_EMPTY_HOOK_B.call(c, 0x0038_ddc4);
    }

    c.cycles += 2;
    c.g[V0][0] = 3;
    if kind == POINT {
        c.cycles += 8;
        c.lwc1(12, light.wrapping_add(INTENSITY));
        c.g[A0][0] = c.g[S1][0];
        c.g[A3][0] = Addr(light.wrapping_add(BYTE_64)).lb(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S4][0];
        return callee::LIGHT_MAN_EMPTY_HOOK.call(c, 0x0038_dde4);
    }

    c.cycles += 2;
    if kind != AMBIENT {
        c.cycles += 2;
        return Ok(());
    }
    c.lwc1(12, light.wrapping_add(INTENSITY));
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[SP][0];
    callee::LIGHT_MAN_EMPTY_HOOK_A.call(c, 0x0038_de98)
}

/// `void LightMan::v05(LightMan* self, Light* lights, int count, int
/// firstId)` (0x38dc70): register `count` light records (0x70 bytes each):
/// the record's address goes in instance slot `firstId + i` (virtual
/// `GetInstance`), then by type the matching hook gets the slot, the colour
/// `{1.0, r, g, b}` and: directional the unit direction and intensity;
/// point the position, intensity and byte +0x64; spot the position, unit
/// direction, cone angle `pi/2 - asin(cone)`, intensity, byte +0x64 and
/// (float) byte +0x65; ambient the intensity. Other types are skipped.
pub fn light_man_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S3, 0x90);
        frame.save(c, S6, 0x60);
        c.g[S3][0] = 0; // the light index
        frame.save(c, S7, 0x50);
        c.g[S6][0] = c.g[A0][0]; // self
        frame.save(c, S0, 0xc0);
        c.g[S7][0] = c.g[A2][0]; // count
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, FP, 0x40);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 21, 0xd8);
        frame.save_fpr(c, 20, 0xd0);
        let any = (c.g[S7][0] as i64) > 0;
        let first_id = frame.at(c, LIGHT_V05_FIRST_ID);
        c.sw(first_id, c.g[A3][0]);
        if any {
            c.cycles += 8;
            c.g[S0][0] = c.g[A1][0]; // the light record
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 20); // 1.0
            c.g[S4][0] = sx(r32(c, S0).wrapping_add(light_record::POS));
            c.mtc1(ZERO, 21); // 0.0
            c.g[FP][0] = sx(sp(c).wrapping_add(LIGHT_V05_DIR));
            c.g[S5][0] = c.g[S4][0];
            loop {
                light_man_v05_light(c)?;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                c.cycles += 5;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(light_record::SIZE));
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(light_record::SIZE));
                let more = (c.g[S3][0] as i64) < (c.g[S7][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(light_record::SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0038_dcd8);
                }
            }
        }
        c.cycles += 14;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore(c, FP, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 21, 0xd8);
        frame.restore_fpr(c, 20, 0xd0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 2D quads

/// The renderer pointer (`gp+0x2a90`), the renderer's vtable pointer and the
/// vtable slot of its `DrawPrims(count, verts, 0)` method.
pub(crate) const RENDER2D_RENDERER: u32 = 0x2a90;

pub(crate) const RENDER2D_RENDERER_VPTR: u32 = 0x10d8;

pub(crate) const RENDER2D_DRAW_PRIMS: u32 = 0x238;

/// A quad vertex (0x30 bytes): texture coordinates (s, t, q) at +0, four
/// colour words (r, g, b, a) at +0x10, the position quadword (x, y, z, w) at
/// +0x20.
pub(crate) const RENDER2D_VERTEX_SIZE: u32 = 0x30;

pub(crate) const RENDER2D_VERTEX_S: u32 = 0x0;

pub(crate) const RENDER2D_VERTEX_T: u32 = 0x4;

pub(crate) const RENDER2D_VERTEX_Q: u32 = 0x8;

pub(crate) const RENDER2D_VERTEX_COLOR: u32 = 0x10;

pub(crate) const RENDER2D_VERTEX_POS: u32 = 0x20;

/// Rects are four floats `{x, y, w, h}` (texture rects `{u, v, du, dv}`):
/// each axis's extent sits 8 bytes after its origin.
pub(crate) const RENDER2D_RECT_X: u32 = 0x0;

pub(crate) const RENDER2D_RECT_Y: u32 = 0x4;

pub(crate) const RENDER2D_RECT_EXTENT: u32 = 0x8;

/// `1.0f`, the w of every position (and of the constant register).
pub(crate) const RENDER2D_ONE_BITS: u64 = 0x3f80_0000;

/// A rect corner: `x` or `x + w` across, `y` or `y + h` down.
#[derive(Clone, Copy)]
pub(crate) struct Render2dCorner {
    pub(crate) right: bool,
    pub(crate) bottom: bool,
}

pub(crate) const RENDER2D_TOP_LEFT: Render2dCorner = Render2dCorner {
    right: false,
    bottom: false,
};

pub(crate) const RENDER2D_TOP_RIGHT: Render2dCorner = Render2dCorner {
    right: true,
    bottom: false,
};

pub(crate) const RENDER2D_BOTTOM_LEFT: Render2dCorner = Render2dCorner {
    right: false,
    bottom: true,
};

pub(crate) const RENDER2D_BOTTOM_RIGHT: Render2dCorner = Render2dCorner {
    right: true,
    bottom: true,
};

/// The position corners in the order every quad helper stores them.
pub(crate) const RENDER2D_POS_CORNERS: [Render2dCorner; 4] = [
    RENDER2D_TOP_LEFT,
    RENDER2D_TOP_RIGHT,
    RENDER2D_BOTTOM_LEFT,
    RENDER2D_BOTTOM_RIGHT,
];

/// How a quad helper fills its four vertices.
pub(crate) struct Render2dQuadLayout {
    /// The vertex (0..3) that takes each of [`RENDER2D_POS_CORNERS`].
    pub(crate) pos_vertex: [u32; 4],
    /// Textured quads: the texture-rect corner that gives that same vertex
    /// its (s, t).
    pub(crate) uv_corner: Option<[Render2dCorner; 4]>,
}

/// Vertex 0..3 = top-left, top-right, bottom-left, bottom-right.
pub(crate) const RENDER2D_LAYOUT_PLAIN: Render2dQuadLayout = Render2dQuadLayout {
    pos_vertex: [0, 1, 2, 3],
    uv_corner: None,
};

/// The positions turned a quarter: vertex 0..3 = bottom-left, top-left,
/// bottom-right, top-right.
pub(crate) const RENDER2D_LAYOUT_ROT90: Render2dQuadLayout = Render2dQuadLayout {
    pos_vertex: [1, 3, 0, 2],
    uv_corner: None,
};

/// Textured, positions as [`RENDER2D_LAYOUT_PLAIN`]: vertex 0..3 take the
/// texture rect's top-left, top-right, bottom-left, bottom-right corner.
pub(crate) const RENDER2D_LAYOUT_UV: Render2dQuadLayout = Render2dQuadLayout {
    pos_vertex: [0, 1, 2, 3],
    uv_corner: Some([
        RENDER2D_TOP_LEFT,
        RENDER2D_TOP_RIGHT,
        RENDER2D_BOTTOM_LEFT,
        RENDER2D_BOTTOM_RIGHT,
    ]),
};

/// Textured with the texture turned a quarter: vertex 0..3 take the texture
/// rect's top-right, bottom-right, top-left, bottom-left corner.
pub(crate) const RENDER2D_LAYOUT_UV_ROT90: Render2dQuadLayout = Render2dQuadLayout {
    pos_vertex: [0, 1, 2, 3],
    uv_corner: Some([
        RENDER2D_TOP_RIGHT,
        RENDER2D_BOTTOM_RIGHT,
        RENDER2D_TOP_LEFT,
        RENDER2D_BOTTOM_LEFT,
    ]),
};

/// One rect corner of `rect` along one axis: `rect[origin]`, plus the extent
/// on the far side. Returns the coordinate and the extent (0 when unread).
pub(crate) fn render2d_rect_axis(
    c: &mut Cpu,
    rect: Addr,
    origin: u32,
    far: bool,
) -> (FpuF32, FpuF32) {
    let start = rect.off(origin).read_fpu(c);
    if far {
        let extent = rect.off(origin + RENDER2D_RECT_EXTENT).read_fpu(c);
        (start + extent, extent)
    } else {
        (start, FpuF32::ZERO)
    }
}

/// `*pos = Vec4(x, y, 0, 1)`: the vector is built in the scratch quadword at
/// `0(sp)` and copied through `v0` (`lq`/`sq`).
pub(crate) fn render2d_put_position(c: &mut Cpu, pos: Addr, x: FpuF32, y: FpuF32) {
    let scratch = Addr(sp(c));
    scratch.write_fpu(c, x);
    scratch.off(0x4).write_fpu(c, y);
    scratch.off(0x8).sw(c, 0);
    scratch.off(0xc).write_fpu(c, FpuF32::ONE);
    c.lq(V0, scratch.0);
    c.sq(V0, pos.0);
}

/// Fill the positions (and, for textured layouts, the texture coordinates)
/// of the four vertices at `verts` from `rect` (and `uv_rect`).
pub(crate) fn render2d_fill_quad(
    c: &mut Cpu,
    verts: Addr,
    rect: Addr,
    uv_rect: Addr,
    layout: &Render2dQuadLayout,
) {
    // f0..f3 as the compiled code leaves them: the last vertex's y and x
    // (untextured) or its t and the dv added to it (textured, both layouts
    // end on a bottom texture corner), then the w and h of the last
    // (bottom-right) position.
    let mut f0 = FpuF32::ZERO;
    let mut f1 = FpuF32::ZERO;
    let mut width = FpuF32::ZERO;
    let mut height = FpuF32::ZERO;
    for (k, corner) in RENDER2D_POS_CORNERS.iter().enumerate() {
        let vertex = verts.off(RENDER2D_VERTEX_SIZE * layout.pos_vertex[k]);
        let (x, w) = render2d_rect_axis(c, rect, RENDER2D_RECT_X, corner.right);
        let (y, h) = render2d_rect_axis(c, rect, RENDER2D_RECT_Y, corner.bottom);
        render2d_put_position(c, vertex.off(RENDER2D_VERTEX_POS), x, y);
        (f0, f1, width, height) = (y, x, w, h);
        if let Some(uv_corner) = layout.uv_corner {
            // s is stored before t is read (the two rects may overlap).
            let (s, _) = render2d_rect_axis(c, uv_rect, RENDER2D_RECT_X, uv_corner[k].right);
            vertex.off(RENDER2D_VERTEX_S).write_fpu(c, s);
            let (t, dv) = render2d_rect_axis(c, uv_rect, RENDER2D_RECT_Y, uv_corner[k].bottom);
            vertex.off(RENDER2D_VERTEX_T).write_fpu(c, t);
            (f0, f1) = (t, dv);
        }
    }
    c.f[0] = f0.0;
    c.f[1] = f1.0;
    c.f[2] = width.0;
    c.f[3] = height.0;
    // Untextured quads keep the 1.0 they build positions with in f4
    // (textured ones in f3, overwritten by h).
    if layout.uv_corner.is_none() {
        c.f[4] = FpuF32::ONE.0;
    }
}

/// `renderer->DrawPrims(4, verts, 0)` through the renderer at `gp+0x2a90`,
/// leaving the renderer, its vtable and the method in the given registers
/// (the caller has set `a1`..`a3`).
pub(crate) fn render2d_draw_prims(
    c: &mut Cpu,
    renderer_reg: usize,
    vtable_reg: usize,
    method_reg: usize,
    ret_to: u32,
) -> Flow {
    let renderer = Addr(r32(c, GP)).off(RENDER2D_RENDERER).lw(c);
    c.g[renderer_reg][0] = renderer;
    let vtable = Addr(renderer as u32).off(RENDER2D_RENDERER_VPTR).lw(c);
    c.g[vtable_reg][0] = vtable;
    let method = VSlot::read(c, vtable, RENDER2D_DRAW_PRIMS);
    c.g[method_reg][0] = method.func;
    c.g[A0][0] = method.this(c.g[renderer_reg][0]);
    call_ptr(c, method.func as u32, ret_to)
}

/// The untransformed quads (`ctx` in a0 is unused, `verts` in a1, the rect
/// in a2, a textured quad's texture rect in a3): fill the vertices, then
/// draw them, returning to `ret_to`.
pub(crate) fn render2d_draw_rect_quad(
    c: &mut Cpu,
    layout: &Render2dQuadLayout,
    cycles: u64,
    ret_to: u32,
) {
    guest(c, |c| {
        c.cycles += cycles;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        c.g[AT][0] = RENDER2D_ONE_BITS;
        let textured = layout.uv_corner.is_some();
        if textured {
            c.g[T0][0] = c.g[A3][0];
        }
        c.g[A2][0] = c.g[A0][0];
        c.g[A1][0] = 4;
        c.g[A3][0] = 0;
        let verts = Addr(r32(c, A0));
        let rect = Addr(r32(c, V1));
        let uv_rect = Addr(r32(c, T0));
        render2d_fill_quad(c, verts, rect, uv_rect, layout);
        if textured {
            render2d_draw_prims(c, T1, V0, V1, ret_to)?;
        } else {
            render2d_draw_prims(c, T0, V1, V0, ret_to)?;
        }
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Render2D_DrawQuad(void *ctx, QuadVertex v[4], const float rect[4])`
/// (0x38e118): sets the positions of `v[0..3]` to the top-left, top-right,
/// bottom-left and bottom-right corners of `rect` `{x, y, w, h}` (z 0, w 1)
/// and draws them with the renderer's `DrawPrims(4, v, 0)`.
pub fn render2d_draw_quad(c: &mut Cpu) {
    render2d_draw_rect_quad(c, &RENDER2D_LAYOUT_PLAIN, 58, 0x0038_e1f4);
}

/// `void Render2D_DrawQuadRot90(void *ctx, QuadVertex v[4], const float
/// rect[4])` (0x38e200): [`render2d_draw_quad`] with the corners turned a
/// quarter: `v[0..3]` = bottom-left, top-left, bottom-right, top-right.
pub fn render2d_draw_quad_rot90(c: &mut Cpu) {
    render2d_draw_rect_quad(c, &RENDER2D_LAYOUT_ROT90, 58, 0x0038_e2dc);
}

/// The transformed quads (`verts` in a1, the rect in a2, the texture rect of
/// a textured quad in a3, the matrix in `matrix_reg`): fill the vertices,
/// transform the four positions in place with `Render2D_TransformPoint`
/// (through the scratch quadword at `0(sp)`), then draw them. `rets`: the
/// return addresses of the four transforms and of the draw call.
pub(crate) fn render2d_draw_rect_quad_xform(
    c: &mut Cpu,
    layout: &Render2dQuadLayout,
    cycles: u64,
    matrix_reg: usize,
    rets: &[u32; 5],
) {
    guest(c, |c| {
        c.cycles += cycles;
        let frame = Frame::open(c, 0x70);
        c.g[V1][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save_ra(c, 0x10);
        // s0 = verts, s1 = the matrix, s2..s4 = the positions of vertices 1..3.
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[matrix_reg][0];
        c.g[S2][0] = crate::abi::addu(c.g[S0][0], 0x50);
        c.g[S3][0] = crate::abi::addu(c.g[S0][0], 0x80);
        c.g[S4][0] = crate::abi::addu(c.g[S0][0], 0xb0);
        c.g[AT][0] = RENDER2D_ONE_BITS;
        let verts = Addr(r32(c, S0));
        let rect = Addr(r32(c, V1));
        let uv_rect = Addr(r32(c, A3));
        render2d_fill_quad(c, verts, rect, uv_rect, layout);
        for (i, &ret_to) in rets[..4].iter().enumerate() {
            let point = match i {
                0 => crate::abi::addu(c.g[S0][0], RENDER2D_VERTEX_POS as u64),
                1 => c.g[S2][0],
                2 => c.g[S3][0],
                _ => c.g[S4][0],
            };
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = point;
            c.g[A2][0] = c.g[S1][0];
            callee::RENDER2D_TRANSFORM_POINT.call(c, ret_to)?;
            c.lq(V0, sp(c));
            let pos =
                r32(c, S0).wrapping_add(RENDER2D_VERTEX_SIZE * i as u32 + RENDER2D_VERTEX_POS);
            c.sq(V0, pos);
        }
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 4;
        c.g[A3][0] = 0;
        render2d_draw_prims(c, T0, V0, V1, rets[4])?;
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

/// `void Render2D_DrawQuadXform(void *ctx, QuadVertex v[4], const float
/// rect[4], const Mat3 *m)` (0x38e2e8): [`render2d_draw_quad`] with each
/// corner position transformed by the 3x3 row-vector matrix `m` before the
/// draw.
pub fn render2d_draw_quad_xform(c: &mut Cpu) {
    render2d_draw_rect_quad_xform(
        c,
        &RENDER2D_LAYOUT_PLAIN,
        96,
        A3,
        &[
            0x0038_e3d4,
            0x0038_e3ec,
            0x0038_e404,
            0x0038_e41c,
            0x0038_e448,
        ],
    );
}

/// `void Render2D_DrawQuadRot90Xform(void *ctx, QuadVertex v[4], const float
/// rect[4], const Mat3 *m)` (0x38e468): [`render2d_draw_quad_rot90`] with
/// each position transformed by `m` before the draw.
pub fn render2d_draw_quad_rot90_xform(c: &mut Cpu) {
    render2d_draw_rect_quad_xform(
        c,
        &RENDER2D_LAYOUT_ROT90,
        96,
        A3,
        &[
            0x0038_e554,
            0x0038_e56c,
            0x0038_e584,
            0x0038_e59c,
            0x0038_e5c8,
        ],
    );
}

/// `void Render2D_DrawQuadUV(void *ctx, QuadVertex v[4], const float
/// rect[4], const float uv[4])` (0x38e5e8): [`render2d_draw_quad`] that also
/// sets each vertex's (s, t) to the matching corner of the texture rect `uv`
/// `{u, v, du, dv}`.
pub fn render2d_draw_quad_uv(c: &mut Cpu) {
    render2d_draw_rect_quad(c, &RENDER2D_LAYOUT_UV, 83, 0x0038_e728);
}

/// `void Render2D_DrawQuadUVRot90(void *ctx, QuadVertex v[4], const float
/// rect[4], const float uv[4])` (0x38e738): [`render2d_draw_quad_uv`] with
/// the texture turned a quarter: `v[0..3]` take the texture's top-right,
/// bottom-right, top-left and bottom-left corners.
pub fn render2d_draw_quad_uv_rot90(c: &mut Cpu) {
    render2d_draw_rect_quad(c, &RENDER2D_LAYOUT_UV_ROT90, 82, 0x0038_e874);
}

/// `void Render2D_DrawQuadUVXform(void *ctx, QuadVertex v[4], const float
/// rect[4], const float uv[4], const Mat3 *m)` (0x38e880):
/// [`render2d_draw_quad_uv`] with each position transformed by `m` (in t0)
/// before the draw.
pub fn render2d_draw_quad_uv_xform(c: &mut Cpu) {
    render2d_draw_rect_quad_xform(
        c,
        &RENDER2D_LAYOUT_UV,
        120,
        T0,
        &[
            0x0038_e9cc,
            0x0038_e9e4,
            0x0038_e9fc,
            0x0038_ea14,
            0x0038_ea40,
        ],
    );
}

/// `void Render2D_DrawQuadUVRot90Xform(void *ctx, QuadVertex v[4], const
/// float rect[4], const float uv[4], const Mat3 *m)` (0x38ea60):
/// [`render2d_draw_quad_uv_rot90`] with each position transformed by `m` (in
/// t0) before the draw.
pub fn render2d_draw_quad_uv_rot90_xform(c: &mut Cpu) {
    render2d_draw_rect_quad_xform(
        c,
        &RENDER2D_LAYOUT_UV_ROT90,
        119,
        T0,
        &[
            0x0038_eba8,
            0x0038_ebc0,
            0x0038_ebd8,
            0x0038_ebf0,
            0x0038_ec1c,
        ],
    );
}

/// The vertex-array constructor loop head of [`render2d_draw_filled_rect`].
pub(crate) const RENDER2D_DRAW_FILLED_RECT_LOOP: u32 = 0x0038_ec60;

/// `void Render2D_DrawFilledRect(void *self, const float p0[2], const float
/// p1[2], const float colour[4], float z)` (0x38ec40): draws the untextured
/// rect with corners `p0` (top-left) and `p1` (bottom-right) at depth `z`
/// from a vertex array on its stack: every vertex gets q = 1, the colour
/// `{a, r, g, b}` as integer words (r, g, b scaled by 255, a by 128) and
/// texture coordinates of its corner (0 or 1). `self` is unused.
pub fn render2d_draw_filled_rect(c: &mut Cpu) {
    // Stack frame: the four vertices at 0, the position being built at
    // +0xc0, the saved ra at +0xd0.
    const FRAME: u32 = 0xe0;
    const POS_SCRATCH: u32 = 0xc0;
    const RA_SLOT: u32 = 0xd0;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, FRAME);
        c.g[T0][0] = c.g[A1][0];
        frame.save_ra(c, RA_SLOT);
        c.g[T1][0] = c.g[A2][0];
        c.g[V1][0] = c.g[A3][0];
        // The vertex array's constructor: an empty body, run once per vertex
        // (v0 counts 3 down to -1).
        c.g[V0][0] = 3;
        c.g[A0][0] = u64::MAX;
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.g[A1][0] = 4;
            if c.g[V0][0] == c.g[A0][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, RENDER2D_DRAW_FILLED_RECT_LOOP);
            }
        }
        c.cycles += 68;
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0;
        let p0 = Addr(r32(c, T0));
        let p1 = Addr(r32(c, T1));
        let colour = Addr(r32(c, V1));
        let byte_scale = FpuF32::from_f32(255.0);
        let alpha_scale = FpuF32::from_f32(128.0);
        let red = colour.off(0x4).read_fpu(c) * byte_scale;
        let green = colour.off(0x8).read_fpu(c) * byte_scale;
        let blue = colour.off(0xc).read_fpu(c) * byte_scale;
        let alpha = colour.read_fpu(c) * alpha_scale;
        let x0 = p0.read_fpu(c);
        let y0 = p0.off(0x4).read_fpu(c);
        let z = crate::abi::farg(c, 0);

        // Vertex 0: colour, q, and the position (x0, y0, z, 1); its s, t and
        // word +0xc are still the stack's old contents here.
        let verts = frame.at(c, 0);
        let vertex = |i: u32| Addr(verts.wrapping_add(RENDER2D_VERTEX_SIZE * i));
        let first = vertex(0);
        let rgba = [red, green, blue, alpha];
        for (i, channel) in rgba.iter().enumerate() {
            let word = channel.to_i32() as u32;
            first
                .off(RENDER2D_VERTEX_COLOR + 4 * i as u32)
                .write_u32(c, word);
        }
        let scratch = Addr(frame.at(c, POS_SCRATCH));
        scratch.write_fpu(c, x0);
        scratch.off(0x4).write_fpu(c, y0);
        scratch.off(0x8).write_fpu(c, z);
        scratch.off(0xc).write_fpu(c, FpuF32::ONE);
        first.off(RENDER2D_VERTEX_Q).write_fpu(c, FpuF32::ONE);

        // v[1..3] = v[0] (copied as quadwords through v0, v1 and a0), then
        // the far corners and the texture coordinates.
        c.lq(A0, scratch.0);
        c.lq(V0, first.0);
        c.lq(V1, first.0.wrapping_add(RENDER2D_VERTEX_COLOR));
        c.sq(A0, first.off(RENDER2D_VERTEX_POS).0);
        for i in 1..4 {
            let v = vertex(i);
            c.sq(V0, v.0);
            c.sq(V1, v.off(RENDER2D_VERTEX_COLOR).0);
            c.sq(A0, v.off(RENDER2D_VERTEX_POS).0);
        }
        let x1 = p1.read_fpu(c);
        let y1 = p1.off(0x4).read_fpu(c);
        let pos = RENDER2D_VERTEX_POS;
        vertex(1).off(pos).write_fpu(c, x1);
        vertex(3).off(pos).write_fpu(c, x1);
        vertex(3).off(pos + 0x4).write_fpu(c, y1);
        vertex(2).off(pos + 0x4).write_fpu(c, y1);
        for i in 0..4 {
            let s = if i & 1 != 0 {
                FpuF32::ONE
            } else {
                FpuF32::ZERO
            };
            let t = if i & 2 != 0 {
                FpuF32::ONE
            } else {
                FpuF32::ZERO
            };
            vertex(i).off(RENDER2D_VERTEX_S).write_fpu(c, s);
            vertex(i).off(RENDER2D_VERTEX_T).write_fpu(c, t);
        }

        // The registers the compiled code leaves for the call.
        c.g[AT][0] = RENDER2D_ONE_BITS;
        c.f[0] = FpuF32::ZERO.0;
        c.f[1] = x1.0;
        c.f[2] = y1.0;
        c.f[3] = blue.0;
        c.f[4] = red.to_i32() as u32;
        c.f[5] = y0.0;
        c.f[6] = FpuF32::ONE.0;
        c.f[7] = alpha.0;
        render2d_draw_prims(c, T0, V0, V1, 0x0038_ed80)?;
        frame.restore_ra(c, RA_SLOT);
        frame.close(c);
        Ok(())
    })
}

/// `void Dma_SendGifChain(void *chain, int flags)` (0x371f98): kick a GIF
/// chain, waiting for VIF1 idle when `flags & 4` (and after, when
/// `flags & 2`).
pub fn dma_send_gif_chain(c: &mut Cpu) {
    guest(c, |c| {
        'b_371fdc: {
            c.cycles += 3;
            c.g[V0][0] = c.g[A1][0] & 0x4;
            let t = c.g[V0][0] == 0u64;
            c.g[A2][0] = c.g[A0][0];
            if t {
                break 'b_371fdc;
            } else {
                c.cycles += 6;
                c.g[V0][0] = 0x10000000;
                c.g[V0][0] = c.g[V0][0] | 0xa000;
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.g[V1][0] & 0x100;
                let t = c.g[V1][0] == 0u64;
                c.g[V1][0] = 0x10000000;
                if t {
                    break 'b_371fdc;
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = c.g[V1][0] | 0xa000;
                    'l_371fc0: loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = c.g[V0][0] & 0x100;
                        let t = c.g[V0][0] != 0u64;
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00371fc0);
                            }
                            continue 'l_371fc0;
                        } else {
                            break 'b_371fdc;
                        }
                    }
                }
            }
        }
        'b_37202c: {
            'b_37201c: {
                c.cycles += 9;
                c.g[V0][0] = 0x10000000;
                c.g[V1][0] = 0x6fff0000;
                c.g[V0][0] = c.g[V0][0] | 0x3010;
                c.g[A0][0] = 0x4;
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                c.g[V1][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = c.g[A2][0] & 0x3ff0;
                if t {
                    break 'b_37201c;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = 0x10000000;
                    c.g[A0][0] = 0xffffffff80000000;
                    c.g[V1][0] = c.g[V1][0] | 0xa030;
                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = c.g[A1][0] & 0x1;
                    break 'b_37202c;
                }
            }
            c.cycles += 4;
            c.g[V0][0] = 0x10000000;
            c.g[V0][0] = c.g[V0][0] | 0xa030;
            c.sw(c.g[V0][0] as u32, c.g[A2][0]);
            c.g[V0][0] = c.g[A1][0] & 0x1;
            break 'b_37202c;
        }
        'b_372040: {
            'b_37203c: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x10000000;
                if t {
                    break 'b_37203c;
                } else {
                    c.cycles += 2;
                    c.g[V1][0] = 0x144;
                    break 'b_372040;
                }
            }
            c.cycles += 1;
            c.g[V1][0] = 0x104;
            break 'b_372040;
        }
        'b_37208c: {
            c.cycles += 5;
            c.g[V0][0] = c.g[V0][0] | 0xa000;
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
            c.g[V0][0] = c.g[A1][0] & 0x2;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x10000000;
            if t {
                break 'b_37208c;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.g[V0][0] | 0xa000;
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.g[V1][0] & 0x100;
                let t = c.g[V1][0] == 0u64;
                c.g[V1][0] = 0x10000000;
                if t {
                    break 'b_37208c;
                } else {
                    c.cycles += 2;
                    c.g[V1][0] = c.g[V1][0] | 0xa000;
                    'l_372070: loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = c.g[V0][0] & 0x100;
                        let t = c.g[V0][0] != 0u64;
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00372070);
                            }
                            continue 'l_372070;
                        } else {
                            break 'b_37208c;
                        }
                    }
                }
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;

    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;

    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::slot as slot_2;

    pub(crate) use crate::game::dirtysock::tags::tests::put_str;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_5;

    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_4;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::fe::ovstatelodge::tests::slot;
    pub(crate) use crate::game::fe::ovstatelodge::tests::VT;
    pub(crate) use crate::game::input::inputparse::tests::EMPTY;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::render::particle::tests::base;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_3;

    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;

    use ssx3_ee as ee;

    #[test]
    fn ps2_graphics_man_reset_color_filter_matches() {
        check_lohi(
            ssx3_ee::s12::f_00390458,
            ps2_graphics_man_reset_color_filter,
            200,
            &|c, seed| {
                let mut r = Rng(0x3904_5800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                for off in [0x14f4, 0x14f8, 0x14fc, 0x1500, 0x1504, 0x1508] {
                    c.bus.write32(GAME_GP + off, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn vfx_screen_tint_trigger_flash_matches() {
        check(
            ssx3_ee::s12::f_00390ec8,
            vfx_screen_tint_trigger_flash,
            200,
            &|c, seed| {
                let mut r = Rng(0x390e_c800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.f[12] = r.edge_f32();
                c.bus.write32(
                    GAME_GP + 0x151c,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
                c.bus.write32(GAME_GP - 0x2698, r.edge_f32());
            },
        );
    }

    #[test]
    fn vfx_screen_tint_poll_flash_sound_matches() {
        check(
            ssx3_ee::s12::f_00390ef8,
            vfx_screen_tint_poll_flash_sound,
            200,
            &|c, seed| {
                let mut r = Rng(0x390e_f800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(
                    GAME_GP + 0x151c,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
                c.bus.write32(
                    GAME_GP + 0x15c4,
                    if seed % 4 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn dma_wait_to_scratchpad_matches() {
        check(
            ee::s12::f_0038f7b0__generated,
            dma_wait_to_scratchpad,
            64,
            &|c, seed| {
                let mut r = Rng(0x38f7_b0 ^ (seed as u64) << 9 | 1);
                scramble(c, &mut r);
                // DMA channel 4 is idle (CHCR bit 8 clear) on a fresh machine; a
                // busy channel spins until the DMAC clears it, which no test
                // machine does, so only the idle path is exercised.
            },
        );
    }

    #[test]
    fn spr_stream_set_mode_matches() {
        driven!(gen_driven, ssx3_ee::s12::f_0038f300__generated);
        driven!(hand_driven, spr_stream_set_mode);
        fn setup(c: &mut Cpu, seed: u32) -> Events {
            let mut r = Rng(0x38f3_00 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let selfp = DATA;
            c.g[A0][0] = selfp as u64;
            let mode = 1 + seed % 8;
            c.g[A1][0] = mode as u64;
            c.bus.write32(selfp, if seed % 5 == 0 { mode } else { 0 });
            put_words(
                c,
                0x49_0000 + 0x2ef0,
                &[
                    0x0038_f344,
                    0x0038_f34c,
                    0x0038_f354,
                    0x0038_f35c,
                    0x0038_f364,
                    0x0038_f36c,
                    0x0038_f36c,
                ],
            );
            put_words(
                c,
                0x49_0000 + 0x2f10,
                &[
                    0x0038_f3a0,
                    0x0038_f3a8,
                    0x0038_f3b0,
                    0x0038_f3b8,
                    0x0038_f3c8,
                    0x0038_f3c0,
                    0x0038_f3c8,
                ],
            );
            schedule(c, &mut r, seed, 0x0038_f300)
        }
        check_events(gen_driven, hand_driven, 60, &setup);
    }

    #[test]
    fn dma_wait_from_scratchpad_matches() {
        check(
            ee::s12::f_0038f768__generated,
            dma_wait_from_scratchpad,
            40,
            &|c, seed| {
                let mut r = Rng(0x9e37_79b9 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn vfx_screen_tint_update_matches() {
        check(
            ssx3_ee::s12::f_00390c60__generated,
            vfx_screen_tint_update,
            300,
            &|c, seed| {
                let mut r = Rng(0x3912_0c60 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let g = GAME_GP;
                // A quiet PRNG state so `Rand_Next` returns immediately.
                for i in 0..0x24u32 {
                    c.bus.write32(0x004f_f018 + i * 4, 0);
                }
                c.bus.write32(g + 0x151c, if seed % 6 == 0 { 0 } else { 1 });
                c.bus.write32(
                    g + 0x15c4,
                    match seed % 3 {
                        0 => 0,
                        1 => 0xffff_ffff,
                        _ => (seed % 20) as u32,
                    },
                );
                // Timer: idle (-1) on some seeds, otherwise inside or past the
                // segment lengths.
                let (d0, d1, d2, d3) = (4u32, 3, 2, 5);
                let timer = match seed % 8 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    2 => 2,
                    3 => 5,
                    4 => 8,
                    5 => 11,
                    6 => 13,
                    _ => (r.next_u32() % 18) as u32,
                };
                c.bus.write32(g + 0x15a8, timer);
                c.bus.write32(
                    g + 0x1520,
                    match seed % 5 {
                        0 => 0xffff_ffff,
                        1 => 0,
                        2 => 20,
                        _ => r.next_u32() & 0x3f,
                    },
                );
                c.bus.write32(g + 0x1524, r.edge_f32());
                c.bus.write32(g + 0x1528, d0);
                c.bus.write32(g + 0x152c, d1);
                c.bus.write32(g + 0x1530, d2);
                c.bus.write32(g + 0x1534, d3);
                for off in [
                    0x1538u32, 0x153c, 0x1540, 0x1544, 0x1548, 0x154c, 0x1550, 0x1554, 0x1558,
                    0x155c, 0x1560, 0x1564, 0x14f4, 0x14f8, 0x14fc,
                ] {
                    c.bus.write32(g + off, r.edge_f32());
                }
                c.bus.write32(g + 0x15ac, r.next_u32());
                c.bus.write32(g + 0x15b0, r.next_u32());
                c.bus.write32(g + 0x15b4, r.next_u32());
                c.bus.write32(g + 0x15b8, r.next_u32());
                c.bus.write32(g + 0x15bc, r.next_u32());
            },
        );
    }

    #[test]
    fn vu0_read_vi1_matches() {
        check(
            ee::s12::f_003914f8__generated,
            vu0_read_vi1,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0039_14f8, seed);
                scramble(c, &mut r);
                c.bus.plat.write_ctrl(1, r.next_u32());
            },
        );
    }

    #[test]
    fn font_dtor_matches() {
        check(ee::s12::f_003916c0__generated, font_dtor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x39_16c0, 0);
            let obj = DATA + 0x1000;
            c.bus
                .write32(obj + 4, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
            c.bus.write32(obj + 0x60, r.next_u32());
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (seed % 4 == 0) as u64;
        });
    }

    #[test]
    fn light_man_v09_matches() {
        check(
            ee::s12::f_0038dee8__generated,
            light_man_v09,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38de_e8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let vt = vtable(c, &mut r, &[0x10]);
                clear(c, this, 0x40);
                c.bus.write32(this, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn vfx_plants_get_sway_matrix_matches() {
        check(
            ee::s12::f_0038f2a8__generated,
            vfx_plants_get_sway_matrix,
            SEEDS_2,
            &|c, seed| {
                let mut r = base(c, seed, 0x38f2a8);
                let pos = DATA + 0x100;
                clear(c, DATA, 0x200);
                c.bus.write32(pos, r.edge_f32());
                c.bus.write32(pos + 4, r.edge_f32());
                c.bus.write32(gp_at(0x1368), r.f32(100.0));
                c.g[A0][0] = sx(pos);
            },
        );
    }

    // ------------------------------------------------------------- cameras

    #[test]
    fn light_man_v10_matches() {
        check(
            ssx3_ee::s12::f_0038df38__generated,
            light_man_v10,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x38df38, 0);
                w(c, OBJ, VT);
                slot(c, VT, 0x10, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = seed as u64;
                c.g[A3][0] = OBJ3 as u64;
                c.f[12] = ((seed as f32) * 0.5).to_bits();
            },
        );
    }

    #[test]
    fn light_man_v11_matches() {
        check(
            ssx3_ee::s12::f_0038df98__generated,
            light_man_v11,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38df98, 0);
                let obj = DATA + 0x1000;
                let vt = DATA + 0x5000;
                c.g[A0][0] = obj as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.f[12] = r.edge_f32();
                w32(c, obj, vt);
                slot_2(c, vt, 0x10, 0, EMPTY);
            },
        );
    }

    /// EAFile's request and file-handle pools (counts `0x519ab0`/`0x519ab4`,
    /// tables `0x519ad8`/`0x519adc`, entries 0x30/0x110 bytes): with no free
    /// entry the allocators spin forever (`0x3de768`).
    pub(crate) fn eafile_pools(c: &mut Cpu) {
        for (count, table, at_3, size) in [
            (0x0051_9ab0, 0x0051_9ad8, 0xf_0000, 0x30),
            (0x0051_9ab4, 0x0051_9adc, 0xf_1000, 0x110),
        ] {
            let base_2 = DATA + at_3;
            c.bus.write32(count, 4);
            c.bus.write32(table, base_2);
            for i in 0..size {
                c.bus.write32(base_2 + 4 * i, 0);
            }
        }
    }

    #[test]
    fn font_load_matches() {
        check(ee::s12::f_00391708__generated, font_load, 8, &|c, seed| {
            let _ = alloc_base(c, seed, 0x3917_08, 8);
            eafile_pools(c);
            let font = DATA;
            let name = DATA + 0x1000;
            put_str(c, name, b"font.x");
            c.g[A0][0] = font as u64;
            c.g[A1][0] = name as u64;
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { 0x100 };
        });
    }

    #[test]
    fn tex_man_swizzle_t8_block_matches() {
        check(
            ee::s12::f_0038f930__generated,
            tex_man_swizzle_t8_block,
            60,
            &|c, seed| {
                let _ = base_3(c, seed, 0x38_f930);
                globals(c);
                for i in 0..0x100u32 {
                    let v = if seed % 2 == 0 {
                        i
                    } else {
                        (seed.wrapping_mul(7) + i) & 0x1ff
                    };
                    w_3(c, 0x0044_bde0 + 4 * i, v);
                    w_3(c, DATA + 0x1000 + 4 * i, i.wrapping_mul(0x0101_0101));
                }
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x2000);
            },
        );
    }

    #[test]
    fn light_man_v12_matches() {
        check(
            ssx3_ee::s12::f_0038e008__generated,
            light_man_v12,
            20,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x38e008, 2);
                let obj = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.g[A3][0] = (DATA + 0x3000) as u64;
                c.g[T0][0] = (DATA + 0x4000) as u64;
                c.g[T1][0] = (DATA + 0x5000) as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.f[14] = r.edge_f32();
                w(c, obj, DATA + 0x6000);
                c.bus.write16(DATA + 0x6000 + 0x10, 0);
                w(c, DATA + 0x6000 + 0x14, 0);
            },
        );
    }

    // ------------------------------------------------------------ VU0

    #[test]
    fn vu0_upload_microcode_a_matches() {
        check(
            ee::s12::f_003912a8__generated,
            vu0_upload_microcode_a,
            20,
            &|c, seed| {
                base_5(c, seed, 0x3912, 1);
                w_4(c, 0x1000_8000, 0);
                w_4(c, 0x1000_8020, 0);
                w_4(c, 0x1000_8030, 0);
            },
        );
    }

    #[test]
    fn vu0_upload_microcode_b_matches() {
        check(
            ee::s12::f_00391360__generated,
            vu0_upload_microcode_b,
            20,
            &|c, seed| {
                base_5(c, seed, 0x3913, 1);
                w_4(c, 0x1000_8000, 0);
                w_4(c, 0x1000_8020, 0);
                w_4(c, 0x1000_8030, 0);
            },
        );
    }

    // ------------------------------------------------- screen tint at a rate

    /// A lightning screen tint at rate `hz`: enabled, segments 4/3/2/5
    /// frames, no hold point, a seeded `Rand_Next` state, idle.
    fn tint_machine(hz: u32) -> Box<Cpu> {
        let mut c = crate::difftest::fresh();
        crate::tick::set_for_test(&mut c, hz);
        c.g[GP][0] = GAME_GP as u64;
        let g = GAME_GP;
        let mut r = Rng(0x0039_0c60);
        for i in 0..6u32 {
            c.bus.write32(0x004f_f018 + i * 4, r.next_u32() | 1);
        }
        c.bus.write32(g + 0x151c, 1);
        c.bus.write32(g + 0x15c4, 0);
        c.bus.write32(g + 0x15a8, 0xffff_ffff);
        c.bus.write32(g + 0x1520, 0xffff_ffff);
        c.bus.write32(g + 0x1524, 0);
        for (off, d) in [(0x1528u32, 4u32), (0x152c, 3), (0x1530, 2), (0x1534, 5)] {
            c.bus.write32(g + off, d);
        }
        for (off, v) in [(0x1538u32, 1.0f32), (0x153c, 0.5), (0x1540, 0.75)] {
            c.bus.write32(g + off, v.to_bits());
        }
        c
    }

    fn tint_tick(c: &mut Cpu) {
        c.g[SP][0] = STACK as u64;
        c.g[RA][0] = TEST_RA as u64;
        vfx_screen_tint_update(c);
        assert_eq!(c.pc, TEST_RA, "update did not return");
    }

    /// A flash from its start (timer 0) at rate `hz`: (timer, level) after
    /// each update until the timer is back to idle.
    fn flash_at(hz: u32) -> Vec<(u32, u32)> {
        let mut c = tint_machine(hz);
        c.bus.write32(GAME_GP + 0x15a8, 0);
        let mut out = Vec::new();
        for _ in 0..1000 {
            tint_tick(&mut c);
            let timer = c.bus.read32(GAME_GP + 0x15a8);
            out.push((timer, c.bus.read32(GAME_GP + 0x15ac)));
            if timer == 0xffff_ffff {
                return out;
            }
        }
        panic!("{hz} Hz: the flash never ended");
    }

    /// The flash runs through its four segments in the same wall-clock time
    /// at every rate (its timer steps on a 60 Hz clock): the state after
    /// 60 Hz update i is the state after update (i - 1) * r / 60 + 1 at r.
    #[test]
    fn vfx_screen_tint_flash_follows_the_rate() {
        let at60 = flash_at(60);
        assert_eq!(at60.len(), 15);
        for hz in [120u32, 240] {
            let q = (hz / 60) as usize;
            let at = flash_at(hz);
            assert_eq!(at.len(), (at60.len() - 1) * q + 1, "{hz} Hz: flash length");
            for (i, e) in at60.iter().enumerate() {
                assert_eq!(at[i * q], *e, "{hz} Hz: 60 Hz update {}", i + 1);
            }
        }
    }

    /// The thunder follows the lightning after the same wall-clock delay:
    /// `VfxScreenTint_TriggerFlash` stores r/60 times the 60 Hz tick count.
    #[test]
    fn vfx_screen_tint_thunder_delay_follows_the_rate() {
        let delay = |hz: u32, intensity: f32| {
            let mut c = tint_machine(hz);
            c.bus
                .write32(GAME_GP - 0x2698, 0.001_807_228_9f32.to_bits());
            c.f[12] = intensity.to_bits();
            c.g[RA][0] = TEST_RA as u64;
            vfx_screen_tint_trigger_flash(&mut c);
            assert_eq!(c.pc, TEST_RA);
            c.bus.read32(GAME_GP + 0x15c4)
        };
        for intensity in [2000.0f32, 50_000.0, 199_999.0] {
            let d60 = delay(60, intensity);
            assert!(d60 > 0);
            for hz in [120u32, 240] {
                assert_eq!(delay(hz, intensity), d60 * hz / 60, "{hz} Hz, {intensity}");
            }
        }
    }

    /// Strikes per second while idle (chance 0.2: p = 0.04 per 60 Hz frame)
    /// over 100 s of updates at rate `hz`, forcing the tint back to idle after
    /// each update.
    fn strikes_at(hz: u32) -> u32 {
        let mut c = tint_machine(hz);
        c.bus.write32(GAME_GP + 0x1524, 0.2f32.to_bits());
        let mut n = 0;
        for _ in 0..100 * hz {
            c.bus.write32(GAME_GP + 0x15a8, 0xffff_ffff);
            tint_tick(&mut c);
            if c.bus.read32(GAME_GP + 0x15a8) == 0 {
                n += 1;
            }
        }
        n
    }

    /// The random strike chance is per 60 Hz frame: as many strikes per
    /// second at every rate (about 240 in 100 s; 25% tolerance).
    #[test]
    fn vfx_screen_tint_strike_chance_follows_the_rate() {
        let n60 = strikes_at(60) as f64;
        assert!((170.0..310.0).contains(&n60), "60 Hz: {n60} strikes");
        for hz in [120u32, 240] {
            let n = strikes_at(hz) as f64;
            assert!(
                (n / n60 - 1.0).abs() < 0.25,
                "{hz} Hz: {n} strikes vs {n60} at 60"
            );
        }
    }

    use crate::difftest::elf_image;

    /// VfxPlants_Update with the game image loaded (real sinf/cosf/sincos),
    /// random tweakables, axis and base matrix, the manager's clock method a
    /// fake (forced small clock; it unwinds for some seeds).
    #[test]
    fn vfx_plants_update_matches() {
        use super::plants::*;
        covered(
            ee::s12::f_0038ee78__generated,
            vfx_plants_update,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38_ee78, 1);
                elf_image(c);
                c.pc = 0x0038_ee78;
                let gp = GAME_GP;
                let manager = ptr(&mut r);
                c.bus.write32(gp + GP_MANAGER, manager);
                let vt = vtable(c, &mut r, &[SLOT_CLOCK]);
                c.bus.write32(manager + MANAGER_VTABLE, vt);
                c.bus.write32(LOG_RESULTS, r.next_u32() % 100_000);
                for off in [
                    GP_SWAY_AMP,
                    GP_SWAY_BASE,
                    GP_WOBBLE_AMP,
                    GP_TWIST_AMP,
                    GP_SWAY_DIR,
                    GP_WOBBLE_DIR,
                ] {
                    let v = r.f32(90.0);
                    c.bus.write32(gp + off, v);
                }
                for off in [
                    GP_SWAY_SPEED,
                    GP_TWIST_SPEED,
                    GP_WOBBLE_SPEED,
                    GP_WOBBLE_PHASE,
                ] {
                    let v = r.next_u32() % 2000;
                    c.bus.write32(gp + off, v);
                }
                for k in 0..3 {
                    let v = r.f32(1.0);
                    c.bus.write32(AXIS + 4 * k, v);
                }
                for k in 0..16 {
                    let v = r.f32(10.0);
                    c.bus.write32(BASE + 4 * k, v);
                }
            },
        );
    }

    /// VfxScreenTint_Render: tint off / alpha zero (no draw), and the draw
    /// with both viewport modes (640, 320 with and without the interlace
    /// offset); the renderer's methods are fakes that may unwind.
    #[test]
    fn vfx_screen_tint_render_matches() {
        covered(
            ee::s12::f_00390f20__generated,
            vfx_screen_tint_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0039_0f20, 7);
                c.pc = 0x0039_0f20;
                let gp = GAME_GP;
                c.bus
                    .write32(gp + 0x151c, if seed % 5 == 0 { 0 } else { 1 });
                let alpha = match seed % 7 {
                    0 => 0,
                    1 => 0x8000_0000,
                    _ => r.f32(1.0),
                };
                c.bus.write32(gp + 0x15ac, alpha);
                for off in [0x15b0, 0x15b4, 0x15b8] {
                    let v = r.f32(1.0);
                    c.bus.write32(gp + off, v);
                }
                let bits = r.next_u32();
                c.bus.write32(gp + 0x15bc, bits);
                let rend = DATA;
                c.bus.write32(gp + 0x2a90, rend);
                let stack = DATA + 0x2000 + (r.next_u32() % 16) * 4;
                c.bus.write32(rend + oc230_renderer::STATE, stack);
                fill(c, &mut r, stack, 16);
                fill(c, &mut r, 0x0050_1420, 5);
                let mode = if seed % 3 == 0 { 1 } else { r.next_u32() % 3 };
                c.bus.write32(rend + oc230_renderer::MODE, mode);
                c.bus
                    .write32(rend + oc230_renderer::INTERLACE, seed / 2 % 2);
                let vt = DATA + 0x3000;
                c.bus.write32(rend + oc230_renderer::VTABLE, vt);
                for (i, slot) in [0xa8u32, 0xf8, 0x128, 0xd0, 0x238, 0x100, 0xb0]
                    .iter()
                    .enumerate()
                {
                    c.bus.write16(vt + slot, (i as u16) * 4);
                    c.bus.write32(vt + slot + 4, FAKE + 0x10 * i as u32);
                }
            },
        );
    }

    /// The graphics manager with a fake slot-16 method, random flags, the
    /// latch set or clear.
    #[test]
    fn vfx_render_toggles_update_matches() {
        use crate::difftest::GAME_GP;
        fakes::covered(
            ee::s12::f_00390c20__generated,
            vfx_render_toggles_update,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x39_0c20, 1);
                let gfx = fakes::ptr(&mut r);
                let vt = fakes::vtable(c, &mut r, &[TOGGLES_SLOT]);
                c.bus.write32(GAME_GP + TOGGLES_GFX_GP, gfx);
                c.bus.write32(gfx + TOGGLES_GFX_VTABLE, vt);
                c.bus.write32(GAME_GP + TOGGLES_FLAGS_GP, r.next_u32());
                let latch = if seed % 2 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(GAME_GP + TOGGLES_LATCH_GP, latch);
                c.pc = 0x0039_0c20;
            },
        );
    }

    /// VfxVisualEffectsTest_Render: the real cWorldPainterMan_DebugDraw over
    /// a test world, with the debug-draw flag (gp+0x13c4) off (its short
    /// path) or on with no painter entry.
    #[test]
    fn vfx_visual_effects_test_render_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        check(
            ee::s12::f_0038f800__generated,
            vfx_visual_effects_test_render,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x0038_f800, 0);
                c.pc = 0x0038_f800;
                let world = DATA + 0x100;
                let man = DATA + 0x200;
                let painter = DATA + 0x300;
                let current = DATA + 0x400;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, man);
                c.bus.write32(man + 0x10, painter);
                c.bus.write32(painter + 0x8, current);
                c.bus.write32(GAME_GP + 0x13c4, seed % 2);
                // Flag on: entry table at gp+0x770 empty (null entry), index 0.
                c.bus.write32(GAME_GP + 0x770, 0);
                c.bus.write32(current + 0x4, 0);
            },
        );
    }

    /// Vu0_CopyOutputMem: random VU0 output memory copied to a random
    /// destination; events off, delivered, or unwind-requesting (ignored at
    /// the back edges, as in the original) at the loop's back edges.
    #[test]
    fn vu0_copy_output_mem_matches() {
        use crate::difftest::events::schedule;
        use crate::difftest::{fill, scramble, Rng, DATA};
        check(
            ee::s12::f_00391480__generated,
            vu0_copy_output_mem,
            120,
            &|c, seed| {
                let mut r = Rng(0x0039_1480 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                for k in 0..0x190u32 {
                    c.bus.write32(VU0_OUTPUT_MEM + 4 * k, r.next_u32());
                }
                let dst = DATA + 0x1000 + 0x10 * (r.next_u32() % 0x100);
                fill(c, &mut r, dst, 0x190);
                c.g[A1][0] = sx(dst);
                schedule(c, &mut r, seed, 0x0039_1480);
            },
        );
    }

    use crate::difftest::RA as SENTINEL;

    /// A microprogram at VU0 address 0 for `Vu0_LoadRegsAndCall`:
    /// `vi1 = vi14`, then end. Hand-written microcode has no translation:
    /// run it with the reference interpreter.
    pub(crate) fn oc300_put_microprogram_at_0(c: &mut Cpu) {
        ssx3_ps2::platform::machine_of(&mut c.bus.plat)
            .hw
            .vu0
            .fallback = Some(ssx3_ps2::vu::reference::run_program);
        let at = 0x1100_0000;
        c.bus.write32(at, 0x8000_7070); // IADD vi1, vi14, vi0
        c.bus.write32(at + 4, 0x4000_02ff); // NOP, E
    }

    /// Loads 16 random quadwords into VF1..VF16 and runs the microprogram
    /// at address 0.
    #[test]
    fn vu0_load_regs_and_call_matches() {
        check(
            ee::s12::f_00391418__generated,
            vu0_load_regs_and_call,
            200,
            &|c, seed| {
                let mut r = Rng(0x3914_1800 ^ seed as u64);
                scramble(c, &mut r);
                oc300_put_microprogram_at_0(c);
                c.g[14][0] = r.next_u32() as u64 & 0xffff;
                c.ctc2(14, 14);
                let regs = DATA + 0x400 + 0x10 * (r.next_u32() % 16);
                for q in 0..16u32 {
                    crate::difftest::put_vec(
                        c,
                        regs + 0x10 * q,
                        [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                    );
                }
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | regs as u64;
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0039_1418;
            },
        );
    }

    #[test]
    fn render2d_transform_point_matches() {
        check(
            ssx3_ee::s12::f_0038e0a8,
            render2d_transform_point,
            200,
            &|c, seed| {
                let mut r = Rng(0x38e0_a800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                for k in 0..2 {
                    c.bus.write32(DATA + 0x100 + k * 4, r.edge_f32());
                }
                for off in [0, 4, 0xc, 0x10, 0x18, 0x1c] {
                    c.bus.write32(DATA + 0x200 + off, r.edge_f32());
                }
            },
        );
    }

    use crate::difftest::check_paths;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // ------------------------------------------------ light manager

    /// The allocator fake's gate (a magic word in the machine's RAM).
    pub(crate) const LIGHT_MAN_CREATE_GATE: u32 = DATA + 0x8_0000;

    pub(crate) const LIGHT_MAN_CREATE_MAGIC: u32 = 0x4c4d_4372;

    pub(crate) static LIGHT_MAN_CREATE_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);

    /// `EA_MemAllocNamed` for `LightMan_Create`'s test: the logging fake
    /// (returns a pointer into the data area, may unwind) when the gate is
    /// set.
    pub(crate) fn light_man_create_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(LIGHT_MAN_CREATE_GATE) != LIGHT_MAN_CREATE_MAGIC {
            return fakes::fall_back(
                c,
                &LIGHT_MAN_CREATE_PREV_ALLOC,
                ee::s10::f_00317d70__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7d70, ra);
    }

    pub(crate) static LIGHT_MAN_CREATE_PREV_CTOR: AtomicUsize = AtomicUsize::new(0);

    /// `LightMan_ctor` for `LightMan_Create`'s test: the logging fake (may
    /// unwind) when the gate and its own control word (gate + 4) are set,
    /// else the real constructor.
    pub(crate) fn light_man_create_fake_ctor(c: &mut Cpu) {
        if c.bus.read32(LIGHT_MAN_CREATE_GATE) != LIGHT_MAN_CREATE_MAGIC
            || c.bus.read32(LIGHT_MAN_CREATE_GATE + 4) != LIGHT_MAN_CREATE_MAGIC
        {
            return fakes::fall_back(
                c,
                &LIGHT_MAN_CREATE_PREV_CTOR,
                ee::s12::f_0038ae10__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0038_ae10, ra);
    }

    /// `LightMan_Create`: a fake allocator (logs the arguments, returns a
    /// pointer); `LightMan_ctor` real (vtable and the global at gp+0x2a9c)
    /// or faked (`seed % 2`); no unwind, an unwind in the allocator or one
    /// in the faked constructor (`seed / 2 % 3`); the vtable store after it.
    /// Paths: full, unwound after the allocation, unwound after the
    /// constructor (seeds = 5 mod 6).
    #[test]
    fn light_man_create_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            light_man_create_fake_alloc,
            &LIGHT_MAN_CREATE_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038AE10),
            light_man_create_fake_ctor,
            &LIGHT_MAN_CREATE_PREV_CTOR,
        );
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x0038_dbd0, 2);
            c.bus.write32(LIGHT_MAN_CREATE_GATE, LIGHT_MAN_CREATE_MAGIC);
            if seed % 2 == 1 {
                c.bus
                    .write32(LIGHT_MAN_CREATE_GATE + 4, LIGHT_MAN_CREATE_MAGIC);
            }
            c.bus.write32(
                fakes::LOG_UNWIND_AT,
                [u32::MAX, 0, 1][(seed / 2 % 3) as usize],
            );
            fill(c, &mut r, DATA, 0x1_0000);
            // The allocation (fake call 0): a non-zero low byte.
            c.bus
                .write32(fakes::LOG_RESULTS, DATA + 0x2_0010 + 0x20 * (seed / 6 % 8));
            c.bus.write32(GAME_GP + 0x2a9c, r.next_u32());
            c.pc = 0x0038_dbd0;
        };
        fakes::covered(
            ee::s12::f_0038dbd0__generated,
            light_man_create,
            204,
            &setup,
        );
        // (constructor faked, fake calls made, ran to the end): the full
        // runs, the allocator unwinding, the faked constructor unwinding.
        crate::game::dirtysock::buddyapi::tests::require_arm_values(
            ee::s12::f_0038dbd0__generated,
            204,
            &setup,
            &|c, seed| {
                let ended = c.pc == crate::difftest::RA;
                (seed % 2, c.bus.read32(fakes::LOG_COUNT), ended)
            },
            &[
                (0, 1, true),
                (0, 1, false),
                (1, 1, false),
                (1, 2, true),
                (1, 2, false),
            ],
        );
    }

    /// Methods that only return (2 cycles): scrambled registers and an
    /// object of random words at `a0`; a body that touched anything or took
    /// another cycle count would differ.
    pub(crate) fn light_man_empty_method_check(generated: GuestFn, hand: GuestFn, salt: u64) {
        check_paths(generated, hand, 100, &[2], &|c, seed| {
            let mut r = Rng(salt ^ (seed as u64) << 24);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x100);
            c.g[A0][0] = sx(DATA + 0x34 + (seed % 4) * 0x40);
        });
    }

    #[test]
    fn light_man_empty_hook_a_matches() {
        light_man_empty_method_check(
            ee::s12::f_0038dc50__generated,
            light_man_empty_hook_a,
            0x38dc50,
        );
    }

    #[test]
    fn light_man_empty_hook_b_matches() {
        light_man_empty_method_check(
            ee::s12::f_0038dc58__generated,
            light_man_empty_hook_b,
            0x38dc58,
        );
    }

    #[test]
    fn light_man_empty_hook_c_matches() {
        light_man_empty_method_check(
            ee::s12::f_0038dc60__generated,
            light_man_empty_hook_c,
            0x38dc60,
        );
    }

    #[test]
    fn light_man_empty_hook_d_matches() {
        light_man_empty_method_check(
            ee::s12::f_0038dc68__generated,
            light_man_empty_hook_d,
            0x38dc68,
        );
    }

    /// Light types of the `LightMan_v05` tests: light `i` takes entry
    /// `(digit + i) % 6`.
    pub(crate) const LIGHT_V05_TEST_TYPES: [i32; 6] = [0, 1, 2, 3, 4, -1];

    pub(crate) const LIGHT_V05_TEST_SELF: u32 = DATA + 0x124;

    pub(crate) const LIGHT_V05_TEST_INSTANCES: u32 = DATA + 0x1000;

    pub(crate) const LIGHT_V05_TEST_LIGHTS: u32 = DATA + 0x4234;

    /// A light manager with the PS2 vtable (the real `GetInstance`, empty
    /// hooks, `Math_Asin`), a random instance table, and `count` light
    /// records of random words with: types from `seed % 6`, directions from
    /// `seed / 6 % 3` (0 random, 1 zero, 2 signed zeros and denormals, i.e.
    /// zero length), random colours, intensities and cones in [-1, 1]. The
    /// first id is random.
    pub(crate) fn light_man_v05_setup(c: &mut Cpu, seed: u32, count: i32) -> Rng {
        use light_record::*;
        let mut r = Rng(0x0038_dc70 ^ (seed as u64) << 16);
        scramble(c, &mut r);
        elf_image(c);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = 0x0038_dc70;
        fill(c, &mut r, LIGHT_V05_TEST_INSTANCES, 0x80);
        c.bus.write32(LIGHT_V05_TEST_SELF, LIGHT_MAN_PS2_VTABLE);
        c.bus.write32(
            LIGHT_V05_TEST_SELF + LIGHT_MAN_INSTANCES,
            LIGHT_V05_TEST_INSTANCES,
        );
        let types = seed % 6;
        let dirs = seed / 6 % 3;
        for i in 0..count.max(0) as u32 {
            let light = LIGHT_V05_TEST_LIGHTS + i * SIZE;
            fill(c, &mut r, light, SIZE / 4);
            let kind = LIGHT_V05_TEST_TYPES[((types + i) % 6) as usize];
            c.bus.write32(light + TYPE, kind as u32);
            c.bus.write32(light + INTENSITY, r.f32(10.0));
            for k in 0..3 {
                c.bus.write32(light + COLOUR + 4 * k, r.f32(2.0));
            }
            let dir = match (dirs + i) % 3 {
                0 => [r.f32(1000.0), r.f32(1000.0), r.f32(1000.0)],
                1 => [0, 0, 0],
                _ => [0x8000_0000, 0x0000_0123, 0x8000_0001],
            };
            for k in 0..3 {
                c.bus.write32(light + DIR + 4 * k as u32, dir[k]);
            }
            c.bus.write32(light + CONE, r.f32(1.0));
        }
        c.g[A0][0] = sx(LIGHT_V05_TEST_SELF);
        c.g[A1][0] = sx(LIGHT_V05_TEST_LIGHTS);
        c.g[A2][0] = sx(count as u32);
        c.g[A3][0] = sx(r.next_u32() % 40);
        r
    }

    /// Gate of the `Math_Asin` fake of the `LightMan_v05` test: set, the
    /// call unwinds (the first spot light's `Math_Asin` returns elsewhere).
    pub(crate) const LIGHT_V05_ASIN_GATE: u32 = DATA + 0x9_0000;

    pub(crate) const LIGHT_V05_ASIN_MAGIC: u32 = 0x4153_494e;

    pub(crate) static LIGHT_V05_PREV_ASIN: AtomicUsize = AtomicUsize::new(0);

    /// `Math_Asin` for the `LightMan_v05` test: unwinds (pc away from the
    /// return address, nothing else changed) when the gate is set, else the
    /// real body.
    pub(crate) fn light_man_v05_fake_asin(c: &mut Cpu) {
        if c.bus.read32(LIGHT_V05_ASIN_GATE) != LIGHT_V05_ASIN_MAGIC {
            return fakes::fall_back(c, &LIGHT_V05_PREV_ASIN, ee::s10::f_0031c128__generated);
        }
        c.pc = 0xdead_0000;
    }

    /// `LightMan_v05`: 1, 2, 3, 0 or -2 lights (`seed % 5`), each type
    /// (`seed / 5 % 6`) and direction kind (`seed / 30 % 3`). Single-light
    /// paths: no lights 32, type -1 75, type 4 77, ambient 81, point 83,
    /// directional with a zero direction 96, with a random one 102 (spot
    /// lights include `Math_Asin`, so their totals are not listed). On
    /// `seed / 90 % 2 == 1` a gated fake makes `Math_Asin` unwind, so the
    /// function returns at once after it.
    #[test]
    fn light_man_v05_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0031C128),
            light_man_v05_fake_asin,
            &LIGHT_V05_PREV_ASIN,
        );
        const COUNTS: [i32; 5] = [1, 2, 3, 0, -2];
        check_paths(
            ee::s12::f_0038dc70__generated,
            light_man_v05,
            180,
            &[32, 75, 77, 81, 83, 96, 102],
            &|c, seed| {
                light_man_v05_setup(c, seed / 5, COUNTS[(seed % 5) as usize]);
                if seed / 90 % 2 == 1 {
                    c.bus.write32(LIGHT_V05_ASIN_GATE, LIGHT_V05_ASIN_MAGIC);
                }
            },
        );
        // The single-light totals by the light's type and direction kind
        // (random, zero, zero length): directional 102 / 96 / 96, spot (with
        // `Math_Asin`) 132 / 127, point 83, ambient 81, type 4 77, type -1
        // 75, so a total another path shares still names its arm.
        crate::game::dirtysock::buddyapi::tests::require_arm_values(
            ee::s12::f_0038dc70__generated,
            90,
            &|c, seed| {
                light_man_v05_setup(c, seed / 5, COUNTS[(seed % 5) as usize]);
            },
            &|c, seed| {
                if seed % 5 != 0 {
                    return (9, 9, 0);
                }
                let k = seed / 5;
                (LIGHT_V05_TEST_TYPES[(k % 6) as usize], k / 6 % 3, c.cycles)
            },
            &[
                (0, 0, 102),
                (0, 1, 96),
                (0, 2, 96),
                (1, 0, 132),
                (1, 2, 127),
                (2, 0, 83),
                (3, 0, 81),
                (4, 0, 77),
                (-1, 0, 75),
            ],
        );
    }

    /// `LightMan_v05` with events due at the light loop's back edge (head
    /// 0x38dcd8, structured: not resumed): 2..4 lights (`seed / 3 % 3`),
    /// types and directions from `seed / 9`.
    #[test]
    fn light_man_v05_events() {
        driven!(gen_v05, ee::s12::f_0038dc70__generated);
        driven!(hand_v05, light_man_v05);
        check_events(gen_v05, hand_v05, 162, &|c, seed| {
            let count = 2 + (seed / 3 % 3) as i32;
            let mut r = light_man_v05_setup(c, seed / 9, count);
            schedule(c, &mut r, seed, 0x0038_dc70)
        });
    }

    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::require_paths;

    /// Fixture addresses: the vertex buffer, the rect, the texture rect (also
    /// the colour), the matrix, the renderer and its vtable.
    pub(crate) const RENDER2D_TEST_VERTS: u32 = DATA + 0x1230;

    pub(crate) const RENDER2D_TEST_RECT: u32 = DATA + 0x2014;

    pub(crate) const RENDER2D_TEST_UV: u32 = DATA + 0x2038;

    pub(crate) const RENDER2D_TEST_MATRIX: u32 = DATA + 0x2104;

    pub(crate) const RENDER2D_TEST_RENDERER: u32 = DATA + 0x3344;

    pub(crate) const RENDER2D_TEST_VTABLE: u32 = DATA + 0x5108;

    /// A real leaf that returns 0 (`v0 = 0; jr ra`, 2 cycles).
    pub(crate) const RENDER2D_TEST_LEAF: u32 = 0x0014_06c0;

    /// The quad helpers' machine. Seed digits: `seed % 2` makes `DrawPrims`
    /// the real leaf (0, a fixed cycle total) or the logging fake (1);
    /// `seed / 2 % 3 == 0` makes the fake unwind (when `unwinds`);
    /// `seed / 6 % 2 == 0` uses edge-case floats, else screen-sized values.
    /// The vertex buffer starts as random bytes; a1 = verts, a2 = rect.
    pub(crate) fn render2d_quad_setup(c: &mut Cpu, seed: u32, entry: u32, unwinds: bool) -> Rng {
        let mut r = machine(c, seed, entry as u64, 0);
        let method = if seed % 2 == 0 {
            RENDER2D_TEST_LEAF
        } else {
            FAKE
        };
        let unwind = unwinds && seed / 2 % 3 == 0;
        c.bus
            .write32(LOG_UNWIND_AT, if unwind { 0 } else { u32::MAX });
        let edge = seed / 6 % 2 == 0;
        c.bus
            .write32(GAME_GP + RENDER2D_RENDERER, RENDER2D_TEST_RENDERER);
        c.bus.write32(
            RENDER2D_TEST_RENDERER + RENDER2D_RENDERER_VPTR,
            RENDER2D_TEST_VTABLE,
        );
        let delta = r.next_u32() as u16;
        c.bus
            .write16(RENDER2D_TEST_VTABLE + RENDER2D_DRAW_PRIMS, delta);
        c.bus
            .write32(RENDER2D_TEST_VTABLE + RENDER2D_DRAW_PRIMS + 4, method);
        // Four vertices of 0x30 bytes.
        fill(c, &mut r, RENDER2D_TEST_VERTS, 4 * RENDER2D_VERTEX_SIZE / 4);
        for (base, words) in [
            (RENDER2D_TEST_RECT, 4),
            (RENDER2D_TEST_UV, 4),
            (RENDER2D_TEST_MATRIX, 8),
        ] {
            for i in 0..words {
                let v = if edge { r.edge_f32() } else { r.f32(640.0) };
                c.bus.write32(base + 4 * i, v);
            }
        }
        c.g[A1][0] = sx(RENDER2D_TEST_VERTS);
        c.g[A2][0] = sx(RENDER2D_TEST_RECT);
        c.pc = entry;
        r
    }

    /// Gate word, unwinding call index and call counter of
    /// [`render2d_fake_transform_point`].
    pub(crate) const RENDER2D_TEST_TRANSFORM_CTRL: u32 = DATA + 0x6010;

    pub(crate) const RENDER2D_TEST_TRANSFORM_MAGIC: u32 = 0x5832_44d7;

    pub(crate) static RENDER2D_PREV_TRANSFORM_POINT: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    /// `Render2D_TransformPoint` for the transformed quads: with the gate word
    /// in this machine's RAM, the call whose index is at CTRL+4 unwinds
    /// (`pc = 0xdead_0000 | n << 4`) instead of transforming; every other call
    /// runs the body the slot held.
    pub(crate) fn render2d_fake_transform_point(c: &mut Cpu) {
        let ctrl = RENDER2D_TEST_TRANSFORM_CTRL;
        if c.bus.read32(ctrl) == RENDER2D_TEST_TRANSFORM_MAGIC {
            let n = c.bus.read32(ctrl + 8);
            c.bus.write32(ctrl + 8, n + 1);
            if n == c.bus.read32(ctrl + 4) {
                c.pc = 0xdead_0000 | n << 4;
                return;
            }
        }
        crate::difftest::fakes::fall_back(
            c,
            &RENDER2D_PREV_TRANSFORM_POINT,
            ee::s12::f_0038e0a8__generated,
        );
    }

    /// The transformed quads' machine: [`render2d_quad_setup`] plus the matrix
    /// (a3, or t0 with the texture rect in a3 when `textured`) and the gated
    /// transform fake. Seed digit `seed / 12 % 8`: the transform call (0..3)
    /// that unwinds, 4..7 none (half the seeds reach the draw call).
    pub(crate) fn render2d_xform_setup(c: &mut Cpu, seed: u32, entry: u32, textured: bool) {
        render2d_quad_setup(c, seed, entry, true);
        if textured {
            c.g[A3][0] = sx(RENDER2D_TEST_UV);
            c.g[T0][0] = sx(RENDER2D_TEST_MATRIX);
        } else {
            c.g[A3][0] = sx(RENDER2D_TEST_MATRIX);
        }
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s12::OVERRIDE_0038E0A8),
            render2d_fake_transform_point,
            &RENDER2D_PREV_TRANSFORM_POINT,
        );
        let ctrl = RENDER2D_TEST_TRANSFORM_CTRL;
        let unwind_at = seed / 12 % 8;
        c.bus.write32(ctrl, RENDER2D_TEST_TRANSFORM_MAGIC);
        c.bus
            .write32(ctrl + 4, if unwind_at < 4 { unwind_at } else { u32::MAX });
        c.bus.write32(ctrl + 8, 0);
    }

    /// Plain rect: the four positions, the draw call's registers (logged by
    /// the fake), its return or unwind. Path (real leaf): 58 + 2.
    #[test]
    fn render2d_draw_quad_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            render2d_quad_setup(c, seed, 0x0038_e118, true);
        };
        covered(
            ee::s12::f_0038e118__generated,
            render2d_draw_quad,
            300,
            &setup,
        );
        require_paths(ee::s12::f_0038e118__generated, 300, &[60], &setup);
    }

    /// The quarter-turned corners. Path (real leaf): 58 + 2.
    #[test]
    fn render2d_draw_quad_rot90_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            render2d_quad_setup(c, seed, 0x0038_e200, true);
        };
        covered(
            ee::s12::f_0038e200__generated,
            render2d_draw_quad_rot90,
            300,
            &setup,
        );
        require_paths(ee::s12::f_0038e200__generated, 300, &[60], &setup);
    }

    /// Plain rect, positions through `Render2D_TransformPoint` with the
    /// matrix in a3. Paths: transform n unwinds 96 + 28 * n (n = 0..3); all
    /// return, real leaf 96 + 4 * 28 + 2.
    #[test]
    fn render2d_draw_quad_xform_matches() {
        let setup = |c: &mut Cpu, seed: u32| render2d_xform_setup(c, seed, 0x0038_e2e8, false);
        covered(
            ee::s12::f_0038e2e8__generated,
            render2d_draw_quad_xform,
            300,
            &setup,
        );
        require_paths(
            ee::s12::f_0038e2e8__generated,
            300,
            &[96, 124, 152, 180, 210],
            &setup,
        );
    }

    /// Quarter-turned corners, transformed. Paths: transform n unwinds
    /// 96 + 28 * n; all return, real leaf 96 + 4 * 28 + 2.
    #[test]
    fn render2d_draw_quad_rot90_xform_matches() {
        let setup = |c: &mut Cpu, seed: u32| render2d_xform_setup(c, seed, 0x0038_e468, false);
        covered(
            ee::s12::f_0038e468__generated,
            render2d_draw_quad_rot90_xform,
            300,
            &setup,
        );
        require_paths(
            ee::s12::f_0038e468__generated,
            300,
            &[96, 124, 152, 180, 210],
            &setup,
        );
    }

    /// Textured: positions plus (s, t) from the texture rect in a3 (kept in
    /// t0). Path (real leaf): 83 + 2.
    #[test]
    fn render2d_draw_quad_uv_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            render2d_quad_setup(c, seed, 0x0038_e5e8, true);
            c.g[A3][0] = sx(RENDER2D_TEST_UV);
        };
        covered(
            ee::s12::f_0038e5e8__generated,
            render2d_draw_quad_uv,
            300,
            &setup,
        );
        require_paths(ee::s12::f_0038e5e8__generated, 300, &[85], &setup);
    }

    /// Textured with the texture turned a quarter. Path (real leaf): 82 + 2.
    #[test]
    fn render2d_draw_quad_uv_rot90_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            render2d_quad_setup(c, seed, 0x0038_e738, true);
            c.g[A3][0] = sx(RENDER2D_TEST_UV);
        };
        covered(
            ee::s12::f_0038e738__generated,
            render2d_draw_quad_uv_rot90,
            300,
            &setup,
        );
        require_paths(ee::s12::f_0038e738__generated, 300, &[84], &setup);
    }

    /// Textured and transformed (texture rect in a3, matrix in t0). Paths:
    /// transform n unwinds 120 + 28 * n; all return, real leaf
    /// 120 + 4 * 28 + 2.
    #[test]
    fn render2d_draw_quad_uv_xform_matches() {
        let setup = |c: &mut Cpu, seed: u32| render2d_xform_setup(c, seed, 0x0038_e880, true);
        covered(
            ee::s12::f_0038e880__generated,
            render2d_draw_quad_uv_xform,
            300,
            &setup,
        );
        require_paths(
            ee::s12::f_0038e880__generated,
            300,
            &[120, 148, 176, 204, 234],
            &setup,
        );
    }

    /// Quarter-turned texture, transformed. Paths: transform n unwinds
    /// 119 + 28 * n; all return, real leaf 119 + 4 * 28 + 2.
    #[test]
    fn render2d_draw_quad_uv_rot90_xform_matches() {
        let setup = |c: &mut Cpu, seed: u32| render2d_xform_setup(c, seed, 0x0038_ea60, true);
        covered(
            ee::s12::f_0038ea60__generated,
            render2d_draw_quad_uv_rot90_xform,
            300,
            &setup,
        );
        require_paths(
            ee::s12::f_0038ea60__generated,
            300,
            &[119, 147, 175, 203, 233],
            &setup,
        );
    }

    /// Seeds per overlap test: every texture-rect start `seed % 48` times
    /// the setup digits `seed / 48`.
    pub(crate) const RENDER2D_OVERLAP_SEEDS: u32 = 48 * 24;

    /// Put the texture rect (a3) inside the vertex buffer: one word before
    /// the buffer up to its last word (`seed % 48`), so a texture coordinate
    /// stored for one corner is the next coordinate read.
    pub(crate) fn render2d_overlap_uv(c: &mut Cpu, seed: u32) {
        let start = RENDER2D_TEST_VERTS - 4 + 4 * (seed % 48);
        c.g[A3][0] = sx(start);
    }

    /// [`render2d_draw_quad_uv_matches`] with the texture rect overlapping
    /// the vertex buffer.
    #[test]
    fn render2d_draw_quad_uv_overlap_matches() {
        check(
            ee::s12::f_0038e5e8__generated,
            render2d_draw_quad_uv,
            RENDER2D_OVERLAP_SEEDS,
            &|c, seed| {
                render2d_quad_setup(c, seed / 48, 0x0038_e5e8, true);
                render2d_overlap_uv(c, seed);
            },
        );
    }

    /// [`render2d_draw_quad_uv_rot90_matches`] with overlapping rects.
    #[test]
    fn render2d_draw_quad_uv_rot90_overlap_matches() {
        check(
            ee::s12::f_0038e738__generated,
            render2d_draw_quad_uv_rot90,
            RENDER2D_OVERLAP_SEEDS,
            &|c, seed| {
                render2d_quad_setup(c, seed / 48, 0x0038_e738, true);
                render2d_overlap_uv(c, seed);
            },
        );
    }

    /// [`render2d_draw_quad_uv_xform_matches`] with overlapping rects.
    #[test]
    fn render2d_draw_quad_uv_xform_overlap_matches() {
        check(
            ee::s12::f_0038e880__generated,
            render2d_draw_quad_uv_xform,
            RENDER2D_OVERLAP_SEEDS,
            &|c, seed| {
                render2d_xform_setup(c, seed / 48, 0x0038_e880, true);
                render2d_overlap_uv(c, seed);
            },
        );
    }

    /// [`render2d_draw_quad_uv_rot90_xform_matches`] with overlapping rects.
    #[test]
    fn render2d_draw_quad_uv_rot90_xform_overlap_matches() {
        check(
            ee::s12::f_0038ea60__generated,
            render2d_draw_quad_uv_rot90_xform,
            RENDER2D_OVERLAP_SEEDS,
            &|c, seed| {
                render2d_xform_setup(c, seed / 48, 0x0038_ea60, true);
                render2d_overlap_uv(c, seed);
            },
        );
    }

    /// The filled rect's machine ([`render2d_quad_setup`]'s seed digits):
    /// p0 and p1 in the rect (a1, a2), the colour in the texture rect (a3,
    /// colour-sized floats or edge cases), z in f12. The stack below `sp`
    /// holds random words (the vertex array's unset s, t and +0xc).
    pub(crate) fn render2d_filled_rect_setup(c: &mut Cpu, seed: u32, unwinds: bool) -> Rng {
        let mut r = render2d_quad_setup(c, seed, 0x0038_ec40, unwinds);
        let edge = seed / 6 % 2 == 0;
        for i in 0..4 {
            let v = if edge { r.edge_f32() } else { r.f32(1.2) };
            c.bus.write32(RENDER2D_TEST_UV + 4 * i, v);
        }
        c.f[12] = if edge { r.edge_f32() } else { r.f32(64.0) };
        c.g[A1][0] = sx(RENDER2D_TEST_RECT);
        c.g[A2][0] = sx(RENDER2D_TEST_RECT + 0x8);
        c.g[A3][0] = sx(RENDER2D_TEST_UV);
        r
    }

    /// The constructor loop (three back edges, then the exit), the vertex
    /// array, the draw call's registers and its return or unwind. Path (real
    /// leaf): 8 + 4 * 7 + 68 + 2.
    #[test]
    fn render2d_draw_filled_rect_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            render2d_filled_rect_setup(c, seed, true);
        };
        covered(
            ee::s12::f_0038ec40__generated,
            render2d_draw_filled_rect,
            300,
            &setup,
        );
        require_paths(ee::s12::f_0038ec40__generated, 300, &[106], &setup);
    }

    driven!(
        render2d_gen_draw_filled_rect,
        ee::s12::f_0038ec40__generated
    );

    driven!(render2d_hand_draw_filled_rect, render2d_draw_filled_rect);

    /// Events due at the constructor loop's back edges (15, 22 and 29 cycles
    /// in; the schedule puts the first one 0..47 cycles in), delivered in
    /// place at 0x38ec60. The draw call never unwinds here.
    #[test]
    fn render2d_draw_filled_rect_events() {
        check_events(
            render2d_gen_draw_filled_rect,
            render2d_hand_draw_filled_rect,
            300,
            &|c, seed| {
                let mut r = render2d_filled_rect_setup(c, seed / 3, false);
                schedule(c, &mut r, seed, 0x0038_ec40)
            },
        );
    }
    pub(crate) use crate::game::misc::tests::SEEDS as SEEDS_4;

    /// `Dma_SendGifChain` over the flag bits (both polls skipped/taken)
    /// and small/large chain addresses; VIF1 STAT bit 8 always reads
    /// clear, so the polls exit at once (the busy spins wait on hardware,
    /// unreachable in the harness).
    #[test]
    fn dma_send_gif_chain_matches() {
        check(
            ee::s12::f_00371f98__generated,
            dma_send_gif_chain,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 1_368_003, 1);
                c.pc = 0x0037_1f98;
                c.g[A0][0] = [DATA + 0x5000, 0x7000_0000][seed as usize % 2] as u64;
                c.g[A1][0] = [0, 1, 2, 3, 4, 5, 6, 7][seed as usize / 2 % 8] as u64;
            },
        );
    }
}
