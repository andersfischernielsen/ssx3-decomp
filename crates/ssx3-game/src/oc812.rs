//! Staging module oc812 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc812`.

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
    (0x003c_fc30, path_execute_action, "PATH_ExecuteAction"),
    (0x003d_16f0, path_post_event, "PATH_PostEvent"),
    (0x003d_25f0, path_add_track, "PATH_AddTrack"),
    (0x003d_2a68, path_stop_tracks, "PATH_StopTracks"),
];

/// `PATH_ExecuteAction` (0x003cfc30): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn path_execute_action(c: &mut Cpu) {
    if c.pc != 0x003cfc30 {
        return ee::s14::f_003cfc30__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x003cfc30;
        loop {
            match pc {
                0x003cfc30 => {
                    c.cycles += 22;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1e0));
                    c.g[V0][0] = 0x450000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.g[S3][0] = 0u64;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                    c.g[S7][0] = c.g[V0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x180));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x170));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x140), c.g[RA][0]);
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x190));
                    c.g[S4][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0xbe0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x130), c.g[A0][0]);
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x91c));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x138), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x134), c.g[V1][0]);
                    c.g[A1][0] = 0x1;
                    pc = 0x003cfc88;
                    continue;
                }
                0x003cfc88 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << ((c.g[S3][0] as u32) & 31));
                    c.g[V0][0] = c.g[V1][0] & c.g[A1][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003d0acc;
                        continue;
                    }
                    pc = 0x003cfc9c;
                    continue;
                }
                0x003cfc9c => {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[A0][0] = 0xffffffffff000000;
                    c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                    c.g[RA][0] = sx(0x003cfcb8);
                    c.g[A0][0] = c.g[A0][0] | c.g[A1][0];
                    Callee {
                        addr: 0x003d5290,
                        f: ee::s14::PATH_FindTrack,
                    }
                    .call(c, 0x003cfcb8)?;
                    c.g[S2][0] = c.g[V0][0];

                    if c.g[S2][0] == 0u64 {
                        c.g[S3][0] = c.g[S5][0];
                        pc = 0x003d0ad0;
                        continue;
                    }
                    pc = 0x003cfcc4;
                    continue;
                }
                0x003cfcc4 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x9));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[A0][0] = c.g[V0][0] & 0xff;
                    c.g[V1][0] = (c.g[A0][0] < 0x10u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        pc = 0x003d0abc;
                        continue;
                    }
                    pc = 0x003cfcdc;
                    continue;
                }
                0x003cfcdc => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5a30));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x003cfcf4 => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[V0][0] = 0x450000;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0xbc8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    c.g[V1][0] = (c.g[V1][0] < c.g[S4][0]) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003d0acc;
                        continue;
                    }
                    pc = 0x003cfd14;
                    continue;
                }
                0x003cfd14 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d0ac0;
                        continue;
                    }
                    pc = 0x003cfd1c;
                    continue;
                }
                0x003cfd1c => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                    c.g[RA][0] = sx(0x003cfd28);
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x002aec28, 0x003cfd28)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003cfd34);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                    Callee {
                        addr: 0x003d32b8,
                        f: ee::s14::PATH_TrackGetBeatPos,
                    }
                    .call(c, 0x003cfd34)?;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x003d0acc;
                        continue;
                    }
                    pc = 0x003cfd3c;
                    continue;
                }
                0x003cfd3c => {
                    c.cycles += 7;
                    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x9), c.g[V0][0]);
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = sx((c.g[T0][0] as u32) >> 4);
                    c.g[A3][0] = c.g[V0][0] & 0xf;

                    if c.g[A3][0] == c.g[V1][0] {
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x124));
                        pc = 0x003cfd84;
                        continue;
                    }
                    pc = 0x003cfd58;
                    continue;
                }
                0x003cfd58 => {
                    c.cycles += 2;
                    let t = c.g[A3][0] == 0u64;
                    c.g[T1][0] = 0x2;
                    if t {
                        pc = 0x003cfd78;
                        continue;
                    }
                    pc = 0x003cfd60;
                    continue;
                }
                0x003cfd60 => {
                    c.cycles += 2;
                    let t = c.g[A3][0] == c.g[T1][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x003cfd8c;
                        continue;
                    }
                    pc = 0x003cfd68;
                    continue;
                }
                0x003cfd68 => {
                    c.cycles += 2;
                    let t = c.g[A3][0] == c.g[V0][0];
                    c.g[S3][0] = c.g[S5][0];
                    if t {
                        pc = 0x003cfd98;
                        continue;
                    }
                    pc = 0x003cfd70;
                    continue;
                }
                0x003cfd70 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 24i64) as u64;
                    if t {
                        pc = 0x003d0ad4;
                        continue;
                    }
                    pc = 0x003cfd78;
                    continue;
                }
                0x003cfd78 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x120));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if t {
                        pc = 0x003d0acc;
                        continue;
                    }
                    pc = 0x003cfd84;
                    continue;
                }
                0x003cfd84 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if t {
                        pc = 0x003d0acc;
                        continue;
                    }
                    pc = 0x003cfd8c;
                    continue;
                }
                0x003cfd8c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x128));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if t {
                        pc = 0x003d0acc;
                        continue;
                    }
                    pc = 0x003cfd98;
                    continue;
                }
                0x003cfd98 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[A2][0] = c.g[T0][0] & 0xf;
                    c.g[V1][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x36));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = c.lh(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    let t = c.g[A2][0] != 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        pc = 0x003cfdf0;
                        continue;
                    }
                    pc = 0x003cfdc4;
                    continue;
                }
                0x003cfdc4 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = 0xffffffffff000000;
                    c.g[V1][0] = c.g[V1][0] | 0xfff;
                    c.g[A0][0] = 0xfffffffffffffff0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 20);
                    c.g[V1][0] = c.g[T0][0] & c.g[V1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 12);
                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                    c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                    c.g[V1][0] = c.g[V1][0] | 0x1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    pc = 0x003cfdf0;
                    continue;
                }
                0x003cfdf0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 12);
                    c.g[V0][0] = c.g[V0][0] & 0xfff;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 20);
                    let t = c.g[V0][0] == c.g[V1][0];
                    c.g[V0][0] = 0x450000;
                    if t {
                        pc = 0x003cfe28;
                        continue;
                    }
                    pc = 0x003cfe0c;
                    continue;
                }
                0x003cfe0c => {
                    c.cycles += 7;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x9), c.g[T1][0]);
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x128));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0xbc8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x130));
                    let t = true;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    if t {
                        pc = 0x003d0acc;
                        continue;
                    }
                    pc = 0x003cfe28;
                    continue;
                }
                0x003cfe28 => {
                    c.cycles += 2;
                    let t = true;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x9), c.g[A3][0]);
                    if t {
                        pc = 0x003d0acc;
                        continue;
                    }
                    pc = 0x003cfe30;
                    continue;
                }
                0x003cfe30 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V1][0] = c.g[S2][0];
                    c.g[V0][0] = c.g[S2][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[FP][0] = c.g[S2][0] | c.g[SP][0];
                    if t {
                        pc = 0x003cfea8;
                        continue;
                    }
                    pc = 0x003cfe44;
                    continue;
                }
                0x003cfe44 => {
                    c.cycles += 4;
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x100));
                    c.g[T0][0] = sx((c.g[S4][0] as u32) << 16);
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 8);
                    pc = 0x003cfe54;
                    continue;
                }
                0x003cfe54 => {
                    c.cycles += 19;
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(6, c.g[V1][0] as u32);
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.ldl(10, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(10, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(6, c.g[A0][0] as u32);
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sdl(10, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.sdr(10, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[A1][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x003cfe54) {
                            return Ok(());
                        }
                        pc = 0x003cfe54;
                        continue;
                    }
                    pc = 0x003cfea0;
                    continue;
                }
                0x003cfea0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    if t {
                        pc = 0x003cfee8;
                        continue;
                    }
                    pc = 0x003cfea8;
                    continue;
                }
                0x003cfea8 => {
                    c.cycles += 4;
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x100));
                    c.g[T0][0] = sx((c.g[S4][0] as u32) << 16);
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 8);
                    pc = 0x003cfeb8;
                    continue;
                }
                0x003cfeb8 => {
                    c.cycles += 11;
                    c.g[A2][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[A2][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[A1][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x003cfeb8) {
                            return Ok(());
                        }
                        pc = 0x003cfeb8;
                        continue;
                    }
                    pc = 0x003cfee4;
                    continue;
                }
                0x003cfee4 => {
                    c.cycles += 1;
                    c.g[S0][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    pc = 0x003cfee8;
                    continue;
                }
                0x003cfee8 => {
                    c.cycles += 12;
                    c.g[S3][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f));
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(5, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(5, c.g[A0][0] as u32);
                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.g[V0][0] = (c.g[V0][0] < 0x10u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003cff30;
                        continue;
                    }
                    pc = 0x003cff18;
                    continue;
                }
                0x003cff18 => {
                    c.cycles += 6;
                    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[S0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xbc));
                    pc = 0x003cff30;
                    continue;
                }
                0x003cff30 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < -15i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < -31i64) as u64;
                    if t {
                        pc = 0x003cff58;
                        continue;
                    }
                    pc = 0x003cff3c;
                    continue;
                }
                0x003cff3c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[S3][0] as u32));
                    if t {
                        pc = 0x003cff58;
                        continue;
                    }
                    pc = 0x003cff44;
                    continue;
                }
                0x003cff44 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S3][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xbc));
                    pc = 0x003cff58;
                    continue;
                }
                0x003cff58 => {
                    c.cycles += 2;
                    let t = (c.g[S0][0] as i64) < 0;
                    c.g[V1][0] = 0x450000;
                    if t {
                        pc = 0x003cff78;
                        continue;
                    }
                    pc = 0x003cff60;
                    continue;
                }
                0x003cff60 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x12));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0x450000;
                    if t {
                        pc = 0x003cffa0;
                        continue;
                    }
                    pc = 0x003cff78;
                    continue;
                }
                0x003cff78 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbc4));
                    c.g[V0][0] = c.g[V0][0] & 0x20;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x490000;
                    if t {
                        pc = 0x003cff94;
                        continue;
                    }
                    pc = 0x003cff88;
                    continue;
                }
                0x003cff88 => {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003cff94);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x59d0));
                    Callee {
                        addr: 0x003d1658,
                        f: ee::s14::PATH_DebugPrintf,
                    }
                    .call(c, 0x003cff94)?;
                    pc = 0x003cff94;
                    continue;
                }
                0x003cff94 => {
                    c.cycles += 3;
                    c.g[S0][0] = 0xffffffffffffffff;
                    let t = true;
                    c.g[T0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x36));
                    if t {
                        pc = 0x003cffc4;
                        continue;
                    }
                    pc = 0x003cffa0;
                    continue;
                }
                0x003cffa0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffffffffffffffdf;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[V0][0] = 0xffffffffffffffe0;
                    if t {
                        pc = 0x003cffb8;
                        continue;
                    }
                    pc = 0x003cffac;
                    continue;
                }
                0x003cffac => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x36));
                    let t = true;
                    c.g[T0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x36));
                    if t {
                        pc = 0x003cffc4;
                        continue;
                    }
                    pc = 0x003cffb8;
                    continue;
                }
                0x003cffb8 => {
                    c.cycles += 2;
                    let t = c.g[S3][0] != c.g[V0][0];
                    c.g[T0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x36));
                    if t {
                        pc = 0x003cffc4;
                        continue;
                    }
                    pc = 0x003cffc0;
                    continue;
                }
                0x003cffc0 => {
                    c.cycles += 1;
                    c.g[S3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    pc = 0x003cffc4;
                    continue;
                }
                0x003cffc4 => {
                    c.cycles += 2;
                    let t = (c.g[S3][0] as i64) < 0;
                    c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    if t {
                        pc = 0x003d0048;
                        continue;
                    }
                    pc = 0x003cffcc;
                    continue;
                }
                0x003cffcc => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x12));
                    let t = c.g[V1][0] == 0u64;
                    c.g[A2][0] = 0u64;
                    if t {
                        pc = 0x003d0034;
                        continue;
                    }
                    pc = 0x003cffdc;
                    continue;
                }
                0x003cffdc => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x38));
                    c.g[T1][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[V1][0];
                    pc = 0x003cffe8;
                    continue;
                }
                0x003cffe8 => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lh(c.g[A0][0] as u32);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 21);
                    c.g[V0][0] = c.g[V0][0] & 0x3f;

                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        pc = 0x003d0024;
                        continue;
                    }
                    pc = 0x003d0008;
                    continue;
                }
                0x003d0008 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh(c.g[A1][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        pc = 0x003d0024;
                        continue;
                    }
                    pc = 0x003d0014;
                    continue;
                }
                0x003d0014 => {
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));

                    if c.g[S0][0] == 0u64 {
                        c.g[S0][0] = c.g[A2][0];
                        pc = 0x003d0030;
                        continue;
                    }
                    pc = 0x003d0020;
                    continue;
                }
                0x003d0020 => {
                    c.cycles += 1;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    pc = 0x003d0024;
                    continue;
                }
                0x003d0024 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[A2][0] < c.g[A3][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                    if t {
                        if back_edge_unwinds(c, 0x003cffe8) {
                            return Ok(());
                        }
                        pc = 0x003cffe8;
                        continue;
                    }
                    pc = 0x003d0030;
                    continue;
                }
                0x003d0030 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    pc = 0x003d0034;
                    continue;
                }
                0x003d0034 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0xffffffffffffffff;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x12));
                    c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[A1][0];
                    }
                    pc = 0x003d0048;
                    continue;
                }
                0x003d0048 => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[T0][0] as u32) << 16);
                    c.g[V0][0] = 0x1000000;
                    c.g[V0][0] = c.g[S4][0] & c.g[V0][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[S6][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
                    if t {
                        pc = 0x003d006c;
                        continue;
                    }
                    pc = 0x003d005c;
                    continue;
                }
                0x003d005c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d0064);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                    call_ptr(c, 0x002af838, 0x003d0064)?;
                    let t = true;
                    c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                    if t {
                        pc = 0x003d00b0;
                        continue;
                    }
                    pc = 0x003d006c;
                    continue;
                }
                0x003d006c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                    if t {
                        pc = 0x003d00b0;
                        continue;
                    }
                    pc = 0x003d0078;
                    continue;
                }
                0x003d0078 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x2f));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = 0xffffffffffffd8f1;
                    if t {
                        pc = 0x003d00b4;
                        continue;
                    }
                    pc = 0x003d0084;
                    continue;
                }
                0x003d0084 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x003d008c);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x003d0cc8,
                        f: ee::s14::PATH_TrackGetStatus,
                    }
                    .call(c, 0x003d008c)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x100));
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                    if t {
                        pc = 0x003d00ac;
                        continue;
                    }
                    pc = 0x003d009c;
                    continue;
                }
                0x003d009c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[V1][0] ^ 0x5;
                    c.g[V0][0] = 0xffffffffffffd8f1;
                    if c.g[V1][0] != 0 {
                        c.g[T1][0] = c.g[V0][0];
                    }
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x138), c.g[T1][0]);
                    pc = 0x003d00ac;
                    continue;
                }
                0x003d00ac => {
                    c.cycles += 1;
                    c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                    pc = 0x003d00b0;
                    continue;
                }
                0x003d00b0 => {
                    c.cycles += 1;
                    c.g[S3][0] = 0xffffffffffffd8f1;
                    pc = 0x003d00b4;
                    continue;
                }
                0x003d00b4 => {
                    c.cycles += 2;
                    let t = c.g[T2][0] == c.g[S3][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                    if t {
                        pc = 0x003d00e0;
                        continue;
                    }
                    pc = 0x003d00bc;
                    continue;
                }
                0x003d00bc => {
                    c.cycles += 9;
                    c.g[A2][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x003d00cc);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x003d3900,
                        f: ee::s14::PATH_TrackQueueNode,
                    }
                    .call(c, 0x003d00cc)?;
                    c.sh((c.g[S2][0] as u32).wrapping_add(0x36), c.g[V0][0]);
                    c.g[RA][0] = sx(0x003d00d8);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x003d41a8,
                        f: ee::s14::PATH_TrackSubmitQueue,
                    }
                    .call(c, 0x003d00d8)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x138), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                    pc = 0x003d00e0;
                    continue;
                }
                0x003d00e0 => {
                    c.cycles += 2;

                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d0ac0;
                        continue;
                    }
                    pc = 0x003d00e8;
                    continue;
                }
                0x003d00e8 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                    let t = c.g[V1][0] == c.g[S3][0];
                    c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    if t {
                        pc = 0x003d0acc;
                        continue;
                    }
                    pc = 0x003d00f4;
                    continue;
                }
                0x003d00f4 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x134));
                    c.g[V0][0] = c.g[FP][0] & 0x7;
                    c.g[A1][0] = c.g[SP][0];
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x91c), c.g[A2][0]);
                    if t {
                        pc = 0x003d0164;
                        continue;
                    }
                    pc = 0x003d010c;
                    continue;
                }
                0x003d010c => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    pc = 0x003d0110;
                    continue;
                }
                0x003d0110 => {
                    c.cycles += 19;
                    c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(7, c.g[A1][0] as u32);
                    c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0xf));
                    c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x8));
                    c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x17));
                    c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x10));
                    c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0x1f));
                    c.ldr(10, (c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(7, c.g[A0][0] as u32);
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sdl(10, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.sdr(10, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x003d0110) {
                            return Ok(());
                        }
                        pc = 0x003d0110;
                        continue;
                    }
                    pc = 0x003d015c;
                    continue;
                }
                0x003d015c => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x003d0194;
                        continue;
                    }
                    pc = 0x003d0164;
                    continue;
                }
                0x003d0164 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    pc = 0x003d0168;
                    continue;
                }
                0x003d0168 => {
                    c.cycles += 11;
                    c.g[V1][0] = c.ld(c.g[A1][0] as u32);
                    c.g[A2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[V1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x003d0168) {
                            return Ok(());
                        }
                        pc = 0x003d0168;
                        continue;
                    }
                    pc = 0x003d0194;
                    continue;
                }
                0x003d0194 => {
                    c.cycles += 9;
                    c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(9, c.g[A1][0] as u32);
                    c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0xf));
                    c.ldr(10, (c.g[A1][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(9, c.g[A0][0] as u32);
                    c.sdl(10, (c.g[A0][0] as u32).wrapping_add(0xf));
                    let t = true;
                    c.sdr(10, (c.g[A0][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d0acc;
                        continue;
                    }
                    pc = 0x003d01b8;
                    continue;
                }
                0x003d01b8 => {
                    c.cycles += 9;
                    c.g[S0][0] = c.g[S4][0] & 0x1;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003d01cc);
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x002af7a0, 0x003d01cc)?;
                    c.sb((c.g[S2][0] as u32).wrapping_add(0x3), c.g[S0][0]);
                    c.g[V1][0] = 0x4000000;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d0ac4;
                        continue;
                    }
                    pc = 0x003d01dc;
                    continue;
                }
                0x003d01dc => {
                    c.cycles += 13;
                    c.g[A3][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[V1][0] = sx((c.g[S4][0] as u32) << 17);
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 25) as u32);
                    c.g[V0][0] = sx((c.g[S4][0] as u32) >> 16);
                    c.g[A1][0] = sx((c.g[S4][0] as u32) << 24);
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.g[A2][0] = c.g[V1][0];
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003d0208);
                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                    Callee {
                        addr: 0x003d6378,
                        f: ee::s14::PATH_TrackStartVolumeFade,
                    }
                    .call(c, 0x003d0208)?;
                    let t = true;
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003d0abc;
                        continue;
                    }
                    pc = 0x003d0210;
                    continue;
                }
                0x003d0210 => {
                    c.cycles += 13;
                    c.g[A3][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[V1][0] = sx((c.g[S4][0] as u32) << 17);
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 25) as u32);
                    c.g[V0][0] = sx((c.g[S4][0] as u32) >> 16);
                    c.g[A1][0] = sx((c.g[S4][0] as u32) << 24);
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    c.g[A2][0] = c.g[V1][0];
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003d023c);
                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                    Callee {
                        addr: 0x003d63f8,
                        f: ee::s14::PATH_TrackStartAuxSendFade,
                    }
                    .call(c, 0x003d023c)?;
                    let t = true;
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003d0abc;
                        continue;
                    }
                    pc = 0x003d0244;
                    continue;
                }
                0x003d0244 => {
                    c.cycles += 13;
                    c.g[A3][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[V1][0] = sx((c.g[S4][0] as u32) << 17);
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 25) as u32);
                    c.g[V0][0] = sx((c.g[S4][0] as u32) >> 16);
                    c.g[A1][0] = sx((c.g[S4][0] as u32) << 24);
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    c.g[A2][0] = c.g[V1][0];
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003d0270);
                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                    Callee {
                        addr: 0x003d6448,
                        f: ee::s14::PATH_TrackStartFade3,
                    }
                    .call(c, 0x003d0270)?;
                    let t = true;
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003d0abc;
                        continue;
                    }
                    pc = 0x003d0278;
                    continue;
                }
                0x003d0278 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                    c.g[A1][0] = sx((c.g[S4][0] as u32) << 24);
                    c.g[A2][0] = sx((c.g[S4][0] as u32) << 16);
                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 24) as u32);
                    c.g[RA][0] = sx(0x003d0294);
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x002af4e0, 0x003d0294)?;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d02f8;
                        continue;
                    }
                    pc = 0x003d029c;
                    continue;
                }
                0x003d029c => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                    c.g[A1][0] = sx((c.g[S4][0] as u32) << 24);
                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                    c.g[RA][0] = sx(0x003d02b0);
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x002af580, 0x003d02b0)?;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d02f8;
                        continue;
                    }
                    pc = 0x003d02b8;
                    continue;
                }
                0x003d02b8 => {
                    c.cycles += 16;
                    c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 24);
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 23) as u32);
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.lh(c.g[V0][0] as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[RA][0] = sx(0x003d02e4);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Callee {
                        addr: 0x003d2068,
                        f: ee::s14::PATHI_copyevent,
                    }
                    .call(c, 0x003d02e4)?;
                    c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003d02f4);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x30));
                    Callee {
                        addr: 0x003d1f80,
                        f: ee::s14::PATH_AddEventToMaps,
                    }
                    .call(c, 0x003d02f4)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    pc = 0x003d02f8;
                    continue;
                }
                0x003d02f8 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x4000000;
                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x003d0acc;
                        continue;
                    }
                    pc = 0x003d0308;
                    continue;
                }
                0x003d0308 => {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 24);
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    if t {
                        pc = 0x003d032c;
                        continue;
                    }
                    pc = 0x003d031c;
                    continue;
                }
                0x003d031c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d0324);
                    Callee {
                        addr: 0x003d2218,
                        f: ee::s14::PATH_ClearEventMarkBits,
                    }
                    .call(c, 0x003d0324)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d0358;
                        continue;
                    }
                    pc = 0x003d032c;
                    continue;
                }
                0x003d032c => {
                    c.cycles += 11;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[A1][0] = sx((c.g[S4][0] as u32) << 16);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                    c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.lh(c.g[V0][0] as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[RA][0] = sx(0x003d0354);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Callee {
                        addr: 0x003d21c0,
                        f: ee::s14::PATH_SetEventMarkBit,
                    }
                    .call(c, 0x003d0354)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    pc = 0x003d0358;
                    continue;
                }
                0x003d0358 => {
                    c.cycles += 3;
                    c.g[V1][0] = 0x4000000;
                    let t = true;
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003d0ac4;
                        continue;
                    }
                    pc = 0x003d0364;
                    continue;
                }
                0x003d0364 => {
                    c.cycles += 7;
                    c.sb((c.g[S2][0] as u32).wrapping_add(0x34), c.g[S4][0]);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x003d0378);
                    c.g[A1][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x4));
                    Callee {
                        addr: 0x003d62c8,
                        f: ee::s14::PATH_TrackSetVolume,
                    }
                    .call(c, 0x003d0378)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d0ac0;
                        continue;
                    }
                    pc = 0x003d0380;
                    continue;
                }
                0x003d0380 => {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 8);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 24);
                    c.g[V0][0] = (c.g[V0][0] < 0x10u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 16);
                    if t {
                        pc = 0x003d03ec;
                        continue;
                    }
                    pc = 0x003d0394;
                    continue;
                }
                0x003d0394 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0xffffffffffffffdf;
                    c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                    let t = c.g[A0][0] != c.g[V1][0];
                    c.g[V0][0] = 0xffffffffffffffe0;
                    if t {
                        pc = 0x003d03b8;
                        continue;
                    }
                    pc = 0x003d03a4;
                    continue;
                }
                0x003d03a4 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x36));
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[S4][0] = c.g[S4][0] & c.g[V0][0];
                    let t = true;
                    c.g[S4][0] = c.g[S4][0] | c.g[V1][0];
                    if t {
                        pc = 0x003d03d4;
                        continue;
                    }
                    pc = 0x003d03b8;
                    continue;
                }
                0x003d03b8 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 8);
                    if t {
                        pc = 0x003d03d8;
                        continue;
                    }
                    pc = 0x003d03c0;
                    continue;
                }
                0x003d03c0 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[V1][0] = 0xffffffffffff0000;
                    c.g[S4][0] = c.g[S4][0] & c.g[V1][0];
                    c.g[A0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[S4][0] = c.g[S4][0] | c.g[A0][0];
                    pc = 0x003d03d4;
                    continue;
                }
                0x003d03d4 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 8);
                    pc = 0x003d03d8;
                    continue;
                }
                0x003d03d8 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sh((c.g[V1][0] as u32).wrapping_add(0xbc), c.g[S4][0]);
                    pc = 0x003d03ec;
                    continue;
                }
                0x003d03ec => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = 0x4000000;
                    let t = true;
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003d0ac4;
                        continue;
                    }
                    pc = 0x003d03fc;
                    continue;
                }
                0x003d03fc => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 16);
                    c.g[A0][0] = c.g[V0][0] & 0xf;
                    c.g[V1][0] = (c.g[A0][0] < 0x6u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        pc = 0x003d09f8;
                        continue;
                    }
                    pc = 0x003d0418;
                    continue;
                }
                0x003d0418 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5a70));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x003d0430 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x003d0438);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x003d0cc8,
                        f: ee::s14::PATH_TrackGetStatus,
                    }
                    .call(c, 0x003d0438)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V0][0] = 0x5;
                    let t = c.g[S4][0] == c.g[V0][0];
                    c.g[V0][0] = 0x6;
                    if t {
                        pc = 0x003d0488;
                        continue;
                    }
                    pc = 0x003d0448;
                    continue;
                }
                0x003d0448 => {
                    c.cycles += 2;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[A1][0] = 0x400000;
                    if t {
                        pc = 0x003d04a0;
                        continue;
                    }
                    pc = 0x003d0450;
                    continue;
                }
                0x003d0450 => {
                    c.cycles += 14;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = 0xfffffffffff00000;
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.g[A0][0] = 0x20000;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                    c.g[V1][0] = 0x4000000;
                    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[V1][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x36));
                    c.sh((c.g[S1][0] as u32).wrapping_add(0xe), 0u64);
                    let t = true;
                    c.sh((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    if t {
                        pc = 0x003d09f8;
                        continue;
                    }
                    pc = 0x003d0488;
                    continue;
                }
                0x003d0488 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d04a0;
                        continue;
                    }
                    pc = 0x003d0494;
                    continue;
                }
                0x003d0494 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003d04a0;
                    continue;
                }
                0x003d04a0 => {
                    c.cycles += 2;
                    let t = c.g[S4][0] != c.g[A0][0];
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d04a8;
                    continue;
                }
                0x003d04a8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d04b0;
                    continue;
                }
                0x003d04b0 => {
                    c.cycles += 14;
                    c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 23);
                    c.g[A1][0] = sx((c.g[S4][0] as u32) << 16);
                    c.g[T1][0] = c.g[V0][0] & 0x7;
                    c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = sx(((c.g[A1][0] as u32) as i32 >> 15) as u32);
                    c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.g[T0][0] = c.g[A1][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[A0][0] = (c.g[T1][0] < 0x7u64) as u64;
                    c.g[V0][0] = c.lh(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    let t = c.g[A0][0] == 0u64;
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        pc = 0x003d09f8;
                        continue;
                    }
                    pc = 0x003d04e8;
                    continue;
                }
                0x003d04e8 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x490000;
                    c.g[V1][0] = sx((c.g[T1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5a90));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x003d0504 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx(((c.g[S4][0] as u32) as i32 >> 16) as u32);
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        pc = 0x003d0550;
                        continue;
                    }
                    pc = 0x003d0514;
                    continue;
                }
                0x003d0514 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x003d052c;
                        continue;
                    }
                    pc = 0x003d051c;
                    continue;
                }
                0x003d051c => {
                    c.cycles += 2;

                    if c.g[V1][0] == 0u64 {
                        c.g[V1][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x36));
                        pc = 0x003d053c;
                        continue;
                    }
                    pc = 0x003d0524;
                    continue;
                }
                0x003d0524 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d052c;
                    continue;
                }
                0x003d052c => {
                    c.cycles += 2;

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                        pc = 0x003d0594;
                        continue;
                    }
                    pc = 0x003d0534;
                    continue;
                }
                0x003d0534 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d053c;
                    continue;
                }
                0x003d053c => {
                    c.cycles += 3;
                    c.g[V0][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0548;
                    continue;
                }
                0x003d0548 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d0550;
                    continue;
                }
                0x003d0550 => {
                    c.cycles += 2;

                    if c.g[A1][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0558;
                    continue;
                }
                0x003d0558 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh(c.g[A1][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0564;
                    continue;
                }
                0x003d0564 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));

                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0570;
                    continue;
                }
                0x003d0570 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = 0xfffffffffff00000;
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                    c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d058c;
                    continue;
                }
                0x003d058c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d0594;
                    continue;
                }
                0x003d0594 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    if t {
                        pc = 0x003d09f8;
                        continue;
                    }
                    pc = 0x003d059c;
                    continue;
                }
                0x003d059c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 21);
                    c.g[V0][0] = c.g[V0][0] & 0x3f;
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d05b0;
                    continue;
                }
                0x003d05b0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d05b8;
                    continue;
                }
                0x003d05b8 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx(((c.g[S4][0] as u32) as i32 >> 16) as u32);
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        pc = 0x003d0604;
                        continue;
                    }
                    pc = 0x003d05c8;
                    continue;
                }
                0x003d05c8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x003d05e0;
                        continue;
                    }
                    pc = 0x003d05d0;
                    continue;
                }
                0x003d05d0 => {
                    c.cycles += 2;

                    if c.g[V1][0] == 0u64 {
                        c.g[V1][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x36));
                        pc = 0x003d05f0;
                        continue;
                    }
                    pc = 0x003d05d8;
                    continue;
                }
                0x003d05d8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d05e0;
                    continue;
                }
                0x003d05e0 => {
                    c.cycles += 2;

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                        pc = 0x003d0648;
                        continue;
                    }
                    pc = 0x003d05e8;
                    continue;
                }
                0x003d05e8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d05f0;
                    continue;
                }
                0x003d05f0 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d05fc;
                    continue;
                }
                0x003d05fc => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d0604;
                    continue;
                }
                0x003d0604 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));

                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0610;
                    continue;
                }
                0x003d0610 => {
                    c.cycles += 2;

                    if c.g[A1][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0618;
                    continue;
                }
                0x003d0618 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh(c.g[A1][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0624;
                    continue;
                }
                0x003d0624 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = 0xfffffffffff00000;
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                    c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                    let t = c.g[V0][0] == c.g[V1][0];
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0640;
                    continue;
                }
                0x003d0640 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d0648;
                    continue;
                }
                0x003d0648 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    if t {
                        pc = 0x003d09f8;
                        continue;
                    }
                    pc = 0x003d0650;
                    continue;
                }
                0x003d0650 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 21);
                    c.g[V0][0] = c.g[V0][0] & 0x3f;
                    let t = c.g[V0][0] == c.g[V1][0];
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0664;
                    continue;
                }
                0x003d0664 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d066c;
                    continue;
                }
                0x003d066c => {
                    c.cycles += 4;
                    c.g[V1][0] = sx(((c.g[S4][0] as u32) as i32 >> 16) as u32);
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        pc = 0x003d06bc;
                        continue;
                    }
                    pc = 0x003d067c;
                    continue;
                }
                0x003d067c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x003d0694;
                        continue;
                    }
                    pc = 0x003d0684;
                    continue;
                }
                0x003d0684 => {
                    c.cycles += 2;

                    if c.g[V1][0] == 0u64 {
                        c.g[V1][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x36));
                        pc = 0x003d06a4;
                        continue;
                    }
                    pc = 0x003d068c;
                    continue;
                }
                0x003d068c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0694;
                    continue;
                }
                0x003d0694 => {
                    c.cycles += 2;

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                        pc = 0x003d0700;
                        continue;
                    }
                    pc = 0x003d069c;
                    continue;
                }
                0x003d069c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d06a4;
                    continue;
                }
                0x003d06a4 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    pc = 0x003d06a8;
                    continue;
                }
                0x003d06a8 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d06b4;
                    continue;
                }
                0x003d06b4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d06bc;
                    continue;
                }
                0x003d06bc => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));

                    if c.g[A0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d06c8;
                    continue;
                }
                0x003d06c8 => {
                    c.cycles += 2;

                    if c.g[A1][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d06d0;
                    continue;
                }
                0x003d06d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh(c.g[A1][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d06dc;
                    continue;
                }
                0x003d06dc => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 20);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 20);
                    pc = 0x003d06ec;
                    continue;
                }
                0x003d06ec => {
                    c.cycles += 3;
                    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d06f8;
                    continue;
                }
                0x003d06f8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d0700;
                    continue;
                }
                0x003d0700 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    if t {
                        pc = 0x003d09f8;
                        continue;
                    }
                    pc = 0x003d0708;
                    continue;
                }
                0x003d0708 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 21);
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] & 0x3f;
                    if t {
                        if back_edge_unwinds(c, 0x003d06ec) {
                            return Ok(());
                        }
                        pc = 0x003d06ec;
                        continue;
                    }
                    pc = 0x003d0718;
                    continue;
                }
                0x003d0718 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx(((c.g[S4][0] as u32) as i32 >> 16) as u32);
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        pc = 0x003d0758;
                        continue;
                    }
                    pc = 0x003d0728;
                    continue;
                }
                0x003d0728 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x003d0740;
                        continue;
                    }
                    pc = 0x003d0730;
                    continue;
                }
                0x003d0730 => {
                    c.cycles += 2;

                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x36));
                        pc = 0x003d0750;
                        continue;
                    }
                    pc = 0x003d0738;
                    continue;
                }
                0x003d0738 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0740;
                    continue;
                }
                0x003d0740 => {
                    c.cycles += 2;

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                        pc = 0x003d079c;
                        continue;
                    }
                    pc = 0x003d0748;
                    continue;
                }
                0x003d0748 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0750;
                    continue;
                }
                0x003d0750 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    if t {
                        if back_edge_unwinds(c, 0x003d06a8) {
                            return Ok(());
                        }
                        pc = 0x003d06a8;
                        continue;
                    }
                    pc = 0x003d0758;
                    continue;
                }
                0x003d0758 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));

                    if c.g[A0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0764;
                    continue;
                }
                0x003d0764 => {
                    c.cycles += 2;

                    if c.g[A1][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d076c;
                    continue;
                }
                0x003d076c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh(c.g[A1][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0778;
                    continue;
                }
                0x003d0778 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 20);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 20);
                    pc = 0x003d0788;
                    continue;
                }
                0x003d0788 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0794;
                    continue;
                }
                0x003d0794 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d079c;
                    continue;
                }
                0x003d079c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    if t {
                        pc = 0x003d09f8;
                        continue;
                    }
                    pc = 0x003d07a4;
                    continue;
                }
                0x003d07a4 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 21);
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] & 0x3f;
                    if t {
                        if back_edge_unwinds(c, 0x003d0788) {
                            return Ok(());
                        }
                        pc = 0x003d0788;
                        continue;
                    }
                    pc = 0x003d07b4;
                    continue;
                }
                0x003d07b4 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx(((c.g[S4][0] as u32) as i32 >> 16) as u32);
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        pc = 0x003d07f8;
                        continue;
                    }
                    pc = 0x003d07c4;
                    continue;
                }
                0x003d07c4 => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = 0x2;
                        pc = 0x003d07dc;
                        continue;
                    }
                    pc = 0x003d07cc;
                    continue;
                }
                0x003d07cc => {
                    c.cycles += 2;

                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x36));
                        pc = 0x003d07ec;
                        continue;
                    }
                    pc = 0x003d07d4;
                    continue;
                }
                0x003d07d4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d07dc;
                    continue;
                }
                0x003d07dc => {
                    c.cycles += 2;

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                        pc = 0x003d0830;
                        continue;
                    }
                    pc = 0x003d07e4;
                    continue;
                }
                0x003d07e4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d07ec;
                    continue;
                }
                0x003d07ec => {
                    c.cycles += 1;
                    c.g[V1][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    pc = 0x003d07f0;
                    continue;
                }
                0x003d07f0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x003d09e0;
                        continue;
                    }
                    pc = 0x003d07f8;
                    continue;
                }
                0x003d07f8 => {
                    c.cycles += 2;

                    if c.g[A1][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0800;
                    continue;
                }
                0x003d0800 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh(c.g[A1][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d080c;
                    continue;
                }
                0x003d080c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0818;
                    continue;
                }
                0x003d0818 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 20);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 20);
                    pc = 0x003d0828;
                    continue;
                }
                0x003d0828 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    if t {
                        pc = 0x003d09e0;
                        continue;
                    }
                    pc = 0x003d0830;
                    continue;
                }
                0x003d0830 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    if t {
                        pc = 0x003d09f8;
                        continue;
                    }
                    pc = 0x003d0838;
                    continue;
                }
                0x003d0838 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 21);
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] & 0x3f;
                    if t {
                        if back_edge_unwinds(c, 0x003d0828) {
                            return Ok(());
                        }
                        pc = 0x003d0828;
                        continue;
                    }
                    pc = 0x003d0848;
                    continue;
                }
                0x003d0848 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx(((c.g[S4][0] as u32) as i32 >> 16) as u32);
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        pc = 0x003d0888;
                        continue;
                    }
                    pc = 0x003d0858;
                    continue;
                }
                0x003d0858 => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = 0x2;
                        pc = 0x003d0870;
                        continue;
                    }
                    pc = 0x003d0860;
                    continue;
                }
                0x003d0860 => {
                    c.cycles += 2;

                    if c.g[V1][0] == 0u64 {
                        c.g[V1][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x36));
                        pc = 0x003d0880;
                        continue;
                    }
                    pc = 0x003d0868;
                    continue;
                }
                0x003d0868 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0870;
                    continue;
                }
                0x003d0870 => {
                    c.cycles += 2;

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                        pc = 0x003d08cc;
                        continue;
                    }
                    pc = 0x003d0878;
                    continue;
                }
                0x003d0878 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0880;
                    continue;
                }
                0x003d0880 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    if t {
                        if back_edge_unwinds(c, 0x003d07f0) {
                            return Ok(());
                        }
                        pc = 0x003d07f0;
                        continue;
                    }
                    pc = 0x003d0888;
                    continue;
                }
                0x003d0888 => {
                    c.cycles += 2;

                    if c.g[A1][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0890;
                    continue;
                }
                0x003d0890 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh(c.g[A1][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d089c;
                    continue;
                }
                0x003d089c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d08a8;
                    continue;
                }
                0x003d08a8 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 20);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 20);
                    pc = 0x003d08b8;
                    continue;
                }
                0x003d08b8 => {
                    c.cycles += 3;
                    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d08c4;
                    continue;
                }
                0x003d08c4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d08cc;
                    continue;
                }
                0x003d08cc => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
                    if t {
                        pc = 0x003d09f8;
                        continue;
                    }
                    pc = 0x003d08d4;
                    continue;
                }
                0x003d08d4 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 21);
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] & 0x3f;
                    if t {
                        if back_edge_unwinds(c, 0x003d08b8) {
                            return Ok(());
                        }
                        pc = 0x003d08b8;
                        continue;
                    }
                    pc = 0x003d08e4;
                    continue;
                }
                0x003d08e4 => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[A2][0] as u32) >> 23);
                    c.g[V1][0] = c.g[V1][0] & 0x7;
                    c.g[V0][0] = (c.g[V1][0] < 0x7u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x34));
                    if t {
                        pc = 0x003d09f8;
                        continue;
                    }
                    pc = 0x003d08f8;
                    continue;
                }
                0x003d08f8 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x490000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5ab0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x003d0914 => {
                    c.cycles += 2;
                    let t = c.g[S4][0] != c.g[A1][0];
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d091c;
                    continue;
                }
                0x003d091c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d0924;
                    continue;
                }
                0x003d0924 => {
                    c.cycles += 2;
                    let t = c.g[S4][0] == c.g[A1][0];
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d092c;
                    continue;
                }
                0x003d092c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d0934;
                    continue;
                }
                0x003d0934 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0940;
                    continue;
                }
                0x003d0940 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d0948;
                    continue;
                }
                0x003d0948 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d0954;
                    continue;
                }
                0x003d0954 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4000000;
                    if t {
                        pc = 0x003d09f0;
                        continue;
                    }
                    pc = 0x003d095c;
                    continue;
                }
                0x003d095c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    if t {
                        pc = 0x003d09e0;
                        continue;
                    }
                    pc = 0x003d0964;
                    continue;
                }
                0x003d0964 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x003d096c);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                    call_ptr(c, 0x002af480, 0x003d096c)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 23);
                    c.g[V1][0] = c.g[V1][0] & 0x7;
                    c.g[V0][0] = (c.g[V1][0] < 0x7u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        pc = 0x003d09f8;
                        continue;
                    }
                    pc = 0x003d0988;
                    continue;
                }
                0x003d0988 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5ad0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x003d09a0 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x003d09a8);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                    call_ptr(c, 0x002af4b0, 0x003d09a8)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 23);
                    c.g[V1][0] = c.g[V1][0] & 0x7;
                    c.g[V0][0] = (c.g[V1][0] < 0x7u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        pc = 0x003d09f8;
                        continue;
                    }
                    pc = 0x003d09c4;
                    continue;
                }
                0x003d09c4 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5af0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x003d09dc => {
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    pc = 0x003d09e0;
                    continue;
                }
                0x003d09e0 => {
                    c.cycles += 2;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d09fc;
                        continue;
                    }
                    pc = 0x003d09e8;
                    continue;
                }
                0x003d09e8 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = 0x4000000;
                    pc = 0x003d09f0;
                    continue;
                }
                0x003d09f0 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003d09f8;
                    continue;
                }
                0x003d09f8 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    pc = 0x003d09fc;
                    continue;
                }
                0x003d09fc => {
                    c.cycles += 4;
                    c.g[S0][0] = 0x4000000;
                    c.g[V0][0] = c.g[V0][0] & c.g[S0][0];

                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x003d0a30;
                        continue;
                    }
                    pc = 0x003d0a0c;
                    continue;
                }
                0x003d0a0c => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[A1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0xe));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[A1][0] & 0x3f;
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
                    c.g[RA][0] = sx(0x003d0a2c);
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    Callee {
                        addr: 0x003d0c20,
                        f: ee::s14::PATH_SkipRemainingActions,
                    }
                    .call(c, 0x003d0a2c)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    pc = 0x003d0a30;
                    continue;
                }
                0x003d0a30 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x400000;
                    c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx(((c.g[A0][0] as u32) as i32 >> 26) as u32);
                    if t {
                        pc = 0x003d0a60;
                        continue;
                    }
                    pc = 0x003d0a40;
                    continue;
                }
                0x003d0a40 => {
                    c.cycles += 8;
                    c.g[V1][0] = 0xfffffffffbff0000;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[V1][0] = c.g[A0][0] & c.g[V1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 26);
                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x003d0a60;
                    continue;
                }
                0x003d0a60 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = 0x300000;
                    c.g[V1][0] = 0x100000;
                    c.g[A0][0] = c.g[A1][0] & c.g[V0][0];
                    let t = c.g[A0][0] == c.g[V1][0];
                    c.g[V0][0] = 0x200000;
                    if t {
                        pc = 0x003d0a80;
                        continue;
                    }
                    pc = 0x003d0a78;
                    continue;
                }
                0x003d0a78 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003d0acc;
                        continue;
                    }
                    pc = 0x003d0a80;
                    continue;
                }
                0x003d0a80 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A1][0] & c.g[S0][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x130));
                    if t {
                        pc = 0x003d0aac;
                        continue;
                    }
                    pc = 0x003d0a8c;
                    continue;
                }
                0x003d0a8c => {
                    c.cycles += 8;
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 26);
                    c.g[A1][0] = c.g[A1][0] & 0x1;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0xe));
                    c.g[A2][0] = c.g[A2][0] & 0x3f;
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 4);
                    c.g[RA][0] = sx(0x003d0aac);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    Callee {
                        addr: 0x003d0b78,
                        f: ee::s14::PATH_SkipCondBlock,
                    }
                    .call(c, 0x003d0aac)?;
                    pc = 0x003d0aac;
                    continue;
                }
                0x003d0aac => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] | c.g[S0][0];
                    if t {
                        pc = 0x003d0ac8;
                        continue;
                    }
                    pc = 0x003d0abc;
                    continue;
                }
                0x003d0abc => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    pc = 0x003d0ac0;
                    continue;
                }
                0x003d0ac0 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0x4000000;
                    pc = 0x003d0ac4;
                    continue;
                }
                0x003d0ac4 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    pc = 0x003d0ac8;
                    continue;
                }
                0x003d0ac8 => {
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003d0acc;
                    continue;
                }
                0x003d0acc => {
                    c.cycles += 1;
                    c.g[S3][0] = c.g[S5][0];
                    pc = 0x003d0ad0;
                    continue;
                }
                0x003d0ad0 => {
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 24i64) as u64;
                    pc = 0x003d0ad4;
                    continue;
                }
                0x003d0ad4 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        if back_edge_unwinds(c, 0x003cfc88) {
                            return Ok(());
                        }
                        pc = 0x003cfc88;
                        continue;
                    }
                    pc = 0x003d0adc;
                    continue;
                }
                0x003d0adc => {
                    c.cycles += 15;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 26);
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x190));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x180));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x170));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x140));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003cfc30, 0x003d0b18);
                    return Ok(());
                }
            }
        }
    });
}

