//! Staging module oc1075 (wave wave266): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1075`.
//!
//! Ports here: `CrowdAudio_dtor`, `MIDIBanks_dtor`, `PATH_PlatStopTrackSound`,
//! `Avalanche_UpdateInstance`, `Avalanche_WriteData`,
//! `Avalanche_GetProximityInfo`, `cBoardSparkFX_dtor`, `cBoardWakeFX_dtor`.
//! (`PATH_PlatStopTrackSound`, `Avalanche_UpdateInstance` and
//! `Avalanche_WriteData` also exist as unregistered dead-code ports in
//! `game::sound::bankmonitor` / `game::visualfx::avalanche`; these copies are
//! the final ones and the dead ones can go at the reorg.)

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002a_6b50, crowd_audio_dtor, "CrowdAudio_dtor"),
    (
        0x002a_f838,
        oc1075_path_plat_stop_track_sound,
        "PATH_PlatStopTrackSound",
    ),
    (
        0x002d_7ca8,
        oc1075_avalanche_update_instance,
        "Avalanche_UpdateInstance",
    ),
    (
        0x002d_82a0,
        oc1075_avalanche_write_data,
        "Avalanche_WriteData",
    ),
    (
        0x002d_a1c0,
        oc1075_avalanche_get_proximity_info,
        "Avalanche_GetProximityInfo",
    ),
    (0x002d_a8e8, board_spark_fx_dtor, "cBoardSparkFX_dtor"),
    (0x002d_cda0, board_wake_fx_dtor, "cBoardWakeFX_dtor"),
];

// ---------------------------------------------------------------------------
// `CrowdAudio_dtor` (0x2a6b50), a.k.a. `cCrowdSys::~cCrowdSys`.

