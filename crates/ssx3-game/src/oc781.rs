//! Staging module oc781 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc781`.

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
///
/// `PATH_PlatInit` and `SongInstance_Create` already have hand-written bodies
/// in `game/sound/bankmonitor.rs` (`path_plat_init`, `song_instance_create`)
/// that were never registered or tested; this module registers them and holds
/// their tests.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002f_64e8,
        rider_base_reset_cheat_code,
        "RiderBase_ResetCheatCode",
    ),
    (0x003d_2290, path_init, "PATH_Init"),
    (
        0x0019_fbe0,
        fe_module_rider_preview_request_bolt_textures,
        "FEModule_RiderPreviewRequestBoltTextures",
    ),
    (
        0x002b_1b98,
        crate::game::sound::bankmonitor::song_instance_create,
        "SongInstance_Create",
    ),
    (
        0x002a_e048,
        crate::game::sound::bankmonitor::path_plat_init,
        "PATH_PlatInit",
    ),
    (
        0x002a_1bd8,
        ssx_audio_system_get_speech_context_mask,
        "SSXAudioSystem_GetSpeechContextMask",
    ),
];

/// `void RiderBase_ResetCheatCode(RiderBase* self)` (0x2f64e8): an empty
/// function (the cheat-code state is cleared by `ClearCheatCode`).
pub fn rider_base_reset_cheat_code(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `int PATH_Init(void)` (0x3d2290): returns -18 when the path system is
/// already initialised. Otherwise zeroes, for each of the four path slots
/// (stride 0x928 at `0x517610`), the words at +0x34 and +0x924 and the 24
/// words at +0x58..+0xb4, then sets the init flag (0x44f42c) to 1, the
/// 0x44f43c field to 0x100, clears 0x44f420 and 0x44f42d and returns 0.
pub fn path_init(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x450000;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0xbd0));
    let t = c.g[V1][0] == 0u64;
    c.g[V0][0] = 0x510000;
    if !t {
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[V0][0] = 0xffff_ffff_ffff_ffee;
        c.pc = ra;
        return;
    }
    c.cycles += 8;
    c.g[A1][0] = 0u64;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7610));
    c.g[T0][0] = 0x928;
    c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x58));
    c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
    c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x924));
    c.g[A3][0] = 0x928;
    c.mult(0, 2, 5, 7);
    'l_3d22c8: loop {
        c.cycles += 10;
        c.mult(0, 3, 5, 8);
        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
        c.g[A1][0] = 0x17;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T3][0] as u32));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.sw(c.g[V1][0] as u32, 0u64);
        loop {
            c.cycles += 7;
            c.sw(c.g[A0][0] as u32, 0u64);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            let more = (c.g[A1][0] as i64) >= 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003d_22f0);
                }
                continue;
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[A2][0];
            c.g[V0][0] = ((c.g[A1][0] as i64) < 4i64) as u64;
            let again = c.g[V0][0] != 0u64;
            c.mult(0, 2, 5, 7);
            if again {
                if event_due(c) {
                    deliver_event_here(c, 0x003d_22c8);
                }
                continue 'l_3d22c8;
            }
            c.cycles += 12;
            c.g[V1][0] = 0x1;
            c.g[V0][0] = 0x450000;
            c.sb((c.g[V0][0] as u32).wrapping_sub(0xbd4), c.g[V1][0]);
            c.g[A1][0] = 0x100;
            c.g[V1][0] = 0x450000;
            c.g[A0][0] = 0x450000;
            c.sw((c.g[V1][0] as u32).wrapping_sub(0xbc4), c.g[A1][0]);
            c.g[V0][0] = 0u64;
            c.g[V1][0] = 0x450000;
            c.sw((c.g[A0][0] as u32).wrapping_sub(0xbe0), 0u64);
            let ra = c.g[RA][0] as u32;
            c.sb((c.g[V1][0] as u32).wrapping_sub(0xbd3), 0u64);
            c.pc = ra;
            return;
        }
    }
}

