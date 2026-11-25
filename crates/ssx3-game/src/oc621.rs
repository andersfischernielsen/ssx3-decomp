//! Staging module oc621 (wave g01.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc621`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0012_a4d0, oc621_path_data_update_wrapper, "AI_PathDataUpdate_Wrapper"),
    (0x0012_a4f0, oc621_init_all_track_paths, "AI_InitAllTrackPaths"),
    (0x0012_a998, oc621_reset_listed_rider_render_slots, "AI_ResetListedRiderRenderSlots"),
];

/// The AI's path data (`gPaths`, 0x4d33a0): the global at `0x4d0000 + 0x33a0`.
const OC621_GPATHS: u32 = 0x004d_33a0;

/// `Rider_InitTrackPaths` (0x112180).
const OC621_RIDER_INIT_TRACK_PATHS: Callee = Callee {
    addr: 0x0011_2180,
    f: ee::s00::f_00112180,
};

/// `Rider_SetSubState` (0x11fec8).
#[allow(dead_code)]
const OC621_RIDER_SET_SUB_STATE: Callee = Callee {
    addr: 0x0011_fec8,
    f: ee::s00::f_0011fec8,
};

/// `Rider_SetMainState` (0x11fe78).
#[allow(dead_code)]
const OC621_RIDER_SET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe78,
    f: ee::s00::f_0011fe78,
};

/// `Rider_Place` (0x11d660).
#[allow(dead_code)]
const OC621_RIDER_PLACE: Callee = Callee {
    addr: 0x0011_d660,
    f: ee::s00::f_0011d660,
};

