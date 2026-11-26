//! Staging module oc836 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc836`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0018_b380,
    cfe_state_options_sound_refresh_controls,
    "cFEStateOptionsSound_RefreshControls",
)];

/// `cFEStateOptionsSound_RefreshControls(self)` (0x18b380,
/// `cFEStateOptionsSound::updateWidget`): fills the sound options screen from
/// the saved options. Seven widget lookups (`Find` = 0x39d860 on
/// `self->f40` with a string hash from 0x317670):
/// - speaker mode list (4 entries: stereo, surround, mono, DTS), selecting the
///   entry of the current sound mode (`0x289dc0(0x28b180())`);
/// - SFX, speech and music volume sliders (`w+0x78 = 12`, `w+0x74` = the
///   saved volume, applied to the audio system with 0x287410 / 0x287488 /
///   0x287520);
/// - DJ speech and arcade SFX off/on lists, selecting the saved flag and
///   applying it (0x287558 / 0x2875d0);
/// - only when `D_004A1A70 == 1`: a last off/on list (option bit 18).
///
/// Each saved value is read from the stack copy of the BE option struct at
/// `0x535610` (`Cfg_18B380()` by value: a 0x288-byte inline copy per use;
/// the source address is a constant, so only the aligned `ld`/`sd` copy
/// loop runs, the `ldl`/`ldr` loop is kept for fidelity). The generated block
/// structure is kept: every copy loop, its register use and event check, and
/// every delay slot are as in the generated body.
pub fn cfe_state_options_sound_refresh_controls(c: &mut Cpu) {
    guest(c, cfe_state_options_sound_refresh_controls_body);
}

