//! Staging module oc1140 (wave wave272): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1140`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_ac38, music_player_get_song_stream_handle, "MusicPlayer_GetSongStreamHandle"),
    (0x002b_ac98, music_player_apply_delay_params, "MusicPlayer_ApplyDelayParams"),
    (0x002b_c9b0, wpigd_lighting_ctor, "tWPIGD_Lighting_ctor"),
    (0x002b_e378, wpigd_mix_load_state, "tWPIGD_Mix_LoadState"),
    (0x002b_e3e0, wpigd_ambience_load_state, "tWPIGD_Ambience_LoadState"),
    (0x002b_e448, wpigd_speech_load_state, "tWPIGD_Speech_LoadState"),
    (0x002b_e4b0, wpigd_camera_load_state, "tWPIGD_Camera_LoadState"),
    (0x002b_e9d0, wpigd_skybox_load_state, "tWPIGD_SkyBox_LoadState"),
];

/// `SongInstance_GetStreamHandle` (0x2b2488), called from the music player.
const MUSIC_PLAYER_STREAM_HANDLE_CALLEE: Callee = Callee {
    addr: 0x002b_2488,
    f: ee::s08::SongInstance_GetStreamHandle,
};

/// `SongInstance_ApplyDelayParams` (0x2b2850), called from the music player.
const MUSIC_PLAYER_APPLY_DELAY_CALLEE: Callee = Callee {
    addr: 0x002b_2850,
    f: ee::s08::SongInstance_ApplyDelayParams,
};

/// The song instance pointer of a music player (`+0x408`).
pub(crate) const MUSIC_PLAYER_SONG_INSTANCE: u32 = 0x408;

/// `tWPInGameData_Lighting` vtable (0x484290, built in two halves).
pub(crate) const WPIGD_LIGHTING_VTABLE: u32 = 0x0048_4290;
/// Travel distance (-99999.0f) is loaded from `gp - 0x42b8`.
pub(crate) const WPIGD_LIGHTING_TRAVEL_GP_OFFSET: u32 = 0x42b8;
/// The null name constant lives at `gp + 0x22e0` (8 bytes).
pub(crate) const WPIGD_LIGHTING_NULL_NAME_GP_OFFSET: u32 = 0x22e0;
/// Offsets of the four light intensity offsets in the lighting block.
pub(crate) const WPIGD_LIGHTING_LAST_AMBIENT: u32 = 0x48;
pub(crate) const WPIGD_LIGHTING_AMBIENT: u32 = 0x4c;
pub(crate) const WPIGD_LIGHTING_LAST_RIM: u32 = 0x50;
pub(crate) const WPIGD_LIGHTING_RIM: u32 = 0x54;

