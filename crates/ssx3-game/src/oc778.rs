//! Staging module oc778 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc778`.

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
    (
        0x001f_bbd8,
        ov_screen_race_pre_on_activate,
        "OVScreenRacePre_OnActivate",
    ),
    (0x001a_2c40, cfe_real_fe_load_start_state, "FEModule_v08"),
    (0x0038_af30, cps2_bezier_man_init, "PS2BezierMan_Init"),
    (
        0x0023_4750,
        cgf_event_start_state_lose_focus,
        "GameStatePreRaceLoad_Exit",
    ),
    (0x002a_6d78, crowd_audio_load, "CrowdAudio_Load"),
];

/// `void cOVState_HEAT_PREEVENT_onGainFocus(cOVState* self)` (0x1fbbd8, `OVScreenRacePre_OnActivate`).
///
/// The pre-race overlay gains focus: runs the base `cUIState::onGainFocus`,
/// then shows the "RIDERPCT_D" rider name text (`sprintf` + `triggerTextFade`)
/// for human rider 1, and for rider 2 as well when the game path
/// (`newRace.cGamePath`, byte 0x535c11) is 2 (split screen). Finally sets the
/// game-comm overlay timeout (`+0x64`) to 1200 when a `cGameComm` exists.
pub fn ov_screen_race_pre_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fbca0: {
            c.cycles += 38;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            call_ptr(c, 0x0039e4c0, 0x001fbbf8)?;
            call_ptr(c, 0x0014dc80, 0x001fbc00)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x001fbc0c)?;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x001fbc14)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x001fbc20)?;
            c.g[V1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2170));
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x00146e98, 0x001fbc34)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x00417828, 0x001fbc44)?;
            c.g[V1][0] = 0x470000;
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1b18));
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, 0x0020a6f0, 0x001fbc5c)?;
            c.g[V1][0] = 0x530000;
            c.g[A0][0] = 0x2;
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
            let t = c.g[V0][0] != c.g[A0][0];
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            if t {
                break 'b_1fbca0;
            } else {
                c.cycles += 12;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x00146e98, 0x001fbc7c)?;
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00417828, 0x001fbc8c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x0020a6f0, 0x001fbc9c)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                break 'b_1fbca0;
            }
        }
        'b_1fbcac: {
            c.cycles += 2;
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = 0x4b0;
            if t {
                break 'b_1fbcac;
            } else {
                c.cycles += 1;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                break 'b_1fbcac;
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `void cRealFE::loadStartState(cRealFE* self)` (0x1a2c40, `FEModule_v08`).
///
/// Pushes the background front-end state, then, by `cRealFE_mStartState`
/// (`gp-0x172c`): 0 pushes the title state, 0x27 the lodge state (and clears
/// the start state), 0x34 the re-login state (closing the comm channel,
/// resetting voip/lobby and the two profiles, then clearing the start state);
/// any other value pushes nothing.
pub fn cfe_real_fe_load_start_state(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a2e10: {
            'b_1a2e0c: {
                'b_1a2e08: {
                    'b_1a2d5c: {
                        c.cycles += 30;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                        c.g[A1][0] = 0x460000;
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[A2][0] = 0u64;
                        c.g[S1][0] = c.g[A0][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[A3][0] = 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1358));
                        c.g[A0][0] = 0x48;
                        call_ptr(c, 0x00317d70, 0x001a2c6c)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0039e2a0, 0x001a2c7c)?;
                        c.g[V1][0] = 0x470000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x43a8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        let t = c.g[V0][0] as u32;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        call_ptr(c, t, 0x001a2c98)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f238, 0x001a2ca8)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x172c));
                        c.g[V0][0] = 0x34;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 53i64) as u64;
                        if t {
                            break 'b_1a2d5c;
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                break 'b_1a2e0c;
                            } else {
                                'b_1a2d18: {
                                    'b_1a2cd8: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[V0][0] = 0x27;
                                        if t {
                                            break 'b_1a2cd8;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_1a2d18;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                if t {
                                                    break 'b_1a2e10;
                                                } else {
                                                    break 'b_1a2cd8;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 16;
                                    c.g[A1][0] = 0x460000;
                                    c.g[A0][0] = 0x4c;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1d70));
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1718), 0u64);
                                    call_ptr(c, 0x00317d70, 0x001a2cf4)?;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x00194738, 0x001a2d00)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f400, 0x001a2d10)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1a2e0c;
                                    } else {
                                        break 'b_1a2d18;
                                    }
                                }
                                c.cycles += 17;
                                c.g[V0][0] = 0x1;
                                c.g[A1][0] = 0x460000;
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1188));
                                c.g[A0][0] = 0x58;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1718), c.g[V0][0]);
                                call_ptr(c, 0x00317d70, 0x001a2d38)?;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x001f3700, 0x001a2d44)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f400, 0x001a2d54)?;
                                let t = true;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x172c), 0u64);
                                if t {
                                    break 'b_1a2e08;
                                } else {
                                    break 'b_1a2d5c;
                                }
                            }
                        }
                    }
                    c.cycles += 43;
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1370));
                    c.g[A0][0] = 0x6d4;
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1718), 0u64);
                    call_ptr(c, 0x00317d70, 0x001a2d78)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x001d5038, 0x001a2d84)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039f400, 0x001a2d94)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6c));
                    call_ptr(c, 0x00255cc8, 0x001a2da4)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                    call_ptr(c, 0x00266e88, 0x001a2dac)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                    call_ptr(c, 0x00266df8, 0x001a2db4)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0025b6d8, 0x001a2dc0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0x7;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x001a2dd0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014eca0, 0x001a2ddc)?;
                    call_ptr(c, 0x0014dc80, 0x001a2de4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014de28, 0x001a2df0)?;
                    call_ptr(c, 0x0014dc80, 0x001a2df8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0014de28, 0x001a2e04)?;
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x172c), 0u64);
                    break 'b_1a2e08;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1a2e0c;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1a2e10;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void cPS2BezierMan::Init(cPS2BezierMan* self)` (0x38af30).