/// `void FEModule_RiderPreviewRequestBoltTextures(FERider* self, BoltList* list,
/// ?, bool keepLoaded)` (0x19fbe0, frame 0x190): releases the rider's current
/// bolt textures, clears the two done flags (+0xcb8, +0xcc0), then for every
/// body-part entry of the character whose bolt has no texture slot yet and
/// carries flag 0x10, builds its file name, acquires the bolt file and records
/// the slot; finally prioritises the character's textures.
pub fn fe_module_rider_preview_request_bolt_textures(c: &mut Cpu) {
    guest(c, |c| {
        'b_19fd20: {
            c.cycles += 37;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x190));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
            c.g[S1][0] = c.g[A0][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[S4][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
            c.g[RA][0] = sx(0x0019fc18);
            c.g[S7][0] = c.g[A3][0];
            call_ptr(c, 0x0014dc80, 0x0019fc18)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0019fc24);
            c.g[A1][0] = 0x9;
            call_ptr(c, 0x0014dd58, 0x0019fc24)?;
            c.g[RA][0] = sx(0x0019fc2c);
            c.g[S6][0] = c.g[V0][0];
            call_ptr(c, 0x0014bdb8, 0x0019fc2c)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
            c.g[RA][0] = sx(0x0019fc48);
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a70));
            call_ptr(c, 0x0019fa78, 0x0019fc48)?;
            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(0xcb8), 0u64);
            c.g[RA][0] = sx(0x0019fc5c);
            c.sw((c.g[S1][0] as u32).wrapping_add(0xcc0), 0u64);
            call_ptr(c, 0x0014d988, 0x0019fc5c)?;
            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
            c.g[S3][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0019fc6c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0014d998, 0x0019fc6c)?;
            let empty = (c.g[S3][0] as i64) <= 0;
            c.g[S2][0] = c.g[V0][0];
            if empty {
                break 'b_19fd20;
            }
            c.cycles += 1;
            'l_19fc78: loop {
                'b_19fca4: {
                    'b_19fca0: {
                        c.cycles += 7;
                        c.g[V0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x288));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                        let neg = (c.g[V0][0] as i64) < 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if neg {
                            break 'b_19fca0;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        break 'b_19fca4;
                    }
                    c.cycles += 1;
                    c.g[S0][0] = 0u64;
                }
                'b_19fd18: {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x0019fcb4);
                    c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                    call_ptr(c, 0x0019cce8, 0x0019fcb4)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                        break 'b_19fd18;
                    }
                    'b_19fd14: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                        c.g[V0][0] &= 0x10;
                        let no_flag = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S6][0];
                        if no_flag {
                            break 'b_19fd14;
                        }
                        c.cycles += 7;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                        c.g[A3][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x0019fce0);
                        c.g[T0][0] = c.g[SP][0];
                        call_ptr(c, 0x0014b988, 0x0019fce0)?;
                        let no_name = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S5][0];
                        if no_name {
                            break 'b_19fd14;
                        }
                        c.cycles += 11;
                        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                        c.g[A2][0] = c.g[SP][0];
                        c.g[A3][0] = 0xb;
                        c.g[T0][0] = c.g[S7][0];
                        c.g[RA][0] = sx(0x0019fd00);
                        c.g[T1][0] = 0x1;
                        call_ptr(c, 0x0019da10, 0x0019fd00)?;
                        c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                        c.g[RA][0] = sx(0x0019fd14);
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        call_ptr(c, 0x0019cd38, 0x0019fd14)?;
                    }
                    c.cycles += 1;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                }
                c.cycles += 2;
                let more = c.g[S3][0] != 0u64;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x38));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019fc78);
                    }
                    continue 'l_19fc78;
                }
                break 'b_19fd20;
            }
        }
        c.cycles += 14;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[RA][0] = sx(0x0019fd2c);
        c.g[A0][0] = c.g[S5][0];
        call_ptr(c, 0x0019e070, 0x0019fd2c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
        c.pc = t;
        Ok(())
    })
}