/// `void CrowdAudio_dtor(CrowdAudio* self, int flags)` (0x2a6b50): resets the
/// vtables (flags 0: also a stack copy of the sound system's and the bank
/// manager's vtables with the this-adjust of entry 1 fixed up), unloads the
/// crowd audio, clears the global crowd system (`gp+0x608`), deletes the three
/// sample lists of each of the three crowd entries (back to front), runs the
/// MIDI banks destructor, and per flag bit 1 (`& 2`) destroys the bank manager
/// and the sound manager reference, per bit 0 frees `self`.
pub fn crowd_audio_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0xc0);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0xb0);
        c.g[V0][0] = sx(0x0048_37f8);
        frame.save(c, S1, 0xa0);
        c.g[A3][0] = 0x0048_0000;
        frame.save(c, S2, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x80);
        c.g[A2][0] = sx(0x0048_3818);
        frame.save(c, S4, 0x70);
        c.g[A0][0] = 0x0048_0000;
        frame.save_ra(c, 0x60);
        c.g[A0][0] = sx(0x0048_3b48);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[T0][0] = 0x0048_0000;
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[T5][0] = sx(0x0048_3838);
        c.g[S0][0] = c.g[A1][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[A0][0]);
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        let flags_set = c.g[S0][0] != 0;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[T5][0]);
        if !flags_set {
            // The sound system's vtable (0x483818) and the bank manager's
            // (0x483838), copied to the stack with entry 1's this-adjust
            // fixed up; the objects' vtable pointers then point at the copies.
            c.cycles += 40;
            c.g[A1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x3818));
            c.g[T4][0] = sx(sp(c).wrapping_add(0x20));
            c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
            c.sd(sp(c), c.g[A1][0]);
            c.sd(sp(c).wrapping_add(0x18), c.g[A0][0]);
            c.sd(sp(c).wrapping_add(0x10), c.g[V1][0]);
            c.sd(sp(c).wrapping_add(0x8), c.g[V0][0]);
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
            c.g[T3][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x3838));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
            c.g[V1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x28));
            c.g[T1][0] = c.lw(c.g[S1][0] as u32);
            c.g[A3][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x1d8));
            c.g[T2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x118));
            c.g[T0][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[A2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sd(sp(c).wrapping_add(0x28), c.g[A3][0]);
            c.sd(sp(c).wrapping_add(0x48), c.g[V1][0]);
            c.sh(sp(c).wrapping_add(0x8), c.g[A0][0]);
            c.sd(sp(c).wrapping_add(0x20), c.g[T3][0]);
            c.sd(sp(c).wrapping_add(0x30), c.g[T2][0]);
            c.sd(sp(c).wrapping_add(0x38), c.g[T0][0]);
            c.sd(sp(c).wrapping_add(0x40), c.g[A2][0]);
            c.sd(sp(c).wrapping_add(0x50), c.g[A1][0]);
            c.sw((c.g[T1][0] as u32).wrapping_add(0x1d4), c.g[T4][0]);
            c.g[V1][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x120));
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sh(sp(c).wrapping_add(0x28), c.g[V1][0]);
        }
        // Unload the crowd audio; clear the global crowd system.
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = c.g[S0][0] & 0x1;
        Callee {
            addr: 0x002a_6f38,
            f: ee::s08::CrowdAudio_Unload,
        }
        .call(c, 0x002a_6c64)?;
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.sw((c.g[GP][0] as u32).wrapping_add(0x608), 0);
        let no_entries = c.g[S2][0] == 0;
        c.g[S3][0] = c.g[S0][0] & 0x2;
        if no_entries {
            // 'b_2a6cb4: skip the entries.
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
        } else {
            c.cycles += 3;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x118));
            let none = c.g[S2][0] == c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            if !none {
                // The three crowd entries (0x54 bytes each), back to front:
                // delete their anticipate, fall and trick sample lists.
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x54));
                loop {
                    c.cycles += 11;
                    c.g[A1][0] = 0x2;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                    Callee {
                        addr: 0x002a_4be0,
                        f: ee::s08::CrowdAudio_DeleteSampleList,
                    }
                    .call(c, 0x002a_6c94)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = 0x2;
                    Callee {
                        addr: 0x002a_4be0,
                        f: ee::s08::CrowdAudio_DeleteSampleList,
                    }
                    .call(c, 0x002a_6ca0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x2;
                    Callee {
                        addr: 0x002a_4be0,
                        f: ee::s08::CrowdAudio_DeleteSampleList,
                    }
                    .call(c, 0x002a_6cac)?;
                    if c.g[S2][0] != c.g[S0][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x54));
                        if event_due(c) {
                            deliver_event_here(c, 0x002a_6c88);
                        }
                        continue;
                    }
                    break;
                }
                // The loop exit joins 'b_2a6cb4 (one more block cycle).
                c.cycles += 1;
                c.g[A0][0] = c.g[S1][0];
            }
            // `none` (self+0x1c == self+0x118: impossible for a real object)
            // jumps straight to the MIDI banks destructor, skipping that cycle.
        }
        // The MIDI banks destructor, then the optional owners.
        c.cycles += 4;
        c.g[A1][0] = 0;
        Callee {
            addr: 0x002a_82d8,
            f: ee::s08::MIDIBanks_dtor,
        }
        .call(c, 0x002a_6cc0)?;
        if c.g[S3][0] != 0 {
            c.cycles += 7;
            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0028_bb10, 0x002a_6cd4)?;
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            c.g[A1][0] = 0;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
            Callee {
                addr: 0x002a_dee8,
                f: ee::s08::SoundManagerRef_dtor,
            }
            .call(c, 0x002a_6ce4)?;
        }
        c.cycles += 2;
        let no_free = c.g[S4][0] == 0;
        frame.restore(c, S0, 0xb0);
        if !no_free {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x002a_6cf4)?;
            frame.restore(c, S0, 0xb0);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MIDIBanks_dtor` (0x2a82d8), a.k.a. `cMidiSys::~cMidiSys`.

/// `void MIDIBanks_dtor(MIDIBanks* self, int flags)` (0x2a82d8): resets the
/// vtables (flags 0: also the stack copies with the this-adjust fixed up),
/// frees the MIDI banks block at `+0x10`, clears the global (`gp+0x654`), shuts
/// the MIDI player down, runs the base destructor and, per flag bit 1
/// (`& 2`), destroys the bank manager and the sound manager reference, per
/// bit 0 frees `self`.
#[allow(dead_code)]
pub fn midi_banks_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0x80);
        c.g[V0][0] = sx(0x0048_3a48);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x60);
        c.g[A3][0] = 0x0048_0000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[A2][0] = sx(0x0048_39f0);
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A0][0] = 0x0048_0000;
        c.g[A0][0] = sx(0x0048_3b48);
        c.g[T0][0] = 0x0048_0000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
        c.g[T5][0] = sx(0x0048_3a10);
        c.g[S1][0] = c.g[A1][0];
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[A0][0]);
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        let flags_set = c.g[S1][0] != 0;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[T5][0]);
        if !flags_set {
            // Stack copies of the vtables at 0x4839f0 and 0x483a10 with
            // entry 1's this-adjust fixed up (the sound system sits 0x14
            // below the MIDI system, the bank manager 0x1c below).
            c.cycles += 40;
            c.g[A1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x39f0));
            c.g[T4][0] = sx(sp(c).wrapping_add(0x20));
            c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
            c.sd(sp(c), c.g[A1][0]);
            c.sd(sp(c).wrapping_add(0x18), c.g[A0][0]);
            c.sd(sp(c).wrapping_add(0x10), c.g[V1][0]);
            c.sd(sp(c).wrapping_add(0x8), c.g[V0][0]);
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
            c.g[T3][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x3a10));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
            c.g[V1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x28));
            c.g[T1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A3][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x1d8));
            c.g[T2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
            c.g[T0][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[A2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sd(sp(c).wrapping_add(0x28), c.g[A3][0]);
            c.sd(sp(c).wrapping_add(0x48), c.g[V1][0]);
            c.sh(sp(c).wrapping_add(0x8), c.g[A0][0]);
            c.sd(sp(c).wrapping_add(0x20), c.g[T3][0]);
            c.sd(sp(c).wrapping_add(0x30), c.g[T2][0]);
            c.sd(sp(c).wrapping_add(0x38), c.g[T0][0]);
            c.sd(sp(c).wrapping_add(0x40), c.g[A2][0]);
            c.sd(sp(c).wrapping_add(0x50), c.g[A1][0]);
            c.sw((c.g[T1][0] as u32).wrapping_add(0x1d4), c.g[T4][0]);
            c.g[V1][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1c));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sh(sp(c).wrapping_add(0x28), c.g[V1][0]);
        }
        // Free the MIDI banks block, if any.
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x002a_83e8)?;
        }
        c.cycles += 8;
        c.sw((c.g[GP][0] as u32).wrapping_add(0x654), 0);
        call_ptr(c, 0x003b_9880, 0x002a_83f0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_fea0, 0x002a_83fc)?;
        c.g[V0][0] = c.g[S1][0] & 0x2;
        let no_owners = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if !no_owners {
            c.cycles += 8;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0028_bb10, 0x002a_8414)?;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            c.g[A1][0] = 0;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
            Callee {
                addr: 0x002a_dee8,
                f: ee::s08::SoundManagerRef_dtor,
            }
            .call(c, 0x002a_8424)?;
            c.g[V0][0] = c.g[S1][0] & 0x1;
        }
        c.cycles += 2;
        let no_free = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x80);
        if !no_free {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x002a_8438)?;
            frame.restore(c, S0, 0x80);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `PATH_PlatStopTrackSound` (0x2af838).

/// `int PATH_PlatStopTrackSound(int idx)` (0x2af838): stop the platform
/// track's sound. Slot `idx` of the track table at 0x4d3e98 gives the track;
/// with the sound lock held, a track with the "playing" flag (`+0x64`) stops
/// its sound handle (`track+0x6c -> +4`), otherwise its stream (`*track`);
/// then the track is marked stopped (`+0x65 = 1`). Returns 0.
pub fn oc1075_path_plat_stop_track_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x004d_0000;
        c.g[V0][0] = sx(0x004d_3e98);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        frame.save(c, S0, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = Addr(r32(c, A0)).lw(c);
        call_ptr(c, 0x003b_58a0, 0x002a_f85c)?; // SND_Lock
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x64)).lb(c);
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x6c)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x4)).lw(c);
            call_ptr(c, 0x003b_89e0, 0x002a_f874)?; // SND_Stop
        } else {
            c.cycles += 2;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            call_ptr(c, 0x003b_78f8, 0x002a_f884)?; // SNDSTRM_Stop
        }
        c.cycles += 9;
        call_ptr(c, 0x003b_58d8, 0x002a_f88c)?; // SND_Unlock
        c.g[V1][0] = 0x1;
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let flag = c.g[V1][0];
        Addr(r32(c, S0).wrapping_add(0x65)).sb(c, flag);
        c.g[V0][0] = 0;
        frame.restore(c, S0, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// `Avalanche_UpdateInstance` (0x2d7ca8): advance one active avalanche node by
// `t` (scaled by the data block's rate), sample its history, and either copy
// the frame into the node and emit particles, or end the node. Returns in V0
// whether the node is still alive.

/// `tActiveAvalancheNode` fields.
pub(crate) const AU_DATA: u32 = 0x2e0;
pub(crate) const AU_FB4: u32 = 0xb4;
pub(crate) const AU_CUR_POS: u32 = 0x60;
pub(crate) const AU_CUR_MAT: u32 = 0x70;
pub(crate) const AU_CUR_COLOUR: u32 = 0xc0;
pub(crate) const AU_EMITTER: u32 = 0xd0;
pub(crate) const AU_F1FC: u32 = 0x1fc;
/// `sAvData` fields: length, rate, type (u16), emits (u8).
pub(crate) const AU_D_LENGTH: u32 = 0x100;
pub(crate) const AU_D_RATE: u32 = 0x10c;
pub(crate) const AU_D_TYPE: u32 = 0xf0;
pub(crate) const AU_D_EMITS: u32 = 0xf2;
/// The sampled frame in this function's stack frame: position at +0, colour
/// at +0x10, matrix at +0x20.
pub(crate) const AU_OUT_COLOUR: u32 = 0x10;
pub(crate) const AU_OUT_MAT: u32 = 0x20;

/// `bool Avalanche_UpdateInstance(self, float t)` (0x2d7ca8). A node without
/// a data block is dead (V0 = 0). Otherwise the history is sampled at
/// `t * rate`; at 30 samples per second a position past the data's length
/// ends the node (released, V0 = 0) unless the data is of type 2 (which stays
/// alive); inside, the sampled position, matrix and colour become the node's
/// current ones and a type-other-than-2 emitting node spawns particles at it
/// through `DynamicEmitter_SpawnAt`. V0 = 1 for a node that lives on.
pub fn oc1075_avalanche_update_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x70);
        frame.save_fpr(c, 20, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x60);
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(AU_DATA)).lw(c);
        let has_data = c.g[V0][0] != 0;
        c.mov_s(20, 12);
        if has_data {
            au_sample_and_emit(c)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// The body for a node with data (V0 = its data block on entry): sample the
/// history at `t * rate`; past the length, end the node; otherwise copy the
/// sampled frame into the node and emit (or just stay alive when it emits
/// nothing).
pub(crate) fn au_sample_and_emit(c: &mut Cpu) -> Flow {
    c.cycles += 15;
    let data = r32(c, V0);
    c.lwc1(0, data.wrapping_add(AU_D_RATE));
    c.g[A0][0] = sx(sp(c));
    c.g[A1][0] = sx(r32(c, S0));
    c.mul_s(20, 20, 0);
    c.mov_s(12, 20);
    Callee {
        addr: 0x002d_5778,
        f: ee::s09::Avalanche_SampleHistory,
    }
    .call(c, 0x002d_7ce8)?;
    c.g[AT][0] = 0x41f0_0000; // 30.0
    c.mtc1(AT, 0);
    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(AU_DATA)).lw(c);
    c.mul_s(0, 20, 0);
    let length = r32(c, V0).wrapping_add(AU_D_LENGTH);
    c.lwc1(1, length);
    c.c_lt_s(0, 1);
    let inside = c.fcc();
    c.g[A1][0] = sx(r32(c, S0).wrapping_add(AU_CUR_POS));
    if !inside {
        return au_past_end(c);
    }
    // Copy the sampled frame into the node: position (v0), matrix, colour.
    c.cycles += 19;
    c.lq(2, sp(c));
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(AU_CUR_MAT));
    let out_mat = sp(c).wrapping_add(AU_OUT_MAT);
    c.g[V1][0] = sx(out_mat);
    c.sq(2, r32(c, S0).wrapping_add(AU_CUR_POS));
    c.lqc2(1, out_mat);
    c.lqc2(2, out_mat.wrapping_add(0x10));
    c.lqc2(3, out_mat.wrapping_add(0x20));
    c.lqc2(4, out_mat.wrapping_add(0x30));
    let cur_mat = r32(c, A0);
    c.sqc2(1, cur_mat);
    c.sqc2(2, cur_mat.wrapping_add(0x10));
    c.sqc2(3, cur_mat.wrapping_add(0x20));
    c.sqc2(4, cur_mat.wrapping_add(0x30));
    c.lq(2, sp(c).wrapping_add(AU_OUT_COLOUR));
    c.g[T1][0] = Addr(r32(c, S0).wrapping_add(AU_DATA)).lw(c);
    c.g[A0][0] = 2;
    c.sq(2, r32(c, S0).wrapping_add(AU_CUR_COLOUR));
    c.g[V1][0] = Addr(r32(c, T1).wrapping_add(AU_D_TYPE)).lhu(c);
    let typed = c.g[V1][0] == c.g[A0][0];
    c.g[A3][0] = sx(r32(c, S0).wrapping_add(AU_CUR_COLOUR));
    if typed {
        return au_alive(c);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, T1).wrapping_add(AU_D_EMITS)).lbu(c);
    let silent = c.g[V0][0] == 0;
    c.g[V0][0] = 0x0050_0000;
    if silent {
        return au_alive(c);
    }
    au_emit(c)
}

/// The sample is past the node's length: a type-2 node stays alive, any other
/// one is released and ends (V0 = 0).
pub(crate) fn au_past_end(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(AU_D_TYPE)).lhu(c);
    c.g[V0][0] = 2;
    let typed = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = sx(r32(c, S0));
    if typed {
        return au_alive(c);
    }
    c.cycles += 4;
    c.g[A1][0] = 0;
    Callee {
        addr: 0x002d_7dd8,
        f: ee::s09::Avalanche_ReleaseInstance,
    }
    .call(c, 0x002d_7db8)?;
    c.g[V0][0] = 0;
    Ok(())
}

/// The node stays alive (V0 = 1) after its frame was copied.
pub(crate) fn au_alive(c: &mut Cpu) -> Flow {
    c.cycles += 1;
    c.g[V0][0] = 1;
    Ok(())
}

/// Emit particles from the node's emitter at the sampled position
/// (`DynamicEmitter_SpawnAt(dt, emitter, pos, &zero, colour, before_end)`);
/// the node stays alive.
pub(crate) fn au_emit(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.lwc1(0, r32(c, S0).wrapping_add(AU_FB4));
    c.g[A2][0] = sx(r32(c, V0).wrapping_sub(0xee0));
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(AU_EMITTER));
    c.swc1(0, r32(c, S0).wrapping_add(AU_F1FC));
    c.lwc1(0, r32(c, T1).wrapping_add(AU_D_LENGTH));
    c.c_lt_s(20, 0);
    let before_end = c.fcc();
    c.g[T0][0] = 1;
    if !before_end {
        c.cycles += 1;
        c.g[T0][0] = 0;
    }
    c.cycles += 5;
    c.lwc1(12, r32(c, GP).wrapping_sub(0x3cd4));
    call_ptr(c, 0x0037_17c0, 0x002d_7d98)?;
    c.g[V0][0] = 1;
    Ok(())
}

// ---------------------------------------------------------------------------
// `Avalanche_WriteData` (0x2d82a0).

/// `int Avalanche_WriteData(direct, handle, offset, src, stride, rows)`
/// (0x2d82a0): write `rows` rows of `stride` bytes at `src` to a file through
/// `EAFile_Write`. A direct write goes out as it is; otherwise each row is
/// byte-reversed into a temporary buffer first (the history file is
/// big-endian), which is written and freed. Returns `stride * rows`.
pub fn oc1075_avalanche_write_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[T1][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[T0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        c.g[S5][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 9;
            c.mult(0, 0, S3, S1);
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.lo[0];
            c.g[T0][0] = 0x64;
            call_ptr(c, 0x003d_ec80, 0x002d_82f4)?;
            c.mult(0, 2, S3, S1);
        } else {
            aw_buffered(c)?;
        }
        // The epilogue (`'b_2d8394`).
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