///
/// Initialises the patch cache (`BezierPatchCache_Init(self + 0x10, 900, 220,
/// 140, 0, 146, 93)`), allocates the three VIF caches (0x38400, 0xdc00,
/// 0x8c00 bytes), sets the render flags and LOD distances, calls the base
/// init (`0x38d638`), and fills the 6x6 (step 0.2) and 8x8 (step 1/7)
/// grid UV tables at 0x5049c0 (`D_005049C0`).
pub fn cps2_bezier_man_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 58;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[T0][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[T1][0] = 0x92;
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[T2][0] = 0x5d;
        c.g[A1][0] = 0x384;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 0xdc;
        c.g[A3][0] = 0x8c;
        call_ptr(c, 0x00372b78, 0x0038af60)?;
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = 0x30000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2eb0));
        c.g[A0][0] = c.g[A0][0] | 0x8400;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x0038af7c)?;
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = 0xdc00;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2ec0));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x440), c.g[V0][0]);
        call_ptr(c, 0x00317e30, 0x0038af98)?;
        c.g[A1][0] = 0x490000;
        c.g[A2][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2ed0));
        c.g[A0][0] = 0x8c00;
        c.g[A3][0] = 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x444), c.g[V0][0]);
        call_ptr(c, 0x00317e30, 0x0038afb4)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x440));
        c.g[V1][0] = 0x1;
        c.g[AT][0] = 0x41000000;
        c.mtc1(1, 2);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x444));
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x26cc));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x26c8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x354), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x44c), c.g[A2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x454), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x350), c.g[V1][0]);
        c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x358));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x448), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x344), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x348), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x34c), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x450), c.g[A1][0]);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x33c));
        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x340));
        call_ptr(c, 0x0038d638, 0x0038b008)?;
        c.g[V0][0] = 0x500000;
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x26c4));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4a00));
        c.g[V1][0] = 0u64;
        'l_38b018: loop {
            c.cycles += 8;
            c.mtc1(3, 0);
            c.cvt_s_w(0, 0);
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = 0u64;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.mul_s(1, 0, 2);
            'l_38b038: loop {
                c.cycles += 13;
                c.mtc1(4, 0);
                c.cvt_s_w(0, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < 6i64) as u64;
                c.mul_s(0, 0, 2);
                c.swc1(0, c.g[SP][0] as u32);
                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(7, c.g[SP][0] as u32);
                c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.sdr(7, c.g[V1][0] as u32);
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0038b038);
                    }
                    continue 'l_38b038;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[A1][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 6i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x500000;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0038b018);
                        }
                        continue 'l_38b018;
                    } else {
                        c.cycles += 3;
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x26c0));
                        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4b80));
                        c.g[V1][0] = 0u64;
                        'l_38b088: loop {
                            c.cycles += 8;
                            c.mtc1(3, 0);
                            c.cvt_s_w(0, 0);
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.g[A0][0] = 0u64;
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.mul_s(1, 0, 2);
                            'l_38b0a8: loop {
                                c.cycles += 13;
                                c.mtc1(4, 0);
                                c.cvt_s_w(0, 0);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[A0][0] as i64) < 8i64) as u64;
                                c.mul_s(0, 0, 2);
                                c.swc1(0, c.g[SP][0] as u32);
                                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                                c.ldr(7, c.g[SP][0] as u32);
                                c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                c.sdr(7, c.g[V1][0] as u32);
                                let t = c.g[V0][0] != 0u64;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0038b0a8);
                                    }
                                    continue 'l_38b0a8;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.g[A1][0];
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 8i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0038b088);
                                        }
                                        continue 'l_38b088;
                                    } else {
                                        c.cycles += 3;
                                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                                        let t = c.g[RA][0] as u32;
                                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
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
    });
}

