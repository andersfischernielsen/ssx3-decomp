//! Staging module oc644 (wave 234): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc644`.
//!
//! Four of the wave's eight functions already have hand ports in the
//! source-tree units (not installed yet): `GameModule_QuitToLoadScreen`
//! (gameload.rs), `GameModule_CanRestartFromPause` (gameload.rs),
//! `InitLoadScreen_Enter` and `cGameLoadScreen_LoadPictures`
//! (loadscreens_prestart.rs). They are not repeated here.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0023_1b50, c_load_screen_draw_splash, "cLoadScreen_drawSplash"),
];

/// cSectionMan's location tables: `cSectionMan_mLocationModeMap` (8 bytes
/// per location: the mode at +4) and `cSectionMan_mLocationMap_CEA8` (0x28
/// bytes per location).
const SM_LOCATION_MODE_MAP: u32 = 0x0044_2820;
const SM_LOCATION_MAP: u32 = 0x0044_2488;
const SM_LOCATION_SIZE: u32 = 0x28;
const SM_LOC_NUM_SECTIONS: u32 = 0x4;
const SM_LOC_SKY_SECTION: u32 = 0x8;
const SM_LOC_SECTION_IDS: u32 = 0x10;
const SM_LOC_TRANSPORT: u32 = 0xc;
/// cSectionMan fields: the current location, the activated flag, the sky swap.
const SM_LOCATION_ID: u32 = 0x1bc;
const SM_ACTIVATED: u32 = 0x1c0;
const SM_SKY_SWAP: u32 = 0x1a4;
/// "No sky section" (IsSection is false for it).
const SM_NO_SKY: u64 = 0x31;

/// `void cSectionMan::load(cSectionMan* self, int idx, int mode)` (0x22cea8):
/// makes location `idx` the current one. Unloads every loaded section the
/// location does not use (sections 43 and 44..=48 always stay), records the
/// location and clears the activated flag, loads the location's sections, its
/// sky section (49 = none) and its transport section, then asks
/// `cSectionMan::loadMode` for `mode` (7: the location's own mode from the
/// mode table; `fp` holds the resolved mode across the calls).
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn c_section_man_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x7;
        frame.save(c, S3, 0x60);
        frame.save(c, S2, 0x70);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x90);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        c.g[A1][0] = c.g[A2][0];
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);

        // Resolved mode: `mode`, or the location's entry in the mode table
        // when `mode == 7`.
        if c.g[A1][0] != c.g[V0][0] {
            c.cycles += 1;
            c.g[FP][0] = c.g[A1][0];
        } else {
            c.cycles += 6;
            c.g[V0][0] = 0x440000;
            c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2820));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[FP][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        }

        // The location's sky section (default: none) and the loop registers.
        c.cycles += 15;
        c.g[V1][0] = 0x28;
        c.g[V0][0] = 0x440000;
        c.mult(0, 3, 19, 3); // v1 = idx * 0x28
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2488)); // SM_LOCATION_MAP
        c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(SM_LOC_SECTION_IDS));
        c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(SM_LOC_NUM_SECTIONS));
        c.g[S6][0] = SM_NO_SKY;
        c.g[A1][0] = 0u64; // i
        c.g[S1][0] = SM_LOCATION_SIZE as u64;
        c.g[S7][0] = 0x4;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(SM_LOC_SKY_SECTION));
        c.g[V1][0] = ((c.g[A0][0] as i64) < 0i64) as u64;
        if c.g[V1][0] == 0 {
            c.g[S6][0] = c.g[A0][0];
        }
        c.g[A3][0] = 0x1;

        // Unload every section i (0..50) that the location does not use.
        'unload: loop {
            c.cycles += 4;
            c.g[A2][0] = 0u64; // j
            c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c)); // i - 44
            c.g[T1][0] = c.g[A1][0] ^ 0x2b; // i ^ 43
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1)); // i + 1
            loop {
                // Scan the location's section ids for i.
                c.cycles += 6;
                c.mult(0, 4, 19, 17); // a0 = idx * 0x28
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                if t {
                    break;
                }
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                if c.g[A1][0] != c.g[V1][0] {
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_cf50);
                    }
                    continue;
                }
                c.cycles += 1;
                c.g[A3][0] = 0u64; // found: i is used by this location
                break;
            }

            // Keep i when it is used (or is 43, or lies in 44..=48); unload
            // it otherwise.
            c.cycles += 5;
            c.g[V0][0] = (c.g[S7][0] < c.g[T0][0]) as u64;
            if c.g[V0][0] == 0 {
                c.g[A3][0] = 0u64;
            }
            if c.g[T1][0] == 0 {
                c.g[A3][0] = 0u64;
            }
            if c.g[A3][0] == 0u64 {
                c.g[A1][0] = c.g[S0][0];
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0022_d598, 0x0022_cf9c)?; // cSectionMan_unloadSection(self, i)
                c.g[A1][0] = c.g[S0][0];
            }

            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 50i64) as u64;
            let more = c.g[V0][0] != 0u64;
            c.g[A3][0] = 0x1;
            if !more {
                break 'unload;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_cf40);
            }
        }

        // Record the location, then load its sections in table order.
        c.cycles += 11;
        c.g[V0][0] = SM_LOCATION_SIZE as u64;
        c.g[V1][0] = 0x440000;
        c.mult(0, 5, 19, 2); // a1 = idx * 0x28
        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2488)); // SM_LOCATION_MAP
        c.sw((c.g[S2][0] as u32).wrapping_add(SM_LOCATION_ID), c.g[S3][0]);
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(SM_LOC_NUM_SECTIONS));
        c.sw((c.g[S2][0] as u32).wrapping_add(SM_ACTIVATED), 0u64);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32)); // &num
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0u64;
        if !none {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(SM_LOC_SECTION_IDS));
            c.g[S4][0] = c.g[A0][0];
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S2][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                call_ptr(c, 0x0022_d3d8, 0x0022_cff8)?; // cSectionMan_loadSection(self, id)
                c.g[V0][0] = c.lw(c.g[S4][0] as u32);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0u64 {
                    break;
                }
                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0022_cfe8);
                }
            }
        }

        // The sky section, unless it is none (this also clears the sky swap).
        c.cycles += 3;
        c.g[V0][0] = c.g[S6][0] ^ SM_NO_SKY;
        let no_sky = c.g[V0][0] == 0u64;
        c.g[A1][0] = c.g[S6][0];
        if !no_sky {
            c.cycles += 3;
            c.sw((c.g[S2][0] as u32).wrapping_add(SM_SKY_SWAP), 0u64);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0022_d3d8, 0x0022_d020)?;
        }

        // The transport section, if the location has one.
        c.cycles += 8;
        c.g[V0][0] = SM_LOCATION_SIZE as u64;
        c.g[A0][0] = 0x440000;
        c.mult(0, 2, 19, 2); // v0 = idx * 0x28
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2488));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(SM_LOC_TRANSPORT));
        let no_transport = (c.g[A1][0] as i64) < 0;
        c.g[A0][0] = c.g[S2][0];
        if !no_transport {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0022_d3d8, 0x0022_d048)?;
            c.g[A0][0] = c.g[S2][0];
        }

        c.cycles += 14;
        c.g[A1][0] = c.g[FP][0];
        call_ptr(c, 0x0022_d640, 0x0022_d054)?; // cSectionMan_loadMode(self, mode)

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

