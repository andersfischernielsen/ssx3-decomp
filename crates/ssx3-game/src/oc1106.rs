//! Staging module oc1106 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1106`.

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
    (0x003d_2fe0, path_shutdown, "PATH_Shutdown"),
    (0x003d_30a8, path_track_call_stream, "PATH_TrackCallStream"),
    (0x003d_3140, path_track_unload_bank, "PATH_TrackUnloadBank"),
    (0x003d_31e8, path_set_map_callbacks, "PATH_SetMapCallbacks"),
];

/// `PATH_Started` byte (0x44f42c, `gp`-relative `-0xbd4` from 0x450000): set while the
/// path layer is started; `PATH_Shutdown` clears it.
const PATH_STARTED_FLAG: Addr = Addr(0x0044_f42c);
/// The selected map id word (0x44f430): 0 = unset, -1 = none.
const PATH_SELECTED_ID: Addr = Addr(0x0044_f430);
/// The selected map pointer word (0x44f420).
const PATH_SELECTED_MAP: Addr = Addr(0x0044_f420);

/// `PATH_StopTracks(int mask)` (0x3d2a68), ported elsewhere.
const PATH_STOP_TRACKS_CALLEE: Callee = Callee {
    addr: 0x003d_2a68,
    f: ee::s14::f_003d2a68,
};
/// `PATH_SelectMap(uint idx)` (0x3d5450).
const PATH_SELECT_MAP_CALLEE: Callee = Callee {
    addr: 0x003d_5450,
    f: ee::s14::f_003d5450,
};
/// `PATH_FindTrack(int id)` (0x3d5290).
const PATH_FIND_TRACK_CALLEE: Callee = Callee {
    addr: 0x003d_5290,
    f: ee::s14::f_003d5290,
};

