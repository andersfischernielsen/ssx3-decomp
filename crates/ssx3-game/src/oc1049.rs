//! Staging module oc1049 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1049`.

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
        0x002b_2578,
        song_instance_get_stream_status,
        "SongInstance_GetStreamStatus",
    ),
    (
        0x002b_2640,
        song_instance_get_time_remaining,
        "SongInstance_GetTimeRemaining",
    ),
    (
        0x002b_26b8,
        song_instance_get_phrase_ms,
        "SongInstance_GetPhraseMs",
    ),
    (
        0x002b_26f8,
        song_instance_get_time_remaining_wrapper,
        "SongInstance_GetTimeRemaining_Wrapper",
    ),
    (
        0x002b_2778,
        song_instance_get_beat_index,
        "SongInstance_GetBeatIndex",
    ),
    (
        0x002b_27c0,
        song_instance_is_loop_bank_busy,
        "SongInstance_IsLoopBankBusy",
    ),
    (0x002b_3c60, music_player_path_cmd_b, "MusicPlayer_PathCmdB"),
    (
        0x002b_3cd8,
        music_player_apply_song_volume,
        "MusicPlayer_ApplySongVolume",
    ),
];

/// `int64 SongInstance_GetStreamStatus(SongInstance* self, void* out)`
/// (0x2b2578): -1 when the song has no stream handle; else the stream's
/// status, from `SNDSTRM_GetStatus(handle, out)`.
pub fn song_instance_get_stream_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Callee {
            addr: 0x002b_2488,
            f: ee::s08::f_002b2488,
        }
        .call(c, 0x002b_2594)?;
        let no_handle = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = u64::MAX;
        if !no_handle {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x002b_2488,
                f: ee::s08::f_002b2488,
            }
            .call(c, 0x002b_25a4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x003b_7b48, 0x002b_25b0)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `s64 SongInstance_GetTimeRemaining(SongInstance* self)` (0x2b2640): the
/// request status's position word (`out + 8`) when the status is 2
/// (playing), else the phrase length from `SongInstance_GetPhraseMs`.
pub fn song_instance_get_time_remaining(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002b_25c8,
            f: ee::s08::f_002b25c8,
        }
        .call(c, 0x002b_2658)?;
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[V0][0] = 2;
        let playing = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = Addr(sp(c)).off(0x8).lw(c);
        if playing {
            c.cycles += 3;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x002b_26b8,
                f: ee::s08::f_002b26b8,
            }
            .call(c, 0x002b_2670)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `float SongInstance_GetPhraseMs(SongInstance* self)` (0x2b26b8): the beat
/// length (`SongInstance_GetBeatMs`) times the song's two beat-count fields
/// (`+0x64`, `+0x68`, as floats).
pub fn song_instance_get_phrase_ms(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002b_2690,
            f: ee::s08::f_002b2690,
        }
        .call(c, 0x002b_26cc)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        frame.restore(c, S0, 0x10);
        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x64));
        c.cvt_s_w(1, 1);
        frame.restore_ra(c, 0);
        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x68));
        c.cvt_s_w(2, 2);
        c.mul_s(0, 0, 1);
        c.mul_s(0, 0, 2);
        frame.close(c);
        Ok(())
    })
}

/// `s64 SongInstance_GetTimeRemaining_Wrapper(SongInstance* self)`
/// (0x2b26f8): a forwarding wrapper for `SongInstance_GetTimeRemaining`.
pub fn song_instance_get_time_remaining_wrapper(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x002b_2640,
            f: ee::s08::f_002b2640,
        }),
        0x002b_2708,
    );
}