/// `PATH_PostEvent` (0x003d16f0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn path_post_event(c: &mut Cpu) {
    if c.pc != 0x003d16f0 {
        return ee::s14::f_003d16f0__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x003d16f0;
        loop {
            match pc {
                0x003d16f0 => {
                    c.cycles += 18;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.g[V0][0] = 0x450000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 16);
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = 0u64;
                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0xbd4));
                    c.g[S6][0] = sx(((c.g[A1][0] as u32) as i32 >> 16) as u32);
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    let t = c.g[V1][0] == 0u64;
                    c.sw(c.g[SP][0] as u32, c.g[A0][0]);
                    if t {
                        pc = 0x003d1758;
                        continue;
                    }
                    pc = 0x003d1738;
                    continue;
                }
                0x003d1738 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x450000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbd0));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0xffffffffffffffee;
                    if t {
                        pc = 0x003d1910;
                        continue;
                    }
                    pc = 0x003d1748;
                    continue;
                }
                0x003d1748 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d1750);
                    c.g[A0][0] = 0xffffffffffffffff;
                    Callee {
                        addr: 0x003d5450,
                        f: ee::s14::PATH_SelectMap,
                    }
                    .call(c, 0x003d1750)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[S6][0] & 0xf00;
                    if t {
                        pc = 0x003d1768;
                        continue;
                    }
                    pc = 0x003d1758;
                    continue;
                }
                0x003d1758 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffee;
                    if t {
                        pc = 0x003d1910;
                        continue;
                    }
                    pc = 0x003d1760;
                    continue;
                }
                0x003d1760 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = 0xfffffffffffffff3;
                    if t {
                        pc = 0x003d18fc;
                        continue;
                    }
                    pc = 0x003d1768;
                    continue;
                }
                0x003d1768 => {
                    c.cycles += 8;
                    c.g[V1][0] = 0x1000000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
                    c.g[S6][0] = c.g[S6][0] & 0xff;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[V0][0] as u32) & 31));
                    c.g[FP][0] = 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.g[V1][0] = 0x928;
                    pc = 0x003d1788;
                    continue;
                }
                0x003d1788 => {
                    c.cycles += 8;
                    c.g[V0][0] = 0x510000;
                    c.mult(0, 4, 30, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7610));
                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = c.g[V1][0] == 0u64;
                    c.g[V1][0] = 0x510000;
                    if t {
                        pc = 0x003d18ec;
                        continue;
                    }
                    pc = 0x003d17a8;
                    continue;
                }
                0x003d17a8 => {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7640));
                    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V1][0] & c.g[A0][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0xf000000;
                    if t {
                        pc = 0x003d18ec;
                        continue;
                    }
                    pc = 0x003d17c4;
                    continue;
                }
                0x003d17c4 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                    c.g[V0][0] = c.g[V0][0] & c.g[A0][0];

                    if c.g[V0][0] == 0u64 {
                        c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1));
                        pc = 0x003d18f0;
                        continue;
                    }
                    pc = 0x003d17d8;
                    continue;
                }
                0x003d17d8 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0xfffffffff0000000;
                    c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                    c.g[V0][0] = c.g[V0][0] & c.g[A0][0];

                    if c.g[V0][0] == 0u64 {
                        c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1));
                        pc = 0x003d18f0;
                        continue;
                    }
                    pc = 0x003d17ec;
                    continue;
                }
                0x003d17ec => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d17f4);
                    c.g[A0][0] = c.g[FP][0] & 0xff;
                    Callee {
                        addr: 0x003d5450,
                        f: ee::s14::PATH_SelectMap,
                    }
                    .call(c, 0x003d17f4)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x450000;
                    if t {
                        pc = 0x003d18ec;
                        continue;
                    }
                    pc = 0x003d17fc;
                    continue;
                }
                0x003d17fc => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0xbe0));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0xf));
                    c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x20000000;
                    if t {
                        if back_edge_unwinds(c, 0x003d1760) {
                            return Ok(());
                        }
                        pc = 0x003d1760;
                        continue;
                    }
                    pc = 0x003d1814;
                    continue;
                }
                0x003d1814 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = sx((c.g[S6][0] as u32) << 1);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = c.lh(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[S2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.g[V1][0] & c.g[A1][0];

                    if c.g[V1][0] != 0u64 {
                        c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1));
                        pc = 0x003d18f0;
                        continue;
                    }
                    pc = 0x003d183c;
                    continue;
                }
                0x003d183c => {
                    c.cycles += 13;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003d1848);
                    c.g[S7][0] = 0xfffffffffffffff8;
                    Callee {
                        addr: 0x003d2068,
                        f: ee::s14::PATHI_copyevent,
                    }
                    .call(c, 0x003d1848)?;
                    c.g[S5][0] = 0u64;
                    c.g[S3][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V1][0] = 0xffffffffff000000;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[S4][0] = 0u64;
                    c.g[S3][0] = c.g[S3][0] & c.g[V1][0];
                    c.g[V1][0] = 0x510000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7610));
                    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x91c));
                    pc = 0x003d1870;
                    continue;
                }
                0x003d1870 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x8e8));

                    if c.g[V0][0] == 0u64 {
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                        pc = 0x003d18d8;
                        continue;
                    }
                    pc = 0x003d187c;
                    continue;
                }
                0x003d187c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x8ec));
                    c.g[V0][0] = c.g[V0][0] & c.g[S3][0];

                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                        pc = 0x003d18d8;
                        continue;
                    }
                    pc = 0x003d188c;
                    continue;
                }
                0x003d188c => {
                    c.cycles += 4;
                    c.g[S0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 16i64) as u64;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        pc = 0x003d18a4;
                        continue;
                    }
                    pc = 0x003d189c;
                    continue;
                }
                0x003d189c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = 0xfffffffffffffff3;
                    if t {
                        pc = 0x003d18ec;
                        continue;
                    }
                    pc = 0x003d18a4;
                    continue;
                }
                0x003d18a4 => {
                    c.cycles += 13;
                    c.g[RA][0] = sx(0x003d18ac);
                    c.g[S7][0] = 0u64;
                    call_ptr(c, 0x002af370, 0x003d18ac)?;
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[A0][0] = 0x510000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x76ec));
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[V1][0] as u32, c.g[S2][0]);
                    c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                    pc = 0x003d18d8;
                    continue;
                }
                0x003d18d8 => {
                    c.cycles += 4;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x928));
                    c.g[V0][0] = ((c.g[S5][0] as i64) < 4i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x928));
                    if t {
                        if back_edge_unwinds(c, 0x003d1870) {
                            return Ok(());
                        }
                        pc = 0x003d1870;
                        continue;
                    }
                    pc = 0x003d18e8;
                    continue;
                }
                0x003d18e8 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.g[S7][0];
                    pc = 0x003d18ec;
                    continue;
                }
                0x003d18ec => {
                    c.cycles += 1;
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1));
                    pc = 0x003d18f0;
                    continue;
                }
                0x003d18f0 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[FP][0] < 0x4u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = 0x928;
                    if t {
                        if back_edge_unwinds(c, 0x003d1788) {
                            return Ok(());
                        }
                        pc = 0x003d1788;
                        continue;
                    }
                    pc = 0x003d18fc;
                    continue;
                }
                0x003d18fc => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x450000;
                    c.g[V1][0] = 0x450000;
                    c.sw((c.g[V0][0] as u32).wrapping_sub(0xbd0), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0xbe0), 0u64);
                    c.g[V0][0] = c.g[S1][0];
                    pc = 0x003d1910;
                    continue;
                }
                0x003d1910 => {
                    c.cycles += 12;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d16f0, 0x003d1940);
                    return Ok(());
                }
            }
        }
    });
}

