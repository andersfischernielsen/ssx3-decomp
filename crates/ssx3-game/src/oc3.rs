//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Small leaf functions from the audio/music bank monitor, the track/path
//! table, the world-trigger instances and the world camera. Each replacement
//! copies the generated body's logic, cycle charging and observable register
//! (and FPU condition) results exactly.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002a_d970, bank_monitor_timer_set_period, "BankMonitor_TimerSetPeriod"),
    (0x002a_dc10, bank_monitor_tick, "BankMonitor_Tick"),
    (0x002a_dfb0, path_get_track_data, "PATH_GetTrackData"),
    (0x002b_3c98, music_player_set_song_volume, "MusicPlayer_SetSongVolume"),
    (0x002b_40f0, music_player_get_song_title, "MusicPlayer_GetSongTitle"),
    (0x002b_4120, music_player_get_song_artist, "MusicPlayer_GetSongArtist"),
    (0x002b_4150, music_player_get_song_album, "MusicPlayer_GetSongAlbum"),
    (0x002b_4868, song_instance_get_duck_to_loops, "SongInstance_GetDuckToLoops"),
    (0x002b_5898, world_trigger_instance_reset, "WorldTriggerInstance_Reset"),
    (0x002d_1c20, world_get_camera_eye, "World_GetCameraEye"),
];

// ---------------------------------------------------------------------------
// Bank monitor

/// `void BankMonitor_TimerSetPeriod(BankMonitorTimer* t, int period)`
/// (0x2ad970): store the period at +4 and +8 of the timer.
pub fn bank_monitor_timer_set_period(c: &mut Cpu) {
    c.cycles += 3;
    let timer = Addr(abi::arg(c, 0));
    let period = c.g[A1][0];
    timer.off(0x8).sw(c, period);
    timer.off(0x4).sw(c, period);
    abi::ret(c);
}