/// The address `base + off` as a 32-bit guest pointer.
#[inline(always)]
fn oc621_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `void AI_PathDataUpdate_Wrapper(void)` (0x12a4d0): forwards to the
/// path-file loaded check with the global path data (`gPaths`). The 8 cycles
/// are the wrapper's own, then the callee's.
pub fn oc621_path_data_update_wrapper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(OC621_GPATHS);
        call_ptr(c, 0x0026_ade8, 0x0012_a4e4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_InitAllTrackPaths(AI* self)` (0x12a4f0): re-initialises the track
/// paths of every allocated rider (`self->Riders[0..count]`, count at +0x78,
/// array at +0x28), each through `Rider_InitTrackPaths(rider, 0)`.
pub fn oc621_init_all_track_paths(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(oc621_at(c.g[S2][0], 0x78));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[S0][0] = sx(oc621_at(c.g[S2][0], 0x28));
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            loop {
                c.cycles += 8;
                c.g[A1][0] = 0;
                c.g[S1][0] = sx(oc621_at(c.g[S1][0], 1));
                c.g[S0][0] = sx(oc621_at(c.g[S0][0], 4));
                OC621_RIDER_INIT_TRACK_PATHS.call(c, 0x0012_a530)?;
                c.g[V0][0] = c.lw(oc621_at(c.g[S2][0], 0x78));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0012_a520);
                }
            }
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

/// `void AI_ResetListedRiderRenderSlots(AI* self)` (0x12a998): for each listed
/// computer (`count` entries at self+0x48, count at +0x80; the rider is at
/// +0x18 of each entry), a rider that is not safe to render while its AI index
/// is in the special list gets its render flag cleared and its safe-render tick
/// set to the current frame count + 4.
pub fn oc621_reset_listed_rider_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(oc621_at(c.g[S3][0], 0x80));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S2][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[S4][0] = u64::MAX;
            c.g[S1][0] = sx(oc621_at(c.g[S3][0], 0x48));
            loop {
                'entry: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A1][0] = c.lw(oc621_at(c.g[V0][0], 0x18));
                    c.g[V1][0] = c.lw(oc621_at(c.g[A1][0], 0x884));
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = c.lw(oc621_at(c.g[S3][0], 0x80));
                        break 'entry;
                    }
                    'hit: {
                        c.cycles += 11;
                        c.g[V1][0] = c.lw(oc621_at(c.g[A1][0], 0x6c0));
                        c.g[A1][0] = sx(oc621_at(c.g[A1][0], 0x6c0));
                        c.g[A0][0] = c.lh(oc621_at(c.g[V1][0], 0x38));
                        c.g[V0][0] = c.lw(oc621_at(c.g[V1][0], 0x3c));
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(oc621_at(c.g[A1][0], c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0012_a9fc)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x0012_a1f0, 0x0012_aa08)?;
                        let not_special = c.g[V0][0] == 0;
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        if not_special {
                            break 'hit;
                        }
                        c.cycles += 13;
                        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                        c.g[V0][0] = c.lw(oc621_at(c.g[V1][0], 0x18));
                        c.sw(oc621_at(c.g[V0][0], 0x888), c.g[S4][0]);
                        c.sw(oc621_at(c.g[V0][0], 0x884), 0);
                        c.g[V1][0] = c.lw(oc621_at(c.g[A1][0], 0x10d8));
                        c.g[S0][0] = c.lw(c.g[S1][0] as u32);
                        c.g[A0][0] = c.lh(oc621_at(c.g[V1][0], 0x390));
                        c.g[V0][0] = c.lw(oc621_at(c.g[V1][0], 0x394));
                        let frames = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(oc621_at(c.g[A1][0], c.g[A0][0] as u32));
                        call_ptr(c, frames, 0x0012_aa38)?;
                        c.g[V1][0] = c.lw(oc621_at(c.g[S0][0], 0x18));
                        c.g[V0][0] = sx(oc621_at(c.g[V0][0], 4));
                        c.sw(oc621_at(c.g[V1][0], 0x888), c.g[V0][0]);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(oc621_at(c.g[S3][0], 0x80));
                }
                c.cycles += 4;
                c.g[S2][0] = sx(oc621_at(c.g[S2][0], 1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(oc621_at(c.g[S1][0], 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_a9d0);
                }
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

/// `void AI_LoadTrackPaths(AI* self, char* filename)` (0x12a2b0), the
/// `cAI_swaploadPathFile` of the reference: purges the global path data, loads
/// `filename` into it, then re-initialises each allocated rider's track paths.
///
/// Not in `OVERRIDES`: its callee `cPathSys_loadPaths` (0x26ada8) has no
/// override slot, so the file loader cannot be faked in a test, and the port
/// is left unverified until that loader is ported.
#[allow(dead_code)]
pub fn oc621_load_track_paths(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx(OC621_GPATHS);
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = 0;
        call_ptr(c, 0x0026_af00, 0x0012_a2e4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0026_ada8, 0x0012_a2f0)?;
        c.g[V0][0] = c.lw(oc621_at(c.g[S3][0], 0x78));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = sx(oc621_at(c.g[S3][0], 0x28));
        if !none {
            c.cycles += 1;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            loop {
                c.cycles += 8;
                c.g[A1][0] = 0;
                c.g[S2][0] = sx(oc621_at(c.g[S2][0], 1));
                c.g[S0][0] = sx(oc621_at(c.g[S0][0], 4));
                OC621_RIDER_INIT_TRACK_PATHS.call(c, 0x0012_a310)?;
                c.g[V0][0] = c.lw(oc621_at(c.g[S3][0], 0x78));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0012_a300);
                }
            }
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

/// `void AI_PlaceRidersAtPathStart(AI* self, int which)` (0x12a560): finds the
/// first track path with a finish line marker (`cTrackPath_getFinishLineParam`
/// != -1). The riders (`which` = -1 for all, else that index) are then stood
/// one-by-one at the 1000-unit approach to the marker, facing along the
/// path, and the search returns.
///
/// Not in `OVERRIDES`: the fixture only reaches the no-marker branch (the
/// finish-line getter's list walk needs a populated path table), so the port
/// is kept unverified for the placement branch.
#[allow(dead_code)]
pub fn oc621_place_riders_at_path_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xd0);
        c.g[V0][0] = 0x004d_0000;
        frame.save(c, S5, 0x60);
        frame.save(c, S0, 0xb0);
        c.g[S5][0] = c.g[V0][0];
        frame.save(c, S3, 0x80);
        c.g[V0][0] = sx(OC621_GPATHS);
        frame.save(c, S4, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0xa0);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S6, 0x50);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 20, 0xc0);
        frame.save(c, S2, 0x90);
        c.g[S2][0] = c.lw(oc621_at(c.g[V0][0], 0x10));
        let none = (c.g[S2][0] as i64) <= 0;
        c.g[S0][0] = 0;
        'outer: {
            'inner: {
                if none {
                    break 'inner;
                }
                c.cycles += 2;
                c.g[A0][0] = sx(oc621_at(c.g[S5][0], 0x33a0));
                loop {
                    'node: {
                        c.cycles += 12;
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x0026_af70, 0x0012_a5b8)?;
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x0026_a138, 0x0012_a5c4)?;
                        c.mov_s(12, 0);
                        c.g[AT][0] = 0xffff_ffff_bf80_0000;
                        c.mtc1(1, 0);
                        c.c_eq_s(12, 0);
                        if c.fcc() {
                            c.g[S0][0] = sx(oc621_at(c.g[S0][0], 1));
                            break 'node;
                        }
                        // The marker is ahead: approach point 1000 units back.
                        c.cycles += 8;
                        c.g[AT][0] = 0x447a_0000;
                        c.mtc1(1, 0);
                        c.mtc1(0, 1);
                        c.sub_s(20, 12, 0);
                        c.c_lt_s(20, 1);
                        if c.fcc() {
                            c.mov_s(20, 1);
                        }
                        c.cycles += 13;
                        c.g[A0][0] = sp(c) as u64;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = 0;
                        c.g[S2][0] = 0;
                        call_ptr(c, 0x0026_ab20, 0x0012_a614)?;
                        c.g[A1][0] = c.g[S1][0];
                        c.mov_s(12, 20);
                        c.g[A0][0] = sx(oc621_at(sp(c) as u64, 0x10));
                        c.g[A2][0] = 0;
                        call_ptr(c, 0x0026_ab20, 0x0012_a628)?;
                        c.g[V0][0] = c.lw(oc621_at(c.g[S3][0], 0x78));
                        let no_riders = (c.g[V0][0] as i64) <= 0;
                        c.g[S6][0] = sx(oc621_at(sp(c) as u64, 0x10));
                        if no_riders {
                            break 'inner;
                        }
                        c.cycles += 5;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 20);
                        c.g[S5][0] = u64::MAX;
                        c.g[S1][0] = 0;
                        c.g[S0][0] = sx(oc621_at(c.g[S3][0], 0x28));
                        loop {
                            'rider: {
                                'pick: {
                                    c.cycles += 2;
                                    if c.g[S4][0] == c.g[S5][0] {
                                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                        break 'pick;
                                    }
                                    c.cycles += 2;
                                    if c.g[S4][0] != c.g[S2][0] {
                                        c.g[V0][0] = c.lw(oc621_at(c.g[S3][0], 0x78));
                                        break 'rider;
                                    }
                                    c.cycles += 1;
                                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                }
                                'place: {
                                    'set: {
                                        c.cycles += 26;
                                        c.g[A1][0] = 0;
                                        OC621_RIDER_SET_SUB_STATE.call(c, 0x0012_a664)?;
                                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                        c.g[A1][0] = 0;
                                        OC621_RIDER_SET_MAIN_STATE.call(c, 0x0012_a670)?;
                                        c.lqc2(3, sp(c));
                                        c.lqc2(4, oc621_at(sp(c) as u64, 0x10));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, oc621_at(sp(c) as u64, 0x30));
                                        c.g[A0][0] = sx(oc621_at(c.g[S3][0], 0x28));
                                        c.lq(2, oc621_at(sp(c) as u64, 0x30));
                                        c.sq(2, oc621_at(sp(c) as u64, 0x20));
                                        c.lqc2(3, oc621_at(sp(c) as u64, 0x20));
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
                                            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                            break 'set;
                                        }
                                        c.cycles += 9;
                                        c.mfc1(2, 0);
                                        c.qmtc2(2, 3);
                                        c.vu0().div(0, 3, 3, 0);
                                        c.lqc2(4, oc621_at(sp(c) as u64, 0x20));
                                        c.vu0().waitq();
                                        c.vu0().mul(0b1111, 5, 4, Src::Q);
                                        c.sqc2(5, oc621_at(sp(c) as u64, 0x20));
                                        c.g[V0][0] = sx(oc621_at(c.g[A0][0], c.g[S1][0] as u32));
                                        break 'place;
                                    }
                                    c.cycles += 4;
                                    c.lq(2, oc621_at(c.g[V1][0], 0x1b0));
                                    c.sq(2, oc621_at(sp(c) as u64, 0x20));
                                    c.sq(2, oc621_at(sp(c) as u64, 0x30));
                                    c.g[V0][0] = sx(oc621_at(c.g[A0][0], c.g[S1][0] as u32));
                                }
                                c.cycles += 7;
                                c.mtc1(0, 12);
                                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                                c.g[A1][0] = c.g[S6][0];
                                c.g[A2][0] = sx(oc621_at(sp(c) as u64, 0x20));
                                c.g[A3][0] = 5;
                                OC621_RIDER_PLACE.call(c, 0x0012_a710)?;
                                c.g[V0][0] = c.lw(oc621_at(c.g[S3][0], 0x78));
                            }
                            c.cycles += 5;
                            c.g[S2][0] = sx(oc621_at(c.g[S2][0], 1));
                            c.g[S1][0] = sx(oc621_at(c.g[S1][0], 4));
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let more = c.g[V0][0] != 0;
                            c.g[S0][0] = sx(oc621_at(c.g[S0][0], 4));
                            if !more {
                                c.cycles += 2;
                                frame.restore(c, S0, 0xb0);
                                break 'outer;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x0012_a648);
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = sx(oc621_at(c.g[S5][0], 0x33a0));
                    if !more {
                        break 'inner;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0012_a5b0);
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0xb0);
        }
        c.cycles += 10;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 20, 0xc0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word in the test machine's RAM: the rider-init fake logs only when
    /// it is set, and otherwise runs the body it replaced.
    const OC621_CTRL: u32 = DATA + 0x3e_0100;
    const OC621_MAGIC: u32 = 0x0621_f00d;

    /// Gated fake of `Rider_InitTrackPaths` (0x112180).
    fn oc621_fake_rider_init(c: &mut Cpu) {
        if c.bus.read32(OC621_CTRL) != OC621_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc621_fake_rider_init),
                ee::s00::f_00112180__generated,
            );
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, 0x0011_2180, ret);
    }

    fn oc621_hook_rider_init() {
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_00112180),
            oc621_fake_rider_init,
            fakes::prev_cell(oc621_fake_rider_init),
        );
    }

    #[test]
    fn oc621_path_data_update_wrapper_matches() {
        check(
            ee::s00::f_0012a4d0__generated,
            oc621_path_data_update_wrapper,
            100,
            &|c, seed| {
                machine(c, seed, 0x12_a4d0, 0);
                c.pc = 0x0012_a4d0;
            },
        );
    }

    #[test]
    fn oc621_init_all_track_paths_matches() {
        oc621_hook_rider_init();
        check(
            ee::s00::f_0012a4f0__generated,
            oc621_init_all_track_paths,
            200,
            &|c, seed| {
                machine(c, seed, 0x12_a4f0, 0);
                c.pc = 0x0012_a4f0;
                c.bus.write32(OC621_CTRL, OC621_MAGIC);
                let this = DATA + 0x2000;
                c.bus.write32(this + 0x78, [0u32, 1, 3, 2][(seed % 4) as usize]);
                for i in 0..4u32 {
                    c.bus.write32(this + 0x28 + 4 * i, DATA + 0x3000 + 0x100 * i);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc621_reset_listed_rider_render_slots_matches() {
        check(
            ee::s00::f_0012a998__generated,
            oc621_reset_listed_rider_render_slots,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12_a998, 0);
                c.pc = 0x0012_a998;
                let this = DATA + 0x2000;
                // The GrMan-like object behind `GP-0x854` and its vtable slot.
                let gp_obj = DATA + 0x7000;
                let gp_vt = DATA + 0x7100;
                c.bus.write32(GAME_GP - 0x854, gp_obj);
                c.bus.write32(gp_obj + 0x10d8, gp_vt);
                c.bus.write16(gp_vt + 0x390, 0);
                c.bus.write32(gp_vt + 0x394, FAKE + 0x40);
                let count = if seed % 7 == 0 { 0 } else { r.next_u32() % 4 };
                c.bus.write32(this + 0x80, count);
                for i in 0..4u32 {
                    let entry = DATA + 0x3000 + 0x100 * i;
                    let rider = DATA + 0x4000 + 0x400 * i;
                    // The rider's object at +0x6c0 with a vtable whose slot
                    // 0x38 (delta) and 0x3c (function) are fakes.
                    let vt = DATA + 0x6000 + 0x100 * i;
                    c.bus.write32(this + 0x48 + 4 * i, entry);
                    c.bus.write32(entry + 0x18, rider);
                    c.bus.write32(rider + 0x884, r.next_u32() % 2);
                    c.bus.write32(rider + 0x6c0, vt);
                    c.bus.write16(vt + 0x38, 0);
                    c.bus.write32(vt + 0x3c, FAKE + 0x10 * i);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `AI_PlaceRidersAtPathStart`: the fixture has no finish-line marker, so
    /// the search runs through the path nodes and returns without placing a
    /// rider (the placement branch is not reached; see the module comment).
    #[test]
    fn oc621_place_riders_at_path_start_matches() {
        check(
            ee::s00::f_0012a560,
            oc621_place_riders_at_path_start,
            200,
            &|c, seed| {
                machine(c, seed, 0x12_a560, 0);
                c.pc = 0x0012_a560;
                let this = DATA + 0x2000;
                c.bus.write32(OC621_GPATHS + 0x10, [0u32, 1, 3][(seed % 3) as usize]);
                c.bus.write32(this + 0x78, seed % 4);
                for i in 0..4u32 {
                    c.bus.write32(this + 0x28 + 4 * i, DATA + 0x3000 + 0x100 * i);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 2 == 0 {
                    u64::MAX
                } else {
                    (seed % 4) as u64
                };
            },
        );
    }
}
