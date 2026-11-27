//! Staging module oc1018 (wave wave261): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1018`.

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
        0x0028_79b0,
        ssx_audio_system_inc_channel_volume,
        "SSXAudioSystem_IncChannelVolume",
    ),
    (
        0x0028_79e0,
        ssx_audio_system_dec_channel_volume,
        "SSXAudioSystem_DecChannelVolume",
    ),
    (
        0x0028_9398,
        ssx_audio_system_prev_debug_mode,
        "SSXAudioSystem_PrevDebugMode",
    ),
    (
        0x0028_93c8,
        ssx_audio_system_next_debug_mode,
        "SSXAudioSystem_NextDebugMode",
    ),
    (
        0x0028_9400,
        ssx_audio_system_schedule_trigger_timer,
        "SSXAudioSystem_ScheduleTriggerTimer",
    ),
    (
        0x0028_9448,
        ssx_audio_system_trigger_timer_callback,
        "SSXAudioSystem_TriggerTimerCallback",
    ),
    (
        0x0028_9680,
        ssx_audio_system_empty_crowd_handler,
        "SSXAudioSystem_EmptyCrowdHandler",
    ),
    (
        0x0028_99f8,
        ssx_audio_system_apply_music_trigger_paint,
        "SSXAudioSystem_ApplyMusicTriggerPaint",
    ),
];