/// The buffered branch of `Avalanche_WriteData`: copy every row reversed into
/// a buffer, write the buffer, and free it. Leaves V0 = stride * rows.
pub(crate) fn aw_buffered(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    c.mult(0, 0, S3, S1);
    c.g[A1][0] = sx(0x004a_3ae8);
    c.g[A2][0] = 0;
    c.g[A0][0] = c.lo[0];
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, 0x002d_8318)?; // operator_new_named
    c.g[S2][0] = c.g[V0][0];
    c.g[V1][0] = 0;
    let no_rows = (c.g[S1][0] as i64) <= 0;
    c.g[A1][0] = c.g[S2][0];
    if !no_rows {
        loop {
            // One row: the bytes from `end` back to `src`, into the buffer.
            c.cycles += 4;
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, S3)));
            c.g[A2][0] = sx(r32(c, V1).wrapping_add(1));
            let empty_row = c.g[S0][0] == c.g[A0][0];
            c.g[V1][0] = c.g[A0][0];
            if !empty_row {
                loop {
                    c.cycles += 7;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.g[V0][0] = Addr(r32(c, V1)).lbu(c);
                    let byte = c.g[V0][0];
                    Addr(r32(c, A1)).sb(c, byte);
                    let more = c.g[S0][0] != c.g[V1][0];
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_8338);
                    }
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[A2][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more_rows = c.g[V0][0] != 0;
            c.g[S0][0] = c.g[A0][0];
            if !more_rows {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_8328);
            }
        }
    }
    c.cycles += 9;
    c.mult(0, 0, S3, S1);
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.lo[0];
    c.g[T0][0] = 0x64;
    call_ptr(c, 0x003d_ec80, 0x002d_8380)?; // EAFile_Write
    let no_buf = c.g[S2][0] == 0;
    c.mult(0, 2, S3, S1);
    if !no_buf {
        c.cycles += 3;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_7e98, 0x002d_8390)?; // EA_MemFree
        c.mult(0, 2, S3, S1);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// `Avalanche_GetProximityInfo` (0x2da1c0).

