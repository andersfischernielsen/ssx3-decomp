//! Staging module oc1551 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1551`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::worldpainter::wpigd::{replay_frame_call, REPLAY_FRAME_READ_SLOT, REPLAY_FRAME_WRITE_SLOT};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_e2d8, twpigd_music_trigger_load_state, "tWPIGD_MusicTrigger_LoadState"),
    (0x002b_e518, twpigd_fog_load_state, "tWPIGD_Fog_LoadState"),
    (0x002b_e698, twpigd_light_glow_load_state, "tWPIGD_LightGlow_LoadState"),
    (0x002b_e850, twpigd_screen_tint_load_state, "tWPIGD_ScreenTint_LoadState"),
    (0x002b_ecc8, twpigd_lighting_load_state, "tWPIGD_Lighting_LoadState"),
    (0x002b_f2a0, twpigd_danger_load_state, "tWPIGD_Danger_LoadState"),
    (0x002b_f340, twpigd_music_trigger_save_state, "tWPIGD_MusicTrigger_SaveState"),
    (0x002b_f580, twpigd_fog_save_state, "tWPIGD_Fog_SaveState"),
];

/// One replay-frame transfer of a state function: `(off, size, ret)` moves
/// `size` bytes at `self + off` (`off` 0 is `self` itself) through the frame's
/// vtable, and `ret` is the address the call returns to.
/// The shared body of the eight `tWPIGD_*_LoadState` and `SaveState` functions
/// below: `s0` = the replay frame, `s1` = the object, one virtual call per row.
pub(crate) fn wpigd_field_transfers(
    c: &mut Cpu,
    slot_off: u32,
    cycles: u64,
    fields: &[(u32, u64, u32)],
) -> Flow {
    c.cycles += cycles;
    let frame = Frame::open(c, 0x30);
    c.g[A2][0] = fields[0].1;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    for &(off, size, ret_to) in fields {
        c.g[A1][0] = if off == 0 {
            c.g[S1][0]
        } else {
            sx((c.g[S1][0] as u32).wrapping_add(off))
        };
        c.g[A2][0] = size;
        replay_frame_call(c, slot_off, ret_to)?;
    }
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `tWPInGameData_MusicTrigger_readFromReplayFrame` (0x002b_e2d8): reads the trigger's type, travel distance, value and travel distance again.
pub fn twpigd_music_trigger_load_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_MusicTrigger_LoadState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
    (0x8, 4, 0x002b_e30c),
    (0x0, 4, 0x002b_e328),
    (0x10, 4, 0x002b_e344),
    (0x0, 4, 0x002b_e360),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_READ_SLOT, 39, FIELDS)
    });
}

/// `tWPInGameData_Fog_readFromReplayFrame` (0x002b_e518): reads the fog density, near and far distances and the RGB colour, each followed by the base word.
pub fn twpigd_fog_load_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_Fog_LoadState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
    (0x8, 4, 0x002b_e54c),
    (0x0, 4, 0x002b_e568),
    (0x10, 4, 0x002b_e584),
    (0x0, 4, 0x002b_e5a0),
    (0x18, 4, 0x002b_e5bc),
    (0x0, 4, 0x002b_e5d8),
    (0x20, 4, 0x002b_e5f4),
    (0x0, 4, 0x002b_e610),
    (0x28, 4, 0x002b_e62c),
    (0x0, 4, 0x002b_e648),
    (0x30, 4, 0x002b_e664),
    (0x0, 4, 0x002b_e680),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_READ_SLOT, 95, FIELDS)
    });
}

/// `tWPInGameData_LightGlow_readFromReplayFrame` (0x002b_e698): reads the light glow's cutoff, scale, copy/source/frame intensities and blend values.
pub fn twpigd_light_glow_load_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_LightGlow_LoadState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
    (0x8, 4, 0x002b_e6cc),
    (0x0, 4, 0x002b_e6e8),
    (0x10, 4, 0x002b_e704),
    (0x0, 4, 0x002b_e720),
    (0x18, 4, 0x002b_e73c),
    (0x0, 4, 0x002b_e758),
    (0x20, 4, 0x002b_e774),
    (0x0, 4, 0x002b_e790),
    (0x28, 4, 0x002b_e7ac),
    (0x0, 4, 0x002b_e7c8),
    (0x30, 4, 0x002b_e7e4),
    (0x0, 4, 0x002b_e800),
    (0x38, 4, 0x002b_e81c),
    (0x0, 4, 0x002b_e838),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_READ_SLOT, 109, FIELDS)
    });
}

/// `tWPInGameData_ScreenTint_readFromReplayFrame` (0x002b_e850): reads the screen tint's scale and offset for R, G and B.
pub fn twpigd_screen_tint_load_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_ScreenTint_LoadState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
    (0x8, 4, 0x002b_e884),
    (0x0, 4, 0x002b_e8a0),
    (0x10, 4, 0x002b_e8bc),
    (0x0, 4, 0x002b_e8d8),
    (0x18, 4, 0x002b_e8f4),
    (0x0, 4, 0x002b_e910),
    (0x20, 4, 0x002b_e92c),
    (0x0, 4, 0x002b_e948),
    (0x28, 4, 0x002b_e964),
    (0x0, 4, 0x002b_e980),
    (0x30, 4, 0x002b_e99c),
    (0x0, 4, 0x002b_e9b8),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_READ_SLOT, 95, FIELDS)
    });
}