/// `void SSXAudioSystem_IncChannelVolume(SSXAudioSystem* self, int ch)` (0x2879b0):
/// raise channel `ch`'s volume (record at +0x62bc, 16 bytes per channel) by
/// the 0.0909 step at gp-0x4628 and re-apply it through
/// `SSXAudioSystem_SetChannelVolume`.
pub fn ssx_audio_system_inc_channel_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 4);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let step = Addr((c.g[GP][0] as u32).wrapping_sub(0x4628)).read_fpu(c);
        c.f[0] = step.0;
        let volume = Addr((c.g[V0][0] as u32).wrapping_add(0x62bc)).read_fpu(c);
        c.f[12] = (volume + step).0;
        Callee {
            addr: 0x0028_7700,
            f: ee::s07::SSXAudioSystem_SetChannelVolume,
        }
        .call(c, 0x0028_79d0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_DecChannelVolume(SSXAudioSystem* self, int ch)` (0x2879e0):
/// the counterpart of [`ssx_audio_system_inc_channel_volume`], lowering the
/// volume by the step at gp-0x4624.
pub fn ssx_audio_system_dec_channel_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 4);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let step = Addr((c.g[GP][0] as u32).wrapping_sub(0x4624)).read_fpu(c);
        c.f[0] = step.0;
        let volume = Addr((c.g[V0][0] as u32).wrapping_add(0x62bc)).read_fpu(c);
        c.f[12] = (volume - step).0;
        Callee {
            addr: 0x0028_7700,
            f: ee::s07::SSXAudioSystem_SetChannelVolume,
        }
        .call(c, 0x0028_7a00)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_PrevDebugMode(SSXAudioSystem* self)` (0x289398): step the
/// debug preset (+0x6470) down, wrapping from 0 to 5, and apply it through
/// `SSXAudioSystem_SetDebugMode`.
pub fn ssx_audio_system_prev_debug_mode(c: &mut Cpu) {
    guest(c, |c| {
        let obj = c.g[A0][0] as u32;
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(obj.wrapping_add(0x6470)).lw(c);
        let stepped = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if !stepped {
            c.cycles += 1;
            c.g[V0][0] = 5;
        }
        c.cycles += 6;
        let preset = c.g[V0][0];
        Addr(obj.wrapping_add(0x6470)).sw(c, preset);
        c.g[RA][0] = sx(0x0028_93bc);
        c.g[A1][0] = Addr(obj.wrapping_add(0x6470)).lw(c);
        Callee {
            addr: 0x0028_8f60,
            f: ee::s07::SSXAudioSystem_SetDebugMode,
        }
        .call(c, 0x0028_93bc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_NextDebugMode(SSXAudioSystem* self)` (0x2893c8): step the
/// debug preset (+0x6470) up, wrapping from 6 to 0, and apply it through
/// `SSXAudioSystem_SetDebugMode`.
pub fn ssx_audio_system_next_debug_mode(c: &mut Cpu) {
    guest(c, |c| {
        let obj = c.g[A0][0] as u32;
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 6;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(obj.wrapping_add(0x6470)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let more = c.g[V0][0] != c.g[V1][0];
        let stepped = c.g[V0][0];
        Addr(obj.wrapping_add(0x6470)).sw(c, stepped);
        if !more {
            c.cycles += 1;
            Addr(obj.wrapping_add(0x6470)).sw(c, 0u64);
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0028_93f0);
        c.g[A1][0] = Addr(obj.wrapping_add(0x6470)).lw(c);
        Callee {
            addr: 0x0028_8f60,
            f: ee::s07::SSXAudioSystem_SetDebugMode,
        }
        .call(c, 0x0028_93f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_ScheduleTriggerTimer(SSXAudioSystem* self)` (0x289400):
/// arm the 250-tick trigger timer on the bank monitor (`self->+0x118->*` +0x1d8)
/// with the callback `SSXAudioSystem_TriggerTimerCallback` (the 8-byte pointer at
/// 0x4a3600 in the game image).
pub fn ssx_audio_system_schedule_trigger_timer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x004a_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xfa;
        c.ldl(A2, (c.g[V1][0] as u32).wrapping_add(0x3607));
        c.ldr(A2, (c.g[V1][0] as u32).wrapping_add(0x3600));
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x118)).lw(c);
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[T3][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        call_ptr(c, 0x002a_dca0, 0x0028_943c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_TriggerTimerCallback(void)` (0x289448): the timer's
/// callback, `WorldTriggerManager` monitor pass (gp+0x21e4, one, no extra) and
/// result 1.
pub fn ssx_audio_system_trigger_timer_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x21e4)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        call_ptr(c, 0x002b_6950, 0x0028_9460)?;
        c.g[V0][0] = 1;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_EmptyCrowdHandler(void)` (0x289680): an empty
/// callback; it only returns.
pub fn ssx_audio_system_empty_crowd_handler(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void SSXAudioSystem_ApplyMusicTriggerPaint(SSXAudioSystem* self, Rider* rider, Obj* obj)`
/// (0x2899f8). When `FindRider(self, -1)` is `rider` and `obj` is set, `obj`'s
/// vtable method (`+4 -> +0x24`) gives a type code: 0x1f and 0x20 choose a
/// song through `MusicPlayer_GetSongTag` and `SSXAudioSystem_PlayMusic`
/// (0x65 / 0x66 end without playing), 0x21 ends, other codes call the
/// method at `self + 0x5558`'s vtable.
pub fn ssx_audio_system_apply_music_trigger_paint(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        'b_289ad8: {
            c.g[A1][0] = u64::MAX;
            Callee {
                addr: 0x0028_5d98,
                f: ee::s07::SSXAudioSystem_FindRider,
            }
            .call(c, 0x0028_9a20)?;
            if c.g[S0][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x30);
                break 'b_289ad8;
            }
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'b_289ad8;
            }
            'b_289ad4: {
                c.cycles += 9;
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0028_9a44)?;
                c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = u64::MAX;
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 0x1f;
                if t {
                    break 'b_289ad4;
                }
                'b_289ab8: {
                    'b_289aa8: {
                        'b_289a80: {
                            c.cycles += 2;
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x20;
                            if t {
                                break 'b_289a80;
                            }
                            c.cycles += 5;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
                            call_ptr(c, 0x002b_49e0, 0x0028_9a64)?;
                            c.g[V1][0] = 0x65;
                            let t = c.g[V0][0] == c.g[V1][0];
                            c.g[A1][0] = 0;
                            if t {
                                break 'b_289ad4;
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = 0;
                            c.g[A3][0] = 0x1;
                            break 'b_289aa8;
                        }
                        c.cycles += 2;
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x21;
                        if t {
                            break 'b_289ab8;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
                        call_ptr(c, 0x002b_49e0, 0x0028_9a90)?;
                        c.g[V1][0] = 0x66;
                        let t = c.g[V0][0] == c.g[V1][0];
                        c.g[A1][0] = 0;
                        if t {
                            break 'b_289ad4;
                        }
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0x2;
                        break 'b_289aa8;
                    }
                    c.cycles += 4;
                    c.g[T0][0] = 0;
                    Callee {
                        addr: 0x0028_cf98,
                        f: ee::s07::SSXAudioSystem_PlayMusic,
                    }
                    .call(c, 0x0028_9ab0)?;
                    frame.restore(c, S0, 0x30);
                    break 'b_289ad8;
                }
                c.cycles += 2;
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
                if t {
                    break 'b_289ad4;
                }
                c.cycles += 5;
                c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x5558)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0028_9ad4)?;
                break 'b_289ad4;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            break 'b_289ad8;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, fakes, scramble, Rng, DATA, GAME_GP};
    use crate::game::sound::ssxaudio::tests::set_channel_volume_setup;
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes, in the test machine's RAM.
    pub(crate) const OC1018_CTRL: u32 = DATA + 0x28_0000;

    pub(crate) const OC1018_MAGIC: u32 = 0x1018_c0de;

    /// Result of the fake `FindRider` (a rider pointer).
    pub(crate) const OC1018_RIDER: u32 = OC1018_CTRL + 0x10;

    /// Result of the fake `MusicPlayer_GetSongTag`.
    pub(crate) const OC1018_TAG: u32 = OC1018_CTRL + 0x14;

    /// The fake `FindRider`: logs the call, returns the rider word when gated on.
    pub(crate) fn oc1018_find_rider(c: &mut Cpu) {
        if c.bus.read32(OC1018_CTRL) != OC1018_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1018_find_rider),
                ee::s07::f_00285d98__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let rider = c.bus.read32(OC1018_RIDER);
        fakes::fake_call(c, 0x0028_5d98);
        if c.pc == ra {
            c.g[V0][0] = sx(rider);
        }
    }

    /// The fake `MusicPlayer_GetSongTag`: logs, returns the tag word when gated on.
    pub(crate) fn oc1018_get_song_tag(c: &mut Cpu) {
        if c.bus.read32(OC1018_CTRL) != OC1018_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1018_get_song_tag),
                ee::s08::f_002b49e0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let tag = c.bus.read32(OC1018_TAG);
        fakes::fake_call(c, 0x002b_49e0);
        if c.pc == ra {
            c.g[V0][0] = sx(tag);
        }
    }

    /// The fake `SSXAudioSystem_PlayMusic`: logs the call only.
    pub(crate) fn oc1018_play_music(c: &mut Cpu) {
        if c.bus.read32(OC1018_CTRL) != OC1018_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1018_play_music),
                ee::s07::f_0028cf98__generated,
            );
        }
        fakes::fake_call(c, 0x0028_cf98);
    }

    /// The fake bank-monitor `AddTimer`: logs the call only.
    pub(crate) fn oc1018_add_timer(c: &mut Cpu) {
        if c.bus.read32(OC1018_CTRL) != OC1018_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1018_add_timer),
                ee::s08::f_002adca0__generated,
            );
        }
        fakes::fake_call(c, 0x002a_dca0);
    }

    /// The fake world-trigger monitor pass: logs the call only.
    pub(crate) fn oc1018_monitor_triggers(c: &mut Cpu) {
        if c.bus.read32(OC1018_CTRL) != OC1018_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1018_monitor_triggers),
                ee::s08::f_002b6950__generated,
            );
        }
        fakes::fake_call(c, 0x002b_6950);
    }

    /// Hooks the fakes into their override slots (once per process).
    pub(crate) fn oc1018_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00285D98),
            oc1018_find_rider,
            fakes::prev_cell(oc1018_find_rider),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B49E0),
            oc1018_get_song_tag,
            fakes::prev_cell(oc1018_get_song_tag),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028CF98),
            oc1018_play_music,
            fakes::prev_cell(oc1018_play_music),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADCA0),
            oc1018_add_timer,
            fakes::prev_cell(oc1018_add_timer),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B6950),
            oc1018_monitor_triggers,
            fakes::prev_cell(oc1018_monitor_triggers),
        );
    }

    /// A scrambled machine with this module's fakes armed, `a0` at the object
    /// `DATA + 0x1000` and `pc` at `entry`.
    pub(crate) fn oc1018_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc1018_install_fakes();
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC1018_CTRL, OC1018_MAGIC);
        c.g[A0][0] = u64::from(DATA + 0x1000);
        c.pc = entry;
        r
    }

    /// The object's debug preset set to a value that covers the wrap cases.
    fn oc1018_preset_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        oc1018_machine(c, seed, salt, 12, entry);
        elf_image(c);
        let preset = match seed % 9 {
            8 => 100,
            m => m,
        };
        c.bus.write32(DATA + 0x1000 + 0x6470, preset);
    }

    /// The channel-volume fixture of `SSXAudioSystem_SetChannelVolume`, the step
    /// constants from the game image, and the entry `pc`.
    fn oc1018_channel_setup(c: &mut Cpu, seed: u32, entry: u32) {
        set_channel_volume_setup(c, seed);
        elf_image(c);
        c.pc = entry;
    }

    /// `SSXAudioSystem_IncChannelVolume`: every channel and the out-of-range ones.
    #[test]
    fn ssx_audio_system_inc_channel_volume_matches() {
        check(
            ee::s07::f_002879b0__generated,
            ssx_audio_system_inc_channel_volume,
            280,
            &|c, seed| oc1018_channel_setup(c, seed, 0x0028_79b0),
        );
    }

    /// `SSXAudioSystem_DecChannelVolume`: every channel and the out-of-range ones.
    #[test]
    fn ssx_audio_system_dec_channel_volume_matches() {
        check(
            ee::s07::f_002879e0__generated,
            ssx_audio_system_dec_channel_volume,
            280,
            &|c, seed| oc1018_channel_setup(c, seed, 0x0028_79e0),
        );
    }

    /// Presets 0..=8 (wrap from 0, then the store and the apply) and 100.
    #[test]
    fn ssx_audio_system_prev_debug_mode_matches() {
        check(
            ee::s07::f_00289398__generated,
            ssx_audio_system_prev_debug_mode,
            240,
            &|c, seed| oc1018_preset_setup(c, seed, 0x9398, 0x0028_9398),
        );
    }

    /// Presets 0..=8 (wrap from 6) and 100.
    #[test]
    fn ssx_audio_system_next_debug_mode_matches() {
        check(
            ee::s07::f_002893c8__generated,
            ssx_audio_system_next_debug_mode,
            240,
            &|c, seed| oc1018_preset_setup(c, seed, 0x93c8, 0x0028_93c8),
        );
    }

    /// The timer arms with the callback pointer from the game image; the bank
    /// monitor is faked (logs its arguments).
    #[test]
    fn ssx_audio_system_schedule_trigger_timer_matches() {
        check(
            ee::s07::f_00289400__generated,
            ssx_audio_system_schedule_trigger_timer,
            200,
            &|c, seed| {
                oc1018_machine(c, seed, 0x9400, 2, 0x0028_9400);
                elf_image(c);
                c.bus.write32(DATA + 0x1000 + 0x118, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, DATA + 0x3000);
            },
        );
    }

    /// The callback calls the world-trigger monitor (faked) and returns 1.
    #[test]
    fn ssx_audio_system_trigger_timer_callback_matches() {
        check(
            ee::s07::f_00289448__generated,
            ssx_audio_system_trigger_timer_callback,
            200,
            &|c, seed| {
                oc1018_machine(c, seed, 0x9448, 2, 0x0028_9448);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x21e4),
                    DATA + 0x9000 + 0x40 * (seed % 4),
                );
            },
        );
    }

    /// The empty callback only returns to `ra`.
    #[test]
    fn ssx_audio_system_empty_crowd_handler_matches() {
        check(
            ee::s07::f_00289680__generated,
            ssx_audio_system_empty_crowd_handler,
            50,
            &|c, seed| {
                let mut r = Rng(0x9680 ^ u64::from(seed));
                scramble(c, &mut r);
                c.g[RA][0] = sx(0x0030_0000 + 8 * seed);
            },
        );
    }

    /// The paint: rider match or not, a null or set object, every type code
    /// (-1, 0x1f, 0x20, 0x21, other) and the song-tag results. The rider and the
    /// song tag come from the faked callees; the miss fakes return a pointer to
    /// the type-code word.
    fn oc1018_paint_setup(c: &mut Cpu, seed: u32) {
        oc1018_machine(c, seed, 0x99f8, 4, 0x0028_99f8);
        let rider = DATA + 0x2000;
        c.g[A1][0] = u64::from(rider);
        c.g[A2][0] = if seed % 5 == 0 {
            0
        } else {
            u64::from(DATA + 0x4000)
        };
        c.bus.write32(
            OC1018_RIDER,
            if seed % 3 == 0 { DATA + 0x2400 } else { rider },
        );
        c.bus
            .write32(OC1018_TAG, [0x65, 0x66, 0][(seed % 3) as usize]);
        // The object's vtable: delta at +0x20, method at +0x24 (a miss fake).
        c.bus.write32(DATA + 0x4004, DATA + 0x5000);
        c.bus.write16(DATA + 0x5020, ((seed % 2) * 0x10) as u16);
        c.bus
            .write32(DATA + 0x5024, fakes::FAKE + 0x10 * (seed % 3));
        // The self's table at +0x5558, the other path's method.
        c.bus.write32(DATA + 0x1000 + 0x5558, DATA + 0x6000);
        c.bus.write16(DATA + 0x6020, 0x10);
        c.bus.write32(DATA + 0x6024, fakes::FAKE + 0x30);
        // The miss fakes return this word's address; the code word is read back.
        let code_word = DATA + 0x7000;
        c.bus.write32(
            code_word,
            [u32::MAX, 0x1f, 0x20, 0x21, 0x33][(seed % 5) as usize],
        );
        for n in 0..8 {
            c.bus.write32(fakes::LOG_RESULTS + 4 * n, code_word);
        }
    }

    /// The paint over every type code, rider and song-tag outcome.
    #[test]
    fn ssx_audio_system_apply_music_trigger_paint_matches() {
        check(
            ee::s07::f_002899f8__generated,
            ssx_audio_system_apply_music_trigger_paint,
            300,
            &oc1018_paint_setup,
        );
    }
}
