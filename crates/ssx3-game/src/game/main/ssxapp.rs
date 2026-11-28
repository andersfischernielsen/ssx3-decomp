//! `src/main/ssxapp.cpp`: hand-written replacements (moved from the
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
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintscore::gp as gp_2;
pub(crate) use crate::game::fe::fepopupmisc::p_2;
pub(crate) use crate::game::fe::fepopupmisc::sw;
pub(crate) use crate::game::fe::festateload::glo;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festaterewards::at_4 as at;
pub(crate) use crate::game::fe::ovstates::at as at_2;
pub(crate) use crate::game::fe::ovstates::close;
pub(crate) use crate::game::fe::ovstates::open;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
use crate::game::input::inputmap::COMPILE_BINDINGS as IM671_BINDINGS;
use crate::game::input::inputmap::STR_INPUT_MAP as IM671_STR_INPUT_MAP;
pub(crate) use crate::game::movie::movieplayer::p;
pub(crate) use crate::game::render::particle::addi;
pub(crate) use crate::game::render::particle::gp;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
use crate::guest::Addr;
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
    // 0x0022_6900 ssx_app_init (SSXApp_Init) is ported but left out: its
    // callees (graphics manager creation, the boot shape load from disc)
    // cannot be bounded in a differential test.
    (0x0022_7e98, ssx_app_next_tick, "SSXApp_NextTick"),
    (0x0022_7f80, mc_manager_get, "MCManager_Get"),
    (
        0x0022_81d8,
        ssx_app_start_game_module,
        "SSXApp_StartGameModule",
    ),
    (0x0022_8238, ssx_app_start_fe_module, "SSXApp_StartFEModule"),
    (0x0022_9278, crowd_man2d_dtor, "CrowdMan2d_dtor"),
    (0x0022_92e0, crowd_man2d_reset, "CrowdMan2d_Reset"),
    (0x0022_9398, crowd_man2d_remove_all, "CrowdMan2d_RemoveAll"),
    (
        0x0022_9408,
        crowd_man2d_remove_by_type,
        "CrowdMan2d_RemoveByType",
    ),
    (0x0022_9498, crowd_man2d_clear, "CrowdMan2d_Clear"),
    (0x0022_94c8, crowd_man2d_update, "CrowdMan2d_Update"),
    (
        0x0022_9530,
        crowd_man2d_spawn_sprites,
        "CrowdMan2d_SpawnSprites",
    ),
    (0x0022_9738, crowd_man_2d_add_sprite, "CrowdMan2d_AddSprite"),
    (0x0022_9788, crowd_man2d_add, "CrowdMan2d_Add"),
    (0x0022_9b90, crowd_man2d_clear_slot, "CrowdMan2d_ClearSlot"),
    (0x0022_9e58, crowdman2d_load_state, "CrowdMan2d_LoadState"),
    (0x0022_9f80, nash_empty_hook, "Nash_EmptyHook"),
    (0x0022_9f88, vu0_upload_occluder, "Vu0_UploadOccluder"),
    (0x0022_9fc8, nash_classify_instance, "Nash_ClassifyInstance"),
    (0x0022_a128, nash_classify_patch, "Nash_ClassifyPatch"),
    (0x0022_a270, nash_classify_effect, "Nash_ClassifyEffect"),
    (
        0x002b_4b48,
        world_trigger_manager_destroy_instance,
        "WorldTriggerManager_DestroyInstance",
    ),
    (0x0022_9f30, vu0_upload_matrix_pair, "Vu0_UploadMatrixPair"),
    (0x0022_a698, nash_collect_patches, "Nash_CollectPatches"),
    (0x0022_add8, nash_gather_visible, "Nash_GatherVisible"),
    (
        0x0022_9ba8,
        crowd_man_2d_render_sprites,
        "CrowdMan2d_RenderSprites",
    ),
    (0x0022_a4a8, nash_classify_effects, "Nash_ClassifyEffects"),
    (0x0022_a770, nash_collect_effects, "Nash_CollectEffects"),
    (
        0x0022_a368,
        nash_classify_instances,
        "Nash_ClassifyInstances",
    ),
    (0x0022_a408, nash_classify_patches, "Nash_ClassifyPatches"),
    (0x0022_7f58, ssx_app_tick, "SSXApp_Tick"),
    (0x0022_97d8, crowd_man2d_remove, "CrowdMan2d_Remove"),
    (0x0022_9e20, crowd_man2d_save_state, "CrowdMan2d_SaveState"),
    (
        0x0022_9820,
        crowd_man2d_bind_instance_frame,
        "CrowdMan2d_BindInstanceFrame",
    ),
    (0x0022_9910, crowd_man_2d_init_slot, "CrowdMan2d_InitSlot"),
    (
        0x0022_7bf0,
        ssx_app_parse_command_line,
        "SSXApp_ParseCommandLine",
    ),
    (
        0x0022_8c08,
        ssx_app_apply_display_options,
        "SSXApp_ApplyDisplayOptions",
    ),
    (0x0022_8ee0, ssx_load_net_modules, "ssx_LoadNetModules"),
    (0x0022_7e68, ssx_app_flush_ticks, "SSXApp_FlushTicks"),
    (0x0022_8a78, ssx_app_load_locale, "SSXApp_LoadLocale"),
    (0x0022_9180, crowd_man2d_ctor, "CrowdMan2d_ctor"),
    (0x0022_6830, ssxapp_ctor, "SSXApp_ctor"),
    (
        0x0022_7fa0,
        mc_manager_exists_wrapper,
        "MCManager_Exists_Wrapper",
    ),
    (0x0022_8bb8, ssxapp_v02, "SSXApp_v02"),
    (0x0022_7fc0, ssxapp_shutdown, "SSXApp_Shutdown"),
    (0x0022_6900, ssx_app_init_3, "SSXApp_Init"),
    (
        0x0022_6b60,
        oc671_input_map_load_from_file,
        "InputMap_LoadFromFile",
    ),
    (0x0022_82d0, ssxapp_boot_load, "SSXApp_BootLoad"),
    (0x0022_b008, game_module_render, "GameModule_Render"),
];

/// Direct and cross-shard callees, by guest address.
pub(crate) mod callee {
    use super::{ee, Callee};
    pub(crate) use crate::game::ai::rider::callee::COMM_SYSTEM_UPDATE;

    pub(crate) use crate::game::ai::rider::callee::INPUT_PAD_UPDATE;

    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_ADVANCE_WRITE;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_GET_PAD_DATA;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_GET_PAD_STATUS;
    pub(crate) use crate::game::ai::rider::callee::USB_KEYBOARD_UPDATE;

    pub(crate) use crate::game::bx::bxstringctor::callee::CROWD_MAN2D_REMOVE_ALL;
    pub(crate) use crate::game::bx::bxstringctor::callee::CROWD_MAN2D_RESET;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::CROWD_MAN2D_INIT_SLOT;

    pub(crate) use crate::game::fe::fepopupmisc::callee::GAME_MODULE_CTOR;

    pub(crate) use crate::game::fe::festateload::callee::WORLD_TRIGGER_MANAGER_DTOR;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MC_MANAGER_GET_INSTANCE;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;

    pub const NASH_CULL_NODE: Callee = Callee {
        addr: 0x0022_a830,
        f: ee::s05::Nash_CullNode,
    };

    pub const VU0_UPLOAD_MATRIX_PAIR: Callee = Callee {
        addr: 0x0022_9f30,
        f: ee::s05::Vu0_UploadMatrixPair,
    };

    pub const NASH_EMPTY_HOOK: Callee = Callee {
        addr: 0x0022_9f80,
        f: ee::s05::Nash_EmptyHook,
    };

    pub const NASH_COLLECT_PATCHES: Callee = Callee {
        addr: 0x0022_a698,
        f: ee::s05::Nash_CollectPatches,
    };

    pub const NASH_COLLECT_INSTANCES: Callee = Callee {
        addr: 0x0022_a5a0,
        f: ee::s05::Nash_CollectInstances,
    };

    pub const NASH_COLLECT_EFFECTS: Callee = Callee {
        addr: 0x0022_a770,
        f: ee::s05::Nash_CollectEffects,
    };

    pub const NASH_CLASSIFY_PATCHES: Callee = Callee {
        addr: 0x0022_a408,
        f: ee::s05::Nash_ClassifyPatches,
    };

    pub const NASH_CLASSIFY_INSTANCES: Callee = Callee {
        addr: 0x0022_a368,
        f: ee::s05::Nash_ClassifyInstances,
    };

    pub const NASH_CLASSIFY_EFFECTS: Callee = Callee {
        addr: 0x0022_a4a8,
        f: ee::s05::Nash_ClassifyEffects,
    };

    pub const WORLD_PAINTER_GET_SUN_TEXTURE: Callee = Callee {
        addr: 0x002e_e220,
        f: ee::s09::WorldPainter_GetSunTexture,
    };

    pub const WORLD_PAINTER_GET_SUN_AZIMUTH: Callee = Callee {
        addr: 0x002e_e0b8,
        f: ee::s09::WorldPainter_GetSunAzimuth,
    };

    pub const WORLD_PAINTER_GET_SUN_ELEVATION: Callee = Callee {
        addr: 0x002e_e070,
        f: ee::s09::WorldPainter_GetSunElevation,
    };
}

/// `gp`-relative globals used here.
pub(crate) mod g {

    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

    /// `OcclusionMan_BuildVisible(man, arg)`.
    pub const OCCLUSION_MAN_BUILD_VISIBLE: u32 = 0x0024_25c0;

    /// `Math_SinCos(angle f12, float* sin, float* cos)`.
    pub const MATH_SIN_COS: u32 = 0x0031_be50;
}

/// `uint CrowdMan2d_AddSprite(CrowdMan2d *man, const void *sprite, uint value)`
/// (0x229738): appends `sprite` (a 16-byte quad) and `value` to the pool at
/// `man + (man->count << 5) + 0x2220`, wraps the count modulo 40 and returns
/// the new count.
pub fn crowd_man_2d_add_sprite(c: &mut Cpu) {
    let man = Addr(abi::arg(c, 0));
    let sprite = abi::arg(c, 1);
    let value = c.g[A2][0];
    c.cycles += 16;
    c.g[V1][0] = man.off(0x2214).lw(c);
    c.lq(A3, sprite);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
    c.g[A1][0] = 0x28;
    c.g[V1][0] = sx(abi::r32(c, A0).wrapping_add(c.g[V1][0] as u32));
    c.sq(A3, abi::r32(c, V1).wrapping_add(0x2220));
    c.g[V0][0] = man.off(0x2214).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
    c.g[V0][0] = sx(abi::r32(c, A0).wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V0][0] as u32).off(0x2230).sw(c, value);
    let count = man.off(0x2214).lw(c);
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    abi::div(c, abi::r32(c, V1), abi::r32(c, A1));
    man.off(0x2214).sw(c, c.g[V1][0]);
    c.cycles += 3;
    c.g[V0][0] = c.hi[0];
    man.off(0x2214).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Crowd / section manager.

/// `void CrowdMan2d_ClearSlot(CrowdMan2d *self, int slot)` (0x229b90): store
/// all-ones in the 32-bit slot at `self + slot*4 + 8`; leaves that address in
/// `a1` and all-ones in `v0`.
pub fn crowd_man2d_clear_slot(c: &mut Cpu) {
    const SLOT0: u32 = 0x8;
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] |= 0xffff; // -> all ones
    Addr((c.g[A1][0] as u32).wrapping_add(SLOT0)).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// The application tick.

/// `int SSXApp_NextTick(App* self)` (0x227e98): advance the pad queue and
/// hand both pads to the input system. Returns 0 when there is no app or the
/// read side of the queue is still ahead of the game, else 1.
pub fn ssx_app_next_tick(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw(gp(c, -0x850));
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        let no_app = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        let mut ticked = false;
        if !no_app {
            c.cycles += 4;
            let queue = c.g[V0][0];
            c.g[A0][0] = queue;
            callee::PAD_QUEUE_ADVANCE_WRITE.call(c, 0x0022_7ec0)?;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw(gp(c, -0x238));
            if !idle {
                // The comm system and the USB keyboard, when present.
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[A0][0] = c.lw(gp(c, 0x300));
                } else {
                    c.cycles += 3;
                    callee::COMM_SYSTEM_UPDATE.call(c, 0x0022_7ed8)?;
                    c.g[A0][0] = c.lw(gp(c, 0x300));
                }
                c.cycles += 2;
                let no_keyboard = c.g[A0][0] == 0;
                c.g[S2][0] = sx(r32(c, S0).wrapping_add(0xa8));
                if !no_keyboard {
                    c.cycles += 2;
                    callee::USB_KEYBOARD_UPDATE.call(c, 0x0022_7eec)?;
                }
                // Both pads.
                c.cycles += 3;
                c.g[S1][0] = 0;
                c.g[A0][0] = c.lw(gp(c, -0x850));
                loop {
                    c.cycles += 15;
                    c.g[A1][0] = c.g[S1][0];
                    callee::PAD_QUEUE_GET_PAD_STATUS.call(c, 0x0022_7f00)?;
                    c.g[A0][0] = c.lw(gp(c, -0x850));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = c.g[V0][0];
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x1));
                    callee::PAD_QUEUE_GET_PAD_DATA.call(c, 0x0022_7f14)?;
                    c.g[A0][0] = c.lw(r32(c, S2));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    addi(c, S2, 0x4);
                    callee::INPUT_PAD_UPDATE.call(c, 0x0022_7f28)?;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lw(gp(c, -0x850));
                    if !more {
                        c.cycles += 2;
                        c.g[V0][0] = 0x1;
                        ticked = true;
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_7ef8);
                    }
                }
            }
        }
        if !ticked {
            c.cycles += 1;
            c.g[V0][0] = 0;
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

// ---------------------------------------------------------------------------
// VU0.

/// `void Vu0_UploadOccluder(int index, const Vec4* v, const Mat4* m)`
/// (0x229f88): copy the vertex and the four matrix rows into VU0 data memory
/// at quadword `index * 5 + 0x50` (the occluder plane slot of a view).
pub fn vu0_upload_occluder(c: &mut Cpu) {
    c.cycles += 16;
    c.g[V0][0] = sx(abi::arg(c, 0) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(abi::arg(c, 0)));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x50));
    c.ctc2(2, 1);
    let v = abi::arg(c, 1);
    let m = abi::arg(c, 2);
    c.lqc2(1, v);
    c.lqc2(2, m);
    c.lqc2(3, p(m, 0x10));
    c.lqc2(4, p(m, 0x20));
    c.lqc2(5, p(m, 0x30));
    c.vu0().sqi(0b1111, 1, 1);
    c.vu0().sqi(0b1111, 2, 1);
    c.vu0().sqi(0b1111, 3, 1);
    c.vu0().sqi(0b1111, 4, 1);
    c.vu0().sqi(0b1111, 5, 1);
    c.pc = c.g[RA][0] as u32;
}

/// `void *MCManager_Get(this)` (0x00227f80): the `cMCOverlayManager`
/// singleton (`MCManager_GetInstance`).
pub fn mc_manager_get(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |_| {},
        callee::MC_MANAGER_GET_INSTANCE,
        0x0022_7f90,
    )
}

// ---------------------------------------------------------------------------
// World triggers.

/// `void WorldTriggerManager_DestroyInstance(void)` (0x2b4b48): destroy the
/// trigger manager singleton (with `a1 = 3`, the delete flavour) and clear
/// the pointer.
pub fn world_trigger_manager_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(glo(c, g::WORLD_TRIGGER_MANAGER)).lw(c);
        let mgr = c.g[A0][0];
        frame.save_ra(c, 0);
        if mgr != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            callee::WORLD_TRIGGER_MANAGER_DTOR.call(c, 0x002b_4b60)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        Addr(glo(c, g::WORLD_TRIGGER_MANAGER)).sw(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `CrowdMan2d_Clear(this)` 0x229498: remove every slot and reset the manager.
pub fn crowd_man2d_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        callee::CROWD_MAN2D_REMOVE_ALL.call(c, 0x0022_94ac)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CROWD_MAN2D_RESET.call(c, 0x0022_94b4)?;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Crowd manager.

/// `void CrowdMan2d_Add(CrowdMan2d* m, int mode)` (0x229788): find the first
/// slot of the 64 entries at +8 whose word is -1 and initialise it with
/// `mode` (and the caller's second argument). 8 + 3 (+3 (+1), +3) per slot +
/// 2 cycles plus the callee. The event check resumes at 0x2297a8.
pub fn crowd_man2d_add(c: &mut Cpu) {
    guest(c, |c| {
        let resume = c.pc == 0x0022_97a8;
        if !resume {
            c.cycles += 8;
            let frame = Frame::open(c, 0x10);
            c.g[A3][0] = 0xffff_ffff_ffff_ffff;
            c.g[A2][0] = c.g[A1][0];
            frame.save_ra(c, 0);
            c.g[A1][0] = 0;
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(8));
        } else {
            c.cycles += 1;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        }
        let frame = Frame { size: 0x10 };
        loop {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 64i64) as u64;
            let exhausted = c.g[V0][0] == 0;
            frame.restore_ra(c, 0);
            if exhausted {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, V1));
            let taken = c.g[V0][0] != c.g[A3][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            if taken {
                if back_edge_unwinds(c, 0x0022_97a8) {
                    return Ok(());
                }
                c.cycles += 1;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                continue;
            }
            c.cycles += 3;
            callee::CROWD_MAN2D_INIT_SLOT.call(c, 0x0022_97cc)?;
            frame.restore_ra(c, 0);
            break;
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void SSXApp_Init(SSXApp* self, int argc, char** argv)` (0x226900): boot
/// the application. Registers the app at `gp-0x848`, creates the pad queue
/// (`gp-0x850`), the PS2 graphics manager (`gp-0x854`, then its vtable slot
/// 0x10 init), the light and bezier managers, the small allocations (the byte
/// at `gp-0x844`, the two 0x2a4-byte pad records at +0xa8), parses the command
/// line, sets the graphics mode (slot 0x160, 0x100), loads the boot shape
/// (`SHPS_Find` + slot 0x170) into the app's +0xa0, makes the 0x1c-byte tint
/// (+0xa4, white), hands the pre-start screen to `App_SetNextState`, and
/// records the game mode manager (+0xc0) and the async front end manager
/// (+0x11c).
///
/// Not in `OVERRIDES`: its callees (graphics manager creation and init, the
/// boot shape load from disc) cannot be bounded in a differential test; the
/// port is otherwise exact (see the ignored `ssx_app_init_matches`).
pub fn ssx_app_init(c: &mut Cpu) {
    /// `SSXApp::mPadRecords[2]` (0x2a4 bytes each).
    const PAD_RECORDS: u32 = 0xa8;
    /// What `func_002C7440` (statemachine.cpp) returns.
    const STATE_CONFIG: u32 = 0x9c;
    const BOOT_SHAPE: u32 = 0xa0;
    const TINT: u32 = 0xa4;
    const GAME_MODE_MAN: u32 = 0xc0;
    const FE_ASYNC_MANAGER: u32 = 0x11c;
    /// `PS2GraphicsMan::vtable` pointer.
    const GFX_VTABLE: u32 = 0x10d8;
    /// `EA_MemAllocNamed(size, name, 0, 0)`.
    const ALLOC: u32 = 0x0031_7d70;
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x80);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x70);
        let app = r32(c, S2);
        c.g[S0][0] = sx(app.wrapping_add(PAD_RECORDS));
        frame.save(c, S5, 0x30);
        c.g[S1][0] = 1;
        frame.save_ra(c, 0x20);
        c.g[S5][0] = 0x0048_0000;
        Addr(gp(c, -0x848)).sw(c, c.g[S2][0]);
        // func_002C7440: the state machine configuration.
        call_ptr(c, 0x002c_7440, 0x0022_6940)?;

        // The pad queue (0x2efc bytes), kept in a stack slot.
        alloc_args(c, 0x2efc, 0x0047_9a40);
        Addr(app).off(STATE_CONFIG).sw(c, c.g[V0][0]);
        call_ptr(c, ALLOC, 0x0022_695c)?;
        Addr(sp(c)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(sp(c)).lw(c);
        // PadQueue_ctor
        call_ptr(c, 0x0032_6a28, 0x0022_696c)?;
        c.g[V0][0] = Addr(sp(c)).lw(c);
        Addr(gp(c, -0x850)).sw(c, c.g[V0][0]);

        // The graphics manager, then its init method (vtable slot 0x10).
        // PS2GraphicsMan_Create
        call_ptr(c, 0x0037_5a08, 0x0022_6978)?;
        let gfx = c.g[V0][0];
        c.g[V1][0] = Addr(gfx as u32).off(GFX_VTABLE).lw(c);
        Addr(gp(c, -0x854)).sw(c, gfx);
        let init = VSlot::read(c, c.g[V1][0], 0x10);
        c.g[A1][0] = init.func;
        c.g[A0][0] = init.this(gfx);
        call_ptr(c, init.func as u32, 0x0022_6990)?;
        // LightMan_construct, BezierMan_construct
        call_ptr(c, 0x0038_dbd0, 0x0022_6998)?;
        call_ptr(c, 0x0038_aef8, 0x0022_69a0)?;

        // One byte at gp-0x844.
        alloc_args(c, 1, 0x0047_9a50);
        call_ptr(c, ALLOC, 0x0022_69b8)?;
        Addr(gp(c, -0x844)).sw(c, c.g[V0][0]);

        // The two pad records, each with its first word cleared.
        loop {
            c.cycles += 10;
            c.g[A0][0] = 0x2a4;
            c.g[A1][0] = sx(r32(c, S5).wrapping_sub(0x65a0));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, ALLOC, 0x0022_69d4)?;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            let record = r32(c, V0);
            Addr(r32(c, S0)).sw(c, c.g[V0][0]);
            Addr(record).sw(c, 0);
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_69c0);
            }
        }

        c.cycles += 94;
        Addr(app).off(0x84).sw(c, 0);
        c.g[A2][0] = c.g[S3][0];
        Addr(app).off(0x80).sw(c, 0);
        c.g[A0][0] = c.g[S2][0];
        Addr(app).off(0x7c).sw(c, 0);
        c.g[A1][0] = c.g[S4][0];
        // SSXApp_ParseCommandLine(self, argc, argv)
        call_ptr(c, 0x0022_7bf0, 0x0022_6a04)?;

        // Graphics vtable slot 0x160 (0x100).
        c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
        c.g[A1][0] = 0x100;
        c.g[V0][0] = Addr(r32(c, A2)).off(GFX_VTABLE).lw(c);
        let mode = VSlot::read(c, c.g[V0][0], 0x160);
        c.g[V1][0] = mode.func;
        c.g[A0][0] = mode.this(c.g[A2][0]);
        call_ptr(c, mode.func as u32, 0x0022_6a20)?;

        // The boot shape: SHPS_Find(0x47ab40, 0x4a28b0), then slot 0x170
        // loads it (name 0x479a70, 0, 1, -1).
        c.g[S0][0] = Addr(gp(c, -0x854)).lw(c);
        c.g[A0][0] = 0x0047_ab40;
        c.g[A1][0] = 0x004a_28b0;
        c.g[S1][0] = Addr(r32(c, S0)).off(GFX_VTABLE).lw(c);
        c.g[V0][0] = Addr(r32(c, S1)).off(0x170).lh(c);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        call_ptr(c, 0x003b_39b8, 0x0022_6a44)?;
        c.g[T2][0] = Addr(r32(c, S1)).off(0x174).lw(c);
        c.g[T0][0] = 1;
        c.g[T1][0] = 0xffff_ffff_ffff_ffff;
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0x0047_9a70;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, r32(c, T2), 0x0022_6a68)?;
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[A0][0] = Addr(gp(c, -0x854)).lw(c);
        Addr(r32(c, V1)).off(BOOT_SHAPE).sw(c, c.g[V0][0]);
        // func_00369FF8(gfx)
        call_ptr(c, 0x0036_9ff8, 0x0022_6a78)?;

        // The tint: { 0, 0, 0, 1.0, 1.0, 1.0, 1.0 }, built on the stack.
        alloc_args(c, 0x1c, 0x0047_9a80);
        call_ptr(c, ALLOC, 0x0022_6a90)?;
        let tint = r32(c, V0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[A1][0] = 0x0048_0000;
        Addr(tint).sw(c, 0);
        c.g[A2][0] = 0x100;
        frame_swc1(c, 0x1c);
        c.g[A3][0] = 0;
        frame_swc1(c, 0x10);
        c.g[A1][0] = 0x0047_9a90;
        frame_swc1(c, 0x14);
        frame_swc1(c, 0x18);
        Addr(tint).off(4).sw(c, 0);
        Addr(tint).off(8).sw(c, 0);
        Addr(app).off(TINT).sw(c, c.g[V0][0]);
        // The unaligned doubleword copy (ldl/ldr, sdl/sdr) of the four
        // floats into +0xc..+0x1c.
        c.g[V1][0] = c.ld(sp(c).wrapping_add(0x10));
        c.g[T0][0] = c.ld(sp(c).wrapping_add(0x18));
        let (lo, hi) = (c.g[V1][0], c.g[T0][0]);
        Addr(tint).off(0xc).sw(c, lo);
        Addr(tint).off(0x10).sw(c, lo >> 32);
        Addr(tint).off(0x14).sw(c, hi);
        Addr(tint).off(0x18).sw(c, hi >> 32);

        // The pre-start screen (0xc bytes, flags 0x100) as the next state.
        c.g[A0][0] = 0xc;
        call_ptr(c, ALLOC, 0x0022_6af0)?;
        c.g[A0][0] = c.g[V0][0];
        // PreStartScreen_ctor
        call_ptr(c, 0x0023_2328, 0x0022_6af8)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        // App_SetNextState(self, screen)
        call_ptr(c, 0x0031_6ef8, 0x0022_6b04)?;
        // GameModeMan_GetGM
        call_ptr(c, 0x0023_7cf8, 0x0022_6b0c)?;
        alloc_args(c, 0x590, 0x0047_9aa0);
        Addr(app).off(GAME_MODE_MAN).sw(c, c.g[V0][0]);
        call_ptr(c, ALLOC, 0x0022_6b28)?;

        // The async front end manager: construct, store, start.
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001a_3590, 0x0022_6b30)?;
        Addr(app).off(FE_ASYNC_MANAGER).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001a_3648, 0x0022_6b3c)?;

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

/// `EA_MemAllocNamed(size, name, 0, 0)` arguments as `SSXApp_Init` sets
/// them (`a0` last).
fn alloc_args(c: &mut Cpu, size: u64, name: u64) {
    c.g[A1][0] = name;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    c.g[A0][0] = size;
}

/// `swc1 f0, off(sp)`.
fn frame_swc1(c: &mut Cpu, off: u32) {
    c.swc1(0, sp(c).wrapping_add(off));
}

/// `void SSXApp_StartGameModule(SSXApp* self)` (0x2281d8): allocate and
/// construct the game module, hand it to the app manager, and record the game
/// mode manager.
pub fn ssx_app_start_game_module(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x0048_0000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x57f8));
        c.g[A0][0] = 0x22c;
        call_ptr(c, 0x0031_7d70, 0x0022_8200)?;
        c.g[A0][0] = c.g[V0][0];
        callee::GAME_MODULE_CTOR.call(c, 0x0022_8208)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        sw(c, p_2(c.g[S0][0], 0x84), c.g[V0][0]);
        call_ptr(c, 0x0031_6ef8, 0x0022_8218)?;
        call_ptr(c, 0x0023_7cf8, 0x0022_8220)?;
        sw(c, p_2(c.g[S0][0], 0xc0), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXApp_StartFEModule(SSXApp* self)` (0x228238): clear the global at
/// `gp-0x10b4`, then allocate and construct the front end module (or, when
/// the app's front end flag at +0x60 is clear, the debug menu module), store
/// it in the app (+0x7c front end, +0x80 debug menu) and hand it to the app
/// manager as the next state.
pub fn ssx_app_start_fe_module(c: &mut Cpu) {
    /// `SSXApp::mUseFrontEnd`: 0 boots the debug menu instead.
    const USE_FRONT_END: u32 = 0x60;
    const FE_MODULE: u32 = 0x7c;
    const DEBUG_MENU_MODULE: u32 = 0x80;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let gp = c.g[GP][0] as u32;
        Addr(gp.wrapping_sub(0x10b4)).sw(c, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let app = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(app).off(USE_FRONT_END).lw(c);
        c.g[A1][0] = 0x0048_0000;
        // Both allocations are named with the string at 0x47a818.
        let (size, ctor, field, ret) = if c.g[V0][0] != 0 {
            c.cycles += 14;
            (0x000b_5ae0, 0x001a_1ce8, FE_MODULE, 0x0022_8270)
        } else {
            c.cycles += 12;
            (0x238, 0x0017_f7b8, DEBUG_MENU_MODULE, 0x0022_82a4)
        };
        c.g[A0][0] = size;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x57e8));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        // EA_MemAllocNamed(size, name, 0, 0)
        call_ptr(c, 0x0031_7d70, ret)?;
        c.g[A0][0] = c.g[V0][0];
        // FEModule_ctor / DebugMenuModule_ctor
        call_ptr(c, ctor, ret + 8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        Addr(app).off(field).sw(c, c.g[V0][0]);
        // App_SetNextState(self, module)
        call_ptr(c, 0x0031_6ef8, ret + 0x18)?;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CrowdMan2d_dtor(CrowdMan2d* self, int flags)` (0x229278): empty the crowd,
/// free the render object, then delete the object when the low flag bit is
/// set.
pub fn crowd_man2d_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0022_9398, 0x0022_9294)?;
        c.g[A0][0] = lw(c, p(this as u32, 4));
        sw(c, gp_2(c, -0x6f0), 0);
        call_ptr(c, 0x0031_7e50, 0x0022_92a0)?;
        c.g[A0][0] = lw(c, this as u32);
        let no_render = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !no_render {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002d_b700, 0x0022_92b4)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_7e50, 0x0022_92c8)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Nash_ClassifyPatch`'s table (`0x480000 - 0x4ca0`), indexed by the heap
/// handle (only handles < 5 are looked up).
pub(crate) const NASH_PATCH_TABLE: u32 = 0x0047_b360;

/// `Nash_ClassifyInstance`'s table (`0x480000 - 0x4cc0`).
pub(crate) const NASH_INSTANCE_TABLE: u32 = 0x0047_b340;

/// `void Nash_ClassifyPatch(Cell *cell, Node *node, int frustum)` (0x22a128):
/// classify one octree node under the cell's heap. A null node takes its
/// handle from VU0 `VI1`; otherwise the node's signed word at `+0x156` is
/// checked for a handle and its byte at `+0x155` indexes the cell's heap
/// (`heap + byte*8 + 0x24`, the slot word must be 6), and the handle selects
/// `heap + handle*0x18 + 0x3f0` whose word must be 3. A hit runs the VU0
/// bounding-box/frustum classification, which leaves the handle in `VI1`
/// (`cfc2`); a handle of five or more stores the node at the cell's list
/// head (`+0x78b0`), a smaller one picks a handler from the cell's table
/// (which appends the old head at `+0x50ac`/`+0x70ac` and then stores too).
///
/// The function makes no calls and checks no events, so the only entry the
/// trampoline can ever use is the top (the generated `__entries` twin's table
/// targets are only reachable from inside).
pub fn nash_classify_patch(c: &mut Cpu) {
    c.cycles += 3;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    let cell = r32(c, A0);
    c.g[A3][0] = c.g[A0][0];
    let node = r32(c, A1);
    let heap;
    if node == 0 {
        c.cycles += 2;
        c.cfc2(4, 1);
        c.g[V0][0] = (c.g[A0][0] < 0x5) as u64;
    } else {
        // 0x22a134: the node's handle and slot.
        c.cycles += 5;
        c.g[V0][0] = Addr(cell).off(0x90).lw(c);
        c.g[T0][0] = Addr(node).off(0x156).lh(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        heap = r32(c, A0);
        c.g[V0][0] = 0;
        if (c.g[T0][0] as i64) >= 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(node).off(0x155).lbu(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V0][0] = sx(heap.wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
            c.g[V1][0] ^= 0x6;
            if c.g[V1][0] == 0 {
                c.cycles += 6;
                c.g[V0][0] = 0x18;
                let i = abi::mult(c, r32(c, T0), 0x18);
                c.g[V1][0] = sx(i);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(heap));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x3f0).lw(c);
                c.g[V1][0] ^= 0x3;
                c.g[V0][0] = (c.g[V1][0] < 0x1) as u64;
            } else {
                c.g[V0][0] = 0;
            }
        }
        // 0x22a17c: a node that did not classify jumps straight to the
        // epilogue, skipping the insert block and its two cycles.
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
            abi::ret(c);
            return;
        }
        // 0x22a184: build the node's box on the stack, classify it with the
        // cell's VU0 routine and keep the handle test for the tail.
        c.cycles += 26;
        for i in 0..6u32 {
            c.lwc1(i as usize, node.wrapping_add(0x158 + 4 * i));
        }
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 6);
        c.swc1(0, sp(c));
        c.swc1(1, sp(c).wrapping_add(0x4));
        c.swc1(2, sp(c).wrapping_add(0x8));
        c.swc1(3, sp(c).wrapping_add(0x10));
        c.swc1(4, sp(c).wrapping_add(0x14));
        c.swc1(5, sp(c).wrapping_add(0x18));
        c.swc1(6, sp(c).wrapping_add(0x1c));
        c.swc1(6, sp(c).wrapping_add(0xc));
        c.cfc2(4, 1);
        c.g[V1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[V0][0] = sx(node.wrapping_add(0x140));
        c.lqc2(20, sp(c));
        c.lqc2(21, r32(c, V1));
        c.lqc2(22, r32(c, V0));
        c.ctc2(6, 14);
        c.vcallms(0xdb8);
        c.g[V0][0] = (c.g[A0][0] < 0x5) as u64;
    }
    // 0x22a1f4/0x22a260: `cfc2` left the VU0 handle in a0. Five or more
    // stores the node at the cell's list head; a smaller handle indexes the
    // cell's table, whose handlers also end in that store.
    c.cycles += 2;
    let high = c.g[V0][0] == 0;
    c.g[V0][0] = 0x0048_0000;
    let mut insert = true;
    if !high {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx(NASH_PATCH_TABLE);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        match c.g[A0][0] as u32 {
            0x0022_a214 => {
                c.cycles += 3;
                c.g[A0][0] = Addr(cell).off(0x78b0).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(cell).off(0x78b0).sw(c, c.g[A1][0]);
                    insert = false;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(cell).off(0x50a8).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx(cell.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    Addr(c.g[V0][0] as u32).off(0x50ac).sw(c, c.g[A0][0]);
                    Addr(cell).off(0x50a8).sw(c, c.g[V1][0]);
                }
            }
            0x0022_a260 => {}
            0x0022_a23c => {
                c.cycles += 3;
                c.g[A0][0] = Addr(cell).off(0x78b0).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(cell).off(0x78b0).sw(c, c.g[A1][0]);
                    insert = false;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = Addr(cell).off(0x70ac).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx(cell.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    Addr(c.g[V0][0] as u32).off(0x70b0).sw(c, c.g[A0][0]);
                    Addr(cell).off(0x70ac).sw(c, c.g[V1][0]);
                }
            }
            target => {
                c.jump_out(target, 0x0022_a128, 0x0022_a26c);
                return;
            }
        }
    }
    if insert {
        c.cycles += 1;
        Addr(cell).off(0x78b0).sw(c, c.g[A1][0]);
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
    abi::ret(c);
}

/// `void Nash_ClassifyInstance(Cell *cell, Node *node, int frustum)`
/// (0x229fc8): the same classification for the other node flavour. The node
/// is skipped unless its `+0x8` word ends in 3; its byte at `+0x7d` (0xff
/// means "no slot") indexes `heap + byte*8 + 0x24` (slot word must be 6) and
/// its signed halfword at `+0x7e` selects `heap + handle*0x18 + 0x3f0` (word
/// must be 3). A hit classifies the box at `+0x60`; the `VI1` handle that
/// leaves the VU0 routine then either stores the node at `+0x50a4` (five or
/// more) or dispatches through the same table (appending at `+0x20a0` or
/// `+0x40a0` before that store).
///
/// Like `Nash_ClassifyPatch` it makes no calls and checks no events, so the
/// top is its only entry.
pub fn nash_classify_instance(c: &mut Cpu) {
    c.cycles += 3;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    let cell = r32(c, A0);
    c.g[A3][0] = c.g[A0][0];
    let node = r32(c, A1);
    let heap: u32;
    if node == 0 {
        c.cycles += 2;
        c.cfc2(4, 1);
        c.g[V0][0] = (c.g[A0][0] < 0x5) as u64;
    } else {
        // 0x229fd4: only nodes whose type word ends in 3 are classified.
        c.cycles += 5;
        c.g[V0][0] = Addr(node).off(0x8).lw(c);
        c.g[V1][0] = 0x3;
        c.g[V0][0] &= 0x3;
        if c.g[V0][0] != c.g[V1][0] {
            c.g[V0][0] = 0;
        } else {
            // 0x229fe8: the node's slot byte indexes the cell's heap.
            c.cycles += 6;
            c.g[V0][0] = Addr(cell).off(0x90).lw(c);
            c.g[A0][0] = 0xff;
            c.g[V1][0] = Addr(node).off(0x7d).lbu(c);
            c.g[T0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = 0x1;
            heap = r32(c, T0);
            if c.g[V1][0] != c.g[A0][0] {
                c.cycles += 7;
                c.g[V0][0] = Addr(node).off(0x7d).lbu(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
                c.g[V1][0] ^= 0x6;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(node).off(0x7e).lh(c);
                    c.g[A0][0] = 0x18;
                    let i = abi::mult(c, r32(c, V0), 0x18);
                    c.g[V1][0] = sx(i);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(heap));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x3f0).lw(c);
                    c.g[V1][0] ^= 0x3;
                    c.g[V0][0] = (c.g[V1][0] < 0x1) as u64;
                }
            }
        }
        // 0x22a038: a classified node is tested again with the VU0 routine;
        // a miss jumps to the epilogue, skipping the insert block's cycles.
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
            abi::ret(c);
            return;
        }
        c.cycles += 26;
        for i in 0..6u32 {
            c.lwc1(i as usize, node.wrapping_add(0x60 + 4 * i));
        }
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 6);
        c.swc1(0, sp(c));
        c.swc1(1, sp(c).wrapping_add(0x4));
        c.swc1(2, sp(c).wrapping_add(0x8));
        c.swc1(3, sp(c).wrapping_add(0x10));
        c.swc1(4, sp(c).wrapping_add(0x14));
        c.swc1(5, sp(c).wrapping_add(0x18));
        c.swc1(6, sp(c).wrapping_add(0x1c));
        c.swc1(6, sp(c).wrapping_add(0xc));
        c.cfc2(4, 1);
        c.g[V1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[V0][0] = sx(node.wrapping_add(0x50));
        c.lqc2(20, sp(c));
        c.lqc2(21, r32(c, V1));
        c.lqc2(22, r32(c, V0));
        c.ctc2(6, 14);
        c.vcallms(0xdb8);
        c.g[V0][0] = (c.g[A0][0] < 0x5) as u64;
    }
    // 0x22a0b0/0x22a11c: `cfc2` left the VU0 handle in a0. Five or more
    // stores the node at the cell's list head; a smaller handle indexes the
    // cell's table, whose handlers also end in that store.
    c.cycles += 2;
    let high = c.g[V0][0] == 0;
    c.g[V0][0] = 0x0048_0000;
    let mut insert = true;
    if !high {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx(NASH_INSTANCE_TABLE);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        match c.g[A0][0] as u32 {
            0x0022_a0d0 => {
                c.cycles += 3;
                c.g[A0][0] = Addr(cell).off(0x50a4).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(cell).off(0x50a4).sw(c, c.g[A1][0]);
                    insert = false;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(cell).off(0x209c).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx(cell.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    Addr(c.g[V0][0] as u32).off(0x20a0).sw(c, c.g[A0][0]);
                    Addr(cell).off(0x209c).sw(c, c.g[V1][0]);
                }
            }
            0x0022_a11c => {}
            0x0022_a0f8 => {
                c.cycles += 3;
                c.g[A0][0] = Addr(cell).off(0x50a4).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(cell).off(0x50a4).sw(c, c.g[A1][0]);
                    insert = false;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = Addr(cell).off(0x40a0).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx(cell.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    Addr(c.g[V0][0] as u32).off(0x40a4).sw(c, c.g[A0][0]);
                    Addr(cell).off(0x40a0).sw(c, c.g[V1][0]);
                }
            }
            target => {
                c.jump_out(target, 0x0022_9fc8, 0x0022_a128);
                return;
            }
        }
    }
    if insert {
        c.cycles += 1;
        Addr(cell).off(0x50a4).sw(c, c.g[A1][0]);
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
    abi::ret(c);
}

/// `void Nash_EmptyHook(void)` (0x229f80): the empty Nash (effect) callback.
pub fn nash_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Nash_ClassifyEffect: a jump-table dispatcher (VU0 setup, then three
// per-effect registration lists).
//
// The port follows the generated `__entries` twin block for block, so a
// thread switch that resumes it at any of the table's case labels behaves
// like the generated function.

/// The two registration lists `Nash_ClassifyEffect` appends to (word at
/// `self + 0x7ab8` is the head; a count at `+0x78b4`/`+0x7abc` indexes the
/// inline arrays at `+0x78b8`/`+0x7ac0`). The effect type also selects one of
/// three lists in the table at `.rodata 0x47b380`.
pub fn nash_classify_effect(c: &mut Cpu) {
    let mut pc = c.pc as u32;
    loop {
        match pc {
            0x0022_a270 => {
                c.cycles += 3;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                let null_node = c.g[A1][0] == 0;
                c.g[A3][0] = c.g[A0][0];
                pc = if null_node { 0x0022_a2e4 } else { 0x0022_a27c };
            }
            0x0022_a27c => {
                c.cycles += 26;
                let node = c.g[A1][0] as u32;
                let sp = c.g[SP][0] as u32;
                c.lwc1(0, node.wrapping_add(0x68));
                c.lwc1(1, node.wrapping_add(0x6c));
                c.lwc1(2, node.wrapping_add(0x70));
                c.lwc1(3, node.wrapping_add(0x74));
                c.lwc1(4, node.wrapping_add(0x78));
                c.lwc1(5, node.wrapping_add(0x7c));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 6);
                c.swc1(0, sp);
                c.swc1(1, sp.wrapping_add(0x4));
                c.swc1(2, sp.wrapping_add(0x8));
                c.swc1(3, sp.wrapping_add(0x10));
                c.swc1(4, sp.wrapping_add(0x14));
                c.swc1(5, sp.wrapping_add(0x18));
                c.swc1(6, sp.wrapping_add(0x1c));
                c.swc1(6, sp.wrapping_add(0xc));
                c.cfc2(A0, 1);
                c.g[V1][0] = sx(sp.wrapping_add(0x10));
                c.g[V0][0] = sx(node.wrapping_add(0x50));
                c.lqc2(20, sp);
                c.lqc2(21, c.g[V1][0] as u32);
                c.lqc2(22, c.g[V0][0] as u32);
                c.ctc2(6, 14);
                c.vcallms(0xdb8);
                c.g[V0][0] = (c.g[A0][0] < 0x5u64) as u64;
                pc = 0x0022_a2ec;
            }
            0x0022_a2e4 => {
                c.cycles += 2;
                c.cfc2(A0, 1);
                c.g[V0][0] = (c.g[A0][0] < 0x5u64) as u64;
                pc = 0x0022_a2ec;
            }
            0x0022_a2ec => {
                c.cycles += 2;
                let out_of_range = c.g[V0][0] == 0;
                c.g[V0][0] = 0x48_0000;
                pc = if out_of_range {
                    0x0022_a358
                } else {
                    0x0022_a2f4
                };
            }
            0x0022_a2f4 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4c80));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let target = c.g[A0][0] as u32;
                pc = match target {
                    0x0022_a30c => 0x0022_a30c,
                    0x0022_a334 => 0x0022_a334,
                    0x0022_a358 => 0x0022_a358,
                    _ => {
                        c.jump_out(target, 0x0022_a270, 0x0022_a364);
                        return;
                    }
                };
            }
            0x0022_a30c => {
                c.cycles += 3;
                let this = c.g[A3][0] as u32;
                c.g[A0][0] = Addr(this.wrapping_add(0x7ab8)).lw(c);
                if c.g[A0][0] == 0 {
                    let v = c.g[A1][0];
                    Addr(this.wrapping_add(0x7ab8)).sw(c, v);
                    pc = 0x0022_a35c;
                } else {
                    pc = 0x0022_a318;
                }
            }
            0x0022_a318 => {
                c.cycles += 7;
                let head = c.g[A3][0] as u32;
                c.g[V1][0] = Addr(head.wrapping_add(0x78b4)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(head));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                let slot = c.g[V0][0] as u32;
                let prev = c.g[A0][0];
                Addr(slot.wrapping_add(0x78b8)).sw(c, prev);
                let count = c.g[V1][0];
                Addr(head.wrapping_add(0x78b4)).sw(c, count);
                pc = 0x0022_a358;
            }
            0x0022_a334 => {
                c.cycles += 3;
                let this = c.g[A3][0] as u32;
                c.g[A0][0] = Addr(this.wrapping_add(0x7ab8)).lw(c);
                if c.g[A0][0] == 0 {
                    let v = c.g[A1][0];
                    Addr(this.wrapping_add(0x7ab8)).sw(c, v);
                    pc = 0x0022_a35c;
                } else {
                    pc = 0x0022_a340;
                }
            }
            0x0022_a340 => {
                c.cycles += 6;
                let head = c.g[A3][0] as u32;
                c.g[V1][0] = Addr(head.wrapping_add(0x7abc)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(head));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                let slot = c.g[V0][0] as u32;
                let prev = c.g[A0][0];
                Addr(slot.wrapping_add(0x7ac0)).sw(c, prev);
                let count = c.g[V1][0];
                Addr(head.wrapping_add(0x7abc)).sw(c, count);
                pc = 0x0022_a358;
            }
            0x0022_a358 => {
                c.cycles += 1;
                let this = c.g[A3][0] as u32;
                let node = c.g[A1][0];
                Addr(this.wrapping_add(0x7ab8)).sw(c, node);
                pc = 0x0022_a35c;
            }
            0x0022_a35c => {
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x0022_a270, 0x0022_a364);
                return;
            }
        }
    }
}

