//! Staging module oc1136 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1136`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_1940, path_flush_events_v2, "PATH_FlushEvents"),
    (0x003d_2c20, path_get_max_branch_ratio, "PATH_GetMaxBranchRatio"),
    (0x003d_5698, path_fade_volume, "PATH_FadeVolume"),
];

/// `D_0044F420`: the current path map (`PathMap*`).
pub(crate) const PATHS_CUR_MAP: u32 = 0x0044_f420;
/// `D_0044F42C`: byte set once the path system is initialised.
pub(crate) const PATHS_INIT_FLAG: u32 = 0x0044_f42c;
/// `D_0044F430`: id of the selected map (0 unset, -1 none).
pub(crate) const PATHS_CUR_ID: u32 = 0x0044_f430;
/// The path-map table `D_00517610` (four maps, 0x928 bytes each).
pub(crate) const PATHS_MAPS: u32 = 0x0051_7610;

/// `PATH_SelectMap(idx)` (0x3d5450).
const PATHS_SELECT_MAP: Callee = Callee {
    addr: 0x003d_5450,
    f: ee::s14::f_003d5450,
};
/// `PATH_RemoveEvent(ev, flags)` (0x3d1ba8).
const PATHS_REMOVE_EVENT: Callee = Callee {
    addr: 0x003d_1ba8,
    f: ee::s14::f_003d1ba8,
};

/// `void PATH_FlushEvents(int mask)` (0x3d1940): for each map that is loaded
/// and whose group bits match `mask`, clears the event slots of the current
/// player path (`PATH_RemoveEvent(slot, 2)`), then forgets the selection.
pub fn path_flush_events_v2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xa0);
        c.g[V1][0] = 0x0045_0000;
        c.g[V0][0] = Addr(PATHS_INIT_FLAG).lbu(c);
        frame.save(c, S2, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        let t = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        'a5c: {
            'a58: {
                if t {
                    break 'a58;
                }
                c.cycles += 4;
                c.g[V1][0] = 0x0045_0000;
                c.g[V0][0] = Addr(PATHS_CUR_ID).lw(c);
                let t = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x90);
                if t {
                    break 'a5c;
                }
                c.cycles += 4;
                c.g[A0][0] = u64::MAX;
                PATHS_SELECT_MAP.call(c, 0x003d_1994)?;
                let t = c.g[V0][0] == 0;
                c.g[V0][0] = 0x0051_0000;
                if t {
                    break 'a58;
                }
                c.cycles += 9;
                c.g[A0][0] = 0;
                c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7644));
                c.g[S3][0] = 0x0045_0000;
                c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x4));
                c.g[S7][0] = 0x928;
                c.g[S6][0] = sx(0xf000_0000);
                c.g[S5][0] = 0x0f00_0000;
                c.mult(0, 5, 4, 23);
                'l19c0: loop {
                    'b1a40: {
                        'b1a3c: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                            let t = c.g[V1][0] == 0;
                            c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            if t {
                                break 'b1a3c;
                            }
                            c.cycles += 6;
                            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[FP][0] as u32));
                            c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
                            c.g[V0][0] = c.g[V1][0] & c.g[S6][0];
                            c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
                            let t = c.g[V0][0] == 0;
                            c.g[V0][0] = c.g[V1][0] & c.g[S5][0];
                            if t {
                                break 'b1a3c;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
                            if c.g[V0][0] == 0 {
                                c.g[A0][0] = c.g[S1][0];
                                break 'b1a40;
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[A0][0] & 0xff;
                            PATHS_SELECT_MAP.call(c, 0x003d_19fc)?;
                            let t = c.g[V0][0] == 0;
                            c.g[A0][0] = c.g[S1][0];
                            if t {
                                break 'b1a40;
                            }
                            c.cycles += 3;
                            c.g[S0][0] = 0;
                            c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_sub(0xbe0)).lw(c);
                            'l1a10: loop {
                                'b1a2c: {
                                    c.cycles += 5;
                                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xdc)).lw(c);
                                    let t = c.g[A0][0] == 0;
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        break 'b1a2c;
                                    }
                                    c.cycles += 2;
                                    c.g[A1][0] = 0x2;
                                    PATHS_REMOVE_EVENT.call(c, 0x003d_1a2c)?;
                                }
                                c.cycles += 4;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[S0][0] as i64) < 16i64) as u64;
                                let t = c.g[V0][0] != 0;
                                c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_sub(0xbe0)).lw(c);
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003d_1a10);
                                    }
                                    continue 'l1a10;
                                } else {
                                    break 'b1a3c;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S1][0];
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 4i64) as u64;
                    if c.g[V0][0] != 0 {
                        c.mult(0, 5, 4, 23);
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_19c0);
                        }
                        continue 'l19c0;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x0045_0000;
                        Addr((c.g[S3][0] as u32).wrapping_sub(0xbe0)).sw(c, 0);
                        Addr((c.g[V0][0] as u32).wrapping_sub(0xbd0)).sw(c, 0);
                        break 'a58;
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
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
    });
}

