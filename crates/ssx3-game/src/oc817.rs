//! Staging module oc817 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc817`.

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
        0x002f_8ff8,
        oc817_attached_emitter_type_data_free_part1,
        "AttachedEmitterTypeData_FreePart1",
    ),
    (
        0x002b_4060,
        oc817_music_player_empty_playlist_hook,
        "MusicPlayer_EmptyPlaylistHook",
    ),
    (
        0x0029_c418,
        oc817_ssx_audio_system_on_countdown_enter,
        "SSXAudioSystem_OnCountdownEnter",
    ),
    (
        0x0028_c8c0,
        oc817_ssx_audio_system_set_countdown_value,
        "SSXAudioSystem_SetCountdownValue",
    ),
    (
        0x0017_d1b8,
        oc817_debug_menu_dispatch_item_action,
        "DebugMenu_DispatchItemAction",
    ),
];

/// `AttachedEmitterTypeData_FreePart1`: an empty function (`jr ra`).
pub fn oc817_attached_emitter_type_data_free_part1(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `MusicPlayer_EmptyPlaylistHook`: an empty function (`jr ra`).
pub fn oc817_music_player_empty_playlist_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `SSXAudioSystem_OnCountdownEnter`: an empty function (`jr ra`).
pub fn oc817_ssx_audio_system_on_countdown_enter(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `SSXAudioSystem_SetCountdownValue(self, value)`: `self->countdown = value`
/// (word at +0x6250).
pub fn oc817_ssx_audio_system_set_countdown_value(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x6250), c.g[A1][0]);
    ret(c);
}

/// `DebugMenu_DispatchItemAction(self, a1, item)`: the AI-rider count for a
/// debug-menu item. Default 5 (4 when `a1 == 2`); items 0..7 jump through the
/// table at 0x45d040 (handler 0x17d1ec gives 0, 0x17d1f4 gives 1).
/// The resume entries run `f_0017d1b8__entries`.
pub fn oc817_debug_menu_dispatch_item_action(c: &mut Cpu) {
    if c.pc != 0x0017d1b8 {
        return ee::s02::f_0017d1b8__entries(c);
    }
    c.cycles += 6;
    c.g[A1][0] ^= 0x2;
    c.g[A0][0] = 0x5;
    c.g[V0][0] = 0x4;
    c.g[V1][0] = (c.g[A2][0] < 0x7u64) as u64;
    let out_of_range = c.g[V1][0] == 0u64;
    if c.g[A1][0] == 0 {
        c.g[A0][0] = c.g[V0][0];
    }
    if !out_of_range {
        c.cycles += 7;
        c.g[V0][0] = 0x460000;
        c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2fc0));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        let t = c.g[A0][0] as u32;
        match t {
            0x0017d1ec => {
                c.cycles += 2;
                c.g[A0][0] = 0u64;
            }
            0x0017d1f4 => {
                c.cycles += 1;
                c.g[A0][0] = 0x1;
            }
            _ => {
                c.jump_out(t, 0x0017d1b8, 0x0017d200);
                return;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A0][0];
    c.pc = t;
}