/// `void CrowdMan2d_RemoveAll(CrowdMan2d* m)` (0x229398): clear all 128 crowd
/// slots that are not already empty.
pub fn crowd_man2d_remove_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        sqsp(c, 18, 0x20);
        sqsp(c, 19, 0x10);
        c.g[S2][0] = 0xffff_ffff_ffff_ffff;
        sqsp(c, 16, 0x40);
        c.g[S3][0] = c.g[A0][0];
        sqsp(c, 17, 0x30);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(8));
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            let empty = c.g[V0][0] == c.g[S2][0];
            c.g[A0][0] = c.g[S3][0];
            if !empty {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call::call_ptr(c, 0x0022_9b90, 0x0022_93dc)?;
            }
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 128) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_93c8);
            }
        }
        c.cycles += 7;
        lqsp(c, 16, 0x40);
        lqsp(c, 17, 0x30);
        lqsp(c, 18, 0x20);
        lqsp(c, 19, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// 2D crowd.

/// `void CrowdMan2d_RemoveByType(CrowdMan2d* self, int id)` (0x229408):
/// clear the 128 slots whose type byte is `id`.
pub fn crowd_man2d_remove_by_type(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S2][0] = c.g[S2][0] | 0xffff;
        frame.save(c, S4, 0x10);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        c.g[S0][0] = sx(p(r32(c, S3), 8));
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if c.g[V0][0] == c.g[S2][0] {
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
                if c.g[V0][0] != c.g[S4][0] {
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    callee::CROWD_MAN2D_CLEAR_SLOT.call(c, 0x0022_9464)?;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 128i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_9440);
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

// ---------------------------------------------------------------------------
// Crowd.

/// `void CrowdMan2d_Reset(CrowdMan2d* self)` (0x2292e0): clear the 0x2000 /
/// 0x500 byte blocks at_2 +0x210 / +0x2220, set the 128 ids to -1 with a random
/// 300..599 ms delay each, and clear the counts.
pub fn crowd_man2d_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        open(c, 0x70);
        c.g[A1][0] = 0;
        c.sq(20, at_2(c.g[SP][0] as u32, 0x20));
        c.g[A2][0] = 0x2000;
        let this = c.g[A0][0];
        c.g[S4][0] = this;
        c.sq(16, at_2(c.g[SP][0] as u32, 0x60));
        c.sq(17, at_2(c.g[SP][0] as u32, 0x50));
        c.g[A0][0] = sx((this as u32).wrapping_add(0x210));
        c.sq(18, at_2(c.g[SP][0] as u32, 0x40));
        c.g[S1][0] = sx((this as u32).wrapping_add(0x240));
        c.sq(19, at_2(c.g[SP][0] as u32, 0x30));
        c.g[S0][0] = sx((this as u32).wrapping_add(8));
        c.sq(21, at_2(c.g[SP][0] as u32, 0x10));
        c.g[S3][0] = 0xffff_ffff_ffff_0000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S3][0] = c.g[S3][0] | 0xffff;
        Addr(this as u32).off(0x2210).sw(c, 0);
        c.g[S5][0] = 0x12c;
        c.g[RA][0] = sx(0x0022_9330);
        c.g[S2][0] = 0x7f;
        call_ptr(c, 0x003e_6448, 0x0022_9330)?;
        loop {
            c.cycles += 7;
            Addr(c.g[S0][0] as u32).sw(c, c.g[S3][0]);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            c.g[RA][0] = sx(0x0022_9340);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            call_ptr(c, 0x0031_77f0, 0x0022_9340)?;
            c.divu(0, 2, 21);
            c.cycles += 5;
            c.g[V1][0] = c.hi[0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x12c));
            Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
            if !more {
                c.cycles += 14;
                Addr(this as u32).off(0x2214).sw(c, 0);
                c.g[A0][0] = sx((this as u32).wrapping_add(0x2220));
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x500;
                call_ptr(c, 0x003e_6448, 0x0022_9374)?;
                c.lq(16, at_2(c.g[SP][0] as u32, 0x60));
                c.lq(17, at_2(c.g[SP][0] as u32, 0x50));
                c.lq(18, at_2(c.g[SP][0] as u32, 0x40));
                c.lq(19, at_2(c.g[SP][0] as u32, 0x30));
                c.lq(20, at_2(c.g[SP][0] as u32, 0x20));
                c.lq(21, at_2(c.g[SP][0] as u32, 0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                close(c, 0x70);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_9330);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Crowd manager.

/// `void CrowdMan2d_LoadState(CrowdMan2d* self, CrowdState* state)` (0x229e58):
/// clear the crowd, fetch the id table and re-create every live slot.
pub fn crowdman2d_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x250);
        frame.save(c, S3, 0x210);
        frame.save(c, S0, 0x240);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x230);
        frame.save(c, S2, 0x220);
        frame.save_ra(c, 0x200);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0022_9398, 0x0022_9e7c)?;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0022_92e0, 0x0022_9e84)?;
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = c.g[SP][0];
        c.g[A0][0] = 0x7f;
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, c.g[V1][0]);
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
            let more = c.g[A0][0] != c.g[A1][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_9e98);
            }
        }
        c.cycles += 13;
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x200;
        c.g[S2][0] = 0xffff_ffff_ffff_0000;
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[S1][0] = 0;
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        c.g[S2][0] = c.g[S2][0] | 0xffff;
        let read = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, read, 0x0022_9edc)?;
        c.g[S0][0] = c.g[SP][0];
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        loop {
            c.cycles += 6;
            c.g[A1][0] = c.g[S1][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[V0][0];
            let empty = c.g[V0][0] == c.g[S2][0];
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !empty {
                c.cycles += 2;
                call_ptr(c, 0x0022_9910, 0x0022_9f08)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 128i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0022_9ee8);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x240);
        frame.restore(c, S1, 0x230);
        frame.restore(c, S2, 0x220);
        frame.restore(c, S3, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 2D crowd camera flashes.

/// `CrowdMan2d` fields of the flash sprites.
mod crowd2d {
    /// Emitter slots (`+8`, 128 words, -1 = free).
    pub const SLOTS: u32 = 0x8;
    /// Per-emitter spawn timer (`+0x240`, emitters 0x40 apart; their
    /// position and spread vectors at `+0x210`, `+0x220`, `+0x230`).
    pub const TIMER: u32 = 0x240;
    /// The crowd reaction last applied.
    pub const REACTION: u32 = 0x2210;
    /// Flash sprites (`+0x2220`, 40 of 0x20 bytes, lifetime at `+0x10`).
    pub const LIFETIME: u32 = 0x2230;
    /// A flash's lifetime in 60 Hz frames.
    pub const FLASH_FRAMES: u32 = 3;
}

/// `cCrowdAnim2D_Update` (through the function table).
const CROWD_ANIM2D_UPDATE: u32 = 0x002d_bac0;
/// `CrowdAudio_GetInstanceReaction` (through the function table).
const CROWD_AUDIO_GET_INSTANCE_REACTION: u32 = 0x002a_77c8;
const CROWD_MAN2D_SPAWN_SPRITES: Callee = Callee {
    addr: 0x0022_9530,
    f: ee::s05::CrowdMan2d_SpawnSprites,
};
const CROWD_MAN2D_ADD_SPRITE: Callee = Callee {
    addr: 0x0022_9738,
    f: ee::s05::CrowdMan2d_AddSprite,
};

/// `void CrowdMan2d_Update(CrowdMan2d* self)` (0x2294c8): update the crowd
/// animation, publish its frame (`+0x10`) as a half at `0x536690`, age the
/// 40 flash sprites by one tick and spawn new flashes.
///
/// Tick rate: the lifetimes count ticks; [`crowd_man2d_spawn_sprites`] gives
/// new flashes 3 60 Hz frames' worth of ticks, so a flash shows as long at
/// any rate.
pub fn crowd_man2d_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).off(4).lw(c);
        call_ptr(c, CROWD_ANIM2D_UPDATE, 0x0022_94e0)?;
        c.g[V0][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0x27;
        c.g[V1][0] = Addr(r32(c, V0)).off(0x10).lw(c);
        c.sh(0x0053_6690, c.g[V1][0]);
        loop {
            c.cycles += 3;
            let life = Addr(r32(c, A1)).off(crowd2d::LIFETIME);
            c.g[V0][0] = life.lw(c);
            let shown = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            if shown {
                c.cycles += 1;
                life.sw(c, c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            let more = (c.g[A2][0] as i64) >= 0;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_94f8);
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        CROWD_MAN2D_SPAWN_SPRITES.call(c, 0x0022_951c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdMan2d_SpawnSprites(CrowdMan2d* self)` (0x229530): take the
/// crowd's reaction (`|CrowdAudio_GetInstanceReaction(gp+0x410, 0)|`; when it
/// changed, store it and give the animation `max(reaction, 1)`), then count
/// every used emitter's timer down by `reaction * 10 + 4`; an emitter that
/// runs out adds a flash sprite (3 frames) at its position plus a random
/// spread in [-1, 1) of its two spread vectors, and reloads 300..599.
///
/// Tick rate: the timers count down once per 60 Hz frame of this manager
/// ([`crate::tick::frame60_due`]) and a flash lives 3 60 Hz frames in ticks
/// ([`crate::tick::ticks_from_frames60`]), so the flashes per second and
/// their length match 60 Hz. Unchanged at 60.
pub fn crowd_man2d_spawn_sprites(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x110);
        c.g[A1][0] = 0;
        frame.save(c, S1, 0xd0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0xe0);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x410)).lw(c);
        frame.save(c, S2, 0xc0);
        frame.save(c, S3, 0xb0);
        frame.save(c, S4, 0xa0);
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save(c, S7, 0x70);
        frame.save(c, FP, 0x60);
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 22, 0x100);
        frame.save_fpr(c, 21, 0xf8);
        frame.save_fpr(c, 20, 0xf0);
        call_ptr(c, CROWD_AUDIO_GET_INSTANCE_REACTION, 0x0022_9578)?;
        // reaction = |v0|
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = (-1i64 < c.g[A2][0] as i64) as u64;
        c.g[A0][0] = sx(0u32.wrapping_sub(r32(c, A2)));
        c.g[A2][0] = c.g[A0][0];
        let this = r32(c, S1);
        c.g[A1][0] = Addr(this).off(crowd2d::REACTION).lw(c);
        if c.g[V1][0] != 0 {
            c.g[A2][0] = c.g[V0][0];
        }
        let same = c.g[A2][0] == c.g[A1][0];
        c.g[V0][0] = 1;
        if !same {
            c.cycles += 5;
            c.g[V1][0] = (0 < c.g[A2][0] as i64) as u64;
            c.g[A0][0] = Addr(this).off(4).lw(c);
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[A2][0];
            }
            Addr(this).off(crowd2d::REACTION).sw(c, c.g[A2][0]);
            Addr(r32(c, A0)).sw(c, c.g[V0][0]);
        }
        if crate::tick::frame60_due(c, this.wrapping_add(crowd2d::TIMER), 0) {
            count_down_emitters(c)?;
        }
        c.cycles += 15;
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 22, 0x100);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// The emitter loop of [`crowd_man2d_spawn_sprites`] (`s1` = the manager).
fn count_down_emitters(c: &mut Cpu) -> Flow {
    let this = r32(c, S1);
    c.cycles += 18;
    c.g[V0][0] = Addr(this).off(crowd2d::REACTION).lw(c);
    c.g[V1][0] = 0xa;
    c.g[S4][0] = 0xffff_ffff_ffff_0000;
    c.g[S3][0] = 0x7f_0000;
    c.mult(0, V0, V0, V1);
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
    c.mtc1(AT, 22);
    c.g[AT][0] = 0x3f80_0000; // 1.0
    c.mtc1(AT, 21);
    c.g[S4][0] |= 0xffff; // -1: a free slot
    c.g[S3][0] |= 0xffff; // mantissa mask
    c.g[S7][0] = 0x3f80_0000;
    c.g[S6][0] = 300;
    c.g[S0][0] = c.g[S1][0];
    c.g[FP][0] = sx(r32(c, V0).wrapping_add(4));
    c.g[S2][0] = sx(this.wrapping_add(crowd2d::SLOTS));
    c.g[S5][0] = 0x7f;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S2)).lw(c);
        if c.g[V0][0] == c.g[S4][0] {
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x40));
        } else {
            c.cycles += 4;
            let timer = Addr(r32(c, S0)).off(crowd2d::TIMER);
            c.g[V0][0] = timer.lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, FP)));
            let waiting = (c.g[V0][0] as i64) > 0;
            timer.sw(c, c.g[V0][0]);
            if !waiting {
                spawn_flash(c)?;
            }
            c.cycles += 1;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x40));
        }
        c.cycles += 3;
        c.g[S5][0] = sx(r32(c, S5).wrapping_sub(1));
        let more = (c.g[S5][0] as i64) >= 0;
        c.g[S2][0] = sx(r32(c, S2).wrapping_add(4));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0022_95f8);
        }
    }
}

/// Emitter `s0` ran out: add a flash at `+0x210 + u * +0x220 + v * +0x230`
/// (u, v random in [-1, 1)) and reload the timer with 300 + rand % 300.
fn spawn_flash(c: &mut Cpu) -> Flow {
    let em = r32(c, S0);
    c.cycles += 50;
    call_ptr(c, callee::RAND_NEXT_MAIN, 0x0022_961c)?;
    // A float in [1, 2) from the random mantissa, then 2 * (x - 1) - 1.
    c.g[V0][0] &= c.g[S3][0];
    c.g[V0][0] |= c.g[S7][0];
    c.mtc1(V0, 0);
    c.sub_s(20, 0, 21);
    c.add_s(20, 20, 20);
    c.add_s(20, 20, 22);
    call_ptr(c, callee::RAND_NEXT_MAIN, 0x0022_9638)?;
    c.g[V0][0] &= c.g[S3][0];
    c.mfc1(A0, 20);
    c.lqc2(4, em.wrapping_add(0x220));
    c.qmtc2(A0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp(c).wrapping_add(0x30));
    c.g[V0][0] |= c.g[S7][0];
    // The flash's lifetime: 3 frames at 60 Hz, in ticks.
    c.g[A2][0] = crate::tick::ticks_from_frames60(c, crowd2d::FLASH_FRAMES) as u64;
    c.mtc1(V0, 1);
    c.g[A0][0] = c.g[S1][0];
    c.lq(V0, sp(c).wrapping_add(0x30));
    c.g[A1][0] = c.g[SP][0];
    c.sub_s(0, 1, 21);
    c.sq(V0, sp(c).wrapping_add(0x20));
    c.lqc2(3, em.wrapping_add(0x210));
    c.lqc2(4, sp(c).wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x30));
    c.add_s(0, 0, 0);
    c.lq(V0, sp(c).wrapping_add(0x30));
    c.sq(V0, sp(c).wrapping_add(0x10));
    c.add_s(0, 0, 22);
    c.mfc1(V1, 0);
    c.lqc2(4, em.wrapping_add(0x230));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp(c).wrapping_add(0x40));
    c.lq(V0, sp(c).wrapping_add(0x40));
    c.sq(V0, sp(c).wrapping_add(0x30));
    c.lqc2(3, sp(c).wrapping_add(0x10));
    c.lqc2(4, sp(c).wrapping_add(0x30));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x40));
    c.lq(V0, sp(c).wrapping_add(0x40));
    c.sq(V0, sp(c));
    CROWD_MAN2D_ADD_SPRITE.call(c, 0x0022_96c8)?;
    call_ptr(c, callee::RAND_NEXT_MAIN, 0x0022_96d0)?;
    c.divu(0, V0, S6);
    c.cycles += 3;
    c.g[V1][0] = c.hi[0];
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(300));
    Addr(r32(c, S0)).off(crowd2d::TIMER).sw(c, c.g[V1][0]);
    Ok(())
}

pub(crate) mod nash {

    /// The occlusion manager (`OcclusionMan*`), +0x110 its occluder mask.
    pub const OCCLUSION: u32 = 0x00;

    /// Argument passed on to `OcclusionMan_BuildVisible` / the empty hook.
    pub const HOOK_ARG: u32 = 0x04;

    /// The camera view matrix (4 quads).
    pub const VIEW_MATRIX: u32 = 0x10;

    /// The world view matrix (4 quads, copied from graphics +0x5800).
    pub const WORLD_MATRIX: u32 = 0x50;

    /// Section tables (`*tables` = section/logical-section state arrays).
    pub const SECTION_TABLES: u32 = 0x90;

    /// Fully visible nodes: count, then (node, clipped) pairs.
    pub const VISIBLE_COUNT: u32 = 0x94;

    pub const VISIBLE: u32 = 0x98;

    /// Partially visible nodes: count, then (node, clipped) pairs.
    pub const PARTIAL_COUNT: u32 = 0x1098;

    pub const PARTIAL: u32 = 0x109c;

    /// The visible patch list (count, then patch pointers).
    pub const PATCH_COUNT: u32 = 0x50a8;

    pub const PATCHES: u32 = 0x50ac;

    /// Every list count the pass resets, in the original store order.
    pub const CLEARED: [u32; 13] = [
        0x94, 0x1098, 0x209c, 0x40a0, 0x50a8, 0x70ac, 0x78b4, 0x7abc, 0x7bc0, 0x7fc4, 0x50a4,
        0x78b0, 0x7ab8,
    ];

    /// The effect-node count (`+0x7bc0`) and array (`+0x7bc4`) of type-8
    /// nodes; the registration head (`+0x7ab8`).
    pub const NODE_COUNT: u32 = 0x7bc0;

    pub const NODES: u32 = 0x7bc4;

    pub const HEAD: u32 = 0x7ab8;

    /// An item (8 bytes): its model (`+0`, node list at `+0x28`) and an
    /// argument (`+4`). A node: next (`+0`), type (`+8`: 7 an effect, 8 a
    /// plain node).
    pub const ITEM_SIZE: u32 = 8;

    pub const ITEM_ARG: u32 = 4;

    pub const MODEL_NODES: u32 = 0x28;

    pub const NODE_TYPE: u32 = 8;

    pub const EFFECT: u64 = 7;

    pub const PLAIN: u64 = 8;
}

/// gp offset (negative) of the graphics object (`D_004A289C`): the camera
/// interface's vtable at +0x10d8, the world view matrix at +0x5800.
pub(crate) const GRAPHICS_GP: u32 = 0x854;

/// Copy a 4x4 matrix through VU0 registers vf1..vf4 (`lqc2` x4, `sqc2` x4).
pub(crate) fn copy_matrix_vu0(c: &mut Cpu, src: u32, dst: u32) {
    for (k, vf) in (1..=4).enumerate() {
        c.lqc2(vf, src.wrapping_add(0x10 * k as u32));
    }
    for (k, vf) in (1..=4).enumerate() {
        c.sqc2(vf, dst.wrapping_add(0x10 * k as u32));
    }
}

/// `Nash* Nash_GatherVisible(Nash* this, NashRoot roots[8], OcclusionMan*
/// occ, void* hook_arg, SectionTables* tables)` (0x22add8): the world
/// pass. Stores its arguments, clears every visible list, copies the camera
/// view matrix (graphics camera interface slot 43, returned on the stack)
/// to +0x10 and the graphics world matrix (+0x5800) to +0x50, builds the
/// occluder set, uploads both matrices to VU0 data memory, then culls the
/// eight octant roots (`Nash_CullNode(ctx, node, cell)`; roots are 0x14
/// apart: cell at +0, node at +0x10) with the context `{this, occ,
/// occ->mask (+0x110), inside = 0}` on the stack, and collects / classifies
/// the patches, instances and effects of the visible (+0x98) and partial
/// (+0x109c) node lists. Returns `this`.
pub fn nash_gather_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 67;
        let frame = Frame::open(c, 0x90);
        c.g[T1][0] = c.lw(r32(c, GP).wrapping_sub(GRAPHICS_GP));
        frame.save(c, S2, 0x60);
        frame.save(c, S1, 0x70);
        c.g[S2][0] = c.g[A0][0]; // this
        frame.save(c, S3, 0x50);
        c.g[A0][0] = c.g[SP][0]; // the returned matrix at sp+0
        frame.save(c, S0, 0x80);
        c.g[S3][0] = c.g[A1][0]; // roots
        frame.save_ra(c, 0x40);
        c.g[S1][0] = sx(at(c, S2, nash::VIEW_MATRIX));
        let this = r32(c, S2);
        c.sw(this.wrapping_add(nash::OCCLUSION), c.g[A2][0]);
        c.sw(this.wrapping_add(nash::HOOK_ARG), c.g[A3][0]);
        c.sw(this.wrapping_add(nash::SECTION_TABLES), c.g[T0][0]);
        for off in nash::CLEARED {
            c.sw(this.wrapping_add(off), 0);
        }
        // The camera view matrix: graphics camera interface slot 43.
        let vt = c.lw(at(c, T1, 0x10d8));
        c.g[V0][0] = vt;
        c.g[A1][0] = c.lh((vt as u32).wrapping_add(0x158));
        c.g[V1][0] = c.lw((vt as u32).wrapping_add(0x15c));
        let method = r32(c, V1);
        c.g[A1][0] = sx(r32(c, T1).wrapping_add(r32(c, A1)));
        call_ptr(c, method, 0x0022_ae58)?;
        copy_matrix_vu0(c, sp(c), r32(c, S1));
        // The world matrix.
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(GRAPHICS_GP));
        c.g[S0][0] = sx(at(c, S2, nash::WORLD_MATRIX));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x5800));
        copy_matrix_vu0(c, r32(c, V0), r32(c, S0));
        c.g[A0][0] = c.lw(at(c, S2, nash::OCCLUSION));
        c.g[A1][0] = c.lw(at(c, S2, nash::HOOK_ARG));
        call_ptr(c, table_callee::OCCLUSION_MAN_BUILD_VISIBLE, 0x0022_aeb0)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::VU0_UPLOAD_MATRIX_PAIR.call(c, 0x0022_aebc)?;
        c.g[A0][0] = c.lw(at(c, S2, nash::HOOK_ARG));
        callee::NASH_EMPTY_HOOK.call(c, 0x0022_aec4)?;
        // The cull context at sp+0: {this, occ, occ->mask, inside = 0}.
        let occ = c.lw(at(c, S2, nash::OCCLUSION));
        c.g[V0][0] = occ;
        c.g[A1][0] = c.lw(at(c, S3, 0x10)); // root 0's node
        c.g[V1][0] = c.lw((occ as u32).wrapping_add(0x110));
        let ctx = sp(c);
        c.sw(ctx.wrapping_add(4), c.g[V0][0]);
        c.sw(ctx.wrapping_add(8), c.g[V1][0]);
        c.sw(ctx, c.g[S2][0]);
        c.sw(ctx.wrapping_add(0xc), 0);
        if c.g[A1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S3][0];
            callee::NASH_CULL_NODE.call(c, 0x0022_aef0)?;
        }
        for k in 1..8u32 {
            let root = 0x14 * k;
            c.cycles += 3;
            c.g[A1][0] = c.lw(at(c, S3, root + 0x10));
            c.g[A2][0] = sx(at(c, S3, root));
            if c.g[A1][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = c.g[SP][0];
                callee::NASH_CULL_NODE.call(c, 0x0022_aef0 + root)?;
            }
        }
        // Collect from the visible list, classify the partial list.
        c.cycles += 34;
        c.g[S0][0] = sx(at(c, S2, nash::VISIBLE));
        for (f, ret_addr) in [
            (callee::NASH_COLLECT_PATCHES, 0x0022_af90),
            (callee::NASH_COLLECT_INSTANCES, 0x0022_afa4),
            (callee::NASH_COLLECT_EFFECTS, 0x0022_afb4),
        ] {
            if ret_addr == 0x0022_afa4 {
                c.g[S1][0] = sx(at(c, S2, nash::PARTIAL));
            }
            c.g[A2][0] = c.lw(at(c, S2, nash::VISIBLE_COUNT));
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            f.call(c, ret_addr)?;
        }
        for (f, ret_addr) in [
            (callee::NASH_CLASSIFY_PATCHES, 0x0022_afc4),
            (callee::NASH_CLASSIFY_INSTANCES, 0x0022_afd4),
            (callee::NASH_CLASSIFY_EFFECTS, 0x0022_afe4),
        ] {
            c.g[A2][0] = c.lw(at(c, S2, nash::PARTIAL_COUNT));
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            f.call(c, ret_addr)?;
        }
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void Nash_CollectPatches(Nash* this, NodePair* nodes, int count)`
/// (0x22a698): append the Bézier patches of the visible nodes (each node's
/// patch list at +0x24, linked through +0) to the patch list at
/// `this + 0x50ac` (count +0x50a8), drawn by
/// `GameModule_RenderBezierPatches`. A patch must belong to an active
/// section: its logical section (`i16` at +0x156) is non-negative, the
/// section (`u8` at +0x155) has state 6 (+0x24 of its 8-byte entry) and the
/// logical section state 3 (+0x3f0 of its 0x18-byte entry; tables from
/// `*(this + 0x90)`). Patches of clipped nodes (pair word +4 non-zero) are
/// culled by the VU0 sphere test microprogram at 0xef0 on the sphere at
/// +0x140 (`vi1 == 2` = outside).
pub fn nash_collect_patches(c: &mut Cpu) {
    const NODE_HEAD: u32 = 0x0022_a6a0;
    const PATCH_HEAD: u32 = 0x0022_a6c8;
    c.cycles += 2;
    if (c.g[A2][0] as i64) > 0 {
        loop {
            c.cycles += 5;
            let node = c.lw(r32(c, A1));
            c.g[V0][0] = node;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.g[T0][0] = c.lw((node as u32).wrapping_add(0x24)); // patch list
            c.g[T4][0] = sx(at(c, A1, 8));
            if c.g[T0][0] != 0 {
                c.cycles += 5;
                c.g[T6][0] = 0x18; // logical section entry size
                c.g[T5][0] = 2; // "outside" from the sphere test
                c.g[T3][0] = sx(at(c, A0, nash::PATCHES));
                c.g[V0][0] = c.lw(at(c, A0, nash::SECTION_TABLES));
                loop {
                    collect_patch(c);
                    c.cycles += 2;
                    if c.g[T0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = c.lw(at(c, A0, nash::SECTION_TABLES));
                    if event_due(c) {
                        deliver_event_here(c, PATCH_HEAD);
                    }
                }
            }
            c.cycles += 2;
            let more = (c.g[A2][0] as i64) > 0;
            c.g[A1][0] = c.g[T4][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, NODE_HEAD);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// One patch of `Nash_CollectPatches` (patch in t0, the section tables
/// holder in v0): the section filter, the cull test and the append; leaves
/// t0 at the next patch.
pub(crate) fn collect_patch(c: &mut Cpu) {
    c.cycles += 5;
    c.g[T1][0] = c.lh(at(c, T0, 0x156)); // logical section
    c.g[A3][0] = c.lw(r32(c, V0)); // section tables
    c.g[T2][0] = c.lw(at(c, A1, 4)); // the node's "clipped" flag
    c.g[V0][0] = 0;
    if (c.g[T1][0] as i64) >= 0 {
        c.cycles += 7;
        let section = c.lbu(at(c, T0, 0x155));
        c.g[V0][0] = sx((section as u32) << 3);
        c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(at(c, V0, 0x24)) ^ 6;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 5;
            c.mult(0, V0, T1, T6); // logical * 0x18
            c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
            c.g[V0][0] = c.lw(at(c, V1, 0x3f0)) ^ 3;
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        }
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = sx(at(c, T0, 0x140)); // bounding sphere
        let culled = c.g[T2][0] != 0 && {
            c.cycles += 7;
            c.lqc2(22, r32(c, V0));
            c.ctc2(T2, 14);
            c.vcallms(0xef0);
            c.cfc2(V1, 1);
            c.g[V1][0] == c.g[T5][0]
        };
        if !culled {
            c.cycles += 7;
            let n = c.lw(at(c, A0, nash::PATCH_COUNT));
            c.g[V1][0] = n;
            c.g[V0][0] = sx((n as u32) << 2);
            c.g[V0][0] = sx(r32(c, T3).wrapping_add(r32(c, V0)));
            c.g[V1][0] = sx((n as u32).wrapping_add(1));
            c.sw(r32(c, V0), c.g[T0][0]);
            c.sw(at(c, A0, nash::PATCH_COUNT), c.g[V1][0]);
        }
    }
    c.g[T0][0] = c.lw(r32(c, T0));
}

/// `void Vu0_UploadMatrixPair(const Mat4* a, const Mat4* b)` (0x229f30):
/// store both 4x4 matrices to VU0 data memory from qword 0x40 (`vi1 =
/// 0x40`, eight `vsqi`), for the culling microprograms.
pub fn vu0_upload_matrix_pair(c: &mut Cpu) {
    c.cycles += 20;
    c.g[V0][0] = 0x40;
    c.ctc2(V0, 1);
    for arg in [A0, A1] {
        let m = r32(c, arg);
        for (k, vf) in (1..=4).enumerate() {
            c.lqc2(vf, m.wrapping_add(0x10 * k as u32));
        }
        for vf in 1..=4u8 {
            c.vu0().sqi(0b1111, vf, 1);
        }
    }
    ret(c);
}

// ------------------------------------------------- CrowdMan2d_RenderSprites

/// `void CrowdMan2d_RenderSprites(CrowdMan2d* self)` (0x229ba8): draw the
/// 40 2D crowd sprites. Each sprite is a 0x20-byte record from `self+0x2220`
/// (position first, its visible count at +0x10); for each visible one call
/// the drawer's sprite method (virtual `+0x270` of `ctx+0x10d8`) with the
/// position, a 100x100 size, zero UVs, unit UV size and white. Before the
/// first one a GS state packet is appended at the render context's packet
/// cursor (`ctx+0xe84`, as in `HaloModifier_DrawQuad`: previous 0x14 bytes
/// repeated, the 0x501420 template, header patched, texture word
/// `ctx+0x1054` at +0x10); after the last one the cursor steps back.
pub fn crowd_man_2d_render_sprites(c: &mut Cpu) {
    /// `gp+0x2a90`: the render context.
    const CTX_GP: u32 = 0x2a90;
    const CTX_PACKET: u32 = 0xe84;
    const CTX_DRAWER: u32 = 0x10d8;
    const CTX_CROWD_TEXTURE: u32 = 0x1054;
    const STATE_TEMPLATE: u32 = 0x0050_1420;
    /// The sprite records.
    const SPRITES: u32 = 0x2220;
    const SPRITE_VISIBLE: u32 = 0x10;
    const SPRITE_SIZE: u32 = 0x20;
    /// Loop head (0x229c48), for event delivery.
    const HEAD: u32 = 0x0022_9c48;
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0xf0);
        let one = 0x3f80_0000u32;
        let hundred = 0x42c8_0000u32;
        c.f[0] = one;
        c.g[AT][0] = sx(hundred);
        c.f[1] = hundred;
        c.g[T2][0] = 0;
        frame.save(c, S0, 0xe0);
        frame.save(c, S3, 0xb0);
        frame.save(c, S4, 0xa0);
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save(c, S7, 0x70);
        frame.save(c, S1, 0xd0);
        frame.save(c, S2, 0xc0);
        frame.save(c, FP, 0x60);
        frame.save_ra(c, 0x50);
        // The header patch masks.
        c.g[S3][0] = 0xffff_ffff_e000_03ff;
        c.g[S4][0] = 0xffff_ffff_fff0_0fff;
        c.g[S5][0] = 0xffff_ffff_ffcf_ffff;
        c.g[S6][0] = 0xffff_ffff_fe7f_ffff;
        c.g[S7][0] = 0xffff_ffff_ffbf_ffff;
        let this = r32(c, A0);
        c.g[S2][0] = sx(this.wrapping_add(SPRITES + SPRITE_VISIBLE));
        c.g[S1][0] = sx(this.wrapping_add(SPRITES));
        let ctx = c.lw(r32(c, GP).wrapping_add(CTX_GP));
        c.g[S0][0] = ctx;
        let ctx = ctx as u32;
        // The sprite arguments: UV (0, 0) at sp+0, UV size (1, 1) at
        // sp+0x10, colour (1, 1, 1, 1) at sp+0x20, size (100, 100) at sp+0x30.
        let sp0 = sp(c);
        c.sw(sp0, 0);
        c.sw(sp0 + 0x4, 0);
        c.g[FP][0] = 0x27;
        for off in [0x10, 0x14, 0x20, 0x24, 0x28, 0x2c] {
            c.sw(sp0 + off, one as u64);
        }
        c.sw(sp0 + 0x30, hundred as u64);
        c.sw(sp0 + 0x34, hundred as u64);

        loop {
            c.cycles += 3;
            let visible = c.lw(r32(c, S2));
            c.g[V0][0] = visible;
            if (visible as i64) > 0 {
                c.cycles += 2;
                if c.g[T2][0] == 0 {
                    c.cycles += 80;
                    c.g[T2][0] = 1;
                    append_state_packet(c, ctx);
                }
                c.g[V0][0] = c.lw(ctx.wrapping_add(CTX_DRAWER));
                // drawer->Sprite(pos, &size, &uv, &uv_size, &colour).
                c.cycles += 12;
                let sp0 = sp(c);
                let vt = r32(c, V0);
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = sx(sp0 + 0x30);
                c.g[A3][0] = c.g[SP][0];
                c.g[A0][0] = c.lh(vt.wrapping_add(0x270));
                c.g[T0][0] = sx(sp0 + 0x10);
                c.g[V1][0] = c.lw(vt.wrapping_add(0x274));
                c.g[T1][0] = sx(sp0 + 0x20);
                c.sq(T2, sp0 + 0x40);
                let target = r32(c, V1);
                c.g[A0][0] = sx(ctx.wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0022_9dc4)?;
                c.lq(T2, sp(c) + 0x40);
            }
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(SPRITE_SIZE));
            c.cycles += 3;
            c.g[FP][0] = sx(r32(c, FP).wrapping_sub(1));
            let more = (c.g[FP][0] as i64) >= 0;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(SPRITE_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
        c.cycles += 2;
        if c.g[T2][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = c.lw(ctx.wrapping_add(CTX_PACKET));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
            c.sw(ctx.wrapping_add(CTX_PACKET), c.g[V0][0]);
        }
        frame.restore(c, S0, 0xe0);
        c.cycles += 11;
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    });

    /// The state packet before the first sprite (0x229c74..0x229d98).
    fn append_state_packet(c: &mut Cpu, ctx: u32) {
        // Repeat the previous 0x14 bytes after the cursor.
        let cur = c.lw(ctx.wrapping_add(CTX_PACKET));
        c.g[V0][0] = cur;
        let cur = cur as u32;
        c.g[A1][0] = 0x50_0000;
        c.g[A2][0] = 0x1_0000;
        c.ldl(V1, cur.wrapping_add(0x7));
        c.ldr(V1, cur);
        c.ldl(A0, cur.wrapping_add(0xf));
        c.ldr(A0, cur.wrapping_add(0x8));
        c.g[A3][0] = c.lw(cur.wrapping_add(0x10));
        c.sdl(V1, cur.wrapping_add(0x1b));
        c.sdr(V1, cur.wrapping_add(0x14));
        c.sdl(A0, cur.wrapping_add(0x23));
        c.sdr(A0, cur.wrapping_add(0x1c));
        c.sw(cur.wrapping_add(0x24), c.g[A3][0]);
        c.g[A2][0] = 0x1_4000;
        // Advance the cursor; the state template after it.
        let cur = c.lw(ctx.wrapping_add(CTX_PACKET));
        c.g[V1][0] = cur;
        let cur = cur as u32;
        c.g[V0][0] = sx(cur.wrapping_add(0x14));
        c.sw(ctx.wrapping_add(CTX_PACKET), c.g[V0][0]);
        c.g[T0][0] = sx(STATE_TEMPLATE);
        c.ldl(V0, STATE_TEMPLATE + 0x7);
        c.ldr(V0, STATE_TEMPLATE);
        c.ldl(A0, STATE_TEMPLATE + 0xf);
        c.ldr(A0, STATE_TEMPLATE + 0x8);
        c.g[A3][0] = c.lw(STATE_TEMPLATE + 0x10);
        c.sdl(V0, cur.wrapping_add(0x1b));
        c.sdr(V0, cur.wrapping_add(0x14));
        c.sdl(A0, cur.wrapping_add(0x23));
        c.sdr(A0, cur.wrapping_add(0x1c));
        c.sw(cur.wrapping_add(0x24), c.g[A3][0]);
        c.g[A3][0] = 0x40_0000;
        c.g[T0][0] = 0xffff_ffff_ffff_fffc;
        // The crowd texture word at +0x10.
        c.g[A0][0] = c.lw(ctx.wrapping_add(CTX_CROWD_TEXTURE));
        c.g[V0][0] = c.lw(ctx.wrapping_add(CTX_PACKET));
        c.sh(r32(c, V0).wrapping_add(0x10), c.g[A0][0]);
        // Patch the header words: (pointer register, offset, mask, or).
        let patch = |c: &mut Cpu, p: usize, off: u32, mask: u64, or: u64| {
            c.g[p][0] = c.lw(ctx.wrapping_add(CTX_PACKET));
            let at = r32(c, p).wrapping_add(off);
            c.g[V0][0] = c.lw(at);
            c.g[V0][0] &= mask;
            c.g[V0][0] |= or;
            c.sw(at, c.g[V0][0]);
        };
        c.g[A0][0] = 0xffff_ffff_ffff_fff3;
        patch(c, V1, 0x0, 0xffff_ffff_ffff_fff3, 0xc);
        let (s7, s6, s5, s4, s3) = (c.g[S7][0], c.g[S6][0], c.g[S5][0], c.g[S4][0], c.g[S3][0]);
        patch(c, A0, 0x4, s7, 0x40_0000);
        patch(c, V1, 0x4, s6, 0);
        patch(c, A0, 0x4, s5, 0);
        patch(c, V1, 0x4, s4, 0x1_4000);
        patch(c, A0, 0x4, 0xffff_ffff_ffff_fffc, 0x1);
        patch(c, V1, 0x8, s3, 0);
        c.g[V1][0] = 0xffff_ffff_ffff_fc1f;
        patch(c, A0, 0x8, 0xffff_ffff_ffff_fc1f, 0x80);
        patch(c, V1, 0x4, 0xffff_ffff_ffff_ff83, 0x1c);
        c.g[A0][0] = 0xffff_ffff_ffff_ff83;
    }
}