/// `int PATH_GetMaxBranchRatio(u32 pathId, int scale)` (0x3d2c20): -18 when
/// the path system is not ready; else finds the map of `pathId`'s top byte,
/// scans the track segments of the branch (lowest set bit of `pathId`) for
/// the largest `(end - start) << 7 / length` ratio and returns it as
/// `scale * ratio` rounded through the soft-float helpers (0 when the map or
/// branch does not exist). Converted from the generated `match pc` body.
pub fn path_get_max_branch_ratio(c: &mut Cpu) {
    if c.pc != 0x003d2c20 {
        return ee::s14::f_003d2c20__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003d2c20;
    loop {
        match pc {
            0x003d2c20 => {
                c.cycles += 14;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.g[V0][0] = 0x450000;
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.mtc1(0, 20);
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.g[S1][0] = c.g[A1][0];
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0xbd4));
                c.mov_s(21, 20);
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[V1][0] == 0u64; 
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t { pc = 0x003d2c78; continue; }
                pc = 0x003d2c58; continue;
            }
            0x003d2c58 => {
                c.cycles += 4;
                c.g[V1][0] = 0x450000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbd0));
                let t = c.g[V0][0] != 0u64; 
                c.g[S2][0] = c.g[V1][0];
                if t { pc = 0x003d2c78; continue; }
                pc = 0x003d2c68; continue;
            }
            0x003d2c68 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003d2c70);
                c.g[A0][0] = 0xffffffffffffffff;
                Callee { addr: 0x003d5450, f: ee::s14::PATH_SelectMap }.call(c, 0x003d2c70)?;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x003d2c80; continue; }
                pc = 0x003d2c78; continue;
            }
            0x003d2c78 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0xffffffffffffffee;
                if t { pc = 0x003d2dd4; continue; }
                pc = 0x003d2c80; continue;
            }
            0x003d2c80 => {
                c.cycles += 5;
                c.g[A0][0] = 0xffffffffff000000;
                c.g[RA][0] = sx(0x003d2c8c);
                c.g[A0][0] = c.g[S0][0] & c.g[A0][0];
                Callee { addr: 0x003d4bb0, f: ee::s14::PATH_FindMap }.call(c, 0x003d2c8c)?;
                let t = c.g[V0][0] == 0u64; 
                c.g[V1][0] = 0u64;
                if t { pc = 0x003d2ce4; continue; }
                pc = 0x003d2c94; continue;
            }
            0x003d2c94 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[V0][0] = c.g[A0][0] << 32;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                let t = true; 
                c.g[S0][0] = 0x450000;
                if t { pc = 0x003d2cbc; continue; }
                pc = 0x003d2ca8; continue;
            }
            0x003d2ca8 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < 0x18u64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = ((c.g[A0][0] as i64) >> (c.g[V1][0] & 63)) as u64;
                if t { pc = 0x003d2ccc; continue; }
                pc = 0x003d2cb4; continue;
            }
            0x003d2cb4 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                pc = 0x003d2cbc; continue;
            }
            0x003d2cbc => {
                c.cycles += 4;
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                c.g[V0][0] = c.g[V0][0] & 0x1;
                
                if c.g[V0][0] != 0u64 { c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1)); if back_edge_unwinds(c, 0x003d2ca8) { return Ok(()); } pc = 0x003d2ca8; continue; }
                pc = 0x003d2ccc; continue;
            }
            0x003d2ccc => {
                c.cycles += 6;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0xbe0));
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xd));
                c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                
                if c.g[V0][0] == 0u64 { c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4c)); pc = 0x003d2cec; continue; }
                pc = 0x003d2ce4; continue;
            }
            0x003d2ce4 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0u64;
                if t { pc = 0x003d2dd4; continue; }
                pc = 0x003d2cec; continue;
            }
            0x003d2cec => {
                c.cycles += 14;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.mtc1(17, 1);
                c.cvt_s_w(1, 1);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x8));
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V0][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[A1][0] = c.g[V1][0];
                if t { pc = 0x003d2dac; continue; }
                pc = 0x003d2d24; continue;
            }
            0x003d2d24 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                pc = 0x003d2d28; continue;
            }
            0x003d2d28 => {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 7);
                
                if (c.g[V0][0] as i64) < 0 { c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1); pc = 0x003d2d4c; continue; }
                pc = 0x003d2d3c; continue;
            }
            0x003d2d3c => {
                c.cycles += 4;
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                let t = true; 
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                if t { pc = 0x003d2d5c; continue; }
                pc = 0x003d2d4c; continue;
            }
            0x003d2d4c => {
                c.cycles += 4;
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.add_s(0, 0, 0);
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                pc = 0x003d2d5c; continue;
            }
            0x003d2d5c => {
                c.cycles += 2;
                let t = (c.g[V0][0] as i64) < 0; 
                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                if t { pc = 0x003d2d74; continue; }
                pc = 0x003d2d64; continue;
            }
            0x003d2d64 => {
                c.cycles += 4;
                c.mtc1(2, 12);
                c.cvt_s_w(12, 12);
                let t = true; 
                
                if t { pc = 0x003d2d88; continue; }
                pc = 0x003d2d74; continue;
            }
            0x003d2d74 => {
                c.cycles += 5;
                c.g[V0][0] = c.g[V0][0] & 0x1;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.mtc1(2, 12);
                c.cvt_s_w(12, 12);
                c.add_s(12, 12, 12);
                pc = 0x003d2d88; continue;
            }
            0x003d2d88 => {
                c.cycles += 5;
                c.div_s(20, 0, 12);
                c.c_lt_s(21, 20);
                
                if c.fcc() { c.mov_s(21, 20); pc = 0x003d2d9c; continue; }
                pc = 0x003d2d9c; continue;
            }
            0x003d2d9c => {
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = (c.g[A0][0] < c.g[A1][0]) as u64;
                
                if c.g[V0][0] != 0u64 { c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8)); if back_edge_unwinds(c, 0x003d2d28) { return Ok(()); } pc = 0x003d2d28; continue; }
                pc = 0x003d2dac; continue;
            }
            0x003d2dac => {
                c.cycles += 10;
                c.g[RA][0] = sx(0x003d2db4);
                c.mul_s(12, 20, 1);
                call_ptr(c, 0x00413af8, 0x003d2db4)?;
                c.g[A1][0] = 0xff80;
                c.g[A1][0] = c.g[A1][0] << 46;
                c.g[RA][0] = sx(0x003d2dc4);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00411f18, 0x003d2dc4)?;
                c.g[RA][0] = sx(0x003d2dcc);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x004126a0, 0x003d2dcc)?;
                c.sw((c.g[S2][0] as u32).wrapping_sub(0xbd0), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_sub(0xbe0), 0u64);
                pc = 0x003d2dd4; continue;
            }
            0x003d2dd4 => {
                c.cycles += 8;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003d2c20, 0x003d2df4); return Ok(()); }
        }
    }
    });
}

