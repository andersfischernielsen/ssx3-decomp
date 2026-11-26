//! Staging module oc729 (wave wave241): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc729`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0026_f8a0, replay_begin_playback, "Replay_BeginPlayback")];

/// `void cReplay_startReplay(cReplay* self, int frame, int instantReplay,
/// int autoReplay)` (0x26f8a0). Does nothing while the replay is in a
/// transition (state 14 or 15). Otherwise: take a resume snapshot if none is
/// held (the field at +0x608), enter the replay state 7, clear the command
/// and auto-replay fields (+0x618, +0x61c), remember the instant flag (+0x620),
/// rewind to `frame`, step one frame, then enable the instant-replay cameras
/// (or the normal ones). A normal, non-automatic replay also starts the audio
/// side (`SSXAudioSystem_OnReplayBegin`).
pub fn replay_begin_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);

        // ok = !(state == 14 || state == 15): the unsigned compare of the
        // sign-extended (state - 14) with 2.
        let state = Addr(c.g[S0][0] as u32).lw(c);
        let skip = sx((state as u32).wrapping_sub(0xe)) < 2;
        c.g[V0][0] = if skip { 0 } else { 1 };
        c.g[S3][0] = c.g[A3][0];

        if skip {
            // 'b_26f960 exit: the two restores, then the epilogue.
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
            c.cycles += 1;
            frame.restore(c, S1, 0x30);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x608).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                call_ptr(c, 0x0026_f7f8, 0x0026_f8f0)?; // Replay_SaveResumeSnapshot
            }
            c.g[A0][0] = c.g[S0][0];

            c.cycles += 13;
            c.g[A1][0] = 7;
            call_ptr(c, 0x0026_f4a0, 0x0026_f8fc)?; // Replay_SetState
            Addr(c.g[S0][0] as u32).off(0x618).sw(c, 0);
            c.g[A1][0] = c.g[S2][0];
            Addr(c.g[S0][0] as u32).off(0x61c).sw(c, 0);
            c.g[A0][0] = c.g[S0][0];
            let instant = c.g[S1][0];
            Addr(c.g[S0][0] as u32).off(0x620).sw(c, instant);
            call_ptr(c, 0x0027_02f8, 0x0026_f914)?; // Replay_SeekToSnapshot
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0027_03f0, 0x0026_f920)?; // Replay_StepFrame

            if c.g[S1][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ef30, 0x0026_f930)?; // Replay_EnableInstantReplayCameras
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ee68, 0x0026_f940)?; // Replay_EnableReplayCameras
            }

            c.cycles += 2;
            frame.restore(c, S0, 0x40);
            if c.g[S1][0] != 0 {
                c.cycles += 1;
                frame.restore(c, S1, 0x30);
            } else {
                c.cycles += 2;
                frame.restore(c, S1, 0x30);
                if c.g[S3][0] == 0 {
                    c.cycles += 4;
                    call_ptr(c, 0x0028_b180, 0x0026_f958)?; // SSXAudioSystem_GetInstance
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0028_f200, 0x0026_f960)?; // SSXAudioSystem_OnReplayBegin
                    c.cycles += 1;
                    frame.restore(c, S0, 0x40);
                    c.cycles += 1;
                    frame.restore(c, S1, 0x30);
                }
            }
        }

        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, DATA};

    /// Gate for the replay callee fakes: they run the real callee (the slot's
    /// previous body) unless this word holds the magic value in the machine's
    /// RAM. Each test's fixture sets it on both sides.
    pub(crate) const OC729_RBP_CTRL: u32 = DATA + 0x2000;
    pub(crate) const OC729_RBP_MAGIC: u32 = 0x729b_0a1d;

    #[inline]
    fn oc729_rbp_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC729_RBP_CTRL) == OC729_RBP_MAGIC
    }

    pub(crate) fn oc729_fake_save_resume(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc729_fake_save_resume),
                ee::s07::f_0026f7f8,
            );
        }
        fakes::fake_call(c, 0x0026_f7f8)
    }
    pub(crate) fn oc729_fake_set_state(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc729_fake_set_state),
                ee::s07::f_0026f4a0,
            );
        }
        fakes::fake_call(c, 0x0026_f4a0)
    }
    pub(crate) fn oc729_fake_seek(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(c, fakes::prev_cell(oc729_fake_seek), ee::s07::f_002702f8);
        }
        fakes::fake_call(c, 0x0027_02f8)
    }
    pub(crate) fn oc729_fake_step(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(c, fakes::prev_cell(oc729_fake_step), ee::s07::f_002703f0);
        }
        fakes::fake_call(c, 0x0027_03f0)
    }
    pub(crate) fn oc729_fake_enable_instant(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc729_fake_enable_instant),
                ee::s07::f_0026ef30,
            );
        }
        fakes::fake_call(c, 0x0026_ef30)
    }
    pub(crate) fn oc729_fake_enable_cameras(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc729_fake_enable_cameras),
                ee::s07::f_0026ee68,
            );
        }
        fakes::fake_call(c, 0x0026_ee68)
    }
    pub(crate) fn oc729_fake_audio_instance(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc729_fake_audio_instance),
                ee::s07::f_0028b180,
            );
        }
        fakes::fake_call(c, 0x0028_b180)
    }
    pub(crate) fn oc729_fake_audio_replay_begin(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc729_fake_audio_replay_begin),
                ee::s07::f_0028f200,
            );
        }
        fakes::fake_call(c, 0x0028_f200)
    }

    /// Hooks the eight callee fakes in their override slots (once per process).
    pub(crate) fn oc729_install_fakes() {
        let slots: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 8] = [
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F7F8),
                oc729_fake_save_resume,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F4A0),
                oc729_fake_set_state,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_002702F8),
                oc729_fake_seek,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_002703F0),
                oc729_fake_step,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026EF30),
                oc729_fake_enable_instant,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026EE68),
                oc729_fake_enable_cameras,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0028B180),
                oc729_fake_audio_instance,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0028F200),
                oc729_fake_audio_replay_begin,
            ),
        ];
        for (slot, fake) in slots {
            fakes::hook(slot, fake, fakes::prev_cell(fake));
        }
    }

    #[test]
    fn replay_begin_playback_matches() {
        use crate::difftest::fakes::machine;
        oc729_install_fakes();
        check(
            ee::s07::f_0026f8a0__generated,
            replay_begin_playback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0026_f8a0, 0);
                c.bus.write32(OC729_RBP_CTRL, OC729_RBP_MAGIC);
                c.pc = 0x0026_f8a0;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(r.next_u32());
                // instantReplay and autoReplay: all four combinations.
                c.g[A2][0] = (seed & 1) as u64;
                c.g[A3][0] = ((seed >> 1) & 1) as u64;
                // state 14 and 15 are transitions (skipped); 0 and 3 run.
                let state = [14u32, 15, 0, 3][(seed % 4) as usize];
                c.bus.write32(DATA, state);
                // The resume snapshot is held or not.
                let snapshot = if (seed / 4) % 2 == 0 {
                    0
                } else {
                    r.next_u32() | 1
                };
                c.bus.write32(DATA + 0x608, snapshot);
            },
        );
    }
}