pub(crate) mod nash_callee {
    use super::ee;
    use super::Callee;

    pub const CLASSIFY_EFFECT: Callee = Callee {
        addr: 0x0022_a270,
        f: ee::s05::Nash_ClassifyEffect,
    };
}

pub(crate) mod nash_fx {

    /// Nash: emitter list (count at +0x78b4, entries from +0x78b8) and
    /// light list (count at +0x7bc0, entries from +0x7bc4).
    pub const EMITTER_COUNT: u32 = 0x78b4;

    pub const EMITTERS: u32 = 0x78b8;

    pub const LIGHT_COUNT: u32 = 0x7bc0;

    pub const LIGHTS: u32 = 0x7bc4;

    /// Visible-node pair (8 bytes): world node at +0, its clip word at +4
    /// (0 = fully inside, no sphere test).
    pub const PAIR_SIZE: u32 = 8;

    pub const PAIR_CLIP: u32 = 4;

    /// World node: effect list head at +0x28. Effect: next at +0, kind at
    /// +8, bounding sphere at +0x50.
    pub const NODE_EFFECTS: u32 = 0x28;

    pub const FX_KIND: u32 = 0x8;

    pub const FX_SPHERE: u32 = 0x50;

    pub const KIND_EMITTER: u64 = 7;

    pub const KIND_LIGHT: u64 = 8;

    /// The sphere test microprogram's result (in vi1) for "outside".
    pub const OUTSIDE: u64 = 2;
}

/// `void Nash_ClassifyEffects(Nash* self, Item* items, int n)` (0x22a4a8):
/// walk the node lists of `n` items: effect nodes (type 7) are classified
/// with the item's argument, plain nodes (type 8) are appended to the node
/// array. Then, if anything is registered, classify a null effect (flush).
pub fn nash_classify_effects(c: &mut Cpu) {
    use nash::*;
    use nash_callee::*;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S1][0] as u32;
        if (c.g[A2][0] as i64) > 0 {
            c.cycles += 1;
            loop {
                // Item at s2; s5 = items left after it.
                let item = c.g[S2][0] as u32;
                c.cycles += 5;
                c.g[V0][0] = Addr(item).lw(c);
                c.g[S5][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(MODEL_NODES)).lw(c);
                c.g[S4][0] = sx(item.wrapping_add(ITEM_SIZE));
                if c.g[S0][0] != 0 {
                    c.cycles += 5;
                    c.g[S7][0] = EFFECT;
                    c.g[S6][0] = PLAIN;
                    c.g[S3][0] = sx(this.wrapping_add(NODES));
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(NODE_TYPE)).lw(c);
                    loop {
                        let node = c.g[S0][0] as u32;
                        let kind = c.g[V0][0];
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S1][0];
                        if kind == EFFECT {
                            c.cycles += 5;
                            c.g[A2][0] = Addr(item.wrapping_add(ITEM_ARG)).lw(c);
                            c.g[A1][0] = c.g[S0][0];
                            CLASSIFY_EFFECT.call(c, 0x0022_a51c)?;
                        } else {
                            c.cycles += 2;
                            if kind == PLAIN {
                                c.cycles += 7;
                                c.g[V1][0] = Addr(this.wrapping_add(NODE_COUNT)).lw(c);
                                let n = c.g[V1][0] as u32;
                                c.g[V0][0] = sx(this.wrapping_add(NODES).wrapping_add(n << 2));
                                c.g[V1][0] = sx(n.wrapping_add(1));
                                Addr(c.g[V0][0] as u32).sw(c, node as i32 as i64 as u64);
                                let count = c.g[V1][0];
                                Addr(this.wrapping_add(NODE_COUNT)).sw(c, count);
                            }
                        }
                        c.g[S0][0] = Addr(node).lw(c);
                        c.cycles += 2;
                        if c.g[S0][0] == 0 {
                            break;
                        }
                        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(NODE_TYPE)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_a508);
                        }
                    }
                }
                c.cycles += 3;
                c.g[A2][0] = c.g[S5][0];
                c.g[S2][0] = c.g[S4][0];
                if (c.g[A2][0] as i64) <= 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_a4e0);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(this.wrapping_add(HEAD)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            CLASSIFY_EFFECT.call(c, 0x0022_a574)?;
        }
        c.cycles += 11;
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

/// `void Nash_CollectEffects(Nash* this, NodePair* nodes, int count)`
/// (0x22a770): walk the effects of every visible world node: emitters
/// (kind 7) are appended to the emitter list unless the node is clipped
/// and the VU0 sphere test (microprogram 0xef0, clip word in vi14) puts the
/// emitter's sphere outside; lights (kind 8) are always appended to the
/// light list.
pub fn nash_collect_effects(c: &mut Cpu) {
    use nash_fx::*;
    c.cycles += 2;
    if (c.g[A2][0] as i64) > 0 {
        let me = r32(c, A0);
        loop {
            c.cycles += 5;
            let pair = r32(c, A1);
            c.g[V0][0] = Addr(pair).lw(c);
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.g[A3][0] = Addr(r32(c, V0).wrapping_add(NODE_EFFECTS)).lw(c);
            c.g[T2][0] = sx(pair.wrapping_add(PAIR_SIZE));
            if c.g[A3][0] != 0 {
                c.cycles += 7;
                c.g[T5][0] = KIND_EMITTER;
                c.g[T4][0] = OUTSIDE;
                c.g[T1][0] = sx(me.wrapping_add(EMITTERS));
                c.g[T3][0] = KIND_LIGHT;
                c.g[T0][0] = sx(me.wrapping_add(LIGHTS));
                c.g[V0][0] = Addr(r32(c, A3).wrapping_add(FX_KIND)).lw(c);
                loop {
                    c.cycles += 2;
                    let fx = r32(c, A3);
                    // Some((count field, list)) when the effect is appended.
                    let append = if c.g[V0][0] == c.g[T5][0] {
                        c.cycles += 3;
                        c.g[V1][0] = Addr(pair.wrapping_add(PAIR_CLIP)).lw(c);
                        c.g[V0][0] = sx(fx.wrapping_add(FX_SPHERE));
                        let culled = c.g[V1][0] != 0 && {
                            c.cycles += 7;
                            c.lqc2(22, r32(c, V0));
                            c.ctc2(V1, 14);
                            c.vcallms(0xef0);
                            c.cfc2(V1, 1);
                            c.g[V1][0] == c.g[T4][0]
                        };
                        if culled {
                            None
                        } else {
                            c.cycles += 7;
                            Some((EMITTER_COUNT, T1))
                        }
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[T3][0] {
                            None
                        } else {
                            c.cycles += 6;
                            Some((LIGHT_COUNT, T0))
                        }
                    };
                    if let Some((count, list)) = append {
                        let count = Addr(me.wrapping_add(count));
                        c.g[V1][0] = count.lw(c);
                        c.g[V0][0] = sx(r32(c, V1) << 2);
                        c.g[V0][0] = sx(r32(c, list).wrapping_add(r32(c, V0)));
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                        let (slot, fx_word, n) = (Addr(r32(c, V0)), c.g[A3][0], c.g[V1][0]);
                        slot.sw(c, fx_word);
                        count.sw(c, n);
                        c.cycles += 1;
                    }
                    c.g[A3][0] = Addr(fx).lw(c);
                    c.cycles += 2;
                    if c.g[A3][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, A3).wrapping_add(FX_KIND)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_a7a8);
                    }
                }
            }
            c.cycles += 2;
            let more = (c.g[A2][0] as i64) > 0;
            c.g[A1][0] = c.g[T2][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_a778);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// Nash_ClassifyPatches (0x22a408)

pub(crate) const NASH_CLASSIFY_PATCH: Callee = Callee {
    addr: 0x0022_a128,
    f: ee::s05::Nash_ClassifyPatch,
};

/// A visible-list entry (8 bytes): the source (+0, whose first patch node
/// is at +0x24) and the VU frustum handed to the classifier (+4).
pub(crate) const VIS_SOURCE: u32 = 0x0;

pub(crate) const VIS_FRUSTUM: u32 = 0x4;

pub(crate) const VIS_ENTRY_SIZE: u32 = 0x8;

pub(crate) const SOURCE_FIRST_PATCH: u32 = 0x24;

/// A patch node's next link.
pub(crate) const PATCH_NEXT: u32 = 0x0;

/// The cell's pending list head (non-zero: one more classify with no node).
pub(crate) const CELL_LIST_HEAD: u32 = 0x78b0;

/// `void Nash_ClassifyPatches(Cell* cell /*a0*/, VisEntry* list /*a1*/,
/// int count /*a2*/)` (0x22a408): run `Nash_ClassifyPatch(cell, node,
/// entry->frustum)` on every patch node of every visible-list entry, then
/// once more with no node when the cell's list head is set. Frame 0x60 (ra
/// at 0, s4..s0 at 0x10..0x50).
pub fn nash_classify_patches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        if (c.g[A2][0] as i64) > 0 {
            loop {
                // s1: the entry, s3: entries left after it, s2: the next.
                c.cycles += 5;
                let entry = r32(c, S1);
                c.g[V0][0] = c.lw(entry.wrapping_add(VIS_SOURCE));
                c.g[S3][0] = sx(r32(c, A2).wrapping_sub(1));
                c.g[S0][0] = c.lw(r32(c, V0).wrapping_add(SOURCE_FIRST_PATCH));
                c.g[S2][0] = sx(entry.wrapping_add(VIS_ENTRY_SIZE));
                if c.g[S0][0] != 0 {
                    c.cycles += 1;
                    c.g[A2][0] = c.lw(entry.wrapping_add(VIS_FRUSTUM));
                    loop {
                        c.cycles += 6;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S4][0];
                        NASH_CLASSIFY_PATCH.call(c, 0x0022_a454)?;
                        c.g[S0][0] = c.lw(r32(c, S0).wrapping_add(PATCH_NEXT));
                        if c.g[S0][0] == 0 {
                            break;
                        }
                        c.g[A2][0] = c.lw(r32(c, S1).wrapping_add(VIS_FRUSTUM));
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_a448);
                        }
                    }
                }
                c.cycles += 3;
                c.g[A2][0] = c.g[S3][0];
                c.g[S1][0] = c.g[S2][0];
                if (c.g[A2][0] as i64) <= 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_a430);
                }
            }
        }
        c.cycles += 3;
        let cell = r32(c, S4);
        c.g[V0][0] = c.lw(cell.wrapping_add(CELL_LIST_HEAD));
        c.g[A0][0] = c.g[S4][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            NASH_CLASSIFY_PATCH.call(c, 0x0022_a484)?;
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
    });
}

/// `void Nash_ClassifyInstances(Cell* cell, List* lists, int count)`
/// (0x22a368, ssxapp.cpp): runs `Nash_ClassifyInstance(cell, node,
/// list->frustum)` over every node (linked through +0x0) of each list's
/// source (`list->src + 0x20` is the head; `+0x4` the frustum; lists are 8
/// bytes apart), then once more with a null node if the cell's instance
/// head (+0x50a4) is set.
pub fn nash_classify_instances(c: &mut Cpu) {
    const CLASSIFY: Callee = Callee {
        addr: 0x0022_9fc8,
        f: ee::s05::f_00229fc8,
    };
    const HEAD: u32 = 0x50a4;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 17, 0x40);
        frame.save(c, 20, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 16, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, 18, 0x30);
        frame.save(c, 19, 0x20);
        frame.save_ra(c, 0);
        if (c.g[A2][0] as i64) > 0 {
            loop {
                c.cycles += 5;
                let list = r32(c, S1);
                let src = Addr(list).lw(c);
                c.g[V0][0] = src;
                c.g[S3][0] = sx(r32(c, A2).wrapping_sub(1));
                c.g[S0][0] = Addr(src as u32).off(0x20).lw(c);
                c.g[S2][0] = sx(list.wrapping_add(8));
                if c.g[S0][0] != 0 {
                    c.cycles += 1;
                    c.g[A2][0] = Addr(list).off(4).lw(c);
                    loop {
                        c.cycles += 6;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S4][0];
                        CLASSIFY.call(c, 0x0022_a3b4)?;
                        c.g[S0][0] = Addr(r32(c, S0)).lw(c);
                        if c.g[S0][0] == 0 {
                            break;
                        }
                        c.g[A2][0] = Addr(r32(c, S1)).off(4).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_a3a8);
                        }
                    }
                }
                c.cycles += 3;
                c.g[A2][0] = c.g[S3][0];
                c.g[S1][0] = c.g[S2][0];
                if (c.g[A2][0] as i64) <= 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_a390);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S4)).off(HEAD).lw(c);
        c.g[A0][0] = c.g[S4][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            CLASSIFY.call(c, 0x0022_a3e4)?;
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
// SSXApp_Tick (0x227f58)

/// gp-relative pointer to the pad queue.
pub(crate) const SSX_APP_PAD_QUEUE_GP: u32 = 0x850;

pub(crate) const SSX_APP_PAD_QUEUE_POLL: u32 = 0x0032_6b88;

/// `void cSSXApp::timerCallback()` (0x227f58, `SSXApp_Tick`): the app's
/// timer tick polls the pad queue (`PadQueue_Poll`) if there is one.
pub fn ssx_app_tick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(SSX_APP_PAD_QUEUE_GP)).lw(c);
        frame.save_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, SSX_APP_PAD_QUEUE_POLL, 0x0022_7f70)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Bytes of the 2D crowd array saved after the object's first word.
pub(crate) const CROWD_MAN2D_SAVE_SIZE: u64 = 0x200;

/// `void CrowdMan2d_SaveState(void* self, cPbStream* s)` (0x229e20): writes the
/// crowd's 0x200-byte array at `self+8` through the stream's virtual write
/// method (vtable entry +8: a 16-bit `this` delta and the code pointer at
/// +0xc).
pub fn crowd_man2d_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = CROWD_MAN2D_SAVE_SIZE;
        // s->v01(self + 8, 0x200): vtable entry at +8.
        let stream = r32(c, A3);
        c.g[V1][0] = Addr(stream).lw(c);
        let slot = r32(c, V1);
        c.g[A0][0] = Addr(slot).off(0x8).lh(c);
        c.g[V0][0] = Addr(slot).off(0xc).lw(c);
        let write = r32(c, V0);
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        call_ptr(c, write, 0x0022_9e48)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CrowdMan2d_Remove (0x2297d8)
// ---------------------------------------------------------------------------

/// The 64 crowd-instance slot words, at `self + 8`.
pub(crate) const CROWD_MAN2D_REMOVE_SLOTS: u32 = 0x8;

/// `void CrowdMan2d_Remove(self, int handle)` (0x2297d8): scan the 64 slot
/// words at `+8` for `handle` and clear the first match
/// (`CrowdMan2d_ClearSlot`). The generated body is a dispatcher and can be
/// re-entered at the loop head after an event unwind, so the replacement
/// keeps its blocks as states.
pub fn crowd_man2d_remove(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0022_97d8 => {
                    c.cycles += 6;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                    c.g[A2][0] = c.g[A1][0]; // the handle to find
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[A1][0] = 0; // the slot index
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(CROWD_MAN2D_REMOVE_SLOTS));
                    pc = 0x0022_97f4;
                }
                0x0022_97f0 => {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    pc = 0x0022_97f4;
                }
                0x0022_97f4 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 64) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    pc = if done { 0x0022_9818 } else { 0x0022_9800 };
                }
                0x0022_9800 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let same = c.g[V0][0] == c.g[A2][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                    if same {
                        pc = 0x0022_980c;
                    } else {
                        if back_edge_unwinds(c, 0x0022_97f0) {
                            return Ok(());
                        }
                        pc = 0x0022_97f0;
                    }
                }
                0x0022_980c => {
                    c.cycles += 3;
                    Callee {
                        addr: 0x0022_9b90,
                        f: ee::s05::CrowdMan2d_ClearSlot,
                    }
                    .call(c, 0x0022_9814)?;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    pc = 0x0022_9818;
                }
                0x0022_9818 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0022_97d8, 0x0022_9820);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// CrowdMan2d_BindInstanceFrame (0x00229820)

/// `void CrowdMan2d::BindInstanceFrame(this, handle, part, bone)` (0x229820): resolve the model of an instance `handle` and, if the part and bone indices are in range, store the 0x536690 constant as the pose entry of that bone.
pub fn crowd_man2d_bind_instance_frame(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        'b_229890: {
            'b_22986c: {
                c.cycles += 17;
                Frame::open(c, 0x40);
                frame.save(c, S0, 0x30);
                frame.save(c, S1, 0x20);
                c.g[S0][0] = c.g[A1][0];
                frame.save(c, S2, 0x10);
                c.g[S1][0] = c.g[A2][0];
                frame.save_ra(c, 0);
                c.g[S2][0] = c.g[A3][0];
                call_ptr(c, 0x002d_1bd8, 0x0022_9844)?;
                c.g[V1][0] = c.lw(r32(c, V0));
                c.g[V0][0] = c.g[S0][0] & 0xff;
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                c.g[V1][0] = c.lw(r32(c, V0));
                if c.g[V1][0] != 0u64 {
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                    break 'b_22986c;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0u64;
                    if t {
                        break 'b_229890;
                    } else {
                        break 'b_22986c;
                    }
                }
            }
            'b_22988c: {
                c.cycles += 7;
                c.g[V0][0] = sx(r32(c, S0) >> 8);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                c.g[V1][0] = c.lw(r32(c, V0));
                c.g[V0][0] = sx(r32(c, V1) >> 8);
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = sx(r32(c, V0) << 2);
                if t {
                    break 'b_22988c;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    break 'b_22988c;
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
            break 'b_229890;
        }
        'b_2298f8: {
            c.cycles += 2;
            let t = c.g[V1][0] == 0u64;
            frame.restore(c, S0, 0x30);
            if t {
                break 'b_2298f8;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x80));
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x94));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x4));
                c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(0x8));
                c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[A3][0] = sx(r32(c, A0).wrapping_add(0x4));
                if t {
                    break 'b_2298f8;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r32(c, S1) << 4);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
                    c.g[V0][0] = c.lw(r32(c, V0).wrapping_add(0x4));
                    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1c));
                    c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V1][0] = sx(r32(c, S2) << 2);
                    if t {
                        break 'b_2298f8;
                    } else {
                        c.cycles += 10;
                        c.g[V0][0] = c.lw(r32(c, V0).wrapping_add(0x20));
                        c.g[A1][0] = 0x530000;
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x6690));
                        c.g[A0][0] = c.lw(r32(c, V1));
                        c.g[V0][0] = c.lh(r32(c, A0));
                        c.g[V0][0] = sx(r32(c, V0) << 2);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
                        c.sw(r32(c, V0), c.g[A1][0]);
                        frame.restore(c, S0, 0x30);
                        break 'b_2298f8;
                    }
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdMan2d_InitSlot(CrowdMan2d* self, int index, int rid)`
/// (0x229910): store the model handle for a crowd slot, look its model up
/// and derive the slot's billboard position and axes from the model's quad
/// corners (graphics vtable slot 97).
pub fn crowd_man_2d_init_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[A0][0] = 0x3;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
        c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[V0][0] = 0xffffffffffffffff;
        'l_229950: loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            let t = c.g[A0][0] != c.g[V0][0];
            c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00229950);
                }
                continue 'l_229950;
            } else {
                'b_2299dc: {
                    'b_2299b8: {
                        c.cycles += 17;
                        c.g[S3][0] = sx((c.g[A1][0] as u32) << 6);
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                        c.g[V1][0] = 0x3;
                        c.g[A0][0] = 0xffffffffffffffff;
                        c.g[V1][0] = 0x3;
                        c.g[A0][0] = 0xffffffffffffffff;
                        c.g[V1][0] = 0x3;
                        c.g[A0][0] = 0xffffffffffffffff;
                        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                        c.g[V1][0] = c.g[A2][0] & 0xff;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = c.lw(c.g[V1][0] as u32);
                        if c.g[V1][0] != 0u64 {
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                            break 'b_2299b8;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[S0][0] = 0u64;
                            if t {
                                break 'b_2299dc;
                            } else {
                                break 'b_2299b8;
                            }
                        }
                    }
                    'b_2299d8: {
                        c.cycles += 7;
                        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_2299d8;
                        } else {
                            c.cycles += 1;
                            c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                            break 'b_2299d8;
                        }
                    }
                    c.cycles += 1;
                    c.g[S0][0] = c.g[A1][0];
                    break 'b_2299dc;
                }
                c.cycles += 15;
                c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                c.g[A3][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x308));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002299fc)?;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 2);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = c.g[S1][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[T0][0] = 0x3;
                'l_229a18: loop {
                    c.cycles += 24;
                    c.lwc1(1, c.g[A3][0] as u32);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xe4));
                    c.lwc1(1, (c.g[A3][0] as u32).wrapping_add(0x8));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xec));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.lqc2(4, c.g[V1][0] as u32);
                    c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[V1][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[V1][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.sq(2, c.g[A0][0] as u32);
                    let t = (c.g[T0][0] as i64) >= 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00229a18);
                        }
                        continue 'l_229a18;
                    } else {
                        c.cycles += 69;
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lqc2(4, c.g[SP][0] as u32);
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 0);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x51f0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.mfc1(4, 0);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.qmtc2(4, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.g[AT][0] = 0x43160000;
                        c.mtc1(1, 2);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.lqc2(3, c.g[SP][0] as u32);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x210));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lqc2(4, c.g[SP][0] as u32);
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.mfc1(4, 1);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.qmtc2(4, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x220));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lqc2(4, c.g[SP][0] as u32);
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.mfc1(4, 1);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.qmtc2(4, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x230));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd8));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xdc), 0u64);
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x210));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x210));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `base + off` as a 32-bit guest address.
#[inline(always)]
pub(crate) fn oc429_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// One 0x20-byte-chunk copy loop of a by-value struct copy in the compiler's
/// inlined memcpy (0x280 bytes; the caller copies the 8-byte tail): `ld`/`sd`
/// through `tmp` when the source is 8-aligned, else `ldl`/`ldr`/`sdl`/`sdr`
/// (11 / 19 cycles per chunk, plus 2 when the unaligned loop ends). `src`,
/// `dst` advance; `end` is the source end. `head_pc` is the loop head the
/// event check resumes at.
pub(crate) fn oc429_chunk_copy(
    c: &mut Cpu,
    unaligned: bool,
    src: usize,
    dst: usize,
    end: usize,
    tmp: [usize; 4],
    head_pc: u32,
) {
    loop {
        if unaligned {
            c.cycles += 19;
            for (k, &t) in tmp.iter().enumerate() {
                let o = 8 * k as u32;
                c.ldl(t, oc429_at(c.g[src][0], o + 7));
                c.ldr(t, oc429_at(c.g[src][0], o));
            }
            for (k, &t) in tmp.iter().enumerate() {
                let o = 8 * k as u32;
                c.sdl(t, oc429_at(c.g[dst][0], o + 7));
                c.sdr(t, oc429_at(c.g[dst][0], o));
            }
        } else {
            c.cycles += 11;
            for (k, &t) in tmp.iter().enumerate() {
                c.g[t][0] = c.ld(oc429_at(c.g[src][0], 8 * k as u32));
            }
            for (k, &t) in tmp.iter().enumerate() {
                c.sd(oc429_at(c.g[dst][0], 8 * k as u32), c.g[t][0]);
            }
        }
        c.g[src][0] = sx(oc429_at(c.g[src][0], 0x20));
        let more = c.g[src][0] != c.g[end][0];
        c.g[dst][0] = sx(oc429_at(c.g[dst][0], 0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, head_pc);
            }
            continue;
        }
        if unaligned {
            c.cycles += 2;
        }
        return;
    }
}

/// Where the game's options record lives (`GetOptions()` is a by-value copy
/// of it): the 0x288-byte struct at 0x535610.
#[cfg(test)]
pub(crate) const SSXAPP_OPTIONS: u32 = 0x0053_5610;

/// `void SSXApp_ApplyDisplayOptions(void)` (0x228c08): fetch the BE interface 4
/// (`BELibrary_GetInstance` / `GetInterface`), then copy the options record
/// (3 times, as the compiled by-value `GetOptions()` temporaries) and apply
/// it to the graphics manager (`gp + 0x2a90`): its vtable slot `+0x140` gets
/// the display mode (`options.word0 >> 20 & 3`, mapped 1 -> 1, 2 -> 2, else 0)
/// and slot `+0x60` gets `(options.byte6, options.byte(0x295 - 0x290 + 5))`.
/// The source is 8-aligned, so the `ldl`/`ldr` copy variants are dead.
pub fn ssx_app_apply_display_options(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x550);
        frame.save(c, S1, 0x530);
        frame.save(c, S0, 0x540);
        frame.save_ra(c, 0x520);
        call_ptr(c, 0x0014_dc80, 0x0022_8c20)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_dd58, 0x0022_8c2c)?; // BELibrary_GetInterface
        c.g[V1][0] = 0x53_0000;
        c.g[S1][0] = c.g[V1][0];
        c.g[V1][0] = sx(oc429_at(c.g[V1][0], 0x5610));
        c.g[V0][0] = c.g[V1][0] & 0x7;
        let aligned = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[SP][0];
        c.cycles += 2;
        c.g[V0][0] = sx(oc429_at(c.g[V1][0], 0x280));
        c.g[S0][0] = sx(oc429_at(c.g[SP][0], 0x290));
        // Copy 1: options -> sp[0..].
        if aligned {
            oc429_chunk_copy(c, false, V1, A0, V0, [T1, T2, T3, A1], 0x0022_8ca8);
        } else {
            oc429_chunk_copy(c, true, V1, A0, V0, [A1, A2, A3, T0], 0x0022_8c4c);
        }
        c.cycles += 10;
        c.ldl(A2, oc429_at(c.g[V1][0], 0x7));
        c.ldr(A2, oc429_at(c.g[V1][0], 0));
        c.sdl(A2, oc429_at(c.g[A0][0], 0x7));
        c.sdr(A2, oc429_at(c.g[A0][0], 0));
        c.g[V1][0] = 1;
        c.g[V0][0] = c.lw(sp(c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 20);
        c.g[A0][0] = c.g[V0][0] & 0x3;
        let mode1 = c.g[A0][0] == c.g[V1][0];
        c.g[A2][0] = c.lw(oc429_at(c.g[GP][0], 0x2a90));
        // 0: slot +0x140 (7 cycles) with a1 = mode; 3: the same call with
        // a1 = 0 (6 cycles).
        let mut other = false;
        if mode1 {
            c.cycles += 1;
            c.g[A1][0] = 1;
        } else {
            c.cycles += 2;
            let zero = c.g[A0][0] == 0;
            c.g[V0][0] = 2;
            if zero {
                c.cycles += 2;
                c.g[A1][0] = 0;
            } else {
                c.cycles += 2;
                let two = c.g[A0][0] == c.g[V0][0];
                c.g[A1][0] = 2;
                if !two {
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    other = true;
                }
            }
        }
        c.cycles += if other { 6 } else { 7 };
        c.g[V0][0] = c.lw(oc429_at(c.g[A2][0], 0x10d8));
        c.g[A0][0] = c.lh(oc429_at(c.g[V0][0], 0x140));
        c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x144));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(oc429_at(c.g[A2][0], c.g[A0][0] as u32));
        call_ptr(c, target, if other { 0x0022_8d50 } else { 0x0022_8d34 })?;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0x2a90));

        // Copy 2: options -> sp[0..] again (the `GetOptions().b6` temporary).
        c.cycles += 8;
        c.g[A3][0] = sx(oc429_at(c.g[S1][0], 0x5610));
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = c.g[A3][0] & 0x7;
        c.g[V1][0] = c.lw(oc429_at(c.g[A0][0], 0x10d8));
        c.g[V0][0] = c.lh(oc429_at(c.g[V1][0], 0x60));
        c.g[T0][0] = sx(oc429_at(c.g[V1][0], 0x64));
        let aligned = c.g[A1][0] == 0;
        c.g[A0][0] = sx(oc429_at(c.g[A0][0], c.g[V0][0] as u32));
        c.cycles += 1;
        c.g[V0][0] = sx(oc429_at(c.g[A3][0], 0x280));
        if aligned {
            oc429_chunk_copy(c, false, A3, A2, V0, [T1, T2, T3, V1], 0x0022_8dd0);
        } else {
            oc429_chunk_copy(c, true, A3, A2, V0, [10, 11, V1, A1], 0x0022_8d78);
        }
        // Tail of copy 2, then copy 3: options -> s0 (`GetOptions().b5`).
        c.cycles += 11;
        c.ldl(A1, oc429_at(c.g[A3][0], 0x7));
        c.ldr(A1, oc429_at(c.g[A3][0], 0));
        c.sdl(A1, oc429_at(c.g[A2][0], 0x7));
        c.sdr(A1, oc429_at(c.g[A2][0], 0));
        c.g[A3][0] = sx(oc429_at(c.g[S1][0], 0x5610));
        c.g[A2][0] = c.g[S0][0];
        c.g[V0][0] = c.lbu(oc429_at(c.g[SP][0], 0x6));
        c.g[V1][0] = c.g[A3][0] & 0x7;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
        let aligned = c.g[V1][0] == 0;
        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
        c.cycles += 1;
        c.g[V1][0] = sx(oc429_at(c.g[A3][0], 0x280));
        if aligned {
            oc429_chunk_copy(c, false, A3, A2, V1, [V0, T1, T2, T3], 0x0022_8e84);
        } else {
            oc429_chunk_copy(c, true, A3, A2, V1, [V0, T1, T2, T3], 0x0022_8e2c);
        }
        c.cycles += 12;
        c.ldl(V0, oc429_at(c.g[A3][0], 0x7));
        c.ldr(V0, oc429_at(c.g[A3][0], 0));
        c.sdl(V0, oc429_at(c.g[A2][0], 0x7));
        c.sdr(V0, oc429_at(c.g[A2][0], 0));
        c.g[V0][0] = c.lw(c.g[T0][0] as u32);
        let target = c.g[V0][0] as u32;
        c.g[A2][0] = c.lb(oc429_at(c.g[SP][0], 0x295));
        call_ptr(c, target, 0x0022_8ecc)?;
        frame.restore(c, S0, 0x540);
        frame.restore(c, S1, 0x530);
        frame.restore_ra(c, 0x520);
        frame.close(c);
        Ok(())
    })
}

/// `ssx_LoadNetModules()` (0x228ee0): load the network IOP modules through
/// the module loader (`0x319718(loader, path, wait, arglen, args)`, loader at
/// `gp + 0xda4`): USBD, USBKB, DEV9, (`0x319930`), INET, NETCNF (its argument
/// string built by `sprintf`), INETCTL, SMAP, PPP, SPDUART (another
/// `sprintf`), PPPOE (only when both the SMAP and the SPDUART load worked);
/// then, if any network adapter loaded (`gp - 0x704`), `yield(10)` and the
/// LIBNET / DRTYSCKF / LGAUD / VOIPF modules and the voice start/stop
/// (`0x3f6a40` or `0x3f6ab0`); finally `0x266788()`. All modules go through
/// `call_ptr`: the constants are string addresses in the `0x47xxxx` data.
pub fn ssx_load_net_modules(c: &mut Cpu) {
    guest(c, |c| {
        const LOAD: u32 = 0x0031_9718;
        const SPRINTF: u32 = 0x0041_7828;
        c.cycles += 77;
        let frame = Frame::open(c, 0x140);
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x48_0000;
        c.g[T0][0] = 0x48_0000;
        frame.save(c, S0, 0x130);
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5678)));
        frame.save(c, S1, 0x120);
        c.g[T0][0] = sx(oc429_at(c.g[T0][0], 0u32.wrapping_sub(0x5668)));
        frame.save(c, S2, 0x110);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x100);
        c.g[A3][0] = 0x2b;
        call_ptr(c, LOAD, 0x0022_8f14)?; // USBD.IRX ("hub")
        c.g[S2][0] = 0;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x48_0000;
        c.g[T0][0] = 0x48_0000;
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5638)));
        c.g[T0][0] = sx(oc429_at(c.g[T0][0], 0u32.wrapping_sub(0x5628)));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0x10;
        call_ptr(c, LOAD, 0x0022_8f38)?; // USBKB.IRX ("keybd")
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x48_0000;
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5618)));
        c.g[T0][0] = 0;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, LOAD, 0x0022_8f54)?; // DEV9.IRX
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        call_ptr(c, 0x0031_9930, 0x0022_8f5c)?;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x48_0000;
        c.g[T0][0] = 0x48_0000;
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5608)));
        c.g[T0][0] = sx(oc429_at(c.g[T0][0], 0u32.wrapping_sub(0x55f8)));
        c.g[A2][0] = 1;
        c.g[A3][0] = 0x12;
        call_ptr(c, LOAD, 0x0022_8f7c)?; // INET.IRX ("mem=96KB")
                                         // sprintf(buf, "icon...", ...)
        c.g[V1][0] = 0x48_0000;
        c.g[V0][0] = 0x4a_0000;
        c.g[S0][0] = sx(oc429_at(c.g[V1][0], 0u32.wrapping_sub(0x66e8)));
        c.g[S1][0] = sx(oc429_at(c.g[V0][0], 0x29f0));
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = c.g[S1][0];
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x55e0)));
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = 0;
        call_ptr(c, SPRINTF, 0x0022_8fb0)?;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x48_0000;
        c.g[T0][0] = c.g[SP][0];
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x55a8)));
        c.g[A3][0] = sx(oc429_at(c.g[V0][0], 1));
        c.g[A2][0] = 1;
        call_ptr(c, LOAD, 0x0022_8fcc)?; // NETCNF.IRX (buf)
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x48_0000;
        c.g[T0][0] = 0x48_0000;
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5598)));
        c.g[T0][0] = sx(oc429_at(c.g[T0][0], 0u32.wrapping_sub(0x5588)));
        c.g[A2][0] = 1;
        c.g[A3][0] = 0x14;
        call_ptr(c, LOAD, 0x0022_8fec)?; // INETCTL.IRX ("-no_auto")
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5570)));
        c.sw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x704)), 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, LOAD, 0x0022_900c)?; // SMAP.IRX
        let smap_failed = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = 1;
        if !smap_failed {
            c.cycles += 2;
            c.g[S2][0] = 1;
            c.sw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x704)), c.g[V0][0]);
        }
        // PPP, then the dial-up string and SPDUART.
        c.cycles += 23;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x4a_0000;
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0x29f8));
        c.g[A2][0] = 1;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, LOAD, 0x0022_9038)?; // PPP.IRX
        c.g[A1][0] = 0x48_0000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5560)));
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[T0][0] = 0;
        call_ptr(c, SPRINTF, 0x0022_9054)?;
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5538)));
        c.g[A3][0] = sx(oc429_at(c.g[V0][0], 1));
        c.g[A2][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        call_ptr(c, LOAD, 0x0022_9070)?; // SPDUART.IRX (buf)
        let spd_failed = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = 1;
        if !spd_failed {
            c.cycles += 1;
            c.sw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x704)), c.g[V0][0]);
        }
        // PPPOE when an adapter loaded and the SMAP one did.
        'pppoe: {
            c.cycles += 3;
            c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x704)));
            if c.g[V0][0] == 0 {
                break 'pppoe;
            }
            c.cycles += 2;
            let no_smap = c.g[S2][0] == 0;
            c.g[A1][0] = 0x48_0000;
            if no_smap {
                break 'pppoe;
            }
            c.cycles += 6;
            c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
            c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5528)));
            c.g[A2][0] = 1;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, LOAD, 0x0022_90a8)?; // PPPOE.IRX
        }
        c.cycles += 5;
        c.g[A0][0] = 10;
        call_ptr(c, 0x003e_52b0, 0x0022_90b0)?; // THREAD_yieldticks(10)
        c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x704)));
        if c.g[V0][0] != 0 {
            'voip: {
                c.cycles += 35;
                c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
                call_ptr(c, 0x0031_97d0, 0x0022_90c4)?;
                c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
                c.g[A1][0] = 0x48_0000;
                c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5518)));
                c.g[A2][0] = 1;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                call_ptr(c, LOAD, 0x0022_90e0)?; // LIBNET.IRX
                c.g[A0][0] = 0;
                call_ptr(c, 0x0040_b130, 0x0022_90e8)?;
                c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
                c.g[A1][0] = 0x48_0000;
                c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5508)));
                c.g[T0][0] = 0;
                c.g[A2][0] = 1;
                c.g[A3][0] = 0;
                call_ptr(c, LOAD, 0x0022_9104)?; // DRTYSCKF.IRX
                c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
                c.g[A1][0] = 0x48_0000;
                c.g[T0][0] = 0x48_0000;
                c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x54f8)));
                c.g[T0][0] = sx(oc429_at(c.g[T0][0], 0u32.wrapping_sub(0x54e8)));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0x12;
                call_ptr(c, LOAD, 0x0022_9124)?; // LGAUD.IRX ("rpc=0")
                c.g[A1][0] = 0x48_0000;
                c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
                c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x54d0)));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                call_ptr(c, LOAD, 0x0022_9140)?; // VOIPF.IRX
                if (c.g[V0][0] as i64) >= 0 {
                    c.cycles += 4;
                    call_ptr(c, 0x003f_6a40, 0x0022_9150)?;
                    break 'voip;
                }
                c.cycles += 2;
                call_ptr(c, 0x003f_6ab0, 0x0022_9160)?;
            }
        }
        c.cycles += 8;
        call_ptr(c, 0x0026_6788, 0x0022_9168)?;
        frame.restore(c, S0, 0x130);
        frame.restore(c, S1, 0x120);
        frame.restore(c, S2, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXApp_ParseCommandLine(SSXApp* self, int argc, char** argv)`