/// `int SongInstance_GetBeatIndex(SongInstance* self)` (0x2b2778): the
/// playback position divided by the beat length, truncated to an int.
pub fn song_instance_get_beat_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x20);
        Callee {
            addr: 0x002b_2600,
            f: ee::s08::f_002b2600,
        }
        .call(c, 0x002b_2790)?;
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(20, 0);
        Callee {
            addr: 0x002b_2690,
            f: ee::s08::f_002b2690,
        }
        .call(c, 0x002b_279c)?;
        c.div_s(20, 20, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.cvt_w_s(0, 20);
        c.mfc1(2, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `bool SongInstance_IsLoopBankBusy(SongInstance* self, MusicPlayer* mp)`
/// (0x2b27c0): true while the song's loops are active (`+0x4c`); else, when
/// the bank entry of the song's bank (`mp+0xacc`, 0x60 bytes each) is
/// marked, whether its bank is in use. The sound lock is held around the
/// bank query.
pub fn song_instance_is_loop_bank_busy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x4c).lw(c);
        let loops_active = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        let busy = if loops_active {
            true
        } else {
            c.cycles += 11;
            c.g[S0][0] = Addr(c.g[A0][0] as u32).off(0x34).lw(c);
            c.g[S2][0] = 0;
            call_ptr(c, 0x003b_58a0, 0x002b_27ec)?;
            c.g[V1][0] = 0x60;
            c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0xacc).lw(c);
            c.mult(0, V1, S0, V1);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[V0][0] ^= 1;
            let entry_not_marked = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if !entry_not_marked {
                c.cycles += 6;
                c.g[A1][0] = c.g[S0][0];
                Callee {
                    addr: 0x002b_9f88,
                    f: ee::s08::f_002b9f88,
                }
                .call(c, 0x002b_2814)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x003b_65d0, 0x002b_281c)?;
                c.g[V0][0] ^= 1;
                c.g[S2][0] = (c.g[V0][0] < 0x1) as u64;
            }
            c.cycles += 4;
            call_ptr(c, 0x003b_58d8, 0x002b_282c)?;
            c.g[V0][0] = 0;
            c.g[S2][0] != 0
        };
        if busy {
            c.cycles += 1;
            c.g[V0][0] = 1;
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

/// `void MusicPlayer_PathCmdB(MusicPlayer* mp)` (0x2b3c60): when the player
/// has a current song (`+0x418`) with a song object (`+0x408`), forward to
/// `SongInstance_PathCmdB` on that song.
pub fn music_player_path_cmd_b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x418).lw(c);
        let no_song = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !no_song {
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x408).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                Callee {
                    addr: 0x002b_2140,
                    f: ee::s08::f_002b2140,
                }
                .call(c, 0x002b_3c88)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_ApplySongVolume(MusicPlayer* mp, int volume)`
/// (0x2b3cd8): when the player has a current song (`+0x418`) with a song
/// object (`+0x408`), set that song's volume through `SongInstance_SetVolume`.
pub fn music_player_apply_song_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x418).lw(c);
        let no_song = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !no_song {
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x408).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                Callee {
                    addr: 0x002b_2160,
                    f: ee::s08::f_002b2160,
                }
                .call(c, 0x002b_3d00)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::{fill, DATA};

    /// A song instance at `DATA + 0x200` whose first word points at its
    /// playback block (`DATA + 0x300`): a beat rate (`+0x90`, a non-zero
    /// float) and the two beat-count fields (`+0x64`, `+0x68`).
    pub(crate) fn oc1049_song_setup(c: &mut Cpu, seed: u32) {
        let song = DATA + 0x200;
        let block = DATA + 0x300;
        c.g[A0][0] = sx(song);
        c.bus.write32(song, block);
        c.bus.write32(block + 0x64, 0x1000 + 0x37 * seed);
        c.bus.write32(block + 0x68, 1 + seed % 5);
        c.bus
            .write32(block + 0x90, (0.5 + (seed % 7) as f32).to_bits());
    }

    /// A music player at `DATA + 0x200`: a current song (`+0x418`) that is
    /// sometimes absent, and a song object (`+0x408`) that is sometimes null.
    pub(crate) fn oc1049_player_setup(c: &mut Cpu, seed: u32) {
        let mp = DATA + 0x200;
        let song = DATA + 0x300;
        c.g[A0][0] = sx(mp);
        c.bus.write32(mp + 0x418, if seed % 3 != 0 { 1 } else { 0 });
        c.bus
            .write32(mp + 0x408, if seed % 2 == 0 { song } else { 0 });
    }

    #[test]
    fn song_instance_get_stream_status_matches() {
        check(
            ee::s08::f_002b2578__generated,
            song_instance_get_stream_status,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_2578, 0);
                oc1049_song_setup(c, seed);
                c.g[A1][0] = sx(DATA + 0x400);
                fill(c, &mut r, DATA + 0x400, 8);
            },
        );
    }

    #[test]
    fn song_instance_get_time_remaining_matches() {
        check(
            ee::s08::f_002b2640__generated,
            song_instance_get_time_remaining,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x2b_2640, 0);
                oc1049_song_setup(c, seed);
            },
        );
    }

    #[test]
    fn song_instance_get_phrase_ms_matches() {
        check(
            ee::s08::f_002b26b8__generated,
            song_instance_get_phrase_ms,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x2b_26b8, 0);
                oc1049_song_setup(c, seed);
            },
        );
    }

    #[test]
    fn song_instance_get_time_remaining_wrapper_matches() {
        check(
            ee::s08::f_002b26f8__generated,
            song_instance_get_time_remaining_wrapper,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x2b_26f8, 0);
                oc1049_song_setup(c, seed);
            },
        );
    }

    #[test]
    fn song_instance_get_beat_index_matches() {
        check(
            ee::s08::f_002b2778__generated,
            song_instance_get_beat_index,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x2b_2778, 0);
                oc1049_song_setup(c, seed);
            },
        );
    }

    #[test]
    fn song_instance_is_loop_bank_busy_matches() {
        check(
            ee::s08::f_002b27c0__generated,
            song_instance_is_loop_bank_busy,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x2b_27c0, 0);
                let song = DATA + 0x200;
                let mgr = DATA + 0x400;
                let table = DATA + 0x800;
                c.g[A0][0] = sx(song);
                c.g[A1][0] = sx(mgr);
                // Loops active (returns 1) on a quarter of the seeds.
                c.bus
                    .write32(song + 0x4c, if seed % 4 == 0 { 1 } else { 0 });
                let bank = seed % 3;
                c.bus.write32(song + 0x34, bank);
                c.bus.write32(mgr + 0xacc, table);
                // The bank entry: marked (1) on half of the seeds.
                c.bus
                    .write32(table + 0x60 * bank, if seed % 2 == 0 { 1 } else { 0 });
            },
        );
    }

    #[test]
    fn music_player_path_cmd_b_matches() {
        check(
            ee::s08::f_002b3c60__generated,
            music_player_path_cmd_b,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2b_3c60, 0);
                oc1049_player_setup(c, seed);
            },
        );
    }

    #[test]
    fn music_player_apply_song_volume_matches() {
        check(
            ee::s08::f_002b3cd8__generated,
            music_player_apply_song_volume,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2b_3cd8, 0);
                oc1049_player_setup(c, seed);
                c.g[A1][0] = sx(seed % 128);
            },
        );
    }
}