/// `void Avalanche_GetProximityInfo(out, pos)` (0x2da1c0), a.k.a.
/// `cAvalanche::getAvalancheDataByPosition`: scans the 16 active avalanche
/// slots (0x1c bytes each at 0x538938: avalanche ptr, node list head) and
/// every live node (`+0x2e0` data, `+0x2e4` next) whose position (`+0x60`) is
/// closer than 15000 units (squared distance below 225e6) to `pos`: sums the
/// positions into `out` (a quad, w = 1 each), sums the node scales (`+0xb0`),
/// takes the smallest squared distance, counts the nodes and sets the "any"
/// flag unless a counted node is of type 2. Finally divides the centre and
/// the scale sum by the count. `out` = { centre.xyzw, minDist, count, total...}.
pub fn oc1075_avalanche_get_proximity_info(c: &mut Cpu) {
    c.cycles += 24;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x60));
    c.g[V0][0] = 0x0050_0000;
    c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
    c.ldr(3, c.g[A1][0] as u32);
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    c.sdl(3, sp(c).wrapping_add(0x7));
    c.sdr(3, sp(c));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
    c.sw(sp(c).wrapping_add(0x8), c.g[A2][0]);
    c.g[T2][0] = c.g[A0][0];
    c.lq(3, c.g[V0][0] as u32);
    c.g[A1][0] = 0x0054_0000;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3cbc));
    c.g[T4][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x76c8));
    c.sq(3, sp(c).wrapping_add(0x10));
    c.g[T0][0] = c.g[SP][0];
    c.swc1(0, sp(c).wrapping_add(0x24));
    c.g[A0][0] = 0;
    c.sw(sp(c).wrapping_add(0x28), 0);
    c.g[T3][0] = 0x1c;
    c.sw(sp(c).wrapping_add(0x20), 0);
    c.sw(sp(c).wrapping_add(0x2c), 0);
    c.mult(0, 2, 4, 11);
    'l_2da220: loop {
        'b_2da32c: {
            'b_2da328: {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                let empty_slot = c.g[V0][0] == 0;
                c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                if empty_slot {
                    break 'b_2da328;
                }
                c.cycles += 3;
                c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                if c.g[A3][0] == 0 {
                    c.g[A0][0] = c.g[T1][0];
                    break 'b_2da32c;
                }
                c.cycles += 5;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 5);
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3cb8));
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2e0));
                loop {
                    // One node: skipped without data, or when too far away.
                    'b_2da320: {
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            c.g[A3][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2e4));
                            break 'b_2da320;
                        }
                        c.cycles += 27;
                        c.lwc1(0, c.g[T0][0] as u32);
                        c.swc1(0, sp(c).wrapping_add(0x40));
                        c.lwc1(1, (c.g[T0][0] as u32).wrapping_add(0x4));
                        c.swc1(1, sp(c).wrapping_add(0x44));
                        c.lwc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                        c.swc1(5, sp(c).wrapping_add(0x4c));
                        c.swc1(0, sp(c).wrapping_add(0x48));
                        c.lqc2(3, sp(c).wrapping_add(0x40));
                        c.lqc2(4, (c.g[A3][0] as u32).wrapping_add(0x60));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, sp(c).wrapping_add(0x50));
                        c.lq(2, sp(c).wrapping_add(0x50));
                        c.sq(2, sp(c).wrapping_add(0x30));
                        c.lqc2(3, sp(c).wrapping_add(0x30));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.vu0().sqrt(4, 0);
                        c.vu0().waitq();
                        c.cfc2(3, 22);
                        c.mtc1(3, 3);
                        c.c_lt_s(3, 4);
                        if !c.fcc() {
                            c.g[A3][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2e4));
                            break 'b_2da320;
                        }
                        // Close enough: accumulate centre, scale, min, count, any.
                        c.cycles += 23;
                        c.lqc2(3, sp(c).wrapping_add(0x10));
                        c.lqc2(4, (c.g[A3][0] as u32).wrapping_add(0x60));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, sp(c).wrapping_add(0x10));
                        c.lwc1(2, (c.g[A3][0] as u32).wrapping_add(0xb0));
                        c.lwc1(1, sp(c).wrapping_add(0x20));
                        c.lwc1(0, sp(c).wrapping_add(0x24));
                        c.add_s(1, 1, 2);
                        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x2c));
                        c.min_s(0, 0, 3);
                        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x28));
                        c.swc1(0, sp(c).wrapping_add(0x24));
                        c.g[A0][0] = c.g[A2][0] | 0x1;
                        c.swc1(1, sp(c).wrapping_add(0x20));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2e0));
                        c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0xf0));
                        c.sw(sp(c).wrapping_add(0x28), c.g[A1][0]);
                        c.g[V1][0] ^= 0x2;
                        if c.g[V1][0] != 0 {
                            c.g[A0][0] = c.g[A2][0];
                        }
                        c.g[A0][0] = (0u64 < c.g[A0][0]) as u64;
                        c.sw(sp(c).wrapping_add(0x2c), c.g[A0][0]);
                        c.g[A3][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2e4));
                    }
                    c.cycles += 2;
                    if c.g[A3][0] != 0 {
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2e0));
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_a250);
                        }
                        continue;
                    }
                    break 'b_2da328;
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[T1][0];
            break 'b_2da32c;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A0][0] as i64) < 16i64) as u64;
        let more_slots = c.g[V0][0] != 0;
        c.mult(0, 2, 4, 11);
        if more_slots {
            if event_due(c) {
                deliver_event_here(c, 0x002d_a220);
            }
            continue 'l_2da220;
        }
        // Divide the centre and the scale sum by the count.
        c.cycles += 21;
        c.lwc1(0, sp(c).wrapping_add(0x28));
        c.cvt_s_w(0, 0);
        c.g[V0][0] = c.g[T2][0];
        c.mfc1(3, 0);
        c.qmtc2(3, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, sp(c).wrapping_add(0x10));
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, sp(c).wrapping_add(0x10));
        c.lwc1(1, sp(c).wrapping_add(0x28));
        c.cvt_s_w(1, 1);
        c.lwc1(0, sp(c).wrapping_add(0x20));
        c.lq(3, sp(c).wrapping_add(0x10));
        c.div_s(0, 0, 1);
        c.sq(3, c.g[T2][0] as u32);
        c.swc1(0, sp(c).wrapping_add(0x20));
        c.lq(3, sp(c).wrapping_add(0x20));
        c.sq(3, (c.g[T2][0] as u32).wrapping_add(0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x60));
        c.pc = ra;
        return;
    }
}