/// (0x227bf0): scan the arguments from 1. An option (`-name`, matched with
/// `strcasecmp` in the order track, username, password, realfe, nohud,
/// autolobby) acts as follows: `-track <path>` stores the file name
/// without directory and extension at `self + 0xcc` (a `cBXString` built,
/// trimmed at the last `/`, `\` or `:` and at the last `.`); `-username`
/// and `-password` take the next argument (`cBXString` at `self + 0xc4` /
/// `0xc8`); `-realfe` sets `self + 0x60 = 1`, `-nohud` clears `self +
/// 0x64`, `-autolobby` sets `self + 0x74 = 1` (only when the last compare
/// result is 0). Finally `gp + 0x1450 = (self->+0x64 == 0)` (disable-HUD).
///
/// A dispatcher-shaped function (its generated body is a `match pc` loop):
/// the port is the same state machine, with the original's addresses as
/// state names; a resume `pc` other than the entry runs the generated body
/// (at install: its `__generated` alias).
pub fn ssx_app_parse_command_line(c: &mut Cpu) {
    if c.pc != 0x0022_7bf0 {
        return ssx_app_parse_command_line_resume(c);
    }
    // The states (blocks of the original), named by address.
    const CHECK_ARG: u32 = 0x0022_7de0;
    const ARG_DASH: u32 = 0x0022_7dec;
    const OPTION: u32 = 0x0022_7dfc;
    const NEXT_ARG: u32 = 0x0022_7ddc;
    const TRACK_OPT: u32 = 0x0022_7c40;
    const TRACK_TRIM_DIR: u32 = 0x0022_7c94;
    const TRACK_FIND_DOT: u32 = 0x0022_7cc4;
    const TRACK_TRIM_EXT: u32 = 0x0022_7cdc;
    const TRACK_STORE: u32 = 0x0022_7d00;
    const IS_USERNAME: u32 = 0x0022_7d20;
    const USERNAME_ARG: u32 = 0x0022_7d34;
    const USERNAME_STORE: u32 = 0x0022_7d44;
    const IS_PASSWORD: u32 = 0x0022_7d58;
    const PASSWORD_ARG: u32 = 0x0022_7d6c;
    const PASSWORD_STORE: u32 = 0x0022_7d7c;
    const IS_REALFE: u32 = 0x0022_7d90;
    const SET_REALFE: u32 = 0x0022_7da4;
    const IS_NOHUD: u32 = 0x0022_7dac;
    const CLEAR_NOHUD: u32 = 0x0022_7dc0;
    const IS_AUTOLOBBY: u32 = 0x0022_7dc8;
    const SKIP: u32 = 0x0022_7dd8;
    const TRACK_NEXT: u32 = 0x0022_7e18;
    const FINISH: u32 = 0x0022_7e28;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S2, 0x90);
        frame.save(c, S3, 0x80);
        c.g[S2][0] = sx(oc429_at(c.g[A2][0], 4));
        frame.save(c, S4, 0x70);
        c.g[S3][0] = 1;
        frame.save(c, S5, 0x60);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, 22, 0x50);
        c.g[S5][0] = sx(oc429_at(c.g[SP][0], 0x10));
        frame.save(c, S7, 0x40);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, FP, 0x30);
        c.g[S7][0] = 1;
        frame.save(c, S0, 0xb0);
        c.g[FP][0] = 0x48_0000;
        frame.save(c, S1, 0xa0);
        frame.save_ra(c, 0x20);
        let mut pc = CHECK_ARG;
        loop {
            match pc {
                CHECK_ARG => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[V1][0] = 0x2d;
                    pc = if done { FINISH } else { ARG_DASH };
                }
                ARG_DASH => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[S2][0] = sx(oc429_at(c.g[S2][0], 4));
                        if back_edge_unwinds(c, NEXT_ARG) {
                            return Ok(());
                        }
                        pc = NEXT_ARG;
                    } else {
                        pc = OPTION;
                    }
                }
                OPTION => {
                    // `-track`?
                    c.cycles += 7;
                    c.g[S0][0] = sx(oc429_at(c.g[A0][0], 1));
                    c.g[V0][0] = 0x4a_0000;
                    c.g[A1][0] = sx(oc429_at(c.g[V0][0], 0x29b0));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0041_aa88, 0x0022_7e10)?; // strcasecmp
                    let other = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x48_0000;
                    if other {
                        if back_edge_unwinds(c, IS_USERNAME) {
                            return Ok(());
                        }
                        pc = IS_USERNAME;
                    } else {
                        pc = TRACK_NEXT;
                    }
                }
                TRACK_NEXT => {
                    c.cycles += 4;
                    c.g[S3][0] = sx(oc429_at(c.g[S3][0], 1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S2][0] = sx(oc429_at(c.g[S2][0], 4));
                    if more {
                        if back_edge_unwinds(c, TRACK_OPT) {
                            return Ok(());
                        }
                        pc = TRACK_OPT;
                    } else {
                        pc = FINISH;
                    }
                }
                TRACK_OPT => {
                    // `cBXString track(argv[i])`; separators `/`, `\`, `:`.
                    c.cycles += 21;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0031_82d8, 0x0022_7c4c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x2f;
                    call_ptr(c, 0x0031_8ca0, 0x0022_7c58)?; // FindFirstOf
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x5c;
                    call_ptr(c, 0x0031_8ca0, 0x0022_7c68)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x3a;
                    call_ptr(c, 0x0031_8ca0, 0x0022_7c78)?;
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    if c.g[V1][0] == 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[V1][0] = c.g[S0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S1][0];
                    }
                    let no_sep = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = c.lw(sp(c));
                    pc = if no_sep {
                        TRACK_FIND_DOT
                    } else {
                        TRACK_TRIM_DIR
                    };
                }
                TRACK_TRIM_DIR => {
                    // track = track.right(len - sep - 1).
                    c.cycles += 12;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A2][0] = c.lw(oc429_at(c.g[V0][0], 0u32.wrapping_sub(8)));
                    c.g[A2][0] = sx(oc429_at(c.g[A2][0], 0u32.wrapping_sub(c.g[V1][0] as u32)));
                    c.g[A2][0] = sx(oc429_at(c.g[A2][0], 0u32.wrapping_sub(1)));
                    call_ptr(c, 0x0031_9028, 0x0022_7cac)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x0031_8448, 0x0022_7cb8)?; // operator=
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_8260, 0x0022_7cc4)?; // ~cBXString
                    pc = TRACK_FIND_DOT;
                }
                TRACK_FIND_DOT => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x2e;
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x0031_8c18, 0x0022_7cd4)?; // FindLastOf
                    let no_dot = (c.g[V0][0] as i64) < 0;
                    c.g[A1][0] = c.g[SP][0];
                    pc = if no_dot { TRACK_STORE } else { TRACK_TRIM_EXT };
                }
                TRACK_TRIM_EXT => {
                    // track = track.left(dot).
                    c.cycles += 9;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x0031_90a8, 0x0022_7ce8)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x0031_8448, 0x0022_7cf4)?;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_8260, 0x0022_7d00)?;
                    pc = TRACK_STORE;
                }
                TRACK_STORE => {
                    c.cycles += 8;
                    c.g[A0][0] = sx(oc429_at(c.g[S4][0], 0xcc));
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0031_8448, 0x0022_7d0c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_8260, 0x0022_7d18)?;
                    c.g[S2][0] = sx(oc429_at(c.g[S2][0], 4));
                    pc = NEXT_ARG;
                }
                IS_USERNAME => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(oc429_at(c.g[V0][0], 0u32.wrapping_sub(0x5828)));
                    call_ptr(c, 0x0041_aa88, 0x0022_7d2c)?;
                    let other = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x48_0000;
                    pc = if other { IS_PASSWORD } else { USERNAME_ARG };
                }
                USERNAME_ARG | PASSWORD_ARG => {
                    // The option's value is the next argument.
                    c.cycles += 4;
                    c.g[S3][0] = sx(oc429_at(c.g[S3][0], 1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let missing = c.g[V0][0] == 0;
                    c.g[S2][0] = sx(oc429_at(c.g[S2][0], 4));
                    pc = if missing {
                        FINISH
                    } else if pc == USERNAME_ARG {
                        USERNAME_STORE
                    } else {
                        PASSWORD_STORE
                    };
                }
                USERNAME_STORE | PASSWORD_STORE => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    let field = if pc == USERNAME_STORE { 0xc4 } else { 0xc8 };
                    c.g[A0][0] = sx(oc429_at(c.g[S4][0], field));
                    call_ptr(
                        c,
                        0x0031_84e8,
                        if pc == USERNAME_STORE {
                            0x0022_7d50
                        } else {
                            0x0022_7d88
                        },
                    )?;
                    c.g[S2][0] = sx(oc429_at(c.g[S2][0], 4));
                    pc = NEXT_ARG;
                }
                IS_PASSWORD => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(oc429_at(c.g[V0][0], 0u32.wrapping_sub(0x5818)));
                    call_ptr(c, 0x0041_aa88, 0x0022_7d64)?;
                    let other = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x4a_0000;
                    pc = if other { IS_REALFE } else { PASSWORD_ARG };
                }
                IS_REALFE => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(oc429_at(c.g[V0][0], 0x29b8));
                    call_ptr(c, 0x0041_aa88, 0x0022_7d9c)?;
                    let other = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x4a_0000;
                    pc = if other { IS_NOHUD } else { SET_REALFE };
                }
                SET_REALFE => {
                    c.cycles += 2;
                    c.sw(oc429_at(c.g[S4][0], 0x60), c.g[S7][0]);
                    pc = SKIP;
                }
                IS_NOHUD => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(oc429_at(c.g[V0][0], 0x29c0));
                    call_ptr(c, 0x0041_aa88, 0x0022_7db8)?;
                    let other = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    pc = if other { IS_AUTOLOBBY } else { CLEAR_NOHUD };
                }
                CLEAR_NOHUD => {
                    c.cycles += 2;
                    c.sw(oc429_at(c.g[S4][0], 0x64), 0);
                    pc = SKIP;
                }
                IS_AUTOLOBBY => {
                    c.cycles += 4;
                    c.g[A1][0] = sx(oc429_at(c.g[FP][0], 0u32.wrapping_sub(0x5808)));
                    call_ptr(c, 0x0041_aa88, 0x0022_7dd0)?;
                    if c.g[V0][0] == 0 {
                        c.sw(oc429_at(c.g[S4][0], 0x74), c.g[S7][0]);
                    }
                    pc = SKIP;
                }
                SKIP => {
                    c.cycles += 1;
                    c.g[S2][0] = sx(oc429_at(c.g[S2][0], 4));
                    pc = NEXT_ARG;
                }
                NEXT_ARG => {
                    c.cycles += 1;
                    c.g[S3][0] = sx(oc429_at(c.g[S3][0], 1));
                    pc = CHECK_ARG;
                }
                _ => {
                    // FINISH
                    debug_assert_eq!(pc, FINISH);
                    break;
                }
            }
        }
        c.cycles += 15;
        c.g[V0][0] = c.lw(oc429_at(c.g[S4][0], 0x64));
        frame.restore(c, S0, 0xb0);
        c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, 22, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        c.sw(oc429_at(c.g[GP][0], 0x1450), c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// Native resume of `ssx_app_parse_command_line` (converted from the `generated` body): any entry
/// other than 0x00227bf0, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn ssx_app_parse_command_line_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00227bf0 => {
                    c.cycles += 19;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S3][0] = 0x1;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S6][0] = c.g[A1][0];
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S7][0] = 0x1;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[FP][0] = 0x480000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    let t = true;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    if t {
                        pc = 0x00227de0;
                        continue;
                    }
                    pc = 0x00227c3c;
                    continue;
                }
                0x00227c3c => {
                    c.cycles += 1;
                    pc = 0x00227c40;
                    continue;
                }
                0x00227c40 => {
                    c.cycles += 21;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x00227c4c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x003182d8, 0x00227c4c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00227c58);
                    c.g[A1][0] = 0x2f;
                    call_ptr(c, 0x00318ca0, 0x00227c58)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00227c68);
                    c.g[A1][0] = 0x5c;
                    call_ptr(c, 0x00318ca0, 0x00227c68)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00227c78);
                    c.g[A1][0] = 0x3a;
                    call_ptr(c, 0x00318ca0, 0x00227c78)?;
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    if c.g[V1][0] == 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[V1][0] = c.g[S0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S1][0];
                    }
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        pc = 0x00227cc4;
                        continue;
                    }
                    pc = 0x00227c94;
                    continue;
                }
                0x00227c94 => {
                    c.cycles += 12;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x8));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[RA][0] = sx(0x00227cac);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    call_ptr(c, 0x00319028, 0x00227cac)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00227cb8);
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x00318448, 0x00227cb8)?;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00227cc4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x00227cc4)?;
                    pc = 0x00227cc4;
                    continue;
                }
                0x00227cc4 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x2e;
                    c.g[RA][0] = sx(0x00227cd4);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x00318c18, 0x00227cd4)?;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[A1][0] = c.g[SP][0];
                    if t {
                        pc = 0x00227d00;
                        continue;
                    }
                    pc = 0x00227cdc;
                    continue;
                }
                0x00227cdc => {
                    c.cycles += 9;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00227ce8);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x003190a8, 0x00227ce8)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00227cf4);
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x00318448, 0x00227cf4)?;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00227d00);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x00227d00)?;
                    pc = 0x00227d00;
                    continue;
                }
                0x00227d00 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0xcc));
                    c.g[RA][0] = sx(0x00227d0c);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x00318448, 0x00227d0c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00227d18);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x00227d18)?;
                    let t = true;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00227ddc;
                        continue;
                    }
                    pc = 0x00227d20;
                    continue;
                }
                0x00227d20 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00227d2c);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5828));
                    call_ptr(c, 0x0041aa88, 0x00227d2c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00227d58;
                        continue;
                    }
                    pc = 0x00227d34;
                    continue;
                }
                0x00227d34 => {
                    c.cycles += 4;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00227e28;
                        continue;
                    }
                    pc = 0x00227d44;
                    continue;
                }
                0x00227d44 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x00227d50);
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0xc4));
                    call_ptr(c, 0x003184e8, 0x00227d50)?;
                    let t = true;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00227ddc;
                        continue;
                    }
                    pc = 0x00227d58;
                    continue;
                }
                0x00227d58 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00227d64);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5818));
                    call_ptr(c, 0x0041aa88, 0x00227d64)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00227d90;
                        continue;
                    }
                    pc = 0x00227d6c;
                    continue;
                }
                0x00227d6c => {
                    c.cycles += 4;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00227e28;
                        continue;
                    }
                    pc = 0x00227d7c;
                    continue;
                }
                0x00227d7c => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x00227d88);
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0xc8));
                    call_ptr(c, 0x003184e8, 0x00227d88)?;
                    let t = true;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x00227ddc;
                        continue;
                    }
                    pc = 0x00227d90;
                    continue;
                }
                0x00227d90 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00227d9c);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x29b8));
                    call_ptr(c, 0x0041aa88, 0x00227d9c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00227dac;
                        continue;
                    }
                    pc = 0x00227da4;
                    continue;
                }
                0x00227da4 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x60), c.g[S7][0]);
                    if t {
                        pc = 0x00227dd8;
                        continue;
                    }
                    pc = 0x00227dac;
                    continue;
                }
                0x00227dac => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00227db8);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x29c0));
                    call_ptr(c, 0x0041aa88, 0x00227db8)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00227dc8;
                        continue;
                    }
                    pc = 0x00227dc0;
                    continue;
                }
                0x00227dc0 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x64), 0u64);
                    if t {
                        pc = 0x00227dd8;
                        continue;
                    }
                    pc = 0x00227dc8;
                    continue;
                }
                0x00227dc8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00227dd0);
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x5808));
                    call_ptr(c, 0x0041aa88, 0x00227dd0)?;

                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x74), c.g[S7][0]);
                        pc = 0x00227dd8;
                        continue;
                    }
                    pc = 0x00227dd8;
                    continue;
                }
                0x00227dd8 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    pc = 0x00227ddc;
                    continue;
                }
                0x00227ddc => {
                    c.cycles += 1;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    pc = 0x00227de0;
                    continue;
                }
                0x00227de0 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0x2d;
                    if t {
                        pc = 0x00227e28;
                        continue;
                    }
                    pc = 0x00227dec;
                    continue;
                }
                0x00227dec => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V0][0] = c.lb(c.g[A0][0] as u32);

                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                        if back_edge_unwinds(c, 0x00227ddc) {
                            return Ok(());
                        }
                        pc = 0x00227ddc;
                        continue;
                    }
                    pc = 0x00227dfc;
                    continue;
                }
                0x00227dfc => {
                    c.cycles += 7;
                    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x29b0));
                    c.g[RA][0] = sx(0x00227e10);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x00227e10)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        if back_edge_unwinds(c, 0x00227d20) {
                            return Ok(());
                        }
                        pc = 0x00227d20;
                        continue;
                    }
                    pc = 0x00227e18;
                    continue;
                }
                0x00227e18 => {
                    c.cycles += 4;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x00227c40) {
                            return Ok(());
                        }
                        pc = 0x00227c40;
                        continue;
                    }
                    pc = 0x00227e28;
                    continue;
                }
                0x00227e28 => {
                    c.cycles += 15;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x64));
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x1450), c.g[V0][0]);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00227bf0, 0x00227e64);
                    return Ok(());
                }
            }
        }
    });
}

/// `void SSXApp_Init(cSSXApp* self, int argc, char** argv)` (0x226900): the
/// application's start-up: console config, the pad cache (0x2ef0-byte player
/// manager), the graphics manager and its vtable setup, light and bezier
/// managers, the two input pads, the command line, the load indicator, the
/// pre-start screen module and the FE async manager.
///
/// NOT registered: `SSXApp_ParseCommandLine` and the console/app-manager
/// constructors it calls are real, unslotted functions that read the
/// disc/config, so no bounded differential fixture exists. The port keeps the
/// generated register sequence and only converts the call protocol.
#[allow(
    unused_labels,
    unreachable_code,
    unused_assignments,
    unused_variables,
    clippy::all
)]
pub fn ssx_app_init_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.g[A2][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S4][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa8));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = 0x1;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[S5][0] = 0x480000;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x848), c.g[S2][0]);
        call_ptr(c, 0x002c7440, 0x00226940)?;
        c.g[A1][0] = 0x480000;
        c.g[A2][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x65c0));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0x2efc;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
        call_ptr(c, 0x00317d70, 0x0022695c)?;
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        call_ptr(c, 0x00326a28, 0x0022696c)?;
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x850), c.g[V0][0]);
        call_ptr(c, 0x00375a08, 0x00226978)?;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10d8));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x854), c.g[V0][0]);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00226990)?;
        call_ptr(c, 0x0038dbd0, 0x00226998)?;
        call_ptr(c, 0x0038aef8, 0x002269a0)?;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = 0x1;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x65b0));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x002269b8)?;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x844), c.g[V0][0]);
        'l_2269c0: loop {
            c.cycles += 10;
            c.g[A0][0] = 0x2a4;
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x65a0));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317d70, 0x002269d4)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.sw(c.g[V0][0] as u32, 0u64);
            let t = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002269c0);
                }
                continue 'l_2269c0;
            } else {
                c.cycles += 94;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x84), 0u64);
                c.g[A2][0] = c.g[S3][0];
                c.sw((c.g[S2][0] as u32).wrapping_add(0x80), 0u64);
                c.g[A0][0] = c.g[S2][0];
                c.sw((c.g[S2][0] as u32).wrapping_add(0x7c), 0u64);
                c.g[A1][0] = c.g[S4][0];
                call_ptr(c, 0x00227bf0, 0x00226a04)?;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A1][0] = 0x100;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x160));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x164));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00226a20)?;
                c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A0][0] = 0x480000;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x54c0));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10d8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28b0));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x170));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x003b39b8, 0x00226a44)?;
                c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x174));
                c.g[A2][0] = 0x480000;
                c.g[T0][0] = 0x1;
                c.g[T1][0] = 0xffffffffffffffff;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6590));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[T2][0] as u32;
                c.g[A3][0] = 0u64;
                call_ptr(c, t, 0x00226a68)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                call_ptr(c, 0x00369ff8, 0x00226a78)?;
                c.g[A1][0] = 0x480000;
                c.g[A0][0] = 0x1c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6580));
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x00226a90)?;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A1][0] = 0x480000;
                c.sw(c.g[V0][0] as u32, 0u64);
                c.g[A2][0] = 0x100;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.g[A3][0] = 0u64;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6570));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x1f));
                c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x13));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x1b));
                c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = 0xc;
                call_ptr(c, 0x00317d70, 0x00226af0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00232328, 0x00226af8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00316ef8, 0x00226b04)?;
                call_ptr(c, 0x00237cf8, 0x00226b0c)?;
                c.g[A1][0] = 0x480000;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6560));
                c.g[A3][0] = 0u64;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
                c.g[A0][0] = 0x590;
                call_ptr(c, 0x00317d70, 0x00226b28)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x001a3590, 0x00226b30)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x11c), c.g[V0][0]);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x001a3648, 0x00226b3c)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.pc = t;
                return Ok(());
            }
        }
        Ok(())
    })
}

/// `void SSXApp_LoadLocale(self)` (0x228a78): makes sure the front-end locale
/// exists (`self + 0x8c`: allocates 0xc0 bytes named "FELocale" and runs its
/// constructor), asks the BE for interface 4, copies the 0x288-byte options
/// block at 0x535610 to the stack and calls the locale's virtual method 3
/// with the language (`options.word0 >> 22 & 7`), then loads the common
/// table (`FELocale_LoadCommonTable`). Frame 0x2b0 (s0).
pub fn ssx_app_load_locale(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x2b0);
        frame.save(c, S0, 0x2a0);
        frame.save_ra(c, 0x290);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[V0][0] = c.lw(this.wrapping_add(0x8c));
        let have_locale = c.g[V0][0] != 0;
        c.g[A0][0] = 0xc0;
        if !have_locale {
            c.cycles += 8;
            c.g[A1][0] = 0x48_0000;
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x5688));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0022_8aa8)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0019_5880, 0x0022_8ab0)?;
            c.sw(this.wrapping_add(0x8c), c.g[V0][0]);
        }
        c.cycles += 15;
        call_ptr(c, 0x0014_dc80, 0x0022_8abc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_dd58, 0x0022_8ac8)?;
        c.g[A1][0] = c.lw(this.wrapping_add(0x8c));
        c.g[V0][0] = 0x53_0000;
        c.g[A3][0] = sx(r32(c, V0).wrapping_add(0x5610));
        c.g[A2][0] = c.g[SP][0];
        let locale = r32(c, A1);
        c.g[V1][0] = c.lw(locale.wrapping_add(4));
        c.g[A0][0] = c.g[A3][0] & 7;
        c.g[V0][0] = c.lh(r32(c, V1).wrapping_add(0x18));
        c.g[T0][0] = sx(r32(c, V1).wrapping_add(0x1c));
        // 0x535610 is 8-aligned: the aligned 8-byte copy loop always runs.
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, A3).wrapping_add(0x280));
        loop {
            c.cycles += 11;
            let src = r32(c, A3);
            let dst = r32(c, A2);
            c.g[V1][0] = c.ld(src);
            c.g[A0][0] = c.ld(src.wrapping_add(8));
            c.g[T1][0] = c.ld(src.wrapping_add(0x10));
            c.g[T2][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[V1][0]);
            c.sd(dst.wrapping_add(8), c.g[A0][0]);
            c.sd(dst.wrapping_add(0x10), c.g[T1][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T2][0]);
            c.g[A3][0] = sx(src.wrapping_add(0x20));
            let more = c.g[A3][0] != c.g[V0][0];
            c.g[A2][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_8b4c);
            }
        }
        c.cycles += 16;
        c.ldl(V0, r32(c, A3).wrapping_add(7));
        c.ldr(V0, r32(c, A3));
        c.sdl(V0, r32(c, A2).wrapping_add(7));
        c.sdr(V0, r32(c, A2));
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = c.lw(r32(c, T0));
        c.g[A1][0] = c.lw(r32(c, SP));
        c.g[A1][0] = sx(r32(c, A1) >> 22);
        let method = r32(c, V0);
        c.g[A1][0] &= 7;
        call_ptr(c, method, 0x0022_8ba0)?;
        c.g[A0][0] = c.lw(this.wrapping_add(0x8c));
        call_ptr(c, 0x0019_5b70, 0x0022_8ba8)?;
        frame.restore(c, S0, 0x2a0);
        frame.restore_ra(c, 0x290);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXApp_FlushTicks(void)` (0x227e68): when the pad-queue global
/// (`gp-0x850`) is set, call the queue flush (0x326c60) and return its result,
/// else return 0.
#[allow(unreachable_code)]
pub fn ssx_app_flush_ticks(c: &mut Cpu) {
    guest(c, |c| {
        'b_227e88: {
            'b_227e80: {
                c.cycles += 4;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x850));
                let t = c.g[A0][0] != 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_227e80;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_227e88;
                    } else {
                        break 'b_227e80;
                    }
                }
            }
            c.cycles += 2;
            call_ptr(c, 0x00326c60, 0x00227e88)?;
            break 'b_227e88;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `CrowdMan2d* CrowdMan2d_ctor(CrowdMan2d* self)` (0x229180): clears the 128 + 40
/// id words (`-1`) of the crowd tables, publishes `self` in the global at `gp - 0x6f0`,
/// builds the 2D crowd renderer and its animation set, sets up the first animation
/// record (`0x536690`) and resets the manager. Returns `self`.
pub fn crowd_man2d_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V1][0] = 0xffffffffffff0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = 0x7f;
        c.g[S1][0] = 0x530000;
        c.g[A1][0] = 0xffffffffffffffff;
        'l_2291b0: loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            let t = c.g[A0][0] != c.g[A1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002291b0);
                }
                continue 'l_2291b0;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x7f;
                c.g[V1][0] = 0xffffffffffffffff;
                c.g[V0][0] = 0x27;
                c.g[V1][0] = 0xffffffffffffffff;
                'l_2291e0: loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[A2][0] = 0x20000000;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002291e0);
                        }
                        continue 'l_2291e0;
                    } else {
                        c.cycles += 31;
                        c.g[A1][0] = 0x480000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4ce0));
                        c.g[A3][0] = 0u64;
                        c.g[A0][0] = 0x10;
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6f0), c.g[S0][0]);
                        call_ptr(c, 0x00317d70, 0x00229214)?;
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x002db6d8, 0x0022921c)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        call_ptr(c, 0x002db980, 0x00229228)?;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6690));
                        c.g[A2][0] = 0xffffffffffffffff;
                        c.g[A3][0] = 0x20000;
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[S0][0];
                        c.sh((c.g[V1][0] as u32).wrapping_add(0x6), c.g[A2][0]);
                        c.sh((c.g[S1][0] as u32).wrapping_add(0x6690), c.g[A1][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[A3][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
                        c.sh((c.g[V1][0] as u32).wrapping_add(0x2), c.g[A2][0]);
                        c.sh((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                        Callee {
                            addr: 0x002292e0,
                            f: ee::s05::CrowdMan2d_Reset,
                        }
                        .call(c, 0x00229260)?;
                        c.g[V0][0] = c.g[S0][0];
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `SSXApp::SSXApp(void* self)` (0x226830): constructs the app manager
/// (0x316cd0), sets the app's fields, clears the 11-word table at
/// `self+0x114` down to `self+0xbc` (the loop runs with event checks), and
/// zeroes three 8-word blocks with `MEM_fill` (0x3e6448).
pub fn ssxapp_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0031_6cd0, 0x0022_6844)?;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xda0)).lw(c);
        c.g[V0][0] = 0x0047_d9c8;
        let base = c.g[S0][0] as u32;
        Addr(base.wrapping_add(0xd4)).sw(c, 0);
        let vtable = c.g[V0][0];
        Addr(base.wrapping_add(0x5c)).sw(c, vtable);
        c.g[A0][0] = 0xa;
        let v1 = c.g[V1][0];
        Addr(base.wrapping_add(0xcc)).sw(c, v1);
        c.g[V0][0] = sx(base.wrapping_add(0x114));
        Addr(base.wrapping_add(0xc4)).sw(c, v1);
        Addr(base.wrapping_add(0xc8)).sw(c, v1);
        Addr(base.wrapping_add(0xd8)).sw(c, 0);
        Addr(base.wrapping_add(0xdc)).sw(c, 0);
        Addr(base.wrapping_add(0xe0)).sw(c, 0);
        Addr(base.wrapping_add(0xe4)).sw(c, 0);
        Addr(base.wrapping_add(0xe8)).sw(c, 0);
        loop {
            c.cycles += 7;
            let slot = c.g[V0][0] as u32;
            Addr(slot).sw(c, 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            let more = (c.g[A0][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_6880);
            }
        }
        c.cycles += 25;
        c.g[V0][0] = 1;
        let base = c.g[S0][0] as u32;
        Addr(base.wrapping_add(0x118)).sw(c, 0);
        Addr(base.wrapping_add(0x74)).sw(c, 0);
        c.g[A0][0] = sx(base.wrapping_add(0xa8));
        Addr(base.wrapping_add(0x68)).sw(c, 0);
        c.g[A1][0] = 0;
        Addr(base.wrapping_add(0x6c)).sw(c, 0);
        c.g[A2][0] = 8;
        Addr(base.wrapping_add(0x70)).sw(c, 0);
        let one = c.g[V0][0];
        Addr(base.wrapping_add(0x64)).sw(c, one);
        Addr(base.wrapping_add(0x60)).sw(c, one);
        call_ptr(c, 0x003e_6448, 0x0022_68cc)?;
        let base = c.g[S0][0] as u32;
        c.g[A0][0] = sx(base.wrapping_add(0xb8));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, 0x003e_6448, 0x0022_68dc)?;
        let base = c.g[S0][0] as u32;
        c.g[A0][0] = sx(base.wrapping_add(0xb0));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, 0x003e_6448, 0x0022_68ec)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool cSSXApp::hasMCManager(void)` (0x227fa0): tail-calls
/// `cMCOverlayManager_hasManager` (0x23c850) through the function table.
pub fn mc_manager_exists_wrapper(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0023_c850), 0x0022_7fb0);
}

/// `void cSSXApp::v02(void* self)` (0x228bb8): tears down the world trigger
/// manager (0x2b4b48), the audio system (0x284c28) and the app base (0x317400),
/// then calls the app's virtual slot at `+0x3bc` through the table.
pub fn ssxapp_v02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002b_4b48, 0x0022_8bcc)?;
        call_ptr(c, 0x0028_4c28, 0x0022_8bd4)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7400, 0x0022_8bdc)?;
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a90)).lw(c);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10d8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x3b8)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x3bc)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0022_8bf4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `operator delete` (0x317e50).
pub(crate) const SSXAPP_OPERATOR_DELETE: u32 = 0x0031_7e50;

/// `FILESYS_delbigsync(file, 100)` (0x3dedc0).
pub(crate) const SSXAPP_DELBIGSYNC: u32 = 0x003d_edc0;

/// `this->field` (`this` is `s2`, read after every call: callees keep it).
pub(crate) fn ssxapp_field(c: &Cpu, off: u32) -> u32 {
    (c.g[S2][0] as u32).wrapping_add(off)
}

/// The global at `gp - off`.
pub(crate) fn ssxapp_global(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// `FILESYS_delbigsync(a0, 100)` after `cost` cycles, returning to `ret_addr`.
pub(crate) fn ssxapp_delbigsync(c: &mut Cpu, cost: u64, ret_addr: u32) -> Flow {
    c.cycles += cost;
    c.g[A1][0] = 0x64;
    call_ptr(c, SSXAPP_DELBIGSYNC, ret_addr)
}

/// The tail of a big-file block: `if (a0) delbigsync(a0, 100)`, then
/// `a0 = this->next` (loaded in both cases).
pub(crate) fn ssxapp_free_then_load(c: &mut Cpu, cost: u64, ret_addr: u32, next: u32) -> Flow {
    if c.g[A0][0] != 0 {
        ssxapp_delbigsync(c, cost, ret_addr)?;
    }
    let a = ssxapp_field(c, next);
    c.g[A0][0] = c.lw(a);
    Ok(())
}

/// A virtual call on `obj`: `v0` is the vtable (`lw` at `obj + vt_off`), `a0`
/// the this-adjustment (`lh` at slot), `v1` the function (`lw` at slot + 4);
/// calls `fn(obj + adjust)` returning to `ret_addr`.
pub(crate) fn ssxapp_vcall(c: &mut Cpu, obj: u32, vt_off: u32, slot: u32, ret_addr: u32) -> Flow {
    let table_at = obj.wrapping_add(vt_off);
    c.g[V0][0] = c.lw(table_at);
    let vt = c.g[V0][0] as u32;
    c.g[A0][0] = c.lh(vt.wrapping_add(slot));
    c.g[V1][0] = c.lw(vt.wrapping_add(slot + 4));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_addr)
}

