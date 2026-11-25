//! Staging module oc674 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc674`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x00232568,
        init_load_screen_enter_load,
        "InitLoadScreen_Enter",
    ),
    (0x00123210, rider_reset_at_matrix, "Rider_ResetAtMatrix"),
    (0x00125108, rider_on_finish_race, "Rider_OnFinishRace"),
    (0x00129c00, ai_create_mission_rider, "AI_CreateMissionRider"),
    (0x00129ff8, ai_create_actor_riders, "AI_CreateActorRiders"),
    (0x0012a2b0, ai_load_track_paths, "AI_LoadTrackPaths"),
    (
        0x0012a560,
        ai_place_riders_at_path_start,
        "AI_PlaceRidersAtPathStart",
    ),
    (
        0x0012a768,
        ai_place_riders_staggered,
        "AI_PlaceRidersStaggered",
    ),
    (
        0x0012e010,
        rider_wipeout_sub_start_get_up,
        "RiderWipeoutSub_StartGetUp",
    ),
    (
        0x0012fc80,
        rider_board_press_sub_handle_input,
        "RiderBoardPressSub_HandleInput",
    ),
];

/// `void InitLoadScreen_Enter(void)` (0x00232568): start the boot load,
/// `cSSXApp_initload(*(gp - 0x848))` (the app object). The pre-existing
/// `init_load_screen_enter` / `_2` in `loadscreens_prestart.rs` are untested
/// duplicates of this: delete them at the reorg.
pub fn init_load_screen_enter_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0022_82d0, 0x0023_257c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_ResetAtMatrix(Rider* self, const t4Matrix* mat)`: put a rider back on a start matrix. Resets its replay events, offsets the matrix translation by the start-grid slot of its AI index (0 to 5, times 100) through the matrix, resets metrics, main/sub state and transitions, places it (`Rider_Place`, keeping its speed along the new forward direction), then resets its effects. Address 0x00123210; every register, cycle and call kept, in the generated body's block structure.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn rider_reset_at_matrix(c: &mut Cpu) {
    guest(c, |c| {
        'b_123328: {
            'b_12331c: {
                'b_123318: {
                    'b_123278: {
                        c.cycles += 21;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[S0][0] = c.g[A1][0];
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.g[S2][0] = c.g[A0][0];
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x86c));
                        c.g[RA][0] = sx(0x00123244);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                        call_ptr(c, 0x00270970, 0x00123244)?;
                        c.lq(2, (c.g[S0][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x86c));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                        c.sq(2, (c.g[SP][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                        let t = c.g[V1][0] != 0u64;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
                        if t {
                            break 'b_123278;
                        } else {
                            c.cycles += 5;
                            c.g[AT][0] = 0x40000000;
                            c.mtc1(1, 0);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                            let t = true;
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_123318;
                            } else {
                                break 'b_123278;
                            }
                        }
                    }
                    'b_123298: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_123298;
                        } else {
                            c.cycles += 5;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                            let t = true;
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_123318;
                            } else {
                                break 'b_123298;
                            }
                        }
                    }
                    'b_123314: {
                        'b_1232b8: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x3;
                            if t {
                                break 'b_1232b8;
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x40000000;
                                c.mtc1(1, 0);
                                c.g[AT][0] = 0xffffffffc0000000;
                                c.mtc1(1, 1);
                                let t = true;
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_123314;
                                } else {
                                    break 'b_1232b8;
                                }
                            }
                        }
                        'b_1232d8: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x4;
                            if t {
                                break 'b_1232d8;
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x40000000;
                                c.mtc1(1, 0);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
                                let t = true;
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_12331c;
                                } else {
                                    break 'b_1232d8;
                                }
                            }
                        }
                        'b_1232f8: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x5;
                            if t {
                                break 'b_1232f8;
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 0);
                                c.g[AT][0] = 0xffffffffc0000000;
                                c.mtc1(1, 1);
                                let t = true;
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_123314;
                                } else {
                                    break 'b_1232f8;
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];

                        if t {
                            break 'b_123328;
                        } else {
                            c.cycles += 5;
                            c.g[AT][0] = 0x40000000;
                            c.mtc1(1, 0);
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 1);
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_123314;
                        }
                    }
                    c.cycles += 1;
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                    break 'b_123318;
                }
                c.cycles += 1;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                break 'b_12331c;
            }
            c.cycles += 3;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_123328;
        }
        c.cycles += 106;
        c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lqc2(4, (c.g[S0][0] as u32));
        c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[S0][0] as u32).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[AT][0] = 0x42c80000;
        c.mtc1(1, 0);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = 0x1;
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.mfc1(3, 0);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = 0x500000;
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
        c.lqc2(8, (c.g[V0][0] as u32));
        c.lqc2(4, (c.g[S0][0] as u32));
        c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[S0][0] as u32).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x790));
        c.g[RA][0] = sx(0x001233d4);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        Callee {
            addr: 0x00119368,
            f: ee::s00::RiderMetrix_OnRiderReset,
        }
        .call(c, 0x001233d4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001233e0);
        c.g[A1][0] = 0u64;
        Callee {
            addr: 0x0011fe78,
            f: ee::s00::Rider_SetMainState,
        }
        .call(c, 0x001233e0)?;
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x001233ec);
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x0011fec8,
            f: ee::s00::Rider_SetSubState,
        }
        .call(c, 0x001233ec)?;
        c.g[RA][0] = sx(0x001233f4);
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x00125038,
            f: ee::s00::Rider_StopTransition,
        }
        .call(c, 0x001233f4)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6c0));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6c0));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
        c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa0));
        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[V1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x18));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0012342c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        call_ptr(c, t, 0x0012342c)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[A1][0] = c.g[V0][0];
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0012343c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, t, 0x0012343c)?;
        c.mtc1(0, 12);
        c.lqc2(3, (c.g[S2][0] as u32).wrapping_add(0x1e0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 20);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A3][0] = 0x5;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0012347c);
        c.g[A1][0] = c.g[SP][0];
        Callee {
            addr: 0x0011d660,
            f: ee::s00::Rider_Place,
        }
        .call(c, 0x0012347c)?;
        c.lq(2, (c.g[S2][0] as u32).wrapping_add(0x1b0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.mfc1(3, 20);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x77c));
        c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x1e0));
        c.g[RA][0] = sx(0x001234ac);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        Callee {
            addr: 0x00111890,
            f: ee::s00::RiderBase_ResetFX,
        }
        .call(c, 0x001234ac)?;
        c.g[RA][0] = sx(0x001234b4);
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x88c));
        call_ptr(c, 0x002f1a00, 0x001234b4)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void Rider_OnFinishRace(Rider* self)`: finish-line event. Flags the race-end data of the world's player record, tells the metrics, extends the finish time, notifies the game mode, starts the post-race camera for the first two players, and informs the audio system. Address 0x00125108; every register, cycle and call kept, in the generated body's block structure.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn rider_on_finish_race(c: &mut Cpu) {
    guest(c, |c| {
        'b_125154: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            let t = c.g[V1][0] == 0u64;
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            if t {
                break 'b_125154;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x480));
                let t = c.g[V0][0] != 0u64;

                if t {
                    break 'b_125154;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x874));
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        break 'b_125154;
                    } else {
                        'b_125150: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x87c));
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_125150;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[V1][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                                if t {
                                    break 'b_125154;
                                } else {
                                    break 'b_125150;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
                        break 'b_125154;
                    }
                }
            }
        }
        'b_12518c: {
            c.cycles += 13;
            c.g[RA][0] = sx(0x0012515c);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x790));
            Callee {
                addr: 0x001193e0,
                f: ee::s00::RiderMetrix_OnRaceFinished,
            }
            .call(c, 0x0012515c)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.mtc1(0, 1);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x470));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x47c));
            c.c_lt_s(0, 1);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            let t = !c.fcc();
            c.sw((c.g[S1][0] as u32).wrapping_add(0x478), c.g[V0][0]);
            if t {
                break 'b_12518c;
            } else {
                c.cycles += 1;
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x470));
                break 'b_12518c;
            }
        }
        'b_1251ec: {
            c.cycles += 17;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c0));
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c0));
            c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001251ac);
            c.g[S0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc0));
            call_ptr(c, t, 0x001251ac)?;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x790));
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001251c0);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xfc));
            call_ptr(c, 0x00238358, 0x001251c0)?;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x870));
            c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_1251ec;
            } else {
                c.cycles += 7;
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[RA][0] = sx(0x001251ec);
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
                call_ptr(c, 0x00162258, 0x001251ec)?;
                break 'b_1251ec;
            }
        }
        'b_125210: {
            c.cycles += 9;
            c.g[RA][0] = sx(0x001251f4);
            call_ptr(c, 0x0028b180, 0x001251f4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00125200);
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x00286ea0, 0x00125200)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x480));
            c.g[V0][0] = 0x1;
            if c.g[V1][0] == c.g[V0][0] {
                c.sw((c.g[S1][0] as u32).wrapping_add(0x100), 0u64);
                break 'b_125210;
            } else {
                break 'b_125210;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void AI_CreateMissionRider(AI* self)`: build the one mission rider (if none was loaded yet): allocate and construct it, scale its model by 0.5, set stat level 5, register it in the rider table, add focus box and metrics, load it and assign its player slot. Address 0x00129c00; every register, cycle and call kept, in the generated body's block structure.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn ai_create_mission_rider(c: &mut Cpu) {
    guest(c, |c| {
        'b_129e04: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x88));
            let t = (c.g[V0][0] as i64) > 0;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
            if t {
                break 'b_129e04;
            } else {
                'b_129da0: {
                    c.cycles += 92;
                    c.g[RA][0] = sx(0x00129c34);
                    call_ptr(c, 0x0014dc80, 0x00129c34)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00129c40);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dd58, 0x00129c40)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S4][0] = c.g[V0][0];
                    c.g[A1][0] = 0x450000;
                    c.g[A3][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[A2][0] = 0xffffffff80000000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7f20));
                    c.g[S2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x78));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[RA][0] = sx(0x00129c6c);
                    c.g[A0][0] = 0x1950;
                    call_ptr(c, 0x00317d70, 0x00129c6c)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xe10));
                    c.g[A0][0] = c.g[V0][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xd20), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc70), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xb40), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xb00), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xaf0), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xad0), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x9c0), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x610), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x520), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x470), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x3b0), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x3a0), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x398), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x384), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x364), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x358), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x340), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x2b8), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x2a4), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x288), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x224), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1f0), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1e4), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1b0), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x100), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    c.g[RA][0] = sx(0x00129cf8);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    Callee {
                        addr: 0x0011b698,
                        f: ee::s00::Rider_ctor,
                    }
                    .call(c, 0x00129cf8)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00129d04);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00125c70,
                        f: ee::s00::Player_ctor,
                    }
                    .call(c, 0x00129d04)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = 0x460000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7dc0));
                    c.g[V1][0] = 0x460000;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x6e8), c.g[V0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6470));
                    c.g[A0][0] = 0x460000;
                    c.g[A1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7d78));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7cc8));
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 12);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[V1][0]);
                    c.mov_s(13, 12);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[A0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xde8), c.g[A1][0]);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xdf4), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xdf0), c.g[V0][0]);
                    c.g[RA][0] = sx(0x00129d64);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x780));
                    call_ptr(c, 0x0030dba0, 0x00129d64)?;
                    c.g[RA][0] = sx(0x00129d6c);
                    call_ptr(c, 0x0014dc80, 0x00129d6c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00129d78);
                    c.g[A1][0] = 0x3;
                    call_ptr(c, 0x0014dd58, 0x00129d78)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00129d88);
                    c.g[A2][0] = 0x5;
                    call_ptr(c, 0x00148aa8, 0x00129d88)?;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    let t = c.g[S1][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_129da0;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        break 'b_129da0;
                    }
                }
                c.cycles += 25;
                c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x86c), c.g[S2][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x870), 0u64);
                c.g[RA][0] = sx(0x00129db8);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                Callee {
                    addr: 0x0011bb40,
                    f: ee::s00::Rider_CreateFocusBox,
                }
                .call(c, 0x00129db8)?;
                c.g[RA][0] = sx(0x00129dc0);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                Callee {
                    addr: 0x0011bb98,
                    f: ee::s00::Rider_CreateRiderMetrix,
                }
                .call(c, 0x00129dc0)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                c.g[RA][0] = sx(0x00129dcc);
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x0011c230,
                    f: ee::s00::Rider_Load,
                }
                .call(c, 0x00129dcc)?;
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x00129de0);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x00144d70, 0x00129de0)?;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x88));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                c.sw((c.g[S3][0] as u32).wrapping_add(0x78), c.g[S2][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x88));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x88), c.g[V0][0]);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                break 'b_129e04;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
        c.pc = t;
        return Ok(());
    });
}