// ---------------------------------------------------------------------------
// `cBoardSparkFX_dtor` (0x2da8e8) and `cBoardWakeFX_dtor` (0x2dcda0).

/// The common head of the two board effect destructors: points the rider's
/// vtable-ish fields (`+0x6e8`, `+0x6d0`, `+0x6c0` of the object at `*self`)
/// at the base vtables (`vt_a` / `vt_b` are the two game-image vtables) and,
/// for flags 0, copies both to the stack (0x48 and 0xa8 bytes) with entry 1's
/// this-adjust (`adj` = `self - (*self - delta)`) fixed up, pointing the rider
/// at the copies. Returns whether `flags != 0` (the copies were skipped).
/// Register results match the generated head, including the stale `T2`/`A3`.
fn board_fx_dtor_head(c: &mut Cpu, vt_a: u32, vt_b: u32, delta: u32) {
    c.cycles += 21;
    let frame = Frame { size: 0x130 };
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x130));
    c.g[V0][0] = 0x0049_0000;
    frame.save(c, S0, 0x120);
    c.g[A2][0] = sx(vt_a);
    frame.save(c, S1, 0x110);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0x100);
    c.g[S1][0] = c.g[A1][0];
    c.g[A0][0] = 0x0046_0000;
    c.g[A3][0] = c.g[V0][0];
    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
    c.g[A0][0] = sx(0x0045_9b90);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
    c.g[V1][0] = 0x0049_0000;
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    c.g[T2][0] = c.g[V1][0];
    c.g[V1][0] = sx(vt_b);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x6d0), c.g[A0][0]);
    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
    let flags_set = c.g[S1][0] != 0;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
    if flags_set {
        return;
    }
    // First vtable: 0x40 bytes in 0x20-byte steps, then one more doubleword.
    c.cycles += 4;
    c.g[A1][0] = c.g[A2][0];
    c.g[V1][0] = c.g[SP][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
    c.g[T0][0] = sx(sp(c).wrapping_add(0x50));
    loop {
        c.cycles += 11;
        c.g[A0][0] = c.ld(c.g[A1][0] as u32);
        c.g[A2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
        c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
        c.g[T3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
        c.sd(c.g[V1][0] as u32, c.g[A0][0]);
        c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
        c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
        c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
        let more = c.g[A1][0] != c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, board_head_pc(vt_a, 0));
            }
            continue;
        }
        break;
    }
    c.cycles += 14;
    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
    c.g[A0][0] = sx(vt_a);
    c.g[A2][0] = c.ld(c.g[A1][0] as u32);
    c.sd(c.g[V1][0] as u32, c.g[A2][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
    c.g[A2][0] = sx(vt_b);
    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
    c.g[A1][0] = c.g[T0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(delta));
    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.sh(sp(c).wrapping_add(0x8), c.g[V1][0]);
    // Second vtable: 0xa0 bytes in 0x20-byte steps, then 16 more bytes.
    loop {
        c.cycles += 11;
        c.g[V0][0] = c.ld(c.g[A2][0] as u32);
        c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
        c.sd(c.g[A1][0] as u32, c.g[V0][0]);
        c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
        c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
        let more = c.g[A2][0] != c.g[T1][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, board_head_pc(vt_a, 1));
            }
            continue;
        }
        break;
    }
    c.cycles += 10;
    c.g[V0][0] = c.ld(c.g[A2][0] as u32);
    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
    c.sd(c.g[A1][0] as u32, c.g[V0][0]);
    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[V0][0] = sx(vt_b);
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T0][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.sh(sp(c).wrapping_add(0x58), c.g[V1][0]);
}

