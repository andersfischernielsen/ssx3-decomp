//! Staging module oc1135 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1135`.

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
    (0x003d_0c20, path_skip_remaining_actions, "PATH_SkipRemainingActions"),
    (0x003d_1470, path_stop_all_tracks, "PATH_StopAllTracks"),
    (0x003d_1608, path_fatal_exit, "PATH_FatalExit"),
];

/// `PATH_SelectMap` (0x3d5450).
const OC1135_SELECT_MAP: Callee = Callee {
    addr: 0x003d_5450,
    f: ee::s14::PATH_SelectMap,
};

/// `PATH_DebugPrintf` (0x3d1658), tail-jumped to by `PATH_FatalExit`.
const OC1135_DEBUG_PRINTF: Callee = Callee {
    addr: 0x003d_1658,
    f: ee::s14::PATH_DebugPrintf,
};

/// Path player initialised flag (`u8`).
pub(crate) const PATH_STOP_ENABLED: u32 = 0x0044_f42c;
/// Selected map id.
pub(crate) const PATH_STOP_CUR_ID: u32 = 0x0044_f430;
/// Selected map pointer.
pub(crate) const PATH_STOP_CUR_MAP: u32 = 0x0044_f420;
/// `&maps[0].id`; map stride 0x928; `loaded` at +4 of it.
pub(crate) const PATH_STOP_MAP0_ID: u32 = 0x0051_7640;
pub(crate) const PATH_STOP_MAP_SIZE: u64 = 0x928;
/// Fake-able callees.
pub(crate) const PATH_PLAT_STOP_TRACK_SOUND: u32 = 0x002a_f838;
pub(crate) const PATH_EXIT: u32 = 0x0041_1310;

/// `void PATH_SkipRemainingActions(PathAction* first, PathAction* last)`
/// (0x3d0c20): walks 16-byte actions from `first + 0x10` while it is not
/// past `last`, and sets bit 0x4000000 in the flags word (+8) of each one
/// until the first whose byte at +9 is 1.
pub fn path_skip_remaining_actions(c: &mut Cpu) {
    'out: {
        c.cycles += 4;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
        c.g[V0][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
        let past = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if past {
            break 'out;
        }
        c.cycles += 3;
        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(9)).lbu(c);
        let stop = c.g[V1][0] == c.g[V0][0];
        c.g[A2][0] = 0x0400_0000;
        if stop {
            break 'out;
        }
        c.cycles += 3;
        c.g[V1][0] = 1;
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(8)).lw(c);
        loop {
            c.cycles += 6;
            c.g[V0][0] |= c.g[A2][0];
            let v = c.g[V0][0];
            Addr(r32(c, A0).wrapping_add(8)).sw(c, v);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
            c.g[V0][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
            if c.g[V0][0] != 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(9)).lbu(c);
            if c.g[V0][0] != c.g[V1][0] {
                c.g[V0][0] = Addr(r32(c, A0).wrapping_add(8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003d_0c48);
                }
                continue;
            }
            break 'out;
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `void PATH_StopAllTracks(u32 mask)` (0x3d1470): when the player is on
/// and no map is selected, query `PATH_SelectMap(-1)`; then for each of the
/// four maps whose id has the mask in its top and second nibbles, select it
/// (`PATH_SelectMap(i)`) and stop the sound of every track slot (24 at
/// +0x58), setting each track's halfword at +0x36 to -1. Clears the
/// selection at the end.
pub fn path_stop_all_tracks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xa0);
        c.g[V1][0] = 0x0045_0000;
        frame.save(c, S4, 0x50);
        c.g[V0][0] = Addr(PATH_STOP_ENABLED).lbu(c);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        let off = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        'epi: {
            'tail: {
                if off {
                    break 'tail;
                }
                c.cycles += 4;
                c.g[V1][0] = 0x0045_0000;
                c.g[V0][0] = Addr(PATH_STOP_CUR_ID).lw(c);
                frame.restore(c, S0, 0x90);
                if c.g[V0][0] != 0 {
                    break 'epi;
                }
                c.cycles += 4;
                c.g[A0][0] = u64::MAX;
                OC1135_SELECT_MAP.call(c, 0x003d_14c4)?;
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = 0x0051_0000;
                if none {
                    break 'tail;
                }
                c.cycles += 7;
                c.g[A0][0] = 0;
                c.g[S6][0] = sx(r32(c, V0).wrapping_add(0x7644));
                c.g[S5][0] = 0x0045_0000;
                c.g[FP][0] = 0xffff_ffff_f000_0000;
                c.g[S7][0] = 0x0f00_0000;
                c.g[V0][0] = PATH_STOP_MAP_SIZE;
                loop {
                    'next: {
                        'skip: {
                            c.cycles += 5;
                            c.mult(0, A1, A0, V0);
                            c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, S6)));
                            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                            let empty = c.g[V1][0] == 0;
                            c.g[S3][0] = sx(r32(c, A0).wrapping_add(1));
                            if empty {
                                break 'skip;
                            }
                            c.cycles += 8;
                            c.g[V0][0] = 0x0051_0000;
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x7640));
                            c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                            c.g[V1][0] = Addr(r32(c, V1)).lw(c);
                            c.g[V0][0] = c.g[V1][0] & c.g[FP][0];
                            c.g[V0][0] &= c.g[S4][0];
                            let no_top = c.g[V0][0] == 0;
                            c.g[V0][0] = c.g[V1][0] & c.g[S7][0];
                            if no_top {
                                break 'skip;
                            }
                            c.cycles += 3;
                            c.g[V0][0] &= c.g[S4][0];
                            if c.g[V0][0] == 0 {
                                c.g[A0][0] = c.g[S3][0];
                                break 'next;
                            }
                            c.cycles += 4;
                            OC1135_SELECT_MAP.call(c, 0x003d_1530)?;
                            let none = c.g[V0][0] == 0;
                            c.g[A0][0] = c.g[S3][0];
                            if none {
                                break 'next;
                            }
                            c.cycles += 4;
                            c.g[S1][0] = 0;
                            c.g[S2][0] = u64::MAX;
                            c.g[V0][0] = Addr(PATH_STOP_CUR_MAP).lw(c);
                            loop {
                                c.cycles += 5;
                                c.g[V1][0] = sx(r32(c, S1) << 2);
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                                c.g[S0][0] = Addr(r32(c, V0).wrapping_add(0x58)).lw(c);
                                if c.g[S0][0] == 0 {
                                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                                } else {
                                    c.cycles += 4;
                                    c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x54)).lw(c);
                                    call_ptr(c, PATH_PLAT_STOP_TRACK_SOUND, 0x003d_1564)?;
                                    let h = c.g[S2][0];
                                    Addr(r32(c, S0).wrapping_add(0x36)).sh(c, h);
                                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[S1][0] as i64) < 24) as u64;
                                let more = c.g[V0][0] != 0;
                                c.g[V0][0] = Addr(PATH_STOP_CUR_MAP).lw(c);
                                if more {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003d_1548);
                                    }
                                    continue;
                                }
                                break 'skip;
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S3][0];
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = PATH_STOP_MAP_SIZE;
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_14e8);
                        }
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x0045_0000;
                    Addr(PATH_STOP_CUR_MAP).sw(c, 0);
                    Addr(r32(c, V0).wrapping_sub(0xbd0)).sw(c, 0);
                    break 'tail;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PATH_FatalExit(const char* fmt, ...)` (0x3d1608): marks the