/// `u32 SSXAudioSystem_GetSpeechContextMask(self, int location, int subLocation)`
/// (0x2a1bd8): maps a location (0..16: one speech-context bit each, through a
/// jump table at 0x482f90) or, for locations 17..21 with a sub-location other
/// than -1 (0xe, 0xf, 0x10, 0x11), a mask bit; 0 when there is none. A leaf:
/// only `v0`, `v1` (one path), `a0` and the cycle count change. Resumes (the
/// jump table can leave the function) go to the generated `__entries`.
pub fn ssx_audio_system_get_speech_context_mask(c: &mut Cpu) {
    if c.pc != 0x002a_1bd8 {
        return ee::s08::f_002a1bd8__entries(c);
    }
    let mut pc: u32 = 0x002a1bd8;
    loop {
        match pc {
            0x002a1bd8 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[A1][0] < 0x11u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x480000;
                if t {
                    pc = 0x002a1c24;
                    continue;
                }
                pc = 0x002a1be4;
                continue;
            }
            0x002a1be4 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2f90));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                let t = c.g[A0][0] as u32;

                pc = t;
                continue;
            }
            0x002a1bfc => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x800;
                c.pc = t;
                return;
            }
            0x002a1c04 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x4000;
                c.pc = t;
                return;
            }
            0x002a1c0c => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x8000;
                c.pc = t;
                return;
            }
            0x002a1c14 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x10000;
                c.pc = t;
                return;
            }
            0x002a1c1c => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x20000;
                c.pc = t;
                return;
            }
            0x002a1c24 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffffffffffffffff;
                let t = c.g[A2][0] == c.g[V0][0];
                c.g[A0][0] = 0x11;
                if t {
                    pc = 0x002a1d3c;
                    continue;
                }
                pc = 0x002a1c30;
                continue;
            }
            0x002a1c30 => {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[A0][0];
                c.g[V0][0] = 0x12;
                if t {
                    pc = 0x002a1c6c;
                    continue;
                }
                pc = 0x002a1c38;
                continue;
            }
            0x002a1c38 => {
                c.cycles += 3;
                c.g[V0][0] = 0xe;
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[V0][0] = 0xf;
                if t {
                    pc = 0x002a1c4c;
                    continue;
                }
                pc = 0x002a1c44;
                continue;
            }
            0x002a1c44 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x1;
                c.pc = t;
                return;
            }
            0x002a1c4c => {
                c.cycles += 2;
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[V0][0] = 0x10;
                if t {
                    pc = 0x002a1c5c;
                    continue;
                }
                pc = 0x002a1c54;
                continue;
            }
            0x002a1c54 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x40;
                c.pc = t;
                return;
            }
            0x002a1c5c => {
                c.cycles += 2;
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x002a1d40;
                    continue;
                }
                pc = 0x002a1c64;
                continue;
            }
            0x002a1c64 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x200;
                c.pc = t;
                return;
            }
            0x002a1c6c => {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x13;
                if t {
                    pc = 0x002a1c98;
                    continue;
                }
                pc = 0x002a1c74;
                continue;
            }
            0x002a1c74 => {
                c.cycles += 3;
                c.g[V0][0] = 0xe;
                let t = c.g[A2][0] != c.g[V0][0];

                if t {
                    pc = 0x002a1c88;
                    continue;
                }
                pc = 0x002a1c80;
                continue;
            }
            0x002a1c80 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x2;
                c.pc = t;
                return;
            }
            0x002a1c88 => {
                c.cycles += 2;
                let t = c.g[A2][0] != c.g[A0][0];
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x002a1d40;
                    continue;
                }
                pc = 0x002a1c90;
                continue;
            }
            0x002a1c90 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x1000;
                c.pc = t;
                return;
            }
            0x002a1c98 => {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x14;
                if t {
                    pc = 0x002a1cd4;
                    continue;
                }
                pc = 0x002a1ca0;
                continue;
            }
            0x002a1ca0 => {
                c.cycles += 3;
                c.g[V0][0] = 0xe;
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[V0][0] = 0x10;
                if t {
                    pc = 0x002a1cb4;
                    continue;
                }
                pc = 0x002a1cac;
                continue;
            }
            0x002a1cac => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x4;
                c.pc = t;
                return;
            }
            0x002a1cb4 => {
                c.cycles += 2;
                let t = c.g[A2][0] != c.g[V0][0];

                if t {
                    pc = 0x002a1cc4;
                    continue;
                }
                pc = 0x002a1cbc;
                continue;
            }
            0x002a1cbc => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x400;
                c.pc = t;
                return;
            }
            0x002a1cc4 => {
                c.cycles += 2;
                let t = c.g[A2][0] != c.g[A0][0];
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x002a1d40;
                    continue;
                }
                pc = 0x002a1ccc;
                continue;
            }
            0x002a1ccc => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x2000;
                c.pc = t;
                return;
            }
            0x002a1cd4 => {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x15;
                if t {
                    pc = 0x002a1d00;
                    continue;
                }
                pc = 0x002a1cdc;
                continue;
            }
            0x002a1cdc => {
                c.cycles += 3;
                c.g[V0][0] = 0xe;
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[V0][0] = 0xf;
                if t {
                    pc = 0x002a1cf0;
                    continue;
                }
                pc = 0x002a1ce8;
                continue;
            }
            0x002a1ce8 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x8;
                c.pc = t;
                return;
            }
            0x002a1cf0 => {
                c.cycles += 2;
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x002a1d40;
                    continue;
                }
                pc = 0x002a1cf8;
                continue;
            }
            0x002a1cf8 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x80;
                c.pc = t;
                return;
            }
            0x002a1d00 => {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x002a1d40;
                    continue;
                }
                pc = 0x002a1d08;
                continue;
            }
            0x002a1d08 => {
                c.cycles += 3;
                c.g[V0][0] = 0xe;
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[V0][0] = 0xf;
                if t {
                    pc = 0x002a1d1c;
                    continue;
                }
                pc = 0x002a1d14;
                continue;
            }
            0x002a1d14 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x10;
                c.pc = t;
                return;
            }
            0x002a1d1c => {
                c.cycles += 2;
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[V1][0] = 0x10;
                if t {
                    pc = 0x002a1d2c;
                    continue;
                }
                pc = 0x002a1d24;
                continue;
            }
            0x002a1d24 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x100;
                c.pc = t;
                return;
            }
            0x002a1d2c => {
                c.cycles += 2;
                let t = c.g[A2][0] == c.g[V1][0];
                c.g[V0][0] = 0x800;
                if t {
                    pc = 0x002a1d40;
                    continue;
                }
                pc = 0x002a1d34;
                continue;
            }
            0x002a1d34 => {
                c.cycles += 2;
                let t = c.g[A2][0] == c.g[A0][0];
                c.g[V0][0] = 0x4000;
                if t {
                    pc = 0x002a1d40;
                    continue;
                }
                pc = 0x002a1d3c;
                continue;
            }
            0x002a1d3c => {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                pc = 0x002a1d40;
                continue;
            }
            0x002a1d40 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;

                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x002a1bd8, 0x002a1d48);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, check_lohi, elf_image, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gated fakes (the suite runs tests in parallel): a fake runs only while
    /// this magic is in the machine's RAM, else the body its slot held.
    pub(crate) const OC781_CTRL: u32 = DATA + 0x2b_0000;
    pub(crate) const OC781_MAGIC: u32 = 0x0781_c001;

    /// Per-callee forced results, one word per fake at `OC781_MAIL + (addr & 0xffffc)`.
    pub(crate) const OC781_MAIL: u32 = DATA + 0x2c_0000;

    /// Result table of the texture-slot fake (0x19cce8: 8 words, indexed by
    /// the id argument & 7).
    pub(crate) const OC781_TABLES: u32 = OC781_CTRL + 0x100;

    pub(crate) fn oc781_mail(addr: u32) -> u32 {
        OC781_MAIL + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (sign-extended).
    pub(crate) fn oc781_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc781_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (else the fake's pointer) unless it unwound.
    pub(crate) fn oc781_fake_body(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let a2 = c.g[A2][0] as u32;
        let mail = c.bus.read32(oc781_mail(addr));
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match addr {
            0x0019_cce8 => c.g[V0][0] = sx(c.bus.read32(OC781_TABLES + 4 * (a2 & 7))),
            _ if mail != 0 => c.g[V0][0] = sx(mail.wrapping_sub(3)),
            _ => {}
        }
    }

    macro_rules! oc781_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC781_CTRL) != OC781_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc781_fake_body(c, $addr);
                }
            )*
            pub(crate) fn oc781_install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc781_fakes!(
        (
            oc781_fake_14dc80,
            OC781_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc781_fake_14dd58,
            OC781_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc781_fake_14bdb8,
            OC781_PREV_14BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8
        ),
        (
            oc781_fake_14d988,
            OC781_PREV_14D988,
            ee::s01::OVERRIDE_0014D988,
            ee::s01::f_0014d988__generated,
            0x0014_d988
        ),
        (
            oc781_fake_14d998,
            OC781_PREV_14D998,
            ee::s01::OVERRIDE_0014D998,
            ee::s01::f_0014d998__generated,
            0x0014_d998
        ),
        (
            oc781_fake_14b988,
            OC781_PREV_14B988,
            ee::s01::OVERRIDE_0014B988,
            ee::s01::f_0014b988__generated,
            0x0014_b988
        ),
        (
            oc781_fake_19cce8,
            OC781_PREV_19CCE8,
            ee::s03::OVERRIDE_0019CCE8,
            ee::s03::f_0019cce8__generated,
            0x0019_cce8
        ),
        (
            oc781_fake_19da10,
            OC781_PREV_19DA10,
            ee::s03::OVERRIDE_0019DA10,
            ee::s03::f_0019da10__generated,
            0x0019_da10
        ),
        (
            oc781_fake_19cd38,
            OC781_PREV_19CD38,
            ee::s03::OVERRIDE_0019CD38,
            ee::s03::f_0019cd38__generated,
            0x0019_cd38
        ),
        (
            oc781_fake_19dc20,
            OC781_PREV_19DC20,
            ee::s03::OVERRIDE_0019DC20,
            ee::s03::f_0019dc20__generated,
            0x0019_dc20
        ),
        (
            oc781_fake_418ef8,
            OC781_PREV_418EF8,
            ee::s15::OVERRIDE_00418EF8,
            ee::s15::f_00418ef8__generated,
            0x0041_8ef8
        ),
        (
            oc781_fake_417828,
            OC781_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc781_fake_317d70,
            OC781_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc781_fake_317e30,
            OC781_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc781_fake_2b4a20,
            OC781_PREV_2B4A20,
            ee::s08::OVERRIDE_002B4A20,
            ee::s08::f_002b4a20__generated,
            0x002b_4a20
        ),
        (
            oc781_fake_2b21e0,
            OC781_PREV_2B21E0,
            ee::s08::OVERRIDE_002B21E0,
            ee::s08::f_002b21e0__generated,
            0x002b_21e0
        ),
        (
            oc781_fake_3b5910,
            OC781_PREV_3B5910,
            ee::s13::OVERRIDE_003B5910,
            ee::s13::f_003b5910__generated,
            0x003b_5910
        ),
        (
            oc781_fake_3e57f8,
            OC781_PREV_3E57F8,
            ee::s14::OVERRIDE_003E57F8,
            ee::s14::f_003e57f8__generated,
            0x003e_57f8
        ),
    );

    /// Common start of a fixture: scrambled registers, the fake hooks, the
    /// gate word and the game's gp.
    fn oc781_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc781_install();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC781_CTRL, OC781_MAGIC);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// `RiderBase_ResetCheatCode`: an empty function.
    #[test]
    fn rider_base_reset_cheat_code_matches() {
        check(
            ee::s09::f_002f64e8__generated,
            rider_base_reset_cheat_code,
            4,
            &|c, _| {
                c.pc = 0x002f_64e8;
            },
        );
    }

    /// `PATH_Init`: already initialised (-18, every third seed), else all four
    /// path slots cleared over pre-set words, with LO/HI compared.
    #[test]
    fn path_init_matches() {
        check_lohi(ee::s14::f_003d2290__generated, path_init, 24, &|c, seed| {
            let mut r = Rng(0x3d22_90 ^ (seed as u64) << 8);
            crate::difftest::scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(
                0x0044_f430,
                if seed % 3 == 0 { r.next_u32() | 1 } else { 0 },
            );
            for i in 0..4u32 {
                let base = 0x0051_7610 + 0x928 * i;
                for off in (0..0x928).step_by(4) {
                    c.bus.write32(base + off, r.next_u32());
                }
            }
            c.pc = 0x003d_2290;
        });
    }

    pub(crate) const BOLTTEX_THIS: u32 = DATA + 0x2000;
    pub(crate) const BOLTTEX_LIST: u32 = DATA + 0x3000;
    pub(crate) const BOLTTEX_MAP: u32 = DATA + 0x3800;
    pub(crate) const BOLTTEX_PARTS: u32 = DATA + 0x4000;

    /// A character with -1..5 body parts (the 0x19fa78 release and the 0x19e070
    /// prioritise run for real; the file manager, slot table and name builder
    /// are gated fakes), bolts mapped to list items or to none (-1), texture
    /// slots present or not, items with and without the 0x10 flag, names found
    /// or not, a negative char or profile index now and then (the release
    /// returns early) and a fake unwinding in some call.
    #[test]
    fn fe_module_rider_preview_request_bolt_textures_matches() {
        covered(
            ee::s03::f_0019fbe0__generated,
            fe_module_rider_preview_request_bolt_textures,
            200,
            &|c, seed| {
                let mut r = oc781_machine(c, seed, 0x0019_fbe0, 14);
                let world = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x7c, DATA + 0x1800);
                c.bus.write32(GAME_GP - 0x1788, r.next_u32());
                let char_id = if seed % 10 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 8
                };
                let profile = if seed % 7 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 8
                };
                c.bus.write32(BOLTTEX_THIS, char_id);
                c.bus.write32(BOLTTEX_THIS + 4, profile);
                c.bus.write32(BOLTTEX_THIS + 0xcb8, r.next_u32());
                c.bus.write32(BOLTTEX_THIS + 0xcc0, r.next_u32());
                let n: i32 = [-1, 0, 1, 2, 3, 4, 5][(seed / 3 % 7) as usize];
                oc781_force(c, 0x0014_d988, n as u32);
                oc781_force(c, 0x0014_d998, BOLTTEX_PARTS);
                oc781_force(c, 0x0014_b988, (seed / 2) % 2);
                c.bus.write32(BOLTTEX_LIST + 0x288, BOLTTEX_MAP);
                for j in 0..5u32 {
                    let v: u16 = if r.next_u32() % 4 == 0 {
                        0xffff
                    } else {
                        (r.next_u32() % 5) as u16
                    };
                    c.bus.write16(BOLTTEX_MAP + 2 * j, v);
                    let flags =
                        (r.next_u32() & 0xe0) | if r.next_u32() % 2 == 0 { 0x10 } else { 0 };
                    c.bus
                        .write16(BOLTTEX_LIST + 0x290 + 4 * j, r.next_u32() as u16);
                    c.bus.write16(BOLTTEX_LIST + 0x292 + 4 * j, flags as u16);
                }
                for i in 0..5u32 {
                    let part = BOLTTEX_PARTS + 0x38 * i;
                    c.bus
                        .write8(part + 3, if r.next_u32() % 3 == 0 { 6 } else { 2 });
                    c.bus.write16(part + 4, (r.next_u32() % 5) as u16);
                }
                for k in 0..8u32 {
                    let slot = if r.next_u32() % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 8
                    };
                    c.bus.write32(OC781_TABLES + 4 * k, slot);
                }
                c.g[A0][0] = sx(BOLTTEX_THIS);
                c.g[A1][0] = sx(BOLTTEX_LIST);
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// `SSXAudioSystem_GetSpeechContextMask` over locations 0..24 (the jump
    /// table needs the game image) and sub-locations -1, 0xd..0x12 and others.
    #[test]
    fn ssx_audio_system_get_speech_context_mask_matches() {
        check(
            ee::s08::f_002a1bd8__generated,
            ssx_audio_system_get_speech_context_mask,
            600,
            &|c, seed| {
                let mut r = Rng(0x2a_1bd8 ^ (seed as u64) << 8);
                crate::difftest::scramble(c, &mut r);
                elf_image(c);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = (seed % 24) as u64;
                c.g[A2][0] = match (seed / 24) % 8 {
                    0 => u64::MAX,
                    1 => 0xe,
                    2 => 0xf,
                    3 => 0x10,
                    4 => 0x11,
                    5 => 0x12,
                    6 => 0,
                    _ => r.next_u32() as u64,
                };
                c.pc = 0x002a_1bd8;
            },
        );
    }

    /// `PATH_PlatInit` (hand-written in `bankmonitor.rs`): clears the 24 path
    /// records over random data, publishes the tables and callbacks and makes
    /// the two (faked) calls, one of which unwinds in some seeds.
    #[test]
    fn path_plat_init_matches() {
        covered(
            ee::s08::f_002ae048__generated,
            crate::game::sound::bankmonitor::path_plat_init,
            48,
            &|c, seed| {
                let mut r = oc781_machine(c, seed, 0x002a_e048, 2);
                for i in 0..24u32 {
                    c.bus.write32(0x004d_3e98 + 4 * i, r.next_u32());
                    for k in 0..10u32 {
                        c.bus.write32(0x004d_3ef8 + 0x28 * i + 4 * k, r.next_u32());
                    }
                }
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.pc = 0x002a_e048;
            },
        );
    }

    pub(crate) const SONGCREATE_P: u32 = DATA + 0x5000;
    pub(crate) const SONGCREATE_Q: u32 = DATA + 0x5400;
    pub(crate) const SONGCREATE_R: u32 = DATA + 0x6000;
    pub(crate) const SONGCREATE_V: u32 = DATA + 0x7000;

    /// `SongInstance_Create` (hand-written in `bankmonitor.rs`): allocation
    /// (fake) and the real `SongInstance_ctor` with random volume tables,
    /// section index, option words, a stack argument that is zero or not (the
    /// update call) and a fake unwinding in some call.
    #[test]
    fn song_instance_create_matches() {
        covered(
            ee::s08::f_002b1b98__generated,
            crate::game::sound::bankmonitor::song_instance_create,
            120,
            &|c, seed| {
                let mut r = oc781_machine(c, seed, 0x002b_1b98, 6);
                elf_image(c);
                c.bus.write32(OC781_CTRL, OC781_MAGIC);
                c.bus.write32(GAME_GP + 0x6fc, DATA + 0x8000);
                c.bus.write32(DATA + 0x8000, DATA + 0x8100);
                c.bus.write32(DATA + 0x8100, DATA + 0x8200);
                for off in (0..0x100u32).step_by(4) {
                    c.bus.write32(SONGCREATE_P + off, r.next_u32());
                }
                c.bus.write32(SONGCREATE_P + 0x88, r.next_u32() % 256);
                c.bus.write32(SONGCREATE_P + 0xa4, seed % 2);
                c.bus.write32(SONGCREATE_Q + 0x1c, r.next_u32() % 4);
                c.bus.write32(SONGCREATE_Q + 0x20, SONGCREATE_R);
                c.bus.write32(SONGCREATE_Q + 0x38, SONGCREATE_V);
                for i in 0..0x40u32 {
                    c.bus.write32(SONGCREATE_R + 4 * i, r.next_u32());
                }
                for i in 0..4u32 {
                    let v: u32 =
                        [0, 1, 126, 127, 128, 200, 0xffff_ffff, 50][(r.next_u32() % 8) as usize];
                    c.bus.write32(SONGCREATE_V + 4 * i, v);
                }
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = sx(DATA + 0x9100);
                c.g[A2][0] = sx(SONGCREATE_P);
                c.g[A3][0] = sx(SONGCREATE_Q);
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = r.next_u32() as u64;
                c.g[T2][0] = r.next_u32() as u64;
                c.g[T3][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.pc = 0x002b_1b98;
            },
        );
    }
}