/// `tWPInGameData_Lighting_readFromReplayFrame` (0x002b_ecc8): reads the four 8-byte irradiance names and the intensity offsets.
pub fn twpigd_lighting_load_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_Lighting_LoadState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
    (0x8, 8, 0x002b_ecfc),
    (0x0, 4, 0x002b_ed18),
    (0x18, 8, 0x002b_ed34),
    (0x0, 4, 0x002b_ed50),
    (0x28, 8, 0x002b_ed6c),
    (0x0, 4, 0x002b_ed88),
    (0x38, 8, 0x002b_eda4),
    (0x0, 4, 0x002b_edc0),
    (0x48, 4, 0x002b_eddc),
    (0x0, 4, 0x002b_edf8),
    (0x50, 4, 0x002b_ee14),
    (0x0, 4, 0x002b_ee30),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_READ_SLOT, 95, FIELDS)
    });
}

/// `tWPInGameData_Danger_readFromReplayFrame` (0x002b_f2a0): reads the danger level, travel distance and point rate.
pub fn twpigd_danger_load_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_Danger_LoadState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
    (0x8, 4, 0x002b_f2d4),
    (0x0, 4, 0x002b_f2f0),
    (0x10, 4, 0x002b_f30c),
    (0x0, 4, 0x002b_f328),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_READ_SLOT, 39, FIELDS)
    });
}

/// `tWPInGameData_MusicTrigger_writeToReplayFrame` (0x002b_f340): writes the trigger's type, travel distance, value and travel distance again.
pub fn twpigd_music_trigger_save_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_MusicTrigger_SaveState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
    (0x8, 4, 0x002b_f374),
    (0x0, 4, 0x002b_f390),
    (0x10, 4, 0x002b_f3ac),
    (0x0, 4, 0x002b_f3c8),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_WRITE_SLOT, 39, FIELDS)
    });
}

/// `tWPInGameData_Fog_writeToReplayFrame` (0x002b_f580): writes the fog density, near and far distances and the RGB colour, each followed by the base word.
pub fn twpigd_fog_save_state(c: &mut Cpu) {
    // The transfers of `tWPIGD_Fog_SaveState`, in call order.
    const FIELDS: &[(u32, u64, u32)] = &[
    (0x8, 4, 0x002b_f5b4),
    (0x0, 4, 0x002b_f5d0),
    (0x10, 4, 0x002b_f5ec),
    (0x0, 4, 0x002b_f608),
    (0x18, 4, 0x002b_f624),
    (0x0, 4, 0x002b_f640),
    (0x20, 4, 0x002b_f65c),
    (0x0, 4, 0x002b_f678),
    (0x28, 4, 0x002b_f694),
    (0x0, 4, 0x002b_f6b0),
    (0x30, 4, 0x002b_f6cc),
    (0x0, 4, 0x002b_f6e8),
    ];
    guest(c, |c| {
        wpigd_field_transfers(c, REPLAY_FRAME_WRITE_SLOT, 95, FIELDS)
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::DATA;

    /// The reader object for a state function: a replay frame whose vtable has
    /// the transfer slot `{delta, FAKE}` at `slot_off`, and an object at
    /// `DATA + 0x2200`. `calls` is the number of transfers, so the fake's
    /// unwind index can fall on any of them.
    fn oc1551_state_setup(c: &mut Cpu, seed: u32, slot_off: u32, calls: u32, salt: u64) {
        let mut r = fakes::machine(c, seed, salt, calls);
        let vt = DATA + 0x2000;
        let delta = (r.next_u32() as i16 as i32 & !3) as u16;
        c.bus.write16(vt + slot_off, delta);
        c.bus.write32(vt + slot_off + 4, fakes::FAKE);
        let frame = DATA + 0x2100;
        c.bus.write32(frame, vt);
        c.g[A0][0] = sx(DATA + 0x2200 + 0x10 * (seed % 4));
        c.g[A1][0] = sx(frame);
    }

    #[test]
    fn twpigd_music_trigger_load_state_matches() {
        fakes::covered(
            ee::s08::f_002be2d8__generated,
            twpigd_music_trigger_load_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_READ_SLOT, 4, 0x1551_e2d8),
        );
    }

    #[test]
    fn twpigd_fog_load_state_matches() {
        fakes::covered(
            ee::s08::f_002be518__generated,
            twpigd_fog_load_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_READ_SLOT, 12, 0x1551_e518),
        );
    }

    #[test]
    fn twpigd_light_glow_load_state_matches() {
        fakes::covered(
            ee::s08::f_002be698__generated,
            twpigd_light_glow_load_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_READ_SLOT, 14, 0x1551_e698),
        );
    }

    #[test]
    fn twpigd_screen_tint_load_state_matches() {
        fakes::covered(
            ee::s08::f_002be850__generated,
            twpigd_screen_tint_load_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_READ_SLOT, 12, 0x1551_e850),
        );
    }

    #[test]
    fn twpigd_lighting_load_state_matches() {
        fakes::covered(
            ee::s08::f_002becc8__generated,
            twpigd_lighting_load_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_READ_SLOT, 12, 0x1551_ecc8),
        );
    }

    #[test]
    fn twpigd_danger_load_state_matches() {
        fakes::covered(
            ee::s08::f_002bf2a0__generated,
            twpigd_danger_load_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_READ_SLOT, 4, 0x1551_f2a0),
        );
    }

    #[test]
    fn twpigd_music_trigger_save_state_matches() {
        fakes::covered(
            ee::s08::f_002bf340__generated,
            twpigd_music_trigger_save_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_WRITE_SLOT, 4, 0x1551_f340),
        );
    }

    #[test]
    fn twpigd_fog_save_state_matches() {
        fakes::covered(
            ee::s08::f_002bf580__generated,
            twpigd_fog_save_state,
            300,
            &|c, seed| oc1551_state_setup(c, seed, REPLAY_FRAME_WRITE_SLOT, 12, 0x1551_f580),
        );
    }
}