/// `void PATH_FadeVolume(u32 mask, int volume, int time)` (0x3d5698): when the
/// path system is initialised and no map is selected, fades every playing
/// track slot (bit set in `mask`) of each loaded map whose class and group
/// bits match `mask` (`PATH_TrackFadeVolume(slot, volume, time, map.+0x18)`),
/// then forgets the selection. Converted from the generated block body.
pub fn path_fade_volume(c: &mut Cpu) {
    guest(c, |c| {
'b_3d57d4: {
'b_3d57d0: {
c.cycles += 17;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
c.g[V0][0] = 0x450000;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S1][0] = c.g[A0][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S3][0] = c.g[A2][0];
c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0xbd4));
c.g[S4][0] = c.g[A1][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] == 0u64;
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
if t {
break 'b_3d57d0;
} else {
c.cycles += 4;
c.g[V1][0] = 0x450000;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbd0));
let t = c.g[V0][0] != 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_3d57d4;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x003d56f4);
c.g[A0][0] = 0xffffffffffffffff;
Callee { addr: 0x003d5450, f: ee::s14::PATH_SelectMap }.call(c, 0x003d56f4)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x510000;
if t {
break 'b_3d57d0;
} else {
c.cycles += 7;
c.g[A0][0] = 0u64;
c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7644));
c.g[S5][0] = 0x450000;
c.g[FP][0] = 0xfffffffff0000000;
c.g[S7][0] = 0xf000000;
c.g[V0][0] = 0x928;
'l_3d5718: loop {
'b_3d57b8: {
'b_3d57b4: {
c.cycles += 5;
c.mult(0, 5, 4, 2);
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S6][0] as u32)));
c.g[V1][0] = c.lw((c.g[V0][0] as u32));
let t = c.g[V1][0] == 0u64;
c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
if t {
break 'b_3d57b4;
} else {
c.cycles += 8;
c.g[V0][0] = 0x510000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7640));
c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V1][0] & c.g[FP][0];
c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.g[V1][0] & c.g[S7][0];
if t {
break 'b_3d57b4;
} else {
c.cycles += 3;
c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
if c.g[V0][0] == 0u64 {
c.g[A0][0] = c.g[S2][0];
break 'b_3d57b8;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x003d5760);
Callee { addr: 0x003d5450, f: ee::s14::PATH_SelectMap }.call(c, 0x003d5760)?;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S2][0];
if t {
break 'b_3d57b8;
} else {
c.cycles += 2;
c.g[S0][0] = 0u64;
c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0xbe0));
'l_3d5770: loop {
'b_3d57a8: {
'b_3d57a4: {
c.cycles += 5;
c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
let t = c.g[A0][0] == 0u64;
c.g[V0][0] = sx((c.g[S1][0] as u32) >> ((c.g[S0][0] as u32) & 31));
if t {
break 'b_3d57a4;
} else {
c.cycles += 4;
c.g[V0][0] = c.g[V0][0] ^ 0x1;
c.g[V0][0] = c.g[V0][0] & 0x1;
if c.g[V0][0] != 0u64 {
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
break 'b_3d57a8;
} else {
c.cycles += 4;
c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
c.g[A1][0] = c.g[S4][0];
c.g[RA][0] = sx(0x003d57a4);
c.g[A2][0] = c.g[S3][0];
Callee { addr: 0x003d6318, f: ee::s14::PATH_TrackFadeVolume }.call(c, 0x003d57a4)?;
break 'b_3d57a4;
}
}
}
c.cycles += 1;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
break 'b_3d57a8;
}
c.cycles += 3;
c.g[V0][0] = ((c.g[S0][0] as i64) < 24i64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0xbe0));
if t {
if event_due(c) { deliver_event_here(c, 0x003d5770); } continue 'l_3d5770;
} else {
break 'b_3d57b4;
}
}
}
}
}
}
}
c.cycles += 1;
c.g[A0][0] = c.g[S2][0];
break 'b_3d57b8;
}
c.cycles += 3;
c.g[V0][0] = ((c.g[A0][0] as i64) < 4i64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x928;
if t {
if event_due(c) { deliver_event_here(c, 0x003d5718); } continue 'l_3d5718;
} else {
c.cycles += 3;
c.g[V0][0] = 0x450000;
c.sw((c.g[S5][0] as u32).wrapping_sub(0xbe0), 0u64);
c.sw((c.g[V0][0] as u32).wrapping_sub(0xbd0), 0u64);
break 'b_3d57d0;
}
}
}
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_3d57d4;
}
c.cycles += 11;
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
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{check, elf_image, fresh, run_generated, DATA};

    /// Hook word `D_0044F428` that `PATH_RemoveEvent` calls when non-zero.
    const PATHS_EVENT_HOOK: u32 = 0x0044_f428;

    /// Distinct cycle totals of the generated body over the seeds: the
    /// fixture really reaches many paths.
    fn paths_seen(
        generated: fn(&mut Cpu),
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    /// Four maps with random flags (+0x30: class nibble 31..28, group nibble
    /// 27..24), loaded word (+0x34), 24 track slots (+0x58) and 16 event
    /// slots (+0xdc), the path flag and the selected id.
    fn paths_maps(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        c.bus
            .write8(PATHS_INIT_FLAG, if seed % 7 == 0 { 0 } else { 1 });
        c.bus.write32(
            PATHS_CUR_ID,
            if seed % 5 == 0 { r.next_u32() | 1 } else { 0 },
        );
        c.bus.write32(PATHS_CUR_MAP, 0);
        for i in 0..4u32 {
            let m = PATHS_MAPS + i * 0x928;
            for w in 0..(0x928 / 4) {
                c.bus.write32(m + 4 * w, r.next_u32());
            }
            let upper = if (seed + i) % 3 == 0 {
                r.next_u32() & 0xff00_0000
            } else {
                0xff00_0000 | (r.next_u32() & 0x1100_0000)
            };
            c.bus.write32(m + 0x30, upper | (r.next_u32() & 0xff_ffff));
            let loaded = if (seed + i) % 4 == 3 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(m + 0x34, loaded);
            c.bus.write32(m + 0x91c, r.next_u32() % 16);
        }
    }

    /// `PATH_FlushEvents(mask)`: the selected map's event slots are removed
    /// (`PATH_RemoveEvent(slot, 2)`), through real callees; the event hook
    /// is a fake for half of the seeds.
    #[test]
    fn path_flush_events_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = fakes::machine(c, seed, 0x3d_1940, 4);
            c.pc = 0x003d_1940;
            paths_maps(c, &mut r, seed);
            c.bus
                .write32(PATHS_EVENT_HOOK, if seed % 2 == 0 { 0 } else { FAKE });
            for i in 0..4u32 {
                let m = PATHS_MAPS + i * 0x928;
                for j in 0..16u32 {
                    let ev = match (seed + i + j) % 4 {
                        0 => 0,
                        1 => DATA + 0x2_0000 + j * 0x100,
                        _ => {
                            // Inside the map's event area: removing it
                            // shifts the rest down (memcpy of the tail).
                            let ev = m + 0x200 + 0x20 * j;
                            c.bus.write16(ev + 0xe, (r.next_u32() % 5) as u16);
                            ev
                        }
                    };
                    c.bus.write32(m + 0xdc + 4 * j, ev);
                }
            }
            c.g[A0][0] = match seed % 4 {
                0 | 1 => sx(0xffff_ffff),
                2 => sx(r.next_u32() | 0x1100_0000),
                _ => sx(r.next_u32()),
            };
        };
        check(
            ee::s14::f_003d1940__generated,
            path_flush_events_v2,
            200,
            &setup,
        );
        assert!(paths_seen(ee::s14::f_003d1940__generated, 200, &setup) >= 8);
    }

    /// `PATH_GetMaxBranchRatio(pathId, scale)`: the segment scan and the
    /// soft-float rounding (real callees; the image holds their constants).
    #[test]
    fn path_get_max_branch_ratio_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = fakes::machine(c, seed, 0x3d_2c20, 4);
            elf_image(c);
            c.pc = 0x003d_2c20;
            paths_maps(c, &mut r, seed);
            let base = DATA + 0x1_0000;
            let tab = DATA + 0x2_0000;
            // Branch table: entry k covers 6 ints (two 8-byte segments);
            // some entries are empty.
            for k in 0..32u32 {
                let start = 0x20 + 6 * k;
                let end = if (seed + k) % 5 == 0 { start } else { start + 6 };
                c.bus.write32(tab + 4 * k, start);
                c.bus.write32(tab + 4 * k + 4, end);
            }
            for w in 0..0x200u32 {
                let v = r.next_u32();
                c.bus.write32(base + 4 * w, if w % 3 == 1 && v % 11 == 0 { 0 } else { v >> (v % 20) });
            }
            c.bus.write8(base + 0xd, [3, 30, 8, 24][(seed % 4) as usize]);
            // `PATH_FindMap` matches a loaded map whose id word equals the
            // top byte of the path id exactly.
            let mut upper = 0;
            for i in 0..4u32 {
                let m = PATHS_MAPS + i * 0x928;
                let id = (0x21 + 0x10 * i + seed % 5) << 24;
                c.bus.write32(m + 0x30, id);
                c.bus.write32(m + 0x4c, tab);
                if (seed + i) % 4 != 3 {
                    c.bus.write32(m + 0x34, base);
                }
                if i == seed % 4 {
                    upper = id;
                }
            }
            let bit = match seed % 8 {
                7 => 0,
                k => 1u32 << (k % 6),
            };
            let top = if seed % 9 == 0 { r.next_u32() & 0xff00_0000 } else { upper };
            c.g[A0][0] = sx(top | bit | (r.next_u32() & 0xe0));
            c.g[A1][0] = sx(r.next_u32() % 1000);
        };
        check(
            ee::s14::f_003d2c20__generated,
            path_get_max_branch_ratio,
            200,
            &setup,
        );
        assert!(paths_seen(ee::s14::f_003d2c20__generated, 200, &setup) >= 8);
    }

    /// `PATH_FadeVolume(mask, volume, time)`: `PATH_TrackFadeVolume` on every
    /// selected slot of each matching map (real callees).
    #[test]
    fn path_fade_volume_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = fakes::machine(c, seed, 0x3d_5698, 4);
            c.pc = 0x003d_5698;
            paths_maps(c, &mut r, seed);
            for i in 0..4u32 {
                let m = PATHS_MAPS + i * 0x928;
                for j in 0..24u32 {
                    let slot = if (seed + i + j) % 3 == 0 {
                        0
                    } else {
                        DATA + 0x3_0000 + (i * 24 + j) * 0x80
                    };
                    c.bus.write32(m + 0x58 + 4 * j, slot);
                }
            }
            c.g[A0][0] = match seed % 3 {
                0 => sx(0xffff_ffff),
                1 => sx(r.next_u32() | 0x1100_0000),
                _ => sx(r.next_u32()),
            };
            c.g[A1][0] = sx(r.next_u32() % 128);
            c.g[A2][0] = sx(r.next_u32() % 64);
        };
        check(
            ee::s14::f_003d5698__generated,
            path_fade_volume,
            200,
            &setup,
        );
        assert!(paths_seen(ee::s14::f_003d5698__generated, 200, &setup) >= 8);
    }
}
