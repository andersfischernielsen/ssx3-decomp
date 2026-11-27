//! Staging module oc1141 (wave wave272): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1141`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_ec60, twpigd_surface_load_state, "tWPIGD_Surface_LoadState"),
    (0x002b_f3e0, twpigd_mix_save_state, "tWPIGD_Mix_SaveState"),
    (0x002b_f448, twpigd_ambience_save_state, "tWPIGD_Ambience_SaveState"),
    (0x002b_f4b0, twpigd_speech_save_state, "tWPIGD_Speech_SaveState"),
    (0x002b_f518, twpigd_camera_save_state, "tWPIGD_Camera_SaveState"),
    (0x002b_fa38, twpigd_sky_box_save_state, "tWPIGD_SkyBox_SaveState"),
    (0x002b_fcc8, twpigd_surface_save_state, "tWPIGD_Surface_SaveState"),
    (0x002f_2588, splash_manager_init_field0, "cSplashManager_InitField0"),
];

/// Replay-frame vtable slots: the `read` entry (`lh` delta at +0x10, function
/// at +0x14) and the `write` entry (+0x8 and +0xc).
const REPLAY_FRAME_READ_SLOT: u32 = 0x10;
const REPLAY_FRAME_WRITE_SLOT: u32 = 0x8;

/// One `frame->read/write(dst, 4)` virtual call: `v1` = the frame's vtable
/// word, `a0` = `frame + delta` (the slot's `this` adjustment), `v0` = the
/// slot's function, `ra` = `ret`, then the call through the function table.
fn replay_frame_call(c: &mut Cpu, slot_off: u32, ret_to: u32) -> Flow {
    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
    let vt = c.g[V1][0];
    let slot = VSlot::read(c, vt, slot_off);
    c.g[A0][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(c.g[S0][0]);
    call_ptr(c, slot.func as u32, ret_to)
}

/// The body shared by the surface, mix, ambience, speech, camera and sky-box
/// `LoadState`/`SaveState` pairs: two 4-byte transfers through the replay
/// frame in `s0`, first the object's word at +8 (`a1 = self + 8`), then the
/// word at +0 (`a1 = self`). `s0` is the frame, `s1` the object. Frame: 0x30
/// bytes, `s0` at +0x20, `s1` at +0x10, `ra` at +0.
fn replay_frame_transfer(c: &mut Cpu, slot_off: u32, first_ret: u32, second_ret: u32) -> Flow {
    c.cycles += 25;
    let frame = Frame::open(c, 0x30);
    c.g[A2][0] = 0x4;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
    replay_frame_call(c, slot_off, first_ret)?;
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = 0x4;
    replay_frame_call(c, slot_off, second_ret)?;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `tWPInGameData_Surface_readFromReplayFrame` (0x002bec60): reads the
/// surface's snow-sparkle density and travel distance from the replay frame.
pub fn twpigd_surface_load_state(c: &mut Cpu) {
    guest(c, |c| {
        replay_frame_transfer(c, REPLAY_FRAME_READ_SLOT, 0x002b_ec94, 0x002b_ecb0)
    });
}

/// `tWPInGameData_Mix_writeToReplayFrame` (0x002bf3e0): writes the mix
/// preset and travel distance to the replay frame.
pub fn twpigd_mix_save_state(c: &mut Cpu) {
    guest(c, |c| {
        replay_frame_transfer(c, REPLAY_FRAME_WRITE_SLOT, 0x002b_f414, 0x002b_f430)
    });
}

/// `tWPInGameData_Ambience_writeToReplayFrame` (0x002bf448): the ambience
/// state's two words to the replay frame.
pub fn twpigd_ambience_save_state(c: &mut Cpu) {
    guest(c, |c| {
        replay_frame_transfer(c, REPLAY_FRAME_WRITE_SLOT, 0x002b_f47c, 0x002b_f498)
    });
}

/// `tWPInGameData_Speech_writeToReplayFrame` (0x002bf4b0): the speech
/// state's two words to the replay frame.
pub fn twpigd_speech_save_state(c: &mut Cpu) {
    guest(c, |c| {
        replay_frame_transfer(c, REPLAY_FRAME_WRITE_SLOT, 0x002b_f4e4, 0x002b_f500)
    });
}

/// `tWPInGameData_Camera_writeToReplayFrame` (0x002bf518): the camera
/// state's two words to the replay frame.
pub fn twpigd_camera_save_state(c: &mut Cpu) {
    guest(c, |c| {
        replay_frame_transfer(c, REPLAY_FRAME_WRITE_SLOT, 0x002b_f54c, 0x002b_f568)
    });
}

/// `tWPInGameData_SkyBox_writeToReplayFrame` (0x002bfa38): the sky-box
/// state's two words to the replay frame.
pub fn twpigd_sky_box_save_state(c: &mut Cpu) {
    guest(c, |c| {
        replay_frame_transfer(c, REPLAY_FRAME_WRITE_SLOT, 0x002b_fa6c, 0x002b_fa88)
    });
}

/// `tWPInGameData_Surface_writeToReplayFrame` (0x002bfcc8): the surface's
/// two words to the replay frame.
pub fn twpigd_surface_save_state(c: &mut Cpu) {
    guest(c, |c| {
        replay_frame_transfer(c, REPLAY_FRAME_WRITE_SLOT, 0x002b_fcfc, 0x002b_fd18)
    });
}

/// `int* (int* p)` (0x002f2588, `cSplashManager_InitField0`): stores 0 in
/// the word at `p` and returns `p` (the reference's `tDrop_tDrop`).
pub fn splash_manager_init_field0(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(c.g[A0][0] as u32).sw(c, 0);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, scramble, Rng, DATA};

    /// A replay frame whose vtable has the transfer slot `{delta, FAKE}` at
    /// `slot_off`, a surface object, and the call log (gated by `machine`).
    fn replay_transfer_setup(c: &mut Cpu, seed: u32, slot_off: u32) {
        let mut r = fakes::machine(c, seed, slot_off as u64, 2);
        let vt = DATA + 0x2000;
        let delta = (r.next_u32() as i16 as i32 & !3) as u16;
        c.bus.write16(vt + slot_off, delta);
        c.bus.write32(vt + slot_off + 4, fakes::FAKE);
        let frame = DATA + 0x2100;
        c.bus.write32(frame, vt);
        fill(c, &mut r, DATA + 0x2200, 16);
        c.g[A0][0] = sx(DATA + 0x2200 + 0x10 * (seed % 4));
        c.g[A1][0] = sx(frame);
    }

    #[test]
    fn twpigd_surface_load_state_matches() {
        fakes::covered(
            ee::s08::f_002bec60__generated,
            twpigd_surface_load_state,
            300,
            &|c, seed| replay_transfer_setup(c, seed, REPLAY_FRAME_READ_SLOT),
        );
    }

    #[test]
    fn twpigd_mix_save_state_matches() {
        fakes::covered(
            ee::s08::f_002bf3e0__generated,
            twpigd_mix_save_state,
            300,
            &|c, seed| replay_transfer_setup(c, seed, REPLAY_FRAME_WRITE_SLOT),
        );
    }

    #[test]
    fn twpigd_ambience_save_state_matches() {
        fakes::covered(
            ee::s08::f_002bf448__generated,
            twpigd_ambience_save_state,
            300,
            &|c, seed| replay_transfer_setup(c, seed, REPLAY_FRAME_WRITE_SLOT),
        );
    }

    #[test]
    fn twpigd_speech_save_state_matches() {
        fakes::covered(
            ee::s08::f_002bf4b0__generated,
            twpigd_speech_save_state,
            300,
            &|c, seed| replay_transfer_setup(c, seed, REPLAY_FRAME_WRITE_SLOT),
        );
    }

    #[test]
    fn twpigd_camera_save_state_matches() {
        fakes::covered(
            ee::s08::f_002bf518__generated,
            twpigd_camera_save_state,
            300,
            &|c, seed| replay_transfer_setup(c, seed, REPLAY_FRAME_WRITE_SLOT),
        );
    }

    #[test]
    fn twpigd_sky_box_save_state_matches() {
        fakes::covered(
            ee::s08::f_002bfa38__generated,
            twpigd_sky_box_save_state,
            300,
            &|c, seed| replay_transfer_setup(c, seed, REPLAY_FRAME_WRITE_SLOT),
        );
    }

    #[test]
    fn twpigd_surface_save_state_matches() {
        fakes::covered(
            ee::s08::f_002bfcc8__generated,
            twpigd_surface_save_state,
            300,
            &|c, seed| replay_transfer_setup(c, seed, REPLAY_FRAME_WRITE_SLOT),
        );
    }

    #[test]
    fn splash_manager_init_field0_matches() {
        check(
            ee::s09::f_002f2588__generated,
            splash_manager_init_field0,
            300,
            &|c, seed| {
                let mut r = Rng(0x2f25_88 ^ seed as u64);
                scramble(c, &mut r);
                let p = DATA + 0x2400 + 0x10 * (seed % 4);
                c.g[A0][0] = sx(p);
                c.bus.write32(p, r.next_u32());
            },
        );
    }
}