/// `void cGame::partialRestart(cGame* self)` (0x230180), `GameModule_ResetRace`:
/// resets the race state: the playback, the wscript missions, the buckets, the
/// AI (virtual slot 5 with mode 3), avalanche, the view, the world map, the
/// replay, the script engine, the camera, both crowds, the particles, the HUD
/// state (handle event 7) and the audio.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn c_game_partial_restart(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 55;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2898)); // gBucketMan_bm
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
        call_ptr(c, 0x0026_ca90, 0x0023_01a0)?; // cPlaybackMan_restart
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
        call_ptr(c, 0x0030_9030, 0x0023_01a8)?; // cWScriptMan_initMissions
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0035_4c98, 0x0023_01b4)?; // cBucketMan_mergeBucket(bm, 1)
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0035_5118, 0x0023_01c0)?; // cBucketMan_purgeBucket(bm, 1)
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x8;
        call_ptr(c, 0x0035_5118, 0x0023_01cc)?; // cBucketMan_purgeBucket(bm, 8)
        // The AI's virtual slot 5 with mode 3.
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 0x3;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
        let slot = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, slot, 0x0023_01e8)?;
        call_ptr(c, 0x002d_9b40, 0x0023_01f0)?; // cAvalanche_reset
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
        call_ptr(c, 0x0022_e7c8, 0x0023_01f8)?; // cGameViewMan_snapAll
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x003a_6800, 0x0023_0200)?; // cWorld_resetMap
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x0026_f228, 0x0023_0208)?; // cReplay_reset
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
        call_ptr(c, 0x0027_89e0, 0x0023_0210)?; // cSSXScriptEngine_Restart
        call_ptr(c, 0x0015_db58, 0x0023_0218)?; // cCamera_onRestart
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x0022_9498, 0x0023_0220)?; // crowd 2d reset
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
        call_ptr(c, 0x0034_3bc0, 0x0023_0228)?; // crowd 3d reset
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x0035_7b38, 0x0023_0230)?; // cMultiParticleMan_reset
        // The HUD state (its hash of "hud" in the UI state stack).
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ab0)); // str_hud_c
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18)); // &mStateStack
        call_ptr(c, 0x0031_7670, 0x0023_0244)?; // GetHashValue32
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_f9d8, 0x0023_0250)?; // cUIStateStack_findState
        c.g[A3][0] = c.g[V0][0];
        c.g[A1][0] = 0x7;
        if c.g[A3][0] != 0u64 {
            c.cycles += 6;
            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[A2][0] = 0u64;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
            let handle_event = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, handle_event, 0x0023_0274)?; // hudState->handleEvent(7, 0)
        }
        c.cycles += 13;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8)); // cWScriptMan_TheWScriptMan
        call_ptr(c, 0x0030_9f18, 0x0023_027c)?; // cWScriptMan_runGlobalScripts
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
        call_ptr(c, 0x0030_8c60, 0x0023_0284)?; // cWScriptMan_setupMissions
        call_ptr(c, 0x0028_b180, 0x0023_028c)?; // cAudioInterface_bxAudio
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0028_7108, 0x0023_0294)?; // cSSXAudio_PartialRestart
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cGame::startReplay(cGame* self, int)` (0x230558),
/// `GameModule_BeginReplayPlayback`: counts the replay up (gp-0x69c), records
/// it in the replay table at 0x5366e8, sets the replay pointer (gp-0x6a0) from
/// 0x442910, then starts the playback and the replay from the game object.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn c_game_start_replay(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[V1][0] = 0x530000;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
        c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
        c.g[A1][0] = 0x440000;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2910));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = 0x8;
        c.g[A1][0] = 0u64;
        c.sw(c.g[V0][0] as u32, c.g[A2][0]);
        c.g[A3][0] = 0u64;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[A0][0]);
        c.g[A2][0] = 0u64;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[T0][0]);
        call_ptr(c, 0x0026_f8a0, 0x0023_05ac)?; // cPlaybackMan_startReplay
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x002c_c0b8, 0x0023_05b8)?; // replay start
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cLoadScreen::drawSplash(void)` (0x231b50), `GameModule_DrawFullscreenQuad`:
/// builds a 4-pass fullscreen-quad vertex block on the stack (zeroed, with a
/// 1.0 in f0), stores its colour words into the draw context at gp-0x854, and
/// draws it through the context's virtual method (slot at +0x23c, this at +0x238).
pub fn c_load_screen_draw_splash(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0xe0);
        c.g[T0][0] = c.g[A1][0];
        frame.save_ra(c, 0xd0);
        c.g[V0][0] = 0x3;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        // Four passes: v0 counts 3, 2, 1, 0.
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = 0x80;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_1b68);
            }
        }
        c.cycles += 5;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0); // f0 = 1.0
        c.g[V1][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
        loop {
            c.cycles += 8;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V0][0] == 0u64 {
                break;
            }
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
            if event_due(c) {
                deliver_event_here(c, 0x0023_1b98);
            }
        }
        c.cycles += 45;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A1][0] = 0x4;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), 0u64);
        c.g[A2][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
        c.g[A3][0] = 0u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), 0u64);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xcc));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), 0u64);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc4));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
        c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[SP][0] as u32), 0u64);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x60), 0u64);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x64));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
        // Store the pass count into the draw context and draw.
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
        c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
        let draw = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, draw, 0x0023_1c60)?;
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, elf_image, DATA, GAME_GP};

    /// cSectionMan: the location (`idx`) and mode (7 = the table's mode)
    /// across the seeds, on the game's own location tables.


    /// cGame::partialRestart: the AI's virtual slot 5 is a fake (the mode-3
    /// call goes to it); every other callee sees the zeroed game object.


    /// cGame::startReplay: the replay counter in gp-0x69c steps up from
    /// seed-dependent values; the replay pointer slot is seeded.


    /// cLoadScreen::drawSplash: the draw context at gp-0x854 has its virtual
    /// draw method at a fake, so the call is logged and returns.
    #[test]
    fn c_load_screen_draw_splash_matches() {
        check(ssx3_ee::s06::f_00231b50__generated, c_load_screen_draw_splash, 200, &|c, seed| {
            machine(c, seed, 0x0023_1b50, 0);
            c.g[A1][0] = (seed % 0x1000) as u64;
            c.bus.write32(GAME_GP - 0x854, DATA);
            c.bus.write32(DATA + 0xe84, DATA + 0x1000);
            c.bus.write32(DATA + 0x10d8, DATA + 0x2000);
            c.bus.write16(DATA + 0x2238, ((seed % 3) * 0x10) as u16);
            c.bus.write32(DATA + 0x223c, FAKE);
        });
    }
}