/// `void PATH_Shutdown(void)` (0x3d2fe0): stops every track
/// (`PATH_StopTracks(-1)`), then clears the started flag and the selected
/// map id.
pub fn path_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0xffff_ffff);
        PATH_STOP_TRACKS_CALLEE.call(c, 0x003d_2ff0)?;
        c.g[V1][0] = 0x0045_0000;
        c.g[A0][0] = 0x0045_0000;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        PATH_STARTED_FLAG.sb(c, 0);
        PATH_SELECTED_ID.sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PATH_TrackCallStream(int track, int a1, int a2)` (0x3d30a8): when the
/// path layer is started, select a map if none is, find the track and hand
/// its stream to the platform layer (`PATH_PlatSetTrackFile`), then clear
/// the selected id and map.
pub fn path_track_call_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = 0x0045_0000;
        c.g[V1][0] = PATH_STARTED_FLAG.lbu(c);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);
        let mut reached_bank = true;
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[S3][0] = 0x0045_0000;
            c.g[V0][0] = PATH_SELECTED_ID.lw(c);
            if c.g[V0][0] != 0 {
                reached_bank = false;
            } else {
                c.cycles += 4;
                c.g[A0][0] = sx(0xffff_ffff);
                PATH_SELECT_MAP_CALLEE.call(c, 0x003d_30f0)?;
                if c.g[V0][0] == 0 {
                    reached_bank = false;
                } else {
                    path_track_call_stream_load(c)?;
                }
            }
        }
        if reached_bank {
            c.cycles += 1;
        }
        frame.restore(c, 16, 0x40);
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The bank part of `PATH_TrackCallStream`: find the track (`-8` when it is
/// missing), then `PATH_PlatSetTrackFile(stream, a1, a2)`, then clear the
/// selected id and map.
fn path_track_call_stream_load(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    PATH_FIND_TRACK_CALLEE.call(c, 0x003d_3100)?;
    let found = c.g[V0][0] != 0;
    c.g[A0][0] = sx(0xffff_fff8);
    if found {
        c.cycles += 1;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x54)).lw(c);
    }
    c.cycles += 6;
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, 0x002a_e768, 0x003d_3118)?;
    c.g[V0][0] = 0x0045_0000;
    PATH_SELECTED_ID.sw(c, 0);
    PATH_SELECTED_MAP.sw(c, 0);
    Ok(())
}

/// `int PATH_TrackUnloadBank(int track, int bank)` (0x3d3140): unloads the
/// bank of a track through `PATH_PlatRemoveBank`, returns its result, or -18
/// when the layer is not started, has a selected map already, or no map can
/// be selected.
pub fn path_track_unload_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 19, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = 0x0045_0000;
        c.g[V1][0] = PATH_STARTED_FLAG.lbu(c);
        c.g[S3][0] = 0;
        frame.save(c, 18, 0x20);
        frame.save_ra(c, 0);
        path_track_unload_bank_flow(c)?;
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The body of `PATH_TrackUnloadBank` between the prologue and the epilogue.
fn path_track_unload_bank_flow(c: &mut Cpu) -> Flow {
    if c.g[V1][0] == 0 {
        return path_track_unload_bank_none(c);
    }
    c.cycles += 4;
    c.g[S2][0] = 0x0045_0000;
    c.g[V0][0] = PATH_SELECTED_ID.lw(c);
    let selected = c.g[V0][0] != 0;
    c.g[V0][0] = sx(0xffff_ffee);
    if selected {
        return Ok(());
    }
    c.cycles += 4;
    c.g[A0][0] = sx(0xffff_ffff);
    PATH_SELECT_MAP_CALLEE.call(c, 0x003d_3188)?;
    if c.g[V0][0] == 0 {
        return path_track_unload_bank_none(c);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    PATH_FIND_TRACK_CALLEE.call(c, 0x003d_31a0)?;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = 0x0045_0000;
    } else {
        c.cycles += 5;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x54)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002a_e328, 0x003d_31b4)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[V0][0] = 0x0045_0000;
    }
    c.cycles += 3;
    PATH_SELECTED_ID.sw(c, 0);
    PATH_SELECTED_MAP.sw(c, 0);
    c.g[V0][0] = c.g[S3][0];
    Ok(())
}

/// The -18 exit of `PATH_TrackUnloadBank`.
fn path_track_unload_bank_none(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[V0][0] = sx(0xffff_ffee);
    Ok(())
}

/// `void PATH_SetMapCallbacks(int mask, int a1, int a2)` (0x3d31e8): when the
/// layer is started and no map is selected, select one and give every map
/// whose mask matches `mask` the two callback words (`+0x1c`, `+0x10`).
pub fn path_set_map_callbacks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x0045_0000;
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A2][0];
        c.g[V1][0] = PATH_STARTED_FLAG.lbu(c);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);
        path_set_map_callbacks_flow(c)?;
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The body of `PATH_SetMapCallbacks` between the prologue and the epilogue.
fn path_set_map_callbacks_flow(c: &mut Cpu) -> Flow {
    if c.g[V1][0] == 0 {
        return Ok(());
    }
    c.cycles += 4;
    c.g[V1][0] = 0x0045_0000;
    c.g[V0][0] = PATH_SELECTED_ID.lw(c);
    c.g[S3][0] = c.g[V1][0];
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 4;
    c.g[A0][0] = sx(0xffff_ffff);
    PATH_SELECT_MAP_CALLEE.call(c, 0x003d_3230)?;
    let selected = c.g[V0][0] != 0;
    c.g[V0][0] = 0x0051_0000;
    if !selected {
        return Ok(());
    }
    c.cycles += 6;
    c.g[V1][0] = 0x0f00_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7610));
    c.g[S0][0] = c.g[S0][0] & c.g[V1][0];
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = 3;
    loop {
        c.cycles += 3;
        let map = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(map.wrapping_add(0x24)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        if !empty {
            c.cycles += 4;
            let masked = Addr(map.wrapping_add(0x20)).lw(c) & c.g[S0][0];
            c.g[V0][0] = masked;
            if masked != 0 {
                c.cycles += 2;
                let s2 = c.g[S2][0];
                Addr(map.wrapping_sub(4)).sw(c, s2);
                let s1 = c.g[S1][0];
                Addr(map).sw(c, s1);
            }
        }
        c.cycles += 2;
        let more = (c.g[A0][0] as i64) >= 0;
        c.g[V1][0] = sx(map.wrapping_add(0x928));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003d_3250);
            }
            continue;
        }
        c.cycles += 3;
        c.g[V0][0] = 0x0045_0000;
        PATH_SELECTED_ID.sw(c, 0);
        PATH_SELECTED_MAP.sw(c, 0);
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fall_back, hook, machine};
    use crate::difftest::{check, DATA};
    use std::sync::atomic::AtomicUsize;

    const SEEDS: u32 = 120;
    /// Gate words (magic in this machine's RAM) of the fakes below.
    const GATE_STOP: u32 = DATA + 0x0f_f000;
    const GATE_BANK: u32 = DATA + 0x0f_f100;
    const GATE_TRACK: u32 = DATA + 0x0f_f200;
    const MAGIC: u32 = 0x1106_c0de;
    /// The selected path map's id and loaded flag (map 0 of `MAPS`).
    const MAPS: u32 = 0x0051_7610;
    const MAP_SIZE: u32 = 0x928;

    static PREV_STOP: AtomicUsize = AtomicUsize::new(0);
    static PREV_BANK: AtomicUsize = AtomicUsize::new(0);
    static PREV_TRACK: AtomicUsize = AtomicUsize::new(0);

    /// `PATH_StopTracks` fake: returns 0 without touching the player.
    fn oc1106_fake_stop(c: &mut Cpu) {
        if c.bus.read32(GATE_STOP) != MAGIC {
            return fall_back(c, &PREV_STOP, ee::s14::f_003d2a68__generated);
        }
        c.g[V0][0] = 0;
        ret(c);
    }

    /// `PATH_PlatRemoveBank` fake: returns the forced result in its mailbox.
    fn oc1106_fake_bank(c: &mut Cpu) {
        if c.bus.read32(GATE_BANK) != MAGIC {
            return fall_back(c, &PREV_BANK, ee::s08::f_002ae328__generated);
        }
        c.g[V0][0] = sx(c.bus.read32(GATE_BANK + 4));
        ret(c);
    }

    /// `PATH_PlatSetTrackFile` fake: returns 0 without touching the platform.
    fn oc1106_fake_track(c: &mut Cpu) {
        if c.bus.read32(GATE_TRACK) != MAGIC {
            return fall_back(c, &PREV_TRACK, ee::s08::f_002ae768__generated);
        }
        c.g[V0][0] = 0;
        ret(c);
    }

    /// Starts the path layer (or not) and selects no map (0) or a map id.
    fn path_state(c: &mut Cpu, seed: u32, r: &mut crate::difftest::Rng) {
        c.bus
            .write8(PATH_STARTED_FLAG.0, if seed % 4 == 0 { 0 } else { 1 });
        let selected = if seed % 3 == 0 { 0 } else { r.next_u32() | 1 };
        c.bus.write32(PATH_SELECTED_ID.0, selected);
        c.bus.write32(PATH_SELECTED_MAP.0, r.next_u32());
    }

    #[test]
    fn path_shutdown_matches() {
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003D2A68),
            oc1106_fake_stop,
            &PREV_STOP,
        );
        check(
            ee::s14::f_003d2fe0__generated,
            path_shutdown,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1106_0001, 0);
                c.bus.write32(GATE_STOP, MAGIC);
                path_state(c, seed, &mut r);
            },
        );
    }

    #[test]
    fn path_track_call_stream_matches() {
        hook(
            std::ptr::addr_of_mut!(ee::s08::OVERRIDE_002AE768),
            oc1106_fake_track,
            &PREV_TRACK,
        );
        check(
            ee::s14::f_003d30a8__generated,
            path_track_call_stream,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1106_0002, 0);
                c.bus.write32(GATE_TRACK, MAGIC);
                path_state(c, seed, &mut r);
                c.g[A0][0] = (r.next_u32() % 8) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                // A loaded first map, so the track search can find something.
                c.bus
                    .write32(MAPS + 0x34, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn path_track_unload_bank_matches() {
        hook(
            std::ptr::addr_of_mut!(ee::s08::OVERRIDE_002AE328),
            oc1106_fake_bank,
            &PREV_BANK,
        );
        check(
            ee::s14::f_003d3140__generated,
            path_track_unload_bank,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1106_0003, 0);
                c.bus.write32(GATE_BANK, MAGIC);
                c.bus.write32(GATE_BANK + 4, r.next_u32());
                path_state(c, seed, &mut r);
                c.g[A0][0] = (r.next_u32() % 8) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.bus
                    .write32(MAPS + 0x34, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn path_set_map_callbacks_matches() {
        check(
            ee::s14::f_003d31e8__generated,
            path_set_map_callbacks,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1106_0004, 0);
                path_state(c, seed, &mut r);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                // Four maps: a loaded count and a mask word each.
                for m in 0..4u32 {
                    let map = MAPS + MAP_SIZE * m;
                    c.bus.write32(map + 0x24, r.next_u32() % 3);
                    c.bus.write32(map + 0x20, r.next_u32());
                }
            },
        );
    }
}
