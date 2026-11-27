//! Staging module oc1028 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1028`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `void SSXAudioSystem::RiderSay2082(SSXAudioSystem* self, Rider* rider, int alt)`
/// (0x29ff80): the rider's "wipeout / recover" speech line for the followed
/// rider. Nothing is said unless speech is on, no replay plays, the rider has
/// not finished (`+0x470 < 0`), it is the followed rider when it is an AI or
/// not local, the speech mask exists and the listener hears it. The line
/// depends on `alt` and the track category (`TrackToCategory`: 1, 2, other):
/// 0x2082/0x209a/0x209c, or with `alt` 0x2090/0x209b/0x209d.
/// (A resume at any other `pc` runs the generated body.)
pub fn ssx_audio_system_rider_say2082(c: &mut Cpu) {
    if c.pc != 0x0029ff80 {
        return ee::s08::f_0029ff80__generated(c);
    }
    guest(c, |c| {
        'b_2a02b8: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x0029ffac);
            c.g[S4][0] = c.g[A2][0];
            Callee {
                addr: 0x0029f0a0,
                f: ee::s08::SSXAudioSystem_IsSpeechVolumeOn,
            }
            .call(c, 0x0029ffac)?;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_2a02b8;
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0029ffbc);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00288ae0, 0x0029ffbc)?;
                if c.g[V0][0] != 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    break 'b_2a02b8;
                } else {
                    'b_29ffe0: {
                        c.cycles += 6;
                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x470));
                        c.mtc1(0, 0);
                        c.c_le_s(0, 1);
                        let t = c.fcc();
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_29ffe0;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0u64;
                            break 'b_29ffe0;
                        }
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        break 'b_2a02b8;
                    } else {
                        'b_2a001c: {
                            c.cycles += 10;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c0));
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c0));
                            c.g[S3][0] = c.g[A1][0];
                            c.g[S2][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x002a0008);
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x002a0008)?;
                            if c.g[V0][0] != 0u64 {
                                c.g[S2][0] = 0x1;
                                break 'b_2a001c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x87c));
                                if c.g[V0][0] == 0u64 {
                                    c.g[S2][0] = 0x1;
                                    break 'b_2a001c;
                                } else {
                                    break 'b_2a001c;
                                }
                            }
                        }
                        'b_2a0038: {
                            c.cycles += 2;
                            let t = c.g[S2][0] == 0u64;

                            if t {
                                break 'b_2a0038;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x002a0030);
                                c.g[A1][0] = 0xffffffffffffffff;
                                call_ptr(c, 0x00285d98, 0x002a0030)?;
                                if c.g[V0][0] != c.g[S1][0] {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    break 'b_2a02b8;
                                } else {
                                    break 'b_2a0038;
                                }
                            }
                        }
                        c.cycles += 14;
                        c.g[RA][0] = sx(0x002a0040);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x0029f330,
                            f: ee::s08::SSXAudioSystem_GetRiderSpeechMask,
                        }
                        .call(c, 0x002a0040)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c0));
                        c.g[S2][0] = c.g[V0][0];
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x002a0058);
                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x002a0058)?;
                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_add(0x5e8));
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x002a0068);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x00288b40, 0x002a0068)?;
                        if (c.g[V0][0] as i64) < 0 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                            break 'b_2a02b8;
                        } else {
                            'b_2a02b4: {
                                'b_2a0260: {
                                    'b_2a023c: {
                                        'b_2a017c: {
                                            c.cycles += 2;
                                            let t = c.g[S4][0] == 0u64;

                                            if t {
                                                break 'b_2a017c;
                                            } else {
                                                'b_2a00f4: {
                                                    c.cycles += 14;
                                                    c.g[RA][0] = sx(0x002a0080);
                                                    call_ptr(c, 0x0028b1e8, 0x002a0080)?;
                                                    c.g[A1][0] = 0u64;
                                                    c.g[RA][0] = sx(0x002a008c);
                                                    c.g[A0][0] = c.g[V0][0];
                                                    call_ptr(c, 0x0014dd58, 0x002a008c)?;
                                                    c.g[RA][0] = sx(0x002a0094);
                                                    c.g[A0][0] = c.g[V0][0];
                                                    call_ptr(c, 0x00144bc0, 0x002a0094)?;
                                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                                    c.g[RA][0] = sx(0x002a00a0);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    Callee {
                                                        addr: 0x002a1e20,
                                                        f: ee::s08::SSXAudioSystem_TrackToCategory,
                                                    }
                                                    .call(c, 0x002a00a0)?;
                                                    c.g[V1][0] = c.g[V0][0];
                                                    c.g[V0][0] = 0x1;
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[V0][0] = 0x2;
                                                    if t {
                                                        break 'b_2a00f4;
                                                    } else {
                                                        c.cycles += 14;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        c.g[RA][0] = sx(0x002a00bc);
                                                        c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add(0x5560));
                                                        call_ptr(c, 0x00123128, 0x002a00bc)?;
                                                        c.mtc1(0, 12);
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A3][0] = c.g[S1][0];
                                                        c.g[T0][0] = c.g[V0][0];
                                                        c.g[A1][0] = 0u64;
                                                        c.g[A2][0] = 0x2090;
                                                        c.g[T1][0] = 0u64;
                                                        c.g[RA][0] = sx(0x002a00e0);
                                                        c.g[T2][0] = 0u64;
                                                        Callee { addr: 0x002b1458, f: ee::s08::SpeechSystem_SetEventContext }.call(c, 0x002a00e0)?;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = 0x1;
                                                        if t {
                                                            break 'b_2a02b4;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = 0u64;
                                                            let t = true;
                                                            c.g[A2][0] = 0x2090;
                                                            if t {
                                                                break 'b_2a023c;
                                                            } else {
                                                                break 'b_2a00f4;
                                                            }
                                                        }
                                                    }
                                                }
                                                'b_2a013c: {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A0][0] = c.g[S1][0];
                                                    if t {
                                                        break 'b_2a013c;
                                                    } else {
                                                        c.cycles += 13;
                                                        c.g[RA][0] = sx(0x002a0104);
                                                        c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add(0x5560));
                                                        call_ptr(c, 0x00123128, 0x002a0104)?;
                                                        c.mtc1(0, 12);
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A3][0] = c.g[S1][0];
                                                        c.g[T0][0] = c.g[V0][0];
                                                        c.g[A1][0] = 0u64;
                                                        c.g[A2][0] = 0x209b;
                                                        c.g[T1][0] = 0u64;
                                                        c.g[RA][0] = sx(0x002a0128);
                                                        c.g[T2][0] = 0u64;
                                                        Callee { addr: 0x002b1458, f: ee::s08::SpeechSystem_SetEventContext }.call(c, 0x002a0128)?;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = 0x1;
                                                        if t {
                                                            break 'b_2a02b4;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A1][0] = 0u64;
                                                            let t = true;
                                                            c.g[A2][0] = 0x209b;
                                                            if t {
                                                                break 'b_2a023c;
                                                            } else {
                                                                break 'b_2a013c;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 13;
                                                c.g[RA][0] = sx(0x002a0144);
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x5560));
                                                call_ptr(c, 0x00123128, 0x002a0144)?;
                                                c.mtc1(0, 12);
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A3][0] = c.g[S1][0];
                                                c.g[T0][0] = c.g[V0][0];
                                                c.g[A1][0] = 0u64;
                                                c.g[A2][0] = 0x209d;
                                                c.g[T1][0] = 0u64;
                                                c.g[RA][0] = sx(0x002a0168);
                                                c.g[T2][0] = 0u64;
                                                Callee {
                                                    addr: 0x002b1458,
                                                    f: ee::s08::SpeechSystem_SetEventContext,
                                                }
                                                .call(c, 0x002a0168)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = 0x1;
                                                if t {
                                                    break 'b_2a02b4;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[A1][0] = 0u64;
                                                    let t = true;
                                                    c.g[A2][0] = 0x209d;
                                                    if t {
                                                        break 'b_2a023c;
                                                    } else {
                                                        break 'b_2a017c;
                                                    }
                                                }
                                            }
                                        }
                                        'b_2a01f8: {
                                            c.cycles += 14;
                                            c.g[RA][0] = sx(0x002a0184);
                                            call_ptr(c, 0x0028b1e8, 0x002a0184)?;
                                            c.g[A1][0] = 0u64;
                                            c.g[RA][0] = sx(0x002a0190);
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x0014dd58, 0x002a0190)?;
                                            c.g[RA][0] = sx(0x002a0198);
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x00144bc0, 0x002a0198)?;
                                            c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                            c.g[RA][0] = sx(0x002a01a4);
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x002a1e20,
                                                f: ee::s08::SSXAudioSystem_TrackToCategory,
                                            }
                                            .call(c, 0x002a01a4)?;
                                            c.g[V1][0] = c.g[V0][0];
                                            c.g[V0][0] = 0x1;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_2a01f8;
                                            } else {
                                                c.cycles += 14;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[RA][0] = sx(0x002a01c0);
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x5560));
                                                call_ptr(c, 0x00123128, 0x002a01c0)?;
                                                c.mtc1(0, 12);
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A3][0] = c.g[S1][0];
                                                c.g[T0][0] = c.g[V0][0];
                                                c.g[A1][0] = 0u64;
                                                c.g[A2][0] = 0x2082;
                                                c.g[T1][0] = 0u64;
                                                c.g[RA][0] = sx(0x002a01e4);
                                                c.g[T2][0] = 0u64;
                                                Callee {
                                                    addr: 0x002b1458,
                                                    f: ee::s08::SpeechSystem_SetEventContext,
                                                }
                                                .call(c, 0x002a01e4)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = 0x1;
                                                if t {
                                                    break 'b_2a02b4;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[A1][0] = 0u64;
                                                    let t = true;
                                                    c.g[A2][0] = 0x2082;
                                                    if t {
                                                        break 'b_2a023c;
                                                    } else {
                                                        break 'b_2a01f8;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[A0][0] = c.g[S1][0];
                                        if t {
                                            break 'b_2a0260;
                                        } else {
                                            c.cycles += 13;
                                            c.g[RA][0] = sx(0x002a0208);
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32).wrapping_add(0x5560));
                                            call_ptr(c, 0x00123128, 0x002a0208)?;
                                            c.mtc1(0, 12);
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A3][0] = c.g[S1][0];
                                            c.g[T0][0] = c.g[V0][0];
                                            c.g[A1][0] = 0u64;
                                            c.g[A2][0] = 0x209a;
                                            c.g[T1][0] = 0u64;
                                            c.g[RA][0] = sx(0x002a022c);
                                            c.g[T2][0] = 0u64;
                                            Callee {
                                                addr: 0x002b1458,
                                                f: ee::s08::SpeechSystem_SetEventContext,
                                            }
                                            .call(c, 0x002a022c)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A0][0] = 0x1;
                                            if t {
                                                break 'b_2a02b4;
                                            } else {
                                                c.cycles += 2;
                                                c.g[A1][0] = 0u64;
                                                c.g[A2][0] = 0x209a;
                                                break 'b_2a023c;
                                            }
                                        }
                                    }
                                    c.cycles += 9;
                                    c.g[RA][0] = sx(0x002a0244);
                                    call_ptr(c, 0x003d8008, 0x002a0244)?;
                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x173c));
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A2][0] = c.g[S2][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x002a0258);
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, t, 0x002a0258)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    if t {
                                        break 'b_2a02b8;
                                    } else {
                                        break 'b_2a0260;
                                    }
                                }
                                c.cycles += 13;
                                c.g[RA][0] = sx(0x002a0268);
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
                                call_ptr(c, 0x00123128, 0x002a0268)?;
                                c.mtc1(0, 12);
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A3][0] = c.g[S1][0];
                                c.g[T0][0] = c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0x209c;
                                c.g[T1][0] = 0u64;
                                c.g[RA][0] = sx(0x002a028c);
                                c.g[T2][0] = 0u64;
                                Callee {
                                    addr: 0x002b1458,
                                    f: ee::s08::SpeechSystem_SetEventContext,
                                }
                                .call(c, 0x002a028c)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = 0x1;
                                if t {
                                    break 'b_2a02b4;
                                } else {
                                    c.cycles += 8;
                                    c.g[A1][0] = 0u64;
                                    c.g[RA][0] = sx(0x002a02a0);
                                    c.g[A2][0] = 0x209c;
                                    call_ptr(c, 0x003d8008, 0x002a02a0)?;
                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x173c));
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A2][0] = c.g[S2][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x002a02b4);
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, t, 0x002a02b4)?;
                                    break 'b_2a02b4;
                                }
                            }
                            c.cycles += 1;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                            break 'b_2a02b8;
                        }
                    }
                }
            }
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