/// `PATH_AddTrack` (0x003d25f0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn path_add_track(c: &mut Cpu) {
    if c.pc != 0x003d25f0 {
        return ee::s14::f_003d25f0__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x003d25f0;
        loop {
            match pc {
                0x003d25f0 => {
                    c.cycles += 24;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                    c.g[V0][0] = 0x450000;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[S3][0] = c.g[T2][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S0][0] = c.g[S3][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S2][0] = 0u64;
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S5][0] = c.g[A1][0];
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S6][0] = c.g[T0][0];
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S7][0] = c.g[A2][0];
                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0xbd4));
                    c.g[FP][0] = c.g[T3][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.sw(c.g[SP][0] as u32, c.g[A3][0]);
                    let t = c.g[V1][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T1][0]);
                    if t {
                        pc = 0x003d2670;
                        continue;
                    }
                    pc = 0x003d2650;
                    continue;
                }
                0x003d2650 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x450000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbd0));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0xffffffffffffffee;
                    if t {
                        pc = 0x003d2a38;
                        continue;
                    }
                    pc = 0x003d2660;
                    continue;
                }
                0x003d2660 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d2668);
                    c.g[A0][0] = 0xffffffffffffffff;
                    Callee {
                        addr: 0x003d5450,
                        f: ee::s14::PATH_SelectMap,
                    }
                    .call(c, 0x003d2668)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.g[S1][0];
                    if t {
                        pc = 0x003d2678;
                        continue;
                    }
                    pc = 0x003d2670;
                    continue;
                }
                0x003d2670 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffee;
                    if t {
                        pc = 0x003d2a38;
                        continue;
                    }
                    pc = 0x003d2678;
                    continue;
                }
                0x003d2678 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[V1][0] << 32;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    if t {
                        pc = 0x003d26a0;
                        continue;
                    }
                    pc = 0x003d2688;
                    continue;
                }
                0x003d2688 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S2][0] < 0x18u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = ((c.g[V1][0] as i64) >> (c.g[S2][0] & 63)) as u64;
                    if t {
                        pc = 0x003d26ac;
                        continue;
                    }
                    pc = 0x003d2694;
                    continue;
                }
                0x003d2694 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] << 32;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    pc = 0x003d26a0;
                    continue;
                }
                0x003d26a0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & 0x1;

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        if back_edge_unwinds(c, 0x003d2688) {
                            return Ok(());
                        }
                        pc = 0x003d2688;
                        continue;
                    }
                    pc = 0x003d26ac;
                    continue;
                }
                0x003d26ac => {
                    c.cycles += 12;
                    c.g[V0][0] = 0xffffffffff000000;
                    c.g[S4][0] = c.g[S1][0] & c.g[V0][0];
                    c.g[RA][0] = sx(0x003d26bc);
                    c.g[A0][0] = c.g[S1][0] & c.g[S4][0];
                    Callee {
                        addr: 0x003d4bb0,
                        f: ee::s14::PATH_FindMap,
                    }
                    .call(c, 0x003d26bc)?;
                    c.g[V0][0] = 0x450000;
                    c.g[A0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0xbe0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = 0xfffffffffffffff3;
                    if t {
                        pc = 0x003d2a24;
                        continue;
                    }
                    pc = 0x003d26dc;
                    continue;
                }
                0x003d26dc => {
                    c.cycles += 31;
                    c.g[S1][0] = c.g[S3][0];
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003d26f0);
                    c.g[A2][0] = 0x110;
                    call_ptr(c, 0x00416210, 0x003d26f0)?;
                    c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x110));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xfc), c.g[S3][0]);
                    c.g[A2][0] = sx((c.g[S6][0] as u32) << 4);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                    c.g[T0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0xf));
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[A3][0];
                    }
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xfc));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                    c.g[S4][0] = c.g[S4][0] << 32;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[A1][0] = 0u64;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S4][0]);
                    c.g[S3][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                    c.g[RA][0] = sx(0x003d2744);
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    call_ptr(c, 0x00416210, 0x003d2744)?;
                    c.g[T3][0] = ((c.g[S7][0] as i64) < 50i64) as u64;
                    c.g[T2][0] = c.g[S7][0] & 0xffff;
                    c.g[T1][0] = c.g[S6][0] & 0xff;
                    let t = (c.g[S6][0] as i64) <= 0;
                    c.g[A2][0] = 0u64;
                    if t {
                        pc = 0x003d277c;
                        continue;
                    }
                    pc = 0x003d2758;
                    continue;
                }
                0x003d2758 => {
                    c.cycles += 2;
                    c.g[A1][0] = 0xffffffffffffffff;
                    pc = 0x003d2760;
                    continue;
                }
                0x003d2760 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xfc));
                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 4);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = ((c.g[A2][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let t = c.g[A0][0] != 0u64;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[A1][0]);
                    if t {
                        if back_edge_unwinds(c, 0x003d2760) {
                            return Ok(());
                        }
                        pc = 0x003d2760;
                        continue;
                    }
                    pc = 0x003d277c;
                    continue;
                }
                0x003d277c => {
                    c.cycles += 45;
                    c.sb(c.g[S1][0] as u32, c.g[S2][0]);
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[V0][0] = c.g[V0][0] | 0xff;
                    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.g[T1][0] & 0x1f;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 14);
                    c.g[A1][0] = c.g[V1][0] | 0xff;
                    c.g[V1][0] = 0x450000;
                    c.g[A1][0] = c.g[A1][0] & c.g[V0][0];
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbe0));
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[V1][0] = 0xfffffffffff80000;
                    c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                    c.g[A3][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0x2c));
                    c.g[V1][0] = c.g[V1][0] | 0x3fff;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x2), 0u64);
                    c.g[T0][0] = c.g[T0][0] & c.g[V1][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x1), c.g[A3][0]);
                    c.g[V0][0] = 0x450000;
                    c.g[T0][0] = c.g[T0][0] | c.g[A0][0];
                    c.g[V1][0] = 0x7f;
                    c.g[A3][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0xbd2));
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x34), c.g[A1][0]);
                    c.g[A2][0] = 0x2;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x4), c.g[A3][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x24), c.g[T2][0]);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x34), c.g[V1][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x3c), c.g[A1][0]);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x2e), c.g[V0][0]);
                    c.sh((c.g[S1][0] as u32).wrapping_add(0x2c), c.g[A0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x100), c.g[A2][0]);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0xe), c.g[V0][0]);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0xd), c.g[V0][0]);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x16), c.g[V0][0]);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x15), c.g[V0][0]);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x1d), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[T0][0]);
                    let t = c.g[T3][0] == 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x38), c.g[A1][0]);
                    if t {
                        pc = 0x003d283c;
                        continue;
                    }
                    pc = 0x003d2830;
                    continue;
                }
                0x003d2830 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = true;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x2f), c.g[V0][0]);
                    if t {
                        pc = 0x003d2844;
                        continue;
                    }
                    pc = 0x003d283c;
                    continue;
                }
                0x003d283c => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[T0][0] | 0x1f0;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                    pc = 0x003d2844;
                    continue;
                }
                0x003d2844 => {
                    c.cycles += 11;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5c));
                    c.g[RA][0] = sx(0x003d2850);
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x004166f4, 0x003d2850)?;
                    c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0xdc));
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x003d2860);
                    c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0xdd));
                    call_ptr(c, 0x00416810, 0x003d2860)?;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = (c.g[S0][0] < c.g[S5][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0xfc));
                    if t {
                        pc = 0x003d28d0;
                        continue;
                    }
                    pc = 0x003d2870;
                    continue;
                }
                0x003d2870 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[V0][0] = 0x5c;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x003d28d4;
                        continue;
                    }
                    pc = 0x003d2880;
                    continue;
                }
                0x003d2880 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2f;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x003d28d4;
                        continue;
                    }
                    pc = 0x003d288c;
                    continue;
                }
                0x003d288c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3a;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x5c;
                    if t {
                        pc = 0x003d28d0;
                        continue;
                    }
                    pc = 0x003d2898;
                    continue;
                }
                0x003d2898 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x2f;
                    c.g[V1][0] = 0x3a;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    pc = 0x003d28a8;
                    continue;
                }
                0x003d28a8 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S0][0] < c.g[S5][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x003d28d4;
                        continue;
                    }
                    pc = 0x003d28b4;
                    continue;
                }
                0x003d28b4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);

                    if c.g[V0][0] == c.g[A1][0] {
                        c.g[V0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                        pc = 0x003d28d4;
                        continue;
                    }
                    pc = 0x003d28c0;
                    continue;
                }
                0x003d28c0 => {
                    c.cycles += 2;

                    if c.g[V0][0] == c.g[A0][0] {
                        c.g[V0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                        pc = 0x003d28d4;
                        continue;
                    }
                    pc = 0x003d28c8;
                    continue;
                }
                0x003d28c8 => {
                    c.cycles += 2;

                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        if back_edge_unwinds(c, 0x003d28a8) {
                            return Ok(());
                        }
                        pc = 0x003d28a8;
                        continue;
                    }
                    pc = 0x003d28d0;
                    continue;
                }
                0x003d28d0 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    pc = 0x003d28d4;
                    continue;
                }
                0x003d28d4 => {
                    c.cycles += 5;
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A2][0] = 0u64;
                    c.g[V1][0] = c.g[V0][0] >> 32;
                    let t = true;
                    c.g[V0][0] = ((c.g[V1][0] as i64) >> 28) as u64;
                    if t {
                        pc = 0x003d28f8;
                        continue;
                    }
                    pc = 0x003d28e8;
                    continue;
                }
                0x003d28e8 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 4i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1c));
                    if t {
                        pc = 0x003d290c;
                        continue;
                    }
                    pc = 0x003d28f4;
                    continue;
                }
                0x003d28f4 => {
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[V1][0] as i64) >> (c.g[V0][0] & 63)) as u64;
                    pc = 0x003d28f8;
                    continue;
                }
                0x003d28f8 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[V0][0] = c.g[V0][0] << 32;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        if back_edge_unwinds(c, 0x003d28e8) {
                            return Ok(());
                        }
                        pc = 0x003d28e8;
                        continue;
                    }
                    pc = 0x003d290c;
                    continue;
                }
                0x003d290c => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x490000;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5b90));
                    c.g[RA][0] = sx(0x003d2920);
                    c.g[A0][0] = c.g[S7][0];
                    call_ptr(c, 0x00417828, 0x003d2920)?;
                    c.g[S0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xe8));
                    c.g[A2][0] = 0x20;
                    let t = true;
                    c.g[V1][0] = 0x61;
                    if t {
                        pc = 0x003d2940;
                        continue;
                    }
                    pc = 0x003d2938;
                    continue;
                }
                0x003d2938 => {
                    c.cycles += 2;
                    c.sb(c.g[S0][0] as u32, c.g[A2][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    pc = 0x003d2940;
                    continue;
                }
                0x003d2940 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S0][0] < c.g[A1][0]) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = 0x450000;
                        pc = 0x003d299c;
                        continue;
                    }
                    pc = 0x003d294c;
                    continue;
                }
                0x003d294c => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x003d2964;
                        continue;
                    }
                    pc = 0x003d2954;
                    continue;
                }
                0x003d2954 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[V0][0] | 0x20;
                    if t {
                        pc = 0x003d2964;
                        continue;
                    }
                    pc = 0x003d2960;
                    continue;
                }
                0x003d2960 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x003d2964;
                    continue;
                }
                0x003d2964 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x61));
                    c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        if back_edge_unwinds(c, 0x003d2940) {
                            return Ok(());
                        }
                        pc = 0x003d2940;
                        continue;
                    }
                    pc = 0x003d2978;
                    continue;
                }
                0x003d2978 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        if back_edge_unwinds(c, 0x003d2940) {
                            return Ok(());
                        }
                        pc = 0x003d2940;
                        continue;
                    }
                    pc = 0x003d2988;
                    continue;
                }
                0x003d2988 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S0][0] < c.g[A0][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = 0u64;
                    if t {
                        if back_edge_unwinds(c, 0x003d2938) {
                            return Ok(());
                        }
                        pc = 0x003d2938;
                        continue;
                    }
                    pc = 0x003d2994;
                    continue;
                }
                0x003d2994 => {
                    c.cycles += 2;
                    c.sb(c.g[S0][0] as u32, 0u64);
                    c.g[V1][0] = 0x450000;
                    pc = 0x003d299c;
                    continue;
                }
                0x003d299c => {
                    c.cycles += 9;
                    c.g[A0][0] = 0xf000000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbe0));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[RA][0] = sx(0x003d29b4);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[S1][0]);
                    Callee {
                        addr: 0x003d5330,
                        f: ee::s14::PATH_CountTracks,
                    }
                    .call(c, 0x003d29b4)?;
                    c.g[V1][0] = 0x1;

                    if c.g[V0][0] != c.g[V1][0] {
                        c.sb((c.g[S1][0] as u32).wrapping_add(0xe7), 0u64);
                        pc = 0x003d29d8;
                        continue;
                    }
                    pc = 0x003d29c0;
                    continue;
                }
                0x003d29c0 => {
                    c.cycles += 6;
                    c.g[A0][0] = 0x3d0000;
                    c.g[A1][0] = 0x3d0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x54c8));
                    c.g[RA][0] = sx(0x003d29d4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x54e8));
                    call_ptr(c, 0x002ae048, 0x003d29d4)?;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0xe7), 0u64);
                    pc = 0x003d29d8;
                    continue;
                }
                0x003d29d8 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A2][0] = c.g[S6][0];
                    c.g[T0][0] = c.g[S3][0];
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[T1][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x003d29f4);
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x002ae4c0, 0x003d29f4)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                    if t {
                        pc = 0x003d2a18;
                        continue;
                    }
                    pc = 0x003d29fc;
                    continue;
                }
                0x003d29fc => {
                    c.cycles += 7;
                    c.g[V1][0] = 0x450000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbe0));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x58), 0u64);
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                    if t {
                        pc = 0x003d2a24;
                        continue;
                    }
                    pc = 0x003d2a18;
                    continue;
                }
                0x003d2a18 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.g[V1][0] = 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    pc = 0x003d2a24;
                    continue;
                }
                0x003d2a24 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x450000;
                    c.sw((c.g[V0][0] as u32).wrapping_sub(0xbd0), 0u64);
                    c.g[V0][0] = c.g[V1][0];
                    c.g[V1][0] = 0x450000;
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0xbe0), 0u64);
                    pc = 0x003d2a38;
                    continue;
                }
                0x003d2a38 => {
                    c.cycles += 12;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d25f0, 0x003d2a68);
                    return Ok(());
                }
            }
        }
    });
}

