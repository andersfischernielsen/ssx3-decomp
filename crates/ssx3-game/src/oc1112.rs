//! Staging module oc1112 (wave wave269): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1112`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_6108, path_set_fade_time, "PATH_SetFadeTime"),
    (0x003d_61e8, path_set_fade_curve, "PATH_SetFadeCurve"),
    (0x003d_6318, path_track_fade_volume, "PATH_TrackFadeVolume"),
    (
        0x003d_6360,
        path_track_set_aux_send_fade,
        "PATH_TrackSetAuxSendFade",
    ),
    (
        0x003d_6378,
        path_track_start_volume_fade,
        "PATH_TrackStartVolumeFade",
    ),
    (
        0x003d_63f8,
        path_track_start_aux_send_fade,
        "PATH_TrackStartAuxSendFade",
    ),
    (0x003d_6448, path_track_start_fade3, "PATH_TrackStartFade3"),
    (
        0x003d_6748,
        spch_add_event_table_default,
        "SPCH_AddEventTableDefault",
    ),
];

/// `D_0044F42C`: the byte that says the path system is set up.
const PATH_FLAG_OC1112: u32 = 0x0044_f42c;
/// `D_0044F430`: the selected map id (0 unset, -1 none).
const CURRENT_ID_OC1112: u32 = 0x0044_f430;
/// `D_0044F420`: the selected path map.
const CURRENT_MAP_OC1112: u32 = 0x0044_f420;
/// `D_0044F438`: the tick the track start ticks are stamped with.
const TICK_NOW_OC1112: u32 = 0x0044_f438;

/// `PATH_SelectMap` (0x3d5450).
const SELECT_MAP_OC1112: Callee = Callee {
    addr: 0x003d_5450,
    f: ee::s14::f_003d5450,
};

/// `SPCH_RegisterEventTable` (0x3d6498).
const REGISTER_EVENT_TABLE_OC1112: Callee = Callee {
    addr: 0x003d_6498,
    f: ee::s14::f_003d6498,
};

/// `PATH_PlatFadeVolume` (0x2af608).
const PLAT_FADE_VOLUME_OC1112: u32 = 0x002a_f608;
/// `PATH_PlatGetTrackVolume` (0x2af428).
const PLAT_GET_TRACK_VOLUME_OC1112: u32 = 0x002a_f428;

/// What a map fade pass writes and where its generated body resumes.
#[derive(Clone, Copy)]
struct MapFadeOc1112 {
    /// Offset of the map field the value goes to (0x14 for the time, 0x18
    /// for the curve).
    field: u32,
    /// Return address of the first `PATH_SelectMap(-1)`.
    top_ret: u32,
    /// Return address of the per-map `PATH_SelectMap(i)`.
    row_ret: u32,
    /// The loop head, where a due event resumes.
    head: u32,
}

/// `PATH_SetFadeTime` (0x3d6108).
const FADE_TIME_OC1112: MapFadeOc1112 = MapFadeOc1112 {
    field: 0x14,
    top_ret: 0x003d_6154,
    row_ret: 0x003d_619c,
    head: 0x003d_6178,
};

/// `PATH_SetFadeCurve` (0x3d61e8).
const FADE_CURVE_OC1112: MapFadeOc1112 = MapFadeOc1112 {
    field: 0x18,
    top_ret: 0x003d_6234,
    row_ret: 0x003d_627c,
    head: 0x003d_6258,
};

/// The map loop shared by `PATH_SetFadeTime` and `PATH_SetFadeCurve`. Reads
/// the mask in s2 and the value in s4 (both set by the caller). When no map
/// is selected yet, a path is set up, the first selection succeeds, every
/// map whose player mask meets the mask gets the value stored in its field,
/// and the selection is cleared at the end. Returns `Ok` on the generated
/// body's early exits, `Err` when a callee unwinds.
fn set_map_fades_oc1112(c: &mut Cpu, m: MapFadeOc1112) -> Flow {
    c.cycles += 4;
    c.g[V1][0] = 0x0045_0000;
    c.g[V0][0] = Addr(CURRENT_ID_OC1112).lw(c);
    let already_selected = c.g[V0][0] != 0;
    c.g[S5][0] = c.g[V1][0];
    if already_selected {
        return Ok(());
    }
    c.cycles += 4;
    c.g[RA][0] = sx(m.top_ret);
    c.g[A0][0] = u64::MAX;
    SELECT_MAP_OC1112.call(c, m.top_ret)?;
    let chosen = c.g[V0][0];
    c.g[V0][0] = 0x0051_0000;
    if chosen == 0 {
        return Ok(());
    }
    // The map table starts at 0x517610, rows of 0x928 bytes.
    c.cycles += 7;
    c.g[V1][0] = 0x0f00_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7610));
    c.g[S2][0] &= c.g[V1][0];
    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
    c.g[S0][0] = 0;
    c.g[S3][0] = 0x0045_0000;
    loop {
        // Row i: m_a at +0x30 (player mask), m_b at +0x34 (loaded flag).
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(4)).lw(c);
        if c.g[V0][0] == 0 {
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[V0][0] &= c.g[S2][0];
            if c.g[V0][0] == 0 {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(m.row_ret);
                c.g[A0][0] = c.g[S0][0] & 0xff;
                SELECT_MAP_OC1112.call(c, m.row_ret)?;
                let row_chosen = c.g[V0][0];
                c.g[V0][0] = Addr(CURRENT_MAP_OC1112).lw(c);
                if row_chosen != 0 {
                    c.cycles += 1;
                    let v = c.g[S4][0];
                    Addr((c.g[V0][0] as u32).wrapping_add(m.field)).sw(c, v);
                }
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x928));
        if more {
            if event_due(c) {
                deliver_event_here(c, m.head);
            }
            continue;
        }
        c.cycles += 2;
        Addr(CURRENT_ID_OC1112).sw(c, 0);
        Addr(CURRENT_MAP_OC1112).sw(c, 0);
        return Ok(());
    }
}