/// `void BankMonitor_Tick(BankMonitor* m)` (0x2adc10): if the timer at +0x94
/// is inactive, advance the counter at +0x88.
pub fn bank_monitor_tick(c: &mut Cpu) {
    c.cycles += 3;
    let bank = Addr(abi::arg(c, 0));
    let active = bank.off(0x94).lw(c);
    c.g[V0][0] = active;
    if active == 0 {
        c.cycles += 3;
        let count = (bank.off(0x88).lw(c) as u32).wrapping_add(1);
        c.g[V0][0] = sx(count);
        bank.off(0x88).sw(c, sx(count));
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Path / track table

/// The base of the track-data pointer table.
const TRACK_TABLE: u32 = 0x004d_3e98;

/// `TrackData* PATH_GetTrackData(int index)` (0x2adfb0): fetch the pointer
/// from `TRACK_TABLE[index]`.
pub fn path_get_track_data(c: &mut Cpu) {
    c.cycles += 6;
    let index = abi::arg(c, 0) as u32;
    let slot = TRACK_TABLE.wrapping_add(index.wrapping_shl(2));
    c.g[A0][0] = sx(slot);
    c.g[V0][0] = Addr(slot).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Music player

/// `void MusicPlayer_SetSongVolume(MusicPlayer* mp, s8 volume)` (0x2b3c98):
/// store the byte volume into the current song instance at +0x40.
pub fn music_player_set_song_volume(c: &mut Cpu) {
    c.cycles += 4;
    let mp = Addr(abi::arg(c, 0));
    let volume = sx(abi::arg(c, 1) as u8 as i8 as u32);
    let song = mp.off(0x418).lw(c);
    c.g[A1][0] = volume;
    c.g[V0][0] = song;
    if song != 0 {
        c.cycles += 3;
        let instance = mp.off(0x408).lw(c);
        c.g[A0][0] = instance;
        if instance != 0 {
            Addr(instance as u32).off(0x40).sb(c, volume);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// The word holding the currently selected song index.
const CUR_SONG: u32 = 0x3f0;

/// Shared body of the `MusicPlayer_GetSong*` accessors: resolve the song
/// index (negative argument means "current song", a negative current song
/// returns null) and return `this + table + index*100`.
fn music_player_get_song_part(c: &mut Cpu, table: u32) {
    c.cycles += 2;
    let arg = abi::arg64(c, 1);
    c.g[V0][0] = 0x64;
    if (arg as i64) >= 0 {
        c.cycles += 1;
        c.mult(0, V0, A1, V0);
    } else {
        c.cycles += 3;
        let current = Addr(abi::arg(c, 0)).off(CUR_SONG).lw(c);
        c.g[A1][0] = current;
        c.mult(0, V0, A1, V0);
        if (current as i64) < 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(table));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `char* MusicPlayer_GetSongTitle(MusicPlayer* mp, int song)` (0x2b40f0).
pub fn music_player_get_song_title(c: &mut Cpu) {
    music_player_get_song_part(c, 0x934);
}

/// `char* MusicPlayer_GetSongArtist(MusicPlayer* mp, int song)` (0x2b4120).
pub fn music_player_get_song_artist(c: &mut Cpu) {
    music_player_get_song_part(c, 0x2234);
}

/// `char* MusicPlayer_GetSongAlbum(MusicPlayer* mp, int song)` (0x2b4150).
pub fn music_player_get_song_album(c: &mut Cpu) {
    music_player_get_song_part(c, 0x3b34);
}

// ---------------------------------------------------------------------------
// Song / world trigger instances

/// `int SongInstance_GetDuckToLoops(SongInstance* s)` (0x2b4868): read the
/// loop count at +0x94 of the owning song.
pub fn song_instance_get_duck_to_loops(c: &mut Cpu) {
    c.cycles += 3;
    let owner = Addr(abi::arg(c, 0)).lw(c);
    c.g[V1][0] = owner;
    c.g[V0][0] = Addr(owner as u32).off(0x94).lw(c);
    abi::ret(c);
}

/// `void WorldTriggerInstance_Reset(WorldTriggerInstance* t)` (0x2b5898):
/// clear the instance, set its state to -1, its timer field to -1.0 and arm
/// it from the global trigger flag.
pub fn world_trigger_instance_reset(c: &mut Cpu) {
    c.cycles += 14;
    let inst = Addr(abi::arg(c, 0));
    inst.sw(c, 0);
    inst.off(0x30).sw(c, u64::MAX);
    const GLOBAL_FLAG: u32 = 0x004a_35a8;
    let armed = Addr(GLOBAL_FLAG).lbu(c);
    inst.off(0x34).sw(c, 0);
    c.g[V1][0] = 0x004a_0000;
    c.f[0] = 0xbf80_0000; // -1.0f, as `lui 0xbf80` + `mtc1 $at, $f0`
    inst.off(0x68).write_fpu(c, FpuF32(0xbf80_0000));
    inst.off(0x40).sb(c, armed);
    c.g[V0][0] = armed;
    inst.off(0x6c).sw(c, 0);
    inst.off(0x60).sw(c, 0);
    inst.off(0x64).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World camera

/// `Vec3* World_GetCameraEye(World* w, int view)` (0x2d1c20): return
/// `view[view].eye + 0x20`.
pub fn world_get_camera_eye(c: &mut Cpu) {
    c.cycles += 8;
    let view = abi::arg(c, 0) as u32;
    let world = Addr(c.g[GP][0] as u32 - 0x848).lw(c);
    let viewer = Addr(world as u32).off(0x84).lw(c);
    let table = Addr(viewer as u32).off(0x84).lw(c);
    let entry = (table as u32).wrapping_add(view.wrapping_shl(2));
    c.g[A1][0] = world;
    c.g[A0][0] = sx(view.wrapping_shl(2));
    c.g[V1][0] = sx(entry);
    let eye = Addr(entry).off(0x4).lw(c);
    c.g[V0][0] = sx((eye as u32).wrapping_add(0x20));
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn bank_monitor_timer_set_period_matches() {
        check(ssx3_ee::s08::f_002ad970__generated, bank_monitor_timer_set_period, 200, &|c, seed| {
            let mut r = Rng(0x1111 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
        });
    }

    #[test]
    fn bank_monitor_tick_matches() {
        check(ssx3_ee::s08::f_002adc10__generated, bank_monitor_tick, 200, &|c, seed| {
            let mut r = Rng(0x2222 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let active = if seed % 3 == 0 { 0 } else { r.next_u32() };
            c.bus.write32(DATA + 0x94, active);
            c.bus.write32(DATA + 0x88, r.next_u32());
        });
    }

    #[test]
    fn path_get_track_data_matches() {
        check(ssx3_ee::s08::f_002adfb0__generated, path_get_track_data, 200, &|c, seed| {
            let mut r = Rng(0x3333 ^ seed as u64);
            scramble(c, &mut r);
            for k in 0..8u32 {
                c.bus.write32(TRACK_TABLE + 4 * k, r.next_u32());
            }
            c.g[A0][0] = (r.next_u32() % 8) as u64;
        });
    }

    #[test]
    fn music_player_set_song_volume_matches() {
        check(ssx3_ee::s08::f_002b3c98__generated, music_player_set_song_volume, 200, &|c, seed| {
            let mut r = Rng(0x4444 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
            c.bus.write32(DATA + 0x418, if seed % 4 == 0 { 0 } else { DATA + 0x300 });
            c.bus.write32(DATA + 0x408, if seed % 5 == 0 { 0 } else { DATA + 0x400 });
        });
    }

    /// Shared setup for the three `MusicPlayer_GetSong*` accessors: cover the
    /// direct-index, current-song and null paths.
    fn song_getter_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x5555 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        match seed % 3 {
            0 => {
                c.g[A1][0] = (r.next_u32() % 10) as u64;
                c.bus.write32(DATA + CUR_SONG, r.next_u32());
            }
            _ => {
                c.g[A1][0] = 0xffff_ffff_ffff_ffff; // negative -> use current song
                let current = if seed % 2 == 0 {
                    (r.next_u32() % 10) as u32
                } else {
                    r.next_u32() | 0x8000_0000
                };
                c.bus.write32(DATA + CUR_SONG, current);
            }
        }
    }

    #[test]
    fn music_player_get_song_title_matches() {
        check_lohi(ssx3_ee::s08::f_002b40f0__generated, music_player_get_song_title, 200, &song_getter_setup);
    }

    #[test]
    fn music_player_get_song_artist_matches() {
        check_lohi(ssx3_ee::s08::f_002b4120__generated, music_player_get_song_artist, 200, &song_getter_setup);
    }

    #[test]
    fn music_player_get_song_album_matches() {
        check_lohi(ssx3_ee::s08::f_002b4150__generated, music_player_get_song_album, 200, &song_getter_setup);
    }

    #[test]
    fn song_instance_get_duck_to_loops_matches() {
        check(ssx3_ee::s08::f_002b4868__generated, song_instance_get_duck_to_loops, 200, &|c, seed| {
            let mut r = Rng(0x6666 ^ seed as u64);
            scramble(c, &mut r);
            let owner = DATA + 0x100;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, owner);
            c.bus.write32(owner + 0x94, r.next_u32());
        });
    }

    #[test]
    fn world_trigger_instance_reset_matches() {
        check(ssx3_ee::s08::f_002b5898__generated, world_trigger_instance_reset, 200, &|c, seed| {
            let mut r = Rng(0x7777 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write8(0x004a_35a8, (r.next_u32() & 0xff) as u8);
        });
    }

    #[test]
    fn world_get_camera_eye_matches() {
        check(ssx3_ee::s09::f_002d1c20__generated, world_get_camera_eye, 200, &|c, seed| {
            let mut r = Rng(0x8888 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = DATA + 0x100;
            let viewer = DATA + 0x200;
            let table = DATA + 0x400;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, viewer);
            c.bus.write32(viewer + 0x84, table);
            for k in 0..16u32 {
                c.bus.write32(table + 4 * k, r.next_u32());
            }
            c.g[A0][0] = (r.next_u32() % 8) as u64;
        });
    }
}
