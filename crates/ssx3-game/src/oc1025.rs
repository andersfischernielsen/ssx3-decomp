//! Staging module oc1025 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1025`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0028_e8c0,
    ssx_audio_system_handle_music_event,
    "SSXAudioSystem_HandleMusicEvent",
)];

/// `SSXAudioSystem_HandleMusicEvent(self, id, flag)`: reacts to a music
/// event of the audio system. Event 19 (race start): with the race music on
/// and the transport call set, starts the race music monitors or speech;
/// event 20 (race end/transport): resets the music state, destroys the
/// monitors, ends the tone/slide/tumble/uber-spin sounds and starts the
/// transport or track music; events 21..=25: updates the music tier and
/// picks a song; anything else is forwarded to the stream system's vtable.
/// The generated body, every register, cycle and call kept (labelled blocks).
pub fn ssx_audio_system_handle_music_event(c: &mut Cpu) {
    if c.pc != 0x0028e8c0 {
        return ee::s07::f_0028e8c0__generated(c);
    }
    guest(c, |c| {
        'b_28ef70: {
            'b_28ef6c: {
                'b_28ebb0: {
                    'b_28ebac: {
                        c.cycles += 10;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                        c.g[V0][0] = 0x13;
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[S3][0] = c.g[A0][0];
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        if t {
                            break 'b_28ebac;
                        } else {
                            c.cycles += 2;
                            let t = c.g[A2][0] == 0u64;
                            c.g[V0][0] = 0x14;
                            if t {
                                break 'b_28ebb0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x627c));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x14;
                                if t {
                                    break 'b_28ebb0;
                                } else {
                                    'b_28eba4: {
                                        'b_28eb64: {
                                            'b_28ea20: {
                                                'b_28ea18: {
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x0028e904);
                                                    call_ptr(c, 0x002a3fe8, 0x0028e904)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S3][0] as u32)
                                                            .wrapping_add(0x578c));
                                                    if t {
                                                        break 'b_28ea18;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] != 0u64;

                                                        if t {
                                                            break 'b_28ea20;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x0028e91c);
                                                            c.g[A0][0] = c.g[S3][0];
                                                            Callee { addr: 0x0028d960, f: ee::s07::SSXAudioSystem_IsGameMusicEnabled }.call(c, 0x0028e91c)?;
                                                            if c.g[V0][0] == 0u64 {
                                                                c.sw(
                                                                    (c.g[S3][0] as u32)
                                                                        .wrapping_add(0x6258),
                                                                    0u64,
                                                                );
                                                                break 'b_28ef6c;
                                                            } else {
                                                                c.cycles += 11;
                                                                c.g[RA][0] = sx(0x0028e92c);
                                                                call_ptr(
                                                                    c, 0x0014dc80, 0x0028e92c,
                                                                )?;
                                                                c.g[A0][0] = c.g[V0][0];
                                                                c.g[RA][0] = sx(0x0028e938);
                                                                c.g[A1][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x0014dd58, 0x0028e938,
                                                                )?;
                                                                c.g[V1][0] = 0x530000;
                                                                c.g[V0][0] = c
                                                                    .lb((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x5c10));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x5));
                                                                c.g[V0][0] =
                                                                    (c.g[V0][0] < 0x2u64) as u64;
                                                                if c.g[V0][0] != 0u64 {
                                                                    c.sw(
                                                                        (c.g[S3][0] as u32)
                                                                            .wrapping_add(0x6258),
                                                                        0u64,
                                                                    );
                                                                    break 'b_28ef6c;
                                                                } else {
                                                                    'b_28e9fc: {
                                                                        'b_28e9b0: {
                                                                            c.cycles += 3;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [S3][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x6258,
                                                                                ));
                                                                            if c.g[V0][0] != 0u64 {
                                                                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                                                                                break 'b_28e9b0;
                                                                            } else {
                                                                                c.cycles += 21;
                                                                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                                                                                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x118));
                                                                                c.g[S1][0] =
                                                                                    0x4a0000;
                                                                                c.g[A0][0] =
                                                                                    c.g[V0][0];
                                                                                c.g[S0][0] = c.lw(
                                                                                    (c.g[V1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3650));
                                                                                c.g[S4][0] =
                                                                                    c.g[V0][0];
                                                                                c.g[RA][0] =
                                                                                    sx(0x0028e980);
                                                                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                                                                                call_ptr(
                                                                                    c, 0x002b4908,
                                                                                    0x0028e980,
                                                                                )?;
                                                                                c.ldl(6, (c.g[S1][0] as u32).wrapping_add(0x7));
                                                                                c.ldr(
                                                                                    6,
                                                                                    (c.g[S1][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[A0][0] =
                                                                                    c.g[S0][0];
                                                                                c.g[T1][0] =
                                                                                    c.g[V0][0];
                                                                                c.g[A1][0] = 0u64;
                                                                                c.g[A3][0] = 0u64;
                                                                                c.g[T0][0] = 0x4;
                                                                                c.g[T2][0] = 0u64;
                                                                                c.g[RA][0] =
                                                                                    sx(0x0028e9a8);
                                                                                c.g[T3][0] = 0u64;
                                                                                call_ptr(
                                                                                    c, 0x002adca0,
                                                                                    0x0028e9a8,
                                                                                )?;
                                                                                let t = true;
                                                                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5558));
                                                                                if t {
                                                                                    break 'b_28e9fc;
                                                                                } else {
                                                                                    break 'b_28e9b0;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 19;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[S3][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x118,
                                                                                ));
                                                                        c.g[S1][0] = 0x4a0000;
                                                                        c.g[A0][0] = c.g[V1][0];
                                                                        c.g[S0][0] =
                                                                            c.lw(
                                                                                (c.g[V0][0] as u32)
                                                                            );
                                                                        c.g[S1][0] =
                                                                            sx((c.g[S1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x3650,
                                                                                ));
                                                                        c.g[S4][0] = c.g[V1][0];
                                                                        c.g[RA][0] = sx(0x0028e9d0);
                                                                        c.g[S0][0] =
                                                                            sx((c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1d8,
                                                                                ));
                                                                        call_ptr(
                                                                            c, 0x002b4908,
                                                                            0x0028e9d0,
                                                                        )?;
                                                                        c.ldl(
                                                                            6,
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x7),
                                                                        );
                                                                        c.ldr(
                                                                            6,
                                                                            (c.g[S1][0] as u32),
                                                                        );
                                                                        c.g[A0][0] = c.g[S0][0];
                                                                        c.g[T1][0] = c.g[V0][0];
                                                                        c.g[A1][0] = 0u64;
                                                                        c.g[A3][0] = 0u64;
                                                                        c.g[T0][0] = 0x5;
                                                                        c.g[T2][0] = 0u64;
                                                                        c.g[RA][0] = sx(0x0028e9f8);
                                                                        c.g[T3][0] = 0u64;
                                                                        call_ptr(
                                                                            c, 0x002adca0,
                                                                            0x0028e9f8,
                                                                        )?;
                                                                        c.g[V1][0] = c.lw((c.g[S3]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x5558));
                                                                        break 'b_28e9fc;
                                                                    }
                                                                    c.cycles += 7;
                                                                    c.g[A1][0] = 0u64;
                                                                    c.g[A0][0] = c
                                                                        .lh((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x20));
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    let t = c.g[V0][0] as u32;
                                                                    c.g[RA][0] = sx(0x0028ea10);
                                                                    c.g[A0][0] = sx((c.g[S4][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[A0][0] as u32),
                                                                        ));
                                                                    call_ptr(c, t, 0x0028ea10)?;
                                                                    let t = true;
                                                                    c.sw(
                                                                        (c.g[S3][0] as u32)
                                                                            .wrapping_add(0x6258),
                                                                        0u64,
                                                                    );
                                                                    if t {
                                                                        break 'b_28ef6c;
                                                                    } else {
                                                                        break 'b_28ea18;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;

                                                if t {
                                                    break 'b_28eb64;
                                                } else {
                                                    break 'b_28ea20;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x0028ea28);
                                            c.g[A0][0] = c.g[S3][0];
                                            Callee {
                                                addr: 0x0028d960,
                                                f: ee::s07::SSXAudioSystem_IsGameMusicEnabled,
                                            }
                                            .call(c, 0x0028ea28)?;
                                            if c.g[V0][0] == 0u64 {
                                                c.sw(
                                                    (c.g[S3][0] as u32).wrapping_add(0x6258),
                                                    0u64,
                                                );
                                                break 'b_28ef6c;
                                            } else {
                                                'b_28ea78: {
                                                    c.cycles += 14;
                                                    c.g[RA][0] = sx(0x0028ea38);
                                                    c.g[S1][0] = 0x1;
                                                    Callee {
                                                        addr: 0x0028b1e8,
                                                        f: ee::s07::SSXAudioSystem_GetBELibrary,
                                                    }
                                                    .call(c, 0x0028ea38)?;
                                                    c.g[A1][0] = 0u64;
                                                    c.g[RA][0] = sx(0x0028ea44);
                                                    c.g[A0][0] = c.g[V0][0];
                                                    call_ptr(c, 0x0014dd58, 0x0028ea44)?;
                                                    c.g[RA][0] = sx(0x0028ea4c);
                                                    c.g[A0][0] = c.g[V0][0];
                                                    call_ptr(c, 0x00144bc0, 0x0028ea4c)?;
                                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                                    c.g[RA][0] = sx(0x0028ea58);
                                                    c.g[A0][0] = c.g[S3][0];
                                                    call_ptr(c, 0x002a1e20, 0x0028ea58)?;
                                                    c.g[S0][0] = c.g[V0][0];
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S3][0] as u32)
                                                            .wrapping_add(0x62b8));
                                                    let t = c.g[V0][0] != 0u64;

                                                    if t {
                                                        break 'b_28ea78;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[AT][0] = 0x40000000;
                                                        c.mtc1(1, 12);
                                                        c.g[RA][0] = sx(0x0028ea78);
                                                        c.g[A0][0] =
                                                            sx((c.g[S3][0] as u32)
                                                                .wrapping_add(0x118));
                                                        call_ptr(c, 0x002b3d48, 0x0028ea78)?;
                                                        break 'b_28ea78;
                                                    }
                                                }
                                                'b_28eb48: {
                                                    'b_28eae4: {
                                                        c.cycles += 2;
                                                        let t = c.g[S0][0] != c.g[S1][0];
                                                        c.g[A0][0] = c.g[S3][0];
                                                        if t {
                                                            break 'b_28eae4;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x0028ea88);
                                                            c.g[A1][0] = 0xb;
                                                            Callee { addr: 0x0028def0, f: ee::s07::SSXAudioSystem_SetMusicState }.call(c, 0x0028ea88)?;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V0][0] = 0x4a0000;
                                                            if t {
                                                                break 'b_28eba4;
                                                            } else {
                                                                c.cycles += 21;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x118));
                                                                c.ldl(
                                                                    6,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x364f),
                                                                );
                                                                c.ldr(
                                                                    6,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x3648),
                                                                );
                                                                c.g[A1][0] = 0xbb8;
                                                                c.g[A0][0] =
                                                                    c.lw((c.g[V1][0] as u32));
                                                                c.g[A3][0] = 0u64;
                                                                c.g[T0][0] = 0u64;
                                                                c.g[T1][0] = 0x1;
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1d8));
                                                                c.g[T2][0] = 0u64;
                                                                c.g[RA][0] = sx(0x0028eac0);
                                                                c.g[T3][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x002adca0, 0x0028eac0,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x118));
                                                                c.g[V0][0] = 0x4a0000;
                                                                c.ldl(
                                                                    6,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x3657),
                                                                );
                                                                c.ldr(
                                                                    6,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x3650),
                                                                );
                                                                c.g[A1][0] = 0x9c4;
                                                                c.g[A0][0] =
                                                                    c.lw((c.g[V1][0] as u32));
                                                                c.g[A3][0] = 0u64;
                                                                let t = true;
                                                                c.g[T0][0] = 0u64;
                                                                if t {
                                                                    break 'b_28eb48;
                                                                } else {
                                                                    break 'b_28eae4;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 25;
                                                    c.g[RA][0] = sx(0x0028eaec);
                                                    c.g[A0][0] = c.g[S3][0];
                                                    Callee {
                                                        addr: 0x0028df08,
                                                        f: ee::s07::SSXAudioSystem_ResetMusicState,
                                                    }
                                                    .call(c, 0x0028eaec)?;
                                                    c.g[RA][0] = sx(0x0028eaf4);
                                                    c.g[A0][0] = c.g[S3][0];
                                                    Callee {
                                                        addr: 0x0028d488,
                                                        f: ee::s07::SSXAudioSystem_SelectRandomSong,
                                                    }
                                                    .call(c, 0x0028eaf4)?;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x118)
                                                        );
                                                    c.g[V0][0] = 0x4a0000;
                                                    c.ldl(
                                                        6,
                                                        (c.g[V0][0] as u32).wrapping_add(0x364f),
                                                    );
                                                    c.ldr(
                                                        6,
                                                        (c.g[V0][0] as u32).wrapping_add(0x3648),
                                                    );
                                                    c.g[A1][0] = 0x7d0;
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                    c.g[A3][0] = 0u64;
                                                    c.g[T0][0] = 0x1;
                                                    c.g[T1][0] = 0x1;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                                    c.g[T2][0] = 0u64;
                                                    c.g[RA][0] = sx(0x0028eb28);
                                                    c.g[T3][0] = 0u64;
                                                    call_ptr(c, 0x002adca0, 0x0028eb28)?;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x118)
                                                        );
                                                    c.g[V0][0] = 0x4a0000;
                                                    c.ldl(
                                                        6,
                                                        (c.g[V0][0] as u32).wrapping_add(0x3657),
                                                    );
                                                    c.ldr(
                                                        6,
                                                        (c.g[V0][0] as u32).wrapping_add(0x3650),
                                                    );
                                                    c.g[A1][0] = 0x5dc;
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                    c.g[A3][0] = 0u64;
                                                    c.g[T0][0] = 0x7;
                                                    break 'b_28eb48;
                                                }
                                                c.cycles += 7;
                                                c.g[T1][0] = 0u64;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                                c.g[T2][0] = 0u64;
                                                c.g[RA][0] = sx(0x0028eb5c);
                                                c.g[T3][0] = 0u64;
                                                call_ptr(c, 0x002adca0, 0x0028eb5c)?;
                                                let t = true;
                                                c.sw(
                                                    (c.g[S3][0] as u32).wrapping_add(0x6258),
                                                    0u64,
                                                );
                                                if t {
                                                    break 'b_28ef6c;
                                                } else {
                                                    break 'b_28eb64;
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x0028eb6c);
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x002a3f90, 0x0028eb6c)?;
                                        if c.g[V0][0] == 0u64 {
                                            c.sw((c.g[S3][0] as u32).wrapping_add(0x6258), 0u64);
                                            break 'b_28ef6c;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x6258));
                                            if c.g[V0][0] != 0u64 {
                                                c.sw(
                                                    (c.g[S3][0] as u32).wrapping_add(0x6258),
                                                    0u64,
                                                );
                                                break 'b_28ef6c;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A0][0] = c.g[S3][0];
                                                c.g[A1][0] = 0u64;
                                                c.g[RA][0] = sx(0x0028eb90);
                                                c.g[A2][0] = 0x1;
                                                call_ptr(c, 0x002a26f0, 0x0028eb90)?;
                                                c.g[A0][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x5560));
                                                c.g[RA][0] = sx(0x0028eb9c);
                                                c.g[A1][0] = 0u64;
                                                call_ptr(c, 0x002b1758, 0x0028eb9c)?;
                                                c.g[V0][0] = 0x1;
                                                c.sw(
                                                    (c.g[S3][0] as u32).wrapping_add(0x574c),
                                                    c.g[V0][0],
                                                );
                                                break 'b_28eba4;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x6258), 0u64);
                                    if t {
                                        break 'b_28ef6c;
                                    } else {
                                        break 'b_28ebac;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x14;
                    break 'b_28ebb0;
                }
                'b_28eee8: {
                    c.cycles += 2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x15));
                    if t {
                        break 'b_28eee8;
                    } else {
                        c.cycles += 2;
                        let t = c.g[A2][0] == 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_28ef6c;
                        } else {
                            'b_28eee0: {
                                'b_28eebc: {
                                    c.cycles += 56;
                                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6284));
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x6258), 0u64);
                                    c.g[A0][0] = c.g[S3][0];
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x577c), 0u64);
                                    c.g[S0][0] = 0x4a0000;
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x5814), c.g[V1][0]);
                                    c.g[S2][0] = c.g[S0][0];
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x6254), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x0028ebe8);
                                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x118));
                                    Callee {
                                        addr: 0x0028df08,
                                        f: ee::s07::SSXAudioSystem_ResetMusicState,
                                    }
                                    .call(c, 0x0028ebe8)?;
                                    c.g[S4][0] = c.g[S1][0];
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x578c), 0u64);
                                    c.g[RA][0] = sx(0x0028ebf8);
                                    c.g[A0][0] = c.g[S3][0];
                                    call_ptr(c, 0x002a4550, 0x0028ebf8)?;
                                    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x364f));
                                    c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x3648));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[RA][0] = sx(0x0028ec18);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                    call_ptr(c, 0x002adde0, 0x0028ec18)?;
                                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                                    c.g[V0][0] = 0x4a0000;
                                    c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x3657));
                                    c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x3650));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    c.g[RA][0] = sx(0x0028ec34);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                    call_ptr(c, 0x002adde0, 0x0028ec34)?;
                                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                                    c.g[V0][0] = 0x4a0000;
                                    c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x365f));
                                    c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x3658));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    c.g[RA][0] = sx(0x0028ec50);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                    call_ptr(c, 0x002adde0, 0x0028ec50)?;
                                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x45c8));
                                    c.g[RA][0] = sx(0x0028ec5c);
                                    c.g[A0][0] = c.g[S3][0];
                                    call_ptr(c, 0x0029d678, 0x0028ec5c)?;
                                    c.g[RA][0] = sx(0x0028ec64);
                                    Callee {
                                        addr: 0x0028b1d8,
                                        f: ee::s07::SSXAudioSystem_GetRaceMan,
                                    }
                                    .call(c, 0x0028ec64)?;
                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                    c.g[RA][0] = sx(0x0028ec70);
                                    c.g[A0][0] = c.g[S3][0];
                                    call_ptr(c, 0x00296e20, 0x0028ec70)?;
                                    c.g[RA][0] = sx(0x0028ec78);
                                    Callee {
                                        addr: 0x0028b1d8,
                                        f: ee::s07::SSXAudioSystem_GetRaceMan,
                                    }
                                    .call(c, 0x0028ec78)?;
                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                    c.g[RA][0] = sx(0x0028ec84);
                                    c.g[A0][0] = c.g[S3][0];
                                    call_ptr(c, 0x00297438, 0x0028ec84)?;
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[RA][0] = sx(0x0028ec90);
                                    c.g[A1][0] = 0u64;
                                    call_ptr(c, 0x0029b3c0, 0x0028ec90)?;
                                    c.g[RA][0] = sx(0x0028ec98);
                                    c.g[A0][0] = c.g[S3][0];
                                    Callee {
                                        addr: 0x0028d960,
                                        f: ee::s07::SSXAudioSystem_IsGameMusicEnabled,
                                    }
                                    .call(c, 0x0028ec98)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6284));
                                        break 'b_28eebc;
                                    } else {
                                        'b_28ed24: {
                                            c.cycles += 5;
                                            c.g[S1][0] = 0x1;
                                            c.g[RA][0] = sx(0x0028ecac);
                                            c.g[A0][0] = c.g[S3][0];
                                            call_ptr(c, 0x002a4040, 0x0028ecac)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x6284));
                                                break 'b_28ed24;
                                            } else {
                                                'b_28ed20: {
                                                    'b_28ecd4: {
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x0028ecbc);
                                                        c.g[A0][0] = c.g[S3][0];
                                                        call_ptr(c, 0x002a4078, 0x0028ecbc)?;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        if t {
                                                            break 'b_28ecd4;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x0028eccc);
                                                            c.g[A0][0] = c.g[S3][0];
                                                            call_ptr(c, 0x002a40e8, 0x0028eccc)?;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                            if t {
                                                                break 'b_28ed20;
                                                            } else {
                                                                break 'b_28ecd4;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                    c.g[V1][0] =
                                                        ((c.g[V1][0] as i64) < 2i64) as u64;
                                                    if c.g[V1][0] != 0u64 {
                                                        c.g[A1][0] = c.lw((c.g[S3][0] as u32)
                                                            .wrapping_add(0x6284));
                                                        break 'b_28ed24;
                                                    } else {
                                                        c.cycles += 14;
                                                        c.g[A0][0] = c.g[S3][0];
                                                        c.g[RA][0] = sx(0x0028ecf4);
                                                        c.g[S1][0] = 0u64;
                                                        Callee { addr: 0x0028d488, f: ee::s07::SSXAudioSystem_SelectRandomSong }.call(c, 0x0028ecf4)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x118));
                                                        c.g[A2][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0xa;
                                                        c.g[A3][0] = 0u64;
                                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                        c.g[T0][0] = 0x3;
                                                        c.g[T1][0] = 0x1;
                                                        c.g[T2][0] = 0u64;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1d8));
                                                        c.g[RA][0] = sx(0x0028ed20);
                                                        c.g[T3][0] = 0u64;
                                                        call_ptr(c, 0x002adca0, 0x0028ed20)?;
                                                        break 'b_28ed20;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x6284));
                                                break 'b_28ed24;
                                            }
                                        }
                                        'b_28edf0: {
                                            'b_28ed58: {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x17;
                                                let t = c.g[A1][0] == c.g[V0][0];

                                                if t {
                                                    break 'b_28ed58;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x0028ed38);
                                                    c.g[A0][0] = c.g[S3][0];
                                                    call_ptr(c, 0x002a4030, 0x0028ed38)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A0][0] = 0x1;
                                                    if t {
                                                        break 'b_28ed58;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32)
                                                            .wrapping_add(0x6284));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[S3][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                            .wrapping_add(0x579c));
                                                        let t = c.g[V1][0] == c.g[A0][0];

                                                        if t {
                                                            break 'b_28edf0;
                                                        } else {
                                                            break 'b_28ed58;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_28eda4: {
                                                c.cycles += 2;
                                                let t = c.g[S1][0] == 0u64;
                                                c.g[V0][0] = 0x17;
                                                if t {
                                                    break 'b_28eda4;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S3][0] as u32)
                                                            .wrapping_add(0x6284));
                                                    let t = c.g[V1][0] == c.g[V0][0];

                                                    if t {
                                                        break 'b_28eda4;
                                                    } else {
                                                        c.cycles += 14;
                                                        c.g[RA][0] = sx(0x0028ed74);
                                                        c.g[A0][0] = c.g[S3][0];
                                                        Callee { addr: 0x0028d488, f: ee::s07::SSXAudioSystem_SelectRandomSong }.call(c, 0x0028ed74)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x118));
                                                        c.g[A1][0] = 0xa;
                                                        c.ldl(
                                                            6,
                                                            (c.g[S2][0] as u32)
                                                                .wrapping_add(0x364f),
                                                        );
                                                        c.ldr(
                                                            6,
                                                            (c.g[S2][0] as u32)
                                                                .wrapping_add(0x3648),
                                                        );
                                                        c.g[A3][0] = 0u64;
                                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                        c.g[T0][0] = 0x2;
                                                        c.g[T1][0] = 0x1;
                                                        c.g[T2][0] = 0u64;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1d8));
                                                        c.g[RA][0] = sx(0x0028eda4);
                                                        c.g[T3][0] = 0u64;
                                                        call_ptr(c, 0x002adca0, 0x0028eda4)?;
                                                        break 'b_28eda4;
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x0028edac);
                                            c.g[A0][0] = c.g[S3][0];
                                            call_ptr(c, 0x002a4718, 0x0028edac)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_28eee0;
                                            } else {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x0028edbc);
                                                c.g[A0][0] = c.g[S3][0];
                                                call_ptr(c, 0x002a4a38, 0x0028edbc)?;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] = 0x1;
                                                if t {
                                                    break 'b_28eee0;
                                                } else {
                                                    c.cycles += 11;
                                                    c.g[A0][0] = c.g[S3][0];
                                                    c.g[A1][0] = 0u64;
                                                    c.g[RA][0] = sx(0x0028edd4);
                                                    c.g[A2][0] = 0x1;
                                                    call_ptr(c, 0x002a26f0, 0x0028edd4)?;
                                                    c.g[A0][0] =
                                                        sx((c.g[S3][0] as u32)
                                                            .wrapping_add(0x5560));
                                                    c.g[RA][0] = sx(0x0028ede0);
                                                    c.g[A1][0] = 0u64;
                                                    call_ptr(c, 0x002b1758, 0x0028ede0)?;
                                                    c.g[V0][0] = 0x1;
                                                    c.sw(
                                                        (c.g[S3][0] as u32).wrapping_add(0x6258),
                                                        c.g[V0][0],
                                                    );
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[S3][0] as u32).wrapping_add(0x5754),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_28eee0;
                                                    } else {
                                                        break 'b_28edf0;
                                                    }
                                                }
                                            }
                                        }
                                        'b_28ee10: {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x0028edf8);
                                            c.g[A0][0] = c.g[S3][0];
                                            call_ptr(c, 0x002a4a38, 0x0028edf8)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S2][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x5558));
                                                break 'b_28ee10;
                                            } else {
                                                c.cycles += 4;
                                                c.g[A0][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x5560));
                                                c.g[RA][0] = sx(0x0028ee0c);
                                                c.g[A1][0] = 0u64;
                                                call_ptr(c, 0x002b11b0, 0x0028ee0c)?;
                                                c.g[S2][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x5558));
                                                break 'b_28ee10;
                                            }
                                        }
                                        'b_28eea4: {
                                            'b_28ee8c: {
                                                c.cycles += 24;
                                                c.g[S1][0] = 0x4a0000;
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x3628));
                                                c.g[A0][0] = c.g[S3][0];
                                                c.g[S0][0] =
                                                    c.lh((c.g[S2][0] as u32).wrapping_add(0x10));
                                                c.g[A1][0] = 0x1;
                                                c.g[A2][0] = 0u64;
                                                c.g[RA][0] = sx(0x0028ee30);
                                                c.g[S0][0] = sx((c.g[S4][0] as u32)
                                                    .wrapping_add((c.g[S0][0] as u32)));
                                                Callee {
                                                    addr: 0x00287968,
                                                    f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
                                                }
                                                .call(c, 0x0028ee30)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                                c.g[A3][0] = c.g[V0][0];
                                                c.g[A2][0] = 0xc;
                                                c.g[T0][0] = 0x1;
                                                c.g[A0][0] = c.g[S0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x0028ee4c);
                                                c.g[A1][0] = c.g[S1][0];
                                                call_ptr(c, t, 0x0028ee4c)?;
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[RA][0] = sx(0x0028ee58);
                                                c.g[A1][0] = 0x191;
                                                call_ptr(c, 0x002b49c0, 0x0028ee58)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x6284));
                                                c.g[RA][0] = sx(0x0028ee64);
                                                c.g[A0][0] = c.g[S3][0];
                                                call_ptr(c, 0x002a1e20, 0x0028ee64)?;
                                                c.g[V1][0] = 0x2;
                                                let t = c.g[V0][0] != c.g[V1][0];
                                                c.g[V1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x5558));
                                                if t {
                                                    break 'b_28ee8c;
                                                } else {
                                                    c.cycles += 7;
                                                    c.g[A1][0] = 0x2;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    c.g[V0][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[RA][0] = sx(0x0028ee84);
                                                    c.g[A0][0] = sx((c.g[S4][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x0028ee84)?;
                                                    let t = true;
                                                    c.g[A0][0] = c.g[S4][0];
                                                    if t {
                                                        break 'b_28eea4;
                                                    } else {
                                                        break 'b_28ee8c;
                                                    }
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[A1][0] = 0x3;
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x0028eea0);
                                            c.g[A0][0] = sx((c.g[S4][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x0028eea0)?;
                                            c.g[A0][0] = c.g[S4][0];
                                            break 'b_28eea4;
                                        }
                                        c.cycles += 6;
                                        c.g[RA][0] = sx(0x0028eeac);
                                        c.g[A1][0] = 0x7f;
                                        call_ptr(c, 0x002b3c28, 0x0028eeac)?;
                                        c.g[RA][0] = sx(0x0028eeb4);
                                        c.g[A0][0] = c.g[S4][0];
                                        call_ptr(c, 0x002b3a70, 0x0028eeb4)?;
                                        let t = true;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_28eee0;
                                        } else {
                                            break 'b_28eebc;
                                        }
                                    }
                                }
                                c.cycles += 9;
                                c.g[RA][0] = sx(0x0028eec4);
                                c.g[A0][0] = c.g[S3][0];
                                Callee {
                                    addr: 0x0028d5a0,
                                    f: ee::s07::SSXAudioSystem_UpdateMusicTier,
                                }
                                .call(c, 0x0028eec4)?;
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5558));
                                c.g[A1][0] = 0x1e;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x0028eedc);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x0028eedc)?;
                                c.g[V0][0] = 0x1;
                                break 'b_28eee0;
                            }
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[S3][0] as u32).wrapping_add(0x627c), c.g[V0][0]);
                            if t {
                                break 'b_28ef6c;
                            } else {
                                break 'b_28eee8;
                            }
                        }
                    }
                }
                'b_28ef58: {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x5u64) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5558));
                        break 'b_28ef58;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0028eefc);
                        c.g[A0][0] = c.g[S3][0];
                        Callee {
                            addr: 0x0028d960,
                            f: ee::s07::SSXAudioSystem_IsGameMusicEnabled,
                        }
                        .call(c, 0x0028eefc)?;
                        let t = c.g[V0][0] == 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        if t {
                            break 'b_28ef70;
                        } else {
                            c.cycles += 21;
                            c.g[RA][0] = sx(0x0028ef0c);
                            c.g[A0][0] = c.g[S3][0];
                            Callee {
                                addr: 0x0028d488,
                                f: ee::s07::SSXAudioSystem_SelectRandomSong,
                            }
                            .call(c, 0x0028ef0c)?;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 12);
                            c.g[RA][0] = sx(0x0028ef1c);
                            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x118));
                            call_ptr(c, 0x002b3d48, 0x0028ef1c)?;
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                            c.g[V1][0] = 0x4a0000;
                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x364f));
                            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x3648));
                            c.g[A1][0] = 0xbb8;
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                            c.g[A3][0] = 0u64;
                            c.g[T0][0] = 0x3;
                            c.g[T1][0] = 0x1;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                            c.g[T2][0] = 0u64;
                            c.g[RA][0] = sx(0x0028ef50);
                            c.g[T3][0] = 0u64;
                            call_ptr(c, 0x002adca0, 0x0028ef50)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                            if t {
                                break 'b_28ef70;
                            } else {
                                break 'b_28ef58;
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x118));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0028ef6c);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0028ef6c)?;
                break 'b_28ef6c;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_28ef70;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// Test-only layout of the audio system (`self`).
