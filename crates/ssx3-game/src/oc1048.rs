//! Staging module oc1048 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1048`.

#![allow(unused_imports)]

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002b_1020,
        speech_system_set_instance_param_8c8,
        "SpeechSystem_SetInstanceParam8c8",
    ),
    (
        0x002b_1058,
        speech_system_request_event,
        "SpeechSystem_RequestEvent",
    ),
    (
        0x002b_1128,
        speech_system_flush_instance,
        "SpeechSystem_FlushInstance",
    ),
    (
        0x002b_11e8,
        speech_system_is_instance_playing,
        "SpeechSystem_IsInstancePlaying",
    ),
    (
        0x002b_1a70,
        music_player_stop_stream,
        "MusicPlayer_StopStream",
    ),
    (
        0x002b_2140,
        song_instance_path_cmd_b,
        "SongInstance_PathCmdB",
    ),
    (
        0x002b_2500,
        song_instance_pick_random_phrase_offset,
        "SongInstance_PickRandomPhraseOffset",
    ),
    (
        0x002b_2550,
        song_instance_get_path_status_field,
        "SongInstance_GetPathStatusField",
    ),
];

/// Callees of this module's replacements (the generated alias of each, which
/// dispatches through its override slot).
const SPEECH_INSTANCE_SET_PARAM_8C8: Callee = Callee {
    addr: 0x002b_07f0,
    f: ee::s08::f_002b07f0,
};
const SPEECH_SYSTEM_REQUEST_EVENT: Callee = Callee {
    addr: 0x002b_1058,
    f: ee::s08::f_002b1058,
};
const SPEECH_INSTANCE_IS_PLAYING: Callee = Callee {
    addr: 0x002b_07c0,
    f: ee::s08::f_002b07c0,
};
const STREAM_ARRAY_STOP_FROM_MUSIC: Callee = Callee {
    addr: 0x002a_ae40,
    f: ee::s08::f_002aae40,
};
const AUDIO_RAND_FOR_PHRASE: Callee = Callee {
    addr: 0x002a_df60,
    f: ee::s08::f_002adf60,
};

/// `cSpeechSys::mSpeechInstance`: the channel table pointer at +0x4.
const SPEECH_CHANNEL_TABLE: u32 = 0x4;
/// Instance fields written by the speech channel code.
const SPEECH_INST_RADIUS: u32 = 0x8f4;
const SPEECH_INST_POSITION: u32 = 0x8cc;
const SPEECH_INST_USER_INFO: u32 = 0x8d0;
/// `cSpeechSys` callback fields.
const SPEECH_SYS_CALLBACK: u32 = 0xc;
const SPEECH_SYS_CALLBACK_INSTANCE: u32 = 0x10;
/// Game-image word that counts the speech flush (`gp + 0x6cc`).
const SPEECH_FLUSH_FLAG_GP: u32 = 0x6cc;
/// Speech sentence player entry (`0x3d8040`), called through the table.
const SPEECH_PLAYER_PLAY: u32 = 0x003d_8040;

