//! Staging module oc791 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc791`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002a_39e0,
        ssx_audio_system_pa_say_20bb,
        "SSXAudioSystem_PASay20BB",
    ),
    (
        0x0028_f5b8,
        ssx_audio_system_on_event_select_enter,
        "SSXAudioSystem_OnEventSelectEnter",
    ),
    (
        0x0028_d8a0,
        ssx_audio_system_get_track_music_group,
        "SSXAudioSystem_GetTrackMusicGroup",
    ),
    (
        0x0018_6658,
        cfe_state_event_select_init,
        "cFEStateEventSelect_Init",
    ),
];

// 0x00289470 SSXAudioSystem_ReadPlaylistNames: a `loop { match pc` block
// dispatcher with no `__entries` twin, skipped.

/// `SSXAudioSystem_PASay20BB(self)` (0x002a39e0): when PA speech is on, set
/// speech event context `0x20bb` on the speech system (`self + 0x5560`), and
/// if it was stored, play the PA line: the location's language and context
/// masks go to the sound call through the function pointer at `gp + 0x173c`
/// (`fn(handle(1, 0, 0x20bb), 2, languageMask, contextMask)`).
pub fn ssx_audio_system_pa_say_20bb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0029_f128,
            f: ee::s08::SSXAudioSystem_IsPASpeechOn,
        }
        .call(c, 0x002a_39f8)?;
        let off = c.g[V0][0] == 0;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
        let mut tail_cycle = true;
        if !off {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20bb;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xb;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            Callee {
                addr: 0x002b_1458,
                f: ee::s08::SpeechSystem_SetEventContext,
            }
            .call(c, 0x002a_3a20)?;
            if c.g[V0][0] == 0 {
                tail_cycle = false;
            } else {
                c.cycles += 24;
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0029_f198,
                    f: ee::s08::SSXAudioSystem_GetLanguageMask,
                }
                .call(c, 0x002a_3a30)?;
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, 0x0028_b1e8, 0x002a_3a38)?;
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dd58, 0x002a_3a44)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_4bc0, 0x002a_3a4c)?;
                c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 0xffff_ffff_ffff_ffff;
                Callee {
                    addr: 0x002a_1bd8,
                    f: ee::s08::SSXAudioSystem_GetSpeechContextMask,
                }
                .call(c, 0x002a_3a5c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20bb;
                call_ptr(c, 0x003d_8008, 0x002a_3a70)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x173c));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S0][0];
                let target = c.g[V1][0] as u32;
                c.g[A1][0] = 2;
                call_ptr(c, target, 0x002a_3a88)?;
            }
        }
        if tail_cycle {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_OnEventSelectEnter(self)` (0x0028f5b8): clears the
/// "paused SFX for map screen" flag (`+0x6088`); when no audio pause is
/// active (`+0x5fb4 == 0`) and the game is in-game: pause the SFX unless
/// `+0x5818` is set (and remember it in `+0x6088`), play menu sound 9, and
/// unless the radio mode (`+0x608c`) is 2 start the loading-screen loop on
/// the music system (`self + 0x118`: volume 1.0, loop, vtable call, 0x7f).
pub fn ssx_audio_system_on_event_select_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x6088), 0);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5fb4));
        let paused = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x20);
        'out: {
            if paused {
                break 'out;
            }
            c.cycles += 4;
            Callee {
                addr: 0x0028_b1b0,
                f: ee::s07::SSXAudioSystem_IsInGame,
            }
            .call(c, 0x0028_f5e4)?;
            let not_in_game = c.g[V0][0] == 0;
            frame.restore(c, S0, 0x20);
            if not_in_game {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5818));
            let skip_pause = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if !skip_pause {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0029_ce28, 0x0028_f600)?;
                c.g[V0][0] = 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6088), c.g[V0][0]);
                c.g[A0][0] = c.g[S1][0];
            }
            c.cycles += 6;
            c.g[A1][0] = 9;
            call_ptr(c, 0x0029_4f78, 0x0028_f614)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x608c));
            c.g[V0][0] = 2;
            let radio2 = c.g[V1][0] == c.g[V0][0];
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x118));
            if !radio2 {
                c.cycles += 15;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 12);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002b_3d48, 0x0028_f634)?;
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0028_f768,
                    f: ee::s07::SSXAudioSystem_PlayLoadingScreenLoop,
                }
                .call(c, 0x0028_f63c)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5558));
                c.g[A1][0] = 0;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0028_f654)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x7f;
                call_ptr(c, 0x002b_3c28, 0x0028_f660)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_GetTrackMusicGroup(self)` (0x0028d8a0): classify the
/// current track of the location's music: asks five predicates
/// (`0x2a4318`, `0x2a4368`, `0x2a40e0`, `0x2a4158`, `0x2a41c8`; the first
/// that holds gives 0, 1, 0, 1, 2) and otherwise `0x2a4238` gives 3 or 4.
pub fn ssx_audio_system_get_track_music_group(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0028_d8b8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0028_d8c4)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x0028_d8cc)?;
        c.g[S1][0] = c.lw(c.g[V0][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_4318, 0x0028_d8d8)?;
        'done: {
            let hit = c.g[V0][0] != 0;
            c.g[A0][0] = 0;
            if hit {
                break 'done;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_4368, 0x0028_d8e8)?;
            let hit = c.g[V0][0] != 0;
            c.g[A0][0] = 1;
            if hit {
                break 'done;
            }
            // (predicate, group of the hit)
            for (target, ret, group) in [
                (0x002a_40e0u32, 0x0028_d8fcu32, 0u64),
                (0x002a_4158, 0x0028_d910, 1),
                (0x002a_41c8, 0x0028_d924, 2),
            ] {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, target, ret)?;
                let hit = c.g[V0][0] != 0;
                // The delay slot of each test loads the group of its hit.
                c.g[A0][0] = group;
                if hit {
                    break 'done;
                }
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x002a_4238, 0x0028_d938)?;
            c.g[A0][0] = 4;
            c.g[V1][0] = 3;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = c.g[V1][0];
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIFontSet_LoadFonts(self)` (0x00398568): for each of the `count` (signed
/// byte at `+0x34`) font entries (12 bytes from `+4`, the name at `+4` of the
/// entry) create the font resource `0x84` (`0x4a4780`), load it by name
/// (`0x3915e8`) and store the result at `entry + 0xc`.
#[cfg(test)] // not installed: its test runs away in unslotted callees
#[allow(dead_code)]
pub fn ui_font_set_load_fonts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x34));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 4;
            c.g[S5][0] = 0x4a_0000;
            c.g[S4][0] = 0xc;
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(4));
            c.g[S0][0] = sx((c.g[S1][0] as u32) << 24);
            loop {
                c.cycles += 19;
                c.g[A0][0] = 0x84;
                c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4780));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0039_85c0)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(4));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0039_15e8, 0x0039_85d8)?;
                c.mult(0, 3, 16, 20);
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[V0][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S1][0] as u32) << 24);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_85a8);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateEventSelect_Init(self)` (0x00186658): load the two event-select
/// screen resources (`0x4a14b0` into `self+0x40` and `0x45dc50`, both from
/// the object at `+0x10`, a non-zero result is passed to `0x39c870(r, 0, 0)`),
/// clear the global at `gp-0xb5c` and call `0x200700(self+0x48, self[0x40], self)`.
pub fn cfe_state_event_select_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x4a_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14b0));
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_7670, 0x0018_667c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_6690)?;
        let none = c.g[V0][0] == 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if !none {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_66a8)?;
        }
        c.cycles += 11;
        c.g[A0][0] = 0x46_0000;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x23b0));
        call_ptr(c, 0x0031_7670, 0x0018_66b8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_66cc)?;
        let none = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[V0][0];
        if !none {
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_66e0)?;
        }
        c.cycles += 10;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x48));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0xb5c), 0);
        call_ptr(c, 0x0020_0700, 0x0018_66f4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA, GAME_GP};

    /// A game-like object area well above the fakes' pointer results.
    const OC791_OBJ: u32 = DATA + 0x8_0000;
    /// The world / game-module record `gp-0x848` points at.
    const OC791_WORLD: u32 = DATA + 0x9_0000;
    /// Per-machine mask word: bit n (n < 31) forces the n-th fake call to
    /// return 0.
    const OC791_MASK: u32 = DATA + 0x20_0000;

    /// Gate word of the oc791 fakes (a magic value in the machine's RAM).
    const OC791_CTRL: u32 = DATA + 0x21_0000;
    const OC791_MAGIC: u32 = 0x4f43_3739;

    /// The standard fake call, then optionally a zero result.
    fn oc791_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc == ra {
            let n = c.bus.read32(fakes::LOG_COUNT).wrapping_sub(1);
            let m = c.bus.read32(OC791_MASK);
            if n < 31 && (m >> n) & 1 != 0 {
                c.g[V0][0] = 0;
            }
        }
    }

    macro_rules! oc791_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC791_CTRL) != OC791_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc791_miss(c, $addr);
                }
            )*
            /// Hook the gated fakes on the callee slots (once per process)
            /// and open the gate in this machine.
            fn oc791_install_fakes(c: &mut Cpu) {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
                c.bus.write32(OC791_CTRL, OC791_MAGIC);
            }
        };
    }

    oc791_fakes!(
        (
            oc791_f_14dc80,
            OC791_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc791_f_14dd58,
            OC791_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc791_f_144bc0,
            OC791_PREV_144BC0,
            ee::s01::OVERRIDE_00144BC0,
            ee::s01::f_00144bc0__generated,
            0x0014_4bc0
        ),
        (
            oc791_f_317670,
            OC791_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc791_f_317d70,
            OC791_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc791_f_397cc0,
            OC791_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc791_f_39c870,
            OC791_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc791_f_2a4318,
            OC791_PREV_2A4318,
            ee::s08::OVERRIDE_002A4318,
            ee::s08::f_002a4318__generated,
            0x002a_4318
        ),
        (
            oc791_f_2a4368,
            OC791_PREV_2A4368,
            ee::s08::OVERRIDE_002A4368,
            ee::s08::f_002a4368__generated,
            0x002a_4368
        ),
        (
            oc791_f_2a40e0,
            OC791_PREV_2A40E0,
            ee::s08::OVERRIDE_002A40E0,
            ee::s08::f_002a40e0__generated,
            0x002a_40e0
        ),
        (
            oc791_f_2a4158,
            OC791_PREV_2A4158,
            ee::s08::OVERRIDE_002A4158,
            ee::s08::f_002a4158__generated,
            0x002a_4158
        ),
        (
            oc791_f_2a41c8,
            OC791_PREV_2A41C8,
            ee::s08::OVERRIDE_002A41C8,
            ee::s08::f_002a41c8__generated,
            0x002a_41c8
        ),
        (
            oc791_f_2a4238,
            OC791_PREV_2A4238,
            ee::s08::OVERRIDE_002A4238,
            ee::s08::f_002a4238__generated,
            0x002a_4238
        ),
        (
            oc791_f_3de670,
            OC791_PREV_3DE670,
            ee::s14::OVERRIDE_003DE670,
            ee::s14::f_003de670__generated,
            0x003d_e670
        ),
    );

    /// `SSXAudioSystem_PASay20BB`: speech on/off (volume 0.0), the context
    /// table free or full, and unwinds in each call.
    #[test]
    fn ssx_audio_system_pa_say_20bb_matches() {
        check(
            ee::s08::f_002a39e0__generated,
            ssx_audio_system_pa_say_20bb,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a39e0, 9);
                c.hooks.miss = oc791_miss;
                c.pc = 0x002a_39e0;
                c.g[A0][0] = sx(OC791_OBJ);
                oc791_install_fakes(c);
                c.bus.write32(OC791_MASK, 0);
                // Channel 3 volume (+0x62ec): 0.0 or 1.0.
                c.bus.write32(
                    OC791_OBJ + 0x62ec,
                    if seed % 4 == 0 { 0 } else { 0x3f80_0000 },
                );
                c.bus.write32(GAME_GP - 0x848, OC791_WORLD);
                c.bus.write32(OC791_WORLD + 0x78, OC791_OBJ + 0x2_0000);
                c.bus.write32(GAME_GP + 0x173c, fakes::FAKE);
                if seed % 3 == 0 {
                    // Every speech context slot in use.
                    for i in 0..10 {
                        let s = OC791_OBJ + 0x5560 + 0x20 * i;
                        c.bus.write32(s, 1 + r.next_u32() % 3);
                        c.bus.write32(s + 4, r.next_u32() | 0x100);
                        c.bus.write32(s + 8, r.next_u32() % 3);
                        c.bus.write32(s + 0xc, r.next_u32() % 3);
                        c.bus.write32(s + 0x10, r.next_u32());
                        c.bus.write32(s + 0x14, r.next_u32());
                        c.bus.write32(s + 0x18, r.next_u32() | 1);
                        c.bus.write32(s + 0x1c, r.next_u32());
                    }
                }
            },
        );
    }

    /// `SSXAudioSystem_OnEventSelectEnter`: paused, not in game, SFX
    /// already paused, radio mode 2 or not, and unwinds in each call.
    #[test]
    fn ssx_audio_system_on_event_select_enter_matches() {
        fakes::covered(
            ee::s07::f_0028f5b8__generated,
            ssx_audio_system_on_event_select_enter,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28f5b8, 8);
                c.hooks.miss = oc791_miss;
                c.pc = 0x0028_f5b8;
                c.g[A0][0] = sx(OC791_OBJ);
                c.bus.write32(OC791_MASK, 0);
                c.bus.write32(GAME_GP - 0x848, OC791_WORLD);
                c.bus
                    .write32(OC791_WORLD + 0x84, if seed % 7 < 2 { 0 } else { 1 });
                c.bus.write32(OC791_OBJ, OC791_OBJ + 0x1_0000);
                c.bus.write32(OC791_OBJ + 0x5fb4, (seed % 5 == 0) as u32);
                c.bus.write32(OC791_OBJ + 0x5818, (seed % 3 == 0) as u32);
                c.bus.write32(
                    OC791_OBJ + 0x608c,
                    if seed % 4 == 0 { 2 } else { r.next_u32() % 3 },
                );
                c.bus.write32(OC791_OBJ + 0x6088, r.next_u32());
                // The music system's vtable record: {delta, fake method}.
                let vt = OC791_OBJ + 0x7000;
                c.bus.write32(OC791_OBJ + 0x5558, vt);
                c.bus.write16(vt + 0x20, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(vt + 0x24, fakes::FAKE);
            },
        );
    }

    /// `SSXAudioSystem_GetTrackMusicGroup`: the first predicate that holds
    /// (0..5 of the 5 return 0 first), and unwinds.
    #[test]
    fn ssx_audio_system_get_track_music_group_matches() {
        check(
            ee::s07::f_0028d8a0__generated,
            ssx_audio_system_get_track_music_group,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x28d8a0, 8);
                c.hooks.miss = oc791_miss;
                oc791_install_fakes(c);
                c.pc = 0x0028_d8a0;
                c.g[A0][0] = sx(OC791_OBJ);
                let stop = seed % 6;
                c.bus.write32(OC791_MASK, ((1u32 << stop) - 1) << 3);
            },
        );
    }

    /// `cFEStateEventSelect_Init`: both resource loads returning 0 or not.
    #[test]
    fn cfe_state_event_select_init_matches() {
        check(
            ee::s02::f_00186658__generated,
            cfe_state_event_select_init,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x186658, 6);
                c.hooks.miss = oc791_miss;
                oc791_install_fakes(c);
                c.pc = 0x0018_6658;
                c.g[A0][0] = sx(OC791_OBJ);
                c.bus.write32(OC791_MASK, r.next_u32() & 0x1e);
                c.bus.write32(OC791_OBJ + 0x10, r.next_u32());
                c.bus.write32(GAME_GP - 0xb5c, r.next_u32());
            },
        );
    }
}
