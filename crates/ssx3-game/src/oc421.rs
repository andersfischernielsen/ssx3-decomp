//! Staging module oc421 (wave wave219): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc421`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0022_ebc8, game_module_load, "GameModule_Load")];

// ---- GameModule_Load (0x22ebc8) --------------------------------------------

/// Entry address of `cGame_load`.
const GAME_LOAD_ENTRY: u32 = 0x0022_ebc8;

// Callees (all reached through the function table unless noted).
const LOAD_PROGRESS_BEGIN: u32 = 0x0031_7530;
const LOAD_PROGRESS_SET: u32 = 0x0031_7550;
const OPERATOR_NEW: u32 = 0x0031_7d70;
const BE_GET_BE: u32 = 0x0014_dc80;
const BE_GET_INTERFACE: u32 = 0x0014_dd58;

// `cGame` fields (the object is `s1` throughout).
const GAME_WORLD: u32 = 0x10;
const GAME_AI: u32 = 0x0c;
const GAME_BX_NASH: u32 = 0x20;
const GAME_DEBUG_FONT: u32 = 0x24;
const GAME_REPLAY: u32 = 0x28;
const GAME_FX_LOADER: u32 = 0x2c;
const GAME_WSCRIPT_MAN: u32 = 0x30;
const GAME_PLAYBACK_MAN: u32 = 0x34;
const GAME_CROWD_MAN_2D: u32 = 0x38;
const GAME_CROWD_MAN_3D: u32 = 0x3c;
const GAME_MULTI_PARTICLE_MAN: u32 = 0x40;
const GAME_UI: u32 = 0x48;
const GAME_AI_VFX_MAN: u32 = 0x50;
const GAME_VFX_MAN: u32 = 0x54;
const GAME_STATS_MAN: u32 = 0x58;
const GAME_TRANSITION_MAN: u32 = 0x5c;
const GAME_CUTSCENE_TRANSITION_MAN: u32 = 0x64;
const GAME_SPLASH_MAN: u32 = 0x68;
const GAME_FLAG_MANAGER: u32 = 0x70;
const GAME_SNOW_FALL: u32 = 0x74;
const GAME_INPUT_SYNC: u32 = 0x7c;
const GAME_VIEW_MAN: u32 = 0x84;
const GAME_INSTANCE_MAN: u32 = 0x88;
const GAME_WORLD_LIGHT_MAN: u32 = 0x90;
const GAME_OCCLUSION_MAN: u32 = 0x8c;
const GAME_LOAD_INDICATOR: u32 = 0x94;
const GAME_STREAM_PAUSE: u32 = 0x98;
const GAME_IS_STREAM_PAUSED: u32 = 0x9c;
const GAME_FLUSH_STREAMER: u32 = 0xa0;
const GAME_MENU_SYS: u32 = 0xa4;
const GAME_TRACK_BIG_FILE: u32 = 0xa8;
const GAME_UPDATE_TICK: u32 = 0xac;
const GAME_BOX_TEXTURE_ID: u32 = 0x220;

/// `r` as the 32-bit address it holds.
#[inline(always)]
fn gml_r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// `gp + off`.
#[inline(always)]
fn gml_gp(c: &Cpu, off: u32) -> u32 {
    gml_r(c, GP).wrapping_add(off)
}

/// `gp - off`.
#[inline(always)]
fn gml_gp_sub(c: &Cpu, off: u32) -> u32 {
    gml_r(c, GP).wrapping_sub(off)
}

/// `reg = sign_extend(value)`.
#[inline(always)]
fn gml_set(c: &mut Cpu, reg: usize, value: u32) {
    c.g[reg][0] = sx(value);
}

/// `operator new(size, name, flags, 0)` (`cMemMan_alloc`): result in `v0`.
fn gml_new(c: &mut Cpu, size: u32, name: u32, flags: u32, ret: u32) -> Flow {
    c.g[A0][0] = size as u64;
    gml_set(c, A1, name);
    c.g[A2][0] = flags as u64;
    c.g[A3][0] = 0;
    call_ptr(c, OPERATOR_NEW, ret)
}

/// `void cGame_load(cGame* self)` (0x22ebc8, "GameModule_Load"): builds every
/// manager of a race (stream pause, occlusion, world, lights, script, crowds,
/// particles, instances, the track and replay, the AI and playback, the view
/// manager, menus, transitions, flags, snow, audio, the UI engine with its HUD
/// state), then plays the replay set-up or loads the first state (10) and
/// waits for it. The progress bar is advanced after each group (13.2, 43.5,
/// 55, 93, 99). Returns nothing; `s1` is the game object throughout.
///
/// Entered at any other `pc` (a resume after a thread unwind), it runs the
/// generated body, which dispatches on `pc` itself.
pub fn game_module_load(c: &mut Cpu) {
    if c.pc != GAME_LOAD_ENTRY {
        return game_module_load_resume(c);
    }
    guest(c, game_module_load_body);
}

/// The generated body: every re-entry label of `cGame_load` lives there.
fn game_module_load_resume(c: &mut Cpu) {
    ee::s05::f_0022ebc8__generated(c)
}

