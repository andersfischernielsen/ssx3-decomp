//! Staging module oc807 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc807`.

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
    (
        0x0028_f478,
        ssx_audio_system_on_song_changed_resumable,
        "SSXAudioSystem_OnSongChanged",
    ),
    (
        0x0029_4f78,
        ssx_audio_system_play_fe_sound,
        "SSXAudioSystem_PlayFESound",
    ),
    (
        0x002a_5d08,
        crowd_audio_instance_add_source,
        "CrowdAudio_InstanceAddSource",
    ),
    (
        0x002a_7040,
        crowd_audio_parse_config,
        "CrowdAudio_ParseConfig",
    ),
];

/// `SSXAudioSystem_OnSongChanged` (0x0028f478): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ssx_audio_system_on_song_changed_resumable(c: &mut Cpu) {
    if c.pc != 0x0028f478 {
        return ee::s07::f_0028f478__generated(c);
    }
    guest(c, |c| {
        'b_28f514: {
            'b_28f510: {
                c.cycles += 15;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[RA][0] = sx(0x0028f490);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                call_ptr(c, 0x002b35a0, 0x0028f490)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1;
                c.g[RA][0] = sx(0x0028f4a0);
                c.g[A2][0] = 0u64;
                Callee {
                    addr: 0x00287930,
                    f: ee::s07::SSXAudioSystem_GetChannelGain,
                }
                .call(c, 0x0028f4a0)?;
                c.mtc1(0, 1);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_28f510;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_28f514;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                        let t = c.g[A0][0] == 0u64;
                        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        if t {
                            break 'b_28f510;
                        } else {
                            c.cycles += 2;
                            if c.g[S0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                break 'b_28f514;
                            } else {
                                c.cycles += 9;
                                c.g[A0][0] = 0x4a0000;
                                c.g[RA][0] = sx(0x0028f4e0);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3660));
                                call_ptr(c, 0x00317670, 0x0028f4e0)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0028f4ec);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039f9d8, 0x0028f4ec)?;
                                c.g[A3][0] = c.g[V0][0];
                                let t = c.g[A3][0] == 0u64;
                                c.g[A1][0] = 0x5;
                                if t {
                                    break 'b_28f510;
                                } else {
                                    c.cycles += 6;
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.g[A2][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x0028f510);
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0028f510)?;
                                    break 'b_28f510;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_28f514;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `SSXAudioSystem_PlayFESound` (0x00294f78): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ssx_audio_system_play_fe_sound(c: &mut Cpu) {
    if c.pc != 0x00294f78 {
        return ee::s08::f_00294f78__generated(c);
    }
    guest(c, |c| {
        'b_294ff8: {
            c.cycles += 6;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
            c.g[A3][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = (c.g[A1][0] < 0x10u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[A2][0] = 0xffffffffffffffff;
            if t {
                break 'b_294ff8;
            } else {
                'b_294ff4: {
                    'b_294fec: {
                        'b_294fe4: {
                            'b_294fdc: {
                                'b_294fd4: {
                                    'b_294fcc: {
                                        'b_294fc4: {
                                            'b_294fbc: {
                                                'b_294fb4: {
                                                    c.cycles += 7;
                                                    c.g[V0][0] = 0x480000;
                                                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x2d60));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[V0][0] as u32));
                                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                    let t = c.g[A0][0] as u32;
                                                    match t {
                                                        0x00294fac => {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A2][0] = 0xffffffffffffffff;
                                                            if t {
                                                                break 'b_294ff8;
                                                            } else {
                                                                break 'b_294fb4;
                                                            }
                                                        }
                                                        0x00294fb4 => {
                                                            break 'b_294fb4;
                                                        }
                                                        0x00294fbc => {
                                                            break 'b_294fbc;
                                                        }
                                                        0x00294fc4 => {
                                                            break 'b_294fc4;
                                                        }
                                                        0x00294fcc => {
                                                            break 'b_294fcc;
                                                        }
                                                        0x00294fd4 => {
                                                            break 'b_294fd4;
                                                        }
                                                        0x00294fdc => {
                                                            break 'b_294fdc;
                                                        }
                                                        0x00294fe4 => {
                                                            break 'b_294fe4;
                                                        }
                                                        0x00294fec => {
                                                            break 'b_294fec;
                                                        }
                                                        0x00294ff4 => {
                                                            break 'b_294ff4;
                                                        }
                                                        _ => {
                                                            c.jump_out(t, 0x00294f78, 0x00295024);
                                                            return Ok(());
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A2][0] = 0x10;
                                                if t {
                                                    break 'b_294ff8;
                                                } else {
                                                    break 'b_294fbc;
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A2][0] = 0x11;
                                            if t {
                                                break 'b_294ff8;
                                            } else {
                                                break 'b_294fc4;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A2][0] = 0x3;
                                        if t {
                                            break 'b_294ff8;
                                        } else {
                                            break 'b_294fcc;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A2][0] = 0x4;
                                    if t {
                                        break 'b_294ff8;
                                    } else {
                                        break 'b_294fd4;
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[A2][0] = 0x2;
                                if t {
                                    break 'b_294ff8;
                                } else {
                                    break 'b_294fdc;
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[A2][0] = 0x1;
                            if t {
                                break 'b_294ff8;
                            } else {
                                break 'b_294fe4;
                            }
                        }
                        c.cycles += 2;
                        let t = true;
                        c.g[A2][0] = 0xd;
                        if t {
                            break 'b_294ff8;
                        } else {
                            break 'b_294fec;
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[A2][0] = 0x12;
                    if t {
                        break 'b_294ff8;
                    } else {
                        break 'b_294ff4;
                    }
                }
                c.cycles += 1;
                c.g[A2][0] = 0x7;
                break 'b_294ff8;
            }
        }
        'b_295018: {
            c.cycles += 3;
            c.g[V0][0] = 0xffffffffffffffff;
            let t = c.g[A2][0] == c.g[V0][0];
            c.g[A1][0] = 0u64;
            if t {
                break 'b_295018;
            } else {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00295018);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00295018)?;
                break 'b_295018;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `CrowdAudio_InstanceAddSource` (0x002a5d08): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn crowd_audio_instance_add_source(c: &mut Cpu) {
    if c.pc != 0x002a5d08 {
        return ee::s08::f_002a5d08__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002a5d08;
        loop {
            match pc {
                0x002a5d08 => {
                    c.cycles += 17;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S5][0] = c.g[A0][0];
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S7][0] = c.g[A2][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[A3][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xd4));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 15i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[FP][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x002a5d6c;
                        continue;
                    }
                    pc = 0x002a5d4c;
                    continue;
                }
                0x002a5d4c => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x24));
                    c.g[T2][0] = c.g[V1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x002a5db8;
                        continue;
                    }
                    pc = 0x002a5d6c;
                    continue;
                }
                0x002a5d6c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x002a62bc;
                        continue;
                    }
                    pc = 0x002a5d74;
                    continue;
                }
                0x002a5d74 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe8));
                    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(0xe0));
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[T0][0] as u32).wrapping_add(0xe8), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[S6][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x002a5e60;
                        continue;
                    }
                    pc = 0x002a5d90;
                    continue;
                }
                0x002a5d90 => {
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[S6][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xe0), c.g[S7][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xe8), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x170));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    let t = true;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x170), c.g[V1][0]);
                    if t {
                        pc = 0x002a5ee8;
                        continue;
                    }
                    pc = 0x002a5db8;
                    continue;
                }
                0x002a5db8 => {
                    c.cycles += 4;
                    c.g[S6][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.g[V1][0];
                    pc = 0x002a5dc8;
                    continue;
                }
                0x002a5dc8 => {
                    c.cycles += 2;

                    if c.g[S6][0] != 0u64 {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        pc = 0x002a5de8;
                        continue;
                    }
                    pc = 0x002a5dd0;
                    continue;
                }
                0x002a5dd0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        pc = 0x002a5de8;
                        continue;
                    }
                    pc = 0x002a5ddc;
                    continue;
                }
                0x002a5ddc => {
                    c.cycles += 3;
                    c.g[S6][0] = c.g[V1][0];
                    c.g[FP][0] = c.g[A2][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    pc = 0x002a5de8;
                    continue;
                }
                0x002a5de8 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 15i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                    if t {
                        if back_edge_unwinds(c, 0x002a5dc8) {
                            return Ok(());
                        }
                        pc = 0x002a5dc8;
                        continue;
                    }
                    pc = 0x002a5df8;
                    continue;
                }
                0x002a5df8 => {
                    c.cycles += 18;
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[A2][0] = 0u64;
                    c.sw((c.g[S5][0] as u32).wrapping_add(0xd4), c.g[V0][0]);
                    c.g[A3][0] = sx((c.g[S5][0] as u32).wrapping_add(0xe0));
                    c.g[T0][0] = c.g[S5][0];
                    c.g[T1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.sw(c.g[S6][0] as u32, c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.sw((c.g[S6][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    pc = 0x002a5e40;
                    continue;
                }
                0x002a5e40 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                    let t = c.g[V0][0] == c.g[S7][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x002a5d74) {
                            return Ok(());
                        }
                        pc = 0x002a5d74;
                        continue;
                    }
                    pc = 0x002a5e4c;
                    continue;
                }
                0x002a5e4c => {
                    c.cycles += 5;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x30));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x30));
                    if t {
                        if back_edge_unwinds(c, 0x002a5e40) {
                            return Ok(());
                        }
                        pc = 0x002a5e40;
                        continue;
                    }
                    pc = 0x002a5e60;
                    continue;
                }
                0x002a5e60 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xd4));
                    c.g[V0][0] = 0x1;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                        pc = 0x002a5eb0;
                        continue;
                    }
                    pc = 0x002a5e70;
                    continue;
                }
                0x002a5e70 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.sw(c.g[S5][0] as u32, c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x3c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x17c), c.g[V1][0]);
                    c.g[A0][0] = c.lw(c.g[S6][0] as u32);
                    c.lq(2, c.g[A0][0] as u32);
                    c.sq(2, (c.g[S5][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                    pc = 0x002a5eb0;
                    continue;
                }
                0x002a5eb0 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[FP][0];
                    if t {
                        pc = 0x002a62bc;
                        continue;
                    }
                    pc = 0x002a5eb8;
                    continue;
                }
                0x002a5eb8 => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0xe8));
                    c.g[V1][0] = c.g[S5][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x90));
                    pc = 0x002a5ec8;
                    continue;
                }
                0x002a5ec8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe0));
                    if t {
                        if back_edge_unwinds(c, 0x002a5d90) {
                            return Ok(());
                        }
                        pc = 0x002a5d90;
                        continue;
                    }
                    pc = 0x002a5ed4;
                    continue;
                }
                0x002a5ed4 => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                    if t {
                        if back_edge_unwinds(c, 0x002a5ec8) {
                            return Ok(());
                        }
                        pc = 0x002a5ec8;
                        continue;
                    }
                    pc = 0x002a5ee8;
                    continue;
                }
                0x002a5ee8 => {
                    c.cycles += 168;
                    c.g[V1][0] = c.lw(c.g[S6][0] as u32);
                    c.g[S3][0] = 0x18;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.lq(2, c.g[V1][0] as u32);
                    c.g[A2][0] = 0x8;
                    c.g[S2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[S4][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xd8));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.mult(0, 3, 3, 19);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
                    c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                    c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
                    c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
                    c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
                    c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.sdr(2, c.g[V1][0] as u32);
                    c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                    c.mult(0, 4, 4, 19);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                    c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[RA][0] = sx(0x002a5fd4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                    call_ptr(c, 0x00416210, 0x002a5fd4)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = 0x64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                    c.g[A1][0] = 0x5a;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A2][0] = 0x32;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = 0x7f;
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[T0][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[A3][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[T0][0]);
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[S4][0]);
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[S7][0]);
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[A0][0]);
                    c.g[A1][0] = c.lw(c.g[A2][0] as u32);
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                    c.mult(0, 4, 4, 19);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    if t {
                        pc = 0x002a61a0;
                        continue;
                    }
                    pc = 0x002a6188;
                    continue;
                }
                0x002a6188 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x002a61cc;
                        continue;
                    }
                    pc = 0x002a61a0;
                    continue;
                }
                0x002a61a0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;

                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = 0x7f;
                        pc = 0x002a61cc;
                        continue;
                    }
                    pc = 0x002a61bc;
                    continue;
                }
                0x002a61bc => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                    pc = 0x002a61cc;
                    continue;
                }
                0x002a61cc => {
                    c.cycles += 60;
                    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[A2][0] = 0x1;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                    c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x17c));
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                    c.g[A1][0] = c.lw(c.g[S5][0] as u32);
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                    c.g[V0][0] = 0x4a0000;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3700));
                    c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                    c.ldr(3, c.g[A3][0] as u32);
                    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.sdr(3, c.g[SP][0] as u32);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.ldr(8, c.g[SP][0] as u32);
                    c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x47));
                    c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x238));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[S5][0]);
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x254));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[RA][0] = sx(0x002a62b0);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                    call_ptr(c, 0x002906b8, 0x002a62b0)?;
                    c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V0][0] = c.g[FP][0];
                    pc = 0x002a62bc;
                    continue;
                }
                0x002a62bc => {
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
                    c.jump_out(pc, 0x002a5d08, 0x002a62ec);
                    return Ok(());
                }
            }
        }
    });
}