/// `void cGFEventStartState::loseFocus(cGFEventStartState* self)` (0x234750,
/// `GameStatePreRaceLoad_Exit`).
///
/// Leaving the pre-race load state: by the game mode (byte 0x535c10) it
/// tears down the AI/camera/race managers (modes 0 vs the others), resets
/// the race manager's view slots (`0x128a48` with 0, and 1 or 2 by mode),
/// runs the mission and script cleanup, the optional NIS cleanup (byte
/// 0x535c11), clears `self + 0x14`, and, for a single view, resets one
/// slot-table entry of the world.
pub fn cgf_event_start_state_lose_focus(c: &mut Cpu) {
    guest(c, |c| {
        'b_2347d8: {
            'b_2347c0: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x002790a0, 0x00234778)?;
                call_ptr(c, 0x0014dc80, 0x00234780)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x0023478c)?;
                c.g[V1][0] = 0x530000;
                c.g[S3][0] = c.g[V1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                c.g[S0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
                let t = c.g[S0][0] != 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                if t {
                    break 'b_2347c0;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x00128958, 0x002347ac)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x001289f0, 0x002347b8)?;
                    let t = true;
                    c.g[S1][0] = 0x4;
                    if t {
                        break 'b_2347d8;
                    } else {
                        break 'b_2347c0;
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x00128a10, 0x002347c8)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x0026f7b8, 0x002347d4)?;
            c.g[S1][0] = 0x4;
            break 'b_2347d8;
        }
        'b_23480c: {
            c.cycles += 2;
            let t = c.g[S0][0] == c.g[S1][0];
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            if t {
                break 'b_23480c;
            } else {
                c.cycles += 11;
                c.g[A1][0] = 0x3;
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002347fc)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                c.g[V1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_23480c;
            }
        }
        'b_234858: {
            'b_234844: {
                'b_234830: {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x00128a48, 0x0023481c)?;
                    c.g[V0][0] = 0x5;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    if t {
                        break 'b_234830;
                    } else {
                        c.cycles += 2;
                        let t = c.g[S0][0] != 0u64;

                        if t {
                            break 'b_234844;
                        } else {
                            break 'b_234830;
                        }
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x00128a48, 0x0023483c)?;
                let t = true;

                if t {
                    break 'b_234858;
                } else {
                    break 'b_234844;
                }
            }
            c.cycles += 2;
            let t = c.g[S0][0] == c.g[S1][0];
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            if t {
                break 'b_234858;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0x2;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x00128a48, 0x00234858)?;
                break 'b_234858;
            }
        }
        'b_2348b8: {
            c.cycles += 19;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00308f38, 0x00234860)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00308c60, 0x00234868)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A1][0] = 0x3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), 0u64);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x00113b10, 0x00234880)?;
            c.g[A0][0] = 0x1;
            call_ptr(c, 0x0014de68, 0x00234888)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x00129160, 0x00234894)?;
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
            c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
            if c.g[V1][0] == 0u64 {
                c.sw((c.g[S2][0] as u32).wrapping_add(0x14), 0u64);
                break 'b_2348b8;
            } else {
                c.cycles += 5;
                call_ptr(c, 0x00155a50, 0x002348ac)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00155e58, 0x002348b4)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x14), 0u64);
                break 'b_2348b8;
            }
        }
        'b_2348f8: {
            c.cycles += 7;
            c.g[V0][0] = 0x1;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
            let t = c.g[V1][0] != c.g[V0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            if t {
                break 'b_2348f8;
            } else {
                c.cycles += 9;
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = 0x50;
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xb0));
                c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                c.mult(0, 2, 2, 6);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x250), 0u64);
                break 'b_2348f8;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void CrowdAudio_Load(CrowdSys* self, char* name, char* ext, int n, int a4)` (0x2a6d78).