/// `void cSSXApp::purge(cSSXApp* self)` (`SSXApp_Shutdown`, 0x227fc0): tears
/// the app down. Frees the input maps, the font interface and the pad cache,
/// deinitialises the script engine and the graphics manager, frees the big
/// files (the texture array is cleared in a loop of 11), deinits the world
/// triggers and the audio, then the BE, the game-mode manager, the async
/// manager and the console config, and clears the global `SSXApp`.
pub fn ssxapp_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        c.g[S0][0] = sx(ssxapp_field(c, 0xb8));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x1;

        // for (i = 0; i < 2; i++) if (InputMaps[i]) operator_delete(InputMaps[i]);
        'l_227fe0: loop {
            'b_227ff4: {
                c.cycles += 3;
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                let t = c.g[A0][0] == 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                if t {
                    break 'b_227ff4;
                }
                c.cycles += 2;
                call_ptr(c, SSXAPP_OPERATOR_DELETE, 0x0022_7ff4)?;
            }
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break 'l_227fe0;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_7fe0);
            }
        }

        // Fonts: if (Fonts) { purgeFonts(Fonts); delete Fonts; } s0 = PadCache.
        c.cycles += 3;
        let a = ssxapp_field(c, 0x88);
        c.g[S0][0] = c.lw(a);
        if c.g[S0][0] == 0 {
            let a = ssxapp_global(c, 0x850);
            c.g[S0][0] = c.lw(a);
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039_8438, 0x0022_8010)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, SSXAPP_OPERATOR_DELETE, 0x0022_8018)?;
            let a = ssxapp_global(c, 0x850);
            c.g[S0][0] = c.lw(a);
        }

        // Fonts = 0; PadCache = 0; cPadCache_purge(mgr); delete mgr.
        c.cycles += 9;
        let a = ssxapp_field(c, 0x88);
        c.sw(a, 0);
        c.g[A0][0] = c.g[S0][0];
        let a = ssxapp_global(c, 0x850);
        c.sw(a, 0);
        call_ptr(c, 0x0032_6a68, 0x0022_802c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, SSXAPP_OPERATOR_DELETE, 0x0022_8034)?;

        // if (ScriptEngine) { DeInitOnce(ScriptEngine); vt[1](engine, 3); ScriptEngine = 0; }
        let a = ssxapp_global(c, 0x84c);
        c.g[A0][0] = c.lw(a);
        let t = c.g[A0][0] == 0;
        let a = ssxapp_global(c, 0x854);
        c.g[A1][0] = c.lw(a);
        if !t {
            'b_228068: {
                c.cycles += 5;
                call_ptr(c, 0x0027_8308, 0x0022_8048)?;
                let a = ssxapp_global(c, 0x84c);
                c.g[A2][0] = c.lw(a);
                let t = c.g[A2][0] == 0;
                c.g[A1][0] = 0x3;
                if t {
                    break 'b_228068;
                }
                c.cycles += 5;
                let engine = c.g[A2][0] as u32;
                ssxapp_vcall(c, engine, 0x2a8, 0x8, 0x0022_8068)?;
            }
            c.cycles += 2;
            let a = ssxapp_global(c, 0x84c);
            c.sw(a, 0);
            let a = ssxapp_global(c, 0x854);
            c.g[A1][0] = c.lw(a);
        }

        // GrMan->Purge(); if (GrMan) GrMan->dtor(3); GrMan = 0.
        c.cycles += 8;
        let grman = c.g[A1][0] as u32;
        ssxapp_vcall(c, grman, 0x10d8, 0x18, 0x0022_8084)?;
        let a = ssxapp_global(c, 0x854);
        c.g[A2][0] = c.lw(a);
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !t {
            c.cycles += 5;
            let obj = c.g[A2][0] as u32;
            ssxapp_vcall(c, obj, 0x10d8, 0x8, 0x0022_80a4)?;
        }

        // Script data, model and the texture array (11 entries from +0xec).
        c.cycles += 3;
        let a = ssxapp_field(c, 0xe4);
        c.g[A0][0] = c.lw(a);
        let a = ssxapp_global(c, 0x854);
        c.sw(a, 0);
        if c.g[A0][0] != 0 {
            ssxapp_delbigsync(c, 2, 0x0022_80b8)?;
        }
        c.cycles += 3;
        let a = ssxapp_field(c, 0xe8);
        c.g[A0][0] = c.lw(a);
        c.g[S0][0] = sx(ssxapp_field(c, 0xec));
        if c.g[A0][0] != 0 {
            ssxapp_delbigsync(c, 3, 0x0022_80cc)?;
            c.g[S0][0] = sx(ssxapp_field(c, 0xec));
        }
        c.cycles += 2;
        c.g[S1][0] = 0xa;
        'l_2280d8: loop {
            'b_2280f0: {
                c.cycles += 3;
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                if c.g[A0][0] == 0 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    break 'b_2280f0;
                }
                ssxapp_delbigsync(c, 3, 0x0022_80ec)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            }
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break 'l_2280d8;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_80d8);
            }
        }

        // Rewards big file.
        c.cycles += 3;
        let a = ssxapp_field(c, 0x118);
        c.g[A0][0] = c.lw(a);
        if c.g[A0][0] != 0 {
            ssxapp_delbigsync(c, 2, 0x0022_810c)?;
        }

        // WorldTriggerManager, audio; then the audio, music and speech big files and BE.
        c.cycles += 7;
        call_ptr(c, 0x002b_4b48, 0x0022_8114)?;
        call_ptr(c, 0x0028_4c28, 0x0022_811c)?;
        let a = ssxapp_field(c, 0xd4);
        c.g[A0][0] = c.lw(a);
        ssxapp_free_then_load(c, 3, 0x0022_8130, 0xd8)?;
        c.cycles += 2;
        ssxapp_free_then_load(c, 3, 0x0022_8144, 0xdc)?;
        c.cycles += 2;
        ssxapp_free_then_load(c, 3, 0x0022_8158, 0xe0)?;
        c.cycles += 2;
        ssxapp_free_then_load(c, 3, 0x0022_816c, 0x78)?;
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, 0x0014_dd98, 0x0022_8180)?;
        }

        // GameModeMan, then the async manager (dtor flags 3) and the console config.
        c.cycles += 5;
        let a = ssxapp_field(c, 0xc0);
        c.g[A0][0] = c.lw(a);
        call_ptr(c, 0x0023_80e8, 0x0022_8188)?;
        let a = ssxapp_field(c, 0x11c);
        c.g[A0][0] = c.lw(a);
        if c.g[A0][0] == 0 {
            let a = ssxapp_field(c, 0x9c);
            c.g[A2][0] = c.lw(a);
        } else {
            c.cycles += 3;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x001a_35e8, 0x0022_819c)?;
            let a = ssxapp_field(c, 0x9c);
            c.g[A2][0] = c.lw(a);
        }
        c.cycles += 2;
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !t {
            c.cycles += 5;
            let console = c.g[A2][0] as u32;
            ssxapp_vcall(c, console, 0x34, 0x8, 0x0022_81bc)?;
        }

        // Epilogue: SSXApp = 0; return.
        c.cycles += 7;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let a = ssxapp_global(c, 0x848);
        c.sw(a, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void InputMap_LoadFromFile(void *self, const char *path)` (0x226b60),
/// `cSSXApp_loadInputMap` in the reference. Reads the map file at `path`
/// (its `#include`s expanded) into a new 0x18-byte `InputMap` initialised for
/// 205 bindings, frees the text, compiles the bindings and purges the file.
/// Returns the map in v0.
///
/// The first callee, `InputPreproc_Run` (0x325f48), has no slot; the test runs
/// it for real with its own callees (`InputPreproc_ProcessFile`, the
/// allocators) faked in their slots.
pub fn oc671_input_map_load_from_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x20);
        // `size` out-parameter in the frame.
        c.g[A2][0] = sx(frame.at(c, 0x10));
        call_ptr(c, 0x0032_5f48, 0x0022_6b7c)?;
        c.g[S1][0] = c.g[V0][0];

        // `new InputMap` (0x18 bytes, tagged str_InputMap), then `init(205)`.
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A1][0] = IM671_STR_INPUT_MAP;
        c.g[A0][0] = 0x18;
        call_ptr(c, 0x0031_7d70, 0x0022_6b98)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0xcd;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0032_0ca0, 0x0022_6ba8)?;

        // loadMapFile(map, data, size): the size is read back from the frame.
        c.g[A2][0] = Addr(frame.at(c, 0x10)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0032_0db0, 0x0022_6bb8)?;

        // Free the text when the loader returned one.
        c.g[A2][0] = 0x0048_0000;
        if c.g[S1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e98, 0x0022_6bc8)?;
            c.g[A2][0] = 0x0048_0000;
        }
        c.cycles += 1032;

        // compileMap(map, index, name) for every binding; a1 and a2 are left
        // as the last call set them.
        for (index, &(ret, name, _)) in IM671_BINDINGS.iter().enumerate() {
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = index as u64;
            c.g[A2][0] = sx(name);
            call_ptr(c, 0x0032_0e18, ret)?;
        }

        // purgeMapFile(map); the result is the map.
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0032_0f10, 0x0022_7bd4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `lui 0x480000`: the string and global pool that the generated code
/// reaches with `0x480000 - off`.
pub(crate) const BOOT_HI_DATA: u32 = 0x0048_0000;

/// `lui 0x4a0000`: the second string pool (`D_004A29xx`).
pub(crate) const BOOT_HI_STR2: u32 = 0x004a_0000;

/// `lui 0x440000`: the texture-name table at `+0x2100`.
pub(crate) const BOOT_HI_TEX: u32 = 0x0044_0000;

/// `lui 0x530000`: the BE option struct at `+0x5610` (copied by `Cfg_82D0()`).
pub(crate) const BOOT_HI_BEOPT: u32 = 0x0053_0000;

/// Offset of the BE option struct from `BOOT_HI_BEOPT` (the `Cfg_82D0()` source).
pub(crate) const BOOT_BEOPT_OFF: u32 = 0x5610;

/// SSXApp fields the load writes or reads.
pub(crate) const BOOT_SELF_BE: u32 = 0x78;

pub(crate) const BOOT_SELF_FONTS: u32 = 0x88;

pub(crate) const BOOT_SELF_ANIM_DB: u32 = 0x90;

pub(crate) const BOOT_SELF_ANIM_MAP: u32 = 0x94;

pub(crate) const BOOT_SELF_PADS: u32 = 0xa8;

pub(crate) const BOOT_SELF_PLAYERS: u32 = 0xb0;

pub(crate) const BOOT_SELF_INPUT_MAPS: u32 = 0xb8;

pub(crate) const BOOT_SELF_CMDLINE_TRACK: u32 = 0xcc;

pub(crate) const BOOT_SELF_BIG_TEX: u32 = 0xec;

pub(crate) const BOOT_SELF_BIG_RWRD: u32 = 0x118;

/// The SSXApp pointer as the register holds it (read after every call).
#[inline(always)]
pub(crate) fn boot_self(c: &Cpu) -> u32 {
    c.g[S4][0] as u32
}

/// `if (BXFILE_exists(name)) FILESYS_addbigsync(name, 0, 100, &self->field)`:
/// one of the four generated blocks that open the audio big files. `pre` is
/// the cycles charged at the block head, `name_lo` the string at
/// `0x480000 - name_lo`, `field` the self field the handle goes to.
pub(crate) fn boot_big_if_exists(
    c: &mut Cpu,
    pre: u64,
    name_lo: u32,
    field: u32,
    ret_exists: u32,
    ret_add: u32,
) -> Flow {
    c.cycles += pre;
    c.g[V0][0] = BOOT_HI_DATA as u64;
    c.g[S0][0] = sx(BOOT_HI_DATA.wrapping_sub(name_lo));
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0031_7f38, ret_exists)?; // BXFILE_exists
    let found = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S0][0];
    if found {
        c.cycles += 4;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x64;
        c.g[A3][0] = sx(boot_self(c).wrapping_add(field));
        call_ptr(c, 0x003d_ed50, ret_add)?; // FILESYS_addbigsync
    }
    Ok(())
}

/// The inline copy of the 0x288-byte BE option snapshot (`Cfg_82D0()` by
/// value) into the stack buffer at SP, as the generated code does it. `src`
/// is the source register; its address is advanced in 32-byte steps. Aligned
/// sources (`src & 7 == 0`) use `ld`/`sd` with the registers `al`; unaligned
/// ones use `ldl`/`ldr`/`sdl`/`sdr` with `ua`. The 8-byte tail is copied by
/// the next block (see [`boot_cfg_tail`]). `heads` are the two loop heads for
/// the event checks.
pub(crate) fn boot_cfg_copy(
    c: &mut Cpu,
    src: usize,
    ua: [usize; 4],
    al: [usize; 4],
    heads: (u32, u32),
) {
    c.g[V0][0] = c.g[src][0] & 0x7;
    let aligned = c.g[V0][0] == 0;
    c.g[V1][0] = c.g[SP][0];
    if !aligned {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[src][0] as u32).wrapping_add(0x280));
        loop {
            c.cycles += 19;
            let s = c.g[src][0] as u32;
            let d = c.g[V1][0] as u32;
            for k in 0..4 {
                let off = (8 * k) as u32;
                c.ldl(ua[k], s.wrapping_add(off + 7));
                c.ldr(ua[k], s.wrapping_add(off));
            }
            for k in 0..4 {
                let off = (8 * k) as u32;
                c.sdl(ua[k], d.wrapping_add(off + 7));
                c.sdr(ua[k], d.wrapping_add(off));
            }
            c.g[src][0] = sx(s.wrapping_add(0x20));
            let more = c.g[src][0] != c.g[V0][0];
            c.g[V1][0] = sx(d.wrapping_add(0x20));
            if more {
                if event_due(c) {
                    deliver_event_here(c, heads.0);
                }
                continue;
            }
            c.cycles += 2;
            return;
        }
    }
    c.cycles += 1;
    c.g[V0][0] = sx((c.g[src][0] as u32).wrapping_add(0x280));
    loop {
        c.cycles += 11;
        let s = c.g[src][0] as u32;
        let d = c.g[V1][0] as u32;
        for k in 0..4 {
            c.g[al[k]][0] = c.ld(s.wrapping_add((8 * k) as u32));
        }
        for k in 0..4 {
            c.sd(d.wrapping_add((8 * k) as u32), c.g[al[k]][0]);
        }
        c.g[src][0] = sx(s.wrapping_add(0x20));
        let more = c.g[src][0] != c.g[V0][0];
        c.g[V1][0] = sx(d.wrapping_add(0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, heads.1);
            }
            continue;
        }
        return;
    }
}

/// The 8-byte tail of a [`boot_cfg_copy`] (`ldl`/`ldr` from the end of the
/// source into `reg`, then `sdl`/`sdr` to SP+0x280). The caller charges the
/// block's cycles.
pub(crate) fn boot_cfg_tail(c: &mut Cpu, src: usize, reg: usize) {
    let s = c.g[src][0] as u32;
    let d = c.g[V1][0] as u32;
    c.ldl(reg, s.wrapping_add(7));
    c.ldr(reg, s);
    c.sdl(reg, d.wrapping_add(7));
    c.sdr(reg, d);
}

/// `cSSXApp_initload(SSXApp* self)` (0x2282d0, `SSXApp_BootLoad`): the game's
/// one-time load. Starts the net modules (`initOnline`), opens the big files
/// that exist, the scripts, the 11 character textures and `rwrdps2.big`,
/// creates the ScriptEngine, loads `input.map` and `input2.map` into the
/// input maps and gives both players a cInput on the first map, loads the
/// fonts, applies the display options and the locale, builds the animation
/// data, starts the audio with the saved volumes, inits the world triggers
/// and copies a non-empty command-line track.
///
/// `Cfg_82D0()` by value is the inline copy of the BE option struct, five
/// times: each `cSSXAudio_*` argument is read from the stack copy.
pub fn ssxapp_boot_load(c: &mut Cpu) {
    guest(c, ssxapp_boot_load_body);
}

pub(crate) fn ssxapp_boot_load_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x0022_82d0);
    c.cycles += 19;
    let frame = Frame::open(c, 0x330);
    frame.save(c, 20, 0x2e0);
    frame.save(c, 16, 0x320);
    frame.save(c, 17, 0x310);
    frame.save(c, 18, 0x300);
    frame.save(c, 19, 0x2f0);
    frame.save(c, 21, 0x2d0);
    frame.save(c, 22, 0x2c0);
    frame.save(c, 23, 0x2b0);
    frame.save(c, 30, 0x2a0);
    frame.save_ra(c, 0x290);

    // initOnline(): net modules.
    c.g[S4][0] = c.g[A0][0];
    Callee {
        addr: 0x0022_8ee0,
        f: ee::s05::f_00228ee0,
    }
    .call(c, 0x0022_8304)?;

    // The four audio big files (music, music2, audio, speech).
    boot_big_if_exists(c, 0, 0x57d8, 0xd8, 0x0022_8314, 0x0022_832c)?;
    boot_big_if_exists(c, 6, 0x57c0, 0xdc, 0x0022_833c, 0x0022_8354)?;
    boot_big_if_exists(c, 6, 0x57a8, 0xd4, 0x0022_8364, 0x0022_837c)?;
    boot_big_if_exists(c, 6, 0x5790, 0xe0, 0x0022_838c, 0x0022_83a4)?;

    // func_00256188(); scdat.big; D_004A29C8 (mdlps2.big).
    c.cycles += 21;
    c.g[RA][0] = sx(0x0022_83ac);
    c.g[S6][0] = sx(boot_self(c).wrapping_add(BOOT_SELF_BIG_RWRD));
    call_ptr(c, 0x0025_6188, 0x0022_83ac)?;
    c.g[A0][0] = BOOT_HI_DATA as u64;
    c.g[A1][0] = 0;
    c.g[A0][0] = sx(BOOT_HI_DATA.wrapping_sub(0x5778));
    c.g[A2][0] = 0x64;
    c.g[A3][0] = sx(boot_self(c).wrapping_add(0xe4));
    c.g[S3][0] = sx(boot_self(c).wrapping_add(BOOT_SELF_PLAYERS));
    call_ptr(c, 0x003d_ed50, 0x0022_83c8)?;
    c.g[S5][0] = sx(boot_self(c).wrapping_add(BOOT_SELF_PADS));
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x728));
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x64;
    c.g[A3][0] = sx(boot_self(c).wrapping_add(0xe8));
    call_ptr(c, 0x003d_ed50, 0x0022_83e0)?;

    // The 11 character textures (D_00442100[i] into fEC[i]).
    c.g[FP][0] = sx(boot_self(c).wrapping_add(BOOT_SELF_FONTS));
    c.g[V1][0] = BOOT_HI_TEX as u64;
    c.g[S7][0] = BOOT_HI_BEOPT as u64;
    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2100));
    c.g[S0][0] = sx(boot_self(c).wrapping_add(BOOT_SELF_BIG_TEX));
    c.g[S1][0] = 0xa;
    loop {
        c.cycles += 9;
        c.g[A0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x64;
        call_ptr(c, 0x003d_ed50, 0x0022_840c)?;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
        let more = (c.g[S1][0] as i64) >= 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0022_83f8);
            }
            continue;
        }
        break;
    }

    // rwrdps2.big (D_004A29CC into f118).
    c.cycles += 29;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x724));
    c.g[A1][0] = 0;
    c.g[A3][0] = c.g[S6][0];
    c.g[A2][0] = 0x64;
    call_ptr(c, 0x003d_ed50, 0x0022_8430)?;

    // ScriptEngine: alloc, construct, InitOnce.
    c.g[S2][0] = c.g[S5][0];
    c.g[A1][0] = sx(BOOT_HI_DATA.wrapping_sub(0x5730));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    c.g[A0][0] = 0xa50;
    c.g[S5][0] = 0x1;
    call_ptr(c, 0x0031_7d70, 0x0022_8450)?; // cMemMan_alloc
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0027_7f20, 0x0022_8458)?; // cSSXScriptEngine ctor
    c.g[A0][0] = c.g[V0][0];
    c.sw((c.g[GP][0] as u32).wrapping_sub(0x84c), c.g[V0][0]); // ScriptEngine = sm
    call_ptr(c, 0x0027_8210, 0x0022_8464)?; // cSSXScriptEngine_InitOnce

    // input.map and input2.map into the two input maps.
    c.g[A1][0] = BOOT_HI_DATA as u64;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = sx(BOOT_HI_DATA.wrapping_sub(0x5720));
    Callee {
        addr: 0x0022_6b60,
        f: ee::s05::f_00226b60,
    }
    .call(c, 0x0022_8474)?; // cSSXApp_loadInputMap
    c.g[A1][0] = BOOT_HI_DATA as u64;
    c.sw(boot_self(c).wrapping_add(BOOT_SELF_INPUT_MAPS), c.g[V0][0]);
    c.g[A1][0] = sx(BOOT_HI_DATA.wrapping_sub(0x5708));
    c.g[A0][0] = c.g[S4][0];
    Callee {
        addr: 0x0022_6b60,
        f: ee::s05::f_00226b60,
    }
    .call(c, 0x0022_8488)?;
    c.sw(
        boot_self(c).wrapping_add(BOOT_SELF_INPUT_MAPS + 4),
        c.g[V0][0],
    );

    // Two cInputs (setPad, setMap) on fB0[j], j = 0, 1.
    loop {
        c.cycles += 19;
        c.g[V0][0] = BOOT_HI_STR2 as u64;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x8;
        c.g[A1][0] = sx(BOOT_HI_STR2.wrapping_add(0x29d0));
        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(1));
        call_ptr(c, 0x0031_7d70, 0x0022_84ac)?; // cMemMan_alloc (str_Input)
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(c.g[S2][0] as u32);
        c.g[S1][0] = c.lw(boot_self(c).wrapping_add(BOOT_SELF_INPUT_MAPS));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0024_4400, 0x0022_84c0)?; // cInput_setPad
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0024_4408, 0x0022_84d0)?; // cInput_setMap
        c.sw(c.g[S3][0] as u32, c.g[S0][0]);
        let more = (c.g[S5][0] as i64) >= 0;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0022_8490);
            }
            continue;
        }
        break;
    }

    // Font interface: alloc 0x44, zero the four entries and the counters.
    c.cycles += 11;
    c.g[A1][0] = BOOT_HI_DATA as u64;
    c.g[A0][0] = 0x44;
    c.g[A1][0] = sx(BOOT_HI_DATA.wrapping_sub(0x56f0));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x0022_84f4)?; // cMemMan_alloc (cUIFontInterface)
    c.sw(c.g[V0][0] as u32, 0);
    c.g[A1][0] = u64::MAX;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
    c.g[A0][0] = 0x3;
    loop {
        c.cycles += 7;
        c.sw(c.g[V1][0] as u32, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        c.sw((c.g[V1][0] as u32).wrapping_add(4), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(8), 0);
        let more = c.g[A0][0] != c.g[A1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0022_8508);
            }
            continue;
        }
        break;
    }

    // Font interface fields, then the two font entries.
    c.cycles += 78;
    c.sw(c.g[FP][0] as u32, c.g[V0][0]); // self->fonts = new interface
    c.g[A0][0] = sx(BOOT_HI_STR2.wrapping_add(0x29d8));
    c.sb((c.g[V0][0] as u32).wrapping_add(0x34), 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x38), 0);
    c.g[S0][0] = BOOT_HI_DATA as u64;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), 0);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x56d8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x40), 0);
    c.g[S1][0] = BOOT_HI_DATA as u64;
    call_ptr(c, 0x0031_7670, 0x0022_8550)?; // name the interface (str_cUIFontInterface)
    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x56c0));
    c.g[A0][0] = c.lw(boot_self(c).wrapping_add(BOOT_SELF_FONTS));
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0039_84b0, 0x0022_8568)?; // cUIFontInterface_addFontEntry(fefont)
    c.g[S2][0] = sx((c.g[S7][0] as u32).wrapping_add(BOOT_BEOPT_OFF));
    c.g[A0][0] = sx(BOOT_HI_STR2.wrapping_add(0x29e0));
    call_ptr(c, 0x0031_7670, 0x0022_8578)?;
    c.g[A0][0] = c.lw(boot_self(c).wrapping_add(BOOT_SELF_FONTS));
    c.g[A1][0] = 1;
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[V0][0];
    call_ptr(c, 0x0039_84b0, 0x0022_858c)?; // cUIFontInterface_addFontEntry(hudfont)
    c.g[A0][0] = c.lw(boot_self(c).wrapping_add(BOOT_SELF_FONTS));
    call_ptr(c, 0x0039_8568, 0x0022_8594)?; // cUIFontInterface_loadFonts
    call_ptr(c, 0x0014_dc80, 0x0022_859c)?; // cBE_getBE
    c.sw(boot_self(c).wrapping_add(BOOT_SELF_BE), c.g[V0][0]);

    // BE options, the locale, the animation data.
    c.g[A0][0] = c.g[S4][0];
    Callee {
        addr: 0x0022_8c08,
        f: ee::s05::f_00228c08,
    }
    .call(c, 0x0022_85a8)?; // SSXApp_ApplyDisplayOptions
    c.g[A0][0] = c.g[S4][0];
    Callee {
        addr: 0x0022_8a78,
        f: ee::s05::f_00228a78,
    }
    .call(c, 0x0022_85b0)?; // SSXApp_LoadLocale
    call_ptr(c, 0x0031_1870, 0x0022_85b8)?; // cRiderAnimBase_initAIAnimEventMap
    c.g[A1][0] = BOOT_HI_DATA as u64;
    c.g[A2][0] = 0;
    c.g[A1][0] = sx(BOOT_HI_DATA.wrapping_sub(0x56a8));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x1848;
    call_ptr(c, 0x0031_7d70, 0x0022_85d0)?; // cMemMan_alloc (GameAnimMap)
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0031_4d98, 0x0022_85d8)?; // cGameAnimMap ctor
    c.g[A0][0] = c.g[V0][0];
    c.sw(boot_self(c).wrapping_add(BOOT_SELF_ANIM_MAP), c.g[V0][0]);
    call_ptr(c, 0x0031_4dd8, 0x0022_85e4)?; // cGameAnimMap_initMap
    c.g[A1][0] = BOOT_HI_DATA as u64;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    c.g[A1][0] = sx(BOOT_HI_DATA.wrapping_sub(0x5698));
    c.g[A0][0] = 0x3fc;
    call_ptr(c, 0x0031_7d70, 0x0022_85fc)?; // cMemMan_alloc (AnimDataBase)
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0031_1048, 0x0022_8604)?; // cAnimDataBase ctor
    c.sw(boot_self(c).wrapping_add(BOOT_SELF_ANIM_DB), c.g[V0][0]);
    call_ptr(c, 0x0014_dc80, 0x0022_860c)?; // cBE_getBE
    c.g[A1][0] = 4;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dd58, 0x0022_8618)?; // cBE_getInterface(be, 4)

    // Audio: SSXAUDIO_Init with the saved output mode, then the volumes.
    call_ptr(c, 0x0028_b180, 0x0022_8620)?; // cAudioInterface_bxAudio
    c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(BOOT_BEOPT_OFF));
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 12);
    c.g[A1][0] = c.g[A1][0] & 0x3;
    call_ptr(c, 0x0028_9d08, 0x0022_8634)?; // cSSXAudio_BEAudioModeToAudioMode
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0028_4bb0, 0x0022_863c)?; // SSXAUDIO_Init
    call_ptr(c, 0x0031_a6b8, 0x0022_8644)?; // func_0031A6B8
    call_ptr(c, 0x0028_b180, 0x0022_864c)?; // cAudioInterface_bxAudio
    c.g[A0][0] = c.g[V0][0];

    // Cfg_82D0().sfxVol: copy A, its tail, then func_00287410.
    boot_cfg_copy(
        c,
        18,
        [5, 6, 7, 8],
        [9, 5, 6, 7],
        (0x0022_8660, 0x0022_86b8),
    );
    c.cycles += 14;
    boot_cfg_tail(c, 18, 8);
    c.g[A1][0] = c.lw(sp(c)) & 0xf;
    call_ptr(c, 0x0028_7410, 0x0022_8700)?; // func_00287410 (sfxVol)

    // Cfg_82D0().speechVol: copy B, its tail, then cSSXAudio_SetSFXVolume.
    call_ptr(c, 0x0028_b180, 0x0022_8708)?; // cAudioInterface_bxAudio
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(BOOT_HI_BEOPT.wrapping_add(BOOT_BEOPT_OFF));
    boot_cfg_copy(c, 5, [6, 7, 8, 9], [6, 7, 8, 9], (0x0022_8720, 0x0022_8778));
    c.cycles += 15;
    boot_cfg_tail(c, 5, 2);
    c.g[A1][0] = sx((c.lw(sp(c)) as u32) >> 8);
    c.g[A1][0] = c.g[A1][0] & 0xf;
    call_ptr(c, 0x0028_7488, 0x0022_87c4)?; // cSSXAudio_SetSFXVolume (speechVol)

    // Cfg_82D0().musicVol: copy C, its tail, then func_00287520.
    call_ptr(c, 0x0028_b180, 0x0022_87cc)?; // cAudioInterface_bxAudio
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(BOOT_HI_BEOPT.wrapping_add(BOOT_BEOPT_OFF));
    boot_cfg_copy(c, 5, [9, 6, 7, 8], [9, 6, 7, 8], (0x0022_87e4, 0x0022_883c));
    c.cycles += 14;
    boot_cfg_tail(c, 5, 9);
    c.g[A1][0] = c.lbu(sp(c)) >> 4;
    call_ptr(c, 0x0028_7520, 0x0022_8884)?; // func_00287520 (musicVol)

    // Cfg_82D0().iUseDJSpeech: copy D, its tail, then cSSXAudio_SetDJSpeech.
    call_ptr(c, 0x0028_b180, 0x0022_888c)?; // cAudioInterface_bxAudio
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(BOOT_HI_BEOPT.wrapping_add(BOOT_BEOPT_OFF));
    boot_cfg_copy(c, 5, [8, 9, 6, 7], [8, 9, 6, 7], (0x0022_88a4, 0x0022_88fc));
    c.cycles += 14;
    boot_cfg_tail(c, 5, 8);
    c.g[A1][0] = sx(((c.lw(sp(c)) as u32) as i32 >> 17) as u32);
    c.g[A1][0] = c.g[A1][0] & 0x1;
    call_ptr(c, 0x0028_7558, 0x0022_8948)?; // cSSXAudio_SetDJSpeech

    // Cfg_82D0().iUseArcadeSFX: copy E, its tail, then cSSXAudio_SetArcadeSFX.
    c.g[S2][0] = sx(BOOT_HI_BEOPT.wrapping_add(BOOT_BEOPT_OFF));
    call_ptr(c, 0x0028_b180, 0x0022_8950)?; // cAudioInterface_bxAudio
    c.g[A0][0] = c.g[V0][0];
    boot_cfg_copy(
        c,
        18,
        [7, 8, 9, 5],
        [6, 7, 8, 9],
        (0x0022_8964, 0x0022_89bc),
    );
    c.cycles += 15;
    boot_cfg_tail(c, 18, 2);
    c.g[A1][0] = c.lh(sp(c).wrapping_add(0x2));
    c.g[A1][0] = c.g[A1][0] & 0x1;
    call_ptr(c, 0x0028_75d0, 0x0022_8a04)?; // cSSXAudio_SetArcadeSFX

    // WORLDTRIGGERMANAGER_Init, then the command-line track.
    call_ptr(c, 0x002b_4b08, 0x0022_8a0c)?; // WORLDTRIGGERMANAGER_Init
    c.g[A1][0] = BOOT_HI_STR2 as u64;
    c.g[A0][0] = c.lw(boot_self(c).wrapping_add(BOOT_SELF_CMDLINE_TRACK));
    c.g[A1][0] = sx(BOOT_HI_STR2.wrapping_add(0x29e8));
    call_ptr(c, 0x0041_65a8, 0x0022_8a1c)?; // strcmp(fCC, D_004A29E8)
    let same = c.g[V0][0] == 0;
    frame.restore(c, 16, 0x320);
    if !same {
        c.cycles += 9;
        call_ptr(c, 0x0014_dc80, 0x0022_8a2c)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0022_8a38)?; // cBE_getInterface(be, 0)
        c.g[A0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1bc8));
        c.g[A1][0] = c.lw(boot_self(c).wrapping_add(BOOT_SELF_CMDLINE_TRACK));
        call_ptr(c, 0x0031_84e8, 0x0022_8a44)?; // cBXString_operator=
        frame.restore(c, 16, 0x320);
    }

    // Epilogue.
    c.cycles += 11;
    frame.restore(c, 17, 0x310);
    frame.restore(c, 18, 0x300);
    frame.restore(c, 19, 0x2f0);
    frame.restore(c, 20, 0x2e0);
    frame.restore(c, 21, 0x2d0);
    frame.restore(c, 22, 0x2c0);
    frame.restore(c, 23, 0x2b0);
    frame.restore(c, 30, 0x2a0);
    frame.restore_ra(c, 0x290);
    frame.close(c);
    Ok(())
}

/// `void SSXApp_Init(cSSXApp* self, int argc, char** argv)` (0x226900): the
/// application's start-up: registers the app at `gp-0x848`, the state config
/// (`+0x9c`), the pad queue (`gp-0x850`, a 0x2efc-byte object), the graphics
/// manager (`gp-0x854`) with its vtable setup, the light and bezier managers,
/// the byte at `gp-0x844` and the two 0x2a4-byte pad records at `+0xa8`, the
/// command line, the graphics mode (slot 0x160, 0x100), the boot shape
/// (`SHPS_Find` + slot 0x170, into `+0xa0`), the 0x1c-byte white tint (`+0xa4`),
/// the pre-start screen (`+0xc0`) and the FE async manager (`+0x11c`).
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut,
    clippy::all
)]
pub fn ssx_app_init_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.g[A2][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S4][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa8));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = 0x1;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[S5][0] = 0x480000;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x848), c.g[S2][0]);
        call_ptr(c, 0x002c7440, 0x00226940)?;
        c.g[A1][0] = 0x480000;
        c.g[A2][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x65c0));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0x2efc;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
        call_ptr(c, 0x00317d70, 0x0022695c)?;
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        call_ptr(c, 0x00326a28, 0x0022696c)?;
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x850), c.g[V0][0]);
        call_ptr(c, 0x00375a08, 0x00226978)?;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10d8));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x854), c.g[V0][0]);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00226990)?;
        call_ptr(c, 0x0038dbd0, 0x00226998)?;
        call_ptr(c, 0x0038aef8, 0x002269a0)?;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = 0x1;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x65b0));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x002269b8)?;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x844), c.g[V0][0]);
        'l_2269c0: loop {
            c.cycles += 10;
            c.g[A0][0] = 0x2a4;
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x65a0));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317d70, 0x002269d4)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.sw(c.g[V0][0] as u32, 0u64);
            let t = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002269c0);
                }
                continue 'l_2269c0;
            } else {
                c.cycles += 94;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x84), 0u64);
                c.g[A2][0] = c.g[S3][0];
                c.sw((c.g[S2][0] as u32).wrapping_add(0x80), 0u64);
                c.g[A0][0] = c.g[S2][0];
                c.sw((c.g[S2][0] as u32).wrapping_add(0x7c), 0u64);
                c.g[A1][0] = c.g[S4][0];
                call_ptr(c, 0x00227bf0, 0x00226a04)?;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A1][0] = 0x100;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x160));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x164));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00226a20)?;
                c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A0][0] = 0x480000;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x54c0));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10d8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28b0));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x170));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x003b39b8, 0x00226a44)?;
                c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x174));
                c.g[A2][0] = 0x480000;
                c.g[T0][0] = 0x1;
                c.g[T1][0] = 0xffffffffffffffff;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6590));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[T2][0] as u32;
                c.g[A3][0] = 0u64;
                call_ptr(c, t, 0x00226a68)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                call_ptr(c, 0x00369ff8, 0x00226a78)?;
                c.g[A1][0] = 0x480000;
                c.g[A0][0] = 0x1c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6580));
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x00226a90)?;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A1][0] = 0x480000;
                c.sw(c.g[V0][0] as u32, 0u64);
                c.g[A2][0] = 0x100;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.g[A3][0] = 0u64;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6570));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x1f));
                c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x13));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x1b));
                c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = 0xc;
                call_ptr(c, 0x00317d70, 0x00226af0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00232328, 0x00226af8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00316ef8, 0x00226b04)?;
                call_ptr(c, 0x00237cf8, 0x00226b0c)?;
                c.g[A1][0] = 0x480000;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6560));
                c.g[A3][0] = 0u64;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
                c.g[A0][0] = 0x590;
                call_ptr(c, 0x00317d70, 0x00226b28)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x001a3590, 0x00226b30)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x11c), c.g[V0][0]);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x001a3648, 0x00226b3c)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.pc = t;
                return Ok(());
            }
        }
        Ok(())
    })
}

