//! Staging module oc789 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc789`.

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
    (0x001a_2f70, ovui_sound_v02, "OVUISound_v02"),
    (
        0x002f_64f0,
        rider_base_clear_cheat_code,
        "RiderBase_ClearCheatCode",
    ),
    (0x002a_e690, path_plat_stop_track, "PATH_PlatStopTrack"),
    (
        0x002a_3eb8,
        ssx_audio_system_arcade_say_2145,
        "SSXAudioSystem_ArcadeSay2145",
    ),
    (0x0019_56f8, fe_module_base_ctor, "FEModule_BaseCtor"),
    (
        0x002b_42b8,
        music_player_select_next_song,
        "MusicPlayer_SelectNextSong",
    ),
];

/// `void cOVSound::playSound(cOVSound* self, int sel)` (0x1a2f70, `OVUISound_v02`).
///
/// Plays a front-end menu sound for `sel` 1..=6: a jump table (0x4613a0) picks
/// the case, which calls `SSXAudioSystem_GetInstance()` and then
/// `SSXAudioSystem_PlayFESound(audio, id)` with id 0xb, 0xc, 0xa, 0xd, 9, 9.
/// Any other `sel` does nothing. The resume entries (`pc != entry`) stay with
/// the generated `__entries`. A table entry equal to the function's own entry
/// (a restart) cannot occur in the game's constant table and is not handled.
pub fn ovui_sound_v02(c: &mut Cpu) {
    if c.pc != 0x001a_2f70 {
        return ee::s03::f_001a2f70__entries(c);
    }
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[V0][0] = (c.g[V1][0] < 6) as u64;
        let out_of_range = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !out_of_range {
            c.cycles += 7;
            c.g[V0][0] = 0x46_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x13a0));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            let target = c.g[A0][0] as u32;
            // (cycles of the case head, returns of its two calls, sound id,
            // falls into the common tail that charges one more cycle).
            let (cycles, ret_get, ret_play, id, via_tail) = match target {
                0x001a_2fa0 => (7, 0x001a_2fa8, 0x001a_2fb4, 0xb, false),
                0x001a_2fbc => (7, 0x001a_2fc4, 0x001a_2fd0, 0xc, false),
                0x001a_2fd8 => (7, 0x001a_2fe0, 0x001a_2fec, 0xa, false),
                0x001a_2ff4 => (7, 0x001a_2ffc, 0x001a_3008, 0xd, false),
                0x001a_3010 => (7, 0x001a_3018, 0x001a_3024, 0x9, false),
                0x001a_302c => (5, 0x001a_3034, 0x001a_3040, 0x9, true),
                _ => {
                    c.jump_out(target, 0x001a_2f70, 0x001a_304c);
                    return Ok(());
                }
            };
            c.cycles += cycles;
            call_ptr(c, 0x0028_b180, ret_get)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = id;
            call_ptr(c, 0x0029_4f78, ret_play)?;
            if via_tail {
                c.cycles += 1;
            }
            c.g[RA][0] = c.ld(sp(c));
        } else {
            c.cycles += 1;
            c.g[RA][0] = c.ld(sp(c));
        }
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `void RiderBase_ClearCheatCode(RiderBase* self)` (0x2f64f0): zeroes the
/// 4 bytes at `self + 4` (`memset(self + 4, 0, 4)`).
pub fn rider_base_clear_cheat_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 4;
        call_ptr(c, 0x0041_6210, 0x002f_6508)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PATH_PlatStopTrack(int track)` (0x2ae690): stops the platform track
/// `track` (slot table at 0x4d3e98). A track whose byte `+0x64` is set walks
/// the bank slots (`PATH_PlatFindBankSlot(track, -1)`; for each slot found
/// call the hook at `gp+0x6a4` when set and the slot's `+0x14` is non-null,
/// then `PATH_PlatBankSlotInit(slot)`) until none is left; a track with the
/// byte clear closes its stream (`SNDSTRM_Close(*track)`, 0x3b7838). Then
/// `PATH_PlatRemoveTrackSlot(track)`. The resume entries (`pc != entry`) stay
/// with the generated `__entries`.
pub fn path_plat_stop_track(c: &mut Cpu) {
    if c.pc != 0x002a_e690 {
        return ee::s08::f_002ae690__entries(c);
    }
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x4d_0000;
        frame.save(c, S1, 0x40);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
        frame.save(c, S4, 0x10);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save_ra(c, 0);
        frame.save(c, S2, 0x30);
        c.g[S2][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x64));
        // `'b_2ae740`: everything before the final RemoveTrackSlot.
        'stop: {
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A1][0] = u64::MAX;
                call_ptr(c, 0x002a_e230, 0x002a_e6d8)?;
                c.g[A0][0] = c.g[V0][0];
                let none = (c.g[A0][0] as i64) < 0;
                c.g[V0][0] = 0x4d_0000;
                if none {
                    break 'stop;
                }
                c.cycles += 3;
                c.g[S4][0] = 0x28;
                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3ef8));
                c.mult(0, V1, A0, S4);
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6a4));
                    let no_hook = c.g[V0][0] == 0;
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    if !no_hook {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        if c.g[A0][0] != 0 {
                            c.cycles += 2;
                            let hook = c.g[V0][0] as u32;
                            call_ptr(c, hook, 0x002a_e710)?;
                        }
                    }
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002a_e1f0, 0x002a_e718)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = u64::MAX;
                    call_ptr(c, 0x002a_e230, 0x002a_e724)?;
                    c.g[A0][0] = c.g[V0][0];
                    let more = (c.g[A0][0] as i64) >= 0;
                    c.mult(0, V1, A0, S4);
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x002a_e6f0);
                        }
                        continue;
                    }
                    c.cycles += 2;
                    break 'stop;
                }
            }
            c.cycles += 2;
            c.g[A0][0] = c.lw(c.g[S2][0] as u32);
            call_ptr(c, 0x003b_7838, 0x002a_e740)?;
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002a_e188, 0x002a_e748)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_ArcadeSay2145(SSXAudioSystem* sys, Obj* who, int arg)`
/// (0x2a3eb8): says arcade phrase 0x2145 when arcade speech is on
/// (`IsArcadeSpeechOn`), `who` is non-null with `who->+0x470 >= 0` (float) and
/// no replay is playing (`IsReplayPlaying(sys)`): sets the speech event
/// context on `sys + 0x5560` and, when that succeeds, makes the id
/// (`SPCH_MakeId(1, 0, 0x2145)`) and calls the hook at `gp+0x173c` with
/// `(id, 1, arg)`.
pub fn ssx_audio_system_arcade_say_2145(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        let ra = 0x002a_3ee0;
        c.g[RA][0] = sx(ra);
        frame.save_fpr(c, 20, 0x40);
        call_ptr(c, 0x0029_f160, ra)?;
        'done: {
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'done;
            }
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'done;
            }
            c.cycles += 6;
            c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x470));
            c.mtc1(0, 20);
            c.c_le_s(20, 0);
            let le = c.fcc();
            c.g[V0][0] = 1;
            if !le {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'done;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0028_8ae0, 0x002a_3f1c)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'done;
            }
            c.cycles += 11;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
            c.mov_s(12, 20);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x2145;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xc;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            call_ptr(c, 0x002b_1458, 0x002a_3f48)?;
            let failed = c.g[V0][0] == 0;
            c.g[A0][0] = 1;
            if !failed {
                c.cycles += 8;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x2145;
                call_ptr(c, 0x003d_8008, 0x002a_3f5c)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x173c));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                let hook = c.g[V1][0] as u32;
                c.g[A1][0] = 1;
                call_ptr(c, hook, 0x002a_3f70)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `FEModule* FEModule_BaseCtor(FEModule* self)` (0x1956f8): base constructor of
/// a front-end module: `AppModule_ctor(self)`, vtable `self[0]` = 0x46d878,
/// `CameraBase_ctor(self + 0x10)`, the second vtable (`+0xa0`) with
/// `FEModule_InitRiderPreviews(self + 0xb0)`, `BoltFile_CacheCtor(self +
/// 0x1a70)`, the UI engine (`self[3] = UIEngine_ctor(EA_MemAllocNamed(0x70,
/// ..))`), `CameraBase_Init(self + 0x10, 0, gp float args)` and the FX loader
/// (`self[0xb0000 + 0x5aa0] = FXLoader_ctor(EA_MemAllocNamed(1, ..), 1)`).
/// Returns `self`.
pub fn fe_module_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 53;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0023_1cd0, 0x0019_5710)?;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = 0x47_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2788));
        c.sw(c.g[S1][0] as u32, c.g[V1][0]);
        call_ptr(c, 0x0015_d698, 0x0019_5728)?;
        c.g[V1][0] = 0x47_0000;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb0));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2728));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
        call_ptr(c, 0x001a_03c0, 0x0019_573c)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1a70));
        call_ptr(c, 0x0019_cdf0, 0x0019_5744)?;
        c.g[A1][0] = 0x46_0000;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f0));
        c.g[A0][0] = 0x70;
        call_ptr(c, 0x0031_7d70, 0x0019_575c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0039_7b08, 0x0019_5764)?;
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x59b8));
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(AT, 13);
        c.g[A1][0] = 0;
        c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x59b4));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x0015_d700, 0x0019_5784)?;
        c.g[S0][0] = 0xb_0000;
        c.g[A1][0] = 0x46_0000;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x200));
        c.g[A0][0] = 1;
        call_ptr(c, 0x0031_7d70, 0x0019_57a4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x002d_bd10, 0x0019_57b0)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5aa0), c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_SelectNextSong(MusicPlayer* p, int by_slot, int slot)`
/// (0x2b42b8): advance the current song index (`+0x3f0`, count at `+0x3e4`,
/// wrapping to 0) to the next song whose bit is set in the 64-bit mask at
/// `+0x3f8`. With `by_slot` set and `p[0x920 + 4*slot]` non-zero the song must
/// also have a non-zero entry in the table at `+0x420` (20 bytes per song, word
/// `slot`).
pub fn music_player_select_next_song(c: &mut Cpu) {
    c.cycles += 2;
    let by_slot = c.g[A1][0] != 0;
    if !by_slot {
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x3e4));
    } else {
        c.cycles += 5;
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x920));
        let filtered = c.g[V1][0] != 0;
        c.g[T1][0] = c.g[A2][0];
        if filtered {
            c.cycles += 5;
            c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x3e4));
            c.g[A2][0] = 0x14;
            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x420));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x3f0));
            loop {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                let in_range = c.g[V1][0] != 0;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x3f0), c.g[V0][0]);
                if !in_range {
                    c.cycles += 1;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x3f0), 0);
                }
                c.cycles += 9;
                music_player_song_bit(c);
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x3f0));
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_4330);
                    }
                    continue;
                }
                c.cycles += 6;
                c.mult(0, T0, V1, A2);
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x3f0));
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_4330);
                    }
                    continue;
                }
                c.cycles += 2;
                ret(c);
                return;
            }
        } else {
            c.cycles += 1;
            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x3e4));
        }
    }
    c.cycles += 1;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x3f0));
    loop {
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let in_range = c.g[V1][0] != 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x3f0), c.g[V0][0]);
        if !in_range {
            c.cycles += 1;
            c.sw((c.g[A0][0] as u32).wrapping_add(0x3f0), 0);
        }
        c.cycles += 9;
        music_player_song_bit(c);
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x3f0));
            if event_due(c) {
                deliver_event_here(c, 0x002b_42dc);
            }
            continue;
        }
        c.cycles += 2;
        ret(c);
        return;
    }
}

/// The shared test of `music_player_select_next_song`'s loops: `v0 = ld
/// p[+0x3f8]; v1 = lw p[+0x3f0]; v0 = !(sext32(v0 >> v1) & 1)` (non-zero
/// while the current song's mask bit is clear), leaving `v1` = the index.
fn music_player_song_bit(c: &mut Cpu) {
    c.g[V0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x3f8));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x3f0));
    let shift = c.g[V1][0] & 63;
    c.g[V0][0] = c.g[V0][0] >> shift;
    c.g[V0][0] <<= 32;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
    c.g[V0][0] ^= 1;
    c.g[V0][0] &= 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, put_words, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc789's callee fakes in a machine's RAM.
    pub(crate) const OC789_CTRL: u32 = DATA + 0x30_0000;
    pub(crate) const OC789_MAGIC: u32 = 0x0789_f4ce;
    /// Per-callee queues of forced results (`+0` next index, `+4..` values).
    pub(crate) const OC789_QUEUE: u32 = DATA + 0x31_0000;

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc789_arm(c: &mut Cpu) {
        c.bus.write32(OC789_CTRL, OC789_MAGIC);
    }

    /// The queue word that makes a fake return `v` (sign-extended) in `v0`.
    pub(crate) fn oc789_force(v: i32) -> u32 {
        (v as u32).wrapping_add(3)
    }

    /// Queue the results of the first calls of the fake for `addr`; later
    /// calls (and zero words) return the fakes' hash pointer.
    pub(crate) fn oc789_queue(c: &mut Cpu, addr: u32, words: &[u32]) {
        put_words(c, OC789_QUEUE + (addr & 0xfff) * 0x40 + 4, words);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// next queued result of its address unless it unwound.
    pub(crate) fn oc789_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let q = OC789_QUEUE + (addr & 0xfff) * 0x40;
        let k = c.bus.read32(q);
        let word = if k < 8 {
            c.bus.read32(q + 4 + 4 * k)
        } else {
            0
        };
        c.bus.write32(q, k + 1);
        fakes::fake_call(c, addr);
        if c.pc == ra && word != 0 {
            c.g[V0][0] = sx(word.wrapping_sub(3));
        }
    }

    macro_rules! oc789_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC789_CTRL) != OC789_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc789_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc789_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc789_fakes!(
        (
            oc789_fk_28b180,
            OC789_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc789_fk_416210,
            OC789_PREV_416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
        (
            oc789_fk_2ae230,
            OC789_PREV_2AE230,
            ee::s08::OVERRIDE_002AE230,
            ee::s08::f_002ae230__generated,
            0x002a_e230
        ),
        (
            oc789_fk_2ae1f0,
            OC789_PREV_2AE1F0,
            ee::s08::OVERRIDE_002AE1F0,
            ee::s08::f_002ae1f0__generated,
            0x002a_e1f0
        ),
        (
            oc789_fk_2ae188,
            OC789_PREV_2AE188,
            ee::s08::OVERRIDE_002AE188,
            ee::s08::f_002ae188__generated,
            0x002a_e188
        ),
        (
            oc789_fk_29f160,
            OC789_PREV_29F160,
            ee::s08::OVERRIDE_0029F160,
            ee::s08::f_0029f160__generated,
            0x0029_f160
        ),
        (
            oc789_fk_288ae0,
            OC789_PREV_288AE0,
            ee::s07::OVERRIDE_00288AE0,
            ee::s07::f_00288ae0__generated,
            0x0028_8ae0
        ),
        (
            oc789_fk_2b1458,
            OC789_PREV_2B1458,
            ee::s08::OVERRIDE_002B1458,
            ee::s08::f_002b1458__generated,
            0x002b_1458
        ),
        (
            oc789_fk_3d8008,
            OC789_PREV_3D8008,
            ee::s14::OVERRIDE_003D8008,
            ee::s14::f_003d8008__generated,
            0x003d_8008
        ),
        (
            oc789_fk_3e18c8,
            OC789_PREV_3E18C8,
            ee::s14::OVERRIDE_003E18C8,
            ee::s14::f_003e18c8__generated,
            0x003e_18c8
        ),
        (
            oc789_fk_231cd0,
            OC789_PREV_231CD0,
            ee::s06::OVERRIDE_00231CD0,
            ee::s06::f_00231cd0__generated,
            0x0023_1cd0
        ),
        (
            oc789_fk_2523a8,
            OC789_PREV_2523A8,
            ee::s06::OVERRIDE_002523A8,
            ee::s06::f_002523a8__generated,
            0x0025_23a8
        ),
        (
            oc789_fk_15d698,
            OC789_PREV_15D698,
            ee::s01::OVERRIDE_0015D698,
            ee::s01::f_0015d698__generated,
            0x0015_d698
        ),
        (
            oc789_fk_15d700,
            OC789_PREV_15D700,
            ee::s01::OVERRIDE_0015D700,
            ee::s01::f_0015d700__generated,
            0x0015_d700
        ),
        (
            oc789_fk_1a03c0,
            OC789_PREV_1A03C0,
            ee::s03::OVERRIDE_001A03C0,
            ee::s03::f_001a03c0__generated,
            0x001a_03c0
        ),
        (
            oc789_fk_19cdf0,
            OC789_PREV_19CDF0,
            ee::s03::OVERRIDE_0019CDF0,
            ee::s03::f_0019cdf0__generated,
            0x0019_cdf0
        ),
        (
            oc789_fk_317d70,
            OC789_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc789_fk_397b08,
            OC789_PREV_397B08,
            ee::s13::OVERRIDE_00397B08,
            ee::s13::f_00397b08__generated,
            0x0039_7b08
        ),
        (
            oc789_fk_3b3900,
            OC789_PREV_3B3900,
            ee::s13::OVERRIDE_003B3900,
            ee::s13::f_003b3900__generated,
            0x003b_3900
        ),
    );

    /// `OVUISound_v02`: sel 0..=8 (out of range, the six cases), the jump
    /// table's targets, an unknown target (jump out), and unwinds of the
    /// `GetInstance` / `PlayFESound` calls. `PlayFESound` itself runs for real
    /// (its jump table is empty in the test RAM, so it jumps out to a fake).
    #[test]
    fn ovui_sound_v02_matches() {
        oc789_install_fakes();
        let table: [u32; 6] = [
            0x001a_2fa0,
            0x001a_2fbc,
            0x001a_2fd8,
            0x001a_2ff4,
            0x001a_3010,
            0x001a_302c,
        ];
        fakes::covered(
            ee::s03::f_001a2f70__generated,
            ovui_sound_v02,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x789a, 3);
                c.pc = 0x001a_2f70;
                oc789_arm(c);
                put_words(c, 0x0046_13a0, &table);
                let sel = seed % 9;
                c.g[A1][0] = if seed % 13 == 12 {
                    r.next_u32() as u64
                } else {
                    sx(sel)
                };
                if seed % 29 == 7 && (1..=6).contains(&sel) {
                    c.bus.write32(0x0046_13a0 + 4 * (sel - 1), 0x001a_2f90);
                }
            },
        );
    }

    /// `RiderBase_ClearCheatCode`: one `memset(self + 4, 0, 4)` call, unwinding or not.
    #[test]
    fn rider_base_clear_cheat_code_matches() {
        oc789_install_fakes();
        fakes::covered(
            ee::s09::f_002f64f0__generated,
            rider_base_clear_cheat_code,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x789c, 1);
                c.pc = 0x002f_64f0;
                oc789_arm(c);
                c.g[A0][0] = sx(DATA + 0x1_0000 + (r.next_u32() & 0xfff0));
            },
        );
    }

    /// `PATH_PlatStopTrack`: tracks whose byte `+0x64` is clear (stream close,
    /// run for real: `SNDSTRM_GetStream` finds no stream in the empty RAM) or
    /// set; no bank slot, one to three slots (with and without the hook at
    /// `gp+0x6a4` and with and without a slot `+0x14` object); unwinds.
    #[test]
    fn path_plat_stop_track_matches() {
        oc789_install_fakes();
        fakes::covered(
            ee::s08::f_002ae690__generated,
            path_plat_stop_track,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x789b, 8);
                c.pc = 0x002a_e690;
                oc789_arm(c);
                let track = seed % 3;
                for i in 0..3u32 {
                    let t = DATA + 0x1_0000 + i * 0x100;
                    c.bus.write32(0x004d_3e98 + 4 * i, t);
                    c.bus.write32(t, r.next_u32() & 0x3);
                    let flag = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        1 + r.next_u32() as u8 % 100
                    };
                    c.bus.write8(t + 0x64, flag);
                }
                for i in 0..4u32 {
                    let nonnull = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x2_0000
                    };
                    c.bus.write32(0x004d_3ef8 + 0x28 * i + 0x14, nonnull);
                }
                let hook = if r.next_u32() & 1 == 0 { FAKE } else { 0 };
                c.bus.write32(GAME_GP + 0x6a4, hook);
                // FindBankSlot results: some slots (0..=3), then -1.
                let mut q = Vec::new();
                if r.next_u32() % 4 != 0 {
                    for _ in 0..r.next_u32() % 4 {
                        q.push(oc789_force((r.next_u32() % 4) as i32));
                    }
                }
                q.push(oc789_force(-1));
                oc789_queue(c, 0x002a_e230, &q);
                c.g[A0][0] = sx(track);
            },
        );
    }

    /// `SSXAudioSystem_ArcadeSay2145`: speech off, no `who`, `who->+0x470`
    /// negative or not (edge floats), replay playing, context refused, and the
    /// full path; unwinds.
    #[test]
    fn ssx_audio_system_arcade_say_2145_matches() {
        oc789_install_fakes();
        fakes::covered(
            ee::s08::f_002a3eb8__generated,
            ssx_audio_system_arcade_say_2145,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x789d, 6);
                c.pc = 0x002a_3eb8;
                oc789_arm(c);
                let who = DATA + 0x2_0000;
                c.g[A0][0] = sx(DATA + 0x1_0000);
                c.g[A1][0] = if r.next_u32() % 6 == 0 { 0 } else { sx(who) };
                c.g[A2][0] = r.next_u32() as u64;
                let f = if r.next_u32() % 3 == 0 {
                    r.f32(100.0)
                } else {
                    r.edge_f32()
                };
                c.bus.write32(who + 0x470, f);
                c.bus.write32(GAME_GP + 0x173c, FAKE);
                let on = if r.next_u32() % 6 == 0 { 0 } else { 1 };
                oc789_queue(c, 0x0029_f160, &[oc789_force(on)]);
                let replay = r.next_u32() % 4 == 0;
                oc789_queue(c, 0x0028_8ae0, &[oc789_force(replay as i32)]);
                let ctx = if r.next_u32() % 3 == 0 { 0 } else { 1 };
                oc789_queue(c, 0x002b_1458, &[oc789_force(ctx)]);
            },
        );
    }

    /// `FEModule_BaseCtor`: the whole constructor chain (callees faked, the FX
    /// loader `FXLoader_ctor` runs for real on the empty RAM), with unwinds.
    #[test]
    fn fe_module_base_ctor_matches() {
        oc789_install_fakes();
        fakes::covered(
            ee::s02::f_001956f8__generated,
            fe_module_base_ctor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x789e, 9);
                c.pc = 0x0019_56f8;
                oc789_arm(c);
                c.g[A0][0] = sx(DATA + 0x10_0000 + (r.next_u32() & 0xf) * 0x10);
                c.bus.write32(GAME_GP.wrapping_sub(0x59b8), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x59b4), r.edge_f32());
            },
        );
    }

    /// `MusicPlayer_SelectNextSong`: with and without the by-slot filter, a
    /// slot whose table word is set or not, the index wrapping, masks with the
    /// next song at various distances (at least one usable song always exists).
    #[test]
    fn music_player_select_next_song_matches() {
        check(
            ee::s08::f_002b42b8__generated,
            music_player_select_next_song,
            400,
            &|c, seed| {
                let mut r = Rng(0x789f_1234 ^ (seed as u64) << 7);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x002b_42b8;
                let p = DATA + 0x1_0000;
                let count = 1 + r.next_u32() % 6;
                let good = r.next_u32() % count;
                c.bus.write32(p + 0x3e4, count);
                c.bus.write32(p + 0x3f0, r.next_u32() % (count + 2));
                let mask = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.bus.write64(p + 0x3f8, mask | 1 << good);
                let slot = r.next_u32() % 4;
                let by_slot = seed % 4 != 0;
                let filtered = seed % 3 != 0;
                c.g[A0][0] = sx(p);
                c.g[A1][0] = if by_slot {
                    1 + r.next_u32() as u64 % 5
                } else {
                    0
                };
                c.g[A2][0] = sx(slot);
                c.bus
                    .write32(p + 0x920 + 4 * slot, if filtered { 1 } else { 0 });
                for song in 0..8u32 {
                    let v = if song == good || r.next_u32() % 2 == 0 {
                        7
                    } else {
                        0
                    };
                    c.bus.write32(p + 0x420 + 0x14 * song + 4 * slot, v);
                }
            },
        );
    }
}