///
/// Allocates `n` crowd voices (0x190 bytes each, `"CrowdInstances"`), clears
/// the three crowd entries and their curves (`0x2a4cf8`), initialises every
/// voice (`CrowdAudio_InitInstance`) with the cheer bank (`a4`), then parses
/// the `.inf` file `name`: the GLOBAL section and the section `ext` (each
/// through `0x2a7040` when found).
pub fn crowd_audio_load(c: &mut Cpu) {
    guest(c, |c| {
        'b_2a6e0c: {
            c.cycles += 31;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x8c0));
            c.g[V0][0] = 0x190;
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x840));
            c.g[S7][0] = c.g[A3][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x850));
            c.mult(0, 2, 23, 2);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x810), c.g[A1][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x8b0));
            c.g[S6][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x8a0));
            c.g[A1][0] = 0x480000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x890));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3108));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x870));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x814), c.g[A2][0]);
            c.g[A3][0] = 0u64;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x880));
            c.g[A2][0] = 0u64;
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x860));
            c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add(0x18));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x830));
            c.g[S1][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x1));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x820), c.g[RA][0]);
            c.sw((c.g[S6][0] as u32).wrapping_add(0x14), c.g[S7][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x818), c.g[T0][0]);
            call_ptr(c, 0x00317e30, 0x002a6de4)?;
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            c.sw(c.g[V0][0] as u32, c.g[S7][0]);
            let t = c.g[S7][0] == 0u64;
            c.g[S0][0] = c.g[S2][0];
            if t {
                break 'b_2a6e0c;
            } else {
                c.cycles += 1;
                c.g[S3][0] = 0xffffffffffffffff;
                'l_2a6df8: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    call_ptr(c, 0x002a4d68, 0x002a6e04)?;
                    let t = c.g[S1][0] != c.g[S3][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x190));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002a6df8);
                        }
                        continue 'l_2a6df8;
                    } else {
                        break 'b_2a6e0c;
                    }
                }
            }
        }
        c.cycles += 9;
        c.sw(c.g[S4][0] as u32, c.g[S2][0]);
        c.g[S5][0] = sx((c.g[S6][0] as u32).wrapping_add(0x34));
        c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4c));
        c.g[S3][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1c));
        c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6c));
        c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x68));
        c.g[S1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x64));
        c.g[FP][0] = sx((c.g[S6][0] as u32).wrapping_add(0x168));
        'l_2a6e30: loop {
            c.cycles += 17;
            c.sw(c.g[S1][0] as u32, 0u64);
            c.g[A0][0] = c.g[S4][0];
            c.sw(c.g[S2][0] as u32, 0u64);
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x54));
            c.sw(c.g[S0][0] as u32, 0u64);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x54));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x54));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x54));
            call_ptr(c, 0x002a4cf8, 0x002a6e54)?;
            c.g[A0][0] = c.g[S5][0];
            call_ptr(c, 0x002a4cf8, 0x002a6e5c)?;
            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x54));
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x002a4cf8, 0x002a6e68)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[FP][0] as i64)) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x54));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002a6e30);
                }
                continue 'l_2a6e30;
            } else {
                'b_2a6e9c: {
                    c.cycles += 2;
                    let t = (c.g[S7][0] as i64) <= 0;
                    c.g[A3][0] = c.g[S7][0];
                    if t {
                        break 'b_2a6e9c;
                    } else {
                        c.cycles += 1;
                        c.g[V1][0] = 0u64;
                        'l_2a6e80: loop {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x18));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x818));
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0xd8), c.g[A0][0]);
                            let t = c.g[A3][0] != 0u64;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x190));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002a6e80);
                                }
                                continue 'l_2a6e80;
                            } else {
                                break 'b_2a6e9c;
                            }
                        }
                    }
                }
                'b_2a6ed4: {
                    c.cycles += 12;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x810));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = 0x100;
                    call_ptr(c, 0x002a77f8, 0x002a6eac)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3570));
                    c.g[A0][0] = c.g[SP][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    call_ptr(c, 0x002a7bf8, 0x002a6ec4)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S6][0];
                    if t {
                        break 'b_2a6ed4;
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x002a7040, 0x002a6ed4)?;
                        break 'b_2a6ed4;
                    }
                }
                'b_2a6ef8: {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x814));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    call_ptr(c, 0x002a7bf8, 0x002a6ee8)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S6][0];
                    if t {
                        break 'b_2a6ef8;
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x002a7040, 0x002a6ef8)?;
                        break 'b_2a6ef8;
                    }
                }
                c.cycles += 15;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x002a7890, 0x002a6f04)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x8b0));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x8a0));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x890));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x880));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x870));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x860));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x850));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x840));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x830));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x820));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8c0));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc778's callee fakes in a machine's RAM.
    pub(crate) const OC778_CTRL: u32 = DATA + 0x30_0000;
    pub(crate) const OC778_MAGIC: u32 = 0x0778_f4ce;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc778_mail(addr: u32) -> u32 {
        DATA + 0x30_1000 + (addr & 0xffc)
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc778_arm(c: &mut Cpu) {
        c.bus.write32(OC778_CTRL, OC778_MAGIC);
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc778_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc778_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc778_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc778_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                2 => a0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc778_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC778_CTRL) != OC778_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc778_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc778_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc778_fakes!(
        (
            oc778_fk_146e98,
            OC778_PREV_00146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x0014_6e98
        ),
        (
            oc778_fk_14dc80,
            OC778_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc778_fk_14dd58,
            OC778_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc778_fk_14de28,
            OC778_PREV_0014DE28,
            ee::s01::OVERRIDE_0014DE28,
            ee::s01::f_0014de28__generated,
            0x0014_de28
        ),
        (
            oc778_fk_14de68,
            OC778_PREV_0014DE68,
            ee::s01::OVERRIDE_0014DE68,
            ee::s01::f_0014de68__generated,
            0x0014_de68
        ),
        (
            oc778_fk_14eca0,
            OC778_PREV_0014ECA0,
            ee::s01::OVERRIDE_0014ECA0,
            ee::s01::f_0014eca0__generated,
            0x0014_eca0
        ),
        (
            oc778_fk_155a50,
            OC778_PREV_00155A50,
            ee::s01::OVERRIDE_00155A50,
            ee::s01::f_00155a50__generated,
            0x0015_5a50
        ),
        (
            oc778_fk_155e58,
            OC778_PREV_00155E58,
            ee::s01::OVERRIDE_00155E58,
            ee::s01::f_00155e58__generated,
            0x0015_5e58
        ),
        (
            oc778_fk_113b10,
            OC778_PREV_00113B10,
            ee::s00::OVERRIDE_00113B10,
            ee::s00::f_00113b10__generated,
            0x0011_3b10
        ),
        (
            oc778_fk_128958,
            OC778_PREV_00128958,
            ee::s00::OVERRIDE_00128958,
            ee::s00::f_00128958__generated,
            0x0012_8958
        ),
        (
            oc778_fk_1289f0,
            OC778_PREV_001289F0,
            ee::s00::OVERRIDE_001289F0,
            ee::s00::f_001289f0__generated,
            0x0012_89f0
        ),
        (
            oc778_fk_128a10,
            OC778_PREV_00128A10,
            ee::s00::OVERRIDE_00128A10,
            ee::s00::f_00128a10__generated,
            0x0012_8a10
        ),
        (
            oc778_fk_128a48,
            OC778_PREV_00128A48,
            ee::s00::OVERRIDE_00128A48,
            ee::s00::f_00128a48__generated,
            0x0012_8a48
        ),
        (
            oc778_fk_129160,
            OC778_PREV_00129160,
            ee::s00::OVERRIDE_00129160,
            ee::s00::f_00129160__generated,
            0x0012_9160
        ),
        (
            oc778_fk_194738,
            OC778_PREV_00194738,
            ee::s02::OVERRIDE_00194738,
            ee::s02::f_00194738__generated,
            0x0019_4738
        ),
        (
            oc778_fk_1d5038,
            OC778_PREV_001D5038,
            ee::s04::OVERRIDE_001D5038,
            ee::s04::f_001d5038__generated,
            0x001d_5038
        ),
        (
            oc778_fk_1f3700,
            OC778_PREV_001F3700,
            ee::s04::OVERRIDE_001F3700,
            ee::s04::f_001f3700__generated,
            0x001f_3700
        ),
        (
            oc778_fk_20a6f0,
            OC778_PREV_0020A6F0,
            ee::s05::OVERRIDE_0020A6F0,
            ee::s05::f_0020a6f0__generated,
            0x0020_a6f0
        ),
        (
            oc778_fk_25b6d8,
            OC778_PREV_0025B6D8,
            ee::s06::OVERRIDE_0025B6D8,
            ee::s06::f_0025b6d8__generated,
            0x0025_b6d8
        ),
        (
            oc778_fk_266df8,
            OC778_PREV_00266DF8,
            ee::s07::OVERRIDE_00266DF8,
            ee::s07::f_00266df8__generated,
            0x0026_6df8
        ),
        (
            oc778_fk_266e88,
            OC778_PREV_00266E88,
            ee::s07::OVERRIDE_00266E88,
            ee::s07::f_00266e88__generated,
            0x0026_6e88
        ),
        (
            oc778_fk_26f7b8,
            OC778_PREV_0026F7B8,
            ee::s07::OVERRIDE_0026F7B8,
            ee::s07::f_0026f7b8__generated,
            0x0026_f7b8
        ),
        (
            oc778_fk_2790a0,
            OC778_PREV_002790A0,
            ee::s07::OVERRIDE_002790A0,
            ee::s07::f_002790a0__generated,
            0x0027_90a0
        ),
        (
            oc778_fk_2a4cf8,
            OC778_PREV_002A4CF8,
            ee::s08::OVERRIDE_002A4CF8,
            ee::s08::f_002a4cf8__generated,
            0x002a_4cf8
        ),
        (
            oc778_fk_2a77f8,
            OC778_PREV_002A77F8,
            ee::s08::OVERRIDE_002A77F8,
            ee::s08::f_002a77f8__generated,
            0x002a_77f8
        ),
        (
            oc778_fk_2a7890,
            OC778_PREV_002A7890,
            ee::s08::OVERRIDE_002A7890,
            ee::s08::f_002a7890__generated,
            0x002a_7890
        ),
        (
            oc778_fk_2a7bf8,
            OC778_PREV_002A7BF8,
            ee::s08::OVERRIDE_002A7BF8,
            ee::s08::f_002a7bf8__generated,
            0x002a_7bf8
        ),
        (
            oc778_fk_308c60,
            OC778_PREV_00308C60,
            ee::s10::OVERRIDE_00308C60,
            ee::s10::f_00308c60__generated,
            0x0030_8c60
        ),
        (
            oc778_fk_308f38,
            OC778_PREV_00308F38,
            ee::s10::OVERRIDE_00308F38,
            ee::s10::f_00308f38__generated,
            0x0030_8f38
        ),
        (
            oc778_fk_317d70,
            OC778_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc778_fk_317e30,
            OC778_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc778_fk_38d638,
            OC778_PREV_0038D638,
            ee::s12::OVERRIDE_0038D638,
            ee::s12::f_0038d638__generated,
            0x0038_d638
        ),
        (
            oc778_fk_39e2a0,
            OC778_PREV_0039E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039_e2a0
        ),
        (
            oc778_fk_39e4c0,
            OC778_PREV_0039E4C0,
            ee::s13::OVERRIDE_0039E4C0,
            ee::s13::f_0039e4c0__generated,
            0x0039_e4c0
        ),
        (
            oc778_fk_39f238,
            OC778_PREV_0039F238,
            ee::s13::OVERRIDE_0039F238,
            ee::s13::f_0039f238__generated,
            0x0039_f238
        ),
        (
            oc778_fk_39f400,
            OC778_PREV_0039F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc778_fk_3e6448,
            OC778_PREV_003E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc778_fk_417828,
            OC778_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
    );

    /// `OVScreenRacePre_OnActivate`: the game path byte (2 = second rider
    /// text), the game-comm pointer (null or set) and the unwinds of its calls.
    #[test]
    fn ov_screen_race_pre_on_activate_matches() {
        oc778_install_fakes();
        fakes::covered(
            ee::s04::f_001fbbd8__generated,
            ov_screen_race_pre_on_activate,
            240,
            &|c, seed| {
                machine(c, seed, 0x778a, 10);
                oc778_arm(c);
                c.g[A0][0] = sx(DATA + 0x1_0000);
                c.bus.write8(0x0053_5c11, [2u8, 0, 1][(seed % 3) as usize]);
                let comm = if (seed / 3) % 2 == 0 {
                    0
                } else {
                    DATA + 0x2_0000
                };
                c.bus.write32(GAME_GP - 0x204, comm);
            },
        );
    }

    /// `FEModule_v08` (`cRealFE::loadStartState`): the start state 0, 0x27,
    /// 0x34 (with the real `0x255cc8` reaching its vtable call or not), below
    /// and above the range, and the unwinds of its calls.
    #[test]
    fn cfe_real_fe_load_start_state_matches() {
        oc778_install_fakes();
        fakes::covered(
            ee::s03::f_001a2c40__generated,
            cfe_real_fe_load_start_state,
            300,
            &|c, seed| {
                machine(c, seed, 0x778b, 14);
                oc778_arm(c);
                let this = DATA + 0x1_0000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0xc, DATA + 0x3_0000);
                // The state vtable image the background state's ctor-call reads.
                c.bus.write16(0x0046_bc78, 0x20);
                c.bus.write32(0x0046_bc7c, fakes::FAKE + 0x10);
                let state = [0x34u32, 0, 0x27, 5, 0x40][(seed % 5) as usize];
                c.bus.write32(GAME_GP - 0x172c, state);
                // State 0x34: `0x255cc8(table, idx)` with the table entry set or not.
                let comm = DATA + 0x4_0000;
                let table = DATA + 0x5_0000;
                let obj = DATA + 0x5_1000;
                let vt = DATA + 0x5_2000;
                let idx = (seed / 5) % 2;
                c.bus.write32(GAME_GP - 0x204, comm);
                c.bus.write32(comm + 0x6c, idx);
                c.bus.write32(GAME_GP - 0x238, table);
                c.bus
                    .write32(table + 4 * idx, if (seed / 10) % 2 == 0 { obj } else { 0 });
                c.bus.write32(obj, 1);
                c.bus.write32(obj + 8, vt);
                c.bus.write16(vt + 0x10, 0x20);
                c.bus.write32(vt + 0x14, fakes::FAKE + 0x20);
            },
        );
    }

    /// `PS2BezierMan_Init`: the real `BezierPatchCache_Init` and the grid
    /// loops over random FPU constants, with the unwinds of its calls.
    #[test]
    fn cps2_bezier_man_init_matches() {
        oc778_install_fakes();
        fakes::covered(
            ee::s12::f_0038af30__generated,
            cps2_bezier_man_init,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x778c, 16);
                oc778_arm(c);
                c.g[A0][0] = sx(DATA + 0x1_0000);
                for off in [0x26ccu32, 0x26c8, 0x26c4, 0x26c0] {
                    let v = r.f32(10.0);
                    c.bus.write32(GAME_GP - off, v);
                }
            },
        );
    }

    /// `GameStatePreRaceLoad_Exit`: the game mode byte (0, 4, 5, other), the
    /// NIS byte, the one-view slot test (`+0x10 == 1`) and the unwinds.
    #[test]
    fn cgf_event_start_state_lose_focus_matches() {
        oc778_install_fakes();
        fakes::covered(
            ee::s06::f_00234750__generated,
            cgf_event_start_state_lose_focus,
            320,
            &|c, seed| {
                machine(c, seed, 0x778d, 14);
                oc778_arm(c);
                c.g[A0][0] = sx(DATA + 0x7_0000);
                c.bus
                    .write8(0x0053_5c10, [0u8, 4, 5, 1][(seed % 4) as usize]);
                c.bus.write8(0x0053_5c11, ((seed / 4) % 2) as u8);
                let game = DATA + 0x1_0000;
                let q = DATA + 0x2_0000;
                let vo = DATA + 0x3_0000;
                c.bus.write32(GAME_GP - 0x488, game);
                c.bus.write32(game + 0xc, q);
                c.bus.write32(q + 0xcc, vo);
                c.bus.write16(vo + 0x28, 0x10);
                c.bus.write32(vo + 0x2c, fakes::FAKE + 0x30);
                let world = DATA + 0x4_0000;
                let a3 = DATA + 0x5_0000;
                let a0p = DATA + 0x6_0000;
                let a1p = DATA + 0x8_0000;
                let table = DATA + 0x9_0000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, a3);
                c.bus.write32(a3 + 0x84, a0p);
                c.bus
                    .write32(a0p + 0x10, if (seed / 8) % 2 == 0 { 1 } else { 2 });
                c.bus.write32(a0p + 4, a1p);
                c.bus
                    .write32(a1p + 0xb0, [0u32, 1, 5][((seed / 16) % 3) as usize]);
                c.bus.write32(a3 + 0x10, table);
                c.bus.write32(table, DATA + 0xa_0000);
            },
        );
    }

    /// `CrowdAudio_Load`: `n` voices (0, 1, 3), the real
    /// `CrowdAudio_InitInstance` per voice, the section lookups forced to
    /// "not found" (the real `0x2a7040` parser is not reached), and the unwinds.
    #[test]
    fn crowd_audio_load_matches() {
        oc778_install_fakes();
        fakes::covered(
            ee::s08::f_002a6d78__generated,
            crowd_audio_load,
            240,
            &|c, seed| {
                machine(c, seed, 0x778f, 14);
                oc778_arm(c);
                oc778_force_zero(c, 0x002a_7bf8);
                c.g[A0][0] = sx(DATA + 0x1_0000);
                c.g[A1][0] = sx(DATA + 0x2_0000);
                c.g[A2][0] = sx(DATA + 0x2_1000);
                c.g[A3][0] = [0u64, 1, 3][(seed % 3) as usize];
            },
        );
    }
}
