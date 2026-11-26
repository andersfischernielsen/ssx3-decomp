//! Staging module oc710 (wave wave240): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc710`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

/// `PATH_ExecuteAction` (0x003cfc30): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (its generated body, `f_003cfc30`; switch to the `__generated` alias at install,
/// or the override would call itself).
#[allow(dead_code)] // not installed yet: no finished differential test (see the wave report)
pub fn path_execute_action(c: &mut Cpu) {
    if c.pc != 0x003cfc30 {
        return ee::s14::f_003cfc30(c);
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
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
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
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
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
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
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
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V0][0] = c.lh((c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    let t = c.g[A2][0] != 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
                    c.ldr(6, (c.g[V1][0] as u32));
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.ldl(10, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(10, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(6, (c.g[A0][0] as u32));
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
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32));
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd((c.g[A0][0] as u32), c.g[A2][0]);
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
                    c.ldr(5, (c.g[V1][0] as u32));
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(5, (c.g[A0][0] as u32));
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
                    c.g[V0][0] = sx(0u32.wrapping_sub((c.g[S0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
                    c.g[V0][0] = sx(0u32.wrapping_sub((c.g[S3][0] as u32)));
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
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
                    c.g[V1][0] = c.lh((c.g[A0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
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
                    c.g[V0][0] = c.lh((c.g[A1][0] as u32));

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
                    c.ldr(7, (c.g[A1][0] as u32));
                    c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0xf));
                    c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x8));
                    c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x17));
                    c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x10));
                    c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0x1f));
                    c.ldr(10, (c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(7, (c.g[A0][0] as u32));
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
                    c.g[V1][0] = c.ld((c.g[A1][0] as u32));
                    c.g[A2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd((c.g[A0][0] as u32), c.g[V1][0]);
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
                    c.ldr(9, (c.g[A1][0] as u32));
                    c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0xf));
                    c.ldr(10, (c.g[A1][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(9, (c.g[A0][0] as u32));
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
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[RA][0] = sx(0x003d02e4);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
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
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[RA][0] = sx(0x003d0354);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
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
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
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
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[A0][0] = (c.g[T1][0] < 0x7u64) as u64;
                    c.g[V0][0] = c.lh((c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    let t = c.g[A0][0] == 0u64;
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
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
                    c.g[V0][0] = c.lh((c.g[A1][0] as u32));

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
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
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
                    c.g[V0][0] = c.lh((c.g[A1][0] as u32));

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
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
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
                    c.g[V0][0] = c.lh((c.g[A1][0] as u32));

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
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
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
                    c.g[V0][0] = c.lh((c.g[A1][0] as u32));

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
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
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
                    c.g[V0][0] = c.lh((c.g[A1][0] as u32));

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
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
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
                    c.g[V0][0] = c.lh((c.g[A1][0] as u32));

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
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
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
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
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
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
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
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
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
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
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
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A2][0] as u32)));
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

#[cfg(test)]
mod tests {
    use super::*;
}
