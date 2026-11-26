//! Staging module oc790 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc790`.

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
        0x002a4d68,
        crowd_audio_init_instance,
        "CrowdAudio_InitInstance",
    ),
    (0x0039e508, ui_state_post_input, "UIState_PostInput"),
    (0x003d71d8, spch_reset_queue, "SPCH_ResetQueue"),
    (
        0x0028be60,
        snd_bank_man_load_bank_from_memory,
        "SndBankMan_LoadBankFromMemory",
    ),
    (0x002b0ce0, spch_init, "SPCH_Init"),
];

/// `SpeechInstance* SpeechInstance_ctor(SpeechInstance* self, a1, a2, a3, t0, t1, t2)`
/// (0x2afa68): stores the arguments (`a1` at `+0`, `a2` at `+4`, `t2` at `+8`), sets the
/// state word at `+0x900` to 1, clears the fields at `+0xc`, `+0xec`, `+0xf0`, `+0x8c4..+0x8d0`
/// and `+0x8f4`, zeroes the 4 double-word slots from `+0xcc`, creates the stream
/// (`StreamArray_CreateStreamDefaultParams` with the audio system at `gp+0x6c4`), loads the
/// speech inf (`SpeechInstance_LoadSpeechInf(self, a1, a3, t1)`) and returns `self`.
///
/// Not installed: its callees (`0x2aa910`, `0x2b0088`: stream and ini-parser code) have no
/// override slot, so no differential test can bound them on fixture data.
#[allow(dead_code)]
pub fn speech_instance_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[V0][0] = 0x1;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = 0x3;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[T1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S3][0] = c.g[A3][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x900), c.g[V0][0]);
        c.g[A2][0] = c.g[T0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[T2][0]);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd0));
        c.sw(c.g[S0][0] as u32, c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xec), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xf0), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8c4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8c8), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8cc), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8d0), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8f4), 0u64);
        c.sb((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        loop {
            c.cycles += 7;
            c.sw((c.g[V0][0] as u32).wrapping_sub(0x4), 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.sw(c.g[V0][0] as u32, 0u64);
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x002afad0);
                }
                continue;
            }
            break;
        }
        c.cycles += 19;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6c4));
        c.g[T0][0] = 0x20000;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
        Callee {
            addr: 0x002aa910,
            f: ee::s08::StreamArray_CreateStreamDefaultParams,
        }
        .call(c, 0x002afb04)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x002b0088,
            f: ee::s08::SpeechInstance_LoadSpeechInf,
        }
        .call(c, 0x002afb18)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    });
}

/// `CrowdAudio* CrowdAudio_InitInstance(CrowdAudio* self)` (0x2a4d68): resets the instance:
/// `+0` and `+0xd8` to -1, `+0x10..+0x1c`, `+0xd4` to 0, the 12-byte entries at `+0x20..+0xdc`
/// to 0, `+0x170..+0x18c` (with `+0x174` = -1, `+0x184` = 3), and the three 0x30-byte slots
/// (`+0xe0`: -1, `+0xe4`: -1, `+0xe8`: 0, `+0x100`: index). Returns `self`.
pub fn crowd_audio_init_instance(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0xffffffffffffffff;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0xd8), c.g[V0][0]);
    c.g[V1][0] = 0x2;
    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x18), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0xd4), 0u64);
    // Three passes that only advance the counter (the pointer in a1 is dead).
    loop {
        c.cycles += 7;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        let more = c.g[V1][0] != c.g[V0][0];
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002a4d90);
            }
            continue;
        }
        break;
    }
    c.cycles += 13;
    c.g[V1][0] = 0x3;
    c.g[V0][0] = 0xffffffffffffffff;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x184), c.g[V1][0]);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x174), c.g[V0][0]);
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x170), 0u64);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x178), 0u64);
    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0xdc));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x17c), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x180), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x188), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x18c), 0u64);
    // The 12-byte entries.
    loop {
        c.cycles += 8;
        c.sw(c.g[V1][0] as u32, 0u64);
        c.sw(c.g[A2][0] as u32, 0u64);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
        c.sw(c.g[A1][0] as u32, 0u64);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A3][0] as i64)) as u64;
        let more = c.g[V0][0] != 0u64;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002a4de0);
            }
            continue;
        }
        break;
    }
    c.cycles += 4;
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0xffffffffffffffff;
    c.g[V1][0] = c.g[A0][0];
    // The three 0x30-byte slots.
    loop {
        c.cycles += 8;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x100), c.g[A1][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0xe0), c.g[A2][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xe4), c.g[A2][0]);
        c.g[V0][0] = ((c.g[A1][0] as i64) < 3i64) as u64;
        c.sw((c.g[V1][0] as u32).wrapping_add(0xe8), 0u64);
        let more = c.g[V0][0] != 0u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002a4e10);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A0][0];
    c.pc = t;
}