/// The frame and arguments of the two fade-time passes: `mask` in a0 and
/// `value` in a1 (kept in s2 and s4 for the map loop).
fn set_map_fades_frame_oc1112(c: &mut Cpu, m: MapFadeOc1112) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x0045_0000;
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[V1][0] = Addr(PATH_FLAG_OC1112).lbu(c);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        if c.g[V1][0] != 0 {
            set_map_fades_oc1112(c, m)?;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PATH_SetFadeTime(uint mask, int time)` (0x3d6108): when the path
/// system is set up and no map is selected, stores `time` into the fade-time
/// field of every map whose player mask meets `mask`.
pub fn path_set_fade_time(c: &mut Cpu) {
    set_map_fades_frame_oc1112(c, FADE_TIME_OC1112)
}

/// `void PATH_SetFadeCurve(uint mask, int curve)` (0x3d61e8): the same pass as
/// [`path_set_fade_time`], storing into the fade-curve field.
pub fn path_set_fade_curve(c: &mut Cpu) {
    set_map_fades_frame_oc1112(c, FADE_CURVE_OC1112)
}

/// `void PATH_TrackFadeVolume(PathTrack* self, int level, int w)` (0x3d6318):
/// stores the level in byte +0xe, clears the ramp byte +0xd, and starts the
/// platform fade `PATH_PlatFadeVolume(handle, level * self[4] / 100, w, 1)`.
pub fn path_track_fade_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = 0x64;
        let this = c.g[V0][0] as u32;
        let v = c.g[A1][0];
        Addr(this.wrapping_add(0xe)).sb(c, v);
        c.g[A3][0] = 1;
        c.g[A0][0] = Addr(this.wrapping_add(4)).lb(c);
        c.cycles += 9;
        Addr(this.wrapping_add(0xd)).sb(c, 0);
        // a1 = a1 * a0 / v1 (100), as the generated mult/div leave it.
        c.mult(0, 5, 5, 4);
        c.div(0, 5, 3);
        c.g[A1][0] = c.lo[0];
        c.g[A0][0] = Addr(this.wrapping_add(0x54)).lw(c);
        call_ptr(c, PLAT_FADE_VOLUME_OC1112, 0x003d_6354)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PATH_TrackSetAuxSendFade(PathTrack* ch, int target, int ramp)`
/// (0x3d6360): stores the target into byte +0x16, and the ramp into +0x15
/// when it is not negative. A leaf.
pub fn path_track_set_aux_send_fade(c: &mut Cpu) {
    c.cycles += 2;
    let channel = c.g[A0][0] as u32;
    let ramp_negative = (c.g[A3][0] as i64) < 0;
    let v = c.g[A1][0];
    Addr(channel.wrapping_add(0x16)).sb(c, v);
    if !ramp_negative {
        c.cycles += 1;
        let v = c.g[A3][0];
        Addr(channel.wrapping_add(0x15)).sb(c, v);
    }
    c.cycles += 2;
    ret(c);
}

/// `void PATH_TrackStartVolumeFade(PathTrack* self, int target, int ramp)`
/// (0x3d6378): stamps the tick, sets the ramp byte, reads the platform volume
/// and stores the old and target levels as truncated integers (the bytes
/// +0xc and +0x34), with the target in +0xe.
pub fn path_track_start_volume_fade(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0045_0000;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = Addr(TICK_NOW_OC1112).lw(c);
        let this = c.g[S0][0] as u32;
        c.g[A0][0] = Addr(this.wrapping_add(0x54)).lw(c);
        let v = c.g[V1][0];
        Addr(this.wrapping_add(8)).sw(c, v);
        let v = c.g[A2][0];
        Addr(this.wrapping_add(0xd)).sb(c, v);
        call_ptr(c, PLAT_GET_TRACK_VOLUME_OC1112, 0x003d_63a8)?;
        // The callee's volume (v0) as a float: the level before the fade.
        let level = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
        abi::set_fpr(c, 1, level);
        let v = c.g[S1][0];
        Addr(this.wrapping_add(0xe)).sb(c, v);
        // The target as a float (mtc1 s1, cvt.s.w).
        let target = FpuF32::from_i32(c.g[S1][0] as u32 as i32);
        abi::set_fpr(c, 0, target);
        c.g[A0][0] = 0x7f;
        // cvt.w.s and mfc1: the old level, truncated, into v0.
        let level_int = FpuF32(level.to_i32() as u32);
        abi::set_fpr(c, 2, level_int);
        c.g[V0][0] = abi::mfc1(level_int);
        // cvt.w.s and mfc1: the target, truncated, into v1.
        let target_int = FpuF32(target.to_i32() as u32);
        abi::set_fpr(c, 1, target_int);
        c.g[V1][0] = abi::mfc1(target_int);
        let v = c.g[V0][0];
        Addr(this.wrapping_add(0xc)).sb(c, v);
        let v = c.g[V1][0];
        Addr(this.wrapping_add(0x34)).sb(c, v);
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The field layout and platform query of a track-start helper.
#[derive(Clone, Copy)]
struct TrackStartOc1112 {
    /// Word: the tick the fade started at.
    tick: u32,
    /// Byte: the ramp flag (a2).
    ramp: u32,
    /// Byte: the target level (a1).
    target: u32,
    /// Byte: the current level the query returned (v0).
    cur: u32,
    /// The platform query called with the track's handle.
    query: u32,
    /// Its return address.
    ret: u32,
}

/// `PATH_TrackStartAuxSendFade` (0x3d63f8), querying the aux send.
const START_AUX_SEND_OC1112: TrackStartOc1112 = TrackStartOc1112 {
    tick: 0x10,
    ramp: 0x15,
    target: 0x16,
    cur: 0x14,
    query: 0x002a_f4b0,
    ret: 0x003d_6428,
};

/// `PATH_TrackStartFade3` (0x3d6448), querying the handle.
const START_FADE3_OC1112: TrackStartOc1112 = TrackStartOc1112 {
    tick: 0x18,
    ramp: 0x1d,
    target: 0x1e,
    cur: 0x1c,
    query: 0x002a_f480,
    ret: 0x003d_6478,
};

/// The shape shared by the two track-start helpers: stamp the tick, set the
/// ramp, query the platform value, store the target and that value.
fn track_start_fade_oc1112(c: &mut Cpu, t: TrackStartOc1112) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0045_0000;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = Addr(TICK_NOW_OC1112).lw(c);
        let this = c.g[S0][0] as u32;
        c.g[A0][0] = Addr(this.wrapping_add(0x54)).lw(c);
        let v = c.g[V1][0];
        Addr(this.wrapping_add(t.tick)).sw(c, v);
        let v = c.g[A2][0];
        Addr(this.wrapping_add(t.ramp)).sb(c, v);
        call_ptr(c, t.query, t.ret)?;
        let v = c.g[S1][0];
        Addr(this.wrapping_add(t.target)).sb(c, v);
        let v = c.g[V0][0];
        Addr(this.wrapping_add(t.cur)).sb(c, v);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PATH_TrackStartAuxSendFade(PathTrack* ch, int target, int ramp)`
/// (0x3d63f8).
pub fn path_track_start_aux_send_fade(c: &mut Cpu) {
    track_start_fade_oc1112(c, START_AUX_SEND_OC1112)
}

/// `void PATH_TrackStartFade3(PathTrack* ch, int target, int ramp)` (0x3d6448).
pub fn path_track_start_fade3(c: &mut Cpu) {
    track_start_fade_oc1112(c, START_FADE3_OC1112)
}

/// `void SPCH_AddEventTableDefault(void* data)` (0x3d6748): registers the
/// bank data with key 0 through `SPCH_RegisterEventTable`.
pub fn spch_add_event_table_default(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save(c, RA, 0);
        c.g[A1][0] = 0;
        REGISTER_EVENT_TABLE_OC1112.call(c, 0x003d_6758)?;
        frame.restore(c, RA, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::{machine, machine_rt};
    use crate::difftest::DATA;

    /// The path system set up or not, no map selected or one, the map table
    /// with masks and loaded flags, and the mask and value arguments (a zero
    /// mask for every sixth seed).
    fn map_fade_setup_oc1112(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.bus
            .write8(PATH_FLAG_OC1112, if seed % 5 == 0 { 0 } else { 1 });
        c.bus.write32(
            CURRENT_ID_OC1112,
            if seed % 4 == 0 { r.next_u32() | 1 } else { 0 },
        );
        c.bus.write32(CURRENT_MAP_OC1112, 0);
        for m in 0..4u32 {
            let base = 0x0051_7610 + m * 0x928;
            let mask_word = if (seed + m) % 3 == 0 { 0 } else { r.next_u32() };
            c.bus.write32(base + 0x30, mask_word);
            c.bus
                .write32(base + 0x34, if (seed >> m) & 1 == 0 { 0 } else { 1 });
        }
        c.g[A0][0] = sx(if seed % 6 == 0 { 0 } else { r.next_u32() });
        c.g[A1][0] = sx(r.next_u32());
    }

    #[test]
    fn path_set_fade_time_matches() {
        check(
            ee::s14::f_003d6108__generated,
            path_set_fade_time,
            300,
            &|c, seed| map_fade_setup_oc1112(c, seed, 0x3d_6108),
        );
    }

    #[test]
    fn path_set_fade_curve_matches() {
        check(
            ee::s14::f_003d61e8__generated,
            path_set_fade_curve,
            300,
            &|c, seed| map_fade_setup_oc1112(c, seed, 0x3d_61e8),
        );
    }

    /// A platform entry for the track's index (the object the fade reads),
    /// the track at `DATA+0x400`, and the level and ramp arguments.
    fn track_fade_setup_oc1112(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine_rt(c, seed, salt, 0);
        let idx = seed % 3;
        let obj = DATA + 0x1_000;
        c.bus.write32(0x004d_3e98 + 4 * idx, obj);
        c.bus.write32(obj, DATA + 0x900);
        c.bus.write32(obj + 8, r.next_u32());
        c.bus.write32(obj + 0x6c, r.next_u32());
        c.bus.write8(obj + 0x64, (seed % 3) as u8);
        let track = DATA + 0x400;
        c.bus.write32(track + 0x54, idx);
        c.bus.write8(track + 4, r.next_u32() as u8);
        c.g[A0][0] = sx(track);
        c.g[A1][0] = sx(r.next_u32() % 0x200);
        c.g[A2][0] = sx(r.next_u32());
    }

    #[test]
    fn path_track_fade_volume_matches() {
        check_lohi(
            ee::s14::f_003d6318__generated,
            path_track_fade_volume,
            120,
            &|c, seed| track_fade_setup_oc1112(c, seed, 0x3d_6318),
        );
    }

    #[test]
    fn path_track_set_aux_send_fade_matches() {
        check(
            ee::s14::f_003d6360__generated,
            path_track_set_aux_send_fade,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_6360, 0);
                let channel = DATA + 0x600;
                c.g[A0][0] = sx(channel);
                c.g[A1][0] = sx(r.next_u32());
                let ramp = r.next_u32();
                c.g[A3][0] = if seed % 2 == 0 {
                    sx(ramp | 0x8000_0000)
                } else {
                    sx(ramp & 0x7fff_ffff)
                };
                c.bus.write8(channel + 0x15, r.next_u32() as u8);
                c.bus.write8(channel + 0x16, r.next_u32() as u8);
            },
        );
    }

    /// A track at `DATA+0x400` whose platform index is small (the table
    /// is empty, so the query answers -8), with the target and ramp.
    fn track_start_setup_oc1112(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let track = DATA + 0x400;
        c.bus.write32(track + 0x54, seed % 8);
        c.g[A0][0] = sx(track);
        c.g[A1][0] = sx(r.next_u32() % 0x200);
        c.g[A2][0] = sx(r.next_u32());
    }

    #[test]
    fn path_track_start_volume_fade_matches() {
        check(
            ee::s14::f_003d6378__generated,
            path_track_start_volume_fade,
            200,
            &|c, seed| track_start_setup_oc1112(c, seed, 0x3d_6378),
        );
    }

    #[test]
    fn path_track_start_aux_send_fade_matches() {
        check(
            ee::s14::f_003d63f8__generated,
            path_track_start_aux_send_fade,
            200,
            &|c, seed| track_start_setup_oc1112(c, seed, 0x3d_63f8),
        );
    }

    #[test]
    fn path_track_start_fade3_matches() {
        check(
            ee::s14::f_003d6448__generated,
            path_track_start_fade3,
            200,
            &|c, seed| track_start_setup_oc1112(c, seed, 0x3d_6448),
        );
    }

    #[test]
    fn spch_add_event_table_default_matches() {
        check(
            ee::s14::f_003d6748__generated,
            spch_add_event_table_default,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x3d_6748, 0);
                // Empty bank data: the magic halfword is not there, so the
                // registration takes its short path.
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }
}