/// `CrowdAudio_ParseConfig` (0x002a7040): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn crowd_audio_parse_config(c: &mut Cpu) {
    if c.pc != 0x002a7040 {
        return ee::s08::f_002a7040__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002a7040;
        loop {
            match pc {
                0x002a7040 => {
                    c.cycles += 15;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4a0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x460));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x440));
                    c.g[S3][0] = c.g[A1][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x430));
                    c.g[S5][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                    c.g[S6][0] = 0x54;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x480));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x470));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x450));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x420));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x410));
                    let t = true;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x400), c.g[RA][0]);
                    if t {
                        pc = 0x002a7280;
                        continue;
                    }
                    pc = 0x002a707c;
                    continue;
                }
                0x002a707c => {
                    c.cycles += 1;
                    pc = 0x002a7080;
                    continue;
                }
                0x002a7080 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002a7090);
                    c.g[A2][0] = c.g[SP][0];
                    Callee {
                        addr: 0x002a7ff8,
                        f: ee::s08::IniParser_GetString,
                    }
                    .call(c, 0x002a7090)?;
                    c.mult(0, 2, 20, 22);
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[RA][0] = sx(0x002a70a8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x34));
                    Callee {
                        addr: 0x002a4c28,
                        f: ee::s08::CrowdAudio_AddSampleFile,
                    }
                    .call(c, 0x002a70a8)?;
                    let t = true;

                    if t {
                        pc = 0x002a7280;
                        continue;
                    }
                    pc = 0x002a70b0;
                    continue;
                }
                0x002a70b0 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002a70c0);
                    c.g[A2][0] = c.g[SP][0];
                    Callee {
                        addr: 0x002a7ff8,
                        f: ee::s08::IniParser_GetString,
                    }
                    .call(c, 0x002a70c0)?;
                    c.mult(0, 2, 20, 22);
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[RA][0] = sx(0x002a70d8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4c));
                    Callee {
                        addr: 0x002a4c28,
                        f: ee::s08::CrowdAudio_AddSampleFile,
                    }
                    .call(c, 0x002a70d8)?;
                    let t = true;

                    if t {
                        pc = 0x002a7280;
                        continue;
                    }
                    pc = 0x002a70e0;
                    continue;
                }
                0x002a70e0 => {
                    c.cycles += 8;
                    c.mult(0, 2, 20, 22);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[RA][0] = sx(0x002a70f8);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x64));
                    Callee {
                        addr: 0x002a8070,
                        f: ee::s08::IniParser_GetFloat,
                    }
                    .call(c, 0x002a70f8)?;
                    let t = true;

                    if t {
                        pc = 0x002a7280;
                        continue;
                    }
                    pc = 0x002a7100;
                    continue;
                }
                0x002a7100 => {
                    c.cycles += 8;
                    c.mult(0, 2, 20, 22);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[RA][0] = sx(0x002a7118);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x68));
                    Callee {
                        addr: 0x002a8070,
                        f: ee::s08::IniParser_GetFloat,
                    }
                    .call(c, 0x002a7118)?;
                    let t = true;

                    if t {
                        pc = 0x002a7280;
                        continue;
                    }
                    pc = 0x002a7120;
                    continue;
                }
                0x002a7120 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[RA][0] = sx(0x002a7134);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6c));
                    Callee {
                        addr: 0x002a8070,
                        f: ee::s08::IniParser_GetFloat,
                    }
                    .call(c, 0x002a7134)?;
                    let t = true;

                    if t {
                        pc = 0x002a7280;
                        continue;
                    }
                    pc = 0x002a713c;
                    continue;
                }
                0x002a713c => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002a7148);
                    c.g[S4][0] = 0u64;
                    Callee {
                        addr: 0x002a7da0,
                        f: ee::s08::IniParser_ReadToken,
                    }
                    .call(c, 0x002a7148)?;
                    c.g[FP][0] = 0x4a0000;
                    c.g[S1][0] = c.g[V0][0];
                    let t = true;
                    c.g[S7][0] = 0x480000;
                    if t {
                        pc = 0x002a7210;
                        continue;
                    }
                    pc = 0x002a7158;
                    continue;
                }
                0x002a7158 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x002a7160);
                    call_ptr(c, 0x00417828, 0x002a7160)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002a716c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002a716c)?;
                    c.g[V1][0] = 0x480000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3118));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.g[S2][0];
                    if t {
                        if back_edge_unwinds(c, 0x002a7080) {
                            return Ok(());
                        }
                        pc = 0x002a7080;
                        continue;
                    }
                    pc = 0x002a7180;
                    continue;
                }
                0x002a7180 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x002a7188);
                    call_ptr(c, 0x00417828, 0x002a7188)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002a7194);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002a7194)?;
                    c.g[V1][0] = 0x480000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3128));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.g[S2][0];
                    if t {
                        if back_edge_unwinds(c, 0x002a70b0) {
                            return Ok(());
                        }
                        pc = 0x002a70b0;
                        continue;
                    }
                    pc = 0x002a71a8;
                    continue;
                }
                0x002a71a8 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x002a71b0);
                    call_ptr(c, 0x00417828, 0x002a71b0)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002a71bc);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002a71bc)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3718));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.g[S2][0];
                    if t {
                        if back_edge_unwinds(c, 0x002a70e0) {
                            return Ok(());
                        }
                        pc = 0x002a70e0;
                        continue;
                    }
                    pc = 0x002a71cc;
                    continue;
                }
                0x002a71cc => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x002a71d4);
                    call_ptr(c, 0x00417828, 0x002a71d4)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002a71e0);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002a71e0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3138));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.g[S2][0];
                    if t {
                        if back_edge_unwinds(c, 0x002a7100) {
                            return Ok(());
                        }
                        pc = 0x002a7100;
                        continue;
                    }
                    pc = 0x002a71f0;
                    continue;
                }
                0x002a71f0 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x002a71f8);
                    call_ptr(c, 0x00417828, 0x002a71f8)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002a7204);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002a7204)?;
                    let t = c.g[V0][0] == 0u64;
                    c.mult(0, 2, 20, 22);
                    if t {
                        if back_edge_unwinds(c, 0x002a7120) {
                            return Ok(());
                        }
                        pc = 0x002a7120;
                        continue;
                    }
                    pc = 0x002a720c;
                    continue;
                }
                0x002a720c => {
                    c.cycles += 1;
                    c.g[S4][0] = c.g[S2][0];
                    pc = 0x002a7210;
                    continue;
                }
                0x002a7210 => {
                    c.cycles += 8;
                    c.g[V0][0] = 0x4a0000;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
                    c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3708));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = ((c.g[S4][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.g[S2][0];
                    if t {
                        pc = 0x002a7280;
                        continue;
                    }
                    pc = 0x002a7230;
                    continue;
                }
                0x002a7230 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x002a7238);
                    call_ptr(c, 0x00417828, 0x002a7238)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002a7244);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002a7244)?;
                    c.g[V1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3710));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = c.g[S2][0];
                    if t {
                        if back_edge_unwinds(c, 0x002a7158) {
                            return Ok(());
                        }
                        pc = 0x002a7158;
                        continue;
                    }
                    pc = 0x002a7258;
                    continue;
                }
                0x002a7258 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002a7268);
                    c.g[A2][0] = c.g[SP][0];
                    Callee {
                        addr: 0x002a7ff8,
                        f: ee::s08::IniParser_GetString,
                    }
                    .call(c, 0x002a7268)?;
                    c.mult(0, 4, 20, 22);
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c));
                    c.g[RA][0] = sx(0x002a7280);
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Callee {
                        addr: 0x002a4c28,
                        f: ee::s08::CrowdAudio_AddSampleFile,
                    }
                    .call(c, 0x002a7280)?;
                    pc = 0x002a7280;
                    continue;
                }
                0x002a7280 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002a7288);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x002a7f90,
                        f: ee::s08::IniParser_AtSectionEnd,
                    }
                    .call(c, 0x002a7288)?;
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                    if t {
                        if back_edge_unwinds(c, 0x002a713c) {
                            return Ok(());
                        }
                        pc = 0x002a713c;
                        continue;
                    }
                    pc = 0x002a7290;
                    continue;
                }
                0x002a7290 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x480));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x470));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x460));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x450));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x440));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x430));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x420));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x410));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x400));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4a0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002a7040, 0x002a72bc);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{fill, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC807_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC807_MAGIC: u32 = 0x4f43_3830;
    pub(crate) const OC807_SELF: u32 = DATA + 0x21_0000;
    pub(crate) const OC807_DESC: u32 = DATA + 0x22_0000;
    pub(crate) const OC807_PG: u32 = DATA + 0x23_0000;
    pub(crate) const OC807_G: u32 = DATA + 0x24_0000;
    /// Twenty 0x400-byte arrays of small numbers the crowd structures point at.
    pub(crate) const OC807_BASE: u32 = DATA + 0x25_0000;
    pub(crate) const OC807_WORLD: u32 = DATA + 0x27_0000;
    pub(crate) const OC807_UI: u32 = DATA + 0x27_1000;
    pub(crate) const OC807_OBJ: u32 = DATA + 0x27_2000;
    pub(crate) const OC807_STATE: u32 = DATA + 0x27_3000;
    pub(crate) const OC807_VT: u32 = DATA + 0x27_4000;
    /// Per-callee scripts: 0x40 bytes at `OC807_QUEUE + (addr >> 2 & 0xfff) * 0x40`.
    pub(crate) const OC807_QUEUE: u32 = DATA + 0x30_0000;

    /// The script of the fake for `addr`: word 0 the next index, word 1 the
    /// length (0: no script, the fake's hash pointer), word 2 the bits `f0`
    /// is set to (0: leave the fake's), words 3.. the `v0` results in call
    /// order (the last one repeats).
    pub(crate) fn oc807_queue(addr: u32) -> u32 {
        OC807_QUEUE + ((addr >> 2) & 0xfff) * 0x40
    }

    /// Script the results (`v0`) of the fake for `addr`.
    pub(crate) fn oc807_script(c: &mut Cpu, addr: u32, vals: &[u32]) {
        let q = oc807_queue(addr);
        c.bus.write32(q, 0);
        c.bus.write32(q + 4, vals.len() as u32);
        for (i, v) in vals.iter().enumerate() {
            c.bus.write32(q + 12 + 4 * i as u32, *v);
        }
    }

    /// Make the fake for `addr` return `bits` in `f0`.
    pub(crate) fn oc807_script_f0(c: &mut Cpu, addr: u32, bits: u32) {
        c.bus.write32(oc807_queue(addr) + 8, bits);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then applies its
    /// script unless the call unwound.
    pub(crate) fn oc807_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let q = oc807_queue(addr);
        let pos = c.bus.read32(q);
        let len = c.bus.read32(q + 4);
        let f0 = c.bus.read32(q + 8);
        fakes::fake_call(c, addr);
        if c.pc == ra {
            if len != 0 {
                let i = pos.min(len - 1);
                c.g[V0][0] = c.bus.read32(q + 12 + 4 * i) as u64;
                c.bus.write32(q, pos + 1);
            }
            if f0 != 0 {
                c.f[0] = f0;
            }
        }
    }

    macro_rules! oc807_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC807_CTRL) != OC807_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc807_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc807_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc807_fakes!(
        (
            oc807_fake_2b35a0,
            OC807_PREV_2B35A0,
            ee::s08::OVERRIDE_002B35A0,
            ee::s08::f_002b35a0__generated,
            0x002b_35a0
        ),
        (
            oc807_fake_287930,
            OC807_PREV_287930,
            ee::s07::OVERRIDE_00287930,
            ee::s07::f_00287930__generated,
            0x0028_7930
        ),
        (
            oc807_fake_317670,
            OC807_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc807_fake_39f9d8,
            OC807_PREV_39F9D8,
            ee::s13::OVERRIDE_0039F9D8,
            ee::s13::f_0039f9d8__generated,
            0x0039_f9d8
        ),
        (
            oc807_fake_2906b8,
            OC807_PREV_2906B8,
            ee::s07::OVERRIDE_002906B8,
            ee::s07::f_002906b8__generated,
            0x0029_06b8
        ),
        (
            oc807_fake_2a4c28,
            OC807_PREV_2A4C28,
            ee::s08::OVERRIDE_002A4C28,
            ee::s08::f_002a4c28__generated,
            0x002a_4c28
        ),
        (
            oc807_fake_2a7ff8,
            OC807_PREV_2A7FF8,
            ee::s08::OVERRIDE_002A7FF8,
            ee::s08::f_002a7ff8__generated,
            0x002a_7ff8
        ),
        (
            oc807_fake_2a8070,
            OC807_PREV_2A8070,
            ee::s08::OVERRIDE_002A8070,
            ee::s08::f_002a8070__generated,
            0x002a_8070
        ),
        (
            oc807_fake_2a7da0,
            OC807_PREV_2A7DA0,
            ee::s08::OVERRIDE_002A7DA0,
            ee::s08::f_002a7da0__generated,
            0x002a_7da0
        ),
        (
            oc807_fake_2a7f90,
            OC807_PREV_2A7F90,
            ee::s08::OVERRIDE_002A7F90,
            ee::s08::f_002a7f90__generated,
            0x002a_7f90
        ),
        (
            oc807_fake_416210,
            OC807_PREV_416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
        (
            oc807_fake_417828,
            OC807_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc807_fake_41aa88,
            OC807_PREV_41AA88,
            ee::s15::OVERRIDE_0041AA88,
            ee::s15::f_0041aa88__generated,
            0x0041_aa88
        ),
    );

    /// Scrambled registers, the fake hooks and their gate.
    pub(crate) fn oc807_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc807_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC807_CTRL, OC807_MAGIC);
        r
    }

    /// A small number or an edge value (counts, volumes: below and at 127).
    pub(crate) fn oc807_small(r: &mut Rng) -> u32 {
        const V: [u32; 9] = [0, 0, 1, 2, 3, 0x7e, 0x7f, 0x80, 0xffff_ffff];
        V[(r.next_u32() % 9) as usize]
    }

    pub(crate) fn oc807_fill_small(c: &mut Cpu, r: &mut Rng, addr: u32, words: u32) {
        for i in 0..words {
            let v = oc807_small(r);
            c.bus.write32(addr + 4 * i, v);
        }
    }

    /// `SSXAudioSystem_OnSongChanged(self)`: starts the song player on
    /// `self + 0x118`, and when channel 1's gain is positive and the UI state
    /// stack has a current state, asks the state found by the hash `0x4a3660`
    /// to handle message 5 through its vtable (`+0xc0` delta, `+0xc4` method).
    /// The gain (`f0`), the world and stack pointers, the state's presence and
    /// the vtable delta all vary with the seed.
    #[test]
    fn ssx_audio_system_on_song_changed_matches() {
        oc807_install_fakes();
        covered(
            ee::s07::f_0028f478__generated,
            ssx_audio_system_on_song_changed_resumable,
            240,
            &|c, seed| {
                let _ = oc807_fixture(c, seed, 0x0028_f478, 5);
                let gain = if seed % 4 == 0 {
                    0xbf80_0000
                } else {
                    0x3f80_0000
                };
                oc807_script_f0(c, 0x0028_7930, gain);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), OC807_WORLD);
                c.bus
                    .write32(OC807_WORLD + 0x84, if seed % 7 == 0 { 0 } else { OC807_UI });
                let top = match (seed / 7) % 4 {
                    0 => 0,
                    1 => 0xffff_ffe8, // self + 0x18 == 0
                    _ => OC807_OBJ,
                };
                c.bus.write32(OC807_UI + 0x48, top);
                let found = if (seed / 3) % 3 == 0 { 0 } else { OC807_STATE };
                oc807_script(c, 0x0039_f9d8, &[found]);
                c.bus.write32(OC807_STATE + 8, OC807_VT);
                c.bus.write16(OC807_VT + 0xc0, 0x10 * (seed % 3) as u16);
                c.bus.write32(OC807_VT + 0xc4, fakes::FAKE + 0x10);
                c.g[A0][0] = sx(OC807_SELF);
                c.pc = 0x0028_f478;
            },
        );
    }

    /// `SSXAudioSystem_PlayFESound(self, id)`: maps the front-end sound id
    /// (0..15, through the jump table at `0x482d60`) to a sound number
    /// (-1: none) and calls `self->vtable[+8/+0xc]` with `(self, 0, n)`.
    /// Ids from 16 up skip the table; one table entry per seed points outside
    /// the function.
    #[test]
    fn ssx_audio_system_play_fe_sound_matches() {
        const LABELS: [u32; 10] = [
            0x0029_4fac,
            0x0029_4fb4,
            0x0029_4fbc,
            0x0029_4fc4,
            0x0029_4fcc,
            0x0029_4fd4,
            0x0029_4fdc,
            0x0029_4fe4,
            0x0029_4fec,
            0x0029_4ff4,
        ];
        oc807_install_fakes();
        covered(
            ee::s08::f_00294f78__generated,
            ssx_audio_system_play_fe_sound,
            200,
            &|c, seed| {
                let _ = oc807_fixture(c, seed, 0x0029_4f78, 1);
                for i in 0..16u32 {
                    c.bus
                        .write32(0x0048_2d60 + 4 * i, LABELS[((i + seed) % 10) as usize]);
                }
                if seed % 11 == 0 {
                    c.bus.write32(0x0048_2d60 + 4 * (seed % 16), 0x0029_5000);
                }
                c.bus.write32(OC807_SELF + 0xc, OC807_VT);
                c.bus.write16(OC807_VT + 8, 0x10 * (seed % 3) as u16);
                c.bus.write32(OC807_VT + 0xc, fakes::FAKE + 0x10);
                c.g[A0][0] = sx(OC807_SELF);
                c.g[A1][0] = (seed % 20) as u64;
                c.pc = 0x0029_4f78;
            },
        );
    }

    /// `CrowdAudio_InstanceAddSource(self, desc, id)`: adds a source (sample
    /// `desc[idx]`) to the crowd instance: finds a free voice slot (15 of
    /// them) and a slot of the three per-id entries (reusing one with the same
    /// id), then fills the shared crowd tables (`gp+0x608`) and calls
    /// `Audio_PlayPushedSound`. The seed varies the slot count, which slot is
    /// free, whether an id matches, whether the entries are in use, the
    /// table index and the volume values (below, at and above 127).
    #[test]
    fn crowd_audio_instance_add_source_matches() {
        oc807_install_fakes();
        covered(
            ee::s08::f_002a5d08__generated,
            crowd_audio_instance_add_source,
            240,
            &|c, seed| {
                let mut r = oc807_fixture(c, seed, 0x002a_5d08, 2);
                let base = |i: u32| OC807_BASE + 0x400 * i;
                for i in 0..20 {
                    oc807_fill_small(c, &mut r, base(i), 0x100);
                }
                // The instance: voice slots at +0x20 (12 bytes, word 0 != 0 is
                // in use), the count at +0xd4, three entries of 0x30 at +0xe0
                // (id, ... refcount at +8).
                oc807_fill_small(c, &mut r, OC807_SELF, 0x80);
                let count = [0u32, 1, 2, 3, 5, 14, 15, 16, 0, 1][(seed % 10) as usize];
                c.bus.write32(OC807_SELF + 0xd4, count);
                let free = (seed / 10) % 15;
                for i in 0..15u32 {
                    let a = OC807_SELF + 0x20 + 12 * i;
                    if i == free {
                        c.bus.write32(a, 0);
                        c.bus.write32(a + 8, 0);
                    } else {
                        c.bus.write32(a, 1 + r.next_u32() % 3);
                    }
                }
                for i in 0..3u32 {
                    let e = OC807_SELF + 0xe0 + 0x30 * i;
                    c.bus.write32(e, 1 + r.next_u32() % 8);
                    let used = r.next_u32() % 2;
                    c.bus.write32(e + 8, used);
                }
                // The source descriptor: index at +0x1c, sample pointers at
                // +0x24, three tables at +0x38/+0x3c/+0x64.
                oc807_fill_small(c, &mut r, OC807_DESC, 0x40);
                let idx = seed % 4;
                c.bus.write32(OC807_DESC + 0x1c, idx);
                for i in 0..4u32 {
                    let p = if i == idx && seed % 9 == 0 {
                        0
                    } else {
                        base(17) + 0x10 * (r.next_u32() % 8)
                    };
                    c.bus.write32(OC807_DESC + 0x24 + 4 * i, p);
                }
                c.bus.write32(OC807_DESC + 0x38, base(14));
                c.bus.write32(OC807_DESC + 0x3c, base(15));
                c.bus.write32(OC807_DESC + 0x64, base(16));
                // The shared crowd state: `gp+0x608` -> {G, table, table}.
                oc807_fill_small(c, &mut r, OC807_G, 0xc0);
                c.bus.write32(OC807_G + 0x1f4, seed % 3);
                for (i, off) in [
                    0x1f8u32, 0x20c, 0x210, 0x214, 0x238, 0x23c, 0x240, 0x244, 0x248, 0x24c, 0x250,
                    0x254, 0x258, 0x25c,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(OC807_G + off, base(i as u32));
                }
                c.bus.write32(OC807_PG, OC807_G);
                c.bus.write32(OC807_PG + 4, base(18));
                c.bus.write32(OC807_PG + 8, base(19));
                c.bus.write32(GAME_GP + 0x608, OC807_PG);
                c.g[A0][0] = sx(OC807_SELF);
                c.g[A1][0] = sx(OC807_DESC);
                c.g[A2][0] = (1 + r.next_u32() % 8) as u64;
                c.pc = 0x002a_5d08;
            },
        );
    }

    /// `CrowdAudio_ParseConfig(self, ini)`: reads the crowd section of an ini
    /// file into the three 0x54-byte entries of `self`: per token (`ReadToken`)
    /// it compares the name against the printed keys (`sprintf` + `strcasecmp`)
    /// and reads sample files and floats, until the section ends. The seed
    /// varies the number of tokens and the comparison results.
    #[test]
    fn crowd_audio_parse_config_matches() {
        oc807_install_fakes();
        covered(
            ee::s08::f_002a7040__generated,
            crowd_audio_parse_config,
            240,
            &|c, seed| {
                let mut r = oc807_fixture(c, seed, 0x002a_7040, 30);
                let sections = 1 + seed % 4;
                let mut ends = vec![0u32; sections as usize];
                ends[sections as usize - 1] = 1;
                oc807_script(c, 0x002a_7f90, &ends);
                let cmp: Vec<u32> = (0..12)
                    .map(|_| (r.next_u32() % 3 == 0) as u32)
                    .chain([1])
                    .collect();
                oc807_script(c, 0x0041_aa88, &cmp);
                fill(c, &mut r, OC807_SELF, 0x100);
                c.g[A0][0] = sx(OC807_SELF);
                c.g[A1][0] = sx(OC807_SELF + 0x800);
                c.pc = 0x002a_7040;
            },
        );
    }
}