/// `void UIState_PostInput(cUIState* self, cUIInput* input)` (0x39e508): empty
/// (`cUIState::onInputAlways`).
pub fn ui_state_post_input(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void SPCH_ResetQueue(void)` (0x3d71d8): resets the speech queue state at 0x44fff8: for the
/// 8 entries of three parallel word tables (`+0` cleared, `+0x20` and `+0x40` set to -1), the
/// count at `+0x60`, then 16 slots of 0x60 bytes (pointers at `+0x64`/`+0x70`/`+0x74` cleared,
/// `+0x68` half-word and `+0x6a` byte cleared, `+0x6b` = 0xff, 20 words from `+0x4c`), and
/// finally the queue head words at `gp+0x1750`/`gp+0x1754`.
pub fn spch_reset_queue(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x450000;
    c.g[A1][0] = 0xffffffffffffffff;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
    c.g[A0][0] = 0x7;
    loop {
        c.cycles += 7;
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x20), c.g[A1][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[A1][0]);
        let more = (c.g[A0][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003d71e8);
            }
            continue;
        }
        break;
    }
    c.cycles += 11;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
    c.g[A0][0] = 0u64;
    c.g[T0][0] = c.g[V0][0];
    c.sw((c.g[V0][0] as u32).wrapping_add(0x60), 0u64);
    c.g[T6][0] = sx((c.g[T0][0] as u32).wrapping_add(0x64));
    c.g[T5][0] = sx((c.g[T0][0] as u32).wrapping_add(0x70));
    c.g[T4][0] = sx((c.g[T0][0] as u32).wrapping_add(0x74));
    c.g[T3][0] = 0x60;
    c.g[T2][0] = 0xff;
    c.g[T1][0] = 0x60;
    c.mult(0, 3, 4, 9);
    loop {
        c.cycles += 14;
        c.mult(0, 2, 4, 11);
        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[A2][0] = 0x13;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T4][0] as u32));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T6][0] as u32));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4c));
        c.sb((c.g[V1][0] as u32).wrapping_add(0x6a), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
        c.sh((c.g[V1][0] as u32).wrapping_add(0x68), 0u64);
        c.sw(c.g[A0][0] as u32, 0u64);
        c.sw(c.g[V0][0] as u32, 0u64);
        c.sb((c.g[V1][0] as u32).wrapping_add(0x6b), c.g[T2][0]);
        loop {
            c.cycles += 7;
            c.sw(c.g[A1][0] as u32, 0u64);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            let more = (c.g[A2][0] as i64) >= 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003d7268);
                }
                continue;
            }
            break;
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[A3][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < 16i64) as u64;
        let more = c.g[V0][0] != 0u64;
        c.mult(0, 3, 4, 9);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003d7230);
            }
            continue;
        }
        break;
    }
    c.cycles += 3;
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1750), 0u64);
    let t = c.g[RA][0] as u32;
    c.sh((c.g[GP][0] as u32).wrapping_add(0x1754), 0u64);
    c.pc = t;
}

/// `void SndBankMan_LoadBankFromMemory(SndBankMan* self, int index, a2, a3)` (0x28be60):
/// `SndBank_LoadFromMemory(self->banks + index * 0x60, a2, a3)`, with the bank array
/// pointer at `self+0xacc`.
pub fn snd_bank_man_load_bank_from_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = 0x60;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.mult(0, 3, 5, 2);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xacc));
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Callee {
            addr: 0x0028b788,
            f: ee::s07::SndBank_LoadFromMemory,
        }
        .call(c, 0x0028be84)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    });
}

/// `SpeechSystem* SPCH_Init(SpeechSystem* self)` (0x2b0ce0): sets the speech library up:
/// `SPCH_CalcBufferSize(0x5622, 16, 1)`, a random seed from the audio system
/// (`Audio_Rand(self->sys->...+0x1d8)`) logged with `Audio_DebugPrintf`,
/// `SPCH_InitState(&table, seed, bufsize)`, allocates the queue memory (`0x252fa0` with
/// `SPCH_GetQueueBytes(500)`), stores it at `self+8`, sets the bank table
/// (`SPCH_SetBankTable(500, mem)`), and writes the callback 0x3e4e98 into word `+8` of the
/// state returned by `SPCH_GetState()`.
pub fn spch_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[A2][0] = 0x1;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 0x10;
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = 0x5622;
        call_ptr(c, 0x003d6778, 0x002b0d08)?;
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
        Callee {
            addr: 0x002adf60,
            f: ee::s08::Audio_Rand,
        }
        .call(c, 0x002b0d1c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x480000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3390));
        c.g[A1][0] = c.g[S0][0];
        Callee {
            addr: 0x002b9100,
            f: ee::s08::Audio_DebugPrintf,
        }
        .call(c, 0x002b0d30)?;
        c.g[A0][0] = 0x2b0000;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x758));
        call_ptr(c, 0x003d6840, 0x002b0d44)?;
        c.g[A0][0] = 0x1f4;
        call_ptr(c, 0x003d6c58, 0x002b0d4c)?;
        c.g[A0][0] = 0x480000;
        c.g[A2][0] = 0u64;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a8));
        call_ptr(c, 0x00252fa0, 0x002b0d60)?;
        c.g[A0][0] = 0x1f4;
        c.g[A1][0] = c.g[V0][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x003d6c60, 0x002b0d70)?;
        call_ptr(c, 0x003d6768, 0x002b0d78)?;
        c.g[V1][0] = 0x3e0000;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e98));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, check_lohi, fill, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc790's callee fakes in a machine's RAM.
    pub(crate) const OC790_CTRL: u32 = DATA + 0x32_0000;
    pub(crate) const OC790_MAGIC: u32 = 0x0790_f4ce;
    /// The object the fixtures build.
    pub(crate) const OC790_OBJ: u32 = DATA + 0x1000;

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc790_arm(c: &mut Cpu) {
        c.bus.write32(OC790_CTRL, OC790_MAGIC);
    }

    macro_rules! oc790_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC790_CTRL) != OC790_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc790_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc790_fakes!(
        (
            oc790_fk_28b788,
            OC790_PREV_28B788,
            ee::s07::OVERRIDE_0028B788,
            ee::s07::f_0028b788__generated,
            0x0028_b788
        ),
        (
            oc790_fk_2adf60,
            OC790_PREV_2ADF60,
            ee::s08::OVERRIDE_002ADF60,
            ee::s08::f_002adf60__generated,
            0x002a_df60
        ),
        (
            oc790_fk_2b9100,
            OC790_PREV_2B9100,
            ee::s08::OVERRIDE_002B9100,
            ee::s08::f_002b9100__generated,
            0x002b_9100
        ),
        (
            oc790_fk_252fa0,
            OC790_PREV_252FA0,
            ee::s06::OVERRIDE_00252FA0,
            ee::s06::f_00252fa0__generated,
            0x0025_2fa0
        ),
        (
            oc790_fk_3d9a40,
            OC790_PREV_3D9A40,
            ee::s14::OVERRIDE_003D9A40,
            ee::s14::f_003d9a40__generated,
            0x003d_9a40
        ),
    );

    /// `CrowdAudio_InitInstance`: random object contents at a random aligned address.
    #[test]
    fn crowd_audio_init_instance_matches() {
        check(
            ee::s08::f_002a4d68__generated,
            crowd_audio_init_instance,
            40,
            &|c, seed| {
                let mut r = Rng(0x790a ^ ((seed as u64) << 8));
                scramble(c, &mut r);
                c.pc = 0x002a_4d68;
                let obj = OC790_OBJ + (r.next_u32() & 0xff0);
                c.g[A0][0] = sx(obj);
                fill(c, &mut r, obj, 0x70);
            },
        );
    }

    /// `UIState_PostInput`: does nothing but return.
    #[test]
    fn ui_state_post_input_matches() {
        check(
            ee::s13::f_0039e508__generated,
            ui_state_post_input,
            8,
            &|c, seed| {
                let mut r = Rng(0x790b ^ ((seed as u64) << 8));
                scramble(c, &mut r);
                c.pc = 0x0039_e508;
            },
        );
    }

    /// `SPCH_ResetQueue`: random contents in the queue area (so the clears show) and in
    /// `a0`..`t3`; LO/HI are compared as the loops `mult` into them.
    #[test]
    fn spch_reset_queue_matches() {
        check_lohi(
            ee::s14::f_003d71d8__generated,
            spch_reset_queue,
            16,
            &|c, seed| {
                let mut r = Rng(0x790c ^ ((seed as u64) << 8));
                scramble(c, &mut r);
                c.pc = 0x003d_71d8;
                c.g[GP][0] = sx(GAME_GP);
                fill(c, &mut r, 0x0044_fff0, 0x1a0);
                fill(c, &mut r, GAME_GP + 0x1750, 2);
            },
        );
    }

    /// `SndBankMan_LoadBankFromMemory`: random bank index, array pointer and arguments; the
    /// callee `SndBank_LoadFromMemory` is a gated fake that sometimes unwinds.
    #[test]
    fn snd_bank_man_load_bank_from_memory_matches() {
        oc790_install_fakes();
        fakes::covered(
            ee::s07::f_0028be60__generated,
            snd_bank_man_load_bank_from_memory,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x790d, 1);
                oc790_arm(c);
                c.pc = 0x0028_be60;
                c.g[A0][0] = sx(OC790_OBJ);
                c.g[A1][0] = sx(r.next_u32() % 40);
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32());
                c.bus.write32(OC790_OBJ + 0xacc, r.next_u32());
            },
        );
    }

    /// `SPCH_Init`: the audio-system object at `self->0` (word `+0` holds the random-state
    /// pointer), Audio_Rand, Audio_DebugPrintf, the allocation (0x252fa0) and
    /// SPCH_ResetSentence are gated fakes that sometimes unwind; the small speech-library
    /// functions (buffer size, init state, bank table, queue bytes, get state) run for real.
    #[test]
    fn spch_init_matches() {
        oc790_install_fakes();
        fakes::covered(
            ee::s08::f_002b0ce0__generated,
            spch_init,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x790e, 4);
                oc790_arm(c);
                c.pc = 0x002b_0ce0;
                c.g[A0][0] = sx(OC790_OBJ);
                // self->sys (word 0) -> obj2, obj2's word 0 -> obj3 with a field at +0x1d8.
                c.bus.write32(OC790_OBJ, OC790_OBJ + 0x400);
                c.bus.write32(OC790_OBJ + 0x400, OC790_OBJ + 0x800);
                c.bus.write32(OC790_OBJ + 0x800 + 0x1d8, r.next_u32());
                fill(c, &mut r, 0x0044_fff0, 0x40);
                c.g[GP][0] = sx(GAME_GP);
            },
        );
    }
}