// Not installed: its callees 0x256038/0x255bf8/0x255d50/0x256bf8 are generated
// code without override slots, and the real network chain runs away on test
// data (allocator loop at pc 0x31ef40), so no differential test can bound it.
// The port is otherwise exact.
#[cfg(test)]
#[allow(dead_code)]
/// `DebugMenu_StartMultiplayer` (0x0017d0a8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn oc817_debug_menu_start_multiplayer(c: &mut Cpu) {
    if c.pc != 0x0017d0a8 {
        // At install: switch this to `f_0017d0a8__generated` (calling the slot
        // wrapper `f_0017d0a8` from here would recurse).
        return ee::s02::f_0017d0a8(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0017d0a8;
        loop {
            match pc {
                0x0017d0a8 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = 0x7;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[RA][0] = sx(0x0017d0c8);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x0017d0c8)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = c.g[V1][0];
                    if t {
                        pc = 0x0017d1a4;
                        continue;
                    }
                    pc = 0x0017d0d8;
                    continue;
                }
                0x0017d0d8 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[RA][0] = sx(0x0017d0e4);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    call_ptr(c, 0x00256038, 0x0017d0e4)?;
                    c.g[V0][0] = 0xa0a0000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.g[V0][0] | 0x3519;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x0017d108;
                        continue;
                    }
                    pc = 0x0017d0f8;
                    continue;
                }
                0x0017d0f8 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2fe8));
                    let t = true;
                    c.g[A0][0] = c.ld((c.g[V0][0] as u32).wrapping_sub(0x2fe8));
                    if t {
                        pc = 0x0017d110;
                        continue;
                    }
                    pc = 0x0017d108;
                    continue;
                }
                0x0017d108 => {
                    c.cycles += 2;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2fd8));
                    c.g[A0][0] = c.ld((c.g[V0][0] as u32).wrapping_sub(0x2fd8));
                    pc = 0x0017d110;
                    continue;
                }
                0x0017d110 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[SP][0] as u32, c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4b30));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[SP][0];
                    if t {
                        pc = 0x0017d13c;
                        continue;
                    }
                    pc = 0x0017d12c;
                    continue;
                }
                0x0017d12c => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    let t = true;
                    c.g[A3][0] = 0x1;
                    if t {
                        pc = 0x0017d148;
                        continue;
                    }
                    pc = 0x0017d13c;
                    continue;
                }
                0x0017d13c => {
                    c.cycles += 3;
                    c.g[A2][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[A3][0] = 0u64;
                    pc = 0x0017d148;
                    continue;
                }
                0x0017d148 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0017d150);
                    call_ptr(c, 0x00255bf8, 0x0017d150)?;
                    let t = true;
                    c.g[S0][0] = c.g[V0][0];
                    if t {
                        pc = 0x0017d170;
                        continue;
                    }
                    pc = 0x0017d158;
                    continue;
                }
                0x0017d158 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x0017d160);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    call_ptr(c, 0x00255a20, 0x0017d160)?;
                    c.g[RA][0] = sx(0x0017d168);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x003e5928, 0x0017d168)?;
                    c.g[RA][0] = sx(0x0017d170);
                    c.g[A0][0] = 0x1;
                    call_ptr(c, 0x003e52b0, 0x0017d170)?;
                    pc = 0x0017d170;
                    continue;
                }
                0x0017d170 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[RA][0] = sx(0x0017d17c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00255d50, 0x0017d17c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4b30));
                    if t {
                        if back_edge_unwinds(c, 0x0017d158) {
                            return Ok(());
                        }
                        pc = 0x0017d158;
                        continue;
                    }
                    pc = 0x0017d184;
                    continue;
                }
                0x0017d184 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0017d190);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    call_ptr(c, 0x00256bf8, 0x0017d190)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                    c.g[RA][0] = sx(0x0017d19c);
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    call_ptr(c, 0x00266d58, 0x0017d19c)?;
                    c.g[RA][0] = sx(0x0017d1a4);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                    call_ptr(c, 0x00266e30, 0x0017d1a4)?;
                    pc = 0x0017d1a4;
                    continue;
                }
                0x0017d1a4 => {
                    c.cycles += 5;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0017d0a8, 0x0017d1b8);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;

    fn oc817_leaf_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x817, 0);
        c.g[A0][0] = sx(fakes::ptr(&mut r));
        c.g[RA][0] = sx(0x0020_0000 + 0x10 * seed);
    }

    /// `AttachedEmitterTypeData_FreePart1`: returns at once.
    #[test]
    fn attached_emitter_type_data_free_part1_matches() {
        check(
            ee::s09::f_002f8ff8__generated,
            oc817_attached_emitter_type_data_free_part1,
            20,
            &oc817_leaf_setup,
        );
    }

    /// `MusicPlayer_EmptyPlaylistHook`: returns at once.
    #[test]
    fn music_player_empty_playlist_hook_matches() {
        check(
            ee::s08::f_002b4060__generated,
            oc817_music_player_empty_playlist_hook,
            20,
            &oc817_leaf_setup,
        );
    }

    /// `SSXAudioSystem_OnCountdownEnter`: returns at once.
    #[test]
    fn ssx_audio_system_on_countdown_enter_matches() {
        check(
            ee::s08::f_0029c418__generated,
            oc817_ssx_audio_system_on_countdown_enter,
            20,
            &oc817_leaf_setup,
        );
    }

    /// `SSXAudioSystem_SetCountdownValue(self, value)` stores the value at +0x6250.
    #[test]
    fn ssx_audio_system_set_countdown_value_matches() {
        check(
            ee::s07::f_0028c8c0__generated,
            oc817_ssx_audio_system_set_countdown_value,
            50,
            &|c, seed| {
                oc817_leaf_setup(c, seed);
                c.g[A1][0] = sx(0x1000 + seed * 0x77);
            },
        );
    }

    /// `DebugMenu_DispatchItemAction(self, a1, item)`: every item 0..8 with a1
    /// 0..3 and the jump table holding the two handlers or a foreign address
    /// (which leaves through `jump_out`).
    #[test]
    fn debug_menu_dispatch_item_action_matches() {
        check(
            ee::s02::f_0017d1b8__generated,
            oc817_debug_menu_dispatch_item_action,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x8171, 0);
                for i in 0..7u32 {
                    let v = match (seed / 9 + i) % 3 {
                        0 => 0x0017_d1ec,
                        1 => 0x0017_d1f4,
                        _ => 0x0017_d200 + 8 * i,
                    };
                    c.bus.write32(0x0045_d040 + 4 * i, v);
                }
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 9) as u64;
                c.g[RA][0] = sx(0x0020_0000 + 0x10 * seed);
                c.pc = 0x0017_d1b8;
            },
        );
    }
}