/// `MusicPlayer_GetSongStreamHandle` (0x2bac38, `cMusicSys_MUSIC_GetStreamHandle`):
/// the stream handle of the player's song instance (`+0x408`), or -1 when
/// there is no song instance.
pub fn music_player_get_song_stream_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32)
            .off(MUSIC_PLAYER_SONG_INSTANCE)
            .lw(c);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            MUSIC_PLAYER_STREAM_HANDLE_CALLEE.call(c, 0x002b_ac54)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MusicPlayer_ApplyDelayParams` (0x2bac98): passes the player's song
/// instance (`+0x408`) and the caller's second argument on to
/// `SongInstance_ApplyDelayParams`. Tail-forwards the callee's result.
pub fn music_player_apply_delay_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A1][0] as u32)
            .off(MUSIC_PLAYER_SONG_INSTANCE)
            .lw(c);
        MUSIC_PLAYER_APPLY_DELAY_CALLEE.call(c, 0x002b_acac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One pair of null name fields: the 8-byte null name at `gp + 0x22e0` is
/// stored at `this + hi` and at `this + hi - 8` (the two elements of a
/// `tWPNameField` pair), in the generated sdl/sdr order. The stack round trip
/// and the reloads of `v1` stay, since they write the stack and leave `v1`.
fn wpigd_null_name_pair(c: &mut Cpu, this: u32, hi: u32) {
    let gp = c.g[GP][0] as u32;
    let stack = sp(c);
    c.ldl(V1, gp.wrapping_add(WPIGD_LIGHTING_NULL_NAME_GP_OFFSET + 7));
    c.ldr(V1, gp.wrapping_add(WPIGD_LIGHTING_NULL_NAME_GP_OFFSET));
    c.sdl(V1, stack.wrapping_add(7));
    c.sdr(V1, stack);
    c.ldl(V1, stack.wrapping_add(7));
    c.ldr(V1, stack);
    c.sdl(V1, this.wrapping_add(hi + 7));
    c.sdr(V1, this.wrapping_add(hi));
    c.ldl(V1, this.wrapping_add(hi + 7));
    c.ldr(V1, this.wrapping_add(hi));
    c.sdl(V1, this.wrapping_add(hi - 1));
    c.sdr(V1, this.wrapping_add(hi - 8));
}

/// `tWPInGameData_Lighting` constructor (0x2bc9b0): `this` in a0, returned in
/// v0. Sets the vtable at `+4`, the travel distance (-99999.0f) at `+0`,
/// clears the eight light-irradiance name fields (`+8..+0x48`) and fills them
/// with the null name, then sets the ambient and rim intensity offsets to
/// 1.0 and 0.75.
pub fn wpigd_lighting_ctor(c: &mut Cpu) {
    c.cycles += 81;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(WPIGD_LIGHTING_TRAVEL_GP_OFFSET));
    let this = Addr(c.g[A0][0] as u32);
    let vtable = sx(WPIGD_LIGHTING_VTABLE);
    this.off(0xc).sw(c, 0);
    this.off(4).sw(c, vtable);
    let frame = Frame::open(c, 0x10);
    c.swc1(0, this.0);
    c.g[V0][0] = c.g[A0][0];
    for j in 0..16 {
        this.off(0x8 + 4 * j).sw(c, 0);
    }
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    wpigd_null_name_pair(c, this.0, 0x10);
    wpigd_null_name_pair(c, this.0, 0x20);
    c.g[AT][0] = 0x3f40_0000;
    c.mtc1(AT, 0);
    wpigd_null_name_pair(c, this.0, 0x30);
    wpigd_null_name_pair(c, this.0, 0x40);
    c.swc1(1, this.off(WPIGD_LIGHTING_LAST_AMBIENT).0);
    c.swc1(0, this.off(WPIGD_LIGHTING_LAST_RIM).0);
    c.swc1(1, this.off(WPIGD_LIGHTING_AMBIENT).0);
    c.swc1(0, this.off(WPIGD_LIGHTING_RIM).0);
    frame.close(c);
}

/// Shared body of the five `tWPIGD_*_LoadState` functions, which differ only
/// in their two return addresses: `this` in a0, the reader object in a1. It
/// calls the reader's virtual read (vtable slot `+0x14`, this-adjust
/// `+0x10`) first with `&this->field8`, then with `this` itself, 4 bytes
/// each.
fn wpigd_load_state_body(c: &mut Cpu, first_ret: u32, second_ret: u32) -> Flow {
    c.cycles += 25;
    let frame = Frame::open(c, 0x30);
    c.g[A2][0] = 4;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, first_ret)?;
    c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = 4;
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, second_ret)?;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `tWPInGameData_Mix_readFromReplayFrame` (0x2be378, `tWPIGD_Mix_LoadState`):
/// reads the mix preset and the travel distance from the replay frame.
pub fn wpigd_mix_load_state(c: &mut Cpu) {
    guest(c, |c| wpigd_load_state_body(c, 0x002b_e3ac, 0x002b_e3c8));
}

/// `tWPIGD_Ambience_LoadState` (0x2be3e0): the shared load-state body.
pub fn wpigd_ambience_load_state(c: &mut Cpu) {
    guest(c, |c| wpigd_load_state_body(c, 0x002b_e414, 0x002b_e430));
}

/// `tWPIGD_Speech_LoadState` (0x2be448): the shared load-state body.
pub fn wpigd_speech_load_state(c: &mut Cpu) {
    guest(c, |c| wpigd_load_state_body(c, 0x002b_e47c, 0x002b_e498));
}

/// `tWPIGD_Camera_LoadState` (0x2be4b0): the shared load-state body.
pub fn wpigd_camera_load_state(c: &mut Cpu) {
    guest(c, |c| wpigd_load_state_body(c, 0x002b_e4e4, 0x002b_e500));
}

/// `tWPIGD_SkyBox_LoadState` (0x2be9d0): the shared load-state body.
pub fn wpigd_skybox_load_state(c: &mut Cpu) {
    guest(c, |c| wpigd_load_state_body(c, 0x002b_ea04, 0x002b_ea20));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, fill, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the stream-handle fake (+4: the handle it returns).
    pub(crate) const OC1140_STREAM_CTRL: u32 = DATA + 0xf_c200;
    pub(crate) const OC1140_STREAM_MAGIC: u32 = 0x0c11_4001;
    /// Gate word of the delay-params fake.
    pub(crate) const OC1140_APPLY_CTRL: u32 = DATA + 0xf_c300;
    pub(crate) const OC1140_APPLY_MAGIC: u32 = 0x0c11_4002;

    pub(crate) static OC1140_PREV_STREAM: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1140_PREV_APPLY: AtomicUsize = AtomicUsize::new(0);

    /// The stream-handle callee, gated: with the gate set it logs the call
    /// and returns the handle word; otherwise it runs the body it replaced.
    pub(crate) fn oc1140_stream_fake(c: &mut Cpu) {
        if c.bus.read32(OC1140_STREAM_CTRL) != OC1140_STREAM_MAGIC {
            return fakes::fall_back(
                c,
                &OC1140_PREV_STREAM,
                ee::s08::f_002b2488__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, 0x002b_2488);
        if c.pc == ra {
            c.g[V0][0] = c.bus.read32(OC1140_STREAM_CTRL + 4) as u64;
        }
    }

    /// The delay-params callee, gated like [`oc1140_stream_fake`].
    pub(crate) fn oc1140_apply_fake(c: &mut Cpu) {
        if c.bus.read32(OC1140_APPLY_CTRL) != OC1140_APPLY_MAGIC {
            return fakes::fall_back(
                c,
                &OC1140_PREV_APPLY,
                ee::s08::f_002b2850__generated,
            );
        }
        fakes::fake_call(c, 0x002b_2850);
    }

    pub(crate) fn oc1140_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B2488),
            oc1140_stream_fake,
            &OC1140_PREV_STREAM,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B2850),
            oc1140_apply_fake,
            &OC1140_PREV_APPLY,
        );
    }

    /// A music player whose `+0x408` holds a song instance (zero on half the
    /// seeds, so both arms run), with the stream-handle fake gated on.
    pub(crate) fn oc1140_player_setup(c: &mut Cpu, seed: u32, salt: u64) {
        oc1140_install_fakes();
        let mut r = machine(c, seed, salt, 1);
        let player = DATA + 0x1000;
        let song = if seed % 2 == 0 {
            0
        } else {
            r.next_u32() | 0x100
        };
        c.bus.write32(player + MUSIC_PLAYER_SONG_INSTANCE, song);
        c.g[A0][0] = sx(player);
        c.bus.write32(OC1140_STREAM_CTRL, OC1140_STREAM_MAGIC);
        c.bus.write32(OC1140_STREAM_CTRL + 4, r.next_u32());
        c.bus.write32(OC1140_APPLY_CTRL, 0);
    }

    /// A music player whose song instance is set, with the delay-params fake
    /// gated on.
    pub(crate) fn oc1140_apply_setup(c: &mut Cpu, seed: u32, salt: u64) {
        oc1140_install_fakes();
        let mut r = machine(c, seed, salt, 1);
        let player = DATA + 0x1000;
        c.bus.write32(player + MUSIC_PLAYER_SONG_INSTANCE, r.next_u32());
        c.g[A0][0] = sx(player);
        c.bus.write32(OC1140_APPLY_CTRL, OC1140_APPLY_MAGIC);
        c.bus.write32(OC1140_STREAM_CTRL, 0);
    }

    #[test]
    fn music_player_get_song_stream_handle_matches() {
        covered(
            ee::s08::f_002bac38__generated,
            music_player_get_song_stream_handle,
            200,
            &|c, seed| oc1140_player_setup(c, seed, 0x1140_01),
        );
    }

    #[test]
    fn music_player_apply_delay_params_matches() {
        covered(
            ee::s08::f_002bac98__generated,
            music_player_apply_delay_params,
            200,
            &|c, seed| oc1140_apply_setup(c, seed, 0x1140_02),
        );
    }

    #[test]
    fn wpigd_lighting_ctor_matches() {
        check(
            ee::s08::f_002bc9b0__generated,
            wpigd_lighting_ctor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1140_03, 0);
                let this = DATA + 0x1000;
                fill(c, &mut r, this, 24);
                fill(c, &mut r, GAME_GP.wrapping_sub(WPIGD_LIGHTING_TRAVEL_GP_OFFSET), 1);
                fill(c, &mut r, GAME_GP + WPIGD_LIGHTING_NULL_NAME_GP_OFFSET, 2);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// The reader object and its vtable for the `LoadState` family: the
    /// vtable's slot `+0x14` is the fake callee, its this-adjust at `+0x10`.
    fn oc1140_load_state_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 2);
        let this = DATA + 0x1000;
        let frame = DATA + 0x2000;
        let vt = DATA + 0x4000;
        c.bus.write16(vt + 0x10, (r.next_u32() & 0xfffc) as u16);
        c.bus.write32(vt + 0x14, fakes::FAKE);
        c.bus.write32(frame, vt);
        fill(c, &mut r, this, 4);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(frame);
    }

    #[test]
    fn wpigd_mix_load_state_matches() {
        covered(
            ee::s08::f_002be378__generated,
            wpigd_mix_load_state,
            200,
            &|c, seed| oc1140_load_state_setup(c, seed, 0x1140_10),
        );
    }

    #[test]
    fn wpigd_ambience_load_state_matches() {
        covered(
            ee::s08::f_002be3e0__generated,
            wpigd_ambience_load_state,
            200,
            &|c, seed| oc1140_load_state_setup(c, seed, 0x1140_11),
        );
    }

    #[test]
    fn wpigd_speech_load_state_matches() {
        covered(
            ee::s08::f_002be448__generated,
            wpigd_speech_load_state,
            200,
            &|c, seed| oc1140_load_state_setup(c, seed, 0x1140_12),
        );
    }

    #[test]
    fn wpigd_camera_load_state_matches() {
        covered(
            ee::s08::f_002be4b0__generated,
            wpigd_camera_load_state,
            200,
            &|c, seed| oc1140_load_state_setup(c, seed, 0x1140_13),
        );
    }

    #[test]
    fn wpigd_skybox_load_state_matches() {
        covered(
            ee::s08::f_002be9d0__generated,
            wpigd_skybox_load_state,
            200,
            &|c, seed| oc1140_load_state_setup(c, seed, 0x1140_14),
        );
    }
}