fn game_module_load_body(c: &mut Cpu) -> Flow {
    // 0x22ebc8: frame, saves, first progress steps, locale, script engine.
    c.cycles += 52;
    let frame = Frame::open(c, 0xb0);
    c.g[V0][0] = c.lw(gml_gp(c, 0x2a74));
    frame.save(c, S1, 0x90);
    c.g[S1][0] = c.g[A0][0];
    c.lwc1(12, gml_gp_sub(c, 0x51c0));
    frame.save(c, S0, 0xa0);
    gml_set(c, A0, gml_r(c, V0).wrapping_add(0x38));
    frame.save(c, S2, 0x80);
    gml_set(c, S0, gml_r(c, S1).wrapping_add(GAME_BX_NASH));
    frame.save(c, S3, 0x70);
    gml_set(c, S2, gml_gp(c, 0x2898));
    frame.save(c, S5, 0x50);
    c.g[S3][0] = 0x530000;
    frame.save(c, S6, 0x40);
    gml_set(c, S5, gml_r(c, S1).wrapping_add(GAME_TRANSITION_MAN));
    frame.save(c, S7, 0x30);
    gml_set(c, S6, gml_r(c, S1).wrapping_add(GAME_SPLASH_MAN));
    frame.save(c, FP, 0x20);
    c.g[S7][0] = 0x480000;
    frame.save(c, S4, 0x60);
    frame.save_ra(c, 0x10);
    c.g[FP][0] = 0x480000;
    call_ptr(c, LOAD_PROGRESS_BEGIN, 0x0022_ec24)?; // (appman + 0x38, 13.2)
    c.g[A0][0] = c.lw(gml_gp(c, 0x2a74));
    c.g[AT][0] = 0x422e_0000;
    c.mtc1(1, 12);
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x38));
    call_ptr(c, LOAD_PROGRESS_SET, 0x0022_ec38)?; // 43.5
    c.g[V0][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[A0][0] = c.lw(gml_r(c, V0).wrapping_add(0x8c));
    call_ptr(c, 0x0019_5de0, 0x0022_ec44)?; // locale
                                            // Graphics manager virtual (slot at +0x160/+0x164 of its vtable).
    c.g[A2][0] = c.lw(gml_gp_sub(c, 0x854));
    c.g[A1][0] = 0;
    c.g[V0][0] = c.lw(gml_r(c, A2).wrapping_add(0x10d8));
    c.g[A0][0] = c.lh(gml_r(c, V0).wrapping_add(0x160));
    c.g[V1][0] = c.lw(gml_r(c, V0).wrapping_add(0x164));
    let method = gml_r(c, V1);
    gml_set(c, A0, gml_r(c, A2).wrapping_add(gml_r(c, A0)));
    call_ptr(c, method, 0x0022_ec60)?;
    c.g[A0][0] = c.lw(gml_gp_sub(c, 0x84c));
    c.sw(gml_r(c, S1).wrapping_add(GAME_IS_STREAM_PAUSED), 0);
    c.sw(gml_r(c, S1).wrapping_add(GAME_FLUSH_STREAMER), 0);
    call_ptr(c, 0x0027_8358, 0x0022_ec70)?; // cSSXScriptEngine_Load
                                            // The 168-byte grid ("Nash"): `new` result in v0.
    gml_new(c, 0xa8, 0x4a_0000u32.wrapping_add(0x2a58), 0, 0x0022_ec88)?;
    c.g[T1][0] = u64::MAX;
    c.g[V1][0] = c.g[V0][0];
    c.g[A0][0] = 1;

    // The grid constructor (inlined): three nested one-pass loops that clear
    // 0x14-byte entries. The state lives in registers; every back edge checks
    // for events (an unwind resumes at the loop head).
    let mut head = 0x0022_ec98u32;
    loop {
        match head {
            0x0022_ec98 => {
                c.cycles += 4;
                gml_set(c, T0, gml_r(c, A0).wrapping_sub(1));
                gml_set(c, A3, gml_r(c, V1).wrapping_add(0x50));
                c.g[A0][0] = 1;
                head = 0x0022_eca8;
            }
            0x0022_eca8 => {
                c.cycles += 4;
                gml_set(c, A2, gml_r(c, A0).wrapping_sub(1));
                gml_set(c, A1, gml_r(c, V1).wrapping_add(0x28));
                c.g[A0][0] = 1;
                head = 0x0022_ecb8;
            }
            0x0022_ecb8 => {
                c.cycles += 7;
                c.sw(gml_r(c, V1).wrapping_add(0x10), 0);
                gml_set(c, A0, gml_r(c, A0).wrapping_sub(1));
                let again = c.g[A0][0] != c.g[T1][0];
                gml_set(c, V1, gml_r(c, V1).wrapping_add(0x14));
                if again {
                    if back_edge_unwinds(c, 0x0022_ecb8) {
                        return Ok(());
                    }
                    head = 0x0022_ecb8;
                } else {
                    head = 0x0022_ecd4;
                }
            }
            0x0022_ecd4 => {
                c.cycles += 3;
                c.g[A0][0] = c.g[A2][0];
                let again = c.g[A0][0] != c.g[T1][0];
                c.g[V1][0] = c.g[A1][0];
                if again {
                    if back_edge_unwinds(c, 0x0022_eca8) {
                        return Ok(());
                    }
                    head = 0x0022_eca8;
                } else {
                    head = 0x0022_ece0;
                }
            }
            0x0022_ece0 => {
                c.cycles += 3;
                c.g[A0][0] = c.g[T0][0];
                let again = c.g[A0][0] != c.g[T1][0];
                c.g[V1][0] = c.g[A3][0];
                if again {
                    if back_edge_unwinds(c, 0x0022_ec98) {
                        return Ok(());
                    }
                    head = 0x0022_ec98;
                } else {
                    break;
                }
            }
            _ => unreachable!(),
        }
    }

    // 0x22ecec: register the grid, debug font, game comm.
    c.cycles += 21;
    c.sw(gml_r(c, S0), c.g[V0][0]); // self->BXNash = grid
    c.sw(gml_r(c, V0).wrapping_add(0xa0), 0);
    c.sw(gml_r(c, V0).wrapping_add(0xa4), 0);
    call_ptr(c, BE_GET_BE, 0x0022_ecfc)?;
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, BE_GET_INTERFACE, 0x0022_ed08)?; // interface 0
    gml_new(c, 0x84, 0x48_0000u32.wrapping_sub(0x4ab8), 0, 0x0022_ed20)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A0][0] = c.g[V0][0];
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4aa8));
    c.g[A2][0] = 0;
    call_ptr(c, 0x0039_15e8, 0x0022_ed34)?; // cFont_cFont (debug font)
    c.g[A0][0] = c.lw(gml_gp_sub(c, 0x204)); // cGameComm_mGameComm
    let no_game_comm = c.g[A0][0] == 0;
    c.sw(gml_r(c, S1).wrapping_add(GAME_DEBUG_FONT), c.g[V0][0]);
    if !no_game_comm {
        c.cycles += 2; // 0x22ed40
        call_ptr(c, 0x0025_8448, 0x0022_ed48)?; // cGameComm_syncGame
    }

    // 0x22ed48: stream, occlusion, world, lights, scripts, crowds, particles.
    c.cycles += 143;
    call_ptr(c, BE_GET_BE, 0x0022_ed50)?;
    c.g[A1][0] = 9;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, BE_GET_INTERFACE, 0x0022_ed5c)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_bd98, 0x0022_ed64)?;
    c.g[A0][0] = c.g[S2][0]; // the bucket manager (gp + 0x2898)
    c.g[A1][0] = 9;
    call_ptr(c, 0x0035_49e0, 0x0022_ed70)?; // cBucketMan_init
    c.g[A1][0] = 1;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0035_5260, 0x0022_ed7c)?; // cBucketMan_setsortflag
    c.g[A0][0] = 0x4c_0000;
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x5830));
    call_ptr(c, 0x0016_c918, 0x0022_ed88)?; // cCameraTriggerMan_init
    call_ptr(c, 0x0024_2d10, 0x0022_ed90)?; // cStreamPause_construct
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a90));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x114;
    c.sw(gml_r(c, S1).wrapping_add(GAME_STREAM_PAUSE), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_edac)?; // OcclusionMan
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0024_2500, 0x0022_edb4)?;
    c.g[A1][0] = 0x4a_0000;
    c.g[A2][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_add(0x2a60));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x18;
    c.sw(gml_r(c, S1).wrapping_add(GAME_OCCLUSION_MAN), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_edd0)?; // World
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x003a_6688, 0x0022_edd8)?; // cWorld_cWorld
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a80));
    c.g[A0][0] = 0x32a0;
    c.sw(gml_r(c, S1).wrapping_add(GAME_WORLD), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_edf4)?; // WorldLightMan
    c.g[A1][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_BX_NASH));
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x002f_5180, 0x0022_ee00)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a70));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x498;
    c.sw(gml_r(c, S1).wrapping_add(GAME_WORLD_LIGHT_MAN), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_ee1c)?; // WScriptMan
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0030_87d0, 0x0022_ee24)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a60));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x2720;
    c.sw(gml_r(c, S1).wrapping_add(GAME_WSCRIPT_MAN), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_ee40)?; // CrowdMan2d
    c.g[A0][0] = c.g[V0][0];
    CROWD_MAN_2D_CTOR.call(c, 0x0022_ee48)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a50));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x9c4;
    c.sw(gml_r(c, S1).wrapping_add(GAME_CROWD_MAN_2D), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_ee64)?; // CrowdMan3d
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0034_3a68, 0x0022_ee6c)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a40));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x10;
    c.sw(gml_r(c, S1).wrapping_add(GAME_CROWD_MAN_3D), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_ee88)?; // MultiParticleMan
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0035_79c8, 0x0022_ee90)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a28));
    c.g[A0][0] = 0x10;
    c.g[A2][0] = 0x2000_0000;
    c.sw(
        gml_r(c, S1).wrapping_add(GAME_MULTI_PARTICLE_MAN),
        c.g[V0][0],
    );
    call_ptr(c, OPERATOR_NEW, 0x0022_eeac)?; // InstancesMan
    c.g[A1][0] = 0x80;
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = 0x40;
    call_ptr(c, 0x0035_1030, 0x0022_eebc)?; // cWScriptCompile_parseKeywords
    c.g[A0][0] = 0x50_0000;
    c.sw(gml_r(c, S1).wrapping_add(GAME_INSTANCE_MAN), c.g[V0][0]);
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x47f8));
    call_ptr(c, 0x0038_acd0, 0x0022_eecc)?; // cIrradianceDataBase_Load
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x6580));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x10;
    call_ptr(c, OPERATOR_NEW, 0x0022_eee4)?; // LoadIndicator
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x001f_30c0, 0x0022_eeec)?;
    c.g[A1][0] = 0x4a_0000;
    c.g[A2][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_add(0x2a68));
    c.g[A3][0] = 0;
    c.g[A0][0] = 8;
    c.sw(gml_r(c, S1).wrapping_add(GAME_LOAD_INDICATOR), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_ef08)?; // BigFile
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0031_6898, 0x0022_ef10)?;
    c.sw(gml_r(c, S1).wrapping_add(GAME_TRACK_BIG_FILE), c.g[V0][0]);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0022_f6b0, 0x0022_ef1c)?; // cGame_loadTrack
    c.g[A1][0] = 0x4a_0000;
    c.g[A2][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_add(0x2a70));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x688;
    call_ptr(c, OPERATOR_NEW, 0x0022_ef34)?; // Replay
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0026_efb8, 0x0022_ef3c)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a18));
    c.g[A0][0] = 1;
    c.sw(gml_r(c, S1).wrapping_add(GAME_REPLAY), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_ef58)?; // FXLoader
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x002d_bd10, 0x0022_ef64)?;
    c.sw(gml_r(c, S1).wrapping_add(GAME_FX_LOADER), c.g[V0][0]);
    c.g[A1][0] = 7;
    c.g[V0][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[A0][0] = c.lw(gml_r(c, V0).wrapping_add(0x78));
    call_ptr(c, BE_GET_INTERFACE, 0x0022_ef78)?; // network interface
    c.g[V0][0] = c.lw(gml_r(c, S3).wrapping_add(0x4b30)); // bIsNetworkGame
    let is_network = c.g[V0][0] != 0;
    gml_set(c, A0, gml_r(c, S3).wrapping_add(0x4b30));
    if is_network {
        // 0x22ef84: the input-sync queue length from the latency.
        c.cycles += 25;
        c.g[V0][0] = c.lw(gml_gp_sub(c, 0x204));
        c.g[A1][0] = 0x48_0000;
        c.g[V1][0] = c.lw(gml_gp_sub(c, 0x848));
        gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a08));
        c.lwc1(1, gml_r(c, V0).wrapping_add(0x120));
        c.cvt_s_w(1, 1);
        c.lwc1(0, gml_r(c, A0).wrapping_add(0x10));
        c.lwc1(2, gml_r(c, V1).wrapping_add(0x14));
        c.g[A0][0] = 0x44;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.mul_s(1, 1, 2);
        c.add_s(0, 0, 1);
        c.div_s(0, 0, 2);
        c.cvt_w_s(1, 0);
        c.mfc1(16, 1);
        gml_set(c, S0, gml_r(c, S0).wrapping_add(2));
        call_ptr(c, OPERATOR_NEW, 0x0022_efcc)?; // InputSync
        c.g[V1][0] = c.lw(gml_gp_sub(c, 0x204));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.lw(gml_r(c, V1).wrapping_add(0x120));
        call_ptr(c, 0x0023_7060, 0x0022_efe0)?; // cInputSync_cInputSync
        c.sw(gml_r(c, S1).wrapping_add(GAME_INPUT_SYNC), c.g[V0][0]);
    } else {
        c.cycles += 4; // 0x22efe8
        call_ptr(c, 0x0031_77f0, 0x0022_eff0)?; // BXrand
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0023_7cb0, 0x0022_eff8)?; // GMsrand
    }

    // 0x22eff8: the AI, playback manager, replay init.
    c.cycles += 38;
    c.g[A1][0] = 0x4a_0000;
    c.g[A2][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_add(0x2a78));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0xd0;
    call_ptr(c, OPERATOR_NEW, 0x0022_f010)?; // AI
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0012_86a0, 0x0022_f018)?; // cAI_cAI
    c.sw(gml_r(c, S1).wrapping_add(GAME_AI), c.g[V0][0]);
    c.g[V1][0] = c.lw(gml_r(c, V0).wrapping_add(0xcc));
    c.g[A0][0] = c.lh(gml_r(c, V1).wrapping_add(0x10));
    c.g[A1][0] = c.lw(gml_r(c, V1).wrapping_add(0x14));
    let ai_method = gml_r(c, A1);
    gml_set(c, A0, gml_r(c, V0).wrapping_add(gml_r(c, A0)));
    call_ptr(c, ai_method, 0x0022_f030)?; // AI vtable slot 2
    c.g[A0][0] = c.lw(gml_gp(c, 0x2a74));
    c.g[AT][0] = 0x425c_0000;
    c.mtc1(1, 12);
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x38));
    call_ptr(c, LOAD_PROGRESS_SET, 0x0022_f044)?; // 55
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x49f8));
    c.g[A0][0] = 0x7c;
    call_ptr(c, OPERATOR_NEW, 0x0022_f05c)?; // PlaybackMan
    c.g[A1][0] = 0x28;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0026_c458, 0x0022_f068)?;
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_REPLAY));
    c.sw(gml_r(c, S1).wrapping_add(GAME_PLAYBACK_MAN), c.g[V0][0]);
    call_ptr(c, 0x0026_f180, 0x0022_f074)?; // cReplay_init
    c.g[V0][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[A1][0] = 5;
    c.g[A0][0] = c.lw(gml_r(c, V0).wrapping_add(0x78));
    call_ptr(c, BE_GET_INTERFACE, 0x0022_f084)?; // save interface
    c.g[S4][0] = c.lw(gml_gp_sub(c, 0x1ec8)); // cBESaveInterface_bDoReplay
    let replay = c.g[S4][0] != 0;
    c.g[S2][0] = c.g[V0][0];
    if replay {
        game_module_load_replay_setup(c)?;
    }

    // 0x22f184: the view manager (the player count, or 1 for network games).
    c.cycles += 3;
    c.g[V0][0] = c.lw(gml_r(c, S3).wrapping_add(0x4b30));
    let is_network = c.g[V0][0] != 0;
    gml_set(c, A1, gml_r(c, S7).wrapping_sub(0x49e8));
    if is_network {
        c.cycles += 7; // 0x22f190
        c.g[A0][0] = 0x38;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, OPERATOR_NEW, 0x0022_f1a0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
    } else {
        c.cycles += 7; // 0x22f1ac
        c.g[A0][0] = 0x38;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, OPERATOR_NEW, 0x0022_f1bc)?;
        c.g[V1][0] = 0x53_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(gml_r(c, V1).wrapping_add(0x5bfc)); // player count
    }
    c.cycles += 6; // 0x22f1c8
    GAME_VIEW_MAN_CTOR.call(c, 0x0022_f1d0)?;
    c.sw(gml_r(c, S1).wrapping_add(GAME_VIEW_MAN), c.g[V0][0]);
    c.g[V0][0] = c.lw(gml_r(c, S3).wrapping_add(0x4b30));
    let is_network = c.g[V0][0] != 0;
    gml_set(c, V1, gml_r(c, S3).wrapping_add(0x4b30));
    let mut chase_camera = false;
    if !is_network {
        c.cycles += 1; // 0x22f20c
        c.g[A1][0] = 0x4a_0000;
    } else {
        c.cycles += 3; // 0x22f1e0
        c.g[V0][0] = c.lw(gml_r(c, V1).wrapping_add(8)); // bIsHost
        let is_host = c.g[V0][0] != 0;
        c.g[A1][0] = 0x4a_0000;
        if !is_host {
            chase_camera = true;
        }
    }
    if chase_camera {
        // Joined client: aim the chase camera at the second player's rider.
        c.cycles += 4; // 0x22f1ec
        c.g[V0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
        c.g[V0][0] = c.lw(gml_r(c, V0).wrapping_add(0x44));
        let no_second_player = c.g[V0][0] == 0;
        c.g[A1][0] = 0;
        if !no_second_player {
            c.cycles += 1; // 0x22f1fc
            c.g[A1][0] = c.lw(gml_r(c, V0).wrapping_add(0x18));
        }
        c.cycles += 3; // 0x22f200
        c.g[V0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_VIEW_MAN));
        c.g[A0][0] = c.lw(gml_r(c, V0).wrapping_add(4));
        call_ptr(c, 0x0015_edc8, 0x0022_f20c)?;
        c.cycles += 1; // 0x22f20c
        c.g[A1][0] = 0x4a_0000;
    }

    // 0x22f210: menu system, menu graphics, tables.
    c.cycles += 36;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_add(0x2a80));
    c.g[A0][0] = 0x68;
    call_ptr(c, OPERATOR_NEW, 0x0022_f224)?; // MenuSys
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x002c_c018, 0x0022_f22c)?; // cMenuSys_cMenuSys
    c.g[V1][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_DEBUG_FONT));
    c.g[A0][0] = 0x48_0000;
    c.sw(gml_r(c, S1).wrapping_add(GAME_MENU_SYS), c.g[V0][0]);
    gml_set(c, A0, gml_r(c, A0).wrapping_sub(0x49d8));
    c.sw(gml_r(c, V0).wrapping_add(0x58), c.g[V1][0]);
    c.g[A1][0] = 0x100;
    c.g[V1][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[A2][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_MENU_SYS));
    c.g[V0][0] = c.lw(gml_r(c, V1).wrapping_add(0xb0));
    c.sw(gml_r(c, A2).wrapping_add(0x5c), c.g[V0][0]);
    call_ptr(c, 0x003e_18c8, 0x0022_f258)?; // FILE_load("menu.ssh")
    c.g[T2][0] = c.lw(gml_gp_sub(c, 0x854));
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = c.lw(gml_r(c, S0).wrapping_add(0x14));
    c.g[A2][0] = 0x4a_0000;
    c.g[V1][0] = c.lw(gml_r(c, T2).wrapping_add(0x10d8));
    gml_set(c, A2, gml_r(c, A2).wrapping_add(0x2a88));
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, S0).wrapping_add(gml_r(c, A1)));
    c.g[A0][0] = c.lh(gml_r(c, V1).wrapping_add(0x170));
    c.g[T0][0] = 1;
    c.g[V0][0] = c.lw(gml_r(c, V1).wrapping_add(0x174));
    c.g[T1][0] = u64::MAX;
    let load_texture = gml_r(c, V0);
    gml_set(c, A0, gml_r(c, T2).wrapping_add(gml_r(c, A0)));
    call_ptr(c, load_texture, 0x0022_f290)?; // graphics vslot: texture from .ssh
    c.g[V1][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_MENU_SYS));
    c.sw(gml_r(c, S1).wrapping_add(GAME_BOX_TEXTURE_ID), c.g[V0][0]);
    let no_file = c.g[S0][0] == 0;
    c.sw(gml_r(c, V1).wrapping_add(0x60), c.g[V0][0]);
    if !no_file {
        c.cycles += 2; // 0x22f2a0
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7e98, 0x0022_f2a8)?; // cMemMan_free
    }
    c.cycles += 8; // 0x22f2a8
    c.g[A0][0] = c.lw(gml_gp(c, 0x2a74));
    c.g[S3][0] = c.g[S6][0];
    c.g[AT][0] = 0x42ba_0000;
    c.mtc1(1, 12);
    c.g[S2][0] = c.g[S5][0];
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x38));
    c.g[S0][0] = 0;
    call_ptr(c, LOAD_PROGRESS_SET, 0x0022_f2c8)?; // 93

    // 0x22f2c8: two transition managers and two splash managers.
    loop {
        c.cycles += 26;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x84;
        gml_set(c, A1, gml_r(c, FP).wrapping_sub(0x49c0));
        c.g[A2][0] = 0;
        call_ptr(c, OPERATOR_NEW, 0x0022_f2dc)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, 0x002e_4228, 0x0022_f2ec)?; // cTransitionMan(3, i)
        c.sw(gml_r(c, S2), c.g[V0][0]);
        c.g[A0][0] = 0x1030;
        c.g[V0][0] = 0x48_0000;
        c.g[A2][0] = 0;
        gml_set(c, A1, gml_r(c, V0).wrapping_sub(0x49b0));
        c.g[A3][0] = 0;
        call_ptr(c, OPERATOR_NEW, 0x0022_f308)?;
        gml_set(c, S2, gml_r(c, S2).wrapping_add(4));
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        gml_set(c, S0, gml_r(c, S0).wrapping_add(1));
        call_ptr(c, 0x002f_3550, 0x0022_f320)?; // cSplashManager(2, i)
        c.sw(gml_r(c, S3), c.g[V0][0]);
        c.g[V0][0] = (c.g[S0][0] < 2) as u64;
        let again = c.g[V0][0] != 0;
        gml_set(c, S3, gml_r(c, S3).wrapping_add(4));
        if !again {
            break;
        }
        if back_edge_unwinds(c, 0x0022_f2c8) {
            return Ok(());
        }
    }

    // 0x22f330: cut-scene transitions, flags, snow, audio, the UI engine.
    c.cycles += 126;
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x49a0));
    c.g[A0][0] = 0x48;
    call_ptr(c, OPERATOR_NEW, 0x0022_f348)?;
    c.g[A1][0] = 3;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x002e_a670, 0x0022_f354)?; // cCutSceneTransitionMan
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4988));
    c.g[A0][0] = 0x1718;
    c.sw(
        gml_r(c, S1).wrapping_add(GAME_CUTSCENE_TRANSITION_MAN),
        c.g[V0][0],
    );
    call_ptr(c, OPERATOR_NEW, 0x0022_f370)?;
    c.g[A1][0] = 2;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0034_c428, 0x0022_f37c)?; // cFlagManager
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4978));
    c.g[A0][0] = 0x60;
    c.sw(gml_r(c, S1).wrapping_add(GAME_FLAG_MANAGER), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_f398)?;
    c.g[A1][0] = 2;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x002e_5920, 0x0022_f3a4)?; // cSnowFall
    c.g[A0][0] = c.lw(gml_gp(c, 0x410)); // gSSXAudio
    c.sw(gml_r(c, S1).wrapping_add(GAME_UPDATE_TICK), 0);
    c.sw(gml_r(c, S1).wrapping_add(GAME_SNOW_FALL), c.g[V0][0]);
    call_ptr(c, 0x0028_62a8, 0x0022_f3b4)?; // cSSXAudio_InGameLoad
    c.g[A0][0] = c.lw(gml_gp(c, 0x2a74));
    c.g[AT][0] = 0x42c6_0000;
    c.mtc1(1, 12);
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x38));
    call_ptr(c, LOAD_PROGRESS_SET, 0x0022_f3c8)?; // 99
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4968));
    c.g[A0][0] = 0x70;
    call_ptr(c, OPERATOR_NEW, 0x0022_f3e0)?; // UIEngine
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0039_7b08, 0x0022_f3e8)?;
    c.g[A0][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[V1][0] = 0x48_0000;
    c.sw(gml_r(c, S1).wrapping_add(GAME_UI), c.g[V0][0]);
    gml_set(c, A1, gml_r(c, V1).wrapping_sub(0x4958));
    c.g[V1][0] = c.lw(gml_r(c, A0).wrapping_add(0x8c)); // FELocale
    c.g[A0][0] = 0x48_0000;
    c.sw(gml_r(c, V0).wrapping_add(0x10), c.g[V1][0]);
    gml_set(c, V1, gml_r(c, A0).wrapping_sub(0x4958));
    // The 11-byte name D_0047B6A8 is copied to the stack (unaligned 8 + 3).
    c.ldl(6, gml_r(c, V1).wrapping_add(7));
    c.ldr(6, gml_r(c, V1));
    c.g[A3][0] = c.lb(gml_r(c, V1).wrapping_add(8));
    c.g[V0][0] = c.lb(gml_r(c, V1).wrapping_add(9));
    c.sdl(6, sp(c).wrapping_add(7));
    c.sdr(6, sp(c));
    c.sb(sp(c).wrapping_add(8), c.g[A3][0]);
    c.sb(sp(c).wrapping_add(9), c.g[V0][0]);
    c.g[A2][0] = c.lb(gml_r(c, V1).wrapping_add(0xa));
    c.sb(sp(c).wrapping_add(0xa), c.g[A2][0]);
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_UI));
    call_ptr(c, 0x0039_7c40, 0x0022_f438)?; // cUIEngine_loadFile
    c.g[A1][0] = 0x48_0000;
    c.g[A0][0] = 0x1c;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4948));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, OPERATOR_NEW, 0x0022_f450)?; // OVUIInput (data)
    c.g[V1][0] = 0x47_0000;
    c.g[T0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_UI));
    gml_set(c, V1, gml_r(c, V1).wrapping_sub(0x2e30));
    c.sb(gml_r(c, V0).wrapping_add(4), 0);
    c.sw(gml_r(c, V0).wrapping_add(8), c.g[V1][0]); // cFEInput vtable
    c.g[A1][0] = 0x48_0000;
    c.sw(gml_r(c, V0).wrapping_add(0x18), 0);
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4938));
    c.sw(gml_r(c, V0).wrapping_add(0xc), 0);
    c.g[A2][0] = 0;
    c.sw(gml_r(c, V0).wrapping_add(0x10), 0);
    c.g[A3][0] = 0;
    c.sw(gml_r(c, V0).wrapping_add(0x14), 0);
    c.g[A0][0] = 8;
    c.sw(gml_r(c, T0).wrapping_add(0xc), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_f490)?; // OVUISound
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x001a_2f38, 0x0022_f498)?;
    c.g[V1][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_UI));
    c.g[A1][0] = 0x48_0000;
    c.g[T0][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[A3][0] = 0;
    c.sw(gml_r(c, V1).wrapping_add(0x14), c.g[V0][0]);
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4928));
    c.g[A0][0] = 0xc98;
    c.g[A2][0] = 0;
    c.g[V1][0] = c.lw(gml_r(c, T0).wrapping_add(0x88)); // Fonts
    c.g[V0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_UI));
    c.sw(gml_r(c, V0).wrapping_add(8), c.g[V1][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_f4c8)?; // cOVStateHUD1P
    c.g[A1][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_UI));
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = 0;
    call_ptr(c, 0x001e_9a30, 0x0022_f4d8)?;
    c.g[V1][0] = c.lw(gml_r(c, V0).wrapping_add(0x1c));
    c.g[A0][0] = 0xffff_ffff_ffff_c0ff;
    c.g[A1][0] = c.g[V0][0];
    c.g[V1][0] &= c.g[A0][0];
    c.g[V1][0] |= 0x500; // mode 5
    c.sw(gml_r(c, V0).wrapping_add(0x1c), c.g[V1][0]);
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_UI));
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x18));
    call_ptr(c, 0x0039_f238, 0x0022_f4fc)?; // cUIStateStack_pushExplicit
                                            // The state machine fields start cleared; `cGameState_mpGame = self`.
    for off in [
        0x204, 0x1fc, 0x200, 0x20c, 0x208, 0x218, 0x214, 0x210, 0x21c,
    ] {
        c.sw(gml_r(c, S1).wrapping_add(off), 0);
    }
    let no_replay = c.g[S4][0] == 0;
    c.sw(gml_gp_sub(c, 0x488), c.g[S1][0]);
    // 1: `cGame_setState(self, a1, 0, 0)` (0x22f598); 2: with a1 = 10 (0x22f5ac).
    let set_state: u32;
    if no_replay {
        c.cycles += 4; // 0x22f540: preload state 10 and wait for it
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x0023_1278, 0x0022_f54c)?; // cGame_setPreloadState
        loop {
            c.cycles += 8; // 0x22f550
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0023_0f40, 0x0022_f558)?; // cGame_updateState
            c.g[A0][0] = 0;
            call_ptr(c, 0x003e_5398, 0x0022_f560)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0023_12d8, 0x0022_f568)?; // cGame_isPreloadedStateReady
            let not_ready = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw(gml_gp_sub(c, 0x204));
            if !not_ready {
                break;
            }
            if back_edge_unwinds(c, 0x0022_f550) {
                return Ok(());
            }
        }
        c.cycles += 2; // 0x22f570
        let no_game_comm = c.g[A0][0] == 0;
        c.g[V0][0] = 0x53_0000;
        if !no_game_comm {
            c.cycles += 3; // 0x22f578
            call_ptr(c, 0x0025_86b0, 0x0022_f580)?; // cGameComm_syncWait
            c.g[V0][0] = 0x53_0000;
        }
        c.cycles += 4; // 0x22f584
        gml_set(c, V0, gml_r(c, V0).wrapping_add(0x5bc8));
        c.g[V1][0] = c.lb(gml_r(c, V0).wrapping_add(0x49));
        let flag = c.g[V1][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        if !flag {
            c.cycles += 1; // 0x22f594
            c.g[A1][0] = 0xa;
            set_state = 1;
        } else {
            set_state = 2;
        }
    } else {
        c.cycles += 6; // 0x22f528: replay: hand the replay its seed, start state 0x10
        c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_REPLAY));
        c.g[A1][0] = c.lw(gml_gp(c, 0x394));
        call_ptr(c, 0x0026_f898, 0x0022_f534)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x10;
        set_state = 1;
    }
    if set_state == 1 {
        c.cycles += 5; // 0x22f598
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0023_1250, 0x0022_f5a4)?; // cGame_setState
        c.g[A1][0] = 0x48_0000;
    } else {
        c.cycles += 5; // 0x22f5ac
        c.g[A1][0] = 0xa;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0023_1250, 0x0022_f5bc)?;
        c.g[A1][0] = 0x48_0000;
    }

    // 0x22f5c0: the three object nodes with their managers.
    c.cycles += 59;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4918));
    c.g[A0][0] = 0x14;
    call_ptr(c, OPERATOR_NEW, 0x0022_f5d4)?; // AIVisualEffectsMan node
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 6;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0035_4648, 0x0022_f5e4)?; // cObjNode_cObjNode1
    c.g[V1][0] = 0x48_0000;
    gml_set(c, V1, gml_r(c, V1).wrapping_sub(0x34a0));
    c.sw(gml_r(c, S0).wrapping_add(0xc), c.g[V1][0]);
    call_ptr(c, 0x002f_0390, 0x0022_f5f4)?; // cAIVisualEffectsMan_construct
    c.sw(gml_r(c, S1).wrapping_add(GAME_AI_VFX_MAN), c.g[S0][0]);
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4900));
    c.sw(gml_r(c, S0).wrapping_add(0x10), c.g[V0][0]);
    c.g[A0][0] = 0x14;
    call_ptr(c, OPERATOR_NEW, 0x0022_f614)?; // VisualEffectsMan node
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 3;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0035_4648, 0x0022_f624)?;
    c.g[V1][0] = 0x48_0000;
    gml_set(c, V1, gml_r(c, V1).wrapping_sub(0x2f90));
    c.sw(gml_r(c, S0).wrapping_add(0xc), c.g[V1][0]);
    call_ptr(c, 0x002f_0510, 0x0022_f634)?; // cVisualEffectsMan_construct
    c.sw(gml_r(c, S1).wrapping_add(GAME_VFX_MAN), c.g[S0][0]);
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x48e8));
    c.sw(gml_r(c, S0).wrapping_add(0x10), c.g[V0][0]);
    c.g[A0][0] = 0x14;
    call_ptr(c, OPERATOR_NEW, 0x0022_f654)?; // SSXStatsMan node
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0035_4648, 0x0022_f664)?;
    c.g[V1][0] = 0x48_0000;
    gml_set(c, V1, gml_r(c, V1).wrapping_sub(0x3518));
    c.sw(gml_r(c, S0).wrapping_add(0xc), c.g[V1][0]);
    call_ptr(c, 0x002d_2100, 0x0022_f674)?; // SSXStatsMan construct
    c.sw(gml_r(c, S1).wrapping_add(GAME_STATS_MAN), c.g[S0][0]);
    c.sw(gml_r(c, S0).wrapping_add(0x10), c.g[V0][0]);
    frame.restore(c, S0, 0xa0);
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
    Ok(())
}