/// `void AI_CreateActorRiders(AI* self)`: build the computer actor riders still required (`required - allocated`): allocate each, register it in the rider table, set its controller/character on the new-player interface, add metrics and record it in the actor table. Address 0x00129ff8; every register, cycle and call kept, in the generated body's block structure.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn ai_create_actor_riders(c: &mut Cpu) {
    guest(c, |c| {
        'b_12a138: {
            c.cycles += 22;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x0012a02c);
            c.g[S0][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x0012a02c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0012a038);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x0012a038)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x94));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
            let t = (c.g[V1][0] as i64) <= 0;
            c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
            if t {
                break 'b_12a138;
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[S6][0] = c.g[V1][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.g[FP][0] = 0x450000;
                'l_12a068: loop {
                    'b_12a0ac: {
                        c.cycles += 16;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = 0x1a90;
                        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x7f40));
                        c.g[A2][0] = 0xffffffff80000000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A3][0] = 0u64;
                        c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[RA][0] = sx(0x0012a08c);
                        c.g[S5][0] = c.lw((c.g[V0][0] as u32));
                        call_ptr(c, 0x00317d70, 0x0012a08c)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0x1;
                        c.g[RA][0] = sx(0x0012a09c);
                        c.g[A2][0] = 0x1;
                        Callee {
                            addr: 0x00108c80,
                            f: ee::s00::ComputerActorRider_ctor,
                        }
                        .call(c, 0x0012a09c)?;
                        c.g[S1][0] = c.g[V0][0];
                        let t = c.g[S1][0] == 0u64;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_12a0ac;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            break 'b_12a0ac;
                        }
                    }
                    c.cycles += 35;
                    c.sw((c.g[S4][0] as u32), c.g[V0][0]);
                    c.g[A1][0] = c.g[S2][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x86c), c.g[S2][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = 0xffffffffffffffff;
                    c.g[RA][0] = sx(0x0012a0c8);
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1));
                    call_ptr(c, 0x00147220, 0x0012a0c8)?;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x0012a0d8);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x00147338, 0x0012a0d8)?;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0012a0ec);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x001473d0, 0x0012a0ec)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0012a100);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0012a100)?;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32));
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x870), c.g[V1][0]);
                    c.g[RA][0] = sx(0x0012a118);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    Callee {
                        addr: 0x0011bb98,
                        f: ee::s00::Rider_CreateRiderMetrix,
                    }
                    .call(c, 0x0012a118)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x5c), c.g[S1][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    let t = c.g[S6][0] != 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x84), c.g[V1][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0012a068);
                        }
                        continue 'l_12a068;
                    } else {
                        break 'b_12a138;
                    }
                }
            }
        }
        c.cycles += 18;
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0012a14c);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0012a14c)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[S2][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `void AI_LoadTrackPaths(AI* self, const char* file)`: free the track path data, load the path file and initialise every rider's track paths. Address 0x0012a2b0; every register, cycle and call kept, in the generated body's block structure.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn ai_load_track_paths(c: &mut Cpu) {
    guest(c, |c| {
        'b_12a320: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = 0x4d0000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x33a0));
            c.g[S3][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0012a2e4);
            c.g[S2][0] = 0u64;
            call_ptr(c, 0x0026af00, 0x0012a2e4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0012a2f0);
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0026ada8, 0x0012a2f0)?;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x78));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x28));
            if t {
                break 'b_12a320;
            } else {
                c.cycles += 1;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                'l_12a300: loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x0012a310);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    Callee {
                        addr: 0x00112180,
                        f: ee::s00::Rider_InitTrackPaths,
                    }
                    .call(c, 0x0012a310)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x78));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                        if event_due(c) {
                            deliver_event_here(c, 0x0012a300);
                        }
                        continue 'l_12a300;
                    } else {
                        break 'b_12a320;
                    }
                }
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void AI_PlaceRidersAtPathStart(AI* self, int which)`: find the first track path with a finish-line marker and place the rider(s) (all if `which == -1`) 1000 units before it, facing along the path. Address 0x0012a560; every register, cycle and call kept, in the generated body's block structure.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn ai_place_riders_at_path_start(c: &mut Cpu) {
    guest(c, |c| {
        'b_12a740: {
            'b_12a73c: {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
                c.g[V0][0] = 0x4d0000;
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[S5][0] = c.g[V0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x33a0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S3][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[S4][0] = c.g[A1][0];
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                let t = (c.g[S2][0] as i64) <= 0;
                c.g[S0][0] = 0u64;
                if t {
                    break 'b_12a73c;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x33a0));
                    'l_12a5b0: loop {
                        'b_12a730: {
                            c.cycles += 12;
                            c.g[RA][0] = sx(0x0012a5b8);
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, 0x0026af70, 0x0012a5b8)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0012a5c4);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x0026a138, 0x0012a5c4)?;
                            c.mov_s(12, 0);
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 0);
                            c.c_eq_s(12, 0);
                            if c.fcc() {
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                break 'b_12a730;
                            } else {
                                'b_12a600: {
                                    c.cycles += 8;
                                    c.g[AT][0] = 0x447a0000;
                                    c.mtc1(1, 0);
                                    c.mtc1(0, 1);
                                    c.sub_s(20, 12, 0);
                                    c.c_lt_s(20, 1);
                                    if c.fcc() {
                                        c.mov_s(20, 1);
                                        break 'b_12a600;
                                    } else {
                                        break 'b_12a600;
                                    }
                                }
                                c.cycles += 13;
                                c.g[A0][0] = c.g[SP][0];
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = 0u64;
                                c.g[RA][0] = sx(0x0012a614);
                                c.g[S2][0] = 0u64;
                                call_ptr(c, 0x0026ab20, 0x0012a614)?;
                                c.g[A1][0] = c.g[S1][0];
                                c.mov_s(12, 20);
                                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x0012a628);
                                c.g[A2][0] = 0u64;
                                call_ptr(c, 0x0026ab20, 0x0012a628)?;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x78));
                                let t = (c.g[V0][0] as i64) <= 0;
                                c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_12a73c;
                                } else {
                                    c.cycles += 5;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 20);
                                    c.g[S5][0] = 0xffffffffffffffff;
                                    c.g[S1][0] = 0u64;
                                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x28));
                                    'l_12a648: loop {
                                        'b_12a714: {
                                            'b_12a65c: {
                                                c.cycles += 2;
                                                if c.g[S4][0] == c.g[S5][0] {
                                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                                    break 'b_12a65c;
                                                } else {
                                                    c.cycles += 2;
                                                    if c.g[S4][0] != c.g[S2][0] {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x78));
                                                        break 'b_12a714;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                                        break 'b_12a65c;
                                                    }
                                                }
                                            }
                                            'b_12a6f8: {
                                                'b_12a6e8: {
                                                    c.cycles += 26;
                                                    c.g[RA][0] = sx(0x0012a664);
                                                    c.g[A1][0] = 0u64;
                                                    Callee {
                                                        addr: 0x0011fec8,
                                                        f: ee::s00::Rider_SetSubState,
                                                    }
                                                    .call(c, 0x0012a664)?;
                                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                                    c.g[RA][0] = sx(0x0012a670);
                                                    c.g[A1][0] = 0u64;
                                                    Callee {
                                                        addr: 0x0011fe78,
                                                        f: ee::s00::Rider_SetMainState,
                                                    }
                                                    .call(c, 0x0012a670)?;
                                                    c.lqc2(3, (c.g[SP][0] as u32));
                                                    c.lqc2(
                                                        4,
                                                        (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                                    c.sqc2(
                                                        5,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.g[A0][0] =
                                                        sx((c.g[S3][0] as u32).wrapping_add(0x28));
                                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                    c.lqc2(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.vu0().sqrt(4, 0);
                                                    c.vu0().waitq();
                                                    c.cfc2(3, 22);
                                                    c.mtc1(3, 0);
                                                    c.c_lt_s(20, 0);
                                                    if !c.fcc() {
                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                                        break 'b_12a6e8;
                                                    } else {
                                                        c.cycles += 9;
                                                        c.mfc1(2, 0);
                                                        c.qmtc2(2, 3);
                                                        c.vu0().div(0, 3, 3, 0);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.vu0().waitq();
                                                        c.vu0().mul(0b1111, 5, 4, Src::Q);
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        let t = true;
                                                        c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add((c.g[S1][0] as u32)));
                                                        if t {
                                                            break 'b_12a6f8;
                                                        } else {
                                                            break 'b_12a6e8;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x1b0));
                                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                    .wrapping_add((c.g[S1][0] as u32)));
                                                break 'b_12a6f8;
                                            }
                                            c.cycles += 7;
                                            c.mtc1(0, 12);
                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                            c.g[A1][0] = c.g[S6][0];
                                            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                            c.g[RA][0] = sx(0x0012a710);
                                            c.g[A3][0] = 0x5;
                                            Callee {
                                                addr: 0x0011d660,
                                                f: ee::s00::Rider_Place,
                                            }
                                            .call(c, 0x0012a710)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x78));
                                            break 'b_12a714;
                                        }
                                        c.cycles += 5;
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] =
                                            ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0012a648);
                                            }
                                            continue 'l_12a648;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                            if t {
                                                break 'b_12a740;
                                            } else {
                                                break 'b_12a730;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x33a0));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0012a5b0);
                            }
                            continue 'l_12a5b0;
                        } else {
                            break 'b_12a73c;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            break 'b_12a740;
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        return Ok(());
    });
}

/// `void AI_PlaceRidersStaggered(AI* self, int n)`: like `AI_PlaceRidersAtPathStart` but stagger the riders: each is put another 1000 units back (counting `n` down) and gets a penalty time of `n * 3600 + 3600`. Address 0x0012a768; every register, cycle and call kept, in the generated body's block structure.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn ai_place_riders_staggered(c: &mut Cpu) {
    guest(c, |c| {
        'b_12a968: {
            'b_12a964: {
                c.cycles += 20;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
                c.g[V0][0] = 0x4d0000;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[S3][0] = c.g[V0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x33a0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S2][0] = c.g[A1][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S5][0] = c.g[A0][0];
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc8));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                let t = (c.g[S1][0] as i64) <= 0;
                c.g[S0][0] = 0u64;
                if t {
                    break 'b_12a964;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x33a0));
                    'l_12a7c0: loop {
                        'b_12a958: {
                            c.cycles += 12;
                            c.g[RA][0] = sx(0x0012a7c8);
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, 0x0026af70, 0x0012a7c8)?;
                            c.g[S4][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0012a7d4);
                            c.g[A0][0] = c.g[S4][0];
                            call_ptr(c, 0x0026a138, 0x0012a7d4)?;
                            c.mov_s(21, 0);
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 0);
                            c.c_eq_s(21, 0);
                            if c.fcc() {
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                break 'b_12a958;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x78));
                                let t = (c.g[V0][0] as i64) <= 0;
                                c.g[S3][0] = 0u64;
                                if t {
                                    break 'b_12a964;
                                } else {
                                    c.cycles += 3;
                                    c.g[AT][0] = 0x447a0000;
                                    c.mtc1(1, 22);
                                    c.g[S6][0] = 0u64;
                                    'l_12a808: loop {
                                        c.cycles += 2;
                                        let t = c.g[S2][0] == 0u64;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                        if t {
                                            break 'b_12a968;
                                        } else {
                                            'b_12a838: {
                                                c.cycles += 9;
                                                c.mtc1(18, 0);
                                                c.cvt_s_w(0, 0);
                                                c.mtc1(0, 1);
                                                c.mul_s(0, 0, 22);
                                                c.sub_s(20, 21, 0);
                                                c.c_lt_s(20, 1);
                                                let t = !c.fcc();
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                if t {
                                                    break 'b_12a838;
                                                } else {
                                                    c.cycles += 1;
                                                    c.mov_s(20, 1);
                                                    break 'b_12a838;
                                                }
                                            }
                                            'b_12a928: {
                                                'b_12a918: {
                                                    c.cycles += 47;
                                                    c.g[A0][0] = c.g[SP][0];
                                                    c.g[A1][0] = c.g[S4][0];
                                                    c.mov_s(12, 21);
                                                    c.g[RA][0] = sx(0x0012a84c);
                                                    c.g[A2][0] = 0u64;
                                                    call_ptr(c, 0x0026ab20, 0x0012a84c)?;
                                                    c.g[S0][0] =
                                                        sx((c.g[S5][0] as u32).wrapping_add(0x28));
                                                    c.g[A0][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                                    c.mov_s(12, 20);
                                                    c.g[A2][0] = 0u64;
                                                    c.g[RA][0] = sx(0x0012a864);
                                                    c.g[A1][0] = c.g[S4][0];
                                                    call_ptr(c, 0x0026ab20, 0x0012a864)?;
                                                    c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                                                    c.g[A1][0] = 0u64;
                                                    c.g[S1][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.g[RA][0] = sx(0x0012a878);
                                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                                    Callee {
                                                        addr: 0x0011fec8,
                                                        f: ee::s00::Rider_SetSubState,
                                                    }
                                                    .call(c, 0x0012a878)?;
                                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                                    c.g[RA][0] = sx(0x0012a884);
                                                    c.g[A1][0] = 0u64;
                                                    Callee {
                                                        addr: 0x0011fe78,
                                                        f: ee::s00::Rider_SetMainState,
                                                    }
                                                    .call(c, 0x0012a884)?;
                                                    c.g[V0][0] = 0xe10;
                                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                                                    c.mult(0, 2, 18, 2);
                                                    c.g[AT][0] = 0x3f800000;
                                                    c.mtc1(1, 0);
                                                    c.g[A1][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0xe10));
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x47c),
                                                        c.g[V0][0],
                                                    );
                                                    c.lqc2(3, (c.g[SP][0] as u32));
                                                    c.lqc2(
                                                        4,
                                                        (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                                    c.sqc2(
                                                        5,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                    c.lqc2(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.vu0().sqrt(4, 0);
                                                    c.vu0().waitq();
                                                    c.cfc2(3, 22);
                                                    c.mtc1(3, 1);
                                                    c.c_lt_s(0, 1);
                                                    if !c.fcc() {
                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                                                        break 'b_12a918;
                                                    } else {
                                                        c.cycles += 9;
                                                        c.mfc1(2, 1);
                                                        c.qmtc2(2, 3);
                                                        c.vu0().div(0, 3, 3, 0);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.vu0().waitq();
                                                        c.vu0().mul(0b1111, 5, 4, Src::Q);
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        let t = true;
                                                        c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add((c.g[S6][0] as u32)));
                                                        if t {
                                                            break 'b_12a928;
                                                        } else {
                                                            break 'b_12a918;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x1b0));
                                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[S6][0] as u32)));
                                                break 'b_12a928;
                                            }
                                            c.cycles += 10;
                                            c.mtc1(0, 12);
                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                            c.g[A3][0] = 0x5;
                                            c.g[RA][0] = sx(0x0012a940);
                                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                            Callee {
                                                addr: 0x0011d660,
                                                f: ee::s00::Rider_Place,
                                            }
                                            .call(c, 0x0012a940)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S5][0] as u32).wrapping_add(0x78));
                                            c.g[V0][0] =
                                                ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0012a808);
                                                }
                                                continue 'l_12a808;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                                if t {
                                                    break 'b_12a968;
                                                } else {
                                                    break 'b_12a958;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x33a0));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0012a7c0);
                            }
                            continue 'l_12a7c0;
                        } else {
                            break 'b_12a964;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            break 'b_12a968;
        }
        c.cycles += 12;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.pc = t;
        return Ok(());
    });
}

/// `void RiderWipeoutSub_StartGetUp(self)`: start the get-up animation after a wipeout (409, or 410 if the root pose override is not 1): sample its pose, re-anchor the rider position/orientation to its hip bone with the animation's offset, then play it. Address 0x0012e010; every register, cycle and call kept, in the generated body's block structure.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn rider_wipeout_sub_start_get_up(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 278;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1a0));
        c.g[A2][0] = 0x19a;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[A0][0] = c.g[SP][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x150), c.g[RA][0]);
        c.g[S2][0] = 0x199;
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.g[S1][0] = 0x500000;
        c.mtc1(0, 12);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1b18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x150));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x784));
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 20);
        if c.g[V0][0] != 0 {
            c.g[S2][0] = c.g[A2][0];
        }
        c.g[RA][0] = sx(0x0012e068);
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, 0x00312660, 0x0012e068)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0u64;
        c.g[RA][0] = sx(0x0012e080);
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x780));
        call_ptr(c, 0x0030ecd8, 0x0012e080)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x89c));
        c.neg_s(6, 6);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x14c));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.lwc1(4, (c.g[V1][0] as u32).wrapping_add(0x140));
        c.lwc1(3, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(0, (c.g[V0][0] as u32));
        c.mul_s(3, 3, 1);
        c.lwc1(5, (c.g[V1][0] as u32).wrapping_add(0x144));
        c.mul_s(0, 0, 4);
        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.lwc1(4, (c.g[V1][0] as u32).wrapping_add(0x148));
        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.mul_s(2, 2, 5);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.mul_s(1, 1, 4);
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x9c));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x94));
        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x64));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x98));
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.neg_s(3, 3);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.neg_s(0, 0);
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x6c));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x89c));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.lq(4, (c.g[V1][0] as u32));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x11c));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x114));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x118));
        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xed0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().opmula(4, 6);
        c.vu0().opmsub(7, 6, 4);
        c.vu0().mula(0b1110, 5, Src::Bc(0, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 7, Src::Bc(0, W));
        c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.mfc1(3, 20);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xed0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xed0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().opmula(4, 6);
        c.vu0().opmsub(7, 6, 4);
        c.vu0().mula(0b1110, 5, Src::Bc(0, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 7, Src::Bc(0, W));
        c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x120));
        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xed0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x120));
        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().opmula(4, 6);
        c.vu0().opmsub(7, 6, 4);
        c.vu0().mula(0b1110, 5, Src::Bc(0, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 7, Src::Bc(0, W));
        c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x110));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x110));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x120));
        c.g[RA][0] = sx(0x0012e354);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        Callee {
            addr: 0x0011e098,
            f: ee::s00::Rider_BuildWorldMatrix,
        }
        .call(c, 0x0012e354)?;
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xb4));
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xb8));
        c.neg_s(2, 2);
        c.neg_s(0, 0);
        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0xbc));
        c.neg_s(1, 1);
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x11c));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x114));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x118));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xed0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().opmula(4, 6);
        c.vu0().opmsub(7, 6, 4);
        c.vu0().mula(0b1110, 5, Src::Bc(0, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 7, Src::Bc(0, W));
        c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.mfc1(3, 20);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xed0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[RA][0] = sx(0x0012e434);
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x784));
        call_ptr(c, 0x00311bf0, 0x0012e434)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        c.g[A1][0] = c.g[S2][0];
        c.mov_s(12, 20);
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x0012e44c);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        call_ptr(c, 0x003128e8, 0x0012e44c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x150));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x190));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.pc = t;
        return Ok(());
    });
}

/// `void RiderBoardPressSub_HandleInput(self, input)`: per-frame input handling of the board-press state. Runs the checks that can end it (finish line, reset, rail, air, prewind), sets boost/slew/lean targets from the pad, then runs the state's update (start, press, phase 2, exit). Address 0x0012fc80; every register, cycle and call kept, in the generated body's block structure.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn rider_board_press_sub_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_12fe88: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.g[RA][0] = sx(0x0012fca0);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            Callee {
                addr: 0x00116378,
                f: ee::s00::Rider_TryEnterFinishLine,
            }
            .call(c, 0x0012fca0)?;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_12fe88;
            } else {
                c.cycles += 8;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32));
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 12) as u32);
                c.g[RA][0] = sx(0x0012fcc0);
                c.g[A1][0] = c.g[A1][0] & 0x1;
                Callee {
                    addr: 0x00116120,
                    f: ee::s00::Rider_CheckResetPath,
                }
                .call(c, 0x0012fcc0)?;
                if c.g[V0][0] != 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    break 'b_12fe88;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0012fcd0);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    Callee {
                        addr: 0x00106848,
                        f: ee::s00::Rider_TryStartRailslide,
                    }
                    .call(c, 0x0012fcd0)?;
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_12fe88;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0012fce0);
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x0012fff8,
                            f: ee::s00::RiderBoardPressSub_HandleAirborne,
                        }
                        .call(c, 0x0012fce0)?;
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_12fe88;
                        } else {
                            c.cycles += 9;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32));
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 16) as u32);
                            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 15) as u32);
                            c.g[A2][0] = c.g[A2][0] & 0x1;
                            c.g[RA][0] = sx(0x0012fd04);
                            c.g[A1][0] = c.g[A1][0] & 0x1;
                            Callee {
                                addr: 0x001162c8,
                                f: ee::s00::Rider_TryEnterPrewind,
                            }
                            .call(c, 0x0012fd04)?;
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                break 'b_12fe88;
                            } else {
                                'b_12fdf4: {
                                    'b_12fd98: {
                                        c.cycles += 15;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 13) as u32);
                                        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 14) as u32);
                                        c.g[A2][0] = c.g[A2][0] & 0x1;
                                        c.g[RA][0] = sx(0x0012fd28);
                                        c.g[A1][0] = c.g[A1][0] & 0x1;
                                        Callee {
                                            addr: 0x00114130,
                                            f: ee::s00::Rider_HandleBoostInput,
                                        }
                                        .call(c, 0x0012fd28)?;
                                        c.mtc1(0, 12);
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        c.g[RA][0] = sx(0x0012fd38);
                                        c.mov_s(13, 12);
                                        Callee {
                                            addr: 0x00113f88,
                                            f: ee::s00::Rider_SetSlew3And4Targets,
                                        }
                                        .call(c, 0x0012fd38)?;
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x328));
                                        if c.g[V0][0] == 0u64 {
                                            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x280));
                                            break 'b_12fd98;
                                        } else {
                                            'b_12fd88: {
                                                c.cycles += 15;
                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x76dc));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                c.g[AT][0] = 0xffffffffbf000000;
                                                c.mtc1(1, 1);
                                                c.g[V0][0] =
                                                    sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                                                c.g[AT][0] = 0x3f000000;
                                                c.mtc1(1, 3);
                                                c.mtc1(2, 0);
                                                c.cvt_s_w(0, 0);
                                                c.mul_s(0, 0, 2);
                                                c.c_le_s(1, 0);
                                                let t = !c.fcc();
                                                c.mov_s(12, 1);
                                                if t {
                                                    break 'b_12fd88;
                                                } else {
                                                    c.cycles += 1;
                                                    c.min_s(12, 0, 3);
                                                    break 'b_12fd88;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x0012fd90);
                                            Callee {
                                                addr: 0x00113f38,
                                                f: ee::s00::Rider_SetSlew5Target,
                                            }
                                            .call(c, 0x0012fd90)?;
                                            let t = true;

                                            if t {
                                                break 'b_12fdf4;
                                            } else {
                                                break 'b_12fd98;
                                            }
                                        }
                                    }
                                    'b_12fdb8: {
                                        c.cycles += 7;
                                        c.abs_s(1, 0);
                                        c.g[AT][0] = 0x3f000000;
                                        c.mtc1(1, 0);
                                        c.c_lt_s(0, 1);
                                        if !c.fcc() {
                                            c.sub_s(3, 0, 1);
                                            break 'b_12fdb8;
                                        } else {
                                            c.cycles += 1;
                                            c.sub_s(3, 1, 0);
                                            break 'b_12fdb8;
                                        }
                                    }
                                    'b_12fdec: {
                                        c.cycles += 12;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                        c.neg_s(1, 3);
                                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x76d8));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                                        c.mtc1(2, 0);
                                        c.cvt_s_w(0, 0);
                                        c.mul_s(0, 0, 2);
                                        c.c_le_s(1, 0);
                                        let t = !c.fcc();
                                        c.mov_s(12, 1);
                                        if t {
                                            break 'b_12fdec;
                                        } else {
                                            c.cycles += 1;
                                            c.min_s(12, 0, 3);
                                            break 'b_12fdec;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[RA][0] = sx(0x0012fdf4);
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                    Callee {
                                        addr: 0x00113e80,
                                        f: ee::s00::Rider_SetSlew0Target,
                                    }
                                    .call(c, 0x0012fdf4)?;
                                    break 'b_12fdf4;
                                }
                                'b_12fe7c: {
                                    'b_12fe68: {
                                        'b_12fe54: {
                                            c.cycles += 8;
                                            c.g[RA][0] = sx(0x0012fdfc);
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            Callee {
                                                addr: 0x00115b58,
                                                f: ee::s00::Rider_UpdateTimedEventAnim,
                                            }
                                            .call(c, 0x0012fdfc)?;
                                            c.g[RA][0] = sx(0x0012fe04);
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            Callee {
                                                addr: 0x00115d48,
                                                f: ee::s00::Rider_UpdateIdleLookAnim,
                                            }
                                            .call(c, 0x0012fe04)?;
                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                            c.g[V0][0] = 0x1;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                            if t {
                                                break 'b_12fe54;
                                            } else {
                                                'b_12fe44: {
                                                    'b_12fe2c: {
                                                        c.cycles += 2;
                                                        if c.g[V0][0] == 0u64 {
                                                            c.g[V0][0] = 0x2;
                                                            break 'b_12fe2c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V1][0] == 0u64;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            if t {
                                                                break 'b_12fe44;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                if t {
                                                                    break 'b_12fe88;
                                                                } else {
                                                                    break 'b_12fe2c;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] == c.g[V0][0];
                                                    c.g[V0][0] = 0x3;
                                                    if t {
                                                        break 'b_12fe68;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[A0][0] = c.g[S0][0];
                                                        if t {
                                                            break 'b_12fe7c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            if t {
                                                                break 'b_12fe88;
                                                            } else {
                                                                break 'b_12fe44;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x0012fe4c);
                                                c.g[A1][0] = c.g[S1][0];
                                                Callee {
                                                    addr: 0x00130228,
                                                    f: ee::s00::RiderBoardPressSub_UpdateStart,
                                                }
                                                .call(c, 0x0012fe4c)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_12fe88;
                                                } else {
                                                    break 'b_12fe54;
                                                }
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x0012fe60);
                                        c.g[A1][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001303e0,
                                            f: ee::s00::RiderBoardPressSub_UpdatePress,
                                        }
                                        .call(c, 0x0012fe60)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_12fe88;
                                        } else {
                                            break 'b_12fe68;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x0012fe74);
                                    c.g[A1][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x001304d0,
                                        f: ee::s00::RiderBoardPressSub_UpdatePhase2,
                                    }
                                    .call(c, 0x0012fe74)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_12fe88;
                                    } else {
                                        break 'b_12fe7c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x0012fe84);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001304e0,
                                    f: ee::s00::RiderBoardPressSub_UpdateEnd,
                                }
                                .call(c, 0x0012fe84)?;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                break 'b_12fe88;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, fill, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC674_MAGIC` (otherwise the body they replaced runs).
    const OC674_CTRL: u32 = DATA + 0x000f_0000;
    const OC674_MAGIC: u32 = 0x0674_c0de;
    /// Eight `(callee address, result)` pairs: a fake called at that address
    /// returns `result` (1 means a real 0) instead of its hash pointer.
    const OC674_FORCE: u32 = DATA + 0x000f_0100;
    /// Objects of the fixtures live here (above the fakes' pointer results,
    /// which are `DATA..DATA + 0x40000`).
    const OC674_BASE: u32 = DATA + 0x0005_0000;
    /// The world (`gp - 0x848`) and the objects reached through it.
    const OC674_WORLD: u32 = OC674_BASE;
    const OC674_G: u32 = OC674_BASE + 0x100;
    const OC674_H: u32 = OC674_BASE + 0x200;
    const OC674_P: u32 = OC674_BASE + 0x300;
    const OC674_S0O: u32 = OC674_BASE + 0x400;
    const OC674_S1O: u32 = OC674_BASE + 0x500;
    const OC674_Y: u32 = OC674_BASE + 0x600;
    const OC674_P2: u32 = OC674_BASE + 0x700;
    /// A vtable-style object `{+0x38: half, +0x3c: callee}`.
    const OC674_OBJ6C0: u32 = OC674_BASE + 0x800;
    const OC674_SUBJECT: u32 = OC674_BASE + 0x2000;
    const OC674_OTHER: u32 = OC674_BASE + 0x4000;

    fn oc674_fake_call(c: &mut Cpu, addr: u32) {
        fakes::fake_call(c, addr);
        for k in 0..8u32 {
            if c.bus.read32(OC674_FORCE + 8 * k) == addr {
                let v = c.bus.read32(OC674_FORCE + 8 * k + 4);
                c.g[V0][0] = if v == 1 { 0 } else { v as u64 };
                break;
            }
        }
    }

    /// Fakes for slotted callees: `(fake, prev cell, slot, generated body, address)`.
    macro_rules! oc674_fakes {
        ($install:ident; $( ($fake:ident, $prev:ident, $slot:path, $gen:path, $addr:expr) ),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC674_CTRL) != OC674_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc674_fake_call(c, $addr);
                }
            )*
            fn $install() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc674_fakes! { oc674_install_slots;
        (oc674_fk_111890, OC674_PREV_111890, ee::s00::OVERRIDE_00111890, ee::s00::f_00111890__generated, 0x0011_1890),
        (oc674_fk_119368, OC674_PREV_119368, ee::s00::OVERRIDE_00119368, ee::s00::f_00119368__generated, 0x0011_9368),
        (oc674_fk_11d660, OC674_PREV_11D660, ee::s00::OVERRIDE_0011D660, ee::s00::f_0011d660__generated, 0x0011_d660),
        (oc674_fk_11fe78, OC674_PREV_11FE78, ee::s00::OVERRIDE_0011FE78, ee::s00::f_0011fe78__generated, 0x0011_fe78),
        (oc674_fk_11fec8, OC674_PREV_11FEC8, ee::s00::OVERRIDE_0011FEC8, ee::s00::f_0011fec8__generated, 0x0011_fec8),
        (oc674_fk_125038, OC674_PREV_125038, ee::s00::OVERRIDE_00125038, ee::s00::f_00125038__generated, 0x0012_5038),
        (oc674_fk_11b698, OC674_PREV_11B698, ee::s00::OVERRIDE_0011B698, ee::s00::f_0011b698__generated, 0x0011_b698),
        (oc674_fk_11bb40, OC674_PREV_11BB40, ee::s00::OVERRIDE_0011BB40, ee::s00::f_0011bb40__generated, 0x0011_bb40),
        (oc674_fk_11bb98, OC674_PREV_11BB98, ee::s00::OVERRIDE_0011BB98, ee::s00::f_0011bb98__generated, 0x0011_bb98),
        (oc674_fk_11c230, OC674_PREV_11C230, ee::s00::OVERRIDE_0011C230, ee::s00::f_0011c230__generated, 0x0011_c230),
        (oc674_fk_125c70, OC674_PREV_125C70, ee::s00::OVERRIDE_00125C70, ee::s00::f_00125c70__generated, 0x0012_5c70),
        (oc674_fk_108c80, OC674_PREV_108C80, ee::s00::OVERRIDE_00108C80, ee::s00::f_00108c80__generated, 0x0010_8c80),
        (oc674_fk_112180, OC674_PREV_112180, ee::s00::OVERRIDE_00112180, ee::s00::f_00112180__generated, 0x0011_2180),
        (oc674_fk_11e098, OC674_PREV_11E098, ee::s00::OVERRIDE_0011E098, ee::s00::f_0011e098__generated, 0x0011_e098),
        (oc674_fk_106848, OC674_PREV_106848, ee::s00::OVERRIDE_00106848, ee::s00::f_00106848__generated, 0x0010_6848),
        (oc674_fk_113e80, OC674_PREV_113E80, ee::s00::OVERRIDE_00113E80, ee::s00::f_00113e80__generated, 0x0011_3e80),
        (oc674_fk_113f38, OC674_PREV_113F38, ee::s00::OVERRIDE_00113F38, ee::s00::f_00113f38__generated, 0x0011_3f38),
        (oc674_fk_113f88, OC674_PREV_113F88, ee::s00::OVERRIDE_00113F88, ee::s00::f_00113f88__generated, 0x0011_3f88),
        (oc674_fk_114130, OC674_PREV_114130, ee::s00::OVERRIDE_00114130, ee::s00::f_00114130__generated, 0x0011_4130),
        (oc674_fk_115b58, OC674_PREV_115B58, ee::s00::OVERRIDE_00115B58, ee::s00::f_00115b58__generated, 0x0011_5b58),
        (oc674_fk_115d48, OC674_PREV_115D48, ee::s00::OVERRIDE_00115D48, ee::s00::f_00115d48__generated, 0x0011_5d48),
        (oc674_fk_116120, OC674_PREV_116120, ee::s00::OVERRIDE_00116120, ee::s00::f_00116120__generated, 0x0011_6120),
        (oc674_fk_1162c8, OC674_PREV_1162C8, ee::s00::OVERRIDE_001162C8, ee::s00::f_001162c8__generated, 0x0011_62c8),
        (oc674_fk_116378, OC674_PREV_116378, ee::s00::OVERRIDE_00116378, ee::s00::f_00116378__generated, 0x0011_6378),
        (oc674_fk_12fff8, OC674_PREV_12FFF8, ee::s00::OVERRIDE_0012FFF8, ee::s00::f_0012fff8__generated, 0x0012_fff8),
        (oc674_fk_1304d0, OC674_PREV_1304D0, ee::s00::OVERRIDE_001304D0, ee::s00::f_001304d0__generated, 0x0013_04d0),
        (oc674_fk_1304e0, OC674_PREV_1304E0, ee::s00::OVERRIDE_001304E0, ee::s00::f_001304e0__generated, 0x0013_04e0),
        (oc674_fk_1199f8, OC674_PREV_1199F8, ee::s00::OVERRIDE_001199F8, ee::s00::f_001199f8__generated, 0x0011_99f8),
        (oc674_fk_10e098, OC674_PREV_10E098, ee::s00::OVERRIDE_0010E098, ee::s00::f_0010e098__generated, 0x0010_e098),
        (oc674_fk_1193e0, OC674_PREV_1193E0, ee::s00::OVERRIDE_001193E0, ee::s00::f_001193e0__generated, 0x0011_93e0),
        (oc674_fk_1306b0, OC674_PREV_1306B0, ee::s00::OVERRIDE_001306B0, ee::s00::f_001306b0__generated, 0x0013_06b0),
        (oc674_fk_162258, OC674_PREV_162258, ee::s01::OVERRIDE_00162258, ee::s01::f_00162258__generated, 0x0016_2258),
        (oc674_fk_144d70, OC674_PREV_144D70, ee::s01::OVERRIDE_00144D70, ee::s01::f_00144d70__generated, 0x0014_4d70),
        (oc674_fk_148aa8, OC674_PREV_148AA8, ee::s01::OVERRIDE_00148AA8, ee::s01::f_00148aa8__generated, 0x0014_8aa8),
        (oc674_fk_14dc80, OC674_PREV_14DC80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated, 0x0014_dc80),
        (oc674_fk_14dd58, OC674_PREV_14DD58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated, 0x0014_dd58),
        (oc674_fk_147220, OC674_PREV_147220, ee::s01::OVERRIDE_00147220, ee::s01::f_00147220__generated, 0x0014_7220),
        (oc674_fk_147338, OC674_PREV_147338, ee::s01::OVERRIDE_00147338, ee::s01::f_00147338__generated, 0x0014_7338),
        (oc674_fk_1473d0, OC674_PREV_1473D0, ee::s01::OVERRIDE_001473D0, ee::s01::f_001473d0__generated, 0x0014_73d0),
        (oc674_fk_238358, OC674_PREV_238358, ee::s06::OVERRIDE_00238358, ee::s06::f_00238358__generated, 0x0023_8358),
        (oc674_fk_270970, OC674_PREV_270970, ee::s07::OVERRIDE_00270970, ee::s07::f_00270970__generated, 0x0027_0970),
        (oc674_fk_28b180, OC674_PREV_28B180, ee::s07::OVERRIDE_0028B180, ee::s07::f_0028b180__generated, 0x0028_b180),
        (oc674_fk_26af00, OC674_PREV_26AF00, ee::s07::OVERRIDE_0026AF00, ee::s07::f_0026af00__generated, 0x0026_af00),
        (oc674_fk_26ab20, OC674_PREV_26AB20, ee::s07::OVERRIDE_0026AB20, ee::s07::f_0026ab20__generated, 0x0026_ab20),
        (oc674_fk_26af70, OC674_PREV_26AF70, ee::s07::OVERRIDE_0026AF70, ee::s07::f_0026af70__generated, 0x0026_af70),
        (oc674_fk_285d98, OC674_PREV_285D98, ee::s07::OVERRIDE_00285D98, ee::s07::f_00285d98__generated, 0x0028_5d98),
        (oc674_fk_26adf0, OC674_PREV_26ADF0, ee::s07::OVERRIDE_0026ADF0, ee::s07::f_0026adf0__generated, 0x0026_adf0),
        (oc674_fk_30dba0, OC674_PREV_30DBA0, ee::s10::OVERRIDE_0030DBA0, ee::s10::f_0030dba0__generated, 0x0030_dba0),
        (oc674_fk_317d70, OC674_PREV_317D70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated, 0x0031_7d70),
        (oc674_fk_30ecd8, OC674_PREV_30ECD8, ee::s10::OVERRIDE_0030ECD8, ee::s10::f_0030ecd8__generated, 0x0030_ecd8),
        (oc674_fk_311bf0, OC674_PREV_311BF0, ee::s10::OVERRIDE_00311BF0, ee::s10::f_00311bf0__generated, 0x0031_1bf0),
        (oc674_fk_312660, OC674_PREV_312660, ee::s10::OVERRIDE_00312660, ee::s10::f_00312660__generated, 0x0031_2660),
        (oc674_fk_3128e8, OC674_PREV_3128E8, ee::s10::OVERRIDE_003128E8, ee::s10::f_003128e8__generated, 0x0031_28e8),
        (oc674_fk_3e22f0, OC674_PREV_3E22F0, ee::s14::OVERRIDE_003E22F0, ee::s14::f_003e22f0__generated, 0x003e_22f0),
    }

    static OC674_PREV_LNM: AtomicUsize = AtomicUsize::new(0);

    /// `ssx_LoadNetModules` (0x228ee0): logs the call and unwinds, so that the
    /// boot driver `cSSXApp_initload` (a huge unported function) stops there.
    fn oc674_fake_lnm(c: &mut Cpu) {
        if c.bus.read32(OC674_CTRL) != OC674_MAGIC {
            return fakes::fall_back(c, &OC674_PREV_LNM, ee::s05::f_00228ee0__generated);
        }
        fakes::fake_call(c, 0x0022_8ee0);
        c.pc = 0xdead_0000;
    }

    fn oc674_install() {
        oc674_install_slots();
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00228EE0),
            oc674_fake_lnm,
            &OC674_PREV_LNM,
        );
    }

    /// A fixture machine: scrambled registers, the gate on, no forced results.
    fn oc674_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC674_CTRL, OC674_MAGIC);
        r
    }

    fn oc674_force(c: &mut Cpu, k: u32, addr: u32, result: u32) {
        c.bus.write32(OC674_FORCE + 8 * k, addr);
        c.bus.write32(OC674_FORCE + 8 * k + 4, result);
    }

    fn oc674_gp(c: &Cpu, off: u32) -> u32 {
        (c.g[GP][0] as u32).wrapping_sub(off)
    }

    /// The world at `gp - 0x848` with the objects reached through it:
    /// `world+0x84 = G`, `G+0xc = Y`, `G+0x84 = H`, `H+4.. = P2`.
    fn oc674_world(c: &mut Cpu, r: &mut Rng) {
        let at = oc674_gp(c, 0x848);
        c.bus.write32(at, OC674_WORLD);
        fill(c, r, OC674_WORLD, 0x40);
        fill(c, r, OC674_G, 0x40);
        fill(c, r, OC674_H, 0x20);
        fill(c, r, OC674_Y, 0x20);
        fill(c, r, OC674_P2, 0x30);
        c.bus.write32(OC674_WORLD + 0x84, OC674_G);
        c.bus.write32(OC674_G + 0xc, OC674_Y);
        c.bus.write32(OC674_G + 0x84, OC674_H);
        for i in 0..8 {
            c.bus.write32(OC674_H + 4 * i, OC674_P2);
        }
    }

    /// A word holding `{+0x38: halfword, +0x3c: callee}` for a virtual call.
    fn oc674_vcall_obj(c: &mut Cpu, r: &mut Rng, at: u32, callee: u32) {
        fill(c, r, at, 0x20);
        c.bus.write16(at + 0x38, (r.next_u32() % 4 * 0x10) as u16);
        c.bus.write32(at + 0x3c, callee);
    }

    fn oc674_f(x: f32) -> u32 {
        x.to_bits()
    }

    /// `InitLoadScreen_Enter` stops at its first callee here; both sides log
    /// the call and agree on everything up to the unwind.
    #[test]
    fn init_load_screen_enter_matches() {
        oc674_install();
        check(
            ee::s06::f_00232568__generated,
            init_load_screen_enter_load,
            40,
            &|c, seed| {
                let mut r = oc674_machine(c, seed, 0x0674_0001, 0);
                c.pc = 0x0023_2568;
                let at = oc674_gp(c, 0x848);
                c.bus.write32(at, r.next_u32() & 0x00ff_fff0);
            },
        );
    }

    /// `Rider_ResetAtMatrix`: AI index 0 to 7 (grid slots and the default
    /// offset), random matrices.
    #[test]
    fn rider_reset_at_matrix_matches() {
        oc674_install();
        fakes::covered(
            ee::s00::f_00123210__generated,
            rider_reset_at_matrix,
            240,
            &|c, seed| {
                let mut r = oc674_machine(c, seed, 0x0674_0002, 9);
                c.pc = 0x0012_3210;
                let rider = OC674_SUBJECT;
                let mat = OC674_OTHER;
                oc674_world(c, &mut r);
                fill(c, &mut r, rider, 0x240);
                for i in 0..16 {
                    c.bus.write32(mat + 4 * i, r.f32(8.0));
                }
                oc674_vcall_obj(c, &mut r, OC674_OBJ6C0, fakes::FAKE);
                c.bus.write32(rider + 0x86c, seed % 8);
                c.bus.write32(rider + 0x6c0, OC674_OBJ6C0);
                c.bus.write32(rider + 0x88c, OC674_OTHER + 0x1000);
                // The chain world -> ... -> S1O: halfword at +0x18, callee at +0x1c.
                c.bus.write32(OC674_H + 4, OC674_P);
                fill(c, &mut r, OC674_P, 0x30);
                c.bus.write32(OC674_P + 0xa0, OC674_S0O);
                fill(c, &mut r, OC674_S0O, 0x20);
                c.bus.write32(OC674_S0O + 0x14, OC674_S1O);
                fill(c, &mut r, OC674_S1O, 0x20);
                c.bus
                    .write16(OC674_S1O + 0x18, (r.next_u32() % 4 * 0x10) as u16);
                c.bus.write32(OC674_S1O + 0x1c, fakes::FAKE + 0x10);
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(mat);
            },
        );
    }

    /// `Rider_OnFinishRace`: every guard (no player record, a started race,
    /// no finish flags) and the camera/audio tail (the audio callee runs
    /// real and leaves at once since `FindRider` is a fake).
    #[test]
    fn rider_on_finish_race_matches() {
        oc674_install();
        fakes::covered(
            ee::s00::f_00125108__generated,
            rider_on_finish_race,
            300,
            &|c, seed| {
                let mut r = oc674_machine(c, seed, 0x0674_0003, 6);
                c.pc = 0x0012_5108;
                let rider = OC674_SUBJECT;
                oc674_world(c, &mut r);
                fill(c, &mut r, rider, 0x240);
                let b = r.next_u32();
                c.bus.write32(rider + 0x480, (b >> 2) % 3);
                c.bus.write32(rider + 0x874, b & 1);
                c.bus.write32(rider + 0x87c, (b >> 1) & 1);
                c.bus.write32(rider + 0x870, (b >> 8) % 4);
                c.bus.write32(
                    rider + 0x470,
                    if b & 0x40 != 0 {
                        oc674_f(-1.0)
                    } else {
                        r.f32(500.0)
                    },
                );
                oc674_vcall_obj(c, &mut r, OC674_OBJ6C0, fakes::FAKE);
                c.bus.write32(rider + 0x6c0, OC674_OBJ6C0);
                let rec = if (b >> 4) & 3 == 0 { 0 } else { OC674_OTHER };
                let at = oc674_gp(c, 0x204);
                c.bus.write32(at, rec);
                c.g[A0][0] = sx(rider);
            },
        );
    }

    /// `AI_CreateMissionRider`: with a mission rider already loaded it leaves
    /// at once; else it builds one from the (forced) allocation.
    #[test]
    fn ai_create_mission_rider_matches() {
        oc674_install();
        fakes::covered(
            ee::s00::f_00129c00__generated,
            ai_create_mission_rider,
            240,
            &|c, seed| {
                let mut r = oc674_machine(c, seed, 0x0674_0004, 13);
                c.pc = 0x0012_9c00;
                let ai = OC674_SUBJECT;
                let alloc = OC674_OTHER + 0x10000;
                oc674_world(c, &mut r);
                fill(c, &mut r, ai, 0x60);
                fill(c, &mut r, alloc, 0x680);
                let loaded = [0u32, 1, 0, 0xffff_ffff, 2, 0][(seed % 6) as usize];
                c.bus.write32(ai + 0x88, loaded);
                c.bus.write32(ai + 0x78, seed / 6 % 4);
                let k = OC674_OTHER + 0x8000;
                fill(c, &mut r, k, 0x40);
                c.bus.write32(OC674_G + 0x34, k);
                oc674_force(c, 0, 0x0031_7d70, alloc);
                c.g[A0][0] = sx(ai);
            },
        );
    }

    /// `AI_CreateActorRiders`: 0 to 3 actors to build (and none when the
    /// required count is below the allocated one).
    #[test]
    fn ai_create_actor_riders_matches() {
        oc674_install();
        fakes::covered(
            ee::s00::f_00129ff8__generated,
            ai_create_actor_riders,
            240,
            &|c, seed| {
                let mut r = oc674_machine(c, seed, 0x0674_0005, 24);
                c.pc = 0x0012_9ff8;
                let ai = OC674_SUBJECT;
                let obj = OC674_OTHER;
                let rider = OC674_OTHER + 0x2000;
                let iface = OC674_OTHER + 0x4000;
                let vt = OC674_OTHER + 0x4100;
                fill(c, &mut r, ai, 0x80);
                fill(c, &mut r, obj, 0x20);
                fill(c, &mut r, iface, 0x20);
                fill(c, &mut r, vt, 0x20);
                let alloc = seed % 3;
                let need = [0u32, 1, 2, 3, 0xffff_ffff, 2][(seed / 3 % 6) as usize];
                c.bus.write32(ai + 0x84, alloc);
                c.bus.write32(ai + 0x94, alloc.wrapping_add(need));
                c.bus.write32(ai + 0x78, seed / 18 % 3);
                c.bus.write32(obj + 0x18, rider);
                c.bus.write32(iface + 0xc, vt);
                c.bus.write16(vt + 8, 0x10);
                c.bus.write32(vt + 0xc, fakes::FAKE + 0x40);
                oc674_force(c, 0, 0x0010_8c80, obj);
                oc674_force(c, 1, 0x0014_dd58, iface);
                c.g[A0][0] = sx(ai);
            },
        );
    }

    /// `AI_LoadTrackPaths`: 0 to 3 riders; the path loader runs real on top
    /// of the file-load and parse fakes.
    #[test]
    fn ai_load_track_paths_matches() {
        oc674_install();
        fakes::covered(
            ee::s00::f_0012a2b0__generated,
            ai_load_track_paths,
            200,
            &|c, seed| {
                let mut r = oc674_machine(c, seed, 0x0674_0006, 6);
                c.pc = 0x0012_a2b0;
                let ai = OC674_SUBJECT;
                fill(c, &mut r, ai, 0x40);
                c.bus.write32(ai + 0x78, seed % 4);
                c.g[A0][0] = sx(ai);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    /// The paths and riders of the two path-placement functions: a global
    /// path count (`0x4d33b0`), one track path node (an event list whose
    /// type-1 entry gives the finish-line marker) and up to three riders.
    fn oc674_place_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        let ai = OC674_SUBJECT;
        let node = OC674_OTHER;
        let ents = OC674_OTHER + 0x100;
        fill(c, r, ai, 0x40);
        c.bus.write32(ai + 0x78, seed % 4);
        for j in 0..3u32 {
            let rider = OC674_OTHER + 0x1000 + 0x600 * j;
            fill(c, r, rider, 0x180);
            c.bus.write32(ai + 0x28 + 4 * j, rider);
        }
        c.bus.write32(0x004d_33b0, (seed / 4) % 4);
        c.bus.write32(node, (seed / 16) % 4);
        c.bus.write32(node + 4, ents);
        for e in 0..4u32 {
            c.bus.write32(ents + 0x10 * e, r.next_u32() % 3);
            let marker = [
                oc674_f(-1.0),
                oc674_f(250.0),
                oc674_f(1500.0),
                oc674_f(5000.0),
            ][(r.next_u32() % 4) as usize];
            c.bus.write32(ents + 0x10 * e + 8, marker);
        }
        oc674_force(c, 0, 0x0026_af70, node);
        ai
    }

    /// `AI_PlaceRidersAtPathStart`: `which` of -1 (all), 0, 1 and 5.
    #[test]
    fn ai_place_riders_at_path_start_matches() {
        oc674_install();
        fakes::covered(
            ee::s00::f_0012a560__generated,
            ai_place_riders_at_path_start,
            300,
            &|c, seed| {
                let mut r = oc674_machine(c, seed, 0x0674_0007, 14);
                c.pc = 0x0012_a560;
                let ai = oc674_place_fixture(c, &mut r, seed);
                c.g[A0][0] = sx(ai);
                c.g[A1][0] = sx([0xffff_ffffu32, 0, 1, 5][(seed % 4) as usize]);
            },
        );
    }

    /// `AI_PlaceRidersStaggered`: `n` of 0 (leaves at once) to 3.
    #[test]
    fn ai_place_riders_staggered_matches() {
        oc674_install();
        fakes::covered(
            ee::s00::f_0012a768__generated,
            ai_place_riders_staggered,
            300,
            &|c, seed| {
                let mut r = oc674_machine(c, seed, 0x0674_0008, 14);
                c.pc = 0x0012_a768;
                let ai = oc674_place_fixture(c, &mut r, seed);
                c.g[A0][0] = sx(ai);
                c.g[A1][0] = sx((seed / 3) % 5);
            },
        );
    }

    /// `RiderWipeoutSub_StartGetUp`: both animation choices (root pose
    /// override 1 or not), random bones and matrices.
    #[test]
    fn rider_wipeout_sub_start_get_up_matches() {
        oc674_install();
        fakes::covered(
            ee::s00::f_0012e010__generated,
            rider_wipeout_sub_start_get_up,
            240,
            &|c, seed| {
                let mut r = oc674_machine(c, seed, 0x0674_0009, 5);
                c.pc = 0x0012_e010;
                let this = OC674_SUBJECT;
                let rider = OC674_OTHER;
                let model = OC674_OTHER + 0x2000;
                let pos = OC674_OTHER + 0x2400;
                let rot = OC674_OTHER + 0x2600;
                fill(c, &mut r, this, 0x40);
                fill(c, &mut r, rider, 0x240);
                fill(c, &mut r, model, 0x60);
                for i in 0..64 {
                    c.bus.write32(pos + 4 * i, r.f32(4.0));
                    c.bus.write32(rot + 4 * i, r.f32(1.0));
                }
                for k in 0..4 {
                    c.bus.write32(0x004f_f130 + 4 * k, r.f32(4.0));
                }
                for off in [0x140u32, 0x144, 0x148, 0x14c] {
                    c.bus.write32(model + off, r.f32(2.0));
                }
                let word = (c.g[GP][0] as u32).wrapping_add(0x1b18);
                c.bus.write32(word, r.next_u32());
                c.bus.write32(this + 0x80, rider);
                c.bus
                    .write32(rider + 0x150, [1u32, 0, 1, 3][(seed % 4) as usize]);
                c.bus.write32(rider + 0x780, model);
                c.bus.write32(rider + 0x89c, (seed / 4) % 6);
                c.bus.write32(model + 0x24, pos);
                c.bus.write32(model + 0x28, rot);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `RiderBoardPressSub_HandleInput`: which of the five early-out checks
    /// fires (or none), railslide or not, and the four sub-states.
    #[test]
    fn rider_board_press_sub_handle_input_matches() {
        oc674_install();
        fakes::covered(
            ee::s00::f_0012fc80__generated,
            rider_board_press_sub_handle_input,
            420,
            &|c, seed| {
                let mut r = oc674_machine(c, seed, 0x0674_000a, 18);
                c.pc = 0x0012_fc80;
                let this = OC674_SUBJECT;
                let rider = OC674_OTHER;
                let input = OC674_OTHER + 0x2000;
                fill(c, &mut r, this, 0x20);
                fill(c, &mut r, rider, 0x240);
                fill(c, &mut r, input, 0x10);
                for (i, off) in [0x76dcu32, 0x76d8, 0x76b4, 0x76b0, 0x76a8]
                    .iter()
                    .enumerate()
                {
                    let at = oc674_gp(c, *off);
                    c.bus
                        .write32(at, oc674_f(0.032_258_064 * (1 + i % 2) as f32));
                }
                c.bus.write32(this + 0x14, rider);
                c.bus.write32(this, (seed / 7) % 5);
                c.bus
                    .write32(rider + 0x328, if r.next_u32() & 1 == 0 { 0 } else { 5 });
                c.bus.write32(rider + 0x280, r.f32(1.0));
                // Exactly one of the first five checks fires (or none).
                let fires = seed % 7;
                let addrs = [
                    0x0011_6378u32,
                    0x0011_6120,
                    0x0010_6848,
                    0x0012_fff8,
                    0x0011_62c8,
                ];
                let mut k = 0;
                for (j, a) in addrs.iter().enumerate() {
                    if j as u32 != fires {
                        oc674_force(c, k, *a, 1);
                        k += 1;
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(input);
            },
        );
    }
}