/// The loop-head pcs of the two copy loops of the destructor heads (the
/// event pc), selected by which vtable base the head was given.
fn board_head_pc(vt_a: u32, second: u32) -> u32 {
    match (vt_a == 0x0048_8cb0, second) {
        (true, 0) => 0x002d_a94c,
        (true, _) => 0x002d_a9b0,
        (false, 0) => 0x002d_ce04,
        (false, _) => 0x002d_ce68,
    }
}

/// `void cBoardSparkFX_dtor(cBoardSparkFX* self, int flags)` (0x2da8e8): the
/// vtable head, then (`self+0x20` effect, if any) its destroy method through
/// its vtable (`+0x1f8` -> `+0x20` delta, `+0x24` method) with arg 3, deletes
/// the buffers at `+0x24` and `+0x28`, per flag bit 1 destroys the rider
/// (`Rider_dtor(*self, 0)`), per bit 0 frees `self`.
pub fn board_spark_fx_dtor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x130 };
        board_fx_dtor_head(c, 0x0048_8cb0, 0x0048_8cf8, 0xb0);
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
        let no_effect = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !no_effect {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1f8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002d_aa24)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        call_ptr(c, 0x0031_7e50, 0x002d_aa2c)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x0031_7e50, 0x002d_aa34)?;
        c.g[V0][0] = c.g[S1][0] & 0x2;
        let no_rider = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if !no_rider {
            c.cycles += 4;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0011_b978, 0x002d_aa4c)?;
            c.g[V0][0] = c.g[S1][0] & 0x1;
        }
        board_fx_dtor_tail(c, frame, 0x002d_aa60)
    })
}

/// The shared tail: `v0` holds `flags & 1`; free `self` when set, then the
/// epilogue.
fn board_fx_dtor_tail(c: &mut Cpu, frame: Frame, free_ret: u32) -> Flow {
    c.cycles += 2;
    let no_free = c.g[V0][0] == 0;
    if no_free {
        frame.restore(c, S0, 0x120);
    } else {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7e50, free_ret)?;
        frame.restore(c, S0, 0x120);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x110);
    frame.restore_ra(c, 0x100);
    frame.close(c);
    Ok(())
}