#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    clippy::all
)]
fn cfe_state_options_sound_refresh_controls_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x0018b380);
    'b_18b488: {
        c.cycles += 23;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x330));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x310));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x320));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x300));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2f0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x2e0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x2d0));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x2b0), c.g[RA][0]);
        c.g[RA][0] = sx(0x0018b3ac);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0014dc80, 0x0018b3ac)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0018b3b8);
        c.g[A1][0] = 0x4;
        call_ptr(c, 0x0014dd58, 0x0018b3b8)?;
        c.g[A0][0] = 0x4a0000;
        c.g[RA][0] = sx(0x0018b3c4);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1568));
        call_ptr(c, 0x00317670, 0x0018b3c4)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x0018b3d0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x0018b3d0)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0u64 {
            c.g[A0][0] = 0x4a0000;
            break 'b_18b488;
        } else {
            'b_18b484: {
                c.cycles += 40;
                c.g[RA][0] = sx(0x0018b3e4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039a670, 0x0018b3e4)?;
                c.g[A0][0] = 0x460000;
                c.g[RA][0] = sx(0x0018b3f0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2030));
                call_ptr(c, 0x00317670, 0x0018b3f0)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0018b400);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039a568, 0x0018b400)?;
                c.g[A0][0] = 0x460000;
                c.g[RA][0] = sx(0x0018b40c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2018));
                call_ptr(c, 0x00317670, 0x0018b40c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = 0x1;
                c.g[RA][0] = sx(0x0018b41c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039a568, 0x0018b41c)?;
                c.g[A0][0] = 0x460000;
                c.g[RA][0] = sx(0x0018b428);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2000));
                call_ptr(c, 0x00317670, 0x0018b428)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = 0x2;
                c.g[RA][0] = sx(0x0018b438);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039a568, 0x0018b438)?;
                c.g[A0][0] = 0x460000;
                c.g[RA][0] = sx(0x0018b444);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fe8));
                call_ptr(c, 0x00317670, 0x0018b444)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = 0x3;
                c.g[RA][0] = sx(0x0018b454);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039a568, 0x0018b454)?;
                c.g[RA][0] = sx(0x0018b45c);
                call_ptr(c, 0x0028b180, 0x0018b45c)?;
                c.g[RA][0] = sx(0x0018b464);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00289dc0, 0x0018b464)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0018b474);
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, 0x0039a768, 0x0018b474)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lbu(c.g[SP][0] as u32);
                if t {
                    break 'b_18b484;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0018b484);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a7a8, 0x0018b484)?;
                    break 'b_18b484;
                }
            }
            c.cycles += 1;
            c.g[A0][0] = 0x4a0000;
            break 'b_18b488;
        }
    }
    'b_18b638: {
        c.cycles += 12;
        c.g[S0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1590));
        c.g[RA][0] = sx(0x0018b498);
        c.g[S2][0] = 0x4a0000;
        call_ptr(c, 0x00317670, 0x0018b498)?;
        c.g[S3][0] = 0x4a0000;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0018b4ac);
        c.g[S6][0] = 0x4a0000;
        call_ptr(c, 0x0039d860, 0x0018b4ac)?;
        c.g[A2][0] = c.g[V0][0];
        let t = c.g[A2][0] == 0u64;
        c.g[V1][0] = 0x530000;
        if t {
            break 'b_18b638;
        } else {
            'b_18b560: {
                'b_18b530: {
                    c.cycles += 8;
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                    c.g[S5][0] = c.g[V1][0];
                    c.g[V0][0] = c.g[A1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = c.g[A0][0];
                    if t {
                        break 'b_18b530;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        'l_18b4dc: loop {
                            c.cycles += 19;
                            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.ldr(3, c.g[A1][0] as u32);
                            c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.sdr(3, c.g[A0][0] as u32);
                            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0xf));
                            c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x17));
                            c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x10));
                            c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x1f));
                            c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018b4dc);
                                }
                                continue 'l_18b4dc;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18b560;
                                } else {
                                    break 'b_18b530;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                'l_18b534: loop {
                    c.cycles += 11;
                    c.g[V1][0] = c.ld(c.g[A1][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[V1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018b534);
                        }
                        continue 'l_18b534;
                    } else {
                        break 'b_18b560;
                    }
                }
            }
            'b_18b61c: {
                'b_18b5ec: {
                    c.cycles += 13;
                    c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[A1][0] as u32);
                    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(2, c.g[A0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.g[RA][0] = sx(0x0018b580);
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                    call_ptr(c, 0x0028b180, 0x0018b580)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5610));
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t {
                        break 'b_18b5ec;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                        'l_18b598: loop {
                            c.cycles += 19;
                            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.ldr(9, c.g[V1][0] as u32);
                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                            c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.sdr(9, c.g[A1][0] as u32);
                            c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.sdr(6, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018b598);
                                }
                                continue 'l_18b598;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18b61c;
                                } else {
                                    break 'b_18b5ec;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                'l_18b5f0: loop {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018b5f0);
                        }
                        continue 'l_18b5f0;
                    } else {
                        break 'b_18b61c;
                    }
                }
            }
            c.cycles += 7;
            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(9, c.g[V1][0] as u32);
            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(9, c.g[A1][0] as u32);
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x0018b638);
            c.g[A1][0] = c.g[A1][0] & 0xf;
            call_ptr(c, 0x00287410, 0x0018b638)?;
            break 'b_18b638;
        }
    }
    'b_18b7e0: {
        c.cycles += 8;
        c.g[RA][0] = sx(0x0018b640);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1598));
        call_ptr(c, 0x00317670, 0x0018b640)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x0018b64c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x0018b64c)?;
        c.g[A2][0] = c.g[V0][0];
        let t = c.g[A2][0] == 0u64;
        c.g[V1][0] = 0x530000;
        if t {
            break 'b_18b7e0;
        } else {
            'b_18b700: {
                'b_18b6d0: {
                    c.cycles += 8;
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                    c.g[S5][0] = c.g[V1][0];
                    c.g[V0][0] = c.g[A1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = c.g[A0][0];
                    if t {
                        break 'b_18b6d0;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        'l_18b67c: loop {
                            c.cycles += 19;
                            c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.ldr(8, c.g[A1][0] as u32);
                            c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.sdr(8, c.g[A0][0] as u32);
                            c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0xf));
                            c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0x17));
                            c.sdr(3, (c.g[A0][0] as u32).wrapping_add(0x10));
                            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x1f));
                            c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018b67c);
                                }
                                continue 'l_18b67c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18b700;
                                } else {
                                    break 'b_18b6d0;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                'l_18b6d4: loop {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[A1][0] as u32);
                    c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018b6d4);
                        }
                        continue 'l_18b6d4;
                    } else {
                        break 'b_18b700;
                    }
                }
            }
            'b_18b7c0: {
                'b_18b790: {
                    c.cycles += 14;
                    c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(8, c.g[A1][0] as u32);
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(8, c.g[A0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.g[RA][0] = sx(0x0018b724);
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                    call_ptr(c, 0x0028b180, 0x0018b724)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5610));
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t {
                        break 'b_18b790;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                        'l_18b73c: loop {
                            c.cycles += 19;
                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.ldr(6, c.g[V1][0] as u32);
                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                            c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                            c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.sdr(6, c.g[A1][0] as u32);
                            c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018b73c);
                                }
                                continue 'l_18b73c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18b7c0;
                                } else {
                                    break 'b_18b790;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                'l_18b794: loop {
                    c.cycles += 11;
                    c.g[A2][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018b794);
                        }
                        continue 'l_18b794;
                    } else {
                        break 'b_18b7c0;
                    }
                }
            }
            c.cycles += 8;
            c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(2, c.g[V1][0] as u32);
            c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[A1][0] as u32);
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 8);
            c.g[RA][0] = sx(0x0018b7e0);
            c.g[A1][0] = c.g[A1][0] & 0xf;
            call_ptr(c, 0x00287488, 0x0018b7e0)?;
            break 'b_18b7e0;
        }
    }
    'b_18b980: {
        c.cycles += 8;
        c.g[RA][0] = sx(0x0018b7e8);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x15a0));
        call_ptr(c, 0x00317670, 0x0018b7e8)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x0018b7f4);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x0018b7f4)?;
        c.g[A2][0] = c.g[V0][0];
        let t = c.g[A2][0] == 0u64;
        c.g[V1][0] = 0x530000;
        if t {
            break 'b_18b980;
        } else {
            'b_18b8a8: {
                'b_18b878: {
                    c.cycles += 8;
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                    c.g[S5][0] = c.g[V1][0];
                    c.g[V0][0] = c.g[A1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = c.g[A0][0];
                    if t {
                        break 'b_18b878;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        'l_18b824: loop {
                            c.cycles += 19;
                            c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.ldr(9, c.g[A1][0] as u32);
                            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.sdr(9, c.g[A0][0] as u32);
                            c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0xf));
                            c.sdr(3, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                            c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                            c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                            c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018b824);
                                }
                                continue 'l_18b824;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18b8a8;
                                } else {
                                    break 'b_18b878;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                'l_18b87c: loop {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A1][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018b87c);
                        }
                        continue 'l_18b87c;
                    } else {
                        break 'b_18b8a8;
                    }
                }
            }
            'b_18b964: {
                'b_18b934: {
                    c.cycles += 13;
                    c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(9, c.g[A1][0] as u32);
                    c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(9, c.g[A0][0] as u32);
                    c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.g[V0][0] >> 4;
                    c.g[RA][0] = sx(0x0018b8c8);
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                    call_ptr(c, 0x0028b180, 0x0018b8c8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5610));
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t {
                        break 'b_18b934;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                        'l_18b8e0: loop {
                            c.cycles += 19;
                            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.ldr(8, c.g[V1][0] as u32);
                            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
                            c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.sdr(8, c.g[A1][0] as u32);
                            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.sdr(6, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018b8e0);
                                }
                                continue 'l_18b8e0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18b964;
                                } else {
                                    break 'b_18b934;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                'l_18b938: loop {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[V1][0] as u32);
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018b938);
                        }
                        continue 'l_18b938;
                    } else {
                        break 'b_18b964;
                    }
                }
            }
            c.cycles += 7;
            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(8, c.g[V1][0] as u32);
            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(8, c.g[A1][0] as u32);
            c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x0018b980);
            c.g[A1][0] = c.g[A1][0] >> 4;
            call_ptr(c, 0x00287520, 0x0018b980)?;
            break 'b_18b980;
        }
    }
    'b_18bb84: {
        c.cycles += 8;
        c.g[RA][0] = sx(0x0018b988);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x15a8));
        call_ptr(c, 0x00317670, 0x0018b988)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x0018b994);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x0018b994)?;
        c.g[S0][0] = c.g[V0][0];
        let t = c.g[S0][0] == 0u64;
        if t {
            break 'b_18bb84;
        } else {
            'b_18ba8c: {
                'b_18ba58: {
                    c.cycles += 23;
                    c.g[RA][0] = sx(0x0018b9a8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a670, 0x0018b9a8)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x0018b9b4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20c0));
                    call_ptr(c, 0x00317670, 0x0018b9b4)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0018b9c4);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x0018b9c4)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x0018b9d0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fd8));
                    call_ptr(c, 0x00317670, 0x0018b9d0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0018b9e0);
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039a568, 0x0018b9e0)?;
                    c.g[V1][0] = 0x530000;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                    c.g[S5][0] = c.g[V1][0];
                    c.g[V0][0] = c.g[A1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = c.g[A0][0];
                    if t {
                        break 'b_18ba58;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2a0));
                        'l_18ba04: loop {
                            c.cycles += 19;
                            c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.ldr(7, c.g[A1][0] as u32);
                            c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.sdr(7, c.g[A0][0] as u32);
                            c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xf));
                            c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x17));
                            c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x10));
                            c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0x1f));
                            c.sdr(3, (c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018ba04);
                                }
                                continue 'l_18ba04;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18ba8c;
                                } else {
                                    break 'b_18ba58;
                                }
                            }
                        }
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2a0));
                'l_18ba60: loop {
                    c.cycles += 11;
                    c.g[A3][0] = c.ld(c.g[A1][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018ba60);
                        }
                        continue 'l_18ba60;
                    } else {
                        break 'b_18ba8c;
                    }
                }
            }
            'b_18bac0: {
                c.cycles += 11;
                c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(7, c.g[A1][0] as u32);
                c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                c.sdr(7, c.g[A0][0] as u32);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 17);
                c.g[RA][0] = sx(0x0018bab0);
                c.g[A1][0] = c.g[A1][0] & 0x1;
                call_ptr(c, 0x0039a768, 0x0018bab0)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x2a0));
                if t {
                    break 'b_18bac0;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0018bac0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a7a8, 0x0018bac0)?;
                    break 'b_18bac0;
                }
            }
            'b_18bb64: {
                'b_18bb34: {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x0018bac8);
                    call_ptr(c, 0x0028b180, 0x0018bac8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5610));
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t {
                        break 'b_18bb34;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                        'l_18bae0: loop {
                            c.cycles += 19;
                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.ldr(6, c.g[V1][0] as u32);
                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                            c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                            c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.sdr(6, c.g[A1][0] as u32);
                            c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018bae0);
                                }
                                continue 'l_18bae0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18bb64;
                                } else {
                                    break 'b_18bb34;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                'l_18bb38: loop {
                    c.cycles += 11;
                    c.g[A2][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018bb38);
                        }
                        continue 'l_18bb38;
                    } else {
                        break 'b_18bb64;
                    }
                }
            }
            c.cycles += 8;
            c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(2, c.g[V1][0] as u32);
            c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[A1][0] as u32);
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
            c.g[RA][0] = sx(0x0018bb84);
            c.g[A1][0] = c.g[A1][0] & 0x1;
            call_ptr(c, 0x00287558, 0x0018bb84)?;
            break 'b_18bb84;
        }
    }
    'b_18bd84: {
        c.cycles += 8;
        c.g[RA][0] = sx(0x0018bb8c);
        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x15b0));
        call_ptr(c, 0x00317670, 0x0018bb8c)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x0018bb98);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x0018bb98)?;
        c.g[S0][0] = c.g[V0][0];
        let t = c.g[S0][0] == 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
        if t {
            break 'b_18bd84;
        } else {
            'b_18bc90: {
                'b_18bc5c: {
                    c.cycles += 23;
                    c.g[RA][0] = sx(0x0018bbac);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a670, 0x0018bbac)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x0018bbb8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20c0));
                    call_ptr(c, 0x00317670, 0x0018bbb8)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0018bbc8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x0018bbc8)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x0018bbd4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fd8));
                    call_ptr(c, 0x00317670, 0x0018bbd4)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0018bbe4);
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039a568, 0x0018bbe4)?;
                    c.g[V1][0] = 0x530000;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                    c.g[S5][0] = c.g[V1][0];
                    c.g[V0][0] = c.g[A1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = c.g[A0][0];
                    if t {
                        break 'b_18bc5c;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2a1));
                        'l_18bc08: loop {
                            c.cycles += 19;
                            c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.ldr(9, c.g[A1][0] as u32);
                            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.sdr(9, c.g[A0][0] as u32);
                            c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0xf));
                            c.sdr(3, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                            c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                            c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                            c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018bc08);
                                }
                                continue 'l_18bc08;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18bc90;
                                } else {
                                    break 'b_18bc5c;
                                }
                            }
                        }
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2a1));
                'l_18bc64: loop {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A1][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018bc64);
                        }
                        continue 'l_18bc64;
                    } else {
                        break 'b_18bc90;
                    }
                }
            }
            'b_18bcc0: {
                c.cycles += 10;
                c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(9, c.g[A1][0] as u32);
                c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                c.sdr(9, c.g[A0][0] as u32);
                c.g[A1][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x12));
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0018bcb0);
                c.g[A1][0] = c.g[A1][0] & 0x1;
                call_ptr(c, 0x0039a768, 0x0018bcb0)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x2a1));
                if t {
                    break 'b_18bcc0;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0018bcc0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a7a8, 0x0018bcc0)?;
                    break 'b_18bcc0;
                }
            }
            'b_18bd64: {
                'b_18bd34: {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x0018bcc8);
                    call_ptr(c, 0x0028b180, 0x0018bcc8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5610));
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t {
                        break 'b_18bd34;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                        'l_18bce0: loop {
                            c.cycles += 19;
                            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.ldr(8, c.g[V1][0] as u32);
                            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
                            c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.sdr(8, c.g[A1][0] as u32);
                            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.sdr(6, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018bce0);
                                }
                                continue 'l_18bce0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18bd64;
                                } else {
                                    break 'b_18bd34;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                'l_18bd38: loop {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[V1][0] as u32);
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018bd38);
                        }
                        continue 'l_18bd38;
                    } else {
                        break 'b_18bd64;
                    }
                }
            }
            c.cycles += 8;
            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(8, c.g[V1][0] as u32);
            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(8, c.g[A1][0] as u32);
            c.g[A1][0] = c.lh((c.g[SP][0] as u32).wrapping_add(0x12));
            c.g[RA][0] = sx(0x0018bd80);
            c.g[A1][0] = c.g[A1][0] & 0x1;
            call_ptr(c, 0x002875d0, 0x0018bd80)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
            break 'b_18bd84;
        }
    }
    'b_18bed0: {
        c.cycles += 3;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        let t = c.g[V0][0] != 0u64;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x320));
        if t {
            break 'b_18bed0;
        } else {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x0018bd9c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15b8));
            call_ptr(c, 0x00317670, 0x0018bd9c)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x0018bda8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x0018bda8)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x320));
                break 'b_18bed0;
            } else {
                'b_18be98: {
                    'b_18be64: {
                        c.cycles += 21;
                        c.g[RA][0] = sx(0x0018bdbc);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0039a670, 0x0018bdbc)?;
                        c.g[A0][0] = 0x460000;
                        c.g[RA][0] = sx(0x0018bdc8);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20c0));
                        call_ptr(c, 0x00317670, 0x0018bdc8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A2][0] = 0u64;
                        c.g[RA][0] = sx(0x0018bdd8);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0039a568, 0x0018bdd8)?;
                        c.g[A0][0] = 0x460000;
                        c.g[RA][0] = sx(0x0018bde4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fd8));
                        call_ptr(c, 0x00317670, 0x0018bde4)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x0018bdf4);
                        c.g[A2][0] = 0x1;
                        call_ptr(c, 0x0039a568, 0x0018bdf4)?;
                        c.g[V1][0] = 0x530000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                        c.g[V0][0] = c.g[V1][0] & 0x7;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_18be64;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2a2));
                            'l_18be10: loop {
                                c.cycles += 19;
                                c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                c.ldr(7, c.g[V1][0] as u32);
                                c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                                c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                                c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x17));
                                c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x10));
                                c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x18));
                                c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                                c.sdr(7, c.g[A0][0] as u32);
                                c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xf));
                                c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                                c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x17));
                                c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x10));
                                c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x18));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0018be10);
                                    }
                                    continue 'l_18be10;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    if t {
                                        break 'b_18be98;
                                    } else {
                                        break 'b_18be64;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2a2));
                    'l_18be6c: loop {
                        c.cycles += 11;
                        c.g[A3][0] = c.ld(c.g[V1][0] as u32);
                        c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sd(c.g[A0][0] as u32, c.g[A3][0]);
                        c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                        c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                        c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0018be6c);
                            }
                            continue 'l_18be6c;
                        } else {
                            break 'b_18be98;
                        }
                    }
                }
                'b_18becc: {
                    c.cycles += 11;
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(7, c.g[V1][0] as u32);
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(7, c.g[A0][0] as u32);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 18);
                    c.g[RA][0] = sx(0x0018bebc);
                    c.g[A1][0] = c.g[A1][0] & 0x1;
                    call_ptr(c, 0x0039a768, 0x0018bebc)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x2a2));
                    if t {
                        break 'b_18becc;
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x0018becc);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0039a7a8, 0x0018becc)?;
                        break 'b_18becc;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x320));
                break 'b_18bed0;
            }
        }
    }
    c.cycles += 9;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x310));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x300));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2f0));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x2e0));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x2d0));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x2c0));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x2b0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x330));
    c.pc = t;
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the fakes (own area, clear of the fake log at DATA+0x100000).
    const OC836_CTRL: u32 = DATA + 0x2a_0000;
    const OC836_MAGIC: u32 = 0x4f43_3836;
    /// Call counter and "absent" bit mask of the `Find` fake (0x39d860).
    const OC836_FIND_N: u32 = OC836_CTRL + 4;
    const OC836_FIND_MASK: u32 = OC836_CTRL + 8;
    /// Same for the `getEntryByData` fake (0x39a768).
    const OC836_ENTRY_N: u32 = OC836_CTRL + 0xc;
    const OC836_ENTRY_MASK: u32 = OC836_CTRL + 0x10;
    /// The screen object (`self`), the BE option struct (`Cfg_18B380()` source)
    /// and the `D_004A1A70` flag (read as `gp - 0x1680`).
    const OC836_SELF: u32 = DATA + 0x2a_1000;
    const OC836_BEOPT: u32 = 0x0053_5610;
    const OC836_FLAG: u32 = GAME_GP - 0x1680;

    /// Logs the call like `fakes::fake_call`; then the `Find` and
    /// `getEntryByData` fakes answer 0 on the calls whose bit is set in their
    /// mask (the other calls keep the fake's pointer).
    fn oc836_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let (n_at, mask_at) = match addr {
            0x0039_d860 => (OC836_FIND_N, OC836_FIND_MASK),
            0x0039_a768 => (OC836_ENTRY_N, OC836_ENTRY_MASK),
            _ => return,
        };
        let n = c.bus.read32(n_at);
        c.bus.write32(n_at, n + 1);
        if (c.bus.read32(mask_at) >> (n % 32)) & 1 != 0 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc836_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC836_CTRL) != OC836_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc836_fake_call(c, $addr);
                }
            )*
            fn oc836_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc836_fakes!(
        (
            oc836_fake_14dc80,
            OC836_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc836_fake_14dd58,
            OC836_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc836_fake_317670,
            OC836_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc836_fake_39d860,
            OC836_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc836_fake_39a670,
            OC836_PREV_39A670,
            ee::s13::OVERRIDE_0039A670,
            ee::s13::f_0039a670__generated,
            0x0039_a670
        ),
        (
            oc836_fake_39a568,
            OC836_PREV_39A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039_a568
        ),
        (
            oc836_fake_28b180,
            OC836_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc836_fake_39a768,
            OC836_PREV_39A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039_a768
        ),
        (
            oc836_fake_39a7a8,
            OC836_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039_a7a8
        ),
        (
            oc836_fake_287410,
            OC836_PREV_287410,
            ee::s07::OVERRIDE_00287410,
            ee::s07::f_00287410__generated,
            0x0028_7410
        ),
        (
            oc836_fake_287488,
            OC836_PREV_287488,
            ee::s07::OVERRIDE_00287488,
            ee::s07::f_00287488__generated,
            0x0028_7488
        ),
        (
            oc836_fake_287520,
            OC836_PREV_287520,
            ee::s07::OVERRIDE_00287520,
            ee::s07::f_00287520__generated,
            0x0028_7520
        ),
        (
            oc836_fake_287558,
            OC836_PREV_287558,
            ee::s07::OVERRIDE_00287558,
            ee::s07::f_00287558__generated,
            0x0028_7558
        ),
        (
            oc836_fake_2875d0,
            OC836_PREV_2875D0,
            ee::s07::OVERRIDE_002875D0,
            ee::s07::f_002875d0__generated,
            0x0028_75d0
        ),
    );

    /// `cFEStateOptionsSound_RefreshControls`: widgets present or absent per
    /// section (bit mask over the seven `Find` calls), entries found or not
    /// (mask over the four `getEntryByData` calls), the optional seventh list
    /// on or off (`D_004A1A70`), random option struct, and unwinds in the
    /// callees. The 0x289dc0 / 0x289d60 sound-mode getters run for real on
    /// the audio object the fake returns.
    #[test]
    fn cfe_state_options_sound_refresh_controls_matches() {
        oc836_install_fakes();
        check(
            ee::s02::f_0018b380__generated,
            cfe_state_options_sound_refresh_controls,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x18b380, 55);
                c.bus.write32(OC836_CTRL, OC836_MAGIC);
                c.bus.write32(OC836_FIND_N, 0);
                c.bus.write32(OC836_ENTRY_N, 0);
                // Mostly present widgets, a few seeds with absent ones.
                let absent = match seed % 4 {
                    0 => 0,
                    1 => r.next_u32() & r.next_u32(),
                    2 => r.next_u32(),
                    _ => 0x7f & r.next_u32(),
                };
                c.bus.write32(OC836_FIND_MASK, absent);
                c.bus.write32(OC836_ENTRY_MASK, r.next_u32());
                c.bus.write32(OC836_FLAG, if seed % 3 == 0 { 0 } else { 1 });
                fill(c, &mut r, OC836_BEOPT, 0xa2);
                for i in 0..0x300u32 {
                    c.bus
                        .write32(crate::difftest::SP - 4 * (i + 1), r.next_u32());
                }
                fill(c, &mut r, OC836_SELF, 0x20);
                c.g[A0][0] = sx(OC836_SELF);
                c.pc = 0x0018_b380;
            },
        );
    }
}
