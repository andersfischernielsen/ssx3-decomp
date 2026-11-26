//! Staging module oc752 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc752`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_8618, ui_group_load, "cUIGroup_Load"),
    (
        0x0029_c420,
        ssx_audio_on_start_light,
        "SSXAudioSystem_OnStartLight",
    ),
    (
        0x0029_f198,
        ssx_audio_get_language_mask,
        "SSXAudioSystem_GetLanguageMask",
    ),
    (0x002a_7e60, ini_parser_read_float, "IniParser_ReadFloat"),
];

/// `UIObject::Load` (0x39fe00).
const OC752_UI_OBJECT_LOAD: Callee = Callee {
    addr: 0x0039_fe00,
    f: ee::s13::f_0039fe00,
};

/// `void cUIGroup_Load(cUIGroup* self)` (0x398618): a forwarding wrapper,
/// `UIObject_Load(self)` (7 cycles, a 0x10 frame holding `ra`).
pub fn ui_group_load(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(OC752_UI_OBJECT_LOAD), 0x0039_8628)
}

/// `IniParser::SkipWhitespace` (0x2a78f8).
const OC752_INI_SKIP_WS: Callee = Callee {
    addr: 0x002a_78f8,
    f: ee::s08::f_002a78f8,
};

/// `IniParser::ReadToken` (0x2a7da0).
const OC752_INI_READ_TOKEN: Callee = Callee {
    addr: 0x002a_7da0,
    f: ee::s08::f_002a7da0,
};

/// `float IniParser_ReadFloat(IniParser* self)` (0x2a7e60, `FileParser_GetFloat`):
/// skips blanks, reads the next token into the buffer at `self+0xc`, and
/// returns the float `sscanf(buffer, fmt, &value)` stored (fmt at 0x4a3730,
/// `"%f"`); the result is loaded into `f0` from the stack slot.
pub fn ini_parser_read_float(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        OC752_INI_SKIP_WS.call(c, 0x002a_7e74)?;
        c.g[A0][0] = c.g[S0][0];
        OC752_INI_READ_TOKEN.call(c, 0x002a_7e7c)?;
        c.g[A1][0] = 0x004a_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3730));
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x0041_78b0, 0x002a_7e90)?;
        let value = frame.at(c, 0);
        c.lwc1(0, value);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// Where `SSXAudioSystem_GetLanguageMask` copies the back-end option block
/// from (`cBEOptionInterface` option storage, 0x290 bytes).
pub(crate) const LANGUAGE_MASK_OPTION_SRC: u32 = 0x0053_5610;