/// `int SSXAudioSystem::GetCharacterVoiceGroup(self, Rider* rider)` (0x2a1fa0):
/// 0 when the rider's alternate (cheat) character is set or its character is
/// 10 or above; else a jump table over the character gives voice group 1 or 2.
/// (A resume at any other `pc` runs the generated body.)
pub fn ssx_audio_system_get_character_voice_group(c: &mut Cpu) {
    if c.pc != 0x002a1fa0 {
        return ee::s08::f_002a1fa0__generated(c);
    }
    guest(c, |c| {
        'b_2a2004: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x002a1fb8);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00123168, 0x002a1fb8)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_2a2004;
            } else {
                'b_2a2000: {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x002a1fc8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00123128, 0x002a1fc8)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        break 'b_2a2000;
                    } else {
                        'b_2a1ff8: {
                            c.cycles += 6;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30a0));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                            let t = c.g[A0][0] as u32;
                            match t {
                                0x002a1ff0 => {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = 0x2;
                                    if t {
                                        break 'b_2a2004;
                                    } else {
                                        break 'b_2a1ff8;
                                    }
                                }
                                0x002a1ff8 => {
                                    break 'b_2a1ff8;
                                }
                                _ => {
                                    c.jump_out(t, 0x002a1fa0, 0x002a2014);
                                    return Ok(());
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = true;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_2a2004;
                        } else {
                            break 'b_2a2000;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_2a2004;
            }
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `void SSXAudioSystem::PlayDJChatter(SSXAudioSystem* self, int pick_any)`
/// (0x2a2e50): the DJ's chatter. Runs the result chatter first when one is
/// pending (`+0x5790`); otherwise picks the next unused chatter line (bit
/// mask `+0x573c`, current index `+0x5798`; the mask resets, with a
/// category/reward check, when all lines are used) and says it through the
/// `DJSay*` family.
/// (A resume at any other `pc` runs the generated body.)
pub fn ssx_audio_system_play_dj_chatter(c: &mut Cpu) {
    if c.pc != 0x002a2e50 {
        return ee::s08::f_002a2e50__generated(c);
    }
    guest(c, |c| {
        'b_2a3148: {
            'b_2a3084: {
                'b_2a2ebc: {
                    c.cycles += 15;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5790));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S5][0] = 0u64;
                    if t {
                        break 'b_2a2ebc;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6258));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = 0x1;
                        if t {
                            break 'b_2a2ebc;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x573c));
                            c.g[A0][0] = 0x3;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x5790), c.g[V1][0]);
                            c.g[V0][0] = c.g[V0][0] | 0x80;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x5798), c.g[A0][0]);
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x573c), c.g[V0][0]);
                            c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5560));
                            let t = true;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x578c), 0u64);
                            if t {
                                break 'b_2a3084;
                            } else {
                                break 'b_2a2ebc;
                            }
                        }
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x002a2ec4);
                c.g[A0][0] = c.g[S2][0];
                Callee {
                    addr: 0x002a4770,
                    f: ee::s08::SSXAudioSystem_PlayDJResultChatter,
                }
                .call(c, 0x002a2ec4)?;
                if c.g[V0][0] != 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    break 'b_2a3148;
                } else {
                    'b_2a2efc: {
                        'b_2a2ef8: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5798));
                            c.g[A0][0] = 0xffffffffffffffff;
                            let t = c.g[V1][0] == c.g[A0][0];
                            c.g[A1][0] = 0x5;
                            if t {
                                break 'b_2a2ef8;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6258));
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x8;
                                if t {
                                    break 'b_2a2efc;
                                } else {
                                    c.cycles += 4;
                                    c.g[S5][0] = c.g[V1][0];
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x5798), c.g[A0][0]);
                                    let t = true;
                                    c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5560));
                                    if t {
                                        break 'b_2a3084;
                                    } else {
                                        break 'b_2a2ef8;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = 0x8;
                        break 'b_2a2efc;
                    }
                    'b_2a2f34: {
                        c.cycles += 5;
                        if c.g[S0][0] == 0 {
                            c.g[A1][0] = c.g[V0][0];
                        }
                        c.g[A0][0] = 0u64;
                        c.g[S3][0] = 0u64;
                        let t = c.g[A1][0] == 0u64;
                        c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5560));
                        if t {
                            break 'b_2a2f34;
                        } else {
                            c.cycles += 2;
                            c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x573c));
                            'l_2a2f18: loop {
                                c.cycles += 7;
                                c.g[V0][0] = sx(((c.g[A2][0] as u32) as i32
                                    >> ((c.g[A0][0] as u32) & 31))
                                    as u32);
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                                let t = c.g[V1][0] != 0u64;
                                c.g[S3][0] =
                                    sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002a2f18);
                                    }
                                    continue 'l_2a2f18;
                                } else {
                                    break 'b_2a2f34;
                                }
                            }
                        }
                    }
                    'b_2a3030: {
                        c.cycles += 2;
                        if c.g[S3][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
                            break 'b_2a3030;
                        } else {
                            'b_2a2fbc: {
                                c.cycles += 29;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x573c), 0u64);
                                c.g[S3][0] = c.g[A1][0];
                                c.g[RA][0] = sx(0x002a2f4c);
                                c.g[S4][0] = 0xe;
                                call_ptr(c, 0x0014dc80, 0x002a2f4c)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x002a2f58);
                                c.g[A1][0] = 0x1;
                                call_ptr(c, 0x0014dd58, 0x002a2f58)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                c.g[RA][0] = sx(0x002a2f68);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00146e98, 0x002a2f68)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x002a2f74);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x00147398, 0x002a2f74)?;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                                c.g[RA][0] = sx(0x002a2f80);
                                c.g[S6][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                call_ptr(c, 0x0028b1e8, 0x002a2f80)?;
                                c.g[A1][0] = 0u64;
                                c.g[RA][0] = sx(0x002a2f8c);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x0014dd58, 0x002a2f8c)?;
                                c.g[RA][0] = sx(0x002a2f94);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x00144bc0, 0x002a2f94)?;
                                c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                c.g[RA][0] = sx(0x002a2fa0);
                                c.g[A0][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x002a1e20,
                                    f: ee::s08::SSXAudioSystem_TrackToCategory,
                                }
                                .call(c, 0x002a2fa0)?;
                                c.g[V1][0] = c.g[V0][0];
                                c.g[V0][0] = 0x1;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V1][0] = c.g[V1][0] ^ 0x2;
                                if t {
                                    break 'b_2a2fbc;
                                } else {
                                    c.cycles += 3;
                                    c.g[S4][0] = 0x10;
                                    c.g[V0][0] = 0xf;
                                    if c.g[V1][0] == 0 {
                                        c.g[S4][0] = c.g[V0][0];
                                    }
                                    break 'b_2a2fbc;
                                }
                            }
                            'b_2a302c: {
                                c.cycles += 23;
                                c.g[RA][0] = sx(0x002a2fc4);
                                call_ptr(c, 0x0014dc80, 0x002a2fc4)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x002a2fd0);
                                c.g[A1][0] = 0xd;
                                call_ptr(c, 0x0014dd58, 0x002a2fd0)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A2][0] = c.g[S6][0];
                                c.g[A3][0] = 0x4;
                                c.g[RA][0] = sx(0x002a2fec);
                                c.g[T0][0] = c.g[S4][0];
                                call_ptr(c, 0x00158910, 0x002a2fec)?;
                                c.g[S0][0] = !(0u64 | c.g[V0][0]);
                                c.g[A0][0] = c.g[S1][0];
                                c.g[S0][0] = (0u64 < c.g[S0][0]) as u64;
                                c.g[A2][0] = c.g[S6][0];
                                c.g[T0][0] = c.g[S4][0];
                                c.g[A1][0] = 0u64;
                                c.g[RA][0] = sx(0x002a300c);
                                c.g[A3][0] = 0x5;
                                call_ptr(c, 0x00158910, 0x002a300c)?;
                                c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                let t = c.g[S0][0] == 0u64;
                                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                                if t {
                                    break 'b_2a302c;
                                } else {
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
                                        break 'b_2a3030;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x573c));
                                        c.g[V0][0] = c.g[V0][0] | 0x8;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x573c), c.g[V0][0]);
                                        break 'b_2a302c;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
                            break 'b_2a3030;
                        }
                    }
                    'b_2a3050: {
                        c.cycles += 8;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                        c.g[RA][0] = sx(0x002a303c);
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                        Callee {
                            addr: 0x002adf60,
                            f: ee::s08::Audio_Rand,
                        }
                        .call(c, 0x002a303c)?;
                        c.g[V0][0] = c.g[V0][0] & 0x7fff;
                        c.g[V1][0] = 0x7fff;
                        c.mult(0, 2, 2, 19);
                        if c.g[V1][0] == 0u64 {
                            break 'b_2a3050;
                        } else {
                            break 'b_2a3050;
                        }
                    }
                    c.cycles += 5;
                    c.divu(0, 2, 3);
                    c.g[V0][0] = c.lo[0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    let t = (c.g[A1][0] as i64) <= 0;
                    c.g[A0][0] = 0u64;
                    if t {
                        break 'b_2a3084;
                    } else {
                        c.cycles += 1;
                        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x573c));
                        'l_2a3068: loop {
                            c.cycles += 7;
                            c.g[V0][0] = sx(((c.g[A2][0] as u32) as i32
                                >> ((c.g[S5][0] as u32) & 31))
                                as u32);
                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                            let t = c.g[V1][0] != 0u64;
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002a3068);
                                }
                                continue 'l_2a3068;
                            } else {
                                break 'b_2a3084;
                            }
                        }
                    }
                }
            }
            'b_2a313c: {
                'b_2a3138: {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x573c));
                    c.g[V1][0] = 0x1;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S5][0] as u32) & 31));
                    c.g[A0][0] = (c.g[S5][0] < 0x8u64) as u64;
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    let t = c.g[A0][0] == 0u64;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x573c), c.g[V0][0]);
                    if t {
                        break 'b_2a3138;
                    } else {
                        'b_2a3130: {
                            'b_2a3120: {
                                'b_2a3110: {
                                    'b_2a3100: {
                                        'b_2a30f0: {
                                            'b_2a30e0: {
                                                'b_2a30cc: {
                                                    c.cycles += 7;
                                                    c.g[V0][0] = 0x480000;
                                                    c.g[V1][0] = sx((c.g[S5][0] as u32) << 2);
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x30d0));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                    let t = c.g[A0][0] as u32;
                                                    match t {
                                                        0x002a30bc => {
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x002a30c4);
                                                            c.g[A0][0] = c.g[S2][0];
                                                            Callee { addr: 0x002a2188, f: ee::s08::SSXAudioSystem_DJSay20CA }.call(c, 0x002a30c4)?;
                                                            let t = true;
                                                            c.g[A0][0] = c.g[S7][0];
                                                            if t {
                                                                break 'b_2a313c;
                                                            } else {
                                                                break 'b_2a30cc;
                                                            }
                                                        }
                                                        0x002a30cc => {
                                                            break 'b_2a30cc;
                                                        }
                                                        0x002a30e0 => {
                                                            break 'b_2a30e0;
                                                        }
                                                        0x002a30f0 => {
                                                            break 'b_2a30f0;
                                                        }
                                                        0x002a3100 => {
                                                            break 'b_2a3100;
                                                        }
                                                        0x002a3110 => {
                                                            break 'b_2a3110;
                                                        }
                                                        0x002a3120 => {
                                                            break 'b_2a3120;
                                                        }
                                                        0x002a3130 => {
                                                            break 'b_2a3130;
                                                        }
                                                        _ => {
                                                            c.jump_out(t, 0x002a2e50, 0x002a3170);
                                                            return Ok(());
                                                        }
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[RA][0] = sx(0x002a30d8);
                                                c.g[A1][0] = 0xffffffffffffffff;
                                                Callee {
                                                    addr: 0x002a23d0,
                                                    f: ee::s08::SSXAudioSystem_DJSay20E9,
                                                }
                                                .call(c, 0x002a30d8)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S7][0];
                                                if t {
                                                    break 'b_2a313c;
                                                } else {
                                                    break 'b_2a30e0;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x002a30e8);
                                            c.g[A0][0] = c.g[S2][0];
                                            Callee {
                                                addr: 0x002a2018,
                                                f: ee::s08::SSXAudioSystem_DJSay20C8,
                                            }
                                            .call(c, 0x002a30e8)?;
                                            let t = true;
                                            c.g[A0][0] = c.g[S7][0];
                                            if t {
                                                break 'b_2a313c;
                                            } else {
                                                break 'b_2a30f0;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x002a30f8);
                                        c.g[A0][0] = c.g[S2][0];
                                        Callee {
                                            addr: 0x002a2a10,
                                            f: ee::s08::SSXAudioSystem_DJSay210B,
                                        }
                                        .call(c, 0x002a30f8)?;
                                        let t = true;
                                        c.g[A0][0] = c.g[S7][0];
                                        if t {
                                            break 'b_2a313c;
                                        } else {
                                            break 'b_2a3100;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x002a3108);
                                    c.g[A0][0] = c.g[S2][0];
                                    Callee {
                                        addr: 0x002a2b88,
                                        f: ee::s08::SSXAudioSystem_DJSay212C,
                                    }
                                    .call(c, 0x002a3108)?;
                                    let t = true;
                                    c.g[A0][0] = c.g[S7][0];
                                    if t {
                                        break 'b_2a313c;
                                    } else {
                                        break 'b_2a3110;
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x002a3118);
                                c.g[A0][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x002a2260,
                                    f: ee::s08::SSXAudioSystem_DJSay20E7,
                                }
                                .call(c, 0x002a3118)?;
                                let t = true;
                                c.g[A0][0] = c.g[S7][0];
                                if t {
                                    break 'b_2a313c;
                                } else {
                                    break 'b_2a3120;
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x002a3128);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x002a2318,
                                f: ee::s08::SSXAudioSystem_DJSay20E8,
                            }
                            .call(c, 0x002a3128)?;
                            let t = true;
                            c.g[A0][0] = c.g[S7][0];
                            if t {
                                break 'b_2a313c;
                            } else {
                                break 'b_2a3130;
                            }
                        }
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x002a3138);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x002a20d0,
                            f: ee::s08::SSXAudioSystem_DJSay20C9,
                        }
                        .call(c, 0x002a3138)?;
                        break 'b_2a3138;
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[S7][0];
                break 'b_2a313c;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x002a3144);
            c.g[A1][0] = 0u64;
            Callee {
                addr: 0x002b1758,
                f: ee::s08::SpeechSystem_CancelPendingInstance,
            }
            .call(c, 0x002a3144)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_2a3148;
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0029_ff80,
        ssx_audio_system_rider_say2082,
        "SSXAudioSystem_RiderSay2082",
    ),
    (
        0x002a_1fa0,
        ssx_audio_system_get_character_voice_group,
        "SSXAudioSystem_GetCharacterVoiceGroup",
    ),
    (
        0x002a_2e50,
        ssx_audio_system_play_dj_chatter,
        "SSXAudioSystem_PlayDJChatter",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, DATA, GAME_GP};
    use crate::game::sound::soundsys::tests::rs207d_world;

    const SYS: u32 = DATA + 0x20_0000;
    const RIDER_A: u32 = DATA + 0x21_0000;
    const RIDER_B: u32 = DATA + 0x21_1000;
    const RIDER_C: u32 = DATA + 0x21_2000;
    /// Race record 0's track id (categories 1, 2, 4 and "none").
    const TRACK_ID: u32 = 0x0043_d950;
    const OC1028_OBJ: u32 = DATA + 0x22_0000;
    const OC1028_VTABLE: u32 = DATA + 0x22_0100;

    /// A decorrelated digit of `seed` (the world uses `seed % k` digits).
    fn oc1028_digit(seed: u32, salt: u32, n: u32) -> u32 {
        (seed.wrapping_mul(0x9e37_79b1) ^ salt.wrapping_mul(0x85eb_ca6b)).rotate_left(13) / 7 % n
    }

    /// The race's track category (1, 2, 4 or none).
    fn oc1028_track(c: &mut Cpu, seed: u32) {
        c.bus.write32(0x0053_5c08, 0);
        c.bus.write32(
            TRACK_ID,
            [0u32, 2, 4, 0x20][oc1028_digit(seed, 1, 4) as usize],
        );
    }

    /// `RiderSay2082` on the speech world of `RiderSay207D`'s test: speech
    /// volume, replay, finished rider (progress 1.0 / 0.0), AI / local
    /// combinations against the followed rider, speech instance not ready,
    /// heard or not, track categories, `alt` 0 and 1, and lines already live.
    #[test]
    fn ssx_audio_system_rider_say2082_matches() {
        check(
            ee::s08::f_0029ff80__generated,
            ssx_audio_system_rider_say2082,
            300,
            &|c, seed| {
                let mut r = rs207d_world(c, seed);
                c.pc = 0x0029_ff80;
                let progress = match seed % 11 {
                    1 => 1.0f32,
                    8 => 0.0,
                    _ => -1.0 - (r.next_u32() % 100) as f32,
                };
                c.bus.write32(RIDER_A + 0x470, progress.to_bits());
                let radius = if seed % 17 == 9 { 0.0001f32 } else { 10.0 };
                c.bus.write32(GAME_GP + 0x5e8, radius.to_bits());
                oc1028_track(c, seed);
                let alt = oc1028_digit(seed, 2, 2);
                if seed % 5 == 1 {
                    let lines: [u32; 3] = if alt != 0 {
                        [0x2090, 0x209b, 0x209d]
                    } else {
                        [0x2082, 0x209a, 0x209c]
                    };
                    let speech = SYS + 0x5560;
                    for (i, code) in lines.into_iter().enumerate() {
                        let slot = speech + 0x18 + 0x20 * i as u32;
                        c.bus.write32(slot, 1);
                        c.bus.write32(slot + 4, 50);
                        c.bus.write32(slot + 8, 0);
                        c.bus.write32(slot + 0xc, code);
                    }
                }
                c.g[A0][0] = SYS as u64;
                c.g[A1][0] = RIDER_A as u64;
                c.g[A2][0] = alt as u64;
            },
        );
    }

    /// `GetCharacterVoiceGroup`: riders 0..2 whose character byte
    /// (`0x534fe0 + 0x1c * slot + 0x11`) is 0..9 (the jump table), 10, 0x7f,
    /// 0x80 or 0xff (signed, so it still is "10 or above" unsigned), with the
    /// alternate character (`+0x12`) set or not.
    #[test]
    fn ssx_audio_system_get_character_voice_group_matches() {
        check(
            ee::s08::f_002a1fa0__generated,
            ssx_audio_system_get_character_voice_group,
            120,
            &|c, seed| {
                let _r = rs207d_world(c, seed);
                c.pc = 0x002a_1fa0;
                let who = seed % 3;
                let character = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0x7f, 0x80, 0xff]
                    [(seed / 3 % 14) as usize];
                let rec = 0x0053_4fe0 + 0x1c * who;
                c.bus.write8(rec + 0x11, character);
                c.bus.write8(rec + 0x12, if seed % 7 == 0 { 1 } else { 0 });
                c.g[A0][0] = SYS as u64;
                c.g[A1][0] = [RIDER_A, RIDER_B, RIDER_C][who as usize] as u64;
            },
        );
    }

    /// `PlayDJChatter`: a pending result chatter (`+0x5790`), speech busy
    /// (`+0x6258`), the used-line mask (empty, full, partial) with the
    /// current index unset or set, `pick_any` 0 / 1, the track category and
    /// the random pick over the unused lines (the `DJSay*` callees run for
    /// real on the speech world).
    #[test]
    fn ssx_audio_system_play_dj_chatter_matches() {
        check(
            ee::s08::f_002a2e50__generated,
            ssx_audio_system_play_dj_chatter,
            300,
            &|c, seed| {
                let mut r = rs207d_world(c, seed);
                c.pc = 0x002a_2e50;
                oc1028_track(c, seed);
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                w(c, SYS + 0x5790, (oc1028_digit(seed, 3, 11) == 3) as u32);
                w(c, SYS + 0x6258, (oc1028_digit(seed, 4, 6) == 1) as u32);
                let mask = [0u32, 0xff, 0x0f, 0xaa, r.next_u32() & 0xff, 0x7f]
                    [oc1028_digit(seed, 5, 6) as usize];
                w(c, SYS + 0x573c, mask);
                let index = match oc1028_digit(seed, 6, 4) {
                    0 => u32::MAX,
                    _ => r.next_u32() % 8,
                };
                w(c, SYS + 0x5798, index);
                w(c, SYS + 0x118, OC1028_OBJ);
                w(c, OC1028_OBJ, OC1028_VTABLE);
                w(c, OC1028_VTABLE + 0x1d8, r.next_u32());
                c.g[A0][0] = SYS as u64;
                c.g[A1][0] = oc1028_digit(seed, 7, 2) as u64;
            },
        );
    }
}