/// selected map id as -1, spills the variadic arguments (a1..t3, f12, f14,
/// f16, f18) into the frame, calls `exit(0)` and then tail-jumps into
/// `PATH_DebugPrintf` with the frame still open.
pub fn path_fatal_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        let frame = Frame::open(c, 0x90);
        c.g[V1][0] = 0x0045_0000;
        c.g[V0][0] |= 0xffff;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        let s = |c: &Cpu, o: u32| frame.at(c, o);
        let a = s(c, 0x58);
        c.sd(a, c.g[A1][0]);
        let v = c.g[V0][0];
        Addr(r32(c, V1).wrapping_sub(0xbd0)).sw(c, v);
        for (o, r) in [(0x60, A2), (0x68, A3), (0x70, T0), (0x78, T1), (0x80, T2), (0x88, T3)] {
            let a = s(c, o);
            c.sd(a, c.g[r][0]);
        }
        for (o, f) in [(0x48, 12), (0x4c, 14), (0x50, 16)] {
            let a = s(c, o);
            c.swc1(f, a);
        }
        c.g[RA][0] = sx(0x003d_1654);
        let a = s(c, 0x54);
        c.swc1(18, a);
        call_ptr(c, PATH_EXIT, 0x003d_1654)?;
        c.pc = OC1135_DEBUG_PRINTF.addr;
        (OC1135_DEBUG_PRINTF.f)(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fake_call, fall_back, hook, machine};
    use crate::difftest::{check, check_lohi, fill, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (own RAM address and magic).
    const OC1135_CTRL: u32 = DATA + 0x20_0100;
    const OC1135_MAGIC: u32 = 0x1135_a5a5;

    static OC1135_PREV_STOP_SOUND: AtomicUsize = AtomicUsize::new(0);
    static OC1135_PREV_EXIT: AtomicUsize = AtomicUsize::new(0);

    fn oc1135_fake_stop_sound(c: &mut Cpu) {
        if c.bus.read32(OC1135_CTRL) != OC1135_MAGIC {
            return fall_back(c, &OC1135_PREV_STOP_SOUND, ee::s08::f_002af838__generated);
        }
        let pc = c.pc;
        fake_call(c, pc);
    }

    fn oc1135_fake_exit(c: &mut Cpu) {
        if c.bus.read32(OC1135_CTRL) != OC1135_MAGIC {
            return fall_back(c, &OC1135_PREV_EXIT, ee::s15::f_00411310__generated);
        }
        let pc = c.pc;
        fake_call(c, pc);
    }

    fn oc1135_install() {
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AF838),
            oc1135_fake_stop_sound,
            &OC1135_PREV_STOP_SOUND,
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00411310),
            oc1135_fake_exit,
            &OC1135_PREV_EXIT,
        );
    }

    /// PATH_SkipRemainingActions: lists of 16-byte actions with random flag
    /// words and a 0/1/2 byte at +9 (1 stops the walk); `last` before, at and
    /// beyond the end of the list.
    #[test]
    fn path_skip_remaining_actions_matches() {
        check(
            ee::s14::f_003d0c20__generated,
            path_skip_remaining_actions,
            300,
            &|c, seed| {
                let mut r = Rng(0x003d_0c20 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003d_0c20;
                let base = DATA + 0x1000;
                for i in 0..10u32 {
                    let a = base + 0x10 * i;
                    fill(c, &mut r, a, 4);
                    let stop = match r.next_u32() % 5 {
                        0 => 1,
                        1 => 2,
                        _ => 0,
                    };
                    c.bus.write8(a + 9, stop as u8);
                }
                c.g[A0][0] = sx(base);
                c.g[A1][0] = sx(match seed % 4 {
                    0 => base,
                    1 => base + 0x10,
                    2 => base + 0x10 * (r.next_u32() % 10),
                    _ => base + 0x200,
                });
            },
        );
    }

    /// PATH_StopAllTracks: player on or off, a map already selected or not,
    /// masks (sometimes all ones) against four maps with random ids and
    /// loaded flags, 24 slots each (empty or a random track); the real
    /// `PATH_SelectMap`, `PATH_PlatStopTrackSound` faked (and unwinding at a
    /// chosen call for some seeds).
    #[test]
    fn path_stop_all_tracks_matches() {
        oc1135_install();
        check_lohi(
            ee::s14::f_003d1470__generated,
            path_stop_all_tracks,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_1470, 6);
                oc1135_machine_extra(c);
                c.pc = 0x003d_1470;
                let enabled = if seed % 8 == 0 { 0 } else { 1 + r.next_u32() % 255 };
                c.bus.write8(PATH_STOP_ENABLED, enabled as u8);
                let id = if seed % 8 == 1 { r.next_u32() | 1 } else { 0 };
                c.bus.write32(PATH_STOP_CUR_ID, id);
                c.bus.write32(PATH_STOP_CUR_MAP, r.next_u32());
                c.g[A0][0] = if seed % 3 == 0 {
                    u64::MAX
                } else {
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64
                };
                for m in 0..4u32 {
                    let map = PATH_STOP_MAP0_ID - 0x30 + m * PATH_STOP_MAP_SIZE as u32;
                    let top = if r.next_u32() % 4 == 0 { 0 } else { 0xff00_0000 };
                    c.bus.write32(map + 0x30, r.next_u32() | top);
                    c.bus
                        .write32(map + 0x34, if r.next_u32() % 4 == 0 { 0 } else { 1 });
                    for j in 0..24u32 {
                        let track = DATA + (m * 24 + j) * 0x80;
                        fill(c, &mut r, track, 0x20);
                        let p = if r.next_u32() % 4 == 0 { 0 } else { track };
                        c.bus.write32(map + 0x58 + 4 * j, p);
                    }
                }
            },
        );
    }

    fn oc1135_machine_extra(c: &mut Cpu) {
        c.bus.write32(OC1135_CTRL, OC1135_MAGIC);
    }

    /// PATH_FatalExit: spills the variadic registers, marks the selected id
    /// -1, calls `exit(0)` (faked, unwinding at it for some seeds) and
    /// tail-jumps to the real `PATH_DebugPrintf`.
    #[test]
    fn path_fatal_exit_matches() {
        oc1135_install();
        check(
            ee::s14::f_003d1608__generated,
            path_fatal_exit,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x3d_1608, 1);
                oc1135_machine_extra(c);
                c.pc = 0x003d_1608;
                for (i, f) in [12usize, 14, 16, 18].into_iter().enumerate() {
                    c.f[f] = 0x3f80_0000 + (seed << 4) + i as u32;
                }
            },
        );
    }
}