/// `void cBoardWakeFX_dtor(cBoardWakeFX* self, int flags)` (0x2dcda0): the
/// vtable head (wake vtables), frees the buffers at `+0xc` and `+0x10`
/// (`EA_MemFree`, when set), per flag bit 1 destroys the rider, per bit 0
/// frees `self`.
pub fn board_wake_fx_dtor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x130 };
        board_fx_dtor_head(c, 0x0048_8da8, 0x0048_8df0, 0xc0);
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        if c.g[A0][0] == 0 {
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x002d_ced0)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        }
        c.cycles += 2;
        let no_second = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 0x2;
        if !no_second {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x002d_cee4)?;
            c.g[V0][0] = c.g[S1][0] & 0x2;
        }
        c.cycles += 2;
        let no_rider = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if !no_rider {
            c.cycles += 4;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0011_b978, 0x002d_cefc)?;
            c.g[V0][0] = c.g[S1][0] & 0x1;
        }
        board_fx_dtor_tail(c, frame, 0x002d_cf10)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, check_reaching, machine, machine_rt, FAKE};
    use crate::difftest::{check, elf_image, fill, put_words, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Fixture objects live in DATA+0x1000.. (clear of the fake log at
    // DATA+0x100000 and of the allocator regions); this module's fakes are
    // gated by one word in the machine's RAM.

    /// Gate word of this module's fakes, and its magic value.
    pub(crate) const OC1075_CTRL: u32 = DATA + 0x2f_0000;
    pub(crate) const OC1075_MAGIC: u32 = 0x4f43_1075;
    /// Where the `EAFile_Write` fake copies the buffer it was given.
    pub(crate) const OC1075_CAPTURE: u32 = DATA + 0x2f_2000;

    /// The gated fake callee: logs the call like `fakes::fake_call` and, for
    /// `EAFile_Write`, captures the written bytes; for
    /// `Avalanche_SampleHistory`, fills the sampled frame (0x60 bytes at `a0`).
    pub(crate) fn oc1075_fake_call(c: &mut Cpu) {
        let target = c.pc;
        let ret = c.g[RA][0] as u32;
        let (a0, a2, a3) = (c.g[A0][0] as u32, c.g[A2][0] as u32, c.g[A3][0] as u32);
        fakes::fake(c, target, ret);
        if c.pc != ret {
            return;
        }
        match target {
            0x003d_ec80 => {
                let n = a3.min(0x100);
                for i in 0..n {
                    let b = c.bus.read8(a2 + i);
                    c.bus.write8(OC1075_CAPTURE + i, b);
                }
                c.bus.write32(OC1075_CAPTURE + 0x100, n);
            }
            0x002d_5778 => {
                for i in 0..0x18u32 {
                    let w = fakes::mix(0x5778 ^ (i as u64) << 20 ^ c.cycles) as u32;
                    c.bus.write32(a0 + 4 * i, w);
                }
            }
            _ => {}
        }
    }

    macro_rules! oc1075_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1075_CTRL) != OC1075_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1075_fake_call(c);
                }
            )*
            pub(crate) fn oc1075_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1075_fakes!(
        (
            oc1075_f_3b58a0,
            OC1075_P_3B58A0,
            ee::s13::OVERRIDE_003B58A0,
            ee::s13::f_003b58a0__generated
        ),
        (
            oc1075_f_3b89e0,
            OC1075_P_3B89E0,
            ee::s13::OVERRIDE_003B89E0,
            ee::s13::f_003b89e0__generated
        ),
        (
            oc1075_f_3b78f8,
            OC1075_P_3B78F8,
            ee::s13::OVERRIDE_003B78F8,
            ee::s13::f_003b78f8__generated
        ),
        (
            oc1075_f_3b58d8,
            OC1075_P_3B58D8,
            ee::s13::OVERRIDE_003B58D8,
            ee::s13::f_003b58d8__generated
        ),
        (
            oc1075_f_3b9880,
            OC1075_P_3B9880,
            ee::s13::OVERRIDE_003B9880,
            ee::s13::f_003b9880__generated
        ),
        (
            oc1075_f_3dec80,
            OC1075_P_3DEC80,
            ee::s14::OVERRIDE_003DEC80,
            ee::s14::f_003dec80__generated
        ),
        (
            oc1075_f_2d5778,
            OC1075_P_2D5778,
            ee::s09::OVERRIDE_002D5778,
            ee::s09::f_002d5778__generated
        ),
        (
            oc1075_f_2d7dd8,
            OC1075_P_2D7DD8,
            ee::s09::OVERRIDE_002D7DD8,
            ee::s09::f_002d7dd8__generated
        ),
        (
            oc1075_f_3717c0,
            OC1075_P_3717C0,
            ee::s12::OVERRIDE_003717C0,
            ee::s12::f_003717c0__generated
        ),
        (
            oc1075_f_2a6f38,
            OC1075_P_2A6F38,
            ee::s08::OVERRIDE_002A6F38,
            ee::s08::f_002a6f38__generated
        ),
        (
            oc1075_f_2a4be0,
            OC1075_P_2A4BE0,
            ee::s08::OVERRIDE_002A4BE0,
            ee::s08::f_002a4be0__generated
        ),
        (
            oc1075_f_2a82d8,
            OC1075_P_2A82D8,
            ee::s08::OVERRIDE_002A82D8,
            ee::s08::f_002a82d8__generated
        ),
        (
            oc1075_f_2adee8,
            OC1075_P_2ADEE8,
            ee::s08::OVERRIDE_002ADEE8,
            ee::s08::f_002adee8__generated
        ),
        (
            oc1075_f_28bb10,
            OC1075_P_28BB10,
            ee::s07::OVERRIDE_0028BB10,
            ee::s07::f_0028bb10__generated
        ),
        (
            oc1075_f_28fea0,
            OC1075_P_28FEA0,
            ee::s07::OVERRIDE_0028FEA0,
            ee::s07::f_0028fea0__generated
        ),
        (
            oc1075_f_317e50,
            OC1075_P_317E50,
            ee::s10::OVERRIDE_00317E50,
            ee::s10::f_00317e50__generated
        ),
        (
            oc1075_f_317e98,
            OC1075_P_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated
        ),
        (
            oc1075_f_11b978,
            OC1075_P_11B978,
            ee::s00::OVERRIDE_0011B978,
            ee::s00::f_0011b978__generated
        ),
    );

    /// A test machine (random registers, miss hooks, a call that may unwind
    /// among the first `calls`), the game image, and the gate open.
    fn oc1075_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        elf_image(c);
        oc1075_install_fakes();
        c.bus.write32(OC1075_CTRL, OC1075_MAGIC);
        r
    }

    const OC1075_THIS: u32 = DATA + 0x1000;
    const OC1075_BASE: u32 = DATA + 0x2000;
    const OC1075_SUB: u32 = DATA + 0x4000;

    /// The object layout `CrowdAudio_dtor` / `MIDIBanks_dtor` touch: `*self`
    /// (+0x1d8 the sound sub-object, +0xab0, +0x1d4 written) and `self+0x10`.
    fn oc1075_midi_like_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = oc1075_machine(c, seed, entry as u64, 8);
        c.bus.write32(OC1075_THIS, OC1075_BASE);
        c.bus.write32(OC1075_BASE + 0x1d8, OC1075_SUB);
        c.bus.write32(
            OC1075_THIS + 0x10,
            if seed % 5 == 0 { 0 } else { DATA + 0x5000 },
        );
        c.bus.write32(OC1075_SUB + 4, r.next_u32());
        c.bus.write32(OC1075_BASE + 0xab0, r.next_u32());
        c.bus.write32(OC1075_BASE + 0x1d4, r.next_u32());
        c.g[A0][0] = sx(OC1075_THIS);
        c.g[A1][0] = (seed % 8) as u64;
        c.pc = entry;
    }

    /// `CrowdAudio_dtor`: flags 0..7 (stack vtable copies only for 0; the
    /// bank manager and sound reference for bit 1; the free for bit 0), the
    /// three crowd entries' sample lists, callees that unwind.
    #[test]
    fn crowd_audio_dtor_matches() {
        check_reaching(
            ee::s08::f_002a6b50__generated,
            crowd_audio_dtor,
            160,
            &[
                0x002a_6c64,
                0x002a_6c94,
                0x002a_6ca0,
                0x002a_6cac,
                0x002a_6cc0,
                0x002a_6cd4,
                0x002a_6ce4,
                0x002a_6cf4,
            ],
            &|c, seed| oc1075_midi_like_setup(c, seed, 0x002a_6b50),
        );
    }

    /// `PATH_PlatStopTrackSound`: all four track slots, the "playing" flag
    /// clear (stream stop) or set (sound stop), callees that unwind.
    #[test]
    fn oc1075_path_plat_stop_track_sound_matches() {
        check_reaching(
            ee::s08::f_002af838__generated,
            oc1075_path_plat_stop_track_sound,
            200,
            &[0x002a_f85c, 0x002a_f874, 0x002a_f884, 0x002a_f88c],
            &|c, seed| {
                let mut r = oc1075_machine(c, seed, 0x2a_f838, 3);
                let i = seed % 4;
                let obj = DATA + 0x1_0000 + i * 0x100;
                c.bus.write32(0x004d_3e98 + 4 * i, obj);
                let flag = if seed % 3 == 0 {
                    0
                } else {
                    1 + (seed % 100) as u8
                };
                c.bus.write8(obj + 0x64, flag);
                c.bus.write32(obj + 0x6c, DATA + 0x2_0000);
                c.bus.write32(DATA + 0x2_0004, r.next_u32());
                c.bus.write32(obj, r.next_u32());
                c.g[A0][0] = sx(i);
                c.pc = 0x002a_f838;
            },
        );
    }

    /// `Avalanche_UpdateInstance`: a node without data, positions past and
    /// inside the data's length, type-2 and other types, emitting or not,
    /// callees (`SampleHistory`, `SpawnAt`, `ReleaseInstance`) that unwind.
    #[test]
    fn oc1075_avalanche_update_instance_matches() {
        check_reaching(
            ee::s09::f_002d7ca8__generated,
            oc1075_avalanche_update_instance,
            400,
            &[0x002d_7ce8, 0x002d_7d98, 0x002d_7db8],
            &|c, seed| {
                let mut r = oc1075_machine(c, seed, 0x2d_7ca8, 3);
                let node = DATA + 0x3000;
                let data = DATA + 0x4_0000;
                c.bus
                    .write32(node + AU_DATA, if r.next_u32() % 7 == 0 { 0 } else { data });
                let rate = [0.5f32, 1.0, 2.0][(r.next_u32() % 3) as usize];
                let length = [1.0f32, 10.0, 100.0][(r.next_u32() % 3) as usize];
                let t = [0.0f32, 0.1, 1.0, 3.0][(r.next_u32() % 4) as usize];
                c.bus.write32(data + AU_D_RATE, rate.to_bits());
                c.bus.write32(data + AU_D_LENGTH, length.to_bits());
                c.bus.write16(data + AU_D_TYPE, (r.next_u32() % 3) as u16);
                c.bus.write8(data + AU_D_EMITS, (r.next_u32() % 2) as u8);
                c.bus.write32(node + AU_FB4, r.next_u32());
                put_words(c, GAME_GP - 0x3cd4, &[(1.0f32 / 60.0).to_bits()]);
                c.g[A0][0] = sx(node);
                c.f[12] = t.to_bits();
                c.pc = 0x002d_7ca8;
            },
        );
    }

    /// `Avalanche_WriteData`: a direct write, and buffered writes of 0..4 rows
    /// of 0..8 bytes (reversed copies; the write fake captures the buffer so
    /// the diff sees it), `EAFile_Write` that unwinds.
    #[test]
    fn oc1075_avalanche_write_data_matches() {
        check(
            ee::s09::f_002d82a0__generated,
            oc1075_avalanche_write_data,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x2d_82a0, 1);
                elf_image(c);
                oc1075_install_fakes();
                c.bus.write32(OC1075_CTRL, OC1075_MAGIC);
                let src = DATA + 0x100;
                fill(c, &mut r, src, 0x40);
                let rows = r.next_u32() % 5;
                let stride = [0u32, 1, 3, 5, 8][(r.next_u32() % 5) as usize];
                c.g[A0][0] = (seed % 3 == 0) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = sx(src);
                c.g[T0][0] = stride as u64;
                c.g[T1][0] = rows as u64;
                c.pc = 0x002d_82a0;
            },
        );
    }

    /// `Avalanche_GetProximityInfo`: 16 slots, empty / active, with 0..3
    /// nodes each (some without data, near and far from the position, types 1
    /// and 2), so the centre, scale, minimum, count and flag all vary.
    #[test]
    fn oc1075_avalanche_get_proximity_info_matches() {
        check(
            ee::s09::f_002da1c0__generated,
            oc1075_avalanche_get_proximity_info,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d_a1c0, 0);
                put_words(c, GAME_GP - 0x3cbc, &[999_999.0f32.to_bits()]);
                put_words(c, GAME_GP - 0x3cb8, &[225_000_000.0f32.to_bits()]);
                let pos = DATA + 0x200;
                for i in 0..3 {
                    c.bus.write32(pos + 4 * i, r.f32(8000.0));
                }
                let mut k = 0u32;
                for slot in 0..16u32 {
                    let at = 0x0053_8938 + 0x1c * slot;
                    let active = r.next_u32() % 3 != 0;
                    let mut head = 0;
                    let mut prev = 0;
                    for _ in 0..(r.next_u32() % 4) {
                        let node = DATA + 0x1000 + 0x400 * k;
                        let nd = DATA + 0x1_8000 + 0x100 * k;
                        k += 1;
                        if prev == 0 {
                            head = node;
                        } else {
                            c.bus.write32(prev + 0x2e4, node);
                        }
                        prev = node;
                        c.bus.write32(node + 0x2e4, 0);
                        c.bus
                            .write32(node + 0x2e0, if r.next_u32() % 5 == 0 { 0 } else { nd });
                        for j in 0..3 {
                            c.bus.write32(node + 0x60 + 4 * j, r.f32(8000.0));
                        }
                        c.bus.write32(node + 0x6c, 1.0f32.to_bits());
                        c.bus.write32(node + 0xb0, r.f32(10.0));
                        c.bus.write16(nd + 0xf0, 1 + (r.next_u32() % 2) as u16);
                    }
                    c.bus.write32(at, active as u32);
                    c.bus.write32(at + 4, head);
                }
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(pos);
                c.pc = 0x002d_a1c0;
            },
        );
    }

    /// The board effect destructors' fixture: the rider object at `*self`
    /// (vtable fields written), flags 0..7, buffers (and for the spark effect
    /// an effect object whose destroy method is a fake vtable slot).
    fn oc1075_board_setup(c: &mut Cpu, seed: u32, entry: u32, wake: bool) {
        let mut r = oc1075_machine(c, seed, entry as u64, 6);
        c.bus.write32(OC1075_THIS, OC1075_BASE);
        if wake {
            for off in [0xc, 0x10] {
                let v = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    DATA + 0x5000
                };
                c.bus.write32(OC1075_THIS + off, v);
            }
        } else {
            let effect = DATA + 0x3000;
            let vt = DATA + 0x6000;
            let has = r.next_u32() % 3 != 0;
            c.bus
                .write32(OC1075_THIS + 0x20, if has { effect } else { 0 });
            c.bus.write32(effect + 0x1f8, vt);
            c.bus.write16(vt + 0x20, (r.next_u32() & 0xfc) as u16);
            c.bus.write32(vt + 0x24, FAKE);
            c.bus.write32(OC1075_THIS + 0x24, DATA + 0x5000);
            c.bus.write32(OC1075_THIS + 0x28, DATA + 0x5100);
        }
        c.g[A0][0] = sx(OC1075_THIS);
        c.g[A1][0] = (seed % 8) as u64;
        c.pc = entry;
    }

    /// `cBoardSparkFX_dtor`: flags 0..7 (vtable copies only for 0, the rider
    /// destroy for bit 1, the free for bit 0), with and without the effect.
    #[test]
    fn board_spark_fx_dtor_matches() {
        check_reaching(
            ee::s09::f_002da8e8__generated,
            board_spark_fx_dtor,
            160,
            &[
                0x002d_aa24,
                0x002d_aa2c,
                0x002d_aa34,
                0x002d_aa4c,
                0x002d_aa60,
            ],
            &|c, seed| oc1075_board_setup(c, seed, 0x002d_a8e8, false),
        );
    }

    /// `cBoardWakeFX_dtor`: flags 0..7, with and without the two buffers.
    #[test]
    fn board_wake_fx_dtor_matches() {
        check_reaching(
            ee::s09::f_002dcda0__generated,
            board_wake_fx_dtor,
            160,
            &[0x002d_ced0, 0x002d_cee4, 0x002d_cefc, 0x002d_cf10],
            &|c, seed| oc1075_board_setup(c, seed, 0x002d_cda0, true),
        );
    }
}