/// `void SpeechSystem_SetInstanceParam8c8(cSpeechSys* self, int idx, int hold)`
/// (0x2b1020): when channel `idx` holds an instance, store `hold` in its
/// `+0x8c8` word (`SpeechInstance_SetParam8c8`).
pub fn speech_system_set_instance_param_8c8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(SPEECH_CHANNEL_TABLE)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            c.g[A1][0] = c.g[A2][0];
            SPEECH_INSTANCE_SET_PARAM_8C8.call(c, 0x002b_1048)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void SpeechSystem_RequestEvent(cSpeechSys* self, int idx, int playArg,
/// void (*callback)(int), int volumeArg, float radius)` (0x2b1058): play the
/// sentence on channel `idx` if it holds an instance. The instance gets the
/// position, radius (kept only when `radius > 0`) and user info; a callback
/// is registered for the duration of the player call, then cleared, and the
/// instance's radius, position and user info are reset to zero.
pub fn speech_system_request_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SPEECH_CHANNEL_TABLE)).lw(c);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[V0][0] != 0 {
            // Radius: only when the float argument is positive (f20 = 0.0).
            c.cycles += 5;
            c.mtc1(0, 20);
            c.c_lt_s(20, 12);
            let not_positive = !c.fcc();
            let play_pos = c.g[A2][0];
            Addr((c.g[V0][0] as u32).wrapping_add(SPEECH_INST_POSITION)).sw(c, play_pos);
            if !not_positive {
                c.cycles += 4;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SPEECH_CHANNEL_TABLE)).lw(c);
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.swc1(12, (c.g[V1][0] as u32).wrapping_add(SPEECH_INST_RADIUS));
            }
            // User info, then the callback pair when a callback is given.
            c.cycles += 5;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SPEECH_CHANNEL_TABLE)).lw(c);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let no_callback = c.g[A3][0] == 0;
            let volume = c.g[T0][0];
            Addr((c.g[V1][0] as u32).wrapping_add(SPEECH_INST_USER_INFO)).sw(c, volume);
            if !no_callback {
                c.cycles += 2;
                let callback = c.g[A3][0];
                Addr((c.g[S0][0] as u32).wrapping_add(SPEECH_SYS_CALLBACK)).sw(c, callback);
                let idx = c.g[A0][0];
                Addr((c.g[S0][0] as u32).wrapping_add(SPEECH_SYS_CALLBACK_INSTANCE)).sw(c, idx);
            }
            c.cycles += 18;
            call_ptr(c, SPEECH_PLAYER_PLAY, 0x002b_10d4)?;
            // Clear the callback and reset the instance's fields.
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SPEECH_CHANNEL_TABLE)).lw(c);
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            let minus_one = c.g[V0][0];
            Addr((c.g[S0][0] as u32).wrapping_add(SPEECH_SYS_CALLBACK_INSTANCE)).sw(c, minus_one);
            Addr((c.g[S0][0] as u32).wrapping_add(SPEECH_SYS_CALLBACK)).sw(c, 0);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.swc1(20, (c.g[V1][0] as u32).wrapping_add(SPEECH_INST_RADIUS));
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SPEECH_CHANNEL_TABLE)).lw(c);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            Addr((c.g[V1][0] as u32).wrapping_add(SPEECH_INST_POSITION)).sw(c, 0);
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SPEECH_CHANNEL_TABLE)).lw(c);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            Addr((c.g[V1][0] as u32).wrapping_add(SPEECH_INST_USER_INFO)).sw(c, 0);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void SpeechSystem_FlushInstance(cSpeechSys* self, int idx)` (0x2b1128):
/// when channel `idx` holds an instance, request its event four times
/// (`idx`, `idx`, `idx`, `idx`, with no argument and zero radius), with the
/// flush flag set around the loop.
pub fn speech_system_flush_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(SPEECH_CHANNEL_TABLE)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 1;
        if !empty {
            c.cycles += 3;
            c.g[S0][0] = 3;
            let flag = c.g[V0][0];
            Addr((c.g[GP][0] as u32).wrapping_add(SPEECH_FLUSH_FLAG_GP)).sw(c, flag);
            loop {
                c.cycles += 10;
                c.mtc1(0, 12);
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                SPEECH_SYSTEM_REQUEST_EVENT.call(c, 0x002b_1184)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                if (c.g[S0][0] as i64) >= 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_1168);
                    }
                } else {
                    c.cycles += 1;
                    Addr((c.g[GP][0] as u32).wrapping_add(SPEECH_FLUSH_FLAG_GP)).sw(c, 0);
                    break;
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool SpeechSystem_IsInstancePlaying(cSpeechSys* self, int idx)`
/// (0x2b11e8): whether channel `idx` holds an instance that is playing.
pub fn speech_system_is_instance_playing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(SPEECH_CHANNEL_TABLE)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            SPEECH_INSTANCE_IS_PLAYING.call(c, 0x002b_1210)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_StopStream(void)` (0x2b1a70): stop stream slot 2 of the
/// music system's stream array (`StreamArray_Stop`).
pub fn music_player_stop_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(GP_MUSIC_SYS)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 2;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        STREAM_ARRAY_STOP_FROM_MUSIC.call(c, 0x002b_1a88)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SongInstance_PathCmdB(void)` (0x2b2140): `PATH` command 0x11000001
/// through the path table (`0x3d15c8`), returning what it returns.
pub fn song_instance_path_cmd_b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1100_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A0][0] | 0x1;
        call_ptr(c, PATH_FIND_TRACK, 0x002b_2154)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SongInstance_PickRandomPhraseOffset(SongInstance* self)` (0x2b2500):
/// a random phrase index (`Audio_Rand % (phrasesPerBank - 1)`) times the
/// configuration's phrase alignment.
pub fn song_instance_pick_random_phrase_offset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(GP_SOUND_RAND)).lw(c);
        AUDIO_RAND_FOR_PHRASE.call(c, 0x002b_2518)?;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x6c)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        // Both branches of the zero test fall through to the same code, so
        // only the division itself is kept.
        c.divu(0, 2, 3);
        c.cycles += 7;
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x74)).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.g[A0][0] = c.hi[0];
        c.mult(0, 2, 4, 2);
        frame.close(c);
        Ok(())
    })
}

/// `int SongInstance_GetPathStatusField(void)` (0x2b2550): the status word
/// the path table writes at `buf[9]` (`PATH_status`).
pub fn song_instance_get_path_status_field(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[A0][0] = 0x1100_0000;
        frame.save_ra(c, 0x30);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[A0][0] | 0x1;
        call_ptr(c, PATH_STATUS, 0x002b_2568)?;
        c.g[V0][0] = Addr(frame.at(c, 0x24)).lw(c);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// Game-image words the functions above read (`gp + off`).
const GP_MUSIC_SYS: u32 = 0x6fc;
const GP_SOUND_RAND: u32 = 0x68c;
/// Path table entries (`0x3d15c8` FindTrack, `0x3d11b8` status).
const PATH_FIND_TRACK: u32 = 0x003d_15c8;
const PATH_STATUS: u32 = 0x003d_11b8;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (own address and magic in the
    /// machine's RAM: other tests' machines run the real bodies).
    const OC1048_CTRL: u32 = DATA + 0x20_0000;
    const OC1048_MAGIC: u32 = 0x1048_f00d;
    /// The value the leaf fakes return (Audio_Rand, IsPlaying, PATH_status).
    const OC1048_MAILBOX: u32 = DATA + 0x20_0008;
    /// Fixture objects (clear of the fakes' result pointers, `DATA..+0x40000`).
    const OC1048_SELF: u32 = DATA + 0x5_0000;
    const OC1048_CHANS: u32 = DATA + 0x5_1000;
    const OC1048_INST: u32 = DATA + 0x5_2000;
    const OC1048_MUSIC: u32 = DATA + 0x5_8000;
    const OC1048_STREAMS: u32 = DATA + 0x5_8100;
    const OC1048_SLOTS: u32 = DATA + 0x5_8200;
    const OC1048_SONG: u32 = DATA + 0x5_9000;
    const OC1048_CONFIG: u32 = DATA + 0x5_9100;

    /// Leaf stand-in for the speech sentence player (`0x3d8040`): returns 0.
    fn oc1048_fake_player(c: &mut Cpu) {
        if c.bus.read32(OC1048_CTRL) != OC1048_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1048_fake_player),
                ee::s14::f_003d8040__generated,
            );
        }
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }

    /// Leaf stand-in for `SpeechInstance_IsPlaying` (`0x2b07c0`): returns the mailbox.
    fn oc1048_fake_is_playing(c: &mut Cpu) {
        if c.bus.read32(OC1048_CTRL) != OC1048_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1048_fake_is_playing),
                ee::s08::f_002b07c0__generated,
            );
        }
        c.g[V0][0] = sx(c.bus.read32(OC1048_MAILBOX));
        c.pc = c.g[RA][0] as u32;
    }

    /// Leaf stand-in for `Audio_Rand` (`0x2adf60`): returns the mailbox.
    fn oc1048_fake_audio_rand(c: &mut Cpu) {
        if c.bus.read32(OC1048_CTRL) != OC1048_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1048_fake_audio_rand),
                ee::s08::f_002adf60__generated,
            );
        }
        c.g[V0][0] = sx(c.bus.read32(OC1048_MAILBOX));
        c.pc = c.g[RA][0] as u32;
    }

    /// Leaf stand-in for the path table's FindTrack (`0x3d15c8`): no effect.
    fn oc1048_fake_find_track(c: &mut Cpu) {
        if c.bus.read32(OC1048_CTRL) != OC1048_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1048_fake_find_track),
                ee::s14::f_003d15c8__generated,
            );
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Leaf stand-in for `PATH_status` (`0x3d11b8`): writes the mailbox at buf[9].
    fn oc1048_fake_path_status(c: &mut Cpu) {
        if c.bus.read32(OC1048_CTRL) != OC1048_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1048_fake_path_status),
                ee::s14::f_003d11b8__generated,
            );
        }
        let buf = c.g[A1][0] as u32;
        let word = c.bus.read32(OC1048_MAILBOX);
        c.bus.write32(buf.wrapping_add(0x24), word);
        c.pc = c.g[RA][0] as u32;
    }

    /// Puts the gated fakes in their override slots (once per process).
    fn oc1048_hook() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D8040),
            oc1048_fake_player,
            fakes::prev_cell(oc1048_fake_player),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B07C0),
            oc1048_fake_is_playing,
            fakes::prev_cell(oc1048_fake_is_playing),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADF60),
            oc1048_fake_audio_rand,
            fakes::prev_cell(oc1048_fake_audio_rand),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D15C8),
            oc1048_fake_find_track,
            fakes::prev_cell(oc1048_fake_find_track),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D11B8),
            oc1048_fake_path_status,
            fakes::prev_cell(oc1048_fake_path_status),
        );
    }

    /// Hooked fakes, scrambled registers, the gate on, the mailbox random.
    fn oc1048_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        oc1048_hook();
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(OC1048_CTRL, OC1048_MAGIC);
        c.bus.write32(OC1048_MAILBOX, r.next_u32());
        r
    }

    /// A `cSpeechSys` whose channel table holds four channels; channel `i` is
    /// empty for some seeds, and each instance has random fields.
    fn oc1048_speech(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus
            .write32(OC1048_SELF + SPEECH_CHANNEL_TABLE, OC1048_CHANS);
        c.bus
            .write32(OC1048_SELF + SPEECH_SYS_CALLBACK, r.next_u32());
        c.bus
            .write32(OC1048_SELF + SPEECH_SYS_CALLBACK_INSTANCE, r.next_u32());
        for i in 0..4u32 {
            let inst = if (seed + i) % 5 == 0 {
                0
            } else {
                OC1048_INST + 0x1000 * i
            };
            c.bus.write32(OC1048_CHANS + 4 * i, inst);
            if inst != 0 {
                for off in [
                    0x8c8,
                    SPEECH_INST_POSITION,
                    SPEECH_INST_USER_INFO,
                    SPEECH_INST_RADIUS,
                ] {
                    c.bus.write32(inst + off, r.next_u32());
                }
            }
        }
    }

    #[test]
    fn speech_system_set_instance_param_8c8_matches() {
        check(
            ee::s08::f_002b1020__generated,
            speech_system_set_instance_param_8c8,
            300,
            &|c, seed| {
                let mut r = oc1048_machine(c, seed, 0x1020);
                oc1048_speech(c, &mut r, seed);
                c.g[A0][0] = sx(OC1048_SELF);
                c.g[A1][0] = sx(seed % 4);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn speech_system_request_event_matches() {
        check(
            ee::s08::f_002b1058__generated,
            speech_system_request_event,
            300,
            &|c, seed| {
                let mut r = oc1048_machine(c, seed, 0x1058);
                oc1048_speech(c, &mut r, seed);
                c.g[A0][0] = sx(OC1048_SELF);
                c.g[A1][0] = sx(seed % 4);
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = if seed % 3 == 0 { 0 } else { sx(r.next_u32()) };
                c.g[T0][0] = sx(r.next_u32());
                c.f[12] = if seed % 2 == 0 {
                    0x3fc0_0000
                } else {
                    r.edge_f32()
                };
            },
        );
    }

    #[test]
    fn speech_system_flush_instance_matches() {
        check(
            ee::s08::f_002b1128__generated,
            speech_system_flush_instance,
            300,
            &|c, seed| {
                let mut r = oc1048_machine(c, seed, 0x1128);
                oc1048_speech(c, &mut r, seed);
                c.g[A0][0] = sx(OC1048_SELF);
                c.g[A1][0] = sx(seed % 4);
            },
        );
    }

    #[test]
    fn speech_system_is_instance_playing_matches() {
        check(
            ee::s08::f_002b11e8__generated,
            speech_system_is_instance_playing,
            300,
            &|c, seed| {
                let mut r = oc1048_machine(c, seed, 0x11e8);
                oc1048_speech(c, &mut r, seed);
                c.g[A0][0] = sx(OC1048_SELF);
                c.g[A1][0] = sx(seed % 4);
            },
        );
    }

    #[test]
    fn music_player_stop_stream_matches() {
        check(
            ee::s08::f_002b1a70__generated,
            music_player_stop_stream,
            200,
            &|c, seed| {
                oc1048_machine(c, seed, 0x1a70);
                // gMusicSys -> stream array -> slot table with stream 2 empty.
                c.bus.write32(GAME_GP + GP_MUSIC_SYS, OC1048_MUSIC);
                c.bus.write32(OC1048_MUSIC, OC1048_STREAMS);
                c.bus.write32(OC1048_STREAMS + 8, OC1048_SLOTS);
                c.bus.write32(OC1048_SLOTS + 8, 0);
            },
        );
    }

    #[test]
    fn song_instance_path_cmd_b_matches() {
        check(
            ee::s08::f_002b2140__generated,
            song_instance_path_cmd_b,
            200,
            &|c, seed| {
                oc1048_machine(c, seed, 0x2140);
            },
        );
    }

    #[test]
    fn song_instance_pick_random_phrase_offset_matches() {
        check(
            ee::s08::f_002b2500__generated,
            song_instance_pick_random_phrase_offset,
            300,
            &|c, seed| {
                let mut r = oc1048_machine(c, seed, 0x2500);
                // self -> config (phrases per bank at +0x6c, align at +0x74).
                c.bus.write32(OC1048_SONG, OC1048_CONFIG);
                c.bus.write32(OC1048_CONFIG + 0x6c, 2 + seed % 7);
                c.bus.write32(OC1048_CONFIG + 0x74, r.next_u32());
                c.bus.write32(GAME_GP + GP_SOUND_RAND, r.next_u32());
                c.g[A0][0] = sx(OC1048_SONG);
            },
        );
    }

    #[test]
    fn song_instance_get_path_status_field_matches() {
        check(
            ee::s08::f_002b2550__generated,
            song_instance_get_path_status_field,
            200,
            &|c, seed| {
                oc1048_machine(c, seed, 0x2550);
            },
        );
    }
}