/// `int SSXAudioSystem_GetLanguageMask(cSSXAudio* self)` (0x29f198): fetch
/// the back-end option interface (`cBE_getInterface(cBE_getBE(), 4)`, result
/// unused), copy the 0x290-byte option block (an inlined struct copy: 20
/// iterations of four doublewords, then one more doubleword) onto the
/// stack, and map the language field (`word0 >> 22 & 7`) to a mask:
/// 0 -> 1, 1 -> 2, 2 -> 4, 3 -> 8, anything else -> 1.
///
/// The generated copy also has an unaligned path (`ldl`/`ldr` pairs), taken
/// when the source address is not doubleword-aligned; the source is a
/// constant (0x535610, aligned), so that path is dead and left out.
pub fn ssx_audio_get_language_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x2a0);
        frame.save_ra(c, 0x290);
        call_ptr(c, 0x0014_dc80, 0x0029_f1a8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x4;
        call_ptr(c, 0x0014_dd58, 0x0029_f1b4)?;
        c.g[V1][0] = sx(LANGUAGE_MASK_OPTION_SRC);
        c.g[V0][0] = c.g[V1][0] & 0x7;
        debug_assert_eq!(c.g[V0][0], 0);
        c.g[A0][0] = c.g[SP][0];
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
        // 0x280 bytes, 32 per iteration.
        loop {
            c.cycles += 11;
            let src = c.g[V1][0] as u32;
            let dst = c.g[A0][0] as u32;
            c.g[A1][0] = c.ld(src);
            c.g[A2][0] = c.ld(src.wrapping_add(0x8));
            c.g[A3][0] = c.ld(src.wrapping_add(0x10));
            c.g[T0][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[A1][0]);
            c.sd(dst.wrapping_add(0x8), c.g[A2][0]);
            c.sd(dst.wrapping_add(0x10), c.g[A3][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T0][0]);
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0029_f224);
            }
        }
        'b_29f2a8: {
            'b_29f2a4: {
                c.cycles += 10;
                // The trailing doubleword of the block.
                let src = c.g[V1][0] as u32;
                let dst = c.g[A0][0] as u32;
                c.ldl(2, src.wrapping_add(0x7));
                c.ldr(2, src);
                c.sdl(2, dst.wrapping_add(0x7));
                c.sdr(2, dst);
                c.g[V0][0] = Addr(sp(c)).lw(c);
                c.g[V1][0] = 0x1;
                c.g[A0][0] = sx((c.g[V0][0] as u32) >> 22);
                c.g[A0][0] &= 0x7;
                let is_one = c.g[A0][0] == c.g[V1][0];
                c.g[V0][0] = 0x2;
                if is_one {
                    break 'b_29f2a4;
                }
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = 0x1;
                    break 'b_29f2a4;
                }
                'b_29f2a0: {
                    'b_29f298: {
                        c.cycles += 2;
                        let is_two = c.g[A0][0] == c.g[V0][0];
                        c.g[V0][0] = 0x3;
                        if is_two {
                            break 'b_29f298;
                        }
                        c.cycles += 2;
                        let is_three = c.g[A0][0] == c.g[V0][0];
                        c.g[V0][0] = 0x1;
                        if is_three {
                            break 'b_29f2a0;
                        }
                        // Any other language: mask 1, no extra cycle.
                        c.cycles += 2;
                        frame.restore_ra(c, 0x290);
                        break 'b_29f2a8;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0x4;
                    break 'b_29f2a4;
                }
                c.cycles += 1;
                c.g[V0][0] = 0x8;
            }
            c.cycles += 1;
            frame.restore_ra(c, 0x290);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void SSXAudioSystem_OnStartLight(cSSXAudio* self, int mode)` (0x29c420,
/// `cSSXAudio_CountdownBeep`): unless a replay is playing, queue the
/// countdown beep voice (an inlined voice-slot allocation: bump the voice
/// count at `audio+0x1f4`, copy the 0x18-byte voice record, clear and set the
/// per-voice parameter arrays, ask `0x287968` for a handle and store it),
/// then act on the music by `mode`:
/// - 3: if the current song tag is 201, 1, 2 or 3 and `self+0x627c` is set,
///   fade the music out (`MusicPlayer_FadeOut(&music, 2.0)`);
/// - 1: if no song plays (`self+0x530 == 0`) or the tag is 201, 1, 2 or 3,
///   and `self+0x627c` is set, pick a random song, start the music and
///   pause it (`0x28d488`, `0x28cf98(self, 0, 0, -1, 0)`,
///   `MusicPlayer_Pause`).
/// The music object is `self+0x118`.
pub fn ssx_audio_on_start_light(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        'b_29c788: {
            frame.save(c, 19, 0x40);
            frame.save(c, 20, 0x30);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, 16, 0x70);
            frame.save(c, 17, 0x60);
            frame.save(c, 18, 0x50);
            frame.save(c, 21, 0x20);
            frame.save(c, 22, 0x10);
            frame.save_ra(c, 0);
            c.g[S4][0] = c.g[A1][0];
            call_ptr(c, 0x00288ae0, 0x0029c450)?;
            if c.g[V0][0] != 0u64 {
                frame.restore(c, 16, 0x70);
                break 'b_29c788;
            } else {
                'b_29c710: {
                    c.cycles += 154;
                    c.g[S0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[T1][0] = 0x18;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x118));
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                    c.g[S5][0] = c.g[A0][0];
                    c.g[A2][0] = 0x8;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[S6][0] = 0x1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                    c.g[S2][0] = 0x4e;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.mult(0, 3, 3, 9);
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
                    c.mult(0, 4, 4, 9);
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
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                    call_ptr(c, 0x00416210, 0x0029c53c)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A3][0] = 0x64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                    c.g[T0][0] = 0x5a;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[T1][0] = 0x32;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[T2][0] = 0x7f;
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[A1][0] = 0x5;
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.g[S3][0];
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
                    c.sw(c.g[V0][0] as u32, c.g[A3][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[T1][0]);
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
                    c.sw(c.g[V0][0] as u32, c.g[T2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[S6][0]);
                    c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[S2][0]);
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                    call_ptr(c, 0x00287968, 0x0029c694)?;
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    call_ptr(c, 0x002906b8, 0x0029c6b4)?;
                    c.g[V1][0] = 0x3;
                    let t = c.g[S4][0] != c.g[V1][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_29c710;
                    } else {
                        'b_29c6f4: {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S5][0];
                            call_ptr(c, 0x002b49e0, 0x0029c6c8)?;
                            c.g[V1][0] = c.g[V0][0];
                            c.g[V0][0] = 0xc9;
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x627c));
                                break 'b_29c6f4;
                            } else {
                                'b_29c6f0: {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[S6][0];
                                    c.g[V0][0] = 0x2;
                                    if t {
                                        break 'b_29c6f0;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[V0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x627c));
                                            break 'b_29c6f4;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[S4][0];
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_29c710;
                                            } else {
                                                break 'b_29c6f0;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x627c));
                                break 'b_29c6f4;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_29c710;
                        } else {
                            c.cycles += 5;
                            c.g[AT][0] = 0x40000000;
                            c.mtc1(1, 12);
                            c.g[A0][0] = c.g[S5][0];
                            call_ptr(c, 0x002b3d48, 0x0029c70c)?;
                            c.g[V0][0] = 0x1;
                            break 'b_29c710;
                        }
                    }
                }
                c.cycles += 2;
                let t = c.g[S4][0] != c.g[V0][0];
                frame.restore(c, 16, 0x70);
                if t {
                    break 'b_29c788;
                } else {
                    'b_29c754: {
                        'b_29c750: {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S5][0];
                            call_ptr(c, 0x002b49e0, 0x0029c720)?;
                            c.g[V1][0] = c.g[V0][0];
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x530));
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0xc9;
                            if t {
                                break 'b_29c750;
                            } else {
                                c.cycles += 2;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x627c));
                                    break 'b_29c754;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[S4][0];
                                    c.g[V0][0] = 0x2;
                                    if t {
                                        break 'b_29c750;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x3;
                                        if t {
                                            break 'b_29c750;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            frame.restore(c, 16, 0x70);
                                            if t {
                                                break 'b_29c788;
                                            } else {
                                                break 'b_29c750;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x627c));
                        break 'b_29c754;
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    frame.restore(c, 16, 0x70);
                    if t {
                        break 'b_29c788;
                    } else {
                        c.cycles += 11;
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x0028d488, 0x0029c764)?;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A3][0] = 0xffffffffffffffff;
                        c.g[T0][0] = 0u64;
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0u64;
                        call_ptr(c, 0x0028cf98, 0x0029c77c)?;
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x002b3a70, 0x0029c784)?;
                        frame.restore(c, 16, 0x70);
                        break 'b_29c788;
                    }
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore(c, 21, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, fill, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Control word of this module's gated fakes (the fakes only act when
    /// the machine's RAM holds the magic here).
    pub(crate) const OC752_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC752_MAGIC: u32 = 0x4f43_3752;

    /// Per-callee forced results: one word per fake.
    const OC752_FORCE: u32 = DATA + 0x30_0000;

    fn oc752_mail(addr: u32) -> u32 {
        OC752_FORCE + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (any value; stored +3).
    pub(crate) fn oc752_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc752_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// the forced result of its mailbox (0 = the fake's own pointer) unless
    /// it unwound.
    fn oc752_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc752_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 3) as u64;
        }
    }

    macro_rules! oc752_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC752_CTRL) != OC752_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc752_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc752_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc752_fakes!(
        (
            oc752_fake_39fe00,
            OC752_PREV_39FE00,
            ee::s13::OVERRIDE_0039FE00,
            ee::s13::f_0039fe00__generated,
            0x0039_fe00
        ),
        (
            oc752_fake_14dc80,
            OC752_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc752_fake_14dd58,
            OC752_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc752_fake_288ae0,
            OC752_PREV_288AE0,
            ee::s07::OVERRIDE_00288AE0,
            ee::s07::f_00288ae0__generated,
            0x0028_8ae0
        ),
        (
            oc752_fake_287968,
            OC752_PREV_287968,
            ee::s07::OVERRIDE_00287968,
            ee::s07::f_00287968__generated,
            0x0028_7968
        ),
        (
            oc752_fake_2906b8,
            OC752_PREV_2906B8,
            ee::s07::OVERRIDE_002906B8,
            ee::s07::f_002906b8__generated,
            0x0029_06b8
        ),
        (
            oc752_fake_2b49e0,
            OC752_PREV_2B49E0,
            ee::s08::OVERRIDE_002B49E0,
            ee::s08::f_002b49e0__generated,
            0x002b_49e0
        ),
        (
            oc752_fake_2b3d48,
            OC752_PREV_2B3D48,
            ee::s08::OVERRIDE_002B3D48,
            ee::s08::f_002b3d48__generated,
            0x002b_3d48
        ),
        (
            oc752_fake_416210,
            OC752_PREV_416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
    );

    /// `cUIGroup_Load`: the one forwarded call, answered or unwound.
    #[test]
    fn ui_group_load_matches() {
        oc752_install_fakes();
        covered(
            ssx3_ee::s13::f_00398618__generated,
            ui_group_load,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x0039_8618, 1);
                c.bus.write32(OC752_CTRL, OC752_MAGIC);
            },
        );
    }

    /// `IniParser_ReadFloat`: the cursor (self+8) points at text with 0-3
    /// leading blanks, an optional sign, digits, a point and digits; the
    /// real `SkipWhitespace`, `ReadToken` and `sscanf` run.
    #[test]
    fn ini_parser_read_float_matches() {
        check(
            ssx3_ee::s08::f_002a7e60__generated,
            ini_parser_read_float,
            200,
            &|c, seed| {
                let mut r = Rng(0x002a_7e60 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                let this = DATA + 0x100;
                let text = DATA + 0x400;
                c.bus.write32(this + 8, text);
                let mut s = Vec::new();
                for _ in 0..(r.next_u32() % 4) {
                    s.push(b' ');
                }
                if seed % 3 == 0 {
                    s.push(b'-');
                }
                for _ in 0..(1 + r.next_u32() % 4) {
                    s.push(b'0' + (r.next_u32() % 10) as u8);
                }
                if seed % 5 != 0 {
                    s.push(b'.');
                    for _ in 0..(1 + r.next_u32() % 4) {
                        s.push(b'0' + (r.next_u32() % 10) as u8);
                    }
                }
                s.push(b' ');
                for (i, &b) in s.iter().enumerate() {
                    c.bus.write8(text + i as u32, b);
                }
                c.bus.write8(text + s.len() as u32, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `SSXAudioSystem_GetLanguageMask`: the option block at 0x535610 holds
    /// language index `seed % 8` (0 to 3 map to masks, 4 to 7 the default) in
    /// bits 22..24 of its first word; the back-end calls are fakes, some of
    /// which unwind.
    #[test]
    fn ssx_audio_get_language_mask_matches() {
        oc752_install_fakes();
        covered(
            ssx3_ee::s08::f_0029f198__generated,
            ssx_audio_get_language_mask,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_f198, 2);
                c.bus.write32(OC752_CTRL, OC752_MAGIC);
                fill(c, &mut r, LANGUAGE_MASK_OPTION_SRC, 0x290 / 4);
                let first = (r.next_u32() & !(7 << 22)) | ((seed % 8) << 22);
                c.bus.write32(LANGUAGE_MASK_OPTION_SRC, first);
            },
        );
    }

    /// Fixture objects of `OnStartLight`, all clear of the fake log and of
    /// the pointers the fakes return (DATA..DATA+0x40000).
    const SL_SELF: u32 = DATA + 0x21_0000;
    const SL_AUDIO: u32 = DATA + 0x22_0000;
    const SL_MUSIC: u32 = DATA + 0x23_0000;
    const SL_TABLES: u32 = DATA + 0x24_0000;
    const SL_ARR4: u32 = DATA + 0x2c_0000;
    const SL_ARR8: u32 = DATA + 0x2d_0000;

    /// `SSXAudioSystem_OnStartLight`: replay on (a fifth of the seeds) or
    /// off; modes 3, 1 and others; song tags 201, 1, 2, 3, other and 0; song
    /// playing or not; the `+0x627c` flag set only in mode 3 (in mode 1 it
    /// would start `SelectRandomSong` and `PlayMusic`, real unported
    /// callees that cannot be faked: that branch's three calls are
    /// transcribed from the generated body and not reached here).
    #[test]
    fn ssx_audio_on_start_light_matches() {
        oc752_install_fakes();
        covered(
            ssx3_ee::s08::f_0029c420__generated,
            ssx_audio_on_start_light,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_c420, 10);
                c.bus.write32(OC752_CTRL, OC752_MAGIC);
                fill(c, &mut r, SL_SELF, 0x200);
                fill(c, &mut r, SL_AUDIO, 0x200);
                let mode = [3u32, 1, 3, 1, 0, 2][(r.next_u32() % 6) as usize];
                let tag = [0xc9u32, 1, 2, 3, 7, 0][(r.next_u32() % 6) as usize];
                let playing = r.next_u32() % 2;
                if seed % 5 != 0 {
                    oc752_force(c, 0x0028_8ae0, 0);
                }
                oc752_force(c, 0x002b_49e0, tag);
                c.bus.write32(SL_SELF, SL_AUDIO);
                c.bus.write32(SL_SELF + 4, SL_ARR4);
                c.bus.write32(SL_SELF + 8, SL_ARR8);
                c.bus.write32(SL_SELF + 0x118, SL_MUSIC);
                c.bus.write32(SL_MUSIC, SL_AUDIO);
                c.bus.write32(SL_SELF + 0x530, playing);
                let flag = if mode == 3 { r.next_u32() % 2 } else { 0 };
                c.bus.write32(SL_SELF + 0x627c, flag);
                // The voice count, and the per-voice parameter arrays the
                // record is stored into (`audio+0x1d8` + these offsets).
                c.bus.write32(SL_AUDIO + 0x1f4, r.next_u32() % 8);
                let fields = [
                    0x20u32, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c,
                    0x80, 0x84,
                ];
                for (k, off) in fields.into_iter().enumerate() {
                    c.bus
                        .write32(SL_AUDIO + 0x1d8 + off, SL_TABLES + 0x1000 * k as u32);
                }
                c.g[A0][0] = sx(SL_SELF);
                c.g[A1][0] = sx(mode);
            },
        );
    }
}