/// `GameModule_Render` (`cGame::render`, 0x0022b008): draws one game frame (the draw path: per view it
/// sets the camera and render state, gathers the visible world and calls the model, fog, flare and
/// bezier-patch passes; then the HUD and menu overlay and the frame-counter tick): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code)]
pub fn game_module_render(c: &mut Cpu) {
    // no resume points: the generated body ignores `c.pc` and always starts at the
    // entry, so every pc runs the body below (the guard is not needed).
    guest(c, |c| {
        'b_22c02c: {
            'b_22b078: {
                c.cycles += 26;
                c.g[T4][0] = 0x83e0;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(c.g[T4][0] as u32));
                c.g[T5][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[SP][0] as u32));
                c.sq(18, (c.g[T5][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[T5][0] as u32).wrapping_sub(0x30));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[T5][0] as u32).wrapping_sub(0x40));
                c.sq(19, (c.g[T5][0] as u32).wrapping_sub(0x60));
                c.sq(20, (c.g[T5][0] as u32).wrapping_sub(0x70));
                c.sq(21, (c.g[T5][0] as u32).wrapping_sub(0x80));
                c.sq(22, (c.g[T5][0] as u32).wrapping_sub(0x90));
                c.sq(23, (c.g[T5][0] as u32).wrapping_sub(0xa0));
                c.sq(30, (c.g[T5][0] as u32).wrapping_sub(0xb0));
                c.sd((c.g[T5][0] as u32).wrapping_sub(0xc0), c.g[RA][0]);
                c.swc1(23, (c.g[T5][0] as u32).wrapping_sub(0x8));
                c.swc1(22, (c.g[T5][0] as u32).wrapping_sub(0x10));
                c.swc1(21, (c.g[T5][0] as u32).wrapping_sub(0x18));
                c.swc1(20, (c.g[T5][0] as u32).wrapping_sub(0x20));
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0022b068);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0022b068)?;
                let t = c.g[V0][0] != 0u64;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                if t {
                    break 'b_22b078;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_22c02c;
                    } else {
                        break 'b_22b078;
                    }
                }
            }
            'b_22b0d8: {
                c.cycles += 16;
                c.g[A1][0] = 0xffffffffffffffff;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x3a0));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3a4));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0022b090);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0022b090)?;
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x98));
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
                c.g[RA][0] = sx(0x0022b0ac);
                c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                Callee {
                    addr: 0x0022e8b8,
                    f: ee::s05::GameViewMan_PrefetchViews,
                }
                .call(c, 0x0022b0ac)?;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x9c));
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = 0u64;
                if t {
                    break 'b_22b0d8;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0022b0c0);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
                    Callee {
                        addr: 0x0022e920,
                        f: ee::s05::GameViewMan_AllViewsReady,
                    }
                    .call(c, 0x0022b0c0)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        break 'b_22b0d8;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = 0x1;
                        c.g[A1][0] = 0u64;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
                        break 'b_22b0d8;
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0022b0e4);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, t, 0x0022b0e4)?;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_22c02c;
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0022b0f4);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00231d60, 0x0022b0f4)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_22c02c;
                } else {
                    'b_22b11c: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x504));
                        let t = c.g[V1][0] == 0u64;

                        if t {
                            break 'b_22b11c;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x0022b114);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x0022c830,
                                f: ee::s05::GameModule_RenderUI,
                            }
                            .call(c, 0x0022b114)?;
                            let t = true;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_22c02c;
                            } else {
                                break 'b_22b11c;
                            }
                        }
                    }
                    'b_22bcbc: {
                        c.cycles += 25;
                        c.g[RA][0] = sx(0x0022b124);
                        c.g[S3][0] = 0u64;
                        call_ptr(c, 0x0036a1e8, 0x0022b124)?;
                        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xf50));
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x44));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw((c.g[A2][0] as u32).wrapping_add(0x10a8), c.g[A1][0]);
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        c.sw((c.g[A2][0] as u32).wrapping_add(0x10ac), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x6490), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.sw((c.g[A2][0] as u32).wrapping_add(0x270), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x8310;
                        if t {
                            break 'b_22bcbc;
                        } else {
                            c.cycles += 24;
                            c.g[V1][0] = 0xfffffffffe7f0000;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[SP][0] as u32));
                            c.g[A2][0] = c.g[V1][0] | 0xffff;
                            c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                            c.g[A1][0] = 0xfffffffffff00000;
                            c.g[A0][0] = 0x8314;
                            c.g[A3][0] = c.g[A1][0] | 0xfff;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[SP][0] as u32));
                            c.g[V0][0] = 0x8318;
                            c.sw(c.g[A0][0] as u32, c.g[A3][0]);
                            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[SP][0] as u32));
                            c.g[FP][0] = 0xffffffffffcf0000;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 22);
                            c.g[FP][0] = c.g[FP][0] | 0xffff;
                            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                            c.g[S1][0] = c.g[SP][0];
                            c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                            c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                            c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                            'l_22b1e0: loop {
                                'b_22b584: {
                                    c.cycles += 232;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10d8));
                                    c.sw((c.g[A0][0] as u32).wrapping_add(0x274), c.g[S3][0]);
                                    c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x210));
                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x214));
                                    let t = c.g[A1][0] as u32;
                                    c.g[RA][0] = sx(0x0022b1f8);
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    call_ptr(c, t, 0x0022b1f8)?;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
                                    c.g[A1][0] = sx((c.g[S3][0] as u32) << 4);
                                    c.g[T1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x18));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S3][0]);
                                    c.g[A2][0] = 0x1;
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
                                    c.g[A3][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x10d8));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                    c.g[A1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0xc0));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[T0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc4));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] =
                                        sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    let t = c.g[T0][0] as u32;
                                    c.g[RA][0] = sx(0x0022b23c);
                                    c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                    call_ptr(c, t, 0x0022b23c)?;
                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x110));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x114));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0022b258);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0022b258)?;
                                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.mov_s(13, 22);
                                    c.lwc1(12, c.g[S0][0] as u32);
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                                    c.lwc1(14, (c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0022b27c);
                                    c.g[A0][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0022b27c)?;
                                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.g[A0][0] = 0x8310;
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[SP][0] as u32));
                                    c.g[A2][0] = 0x800000;
                                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                    c.g[A3][0] = 0x300000;
                                    c.g[A0][0] = c.lw(c.g[A0][0] as u32);
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                                    c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                                    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                                    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                    c.g[A0][0] = 0x8314;
                                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[SP][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[A0][0] as u32);
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                                    c.g[V0][0] = c.g[V0][0] | 0xd000;
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0022b2f4);
                                    c.g[A0][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0022b2f4)?;
                                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.lwc1(12, c.g[S0][0] as u32);
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                                    c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                    c.lwc1(14, (c.g[S0][0] as u32).wrapping_add(0x8));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0022b318);
                                    c.g[A0][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0022b318)?;
                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x3a0));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3a4));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0022b334);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0022b334)?;
                                    c.g[A2][0] = 0x8318;
                                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.g[A2][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[SP][0] as u32));
                                    c.g[A3][0] = 0xfffffffffffffc1f;
                                    c.g[A0][0] = c.lw(c.g[A2][0] as u32);
                                    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                    c.g[T0][0] = 0xfffffffffffff07f;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                                    c.g[V0][0] = c.g[V0][0] | 0x80;
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
                                    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[A1][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xf50));
                                    c.sh((c.g[A2][0] as u32).wrapping_add(0x12), c.g[V0][0]);
                                    c.g[A2][0] = 0x800000;
                                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                                    c.g[V0][0] = c.g[V0][0] | 0x380;
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
                                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                    c.g[V0][0] = c.g[V0][0] | 0x30;
                                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                    c.g[V1][0] = 0x8310;
                                    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[SP][0] as u32));
                                    c.g[V1][0] = c.lw(c.g[V1][0] as u32);
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                    c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                    c.g[A2][0] = 0x8314;
                                    c.g[A1][0] = 0x300000;
                                    c.g[A2][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[SP][0] as u32));
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                                    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                    c.g[V1][0] = 0xfffffffffffffc1f;
                                    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
                                    c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                                    c.g[V0][0] = c.g[V0][0] | 0xd000;
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                    c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                    c.g[V0][0] = c.g[V0][0] | 0x80;
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
                                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x158));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x15c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x0022b428);
                                    c.g[A1][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    call_ptr(c, t, 0x0022b428)?;
                                    c.g[A1][0] = 0x8318;
                                    c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[A1][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(c.g[SP][0] as u32));
                                    c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                                    c.g[A1][0] = c.lw(c.g[A1][0] as u32);
                                    c.lqc2(1, c.g[A1][0] as u32);
                                    c.lqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                                    c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
                                    c.lqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
                                    c.sqc2(1, c.g[A0][0] as u32);
                                    c.sqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                                    c.sqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                                    c.sqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
                                    c.g[RA][0] = sx(0x0022b468);
                                    call_ptr(c, 0x0031b6c8, 0x0022b468)?;
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                    c.lq(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                                    c.lq(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                    c.sq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                    c.sq(5, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.mfc1(3, 20);
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.qmtc2(3, 3);
                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.sq(2, c.g[SP][0] as u32);
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.mfc1(3, 20);
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.qmtc2(3, 3);
                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lwc1(0, c.g[SP][0] as u32);
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.c_lt_s(1, 0);
                                    let t = !c.fcc();
                                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
                                    if t {
                                        break 'b_22b584;
                                    } else {
                                        c.cycles += 1;
                                        c.swc1(1, c.g[SP][0] as u32);
                                        break 'b_22b584;
                                    }
                                }
                                'b_22b59c: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        break 'b_22b59c;
                                    } else {
                                        break 'b_22b59c;
                                    }
                                }
                                'b_22b5b4: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        break 'b_22b5b4;
                                    } else {
                                        break 'b_22b5b4;
                                    }
                                }
                                'b_22b5cc: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        break 'b_22b5cc;
                                    } else {
                                        break 'b_22b5cc;
                                    }
                                }
                                'b_22b5e4: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                                        break 'b_22b5e4;
                                    } else {
                                        break 'b_22b5e4;
                                    }
                                }
                                'b_22b5fc: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                                        break 'b_22b5fc;
                                    } else {
                                        break 'b_22b5fc;
                                    }
                                }
                                'b_22b680: {
                                    c.cycles += 32;
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.mfc1(3, 20);
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.qmtc2(3, 3);
                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lwc1(0, c.g[SP][0] as u32);
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.c_lt_s(1, 0);
                                    let t = !c.fcc();
                                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
                                    if t {
                                        break 'b_22b680;
                                    } else {
                                        c.cycles += 1;
                                        c.swc1(1, c.g[SP][0] as u32);
                                        break 'b_22b680;
                                    }
                                }
                                'b_22b698: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        break 'b_22b698;
                                    } else {
                                        break 'b_22b698;
                                    }
                                }
                                'b_22b6b0: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        break 'b_22b6b0;
                                    } else {
                                        break 'b_22b6b0;
                                    }
                                }
                                'b_22b6c8: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        break 'b_22b6c8;
                                    } else {
                                        break 'b_22b6c8;
                                    }
                                }
                                'b_22b6e0: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
                                        break 'b_22b6e0;
                                    } else {
                                        break 'b_22b6e0;
                                    }
                                }
                                'b_22b6f8: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
                                        break 'b_22b6f8;
                                    } else {
                                        break 'b_22b6f8;
                                    }
                                }
                                'b_22b77c: {
                                    c.cycles += 33;
                                    c.lqc2(3, c.g[S4][0] as u32);
                                    c.lqc2(4, c.g[S5][0] as u32);
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(4, c.g[S7][0] as u32);
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(4, c.g[S6][0] as u32);
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.mfc1(3, 20);
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.qmtc2(3, 3);
                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.lwc1(0, c.g[S1][0] as u32);
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, c.g[S1][0] as u32);
                                        break 'b_22b77c;
                                    } else {
                                        break 'b_22b77c;
                                    }
                                }
                                'b_22b794: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
                                        break 'b_22b794;
                                    } else {
                                        break 'b_22b794;
                                    }
                                }
                                'b_22b7ac: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_22b7ac;
                                    } else {
                                        break 'b_22b7ac;
                                    }
                                }
                                'b_22b7c4: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x10));
                                        break 'b_22b7c4;
                                    } else {
                                        break 'b_22b7c4;
                                    }
                                }
                                'b_22b7dc: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
                                        break 'b_22b7dc;
                                    } else {
                                        break 'b_22b7dc;
                                    }
                                }
                                'b_22b7f4: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
                                        break 'b_22b7f4;
                                    } else {
                                        break 'b_22b7f4;
                                    }
                                }
                                'b_22b878: {
                                    c.cycles += 33;
                                    c.lqc2(3, c.g[S4][0] as u32);
                                    c.lqc2(4, c.g[S5][0] as u32);
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(4, c.g[S7][0] as u32);
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(4, c.g[S6][0] as u32);
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.mfc1(3, 21);
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.qmtc2(3, 3);
                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.lwc1(0, c.g[S1][0] as u32);
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, c.g[S1][0] as u32);
                                        break 'b_22b878;
                                    } else {
                                        break 'b_22b878;
                                    }
                                }
                                'b_22b890: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
                                        break 'b_22b890;
                                    } else {
                                        break 'b_22b890;
                                    }
                                }
                                'b_22b8a8: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_22b8a8;
                                    } else {
                                        break 'b_22b8a8;
                                    }
                                }
                                'b_22b8c0: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x10));
                                        break 'b_22b8c0;
                                    } else {
                                        break 'b_22b8c0;
                                    }
                                }
                                'b_22b8d8: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
                                        break 'b_22b8d8;
                                    } else {
                                        break 'b_22b8d8;
                                    }
                                }
                                'b_22b8f0: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
                                        break 'b_22b8f0;
                                    } else {
                                        break 'b_22b8f0;
                                    }
                                }
                                'b_22b974: {
                                    c.cycles += 33;
                                    c.lqc2(3, c.g[S4][0] as u32);
                                    c.lqc2(4, c.g[S5][0] as u32);
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(4, c.g[S7][0] as u32);
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(4, c.g[S6][0] as u32);
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.mfc1(3, 21);
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.qmtc2(3, 3);
                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.lwc1(0, c.g[S1][0] as u32);
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, c.g[S1][0] as u32);
                                        break 'b_22b974;
                                    } else {
                                        break 'b_22b974;
                                    }
                                }
                                'b_22b98c: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
                                        break 'b_22b98c;
                                    } else {
                                        break 'b_22b98c;
                                    }
                                }
                                'b_22b9a4: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_22b9a4;
                                    } else {
                                        break 'b_22b9a4;
                                    }
                                }
                                'b_22b9bc: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x10));
                                        break 'b_22b9bc;
                                    } else {
                                        break 'b_22b9bc;
                                    }
                                }
                                'b_22b9d4: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
                                        break 'b_22b9d4;
                                    } else {
                                        break 'b_22b9d4;
                                    }
                                }
                                'b_22b9ec: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
                                        break 'b_22b9ec;
                                    } else {
                                        break 'b_22b9ec;
                                    }
                                }
                                'b_22ba70: {
                                    c.cycles += 33;
                                    c.lqc2(3, c.g[S4][0] as u32);
                                    c.lqc2(4, c.g[S5][0] as u32);
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(4, c.g[S7][0] as u32);
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(4, c.g[S6][0] as u32);
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.mfc1(3, 21);
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.qmtc2(3, 3);
                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.lwc1(0, c.g[S1][0] as u32);
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, c.g[S1][0] as u32);
                                        break 'b_22ba70;
                                    } else {
                                        break 'b_22ba70;
                                    }
                                }
                                'b_22ba88: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
                                        break 'b_22ba88;
                                    } else {
                                        break 'b_22ba88;
                                    }
                                }
                                'b_22baa0: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_22baa0;
                                    } else {
                                        break 'b_22baa0;
                                    }
                                }
                                'b_22bab8: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x10));
                                        break 'b_22bab8;
                                    } else {
                                        break 'b_22bab8;
                                    }
                                }
                                'b_22bad0: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
                                        break 'b_22bad0;
                                    } else {
                                        break 'b_22bad0;
                                    }
                                }
                                'b_22bae8: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
                                        break 'b_22bae8;
                                    } else {
                                        break 'b_22bae8;
                                    }
                                }
                                'b_22bb6c: {
                                    c.cycles += 33;
                                    c.lqc2(3, c.g[S4][0] as u32);
                                    c.lqc2(4, c.g[S5][0] as u32);
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                                    c.lqc2(4, c.g[S7][0] as u32);
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
                                    c.lqc2(4, c.g[S6][0] as u32);
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.mfc1(3, 21);
                                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                    c.qmtc2(3, 3);
                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.lwc1(0, c.g[S1][0] as u32);
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, c.g[S1][0] as u32);
                                        break 'b_22bb6c;
                                    } else {
                                        break 'b_22bb6c;
                                    }
                                }
                                'b_22bb84: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
                                        break 'b_22bb84;
                                    } else {
                                        break 'b_22bb84;
                                    }
                                }
                                'b_22bb9c: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.c_lt_s(1, 0);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_22bb9c;
                                    } else {
                                        break 'b_22bb9c;
                                    }
                                }
                                'b_22bbb4: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x10));
                                        break 'b_22bbb4;
                                    } else {
                                        break 'b_22bbb4;
                                    }
                                }
                                'b_22bbcc: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
                                        break 'b_22bbcc;
                                    } else {
                                        break 'b_22bbcc;
                                    }
                                }
                                'b_22bbe4: {
                                    c.cycles += 6;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
                                    c.c_lt_s(0, 1);
                                    if c.fcc() {
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
                                        break 'b_22bbe4;
                                    } else {
                                        break 'b_22bbe4;
                                    }
                                }
                                c.cycles += 54;
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                                c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8c));
                                c.g[A3][0] = c.g[SP][0];
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                                c.g[RA][0] = sx(0x0022bc00);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x0022add8,
                                    f: ee::s05::Nash_GatherVisible,
                                }
                                .call(c, 0x0022bc00)?;
                                c.g[RA][0] = sx(0x0022bc08);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x00230640, 0x0022bc08)?;
                                c.g[RA][0] = sx(0x0022bc10);
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                Callee {
                                    addr: 0x00229ba8,
                                    f: ee::s05::CrowdMan2d_RenderSprites,
                                }
                                .call(c, 0x0022bc10)?;
                                c.g[RA][0] = sx(0x0022bc18);
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3c));
                                call_ptr(c, 0x00344138, 0x0022bc18)?;
                                c.g[RA][0] = sx(0x0022bc20);
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                call_ptr(c, 0x00357c50, 0x0022bc20)?;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.g[A2][0] = 0xfffffffffffffc1f;
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                                c.g[V0][0] = c.g[V0][0] | 0x80;
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xcc));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x0022bc54);
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0022bc54)?;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x0022bc60);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x0022c078,
                                    f: ee::s05::GameModule_RenderModels,
                                }
                                .call(c, 0x0022bc60)?;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x0022bc6c);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x0022c708,
                                    f: ee::s05::GameModule_RenderFogParticles,
                                }
                                .call(c, 0x0022bc6c)?;
                                c.g[A2][0] = c.g[S3][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0022bc80);
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                Callee {
                                    addr: 0x0022c790,
                                    f: ee::s05::GameModule_RenderLensFlares,
                                }
                                .call(c, 0x0022bc80)?;
                                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.g[A3][0] = 0xfffffffffffffc1f;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                                c.g[V0][0] = c.g[V0][0] | 0x40;
                                c.g[RA][0] = sx(0x0022bca8);
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                Callee {
                                    addr: 0x0022c620,
                                    f: ee::s05::GameModule_RenderBezierPatches,
                                }
                                .call(c, 0x0022bca8)?;
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0022b1e0);
                                    }
                                    continue 'l_22b1e0;
                                } else {
                                    break 'b_22bcbc;
                                }
                            }
                        }
                    }
                    'b_22bd14: {
                        'b_22bcf8: {
                            c.cycles += 11;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10d8));
                            c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbc));
                            c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0022bce0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            call_ptr(c, t, 0x0022bce0)?;
                            let t = (c.g[V0][0] as i64) < 0;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                            if t {
                                break 'b_22bcf8;
                            } else {
                                c.cycles += 4;
                                c.mtc1(2, 20);
                                c.cvt_s_w(20, 20);
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                                if t {
                                    break 'b_22bd14;
                                } else {
                                    break 'b_22bcf8;
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                        c.mtc1(2, 20);
                        c.cvt_s_w(20, 20);
                        c.add_s(20, 20, 20);
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                        break 'b_22bd14;
                    }
                    'b_22bd50: {
                        'b_22bd3c: {
                            c.cycles += 6;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0022bd24);
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0022bd24)?;
                            let t = (c.g[V0][0] as i64) < 0;
                            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                            if t {
                                break 'b_22bd3c;
                            } else {
                                c.cycles += 4;
                                c.mtc1(2, 15);
                                c.cvt_s_w(15, 15);
                                let t = true;

                                if t {
                                    break 'b_22bd50;
                                } else {
                                    break 'b_22bd3c;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                        c.mtc1(2, 15);
                        c.cvt_s_w(15, 15);
                        c.add_s(15, 15, 15);
                        break 'b_22bd50;
                    }
                    'b_22bef0: {
                        c.cycles += 81;
                        c.mtc1(0, 12);
                        c.g[A0][0] = c.g[S0][0];
                        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                        c.mov_s(14, 20);
                        c.mov_s(21, 12);
                        c.mov_s(13, 12);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 23);
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0022bd78);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, t, 0x0022bd78)?;
                        c.g[S0][0] = 0xfffffffffffffc1f;
                        c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[A1][0] = 0u64;
                        c.g[AT][0] = 0x44200000;
                        c.mtc1(1, 14);
                        c.g[A2][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
                        c.mov_s(16, 21);
                        c.g[AT][0] = 0x43f00000;
                        c.mtc1(1, 15);
                        c.mov_s(17, 23);
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xd0));
                        c.mov_s(12, 21);
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd4));
                        c.mov_s(13, 21);
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0022bdc0);
                        c.g[S1][0] = 0xffffffffffffff83;
                        call_ptr(c, t, 0x0022bdc0)?;
                        c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x51e4));
                        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[A1][0] = 0x500000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe60));
                        c.g[AT][0] = 0x40000000;
                        c.mtc1(1, 22);
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x110));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x114));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0022bdec);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0022bdec)?;
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
                        c.g[V0][0] = c.g[V0][0] | 0x300;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
                        c.g[V0][0] = c.g[V0][0] | 0x14;
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x30));
                        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x34));
                        c.mul_s(0, 0, 20);
                        c.mul_s(1, 1, 20);
                        c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x38));
                        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x3c));
                        c.swc1(22, c.g[SP][0] as u32);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[SP][0] as u32);
                        c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x2f));
                        c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x28));
                        c.swc1(23, c.g[SP][0] as u32);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(7, c.g[SP][0] as u32);
                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x47));
                        c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x40));
                        c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4f));
                        c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[RA][0] = sx(0x0022be8c);
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa4));
                        call_ptr(c, 0x002cc098, 0x0022be8c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        if t {
                            break 'b_22bef0;
                        } else {
                            c.cycles += 23;
                            c.g[RA][0] = sx(0x0022be9c);
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa4));
                            call_ptr(c, 0x002cc350, 0x0022be9c)?;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                            c.swc1(22, c.g[SP][0] as u32);
                            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x4));
                            c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                            c.ldr(6, c.g[SP][0] as u32);
                            c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x2f));
                            c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x28));
                            c.swc1(23, c.g[SP][0] as u32);
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc));
                            c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                            c.ldr(5, c.g[SP][0] as u32);
                            c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
                            c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x47));
                            c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x40));
                            c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x4f));
                            c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                            break 'b_22bef0;
                        }
                    }
                    'b_22bfd8: {
                        c.cycles += 47;
                        c.g[V1][0] = 0x500000;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1420));
                        c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x7));
                        c.ldr(5, c.g[V0][0] as u32);
                        c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                        c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                        c.sdr(5, c.g[A0][0] as u32);
                        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                        c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
                        c.g[V0][0] = c.g[V0][0] | 0x300;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
                        c.g[V0][0] = c.g[V0][0] | 0x14;
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                        c.swc1(22, c.g[SP][0] as u32);
                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[SP][0] as u32);
                        c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x2f));
                        c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x28));
                        c.swc1(23, c.g[SP][0] as u32);
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(7, c.g[SP][0] as u32);
                        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x47));
                        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x40));
                        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x4f));
                        c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x48));
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1450));
                        let t = c.g[V0][0] != 0u64;

                        if t {
                            break 'b_22bfd8;
                        } else {
                            c.cycles += 11;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x4));
                            c.swc1(21, c.g[SP][0] as u32);
                            c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                            c.ldr(6, c.g[SP][0] as u32);
                            c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x2f));
                            c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x28));
                            c.g[RA][0] = sx(0x0022bfd0);
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x48));
                            call_ptr(c, 0x00397df8, 0x0022bfd0)?;
                            c.g[RA][0] = sx(0x0022bfd8);
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                            call_ptr(c, 0x0030b4b8, 0x0022bfd8)?;
                            break 'b_22bfd8;
                        }
                    }
                    c.cycles += 21;
                    c.g[RA][0] = sx(0x0022bfe0);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    call_ptr(c, 0x002789c0, 0x0022bfe0)?;
                    c.g[RA][0] = sx(0x0022bfe8);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x94));
                    call_ptr(c, 0x001f31e0, 0x0022bfe8)?;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0022c000);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0022c000)?;
                    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0xac));
                    c.cvt_s_w(0, 0);
                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x51e0));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x410));
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x0022c01c);
                    c.mul_s(12, 0, 12);
                    call_ptr(c, 0x00285bf8, 0x0022c01c)?;
                    c.g[RA][0] = sx(0x0022c024);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x21e4));
                    call_ptr(c, 0x002b5d78, 0x0022c024)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xac), 0u64);
                    c.g[V0][0] = 0x1;
                    break 'b_22c02c;
                }
            }
        }
        c.cycles += 18;
        c.g[T4][0] = 0x83e0;
        c.g[T5][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[SP][0] as u32));
        c.lq(16, (c.g[T5][0] as u32).wrapping_sub(0x30));
        c.lq(17, (c.g[T5][0] as u32).wrapping_sub(0x40));
        c.lq(18, (c.g[T5][0] as u32).wrapping_sub(0x50));
        c.lq(19, (c.g[T5][0] as u32).wrapping_sub(0x60));
        c.lq(20, (c.g[T5][0] as u32).wrapping_sub(0x70));
        c.lq(21, (c.g[T5][0] as u32).wrapping_sub(0x80));
        c.lq(22, (c.g[T5][0] as u32).wrapping_sub(0x90));
        c.lq(23, (c.g[T5][0] as u32).wrapping_sub(0xa0));
        c.lq(30, (c.g[T5][0] as u32).wrapping_sub(0xb0));
        c.g[RA][0] = c.ld((c.g[T5][0] as u32).wrapping_sub(0xc0));
        c.lwc1(23, (c.g[T5][0] as u32).wrapping_sub(0x8));
        c.lwc1(22, (c.g[T5][0] as u32).wrapping_sub(0x10));
        c.lwc1(21, (c.g[T5][0] as u32).wrapping_sub(0x18));
        c.lwc1(20, (c.g[T5][0] as u32).wrapping_sub(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[T4][0] as u32));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::game::camera::trigger::cameratriggerlist::tests::{
        fake_mem_alloc_named, PREV_MEM_ALLOC_NAMED,
    };
    use crate::game::main::game::tests::fake_get_be;
    use crate::game::main::game::tests::PREV_GET_BE;
    use crate::game::main::game::tests::{fake_be_get_interface, PREV_BE_GET_INTERFACE};
    use crate::game::wscript::wscriptman::tests::oc429_install_fakes;

    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;

    use crate::difftest::events::schedule;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as RET;

    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::fake_check;
    pub(crate) use crate::game::ai::ai::tests::leaf;

    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::heap_init_5 as heap_init;

    pub(crate) use crate::game::fe::fepopupmisc::tests::setup;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::ovstates::tests::base_3;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup as setup_2;

    pub(crate) use crate::game::lib::sif::tests::SEEDS;
    pub(crate) use crate::game::movie::movieplayer::tests::globals as globals_2;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_4;

    use ssx3_ee as ee;

    use ssx3_ee::s08;

    #[test]
    fn crowd_man_2d_add_sprite_matches() {
        check_lohi(
            ssx3_ee::s05::f_00229738,
            crowd_man_2d_add_sprite,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c10_0014 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.bus.write32(DATA + 0x2214, (seed % 8) as u32);
                for i in 0..4 {
                    c.bus.write32(DATA + 0x100 + i * 4, r.next_u32());
                }
                c.bus.write32(DATA + 0x2230, r.next_u32());
            },
        );
    }

    #[test]
    fn crowd_man2d_clear_slot_matches() {
        check(
            ssx3_ee::s05::f_00229b90,
            crowd_man2d_clear_slot,
            64,
            &|c, seed| {
                let mut r = Rng(0x0c11_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn ssx_app_next_tick_matches() {
        check(
            ssx3_ee::s05::f_00227e98__generated,
            ssx_app_next_tick,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 23, 0);
                let queue = DATA + 0x4_0000;
                let app = DATA + 0x300;
                // No app half the time; else a pad queue whose read cursor is
                // either still ahead (early out) or one step behind.
                if seed % 5 == 0 {
                    c.bus.write32(GAME_GP - 0x850, 0);
                } else {
                    c.bus.write32(GAME_GP - 0x850, queue);
                    let cursor = if seed % 3 == 0 { r.next_u32() % 30 } else { 0 };
                    let count = if seed % 3 == 0 {
                        (cursor + 1) % 30
                    } else {
                        r.next_u32() % 30
                    };
                    c.bus.write32(queue + 0x2ee0, cursor);
                    c.bus.write32(queue + 0x2ee4, count);
                }
                c.bus.write32(GAME_GP - 0x238, 0);
                c.bus.write32(GAME_GP + 0x300, 0);
                for i in 0..2u32 {
                    c.bus.write32(app + 0xa8 + i * 4, DATA + 0x800 + i * 0x100);
                }
                c.g[A0][0] = sx(app);
            },
        );
    }

    #[test]
    fn vu0_upload_occluder_matches() {
        check(
            ee::s05::f_00229f88__generated,
            vu0_upload_occluder,
            120,
            &|c, seed| {
                let mut r = Rng(0x51ed_2701 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx((seed % 4) as u32);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x140);
                for i in 0..4 {
                    put_vec(
                        c,
                        DATA + 0x100 + i * 0x10,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    );
                    put_vec(
                        c,
                        DATA + 0x140 + i * 0x10,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    );
                }
            },
        );
    }

    #[test]
    fn mc_manager_get_matches() {
        check(
            ee::s05::f_00227f80__generated,
            mc_manager_get,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22_7f80, 0);
                // The overlay manager already exists: take the cheap path.
                c.bus.write32(GAME_GP.wrapping_sub(0x47c), DATA + 0x500);
                c.g[A0][0] = r.next_u32() as u64;
            },
        );
    }

    // -----------------------------------------------------------------
    // World triggers.

    #[test]
    fn world_trigger_manager_destroy_instance_matches() {
        check(
            s08::f_002b4b48__generated,
            world_trigger_manager_destroy_instance,
            120,
            &|c, seed| {
                machine(c, seed, 0x2b4b48, 1);
                globals(c);
                c.g[A0][0] = sx(OBJ);
                clear(c, CHILD, 0x40);
                // A live singleton (destroyed, then cleared) or none.
                let mgr = if seed % 3 == 0 { 0 } else { CHILD };
                c.bus.write32(GAME_GP.wrapping_add(0x21e4), mgr);
            },
        );
    }

    #[test]
    fn crowd_man2d_clear_matches() {
        check(
            ssx3_ee::s05::f_00229498__generated,
            crowd_man2d_clear,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0xb0, 0);
                globals_2(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    // -------------------------------------------------------------- crowd

    #[test]
    fn crowd_man2d_add_matches() {
        check(
            ee::s05::f_00229788__generated,
            crowd_man2d_add,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x2297);
                let m = DATA;
                for i in 0..64u32 {
                    let v = if i == seed % 64 {
                        r.next_u32() & 0xffff_fffe
                    } else {
                        0xffff_ffff
                    };
                    c.bus.write32(m + 8 + 4 * i, v);
                }
                c.g[A0][0] = sx(m);
                c.g[A1][0] = sx(r.next_u32() % 8);
                c.pc = 0x0022_9788;
            },
        );
    }

    /// The real boot callees (graphics manager creation and init, the boot
    /// shape load from disc) do not finish in the test machine, so this is
    /// ignored and `ssx_app_init` stays out of `OVERRIDES`; run it with
    /// `--ignored` when trying a richer fixture.
    #[test]
    #[ignore]
    fn ssx_app_init_matches() {
        check(ssx3_ee::s05::f_00226900, ssx_app_init, 4, &|c, seed| {
            let _r = setup(c, seed, 0x226900);
            elf_image(c);
            heap_init(c);
            c.g[A0][0] = DATA as u64;
            // No command line arguments.
            c.g[A1][0] = 0;
            c.g[A2][0] = (DATA + 0x800) as u64;
            c.bus.write32(DATA + 0x800, 0);
        });
    }

    #[test]
    fn ssx_app_start_game_module_matches() {
        check(
            ssx3_ee::s05::f_002281d8__generated,
            ssx_app_start_game_module,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x2281d8);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// Both branches with the real allocator and constructors: the front
    /// end module (0xb5ae0 bytes) still fits the test arena.
    #[test]
    fn ssx_app_start_fe_module_matches() {
        check(
            ssx3_ee::s05::f_00228238__generated,
            ssx_app_start_fe_module,
            40,
            &|c, seed| {
                let mut r = setup(c, seed, 0x228238);
                heap_init(c);
                // The front-end constructor opens files: keep the lazy disc
                // load (0x3de768) bounded.
                crate::game::render::lightman::tests::eafile_pools(c);
                c.g[A0][0] = DATA as u64;
                let use_front_end = if seed % 2 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(DATA + 0x60, use_front_end);
                c.bus.write32(GAME_GP.wrapping_sub(0x10b4), r.next_u32());
            },
        );
    }

    #[test]
    fn crowd_man2d_dtor_matches() {
        check(
            ssx3_ee::s05::f_00229278__generated,
            crowd_man2d_dtor,
            60,
            &|c, seed| {
                let mut r = setup_2(c, seed, 0x229278);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0x400 + 4, 0);
                c.bus.write32(DATA + 0x400, 0);
                let _ = r.next_u32();
            },
        );
    }

    /// The shipped Nash patch table at [`NASH_PATCH_TABLE`].
    pub(crate) const NASH_PATCH_HANDLERS: [u32; 5] = [
        0x0022_a214,
        0x0022_a260,
        0x0022_a260,
        0x0022_a214,
        0x0022_a23c,
    ];

    /// The shipped Nash instance table at [`NASH_INSTANCE_TABLE`].
    pub(crate) const NASH_INSTANCE_HANDLERS: [u32; 5] = [
        0x0022_a0d0,
        0x0022_a11c,
        0x0022_a11c,
        0x0022_a0d0,
        0x0022_a0f8,
    ];

    // ------------------------------------------- replay / audio jump tables

    /// [`check`] plus soft state and LO/HI: these ports reproduce every
    /// register write of the generated body, so caller-saved temporaries and
    /// the multiply pipeline must match too.
    pub(crate) fn strict_check(
        gen: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = crate::difftest::compare(gen, hand, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard {:#?} soft {:#?} lohi {:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    // ------------------------------------------------ Nash octree classifiers

    /// Wire a cell's heap word to a heap block: `*(cell+0x90)` = `block` and
    /// `*block` = `heap`, then make a node's slot/handle words classify
    /// (`heap + slot*8 + 0x24` = 6 and `heap + handle*0x18 + 0x3f0` = 3).
    pub(crate) fn nash_link(
        c: &mut Cpu,
        cell: u32,
        block: u32,
        heap: u32,
        node: u32,
        slot: u8,
        handle: u16,
        handle_word: u32,
    ) {
        c.bus.write32(cell + 0x90, block);
        c.bus.write32(block, heap);
        c.bus.write8(node + 0x7d, slot);
        c.bus.write16(node + 0x7e, handle);
        c.bus
            .write32(heap.wrapping_add((slot as u32) << 3).wrapping_add(0x24), 6);
        c.bus.write32(
            heap.wrapping_add((handle as u32) * 0x18)
                .wrapping_add(0x3f0),
            handle_word,
        );
    }

    #[test]
    fn nash_classify_patch_matches() {
        strict_check(
            ssx3_ee::s05::f_0022a128__generated,
            nash_classify_patch,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22a1_28, 0);
                c.pc = 0x0022_a128;
                // A null node takes its "heap" from VU0 VI1 (`cfc2`), which the
                // tail also uses as the table index.
                c.bus.plat.write_ctrl(1, seed % 8);
                put_words(c, NASH_PATCH_TABLE, &NASH_PATCH_HANDLERS);
                let cell = DATA + 0x1000;
                let block = DATA + 0x2800;
                let node = DATA + 0x2000;
                let heap = DATA + 0x3000;
                c.g[A0][0] = sx(cell);
                // The insert heads/counts (both branches of every handler).
                c.bus
                    .write32(cell + 0x78b0, if seed % 2 == 0 { 0 } else { node });
                c.bus.write32(cell + 0x50a8, r.next_u32() & 3);
                c.bus.write32(cell + 0x70ac, r.next_u32() & 3);
                // A classifying node by default; the cases below break one link.
                nash_link(c, cell, block, heap, node, 2, 1, 3);
                c.bus.write16(node + 0x156, 1);
                c.bus.write8(node + 0x155, 2);
                match seed % 6 {
                    // Null node with the real cell as the list owner.
                    0 => c.g[A1][0] = 0,
                    // Null node whose "cell" is a small handle: the tail stores
                    // the node there and indexes the table by VI1.
                    1 => {
                        let k = (seed / 6) % 5;
                        c.g[A0][0] = sx(k);
                        c.bus
                            .write32(0x78b0 + k, if seed % 2 == 0 { 0 } else { node });
                        c.g[A1][0] = 0;
                    }
                    // A negative handle word: an early miss.
                    2 => {
                        c.bus.write16(node + 0x156, 0xffff);
                        c.g[A1][0] = sx(node);
                    }
                    // Slot word is not 6.
                    3 => {
                        c.bus.write32(heap + (2 * 8) + 0x24, 0);
                        c.g[A1][0] = sx(node);
                    }
                    // Handle word is not 3.
                    4 => {
                        c.bus.write32(heap + (1 * 0x18) + 0x3f0, 0);
                        c.g[A1][0] = sx(node);
                    }
                    // Fully classified: the heap block lives at `h`, and the tail
                    // dispatches on the VI1 handle (small: table, else: head).
                    _ => {
                        let h = if (seed / 6) % 2 == 0 {
                            heap
                        } else {
                            (seed / 12) % 5
                        };
                        nash_link(c, cell, block, h, node, 2, 1, 3);
                        c.bus.write16(node + 0x156, 1);
                        c.bus.write8(node + 0x155, 2);
                        c.g[A1][0] = sx(node);
                    }
                }
            },
        );
    }

    #[test]
    fn nash_classify_instance_matches() {
        strict_check(
            ssx3_ee::s05::f_00229fc8__generated,
            nash_classify_instance,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x229f_c8, 0);
                c.pc = 0x0022_9fc8;
                // A null node takes its "heap" from VU0 VI1 (`cfc2`), which the
                // tail also uses as the table index.
                c.bus.plat.write_ctrl(1, seed % 8);
                put_words(c, NASH_INSTANCE_TABLE, &NASH_INSTANCE_HANDLERS);
                let cell = DATA + 0x1000;
                let block = DATA + 0x2800;
                let node = DATA + 0x2000;
                let heap = DATA + 0x3000;
                c.g[A0][0] = sx(cell);
                c.bus
                    .write32(cell + 0x50a4, if seed % 2 == 0 { 0 } else { node });
                c.bus.write32(cell + 0x209c, r.next_u32() & 3);
                c.bus.write32(cell + 0x40a0, r.next_u32() & 3);
                c.bus.write32(node + 0x8, 3);
                nash_link(c, cell, block, heap, node, 2, 1, 3);
                match seed % 6 {
                    // Null node with the real cell as the list owner.
                    0 => c.g[A1][0] = 0,
                    // Null node whose "cell" is a small handle: inserts there.
                    1 => {
                        let k = (seed / 6) % 5;
                        c.g[A0][0] = sx(k);
                        c.bus
                            .write32(0x50a4 + k, if seed % 2 == 0 { 0 } else { node });
                        c.g[A1][0] = 0;
                    }
                    // Type word does not end in 3.
                    2 => {
                        c.bus.write32(node + 0x8, 4);
                        c.g[A1][0] = sx(node);
                    }
                    // Byte 0xff ("no slot"): classified, then the VI1 tail test.
                    3 => {
                        let h = if (seed / 6) % 2 == 0 {
                            heap
                        } else {
                            (seed / 12) % 5
                        };
                        nash_link(c, cell, block, h, node, 2, 1, 3);
                        c.bus.write8(node + 0x7d, 0xff);
                        c.g[A1][0] = sx(node);
                    }
                    // Slot word is not 6.
                    4 => {
                        c.bus.write32(heap + (2 * 8) + 0x24, 0);
                        c.g[A1][0] = sx(node);
                    }
                    // Fully classified: dispatches on the VI1 handle in the tail.
                    _ => {
                        let h = if (seed / 6) % 2 == 0 {
                            heap
                        } else {
                            (seed / 12) % 5
                        };
                        nash_link(c, cell, block, h, node, 2, 1, 3);
                        c.g[A1][0] = sx(node);
                    }
                }
            },
        );
    }

    #[test]
    fn nash_empty_hook_matches() {
        leaf(ssx3_ee::s05::f_00229f80__generated, nash_empty_hook, 9);
    }

    // ---------------------------------------------------- Nash_ClassifyEffect

    #[test]
    fn nash_classify_effect_matches() {
        fake_check(
            ssx3_ee::s05::f_0022a270__generated,
            nash_classify_effect,
            39,
            &|c, seed, r| {
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    (DATA + 0x100) as u64
                };
                // VI1 selects the effect-kind table entry (the code moves it into
                // a0 with cfc2).
                c.bus.plat.write_ctrl(1, seed % 5);
                for i in 0..0x400 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
                // The effect-kind table (`.rodata 0x47b380`).
                put_words(
                    c,
                    0x0047_b380,
                    &[
                        0x0022_a30c,
                        0x0022_a358,
                        0x0022_a358,
                        0x0022_a30c,
                        0x0022_a334,
                    ],
                );
                // Registration heads/counts for the list-append branches.
                c.bus
                    .write32(DATA + 0x7ab8, if seed % 2 == 0 { 0 } else { DATA });
                c.bus.write32(DATA + 0x78b4, r.next_u32() & 3);
                c.bus.write32(DATA + 0x7abc, r.next_u32() & 3);
            },
        );
    }

    #[test]
    fn crowd_man2d_remove_all_matches() {
        check(
            ssx3_ee::s05::f_00229398__generated,
            crowd_man2d_remove_all,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x229398, 0);
                let m = DATA + 0x1000;
                c.g[A0][0] = m as u64;
                for i in 0..128u32 {
                    w32(
                        c,
                        m + 8 + 4 * i,
                        if (seed as u32 + i) % 3 == 0 {
                            0xffff_ffff
                        } else {
                            r.next_u32()
                        },
                    );
                }
            },
        );
    }

    #[test]
    fn crowd_man2d_remove_by_type_matches() {
        check(
            ee::s05::f_00229408__generated,
            crowd_man2d_remove_by_type,
            100,
            &|c, seed| {
                let _ = base_2(c, seed, 0x22_9408);
                globals_3(c);
                for i in 0..128u32 {
                    let slot = DATA + 8 + 4 * i;
                    match (seed.wrapping_add(i)) % 4 {
                        0 => w(c, slot, 0xffff_ffff),
                        1 => w(c, slot, seed & 0xff),
                        _ => w(c, slot, 0x0100_0000u32 | (i & 0xff)),
                    }
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(seed & 0xff);
            },
        );
    }

    // ---------------------------------------------------------------- Crowd.

    #[test]
    fn crowd_man2d_reset_matches() {
        check(
            ee::s05::f_002292e0__generated,
            crowd_man2d_reset,
            5,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x22_92e0);
                c.g[A0][0] = sx(DATA);
                for i in 0..0x2a0u32 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn crowdman2d_load_state_matches() {
        check(
            ee::s05::f_00229e58__generated,
            crowdman2d_load_state,
            8,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x229e58, 2);
                let this = DATA;
                clear_2(c, this, 0x1000);
                let state = DATA + 0x800;
                clear_2(c, state, 0x100);
                let vt = vtable(c, &mut r, &[0x10]);
                w(c, state, vt);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = state as u64;
                for i in 0..128u32 {
                    w(c, SP - 0x250 + 4 * i, 0xffff_ffff);
                }
            },
        );
    }

    /// A `CrowdMan2d` at `man`: its animation takes the short path, the
    /// crowd reaction is `reaction`, every ninth emitter slot is used (timers
    /// 0..59) and the 40 flash lifetimes are -2..4.
    fn crowd_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, man: u32, reaction: i32) {
        let (anim, clock, audio, table) = (
            DATA + 0x3_0000,
            DATA + 0x3_0100,
            DATA + 0x3_0200,
            DATA + 0x3_0300,
        );
        c.bus.write32(man + 4, anim);
        c.bus.write32(anim + 0x10, r.next_u32());
        c.bus.write32(anim + 0x14, 0x7fff_ffff);
        c.bus.write32(anim + 0x18, 0);
        c.bus.write32(GAME_GP + 0x2a74, clock);
        c.bus.write32(clock + 0x1c, 100);
        c.bus.write32(GAME_GP + 0x410, audio);
        c.bus.write32(audio + 0x14, 1);
        c.bus.write32(audio + 0x18, table);
        c.bus.write32(table + 0x18c, reaction as u32);
        let old = if seed % 3 == 0 {
            reaction.unsigned_abs()
        } else {
            r.next_u32() % 5
        };
        c.bus.write32(man + 0x2210, old);
        c.bus.write32(man + 0x2214, seed % 40);
        for i in 0..128u32 {
            let used = (i + seed) % 9 == 0;
            c.bus
                .write32(man + 8 + 4 * i, if used { i } else { 0xffff_ffff });
            let em = man + 0x40 * i;
            for k in 0..12 {
                c.bus.write32(em + 0x210 + 4 * k, r.f32(100.0));
            }
            c.bus.write32(em + 0x240, r.next_u32() % 60);
        }
        for k in 0..40 {
            c.bus
                .write32(man + 0x2230 + 0x20 * k, (r.next_u32() % 7).wrapping_sub(2));
        }
    }

    #[test]
    fn crowd_man2d_spawn_sprites_matches() {
        check(
            ee::s05::f_00229530__generated,
            crowd_man2d_spawn_sprites,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22_9530, 0);
                crowd_fixture(c, &mut r, seed, DATA, (seed % 7) as i32 - 3);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn crowd_man2d_update_matches() {
        check(
            ee::s05::f_002294c8__generated,
            crowd_man2d_update,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22_94c8, 0);
                crowd_fixture(c, &mut r, seed, DATA, (seed % 7) as i32 - 3);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// Over one second of ticks the crowd flashes (how many, and how long
    /// they show in total) are the same at 60, 120 and 240 Hz. Each tick ages
    /// the lifetimes as `CrowdMan2d_Update` does, then spawns.
    #[test]
    fn crowd_man2d_flashes_follow_the_rate() {
        fn second(hz: u32) -> (u32, f64) {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = machine(c, 3, 0x22_94c9, 0);
            // (Its own address: frame60_due keeps a clock per manager.)
            let man = DATA + 0x4_0000 + 0x1_0000 * (hz / 60);
            crowd_fixture(c, &mut r, 0, man, 10);
            // After the fixture: `gp+0x2a74` is the app manager pointer.
            crate::tick::set_for_test(c, hz);
            c.bus.write32(man + 0x2210, 10);
            for k in 0..40 {
                c.bus.write32(man + 0x2230 + 0x20 * k, 0);
            }
            let (mut spawned, mut shown) = (0u32, 0u64);
            for _ in 0..hz {
                for k in 0..40 {
                    let life = man + 0x2230 + 0x20 * k;
                    let v = c.bus.read32(life) as i32;
                    if v > 0 {
                        c.bus.write32(life, (v - 1) as u32);
                    }
                }
                let before = c.bus.read32(man + 0x2214);
                c.g[A0][0] = man as u64;
                crowd_man2d_spawn_sprites(c);
                assert_eq!(c.pc, RET);
                spawned += (c.bus.read32(man + 0x2214) + 40 - before) % 40;
                shown += (0..40)
                    .filter(|k| c.bus.read32(man + 0x2230 + 0x20 * k) as i32 > 0)
                    .count() as u64;
            }
            (spawned, shown as f64 / hz as f64)
        }
        let (n60, t60) = second(60);
        assert!(n60 > 20, "{n60} flashes");
        for hz in [120, 240] {
            let (n, t) = second(hz);
            assert_eq!(n, n60, "{hz} Hz: {n} flashes vs {n60} at 60");
            assert!(
                (t - t60).abs() < 1e-9,
                "{hz} Hz: shown {t} s vs {t60} s at 60"
            );
        }
    }

    use ssx3_ee_rt::cpu::sx;
    use ssx3_ee_rt::Cpu;

    /// A random float in [-range, range) as bits.
    pub(crate) fn f(r: &mut Rng, range: f32) -> u32 {
        r.f32(range)
    }

    // ------------------------------------------------------------ Nash pass

    /// The VU0 microprogram at 0xef0 for the tests: `vi1 = vi14` (the
    /// node's clip word), so a clip word of 2 culls.
    pub(crate) fn put_cull_microprogram(c: &mut Cpu) {
        ssx3_ps2::platform::machine_of(&mut c.bus.plat)
            .hw
            .vu0
            .fallback = Some(ssx3_ps2::vu::reference::run_program);
        let at = 0x1100_0000 + 0xef0;
        c.bus.write32(at, 0x8000_7070); // IADD vi1, vi14, vi0
        c.bus.write32(at + 4, 0x4000_02ff); // NOP, E bit
        c.bus.write32(at + 8, 0x8000_033c); // NOP
        c.bus.write32(at + 12, 0x0000_02ff); // NOP
    }

    /// `Nash_CollectPatches` input: `nodes` (node, clipped) pairs with up
    /// to `per_node` patches each, the section tables, a patch count.
    pub(crate) fn patches_setup(c: &mut Cpu, r: &mut Rng, nodes: u32, per_node: u32) -> u32 {
        put_cull_microprogram(c);
        let this = DATA + 0x400;
        let tables = DATA + 0x40000;
        c.bus.write32(this + 0x90, DATA + 0x100);
        c.bus.write32(DATA + 0x100, tables);
        for s in 0..16 {
            let state = if r.next_u32() % 2 == 0 {
                6
            } else {
                r.next_u32() % 8
            };
            c.bus.write32(tables + 8 * s + 0x24, state);
        }
        for l in 0..32u32 {
            let state = if r.next_u32() % 2 == 0 {
                3
            } else {
                r.next_u32() % 8
            };
            c.bus.write32(tables + 0x18 * l + 0x3f0, state);
        }
        c.bus.write32(this + 0x50a8, r.next_u32() % 10);
        let pairs = DATA + 0x200;
        let mut patch = DATA + 0x10000;
        for i in 0..nodes {
            let node = DATA + 0x8000 + 0x40 * i;
            c.bus.write32(pairs + 8 * i, node);
            c.bus.write32(
                pairs + 8 * i + 4,
                [0, 1, 2, 2, r.next_u32()][(r.next_u32() % 5) as usize],
            );
            let n = r.next_u32() % (per_node + 1);
            let mut head = 0;
            for _ in 0..n {
                fill(c, r, patch + 0x140, 4);
                c.bus.write32(patch, head);
                c.bus.write8(patch + 0x155, (r.next_u32() % 16) as u8);
                let logical = if r.next_u32() % 5 == 0 {
                    -((r.next_u32() % 8) as i32) - 1
                } else {
                    (r.next_u32() % 32) as i32
                };
                c.bus.write16(patch + 0x156, logical as u16);
                head = patch;
                patch += 0x200;
            }
            c.bus.write32(node + 0x24, head);
        }
        c.g[A0][0] = this as u64;
        c.g[A1][0] = pairs as u64;
        c.g[A2][0] = if nodes == 0 && r.next_u32() % 2 == 0 {
            sx(0xffff_fff0)
        } else {
            nodes as u64
        };
        this
    }

    #[test]
    fn nash_collect_patches_matches() {
        let gen = ssx3_ee::s05::f_0022a698__generated;
        let hand = super::nash_collect_patches;
        check_events(gen, hand, 300, &|c, seed| {
            let mut r = Rng(0x7061_7463 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            let nodes = [0, 1, 3, 5][(seed / 3 % 4) as usize];
            patches_setup(c, &mut r, nodes, 5);
            schedule(c, &mut r, seed, 0x0022_a698)
        });
        // Every patch path (negative logical / section / logical filter,
        // clipped kept / culled, unclipped) for one patch.
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..400 {
            let mut c = crate::difftest::fresh();
            let mut r = Rng(0x7031 ^ (seed as u64) << 20);
            scramble(&mut c, &mut r);
            let this = patches_setup(&mut c, &mut r, 1, 1);
            let before = c.bus.read32(this + 0x50a8);
            crate::difftest::run_generated(&mut c, gen);
            seen.insert((c.cycles, c.bus.read32(this + 0x50a8) - before));
        }
        assert!(seen.len() >= 5, "patch paths seen: {seen:?}");
    }

    #[test]
    fn vu0_upload_matrix_pair_matches() {
        check(
            ssx3_ee::s05::f_00229f30__generated,
            super::vu0_upload_matrix_pair,
            100,
            &|c, seed| {
                let mut r = Rng(0x7570_6c64 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA, 32);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x40);
            },
        );
    }

    /// `Nash_GatherVisible` input: the world at `this`, the eight roots
    /// (each present with probability 1/2; small octrees of level 11..12
    /// with empty instance / patch / effect lists), an occlusion manager
    /// with no views (`OcclusionMan_BuildVisible` takes its short path),
    /// the graphics object with a fake camera interface slot 43 and a world
    /// matrix. `Nash_CullNode` runs on the hand-written VU0 routines.
    pub(crate) fn gather_setup(c: &mut Cpu, r: &mut Rng) {
        ssx3_ps2::platform::machine_of(&mut c.bus.plat)
            .hw
            .vu0
            .fallback = Some(ssx3_vu::vu::vu0lib::run);
        let this = DATA + 0x20000;
        let roots = DATA + 0x300;
        let occ = DATA + 0x600;
        let tables = DATA + 0x700;
        let gfx = DATA + 0x30000;
        let vt = DATA + 0x38000;
        c.bus.write32(GAME_GP - 0x854, gfx);
        c.bus.write32(gfx + 0x10d8, vt);
        let delta = (r.next_u32() as i16 as i32 % 64) & !3;
        c.bus.write16(vt + 0x158, delta as u16);
        c.bus.write32(vt + 0x15c, FAKE);
        for i in 0..16 {
            c.bus.write32(gfx + 0x5800 + 4 * i, f(r, 2.0));
        }
        // Stale list counts and patch counts (the pass clears them).
        for off in [0x94u32, 0x1098, 0x209c, 0x50a8, 0x7fc4] {
            c.bus.write32(this + off, r.next_u32() % 50);
        }
        c.bus.write32(occ + 4, 0);
        c.bus.write32(occ + 0x110, r.next_u32());
        c.bus.write32(tables, DATA + 0x800);
        let mut next = DATA + 0x10000;
        fn tree(c: &mut Cpu, r: &mut Rng, next: &mut u32, depth: u32) -> u32 {
            let node = *next;
            *next += 0x40;
            for k in 0..8 {
                let child = if depth > 0 && r.next_u32() % 2 == 0 {
                    tree(c, r, next, depth - 1)
                } else {
                    0
                };
                c.bus.write32(node + 4 * k, child);
            }
            node
        }
        for k in 0..8u32 {
            let cell = roots + 0x14 * k;
            let level = 11 + r.next_u32() % 2;
            c.bus.write32(cell, level);
            for a in 1..4u32 {
                c.bus
                    .write32(cell + 4 * a, ((r.next_u32() % 9) as i32 - 4) as u32);
            }
            let node = if r.next_u32() % 2 == 0 {
                tree(c, r, &mut next, level - 11)
            } else {
                0
            };
            c.bus.write32(cell + 0x10, node);
        }
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(roots);
        c.g[A2][0] = sx(occ);
        c.g[A3][0] = r.next_u32() as u64;
        c.g[T0][0] = sx(tables);
    }

    #[test]
    fn nash_gather_visible_matches() {
        covered(
            ssx3_ee::s05::f_0022add8__generated,
            super::nash_gather_visible,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x6761_7468, 1);
                gather_setup(c, &mut r);
            },
        );
    }

    // ------------------------------------------------- CrowdMan2d_RenderSprites

    /// `CrowdMan2d_RenderSprites`: 40 sprite records with random visible
    /// counts (none visible for some seeds: no packet), the drawer's sprite
    /// method a fake (one of the calls may unwind), the packet cursor at
    /// various alignments.
    #[test]
    fn crowd_man_2d_render_sprites_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        check(
            ee::s05::f_00229ba8__generated,
            crowd_man_2d_render_sprites,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x229b, 40);
                c.pc = 0x0022_9ba8;
                let (this, ctx, vt) = (DATA, DATA + 0x10000, DATA + 0x14000);
                let cur = DATA + 0x8000 + 4 * (seed % 4);
                for i in 0..40u32 {
                    let rec = this + 0x2220 + 0x20 * i;
                    fill(c, &mut r, rec, 4);
                    let visible = match (seed % 4, r.next_u32() % 3) {
                        (0, _) => 0,
                        (_, 0) => 0,
                        (_, 1) => r.next_u32(),
                        _ => 1,
                    };
                    c.bus.write32(rec + 0x10, visible);
                }
                c.bus.write32(GAME_GP + 0x2a90, ctx);
                c.bus.write32(ctx + 0xe84, cur);
                c.bus.write32(ctx + 0x1054, r.next_u32());
                c.bus.write32(ctx + 0x10d8, vt);
                c.bus.write16(vt + 0x270, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + 0x274, FAKE);
                fill(c, &mut r, cur - 0x20, 0x20);
                fill(c, &mut r, 0x0050_1420, 6);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// Nash_ClassifyEffects: 0-3 items whose models have no nodes or lists
    /// of effect (7), plain (8) and other nodes; the registration head set
    /// or not. Nash_ClassifyEffect runs for real on the fixture of its own
    /// test (effect-kind table, VI1, small list counts).
    #[test]
    fn nash_classify_effects_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, put_words, DATA};
        check(
            ee::s05::f_0022a4a8__generated,
            nash_classify_effects,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22_a4a8, 2);
                c.pc = 0x0022_a4a8;
                let this = DATA;
                fill(c, &mut r, this, 0x400);
                c.bus.plat.write_ctrl(1, seed % 5);
                put_words(
                    c,
                    0x0047_b380,
                    &[
                        0x0022_a30c,
                        0x0022_a358,
                        0x0022_a358,
                        0x0022_a30c,
                        0x0022_a334,
                    ],
                );
                c.bus
                    .write32(this + 0x7ab8, if seed % 2 == 0 { 0 } else { this });
                c.bus.write32(this + 0x78b4, r.next_u32() & 3);
                c.bus.write32(this + 0x7abc, r.next_u32() & 3);
                c.bus.write32(this + 0x7bc0, r.next_u32() & 3);
                let items = DATA + 0x9000;
                let n = if seed % 7 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 3
                };
                let mut node_at = DATA + 0xa000;
                for i in 0..n {
                    let item = items + 8 * i;
                    let model = DATA + 0x9800 + 0x40 * i;
                    c.bus.write32(item, model);
                    c.bus.write32(item + 4, r.next_u32() % 4);
                    let len = r.next_u32() % 5;
                    let mut next = 0;
                    for _ in 0..len {
                        let node = node_at;
                        node_at += 0x100;
                        fill(c, &mut r, node, 0x40);
                        c.bus.write32(node, next);
                        let kind = [7, 8, 7, 8, r.next_u32() % 16][(r.next_u32() % 5) as usize];
                        c.bus.write32(node + 8, kind);
                        next = node;
                    }
                    c.bus.write32(model + 0x28, next);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(items);
                c.g[A2][0] = if seed % 11 == 3 {
                    sx(0xffff_fffe)
                } else {
                    n as u64
                };
            },
        );
    }

    /// `Nash_CollectEffects`: zero, negative and 1-4 visible nodes (clipped
    /// or not), each with up to 5 effects of kind 7 (emitter), 8 (light) or
    /// other; the test sphere microprogram culls clip word 2.
    #[test]
    fn nash_collect_effects_matches() {
        use super::nash_fx::*;
        use crate::game::main::ssxapp::tests::put_cull_microprogram;
        crate::difftest::check(
            ee::s05::f_0022a770__generated,
            nash_collect_effects,
            300,
            &|c, seed| {
                let mut r = Rng(0x0022_a770 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                put_cull_microprogram(c);
                c.pc = 0x0022_a770;
                let (me, pairs) = (DATA, DATA + 0x9000);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(pairs);
                c.g[A2][0] = match seed % 6 {
                    0 => 0,
                    1 => sx((r.next_u32() % 3 + 1).wrapping_neg()),
                    _ => (1 + r.next_u32() % 4) as u64,
                };
                c.bus.write32(me + EMITTER_COUNT, r.next_u32() % 16);
                c.bus.write32(me + LIGHT_COUNT, r.next_u32() % 16);
                let mut fx = DATA + 0xa000;
                for p in 0..4u32 {
                    let node = DATA + 0x9100 + 0x40 * p;
                    let pair = pairs + PAIR_SIZE * p;
                    c.bus.write32(pair, node);
                    let clip = match r.next_u32() % 3 {
                        0 => 0,
                        1 => OUTSIDE as u32,
                        _ => r.next_u32() % 4,
                    };
                    c.bus.write32(pair + PAIR_CLIP, clip);
                    let mut next = 0;
                    for _ in 0..r.next_u32() % 6 {
                        let kind = match r.next_u32() % 4 {
                            0 | 1 => KIND_EMITTER as u32,
                            2 => KIND_LIGHT as u32,
                            _ => r.next_u32() % 16,
                        };
                        c.bus.write32(fx, next);
                        c.bus.write32(fx + FX_KIND, kind);
                        fill(c, &mut r, fx + FX_SPHERE, 4);
                        next = fx;
                        fx += 0x60;
                    }
                    c.bus.write32(node + NODE_EFFECTS, next);
                }
            },
        );
    }

    // ---------------------------------------------------- Nash_ClassifyPatches

    /// `Nash_ClassifyPatches`: zero, negative and up to four list entries,
    /// sources with no patches and with chains of one to three; the cell's
    /// list head set or clear (the extra no-node classify, whose `VI1`
    /// handle picks every arm of the patch table). The patch nodes take the
    /// classifier's early misses (negative handle word, or a slot word that
    /// is not 6), so no VU0 micro program runs.
    #[test]
    fn nash_classify_patches_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, put_words, DATA};
        use crate::game::main::ssxapp::tests::NASH_PATCH_HANDLERS;
        use crate::game::main::ssxapp::NASH_PATCH_TABLE;
        check(
            ee::s05::f_0022a408__generated,
            nash_classify_patches,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_a408, 0);
                c.pc = 0x0022_a408;
                put_words(c, NASH_PATCH_TABLE, &NASH_PATCH_HANDLERS);
                c.bus.plat.write_ctrl(1, seed % 8);
                let cell = DATA + 0x1_0000;
                let block = DATA + 0x2_0000;
                let heap = DATA + 0x3_0000;
                let list = DATA + 0x4_0000;
                let sources = DATA + 0x5_0000;
                let nodes = DATA + 0x6_0000;
                c.g[A0][0] = sx(cell);
                c.g[A1][0] = sx(list);
                // The classifier's heap: slot words all 0 (not 6), so a node with
                // a non-negative handle misses too.
                c.bus.write32(cell + 0x90, block);
                c.bus.write32(block, heap);
                c.bus.write32(cell + 0x50a8, r.next_u32() & 3);
                c.bus.write32(cell + 0x70ac, r.next_u32() & 3);
                c.bus.write32(
                    cell + CELL_LIST_HEAD,
                    if seed % 3 == 0 { 0 } else { nodes + 0x8000 },
                );
                let count = match seed % 7 {
                    0 => 0,
                    1 => (-(r.next_u32() as i32 & 0xff) - 1) as u32,
                    _ => 1 + r.next_u32() % 4,
                };
                c.g[A2][0] = sx(count);
                let entries = if (count as i32) > 0 { count } else { 0 };
                let mut node = nodes;
                for i in 0..entries {
                    let src = sources + 0x40 * i;
                    c.bus.write32(list + VIS_ENTRY_SIZE * i + VIS_SOURCE, src);
                    c.bus
                        .write32(list + VIS_ENTRY_SIZE * i + VIS_FRUSTUM, r.next_u32());
                    let chain = r.next_u32() % 4;
                    let mut link = src + SOURCE_FIRST_PATCH;
                    for _ in 0..chain {
                        c.bus.write32(link, node);
                        let handle = if r.next_u32() % 2 == 0 {
                            0xffff
                        } else {
                            r.next_u32() as u16 & 0x7
                        };
                        c.bus.write16(node + 0x156, handle);
                        c.bus.write8(node + 0x155, r.next_u32() as u8);
                        link = node + PATCH_NEXT;
                        node += 0x200;
                    }
                    c.bus.write32(link, 0);
                }
            },
        );
    }

    /// `Nash_ClassifyInstances`: 0..4 lists (or a negative count), each
    /// source with 0..4 nodes; the real `Nash_ClassifyInstance` runs its
    /// short paths: node type words not ending in 3 (a miss), or ending in
    /// 3 with a heap slot word that is not 6 (a miss after the heap look-up);
    /// the trailing null-node call (instance head at +0x50a4 set on half the
    /// seeds) reads VU0 VI1 = 5..7 and stores the null node at the head.
    #[test]
    fn nash_classify_instances_matches() {
        use crate::difftest::check;
        const CELL: u32 = DATA;
        const HEAPPTR: u32 = DATA + 0x6000;
        const HEAP: u32 = DATA + 0x6100;
        const LISTS: u32 = DATA + 0x8000;
        const SRCS: u32 = DATA + 0x8100;
        const NODES: u32 = DATA + 0x9000;
        check(
            ssx3_ee::s05::f_0022a368__generated,
            nash_classify_instances,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x22a368, 0);
                c.pc = 0x0022_a368;
                c.bus.plat.write_ctrl(1, 5 + seed % 3);
                c.g[A0][0] = sx(CELL);
                c.bus.write32(
                    CELL + 0x50a4,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(CELL + 0x90, HEAPPTR);
                c.bus.write32(HEAPPTR, HEAP);
                for i in 0..0x40 {
                    c.bus.write32(HEAP + 4 * i, r.next_u32() & !2);
                }
                let n = r.next_u32() % 5;
                c.g[A1][0] = sx(LISTS);
                c.g[A2][0] = if seed % 13 == 5 {
                    sx(0xffff_fffd)
                } else {
                    n as u64
                };
                let mut node_at = NODES;
                for i in 0..n {
                    let src = SRCS + 0x40 * i;
                    c.bus.write32(LISTS + 8 * i, src);
                    c.bus.write32(LISTS + 8 * i + 4, r.next_u32());
                    let mut next = 0;
                    for _ in 0..r.next_u32() % 5 {
                        let node = node_at;
                        node_at += 0x100;
                        c.bus.write32(node, next);
                        let kind = r.next_u32();
                        c.bus
                            .write32(node + 8, if kind % 3 == 0 { kind | 3 } else { kind & !1 });
                        c.bus.write8(node + 0x7d, (r.next_u32() % 6) as u8);
                        next = node;
                    }
                    c.bus.write32(src + 0x20, next);
                }
            },
        );
    }

    /// SSXApp_Tick: no pad queue, or one with the real PadQueue_Poll (the
    /// fixture of its own test: 0..4 devices, or none, whose poll methods
    /// are fakes unwinding on some seeds).
    #[test]
    fn ssx_app_tick_matches() {
        crate::difftest::check(
            ee::s05::f_00227f58__generated,
            ssx_app_tick,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22_7f58, 4);
                c.pc = 0x0022_7f58;
                let q = DATA;
                let present = seed % 5 != 0;
                c.bus
                    .write32(GAME_GP - SSX_APP_PAD_QUEUE_GP, if present { q } else { 0 });
                let n = if seed % 9 == 0 { u32::MAX } else { seed % 5 };
                c.bus.write32(q + 0x2ee8, n);
                c.bus.write32(
                    q + 0x2ee4,
                    if seed % 3 == 0 { 29 } else { r.next_u32() % 30 },
                );
                for i in 0..4u32 {
                    let dev = DATA + 0x4000 + 0x40 * i;
                    let vt = DATA + 0x5000 + 0x40 * i;
                    c.bus.write32(q + 0x2eec + 4 * i, dev);
                    c.bus.write32(dev, vt);
                    c.bus
                        .write16(vt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
                    c.bus.write32(vt + 0x14, FAKE + 0x10 * i);
                }
            },
        );
    }

    /// `self` at DATA+0x800, the stream at DATA+0x10 whose vtable slot 8 is a
    /// fake write (random `this` delta; some calls unwind).
    #[test]
    fn crowd_man2d_save_state_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        covered(
            ee::s05::f_00229e20__generated,
            crowd_man2d_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_9e20, 1);
                let stream = DATA + 0x10;
                let vt = vtable(c, &mut r, &[0x8]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(stream);
                c.pc = 0x0022_9e20;
            },
        );
    }

    /// `CrowdMan2d_Remove`: the handle found at some slot (cleared) or in no
    /// slot (all 64 scanned).
    #[test]
    fn crowd_man2d_remove_matches() {
        check(
            ee::s05::f_002297d8__generated,
            crowd_man2d_remove,
            200,
            &|c, seed| {
                let mut r = Rng(0x2297_d800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0022_97d8;
                c.g[A0][0] = DATA as u64;
                let handle = 0x1000 + (seed % 8) as u32;
                c.g[A1][0] = handle as u64;
                let hit = if seed % 3 == 0 { None } else { Some(seed % 64) };
                for i in 0..64u32 {
                    let v = if Some(i) == hit { handle } else { r.next_u32() };
                    c.bus.write32(DATA + CROWD_MAN2D_REMOVE_SLOTS + 4 * i, v);
                }
            },
        );
    }

    pub(crate) use crate::game::world::worldview::tests::oc410_machine;
    pub(crate) use crate::game::world::worldview::tests::oc410_result;

    pub(crate) use crate::game::ai::ai::tests::OC414_SOUP;
    pub(crate) use crate::game::ai::rider::tests::w32_2;

    pub(crate) use crate::game::ai::ai::tests::oc414_env;

    /// `CrowdMan2d_BindInstanceFrame`: handle -> model -> part -> bone.
    #[test]
    fn crowd_man2d_bind_instance_frame_matches() {
        const TOP: u32 = DATA + 0x1000;
        const TABLE1: u32 = DATA + 0x1100;
        const SLOTS: u32 = DATA + 0x1200;
        const CELL: u32 = DATA + 0x2000;
        const TABLE2: u32 = DATA + 0x2100;
        const MODEL: u32 = DATA + 0x3000;
        const SKEL: u32 = DATA + 0x4000;
        const POSE: u32 = DATA + 0x4100;
        const PARTS: u32 = DATA + 0x4200;
        check(
            ee::s05::f_00229820__generated,
            crowd_man2d_bind_instance_frame,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0022_9820, 1);
                c.pc = 0x0022_9820;
                let lo = seed % 3;
                let hi = seed % 2;
                c.g[A1][0] = ((hi << 8) | lo) as u64;
                c.g[A2][0] = if seed % 11 == 0 {
                    u64::MAX
                } else {
                    (seed % 5) as u64
                };
                c.g[A3][0] = ((seed / 5) % 4) as u64;
                w32_2(c, oc410_result(29), TOP);
                w32_2(c, TOP, TABLE1);
                w32_2(c, TABLE1 + 8, SLOTS);
                for i in 0..3 {
                    w32_2(
                        c,
                        SLOTS + 4 * i,
                        if seed % 4 == 1 && i == lo { 0 } else { CELL },
                    );
                }
                w32_2(c, CELL + 0x1c, TABLE2);
                for i in 0..2 {
                    let x = if seed % 7 == 0 {
                        0
                    } else {
                        ((MODEL >> 2) << 8) | (r.next_u32() & 0xff)
                    };
                    w32_2(c, TABLE2 + 4 * i, x);
                }
                w32_2(c, MODEL + 0x80, SKEL);
                w32_2(c, MODEL + 0x94, POSE);
                w32_2(c, SKEL + 4, 3);
                w32_2(c, SKEL + 8, PARTS);
                for i in 0..3u32 {
                    let part = DATA + 0x4300 + 0x100 * i;
                    w32_2(c, PARTS + 0x10 * i + 4, part);
                    w32_2(c, part + 0x1c, 2 + i % 2);
                    w32_2(c, part + 0x20, part + 0x80);
                    for k in 0..3u32 {
                        let idx_at = DATA + 0x4800 + 0x20 * i + 4 * k;
                        w32_2(c, part + 0x80 + 4 * k, idx_at);
                        c.bus.write16(idx_at, (r.next_u32() % 4) as u16);
                    }
                }
            },
        );
    }

    /// `CrowdMan2d_InitSlot`: a model handle that resolves (a model block with a
    /// matrix) or not, several slot indices; the graphics vtable is the
    /// pointer soup, so the quad fetch is a fake call.
    #[test]
    fn crowd_man_2d_init_slot_matches() {
        check(
            ee::s05::f_00229910__generated,
            crowd_man_2d_init_slot,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x229910, 0);
                oc414_env(c);
                c.pc = 0x0022_9910;
                let this = DATA + 0x100;
                // The handle: low byte selects a table slot, high bits a
                // sub-entry; the model block is at `model`.
                let id = (seed % 3) | ((seed % 5) << 8);
                let model = DATA + 0x3000;
                let (g, h, x, y) = (DATA + 0x2000, DATA + 0x2100, DATA + 0x2200, DATA + 0x2300);
                c.bus.write32(GAME_GP + 0x16c8, g);
                c.bus.write32(g, h);
                if seed % 4 != 0 {
                    c.bus.write32(h + 8 + 4 * (id & 0xff), x);
                    c.bus.write32(x + 0x1c, y);
                    let z = if seed % 7 == 0 {
                        0
                    } else {
                        ((model >> 2) << 8) | 1
                    };
                    c.bus.write32(y + 4 * (id >> 8), z);
                }
                for k in 0..0x30u32 {
                    c.bus.write32(model + 4 * k, r.f32(100.0));
                }
                c.bus.write32(GAME_GP + 0x2a90, OC414_SOUP);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = id as u64;
            },
        );
    }

    use crate::difftest::fakes;

    /// Base of the fixtures: above the fakes' pointer range
    /// (`DATA..DATA + 0x40000`), so a callee writing through a fake's result
    /// cannot hit a fixture.
    pub(crate) const OC429_FX: u32 = DATA + 0x8_0000;

    /// Gate word of this module's callee fakes.
    pub(crate) const OC429_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC429_MAGIC: u32 = 0x0c42_9a11;

    /// Result queues of the faked callees: `[head, len, values: u64 x 30]`,
    /// 0x100 bytes per callee, in the order of `OC429_CALLEES`.
    pub(crate) const OC429_QUEUES: u32 = OC429_CTRL + 0x100;

    pub(crate) fn oc429_queue_base(addr: u32) -> u32 {
        let i = OC429_CALLEES
            .iter()
            .position(|&a| a == addr)
            .expect("callee without a fake");
        OC429_QUEUES + 0x100 * i as u32
    }

    /// The faked callee `addr` returns `values` (64-bit, in v0) from its next
    /// calls, then its default hashed pointer.
    pub(crate) fn oc429_force(c: &mut Cpu, addr: u32, values: &[u64]) {
        let q = oc429_queue_base(addr);
        c.bus.write32(q, 0);
        c.bus.write32(q + 4, values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            c.bus.write64(q + 8 + 8 * i as u32, *v);
        }
    }

    /// Scrambled machine with the gated fakes on (and `pc` at `entry`);
    /// `calls` bounds the call index at which a fake may unwind.
    pub(crate) fn oc429_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc429_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.pc = entry;
        c.bus.write32(OC429_CTRL, OC429_MAGIC);
        r
    }

    pub(crate) fn oc429_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    pub(crate) fn oc429_gp(off: i32) -> u32 {
        (GAME_GP as i32 + off) as u32
    }

    pub(crate) const OC429_CALLEES: &[u32] = &[
        0x0010_f1c0,
        0x0014_4bc0,
        0x0014_dc80,
        0x0014_dd58,
        0x0015_4278,
        0x0015_4588,
        0x0015_6a60,
        0x0015_89b0,
        0x0015_8a50,
        0x0016_9340,
        0x0016_94a8,
        0x0016_94b8,
        0x0016_9cf8,
        0x0016_a458,
        0x0016_ad38,
        0x0026_d628,
        0x0026_e950,
        0x0027_2488,
        0x0027_29a8,
        0x0027_3938,
        0x0027_39b8,
        0x0027_43c8,
        0x0027_5718,
        0x0027_57f0,
        0x0027_5b98,
        0x0027_5cf8,
        0x0027_7450,
        0x0027_76e0,
        0x0027_7dd8,
        0x0027_97a0,
        0x0028_2338,
        0x0028_2bf0,
        0x0028_2c88,
        0x0028_2cb0,
        0x0028_3bb8,
        0x0028_3d28,
        0x0028_3d70,
        0x0030_9568,
        0x0031_7d70,
        0x0031_7e30,
        0x0031_7e50,
        0x0031_7e98,
        0x0031_8260,
        0x0031_82d8,
        0x0031_8448,
        0x0031_84e8,
        0x0031_9028,
        0x0031_90a8,
        0x0032_84b8,
        0x0032_8c20,
        0x0032_8f28,
        0x0035_81b8,
        0x003a_6c68,
        0x003a_6c98,
        0x003a_9800,
        0x003e_2580,
        0x003e_6448,
        0x0041_66f4,
        0x0041_aa88,
    ];

    /// `SSXApp_ApplyDisplayOptions`: each display mode (0, 1, 2 and the
    /// out-of-range 3), with the options record at `0x535610` random; the
    /// two graphics-manager vtable calls and the two BE calls are fakes (any
    /// of them can unwind). The record is 8-aligned in the game, so only the
    /// `ld`/`sd` copy loops run.
    #[test]
    fn ssx_app_apply_display_options_matches() {
        covered(
            ee::s05::f_00228c08__generated,
            ssx_app_apply_display_options,
            80,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x228c08, 4, 0x0022_8c08);
                let gfx = OC429_FX + 0x800;
                let vt = OC429_FX + 0x900;
                oc429_w(c, oc429_gp(0x2a90), gfx);
                oc429_w(c, gfx + 0x10d8, vt);
                c.bus.write16(vt + 0x140, (r.next_u32() as u16) & !3);
                oc429_w(c, vt + 0x144, fakes::FAKE);
                c.bus.write16(vt + 0x60, (r.next_u32() as u16) & !3);
                oc429_w(c, vt + 0x64, fakes::FAKE + 0x10);
                fill(c, &mut r, SSXAPP_OPTIONS, 0xa2);
                let w0 = r.next_u32() & !(3 << 20) | (seed % 4) << 20;
                oc429_w(c, SSXAPP_OPTIONS, w0);
            },
        );
    }

    /// `SSXApp_ParseCommandLine`: 1..5 arguments of every kind (`-track`,
    /// `-username`, `-password`, `-realfe`, `-nohud`, `-autolobby`, an unknown
    /// option, a plain word), options missing their value, the track path
    /// with and without a separator or an extension. The `cBXString`
    /// constructor / assignments and `strcasecmp` are fakes (`strcasecmp` with
    /// forced results: the test walks the argument list as the function does
    /// to queue them); the separator searches run for real.
    #[test]
    fn ssx_app_parse_command_line_matches() {
        covered(
            ee::s05::f_00227bf0__generated,
            ssx_app_parse_command_line,
            200,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x227bf0, 24, 0x0022_7bf0);
                let this = OC429_FX + 0x400;
                let argv = OC429_FX + 0x2000;
                let argc = 1 + (r.next_u32() % 5);
                // Kind per argument: 0 track, 1 username, 2 password, 3 realfe,
                // 4 nohud, 5 autolobby, 6 unknown option, 7 plain word.
                let kinds: Vec<u32> = (0..argc).map(|_| r.next_u32() % 8).collect();
                for i in 0..argc {
                    let s = OC429_FX + 0x2100 + 0x10 * i;
                    oc429_w(c, argv + 4 * i, s);
                    let dash = kinds[i as usize] != 7;
                    c.bus.write8(s, if dash { b'-' } else { b'a' });
                    c.bus.write8(s + 1, b'x');
                    c.bus.write8(s + 2, 0);
                }
                let mut strcmp: Vec<u64> = vec![];
                let mut i = 1;
                while i < argc {
                    let k = kinds[i as usize];
                    if k != 7 {
                        // strcasecmp: 1 for each option before the match, then 0.
                        for opt in 0..6u32 {
                            if opt == k {
                                strcmp.push(0);
                                break;
                            }
                            strcmp.push(1);
                        }
                        match k {
                            0 => {
                                i += 1;
                                if i >= argc {
                                    break;
                                }
                            }
                            1 | 2 => {
                                i += 1;
                                if i >= argc {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    i += 1;
                }
                oc429_force(c, 0x0041_aa88, &strcmp);
                // The track string object at the frame base (sp = SP - 0xc0):
                // its character data pointer (length word in front). The
                // `cBXString` search functions run for real on it; the
                // constructor is a fake, so the string is whatever is here.
                let text: &[u8] = [
                    &b"dir/trk.ext\0"[..],
                    &b"a\\b\\c.d\0"[..],
                    &b"c:foo\0"[..],
                    &b"plain\0"[..],
                    &b"x.y/z.w:v\0"[..],
                    &b".hidden\0"[..],
                ][(seed % 6) as usize];
                c.bus.write32(crate::difftest::SP - 0xc0, OC429_FX + 0x1000);
                oc429_w(c, OC429_FX + 0x1000 - 8, text.len() as u32 - 1);
                for (k, b) in text.iter().enumerate() {
                    c.bus.write8(OC429_FX + 0x1000 + k as u32, *b);
                }
                oc429_w(c, this + 0x64, r.next_u32() % 2);
                oc429_w(c, this + 0x60, r.next_u32());
                oc429_w(c, this + 0x74, r.next_u32());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = argc as u64;
                c.g[A2][0] = argv as u64;
            },
        );
    }

    pub(crate) use crate::game::ai::ai::tests::oc432_fake;
    pub(crate) use crate::game::ai::ai::tests::oc434_setup;
    pub(crate) use crate::game::ai::ai::tests::OC432_CTRL;
    pub(crate) use crate::game::ai::ai::tests::OC432_MAGIC;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_gp;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_run;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC433_FIX;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    oc432_fake!(
        fake_fe_locale_ctor,
        PREV_FE_LOCALE_CTOR,
        ee::s02::f_00195880__generated,
        0x0019_5880,
        |c: &mut Cpu| {
            c.g[V0][0] = (DATA + 0x5_2000) as u64;
        }
    );

    oc432_fake!(
        fake_fe_locale_load_common,
        PREV_FE_LOCALE_LOAD_COMMON,
        ee::s02::f_00195b70__generated,
        0x0019_5b70
    );

    /// `SSXApp_LoadLocale`: locale already there or created (the allocation
    /// and constructor are fakes), the options block (0x535610) with random
    /// words so every language value shows, the locale's virtual method 3 is
    /// a fake.
    #[test]
    fn ssx_app_load_locale_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            fake_mem_alloc_named,
            &PREV_MEM_ALLOC_NAMED,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00195880),
            fake_fe_locale_ctor,
            &PREV_FE_LOCALE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00195B70),
            fake_fe_locale_load_common,
            &PREV_FE_LOCALE_LOAD_COMMON,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            fake_get_be,
            &PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            fake_be_get_interface,
            &PREV_BE_GET_INTERFACE,
        );
        fakes::covered(
            ee::s05::f_00228a78__generated,
            ssx_app_load_locale,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x228a78, 8);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                let this = DATA + 0x5_0000;
                let locale = DATA + 0x5_2000;
                let vt = DATA + 0x5_3000;
                c.g[A0][0] = this as u64;
                c.bus
                    .write32(this + 0x8c, if r.next_u32() % 2 == 0 { 0 } else { locale });
                c.bus.write32(locale + 4, vt);
                c.bus.write16(vt + 0x18, (r.next_u32() % 16) as u16);
                c.bus.write32(vt + 0x1c, fakes::FAKE);
                for i in 0..(0x288 / 4) {
                    c.bus.write32(0x0053_5610 + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// `SSXApp_FlushTicks` against the generated body: the pad-queue global
    /// set (the flush call, sometimes unwinding) or clear.
    #[test]
    fn ssx_app_flush_ticks_matches() {
        oc433_run(
            ee::s05::f_00227e68__generated,
            ssx_app_flush_ticks,
            0x0022_7e68,
            1,
            40,
            &|c, _r, seed| {
                oc433_gp(c, -0x850, if seed % 3 == 0 { 0 } else { OC433_FIX });
            },
        );
    }

    /// `CrowdMan2d_ctor`: the real 2D renderer and animation constructors run (the texture
    /// file load fails: the file queue fake returns 0), the allocations are fakes.
    #[test]
    fn crowd_man2d_ctor_matches() {
        fakes::covered(
            ee::s05::f_00229180__generated,
            crowd_man2d_ctor,
            16,
            &|c, seed| {
                let _ = oc434_setup(c, seed, 0x229180, 6, 0x0022_9180);
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    pub(crate) use crate::game::main::all_in_one_main::tests::object_machine;

    #[test]
    fn ssxapp_ctor_matches() {
        check(
            ee::s05::f_00226830__generated,
            ssxapp_ctor,
            40,
            &|c, seed| {
                let mut r = object_machine(c, seed, 0x6830);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP.wrapping_add(0xda0), r.next_u32());
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn mc_manager_exists_wrapper_matches() {
        check(
            ee::s05::f_00227fa0__generated,
            mc_manager_exists_wrapper,
            30,
            &|c, seed| {
                let _ = machine(c, seed, 0x7fa0, 0);
            },
        );
    }

    #[test]
    fn ssxapp_v02_matches() {
        check(
            ee::s05::f_00228bb8__generated,
            ssxapp_v02,
            40,
            &|c, seed| {
                let _ = object_machine(c, seed, 0x8bb8);
                c.bus.write32(GAME_GP.wrapping_add(0x2a90), DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x10d8, DATA + 0x2000);
                c.bus.write16(DATA + 0x2000 + 0x3b8, (seed % 2) as u16);
                c.bus.write32(DATA + 0x2000 + 0x3bc, 0x0070_0000);
            },
        );
    }

    /// Gate word at `OC642_CTRL`; the word after it holds the test's flags.
    pub(crate) const OC642_CTRL: u32 = DATA + 0x7_2000;

    pub(crate) const OC642_MAGIC: u32 = 0x0642_0c0d;

    /// Flags: clear ScriptEngine during its DeInit (bit 0), clear GrMan
    /// during its Purge (bit 1).
    pub(crate) const OC642_FLAG_SCRIPT: u32 = 1;

    pub(crate) const OC642_FLAG_GRMAN: u32 = 2;

    /// Vtables of the three objects, and the fake targets their slots hold.
    pub(crate) const OC642_VT_SCRIPT: u32 = DATA + 0x2_0000;

    pub(crate) const OC642_VT_GRMAN: u32 = DATA + 0x2_0100;

    pub(crate) const OC642_VT_CONSOLE: u32 = DATA + 0x2_0200;

    pub(crate) const OC642_SLOT_SCRIPT_DTOR: u32 = FAKE + 0x100;

    pub(crate) const OC642_SLOT_GRMAN_DTOR: u32 = FAKE + 0x110;

    pub(crate) const OC642_SLOT_GRMAN_PURGE: u32 = FAKE + 0x120;

    pub(crate) const OC642_SLOT_CONSOLE_DTOR: u32 = FAKE + 0x130;

    /// A gated fake for a callee with an override slot: logs the call with
    /// the usual fake clobbers, or runs the body in `PREV` with the gate off.
    macro_rules! oc642_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC642_CTRL) != OC642_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    oc642_gated!(
        oc642_fake_operator_delete,
        OC642_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );

    oc642_gated!(
        oc642_fake_font_purge,
        OC642_PREV_FONT,
        0x0039_8438,
        ee::s13::f_00398438__generated
    );

    oc642_gated!(
        oc642_fake_pad_purge,
        OC642_PREV_PAD,
        0x0032_6a68,
        ee::s10::f_00326a68__generated
    );

    oc642_gated!(
        oc642_fake_delbigsync,
        OC642_PREV_BIG,
        0x003d_edc0,
        ee::s14::f_003dedc0__generated
    );

    oc642_gated!(
        oc642_fake_world_deinit,
        OC642_PREV_WORLD,
        0x002b_4b48,
        ee::s08::f_002b4b48__generated
    );

    oc642_gated!(
        oc642_fake_audio_deinit,
        OC642_PREV_AUDIO,
        0x0028_4c28,
        ee::s07::f_00284c28__generated
    );

    oc642_gated!(
        oc642_fake_be_purge,
        OC642_PREV_BE,
        0x0014_dd98,
        ee::s01::f_0014dd98__generated
    );

    oc642_gated!(
        oc642_fake_gm_purge,
        OC642_PREV_GM,
        0x0023_80e8,
        ee::s06::f_002380e8__generated
    );

    oc642_gated!(
        oc642_fake_async_dtor,
        OC642_PREV_ASYNC,
        0x001a_35e8,
        ee::s03::f_001a35e8__generated
    );

    pub(crate) static OC642_PREV_SCRIPT: AtomicUsize = AtomicUsize::new(0);

    /// ScriptEngine's DeInit: with the script flag it clears the global first,
    /// as the engine's own teardown would (so the dtor branch is skipped).
    pub(crate) fn oc642_fake_script_deinit(c: &mut Cpu) {
        if c.bus.read32(OC642_CTRL) != OC642_MAGIC {
            return fakes::fall_back(c, &OC642_PREV_SCRIPT, ee::s07::f_00278308__generated);
        }
        if c.bus.read32(OC642_CTRL + 4) & OC642_FLAG_SCRIPT != 0 {
            c.bus.write32(GAME_GP.wrapping_sub(0x84c), 0);
        }
        fakes::fake_call(c, 0x0027_8308);
    }

    /// The machine's miss hook: the GrMan vtable's Purge slot clears the
    /// global first when flagged (so the GrMan dtor branch is skipped).
    pub(crate) fn oc642_miss(c: &mut Cpu, target: u32) {
        if target == OC642_SLOT_GRMAN_PURGE && c.bus.read32(OC642_CTRL + 4) & OC642_FLAG_GRMAN != 0
        {
            c.bus.write32(GAME_GP.wrapping_sub(0x854), 0);
        }
        fakes::fake_call(c, target);
    }

    pub(crate) fn oc642_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc642_fake_operator_delete,
            &OC642_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00398438),
            oc642_fake_font_purge,
            &OC642_PREV_FONT,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00326A68),
            oc642_fake_pad_purge,
            &OC642_PREV_PAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278308),
            oc642_fake_script_deinit,
            &OC642_PREV_SCRIPT,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DEDC0),
            oc642_fake_delbigsync,
            &OC642_PREV_BIG,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B4B48),
            oc642_fake_world_deinit,
            &OC642_PREV_WORLD,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00284C28),
            oc642_fake_audio_deinit,
            &OC642_PREV_AUDIO,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD98),
            oc642_fake_be_purge,
            &OC642_PREV_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002380E8),
            oc642_fake_gm_purge,
            &OC642_PREV_GM,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A35E8),
            oc642_fake_async_dtor,
            &OC642_PREV_ASYNC,
        );
    }

    /// A pointer field: null on about a quarter of the seeds.
    pub(crate) fn oc642_opt(r: &mut Rng, v: u32) -> u32 {
        if r.next_u32() % 4 == 0 {
            0
        } else {
            v
        }
    }

    /// One vtable slot: a random 16-bit this-adjustment at `at`, the target at `at + 4`.
    pub(crate) fn oc642_slot(c: &mut Cpu, r: &mut Rng, at: u32, target: u32) {
        c.bus.write16(at, (r.next_u32() as u16) & 0xfffc);
        c.bus.write32(at + 4, target);
    }

    /// The app object at DATA (pointer fields null or not per seed), the
    /// globals at gp, and the three objects whose vtables hold fake slots.
    pub(crate) fn oc642_fixture(c: &mut Cpu, r: &mut Rng) {
        let app = DATA;
        fill(c, r, app, 0x60);
        let engine = DATA + 0x8000;
        let grman = DATA + 0x4000;
        let console = DATA + 0xa000;
        let big = |k: u32| DATA + 0x1_0000 + 0x40 * k;
        for (off, k) in [
            (0x78u32, 1u32),
            (0x88, 2),
            (0xb8, 3),
            (0xbc, 4),
            (0xc0, 5),
            (0xd4, 6),
            (0xd8, 7),
            (0xdc, 8),
            (0xe0, 9),
            (0xe4, 10),
            (0xe8, 11),
            (0x118, 12),
            (0x11c, 13),
        ] {
            let v = oc642_opt(r, big(k));
            c.bus.write32(app + off, v);
        }
        for i in 0..11u32 {
            let v = oc642_opt(r, big(16 + i));
            c.bus.write32(app + 0xec + 4 * i, v);
        }
        let v = oc642_opt(r, console);
        c.bus.write32(app + 0x9c, v);
        let v = oc642_opt(r, engine);
        c.bus.write32(GAME_GP.wrapping_sub(0x84c), v);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), app);
        c.bus.write32(GAME_GP.wrapping_sub(0x850), DATA + 0x6000);
        c.bus.write32(GAME_GP.wrapping_sub(0x854), grman);

        c.bus.write32(engine + 0x2a8, OC642_VT_SCRIPT);
        c.bus.write32(grman + 0x10d8, OC642_VT_GRMAN);
        c.bus.write32(console + 0x34, OC642_VT_CONSOLE);
        oc642_slot(c, r, OC642_VT_SCRIPT + 8, OC642_SLOT_SCRIPT_DTOR);
        oc642_slot(c, r, OC642_VT_GRMAN + 8, OC642_SLOT_GRMAN_DTOR);
        oc642_slot(c, r, OC642_VT_GRMAN + 0x18, OC642_SLOT_GRMAN_PURGE);
        oc642_slot(c, r, OC642_VT_CONSOLE + 8, OC642_SLOT_CONSOLE_DTOR);
    }

    #[test]
    fn ssxapp_shutdown_matches() {
        oc642_install();
        fakes::covered(
            ee::s05::f_00227fc0__generated,
            ssxapp_shutdown,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_7fc0, 12);
                c.hooks.miss = oc642_miss;
                c.bus.write32(OC642_CTRL, OC642_MAGIC);
                let flags = r.next_u32() & 3;
                c.bus.write32(OC642_CTRL + 4, flags);
                c.g[A0][0] = DATA as u64;
                oc642_fixture(c, &mut r);
            },
        );
    }

    use crate::difftest::fakes::fake;
    use crate::difftest::RA as DT_RA;
    use ssx3_ee_rt::funcs::register_functions;
    use ssx3_ee_rt::funcs::FnEntry;

    pub(crate) use crate::game::be::beintstat::tests::oc673_machine;
    pub(crate) use crate::game::be::beintstat::tests::oc673_result_at;

    /// Gate of the callee fakes (otherwise the body they replaced runs).
    pub(crate) const OC671_CTRL: u32 = DATA + 0x000e_4000;

    pub(crate) const OC671_MAGIC: u32 = 0x494d_5036;

    /// Call index at which the `operator_new_named` fake returns 0.
    pub(crate) const OC671_ZERO_AT: u32 = DATA + 0x000e_4004;

    pub(crate) const OC671_PATH: u32 = DATA + 0x2000;

    /// A gated fake for one callee slot: logs the call and returns.
    macro_rules! oc671_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC671_CTRL) != OC671_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let n = c.bus.read32(fakes::LOG_COUNT);
                fakes::fake_call(c, $addr);
                if $addr == 0x0031_7e30 && n == c.bus.read32(OC671_ZERO_AT) {
                    c.g[V0][0] = 0;
                }
            }
        };
    }

    oc671_fake!(
        oc671_f_new,
        OC671_PREV_NEW,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );

    oc671_fake!(
        oc671_f_alloc,
        OC671_PREV_ALLOC,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );

    oc671_fake!(
        oc671_f_free,
        OC671_PREV_FREE,
        0x0031_7e98,
        ee::s10::f_00317e98__generated
    );

    oc671_fake!(
        oc671_f_init,
        OC671_PREV_INIT,
        0x0032_0ca0,
        ee::s10::f_00320ca0__generated
    );

    oc671_fake!(
        oc671_f_parser,
        OC671_PREV_PARSER,
        0x0032_0db0,
        ee::s10::f_00320db0__generated
    );

    oc671_fake!(
        oc671_f_compile,
        OC671_PREV_COMPILE,
        0x0032_0e18,
        ee::s10::f_00320e18__generated
    );

    oc671_fake!(
        oc671_f_purge,
        OC671_PREV_PURGE,
        0x0032_0f10,
        ee::s10::f_00320f10__generated
    );

    oc671_fake!(
        oc671_f_process,
        OC671_PREV_PROCESS,
        0x0032_6078,
        ee::s10::f_00326078__generated
    );

    pub(crate) fn oc671_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc671_f_new,
            &OC671_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc671_f_alloc,
            &OC671_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc671_f_free,
            &OC671_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320CA0),
            oc671_f_init,
            &OC671_PREV_INIT,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320DB0),
            oc671_f_parser,
            &OC671_PREV_PARSER,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320E18),
            oc671_f_compile,
            &OC671_PREV_COMPILE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320F10),
            oc671_f_purge,
            &OC671_PREV_PURGE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00326078),
            oc671_f_process,
            &OC671_PREV_PROCESS,
        );
    }

    /// `InputMap_LoadFromFile(self, path)`: loads the input map file (205
    /// bindings) through the preprocessor, allocates and compiles the map.
    /// `InputPreproc_Run` runs for real; the file reader and every allocator
    /// and map call is a gated fake. Fixture: a path with or without a
    /// directory part, a zero text pointer (no free) on some seeds, and
    /// unwinds at random calls.
    #[test]
    fn oc671_input_map_load_from_file_matches() {
        oc671_install_fakes();
        check(
            ee::s05::f_00226b60__generated,
            oc671_input_map_load_from_file,
            90,
            &|c, seed| {
                let _ = fakes::machine(c, seed, 0x0671_0000, 215);
                c.pc = 0x0022_6b60;
                c.bus.write32(OC671_CTRL, OC671_MAGIC);
                // The 3rd allocation call of the preprocessor is the text copy
                // (new #0, ProcessFile #1, new #2); zero it on every 3rd seed.
                let zero = if seed % 3 == 0 { 2 } else { u32::MAX };
                c.bus.write32(OC671_ZERO_AT, zero);
                let path: &[u8] = if seed % 2 == 0 {
                    b"data/input/ssx.map\0"
                } else {
                    b"ssx.map\0"
                };
                c.bus.write_bytes(OC671_PATH, path);
                c.g[A0][0] = sx(DATA + 0x3000);
                c.g[A1][0] = sx(OC671_PATH);
            },
        );
    }

    /// Environment switch of the child process (see the test).
    pub(crate) const OC672_CHILD_ENV: &str = "SSX3_OC672_CHILD";

    /// Stub controls: the BXFILE_exists answer bits (+0), its call counter
    /// (+4), the strcmp answer (+8).
    pub(crate) const OC672_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC672_SELF: u32 = DATA + 0x1000;

    pub(crate) const OC672_TRACK: u32 = DATA + 0x2000;

    /// `BXFILE_exists`: bit n of the control word answers call n.
    pub(crate) fn oc672_stub_exists(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        let n = c.bus.read32(OC672_CTRL + 4);
        c.bus.write32(OC672_CTRL + 4, n + 1);
        fake(c, 0x0031_7f38, ret);
        if c.pc == ret {
            let mask = c.bus.read32(OC672_CTRL);
            c.g[V0][0] = ((mask >> (n & 31)) & 1) as u64;
        }
    }

    /// `strcmp`: 0 (same track) or 1.
    pub(crate) fn oc672_stub_strcmp(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        fake(c, 0x0041_65a8, ret);
        if c.pc == ret {
            c.g[V0][0] = (c.bus.read32(OC672_CTRL + 8) != 0) as u64;
        }
    }

    pub(crate) static OC672_STUBS: [FnEntry; 2] = [
        FnEntry {
            start: 0x0031_7f38,
            end: 0x0031_7f3c,
            f: oc672_stub_exists,
        },
        FnEntry {
            start: 0x0041_65a8,
            end: 0x0041_65ac,
            f: oc672_stub_strcmp,
        },
    ];

    /// A direct callee replaced by a logging fake (child process only).
    pub(crate) fn oc672_fake_net(c: &mut Cpu) {
        let ra = c.g[RA][0] as u32;
        fake(c, 0x0022_8ee0, ra)
    }

    pub(crate) fn oc672_fake_display(c: &mut Cpu) {
        let ra = c.g[RA][0] as u32;
        fake(c, 0x0022_8c08, ra)
    }

    pub(crate) fn oc672_fake_locale(c: &mut Cpu) {
        let ra = c.g[RA][0] as u32;
        fake(c, 0x0022_8a78, ra)
    }

    /// Child process only: a function table with just the two answer stubs,
    /// and fakes in the slots of the directly called ported functions.
    pub(crate) fn oc672_install() {
        if std::env::var_os(OC672_CHILD_ENV).is_none() {
            return;
        }
        register_functions(&OC672_STUBS);
        // SAFETY: the child process runs this one test, single-threaded.
        unsafe {
            ee::s05::OVERRIDE_00228EE0 = oc672_fake_net;
            ee::s05::OVERRIDE_00228C08 = oc672_fake_display;
            ee::s05::OVERRIDE_00228A78 = oc672_fake_locale;
        }
    }

    pub(crate) fn oc672_setup(c: &mut Cpu, seed: u32) {
        oc672_install();
        let mut r = machine(c, seed, 0x22_82d0, 90);
        c.pc = 0x0022_82d0;
        c.g[RA][0] = DT_RA as u64;
        c.g[A0][0] = OC672_SELF as u64;
        // Which of the four big files exist; same or different track.
        c.bus.write32(OC672_CTRL, r.next_u32());
        c.bus.write32(OC672_CTRL + 4, 0);
        c.bus
            .write32(OC672_CTRL + 8, (r.next_u32() & 1) | (seed & 1));
        c.bus
            .write32(OC672_SELF + BOOT_SELF_CMDLINE_TRACK, OC672_TRACK);
        c.bus
            .write32(OC672_SELF + BOOT_SELF_INPUT_MAPS, r.next_u32());
        for off in [0x728u32, 0x724] {
            c.bus.write32(GAME_GP.wrapping_sub(off), r.next_u32());
        }
        // The BE option snapshot (0x288 bytes) with random volumes and modes.
        fill(c, &mut r, BOOT_HI_BEOPT + BOOT_BEOPT_OFF, 0x288 / 4);
    }

    /// `cSSXApp_initload` against the generated body: the one-time game load.
    /// All ~70 callees are logging fakes (the child process's function table
    /// holds only the `BXFILE_exists` and `strcmp` answer stubs; the ported
    /// direct callees get fakes in their slots), so the diff compares every
    /// call's registers, the BE option copies and the unwinds at random call
    /// indices. The option struct sits at a constant aligned address, so the
    /// unaligned ldl/ldr copy paths are unreachable.
    #[test]
    fn ssxapp_boot_load_matches() {
        if std::env::var_os(OC672_CHILD_ENV).is_some() {
            return covered(
                ee::s05::f_002282d0__generated,
                ssxapp_boot_load,
                160,
                &oc672_setup,
            );
        }
        let name = format!(
            "{}::ssxapp_boot_load_matches",
            module_path!().split_once("::").map_or("", |p| p.1)
        );
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([name.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(OC672_CHILD_ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child run of {name}:\n{text}"
        );
    }

    /// `SSXApp_Init` against the generated body: the boot sequence with the
    /// graphics manager (a fixture whose vtable slots are fake methods), the
    /// pad queue, light and bezier managers (the real, bounded constructors
    /// run), the pad records, the command line, the boot shape and tint, and
    /// unwinds in each call.
    #[test]
    fn ssx_app_init_matches_2() {
        fakes::covered(
            ee::s05::f_00226900__generated,
            ssx_app_init_3,
            40,
            &|c, seed| {
                let mut r = oc673_machine(c, seed, 0x226900, 50, 0x0022_6900);
                // Above the reach of the real constructors' writes through the fake
                // allocator's pointers (DATA .. DATA + 0xe8000).
                let this = DATA + 0xf_2000;
                let gm = DATA + 0xf_0000;
                let g = DATA + 0xf_1000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() % 4);
                c.g[A2][0] = sx(DATA + 0xf_3000);
                c.bus.write32(gm + 0x10d8, g);
                for off in [0x14, 0x164, 0x174] {
                    c.bus.write32(g + off, FAKE);
                }
                for off in [0x10, 0x160, 0x170] {
                    c.bus.write16(g + off, ((r.next_u32() % 64) * 4) as u16);
                }
                oc673_result_at(c, 0x0037_5a08, gm);
            },
        );
    }

    use crate::difftest;
    use crate::difftest::fresh;

    /// Gate word of this test's callee fakes and their mailbox (in the machine's own RAM,
    /// outside the random arena).
    pub(crate) const OC1468_CTRL: u32 = DATA + 0x0f_0000;

    pub(crate) const OC1468_MAGIC: u32 = 0x1468_0b00;

    /// The random arena: words that are pointers into it, floats, small integers or zero.
    pub(crate) const OC1468_ARENA: u32 = DATA;

    pub(crate) const OC1468_ARENA_WORDS: u32 = 0x1_0000;

    pub(crate) const OC1468_SELF: u32 = DATA + 0x1000;

    pub(crate) const OC1468_GFX: u32 = DATA + 0x4000;

    pub(crate) const OC1468_TOP: u32 = DATA + 0x6000;

    pub(crate) const OC1468_X: u32 = DATA + 0x6400;

    pub(crate) const OC1468_VIEWS: u32 = DATA + 0x7000;

    pub(crate) const OC1468_HUD: u32 = DATA + 0x7400;

    pub(crate) const OC1468_TARGET: u32 = DATA + 0x7800;

    pub(crate) const OC1468_CAMS: u32 = DATA + 0x8000;

    /// Every callee of `GameModule_Render` with an override slot (all are faked: the real
    /// draw passes run away on random data).
    pub(crate) const OC1468_ADDRS: [u32; 23] = [
        0x0022_e8b8,
        0x0022_e920,
        0x0023_1d60,
        0x0036_a1e8,
        0x0022_c830,
        0x0031_b6c8,
        0x0022_add8,
        0x0023_0640,
        0x0022_9ba8,
        0x0034_4138,
        0x0035_7c50,
        0x0022_c078,
        0x0022_c708,
        0x0022_c790,
        0x0022_c620,
        0x002c_c098,
        0x002c_c350,
        0x0039_7df8,
        0x0030_b4b8,
        0x0027_89c0,
        0x001f_31e0,
        0x0028_5bf8,
        0x002b_5d78,
    ];

    pub(crate) fn oc1468_idx(addr: u32) -> u32 {
        OC1468_ADDRS
            .iter()
            .position(|&a| a == addr)
            .expect("fake address") as u32
    }

    /// The result the fixture forces for callee `addr` (a word in the mailbox).
    pub(crate) fn oc1468_result_word(addr: u32) -> u32 {
        OC1468_CTRL + 0x100 + 4 * oc1468_idx(addr)
    }

    /// A slot fake: logs the call like `fakes::fake_call`, then returns the forced result;
    /// the matrix inverse also writes a random matrix to its output (so the view box grows).
    pub(crate) fn oc1468_slot_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let a0 = c.g[A0][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let res = c.bus.read32(oc1468_result_word(addr));
        c.g[V0][0] = res as u64;
        if addr == 0x0031_b6c8 {
            for i in 0..16u32 {
                let h = fakes::mix((a0 as u64) << 8 ^ (i as u64) ^ (n as u64) << 40);
                let f = ((h >> 8) as u32 % 2000) as f32 / 100.0 - 10.0;
                c.bus.write32(a0 + 4 * i, f.to_bits());
            }
        }
    }

    macro_rules! oc1468_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1468_CTRL) != OC1468_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1468_slot_call(c, $addr);
                }
            )*
            fn oc1468_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1468_fakes!(
        (
            oc1468_f_prefetch,
            OC1468_P_PREFETCH,
            ee::s05::OVERRIDE_0022E8B8,
            ee::s05::f_0022e8b8__generated,
            0x0022_e8b8
        ),
        (
            oc1468_f_ready,
            OC1468_P_READY,
            ee::s05::OVERRIDE_0022E920,
            ee::s05::f_0022e920__generated,
            0x0022_e920
        ),
        (
            oc1468_f_231d60,
            OC1468_P_231D60,
            ee::s06::OVERRIDE_00231D60,
            ee::s06::f_00231d60__generated,
            0x0023_1d60
        ),
        (
            oc1468_f_36a1e8,
            OC1468_P_36A1E8,
            ee::s12::OVERRIDE_0036A1E8,
            ee::s12::f_0036a1e8__generated,
            0x0036_a1e8
        ),
        (
            oc1468_f_ui,
            OC1468_P_UI,
            ee::s05::OVERRIDE_0022C830,
            ee::s05::f_0022c830__generated,
            0x0022_c830
        ),
        (
            oc1468_f_invert,
            OC1468_P_INVERT,
            ee::s10::OVERRIDE_0031B6C8,
            ee::s10::f_0031b6c8__generated,
            0x0031_b6c8
        ),
        (
            oc1468_f_gather,
            OC1468_P_GATHER,
            ee::s05::OVERRIDE_0022ADD8,
            ee::s05::f_0022add8__generated,
            0x0022_add8
        ),
        (
            oc1468_f_230640,
            OC1468_P_230640,
            ee::s06::OVERRIDE_00230640,
            ee::s06::f_00230640__generated,
            0x0023_0640
        ),
        (
            oc1468_f_crowd,
            OC1468_P_CROWD,
            ee::s05::OVERRIDE_00229BA8,
            ee::s05::f_00229ba8__generated,
            0x0022_9ba8
        ),
        (
            oc1468_f_344138,
            OC1468_P_344138,
            ee::s11::OVERRIDE_00344138,
            ee::s11::f_00344138__generated,
            0x0034_4138
        ),
        (
            oc1468_f_357c50,
            OC1468_P_357C50,
            ee::s11::OVERRIDE_00357C50,
            ee::s11::f_00357c50__generated,
            0x0035_7c50
        ),
        (
            oc1468_f_models,
            OC1468_P_MODELS,
            ee::s05::OVERRIDE_0022C078,
            ee::s05::f_0022c078__generated,
            0x0022_c078
        ),
        (
            oc1468_f_fog,
            OC1468_P_FOG,
            ee::s05::OVERRIDE_0022C708,
            ee::s05::f_0022c708__generated,
            0x0022_c708
        ),
        (
            oc1468_f_flares,
            OC1468_P_FLARES,
            ee::s05::OVERRIDE_0022C790,
            ee::s05::f_0022c790__generated,
            0x0022_c790
        ),
        (
            oc1468_f_patches,
            OC1468_P_PATCHES,
            ee::s05::OVERRIDE_0022C620,
            ee::s05::f_0022c620__generated,
            0x0022_c620
        ),
        (
            oc1468_f_2cc098,
            OC1468_P_2CC098,
            ee::s09::OVERRIDE_002CC098,
            ee::s09::f_002cc098__generated,
            0x002c_c098
        ),
        (
            oc1468_f_2cc350,
            OC1468_P_2CC350,
            ee::s09::OVERRIDE_002CC350,
            ee::s09::f_002cc350__generated,
            0x002c_c350
        ),
        (
            oc1468_f_397df8,
            OC1468_P_397DF8,
            ee::s13::OVERRIDE_00397DF8,
            ee::s13::f_00397df8__generated,
            0x0039_7df8
        ),
        (
            oc1468_f_30b4b8,
            OC1468_P_30B4B8,
            ee::s10::OVERRIDE_0030B4B8,
            ee::s10::f_0030b4b8__generated,
            0x0030_b4b8
        ),
        (
            oc1468_f_2789c0,
            OC1468_P_2789C0,
            ee::s07::OVERRIDE_002789C0,
            ee::s07::f_002789c0__generated,
            0x0027_89c0
        ),
        (
            oc1468_f_1f31e0,
            OC1468_P_1F31E0,
            ee::s04::OVERRIDE_001F31E0,
            ee::s04::f_001f31e0__generated,
            0x001f_31e0
        ),
        (
            oc1468_f_285bf8,
            OC1468_P_285BF8,
            ee::s07::OVERRIDE_00285BF8,
            ee::s07::f_00285bf8__generated,
            0x0028_5bf8
        ),
        (
            oc1468_f_2b5d78,
            OC1468_P_2B5D78,
            ee::s08::OVERRIDE_002B5D78,
            ee::s08::f_002b5d78__generated,
            0x002b_5d78
        ),
    );

    /// The miss hook (the vtable and function-pointer calls): logs like `fakes::fake_call`;
    /// the fixture's mask word zeroes the result of some of the first 32 calls.
    pub(crate) fn oc1468_miss(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc == ra && n < 32 && c.bus.read32(OC1468_CTRL + 4) >> n & 1 != 0 {
            c.g[V0][0] = 0;
        }
    }

    /// A game object `cGame` with its graphics object, view manager, camera array, HUD and
    /// target, all in a random arena (every pointer chain stays inside it); the seed picks
    /// the view count (0-3), the early exits (not ready, loader not ready, UI-only frame),
    /// the menu branch, the HUD-off branch and the unwinding callee.
    pub(crate) fn oc1468_setup(c: &mut Cpu, seed: u32) {
        oc1468_install_fakes();
        let mut r = fakes::machine(c, seed, 0x1468_0b00, 90);
        c.hooks.miss = oc1468_miss;
        difftest::elf_image(c);
        for i in 0..OC1468_ARENA_WORDS {
            let w = match r.next_u32() % 10 {
                0..=5 => OC1468_ARENA + (r.next_u32() & 0x3_fff0),
                6 | 7 => r.f32(300.0),
                8 => r.next_u32() % 9,
                _ => 0,
            };
            c.bus.write32(OC1468_ARENA + 4 * i, w);
        }
        for a in 0..0x200u32 {
            c.bus.write32(OC1468_CTRL + 4 * a, 0);
        }
        c.bus.write32(OC1468_CTRL, OC1468_MAGIC);
        // Zeroed results of the first calls: sparse, and the first (isReady) only now and then.
        let mut mask = r.next_u32() & r.next_u32() & r.next_u32();
        if seed % 17 != 16 {
            mask &= !1;
        }
        c.bus.write32(OC1468_CTRL + 4, mask);
        c.bus
            .write32(oc1468_result_word(0x0022_e920), seed >> 1 & 1);
        c.bus
            .write32(oc1468_result_word(0x0023_1d60), (seed % 13 == 12) as u32);
        c.bus
            .write32(oc1468_result_word(0x002c_c098), seed >> 3 & 1);
        // Globals: the graphics object, the 004a28a4 object and the three plain globals.
        c.bus.write32(GAME_GP - 0x854, OC1468_GFX);
        c.bus.write32(GAME_GP - 0x84c, OC1468_X);
        c.bus.write32(GAME_GP + 0x1450, seed >> 2 & 1);
        c.bus.write32(GAME_GP + 0x410, OC1468_ARENA + 0x1_2340);
        c.bus.write32(GAME_GP + 0x21e4, OC1468_ARENA + 0x2_3450);
        // 004a28a4 + 0x504: a UI-only frame for some seeds.
        c.bus
            .write32(OC1468_X + 0x504, ((seed / 4) % 3 == 2) as u32);
        // The graphics object: vtable, render-state block, layer indices.
        c.bus.write32(OC1468_GFX + 0x10d8, OC1468_ARENA + 0x2_0000);
        c.bus.write32(OC1468_GFX + 0xe84, OC1468_TOP);
        c.bus.write32(OC1468_GFX + 0x24, r.next_u32() % 4);
        c.bus.write32(OC1468_GFX + 0x44, r.next_u32() % 4);
        // The game object.
        let s = OC1468_SELF;
        c.bus.write32(s + 0x84, OC1468_VIEWS);
        c.bus.write32(s + 0x1c, OC1468_TARGET);
        c.bus.write32(s + 0x24, OC1468_HUD);
        c.bus.write32(s + 0x9c, (seed >> 2 & 1) ^ 1);
        c.bus.write32(OC1468_VIEWS + 0x10, seed % 4);
        // Three cameras at views+4..+0x10 (views+0x10 is the view count: a fourth
        // pointer there made the loop endless).
        for i in 0..3u32 {
            c.bus
                .write32(OC1468_VIEWS + 4 + 4 * i, OC1468_CAMS + 0x80 * i);
            // The camera: fov, near and far clip, then the view matrix at +0x40.
            for k in 0..0x20u32 {
                let f = match k {
                    0 => 0.9f32,
                    1 => 0.5 + (r.next_u32() % 8) as f32,
                    2 => 100.0 + (r.next_u32() % 900) as f32,
                    _ => ((r.next_u32() % 2000) as f32 - 1000.0) / 100.0,
                };
                c.bus.write32(OC1468_CAMS + 0x80 * i + 4 * k, f.to_bits());
            }
        }
        for k in 0..0x10u32 {
            let f = (r.next_u32() % 400) as f32 / 100.0;
            c.bus.write32(OC1468_HUD + 4 * k, f.to_bits());
        }
        c.g[A0][0] = s as u64;
        c.pc = 0x0022_b008;
    }

    /// `GameModule_Render` against the generated body: random arenas, every callee faked.
    #[test]
    fn game_module_render_matches() {
        crate::difftest::check(
            ee::s05::f_0022b008__generated,
            game_module_render,
            48,
            &|c, seed| oc1468_setup(c, seed),
        );
    }

    /// The seeds reach the early exits, the view loop (0-3 views) and the tail: several
    /// distinct cycle totals, some normal returns and some long runs.
    #[test]
    fn game_module_render_paths() {
        let mut seen = std::collections::BTreeSet::new();
        let (mut returned, mut long) = (0, 0);
        for seed in 0..48 {
            let mut c = fresh();
            oc1468_setup(&mut c, seed);
            let before = c.cycles;
            difftest::run_generated(&mut c, ee::s05::f_0022b008__generated);
            seen.insert(c.cycles - before);
            returned += (c.pc == difftest::RA) as u32;
            long += (c.cycles - before > 1500) as u32;
        }
        assert!(seen.len() >= 8, "paths {seen:?}");
        assert!(returned >= 8, "returned {returned}");
        assert!(long >= 4, "long runs {long}");
    }
}