/// `PATH_StopTracks` (0x003d2a68): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn path_stop_tracks(c: &mut Cpu) {
    if c.pc != 0x003d2a68 {
        return ee::s14::f_003d2a68__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x003d2a68;
        loop {
            match pc {
                0x003d2a68 => {
                    c.cycles += 17;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.g[V0][0] = 0x450000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S3][0] = 0u64;
                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0xbd4));
                    c.g[FP][0] = c.g[V0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        pc = 0x003d2acc;
                        continue;
                    }
                    pc = 0x003d2aac;
                    continue;
                }
                0x003d2aac => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x450000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbd0));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x003d2bec;
                        continue;
                    }
                    pc = 0x003d2abc;
                    continue;
                }
                0x003d2abc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d2ac4);
                    c.g[A0][0] = 0xffffffffffffffff;
                    Callee {
                        addr: 0x003d5450,
                        f: ee::s14::PATH_SelectMap,
                    }
                    .call(c, 0x003d2ac4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x510000;
                    if t {
                        pc = 0x003d2ae4;
                        continue;
                    }
                    pc = 0x003d2acc;
                    continue;
                }
                0x003d2acc => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x003d2bec;
                        continue;
                    }
                    pc = 0x003d2ad4;
                    continue;
                }
                0x003d2ad4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d2adc);
                    c.sw(c.g[A1][0] as u32, 0u64);
                    call_ptr(c, 0x002ae690, 0x003d2adc)?;
                    let t = true;
                    c.g[A0][0] = 0x1000000;
                    if t {
                        pc = 0x003d2b8c;
                        continue;
                    }
                    pc = 0x003d2ae4;
                    continue;
                }
                0x003d2ae4 => {
                    c.cycles += 7;
                    c.g[S4][0] = 0x450000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7610));
                    c.g[S7][0] = c.g[S4][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = 0u64;
                    c.g[S6][0] = 0xfffffffff0000000;
                    c.g[S5][0] = 0xf000000;
                    pc = 0x003d2b00;
                    continue;
                }
                0x003d2b00 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));

                    if c.g[V0][0] == 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x003d2b7c;
                        continue;
                    }
                    pc = 0x003d2b0c;
                    continue;
                }
                0x003d2b0c => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.g[V1][0] & c.g[S6][0];
                    c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[V1][0] & c.g[S5][0];
                    if t {
                        pc = 0x003d2b78;
                        continue;
                    }
                    pc = 0x003d2b20;
                    continue;
                }
                0x003d2b20 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & c.g[S0][0];

                    if c.g[V0][0] == 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x003d2b7c;
                        continue;
                    }
                    pc = 0x003d2b2c;
                    continue;
                }
                0x003d2b2c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d2b34);
                    c.g[A0][0] = c.g[S2][0] & 0xff;
                    Callee {
                        addr: 0x003d5450,
                        f: ee::s14::PATH_SelectMap,
                    }
                    .call(c, 0x003d2b34)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    if t {
                        pc = 0x003d2b78;
                        continue;
                    }
                    pc = 0x003d2b3c;
                    continue;
                }
                0x003d2b3c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0u64;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[A1][0] = c.g[V1][0];
                    pc = 0x003d2b48;
                    continue;
                }
                0x003d2b48 => {
                    c.cycles += 3;
                    c.g[S3][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[S3][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) >> ((c.g[A0][0] as u32) & 31));
                    if t {
                        pc = 0x003d2b64;
                        continue;
                    }
                    pc = 0x003d2b54;
                    continue;
                }
                0x003d2b54 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    c.g[V0][0] = c.g[V0][0] & 0x1;

                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x54));
                        if back_edge_unwinds(c, 0x003d2ad4) {
                            return Ok(());
                        }
                        pc = 0x003d2ad4;
                        continue;
                    }
                    pc = 0x003d2b64;
                    continue;
                }
                0x003d2b64 => {
                    c.cycles += 5;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 24i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x003d2b48) {
                            return Ok(());
                        }
                        pc = 0x003d2b48;
                        continue;
                    }
                    pc = 0x003d2b78;
                    continue;
                }
                0x003d2b78 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x003d2b7c;
                    continue;
                }
                0x003d2b7c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 4i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x928));
                    if t {
                        if back_edge_unwinds(c, 0x003d2b00) {
                            return Ok(());
                        }
                        pc = 0x003d2b00;
                        continue;
                    }
                    pc = 0x003d2b88;
                    continue;
                }
                0x003d2b88 => {
                    c.cycles += 1;
                    c.g[A0][0] = 0x1000000;
                    pc = 0x003d2b8c;
                    continue;
                }
                0x003d2b8c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d2b94);
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[S2][0] as u32) & 31));
                    Callee {
                        addr: 0x003d5330,
                        f: ee::s14::PATH_CountTracks,
                    }
                    .call(c, 0x003d2b94)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0xbe0));
                    if t {
                        pc = 0x003d2ba8;
                        continue;
                    }
                    pc = 0x003d2b9c;
                    continue;
                }
                0x003d2b9c => {
                    c.cycles += 3;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xb8), 0u64);
                    c.g[RA][0] = sx(0x003d2ba8);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x34), 0u64);
                    Callee {
                        addr: 0x003d4c40,
                        f: ee::s14::PATH_SortMaps,
                    }
                    .call(c, 0x003d2ba8)?;
                    pc = 0x003d2ba8;
                    continue;
                }
                0x003d2ba8 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x003d2bb0);
                    c.g[A0][0] = 0xf000000;
                    Callee {
                        addr: 0x003d5330,
                        f: ee::s14::PATH_CountTracks,
                    }
                    .call(c, 0x003d2bb0)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = c.lbu((c.g[FP][0] as u32).wrapping_sub(0xbd4));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x450000;
                    if t {
                        pc = 0x003d2be0;
                        continue;
                    }
                    pc = 0x003d2bc0;
                    continue;
                }
                0x003d2bc0 => {
                    c.cycles += 2;

                    if c.g[V1][0] != 0u64 {
                        c.sw((c.g[S4][0] as u32).wrapping_sub(0xbe0), 0u64);
                        pc = 0x003d2be4;
                        continue;
                    }
                    pc = 0x003d2bc8;
                    continue;
                }
                0x003d2bc8 => {
                    c.cycles += 2;

                    if c.g[S3][0] == 0u64 {
                        c.sw((c.g[S4][0] as u32).wrapping_sub(0xbe0), 0u64);
                        pc = 0x003d2be4;
                        continue;
                    }
                    pc = 0x003d2bd0;
                    continue;
                }
                0x003d2bd0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d2bd8);
                    call_ptr(c, 0x002ae100, 0x003d2bd8)?;
                    c.sb((c.g[FP][0] as u32).wrapping_sub(0xbd4), 0u64);
                    c.g[V0][0] = 0x450000;
                    pc = 0x003d2be0;
                    continue;
                }
                0x003d2be0 => {
                    c.cycles += 1;
                    c.sw((c.g[S4][0] as u32).wrapping_sub(0xbe0), 0u64);
                    pc = 0x003d2be4;
                    continue;
                }
                0x003d2be4 => {
                    c.cycles += 2;
                    c.sw((c.g[V0][0] as u32).wrapping_sub(0xbd0), 0u64);
                    c.g[V0][0] = c.g[S3][0];
                    pc = 0x003d2bec;
                    continue;
                }
                0x003d2bec => {
                    c.cycles += 12;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d2a68, 0x003d2c1c);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, fresh, run_generated, Rng, DATA};
    use std::collections::BTreeSet;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the oc812 fakes: they act only on machines that hold the magic here.
    pub(crate) const OC812_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC812_MAGIC: u32 = 0x0c81_2a5a;
    /// Forced results of the fakes: one word per callee at `OC812_FORCE + (addr & 0xffffc)`.
    pub(crate) const OC812_FORCE: u32 = DATA + 0x30_0000;
    /// Bytes the `sprintf` fake writes at its destination (the track's name buffer).
    pub(crate) const OC812_TPL: u32 = DATA + 0x20_1000;
    /// The four path maps (players), 0x928 bytes each.
    pub(crate) const OC812_MAPS: u32 = 0x0051_7610;
    pub(crate) const OC812_MAP_SIZE: u32 = 0x928;
    /// Path system ready byte, selected map pointer, selection/busy word, time, debug flags.
    pub(crate) const OC812_READY: u32 = 0x0044_f42c;
    pub(crate) const OC812_CUR: u32 = 0x0044_f420;
    pub(crate) const OC812_BUSY: u32 = 0x0044_f430;
    pub(crate) const OC812_TIME: u32 = 0x0044_f438;
    pub(crate) const OC812_FLAGS: u32 = 0x0044_f43c;
    /// The 24 shared track structures (0x200 bytes each) the map slots point at.
    pub(crate) const OC812_TRACKS: u32 = DATA + 0x1_0000;
    /// Path data header (events, node table), event index tables, a node record.
    pub(crate) const OC812_HDR: u32 = DATA + 0x2000;
    pub(crate) const OC812_IDXA: u32 = DATA + 0x4000;
    pub(crate) const OC812_IDXB: u32 = DATA + 0x4800;
    pub(crate) const OC812_NODE: u32 = DATA + 0x5000;

    pub(crate) fn oc812_mail(addr: u32) -> u32 {
        OC812_FORCE + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` (sign-extended) in `v0`.
    pub(crate) fn oc812_force(c: &mut Cpu, addr: u32, v: i32) {
        c.bus.write32(oc812_mail(addr), (v as u32) ^ 0x8000_0000);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the forced result of
    /// its mailbox (0 = the fake's pointer) unless it unwound. The `sprintf` fake first writes
    /// the template bytes into its destination.
    pub(crate) fn oc812_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0] as u32;
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc812_mail(addr));
        if addr == 0x0041_7828 {
            for i in 0..0x20u32 {
                let b = c.bus.read8(OC812_TPL + i);
                c.bus.write8(a0.wrapping_add(i), b);
            }
        }
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail ^ 0x8000_0000) as i32 as i64 as u64;
        }
    }

    macro_rules! oc812_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC812_CTRL) != OC812_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc812_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc812_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc812_fakes!(
        (
            oc812_fake_2af370,
            OC812_PREV_2AF370,
            ee::s08::OVERRIDE_002AF370,
            ee::s08::f_002af370__generated,
            0x002a_f370
        ),
        (
            oc812_fake_2ae048,
            OC812_PREV_2AE048,
            ee::s08::OVERRIDE_002AE048,
            ee::s08::f_002ae048__generated,
            0x002a_e048
        ),
        (
            oc812_fake_2ae100,
            OC812_PREV_2AE100,
            ee::s08::OVERRIDE_002AE100,
            ee::s08::f_002ae100__generated,
            0x002a_e100
        ),
        (
            oc812_fake_2ae4c0,
            OC812_PREV_2AE4C0,
            ee::s08::OVERRIDE_002AE4C0,
            ee::s08::f_002ae4c0__generated,
            0x002a_e4c0
        ),
        (
            oc812_fake_2ae690,
            OC812_PREV_2AE690,
            ee::s08::OVERRIDE_002AE690,
            ee::s08::f_002ae690__generated,
            0x002a_e690
        ),
        (
            oc812_fake_2aec28,
            OC812_PREV_2AEC28,
            ee::s08::OVERRIDE_002AEC28,
            ee::s08::f_002aec28__generated,
            0x002a_ec28
        ),
        (
            oc812_fake_2af428,
            OC812_PREV_2AF428,
            ee::s08::OVERRIDE_002AF428,
            ee::s08::f_002af428__generated,
            0x002a_f428
        ),
        (
            oc812_fake_2af4b0,
            OC812_PREV_2AF4B0,
            ee::s08::OVERRIDE_002AF4B0,
            ee::s08::f_002af4b0__generated,
            0x002a_f4b0
        ),
        (
            oc812_fake_2af580,
            OC812_PREV_2AF580,
            ee::s08::OVERRIDE_002AF580,
            ee::s08::f_002af580__generated,
            0x002a_f580
        ),
        (
            oc812_fake_2af7a0,
            OC812_PREV_2AF7A0,
            ee::s08::OVERRIDE_002AF7A0,
            ee::s08::f_002af7a0__generated,
            0x002a_f7a0
        ),
        (
            oc812_fake_3b58a0,
            OC812_PREV_3B58A0,
            ee::s13::OVERRIDE_003B58A0,
            ee::s13::f_003b58a0__generated,
            0x003b_58a0
        ),
        (
            oc812_fake_3d3900,
            OC812_PREV_3D3900,
            ee::s14::OVERRIDE_003D3900,
            ee::s14::f_003d3900__generated,
            0x003d_3900
        ),
        (
            oc812_fake_3d41a8,
            OC812_PREV_3D41A8,
            ee::s14::OVERRIDE_003D41A8,
            ee::s14::f_003d41a8__generated,
            0x003d_41a8
        ),
        (
            oc812_fake_3d62c8,
            OC812_PREV_3D62C8,
            ee::s14::OVERRIDE_003D62C8,
            ee::s14::f_003d62c8__generated,
            0x003d_62c8
        ),
        (
            oc812_fake_417828,
            OC812_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
    );

    /// Distinct total cycle counts of the generated body over the seeds (one per path taken).
    fn oc812_paths(generated: fn(&mut Cpu), seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) -> usize {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    fn oc812_pick(r: &mut Rng, xs: &[u32]) -> u32 {
        xs[(r.next_u32() as usize) % xs.len()]
    }

    const OC812_TOPS: [u32; 8] = [0xff, 0x11, 0x33, 0x0f, 0xf0, 0x1f, 0x10, 0x01];

    /// Random maps and tracks: pending lists empty (count 0..11), the 24 track slots of each map
    /// present one in `slot_one_in` (0 = none) and pointing at the shared track structures.
    fn oc812_world(c: &mut Cpu, r: &mut Rng, slot_one_in: u32) {
        for j in 0..24u32 {
            fill(c, r, OC812_TRACKS + 0x200 * j, 0x80);
        }
        for m in 0..4u32 {
            let map = OC812_MAPS + OC812_MAP_SIZE * m;
            fill(c, r, map, OC812_MAP_SIZE / 4);
            for k in 0..16u32 {
                c.bus.write32(map + 0xdc + 4 * k, 0);
            }
            c.bus.write32(map + 0x91c, r.next_u32() % 12);
            for j in 0..24u32 {
                let present = slot_one_in != 0 && r.next_u32() % slot_one_in == 0;
                let v = if present { OC812_TRACKS + 0x200 * j } else { 0 };
                c.bus.write32(map + 0x58 + 4 * j, v);
            }
        }
    }

    fn post_event_setup(c: &mut Cpu, seed: u32) {
        oc812_install_fakes();
        let mut r = machine(c, seed, 0x16f0, 4);
        c.pc = 0x003d_16f0;
        c.bus.write32(OC812_CTRL, OC812_MAGIC);
        oc812_world(c, &mut r, 0);
        let (hdr, idx) = (OC812_HDR, OC812_IDXA);
        fill(c, &mut r, hdr, 0x440);
        c.bus.write8(hdr + 0xf, 1 + (r.next_u32() % 6) as u8);
        for k in 0..8u32 {
            let ev = hdr + 0x40 + 0x40 * k;
            c.bus.write16(idx + 2 * k, ((0x40 + 0x40 * k) / 4) as u16);
            c.bus.write16(ev + 0xe, (r.next_u32() % 4) as u16);
            let off = if r.next_u32() % 5 == 0 {
                0x2000_0000
            } else {
                0
            };
            let flags = (c.bus.read32(ev + 0xc) & !0x2000_0000) | off;
            c.bus.write32(ev + 0xc, flags);
        }
        for m in 0..4u32 {
            let map = OC812_MAPS + OC812_MAP_SIZE * m;
            let top = oc812_pick(&mut r, &OC812_TOPS) << 24;
            c.bus.write32(map + 0x30, top);
            let h = if r.next_u32() % 6 == 0 { 0 } else { hdr };
            c.bus.write32(map + 0x34, h);
            c.bus.write32(map + 0x40, idx);
            if r.next_u32() % 8 == 0 {
                // A pending event near the end of the buffer: no room for the copy.
                c.bus.write32(map + 0xdc, map + 0x900);
            }
        }
        c.bus
            .write8(OC812_READY, if seed % 10 == 0 { 0 } else { 1 });
        c.bus
            .write32(OC812_BUSY, if seed % 10 == 3 { 1 } else { 0 });
        let flags = oc812_pick(&mut r, &[0, 0x20, 0x100, 0x120]);
        c.bus.write32(OC812_FLAGS, flags);
        let rnd = r.next_u32();
        let mask = oc812_pick(
            &mut r,
            &[
                0xffff_ffff,
                0x1100_0000,
                0x3300_0000,
                0xff00_0000,
                0x0100_0000,
                rnd,
            ],
        );
        c.g[A0][0] = sx(mask);
        let mut prog = ((r.next_u32() % 4) << 8) | (r.next_u32() % 8);
        if seed % 7 == 0 {
            prog |= r.next_u32() & 0xf000;
        }
        c.g[A1][0] = sx(prog);
    }

    /// `PATH_PostEvent`: queues an event of the path data on every player of its group (and the
    /// failure answers: system not ready or busy, bad event number, full list, disabled event,
    /// no room in the buffer), with unwinds in the time stamp calls. The `SelectMap(-1) == 0`
    /// exit cannot happen with the real `PATH_SelectMap` (it returns -1 or the id), so it is not
    /// reached.
    #[test]
    fn path_post_event_matches() {
        check(
            ee::s14::f_003d16f0__generated,
            path_post_event,
            300,
            &post_event_setup,
        );
        assert!(oc812_paths(ee::s14::f_003d16f0__generated, 300, &post_event_setup) >= 5);
    }

    fn stop_tracks_setup(c: &mut Cpu, seed: u32) {
        oc812_install_fakes();
        let mut r = machine(c, seed, 0x2a68, 4);
        c.pc = 0x003d_2a68;
        c.bus.write32(OC812_CTRL, OC812_MAGIC);
        oc812_world(c, &mut r, 3);
        for m in 0..4u32 {
            let map = OC812_MAPS + OC812_MAP_SIZE * m;
            let top = oc812_pick(&mut r, &OC812_TOPS) << 24;
            c.bus.write32(map + 0x30, top);
            let h = if r.next_u32() % 5 == 0 { 0 } else { OC812_HDR };
            c.bus.write32(map + 0x34, h);
        }
        for j in 0..24u32 {
            let handle = r.next_u32() % 6;
            c.bus.write32(OC812_TRACKS + 0x200 * j + 0x54, handle);
        }
        c.bus
            .write8(OC812_READY, if seed % 11 == 0 { 0 } else { 1 });
        c.bus
            .write32(OC812_BUSY, if seed % 11 == 5 { 1 } else { 0 });
        let rnd = r.next_u32();
        let bits = (1u32 << (rnd % 24)) | (1u32 << ((rnd >> 8) % 24));
        let top = oc812_pick(&mut r, &[0xff, 0x11, 0x33, 0x0f, 0xf0, 0x01]) << 24;
        let mask = match seed % 4 {
            0 => u32::MAX,
            1 => top | bits,
            2 => top | (rnd & 0xff_ffff),
            _ => bits,
        };
        c.g[A0][0] = sx(mask);
    }

    /// `PATH_StopTracks`: stops the first track a mask selects (or none), then empties the player
    /// (sorting the maps) and shuts the platform layer down when no track is left, with every
    /// not-ready/busy exit and unwinds in the platform calls.
    #[test]
    fn path_stop_tracks_matches() {
        check(
            ee::s14::f_003d2a68__generated,
            path_stop_tracks,
            300,
            &stop_tracks_setup,
        );
        assert!(oc812_paths(ee::s14::f_003d2a68__generated, 300, &stop_tracks_setup) >= 5);
    }

    fn add_track_setup(c: &mut Cpu, seed: u32) {
        oc812_install_fakes();
        let mut r = machine(c, seed, 0x25f0, 6);
        c.pc = 0x003d_25f0;
        c.bus.write32(OC812_CTRL, OC812_MAGIC);
        let slots = if seed % 3 == 0 { 0 } else { 6 };
        oc812_world(c, &mut r, slots);
        let ids = [0x0100_0000u32, 0x0200_0000, 0x1100_0000, 0x2000_0000];
        for m in 0..4u32 {
            let map = OC812_MAPS + OC812_MAP_SIZE * m;
            c.bus.write32(map + 0x30, ids[m as usize]);
            let h = if r.next_u32() % 4 == 0 { 0 } else { OC812_HDR };
            c.bus.write32(map + 0x34, h);
        }
        c.bus
            .write8(OC812_READY, if seed % 13 == 0 { 0 } else { 1 });
        c.bus.write8(OC812_READY + 2, r.next_u32() as u8);
        c.bus
            .write32(OC812_BUSY, if seed % 13 == 5 { 1 } else { 0 });
        // The track name: path separators, a drive colon, letters, digits and junk.
        let name = DATA + 0x5400;
        let chars = b"abXZ09_.-\\/: ~";
        let len = r.next_u32() % 24;
        for i in 0..len {
            let ch = chars[(r.next_u32() as usize) % chars.len()];
            c.bus.write8(name + i, ch);
        }
        c.bus.write8(name + len, 0);
        // The bytes the `sprintf` fake leaves in the name buffer (some NULs).
        let tpl = b"abXZ09_.-\\/: ~\0";
        for i in 0..0x20u32 {
            let ch = tpl[(r.next_u32() as usize) % tpl.len()];
            c.bus.write8(OC812_TPL + i, ch);
        }
        let top = ids[(r.next_u32() % 4) as usize];
        let rnd = r.next_u32();
        let bits = match seed % 4 {
            0 => 1u32 << (rnd % 24),
            1 => rnd & 0xff_ffff,
            2 => 0,
            _ => 1 << 23,
        };
        c.g[A0][0] = sx(top | bits);
        c.g[A1][0] = name as u64;
        let a2 = match r.next_u32() % 3 {
            0 => r.next_u32() % 0x32,
            1 => 0x32 + r.next_u32() % 100,
            _ => r.next_u32(),
        };
        c.g[A2][0] = sx(a2);
        c.g[A3][0] = sx(r.next_u32());
        c.g[T0][0] = (r.next_u32() % 6) as u64;
        c.g[T1][0] = sx(r.next_u32());
        c.g[T2][0] = (DATA + 0x6000) as u64;
        c.g[T3][0] = (0x200 + r.next_u32() % 0x1000) as u64;
        let ret = [-5, 0, 3][(r.next_u32() % 3) as usize];
        oc812_force(c, 0x002a_e4c0, ret);
    }

    /// `PATH_AddTrack`: adds a named track to the map of an id (clears and fills its record, the
    /// upper-cased 8-character name, starts the platform layer for the first track, hands the
    /// record to the platform: failure clears the slot), with the already-present, not-ready and
    /// busy exits and unwinds in the callees.
    #[test]
    fn path_add_track_matches() {
        check(
            ee::s14::f_003d25f0__generated,
            path_add_track,
            300,
            &add_track_setup,
        );
        assert!(oc812_paths(ee::s14::f_003d25f0__generated, 300, &add_track_setup) >= 5);
    }

    fn execute_action_setup(c: &mut Cpu, seed: u32) {
        oc812_install_fakes();
        let mut r = machine(c, seed, 0xfc30, 8);
        c.pc = 0x003c_fc30;
        c.bus.write32(OC812_CTRL, OC812_MAGIC);
        oc812_world(c, &mut r, 2);
        let ev = DATA + 0x1000;
        fill(c, &mut r, ev, 0x100);
        fill(c, &mut r, OC812_HDR, 0x440);
        for i in 0..0x300u32 {
            let a = (r.next_u32() & 0x3ff) as u16;
            let b = (r.next_u32() & 0x3ff) as u16;
            c.bus.write16(OC812_IDXA + 2 * i, a);
            c.bus.write16(OC812_IDXB + 2 * i, b);
        }
        fill(c, &mut r, OC812_NODE, 0x80);
        let count = (r.next_u32() % 10) as u16;
        c.bus.write16(OC812_HDR + 0x12, count);
        for j in 0..24u32 {
            let t = OC812_TRACKS + 0x200 * j;
            let node = if r.next_u32() % 4 == 0 { 0 } else { OC812_NODE };
            c.bus.write32(t + 0x30, node);
            c.bus.write16(t + 0x36, (r.next_u32() % 40) as u16);
            c.bus.write32(t + 0x54, r.next_u32() % 6);
            c.bus.write8(t + 0x2f, (r.next_u32() % 2) as u8);
            c.bus.write32(t + 0x4c, r.next_u32() % 2);
        }
        let tops = [0xffu32, 0xff, 0x11, 0x33, 0x0f, 0xf0, 0x01];
        for m in 0..4u32 {
            let map = OC812_MAPS + OC812_MAP_SIZE * m;
            let top = oc812_pick(&mut r, &tops) << 24;
            c.bus.write32(map + 0x30, top);
            let h = if r.next_u32() % 5 == 0 { 0 } else { OC812_HDR };
            c.bus.write32(map + 0x34, h);
            c.bus.write32(map + 0x38, OC812_IDXA + 0x200);
            c.bus.write32(map + 0x40, OC812_IDXB + 0x200);
        }
        let g = OC812_MAPS + OC812_MAP_SIZE * (r.next_u32() % 4);
        c.bus.write32(g + 0x34, OC812_HDR);
        c.bus.write32(OC812_CUR, g);
        c.bus.write32(OC812_TIME, r.next_u32() % 100);
        let flags = oc812_pick(&mut r, &[0, 0x20]);
        c.bus.write32(OC812_FLAGS, flags);
        // The event and its action list.
        let n = 2 + r.next_u32() % 12;
        let hi = (r.next_u32() & 0xffc0) as u16;
        c.bus.write16(ev + 0xe, hi | n as u16);
        c.bus.write32(ev + 8, r.next_u32() % 100);
        let act = ev + 0x10 + 0x10 * (r.next_u32() % (n - 1));
        let top = oc812_pick(&mut r, &[0xff, 0x11, 0x33, 0x01, 0x0f]) << 24;
        let rnd = r.next_u32();
        let bits = match seed % 4 {
            0 => 1u32 << (rnd % 24),
            1 => (1u32 << (rnd % 24)) | (1u32 << ((rnd >> 8) % 24)),
            2 => 0xff_ffff,
            _ => rnd & 0xff_ffff,
        };
        c.bus.write32(act, top | bits);
        c.bus.write32(act + 4, r.next_u32());
        let opcode = r.next_u32() % 18;
        let flags = (r.next_u32() & !0xff00) | (opcode << 8);
        c.bus.write32(act + 8, flags);
        let value = match r.next_u32() % 8 {
            0 => r.next_u32(),
            1 => 5,
            2 => 6,
            3 => 0xffff_ffdf,
            4 => 0xffff_ffe0,
            5 => ((r.next_u32() % 4) << 16) | (r.next_u32() % 40),
            6 => ((r.next_u32() % 3) << 16) | (r.next_u32() % 64),
            _ => (r.next_u32() % 64) | 0x100_0000,
        };
        c.bus.write32(act + 12, value);
        let submit = [0, 3, -1, -4, -9999][(r.next_u32() % 5) as usize];
        oc812_force(c, 0x003d_41a8, submit);
        let node = (r.next_u32() % 20) as i32;
        oc812_force(c, 0x003d_3900, node);
        let aux = (r.next_u32() % 10) as i32;
        oc812_force(c, 0x002a_f4b0, aux);
        c.g[A0][0] = ev as u64;
        c.g[A1][0] = act as u64;
    }

    /// `PATH_ExecuteAction`: runs the action of an event on every track its mask names, by opcode:
    /// conditions on the play state / node / volume / aux send (setting the "met" flag and
    /// skipping the rest of the action list or the conditional block), waits, beat positions,
    /// node jumps (saving and restoring the track record on a failed submit), pausing, volume,
    /// aux send and level, variable stores, fades, event posting and event mark bits.
    #[test]
    fn path_execute_action_matches() {
        check(
            ee::s14::f_003cfc30__generated,
            path_execute_action,
            400,
            &execute_action_setup,
        );
        assert!(oc812_paths(ee::s14::f_003cfc30__generated, 400, &execute_action_setup) >= 8);
    }
}
