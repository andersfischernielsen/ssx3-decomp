//! Staging module oc814 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc814`.

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
    (
        0x0019_1c48,
        fe_state_rule_select_on_activate,
        "cFEStateRuleSelect_OnActivate",
    ),
    (
        0x0028_b928,
        snd_bank_get_buffer_size,
        "SndBank_GetBufferSize",
    ),
    (
        0x0020_2768,
        event_select_handle_input,
        "EventSelect_HandleInput",
    ),
    (
        0x0028_df08,
        ssx_audio_system_reset_music_state,
        "SSXAudioSystem_ResetMusicState",
    ),
    (0x003d_a3a8, spch_get_bank_id_bytes, "SPCH_GetBankIdBytes"),
    (
        0x002b_4ae8,
        music_player_get_song_data_buffer,
        "MusicPlayer_GetSongDataBuffer",
    ),
];

/// `cFEStateRuleSelect::OnActivate` (0x191c48): forwards to `UIState_OnActivate`
/// (0x39e4c0) through the function table, returning its result.
pub fn fe_state_rule_select_on_activate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0039_e4c0), 0x0019_1c58);
}

/// `SndBank_GetBufferSize` (0x28b928): returns `bank->buffer_size` (+0x14).
pub fn snd_bank_get_buffer_size(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
    c.pc = t;
}

/// `EventSelect_HandleInput` (0x202768): stub returning 0.
pub fn event_select_handle_input(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
}

/// `SSXAudioSystem_ResetMusicState` (0x28df08): `audio->current_song (+0x629c) = -1`.
pub fn ssx_audio_system_reset_music_state(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x629c), c.g[V0][0]);
    c.pc = t;
}

/// `SPCH_GetBankIdBytes` (0x3da3a8): `*a1 = byte(a0+8)`, `*a2 = byte(a0+9)`.
pub fn spch_get_bank_id_bytes(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x8));
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x9));
    let t = c.g[RA][0] as u32;
    c.sw(c.g[A2][0] as u32, c.g[V1][0]);
    c.pc = t;
}

/// `MusicPlayer_GetSongDataBuffer` (0x2b4ae8): returns the word at `player + 0x5434`.
pub fn music_player_get_song_data_buffer(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5434));
    c.pc = t;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, ptr};
    use crate::difftest::{check, DATA};

    /// Random bytes at `a` (words), so loads see varied data.
    fn oc814_fill(c: &mut Cpu, r: &mut crate::difftest::Rng, a: u32, words: u32) {
        for i in 0..words {
            let v = r.next_u32();
            c.bus.write32(a + 4 * i, v);
        }
    }

    /// A pointer in the low half of the data area (offsets up to 0x6300 stay
    /// clear of the fakes' call log at DATA+0x100000).
    fn oc814_base(c: &mut Cpu, r: &mut crate::difftest::Rng) -> u32 {
        let a = DATA + (r.next_u32() & 0x7_fff0);
        oc814_fill(c, r, a, 8);
        a
    }

    /// The wrapper calls UIState_OnActivate through the table; a0 points at
    /// zeroed data (empty list), the callee runs for real.
    #[test]
    fn fe_state_rule_select_on_activate_matches() {
        check(
            ee::s02::f_00191c48__generated,
            fe_state_rule_select_on_activate,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x191c_48, 0);
                c.g[A0][0] = (DATA + 0x1000 + (r.next_u32() & 0xff0)) as u64;
                c.pc = 0x0019_1c48;
            },
        );
    }

    #[test]
    fn snd_bank_get_buffer_size_matches() {
        check(
            ee::s07::f_0028b928__generated,
            snd_bank_get_buffer_size,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28b9_28, 0);
                let a = oc814_base(c, &mut r);
                oc814_fill(c, &mut r, a + 0x10, 2);
                c.g[A0][0] = a as u64;
                c.pc = 0x0028_b928;
            },
        );
    }

    #[test]
    fn event_select_handle_input_matches() {
        check(
            ee::s05::f_00202768__generated,
            event_select_handle_input,
            20,
            &|c, seed| {
                let _ = machine(c, seed, 0x2027_68, 0);
                c.pc = 0x0020_2768;
            },
        );
    }

    #[test]
    fn ssx_audio_system_reset_music_state_matches() {
        check(
            ee::s07::f_0028df08__generated,
            ssx_audio_system_reset_music_state,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28df_08, 0);
                let a = oc814_base(c, &mut r);
                oc814_fill(c, &mut r, a + 0x6298, 3);
                c.g[A0][0] = a as u64;
                c.pc = 0x0028_df08;
            },
        );
    }

    #[test]
    fn spch_get_bank_id_bytes_matches() {
        check(
            ee::s14::f_003da3a8__generated,
            spch_get_bank_id_bytes,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3da3_a8, 0);
                let a = oc814_base(c, &mut r);
                c.g[A0][0] = a as u64;
                c.g[A1][0] = (DATA + 0x9_0000 + (r.next_u32() & 0xff0)) as u64;
                c.g[A2][0] = (DATA + 0xa_0000 + (r.next_u32() & 0xff0)) as u64;
                c.pc = 0x003d_a3a8;
            },
        );
    }

    #[test]
    fn music_player_get_song_data_buffer_matches() {
        check(
            ee::s08::f_002b4ae8__generated,
            music_player_get_song_data_buffer,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b4a_e8, 0);
                let a = oc814_base(c, &mut r);
                oc814_fill(c, &mut r, a + 0x5430, 2);
                c.g[A0][0] = a as u64;
                c.pc = 0x002b_4ae8;
            },
        );
    }
}