/// The `cCrowdMan2d` and `cGameViewMan` constructors: the two
/// direct (same-shard) calls of `cGame_load`.
const CROWD_MAN_2D_CTOR: Callee = Callee {
    addr: 0x0022_9180,
    f: ee::s05::CrowdMan2d_ctor,
};
const GAME_VIEW_MAN_CTOR: Callee = Callee {
    addr: 0x0022_e3b8,
    f: ee::s05::GameViewMan_ctor,
};

/// The replay branch of `cGame_load` (0x22f090..0x22f184): initialise the AI
/// riders and the game mode, hand the AI the podium actor list from the
/// replay buffer, start the AI actors unless the mode/sub-mode pair says
/// otherwise, wait until the AI is loaded, then load the replay from the
/// buffer and release it.
fn game_module_load_replay_setup(c: &mut Cpu) -> Flow {
    c.cycles += 17; // 0x22f090
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
    call_ptr(c, 0x0012_8958, 0x0022_f098)?; // cAI_initAIRiders
    c.g[V0][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[S0][0] = 0x53_0000;
    gml_set(c, S0, gml_r(c, S0).wrapping_add(0x5bc8)); // cBENewRaceInterface_newRace
    c.g[A0][0] = c.lw(gml_r(c, V0).wrapping_add(0xc0));
    c.g[A1][0] = c.lb(gml_r(c, S0).wrapping_add(0x4a)); // sub mode
    call_ptr(c, 0x0023_8160, 0x0022_f0b0)?; // cGameModeMan_initGameMode
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0015_26d0, 0x0022_f0b8)?; // getGameReplayBuffer
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
    gml_set(c, A1, gml_r(c, V0).wrapping_add(0x20));
    call_ptr(c, 0x0012_bac0, 0x0022_f0c4)?; // cAI_setPodiumActorList
    c.g[V1][0] = c.lb(gml_r(c, S0).wrapping_add(0x48)); // mode
    c.g[V0][0] = 4;
    let mode_is_4 = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = c.lb(gml_r(c, S0).wrapping_add(0x4a));
    'chain: {
        if mode_is_4 {
            break 'chain;
        }
        c.cycles += 3; // 0x22f0d4
        c.g[A1][0] = 5;
        if c.g[V1][0] != c.g[A1][0] {
            // (branch-likely: the delay slot only runs when taken)
            c.g[V0][0] = 6;
        } else {
            c.cycles += 2; // 0x22f0e0: mode 5 needs sub mode 4
            let skip = c.g[A0][0] != c.g[V0][0];
            c.g[V0][0] = 6;
            if skip {
                break 'chain;
            }
        }
        c.cycles += 2; // 0x22f0e8
        if c.g[V1][0] != c.g[V0][0] {
            c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
        } else {
            c.cycles += 2; // 0x22f0f0: mode 6 needs sub mode 5
            let skip = c.g[A0][0] != c.g[A1][0];
            if skip {
                break 'chain;
            }
            c.cycles += 1; // 0x22f0f8
            c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
        }
        c.cycles += 3; // 0x22f0fc
        c.g[V0][0] = c.lw(gml_r(c, A0).wrapping_add(0x80)); // AI already has actors
        if c.g[V0][0] != 0 {
            break 'chain;
        }
        c.cycles += 2; // 0x22f108
        if c.g[V1][0] != 0 {
            c.cycles += 3; // 0x22f120
            c.g[A1][0] = 2;
            call_ptr(c, 0x0012_8998, 0x0022_f128)?; // cAI_initAIActors(ai, 2)
        } else {
            c.cycles += 4; // 0x22f110
            c.g[A1][0] = 0;
            call_ptr(c, 0x0012_8998, 0x0022_f118)?; // cAI_initAIActors(ai, 0)
        }
        c.g[V0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
        c.cycles += 2; // 0x22f12c
        c.sw(gml_r(c, V0).wrapping_add(0x8c), 0);
    }
    // 0x22f148: wait until the AI is loaded and ready, yielding a tick each time.
    loop {
        c.cycles += 4;
        c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
        call_ptr(c, 0x0012_a180, 0x0022_f150)?; // cAI_isAILoadedAndReady
        if c.g[V0][0] != 0 {
            break;
        }
        if back_edge_unwinds(c, 0x0022_f138) {
            return Err(Unwound);
        }
        c.cycles += 4; // 0x22f138
        c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
        call_ptr(c, 0x0012_96f8, 0x0022_f140)?; // cAI_loadRiders
        c.g[A0][0] = 1;
        call_ptr(c, 0x003e_52b0, 0x0022_f148)?; // THREAD_yieldticks(1)
    }
    c.cycles += 11; // 0x22f158
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_REPLAY));
    call_ptr(c, 0x0027_0ef8, 0x0022_f160)?; // cReplay_initLoad
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0015_26d0, 0x0022_f168)?;
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_REPLAY));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0027_1758, 0x0022_f174)?; // cReplay_load
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_REPLAY));
    call_ptr(c, 0x0027_0f18, 0x0022_f17c)?; // cReplay_purgeLoad
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0015_2728, 0x0022_f184)?; // releaseReplayBuffer
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake, machine};
    use crate::difftest::{Rng, DATA};
    use ssx3_ee_rt::funcs::{register_functions, FnEntry};

    /// `cGame_load` calls about 90 guest functions, half of them unported and
    /// all of them real code that would run on test data. The test therefore
    /// runs in a child process (the same test binary, this one test, `--exact`)
    /// whose function table holds only the stubs below: every other callee
    /// misses the table and lands in the logging fake (`fakes::fake_call`).
    /// A table swap in the shared test process would race with the other
    /// tests' `fresh()`.
    const GML_CHILD_ENV: &str = "SSX3_OC421_CHILD";

    /// Fixture layout (all in `DATA..DATA+0xfffff`, above the fakes' pointer
    /// range `0x800000..0x840000`).
    const GAME: u32 = DATA + 0x8_0000;
    const APP: u32 = DATA + 0x8_0800;
    const GRMAN: u32 = DATA + 0x8_4000;
    const GRMAN_VT: u32 = DATA + 0x8_6000;
    const AI_OBJ: u32 = DATA + 0x8_8000;
    const AI_VT: u32 = DATA + 0x8_9000;
    const SECOND_PLAYER: u32 = DATA + 0x8_9400;
    const GAME_COMM: u32 = DATA + 0x8_a000;
    /// Stub controls (`DATA + 0x200000` and up).
    const CTRL: u32 = DATA + 0x20_0000;
    /// `cBENetworkInterface_networkOpt` and `cBENewRaceInterface_newRace`.
    const NETWORK_OPT: u32 = 0x0053_4b30;
    const NEW_RACE: u32 = 0x0053_5bc8;

    /// A stub for a "ready?" query: returns 0 for the first `n` calls
    /// (`n` at `zeros_at`), then the fake's non-zero pointer.
    fn stub_ready(c: &mut Cpu, target: u32, zeros_at: u32, count_at: u32) {
        let ret = c.g[RA][0] as u32;
        let n = c.bus.read32(count_at);
        c.bus.write32(count_at, n + 1);
        fake(c, target, ret);
        if c.pc == ret && n < c.bus.read32(zeros_at) {
            c.g[V0][0] = 0;
        }
    }
    fn stub_ai_ready(c: &mut Cpu) {
        stub_ready(c, 0x0012_a180, CTRL, CTRL + 4)
    }
    fn stub_preload_ready(c: &mut Cpu) {
        stub_ready(c, 0x0023_12d8, CTRL + 8, CTRL + 0xc)
    }
    /// `cAI_cAI`: returns the fixture's AI object.
    fn stub_ai_new(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        fake(c, 0x0012_86a0, ret);
        if c.pc == ret {
            c.g[V0][0] = AI_OBJ as u64;
        }
    }
    /// `FILE_load`: a file pointer, or 0 (load failed).
    fn stub_file_load(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        fake(c, 0x003e_18c8, ret);
        if c.pc == ret {
            c.g[V0][0] = if c.bus.read32(CTRL + 0x10) != 0 {
                0
            } else {
                (DATA + 0x8_c000) as u64
            };
        }
    }

    static STUBS: [FnEntry; 4] = [
        FnEntry {
            start: 0x0012_86a0,
            end: 0x0012_86a4,
            f: stub_ai_new,
        },
        FnEntry {
            start: 0x0012_a180,
            end: 0x0012_a184,
            f: stub_ai_ready,
        },
        FnEntry {
            start: 0x0023_12d8,
            end: 0x0023_12dc,
            f: stub_preload_ready,
        },
        FnEntry {
            start: 0x003e_18c8,
            end: 0x003e_18cc,
            f: stub_file_load,
        },
    ];

    /// In the child process only: replace the function table by the stubs.
    fn gml_install_stub_table() {
        if std::env::var_os(GML_CHILD_ENV).is_some() {
            register_functions(&STUBS);
        }
    }

    /// The fixture for one seed: the game object in `a0`, the globals the
    /// function reads (network/replay flags, mode bytes, vtables), the stub
    /// controls; the branch picks come from the seed's random bits.
    fn gml_setup(c: &mut Cpu, seed: u32) {
        gml_install_stub_table();
        let mut r: Rng = machine(c, seed, 0x22_ebc8, 100);
        let bits = r.next_u32();
        let gp = crate::difftest::GAME_GP;
        c.pc = GAME_LOAD_ENTRY;
        c.g[A0][0] = GAME as u64;
        // The application object (gp - 0x848): clock step, back end, fonts, ...
        c.bus.write32(gp.wrapping_sub(0x848), APP);
        c.bus.write32(APP + 0x14, 0x3c88_8889); // 1/60
        for off in [0x78u32, 0x88, 0x8c, 0xb0, 0xc0] {
            c.bus.write32(APP + off, r.next_u32());
        }
        // Graphics manager (gp - 0x854) and its vtable slots (delta, method).
        c.bus.write32(gp.wrapping_sub(0x854), GRMAN);
        c.bus.write32(GRMAN + 0x10d8, GRMAN_VT);
        for off in [0x160u32, 0x170] {
            c.bus.write16(GRMAN_VT + off, (r.next_u32() & 0xfc) as u16);
            c.bus
                .write32(GRMAN_VT + off + 4, if bits & 1 == 0 { 0x4000 } else { 0 });
        }
        // Game comm (gp - 0x204), network options, replay flag.
        let comm = bits & 2 != 0;
        c.bus
            .write32(gp.wrapping_sub(0x204), if comm { GAME_COMM } else { 0 });
        c.bus.write32(GAME_COMM + 0x120, 2 + (bits >> 8) % 6);
        let network = bits & 4 != 0;
        c.bus.write32(NETWORK_OPT, network as u32);
        c.bus.write32(NETWORK_OPT + 8, (bits & 8 != 0) as u32); // bIsHost
        c.bus
            .write32(NETWORK_OPT + 0x10, 0x4220_0000 + (r.next_u32() & 0xffff)); // latency
        let replay = bits & 0x10 != 0;
        c.bus.write32(gp.wrapping_sub(0x1ec8), replay as u32);
        c.bus.write32(gp + 0x394, r.next_u32());
        // Race set-up bytes: mode, flag, sub mode.
        let modes: [(u8, u8); 8] = [
            (4, 0),
            (5, 4),
            (5, 1),
            (6, 5),
            (6, 2),
            (0, 0),
            (3, 1),
            (5, 5),
        ];
        let (mode, sub) = modes[((bits >> 5) & 7) as usize];
        c.bus.write8(NEW_RACE + 0x48, mode);
        c.bus.write8(NEW_RACE + 0x49, (bits >> 9 & 1) as u8);
        c.bus.write8(NEW_RACE + 0x4a, sub);
        c.bus.write32(0x0053_5bfc, 1 + (bits >> 10) % 4); // player count
                                                          // AI object: vtable slot 2, "already has actors", second player.
        c.bus.write32(AI_OBJ + 0xcc, AI_VT);
        c.bus.write16(AI_VT + 0x10, 0);
        c.bus.write32(AI_VT + 0x14, 0x4000);
        c.bus.write32(AI_OBJ + 0x80, (bits >> 12 & 1) as u32);
        c.bus.write32(
            AI_OBJ + 0x44,
            if bits >> 13 & 1 != 0 {
                SECOND_PLAYER
            } else {
                0
            },
        );
        c.bus.write32(SECOND_PLAYER + 0x18, r.next_u32());
        // Stub controls: loops that go round 0..2 times, file load failing.
        c.bus.write32(CTRL, (bits >> 14) % 3);
        c.bus.write32(CTRL + 8, (bits >> 16) % 3);
        c.bus.write32(
            CTRL + 0x10,
            (bits >> 18 & 1 != 0 && bits >> 19 & 1 != 0) as u32,
        );
    }

    fn gml_run_checked() {
        covered(ssx3_ee::s05::f_0022ebc8__generated, game_module_load, 160, &gml_setup);
    }

    /// `cGame_load` against the generated body: registers (strict: scratch
    /// registers and the arguments of all ~90 calls), cycles and memory, over
    /// network/local games, replay/non-replay (every mode and sub-mode pair,
    /// both AI-actor starts), the wait loops (0-2 rounds), a failing menu file
    /// load and unwinds at random call indices.
    #[test]
    fn game_module_load_matches() {
        if std::env::var_os(GML_CHILD_ENV).is_some() {
            return gml_run_checked();
        }
        let name = format!(
            "{}::game_module_load_matches",
            module_path!().split_once("::").map_or("", |p| p.1)
        );
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([name.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(GML_CHILD_ENV, "1")
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
}