#[cfg(test)]
pub(crate) const OC1025_TRACK_OFF: u32 = 0x6284;
#[cfg(test)]
pub(crate) const OC1025_FIRST_TIME_OFF: u32 = 0x579c;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Fixture objects live in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).

    /// Gate word of this module's fakes (their forced results sit at
    /// `CTRL + 0x100 + 4 * n`), and its magic value.
    pub(crate) const OC1025_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC1025_MAGIC: u32 = 0x4f43_1025;
    /// The audio system (`self`), its manager, the sub-object and the vtable object.
    pub(crate) const OC1025_SELF: u32 = DATA + 0x21_0000;
    pub(crate) const OC1025_MGR: u32 = DATA + 0x23_0000;
    pub(crate) const OC1025_OBJ: u32 = DATA + 0x24_0000;
    pub(crate) const OC1025_VTOBJ: u32 = DATA + 0x25_0000;
    pub(crate) const OC1025_WORLD: u32 = DATA + 0x26_0000;
    pub(crate) const OC1025_MODE: u32 = DATA + 0x26_1000;

    /// The fake callee: logs the call like `fakes::fake_call`, then, when the
    /// mailbox of call `mail` holds `m != 0`, returns `m - 1` in v0 (0, 1, 2).
    pub(crate) fn oc1025_fake_call(c: &mut Cpu, addr: u32, mail: u32) {
        let m = c.bus.read32(OC1025_CTRL + 0x100 + 4 * mail);
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if m != 0 && c.pc == ra {
            c.g[V0][0] = (m - 1) as u64;
        }
    }

    macro_rules! oc1025_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $mail:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1025_CTRL) != OC1025_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1025_fake_call(c, $addr, $mail);
                }
            )*
            fn oc1025_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1025_fakes!(
        (
            oc1025_fake_2a3fe8,
            OC1025_PREV_2A3FE8,
            ee::s08::OVERRIDE_002A3FE8,
            ee::s08::f_002a3fe8__generated,
            0x002a_3fe8,
            0
        ),
        (
            oc1025_fake_28d960,
            OC1025_PREV_28D960,
            ee::s07::OVERRIDE_0028D960,
            ee::s07::f_0028d960__generated,
            0x0028_d960,
            1
        ),
        (
            oc1025_fake_14dc80,
            OC1025_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            2
        ),
        (
            oc1025_fake_14dd58,
            OC1025_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            3
        ),
        (
            oc1025_fake_2b4908,
            OC1025_PREV_2B4908,
            ee::s08::OVERRIDE_002B4908,
            ee::s08::f_002b4908__generated,
            0x002b_4908,
            4
        ),
        (
            oc1025_fake_2adca0,
            OC1025_PREV_2ADCA0,
            ee::s08::OVERRIDE_002ADCA0,
            ee::s08::f_002adca0__generated,
            0x002a_dca0,
            5
        ),
        (
            oc1025_fake_28b1e8,
            OC1025_PREV_28B1E8,
            ee::s07::OVERRIDE_0028B1E8,
            ee::s07::f_0028b1e8__generated,
            0x0028_b1e8,
            6
        ),
        (
            oc1025_fake_144bc0,
            OC1025_PREV_144BC0,
            ee::s01::OVERRIDE_00144BC0,
            ee::s01::f_00144bc0__generated,
            0x0014_4bc0,
            7
        ),
        (
            oc1025_fake_2a1e20,
            OC1025_PREV_2A1E20,
            ee::s08::OVERRIDE_002A1E20,
            ee::s08::f_002a1e20__generated,
            0x002a_1e20,
            8
        ),
        (
            oc1025_fake_2b3d48,
            OC1025_PREV_2B3D48,
            ee::s08::OVERRIDE_002B3D48,
            ee::s08::f_002b3d48__generated,
            0x002b_3d48,
            9
        ),
        (
            oc1025_fake_28def0,
            OC1025_PREV_28DEF0,
            ee::s07::OVERRIDE_0028DEF0,
            ee::s07::f_0028def0__generated,
            0x0028_def0,
            10
        ),
        (
            oc1025_fake_28df08,
            OC1025_PREV_28DF08,
            ee::s07::OVERRIDE_0028DF08,
            ee::s07::f_0028df08__generated,
            0x0028_df08,
            11
        ),
        (
            oc1025_fake_28d488,
            OC1025_PREV_28D488,
            ee::s07::OVERRIDE_0028D488,
            ee::s07::f_0028d488__generated,
            0x0028_d488,
            12
        ),
        (
            oc1025_fake_2a3f90,
            OC1025_PREV_2A3F90,
            ee::s08::OVERRIDE_002A3F90,
            ee::s08::f_002a3f90__generated,
            0x002a_3f90,
            13
        ),
        (
            oc1025_fake_2a26f0,
            OC1025_PREV_2A26F0,
            ee::s08::OVERRIDE_002A26F0,
            ee::s08::f_002a26f0__generated,
            0x002a_26f0,
            14
        ),
        (
            oc1025_fake_2b1758,
            OC1025_PREV_2B1758,
            ee::s08::OVERRIDE_002B1758,
            ee::s08::f_002b1758__generated,
            0x002b_1758,
            15
        ),
        (
            oc1025_fake_2a4550,
            OC1025_PREV_2A4550,
            ee::s08::OVERRIDE_002A4550,
            ee::s08::f_002a4550__generated,
            0x002a_4550,
            16
        ),
        (
            oc1025_fake_2adde0,
            OC1025_PREV_2ADDE0,
            ee::s08::OVERRIDE_002ADDE0,
            ee::s08::f_002adde0__generated,
            0x002a_dde0,
            17
        ),
        (
            oc1025_fake_29d678,
            OC1025_PREV_29D678,
            ee::s08::OVERRIDE_0029D678,
            ee::s08::f_0029d678__generated,
            0x0029_d678,
            18
        ),
        (
            oc1025_fake_28b1d8,
            OC1025_PREV_28B1D8,
            ee::s07::OVERRIDE_0028B1D8,
            ee::s07::f_0028b1d8__generated,
            0x0028_b1d8,
            19
        ),
        (
            oc1025_fake_296e20,
            OC1025_PREV_296E20,
            ee::s08::OVERRIDE_00296E20,
            ee::s08::f_00296e20__generated,
            0x0029_6e20,
            20
        ),
        (
            oc1025_fake_297438,
            OC1025_PREV_297438,
            ee::s08::OVERRIDE_00297438,
            ee::s08::f_00297438__generated,
            0x0029_7438,
            21
        ),
        (
            oc1025_fake_29b3c0,
            OC1025_PREV_29B3C0,
            ee::s08::OVERRIDE_0029B3C0,
            ee::s08::f_0029b3c0__generated,
            0x0029_b3c0,
            22
        ),
        (
            oc1025_fake_2a4040,
            OC1025_PREV_2A4040,
            ee::s08::OVERRIDE_002A4040,
            ee::s08::f_002a4040__generated,
            0x002a_4040,
            23
        ),
        (
            oc1025_fake_2a4078,
            OC1025_PREV_2A4078,
            ee::s08::OVERRIDE_002A4078,
            ee::s08::f_002a4078__generated,
            0x002a_4078,
            24
        ),
        (
            oc1025_fake_2a40e8,
            OC1025_PREV_2A40E8,
            ee::s08::OVERRIDE_002A40E8,
            ee::s08::f_002a40e8__generated,
            0x002a_40e8,
            25
        ),
        (
            oc1025_fake_2a4718,
            OC1025_PREV_2A4718,
            ee::s08::OVERRIDE_002A4718,
            ee::s08::f_002a4718__generated,
            0x002a_4718,
            26
        ),
        (
            oc1025_fake_2a4a38,
            OC1025_PREV_2A4A38,
            ee::s08::OVERRIDE_002A4A38,
            ee::s08::f_002a4a38__generated,
            0x002a_4a38,
            27
        ),
        (
            oc1025_fake_2b11b0,
            OC1025_PREV_2B11B0,
            ee::s08::OVERRIDE_002B11B0,
            ee::s08::f_002b11b0__generated,
            0x002b_11b0,
            28
        ),
        (
            oc1025_fake_287968,
            OC1025_PREV_287968,
            ee::s07::OVERRIDE_00287968,
            ee::s07::f_00287968__generated,
            0x0028_7968,
            29
        ),
        (
            oc1025_fake_2b49c0,
            OC1025_PREV_2B49C0,
            ee::s08::OVERRIDE_002B49C0,
            ee::s08::f_002b49c0__generated,
            0x002b_49c0,
            30
        ),
        (
            oc1025_fake_2b3c28,
            OC1025_PREV_2B3C28,
            ee::s08::OVERRIDE_002B3C28,
            ee::s08::f_002b3c28__generated,
            0x002b_3c28,
            31
        ),
        (
            oc1025_fake_2b3a70,
            OC1025_PREV_2B3A70,
            ee::s08::OVERRIDE_002B3A70,
            ee::s08::f_002b3a70__generated,
            0x002b_3a70,
            32
        ),
        (
            oc1025_fake_28d5a0,
            OC1025_PREV_28D5A0,
            ee::s07::OVERRIDE_0028D5A0,
            ee::s07::f_0028d5a0__generated,
            0x0028_d5a0,
            33
        ),
        (
            oc1025_fake_2a4030,
            OC1025_PREV_2A4030,
            ee::s08::OVERRIDE_002A4030,
            ee::s08::f_002a4030__generated,
            0x002a_4030,
            34
        ),
    );

    /// `SSXAudioSystem_HandleMusicEvent`: events 19 (race music start), 20
    /// (race end / transport track), 21..=25 (tier update) and others (vtable
    /// forward), the race-music and transport flags, the game mode byte, the
    /// track and its first-time table, and 0/1/2 results of every callee
    /// (all callees are gated fakes; some seeds unwind at a call).
    #[test]
    fn ssx_audio_system_handle_music_event_matches() {
        oc1025_install_fakes();
        let rets = [
            0x0028_e904,
            0x0028_e91c,
            0x0028_e92c,
            0x0028_e980,
            0x0028_e9d0,
            0x0028_ea10,
            0x0028_ea28,
            0x0028_ea38,
            0x0028_ea58,
            0x0028_eb6c,
            0x0028_eb90,
            0x0028_ebf8,
            0x0028_ec98,
            0x0028_ecac,
            0x0028_ecbc,
            0x0028_ecf4,
            0x0028_edac,
            0x0028_edbc,
            0x0028_edd4,
            0x0028_edf8,
            0x0028_ee0c,
            0x0028_ee30,
            0x0028_ee84,
            0x0028_eeb4,
            0x0028_eec4,
            0x0028_eefc,
            0x0028_ef0c,
            0x0028_ef6c,
        ];
        fakes::check_reaching(
            ee::s07::f_0028e8c0__generated,
            ssx_audio_system_handle_music_event,
            1500,
            &rets,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28e8c0, 24);
                c.bus.write32(OC1025_CTRL, OC1025_MAGIC);
                for n in 0..35u32 {
                    let m = r.next_u32() % 4;
                    c.bus.write32(OC1025_CTRL + 0x100 + 4 * n, m);
                }
                let s = OC1025_SELF;
                // Random self, then the fields the function branches on.
                for off in (0..0x6300u32).step_by(4) {
                    let v = r.next_u32() % 3;
                    c.bus.write32(s + off, v);
                }
                c.bus.write32(s + 0x118, OC1025_MGR);
                c.bus.write32(OC1025_MGR, OC1025_OBJ);
                c.bus.write32(s + 0x5558, OC1025_VTOBJ);
                c.bus
                    .write16(OC1025_VTOBJ + 0x20, (r.next_u32() % 8 * 4) as u16);
                c.bus.write32(OC1025_VTOBJ + 0x24, fakes::FAKE);
                let track = if r.next_u32() % 2 == 0 {
                    0x17
                } else {
                    r.next_u32() % 0x17
                };
                c.bus.write32(s + OC1025_TRACK_OFF, track);
                for t in 0..0x18u32 {
                    c.bus
                        .write32(s + OC1025_FIRST_TIME_OFF + 4 * t, r.next_u32() % 2);
                }
                for off in [0x627c, 0x578c, 0x6258, 0x62b8, 0x574c, 0x5754] {
                    let v = (r.next_u32() % 4 != 0) as u32 & (r.next_u32() % 2);
                    let v = if r.next_u32() % 2 == 0 { 1 } else { v };
                    c.bus.write32(s + off, v);
                }
                // The 0x627c flag mostly on (events 19 need it).
                c.bus.write32(s + 0x627c, (r.next_u32() % 4 != 0) as u32);
                // Game mode byte (5 and 6 are the challenge modes).
                let modes = [5u8, 6, 7, 0, 4, 5, 6, 9];
                let mode = modes[(r.next_u32() % 8) as usize];
                c.bus.write8(0x0053_5c10, mode);
                // SSXApp game-mode manager: *(*(gp - 0x848) + 0xc0) < 2 or not.
                c.bus.write32(GAME_GP - 0x848, OC1025_WORLD);
                c.bus.write32(OC1025_WORLD + 0xc0, OC1025_MODE);
                c.bus.write32(OC1025_MODE, r.next_u32() % 4);
                c.bus.write32(GAME_GP - 0x45c8, 5.0f32.to_bits());
                let ids = [
                    19u32, 19, 19, 19, 20, 20, 20, 20, 21, 22, 23, 24, 25, 0, 7, 26, 18, 30,
                ];
                let id = ids[(r.next_u32() % ids.len() as u32) as usize];
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(id);
                c.g[A2][0] = if r.next_u32() % 5 == 0 {
                    0
                } else {
                    sx(1 + r.next_u32() % 3)
                };
                c.pc = 0x0028_e8c0;
            },
        );
    }
}
